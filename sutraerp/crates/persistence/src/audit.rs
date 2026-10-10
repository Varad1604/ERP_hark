use crate::map_db_error;
use sqlx::{PgExecutor, PgPool};
use sutra_kernel::audit::{AuditEntry, AuditSink};
use sutra_kernel::error::KernelError;

/// Inserts an audit entry using any executor (pool or an open transaction, so audit evidence can
/// commit atomically with the state change it describes).
pub async fn insert<'e, E: PgExecutor<'e>>(executor: E, e: &AuditEntry) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO platform.audit_log (id, occurred_at, tenant_id, institution_id, actor_kind, actor_id, action, \
         target_type, target_id, outcome, correlation_id, details) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
    )
    .bind(e.id)
    .bind(e.occurred_at)
    .bind(e.scope.tenant_id.as_uuid())
    .bind(e.scope.institution_id.map(|i| i.as_uuid()))
    .bind(e.actor_kind.as_str())
    .bind(&e.actor_id)
    .bind(&e.action)
    .bind(&e.target_type)
    .bind(&e.target_id)
    .bind(e.outcome.as_str())
    .bind(&e.correlation_id)
    .bind(&e.details)
    .execute(executor)
    .await?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct PgAuditSink {
    pool: PgPool,
}

impl PgAuditSink {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuditSink for PgAuditSink {
    async fn record(&self, entry: AuditEntry) -> Result<(), KernelError> {
        insert(&self.pool, &entry).await.map_err(map_db_error)
    }
}
