# L4-E-048 — Approved or Reserved Leave Cancellation

**Scenario ID:** L4-E-048  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-048 — Approved or Reserved Leave Cancellation  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-048. Leave Management owns cancellation and related leave-ledger outcomes.

## 2. System Behavior Purpose
Specify observable behavior for validating and completing cancellation of approved or reserved leave while preserving authoritative history.

## 3. System Behavior Outcome
Authorized cancellation is established, applicable reservation/consumption effects are reversed through governed ledger behavior, and the leave request outcome is recorded.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
A cancellation request is raised for an existing approved or reserved leave.
### 4.2 End Boundary
Cancellation and required reversal/ledger outcomes are recorded.
### 4.3 In Scope
Cancellation validation, policy checks, reversal behavior, ledger entry, request outcome, and business-event publication.
### 4.4 Out of Scope
Initial approval, accrual, carry-forward, independent adjustment, and technical implementation.

## 5. Ownership and Participating Domains
Leave Management owns cancellation, Leave Request lifecycle, policy restrictions, and reversal ledger outcomes.

## 6. Business Actors, Roles and Responsibilities
Employee or authorized business user raises cancellation; applicable approver participates where required; Leave Management governs and records the cancellation outcome.

## 7. Preconditions and Entry Criteria
- Leave request exists.
- Cancellation is permitted by policy.
- Current leave state is cancellable.

## 8. Trigger
An employee or authorized user requests cancellation.

## 9. Main System Behavior
1. Validate cancellation request and policy.
2. Determine whether the current leave state is cancellable.
3. Reverse reservation or consumption as required by the authoritative path.
4. Record required reversal ledger transaction.
5. Record leave-request cancellation outcome.
6. Publish applicable cancellation and balance-update outcomes.

## 10. Alternate System Behaviors
- Policy restrictions may reject cancellation.
- Reservation reversal and consumption reversal follow the applicable authoritative path.

## 11. Failure, Interruption and Business Recovery
Invalid or prohibited cancellation leaves the existing authoritative leave state unchanged. Corrections preserve original ledger history and use governed reversal transactions. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- Cancellation is governed by leave policy.
- Leave Request Aggregate owns cancellation lifecycle.
- Balance-affecting corrections use ledger transactions.
- Historical ledger transactions remain immutable.

## 13. State and Lifecycle Impact
The Leave Request reaches the authoritative cancellation outcome from an approved or reserved position. No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
`EmployeeLeaveCancelled` is published for the cancellation outcome. `EmployeeLeaveBalanceUpdated` is published when represented balance changes. Leave Management remains the canonical publisher.

## 15. Business Authorization, Approval and Audit Obligations
Cancellation request, policy decision, reversal activity, and resulting ledger evidence remain attributable and traceable.

## 16. Completion Criteria and Postconditions
- Cancellation validated.
- Policy restrictions evaluated.
- Required reversal recorded.
- Cancellation outcome recorded.
- Applicable events published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, lifecycle, or outcome was identified in the locked L3 parent.

**Authoring Classification:** PASS — source-grounded Draft.
