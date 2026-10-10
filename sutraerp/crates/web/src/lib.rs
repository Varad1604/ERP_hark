//! # sutra-web
//!
//! Shared HTTP platform layer for every module router (Axum/Tower, ADR-L5-0019):
//!
//! * [`problem`] – RFC 9457 `application/problem+json` errors with SutraERP extensions
//!   (`errorCode`, `correlationId`, `validationErrors`) — ADR-L5-0034 §3.4
//! * [`request_id`] – request/correlation id propagation (`X-Request-Id`)
//! * [`scope`] – Tenant/Institution execution-scope extractor (header based in dev) — ADR-L5-0004
//! * [`idempotency`] – `Idempotency-Key` middleware — ADR-L5-0034 §3.6
//! * [`health`] – `/health/live` and `/health/ready` — ADR-L5-0044
//!
//! Framework types defined here never leak into canonical contracts (`sutra-contracts`).

pub mod health;
pub mod idempotency;
pub mod problem;
pub mod request_id;
pub mod scope;

pub use problem::{ApiError, ApiResult, Problem};
pub use scope::{Scope, ScopeHeaders};
