# Lost Library Resource to Replacement Obligation

## 1. Specification Metadata
**Specification ID:** L4-G-139  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-139 — Lost Library Resource to Replacement Obligation  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-139 — Lost Library Resource to Replacement Obligation

## 2. Behavioral Objective & Boundary
SutraERP shall establish an authoritative lost-resource outcome and, where policy requires, the associated replacement/fine obligation without executing payment or procurement.

**Start:** Borrowed resource is reported or determined lost.  
**End:** Loss and applicable replacement obligation are recorded.

## 3. Required Business Context
- Library Copy/resource is identifiable.
- Relevant borrowing/member relationship is authoritative where applicable.
- Lost-resource and fine/replacement policy is available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-139-BH-001 | Establish the lost-resource condition against the authoritative resource/circulation context. | L3-G-139 §§7,9 |
| L4-G-139-BH-002 | Record Lost Resource Information and preserve relevant circulation history. | L3-G-139 §§2,9 |
| L4-G-139-BH-003 | Evaluate the applicable fine/replacement policy without inventing a replacement value. | L3-G-139 §§3,12 |
| L4-G-139-BH-004 | Record the applicable Lost Resource or Replacement Charge obligation where policy requires. | L3-G-139 §§2,9,12 |
| L4-G-139-BH-005 | Preserve the loss, obligation and financial-reference history. | L3-G-139 §§15,16 |

## 5. Cross-Domain & Event Behavior
No dedicated lost-resource event is asserted by the authoritative source. Finance receives financial reference information and H receives notification requirements; G retains ownership.

## 6. Failure / Rejection / Continuation
- If loss cannot be established, the resource retains its current authoritative condition.
- If member/resource relationship cannot be established, no unsupported liability is assigned.
- If replacement policy cannot be established, no invented amount is calculated.
- Payment and procurement remain downstream responsibilities.

## 7. Audit & Evidence Behavior
Preserve resource, borrowing/member relationship, loss determination, policy basis, obligation category, authority and financial reference.

## 8. Observable Completion Conditions
Completion occurs when the loss is authoritative and any applicable replacement/fine obligation is recorded, with history preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-139 — Lost Library Resource to Replacement Obligation  
**Primary verification source:** L3-G-139; locked L2 Fine/Library Copy workflows

## 10. Open Issues / Deferred Design
Replacement values and dedicated event vocabulary remain policy/upstream concerns. Finance payment and procurement execution remain outside L4.