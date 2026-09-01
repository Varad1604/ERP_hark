# Candidate Examination Registration to Confirmed Registration

## 1. Specification Metadata
**Specification ID:** L4-I-017  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-017 — Candidate Examination Registration to Confirmed Registration  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-I-017 — Candidate Examination Registration to Confirmed Registration; L2-I04 — Examination Registration Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an eligible candidate's examination registration through validation, review, approval, and confirmation so that the Examination Registration becomes the authoritative confirmed participation record.

**Start:** Eligible candidate initiates registration during the approved registration context.  
**End:** Examination Registration reaches Confirmed and is available for downstream examination workflows.

## 3. Required Business Context
- Active Examination Session.
- Published Examination Schedule.
- Open registration window or authorised exception.
- Candidate and academic context.
- Valid examination subjects.
- Financial eligibility where applicable.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-017-BH-001 | The system shall identify the active Examination Session and Published Examination Schedule applicable to the candidate's registration. | L3-I-017 §§3, 7–9 |
| L4-I-017-BH-002 | The system shall validate candidate eligibility and applicable registration restrictions using authoritative business context. | L3-I-017 §§3, 7, 9, 12 |
| L4-I-017-BH-003 | The system shall validate that each registered examination subject is applicable to the candidate and examination context. | L3-I-017 §§4, 7, 12 |
| L4-I-017-BH-004 | Where required, the system shall recognise authoritative financial eligibility without transferring financial ownership from Finance. | L3-I-017 §§3, 5, 10 |
| L4-I-017-BH-005 | The system shall establish the Examination Registration through the governed I04 registration lifecycle. | L3-I-017 §§2, 9, 13 |
| L4-I-017-BH-006 | The system shall require applicable review and approval before a registration may become Confirmed. | L3-I-017 §§3, 9, 12 |
| L4-I-017-BH-007 | On successful completion of required conditions, the system shall transition the Examination Registration to Confirmed. | L3-I-017 §§2, 13, 16 |
| L4-I-017-BH-008 | The system shall preserve registration history and the evidence supporting the Confirmed outcome. | L3-I-017 §§14–16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-017-BH-009 | On transition to Confirmed, the system shall publish the canonical `Registration Confirmed` business event through Examination Registration. | L3-I-017 §14; L2-I04 |
| L4-I-017-BH-010 | The system shall consume authoritative student, academic/curriculum, and financial facts without transferring ownership of those facts to Examination Registration. | L3-I-017 §5 |

No schema, broker, queue, transport, or implementation mechanism is defined.

## 6. Failure / Rejection / Continuation
- If the Examination Session is not Active or the Schedule is not Published, the system shall prevent confirmation.
- If candidate eligibility, subject applicability, required financial eligibility, or approval is not satisfied, the system shall prevent progression to Confirmed.
- If an existing valid conflicting registration is identified, the system shall prevent creation of a duplicate registration and retain the authoritative existing position.

## 7. Audit & Evidence Behavior
The system shall preserve or make attributable candidate context, examination session/schedule, eligibility outcome, subject scope, approval, confirmation, and material registration history.

## 8. Observable Completion Conditions
Completion occurs when the Examination Registration is Confirmed, the confirmation is traceable, the canonical `Registration Confirmed` event is published, and downstream examination workflows may consume the registration.

## 9. Traceability & Provenance
**Parent:** L3-I-017 — Candidate Examination Registration to Confirmed Registration  
**Primary verification source:** L2-I04 — Examination Registration Workflows

## 10. Open Issues / Deferred Design
Institution-specific approval sequencing and registration-window values remain governed by policy/configuration. Technical interfaces, persistence, and implementation mechanics remain outside L4.