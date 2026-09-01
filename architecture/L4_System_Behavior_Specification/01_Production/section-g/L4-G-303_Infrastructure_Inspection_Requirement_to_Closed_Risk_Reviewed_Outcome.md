# Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome

## 1. Specification Metadata
**Specification ID:** L4-G-303  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-303  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.1.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-303; L2-G10 and L2-G00 2026-09-01 amendments

## 2. Behavioral Objective & Boundary
Complete a G10 Infrastructure Inspection while keeping Facilities Inspection, maintenance execution and Operational Risk under their canonical owners.

## 3. Required Business Context
Canonical Infrastructure exists; authorized G10 inspection criteria/roles are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-G-303-BH-001 | Create/schedule the G10 Inspection Record and publish `InfrastructureInspectionScheduled`. |
| L4-G-303-BH-002 | Permit only an authorized Infrastructure Inspector to progress the inspection through In Progress and record findings. |
| L4-G-303-BH-003 | Reconcile the authoritative G10 Infrastructure condition from valid inspection findings. |
| L4-G-303-BH-004 | For corrective-maintenance findings, create only the G10 requirement/handoff to G01; do not create a G01 Work Order from G10. |
| L4-G-303-BH-005 | For material risk-only findings, refer evidence to G00; G00 alone may register Operational Risk. |
| L4-G-303-BH-006 | Require Infrastructure Manager approval before closure and publish `InfrastructureInspectionCompleted` for the G10 inspection fact. |

## 5. Cross-Domain & Event Behavior
G01 retains `FacilityInspection*` events for its separate aggregate; G00 retains Operational Risk events.

## 6. Failure / Rejection / Continuation
Material findings without a governed follow-up keep the G10 inspection from closing.

## 7. Audit & Evidence Behavior
Preserve criteria, actor, findings, condition reconciliation, referrals and closure approval.

## 8. Observable Completion Conditions
G10 Inspection Record is Closed and every significant finding has a traceable governed path.

## 9. Traceability & Provenance
Parent: L3-G-303; governing amendments dated 2026-09-01.

## 10. Open Issues / Deferred Design
None at L4.
