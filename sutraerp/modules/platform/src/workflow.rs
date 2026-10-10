//! Batch 0 no-op realisation of [`WorkflowPort`]. Records intent in the trace and returns a
//! synthetic process reference. Replaced by the Flowable OSS 8 REST adapter (ADR-L5-0023/0042).

use std::collections::BTreeMap;
use sutra_contracts::platform::workflow::{ProcessInstanceRef, StartProcess, WorkflowPort};
use sutra_kernel::error::KernelError;
use sutra_kernel::scope::ExecutionScope;

#[derive(Debug, Default, Clone)]
pub struct NoopWorkflowAdapter;

#[async_trait::async_trait]
impl WorkflowPort for NoopWorkflowAdapter {
    async fn start_process(
        &self,
        scope: &ExecutionScope,
        req: StartProcess,
    ) -> Result<ProcessInstanceRef, KernelError> {
        tracing::info!(tenant = %scope.tenant_id, process = %req.process_definition_key, business_key = %req.business_key, "noop workflow: start_process");
        Ok(ProcessInstanceRef {
            process_definition_key: req.process_definition_key,
            process_instance_id: format!("noop-{}", sutra_kernel::id::new_v7()),
            business_key: req.business_key,
        })
    }

    async fn complete_task(
        &self,
        scope: &ExecutionScope,
        task_id: &str,
        _variables: BTreeMap<String, serde_json::Value>,
    ) -> Result<(), KernelError> {
        tracing::info!(tenant = %scope.tenant_id, task_id, "noop workflow: complete_task");
        Ok(())
    }

    async fn signal(
        &self,
        scope: &ExecutionScope,
        process_instance_id: &str,
        signal: &str,
    ) -> Result<(), KernelError> {
        tracing::info!(tenant = %scope.tenant_id, process_instance_id, signal, "noop workflow: signal");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sutra_kernel::scope::TenantId;

    #[tokio::test]
    async fn start_returns_reference() {
        let r = NoopWorkflowAdapter
            .start_process(
                &ExecutionScope::tenant(TenantId::new()),
                StartProcess {
                    process_definition_key: "admission".into(),
                    business_key: "APP-1".into(),
                    variables: BTreeMap::new(),
                },
            )
            .await
            .unwrap();
        assert!(r.process_instance_id.starts_with("noop-"));
        assert_eq!(r.business_key, "APP-1");
    }
}
