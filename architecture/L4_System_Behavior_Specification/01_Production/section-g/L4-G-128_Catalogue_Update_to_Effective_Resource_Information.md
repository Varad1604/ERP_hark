# Catalogue Update to Effective Resource Information

## 1. Specification Metadata

**Specification ID:** L4-G-128  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-128 — Catalogue Update to Effective Resource Information  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-128; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised catalogue update through validation and effective adoption while preserving the prior authoritative resource information.

## 3. Required Business Context

- An authoritative catalogue record exists.
- Applicable cataloguing governance is available.
- Library Management owns catalogue information.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-128-BH-001 | Register the proposed catalogue update against the authoritative catalogue record. | L3-G-128 |
| L4-G-128-BH-002 | Validate changed information against applicable cataloguing rules. | L3-G-128 |
| L4-G-128-BH-003 | Apply the governed review and authorisation path. | L3-G-128 |
| L4-G-128-BH-004 | On approval, establish the changed information as effective catalogue information. | L3-G-128 |
| L4-G-128-BH-005 | Preserve prior information and update history for traceability. | L3-G-128 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-128-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Library Management ownership. | L3-G-128 §14 |

## 6. Failure / Rejection / Continuation

- Invalid updates shall not become effective.
- Missing authorisation shall retain the update in its applicable governance position.
- Effective changes shall preserve prior historical information.

## 7. Audit & Evidence Behavior

Preserve proposal, validation, authority, effective outcome, and historical versions required for traceability.

## 8. Observable Completion Conditions

Completion occurs when the authorised catalogue update is effective and historical traceability is preserved, or a governed terminal rejection is recorded.

## 9. Traceability & Provenance

**Parent:** L3-G-128 — Catalogue Update to Effective Resource Information  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
