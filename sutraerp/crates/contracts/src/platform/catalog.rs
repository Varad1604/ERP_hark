//! Workspace / workflow catalog DTOs (navigation metadata from the product prototype:
//! 99 workspaces, 595 workflows). Read-only reference data, not business state.

use super::auth::Role;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowEntry {
    pub label: String,
    pub roles: Vec<Role>,
    pub scenarios: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    /// Stable workspace key, e.g. `W-C01`.
    pub key: String,
    pub name: String,
    /// Navigation group (platform, institution, academics, students, …).
    pub group: String,
    pub roles: Vec<Role>,
    pub workflows: Vec<WorkflowEntry>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceList {
    pub items: Vec<Workspace>,
    pub total: usize,
    pub workflow_total: usize,
}
