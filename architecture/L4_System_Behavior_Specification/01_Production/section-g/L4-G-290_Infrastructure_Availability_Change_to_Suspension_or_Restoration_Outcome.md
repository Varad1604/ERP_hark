# Infrastructure Availability Change to Suspension or Restoration Outcome

## 1. Specification Metadata
**Specification ID:** L4-G-290  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-290  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-290_Infrastructure_Availability_Change_to_Suspension_or_Restoration_Outcome.md`; L2-G10 — Infrastructure Management Workflows; L2-D03 — Timetable Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern changes to G10-owned resource availability and expose the current usability fact to Academic Delivery without transferring availability or timetable ownership.

## 3. Required Business Context
The affected resource, current lifecycle/availability, business condition, applicable policy, effective scope/period and authorization shall be known.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-290-BH-001 | The system shall validate authorization, lifecycle compatibility and availability policy before changing the current usability fact. | L3-G-290 §§7,9,12 |
| L4-G-290-BH-002 | The system shall apply the availability change only to the governed resource/hierarchy scope and effective period. | L3-G-290 §§4,9–10 |
| L4-G-290-BH-003 | Restoration shall establish the source-defined usable condition only when the governing restriction no longer applies. | L3-G-290 §§9–10,12–13 |
| L4-G-290-BH-004 | The system shall not create new `Suspended` or `Restored` lifecycle states and shall not change physical lifecycle solely because availability changes. | L3-G-290 §§12–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-290-BH-005 | On any valid change to an existing availability condition, the system shall publish `InfrastructureAvailabilityChanged` from G10. | L3-G-290 §§9,12,14 |
| L4-G-290-BH-006 | The current G10 usability fact shall be available to D03; D03 shall retain ownership of any timetable/scheduling response. | L3-G-290 §§2–5,9,14 |

## 6. Failure / Rejection / Continuation
Failed policy, lifecycle or authorization validation shall preserve the prior authoritative availability fact.

## 7. Audit & Evidence Behavior
Evidence shall preserve prior/current availability, scope/effective period, reason, actor/authority and `InfrastructureAvailabilityChanged` publication.

## 8. Observable Completion Conditions
The G10 usability fact reflects the governed restriction/suspension/restoration, D03 can consume it without ownership transfer, physical lifecycle is unchanged, and the canonical event is published.

## 9. Traceability & Provenance
**Parent:** L3-G-290 — Infrastructure Availability Change to Suspension or Restoration Outcome  
**Primary verification sources:** L2-G10; L2-D03

## 10. Open Issues / Deferred Design
No new availability lifecycle state or D03 transition is implied. Implementation design is deferred.
