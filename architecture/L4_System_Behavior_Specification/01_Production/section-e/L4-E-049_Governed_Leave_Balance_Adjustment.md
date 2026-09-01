# L4-E-049 — Governed Leave Balance Adjustment

**Scenario ID:** L4-E-049  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-049 — Governed Leave Balance Adjustment  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-049. Leave Management is the canonical owner of leave balance adjustment and Leave Ledger history.

## 2. System Behavior Purpose
Specify observable behavior for processing an authorized correction to a leave balance while preserving immutable ledger history.

## 3. System Behavior Outcome
A governed adjustment is approved where required, recorded through authoritative ledger transactions, and reflected in the balance projection.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An authorized leave-balance adjustment request is raised.
### 4.2 End Boundary
Required approval and correction/reversal transactions are recorded and the balance projection reflects the authoritative ledger position.
### 4.3 In Scope
Adjustment validation, reason review, approval, reversal where required, correction transaction, ledger update, balance projection, outcome publication.
### 4.4 Out of Scope
Normal accrual, leave approval, leave consumption, carry-forward, cancellation, and technical implementation.

## 5. Ownership and Participating Domains
Leave Management owns the adjustment workflow, Leave Ledger transactions, and balance projection. External domains may consume the corrected leave fact without acquiring ownership.

## 6. Business Actors, Roles and Responsibilities
Leave Management processes the adjustment; authorized approving authority establishes the decision where required; relevant workforce context is supplied from authoritative sources.

## 7. Preconditions and Entry Criteria
- Relevant leave balance/context exists.
- Adjustment authority is determinable.
- Applicable policy/rules are available.
- Business reason for correction is established.

## 8. Trigger
An authorized balance adjustment request is raised.

## 9. Main System Behavior
1. Validate adjustment authority and scope.
2. Review the business reason and relevant leave context.
3. Obtain required approval.
4. Create a reversal transaction where an existing transaction requires correction.
5. Record the governed adjustment/correction transaction.
6. Update the balance projection from the ledger.
7. Publish the applicable adjustment/balance outcome.

## 10. Alternate System Behaviors
- Rejected adjustment produces no corrective ledger outcome.
- Corrections requiring reversal preserve the original transaction and offset it through governed ledger history.

## 11. Failure, Interruption and Business Recovery
Failed authority or rejection leaves the existing authoritative leave position unchanged. Existing transactions remain immutable; required correction uses reversal followed by the governed correction transaction. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns leave balance adjustment.
- Leave Ledger is authoritative transaction history.
- Historical transactions are immutable.
- Corrections use reversal/correction transactions.
- Balance projection is derived from ledger transactions.

## 13. State and Lifecycle Impact
The leave balance position changes only through authoritative ledger transactions. No new local lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
The upstream workflow defines an adjustment outcome and explicitly identifies `EmployeeLeaveBalanceUpdated` for balance updates. No adjustment-event name is invented where not exposed by authoritative evidence.

## 15. Business Authorization, Approval and Audit Obligations
Adjustment request, business reason, approval decision, reversal/correction entries, and resulting balance must remain attributable and traceable.

## 16. Completion Criteria and Postconditions
- Authority validated.
- Reason reviewed.
- Required approval completed.
- Required ledger reversal/correction recorded.
- Balance projection updated.
- Applicable business outcome published.

## 17. Open Issues, Assumptions and Deferred Questions
Canonical adjustment-event identifier requires confirmation before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft.
