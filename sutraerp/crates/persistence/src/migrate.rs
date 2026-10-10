//! Embedded migrations (`/migrations`). Executed by `sutra-api migrate` — a separate step from
//! normal serving, per ADR-L5-0008 §3.4. Applied history is immutable; fixes move forward.

use sqlx::migrate::Migrator;
use sqlx::PgPool;

pub static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

pub async fn run(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(pool).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_are_embedded_and_ordered() {
        let versions: Vec<i64> = MIGRATOR.iter().map(|m| m.version).collect();
        assert!(versions.len() >= 5);
        let mut sorted = versions.clone();
        sorted.sort_unstable();
        assert_eq!(versions, sorted);
    }

    #[test]
    fn migrations_never_enable_extensions() {
        // ADR-L5-0036 §3.8: extensions are deny-by-default.
        for m in MIGRATOR.iter() {
            assert!(
                !m.sql.to_lowercase().contains("create extension"),
                "{}",
                m.description
            );
        }
    }
}
