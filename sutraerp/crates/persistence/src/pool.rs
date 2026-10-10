use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct PoolSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub application_name: String,
}

/// Builds a bounded pool that connects lazily, so the API can start (and report not-ready)
/// while PostgreSQL is still coming up.
pub fn connect_lazy(s: &PoolSettings) -> Result<PgPool, sqlx::Error> {
    let opts = PgConnectOptions::from_str(&s.url)?
        .application_name(&s.application_name)
        .options([("timezone", "UTC")]);
    Ok(PgPoolOptions::new()
        .max_connections(s.max_connections)
        .min_connections(s.min_connections)
        .acquire_timeout(s.acquire_timeout)
        .connect_lazy_with(opts))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn lazy_pool_does_not_need_a_server() {
        let pool = connect_lazy(&PoolSettings {
            url: "postgres://sutra:sutra@127.0.0.1:1/sutra".into(),
            max_connections: 2,
            min_connections: 0,
            acquire_timeout: Duration::from_millis(200),
            application_name: "test".into(),
        });
        assert!(pool.is_ok());
    }

    #[test]
    fn rejects_bad_url() {
        let r = connect_lazy(&PoolSettings {
            url: "not a url".into(),
            max_connections: 1,
            min_connections: 0,
            acquire_timeout: Duration::from_secs(1),
            application_name: "t".into(),
        });
        assert!(r.is_err());
    }
}
