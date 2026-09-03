# Permanent Route Revision to New Operational Route

## 1. Specification Metadata

**Specification ID:** L4-G-119  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-119 — Permanent Route Revision to New Operational Route  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-119; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised permanent revision of an operational route and establish the resulting route as the effective operational arrangement without inventing a new route lifecycle state.

## 3. Required Business Context

- Existing operational route is authoritative.
- Proposed permanent revision and affected operational context are identifiable.
- Transport Management owns the route revision.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-119-BH-001 | Identify the route subject to permanent revision and the business reason for change. | L3-G-119 §§3, 7–9 |
| L4-G-119-BH-002 | Evaluate the proposed route revision and affected operational context against applicable route policy and constraints. | L3-G-119 §§9, 12 |
| L4-G-119-BH-003 | Record the authorised route revision only after required review, impact assessment and approval. | L3-G-119 §§9, 15 |
| L4-G-119-BH-004 | Establish the revised route arrangement as authoritative for subsequent applicable transport operations without creating a `Revised` lifecycle state. | L3-G-119 §§3, 13, 16 |
| L4-G-119-BH-005 | Preserve prior route information and revision history. | L3-G-119 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-119-BH-006 | On effective permanent route revision, Transport Management shall publish `RouteModified`. Any affected allocation event shall be published only when its own authoritative allocation transition actually occurs. | L3-G-119 §§14, 16 |

## 6. Failure / Rejection / Continuation

- Invalid or rejected revisions shall not replace the existing authoritative route.
- A temporary requirement shall use the temporary diversion/stop-change scenario instead of being treated as permanent.
- Affected allocations or assignments shall change only through their authoritative G04 outcomes.

## 7. Audit & Evidence Behavior

Preserve existing route, revision reason, impact assessment, authority, effective revised route and historical route evidence.

## 8. Observable Completion Conditions

The revised route is authorised and effective, `RouteModified` is published, and the superseded route context remains historically traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-119 — Permanent Route Revision to New Operational Route  
**Primary verification source:** L3-G-119 §§3, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No unresolved event identifier remains for the core route-update outcome. Any future distinct route-version lifecycle or permanent-revision event requires upstream governance. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
