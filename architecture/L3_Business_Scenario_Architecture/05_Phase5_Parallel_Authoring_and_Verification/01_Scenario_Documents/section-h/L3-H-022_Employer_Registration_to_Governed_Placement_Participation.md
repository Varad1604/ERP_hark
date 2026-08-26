# Employer Registration to Governed Placement Participation

# 1. Scenario Metadata

**Scenario ID:** L3-H-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Placement drive eligibility and employer participation governance

# 2. Scenario Purpose

Govern the institutional placement relationship with an employer from participation request through H04 registration and verification while preserving Section A authorization/audit ownership.

# 3. Business Outcome

The employer is verified and activated, rejected, suspended, or retired under placement governance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an employer requests institutional placement participation.

## 4.2 End Boundary

H04 supports registration, verification, rejection, suspension, restoration from suspension, and inactivity. It does **not** define `Verified → Active` in the Employer transition table and defines no `Retired` state, transition, command, or event. The frozen end boundary cannot therefore be fully reached.

## 4.3 In Scope

- Record employer-originated participation intent through authorized Placement staff.
- Register one Placement-owned Employer aggregate.
- Submit and decide employer verification.
- Apply only H04-defined employer status transitions.
- Preserve immutable verification/status history.

## 4.4 Out of Scope

- Employer authentication accounts; H04 explicitly assumes none.
- Invented activation or retirement lifecycle semantics.
- Placement-drive execution after employer onboarding.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization and canonical audit evidence. |
| Placement / H | Owns the institutional Placement Employer aggregate, verification status, participation lifecycle, and Employer events. |

The Employer aggregate represents the institutional placement relationship, not the external company's corporate lifecycle.

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Employer | Originates the placement-participation request outside SutraERP. | Requests participation. |
| Placement Officer | Registers and verifies employers. | H04 assigns employer verification to this role. |
| Placement Coordinator | May register employer-originated actions. | H04 authorization matrix permits employer registration. |

# 7. Preconditions and Entry Criteria

- Employer participation intent is available.
- Authorized Placement staff can record it.
- Duplicate Employer creation is prevented.
- Mandatory organization/contact information is available.

# 8. Trigger

An employer requests institutional placement participation

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Record the employer request and validate mandatory information and uniqueness. | Placement Officer/Coordinator / H | Registration can proceed safely. |
| 2 | Create the Placement Employer record. | H | Employer is `Registered`; applicable registration events are published. |
| 3 | Submit the Employer for verification. | Placement Officer / H | Employer becomes `Verification Pending`; `EmployerVerificationRequested` is published. |
| 4 | Review supporting evidence and decide verification. | Placement Officer / H | Approval yields `Verified`; rejection yields `Inactive`. |
| 5 | Apply H04-defined later status changes when applicable. | H | `Verified → Suspended`, `Suspended → Active`, and `Active → Inactive` are defined. |
| 6 | Complete direct activation from `Verified` and retirement when required by the frozen outcome. | H | **BLOCKED:** required lifecycle semantics are absent upstream. |

# 10. Alternate Business Flows

- Duplicate/incomplete registration: do not create or advance a new Employer.
- Verification rejected: `Verification Pending → Inactive` with immutable decision evidence.
- Verified employer suspended: `Verified → Suspended`.
- Suspended employer restored: `Suspended → Active`.
- Retirement required: no H04 `Retired` state exists; no local substitute is permitted.

# 11. Failure, Interruption and Business Recovery

Incomplete verification evidence prevents approval. Rejected employers cannot create drives. Missing direct activation and retirement semantics require upstream resolution rather than inferred state transitions.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Employer code must be unique and a primary contact is required. | H | Registration integrity is preserved. |
| Verification is mandatory before recruitment participation. | H | Unverified employers cannot create governed drives. |
| Only authorized officers may verify employers. | H / A | Verification authority is governed. |
| Verification decisions/history are immutable. | H | Historical evidence cannot be rewritten. |
| Closed/inactive employer participation cannot create new drives where H04 disallows it. | H | Employer status constrains participation. |

# 13. State and Lifecycle Impact

H04 explicitly defines `Registered → Verification Pending → Verified` or `Inactive`, plus `Verified → Suspended → Active → Inactive`. It does not define `Verified → Active` and has no `Retired` state. Those missing positions materially affect the frozen outcome.

# 14. Business Events and Cross-Domain Collaboration

Exact H04 Employer events include `EmployerRegistrationInitiated`, `EmployerRegistered`, `EmployerVerificationRequested`, `EmployerVerified`, `EmployerVerificationRejected`, and `EmployerStatusUpdated`, published by Placement. No retirement event exists.

# 15. Business Authorization, Approval and Audit Obligations

Placement Officer is the source-supported verification role; Placement Coordinator may register but does not verify in H04's authorization matrix. Evidence must preserve registration, verification request, reviewing officer, decision, supporting evidence, status changes, and resulting Employer events. Section A retains authorization/audit ownership.

# 16. Completion Criteria and Postconditions

Registration/verification can complete through the defined H04 transitions. Full frozen completion is blocked until direct activation from `Verified` and the `Retired` outcome are defined upstream. No missing state is inferred.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H04 transition table omits `Verified → Active` although the lifecycle diagram depicts Active participation. | Frozen successful activation lacks a defined transition. | Define the canonical activation transition/guard. |
| Frozen outcome includes `Retired`, but H04 defines no Retired state/transition/event. | Mandatory terminal branch is absent. | Define Employer retirement semantics upstream. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
