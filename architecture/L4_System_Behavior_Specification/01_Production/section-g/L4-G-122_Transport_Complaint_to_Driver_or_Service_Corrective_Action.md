# Transport Complaint to Driver or Service Corrective Action

## 1. Specification Metadata

**Specification ID:** L4-G-122  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-122 — Transport Complaint to Driver or Service Corrective Action  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-122; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a transport complaint through assessment and an authorised corrective action concerning driver conduct or transport service.

## 3. Required Business Context

- Authoritative Transport Management context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-122-BH-001 | Register the complaint and affected transport context. | L3-G-122 |
| L4-G-122-BH-002 | Preserve evidence required to assess the alleged driver or service deficiency. | L3-G-122 |
| L4-G-122-BH-003 | Evaluate the complaint through the governed review path. | L3-G-122 |
| L4-G-122-BH-004 | Record an authorised corrective action where the review establishes that action is warranted. | L3-G-122 |
| L4-G-122-BH-005 | Preserve the complaint, decision, corrective-action and hand-off history. | L3-G-122 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-122-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Transport Management ownership. | L3-G-122 §14 |

## 6. Failure / Rejection / Continuation

- Insufficient evidence shall prevent unsupported corrective action. If the complaint is not substantiated, the governed non-corrective resolution shall be recorded. Required authority must exist before corrective action is finalised.

## 7. Audit & Evidence Behavior

Preserve complaint evidence, assessment, authority, decision, corrective action and cross-domain hand-off evidence.

## 8. Observable Completion Conditions

The complaint reaches a governed resolution, any authorised corrective action is recorded, and relevant history remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-122 — Transport Complaint to Driver or Service Corrective Action  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
