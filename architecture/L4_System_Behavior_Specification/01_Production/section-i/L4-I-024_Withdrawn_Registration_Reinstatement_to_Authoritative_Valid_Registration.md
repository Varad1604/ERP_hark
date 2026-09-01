# Withdrawn Registration Reinstatement to Authoritative Valid Registration

## 1. Specification Metadata
**Specification ID:** L4-I-024  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-024 — Withdrawn Registration Reinstatement to Authoritative Valid Registration  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-I-024 — Withdrawn Registration Reinstatement to Authoritative Valid Registration; L2-I04 — Examination Registration Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern reinstatement of a Withdrawn Examination Registration only when the applicable policy, eligibility, authority, and registration context permit restoration to a valid registration position.

**Start:** A Withdrawn Examination Registration is submitted for permitted reinstatement.  
**End:** The registration is restored to an authoritative valid position or reinstatement is rejected.

## 3. Required Business Context
- Withdrawn Examination Registration.
- Candidate and examination context.
- Current registration window or authorised exception.
- Applicable reinstatement policy.
- Eligibility and subject applicability.
- Required authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-024-BH-001 | The system shall identify the Withdrawn Examination Registration proposed for reinstatement. | L3-I-024 §§2, 4, 7–9 |
| L4-I-024-BH-002 | The system shall validate the reinstatement request against applicable policy, registration window, eligibility, and authority requirements. | L3-I-024 §§7, 12, 13 |
| L4-I-024-BH-003 | The system shall preserve the original withdrawal history while evaluating reinstatement. | L3-I-024 §§3, 12, 15 |
| L4-I-024-BH-004 | The system shall prevent reinstatement where required eligibility, policy, or authority conditions are not satisfied. | L3-I-024 §§10–12 |
| L4-I-024-BH-005 | On an authorised reinstatement, the system shall restore the Examination Registration to the applicable valid position in the canonical I04 lifecycle. | L3-I-024 §§3, 9, 13, 16 |
| L4-I-024-BH-006 | The system shall preserve the reinstatement decision and resulting registration history. | L3-I-024 §§3, 15, 16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-024-BH-007 | The system shall use the canonical I04 registration event vocabulary for any resulting lifecycle transition and shall not invent a dedicated reinstatement event. | L3-I-024 §14; L2-I04 |
| L4-I-024-BH-008 | The system shall consume authoritative Student and other participating-domain context without transferring ownership. | L3-I-024 §5 |

## 6. Failure / Rejection / Continuation
- If reinstatement is not permitted, the Withdrawn registration shall remain authoritative.
- If eligibility or policy validation fails, the system shall prevent restoration to a valid active registration position.
- If required approval is denied or unavailable, the system shall retain the Withdrawn position.
- The system shall not treat reinstatement as erasing the original withdrawal event/history.

## 7. Audit & Evidence Behavior
Preserve original withdrawal, reinstatement request, eligibility/validation basis, authority, resulting registration position, and related event history.

## 8. Observable Completion Conditions
Completion occurs when the registration is authoritatively restored to the applicable valid position with withdrawal/reinstatement history preserved, or the reinstatement request reaches a governed rejected outcome.

## 9. Traceability & Provenance
**Parent:** L3-I-024 — Withdrawn Registration Reinstatement to Authoritative Valid Registration  
**Primary verification source:** L2-I04 — Examination Registration Workflows

## 10. Open Issues / Deferred Design
Exact reinstatement authority, window, fees/refund implications, and technical implementation remain policy or downstream concerns.