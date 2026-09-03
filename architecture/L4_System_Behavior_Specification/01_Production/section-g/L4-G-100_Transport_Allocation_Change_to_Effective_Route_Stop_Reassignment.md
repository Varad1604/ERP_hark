# Transport Allocation Change to Effective Route/Stop Reassignment

## 1. Specification Metadata

**Specification ID:** L4-G-100  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-100 — Transport Allocation Change to Effective Route/Stop Reassignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-100; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised change to an existing Transport Allocation and make the revised route/stop relationship effective while preserving allocation history.

## 3. Required Business Context

- An existing Transport Allocation is authoritative.
- The current and requested route/stop relationships are identifiable.
- The target Route is active and the target Stop is operationally available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-100-BH-001 | Identify the existing allocation, its current route/stop relationship, and the requested reassignment. | L3-G-100 §§7–9 |
| L4-G-100-BH-002 | Validate the target Route and Stop against the applicable allocation constraints before reassignment. | L3-G-100 §§7, 9, 12 |
| L4-G-100-BH-003 | Complete required impact assessment and authorization before the new relationship becomes effective. | L3-G-100 §§9, 15–16 |
| L4-G-100-BH-004 | Make the authorised revised route/stop relationship effective without creating a new allocation lifecycle state. | L3-G-100 §§3, 13, 16 |
| L4-G-100-BH-005 | Preserve the previous and revised allocation relationships in allocation history. | L3-G-100 §§9, 12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-100-BH-006 | On effective allocation reassignment, Transport Management shall publish `TransportAllocationUpdated`. | L3-G-100 §§9, 14, 16 |
| L4-G-100-BH-007 | The changed allocation fact may be consumed by the parent-defined participating domains without transferring Transport Allocation, Student, Workforce, or communication ownership. | L3-G-100 §§5, 14 |

## 6. Failure / Rejection / Continuation

- An inactive target Route or unavailable target Stop shall prevent reassignment.
- Rejected authorization or failed validation shall preserve the current allocation relationship.
- A request requiring route redesign rather than allocation reassignment shall remain with the separate route scenario.

## 7. Audit & Evidence Behavior

Preserve original allocation, requested change, target validation, authorization, effective reassignment, `TransportAllocationUpdated` publication, and historical evidence.

## 8. Observable Completion Conditions

The authorised revised route/stop relationship is effective, `TransportAllocationUpdated` is published, and the prior relationship remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-100 — Transport Allocation Change to Effective Route/Stop Reassignment  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No material upstream event or lifecycle gap remains for the reassignment outcome. Technical APIs, persistence, UI, communication delivery mechanics, and retry behavior are deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
