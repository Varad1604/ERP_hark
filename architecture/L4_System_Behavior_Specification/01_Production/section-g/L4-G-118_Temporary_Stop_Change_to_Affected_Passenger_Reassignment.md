# L4-G-118 — Temporary Stop Change to Affected Passenger Reassignment

## 1. Specification Metadata
**Specification ID:** L4-G-118  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-118 — Temporary Stop Change to Affected Passenger Reassignment  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern a temporary stop change and reassignment of affected passengers to the resulting operational stop context.

**Start:** A temporary stop change is authorised or required.  
**End:** Affected passenger assignments reflect the governed temporary stop arrangement.

## 3. Required Business Context
The affected trip/route, temporary stop, passenger context and applicable operational authority shall be authoritative.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-118-BH-001 | Identify the affected operational trip and stop context. | L3-G-118 |
| L4-G-118-BH-002 | Establish the authorised temporary stop change. | L3-G-118 |
| L4-G-118-BH-003 | Identify passengers affected by the stop change. | L3-G-118 |
| L4-G-118-BH-004 | Reassign affected passengers to the governed temporary stop context. | L3-G-118 |
| L4-G-118-BH-005 | Preserve original and temporary stop assignment history. | L3-G-118 |

## 5. Cross-Domain & Event Behavior
Transport Management remains owner of operational transport assignment. Student/passenger identity remains with its canonical owner. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
An unauthorised stop change shall not be applied. Passenger reassignment shall not be completed where the temporary stop is operationally invalid.

## 7. Audit & Evidence Behavior
Preserve authority, affected stop, temporary stop, passenger reassignment and effective-period evidence.

## 8. Observable Completion Conditions
The temporary stop arrangement is authoritative, affected passenger assignments are governed against it, and prior assignment history remains traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-118 — Temporary Stop Change to Affected Passenger Reassignment

## 10. Open Issues / Deferred Design
Technical geospatial services, persistence, interfaces and notification delivery remain outside L4.