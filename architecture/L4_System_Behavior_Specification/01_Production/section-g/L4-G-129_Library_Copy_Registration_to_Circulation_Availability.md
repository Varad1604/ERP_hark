# Library Copy Registration to Circulation Availability

## 1. Specification Metadata

**Specification ID:** L4-G-129  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-129 — Library Copy Registration to Circulation Availability  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-129; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern registration of a distinct physical Library Copy against exactly one authoritative Catalogue record until the copy becomes Available for Circulation.

## 3. Required Business Context

- A valid Catalogue record exists.
- A distinct physical resource is ready for Library Copy registration.
- Required copy identity, location/condition and configured authority are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-129-BH-001 | Register the physical Library Copy against exactly one authoritative Catalogue record. | L3-G-129 §§2,9,12 |
| L4-G-129-BH-002 | Establish required operational copy identity, including accession and barcode information as supported by the parent. | L3-G-129 §§4,9,16 |
| L4-G-129-BH-003 | Record applicable branch/location, physical condition and operational status information. | L3-G-129 §§9,12 |
| L4-G-129-BH-004 | Progress the Library Copy through `Added → Available` when circulation-availability conditions are satisfied. | L3-G-129 §§3,13,16 |
| L4-G-129-BH-005 | Preserve copy registration, Catalogue relationship, condition and availability history. | L3-G-129 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-129-BH-006 | The retrieved G05 event catalogue does not expose a dedicated Library Copy registration/availability event; this specification shall not invent one. Library Management owns the resulting Library Copy and its availability; E may provide personnel context without acquiring copy ownership. | L3-G-129 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- A copy without a valid Catalogue relationship shall not become circulation-available.
- Missing required copy identity shall prevent the Available outcome.
- A copy whose condition/status does not permit ordinary circulation shall not be treated as Available.
- Reference Only or Under Verification treatment shall remain governed by supported/configured operational status.

## 7. Audit & Evidence Behavior

Preserve Catalogue relationship, copy identity, location, condition, authority, operational status and availability history.

## 8. Observable Completion Conditions

Completion occurs when a distinct Library Copy references exactly one Catalogue record, satisfies circulation criteria, reaches `Available`, and registration history is traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-129 — Library Copy Registration to Circulation Availability  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

A dedicated Library Copy registration/availability event is unresolved upstream and shall not be invented. Exact identifier uniqueness and availability rules remain configuration governed. Barcode hardware, APIs, databases, UI and infrastructure are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
