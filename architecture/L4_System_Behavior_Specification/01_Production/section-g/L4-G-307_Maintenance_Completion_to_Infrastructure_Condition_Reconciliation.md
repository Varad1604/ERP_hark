# Maintenance Completion to Infrastructure Condition Reconciliation

## 1. Specification Metadata
**Specification ID:** L4-G-307  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-307  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-307_Maintenance_Completion_to_Infrastructure_Condition_Reconciliation.md`; L2-G10 — Infrastructure Management Workflows; L2-G01 — Facilities Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall consume source-owned maintenance-completion evidence and reconcile the authoritative G10 Infrastructure condition without rewriting the Facilities/Maintenance execution record.

## 3. Required Business Context
The canonical Infrastructure, authoritative G01 maintenance-completion fact, post-maintenance condition evidence and applicable G10 lifecycle/health/compliance rules shall be available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-307-BH-001 | The system shall treat G01 maintenance completion as evidence/input and shall not mutate G01 Work Order history. | L3-G-307 §§4–5,9,12–14 |
| L4-G-307-BH-002 | The system shall validate the post-maintenance Infrastructure condition before reconciling G10 lifecycle or health facts. | L3-G-307 §§7,9,12 |
| L4-G-307-BH-003 | Where the source-defined conditions are satisfied, the system may reconcile G10 Infrastructure from `Under Maintenance` to `Operational`. | L3-G-307 §§9,12–13 |
| L4-G-307-BH-004 | Building or Space health shall be updated only where the corresponding authoritative condition has changed. | L3-G-307 §§9,12–13 |
| L4-G-307-BH-005 | Failed validation shall preserve the current G10 condition and shall not claim maintenance reconciliation completion. | L3-G-307 §§11,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-307-BH-006 | When G10 completes the valid maintenance reconciliation, the system shall publish `MaintenanceCompleted`; when applicable health facts change it may additionally publish `BuildingHealthUpdated` or `SpaceHealthUpdated` according to the affected aggregate. | L3-G-307 §§9,12,14 |

## 6. Failure / Rejection / Continuation
Incomplete or inconsistent completion evidence, unresolved compliance/condition concerns or invalid lifecycle transition shall prevent reconciliation and preserve current G10 facts.

## 7. Audit & Evidence Behavior
The system shall preserve the consumed maintenance-completion reference, condition evidence, resulting G10 state/health facts, actor/authority and canonical G10 events.

## 8. Observable Completion Conditions
The authoritative G10 condition is reconciled from valid source evidence, any permitted lifecycle/health changes are recorded, canonical events are published, and G01 history remains unchanged.

## 9. Traceability & Provenance
**Parent:** L3-G-307 — Maintenance Completion to Infrastructure Condition Reconciliation  
**Primary verification sources:** L2-G10; L2-G01

## 10. Open Issues / Deferred Design
No ownership transfer or implementation-level synchronization mechanism is defined.
