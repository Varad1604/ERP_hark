# Transport Request to Active Allocation

## 1. Specification Metadata

**Specification ID:** L4-G-099  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-099 — Transport Request to Active Allocation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-099; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a transport request through eligibility, allocation and activation of the authorised transport allocation.

## 3. Required Business Context

- The requesting subject and transport context are authoritative.
- An eligible route/stop context is available.
- Applicable allocation rules and authority are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-099-BH-001 | Register the transport request against its authoritative subject and operational context. | L3-G-099 |
| L4-G-099-BH-002 | Evaluate the request against applicable transport eligibility and allocation rules. | L3-G-099 |
| L4-G-099-BH-003 | Identify an eligible transport allocation context. | L3-G-099 |
| L4-G-099-BH-004 | Complete required approval/authorization before activation. | L3-G-099 |
| L4-G-099-BH-005 | Establish the authorised Transport Allocation as Active. | L3-G-099 |
| L4-G-099-BH-006 | Preserve allocation evidence and history. | L3-G-099 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-099-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Transport Management ownership. | L3-G-099 §14 |

## 6. Failure / Rejection / Continuation

- Ineligible requests shall not receive an active allocation.
- No available compatible allocation shall prevent activation until a valid business resolution exists.
- Failed approval shall retain the request in its applicable non-active position.

## 7. Audit & Evidence Behavior

Preserve request, eligibility assessment, allocation decision, authorization and active allocation evidence.

## 8. Observable Completion Conditions

An authorised valid Transport Allocation is Active and traceable to the originating transport request.

## 9. Traceability & Provenance

**Parent:** L3-G-099 — Transport Request to Active Allocation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
