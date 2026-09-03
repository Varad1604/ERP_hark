# Registration Reopening to Available Examination Registration

## 1. Specification Metadata

**Specification ID:** L4-I-025  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-025 — Registration Reopening to Available Examination Registration  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-025; applicable locked I architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern reopening of an Examination Registration so that a valid registration becomes available for permitted further registration activity without creating a competing registration record.

## 3. Required Business Context

- Existing Examination Registration.
- Registration lifecycle position eligible for reopening.
- Applicable reopening authority/policy.
- Candidate and examination context.
- Any required exceptional approval.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-025-BH-001 | The system shall identify the Examination Registration proposed for reopening and its current authoritative lifecycle position. | L3-I-025 §§2, 4, 7–9 |
| L4-I-025-BH-002 | The system shall validate reopening eligibility against the applicable registration policy and lifecycle conditions. | L3-I-025 §§7, 12, 13 |
| L4-I-025-BH-003 | The system shall require applicable institutional authority before reopening where reopening is restricted or exceptional. | L3-I-025 §§3, 9, 15 |
| L4-I-025-BH-004 | The system shall prevent reopening when the required business conditions or authority are not satisfied. | L3-I-025 §§10–12 |
| L4-I-025-BH-005 | On authorised reopening, the system shall return the Examination Registration to the parent-supported available registration position. | L3-I-025 §§3, 9, 13, 16 |
| L4-I-025-BH-006 | The system shall preserve the prior lifecycle position and reopening history. | L3-I-025 §§12, 15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-025-BH-007 | The system shall use the existing canonical I04 registration event vocabulary for the reopened registration and shall not invent a dedicated reopening event. | L3-I-025 §14; L2-I04 |
| L4-I-025-BH-008 | The system shall retain ownership of student, academic, and other source-domain facts with their canonical owners. | L3-I-025 §5 |

## 6. Failure / Rejection / Continuation

- If reopening is not permitted in the current lifecycle position, the system shall leave the authoritative registration unchanged.
- If required exceptional authority is unavailable, reopening shall not be completed.
- If the registration context is no longer valid, reopening shall be prevented until the applicable business condition is resolved.

## 7. Audit & Evidence Behavior

Preserve reopening basis, current state, authority, decision, resulting available position, and lifecycle history.

## 8. Observable Completion Conditions

Completion occurs when the Examination Registration is authoritatively reopened to its permitted available position with traceable history, or reopening is rejected without changing the authoritative registration.

## 9. Traceability & Provenance

**Parent:** L3-I-025 — Registration Reopening to Available Examination Registration  
**Primary verification source:** Authoritative Examination L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
