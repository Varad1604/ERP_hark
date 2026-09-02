# Curfew Violation to Disciplinary Action

## 1. Specification Metadata

**Specification ID:** L4-G-086  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-086 — Curfew Violation to Disciplinary Action  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-086; L2-G03 — Hostel Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Govern an unauthorised curfew violation through confirmation and the applicable hostel disciplinary action while preserving the distinction between an authorised curfew exception and a violation.

## 3. Required Business Context

- Active hostel residency exists.
- Applicable curfew rule is authoritative.
- No valid authorised exception covers the late return.
- Configured hostel disciplinary authority is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-086-BH-001 | The system shall register the curfew violation against the resident and applicable curfew condition. | L3-G-086 §§7–9 |
| L4-G-086-BH-002 | The system shall evaluate whether a valid authorised curfew exception exists before treating the condition as an unauthorised violation. | L3-G-086 §§9–12 |
| L4-G-086-BH-003 | When the violation is substantiated, the system shall progress it through the configured Hostel Discipline review and authority decision. | L3-G-086 §§6, 9, 12, 15 |
| L4-G-086-BH-004 | The system shall record the authorised disciplinary action or supported closure without inventing unsupported curfew-violation lifecycle states. | L3-G-086 §§9–13, 16–17 |
| L4-G-086-BH-005 | The system shall preserve the violation, rule basis, review, authority and outcome history. | L3-G-086 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-086-BH-006 | G03 Hostel Management remains the canonical publisher of the authoritative curfew-violation/disciplinary fact; exact event identifiers shall not be invented because the Locked parent records them as unresolved. | L3-G-086 §§14, 17 |
| L4-G-086-BH-007 | Where a separate Student Lifecycle consequence may be required, Hostel Management shall hand off the authoritative hostel outcome to C02 without changing Student Lifecycle state itself. | L3-G-086 §§5, 11, 14 |
| L4-G-086-BH-008 | Notifications may consume the outcome without acquiring disciplinary ownership. | L3-G-086 §§5, 14 |

## 6. Failure / Rejection / Continuation

- A valid authorised exception shall prevent classification as an unauthorised curfew violation.
- Insufficient evidence shall prevent unsupported disciplinary action.
- Missing authority shall leave the disciplinary outcome pending.

## 7. Audit & Evidence Behavior

Preserve resident context, curfew rule, exception check, violation evidence, authority decision and disciplinary outcome.

## 8. Observable Completion Conditions

Completion requires the condition to be resolved as a valid exception, unsubstantiated violation, or authorised disciplinary outcome, with history preserved and no direct Student Lifecycle mutation.

## 9. Traceability & Provenance

**Parent:** L3-G-086 — Curfew Violation to Disciplinary Action  
**Primary verification source:** L3-G-086 §§3, 5, 9, 11–17 and authoritative L2-G03 architecture.

## 10. Open Issues / Deferred Design

Exact curfew-violation lifecycle/event identifiers remain unresolved upstream. Technical implementation remains deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
