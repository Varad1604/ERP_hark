# Curfew Violation to Disciplinary Action

## 1. Specification Metadata

**Specification ID:** L4-G-086  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-086 — Curfew Violation to Disciplinary Action  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-086; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a recorded curfew violation through assessment and the applicable disciplinary action without inventing a Student Lifecycle transition.

## 3. Required Business Context

- An identifiable resident and hostel context exist.
- The applicable curfew rule is authoritative.
- Hostel disciplinary authority is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-086-BH-001 | The system shall register the curfew violation against the applicable resident and hostel context. | L3-G-086 §§2, 8, 9 |
| L4-G-086-BH-002 | The system shall evaluate the recorded violation against the applicable curfew and disciplinary criteria. | L3-G-086 §§3, 7, 12 |
| L4-G-086-BH-003 | the system shall govern the governed disciplinary review and decision process. | L3-G-086 §§3, 9 |
| L4-G-086-BH-004 | The system shall record the applicable disciplinary action or governed resolution. | L3-G-086 §§3, 13, 16 |
| L4-G-086-BH-005 | The system shall preserve attributable evidence of the violation, decision authority, decision basis, and resulting action. | L3-G-086 §§15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-086-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Hostel Management ownership. | L3-G-086 §14 |

## 6. Failure / Rejection / Continuation

- If the violation cannot be substantiated under applicable business rules, the system shall retain the appropriate non-disciplinary resolution.
- If required decision authority is unavailable, the system shall not finalise an unauthorised disciplinary action.
- Student Lifecycle shall not be changed by this scenario unless a separate authorised scenario governs such a transition.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of violation registration, applicable rule, review, decision authority, disciplinary action, and final resolution.

## 8. Observable Completion Conditions

Completion occurs when the curfew violation reaches a governed disciplinary action or other supported resolution and the relevant history is preserved.

## 9. Traceability & Provenance

**Parent:** L3-G-086 — Curfew Violation to Disciplinary Action  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
