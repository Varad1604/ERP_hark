//! Durable Publication Intent writer (ADR-L5-0007).
//!
//! Call [`enqueue`] with the *same transaction* that mutates owner state:
//!
//! ```ignore
//! let mut tx = pool.begin().await?;
//! sqlx::query("UPDATE institution.institution SET ...").execute(&mut *tx).await?;
//! outbox::enqueue(&mut *tx, &envelope).await?;
//! tx.commit().await?;
//! ```

use sqlx::PgExecutor;
use sutra_kernel::events::EventEnvelope;

pub async fn enqueue<'e, E: PgExecutor<'e>>(
    executor: E,
    env: &EventEnvelope,
) -> Result<(), sqlx::Error> {
    let envelope = serde_json::to_value(env).map_err(|e| sqlx::Error::Encode(Box::new(e)))?;
    sqlx::query(
        "INSERT INTO platform.outbox (id, tenant_id, institution_id, source, event_type, dataschema, subject, envelope) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(env.id)
    .bind(env.tenantid)
    .bind(env.institutionid)
    .bind(&env.source)
    .bind(&env.event_type)
    .bind(&env.dataschema)
    .bind(&env.subject)
    .bind(envelope)
    .execute(executor)
    .await?;
    Ok(())
}
