//! # Domain J — Resources module
//!
//! **Batch 0 scaffold.** Owns PostgreSQL schema `resources` and the `/api/v1/resources/...` route space.
//! Implementation lands in Batch 8 (see `docs/batches.md`). Prototype navigation group:
//! `resources`.
//!
//! Boundary rules (ADR-L5-0003): this crate may only depend on `sutra-kernel`,
//! `sutra-contracts`, `sutra-web` and `sutra-persistence`; it publishes its ports in
//! `sutra_contracts::resources` and never reads another module's schema (ADR-L5-0036 §3.2).

use sutra_kernel::module::{ModuleDescriptor, ModuleStatus};
use utoipa_axum::router::OpenApiRouter;

pub const MODULE: ModuleDescriptor = ModuleDescriptor {
    domain: "J",
    key: "resources",
    name: "Resources",
    schema: "resources",
    batch: 8,
    status: ModuleStatus::Scaffolded,
};

/// Module routes (empty until Batch 8).
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_matches_crate() {
        assert_eq!(MODULE.key, "resources");
        assert_eq!(MODULE.schema, MODULE.key);
    }
}
