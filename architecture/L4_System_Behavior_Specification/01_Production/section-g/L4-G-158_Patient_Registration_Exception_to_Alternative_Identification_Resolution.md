# Patient Registration Exception to Alternative Identification Resolution

## 1. Specification Metadata

**Specification ID:** L4-G-158  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-158 — Patient Registration Exception to Alternative Identification Resolution  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-158; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall preserve and resolve a G06 Patient Registration identification exception through an authorised alternative identification path, while retaining C/E ownership of institutional identity and avoiding creation of substitute canonical person identities.

## 3. Required Business Context

- Normal Patient Registration has encountered an identification/validation exception.
- The exception cause and applicable source identity domain can be determined.
- C/E authoritative identity/personnel context may be consulted where relevant.
- A may provide foundation capability without acquiring business ownership.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-158-BH-001 | Record the attempted registration and authoritative exception reason. | L3-G-158 §§3,8–9 |
| L4-G-158-BH-002 | Determine the applicable authoritative identity source and obtain source-owned context without creating a replacement canonical identity. | L3-G-158 §§5,9,12 |
| L4-G-158-BH-003 | Apply only an authorised alternative identification/resolution procedure and validate the resulting patient context against configurable registration rules. | L3-G-158 §§9,12,15 |
| L4-G-158-BH-004 | When sufficient identity/context is established, retrieve medical history where available, register the healthcare visit and establish `Consultation Ready`. | L3-G-158 §§9,13,16 |
| L4-G-158-BH-005 | If sufficient authoritative identity/context cannot be established, keep the exception unresolved and do not create an invented visit identity. | L3-G-158 §§10–11 |
| L4-G-158-BH-006 | Preserve the exception, consulted source, resolution evidence and registration result as auditable history. | L3-G-158 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-158-BH-007 | No dedicated exception-resolution event is defined upstream. When the visit is successfully registered after resolution, publish the existing canonical `PatientRegistered` event from G06; do not invent `AlternativeIdentityResolved` or equivalent. | L3-G-158 §§14,16–17 |

## 6. Failure / Rejection / Continuation

Do not infer student/employee status, attach the wrong medical history, or create a G06-owned person identity. Where emergency handling applies, preserve the emergency-care boundary while the identity exception remains traceable.

## 7. Audit & Evidence Behavior

Preserve original registration context, exception reason, authoritative source consulted, alternative-identification evidence/context, resolution decision, responsible actor and resulting registration outcome.

## 8. Observable Completion Conditions

Completion occurs when sufficient authorised identity/context is established, the visit is registered and `PatientRegistered` is published, or when the exception remains explicitly unresolved without fabricated identity or visit facts.

## 9. Traceability & Provenance

**Parent:** L3-G-158 — Patient Registration Exception to Alternative Identification Resolution  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The exact alternative-identification procedure/evidence set and a dedicated exception event are not defined upstream. L4 preserves these gaps. Technical matching, authentication, APIs and persistence are deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
