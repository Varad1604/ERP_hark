# Catalogue Resource Identification to Published Catalogue Record

## 1. Specification Metadata

**Specification ID:** L4-G-127  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-127 — Catalogue Resource Identification to Published Catalogue Record  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-127; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern identification, preparation, verification and publication of an intellectual library resource as an authoritative Catalogue record without creating a physical Library Copy.

## 3. Required Business Context

- The intellectual resource is identifiable.
- Applicable catalogue metadata, resource-type and classification rules are available.
- Library Management owns the Catalogue lifecycle; Library Copy remains a separate aggregate.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-127-BH-001 | Register the identified intellectual resource for catalogue preparation. | L3-G-127 §§4,9 |
| L4-G-127-BH-002 | Prepare and validate supported bibliographic, classification, resource-type and configured metadata. | L3-G-127 §§9,12 |
| L4-G-127-BH-003 | Apply configured catalogue review and approval. | L3-G-127 §§9,15 |
| L4-G-127-BH-004 | Progress the Catalogue through `Draft → Verified → Published` when required conditions are satisfied. | L3-G-127 §§9,13,16 |
| L4-G-127-BH-005 | Preserve catalogue preparation, verification, publication and historical evidence. | L3-G-127 §§12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-127-BH-006 | On publication, Library Management shall publish the exact canonical event `CatalogueCreated`. The event name shall not be renamed even though its locked meaning is Catalogue published. E may provide personnel context and H may consume published information without acquiring catalogue ownership. | L3-G-127 §§5,14,16 |

## 6. Failure / Rejection / Continuation

- Incomplete or unverifiable catalogue information shall prevent publication.
- Rejected review shall return the record to the applicable pre-publication business position without inventing a new lifecycle state.
- Publication shall not imply that a physical Library Copy exists.

## 7. Audit & Evidence Behavior

Preserve resource identity, catalogue information, classification, verification, approval, publication and history.

## 8. Observable Completion Conditions

Completion occurs when the Catalogue reaches `Published`, `CatalogueCreated` is published by Library Management, and the catalogue record is available to subsequent library operations while physical-copy management remains separate.

## 9. Traceability & Provenance

**Parent:** L3-G-127 — Catalogue Resource Identification to Published Catalogue Record  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Metadata completeness, classification choice, multilingual metadata and approval requirements remain configuration governed. Technical search/indexing, APIs, database persistence schemas, UI screens and infrastructure are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
