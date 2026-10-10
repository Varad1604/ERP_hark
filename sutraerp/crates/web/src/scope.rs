//! Tenant/Institution scope extraction (ADR-L5-0004).
//!
//! **Batch 0 / dev mode:** scope is read from `X-Sutra-Tenant-Id` / `X-Sutra-Institution-Id`.
//! **Batch 1:** the authenticated session becomes the scope authority; headers may then only
//! *narrow* the session scope (validated with `ExecutionScope::ensure_within`). Missing or
//! malformed scope always fails closed with a problem response — there is no default tenant.

use crate::problem::ApiError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sutra_kernel::scope::ExecutionScope;

pub const TENANT_HEADER: &str = "x-sutra-tenant-id";
pub const INSTITUTION_HEADER: &str = "x-sutra-institution-id";

/// Extractor yielding the validated [`ExecutionScope`] of the request.
#[derive(Debug, Clone, Copy)]
pub struct Scope(pub ExecutionScope);

impl<S: Send + Sync> FromRequestParts<S> for Scope {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let get = |name: &str| parts.headers.get(name).and_then(|v| v.to_str().ok());
        ExecutionScope::parse(get(TENANT_HEADER), get(INSTITUTION_HEADER))
            .map(Scope)
            .map_err(ApiError::from)
    }
}

/// OpenAPI documentation of the scope headers (`params(ScopeHeaders)`).
#[derive(Debug, utoipa::IntoParams)]
#[into_params(parameter_in = Header)]
#[allow(dead_code)]
pub struct ScopeHeaders {
    /// Tenant scope (UUID). Mandatory; requests without it fail closed.
    #[param(rename = "X-Sutra-Tenant-Id", value_type = String, format = Uuid)]
    tenant: String,
    /// Institution scope (UUID). Optional; must belong to the tenant.
    #[param(rename = "X-Sutra-Institution-Id", value_type = Option<String>, format = Uuid)]
    institution: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    async fn extract(req: Request<()>) -> Result<Scope, ApiError> {
        let (mut parts, _) = req.into_parts();
        Scope::from_request_parts(&mut parts, &()).await
    }

    #[tokio::test]
    async fn fails_closed_without_tenant() {
        let r = extract(Request::builder().body(()).unwrap_or_default()).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn extracts_scope() {
        let r = extract(
            Request::builder()
                .header(TENANT_HEADER, "0192f5a0-7c3e-7000-8000-000000000001")
                .body(())
                .unwrap_or_default(),
        )
        .await;
        assert!(r.is_ok_and(|s| s.0.institution_id.is_none()));
    }
}
