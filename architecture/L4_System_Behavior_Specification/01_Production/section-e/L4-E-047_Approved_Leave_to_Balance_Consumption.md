# L4-E-047 — Approved Leave to Balance Consumption

**Scenario ID:** L4-E-047  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-047 — Approved Leave to Balance Consumption  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-047. Leave Management owns leave consumption and resulting ledger/balance outcomes.

## 2. System Behavior Purpose
Specify observable behavior for converting approved leave reservation into authoritative leave consumption.

## 3. System Behavior Outcome
Approved leave is consumed at the applicable consumption point, the Leave Ledger records the consumption, the balance projection is updated, and the applicable business outcome is published.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
Approved leave reaches its consumption date with a valid reservation.
### 4.2 End Boundary
Consumption and resulting balance outcome are recorded through authoritative Leave Management processing.
### 4.3 In Scope
Consumption eligibility, reservation-to-consumption transition, ledger transaction, balance projection, consumption publication.
### 4.4 Out of Scope
Initial request approval, policy evaluation, cancellation, accrual, manual adjustment, and technical implementation.

## 5. Ownership and Participating Domains
Leave Management owns leave consumption, ledger transaction, and balance projection. Attendance may participate as business context but does not own the leave fact.

## 6. Business Actors, Roles and Responsibilities
Leave Management executes the consumption behavior; participating domains supply or consume authoritative business context.

## 7. Preconditions and Entry Criteria
- Leave is approved.
- Valid balance reservation exists.
- Applicable consumption point has been reached.

## 8. Trigger
Approved leave reaches its consumption date.

## 9. Main System Behavior
1. Validate approved leave and reservation.
2. Confirm consumption eligibility.
3. Convert reservation to consumption.
4. Record the consumption transaction in the Leave Ledger.
5. Update the balance projection.
6. Publish the applicable consumption and balance-update business outcomes.

## 10. Alternate System Behaviors
- Missing reservation prevents consumption through this workflow.
- Consumption conditions not satisfied preserve the existing leave state until the governed business point.

## 11. Failure, Interruption and Business Recovery
No valid reservation or unmet consumption condition leaves the leave unconsumed. The existing authoritative leave state remains unchanged until the applicable business conditions are met. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- Approved leave requires a valid balance reservation before consumption.
- Leave Ledger is the authoritative record of balance-affecting transactions.
- Historical transactions are immutable.
- Balance projection follows ledger transactions.

## 13. State and Lifecycle Impact
Leave Request advances from `Reserved` to `Consumed` for the applicable leave outcome. No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
`LeaveConsumed` is the canonical consumption event and `EmployeeLeaveBalanceUpdated` represents the resulting balance update. Leave Management remains the publisher.

## 15. Business Authorization, Approval and Audit Obligations
Reservation validation, consumption decision, ledger transaction, and resulting balance update remain attributable and traceable.

## 16. Completion Criteria and Postconditions
- Approved leave reaches its consumption point.
- Reservation is converted to consumption.
- Consumption transaction recorded.
- Balance projection updated.
- Applicable consumption/balance events published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, lifecycle, or outcome was identified in the locked L3 parent.

**Authoring Classification:** PASS — source-grounded Draft.
