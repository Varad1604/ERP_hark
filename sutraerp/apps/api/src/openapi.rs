//! OpenAPI document root. Paths/schemas are collected from the module routers (utoipa-axum).

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "SutraERP API",
        version = "1.0.0",
        description = "SutraERP education ERP — synchronous REST/JSON API (major version v1).\n\n\
            Conventions (ADR-L5-0034): camelCase JSON members; URL major versioning (`/api/v1`); \
            RFC 9457 `application/problem+json` errors with `errorCode`, `correlationId` and \
            `validationErrors` extensions; retry-sensitive commands accept `Idempotency-Key`; \
            tenant scope via `X-Sutra-Tenant-Id` (dev header mode, ADR-L5-0004)."
    ),
    tags(
        (name = "health", description = "Liveness and readiness probes"),
        (name = "auth", description = "D01 authentication boundary (Keycloak-delegated, server-side session)"),
        (name = "platform", description = "A Platform: catalog and module registry"),
        (name = "institution", description = "B Institution (reference aggregate)")
    ),
    components(schemas(sutra_web::Problem))
)]
pub struct ApiDoc;

/// Pretty JSON of the full generated specification (used by `sutra-api openapi` and CI drift check).
pub fn spec_json() -> Result<String, serde_json::Error> {
    let (_, api) = crate::app::build(crate::app::AppContext::for_spec()).split_for_parts();
    let mut s = serde_json::to_string_pretty(&api)?;
    s.push('\n');
    Ok(s)
}
