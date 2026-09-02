# Late Examination Registration

## 1. Specification Metadata

**Specification ID:** L4-I-019  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-019 — Late Examination Registration  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-I-019; applicable locked I architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern examination registration submitted after the normal registration window and permit confirmation only when an authorised late, extended, special, or exception path is satisfied.

## 3. Required Business Context

- Active Examination Session.
- Published Examination Schedule.
- Closed standard registration window.
- Applicable late/special registration policy or exception.
- Candidate and academic eligibility.
- Financial eligibility where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-019-BH-001 | The system shall identify that the registration request occurs outside the standard approved registration window. | L3-I-019 §§2, 7–9 |
| L4-I-019-BH-002 | The system shall determine whether an authorised late, extended, or special registration path applies. | L3-I-019 §§2, 9, 13 |
| L4-I-019-BH-003 | The system shall revalidate candidate eligibility, applicable subjects, and required financial eligibility before confirmation. | L3-I-019 §§4, 7, 9, 12 |
| L4-I-019-BH-004 | The system shall require the applicable institutional exception or approval before a late registration may progress to confirmation where such approval is required. | L3-I-019 §§3, 9, 15 |
| L4-I-019-BH-005 | On successful completion of all applicable requirements, the system shall confirm the Examination Registration using the canonical I04 lifecycle. | L3-I-019 §§3, 13, 16 |
| L4-I-019-BH-006 | The system shall preserve the late-registration basis, decision, and resulting registration history. | L3-I-019 §§2, 12, 15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-019-BH-007 | On confirmation, the system shall publish the canonical `Registration Confirmed` event through Examination Registration. | L3-I-019 §14; L2-I04 |
| L4-I-019-BH-008 | The system shall consume Student and Finance eligibility facts without transferring ownership of those facts to Examination Registration. | L3-I-019 §§5, 12, 14 |

## 6. Failure / Rejection / Continuation

- If no authorised late/special/extended path exists, the system shall prevent confirmation outside the approved window.
- If candidate eligibility, subject applicability, or required financial eligibility fails, the system shall prevent confirmation.
- If required exception approval is denied or unavailable, the system shall retain the request in its non-confirmed governed position.
- If a valid registration already exists, the system shall preserve that registration as authoritative and prevent duplication.

## 7. Audit & Evidence Behavior

Preserve standard-window context, late-registration policy/exception basis, eligibility outcomes, approval, confirmation/rejection, and relevant registration events.

## 8. Observable Completion Conditions

Completion occurs when an authorised late registration is Confirmed with traceable history and canonical confirmation event, or when the late request receives a governed non-confirmed terminal outcome.

## 9. Traceability & Provenance

**Parent:** L3-I-019 — Late Examination Registration  
**Primary verification source:** Authoritative Examination L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
