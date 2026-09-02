# Transport Pass Issuance to Active Transport Authorisation

## 1. Specification Metadata

**Specification ID:** L4-G-113  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-113 — Transport Pass Issuance to Active Transport Authorisation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-113; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an eligible transport pass from issuance through active transport authorisation.

## 3. Required Business Context

- Canonical student/person context is available from its owning domain.
- Transport eligibility and policy are authoritative.
- Transport Management owns the Transport Pass lifecycle.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-113-BH-001 | Establish a Transport Pass request against the applicable authorised transport context. | L3-G-113 |
| L4-G-113-BH-002 | Evaluate eligibility and required approval conditions. | L3-G-113 |
| L4-G-113-BH-003 | Issue the Transport Pass when all authoritative conditions are satisfied. | L3-G-113 |
| L4-G-113-BH-004 | Transition the Transport Pass to Active authorisation. | L3-G-113 |
| L4-G-113-BH-005 | Preserve issuance, authority, validity and lifecycle history. | L3-G-113 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-113-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Transport Management ownership. | L3-G-113 §14 |

## 6. Failure / Rejection / Continuation

- Ineligible or incomplete requests shall not become Active. Missing authoritative context shall keep issuance unresolved until corrected.

## 7. Audit & Evidence Behavior

Preserve attributable evidence of eligibility, approval, issuance, validity and activation.

## 8. Observable Completion Conditions

The pass is issued, Active authorisation is recorded, validity is governed, and issuance history is traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-113 — Transport Pass Issuance to Active Transport Authorisation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
