//! Structured logging + tracing hooks (ADR-L5-0029).
//!
//! * JSON logs in production, pretty logs in development (`SUTRA_LOG_FORMAT`).
//! * Every HTTP request runs in an `http.request` span carrying `request_id`, method, matched
//!   route and the inbound W3C `traceparent`, so logs correlate with distributed traces.
//! * **OpenTelemetry hook:** [`init`] builds a `tracing_subscriber::Registry`; the OTLP exporter
//!   layer (`tracing-opentelemetry` + `opentelemetry-otlp`) is attached here in Batch 9 when
//!   `OTEL_EXPORTER_OTLP_ENDPOINT` is set. It is intentionally not compiled in Batch 0 to keep the
//!   dependency graph small.

use crate::config::{Config, LogFormat};
use axum::extract::MatchedPath;
use axum::http::Request;
use sutra_web::request_id::RequestId;
use tracing::Span;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init(cfg: &Config) {
    let filter = EnvFilter::try_new(&cfg.log_filter).unwrap_or_else(|_| EnvFilter::new("info"));
    let registry = tracing_subscriber::registry().with(filter);
    let result = match cfg.log_format {
        LogFormat::Json => registry
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .flatten_event(true)
                    .with_current_span(true)
                    .with_span_list(false),
            )
            .try_init(),
        LogFormat::Pretty => registry
            .with(tracing_subscriber::fmt::layer().compact())
            .try_init(),
    };
    if result.is_err() {
        // Already initialised (tests); keep the existing subscriber.
        return;
    }
    if let Some(endpoint) = &cfg.otel_endpoint {
        tracing::warn!(%endpoint, "OTEL_EXPORTER_OTLP_ENDPOINT is set but the OTLP exporter is not compiled in Batch 0");
    }
}

/// Span factory for `tower_http::trace::TraceLayer`.
pub fn make_span<B>(req: &Request<B>) -> Span {
    let request_id = req
        .extensions()
        .get::<RequestId>()
        .map(|r| r.0.as_str())
        .unwrap_or("-");
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or("-");
    let traceparent = req
        .headers()
        .get("traceparent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    tracing::info_span!(
        "http.request",
        request_id = %request_id,
        method = %req.method(),
        route = %route,
        path = %req.uri().path(),
        traceparent = %traceparent,
        status = tracing::field::Empty,
    )
}
