//! # Domain C — Student Lifecycle module
//!
//! **Batch 0 scaffold.** Owns PostgreSQL schema `student` and the `/api/v1/student/...` route space.
//! Implementation lands in Batch 3 (see `docs/batches.md`). Prototype navigation group:
//! `students`.
//!
//! Boundary rules (ADR-L5-0003): this crate may only depend on `sutra-kernel`,
//! `sutra-contracts`, `sutra-web` and `sutra-persistence`; it publishes its ports in
//! `sutra_contracts::student` and never reads another module's schema (ADR-L5-0036 §3.2).

use sutra_kernel::module::{ModuleDescriptor, ModuleStatus};
use utoipa_axum::router::OpenApiRouter;

pub const MODULE: ModuleDescriptor = ModuleDescriptor {
    domain: "C",
    key: "student",
    name: "Student Lifecycle",
    schema: "student",
    batch: 3,
    status: ModuleStatus::Scaffolded,
};

/// Module routes (empty until Batch 3).
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_matches_crate() {
        assert_eq!(MODULE.key, "student");
        assert_eq!(MODULE.schema, MODULE.key);
    }
}
