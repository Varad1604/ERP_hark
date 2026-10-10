//! Workflow orchestration port (A05 — ADR-L5-0023 / ADR-L5-0042).
//!
//! BPMN 2.x on Flowable OSS 8.x runs as an *isolated JVM runtime* (governed runtime exception to
//! the Rust-primary rule). Rust modules never call Flowable directly: they use [`WorkflowPort`],
//! realised by a Flowable REST adapter in a later batch and by a no-op adapter in Batch 0.
//! Workflows orchestrate; owning domains keep business rules and state.

use std::collections::BTreeMap;
use sutra_kernel::error::KernelError;
use sutra_kernel::scope::ExecutionScope;

/// Reference to a started process instance.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInstanceRef {
    pub process_definition_key: String,
    pub process_instance_id: String,
    pub business_key: String,
}

/// Request to start a process. Variables are plain JSON values; scope travels explicitly so the
/// workflow runtime can propagate it to every callback (ADR-L5-0004 §3.5).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartProcess {
    pub process_definition_key: String,
    pub business_key: String,
    pub variables: BTreeMap<String, serde_json::Value>,
}

#[async_trait::async_trait]
pub trait WorkflowPort: Send + Sync + 'static {
    async fn start_process(
        &self,
        scope: &ExecutionScope,
        req: StartProcess,
    ) -> Result<ProcessInstanceRef, KernelError>;
    async fn complete_task(
        &self,
        scope: &ExecutionScope,
        task_id: &str,
        variables: BTreeMap<String, serde_json::Value>,
    ) -> Result<(), KernelError>;
    async fn signal(
        &self,
        scope: &ExecutionScope,
        process_instance_id: &str,
        signal: &str,
    ) -> Result<(), KernelError>;
}
