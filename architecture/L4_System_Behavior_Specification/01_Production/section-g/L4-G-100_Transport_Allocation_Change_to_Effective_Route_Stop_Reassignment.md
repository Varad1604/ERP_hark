# L4-G-100 — Transport Allocation Change to Effective Route/Stop Reassignment

## 1. Specification Metadata
**Specification ID:** L4-G-100  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-100 — Transport Allocation Change to Effective Route/Stop Reassignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-100; L2-G04 Transport Management Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern an authorised change to an existing Transport Allocation and make the revised route/stop assignment effective while preserving allocation history.

**Start:** A transport allocation change is requested.  
**End:** The authorised revised route/stop allocation is effective.

## 3. Required Business Context
- An existing Transport Allocation is authoritative.
- The requested reassignment and affected route/stop context are identifiable.
- Applicable allocation rules and authority are available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-100-BH-001 | Identify the existing allocation and requested route/stop reassignment. | L3-G-100 |
| L4-G-100-BH-002 | Evaluate the proposed reassignment against applicable allocation constraints. | L3-G-100 |
| L4-G-100-BH-003 | Validate the target route and stop context before reassignment. | L3-G-100 |
| L4-G-100-BH-004 | Complete required authorization for the allocation change. | L3-G-100 |
| L4-G-100-BH-005 | Make the authorised revised route/stop assignment effective. | L3-G-100 |
| L4-G-100-BH-006 | Preserve the prior allocation and reassignment history. | L3-G-100 |

## 5. Cross-Domain & Event Behavior
Transport Management remains the canonical owner and publisher of the allocation outcome. Exact event names are not invented.

## 6. Failure / Rejection / Continuation
- Invalid target route/stop context shall prevent reassignment.
- Failed authorization shall preserve the current allocation.
- Incomplete validation shall prevent the revised allocation from becoming effective.

## 7. Audit & Evidence Behavior
Preserve original allocation, requested change, validation, authorization, effective reassignment and historical evidence.

## 8. Observable Completion Conditions
The authorised revised route/stop assignment is effective, the prior allocation remains traceable, and the change evidence is preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-100 — Transport Allocation Change to Effective Route/Stop Reassignment  
**Authoritative workflow:** L2-G04 — Transport Management Workflows

## 10. Open Issues / Deferred Design
Technical allocation matching, persistence, APIs, notifications and implementation mechanics remain outside L4.