//! Tenant-rooted Validated Execution Scope (ADR-L5-0004 §7).
//!
//! * Tenant is the mandatory root protected scope.
//! * Institution is optional and subordinate; it can never widen or replace the Tenant.
//! * Missing/ambiguous/mismatched scope fails closed — there is no default Tenant fallback.

use crate::error::KernelError;
use crate::typed_id;

typed_id!(
    /// Root protected technical scope.
    TenantId
);
typed_id!(
    /// Institution scope, always a descendant of a Tenant.
    InstitutionId
);

/// The validated scope a protected operation executes in. Must be carried across every
/// synchronous, asynchronous, workflow and persistence hand-off.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionScope {
    pub tenant_id: TenantId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<InstitutionId>,
}

impl ExecutionScope {
    pub fn tenant(tenant_id: TenantId) -> Self {
        Self {
            tenant_id,
            institution_id: None,
        }
    }

    pub fn institution(tenant_id: TenantId, institution_id: InstitutionId) -> Self {
        Self {
            tenant_id,
            institution_id: Some(institution_id),
        }
    }

    /// Parses raw (untrusted) scope values. Fails closed on anything missing or malformed.
    pub fn parse(tenant: Option<&str>, institution: Option<&str>) -> Result<Self, KernelError> {
        let tenant = tenant
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| KernelError::Scope("tenant scope is required".into()))?;
        let tenant_id: TenantId = tenant
            .parse()
            .map_err(|_| KernelError::Scope("tenant scope is malformed".into()))?;
        let institution_id = match institution.map(str::trim).filter(|s| !s.is_empty()) {
            None => None,
            Some(raw) => Some(
                raw.parse::<InstitutionId>()
                    .map_err(|_| KernelError::Scope("institution scope is malformed".into()))?,
            ),
        };
        Ok(Self {
            tenant_id,
            institution_id,
        })
    }

    /// Requires an institution-level scope for operations that are institution-bound.
    pub fn require_institution(&self) -> Result<InstitutionId, KernelError> {
        self.institution_id
            .ok_or_else(|| KernelError::Scope("institution scope is required".into()))
    }

    /// Verifies that `other` does not switch or widen this scope. Used when a hand-off carries
    /// its own scope claim (event, workflow callback, job) that must be revalidated.
    pub fn ensure_within(&self, other: &ExecutionScope) -> Result<(), KernelError> {
        if self.tenant_id != other.tenant_id {
            return Err(KernelError::Scope("tenant scope mismatch".into()));
        }
        match (self.institution_id, other.institution_id) {
            (Some(a), Some(b)) if a != b => {
                Err(KernelError::Scope("institution scope mismatch".into()))
            }
            (Some(_), None) => Err(KernelError::Scope(
                "institution scope may not be widened".into(),
            )),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const T: &str = "0192f5a0-7c3e-7000-8000-000000000001";
    const I: &str = "0192f5a0-7c3e-7000-8000-0000000000aa";

    #[test]
    fn missing_tenant_fails_closed() {
        assert!(matches!(
            ExecutionScope::parse(None, None),
            Err(KernelError::Scope(_))
        ));
        assert!(matches!(
            ExecutionScope::parse(Some("  "), None),
            Err(KernelError::Scope(_))
        ));
    }

    #[test]
    fn malformed_values_fail_closed() {
        assert!(ExecutionScope::parse(Some("tenant-1"), None).is_err());
        assert!(ExecutionScope::parse(Some(T), Some("nope")).is_err());
    }

    #[test]
    fn parses_valid_scope() {
        let s = ExecutionScope::parse(Some(T), Some(I)).expect("valid");
        assert_eq!(s.tenant_id.to_string(), T);
        assert_eq!(
            s.require_institution().map(|i| i.to_string()).ok(),
            Some(I.to_string())
        );
    }

    #[test]
    fn widening_or_switching_is_rejected() {
        let inst = ExecutionScope::parse(Some(T), Some(I)).expect("valid");
        let tenant_only = ExecutionScope::parse(Some(T), None).expect("valid");
        let other = ExecutionScope::tenant(TenantId::new());
        assert!(inst.ensure_within(&inst).is_ok());
        assert!(
            tenant_only.ensure_within(&inst).is_ok(),
            "narrowing is allowed"
        );
        assert!(
            inst.ensure_within(&tenant_only).is_err(),
            "widening is rejected"
        );
        assert!(
            inst.ensure_within(&other).is_err(),
            "switching tenant is rejected"
        );
    }
}
