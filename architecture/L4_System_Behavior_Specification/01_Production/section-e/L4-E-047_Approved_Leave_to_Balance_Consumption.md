# Approved Leave to Balance Consumption

## 1. Specification Metadata

**Specification ID:** L4-E-047  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-047 — Approved Leave to Balance Consumption  
**Primary Business Owner:** Leave Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-047; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall convert a valid approved-leave reservation into authoritative leave consumption at the applicable consumption point, preserving Leave Management ownership of the Leave Request, Leave Ledger and balance projection.

## 3. Required Business Context

- Leave has been approved.
- A valid balance reservation exists.
- The applicable consumption date has been reached.
- Leave Management owns leave consumption, ledger transactions and balance projection.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-047-BH-001 | The system shall confirm that approved leave has reached the applicable consumption point and that a valid balance reservation exists. | L3-E-047 §§7, 8, 9 |
| L4-E-047-BH-002 | The system shall convert the valid reservation into leave consumption under Leave Management ownership. | L3-E-047 §§2, 3, 9, 12 |
| L4-E-047-BH-003 | The system shall create the authoritative consumption transaction in the Leave Ledger before changing the represented balance. | L3-E-047 §§3, 9, 12 |
| L4-E-047-BH-004 | The system shall update the leave balance projection from the recorded consumption ledger transaction. | L3-E-047 §§3, 9, 12, 16 |
| L4-E-047-BH-005 | The system shall transition the Leave Request Aggregate from `Reserved` to `Consumed` when the governed consumption conditions are satisfied. | L3-E-047 §13 |
| L4-E-047-BH-006 | The system shall preserve reservation validation, consumption decision, ledger transaction and balance outcome as traceable business evidence. | L3-E-047 §§12, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-047-BH-007 | When leave consumption is established, the system shall publish `LeaveConsumed`. | L3-E-047 §§14.1, 16 |
| L4-E-047-BH-008 | After the balance projection is updated, the system shall publish `EmployeeLeaveBalanceUpdated`. | L3-E-047 §§14.1, 16 |
| L4-E-047-BH-009 | The consumption outcome may be made available to Attendance and other authoritative downstream participants without transferring ownership of the leave fact from Leave Management. | L3-E-047 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If no valid reservation exists, the system shall not establish consumption and shall preserve the existing authoritative leave state.
- If governed consumption conditions are not met, the leave shall remain unconsumed until the applicable business point is reached.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of reservation validation, the `Reserved` to `Consumed` transition, the consumption ledger transaction, resulting balance projection and published business events.

## 8. Observable Completion Conditions

Completion requires approved leave to reach its consumption point, a valid reservation to be converted into consumption, the consumption ledger transaction to be recorded, the balance projection to be updated, and both `LeaveConsumed` and `EmployeeLeaveBalanceUpdated` to be published.

## 9. Traceability & Provenance

**Parent:** L3-E-047 — Approved Leave to Balance Consumption  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

No material ownership, lifecycle or event uncertainty is identified by the Locked parent. Technical implementation details remain outside L4.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
