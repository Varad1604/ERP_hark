# Revaluation Request to Revised Marks Outcome

## 1. Specification Metadata

**Provisional Specification ID:** L4-I-118  
**Architecture Level:** L4 — System Behavior Specification  
**Realizes Scenario:** L3-I-118  
**Primary Business Owner:** Revaluation & Rechecking  
**Status:** Draft  
**Pilot Classification:** Provisional — not canonical naming or granularity  
**Primary Sources:** L3-I-118 v1.1 Architecture Approved; locked L2-I11  

## 2. Behavioral Objective & Boundary

SutraERP must govern a Revaluation Request from an officially published result through eligibility, review, outcome verification and revised-marks approval where marks change, while preserving Finance, Person & Identity, Result Processing & Computation and Result Publication ownership boundaries.

The behavior begins when a Candidate submits a Revaluation Request against an officially published result and ends when the review reaches a governed completed outcome, including approved revised marks where applicable, without performing downstream result recomputation or republication.

## 3. Required Business Context

- Officially published result from Result Publication.
- Revaluation eligibility policy and approved application-window context.
- Candidate/evaluator/authority identity information from Person & Identity.
- Finance-owned payment confirmation where review fee policy requires payment.
- Authorized assessment evidence from Marks Collection & Evaluation.
- Source-supported institutional review and revised-marks approval authority.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-118-BH-001 | The system shall prevent revaluation from proceeding against an unpublished, provisional or draft examination result. | L3-I-118 §§4, 7, 12; L2-I11 §9.1 |
| L4-I-118-BH-002 | The system shall evaluate the Revaluation Request against the applicable institutional eligibility rules before accepting it for review. | L3-I-118 §§7, 9, 12; L2-I11 §9.2 |
| L4-I-118-BH-003 | The system shall enforce the applicable institutional review window and permit an out-of-window request only where an authorized exception path exists. | L3-I-118 §§7, 10, 12; L2-I11 §9.3 |
| L4-I-118-BH-004 | Where a review fee is required, the system shall use Finance-owned payment confirmation and shall not create or alter Finance payment truth. | L3-I-118 §§5, 7, 9, 14 |
| L4-I-118-BH-005 | The system shall prevent academic reassessment from beginning until the Revaluation Request is approved under the applicable institutional review policy. | L3-I-118 §§9, 12, 15; L2-I11 §§5.6, 8.6 |
| L4-I-118-BH-006 | The system shall make authorized assessment evidence available for revaluation without modifying the source assessment record. | L3-I-118 §§4, 9, 14; L2-I11 §§5.7, 8.7 |
| L4-I-118-BH-007 | The system shall permit revaluation only to a source-supported authorized evaluator under institutional regulations. | L3-I-118 §§6, 9, 12, 15; L2-I11 §9.5 |
| L4-I-118-BH-008 | The system shall require institutional verification of the completed review outcome before revised marks can proceed to approval. | L3-I-118 §§9, 12; L2-I11 §§5.10, 8.9 |
| L4-I-118-BH-009 | The system shall prevent revised marks from becoming authoritative until the required institutional approval has been completed. | L3-I-118 §§3, 9, 12, 16; L2-I11 §9.6 |
| L4-I-118-BH-010 | The system shall preserve a verified no-change outcome without inventing revised marks where the reassessment confirms the existing marks. | L3-I-118 §§4, 10, 13 |
| L4-I-118-BH-011 | The system shall not compute grades, grade points, credit points, SGPA, CGPA, academic standing or classification as part of this scenario. | L3-I-118 §§4, 12, 16; L2-I11 §9.7 |
| L4-I-118-BH-012 | The system shall not publish or republish the official examination result as part of this scenario. | L3-I-118 §§4, 12, 16; L2-I11 §9.8 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-118-BH-013 | The system shall recognize `Published Result Available` as the authoritative upstream event establishing an eligible published-result context where event-driven collaboration is used. | L3-I-118 §14; L2-I11 §12 |
| L4-I-118-BH-014 | The system shall publish `Revaluation Requested` when the governed request is registered. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-015 | The system shall publish `Review Eligibility Verified` when eligibility verification completes successfully. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-016 | Where a fee applies, the system shall publish `Review Fee Verified` after Finance-owned payment confirmation is successfully verified for the review. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-017 | The system shall publish `Review Request Approved` when the request is institutionally approved to proceed. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-018 | The system shall publish `Revaluation Started` when authorized academic reassessment begins. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-019 | The system shall publish `Review Outcome Verified` when institutional verification of the review outcome completes. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-020 | The system shall publish `Revised Marks Approved` when revised marks become the authorized review outcome. | L3-I-118 §§14, 16; L2-I11 §11 |
| L4-I-118-BH-021 | Where approved revised marks require downstream result revision, the system shall publish `Result Revision Initiated` without performing the Result Processing or Result Publication outcome itself. | L3-I-118 §§14, 16; L2-I11 §§8.11, 11 |

## 6. Failure / Rejection / Continuation

- If mandatory eligibility is not satisfied, the system shall prevent progression unless an authorized exception path applies.
- If a required fee cannot be authoritatively verified, the system shall retain the request in the source-supported pending/rejected business position rather than infer payment.
- If authorized assessment evidence or an eligible evaluator is unavailable, the system shall preserve the last valid review position until the institutional exception condition is resolved.
- Unapproved revised marks shall not become authoritative and shall not alter downstream academic outcomes.

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to establish the published-result reference, request identity, eligibility result, application-window result, applicable fee verification, request approval, assessment evidence, evaluator/review evidence, outcome verification, prior marks, proposed or revised marks where applicable, approval authority, final review outcome and downstream revision initiation.

## 8. Observable Completion Conditions

The behavior is complete when the Revaluation Request reaches a governed completed outcome, the review outcome has been verified, changed marks are authoritative only if approved, source-supported events/evidence are preserved, and any required downstream result revision has only been initiated—not performed—by this scenario.

## 9. Traceability & Provenance

**Parent:** L3-I-118 — Revaluation Request to Revised Marks Outcome  
**Primary verification source:** L2-I11 — Revaluation & Rechecking Workflows  

Every BH derives from the approved L3 parent. L2-I11 is used only to verify exact actors, rules, lifecycle behavior, event names, publishers and downstream ownership boundaries.

## 10. Open Issues / Deferred Design

No pilot-blocking upstream issue remains after the 2026-08-26 L3 remediation and re-review.

Persistence, API/integration design, event transport, access-control implementation, notification channels, computation design, UI and infrastructure remain deferred to later architecture levels.
