//! Keycloak 26.x credential adapter — **interface only in Batch 0**.
//!
//! Batch 1 implements credential delegation through a supported Keycloak-side mechanism (custom
//! authenticator / brokered flow), explicitly **not** the OAuth Resource-Owner-Password /
//! Direct-Access-Grant (ADR-L5-0038 §3.3, §5). Until then every call returns `501`.

use sutra_contracts::platform::auth::{
    AuthOutcome, CredentialAuthenticator, Credentials, Principal,
};
use sutra_kernel::error::KernelError;

#[derive(Clone, Debug)]
pub struct KeycloakSettings {
    pub base_url: String,
    pub realm: String,
    pub client_id: String,
}

pub struct KeycloakAuthenticator {
    #[allow(dead_code)]
    settings: KeycloakSettings,
}

impl KeycloakAuthenticator {
    pub fn new(settings: KeycloakSettings) -> Self {
        Self { settings }
    }
}

#[async_trait::async_trait]
impl CredentialAuthenticator for KeycloakAuthenticator {
    async fn authenticate(&self, _credentials: Credentials) -> Result<AuthOutcome, KernelError> {
        Err(KernelError::NotImplemented(
            "Keycloak credential delegation lands in Batch 1",
        ))
    }

    async fn verify_mfa(&self, _challenge_id: &str, _code: &str) -> Result<Principal, KernelError> {
        Err(KernelError::NotImplemented(
            "Keycloak MFA continuation lands in Batch 1",
        ))
    }
}
