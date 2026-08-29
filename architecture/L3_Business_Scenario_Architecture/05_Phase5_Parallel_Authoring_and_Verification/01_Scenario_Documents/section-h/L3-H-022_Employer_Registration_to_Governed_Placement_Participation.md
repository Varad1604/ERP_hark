# Employer Registration to Governed Placement Participation

# 1. Scenario Metadata

**Scenario ID:** L3-H-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** Placement drive eligibility and employer participation governance; L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern the institutional placement relationship with an employer from participation request through registration, verification, activation, suspension/reactivation, rejection or retirement while preserving Section A authorization/audit ownership.

# 3. Business Outcome

The employer is verified and activated, rejected, suspended, or retired under placement governance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an employer requests institutional placement participation.

## 4.2 End Boundary

The scenario ends when the Placement-owned Employer reaches the applicable canonical H04 participation outcome: `Active`, `Rejected`, `Suspended`, or `Retired`, with verification and status history preserved.

## 4.3 In Scope

- Record employer-originated participation intent through authorized Placement staff.
- Register one Placement-owned Employer aggregate.
- Submit and decide employer verification.
- Activate verified participation.
- Govern suspension/reactivation and retirement where applicable.
- Preserve immutable verification/status history.

## 4.4 Out of Scope

- Employer authentication accounts; H04 introduces no employer account/technical role.
- The external company's corporate lifecycle.
- Placement-drive execution after employer onboarding.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization and canonical audit capability. |
| Placement / H | Owns the institutional Placement Employer aggregate, participation lifecycle and Employer events. |

The Employer aggregate represents the institutional placement relationship, not the external company's corporate lifecycle.

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Employer | Originates the placement-participation request outside SutraERP. | Supplies employer-originated facts through the governed process. |
| Placement Officer | Primary business authority for Employer participation decisions. | Registers, verifies, activates, suspends, reactivates and retires Employer participation according to H04. |
| Placement Coordinator | Supports employer/placement administration where H04 assigns responsibility. | Does not replace Placement Officer authority where H04 reserves the decision. |

# 7. Preconditions and Entry Criteria

- Employer participation intent is available.
- Authorized Placement staff can record it.
- Duplicate Employer creation is prevented.
- Mandatory organization/contact information is available.

# 8. Trigger

An employer requests institutional placement participation

# 9. Main Business Flow

1. Record the employer request and validate mandatory information and uniqueness.
2. Create the Placement Employer record in `Registered`.
3. Move the Employer to `Verification Pending` and perform governed verification.
4. If verification fails, move `Verification Pending → Rejected` and preserve decision evidence.
5. If verified, move to `Verified` and then `Active` for current governed participation.
6. Where required, move `Active → Suspended` and later `Suspended → Active` after governed restoration.
7. When the participation relationship is ended, move `Active` or `Suspended → Retired` and preserve the historical record.
8. Publish the applicable canonical H04 Employer lifecycle events.

# 10. Alternate Business Flows

- Verification rejected: `Verification Pending → Rejected`.
- Active participation suspended: `Active → Suspended`.
- Suspended participation restored: `Suspended → Active`.
- Suspended or Active participation ended: `→ Retired`.

# 11. Failure, Interruption and Business Recovery

Incomplete verification evidence prevents activation. Suspension preserves the Employer record and may be reversed through governed reactivation. `Retired` is terminal for that participation record; future participation requires a new governed relationship where policy permits.

# 12. Business Rules and Decision Constraints

- Employer verification precedes Active participation.
- Placement Officer is the primary business authority for Employer decisions.
- Employer participation lifecycle is `Registered → Verification Pending → Verified → Active → Retired` with Rejected and Suspended branches.
- `Retired` preserves historical recruitment evidence.
- Section A provides authorization/audit capability without owning H04 decisions.

# 13. State and Lifecycle Impact

Canonical Employer participation lifecycle:

`Registered → Verification Pending → Verified → Active → Retired`

Branches:

- `Verification Pending → Rejected`
- `Active → Suspended → Active`
- `Suspended → Retired`

# 14. Business Events and Cross-Domain Collaboration

Canonical H04 Employer events include existing registration/verification facts plus:

- `EmployerActivated`
- `EmployerSuspended`
- `EmployerReactivated`
- `EmployerRetired`

Placement remains the semantic publisher. Section A contributes authorization/audit capability only.

# 15. Business Authorization, Approval and Audit Obligations

Placement Officer is the canonical business authority for the Employer participation decisions in this scenario. Evidence preserves registration, verification, activation/suspension/reactivation/retirement decisions, actor, rationale and immutable historical status.

# 16. Completion Criteria and Postconditions

Completion occurs when the Employer reaches the applicable governed participation outcome and the decision/history is preserved. Active participation enables governed Placement activity; Rejected, Suspended and Retired outcomes constrain participation according to H04.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing `Verified → Active` and `Retired` semantics were resolved by locked `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
