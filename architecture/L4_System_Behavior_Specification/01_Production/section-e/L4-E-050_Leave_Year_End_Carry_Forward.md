# Leave Year End Carry Forward

## 1. Specification Metadata

**Specification ID:** L4-E-050  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-050 — Leave Year End Carry Forward  
**Primary Business Owner:** Leave Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-050; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall process leave-year closure by evaluating the active carry-forward rules, recording eligible carry-forward and ineligible expiry outcomes through the authoritative Leave Ledger, and updating the balance projection from those transactions.

## 3. Required Business Context

- Leave-year closure has been reached for the applicable leave-policy period.
- The active leave policy and carry-forward/expiry rules are available.
- Required employee and leave-balance context is available.
- Leave Management remains canonical owner of carry-forward, expiry, ledger transactions and balance projection.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-050-BH-001 | The system shall establish the applicable leave-year closure context and active leave policy. | L3-E-050 §§7–9 |
| L4-E-050-BH-002 | The system shall evaluate the active carry-forward rules against the applicable unused leave balance. | L3-E-050 §§3, 9, 12 |
| L4-E-050-BH-003 | For eligible balance, the system shall create the governed carry-forward transaction in the Leave Ledger. | L3-E-050 §§3, 9, 12 |
| L4-E-050-BH-004 | For ineligible remaining balance, the system shall apply the governed expiry outcome through the applicable ledger transaction. | L3-E-050 §§3, 9–12 |
| L4-E-050-BH-005 | Where only part of the balance is eligible, the system shall apply the active policy rule to separate the carry-forward and expiry outcomes without directly mutating the balance. | L3-E-050 §§10–12 |
| L4-E-050-BH-006 | The system shall update the Leave Balance projection from the resulting authoritative ledger transactions. | L3-E-050 §§3, 9, 12–13 |
| L4-E-050-BH-007 | The system shall preserve year-end rule evaluation and the resulting carry-forward/expiry ledger history as auditable business evidence. | L3-E-050 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-050-BH-008 | The system shall not invent a canonical carry-forward event identifier because the Locked parent states that the accessible authoritative evidence does not expose its exact identifier/name. | L3-E-050 §§14.1, 17.1 |
| L4-E-050-BH-009 | Resulting Leave Management facts may be made available to downstream leave consumers, including Payroll and Analytics where governed, without transfer of Leave Management ownership. | L3-E-050 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If an employee or balance is not eligible for carry-forward, the system shall establish the applicable expiry outcome rather than creating a carry-forward transaction.
- If the active policy excludes part of the balance, the system shall carry forward only the eligible portion and apply the applicable expiry rule to the remainder.
- If required policy or leave context is unavailable, the system shall not establish a valid year-end carry-forward/expiry outcome until the required business context is available.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve the leave-year context, applicable policy/version, eligibility evaluation, carry-forward transaction, expiry transaction and resulting balance projection as attributable business evidence.

## 8. Observable Completion Conditions

Completion requires year-end context and active rules to be established, eligible balance to be recorded through carry-forward transaction, ineligible balance to be recorded through expiry transaction where applicable, and the Leave Balance projection to reflect the resulting Leave Ledger position.

## 9. Traceability & Provenance

**Parent:** L3-E-050 — Leave Year End Carry Forward  
**Primary verification source:** Authoritative People & Workforce L2-E05 Leave Management workflow architecture.

## 10. Open Issues / Deferred Design

The canonical identifier/name of the carry-forward event remains unresolved upstream as recorded by L3-E-050; L4 does not invent one. Technical APIs, persistence schemas, UI, transport and retry mechanics are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
