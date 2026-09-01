# L4-E-050 — Leave Year-End Carry Forward

**Scenario ID:** L4-E-050  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-050 — Leave Year-End Carry Forward  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-050. Leave Management remains the canonical owner of leave balance, ledger, carry-forward, and expiry outcomes.

## 2. System Behavior Purpose
Specify observable behavior for processing eligible unused leave at leave-year closure under active carry-forward and expiry rules.

## 3. System Behavior Outcome
Eligible unused leave is carried forward through an authoritative ledger transaction, while ineligible balance is handled through the applicable expiry outcome and the resulting balance projection is updated.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
The applicable leave-year closure is reached for the governed leave policy period.
### 4.2 End Boundary
Carry-forward and expiry outcomes are recorded in the Leave Ledger and the resulting balance projection is established.
### 4.3 In Scope
Year-end context establishment, active-policy evaluation, carry-forward eligibility, carry-forward transaction, expiry transaction, ledger update, balance projection.
### 4.4 Out of Scope
Leave accrual, leave request approval, normal leave consumption, cancellation, manual adjustment, and technical implementation.

## 5. Ownership and Participating Domains
Leave Management owns the carry-forward/expiry decision, ledger transactions, and balance projection. External domains may consume resulting leave facts without acquiring ownership.

## 6. Business Actors, Roles and Responsibilities
Leave Management evaluates applicable rules and records authoritative outcomes. Employee/workforce context supplies relevant business facts where required.

## 7. Preconditions and Entry Criteria
- Applicable leave policy is active.
- Relevant employee/leave context is available.
- Leave-year closure condition has been reached.

## 8. Trigger
Leave-year closure is reached for the applicable leave policy period.

## 9. Main System Behavior
1. Establish the applicable year-end and policy context.
2. Evaluate carry-forward eligibility under the active policy.
3. Record eligible balance through the governed carry-forward transaction.
4. Record ineligible remaining balance through the applicable expiry outcome.
5. Update the balance projection from the authoritative ledger position.
6. Publish the applicable business outcome where defined by upstream architecture.

## 10. Alternate System Behaviors
- Eligible balance follows the carry-forward rule.
- Ineligible balance follows the applicable expiry rule.
- Partial eligibility results in separate governed carry-forward and expiry outcomes where required.

## 11. Failure, Interruption and Business Recovery
Where an employee or balance is not eligible for carry-forward, the applicable expiry outcome is recorded. Policy exclusions are handled according to the active rule set. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- Carry-forward is governed by the active leave policy.
- Leave Ledger is the authoritative transaction history.
- Balance is represented from ledger transactions.
- Historical transactions remain immutable.
- Carry-forward and expiry remain distinct business outcomes.

## 13. State and Lifecycle Impact
The scenario changes the leave balance position through authoritative carry-forward/expiry ledger transactions. No new Leave Request lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
The locked upstream workflow defines carry-forward/expiry processing but does not expose a canonical carry-forward event identifier. No event name is invented.

## 15. Business Authorization, Approval and Audit Obligations
Year-end rule evaluation, eligibility result, ledger transactions, and resulting balance outcome remain traceable and auditable.

## 16. Completion Criteria and Postconditions
- Year-end context established.
- Carry-forward rules evaluated.
- Eligible balance recorded through carry-forward transaction.
- Ineligible balance handled through expiry transaction.
- Balance projection reflects resulting authoritative ledger position.

## 17. Open Issues, Assumptions and Deferred Questions
Canonical carry-forward event identifier requires confirmation before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft.
