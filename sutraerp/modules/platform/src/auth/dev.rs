//! **Development-only** credential authenticator (Batch 0).
//!
//! Recognises three seeded personas that mirror the product prototype and a single shared dev
//! password. The composition root refuses to wire it when `SUTRA_ENV=production`.

use std::collections::HashMap;
use std::sync::Mutex;
use sutra_contracts::platform::auth::{
    AuthOutcome, CredentialAuthenticator, Credentials, MfaMethod, MfaPolicy, Principal, Role,
};
use sutra_kernel::error::KernelError;
use sutra_kernel::scope::ExecutionScope;

/// Dev MFA code accepted by [`DevAuthenticator::verify_mfa`].
pub const DEV_MFA_CODE: &str = "000000";

pub struct DevPersona {
    pub login_id: &'static str,
    pub display_name: &'static str,
    pub role: Role,
}

pub const DEV_PERSONAS: [DevPersona; 3] = [
    DevPersona {
        login_id: "admin@sutra.dev",
        display_name: "Dhruv Naik",
        role: Role::Administrator,
    },
    DevPersona {
        login_id: "meera.shah@sutra.dev",
        display_name: "Dr. Meera Shah",
        role: Role::Faculty,
    },
    DevPersona {
        login_id: "aarav.desai@sutra.dev",
        display_name: "Aarav Desai",
        role: Role::Student,
    },
];

pub struct DevAuthenticator {
    password: String,
    scope: ExecutionScope,
    mfa: Box<dyn MfaPolicy>,
    challenges: Mutex<HashMap<String, Principal>>,
}

impl DevAuthenticator {
    pub fn new(password: String, scope: ExecutionScope, mfa: Box<dyn MfaPolicy>) -> Self {
        Self {
            password,
            scope,
            mfa,
            challenges: Mutex::new(HashMap::new()),
        }
    }

    fn principal(&self, p: &DevPersona) -> Principal {
        Principal {
            subject: format!("dev|{}", p.login_id),
            display_name: p.display_name.into(),
            login_id: p.login_id.into(),
            roles: vec![p.role],
            scope: self.scope,
        }
    }
}

/// Constant-time-ish comparison so the dev stub does not model a timing oracle.
fn eq_ct(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut diff = a.len() ^ b.len();
    for i in 0..a.len().max(b.len()) {
        diff |= usize::from(a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0));
    }
    diff == 0
}

#[async_trait::async_trait]
impl CredentialAuthenticator for DevAuthenticator {
    async fn authenticate(&self, c: Credentials) -> Result<AuthOutcome, KernelError> {
        let login = c.login_id.trim().to_lowercase();
        let persona = DEV_PERSONAS.iter().find(|p| p.login_id == login);
        let password_ok = eq_ct(&c.password, &self.password);
        // Same generic failure for unknown user and wrong password (no enumeration).
        let Some(persona) = persona.filter(|_| password_ok) else {
            return Err(KernelError::Unauthenticated);
        };
        let principal = self.principal(persona);
        if self.mfa.requires_mfa(&principal) {
            let challenge_id = sutra_kernel::id::new_v7().to_string();
            self.challenges
                .lock()
                .map_err(|_| KernelError::internal_msg("challenge store poisoned"))?
                .insert(challenge_id.clone(), principal.clone());
            return Ok(AuthOutcome::MfaRequired {
                challenge_id,
                methods: vec![MfaMethod::Totp],
                principal,
            });
        }
        Ok(AuthOutcome::Authenticated(principal))
    }

    async fn verify_mfa(&self, challenge_id: &str, code: &str) -> Result<Principal, KernelError> {
        let mut map = self
            .challenges
            .lock()
            .map_err(|_| KernelError::internal_msg("challenge store poisoned"))?;
        // A challenge is single-use regardless of outcome.
        let principal = map
            .remove(challenge_id)
            .ok_or(KernelError::Unauthenticated)?;
        if eq_ct(code, DEV_MFA_CODE) {
            Ok(principal)
        } else {
            Err(KernelError::Unauthenticated)
        }
    }
}

/// MFA policy for dev: privileged roles need MFA only when `enforce` is on.
pub struct PrivilegedRoleMfaPolicy {
    pub enforce: bool,
}

impl MfaPolicy for PrivilegedRoleMfaPolicy {
    fn requires_mfa(&self, p: &Principal) -> bool {
        self.enforce && p.roles.iter().any(Role::is_privileged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sutra_kernel::scope::TenantId;

    fn auth(enforce: bool) -> DevAuthenticator {
        DevAuthenticator::new(
            "sutra".into(),
            ExecutionScope::tenant(TenantId::new()),
            Box::new(PrivilegedRoleMfaPolicy { enforce }),
        )
    }

    fn creds(l: &str, p: &str) -> Credentials {
        Credentials {
            login_id: l.into(),
            password: p.into(),
        }
    }

    #[tokio::test]
    async fn accepts_known_persona() {
        let r = auth(false)
            .authenticate(creds("Meera.Shah@sutra.dev", "sutra"))
            .await;
        assert!(matches!(r, Ok(AuthOutcome::Authenticated(p)) if p.roles == vec![Role::Faculty]));
    }

    #[tokio::test]
    async fn generic_failure_for_unknown_or_wrong_password() {
        let a = auth(false);
        assert!(matches!(
            a.authenticate(creds("nobody@sutra.dev", "sutra")).await,
            Err(KernelError::Unauthenticated)
        ));
        assert!(matches!(
            a.authenticate(creds("admin@sutra.dev", "wrong")).await,
            Err(KernelError::Unauthenticated)
        ));
    }

    #[tokio::test]
    async fn privileged_mfa_challenge_flow() {
        let a = auth(true);
        let Ok(AuthOutcome::MfaRequired { challenge_id, .. }) =
            a.authenticate(creds("admin@sutra.dev", "sutra")).await
        else {
            panic!("expected MFA challenge");
        };
        assert!(a.verify_mfa(&challenge_id, "123456").await.is_err());
        // single use: a failed attempt burns the challenge
        assert!(a.verify_mfa(&challenge_id, DEV_MFA_CODE).await.is_err());

        let Ok(AuthOutcome::MfaRequired { challenge_id, .. }) =
            a.authenticate(creds("admin@sutra.dev", "sutra")).await
        else {
            panic!("expected MFA challenge");
        };
        assert!(a.verify_mfa(&challenge_id, DEV_MFA_CODE).await.is_ok());
        // students are never challenged
        assert!(matches!(
            a.authenticate(creds("aarav.desai@sutra.dev", "sutra"))
                .await,
            Ok(AuthOutcome::Authenticated(_))
        ));
    }
}
