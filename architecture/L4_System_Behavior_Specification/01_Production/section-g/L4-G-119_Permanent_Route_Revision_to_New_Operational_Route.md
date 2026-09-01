# L4-G-119 — Permanent Route Revision to New Operational Route

## 1. Specification Metadata
**Specification ID:** L4-G-119  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-119 — Permanent Route Revision to New Operational Route  
**Primary Business Owner:** Transport Management  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern an authorised permanent revision of an operational route through establishment of the resulting route context.

**Start:** A permanent route revision is approved for operational use.  
**End:** The revised route is established as the applicable operational route.

## 3. Required Business Context
The existing route, proposed revision, route authority and affected operational context shall be authoritative.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-119-BH-001 | Identify the route subject to permanent revision. | L3-G-119 |
| L4-G-119-BH-002 | Evaluate the proposed route revision against applicable route rules and operational constraints. | L3-G-119 |
| L4-G-119-BH-003 | Record the authorised route revision. | L3-G-119 |
| L4-G-119-BH-004 | Establish the revised route as the new operational route when effective. | L3-G-119 |
| L4-G-119-BH-005 | Preserve the prior route version and revision history. | L3-G-119 |

## 5. Cross-Domain & Event Behavior
Transport Management retains operational route ownership. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
Invalid or unauthorised revisions shall not become operational. Affected trips shall remain governed by their valid route until the revision is effective.

## 7. Audit & Evidence Behavior
Preserve proposal, evaluation, authority, effective revision and historical route evidence.

## 8. Observable Completion Conditions
The revised route is authorised and operationally effective, and the superseded route remains historically traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-119 — Permanent Route Revision to New Operational Route

## 10. Open Issues / Deferred Design
Technical routing, version persistence, interfaces and implementation mechanisms remain outside L4.