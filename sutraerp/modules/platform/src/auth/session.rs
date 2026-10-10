//! In-memory server-side session store (Batch 0). Batch 1 moves sessions to a durable D01-owned
//! store bound to the Keycloak session. Valkey may cache but is never session authority
//! (ADR-L5-0037 §2.8).

use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Mutex;
use sutra_contracts::platform::auth::{Principal, Session, SessionStore};
use sutra_kernel::error::KernelError;

/// 256-bit opaque session id, hex encoded.
pub fn new_session_id() -> Result<String, KernelError> {
    let mut buf = [0u8; 32];
    getrandom::fill(&mut buf)
        .map_err(|e| KernelError::internal_msg(format!("entropy unavailable: {e}")))?;
    Ok(buf.iter().map(|b| format!("{b:02x}")).collect())
}

pub struct InMemorySessionStore {
    ttl: Duration,
    sessions: Mutex<HashMap<String, Session>>,
}

impl InMemorySessionStore {
    pub fn new(ttl_secs: i64) -> Self {
        Self {
            ttl: Duration::seconds(ttl_secs),
            sessions: Mutex::new(HashMap::new()),
        }
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, HashMap<String, Session>>, KernelError> {
        self.sessions
            .lock()
            .map_err(|_| KernelError::internal_msg("session store poisoned"))
    }
}

#[async_trait::async_trait]
impl SessionStore for InMemorySessionStore {
    async fn create(&self, principal: Principal) -> Result<Session, KernelError> {
        let now = Utc::now();
        let s = Session {
            id: new_session_id()?,
            principal,
            created_at: now,
            expires_at: now + self.ttl,
        };
        let mut map = self.lock()?;
        map.retain(|_, v| v.expires_at > now);
        map.insert(s.id.clone(), s.clone());
        Ok(s)
    }

    async fn get(&self, id: &str) -> Result<Option<Session>, KernelError> {
        let now = Utc::now();
        Ok(self.lock()?.get(id).filter(|s| s.expires_at > now).cloned())
    }

    async fn revoke(&self, id: &str) -> Result<(), KernelError> {
        self.lock()?.remove(id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sutra_contracts::platform::auth::Role;
    use sutra_kernel::scope::{ExecutionScope, TenantId};

    fn p() -> Principal {
        Principal {
            subject: "s".into(),
            display_name: "d".into(),
            login_id: "l".into(),
            roles: vec![Role::Student],
            scope: ExecutionScope::tenant(TenantId::new()),
        }
    }

    #[test]
    fn ids_are_256_bit_hex_and_unique() {
        let a = new_session_id().unwrap();
        let b = new_session_id().unwrap();
        assert_eq!(a.len(), 64);
        assert_ne!(a, b);
    }

    #[tokio::test]
    async fn create_get_revoke_expire() {
        let s = InMemorySessionStore::new(60);
        let sess = s.create(p()).await.unwrap();
        assert!(s.get(&sess.id).await.unwrap().is_some());
        s.revoke(&sess.id).await.unwrap();
        assert!(s.get(&sess.id).await.unwrap().is_none());

        let expired = InMemorySessionStore::new(-1);
        let sess = expired.create(p()).await.unwrap();
        assert!(expired.get(&sess.id).await.unwrap().is_none());
    }
}
