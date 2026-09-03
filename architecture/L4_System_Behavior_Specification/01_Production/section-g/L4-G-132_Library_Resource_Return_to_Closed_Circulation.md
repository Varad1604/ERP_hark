# Library Resource Return to Closed Circulation

## 1. Specification Metadata

**Specification ID:** L4-G-132  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-132 — Library Resource Return to Closed Circulation  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-132; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern return of an actively borrowed Library Copy through circulation closure, condition assessment, resulting copy treatment, and applicable overdue/fine business outcomes without performing Finance settlement.

## 3. Required Business Context

- An Active borrowing exists and the borrowed Library Copy is identifiable.
- Due-date, return, condition and fine policies are available.
- Library Management owns circulation, copy operational status and Library Fine calculation; Finance owns accounting/payment and H owns communication delivery.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-132-BH-001 | Match the returned Library Copy to its authoritative Active borrowing and member. | L3-G-132 §§7,9 |
| L4-G-132-BH-002 | Record return date/time and applicable physical-condition, damage or loss observations. | L3-G-132 §§2,9,12 |
| L4-G-132-BH-003 | Evaluate overdue and fine implications using configured library policy where applicable. | L3-G-132 §§9–10,12 |
| L4-G-132-BH-004 | Progress the circulation through `Active → Returned → Closed` when the return is valid. | L3-G-132 §§3,13,16 |
| L4-G-132-BH-005 | Establish the returned copy's authoritative operational status; do not automatically make every returned copy Available. | L3-G-132 §§3,9–10,13 |
| L4-G-132-BH-006 | Preserve immutable borrowing, return, condition and fine history. | L3-G-132 §§12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-132-BH-007 | Library Management shall publish `ResourceReturned` for the return outcome and, where their authoritative conditions are met, `ResourceOverdue` and `FineCalculated`. Finance receives the applicable financial reference without acquiring Library Fine ownership; H may consume return/fine outcomes for communication. | L3-G-132 §§5,14,16 |

## 6. Failure / Rejection / Continuation

- A return that cannot be matched to an Active borrowing shall not close an unrelated circulation.
- Unresolved copy condition shall not result in unsupported Available status.
- Fine/accounting settlement shall not be claimed by Library Management.
- Historical circulation records shall remain immutable.

## 7. Audit & Evidence Behavior

Preserve borrowing identity, return time, condition observations, overdue/fine basis, closure decision, resulting copy status and cross-domain references.

## 8. Observable Completion Conditions

Completion occurs when circulation is `Closed`, `ResourceReturned` is published, the copy has an authoritative resulting operational status, and any applicable fine/Finance/H outcomes are traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-132 — Library Resource Return to Closed Circulation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Fine amounts, grace periods and copy-condition thresholds remain configuration governed. Technical payment execution, notification delivery, APIs, database persistence schemas, UI screens and infrastructure are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
