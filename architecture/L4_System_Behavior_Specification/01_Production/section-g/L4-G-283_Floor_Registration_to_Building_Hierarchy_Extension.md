# Floor Registration to Building Hierarchy Extension

## 1. Specification Metadata
**Specification ID:** L4-G-283  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-283  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-283_Floor_Registration_to_Building_Hierarchy_Extension.md`; L2-G10 — Infrastructure Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall establish one canonical Floor beneath exactly one valid Building while preserving G10 hierarchy, identity and lifecycle rules.

## 3. Required Business Context
The parent Building shall exist and satisfy applicable parent conditions; Floor Number shall be unique within that Building; required information and authorization shall be valid.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-283-BH-001 | The system shall reject establishment when parent eligibility, Floor Number uniqueness, required information or authorization fails. | L3-G-283 §§7,9–12 |
| L4-G-283-BH-002 | On valid establishment, the system shall create one Floor under exactly one Building with immutable canonical identity. | L3-G-283 §§2–5,9,12 |
| L4-G-283-BH-003 | The system shall place the new Floor at `Proposed` and shall not make it Operational by registration alone. | L3-G-283 §§9,12–13 |
| L4-G-283-BH-004 | The system shall prevent a Floor lifecycle position from exceeding the parent Building lifecycle where the source rule applies. | L3-G-283 §12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-283-BH-005 | On successful establishment, the system shall publish `FloorCreated` from G10. | L3-G-283 §§9,14 |
| L4-G-283-BH-006 | The system shall not treat legacy `FloorRegistered` wording as a distinct semantic event. | L3-G-283 §§12,14 |

## 6. Failure / Rejection / Continuation
Invalid parent, duplicate Floor Number, invalid information or failed authorization shall leave infrastructure unchanged. Later lifecycle progression remains separate.

## 7. Audit & Evidence Behavior
The system shall preserve parent Building, Floor identity/number, validated input, actor/authorization context and `FloorCreated` publication.

## 8. Observable Completion Conditions
A unique canonical Floor exists under the correct Building at `Proposed`, remains G10-owned, and `FloorCreated` is published with auditable evidence.

## 9. Traceability & Provenance
**Parent:** L3-G-283 — Floor Registration to Building Hierarchy Extension  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows

## 10. Open Issues / Deferred Design
The parent records the historical registration-event/lifecycle ambiguity as resolved. Implementation design is deferred.
