# Overdue Borrowing to Fine Calculation

## 1. Specification Metadata

**Specification ID:** L4-G-137  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-137 — Overdue Borrowing to Fine Calculation  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-137; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall recognise an overdue Active borrowing, apply authoritative configurable Library Fine policy, calculate and record the applicable fine, and make the resulting financial/communication business information available without performing Finance settlement or notification delivery.

## 3. Required Business Context

- Active borrowing and authoritative due date exist.
- Member/resource context is identifiable.
- Fine policy, grace-period/resource/membership category inputs and institutional calculation rules are available where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-137-BH-001 | Establish that the authoritative due date of the Active borrowing has been exceeded without completed return. | L3-G-137 §§7–9 |
| L4-G-137-BH-002 | Determine fine applicability using configured Library Management policy. | L3-G-137 §§9–10,12 |
| L4-G-137-BH-003 | Apply authoritative resource category, membership category, grace period, overdue duration and institutional policy inputs without inventing defaults. | L3-G-137 §§9,12 |
| L4-G-137-BH-004 | Where a fine is due, calculate and record the Library Fine in its `Calculated` condition and preserve the borrowing/fine relationship. | L3-G-137 §§3,9,13,16 |
| L4-G-137-BH-005 | Prevent duplicate fine creation for the same authoritative overdue obligation and preserve calculation history. | L3-G-137 §§10,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-137-BH-006 | Library Management shall publish `ResourceOverdue` when the due-date-exceeded business fact is established and `FineCalculated` when a fine is generated. Finance receives the applicable financial reference and remains owner of accounting/payment; H receives the notification outcome and remains owner of delivery. | L3-G-137 §§5,14,16 |

## 6. Failure / Rejection / Continuation

- If due-date exceedance cannot be established, fine calculation shall not proceed.
- If authoritative calculation policy or required inputs are unavailable, no invented default shall be applied.
- If policy establishes that no fine is due, no unsupported financial amount shall be created.
- Finance processing or notification availability shall not transfer Library Fine ownership away from Library Management.
- This scenario shall not settle payment, approve waiver, close the fine, or close the borrowing.

## 7. Audit & Evidence Behavior

Preserve borrowing identity, due-date basis, policy inputs, fine applicability, category/amount, canonical events, authority, Finance reference and notification business outcome.

## 8. Observable Completion Conditions

Completion occurs when overdue status is established, `ResourceOverdue` is published, any applicable Library Fine is recorded in `Calculated`, `FineCalculated` is published where a fine is generated, and Finance/H business information is available with ownership boundaries preserved.

## 9. Traceability & Provenance

**Parent:** L3-G-137 — Overdue Borrowing to Fine Calculation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact fine values, category-specific rules, grace periods, caps and calculation thresholds remain configuration governed. Payment execution, accounting settlement, notification delivery, technical scheduling, APIs, database persistence schemas and UI are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
