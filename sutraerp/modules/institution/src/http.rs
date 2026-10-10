use crate::domain::{Institution, RegisterInstitution, RenameInstitution};
use crate::repo::{self, Actor};
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sqlx::PgPool;
use sutra_kernel::concurrency::Version;
use sutra_kernel::error::KernelError;
use sutra_kernel::scope::InstitutionId;
use sutra_web::idempotency::require_idempotency_key;
use sutra_web::request_id::current_correlation_id;
use sutra_web::{ApiResult, Problem, Scope, ScopeHeaders};
use utoipa_axum::{router::OpenApiRouter, routes};

fn with_etag(status: StatusCode, inst: Institution) -> Response {
    let etag = HeaderValue::from_str(&inst.version.to_etag()).ok();
    let mut res = (status, Json(inst)).into_response();
    if let Some(e) = etag {
        res.headers_mut().insert(header::ETAG, e);
    }
    res
}

fn actor() -> Actor {
    // Batch 1 resolves the authenticated principal here.
    Actor {
        id: None,
        correlation_id: current_correlation_id(),
    }
}

/// Register an institution (reference aggregate).
#[utoipa::path(
    post,
    path = "/api/v1/institution/institutions",
    tag = "institution",
    params(ScopeHeaders, ("Idempotency-Key" = String, Header, description = "Required. Retry-safe key (1-255 visible ASCII).")),
    request_body = RegisterInstitution,
    responses(
        (status = 201, body = Institution, headers(("ETag" = String, description = "Current version"))),
        (status = 400, body = Problem, content_type = "application/problem+json"),
        (status = 409, body = Problem, content_type = "application/problem+json"),
        (status = 422, body = Problem, content_type = "application/problem+json"),
    )
)]
pub async fn register(
    State(pool): State<PgPool>,
    Scope(scope): Scope,
    headers: HeaderMap,
    Json(cmd): Json<RegisterInstitution>,
) -> ApiResult<Response> {
    require_idempotency_key(&headers)?;
    let inst = repo::register(&pool, &scope, cmd, &actor()).await?;
    Ok(with_etag(StatusCode::CREATED, inst))
}

/// Get an institution.
#[utoipa::path(
    get,
    path = "/api/v1/institution/institutions/{id}",
    tag = "institution",
    params(ScopeHeaders, ("id" = String, Path, format = Uuid)),
    responses(
        (status = 200, body = Institution, headers(("ETag" = String))),
        (status = 404, body = Problem, content_type = "application/problem+json"),
    )
)]
pub async fn get(
    State(pool): State<PgPool>,
    Scope(scope): Scope,
    Path(id): Path<InstitutionId>,
) -> ApiResult<Response> {
    Ok(with_etag(
        StatusCode::OK,
        repo::get(&pool, &scope, id).await?,
    ))
}

/// Rename an institution (optimistic concurrency via `If-Match`).
#[utoipa::path(
    patch,
    path = "/api/v1/institution/institutions/{id}",
    tag = "institution",
    params(ScopeHeaders, ("id" = String, Path, format = Uuid), ("If-Match" = String, Header, description = "ETag of the version being modified")),
    request_body = RenameInstitution,
    responses(
        (status = 200, body = Institution, headers(("ETag" = String))),
        (status = 404, body = Problem, content_type = "application/problem+json"),
        (status = 412, body = Problem, content_type = "application/problem+json"),
        (status = 428, body = Problem, content_type = "application/problem+json"),
    )
)]
pub async fn rename(
    State(pool): State<PgPool>,
    Scope(scope): Scope,
    Path(id): Path<InstitutionId>,
    headers: HeaderMap,
    Json(cmd): Json<RenameInstitution>,
) -> ApiResult<Response> {
    let expected = headers
        .get(header::IF_MATCH)
        .and_then(|v| v.to_str().ok())
        .map(|v| {
            Version::from_if_match(v).ok_or_else(|| {
                KernelError::BadRequest("If-Match must be a strong ETag such as \"3\"".into())
            })
        })
        .transpose()?;
    let Some(expected) = expected else {
        return Ok(Problem::new(
            StatusCode::PRECONDITION_REQUIRED,
            "precondition-required",
            "If-Match header is required",
        )
        .into_response());
    };
    cmd.validate()?;
    Ok(with_etag(
        StatusCode::OK,
        repo::rename(&pool, &scope, id, expected, &cmd.name, &actor()).await?,
    ))
}

pub fn router(pool: PgPool) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register))
        .routes(routes!(get, rename))
        .with_state(pool)
}
