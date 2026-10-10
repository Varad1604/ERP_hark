//! HTTP surface of the auth boundary.

use super::CookieSettings;
use crate::PlatformDeps;
use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::{header, request::Parts, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::sync::Arc;
use sutra_contracts::platform::auth::{
    AuthOutcome, CredentialAuthenticator, Credentials, MfaMethod, Principal, Session, SessionStore,
};
use sutra_kernel::audit::{ActorKind, AuditEntry, AuditOutcome, AuditSink};
use sutra_kernel::error::{FieldViolation, KernelError};
use sutra_web::request_id::current_correlation_id;
use sutra_web::{ApiError, ApiResult, Problem};
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Clone)]
pub struct AuthState {
    pub authenticator: Arc<dyn CredentialAuthenticator>,
    pub sessions: Arc<dyn SessionStore>,
    pub audit: Arc<dyn AuditSink>,
    pub cookie: CookieSettings,
}

impl FromRef<AuthState> for Arc<dyn SessionStore> {
    fn from_ref(s: &AuthState) -> Self {
        s.sessions.clone()
    }
}

impl FromRef<AuthState> for CookieSettings {
    fn from_ref(s: &AuthState) -> Self {
        s.cookie.clone()
    }
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// Governed login identifier (e-mail in the dev realm).
    #[schema(example = "admin@sutra.dev")]
    pub login_id: String,
    #[schema(format = Password)]
    pub password: String,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MfaVerifyRequest {
    pub challenge_id: String,
    pub code: String,
}

#[derive(Debug, Clone, Copy, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum LoginOutcome {
    Authenticated,
    MfaRequired,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub outcome: LoginOutcome,
    /// Present when `outcome = authenticated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<Principal>,
    /// Present when `outcome = mfaRequired`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mfa_methods: Vec<MfaMethod>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub principal: Principal,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

fn session_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|kv| kv.trim().split_once('='))
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v.to_owned())
}

/// Extractor: the caller's live session, or 401.
pub struct CurrentSession(pub Session);

impl<S> FromRequestParts<S> for CurrentSession
where
    S: Send + Sync,
    Arc<dyn SessionStore>: FromRef<S>,
    CookieSettings: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie = CookieSettings::from_ref(state);
        let store = <Arc<dyn SessionStore>>::from_ref(state);
        let token =
            session_cookie(&parts.headers, cookie.name()).ok_or(KernelError::Unauthenticated)?;
        let session = store
            .get(&token)
            .await?
            .ok_or(KernelError::Unauthenticated)?;
        Ok(CurrentSession(session))
    }
}

async fn establish(
    state: &AuthState,
    headers: &HeaderMap,
    principal: Principal,
) -> ApiResult<Response> {
    // Session fixation defence: always mint a new id and revoke any presented one.
    if let Some(old) = session_cookie(headers, state.cookie.name()) {
        state.sessions.revoke(&old).await?;
    }
    let session = state.sessions.create(principal.clone()).await?;
    let _ = state
        .audit
        .record(
            AuditEntry::new(
                principal.scope,
                "platform.auth.login",
                AuditOutcome::Succeeded,
            )
            .actor(ActorKind::User, Some(principal.subject.clone()))
            .correlation(current_correlation_id()),
        )
        .await;
    let body = LoginResponse {
        outcome: LoginOutcome::Authenticated,
        principal: Some(principal),
        challenge_id: None,
        mfa_methods: vec![],
    };
    let mut res = Json(body).into_response();
    let cookie = HeaderValue::from_str(&state.cookie.set_cookie(&session.id))
        .map_err(KernelError::internal)?;
    res.headers_mut().insert(header::SET_COOKIE, cookie);
    res.headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok(res)
}

