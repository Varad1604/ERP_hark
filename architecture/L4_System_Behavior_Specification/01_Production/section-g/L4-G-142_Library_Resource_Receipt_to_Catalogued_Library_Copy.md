# Library Resource Receipt to Catalogued Library Copy

## 1. Specification Metadata

**Specification ID:** L4-G-142  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-142 — Library Resource Receipt to Catalogued Library Copy  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-142; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an approved acquisition from physical receipt through verified cataloguing and Library Copy creation, while preserving the subsequent copy-registration/availability boundary and excluding procurement/payment execution.

## 3. Required Business Context

- An approved acquisition exists.
- Procurement/vendor reference exists where applicable.
- The received resource can be matched to the acquisition.
- Required catalogue information and Library Copy creation authority are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-142-BH-001 | Record receipt and verify the received resource against the approved acquisition and applicable reference. | L3-G-142 §§7,9 |
| L4-G-142-BH-002 | Prevent unrelated catalogue/copy creation where acquisition/resource matching is unresolved. | L3-G-142 §§10–11 |
| L4-G-142-BH-003 | Establish and validate the authoritative Catalogue representation and preserve the acquisition-to-catalogue relationship. | L3-G-142 §§9,12–13 |
| L4-G-142-BH-004 | Create one or more physical Library Copy representations as required and associate each with its authoritative Catalogue record. | L3-G-142 §§3,9,16 |
| L4-G-142-BH-005 | Establish the Library Copy only in its appropriate initial condition; do not silently advance it to Available. | L3-G-142 §§13,16 |
| L4-G-142-BH-006 | Preserve acquisition, receipt, catalogue and copy lineage/history. | L3-G-142 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-142-BH-007 | The retrieved source does not expose dedicated canonical events for resource receipt, cataloguing completion or Library Copy creation. This specification shall not invent `ResourceReceived`, `Catalogued` or `LibraryCopyCreated`. G owns acquisition/catalogue/copy outcomes; E and F participate only within their authoritative context. | L3-G-142 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- Resource mismatch shall prevent unrelated catalogue/copy creation.
- Incomplete authoritative catalogue information shall prevent completed copy creation.
- Unauthorised copy creation shall not produce a completed Library Copy outcome.
- Financial receipt information remaining unresolved shall not transfer Finance ownership to G.

## 7. Audit & Evidence Behavior

Preserve acquisition reference, receipt outcome, resource verification, catalogue decision, Catalogue relationship, Library Copy creation authority and end-to-end lineage.

## 8. Observable Completion Conditions

Completion occurs when receipt is verified, cataloguing is complete, the Library Copy exists against its Catalogue record, lineage is preserved, and the copy is ready for the separate registration/availability workflow without being falsely marked Available.

## 9. Traceability & Provenance

**Parent:** L3-G-142 — Library Resource Receipt to Catalogued Library Copy  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Dedicated receipt/cataloguing/copy-creation event names are not exposed upstream and are not invented. Exact catalogue completeness and downstream copy-registration rules remain upstream-owned. Technical implementation is deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
