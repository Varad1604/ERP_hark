//! Authentication boundary contracts (D01 — ADR-L5-0024, ADR-L5-0038).
//!
//! Keycloak 26.x is the credential authority. SutraERP exposes its own
//! `POST /api/v1/auth/login`, delegates credential verification to a [`CredentialAuthenticator`]
//! (Keycloak adapter in Batch 1; dev stub in Batch 0) and establishes an opaque server-side session
//! carried in a `Secure; HttpOnly; SameSite` cookie. No password hash is ever stored by SutraERP and
//! the OAuth Resource-Owner-Password grant is **not** used.

use chrono::{DateTime, Utc};
use std::fmt;
use sutra_kernel::error::KernelError;
use sutra_kernel::scope::ExecutionScope;

/// Application role as presented by the web client. Authorization truth stays with D02 policy;
/// this is a coarse persona used for navigation and default scoping.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Administrator,
    Faculty,
    Student,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Administrator => "administrator",
            Role::Faculty => "faculty",
            Role::Student => "student",
        }
    }

    /// Privileged roles require MFA (ADR-L5-0038 §3.8).
    pub fn is_privileged(&self) -> bool {
        matches!(self, Role::Administrator)
    }
}

/// Transient credentials. `Debug` is redacted so the password can never reach a log line.
#[derive(Clone, serde::Deserialize)]
pub struct Credentials {
    pub login_id: String,
    pub password: String,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("login_id", &self.login_id)
            .field("password", &"<redacted>")
            .finish()
    }
}

/// The authenticated principal (technical identity bound to a SutraERP canonical identity).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Principal {
    /// Stable subject reference (Keycloak `sub` in production).
    pub subject: String,
    pub display_name: String,
    pub login_id: String,
    pub roles: Vec<Role>,
    pub scope: ExecutionScope,
}

/// MFA method family (WebAuthn preferred for privileged users, TOTP fallback).
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum MfaMethod {
    Webauthn,
    Totp,
}

/// Result of a primary credential check.
#[derive(Clone, Debug)]
pub enum AuthOutcome {
    Authenticated(Principal),
    /// Primary factor accepted; a second factor is required before a session is issued.
    MfaRequired {
        challenge_id: String,
        methods: Vec<MfaMethod>,
        principal: Principal,
    },
}

/// Credential verification port. Implementations: `DevAuthenticator` (Batch 0),
/// `KeycloakAuthenticator` (Batch 1).
#[async_trait::async_trait]
pub trait CredentialAuthenticator: Send + Sync + 'static {
    /// Must return [`KernelError::Unauthenticated`] for any failure (no account enumeration).
    async fn authenticate(&self, credentials: Credentials) -> Result<AuthOutcome, KernelError>;
    /// Completes an MFA challenge.
    async fn verify_mfa(&self, challenge_id: &str, code: &str) -> Result<Principal, KernelError>;
}

/// MFA policy hook: decides whether a principal must complete MFA.
pub trait MfaPolicy: Send + Sync + 'static {
    fn requires_mfa(&self, principal: &Principal) -> bool;
}

/// Server-side session record. The cookie only carries the opaque id.
#[derive(Clone, Debug)]
pub struct Session {
    pub id: String,
    pub principal: Principal,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Session storage port (D01-owned). In-memory in Batch 0; PostgreSQL/Valkey-backed in Batch 1.
#[async_trait::async_trait]
pub trait SessionStore: Send + Sync + 'static {
    async fn create(&self, principal: Principal) -> Result<Session, KernelError>;
    async fn get(&self, id: &str) -> Result<Option<Session>, KernelError>;
    async fn revoke(&self, id: &str) -> Result<(), KernelError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_debug_is_redacted() {
        let c = Credentials {
            login_id: "a@b".into(),
            password: "s3cret".into(),
        };
        let s = format!("{c:?}");
        assert!(!s.contains("s3cret"));
        assert!(s.contains("redacted"));
    }

    #[test]
    fn administrator_is_privileged() {
        assert!(Role::Administrator.is_privileged());
        assert!(!Role::Student.is_privileged());
    }
}
