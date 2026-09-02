# Vehicle Breakdown to Replacement Vehicle Assignment

## 1. Specification Metadata

**Specification ID:** L4-G-109  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-109 — Vehicle Breakdown to Replacement Vehicle Assignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-109; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an operational vehicle breakdown through identification of the affected assignment, replacement assessment and establishment of an eligible replacement vehicle assignment.

## 3. Required Business Context

- A Trip has an operational vehicle assignment.
- Vehicle ownership and maintenance remain outside G04.
- Replacement eligibility is governed by transport policy.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-109-BH-001 | Record the breakdown condition against the affected Trip and vehicle assignment. | L3-G-109 |
| L4-G-109-BH-002 | Remove the affected vehicle from operational use for the Trip when the breakdown makes it unavailable. | L3-G-109 |
| L4-G-109-BH-003 | Evaluate eligible replacement vehicles against applicable operational constraints. | L3-G-109 |
| L4-G-109-BH-004 | Establish an authorised replacement vehicle assignment when conditions are satisfied. | L3-G-109 |
| L4-G-109-BH-005 | Preserve the original and replacement assignment history. | L3-G-109 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-109-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Transport Management ownership. | L3-G-109 §14 |

## 6. Failure / Rejection / Continuation

- An ineligible replacement shall not be assigned.
- If no replacement is available, the Trip shall remain in its applicable exception/non-ready position.
- Maintenance action follows the separate owning process.

## 7. Audit & Evidence Behavior

Preserve breakdown observation, affected assignment, replacement assessment, authority, replacement assignment and historical evidence.

## 8. Observable Completion Conditions

A valid replacement vehicle assignment is established and traceable, or a supported non-assignment outcome is recorded.

## 9. Traceability & Provenance

**Parent:** L3-G-109 — Vehicle Breakdown to Replacement Vehicle Assignment  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
