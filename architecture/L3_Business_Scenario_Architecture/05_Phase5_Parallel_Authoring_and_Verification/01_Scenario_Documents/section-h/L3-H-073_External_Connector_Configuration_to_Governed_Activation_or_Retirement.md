# External Connector Configuration to Governed Activation or Retirement

## 1. Scenario Metadata

**Scenario ID:** L3-H-073  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** A, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-A03_Authentication_Workflows.md; L2-H10_External_Education_Integration_Workflows.md; `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

## 2. Scenario Purpose

Govern a new or changed External Connector Configuration through review, approval, activation, suspension/restoration, rollback and retirement while preserving A03 authentication ownership.

## 3. Business Outcome

The connector and versioned configuration are validated, activated, suspended, rolled back, or retired with complete audit history

## 4. Scenario Boundary and Scope

The scenario begins when an approved provider requires a new/changed connector configuration and ends when that H10 configuration reaches Active, Suspended/restored, Rolled Back or Retired under governed H10 authority.

## 5. Ownership and Participating Domains

H10 owns External Connector Configuration governance/lifecycle. A03 retains sole ownership of Authentication Provider/Credential identity/configuration/lifecycle facts referenced by H10.

## 6. Business Actors, Roles and Responsibilities

The **External Education Integration Governance Authority** is the canonical business authority for connector approval and rollback decisions. Integration administration executes recorded decisions without acquiring A03 ownership.

## 7. Preconditions and Entry Criteria

- Approved Provider Trust/context exists.
- Authentication references remain A03-owned.
- A connector configuration proposal/version is available for governance.

## 8. Trigger

An approved provider requires a new or changed connector

## 9. Main Business Flow

1. Establish the configuration in `Draft`.
2. Validate configuration and move to `Under Review`.
3. If approved, move to `Approved` and publish `ExternalConnectorConfigurationApproved`.
4. Activate it, move to `Active`, and publish `ExternalConnectorActivated`.
5. Preserve immutable version/decision history.

## 10. Alternate Business Flows

- `Under Review → Rejected`.
- `Active → Suspended → Active`, publishing `ExternalConnectorSuspended` when suspended.
- Harmful/invalid newly effective version: authorize governed rollback, publish `ExternalConnectorRollbackAuthorized`, restore an eligible prior approved configuration through a new effective decision, and publish `ExternalConnectorRolledBack`.
- `Active / Suspended → Retired`, publishing `ExternalConnectorRetired`.

## 11. Failure, Interruption and Business Recovery

Failed review prevents activation. Rollback creates a new governed effective decision and never rewrites immutable version history. H10 never changes A03-owned authentication lifecycles.

## 12. Business Rules and Decision Constraints

Canonical H10 lifecycle: `Draft → Under Review → Approved → Active → Retired`, with Rejected, Suspended/restored and governed rollback branches. A03 authentication ownership is exclusive.

## 13. State and Lifecycle Impact

Only the External Connector Configuration lifecycle changes. Credential / Authentication Provider state remains unchanged by H10.

## 14. Business Events and Cross-Domain Collaboration

H10 publishes `ExternalConnectorConfigurationApproved`, `ExternalConnectorActivated`, `ExternalConnectorSuspended`, `ExternalConnectorRollbackAuthorized`, `ExternalConnectorRolledBack`, and `ExternalConnectorRetired`. A03 facts are referenced only.

## 15. Business Authorization, Approval and Audit Obligations

External Education Integration Governance Authority owns approval/rollback decisions. Evidence preserves configuration version, review, approval, activation, suspension/restoration, rollback, retirement and A03 references.

## 16. Completion Criteria and Postconditions

Completion requires the configuration to reach its governed H10 outcome with immutable history and without ownership transfer from A03.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Resolved Historical Blocker

The former rollback/lifecycle/event ambiguity was resolved by locked `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No additional scenario-specific questions.
