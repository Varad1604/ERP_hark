# Institutional Infrastructure Registration to Physical Campus Establishment

## 1. Specification Metadata
**Specification ID:** L4-G-278  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-278  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-278_Institutional_Infrastructure_Registration_to_Physical_Campus_Establishment.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; L2-A01 — Platform & Tenant Foundation  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall establish one canonical G10 Physical Campus from an authorised registration while preserving B03 ownership of InstitutionalCampus identity/governance. Registration shall not make the campus operational.

## 3. Required Business Context
The B03 InstitutionalCampus reference, unique campus code, required physical registration information and authorization shall be resolvable before establishment.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-278-BH-001 | The system shall reject registration when the institutional reference, required physical information, authorization or campus-code uniqueness cannot be validated. | L3-G-278 §§5–9,12 |
| L4-G-278-BH-002 | On valid registration, the system shall establish exactly one G10-owned Physical Campus with immutable canonical identity and retain the B03 InstitutionalCampus only as a reference. | L3-G-278 §§2–5,9,12–13 |
| L4-G-278-BH-003 | The system shall place the newly established Physical Campus at `Proposed`. | L3-G-278 §§9,13 |
| L4-G-278-BH-004 | The system shall not treat registration as operational activation. | L3-G-278 §§3–4,9,13 |
| L4-G-278-BH-005 | The system shall preserve the campus code and canonical physical identity as immutable after establishment. | L3-G-278 §§12,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-278-BH-006 | On successful registration, the system shall publish `CampusRegistered` as the G10 registration fact and shall not publish `CampusCreated` as an alias. | L3-G-278 §§9,12,14 |

B03 retains InstitutionalCampus governance; Section A may provide authorization, event and audit capabilities without owning the physical fact.

## 6. Failure / Rejection / Continuation
Invalid identity, duplicate code, missing reference, incomplete required information or failed authorization shall leave canonical infrastructure unchanged. Later activation shall use a separately governed lifecycle.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-278-BH-007 | The system shall preserve evidence of the canonical Physical Campus identity, B03 reference, campus code, registration information, authority, resulting `Proposed` position and `CampusRegistered` publication. | L3-G-278 §§15–16 |

## 8. Observable Completion Conditions
Exactly one G10 Physical Campus exists at `Proposed`, immutable identity and B03 reference are preserved, and `CampusRegistered` is published. Operational availability is not a completion condition.

## 9. Traceability & Provenance
**Parent:** L3-G-278 — Institutional Infrastructure Registration to Physical Campus Establishment  
**Primary verification sources:** L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows

## 10. Open Issues / Deferred Design
No L4 correctness issue remains for this outcome. Implementation mechanisms remain deferred.
