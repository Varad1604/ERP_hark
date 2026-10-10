//! # Domain D — Academic module
//!
//! **Batch 0 scaffold.** Owns PostgreSQL schema `academic` and the `/api/v1/academic/...` route space.
//! Implementation lands in Batch 4 (see `docs/batches.md`). Prototype navigation group:
//! `academics`.
//!
//! Boundary rules (ADR-L5-0003): this crate may only depend on `sutra-kernel`,
//! `sutra-contracts`, `sutra-web` and `sutra-persistence`; it publishes its ports in
//! `sutra_contracts::academic` and never reads another module's schema (ADR-L5-0036 §3.2).

use sutra_kernel::module::{ModuleDescriptor, ModuleStatus};
use utoipa_axum::router::OpenApiRouter;

pub const MODULE: ModuleDescriptor = ModuleDescriptor {
    domain: "D",
    key: "academic",
    name: "Academic",
    schema: "academic",
    batch: 4,
    status: ModuleStatus::Scaffolded,
};

/// Module routes (empty until Batch 4).
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_matches_crate() {
        assert_eq!(MODULE.key, "academic");
        assert_eq!(MODULE.schema, MODULE.key);
    }
}
