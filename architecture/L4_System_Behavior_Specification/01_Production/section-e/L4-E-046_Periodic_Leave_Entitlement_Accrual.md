# Periodic Leave Entitlement Accrual

## 1. Specification Metadata

**Specification ID:** L4-E-046  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-046 — Periodic Leave Entitlement Accrual  
**Primary Business Owner:** Leave Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-046; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall evaluate an applicable accrual trigger under the active leave policy and, for eligible employees, establish leave entitlement through an authoritative Leave Ledger transaction followed by balance projection update.

## 3. Required Business Context

- An active leave policy is available.
- An authoritative accrual trigger is applicable.
- Employee eligibility can be evaluated from authoritative context.
- Leave Management owns accrual rules, ledger transactions and balance projections.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-046-BH-001 | The system shall identify the applicable accrual trigger and active leave policy before evaluating entitlement. | L3-E-046 §§7, 8, 9 |
| L4-E-046-BH-002 | The system shall evaluate employee eligibility under the active policy and determine the eligible population for the applicable accrual cycle. | L3-E-046 §§3, 9, 12 |
| L4-E-046-BH-003 | For each eligible employee, the system shall calculate the applicable accrual under the configured authoritative rules. | L3-E-046 §§2, 9, 12 |
| L4-E-046-BH-004 | The system shall create the authoritative accrual transaction in the Leave Ledger before changing the represented balance. | L3-E-046 §§3, 9, 12 |
| L4-E-046-BH-005 | The system shall update the leave balance projection from the recorded ledger transaction. | L3-E-046 §§3, 9, 12, 16 |
| L4-E-046-BH-006 | The system shall preserve accrual trigger, policy/rule selection, eligibility evaluation, ledger transaction and balance outcome as traceable business evidence. | L3-E-046 §§12, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-046-BH-007 | After the balance projection is updated, the system shall publish the accrual business event required by the authoritative L2 workflow, but L4 shall not invent its canonical identifier because the Locked parent does not expose one. | L3-E-046 §§9, 14.1, 17.1 |
| L4-E-046-BH-008 | The accrued entitlement and ledger history shall remain owned by Leave Management; downstream leave processing may consume the outcome without ownership transfer. | L3-E-046 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- If an employee is ineligible, the system shall establish no accrual entitlement for that employee for the applicable cycle.
- If the accrual rule does not apply, the system shall create no accrual transaction for that rule/employee.
- If a later correction is required, the historical accrual transaction shall remain immutable and correction shall occur through the governed adjustment/reversal process.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of the accrual trigger, active policy and rule selection, employee eligibility result, authoritative ledger transaction and resulting balance projection.

## 8. Observable Completion Conditions

Completion requires the applicable accrual rules and eligibility to be evaluated, the accrual to be recorded in the Leave Ledger where applicable, the balance projection to reflect the ledger transaction, and the required accrual business event to be published without inventing an unsupported canonical identifier.

## 9. Traceability & Provenance

**Parent:** L3-E-046 — Periodic Leave Entitlement Accrual  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

The Locked parent confirms that L2-E05 requires an accrual event after balance projection update but does not expose its canonical identifier/name. That identifier remains an upstream architecture question before approval/lock. Technical implementation details remain outside L4.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
