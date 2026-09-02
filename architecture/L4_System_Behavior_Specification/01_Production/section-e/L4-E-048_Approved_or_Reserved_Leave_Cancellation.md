# Approved or Reserved Leave Cancellation

## 1. Specification Metadata

**Specification ID:** L4-E-048  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-048 — Approved or Reserved Leave Cancellation  
**Primary Business Owner:** Leave Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-048; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall validate and complete an authorized cancellation of approved or reserved leave while preserving the authoritative Leave Request lifecycle, immutable Leave Ledger history and Leave Management ownership.

## 3. Required Business Context

- The Leave Request exists.
- Applicable cancellation policy permits evaluation of the requested action.
- The Leave Request is in a business state for which cancellation is allowed.
- Leave Management owns cancellation, Leave Request lifecycle and balance-affecting reversal history.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-048-BH-001 | The system shall validate the cancellation request against the existing Leave Request and applicable cancellation policy. | L3-E-048 §§7, 8, 9 |
| L4-E-048-BH-002 | The system shall determine whether the current leave position permits cancellation before establishing a cancellation outcome. | L3-E-048 §§3, 9, 10, 12 |
| L4-E-048-BH-003 | Where cancellation requires reversal of a reservation or prior consumption, the system shall apply the authoritative cancellation path without directly altering historical ledger transactions. | L3-E-048 §§3, 9, 11, 12 |
| L4-E-048-BH-004 | The system shall create the required reversal ledger entry where a balance-affecting correction is required. | L3-E-048 §§3, 9, 12, 16 |
| L4-E-048-BH-005 | The system shall record the cancellation outcome on the Leave Request Aggregate using the authoritative lifecycle without inventing a new cancellation state. | L3-E-048 §§9, 12, 13 |
| L4-E-048-BH-006 | The system shall preserve cancellation request, policy decision, reversal activity, ledger evidence and resulting Leave Request history. | L3-E-048 §§12, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-048-BH-007 | When cancellation is established, the system shall publish `EmployeeLeaveCancelled`. | L3-E-048 §§14.1, 16 |
| L4-E-048-BH-008 | When the cancellation changes the represented leave balance, the system shall publish `EmployeeLeaveBalanceUpdated` after the governed ledger reversal and balance outcome are established. | L3-E-048 §§14.1, 16 |
| L4-E-048-BH-009 | Cancellation outcomes may be consumed by downstream participants such as Payroll, Attendance and Notification without transferring ownership of the leave fact from Leave Management. | L3-E-048 §14.2 |

## 6. Failure / Rejection / Continuation

- If cancellation policy does not permit the action, the system shall reject the cancellation and preserve the existing authoritative leave state and history.
- If the cancellation targets an invalid or non-cancellable state, the system shall establish no cancellation outcome and shall follow the applicable governed leave workflow.
- If a prior consumption requires correction, the original ledger transaction shall remain immutable and the correction shall be represented through a reversal ledger entry.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of cancellation initiation, policy evaluation, any required approval/authority, reversal action, immutable original ledger history, reversal ledger entry and resulting Leave Request outcome.

## 8. Observable Completion Conditions

Completion requires the cancellation request and policy restrictions to be evaluated, any required reservation or consumption reversal and ledger entry to be recorded, the Leave Request cancellation outcome to be established, `EmployeeLeaveCancelled` to be published, and `EmployeeLeaveBalanceUpdated` to be published where the represented balance changes.

## 9. Traceability & Provenance

**Parent:** L3-E-048 — Approved or Reserved Leave Cancellation  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

No material ownership, lifecycle or event uncertainty is identified by the Locked parent. Technical implementation details remain outside L4.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
