# Employee Leave Request to Final Decision

**Scenario ID:** L3-E-045  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Varad  
**Last Updated:** 2026-08-23  
**Depends On:** `L2-E05_Leave_Management_Workflows.md`; authoritative employee context and leave-policy governance  
**Referenced By:** None yet

## 1. Scenario Metadata

Frozen Stage 3 semantics are preserved: `L3-E-045`, **Employee Leave Request to Final Decision**, Primary Owner **Leave Management**, Participating Domains **E**, Scenario Type **Critical**, Priority **P0**.

## 2. Scenario Purpose

Process an employee leave request through validation, eligibility, balance checking and governed approval so that the institution reaches a final approved or rejected leave decision.

## 3. Business Outcome

A final authoritative leave decision is established for the Leave Request Aggregate: either `Approved` or `Rejected`.

The matching canonical decision event is published by the Leave Request Aggregate.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when an Employee creates/submits a leave request for an applicable leave type and period.

### 4.2 End Boundary

The scenario ends when the Leave Request has reached the authoritative approval decision state (`Approved` or `Rejected`) and the corresponding canonical decision event has been published.

### 4.3 In Scope

- Leave request submission.
- Request validation.
- Eligibility evaluation under the active leave policy.
- Leave-balance availability check.
- Approval routing to the assigned Manager / Approver.
- Final approval or rejection decision.
- Publication of the matching canonical decision event.

### 4.4 Out of Scope

- Post-approval balance reservation.
- Leave consumption.
- Leave Ledger processing.
- Cancellation after approval.
- Leave accrual, carry forward, adjustment, or encashment.
- Technical authorization, APIs, database design, UI, queues, services, or infrastructure.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Leave Management / Section E | Owns the Leave Request Aggregate, request validation, eligibility evaluation, approval lifecycle, final leave decision, and decision-event publication. |

Authoritative employee context is consumed as an input; ownership of employee identity remains outside Leave Management.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Employee | Creates/submits the leave request and provides the requested leave type and period. | Initiates the request; cannot approve own leave request under L2-E05. |
| Manager / Approver | Reviews the routed leave request and approves or rejects it. | Authorized to approve/reject requests; cannot modify leave policy or adjust balances through this role. |

Leave Management remains the business-domain owner of the request lifecycle and decision; the domain itself is not treated as a business actor.

## 7. Preconditions and Entry Criteria

- The Employee exists in authoritative employee context.
- An active leave policy exists.
- The requested leave type is available under that policy.
- The Employee is eligible for the requested leave type.
- An approval route can be determined.

## 8. Trigger

The Employee submits a leave request for decision.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Submit the leave request with requested leave type and period. | Employee | Leave request enters the authoritative request lifecycle. |
| 2 | Validate the request and applicable policy conditions. | Leave Management | Request validation result is established. |
| 3 | Evaluate Employee eligibility for the requested leave. | Leave Management | Eligibility result is established. |
| 4 | Check available leave balance. | Leave Management | Balance availability is established for decision processing. |
| 5 | Route the validated request to the assigned Manager / Approver. | Leave Management | Request reaches the approval decision point. |
| 6 | Approve or reject the request. | Manager / Approver | Final decision is established. |
| 7 | Transition the Leave Request to `Approved` or `Rejected` and publish the corresponding decision event. | Leave Request Aggregate / Leave Management | Authoritative final-decision outcome is available to downstream consumers. |

The scenario stops at the final approval decision and does not continue into balance reservation, consumption or ledger processing.

## 10. Alternate Business Flows

No scenario-specific alternate business flow toward the same defined final decision outcome is required beyond the normal approved and rejected decision branches represented in the Main Business Flow.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Request validation fails | The request cannot proceed to normal approval. | Reject or follow the governed exception path defined by Leave Management policy. | Leave Management |
| Employee is not eligible | The request cannot proceed as an approvable request. | Reject or follow the governed exception path. | Leave Management |
| Available balance is insufficient | The request cannot proceed normally where balance is required. | Reject or follow the controlled exception path. | Leave Management |
| Approval is not completed | Final decision remains pending. | Continue through the authoritative approval/escalation workflow. | Leave Management / Manager / Approver |

