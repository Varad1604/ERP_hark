# L4-E-046 — Periodic Leave Entitlement Accrual

**Scenario ID:** L4-E-046  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-046 — Periodic Leave Entitlement Accrual  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-046. Leave Management remains the canonical owner of entitlement accrual, leave ledger transactions, and balance projection.

## 2. System Behavior Purpose
Specify observable behavior for applying an applicable leave accrual cycle using the active policy and authoritative eligibility rules.

## 3. System Behavior Outcome
The applicable accrual is recorded in the Leave Ledger and the resulting balance representation is updated.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An applicable scheduled accrual cycle or policy-defined accrual trigger is reached.
### 4.2 End Boundary
Eligible accrual is recorded as an authoritative ledger transaction and the balance projection reflects it.
### 4.3 In Scope
Accrual trigger recognition, active-policy evaluation, eligibility determination, accrual calculation, ledger transaction, balance projection, business outcome publication.
### 4.4 Out of Scope
Leave requests, leave approval, leave consumption, cancellation, carry-forward, manual adjustment, and technical implementation.

## 5. Ownership and Participating Domains
Leave Management owns accrual, entitlement, ledger history, and balance projection. External domains supply authoritative context without taking ownership.

## 6. Business Actors, Roles and Responsibilities
Leave Management executes the accrual business behavior; authoritative employee context supplies eligibility context; policy/rule authority supplies applicable rules.

## 7. Preconditions and Entry Criteria
- Active leave policy exists.
- Accrual trigger applies.
- Employee eligibility can be evaluated.

## 8. Trigger
A scheduled accrual cycle or other authoritative accrual trigger occurs.

## 9. Main System Behavior
1. Establish the applicable accrual context and policy version.
2. Evaluate employee eligibility.
3. Determine the governed accrual outcome.
4. Record eligible accrual through the Leave Ledger.
5. Update the balance projection from the ledger transaction.
6. Publish the applicable accrual business outcome where defined by authoritative upstream architecture.

## 10. Alternate System Behaviors
- Ineligible employees receive no accrual outcome for the applicable cycle.
- Policy-defined exclusions result in governed non-accrual outcomes.

## 11. Failure, Interruption and Business Recovery
Ineligibility or policy exclusion does not create an entitlement transaction. Subsequent correction uses the governed adjustment/reversal process; historical transactions remain immutable. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns accrual.
- Leave Ledger is the authoritative transaction history.
- Balance is a projection from ledger transactions.
- Historical transactions are immutable.
- Effective-dated policies and rules govern applicable accrual outcomes.

## 13. State and Lifecycle Impact
The entitlement/balance position is advanced through an authoritative ledger transaction and resulting balance projection. No new Leave Request lifecycle state is created.

## 14. Business Events and Cross-Domain Collaboration
The L2 workflow requires an accrual outcome publication, but the accessible locked source does not expose a canonical event identifier. No event name is invented.

## 15. Business Authorization, Approval and Audit Obligations
Accrual trigger, policy/rule selection, eligibility evaluation, resulting ledger transaction, and balance outcome remain traceable.

## 16. Completion Criteria and Postconditions
- Accrual context established.
- Eligibility evaluated.
- Applicable accrual recorded in the ledger where eligible.
- Balance projection updated.
- Applicable business outcome published where defined.

## 17. Open Issues, Assumptions and Deferred Questions
Canonical accrual-event identifier requires confirmation before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft.
