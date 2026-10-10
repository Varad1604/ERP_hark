//! Liveness / readiness endpoints (ADR-L5-0044).
//!
//! * `GET /health/live` – process is up; never touches dependencies.
//! * `GET /health/ready` – all registered [`ReadinessCheck`]s pass and the process is not
//!   draining. During graceful shutdown readiness flips to 503 first so load balancers stop
//!   routing before connections are drained.

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use sutra_kernel::health::{CheckResult, ReadinessCheck};
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Clone, Default)]
pub struct Readiness {
    checks: Vec<Arc<dyn ReadinessCheck>>,
    draining: Arc<AtomicBool>,
}

impl Readiness {
    pub fn new(checks: Vec<Arc<dyn ReadinessCheck>>) -> Self {
        Self {
            checks,
            draining: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Marks the process as draining (called on shutdown signal).
    pub fn start_draining(&self) {
        self.draining.store(true, Ordering::SeqCst);
    }

    pub fn is_draining(&self) -> bool {
        self.draining.load(Ordering::SeqCst)
    }
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthStatus {
    /// `ok`, `degraded` or `draining`.
    pub status: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub checks: Vec<CheckResult>,
}

/// Liveness probe.
#[utoipa::path(get, path = "/health/live", tag = "health", responses((status = 200, body = HealthStatus)))]
pub async fn live() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "ok".into(),
        checks: vec![],
    })
}

/// Readiness probe.
#[utoipa::path(
    get,
    path = "/health/ready",
    tag = "health",
    responses((status = 200, body = HealthStatus), (status = 503, body = HealthStatus))
)]
pub async fn ready(State(r): State<Readiness>) -> (StatusCode, Json<HealthStatus>) {
    if r.is_draining() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthStatus {
                status: "draining".into(),
                checks: vec![],
            }),
        );
    }
    let mut results = Vec::with_capacity(r.checks.len());
    for c in &r.checks {
        let outcome = tokio::time::timeout(Duration::from_secs(2), c.check()).await;
        let (ok, detail) = match outcome {
            Ok(Ok(())) => (true, None),
            Ok(Err(e)) => (false, Some(e)),
            Err(_) => (false, Some("timed out".to_owned())),
        };
        results.push(CheckResult {
            name: c.name().to_owned(),
            ok,
            detail,
        });
    }
    let all_ok = results.iter().all(|c| c.ok);
    let status = if all_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(HealthStatus {
            status: if all_ok { "ok" } else { "degraded" }.into(),
            checks: results,
        }),
    )
}

pub fn router(readiness: Readiness) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(live))
        .routes(routes!(ready))
        .with_state(readiness)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    struct Fails;
    #[async_trait::async_trait]
    impl ReadinessCheck for Fails {
        fn name(&self) -> &'static str {
            "postgres"
        }
        async fn check(&self) -> Result<(), String> {
            Err("connection refused".into())
        }
    }

    #[tokio::test]
    async fn live_is_always_ok() {
        let app: axum::Router = router(Readiness::new(vec![Arc::new(Fails)])).into();
        let res = app
            .oneshot(Request::get("/health/live").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn ready_reflects_checks_and_draining() {
        let ok = Readiness::new(vec![]);
        let app: axum::Router = router(ok.clone()).into();
        let res = app
            .clone()
            .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        ok.start_draining();
        let res = app
            .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);

        let failing: axum::Router = router(Readiness::new(vec![Arc::new(Fails)])).into();
        let res = failing
            .oneshot(Request::get("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
