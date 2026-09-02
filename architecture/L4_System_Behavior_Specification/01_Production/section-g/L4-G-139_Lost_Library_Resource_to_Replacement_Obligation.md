# Lost Library Resource to Replacement Obligation

## 1. Specification Metadata

**Specification ID:** L4-G-139  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-139 — Lost Library Resource to Replacement Obligation  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-139; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish an authoritative lost-resource outcome and, where policy requires, the associated Library Management replacement/fine obligation without executing Finance payment/accounting or replacement procurement.

## 3. Required Business Context

- The affected Library Copy/resource is identifiable.
- Relevant borrowing/member relationship is authoritative where applicable.
- Lost-resource and fine/replacement policy is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-139-BH-001 | Identify the Library Copy/resource and relevant member/borrowing context and establish the authoritative lost-resource condition. | L3-G-139 §§7,9 |
| L4-G-139-BH-002 | Record Lost Resource Information and preserve the relevant circulation history. | L3-G-139 §§2,9,12 |
| L4-G-139-BH-003 | Evaluate the applicable configurable lost-resource/replacement policy without inventing a replacement value, depreciation rule, liability rule or payment deadline. | L3-G-139 §§10,12 |
| L4-G-139-BH-004 | Where policy requires, establish the applicable Lost Resource or Replacement Charge obligation and its financial reference under G ownership. | L3-G-139 §§3,9,12 |
| L4-G-139-BH-005 | Do not represent the loss outcome as an invented Circulation lifecycle state; where a charge applies, preserve the canonical Fine lifecycle `Calculated → Pending → Paid / Waived → Closed`. | L3-G-139 §§12–13 |
| L4-G-139-BH-006 | Preserve loss determination, relationship evidence, policy basis, obligation category and downstream financial/notification references. | L3-G-139 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-139-BH-007 | The retrieved authoritative source does not expose a dedicated canonical lost-resource or replacement-obligation event. This specification shall not invent `ResourceLost` or `ReplacementObligationCreated`. G owns the loss and library obligation; F owns accounting/payment and H owns notification delivery. | L3-G-139 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- If loss cannot be established, retain the current authoritative circulation/condition outcome.
- If member/resource relationship cannot be established, do not assign unsupported liability.
- If the replacement rule cannot be established, record the loss without inventing a financial amount.
- Finance processing may remain downstream without invalidating G's authoritative library obligation.
- Physical replacement/acquisition is a separate process.

## 7. Audit & Evidence Behavior

Preserve resource identity, borrowing/member relationship where applicable, loss determination, Lost Resource Information, policy basis, obligation category, authority and financial/notification references.

## 8. Observable Completion Conditions

Completion occurs when the loss is authoritative and any policy-required replacement/fine obligation is recorded, with history preserved and downstream Finance/H responsibilities clearly bounded.

## 9. Traceability & Provenance

**Parent:** L3-G-139 — Lost Library Resource to Replacement Obligation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The upstream source does not expose a dedicated lost-resource event or fixed replacement-value rule. No event, amount, depreciation rule or procurement behavior is invented. Technical APIs, persistence, UI, infrastructure and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
