# Overdue Borrowing to Fine Calculation

## 1. Specification Metadata
**Specification ID:** L4-G-137  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-137 — Overdue Borrowing to Fine Calculation  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-137 — Overdue Borrowing to Fine Calculation

## 2. Behavioral Objective & Boundary
SutraERP shall recognise an overdue borrowing, apply authoritative configurable fine policy, calculate and record the applicable Library Fine, and expose downstream business information without settling payment.

**Start:** Active borrowing due date is exceeded.  
**End:** Fine is Calculated and available for subsequent processing.

## 3. Required Business Context
- Active borrowing and authoritative due date exist.
- Member/resource context is identifiable.
- Fine policy and applicable calculation inputs are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-137-BH-001 | Establish due-date exceedance for the Active borrowing. | L3-G-137 §§7,9 |
| L4-G-137-BH-002 | Apply configured fine rules using authoritative resource, membership, grace-period and institutional policy inputs. | L3-G-137 §§9,12 |
| L4-G-137-BH-003 | Prevent unsupported fine calculation when required authoritative inputs or policy are unavailable. | L3-G-137 §11 |
| L4-G-137-BH-004 | Record the applicable Library Fine and category when a fine is due. | L3-G-137 §§3,9,12 |
| L4-G-137-BH-005 | Prevent duplicate fine creation for the same authoritative overdue obligation. | L3-G-137 §10 |

## 5. Cross-Domain & Event Behavior
On the source-defined outcomes, Library Management shall publish canonical `ResourceOverdue` and `FineCalculated` events. Finance receives the financial reference for accounting/payment; H consumes notification information. G remains publisher and owner. | L3-G-137 §14

## 6. Failure / Rejection / Continuation
- If due-date exceedance cannot be established, fine calculation shall not proceed.
- If policy inputs cannot be established, no invented default calculation shall be applied.
- If no fine is due, no unsupported financial amount shall be created.
- Finance processing does not alter Library Management ownership of the calculated fine.

## 7. Audit & Evidence Behavior
Preserve borrowing identity, due-date basis, policy inputs, fine applicability, category/amount, authority and financial reference.

## 8. Observable Completion Conditions
Completion occurs when the overdue condition is established, applicable fine calculation is recorded, `FineCalculated` is published where applicable, and the fine is available for subsequent lifecycle processing.

## 9. Traceability & Provenance
**Parent:** L3-G-137 — Overdue Borrowing to Fine Calculation  
**Primary verification source:** L3-G-137; locked L2 Fine Management workflow

## 10. Open Issues / Deferred Design
Exact monetary values remain configurable. Payment, accounting, APIs, persistence and notification transport remain outside L4.