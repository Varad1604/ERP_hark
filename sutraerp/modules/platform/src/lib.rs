//! # Domain A — Platform module
//!
//! Batch 0 scope:
//! * **Auth (D01 boundary)** — `POST /api/v1/auth/login`, `POST /api/v1/auth/mfa/verify`,
//!   `GET /api/v1/auth/session`, `POST /api/v1/auth/logout` with an opaque server-side session
//!   cookie. Credential verification goes through `CredentialAuthenticator`: a **dev stub** now,
//!   the Keycloak 26 adapter in Batch 1 (ADR-L5-0024 / ADR-L5-0038).
//! * **Workflow port** — `NoopWorkflowAdapter` behind `WorkflowPort` (Flowable adapter later,
//!   ADR-L5-0023).
//! * **Catalog** — read-only workspace/workflow navigation catalog (99 workspaces / 595 workflows).
//! * **Modules** — enumerates the modular-monolith module descriptors.

pub mod auth;
pub mod catalog;
pub mod workflow;

use std::sync::Arc;
use sutra_contracts::platform::auth::{CredentialAuthenticator, MfaPolicy, SessionStore};
use sutra_kernel::audit::AuditSink;
use sutra_kernel::module::{ModuleDescriptor, ModuleStatus};
use utoipa_axum::router::OpenApiRouter;

pub const MODULE: ModuleDescriptor = ModuleDescriptor {
    domain: "A",
    key: "platform",
    name: "Platform",
    schema: "platform",
    batch: 1,
    status: ModuleStatus::Active,
};

/// Dependencies injected by the composition root.
#[derive(Clone)]
pub struct PlatformDeps {
    pub authenticator: Arc<dyn CredentialAuthenticator>,
    pub sessions: Arc<dyn SessionStore>,
    pub mfa: Arc<dyn MfaPolicy>,
    pub audit: Arc<dyn AuditSink>,
    pub cookie: auth::CookieSettings,
    pub modules: Vec<ModuleDescriptor>,
}

pub fn router(deps: PlatformDeps) -> OpenApiRouter {
    let modules = deps.modules.clone();
    OpenApiRouter::new()
        .merge(auth::http::router(deps))
        .merge(catalog::router(modules))
}
