# Lost Library Resource to Replacement Obligation

## 1. Specification Metadata

**Specification ID:** L4-G-139  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-139 — Lost Library Resource to Replacement Obligation  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-139; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish an authoritative lost-resource outcome and, where policy requires, the associated replacement/fine obligation without executing payment or procurement.

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

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-139-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Library Management ownership. | L3-G-139 §14 |

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
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
