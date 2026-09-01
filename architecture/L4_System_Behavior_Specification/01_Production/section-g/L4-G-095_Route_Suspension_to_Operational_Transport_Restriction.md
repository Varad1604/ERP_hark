# L4-G-095 — Route Suspension to Operational Transport Restriction

## 1. Specification Metadata
**Specification ID:** L4-G-095  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-095 — Route Suspension to Operational Transport Restriction  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-G-095; L2-G04 Transport Management Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern suspension of an operational Transport Route so that the route is restricted from further operational use while preserving its historical state and ownership.

**Start:** A valid route suspension condition or request is established.  
**End:** The route is suspended and operational use is restricted.

## 3. Required Business Context
- Transport Management owns the Transport Route lifecycle.
- The route and suspension basis are identifiable.
- Required authority for suspension is available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-095-BH-001 | Identify the affected Transport Route and suspension basis. | L3-G-095 §§2–5 |
| L4-G-095-BH-002 | Evaluate the suspension against applicable operational conditions and authority. | L3-G-095 §§3, 8–9 |
| L4-G-095-BH-003 | Record the authorised suspension outcome. | L3-G-095 §§3, 16 |
| L4-G-095-BH-004 | Restrict the suspended route from new operational use. | L3-G-095 §§2–3 |
| L4-G-095-BH-005 | Preserve the prior route history and suspension evidence. | L3-G-095 §§3, 16 |

## 5. Cross-Domain & Event Behavior
Transport Management remains the canonical owner and publisher of the route suspension outcome. Exact event naming is not invented where not frozen upstream.

## 6. Failure / Rejection / Continuation
- Unauthorised suspension shall not change the route's operational position.
- If the suspension basis is incomplete, the route shall remain governed by its current authoritative position until resolved.
- Downstream allocations and trips are separate scenarios and are not silently rewritten by this specification.

## 7. Audit & Evidence Behavior
Preserve suspension request/basis, authority, resulting route position and affected operational history.

## 8. Observable Completion Conditions
The route has reached the governed suspended position, new operational use is restricted, and suspension evidence is preserved.

## 9. Traceability & Provenance
**Parent:** L3-G-095 — Route Suspension to Operational Transport Restriction  
**Authoritative workflow:** L2-G04 — Transport Management Workflows

## 10. Open Issues / Deferred Design
Exact event names, technical restriction mechanisms, persistence and interfaces remain outside L4.