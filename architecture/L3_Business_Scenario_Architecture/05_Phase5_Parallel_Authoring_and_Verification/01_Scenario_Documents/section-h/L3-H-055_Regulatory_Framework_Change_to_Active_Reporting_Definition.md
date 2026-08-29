# Regulatory Framework Change to Active Reporting Definition

# 1. Scenario Metadata

**Scenario ID:** L3-H-055  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows; `L2-H08_Regulatory_Reporting_Authority_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern introduction or revision of an H08 Reporting Framework Definition so future regulatory reporting uses an approved, versioned and active definition without changing canonical source-domain facts.

# 3. Business Outcome

A versioned reporting definition is approved and made effective without changing canonical source-domain facts

# 4. Scenario Boundary and Scope

The scenario begins when a regulatory/institutional reporting framework is introduced or revised and ends when the applicable Reporting Framework Definition reaches Active, or is Rejected/Retired through governed alternatives, with immutable version history preserved.

# 5. Ownership and Participating Domains

H08 owns Reporting Framework Definitions, reporting metadata and their lifecycle. Source-owning domains retain all operational business facts later consumed by reporting.

# 6. Business Actors, Roles and Responsibilities

The **Regulatory Reporting Governance Authority** is the canonical institutional business authority for approval/rejection of Reporting Framework Definitions and material versions, and for governed retirement/supersession decisions where required.

# 7. Preconditions and Entry Criteria

- A new or revised reporting-framework requirement exists.
- Required definition metadata is available.
- Historical framework versions remain traceable.
- Source-domain ownership remains unchanged.

# 8. Trigger

A NEP, NAAC, AISHE, or institutional reporting framework is introduced or revised

# 9. Main Business Flow

1. Create a successor `Draft` Reporting Framework Definition for a new or material change.
2. Move it to `Under Review` and evaluate the governed reporting requirements.
3. If approved, move to `Approved` and publish `ReportingFrameworkDefinitionApproved`.
4. Activate the approved version, move to `Active`, and publish `ReportingFrameworkDefinitionActivated`.
5. When a successor version becomes Active, mark the predecessor `Superseded` and publish `ReportingFrameworkDefinitionSuperseded`.
6. Preserve immutable framework-version history and source-data ownership.

# 10. Alternate Business Flows

- `Under Review → Rejected`, publishing `ReportingFrameworkDefinitionRejected`.
- `Applicable prior version → Retired`, publishing `ReportingFrameworkDefinitionRetired` where governance retires the definition rather than superseding it.

# 11. Failure, Interruption and Business Recovery

Failed review prevents activation. A rejected or superseded version remains historical and is not rewritten. Operational source facts are never changed by H08 framework governance.

# 12. Business Rules and Decision Constraints

- Canonical lifecycle: `Draft → Under Review → Approved → Active → Superseded / Retired`.
- `Under Review → Rejected` is the rejection branch.
- Material change creates a successor Draft version.
- Historical versions remain immutable/traceable.
- H08 governance does not mutate source-domain operational facts.

# 13. State and Lifecycle Impact

The Reporting Framework Definition follows the canonical H08 amendment lifecycle above. Activation makes the approved version current for governed reporting; it does not alter any source-domain lifecycle.

# 14. Business Events and Cross-Domain Collaboration

H08 publishes `ReportingFrameworkDefinitionApproved`, `ReportingFrameworkDefinitionRejected`, `ReportingFrameworkDefinitionActivated`, `ReportingFrameworkDefinitionSuperseded`, and `ReportingFrameworkDefinitionRetired`. Source facts remain published/owned by their canonical domains.

# 15. Business Authorization, Approval and Audit Obligations

Regulatory Reporting Governance Authority owns the framework approval/rejection and applicable retirement/supersession decisions. Evidence preserves version, review, decision, activation, supersession/retirement and governing requirement history.

# 16. Completion Criteria and Postconditions

Completion requires the applicable definition to reach Active after governed approval or to reach another authoritative terminal alternative, with source-domain facts unchanged and historical versions preserved.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing Reporting Framework approval authority/state/event was resolved by locked `L2-H08_Regulatory_Reporting_Authority_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
