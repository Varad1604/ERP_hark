# Physical Zone Establishment to Infrastructure Grouping

## 1. Specification Metadata

**Specification ID:** L4-G-285  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-285  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-285_Physical_Zone_Establishment_to_Infrastructure_Grouping.md`; L2-G10 — Infrastructure Management Workflows; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern establishment of a G10 Zone as a logical operational grouping of existing physical infrastructure, preserving physical hierarchy and allowing source-supported multiple Zone memberships. Current Locked G10 still does not provide one canonical Zone-registration event or initial Zone lifecycle position.

## 3. Required Business Context

- The required Zone Type is configured and the proposed Zone Name satisfies G10 rules.
- Infrastructure to be grouped already exists canonically in G10.
- Infrastructure Administrator is authorized to establish Zones and manage membership.
- Zone membership is logical grouping and never re-parents physical infrastructure.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-285-BH-001 | The system shall validate configured Zone Type, Zone metadata and applicable Zone Name uniqueness before establishment. | L3-G-285 §§7,9,11–12 |
| L4-G-285-BH-002 | When source-defined Zone establishment conditions are satisfied, the system shall preserve the Zone as a G10-owned grouping concept without inserting it into the Campus → Building → Floor → Space hierarchy. | L3-G-285 §§2–5,9,12 |
| L4-G-285-BH-003 | The system shall validate and establish Zone Membership only for canonical G10 infrastructure references and shall not re-parent those aggregates. | L3-G-285 §§4,9,12–13 |
| L4-G-285-BH-004 | The system shall permit multiple valid Zone memberships where G10 rules allow them and shall preserve existing memberships unless separately changed. | L3-G-285 §§10,12 |
| L4-G-285-BH-005 | The system shall not assert a specific newly established Zone lifecycle position while Locked G10 remains inconsistent between generic `Draft` wording and the Zone-specific `Proposed → Operational → Archived` model. | L3-G-285 §§3,13,16–17 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-285-BH-006 | On successful Zone membership establishment, the system shall publish `AssignedToZone` from G10. | L3-G-285 §§9,12,14 |

The later G10 completion amendment does not normalize Zone registration. `ZoneRegistered` versus `ZoneCreated` therefore remains unresolved and no local selection is made.

## 6. Failure / Rejection / Continuation

Invalid Zone configuration/name, missing infrastructure reference, authorization failure or an attempted physical re-parenting shall prevent the affected establishment/membership action. An unresolved registration-event identity shall prevent certification of the full frozen Zone-establishment outcome.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-285-BH-007 | The system shall preserve Zone definition, referenced infrastructure memberships, actor/authorization context, `AssignedToZone` publication and the unresolved Zone registration-event/lifecycle condition. | L3-G-285 §§15–17 |

## 8. Observable Completion Conditions

Membership completion is observable through valid Zone Membership and `AssignedToZone`. Full Zone establishment cannot be certified until upstream resolves the canonical Zone registration event and the initial Zone lifecycle position.

## 9. Traceability & Provenance

**Parent:** L3-G-285 — Physical Zone Establishment to Infrastructure Grouping  
**Primary verification source:** L2-G10 — Infrastructure Management Workflows  
**Current-source check:** G10 Governance Completion Amendment v1.2.0 does not change Zone registration/lifecycle vocabulary.

## 10. Open Issues / Deferred Design

`ZoneRegistered` versus `ZoneCreated` and the initial Zone lifecycle conflict remain active parent blockers. No new physical hierarchy level, Zone lifecycle or event alias is introduced.
