# Governed Leave Balance Adjustment

## 1. Specification Metadata

**Specification ID:** L4-E-049  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-049 — Governed Leave Balance Adjustment  
**Primary Business Owner:** Leave Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-049; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall process an authorized leave-balance correction through governed approval and Leave Ledger transactions while preserving immutable history. The Leave Balance representation shall change only as a projection of the resulting authoritative ledger position.

## 3. Required Business Context

- An authorized leave-balance adjustment request exists.
- Applicable leave policy, employee and leave context are available.
- The business reason and required approval authority can be established.
- Leave Management remains canonical owner of the Leave Ledger and balance outcome.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-049-BH-001 | The system shall receive the adjustment request and validate the authority under which the adjustment is requested. | L3-E-049 §§7–9, 11 |
| L4-E-049-BH-002 | The system shall record and review the business reason and applicable leave context before an adjustment decision is established. | L3-E-049 §§9, 15 |
| L4-E-049-BH-003 | The system shall obtain the required business approval and record the resulting adjustment decision. | L3-E-049 §§9–12, 15 |
| L4-E-049-BH-004 | Where an existing balance-affecting transaction requires correction, the system shall create the required reversal transaction without changing the original historical transaction. | L3-E-049 §§9, 11–12 |
| L4-E-049-BH-005 | When approved, the system shall create the governed correction/adjustment transaction in the authoritative Leave Ledger. | L3-E-049 §§3, 9, 12 |
| L4-E-049-BH-006 | The system shall update the Leave Balance projection from the resulting authoritative ledger position rather than directly mutating the balance. | L3-E-049 §§3, 9, 12–13 |
| L4-E-049-BH-007 | The system shall preserve the adjustment request, business reason, approval decision, reversal/correction transactions and resulting balance history. | L3-E-049 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-049-BH-008 | When the adjustment changes the represented leave balance, the system shall publish the canonical **EmployeeLeaveBalanceUpdated** event under Leave Management ownership. | L3-E-049 §14.1 |
| L4-E-049-BH-009 | The system shall not invent a canonical identifier for the separate adjustment event because the Locked parent states that its exact identifier/name is not exposed in the accessible authoritative evidence. | L3-E-049 §§14.1, 17.1 |
| L4-E-049-BH-010 | Corrected Leave Management facts may be consumed by downstream Payroll, Analytics, Notification and Audit participants without transfer of Leave Management ownership. | L3-E-049 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If authority validation fails, the system shall not apply an adjustment and shall preserve the existing authoritative leave position.
- If the adjustment is rejected, the system shall record the rejected decision and shall not create corrective ledger transactions.
- If an existing transaction requires correction, the original transaction shall remain immutable and the correction shall use the governed reversal/correction path.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of adjustment authority, business reason, approval decision, affected prior transaction where applicable, reversal/correction entries and resulting Leave Balance projection.

## 8. Observable Completion Conditions

Completion requires authority and business reason to be validated, required approval to be recorded, applicable reversal/correction transactions to be recorded in the Leave Ledger, and the Leave Balance projection to reflect the authoritative ledger outcome. **EmployeeLeaveBalanceUpdated** is published when the represented balance changes.

## 9. Traceability & Provenance

**Parent:** L3-E-049 — Governed Leave Balance Adjustment  
**Primary verification source:** Authoritative People & Workforce L2-E05 Leave Management workflow architecture.

## 10. Open Issues / Deferred Design

The canonical identifier/name of the separate adjustment event remains unresolved upstream as recorded by L3-E-049; L4 does not invent one. Technical APIs, persistence schemas, UI, transport and retry mechanics are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
