# Vehicle Availability to Operational Vehicle Assignment

## 1. Specification Metadata

**Specification ID:** L4-G-108  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-108 — Vehicle Availability to Operational Vehicle Assignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-108; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish an operational vehicle assignment for a Trip from an available vehicle context without transferring vehicle ownership to Transport Management.

## 3. Required Business Context

- Trip and route context are authoritative.
- Vehicle availability can be evaluated.
- Vehicle asset ownership remains outside G04.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-108-BH-001 | Identify the Trip requirement for operational vehicle assignment. | L3-G-108 §§3, 7, 9 |
| L4-G-108-BH-002 | Evaluate available vehicles against applicable operational eligibility conditions. | L3-G-108 §§9, 12 |
| L4-G-108-BH-003 | Establish the authorised operational vehicle assignment. | L3-G-108 §§3, 9, 16 |
| L4-G-108-BH-004 | Prevent assignment of a vehicle that fails applicable operational conditions. | L3-G-108 §§10–12 |
| L4-G-108-BH-005 | Preserve assignment history and the vehicle context used for the decision. | L3-G-108 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-108-BH-006 | The exact canonical event identifier for operational Vehicle Assignment is unresolved upstream; this L4 specification shall not invent one. The effective G04-owned operational vehicle assignment shall remain available to authorised downstream consumers. | L3-G-108 §§14, 17 |

## 6. Failure / Rejection / Continuation

- No eligible vehicle means the Trip cannot satisfy vehicle-dependent readiness conditions.
- An unavailable or ineligible vehicle shall not be assigned.
- Vehicle maintenance or ownership decisions remain outside this scenario.

## 7. Audit & Evidence Behavior

Preserve vehicle availability assessment, eligibility basis, authority, assignment outcome and historical assignment changes.

## 8. Observable Completion Conditions

An eligible vehicle is operationally assigned to the Trip and the assignment is traceable, without creating vehicle ownership in G04.

## 9. Traceability & Provenance

**Parent:** L3-G-108 — Vehicle Availability to Operational Vehicle Assignment  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The exact canonical Vehicle Assignment event identifier remains unresolved upstream and shall not be invented at L4. Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
