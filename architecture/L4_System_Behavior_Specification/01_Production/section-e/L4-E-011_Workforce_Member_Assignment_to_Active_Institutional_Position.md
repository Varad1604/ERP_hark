# Workforce Member Assignment to Active Institutional Position

## 1. Specification Metadata

**Specification ID:** L4-E-011  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-011 — Workforce Member Assignment to Active Institutional Position  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-011; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Specify observable behavior for establishing an authoritative Position Assignment linking an active Workforce Engagement to an eligible institutional Position.

## 3. Required Business Context

- Authoritative Position & Assignment context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-011-BH-001 | The system shall resolve Workforce Engagement and Position Definition. | L3-E-011 §§3, 9, 12 |
| L4-E-011-BH-002 | The system shall validate occupancy eligibility and dates. | L3-E-011 §§3, 9, 12 |
| L4-E-011-BH-003 | The system shall establish Position Assignment. | L3-E-011 §§3, 9, 12 |
| L4-E-011-BH-004 | The system shall apply required approval/activation lifecycle. | L3-E-011 §§3, 9, 12 |
| L4-E-011-BH-005 | The system shall preserve assignment history. | L3-E-011 §§3, 9, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-011-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Position & Assignment ownership. | L3-E-011 §14 |

## 6. Failure / Rejection / Continuation

- If validation or governance fails, no new operational assignment outcome is established and the existing authoritative Workforce Engagement remains unchanged. Technical recovery is outside L4.

## 7. Audit & Evidence Behavior

Assignment approval and activation follow E02 governance and remain auditable. No fixed technical approver is introduced.

## 8. Observable Completion Conditions

- Position Assignment reaches the authoritative operational outcome. - Organisational occupancy is established where applicable. - Effective dates and history are preserved.

## 9. Traceability & Provenance

**Parent:** L3-E-011 — Workforce Member Assignment to Active Institutional Position  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
