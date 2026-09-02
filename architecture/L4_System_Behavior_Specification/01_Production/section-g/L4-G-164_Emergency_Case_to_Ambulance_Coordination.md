# Emergency Case to Ambulance Coordination

## 1. Specification Metadata

**Specification ID:** L4-G-164  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-164 — Emergency Case to Ambulance Coordination  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-164; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall record truthful institutional ambulance coordination for an Emergency Case when ambulance transport is medically required, without assuming ownership of external ambulance fleet, vehicle or crew operations.

## 3. Required Business Context

- Emergency Case and patient context are established.
- Ambulance need is determined by authorised healthcare authority.
- Provider and destination can be established.
- Departure and arrival confirmation are recorded only when evidenced.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-164-BH-001 | Establish the ambulance request and associate it with the Emergency Case and patient. | L3-G-164 §§3,8–9 |
| L4-G-164-BH-002 | Record the authorised provider and destination without creating an external fleet lifecycle. | L3-G-164 §§4,9,12 |
| L4-G-164-BH-003 | Record departure time only when departure is evidenced. | L3-G-164 §§9,11,16 |
| L4-G-164-BH-004 | Record arrival confirmation only when available and distinguish confirmed from unconfirmed arrival. | L3-G-164 §§9,11,16 |
| L4-G-164-BH-005 | Preserve traceability from ambulance coordination to the originating Emergency Case and any referral. | L3-G-164 §§9,15–16 |
| L4-G-164-BH-006 | Where configured, provide emergency facts to H for notification delivery without transferring G06 ownership. | L3-G-164 §§5,9,14 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-164-BH-007 | No dedicated canonical ambulance event name is exposed in the Locked parent. No event shall be invented; G06 remains owner of the institutional coordination facts and H remains notification owner. | L3-G-164 §§14,17 |

## 6. Failure / Rejection / Continuation

If provider, destination, departure or arrival evidence is unresolved, preserve the unresolved condition rather than asserting completion. If medical reassessment removes ambulance need, record that governed outcome without representing transport as having occurred.

## 7. Audit & Evidence Behavior

Preserve Emergency Case, Ambulance Request, provider, destination, departure time where applicable, arrival confirmation where available, responsible healthcare actor and coordination outcome.

## 8. Observable Completion Conditions

Completion occurs when the applicable ambulance coordination facts are truthfully recorded and linked to the Emergency Case; external ambulance operations remain outside G06.

## 9. Traceability & Provenance

**Parent:** L3-G-164 — Emergency Case to Ambulance Coordination  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Provider selection, arrival evidence and a canonical ambulance event remain unresolved upstream. Technical dispatch, APIs and external fleet operations are outside scope/deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
