//! Audit evidence primitives.
//!
//! Every protected mutation writes an [`AuditEntry`]. The PostgreSQL sink stores entries in the
//! append-only `platform.audit_log` table; tamper-evident chaining/anchoring (ADR-L5-0012) is
//! scheduled for Batch 9 and plugs in behind the same [`AuditSink`] port.

use crate::error::KernelError;
use crate::scope::ExecutionScope;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuditOutcome {
    Succeeded,
    Rejected,
    Failed,
}

impl AuditOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Rejected => "rejected",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActorKind {
    User,
    Service,
    System,
    Anonymous,
}

impl ActorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Service => "service",
            Self::System => "system",
            Self::Anonymous => "anonymous",
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEntry {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub scope: ExecutionScope,
    pub actor_kind: ActorKind,
    pub actor_id: Option<String>,
    /// Dotted action name, e.g. `institution.institution.rename`.
    pub action: String,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
    pub outcome: AuditOutcome,
    pub correlation_id: Option<String>,
    /// Structured, already-redacted details. Never place credentials or secrets here.
    pub details: serde_json::Value,
}

impl AuditEntry {
    pub fn new(scope: ExecutionScope, action: impl Into<String>, outcome: AuditOutcome) -> Self {
        Self {
            id: crate::id::new_v7(),
            occurred_at: Utc::now(),
            scope,
            actor_kind: ActorKind::System,
            actor_id: None,
            action: action.into(),
            target_type: None,
            target_id: None,
            outcome,
            correlation_id: None,
            details: serde_json::Value::Object(Default::default()),
        }
    }

    pub fn actor(mut self, kind: ActorKind, id: Option<String>) -> Self {
        self.actor_kind = kind;
        self.actor_id = id;
        self
    }

    pub fn target(mut self, kind: impl Into<String>, id: impl Into<String>) -> Self {
        self.target_type = Some(kind.into());
        self.target_id = Some(id.into());
        self
    }

    pub fn correlation(mut self, id: Option<String>) -> Self {
        self.correlation_id = id;
        self
    }

    pub fn details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }
}

#[async_trait::async_trait]
pub trait AuditSink: Send + Sync + 'static {
    async fn record(&self, entry: AuditEntry) -> Result<(), KernelError>;
}

/// Sink that emits audit entries as structured `audit` log events (used when no DB is wired).
#[derive(Debug, Default, Clone)]
pub struct TracingAuditSink;

#[async_trait::async_trait]
impl AuditSink for TracingAuditSink {
    async fn record(&self, entry: AuditEntry) -> Result<(), KernelError> {
        tracing::info!(
            target: "audit",
            audit_id = %entry.id,
            action = %entry.action,
            outcome = entry.outcome.as_str(),
            tenant = %entry.scope.tenant_id,
            actor = entry.actor_id.as_deref().unwrap_or("-"),
            "audit"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scope::TenantId;

    #[tokio::test]
    async fn builds_and_records() {
        let e = AuditEntry::new(
            ExecutionScope::tenant(TenantId::new()),
            "test.action",
            AuditOutcome::Succeeded,
        )
        .actor(ActorKind::User, Some("u1".into()))
        .target("thing", "t1")
        .details(serde_json::json!({"field": "name"}));
        assert_eq!(e.id.get_version_num(), 7);
        assert_eq!(e.target_type.as_deref(), Some("thing"));
        assert!(TracingAuditSink.record(e).await.is_ok());
    }
}
