//! Module descriptors. The composition root registers one per domain module so tooling, health
//! and the `/api/v1/platform/modules` endpoint can enumerate the modular monolith.

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDescriptor {
    /// Domain letter from the architecture (A–J).
    pub domain: &'static str,
    /// Crate / route key, e.g. `student`.
    pub key: &'static str,
    pub name: &'static str,
    /// Owned PostgreSQL schema (ADR-L5-0036 §3.1).
    pub schema: &'static str,
    /// Delivery batch that implements the module.
    pub batch: u8,
    pub status: ModuleStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ModuleStatus {
    Scaffolded,
    Active,
}
