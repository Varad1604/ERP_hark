# Vehicle Operational Issue to Maintenance Recommendation

## 1. Specification Metadata

**Specification ID:** L4-G-110  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-110 — Vehicle Operational Issue to Maintenance Recommendation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-110; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall record a vehicle operational issue and establish an authorised maintenance recommendation without transferring maintenance or asset ownership to Transport Management.

## 3. Required Business Context

- Vehicle context is identifiable.
- The issue is relevant to transport operations.
- Maintenance execution remains with G01.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-110-BH-001 | Record the operational vehicle issue and affected operational context. | L3-G-110 §§3, 9 |
| L4-G-110-BH-002 | Assess the issue against applicable operational criteria. | L3-G-110 §§9, 12 |
| L4-G-110-BH-003 | Where maintenance is indicated, establish the G04-owned maintenance recommendation/service need. | L3-G-110 §§3, 9, 16 |
| L4-G-110-BH-004 | Make the recommendation/service need available to the maintenance-owning process without representing maintenance execution as a G04 outcome. | L3-G-110 §§4, 9, 12, 16 |
| L4-G-110-BH-005 | Preserve issue, assessment, recommendation and hand-off history. | L3-G-110 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-110-BH-006 | The exact canonical event identifier for the maintenance recommendation is unresolved upstream; this L4 specification shall not invent one. The authoritative G04 recommendation/service need shall remain available to the maintenance-owning process. | L3-G-110 §§14, 17 |

## 6. Failure / Rejection / Continuation

- Insufficient issue information shall prevent a final recommendation.
- Where maintenance is not indicated, record the applicable transport operational outcome without creating a maintenance recommendation.
- Maintenance execution shall not be performed as part of this scenario.

## 7. Audit & Evidence Behavior

Preserve issue observation, assessment basis, recommendation, authority and hand-off evidence.

## 8. Observable Completion Conditions

The operational issue is assessed; where maintenance is indicated, the recommendation/service need is established and available to the maintenance-owning process; maintenance execution remains outside this scenario.

## 9. Traceability & Provenance

**Parent:** L3-G-110 — Vehicle Operational Issue to Maintenance Recommendation  
**Primary verification source:** L3-G-110 §§3, 9, 11–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

The exact canonical maintenance-recommendation event identifier remains unresolved upstream and shall not be invented at L4. Technical APIs, persistence, UI, event transport and other implementation design remain deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
