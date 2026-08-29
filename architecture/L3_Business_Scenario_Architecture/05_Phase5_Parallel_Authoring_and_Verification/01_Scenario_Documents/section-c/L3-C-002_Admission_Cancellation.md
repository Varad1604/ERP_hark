# Admission Cancellation

**Scenario ID:** L3-C-002  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Varad  
**Last Updated:** 2026-08-23  
**Depends On:** `L2-C01_Admissions_Workflows.md`, especially `WF-C01-06 — Admission Cancellation`  
**Referenced By:** None yet

## 1. Scenario Metadata

Frozen Phase 5 identity is preserved: `L3-C-002`, **Admission Cancellation**, Primary Owner **Admissions**, Participating Domains **C**, Scenario Type **Simple**, Priority **P1**.

The authoritative L2 workflow is `WF-C01-06 — Admission Cancellation`.

## 2. Scenario Purpose

Govern the cancellation of an eligible Admission Record before successful Student creation while preserving the Admission as institutional history.

## 3. Business Outcome

The eligible Admission Record reaches the canonical `Cancelled` state, no Student is created from that Admission Record, and Admissions publishes the completed cancellation fact `AdmissionCancelled`.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when an applicant or institution raises a valid cancellation request or decision for an existing Admission Record before successful Student creation.

### 4.2 End Boundary

The scenario ends when the Admission Record has reached `Cancelled`, the cancellation is preserved in Admission history, and `AdmissionCancelled` has been published.

### 4.3 In Scope

- Validate that the Admission Record is eligible for cancellation.
- Validate that the initiating/deciding actor is authorized under the governing Admissions policy.
- Capture the cancellation reason required by the authoritative workflow.
- Establish the cancellation decision.
- Transition an eligible Admission Record to `Cancelled`.
- Preserve Admission history and publish `AdmissionCancelled`.

### 4.4 Out of Scope

- Student creation or post-creation Student Lifecycle action.
- Admission Withdrawal, Revocation, Expiry, or Readmission.
- Finance-owned refund, payment, ledger, or accounting processing.
- Technical authorization design, APIs, data models, services, UI, queues, or infrastructure.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Admissions / Section C | Owns the Admission Record, cancellation decision, Admission lifecycle transition, historical Admission record, and `AdmissionCancelled` publication. |

Finance and other downstream consumers may react to `AdmissionCancelled` after completion, but that does not make them owners or participating domains in this frozen Simple scenario.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Applicant | May request cancellation where the governing institutional policy permits applicant-initiated cancellation. | Initiates own cancellation subject to the L2-C01 permission restriction and applicable policy. |
| Admission Officer | Processes the cancellation request, validates the Admission context, and performs authorized cancellation activity. | L2-C01 permits the Admission Officer to cancel an Admission. |
| Registrar | Exercises institutional administrative authority for cancellation where applicable. | L2-C01 permits the Registrar to cancel an Admission. |

Institution-specific routing among these upstream-supported roles remains configurable; L3 does not create additional approval-role names.

## 7. Preconditions and Entry Criteria

- A valid Admission Record exists.
- Successful Student creation has not occurred for that Admission Record.
- A valid cancellation basis/reason exists.
- The initiating or deciding actor is authorized under L2-C01 and the applicable institutional policy.
- The current Admission state is unambiguously eligible for cancellation under authoritative L2 architecture.
- For `Admission Approved` and `Financial Clearance`, L3 does not assume cancellation eligibility because L2-C01 contains conflicting transition matrices; see `OI-C002-01`.

## 8. Trigger

A valid applicant cancellation request or authorized institutional cancellation decision is raised for an Admission Record before successful Student creation.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Admission Record and cancellation basis. | Applicant / Admission Officer / Registrar | The Admission Record and cancellation intent are established. |
| 2 | Validate Student-creation boundary, actor authority, and the current Admission state against the authoritative cancellation rules. | Admission Officer / Registrar | Cancellation is confirmed as permitted for the current case. |
| 3 | Capture the cancellation reason required by the authoritative workflow. | Admission Officer / Registrar | The cancellation basis is attributable and auditable. |
| 4 | Establish the cancellation decision and transition the eligible Admission Record to `Cancelled`. | Admissions | The Admission lifecycle reaches the cancellation outcome. |
| 5 | Preserve the completed cancellation in Admission history. | Admissions | Historical integrity is maintained. |
| 6 | Publish `AdmissionCancelled`. | Admissions | The completed cancellation becomes available to authorized downstream consumers. |

