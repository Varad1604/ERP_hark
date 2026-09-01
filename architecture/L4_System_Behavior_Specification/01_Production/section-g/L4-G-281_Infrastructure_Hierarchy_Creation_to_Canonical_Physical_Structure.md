# Infrastructure Hierarchy Creation to Canonical Physical Structure

## 1. Specification Metadata

**Specification ID:** L4-G-281  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-281  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-281_Infrastructure_Hierarchy_Creation_to_Canonical_Physical_Structure.md`; L2-G10 — Infrastructure Management Workflows; `L2-G10_Infrastructure_Management_Workflows_Amendment_2026-08-26.md`; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall establish or reuse the independent G10 Physical Campus, Building, Floor and Space aggregates needed for the canonical physical hierarchy while validating parentage, uniqueness and acyclic structure. Later Locked G10 amendments resolve the historical structural registration-event and initial-lifecycle conflicts without creating a cross-aggregate hierarchy lifecycle.

## 3. Required Business Context

- The intended Physical Campus → Building → optional Wing → Floor → Space structure is defined.
- Existing parent aggregates, where reused, are canonical and valid.
- Required codes and parent-child relationships satisfy G10 uniqueness and acyclic constraints.
- Each newly required aggregate is established through its own G10 workflow; hierarchy creation is not one aggregate transaction.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-281-BH-001 | The system shall validate parent existence, required parent lifecycle conditions, unique codes and absence of circular physical relationships before establishing each missing hierarchy member. | L3-G-281 §§7,9,11–12 |
| L4-G-281-BH-002 | The system shall reuse existing canonical parent identities and shall create only missing child aggregates under their source-defined parent relationships. | L3-G-281 §§9–12 |
| L4-G-281-BH-003 | Each Building shall belong to exactly one Physical Campus, each Floor to exactly one Building, and each Space to exactly one Floor; optional Wing shall not create an unauthorized hierarchy model. | L3-G-281 §§10,12 |
| L4-G-281-BH-004 | A newly established Physical Campus, Building, Floor or Space shall enter `Proposed`; registration itself shall not make that aggregate Operational. | L3-G-281 §13; L2-G10 Amendments v1.1 §2 and v1.2 §2 |
| L4-G-281-BH-005 | The system shall not create a separate hierarchy aggregate, hierarchy lifecycle or atomic cross-aggregate state transition for this scenario. | L3-G-281 §§4,12–13,17 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-281-BH-006 | When a new Physical Campus is registered, the system shall publish `CampusRegistered` from G10. | L3-G-281 §§9,14; L2-G10 Amendment v1.1 §1 |
| L4-G-281-BH-007 | When new structural members are established, the system shall publish `BuildingCreated`, `FloorCreated` and `SpaceCreated` respectively from G10 and shall not publish legacy `*Registered` aliases for those same facts. | L3-G-281 §§9,14; L2-G10 Governance Completion Amendment §2 |

No single `HierarchyCreated` event exists or is required. Existing hierarchy members reused by this scenario are not re-published as newly created facts.

## 6. Failure / Rejection / Continuation

Invalid parentage, duplicate codes or circular relationships shall reject only the affected registration path and preserve existing canonical structure. Failure of one child registration shall not rewrite independently established aggregates.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-281-BH-008 | The system shall preserve each aggregate identity, parent reference, validation result, registration actor/authorization context and applicable establishment event independently for every newly created hierarchy member. | L3-G-281 §§15–16 |

## 8. Observable Completion Conditions

Completion is observable when the required canonical Physical Campus/Building/Floor/Space relationships exist, all structural invariants hold, every newly established member is at `Proposed`, and the applicable canonical G10 establishment events have been published. No hierarchy-completion event is needed.

## 9. Traceability & Provenance

**Parent:** L3-G-281 — Infrastructure Hierarchy Creation to Canonical Physical Structure  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows  
**Later Locked resolution:** G10 Amendments v1.1.0 and v1.2.0 resolve the parent's registration-event and initial-lifecycle conflicts.

## 10. Open Issues / Deferred Design

The parent’s blocking event-name conflict and initial lifecycle ambiguity are resolved by later Locked L2 amendments. No cross-aggregate hierarchy completion event or hierarchy lifecycle is defined; this remains a composition of independently owned G10 workflows. Implementation orchestration is deferred.