No technical retries, rollback, queue handling, infrastructure recovery or implementation mechanics are defined at L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Leave Management owns leave decisions. | Leave Management | No external domain may independently establish the leave decision. | L2-E05 §5 ownership boundaries |
| Leave Request Aggregate owns request lifecycle and approval state. | Leave Management | `Approved` / `Rejected` are established only through the Leave Request lifecycle. | L2-E05 §6.4, §6.7 |
| Active leave policy and available leave type are required for normal processing. | Leave Management | Missing policy/type prevents normal approval progression. | L2-E05 Leave Request workflow / business rules |
| Eligibility is evaluated before the approval decision. | Leave Management | Ineligible requests cannot proceed as normal approved requests. | L2-E05 §7.1 |
| Balance availability is checked before approval progression. | Leave Management | Balance constraints affect whether the request may proceed normally. | L2-E05 §7.1 |
| Employee cannot approve own leave request. | Leave Management | Approval authority remains with Manager / Approver. | L2-E05 authorization role model |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Leave Request Aggregate | `Draft` | Employee submission starts the governed request lifecycle. | `Submitted` | Leave Request Aggregate / Leave Management | L2-E05 §6.4 lifecycle |
| Leave Request Aggregate | `Submitted` | Request validation completes. | `Validated` | Leave Request Aggregate / Leave Management | L2-E05 §6.4 lifecycle |
| Leave Request Aggregate | `Validated` | Approval routing/decision processing begins. | `PendingApproval` | Leave Request Aggregate / Leave Management | L2-E05 §6.4 lifecycle |
| Leave Request Aggregate | `PendingApproval` | Manager / Approver makes the final decision. | `Approved` or `Rejected` | Leave Request Aggregate / Leave Management | L2-E05 §6.4; approval workflow |

Post-approval `Approved → Reserved → Consumed → Closed` processing is authoritative L2-E05 behaviour but is outside this scenario's end boundary. `Rejected → Closed` is also outside the final-decision boundary once rejection has been established.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `EmployeeLeaveRequested` | Leave Request Aggregate | Interested Leave Management/downstream consumers | Represents request submission. | L2-E05 event catalogue |
| `EmployeeLeaveApproved` | Leave Request Aggregate | Notification, Attendance, Payroll, Analytics and other consumers defined by L2-E05 | Represents completed approval. | L2-E05 event catalogue / integrations |
| `EmployeeLeaveRejected` | Leave Request Aggregate | Notification and other consumers defined by L2-E05 | Represents completed rejection. | L2-E05 event catalogue |

The decision event at scenario completion must match the actual final state: `Approved` → `EmployeeLeaveApproved`; `Rejected` → `EmployeeLeaveRejected`.

### 14.2 Business Contracts and Cross-Domain Hand-offs

The scenario consumes authoritative Employee context as an upstream business fact. No synchronous cross-domain hand-off is required to establish the final leave decision beyond those already governed by L2-E05.

Subsequent Attendance, Payroll, Notification or Analytics reactions consume Leave Management facts without acquiring ownership of the Leave Request or decision.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Create / submit own leave request | Employee | May initiate own request; cannot approve requests. | Leave Management |
| Approve leave request | Manager / Approver | L2-E05 permits approval of routed requests. | Leave Management |
| Reject leave request | Manager / Approver | L2-E05 permits rejection of routed requests. | Leave Management |

### 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Employee request versus approval decision | Employee; Manager / Approver | Employee initiates the request but approval/rejection authority belongs to the Manager / Approver role. |

No additional override or separation-of-duty rule is introduced at L3.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Final leave decision must be attributable. | Leave Request history showing the responsible Manager / Approver and the final decision. | Leave Management |
| Policy/eligibility evaluation must remain traceable. | Preserved Leave Request lifecycle and decision history sufficient to establish the governed decision. | Leave Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The Leave Request has been validated.
- Employee eligibility and balance availability have been evaluated.
- The applicable approval route has reached a decision.
- The Leave Request reaches `Approved` or `Rejected`.
- `EmployeeLeaveApproved` is published for `Approved`, or `EmployeeLeaveRejected` is published for `Rejected`.

### 16.2 Postconditions

- An approved request may continue into balance reservation and later Leave Management lifecycle processing outside this scenario.
- A rejected request has reached its final decision boundary; later closure remains governed by L2-E05 outside this scenario.
- Downstream domains may consume the decision event without re-owning the Leave Request.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None material identified for this scenario from the current locked L2-E05 evidence.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.
