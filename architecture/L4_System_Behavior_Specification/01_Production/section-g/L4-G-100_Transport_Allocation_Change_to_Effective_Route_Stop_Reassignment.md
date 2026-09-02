# Transport Allocation Change to Effective Route Stop Reassignment

## 1. Specification Metadata

**Specification ID:** L4-G-100  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-100 — Transport Allocation Change to Effective Route Stop Reassignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-100; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised change to an existing Transport Allocation and make the revised route/stop assignment effective while preserving allocation history.

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

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-100-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Transport Management ownership. | L3-G-100 §14 |

## 6. Failure / Rejection / Continuation

- Invalid target route/stop context shall prevent reassignment.
- Failed authorization shall preserve the current allocation.
- Incomplete validation shall prevent the revised allocation from becoming effective.

## 7. Audit & Evidence Behavior

Preserve original allocation, requested change, validation, authorization, effective reassignment and historical evidence.

## 8. Observable Completion Conditions

The authorised revised route/stop assignment is effective, the prior allocation remains traceable, and the change evidence is preserved.

## 9. Traceability & Provenance

**Parent:** L3-G-100 — Transport Allocation Change to Effective Route Stop Reassignment  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
