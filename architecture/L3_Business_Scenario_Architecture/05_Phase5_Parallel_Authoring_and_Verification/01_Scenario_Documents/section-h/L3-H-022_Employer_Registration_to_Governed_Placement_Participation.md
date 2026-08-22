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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; Section A identity, authorization, and audit architecture  
**Referenced By:** Employer onboarding, placement-drive eligibility, employer governance, placement reporting, notifications, search, and audit activities

# 2. Scenario Purpose

Govern employer onboarding into institutional placement participation from an external employer request through placement-owned registration, verification, and permitted employer status outcomes while preserving Section A identity/authorization boundaries.

# 3. Business Outcome

The employer is verified and activated, rejected, suspended, or retired under placement governance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an employer requests institutional placement participation.

## 4.2 End Boundary

The frozen end boundary requires the employer to become verified and activated, rejected, suspended, or retired. Locked H04 defines registration, verification, rejection, suspension, active, and inactive concepts, but does not define a `Retired` state and does not define a `Verified → Active` transition. The complete frozen end boundary is therefore unresolved.

## 4.3 In Scope

- Recording employer-originated placement participation intent through authorized placement staff.
- Registering a unique Placement-owned Employer aggregate.
- Capturing mandatory employer organization/contact information.
- Submitting the employer for verification.
- Verifying or rejecting employer participation under H04 rules.
- Applying source-defined employer status changes where transitions exist.
- Preserving immutable verification and status history.

## 4.4 Out of Scope

- Creating employer authentication accounts; H04 explicitly assumes none.
- Inventing a `Retired` employer state or retirement transition.
- Inventing the missing `Verified → Active` transition.
- Creating placement drives before employer governance permits participation.
- Owning Section A identity, authentication, authorization, or audit infrastructure.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains authentication, authorization, identity support, and canonical audit infrastructure ownership. |
| Placement / Section H | Owns the Employer aggregate, employer registration, verification status, placement participation lifecycle, verification history, and placement-owned employer events. |

The employer is an external business participant; H04 owns the institutional placement relationship record, not the employer's external corporate lifecycle.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Source Support |
|---|---|---|
| Employer | Requests institutional placement participation outside the system boundary. | Frozen trigger and L2-H04 §5 employer-originated actions. |
| Placement Officer | Records employer registration, initiates verification, reviews employer details, and performs H04 verification decisions where authorized. | L2-H04 §§12.1, 19.1, 19.2. |
| Placement Coordinator | May record employer-originated placement actions where H04 permits placement-coordinator participation; no independent employer authentication is assumed. | L2-H04 §5 and aggregate ownership context. |

H04's canonical crosswalk records its formal Actors section as absent, but the workflows explicitly name Placement Officer and Placement Coordinator responsibilities used here.

# 7. Preconditions and Entry Criteria

- Employer participation intent has been received.
- Authorized placement staff can record the employer-originated request.
- Employer does not already exist as a duplicate Placement Employer record.
- Mandatory organization and primary contact information are available.
- Placement verification governance is available.

# 8. Trigger

