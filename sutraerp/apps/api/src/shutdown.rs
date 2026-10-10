//! Graceful shutdown (ADR-L5-0044): on SIGTERM/SIGINT readiness flips to `draining`, in-flight
//! requests finish, and the process exits after at most the configured grace period.

use std::time::Duration;
use sutra_web::health::Readiness;
use tokio::sync::watch;

/// Resolves when SIGINT or SIGTERM is received.
pub async fn signal() {
    let ctrl_c = async {
        if tokio::signal::ctrl_c().await.is_err() {
            std::future::pending::<()>().await;
        }
    };
    #[cfg(unix)]
    let term = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = term => {},
    }
}

/// Coordinates draining: returns the future handed to `axum::serve(..).with_graceful_shutdown`
/// and a receiver that fires when shutdown began (used to enforce the grace deadline).
pub fn coordinator(
    readiness: Readiness,
) -> (
    impl std::future::Future<Output = ()> + Send + 'static,
    watch::Receiver<bool>,
) {
    let (tx, rx) = watch::channel(false);
    let fut = async move {
        signal().await;
        tracing::info!("shutdown signal received; draining");
        readiness.start_draining();
        let _ = tx.send(true);
    };
    (fut, rx)
}

/// Resolves `grace` after shutdown started.
pub async fn deadline(mut started: watch::Receiver<bool>, grace: Duration) {
    while !*started.borrow() {
        if started.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
    tokio::time::sleep(grace).await;
}
