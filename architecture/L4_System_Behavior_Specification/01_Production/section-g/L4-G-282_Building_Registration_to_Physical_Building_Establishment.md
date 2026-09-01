# Building Registration to Physical Building Establishment

## 1. Specification Metadata
**Specification ID:** L4-G-282  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-282  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-282_Building_Registration_to_Physical_Building_Establishment.md`; L2-G10 — Infrastructure Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall establish one canonical G10 Building beneath exactly one valid Physical Campus. Establishment shall remain distinct from later operational lifecycle progression.

## 3. Required Business Context
The parent Physical Campus shall exist and satisfy applicable establishment conditions; Building code, required metadata and authorization shall be valid.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-282-BH-001 | The system shall reject establishment if the parent Campus is invalid, Building code is not unique within Campus, required Building information is invalid, Gross Area is non-positive where applicable, Construction Year is future-dated where applicable, or authorization fails. | L3-G-282 §§7,9–12 |
| L4-G-282-BH-002 | On valid establishment, the system shall create one Building under exactly one Physical Campus with immutable canonical identity. | L3-G-282 §§2–5,9,12 |
| L4-G-282-BH-003 | The newly established Building shall enter `Proposed`; establishment shall not make it Operational. | L3-G-282 §§9,12–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-282-BH-004 | On successful establishment, the system shall publish `BuildingCreated` from G10. | L3-G-282 §§9,12,14 |
| L4-G-282-BH-005 | The system shall not treat legacy `BuildingRegistered` vocabulary as a distinct event. | L3-G-282 §§12,14 |

## 6. Failure / Rejection / Continuation
Failed parent, uniqueness, invariant or authorization validation shall leave existing infrastructure unchanged. Operational progression is separate.

## 7. Audit & Evidence Behavior
The system shall preserve parent reference, Building identity/code, validated metadata, actor/authorization context and `BuildingCreated` publication.

## 8. Observable Completion Conditions
A unique canonical Building exists under one valid Physical Campus at `Proposed`, with immutable identity, evidence preserved and `BuildingCreated` published.

## 9. Traceability & Provenance
**Parent:** L3-G-282 — Building Registration to Physical Building Establishment  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows

## 10. Open Issues / Deferred Design
The parent records the historical registration-event/lifecycle ambiguity as resolved. Implementation design is deferred.