/// Authenticate with login id + password.
///
/// Delegates to the configured credential authority (Keycloak in production, dev stub in Batch 0)
/// and, on success, sets an opaque `HttpOnly; Secure; SameSite=Strict` session cookie. Privileged
/// principals may receive `mfaRequired` instead and must call `/api/v1/auth/mfa/verify`.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authenticated or MFA required", body = LoginResponse),
        (status = 401, description = "Invalid credentials (generic)", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "Validation failed", body = Problem, content_type = "application/problem+json"),
    )
)]
pub async fn login(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Response> {
    let mut violations = vec![];
    if req.login_id.trim().is_empty() {
        violations.push(FieldViolation::new(
            "loginId",
            "required",
            "login id is required",
        ));
    }
    if req.password.is_empty() {
        violations.push(FieldViolation::new(
            "password",
            "required",
            "password is required",
        ));
    }
    if !violations.is_empty() {
        return Err(KernelError::Validation(violations).into());
    }
    let login_hint = req.login_id.clone();
    let outcome = state
        .authenticator
        .authenticate(Credentials {
            login_id: req.login_id,
            password: req.password,
        })
        .await;
    match outcome {
        Ok(AuthOutcome::Authenticated(principal)) => establish(&state, &headers, principal).await,
        Ok(AuthOutcome::MfaRequired {
            challenge_id,
            methods,
            ..
        }) => Ok(Json(LoginResponse {
            outcome: LoginOutcome::MfaRequired,
            principal: None,
            challenge_id: Some(challenge_id),
            mfa_methods: methods,
        })
        .into_response()),
        Err(e) => {
            // Attributable internal evidence without leaking which factor failed.
            tracing::warn!(login = %login_hint, code = e.code(), "login rejected");
            Err(e.into())
        }
    }
}

/// Complete an MFA challenge issued by `login`.
#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/verify",
    tag = "auth",
    request_body = MfaVerifyRequest,
    responses(
        (status = 200, body = LoginResponse),
        (status = 401, body = Problem, content_type = "application/problem+json"),
    )
)]
pub async fn verify_mfa(
    State(state): State<AuthState>,
    headers: HeaderMap,
    Json(req): Json<MfaVerifyRequest>,
) -> ApiResult<Response> {
    let principal = state
        .authenticator
        .verify_mfa(&req.challenge_id, &req.code)
        .await?;
    establish(&state, &headers, principal).await
}

/// Current session principal.
#[utoipa::path(
    get,
    path = "/api/v1/auth/session",
    tag = "auth",
    responses(
        (status = 200, body = SessionInfo),
        (status = 401, body = Problem, content_type = "application/problem+json"),
    )
)]
pub async fn session(CurrentSession(s): CurrentSession) -> Json<SessionInfo> {
    Json(SessionInfo {
        principal: s.principal,
        expires_at: s.expires_at,
    })
}

/// End the current session (idempotent).
#[utoipa::path(post, path = "/api/v1/auth/logout", tag = "auth", responses((status = 204, description = "Signed out")))]
pub async fn logout(State(state): State<AuthState>, headers: HeaderMap) -> ApiResult<Response> {
    if let Some(token) = session_cookie(&headers, state.cookie.name()) {
        state.sessions.revoke(&token).await?;
    }
    let mut res = StatusCode::NO_CONTENT.into_response();
    let cookie =
        HeaderValue::from_str(&state.cookie.clear_cookie()).map_err(KernelError::internal)?;
    res.headers_mut().insert(header::SET_COOKIE, cookie);
    Ok(res)
}

