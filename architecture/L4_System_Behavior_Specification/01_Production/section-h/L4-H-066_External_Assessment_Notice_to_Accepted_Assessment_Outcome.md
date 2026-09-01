# External Assessment Notice to Accepted Assessment Outcome

## 1. Specification Metadata
**Specification ID:** L4-H-066  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accreditation & Compliance  
**Realizes Scenario:** L3-H-066  
**Primary Business Owner:** Accreditation & Compliance  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-066; L2-H09 — Accreditation & Compliance Workflows; `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`; applicable Locked external/institutional architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern H09's institutional response to an authoritative external assessment outcome through acceptance or governed challenge/resolution while preserving external evidence and source-domain evidence ownership.

## 3. Required Business Context
An external assessment schedule/context and applicable Accreditation Cycle/evidence context exist; source evidence can be supplied by reference; the Accreditation and Compliance Governance Authority owns institutional acceptance/challenge decisions; H09 owns only the institutional response lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-066-BH-001 | The system shall admit the external assessment only when the assessment schedule/context and applicable accreditation/evidence context are valid. | L3-H-066 §§7–9 |
| L4-H-066-BH-002 | The system shall preserve the authoritative external assessment outcome as external evidence and shall establish the H09 institutional response in `Received`. | L3-H-066 §§3–5,9,12–13 |
| L4-H-066-BH-003 | The system shall transition the H09 institutional response `Received → Under Review` for governed institutional consideration. | L3-H-066 §§9,12–13 |
| L4-H-066-BH-004 | When the Accreditation and Compliance Governance Authority accepts the external outcome, the system shall transition `Under Review → Accepted` and publish `ExternalAssessmentOutcomeAccepted`. | L3-H-066 §§6,9,12–15 |
| L4-H-066-BH-005 | When the authoritative institutional decision is to challenge, the system shall transition `Under Review → Challenged` and publish `ExternalAssessmentOutcomeChallenged`. | L3-H-066 §§6,10,12–15 |
| L4-H-066-BH-006 | When the challenge is externally/governedly resolved, the system shall transition `Challenged → Resolved`, publish `ExternalAssessmentChallengeResolved`, and then progress to `Accepted` or `Replaced` according to the authoritative later outcome. | L3-H-066 §§10,12–14 |
| L4-H-066-BH-007 | When a later authoritative external outcome replaces the prior one, the system shall place the H09 response in `Replaced`, publish `ExternalAssessmentOutcomeReplaced`, and preserve the prior external outcome as historical evidence. | L3-H-066 §§10,12–15 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-066-BH-008 | H09 shall publish `ExternalAssessmentOutcomeAccepted`, `ExternalAssessmentOutcomeChallenged`, `ExternalAssessmentChallengeResolved`, and `ExternalAssessmentOutcomeReplaced` only for the H09 institutional-response facts represented by those events. | L3-H-066 §14 |
| L4-H-066-BH-009 | Source evidence shall remain owned by `VAR:Evidence-Owning Domain`; H09 shall not alter external assessment evidence or source-domain facts to resolve a discrepancy. | L3-H-066 §§5,11–14 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-066-BH-010 | Missing required evidence or an unresolved challenge shall prevent final `Accepted` completion and preserve the current governed response/history. | L3-H-066 §11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-066-BH-011 | The system shall preserve assessment schedule, source references, observations/findings, authoritative external outcome, institutional decision authority, challenge/resolution evidence, replacement lineage and immutable response history. | L3-H-066 §15 |

## 8. Observable Completion Conditions
The behavior is complete when the H09 institutional response reaches authoritative `Accepted` status or completes the governed challenge/resolution disposition, with source evidence ownership and external outcome history preserved.

## 9. Traceability & Provenance
**Parent:** L3-H-066 — External Assessment Notice to Accepted Assessment Outcome  
**Primary verification source:** L2-H09 — Accreditation & Compliance Workflows  
**Lifecycle/authority verification:** `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. External protocol, document and UI design remain deferred.
