//! Idempotency store port (ADR-L5-0034 §3.6).
//!
//! The HTTP middleware fingerprints a mutating request carrying `Idempotency-Key` and asks the
//! store to *begin* it. Keys are owner-scoped: the scope key combines Tenant + principal/route so
//! one tenant can never observe another's stored response. Retention is a profile concern
//! (no universal duration is invented here).

use crate::error::KernelError;
use std::collections::HashMap;
use std::sync::Mutex;

/// A completed response captured for replay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredResponse {
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BeginOutcome {
    /// First time this key is seen: the caller must execute and then `complete` or `abandon`.
    Started,
    /// Same key + same fingerprint already completed: replay the stored response.
    Replay(StoredResponse),
    /// Same key + same fingerprint is still executing.
    InFlight,
    /// Same key with a materially different request.
    Mismatch,
}

#[async_trait::async_trait]
pub trait IdempotencyStore: Send + Sync + 'static {
    async fn begin(
        &self,
        scope_key: &str,
        key: &str,
        fingerprint: &str,
    ) -> Result<BeginOutcome, KernelError>;
    async fn complete(
        &self,
        scope_key: &str,
        key: &str,
        response: StoredResponse,
    ) -> Result<(), KernelError>;
    /// Releases a key whose execution failed with a retryable (5xx) outcome.
    async fn abandon(&self, scope_key: &str, key: &str) -> Result<(), KernelError>;
}

type EntryMap = HashMap<(String, String), Entry>;

#[derive(Debug, Clone)]
enum Entry {
    InFlight {
        fingerprint: String,
    },
    Done {
        fingerprint: String,
        response: StoredResponse,
    },
}

/// Process-local store for development and tests. Production uses the PostgreSQL store.
#[derive(Debug, Default)]
pub struct InMemoryIdempotencyStore {
    entries: Mutex<EntryMap>,
}

impl InMemoryIdempotencyStore {
    fn lock(&self) -> Result<std::sync::MutexGuard<'_, EntryMap>, KernelError> {
        self.entries
            .lock()
            .map_err(|_| KernelError::internal_msg("idempotency store poisoned"))
    }
}

#[async_trait::async_trait]
impl IdempotencyStore for InMemoryIdempotencyStore {
    async fn begin(
        &self,
        scope_key: &str,
        key: &str,
        fingerprint: &str,
    ) -> Result<BeginOutcome, KernelError> {
        let mut map = self.lock()?;
        let k = (scope_key.to_owned(), key.to_owned());
        Ok(match map.get(&k) {
            None => {
                map.insert(
                    k,
                    Entry::InFlight {
                        fingerprint: fingerprint.to_owned(),
                    },
                );
                BeginOutcome::Started
            }
            Some(Entry::InFlight { fingerprint: f }) if f == fingerprint => BeginOutcome::InFlight,
            Some(Entry::Done {
                fingerprint: f,
                response,
            }) if f == fingerprint => BeginOutcome::Replay(response.clone()),
            Some(_) => BeginOutcome::Mismatch,
        })
    }

    async fn complete(
        &self,
        scope_key: &str,
        key: &str,
        response: StoredResponse,
    ) -> Result<(), KernelError> {
        let mut map = self.lock()?;
        let k = (scope_key.to_owned(), key.to_owned());
        if let Some(Entry::InFlight { fingerprint }) = map.get(&k).cloned() {
            map.insert(
                k,
                Entry::Done {
                    fingerprint,
                    response,
                },
            );
        }
        Ok(())
    }

    async fn abandon(&self, scope_key: &str, key: &str) -> Result<(), KernelError> {
        let mut map = self.lock()?;
        let k = (scope_key.to_owned(), key.to_owned());
        if matches!(map.get(&k), Some(Entry::InFlight { .. })) {
            map.remove(&k);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resp() -> StoredResponse {
        StoredResponse {
            status: 201,
            content_type: Some("application/json".into()),
            body: b"{}".to_vec(),
        }
    }

    #[tokio::test]
    async fn lifecycle() {
        let s = InMemoryIdempotencyStore::default();
        assert_eq!(
            s.begin("t1", "k", "f1").await.ok(),
            Some(BeginOutcome::Started)
        );
        assert_eq!(
            s.begin("t1", "k", "f1").await.ok(),
            Some(BeginOutcome::InFlight)
        );
        assert_eq!(
            s.begin("t1", "k", "f2").await.ok(),
            Some(BeginOutcome::Mismatch)
        );
        assert!(s.complete("t1", "k", resp()).await.is_ok());
        assert_eq!(
            s.begin("t1", "k", "f1").await.ok(),
            Some(BeginOutcome::Replay(resp()))
        );
        assert_eq!(
            s.begin("t1", "k", "f2").await.ok(),
            Some(BeginOutcome::Mismatch)
        );
        // Owner-scoped: a different tenant never collides.
        assert_eq!(
            s.begin("t2", "k", "f2").await.ok(),
            Some(BeginOutcome::Started)
        );
    }

    #[tokio::test]
    async fn abandon_releases_in_flight_key() {
        let s = InMemoryIdempotencyStore::default();
        assert_eq!(
            s.begin("t", "k", "f").await.ok(),
            Some(BeginOutcome::Started)
        );
        assert!(s.abandon("t", "k").await.is_ok());
        assert_eq!(
            s.begin("t", "k", "f").await.ok(),
            Some(BeginOutcome::Started)
        );
    }
}