pub fn router(deps: PlatformDeps) -> OpenApiRouter {
    let state = AuthState {
        authenticator: deps.authenticator,
        sessions: deps.sessions,
        audit: deps.audit,
        cookie: deps.cookie,
    };
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(verify_mfa))
        .routes(routes!(session))
        .routes(routes!(logout))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::dev::{DevAuthenticator, PrivilegedRoleMfaPolicy, DEV_MFA_CODE};
    use crate::auth::session::InMemorySessionStore;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use sutra_kernel::audit::TracingAuditSink;
    use sutra_kernel::scope::{ExecutionScope, TenantId};
    use tower::ServiceExt;

    fn app(enforce_mfa: bool) -> axum::Router {
        let deps = PlatformDeps {
            authenticator: Arc::new(DevAuthenticator::new(
                "sutra".into(),
                ExecutionScope::tenant(TenantId::new()),
                Box::new(PrivilegedRoleMfaPolicy {
                    enforce: enforce_mfa,
                }),
            )),
            sessions: Arc::new(InMemorySessionStore::new(3600)),
            mfa: Arc::new(PrivilegedRoleMfaPolicy {
                enforce: enforce_mfa,
            }),
            audit: Arc::new(TracingAuditSink),
            cookie: CookieSettings {
                secure: true,
                max_age_secs: 3600,
            },
            modules: vec![],
        };
        router(deps).into()
    }

    fn post(path: &str, body: serde_json::Value, cookie: Option<&str>) -> Request<Body> {
        let mut b = Request::post(path).header("content-type", "application/json");
        if let Some(c) = cookie {
            b = b.header("cookie", c);
        }
        b.body(Body::from(body.to_string())).unwrap()
    }

    async fn json(res: Response) -> serde_json::Value {
        serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes()).unwrap()
    }

    fn cookie_pair(res: &Response) -> String {
        res.headers()[header::SET_COOKIE]
            .to_str()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .to_owned()
    }

    #[tokio::test]
    async fn login_session_logout_flow() {
        let app = app(false);
        let res = app
            .clone()
            .oneshot(post(
                "/api/v1/auth/login",
                serde_json::json!({"loginId": "aarav.desai@sutra.dev", "password": "sutra"}),
                None,
            ))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let set = res.headers()[header::SET_COOKIE]
            .to_str()
            .unwrap()
            .to_owned();
        assert!(
            set.contains("HttpOnly") && set.contains("Secure") && set.contains("SameSite=Strict")
        );
        let cookie = cookie_pair(&res);
        let body = json(res).await;
        assert_eq!(body["outcome"], "authenticated");
        assert_eq!(body["principal"]["roles"][0], "student");
        assert!(!body.to_string().contains("password"));

        let me = app
            .clone()
            .oneshot(
                Request::get("/api/v1/auth/session")
                    .header("cookie", &cookie)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(me.status(), StatusCode::OK);
        assert_eq!(json(me).await["principal"]["displayName"], "Aarav Desai");

        let out = app
            .clone()
            .oneshot(post(
                "/api/v1/auth/logout",
                serde_json::json!({}),
                Some(&cookie),
            ))
            .await
            .unwrap();
        assert_eq!(out.status(), StatusCode::NO_CONTENT);
        let me = app
            .oneshot(
                Request::get("/api/v1/auth/session")
                    .header("cookie", &cookie)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(me.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn bad_credentials_are_generic_problem() {
        let res = app(false)
            .oneshot(post(
                "/api/v1/auth/login",
                serde_json::json!({"loginId": "admin@sutra.dev", "password": "nope"}),
                None,
            ))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            res.headers()[header::CONTENT_TYPE],
            "application/problem+json"
        );
        assert_eq!(json(res).await["errorCode"], "unauthenticated");
    }

    #[tokio::test]
    async fn empty_fields_are_validation_errors() {
        let res = app(false)
            .oneshot(post(
                "/api/v1/auth/login",
                serde_json::json!({"loginId": "", "password": ""}),
                None,
            ))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(
            json(res).await["validationErrors"].as_array().map(Vec::len),
            Some(2)
        );
    }

    #[tokio::test]
    async fn mfa_hook_for_privileged_role() {
        let app = app(true);
        let res = app
            .clone()
            .oneshot(post(
                "/api/v1/auth/login",
                serde_json::json!({"loginId": "admin@sutra.dev", "password": "sutra"}),
                None,
            ))
            .await
            .unwrap();
        assert!(
            res.headers().get(header::SET_COOKIE).is_none(),
            "no session before MFA"
        );
        let body = json(res).await;
        assert_eq!(body["outcome"], "mfaRequired");
        let challenge = body["challengeId"].as_str().unwrap().to_owned();
        let res = app
            .oneshot(post(
                "/api/v1/auth/mfa/verify",
                serde_json::json!({"challengeId": challenge, "code": DEV_MFA_CODE}),
                None,
            ))
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert!(res.headers().get(header::SET_COOKIE).is_some());
    }

    #[tokio::test]
    async fn login_rotates_presented_session() {
        let app = app(false);
        let login = || {
            post(
                "/api/v1/auth/login",
                serde_json::json!({"loginId": "meera.shah@sutra.dev", "password": "sutra"}),
                None,
            )
        };
        let first = cookie_pair(&app.clone().oneshot(login()).await.unwrap());
        let second_res = app
            .clone()
            .oneshot(post(
                "/api/v1/auth/login",
                serde_json::json!({"loginId": "meera.shah@sutra.dev", "password": "sutra"}),
                Some(&first),
            ))
            .await
            .unwrap();
        let second = cookie_pair(&second_res);
        assert_ne!(first, second);
        let old = app
            .oneshot(
                Request::get("/api/v1/auth/session")
                    .header("cookie", &first)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(old.status(), StatusCode::UNAUTHORIZED);
    }
}
