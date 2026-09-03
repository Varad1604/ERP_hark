# Route Suspension to Operational Transport Restriction

## 1. Specification Metadata

**Specification ID:** L4-G-095  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-095 — Route Suspension to Operational Transport Restriction  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-095; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern suspension of an operational Transport Route so that the route reaches Suspended, further operational dispatch against it is restricted, affected trip/allocation consequences required by the parent scenario are represented, and the suspension outcome remains traceable.

## 3. Required Business Context

- Transport Management owns the Transport Route lifecycle.
- The route is Active and the suspension basis is identifiable.
- Required suspension authority is available.
- Affected scheduled trips and commuter allocations can be identified from authoritative transport context.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-095-BH-001 | Identify the Active Transport Route and the authoritative suspension basis. | L3-G-095 §§7–9 |
| L4-G-095-BH-002 | Validate the suspension against the applicable operational authority and current route position. | L3-G-095 §§7, 9, 15 |
| L4-G-095-BH-003 | When suspension is authorised, transition the Transport Route from Active to Suspended. | L3-G-095 §§9, 13, 16 |
| L4-G-095-BH-004 | While the route is Suspended, prevent new trip dispatch against that route. | L3-G-095 §§3–4, 16 |
| L4-G-095-BH-005 | Identify affected scheduled trips and apply the parent-required cancellation or re-routing outcome through the authoritative transport process. | L3-G-095 §§3–4, 9, 13 |
| L4-G-095-BH-006 | Reflect the parent-required operational restriction or re-routing consequence for affected passenger allocations without transferring Student Lifecycle or Workforce ownership. | L3-G-095 §§3, 5, 9 |
| L4-G-095-BH-007 | Preserve the suspension authority, basis, affected operational consequences, and historical route evidence. | L3-G-095 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-095-BH-008 | On authoritative route suspension, Transport Management shall publish `TransportRouteSuspended` for the parent-defined downstream consumers. | L3-G-095 §§9, 14, 16 |
| L4-G-095-BH-009 | The authoritative suspension outcome shall be available for governed communication to affected commuters; communication ownership remains with the participating Institutional Services domain. | L3-G-095 §§3–5, 14 |

## 6. Failure / Rejection / Continuation

- An unauthorised or unresolved suspension shall not change the route from its current authoritative state.
- If the route is not Active, this suspension path shall not force an unsupported lifecycle transition.
- If an affected trip or allocation cannot be resolved through a parent-supported cancellation/re-routing outcome, the unresolved operational condition shall remain visible for governed Transport Management resolution rather than being silently completed.

## 7. Audit & Evidence Behavior

Preserve the suspension trigger and evidence, authorizing decision, effective suspension, affected trip/allocation consequences, publication of `TransportRouteSuspended`, and resulting operational history.

## 8. Observable Completion Conditions

The Transport Route is Suspended, new dispatch against it is restricted, the parent-required affected trip/allocation consequences are represented, `TransportRouteSuspended` is published, and suspension evidence is preserved.

## 9. Traceability & Provenance

**Parent:** L3-G-095 — Route Suspension to Operational Transport Restriction  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The locked parent names `Diverted` as an alternate route position but does not make that state part of this scenario's primary lifecycle path; this specification does not introduce it into the suspension transition. Technical notification channels, dispatch mechanisms, APIs, persistence, UI, routing algorithms, and retry mechanics are deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
