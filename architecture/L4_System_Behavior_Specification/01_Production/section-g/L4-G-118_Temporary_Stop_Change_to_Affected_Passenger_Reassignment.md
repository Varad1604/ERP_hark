# Temporary Stop Change to Affected Passenger Reassignment

## 1. Specification Metadata

**Specification ID:** L4-G-118  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-118 — Temporary Stop Change to Affected Passenger Reassignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-118; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a temporary stop change and reassignment of affected passengers to the resulting operational stop context.

## 3. Required Business Context

- Authoritative Transport Management context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-118-BH-001 | Identify the affected operational trip and stop context. | L3-G-118 |
| L4-G-118-BH-002 | Establish the authorised temporary stop change. | L3-G-118 |
| L4-G-118-BH-003 | Identify passengers affected by the stop change. | L3-G-118 |
| L4-G-118-BH-004 | Reassign affected passengers to the governed temporary stop context. | L3-G-118 |
| L4-G-118-BH-005 | Preserve original and temporary stop assignment history. | L3-G-118 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-118-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Transport Management ownership. | L3-G-118 §14 |

## 6. Failure / Rejection / Continuation

- An unauthorised stop change shall not be applied. Passenger reassignment shall not be completed where the temporary stop is operationally invalid.

## 7. Audit & Evidence Behavior

Preserve authority, affected stop, temporary stop, passenger reassignment and effective-period evidence.

## 8. Observable Completion Conditions

The temporary stop arrangement is authoritative, affected passenger assignments are governed against it, and prior assignment history remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-118 — Temporary Stop Change to Affected Passenger Reassignment  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