The scenario ends at the cancellation outcome; downstream refund, accounting, notification delivery, or other domain-owned consequences do not extend this scenario boundary.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Applicant-initiated cancellation | Institutional policy permits the Applicant to cancel the Admission. | Applicant raises the request; Admissions validates authority and lifecycle eligibility and completes the governed cancellation. | Admission reaches `Cancelled`. | Admissions |
| Institution-initiated cancellation | An authorized institutional cancellation basis exists. | Admission Officer or Registrar performs the governed cancellation after validating authority and lifecycle eligibility. | Admission reaches `Cancelled`. | Admissions |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Actor is not authorized | Cancellation cannot proceed. | Reject the cancellation attempt; the Admission remains in its current valid state. | Admissions |
| Admission state is invalid for cancellation | Cancellation cannot proceed. | Continue through the applicable authoritative Admission lifecycle path. | Admissions |
| Admission is already `Cancelled` | No new cancellation outcome is created. | Preserve and return the existing cancellation outcome. | Admissions |
| Student creation has already occurred | This scenario is no longer valid. | Use the applicable Revocation or downstream Student Lifecycle process defined upstream. | Admissions / Student Lifecycle boundary |
| Current state is `Admission Approved` or `Financial Clearance` | L3 cannot determine eligibility because authoritative L2 transition matrices conflict. | Do not assert or lock the disputed transition; escalate `OI-C002-01` for upstream resolution. | Admissions architecture governance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Admission Cancellation applies before successful Student creation. | Admissions | Post-Student-creation cases cannot complete through C-002. | L2-C01 `WF-C01-06` §1.1 |
| Cancellation requires an authorized actor and a valid Admission state. | Admissions | Unauthorized or invalid-state cases do not transition to `Cancelled`. | L2-C01 `WF-C01-06` decision matrix |
| Cancellation may be initiated by the Applicant or institution subject to policy. | Admissions | Both initiation paths are supported without changing ownership. | L2-C01 Admission Cancellation workflow and role permissions |
| Cancelled Admissions remain preserved as institutional history. | Admissions | Cancellation does not delete the Admission Record. | L2-C01 Admission lifecycle principles and `Cancelled` definition |
| Readmission creates a new Admission Record where permitted. | Admissions | C-002 does not reopen the cancelled Admission Record. | L2-C01 canonical transition matrix / Readmission rule |
| Finance retains ownership of refunds, payments, ledger and accounting. | Finance | Admissions publishes the cancellation fact but does not perform financial settlement. | L2-C01 domain ownership boundaries |

L3 applies these upstream constraints and does not create a local rule namespace.

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Admission Record | `Admission Initiated`, `Documents Submitted`, `Verification Completed` | Authorized cancellation terminates the current Admission Record. | `Cancelled` | Admissions | L2-C01 §6.9, §6.9.1 and `WF-C01-06` |
| Admission Record | `Admission Approved`, `Financial Clearance` | L2-C01 is internally inconsistent on whether cancellation is permitted from these states; L3 makes no transition assertion. | **Unresolved upstream** | Admissions | L2-C01 §6.9 versus §6.9.1 |

`Cancelled` terminates the lifecycle of the current Admission Record. Where Readmission is allowed, a new Admission Record is created rather than reopening the cancelled one.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AdmissionCancelled` | Admissions / Admission Domain | Finance; Notification; Audit; Analytics as defined by L2-C01 | Represents the completed Admission cancellation. | L2-C01 `WF-C01-06`, §9 event catalogue and consumers |

### 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain business contract is materially required to reach the C-002 outcome. Downstream domains consume the completed `AdmissionCancelled` fact and retain ownership of their own consequences.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request own Admission cancellation | Applicant | Policy-controlled permission under L2-C01. | Admissions |
| Process / execute Admission cancellation | Admission Officer | Permitted by the L2-C01 role/permission model and governing institutional policy. | Admissions |
| Exercise institutional cancellation authority | Registrar | Permitted by the L2-C01 role/permission model and governing institutional policy. | Admissions |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty or override rule is established beyond the upstream authorization and valid-state requirements.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Cancellation must be attributable and auditable. | Admission Record history containing the governed cancellation decision and cancellation reason/basis required by the workflow. | Admissions |
| Lifecycle history must remain preserved. | Immutable Admission history showing the completed `Cancelled` outcome. | Admissions |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The actor is authorized and the Admission state is unambiguously eligible for cancellation.
- The required cancellation basis/reason is established.
- The Admission Record reaches `Cancelled`.
- No Student has been created from the cancelled Admission Record.
- Admission history is preserved.
- `AdmissionCancelled` is published by Admissions.

### 16.2 Postconditions

- The current Admission Record remains terminated in `Cancelled`.
- Any downstream financial or other consequences remain owned by their respective domains.
- A future Readmission, where permitted, uses a new Admission Record.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

**OI-C002-01 — Conflicting L2-C01 cancellation-state matrices.** L2-C01 §6.9 allows cancellation from `Admission Initiated`, `Documents Submitted`, and `Verification Completed` but does not allow `Admission Approved → Cancelled` or `Financial Clearance → Cancelled`. L2-C01 §6.9.1 explicitly allows both `Admission Approved → Cancelled` and `Financial Clearance → Cancelled`. L3 does not choose between these conflicting authoritative statements. **Impact:** cancellation from those two states cannot be asserted or locked in C-002 until L2-C01 is resolved by the appropriate architecture authority.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

Upstream governance must resolve `OI-C002-01`. Institution-specific routing among the already-defined Applicant, Admission Officer, and Registrar roles remains configuration-driven and does not create a new canonical role.