An employer requests institutional placement participation

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the employer's participation request and record it through authorized placement staff. | Employer / Placement Officer or Placement Coordinator | Placement onboarding intent is established without assuming employer authentication. |
| 2 | Validate uniqueness and mandatory employer organization/contact information. | Placement | Duplicate or incomplete employer registrations do not proceed. |
| 3 | Create the Placement-owned Employer record and preserve its placement identity/history. | Placement Officer / Placement | Employer reaches the source-defined `Registered` position. |
| 4 | Submit the employer for verification. | Placement Officer / Placement | Employer reaches `Verification Pending`; `EmployerVerificationRequested` is source-supported. |
| 5 | Review employer authenticity and supporting evidence under placement governance. | Placement Officer / Placement | Verification decision can be made. |
| 6 | Approve or reject verification using the locked H04 transition. | Placement Officer / Placement | Approved employer becomes `Verified`; rejected employer becomes `Inactive`. |
| 7 | **Activation/retirement completion unresolved:** progress a verified employer to active participation or retire it when applicable. | **BLOCKED — required transitions/states not fully defined upstream** | Frozen final outcomes cannot all be reached authoritatively. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Duplicate employer detected | Employer already exists. | Reject duplicate registration and retain the existing Employer identity/history. | No duplicate Employer aggregate is created. | Placement |
| Verification rejected | Employer evidence does not satisfy H04 verification. | Apply `RejectEmployer` from `Verification Pending`. | Employer becomes `Inactive`; verification rejection evidence is preserved. | Placement |
| Employer suspended | A source-supported status change applies after verification. | Apply the defined `Verified → Suspended` transition. | Employer becomes `Suspended`. | Placement |
| Suspended employer restored | H04 permits status restoration. | Apply the defined `Suspended → Active` transition. | Employer becomes `Active`. | Placement |
| Employer retirement required | Frozen semantics require retirement. | **No H04 `Retired` state/transition exists.** | Upstream resolution required. | Placement |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Mandatory employer information missing | Employer cannot be reliably registered. | Do not create/advance the Employer until required information is available. | Placement |
| Verification evidence incomplete or invalid | Employer cannot become verified. | Preserve `Verification Pending` or apply rejection according to H04 governance. | Placement |
| Activation is required immediately after verification | Frozen outcome expects activated participation. | H04 has no defined `Verified → Active` transition; do not invent one. | Placement |
| Retirement is required | Frozen outcome expects a retired terminal position. | H04 has no `Retired` state or transition; require upstream architecture change control. | Placement |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Employer code must be unique. | Placement | Duplicate employer records are prohibited. | L2-H04 §17.1 / §19.1 |
| Every employer must have at least one primary contact. | Placement | Registration requires mandatory contact information. | L2-H04 §17.1 |
| Verification is mandatory before recruitment activities. | Placement | Unverified employer cannot participate in placement drives. | L2-H04 §19.1 |
| Only authorized officers may verify employers. | Placement / Section A | Verification decision requires governed authority. | L2-H04 §19.2 |
| Verification decisions and history are immutable. | Placement | Verification evidence is preserved. | L2-H04 §§17.1, 19.2 |
| Closed employers cannot create new placement drives. | Placement | Employer lifecycle controls future drive eligibility. | L2-H04 §17.1 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Employer | No Placement Employer record | Registration creates `Registered`; submission moves to `Verification Pending`. | `Verification Pending` | Placement | L2-H04 §§17.1, 19.1 |
| Employer | `Verification Pending` | Verification approval or rejection. | `Verified` or `Inactive` | Placement | L2-H04 §§17.1, 19.2 |
| Employer | `Verified` | H04 defines `Verified → Suspended`; it depicts `Active` in lifecycle but does not define `Verified → Active`. | `Suspended` is defined; direct activation is **undefined**. | Placement | L2-H04 §17.1 |
| Employer | `Suspended` | H04 defines restoration through status update. | `Active` | Placement | L2-H04 §17.1 |
| Employer | Any state | Frozen outcome requires retirement. | **`Retired` not defined.** | Placement | Gap against L2-H04 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The Employer aggregate/workflows define the exact Placement events `EmployerRegistrationInitiated`, `EmployerRegistered`, `EmployerVerificationRequested`, `EmployerVerified`, `EmployerVerificationRejected`, and `EmployerStatusUpdated`, published by the Placement-owned Employer aggregate.

No exact event is defined for employer retirement, and no event is defined specifically for a missing `Verified → Active` transition. L3 does not invent either.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Employer-originated participation intent is recorded by authorized placement staff. Section A supplies identity/authorization/audit context. Placement retains all employer-placement relationship state and publishes its own employer lifecycle events for authorized downstream consumers such as Notification, Search, and Dashboard capabilities.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Employer participation intent originates externally. H04 assigns employer registration and verification activity to Placement Officer, and its verification workflow states that only authorized officers may verify employers.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty rule is defined for employer verification. L3 does not create authority for activation or retirement transitions missing from the locked lifecycle.

## 15.3 Business Audit and Evidence Obligations

Evidence must preserve registration initiator, registration timestamp, employer information, verification request, reviewing officer, verification outcome, decision timestamp, supporting evidence, status changes, and the applicable Employer events. Verification history remains immutable; Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

Registration and verification can complete through the source-defined `Registered → Verification Pending → Verified/Inactive` path. The frozen scenario as a whole cannot complete authoritatively until H04 defines the missing activation path from `Verified` and the required `Retired` outcome.

## 16.2 Postconditions

- Placement owns one governed Employer record for the institutional placement relationship.
- Employer verification history remains immutable.
- Rejected employers cannot create placement drives.
- No direct `Verified → Active` transition or `Retired` outcome is invented.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen final outcome requires the employer to be **activated**, but H04's Employer lifecycle/transition table has no `Verified → Active` transition. `Active` is reachable only from `Suspended` in the explicit table. | A mandatory successful participation outcome lacks a valid lifecycle transition. | Define the canonical activation transition/guard from verified employer to active placement participation. |
| Frozen final outcome includes **retired**, but H04 defines no `Retired` Employer state, transition, command, or event. | A mandatory terminal outcome is absent from the locked lifecycle. | Add/clarify canonical Employer retirement semantics through upstream architecture change control. |
| H04's canonical crosswalk records the formal Actors section as absent, although workflows name Placement Officer/Coordinator. | Actor model is only partially formalized. | Complete the canonical H04 Actors section upstream. |

**Architecture assessment:** `BLOCKED`

Registration, verification, rejection, suspension, and suspended-to-active restoration are source-supported. However, the frozen successful activation path and retired outcome are materially missing from the locked Employer lifecycle, so L3 cannot complete the scenario without upstream resolution.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
