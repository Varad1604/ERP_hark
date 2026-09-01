# Space Registration to Physical Space Establishment

## 1. Specification Metadata

**Specification ID:** L4-G-284  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-284  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-284_Space_Registration_to_Physical_Space_Establishment.md`; L2-G10 — Infrastructure Management Workflows; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall establish one canonical G10 Space under exactly one eligible Floor with valid identity, Space Type, capacity and area information. The later Locked G10 completion amendment resolves the parent's event-name and initial-lifecycle conflicts: the new Space enters `Proposed` and publishes `SpaceCreated`.

## 3. Required Business Context

- The parent Floor exists and is `Operational` as required by the detailed Space Registration Workflow.
- Space Code is unique within the parent Building.
- Required Space Name, Code, Type, Capacity, Area and parent information are available and valid.
- Infrastructure Administrator is authorized to register the Space.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-284-BH-001 | The system shall reject Space establishment when the parent Floor is absent/ineligible, Space Code is not unique within the Building, required information is invalid, or authorization fails. | L3-G-284 §§7,9,11–12 |
| L4-G-284-BH-002 | On valid establishment, the system shall create one canonical Space under exactly one Floor while preserving G10 ownership and immutable Space identity. | L3-G-284 §§2–5,9,12 |
| L4-G-284-BH-003 | The newly established Space shall enter `Proposed`; establishment shall not make the Space Operational. | L3-G-284 §13; L2-G10 Governance Completion Amendment §2 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-284-BH-004 | On successful establishment, the system shall publish `SpaceCreated` from Infrastructure Management / G10. | L3-G-284 §§9,14; L2-G10 Governance Completion Amendment §2 |
| L4-G-284-BH-005 | The system shall not publish legacy `SpaceRegistered` vocabulary as a separate semantic event for the same establishment fact. | L3-G-284 §§14,17; L2-G10 Governance Completion Amendment §2 |

Classification, capability, capacity, availability and later lifecycle changes are separate G10 behaviors and are not implied by Space establishment.

## 6. Failure / Rejection / Continuation

Failed parent, uniqueness, physical-value or authorization validation shall leave existing infrastructure unchanged and shall not publish `SpaceCreated`.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-284-BH-006 | The system shall preserve parent Floor reference, canonical Space identity/code, validated registration information, actor/authorization context and `SpaceCreated` publication evidence. | L3-G-284 §§15–16 |

## 8. Observable Completion Conditions

A canonical Space exists under exactly one valid Floor at `Proposed`, required registration rules are satisfied, audit evidence is preserved and `SpaceCreated` has been published by G10.

## 9. Traceability & Provenance

**Parent:** L3-G-284 — Space Registration to Physical Space Establishment  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows  
**Later Locked resolution:** G10 Governance Completion Amendment v1.2.0 resolves `SpaceCreated` and `Proposed` for this establishment fact.

## 10. Open Issues / Deferred Design

The parent's historical `SpaceRegistered`/`SpaceCreated` and initial-lifecycle conflicts are resolved by the later Locked amendment. Subsequent Space classification and operational progression remain separate specifications; implementation design is deferred.
