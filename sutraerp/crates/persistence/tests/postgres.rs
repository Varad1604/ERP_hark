//! Live-database integration tests. Run with:
//!
//! ```sh
//! DATABASE_URL=postgres://sutra:sutra@localhost:5432/sutra cargo test -p sutra-persistence --features db-tests
//! ```
#![cfg(feature = "db-tests")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use sutra_kernel::audit::{AuditEntry, AuditOutcome};
use sutra_kernel::events::{EventEnvelope, NewEvent};
use sutra_kernel::idempotency::{BeginOutcome, IdempotencyStore, StoredResponse};
use sutra_kernel::scope::{ExecutionScope, TenantId};
use sutra_persistence::{audit, idempotency::PgIdempotencyStore, migrate, outbox, pool};

async fn db() -> sqlx::PgPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for db-tests");
    let p = pool::connect_lazy(&pool::PoolSettings {
        url,
        max_connections: 4,
        min_connections: 0,
        acquire_timeout: std::time::Duration::from_secs(5),
        application_name: "sutra-db-tests".into(),
    })
    .expect("pool");
    migrate::run(&p).await.expect("migrations");
    p
}

#[tokio::test]
async fn audit_log_is_append_only() {
    let p = db().await;
    let e = AuditEntry::new(
        ExecutionScope::tenant(TenantId::new()),
        "test.append",
        AuditOutcome::Succeeded,
    );
    audit::insert(&p, &e).await.expect("insert");
    let upd = sqlx::query("UPDATE platform.audit_log SET action = 'x' WHERE id = $1")
        .bind(e.id)
        .execute(&p)
        .await;
    assert!(upd.is_err(), "updates must be rejected");
}

#[tokio::test]
async fn outbox_commits_with_transaction() {
    let p = db().await;
    let scope = ExecutionScope::tenant(TenantId::new());
    let env = EventEnvelope::new(
        &scope,
        NewEvent {
            source: "/t",
            event_type: "t.happened",
            dataschema: "urn:t:1",
            subject: None,
            data: serde_json::json!({}),
        },
    );
    let mut tx = p.begin().await.expect("tx");
    outbox::enqueue(&mut *tx, &env).await.expect("enqueue");
    tx.rollback().await.expect("rollback");
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM platform.outbox WHERE id = $1")
        .bind(env.id)
        .fetch_one(&p)
        .await
        .expect("count");
    assert_eq!(n, 0, "rolled back intent must not exist");
}

#[tokio::test]
async fn idempotency_store_roundtrip() {
    let p = db().await;
    let s = PgIdempotencyStore::new(p);
    let scope = TenantId::new().to_string();
    assert_eq!(
        s.begin(&scope, "k", "f").await.expect("begin"),
        BeginOutcome::Started
    );
    assert_eq!(
        s.begin(&scope, "k", "f").await.expect("begin"),
        BeginOutcome::InFlight
    );
    s.complete(
        &scope,
        "k",
        StoredResponse {
            status: 201,
            content_type: None,
            body: b"{}".to_vec(),
        },
    )
    .await
    .expect("complete");
    assert!(matches!(
        s.begin(&scope, "k", "f").await.expect("begin"),
        BeginOutcome::Replay(_)
    ));
    assert_eq!(
        s.begin(&scope, "k", "other").await.expect("begin"),
        BeginOutcome::Mismatch
    );
}
