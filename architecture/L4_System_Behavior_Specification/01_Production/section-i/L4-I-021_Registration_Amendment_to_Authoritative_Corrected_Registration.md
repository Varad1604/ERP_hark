# Registration Amendment to Authoritative Corrected Registration

## 1. Specification Metadata

**Specification ID:** L4-I-021  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-021 — Registration Amendment to Authoritative Corrected Registration  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-I-021; applicable locked I architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a permitted amendment to an existing Examination Registration through validation and required approval so the corrected registration becomes authoritative without erasing prior history.

## 3. Required Business Context

- Existing Examination Registration.
- Candidate and examination context.
- Applicable registration policy.
- Academic/subject context where relevant.
- Amendment authority.
- Registration locking position.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-021-BH-001 | The system shall identify the amendment requirement against the authoritative Examination Registration. | L3-I-021 §§2, 4, 7–9 |
| L4-I-021-BH-002 | The system shall validate the requested amendment against applicable registration, policy, academic, and subject requirements. | L3-I-021 §§4, 7, 12 |
| L4-I-021-BH-003 | The system shall require applicable institutional approval before the amendment becomes authoritative when approval is required. | L3-I-021 §§3, 9, 15 |
| L4-I-021-BH-004 | The system shall prevent an unauthorised amendment, including an amendment to a locked registration without the required exceptional authority. | L3-I-021 §§7, 11, 12, 13 |
| L4-I-021-BH-005 | On approved amendment, the system shall establish the corrected Examination Registration as the authoritative registration position. | L3-I-021 §§3, 9, 16 |
| L4-I-021-BH-006 | The system shall preserve the prior registration position and amendment history for auditability. | L3-I-021 §§3, 12, 15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-021-BH-007 | On approved amendment establishment, the system shall publish the canonical `Registration Amended` business event through Examination Registration. | L3-I-021 §14; L2-I04 |
| L4-I-021-BH-008 | The system shall consume applicable Student and Academic Delivery information without transferring ownership of those facts to Examination Registration. | L3-I-021 §§5, 14 |

## 6. Failure / Rejection / Continuation

- If the requested amendment is not permitted, the system shall leave the authoritative registration unchanged.
- If required validation fails, the system shall prevent establishment of the corrected registration.
- If required approval is rejected, the existing authoritative registration shall remain unchanged.
- If a locked registration lacks exceptional authority, the system shall prevent amendment.

## 7. Audit & Evidence Behavior

Preserve requested amendment, validation basis, approval, corrected registration position, prior position, amendment event, and attributable history.

## 8. Observable Completion Conditions

Completion occurs when the approved corrected registration is authoritative and `Registration Amended` is published, with prior history preserved; or when a governed rejection leaves the original registration authoritative.

## 9. Traceability & Provenance

**Parent:** L3-I-021 — Registration Amendment to Authoritative Corrected Registration  
**Primary verification source:** Authoritative Examination L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
