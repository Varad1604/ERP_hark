use sqlx::PgPool;
use sutra_kernel::health::ReadinessCheck;

pub struct PgReadiness {
    pool: PgPool,
}

impl PgReadiness {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ReadinessCheck for PgReadiness {
    fn name(&self) -> &'static str {
        "postgres"
    }

    async fn check(&self) -> Result<(), String> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|_| "unreachable".to_owned())
    }
}
