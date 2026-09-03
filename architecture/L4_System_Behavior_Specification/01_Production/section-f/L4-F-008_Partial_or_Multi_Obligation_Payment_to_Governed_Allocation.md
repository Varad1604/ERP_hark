# Partial or Multi Obligation Payment to Governed Allocation

## 1. Specification Metadata

**Specification ID:** L4-F-008  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Finance  
**Realizes Scenario:** L3-F-008 — Partial or Multi Obligation Payment to Governed Allocation  
**Primary Business Owner:** Payment Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-F-008; L2-F02 — Payment Management; L2-F01 — Student Charge; L2-F07 — Accounts Receivable  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Establish governed Payment Allocation between one or more canonical Payments and one or more canonical Student Charges while preserving separate aggregate ownership and deriving settlement from allocation.

## 3. Required Business Context

- Canonical Payment eligible for allocation.
- One or more identifiable canonical Student Charges where allocation is intended.
- Authoritative allocation context sufficient to preserve monetary integrity.
- Unallocated-advance treatment where no valid charge can receive value.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-008-BH-001 | The system shall identify the canonical Payment and the applicable Student Charge or Charges before allocation is established. | L3-F-008 §§3, 7–9 |
| L4-F-008-BH-002 | The system shall establish Payment Allocation as the governed relationship between Payment and Student Charge without merging either aggregate. | L3-F-008 §§2–5, 9, 12 |
| L4-F-008-BH-003 | The system shall support the canonical one-to-one, one-to-many, many-to-one, and many-to-many allocation relationships when authoritative allocation context supports them. | L3-F-008 §§3, 9–10, 12 |
| L4-F-008-BH-004 | The system shall preserve monetary integrity and establish partial or complete allocation according to the actual available Payment value and obligation position. | L3-F-008 §§3, 9–12 |
| L4-F-008-BH-005 | The settlement outcome shall be derived from the authoritative Payment Allocation and shall not independently own Payment or Student Charge. | L3-F-008 §§2–3, 9, 12–13 |
| L4-F-008-BH-006 | Any remaining outstanding amount shall remain represented by the canonical Student Charge/receivable architecture rather than being silently balanced. | L3-F-008 §§3, 10–13 |
| L4-F-008-BH-007 | If no applicable Student Charge can yet receive the Payment, the system shall preserve the value as the canonical Unallocated Advance/Liability until a future Charge Allocation consumes it. | L3-F-008 §§10–12, 16 |
| L4-F-008-BH-008 | The system shall preserve Payment, Student Charge, allocation amount, settlement outcome and remaining-position lineage. | L3-F-008 §§9, 12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-008-BH-009 | Payment Management shall publish `PaymentAllocationCompleted` when the canonical allocation is established. | L3-F-008 §14.1 |
| L4-F-008-BH-010 | Payment Management shall publish `PaymentSettlementCompleted` only when the derived settlement condition is actually completed. | L3-F-008 §14.1 |
| L4-F-008-BH-011 | Accounts Receivable shall remain publisher of its own settlement events, including `ReceivableSettlementRecorded` and `ReceivableFullySettled`, when its aggregate reaches those conditions. | L3-F-008 §14.1 |
| L4-F-008-BH-012 | Student Charge context shall remain F01-owned and any Student Lifecycle context shall remain C-owned. | L3-F-008 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- No allocation shall be made to an inferred Student Charge.
- Insufficient value produces the governed partial-settlement outcome rather than an overstated settlement.
- Ambiguous multi-obligation context remains unresolved until authoritative allocation information is available.
- Aggregate ownership shall not be bypassed to force allocation.

## 7. Audit & Evidence Behavior

Payment identity, Student Charge identities, allocation relationships and amounts, resulting settlement position, outstanding consequence and event history shall remain reconstructable.

## 8. Observable Completion Conditions

A governed allocation exists or the Payment is explicitly retained as an Unallocated Advance; monetary integrity is preserved; settlement/outstanding consequences are authoritative; Payment and Student Charge remain separately owned.

## 9. Traceability & Provenance

**Parent:** L3-F-008 — Partial or Multi Obligation Payment to Governed Allocation  
**Primary verification source:** L3-F-008 §§3, 5, 9–16 and authoritative L2-F02/F01/F07 architecture.

## 10. Open Issues / Deferred Design

No material upstream ambiguity is identified for the allocation/event boundary. Technical allocation algorithms and transaction mechanics remain deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
