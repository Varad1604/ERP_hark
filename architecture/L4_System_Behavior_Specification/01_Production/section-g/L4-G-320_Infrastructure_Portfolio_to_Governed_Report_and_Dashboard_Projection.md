# Infrastructure Portfolio to Governed Report and Dashboard Projection

## 1. Specification Metadata
**Specification ID:** L4-G-320  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-320  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-320_Infrastructure_Portfolio_to_Governed_Report_and_Dashboard_Projection.md`; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; L2-D03 — Timetable Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall produce G10-owned Infrastructure reports, KPIs or dashboard-ready governed projections from authoritative G10 facts while preserving B03 institutional-campus governance, D03 timetable ownership and H07 dashboard-composition ownership.

## 3. Required Business Context
The authorized reporting scope, current authoritative G10 portfolio/capacity/availability/compliance/condition/reservation/allocation facts, governed definitions where applicable, and any source-owned B03/D03 context shall be available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-320-BH-001 | The system shall resolve report/KPI/projection inputs from canonical G10 facts and shall not replace missing source facts with invented values. | L3-G-320 §§4,7,9,12 |
| L4-G-320-BH-002 | The system shall preserve B03 InstitutionalCampus and D03 scheduling facts as externally owned context when such facts are required. | L3-G-320 §§4–5,9,12–13 |
| L4-G-320-BH-003 | G10 shall remain owner of Infrastructure-derived computation, report, KPI and governed projection under the controlling dependency-map ruling. | L3-G-320 §§3–5,12,17 |
| L4-G-320-BH-004 | The system shall not change any Infrastructure lifecycle/state merely by producing or consuming a report, KPI or projection. | L3-G-320 §§4,11,13 |
| L4-G-320-BH-005 | If a requested KPI definition is not governed upstream, the system shall not invent its formula or threshold. | L3-G-320 §§4,7,10,17 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-320-BH-006 | H07 may compose a G10-owned governed projection but shall not recalculate, own or become an alternative source for G10 Infrastructure facts, report logic or KPI definitions. | L3-G-320 §§2–5,9,12,14,17 |
| L4-G-320-BH-007 | D03 may consume G10 physical-resource insight for its own timetable decisions without transferring scheduling ownership to G10. | L3-G-320 §§4–5,12,14 |
| L4-G-320-BH-008 | The system shall not invent `InfrastructureReportGenerated`, `InfrastructureKPIUpdated` or another report-publication event because none is mandatory in the parent sources. | L3-G-320 §14 |

## 6. Failure / Rejection / Continuation
Incomplete/inconsistent source Infrastructure facts shall prevent the information product from being treated as governed until the G10 source facts are reconciled. H07 composition unavailability shall not invalidate an otherwise governed G10 report/projection.

## 7. Audit & Evidence Behavior
Where source governance requires lineage, the system shall preserve the authoritative G10 source population and governed definition/version used for the report/KPI/projection without rewriting historical business facts.

## 8. Observable Completion Conditions
The authorized G10 report/KPI/projection is produced from authoritative source facts, ownership boundaries are preserved, H07 composition (if used) does not acquire computation ownership, and no unsupported event/state change occurs.

## 9. Traceability & Provenance
**Parent:** L3-G-320 — Infrastructure Portfolio to Governed Report and Dashboard Projection  
**Primary verification sources:** `DEPENDENCY_MAP.md`; L2-G10; L2-B03; L2-D03; L2-H07

## 10. Open Issues / Deferred Design
The parent records lower-level G10 reporting wording inconsistent with the higher-authority dependency map, plus no single report/KPI lifecycle or mandatory publication event. This L4 applies the higher-authority ownership ruling and invents neither lifecycle nor event.
