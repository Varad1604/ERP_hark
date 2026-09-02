# Vehicle Breakdown to Replacement Vehicle Assignment

## 1. Specification Metadata

**Specification ID:** L4-G-109  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-109 — Vehicle Breakdown to Replacement Vehicle Assignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-109; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an operational vehicle breakdown through identification of the affected assignment, replacement assessment and establishment of an eligible replacement vehicle assignment, while keeping maintenance execution outside Transport Management.

## 3. Required Business Context

- A Trip has an operational vehicle assignment.
- Vehicle ownership and maintenance execution remain outside G04.
- Replacement eligibility is governed by transport policy.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-109-BH-001 | Record the breakdown condition against the affected Trip and vehicle assignment. | L3-G-109 §§3, 9 |
| L4-G-109-BH-002 | Treat the affected vehicle as unavailable for the Trip when the breakdown prevents continued operation. | L3-G-109 §§9, 12–13 |
| L4-G-109-BH-003 | Evaluate eligible replacement vehicles against applicable operational constraints. | L3-G-109 §§9–12 |
| L4-G-109-BH-004 | Establish an authorised replacement vehicle assignment when conditions are satisfied. | L3-G-109 §§3, 9, 16 |
| L4-G-109-BH-005 | Where the breakdown indicates maintenance need, establish the G04 maintenance recommendation/service need without executing maintenance. | L3-G-109 §§9, 11–12, 16 |
| L4-G-109-BH-006 | Preserve the original and replacement assignment history and any maintenance recommendation evidence. | L3-G-109 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-109-BH-007 | The exact canonical breakdown/replacement event identifier is unresolved upstream; this L4 specification shall not invent one. The authoritative replacement assignment and any maintenance recommendation shall remain available to their approved consumers while maintenance execution remains G01-owned. | L3-G-109 §§14, 17 |

## 6. Failure / Rejection / Continuation

- An ineligible replacement shall not be assigned.
- If no replacement is available, the Trip shall remain without restored vehicle readiness until an authorised transport decision resolves the condition.
- Maintenance execution follows the separate G01-owned process.

## 7. Audit & Evidence Behavior

Preserve breakdown observation, affected assignment, replacement assessment, authority, replacement assignment, maintenance recommendation where applicable, and historical evidence.

## 8. Observable Completion Conditions

A valid replacement vehicle assignment is established and traceable; where maintenance is indicated, the recommendation/service need is recorded without claiming maintenance completion.

## 9. Traceability & Provenance

**Parent:** L3-G-109 — Vehicle Breakdown to Replacement Vehicle Assignment  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact canonical breakdown/replacement event vocabulary remains unresolved upstream and shall not be invented at L4. Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
