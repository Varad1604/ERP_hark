# Catalogue Update to Effective Resource Information

## 1. Specification Metadata

**Specification ID:** L4-G-128  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-128 — Catalogue Update to Effective Resource Information  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-128; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised update to an existing Catalogue record so revised resource information becomes effective while preserving Catalogue identity, historical information, and separation from physical Library Copy management.

## 3. Required Business Context

- An authoritative Catalogue record exists.
- The update continues to represent the same intellectual work.
- Applicable cataloguing, classification and approval rules are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-128-BH-001 | Identify the Catalogue record and information requiring update. | L3-G-128 §§7–9 |
| L4-G-128-BH-002 | Prepare and validate revised bibliographic/classification/resource-type information as applicable. | L3-G-128 §§9,12 |
| L4-G-128-BH-003 | Confirm that the change remains an update to the existing intellectual work rather than creation of a new Catalogue identity. | L3-G-128 §§9–10 |
| L4-G-128-BH-004 | Apply configured catalogue review and authorisation before revised information becomes effective. | L3-G-128 §§9,15 |
| L4-G-128-BH-005 | Preserve prior catalogue information and update history. | L3-G-128 §§12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-128-BH-006 | When the authorised catalogue update becomes effective, Library Management shall publish `CatalogueUpdated`. No alternative event name or physical-copy lifecycle consequence shall be introduced. | L3-G-128 §§14,16 |

## 6. Failure / Rejection / Continuation

- Unverifiable or unauthorised changes shall not become effective.
- If the change represents a different intellectual work, it shall exit to the catalogue-creation workflow.
- Historical catalogue information shall not be deleted.
- Existing Library Copies shall not have their lifecycle changed solely by this catalogue-information update.

## 7. Audit & Evidence Behavior

Preserve update reason, revised information, validation, authority, effective outcome and historical catalogue information.

## 8. Observable Completion Conditions

Completion occurs when the revised information is authoritative for the existing Catalogue, `CatalogueUpdated` is published, and prior information remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-128 — Catalogue Update to Effective Resource Information  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact metadata completeness and approval thresholds remain configuration governed. Technical search indexing, APIs, database persistence schemas, UI screens and infrastructure are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
