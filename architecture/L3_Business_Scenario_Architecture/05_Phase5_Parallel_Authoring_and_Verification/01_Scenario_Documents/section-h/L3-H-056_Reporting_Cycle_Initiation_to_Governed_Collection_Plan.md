# Reporting Cycle Initiation to Governed Collection Plan

# 1. Scenario Metadata

**Scenario ID:** L3-H-056  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows; `L2-H08_Regulatory_Reporting_Authority_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Establish the governed Reporting Collection Plan for a statutory/accreditation reporting period while preserving each evidence-owning domain's authority over the facts the cycle later consumes.

# 3. Business Outcome

Scope, responsibilities, deadlines, source contracts, validations, and approvals are established in an active reporting cycle

The frozen phrase `active reporting cycle` is a business readiness description, not a new Reporting Cycle lifecycle state. Readiness is established by an `Active` H08 Reporting Collection Plan associated with the governed cycle.

# 4. Scenario Boundary and Scope

The scenario begins when a reporting period opens and ends when the Reporting Collection Plan is Approved and Active, with scope/source responsibilities/governance established for the associated Reporting Cycle.

# 5. Ownership and Participating Domains

- **H08:** owns the Reporting Collection Plan, Reporting Cycle metadata and reporting governance.
- **VAR:Evidence-Owning Domain:** remains the canonical owner of each authoritative source fact/evidence item consumed under the plan.

# 6. Business Actors, Roles and Responsibilities

The **Regulatory Reporting Governance Authority** is the canonical institutional business authority for approving the Reporting Collection Plan. Source-domain responsibilities are resolved according to the actual evidence-owning domain without transferring ownership to H08.

# 7. Preconditions and Entry Criteria

- Applicable governed Reporting Framework Definition is available.
- A Reporting Cycle/reporting period exists.
- Required source-domain collection scope and evidence obligations can be identified.
- `VAR:Evidence-Owning Domain` remains intentionally variable until execution context identifies the source owner.

# 8. Trigger

A statutory or accreditation reporting period opens

# 9. Main Business Flow

1. Establish a `Draft` Reporting Collection Plan for the governed Reporting Cycle.
2. Define required source-domain collection scope, responsibilities, deadlines, evidence obligations and reporting-contract references.
3. Move the plan to `Under Review`.
4. Regulatory Reporting Governance Authority evaluates the plan.
5. If approved, move to `Approved` and publish `ReportingCollectionPlanApproved`.
6. Activate the approved plan, move to `Active`, and publish `ReportingCollectionPlanActivated`.
7. Make the active plan available to govern later source-data collection without changing source ownership.

# 10. Alternate Business Flows

- `Under Review → Revision Required → Draft`, publishing `ReportingCollectionPlanRevisionRequired`.
- `Active → Suspended → Active` where collection governance is temporarily halted/restored.
- `Active / Suspended → Cancelled` where the reporting cycle is formally cancelled, publishing `ReportingCollectionPlanCancelled`.
- After governed collection obligations are complete, move `Active → Completed` and publish `ReportingCollectionPlanCompleted`.

# 11. Failure, Interruption and Business Recovery

Incomplete scope/responsibility/source-contract governance prevents plan activation. Missing source facts are resolved by their owning domains; H08 does not fabricate or modify them.

# 12. Business Rules and Decision Constraints

- Canonical Collection Plan lifecycle is `Draft → Under Review → Approved → Active → Completed`.
- Branches include Revision Required, Suspended/restored and Cancelled.
- H08 owns collection-plan governance only; source domains own operational facts.
- No Reporting Cycle state named `Active` is introduced by this scenario.

# 13. State and Lifecycle Impact

The Reporting Collection Plan reaches `Active`, establishing the business-ready collection context for the associated Reporting Cycle. The base Reporting Cycle lifecycle remains governed separately and is not renamed.

# 14. Business Events and Cross-Domain Collaboration

H08 publishes `ReportingCollectionPlanApproved`, `ReportingCollectionPlanRevisionRequired`, `ReportingCollectionPlanActivated`, `ReportingCollectionPlanCompleted`, and `ReportingCollectionPlanCancelled`. Source-domain facts/events retain their original owners/publishers.

# 15. Business Authorization, Approval and Audit Obligations

Regulatory Reporting Governance Authority approves the plan. Evidence preserves scope, responsibilities, deadlines, source references, review, decision, activation and subsequent plan lifecycle history.

# 16. Completion Criteria and Postconditions

Completion requires an approved and Active Reporting Collection Plan governing the Reporting Cycle's source collection obligations, with source ownership preserved. The frozen `active reporting cycle` description is satisfied without inventing a Reporting Cycle `Active` state.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing collection-plan approval authority/lifecycle and ambiguity around an `active` reporting context were resolved by locked `L2-H08_Regulatory_Reporting_Authority_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions. `VAR:Evidence-Owning Domain` remains preserved exactly as frozen.

## 17.3 Deferred Questions

No additional scenario-specific questions.
