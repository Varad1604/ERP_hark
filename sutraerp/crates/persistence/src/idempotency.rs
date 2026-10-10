use crate::map_db_error;
use sqlx::{PgPool, Row};
use sutra_kernel::error::KernelError;
use sutra_kernel::idempotency::{BeginOutcome, IdempotencyStore, StoredResponse};

#[derive(Clone, Debug)]
pub struct PgIdempotencyStore {
    pool: PgPool,
}

impl PgIdempotencyStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl IdempotencyStore for PgIdempotencyStore {
    async fn begin(
        &self,
        scope_key: &str,
        key: &str,
        fingerprint: &str,
    ) -> Result<BeginOutcome, KernelError> {
        // Expired records no longer protect the key.
        sqlx::query("DELETE FROM platform.idempotency_key WHERE scope_key = $1 AND idempotency_key = $2 AND expires_at < now()")
            .bind(scope_key)
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(map_db_error)?;
        let inserted = sqlx::query(
            "INSERT INTO platform.idempotency_key (scope_key, idempotency_key, fingerprint, state) \
             VALUES ($1, $2, $3, 'in_flight') ON CONFLICT DO NOTHING",
        )
        .bind(scope_key)
        .bind(key)
        .bind(fingerprint)
        .execute(&self.pool)
        .await
        .map_err(map_db_error)?
        .rows_affected();
        if inserted == 1 {
            return Ok(BeginOutcome::Started);
        }
        let row = sqlx::query(
            "SELECT fingerprint, state, response_status, response_type, response_body \
             FROM platform.idempotency_key WHERE scope_key = $1 AND idempotency_key = $2",
        )
        .bind(scope_key)
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_db_error)?;
        let Some(row) = row else {
            // Raced with an expiry sweep; treat as in flight so the client retries.
            return Ok(BeginOutcome::InFlight);
        };
        let stored_fp: String = row.try_get("fingerprint").map_err(map_db_error)?;
        if stored_fp != fingerprint {
            return Ok(BeginOutcome::Mismatch);
        }
        let state: String = row.try_get("state").map_err(map_db_error)?;
        if state != "completed" {
            return Ok(BeginOutcome::InFlight);
        }
        let status: Option<i16> = row.try_get("response_status").map_err(map_db_error)?;
        Ok(BeginOutcome::Replay(StoredResponse {
            status: status.and_then(|s| u16::try_from(s).ok()).unwrap_or(200),
            content_type: row.try_get("response_type").map_err(map_db_error)?,
            body: row
                .try_get::<Option<Vec<u8>>, _>("response_body")
                .map_err(map_db_error)?
                .unwrap_or_default(),
        }))
    }

    async fn complete(
        &self,
        scope_key: &str,
        key: &str,
        response: StoredResponse,
    ) -> Result<(), KernelError> {
        sqlx::query(
            "UPDATE platform.idempotency_key SET state = 'completed', response_status = $3, response_type = $4, \
             response_body = $5, completed_at = now() WHERE scope_key = $1 AND idempotency_key = $2 AND state = 'in_flight'",
        )
        .bind(scope_key)
        .bind(key)
        .bind(i16::try_from(response.status).unwrap_or(500))
        .bind(response.content_type)
        .bind(response.body)
        .execute(&self.pool)
        .await
        .map_err(map_db_error)?;
        Ok(())
    }

    async fn abandon(&self, scope_key: &str, key: &str) -> Result<(), KernelError> {
        sqlx::query("DELETE FROM platform.idempotency_key WHERE scope_key = $1 AND idempotency_key = $2 AND state = 'in_flight'")
            .bind(scope_key)
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(map_db_error)?;
        Ok(())
    }
}
