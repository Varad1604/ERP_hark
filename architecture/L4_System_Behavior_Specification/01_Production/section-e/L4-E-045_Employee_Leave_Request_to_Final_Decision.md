# L4-E-045 — Employee Leave Request to Final Decision

**Scenario ID:** L4-E-045  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-045 — Employee Leave Request to Final Decision  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-045. Leave Management owns the Leave Request Aggregate and final leave decision.

## 2. System Behavior Purpose
Specify observable behavior for processing an employee leave request through validation, eligibility, balance checking, approval routing, and final decision.

## 3. System Behavior Outcome
The Leave Request reaches an authoritative `Approved` or `Rejected` outcome and the matching canonical decision event is published.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An employee submits a leave request for an applicable leave type and period.
### 4.2 End Boundary
The Leave Request reaches `Approved` or `Rejected` and the matching canonical decision event is published.
### 4.3 In Scope
Request submission, validation, eligibility evaluation, balance check, approval routing, final approval/rejection, decision-event publication.
### 4.4 Out of Scope
Reservation, consumption, ledger processing, cancellation after approval, accrual, carry-forward, adjustment, APIs, UI, database design, queues, and infrastructure.

## 5. Ownership and Participating Domains
Leave Management owns the Leave Request Aggregate, decision lifecycle, and decision-event publication. Authoritative employee context is consumed but not re-owned.

## 6. Business Actors, Roles and Responsibilities
Employee submits the request. Manager/Approver reviews and establishes the approval decision. Leave Management governs the request lifecycle and outcome.

## 7. Preconditions and Entry Criteria
- Employee exists in authoritative context.
- Active leave policy exists.
- Leave type is available.
- Eligibility can be established.
- Approval route is determinable.

## 8. Trigger
Employee submits a leave request for decision.

## 9. Main System Behavior
1. Establish the submitted leave request.
2. Validate request and policy conditions.
3. Evaluate employee eligibility.
4. Check available leave balance where required.
5. Route the request to the assigned Manager/Approver.
6. Record approval or rejection.
7. Transition the request to `Approved` or `Rejected`.
8. Publish `EmployeeLeaveApproved` or `EmployeeLeaveRejected` matching the final state.

## 10. Alternate System Behaviors
- Validation or eligibility failure prevents normal approval progression.
- Insufficient balance prevents normal approval where balance is required.
- Pending approval leaves the request in its governed approval position.

## 11. Failure, Interruption and Business Recovery
Invalid requests remain outside normal approval progression until corrected or rejected through the governed business path. Incomplete approval remains pending/escalated under the authoritative workflow. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns leave decisions.
- Leave Request Aggregate owns approval state.
- Eligibility and balance checks precede normal approval progression.
- Employee cannot approve own request.
- Decision history remains attributable and traceable.

## 13. State and Lifecycle Impact
Draft → Submitted → Validated → PendingApproval → Approved/Rejected. Subsequent Reserved/Consumed/Closed processing is outside this scenario.

## 14. Business Events and Cross-Domain Collaboration
- `EmployeeLeaveRequested` represents request submission.
- `EmployeeLeaveApproved` is published for an Approved outcome.
- `EmployeeLeaveRejected` is published for a Rejected outcome.
Leave Management remains the canonical publisher.

## 15. Business Authorization, Approval and Audit Obligations
Approval/rejection authority is attributable to the Manager/Approver. Request validation, eligibility, balance evaluation, route, and final decision remain traceable in the Leave Request history.

## 16. Completion Criteria and Postconditions
- Request validated.
- Eligibility and balance evaluated.
- Approval route reaches decision.
- Request reaches `Approved` or `Rejected`.
- Matching canonical event is published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, lifecycle, or outcome was identified in the locked L3 parent.

**Authoring Classification:** PASS — source-grounded Draft.
