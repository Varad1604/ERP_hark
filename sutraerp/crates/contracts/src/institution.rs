//! Domain B — Institution contracts.

use sutra_kernel::error::KernelError;
use sutra_kernel::scope::{ExecutionScope, InstitutionId};

/// Read port other modules use to confirm an institution exists inside the caller's tenant.
#[async_trait::async_trait]
pub trait InstitutionDirectory: Send + Sync + 'static {
    async fn exists(&self, scope: &ExecutionScope, id: InstitutionId) -> Result<bool, KernelError>;
}
