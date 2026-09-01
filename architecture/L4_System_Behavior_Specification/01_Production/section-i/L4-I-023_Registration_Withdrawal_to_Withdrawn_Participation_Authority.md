# Registration Withdrawal to Withdrawn Participation Authority

## 1. Specification Metadata
**Specification ID:** L4-I-023  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-023 — Registration Withdrawal to Withdrawn Participation Authority  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-I-023 — Registration Withdrawal to Withdrawn Participation Authority; L2-I04 — Examination Registration Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern withdrawal of an Examination Registration so that the registration reaches the authoritative Withdrawn position when permitted by policy.

**Start:** An existing Examination Registration is subject to a valid withdrawal request.  
**End:** The registration is Withdrawn with the applicable history preserved, or withdrawal is rejected.

## 3. Required Business Context
- Existing Examination Registration.
- Candidate context.
- Applicable withdrawal policy and timing rules.
- Required authority where withdrawal is not self-service.
- Registration lifecycle position.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-023-BH-001 | The system shall identify the Examination Registration subject to withdrawal. | L3-I-023 §§2, 4, 7–9 |
| L4-I-023-BH-002 | The system shall validate the withdrawal request against the applicable registration state, policy, and authority requirements. | L3-I-023 §§7, 12, 13 |
| L4-I-023-BH-003 | The system shall prevent withdrawal where applicable policy or lifecycle conditions do not permit it. | L3-I-023 §§10–12 |
| L4-I-023-BH-004 | On an authorised withdrawal, the system shall transition the Examination Registration to Withdrawn using the canonical I04 lifecycle. | L3-I-023 §§3, 13, 16 |
| L4-I-023-BH-005 | The system shall preserve the withdrawal basis and prior registration history. | L3-I-023 §§3, 12, 15 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-023-BH-006 | On authorised withdrawal, the system shall publish the canonical `Registration Withdrawn` business event through Examination Registration. | L3-I-023 §14; L2-I04 |
| L4-I-023-BH-007 | The system shall consume candidate context from Student without transferring Student ownership. | L3-I-023 §5 |

No separate withdrawal event is introduced.

## 6. Failure / Rejection / Continuation
- If withdrawal is not permitted in the current registration position, the system shall leave the authoritative registration unchanged.
- If required authority is absent or the request is rejected, the registration shall not become Withdrawn.
- If the request identifies an invalid registration, progression shall be prevented until the context is resolved.

## 7. Audit & Evidence Behavior
Preserve withdrawal request, policy basis, authority, decision, prior registration state, resulting Withdrawn state, and event history.

## 8. Observable Completion Conditions
Completion occurs when the registration is Withdrawn through an authorised path and `Registration Withdrawn` is published with traceable history, or a governed rejection leaves the registration unchanged.

## 9. Traceability & Provenance
**Parent:** L3-I-023 — Registration Withdrawal to Withdrawn Participation Authority  
**Primary verification source:** L2-I04 — Examination Registration Workflows

## 10. Open Issues / Deferred Design
Institution-specific withdrawal windows, authority, fees/refunds, technical interfaces, and persistence remain outside L4.