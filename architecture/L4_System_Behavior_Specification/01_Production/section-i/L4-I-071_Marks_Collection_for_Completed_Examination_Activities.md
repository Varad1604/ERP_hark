# Marks Collection for Completed Examination Activities

## 1. Specification Metadata

**Specification ID:** L4-I-071  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Marks Collection & Evaluation  
**Realizes Scenario:** L3-I-071  
**Primary Business Owner:** Marks Collection & Evaluation  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Architecture Approval:** Approved — Vikram Desai, Enterprise Architecture Governance Lead  
**Approval Recorded:** 2026-09-03  
**Owner:** Pallav  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-071 — Locked canonical parent; L2-I08 — Locked primary verification source; L4 Governance and Architecture Standard; L4 Validation Standard  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP must collect marks from authorized evaluators only after authoritative examination/assessment completion evidence is available, preserve traceable assessment evidence and attribution, and end at submitted marks without absorbing validation, approval, finalisation or result processing.

## 3. Required Business Context

- Authoritative completed practical/theory examination or assessment evidence.
- Candidate, assessment/examination activity and evaluator identity/assignment context.
- Approved marking scheme and evaluation policy.
- Authorized evaluator authority.
- Marks Collection & Evaluation ownership of collected/submitted marks.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-071-BH-001 | The system shall prevent marks collection from beginning until the applicable examination or assessment completion fact is authoritative. | L3-I-071 §§4, 7, 9, 12 |
| L4-I-071-BH-002 | The system shall associate received assessment evidence with the authoritative candidate, activity and evaluation scope without changing source ownership. | L3-I-071 §§5, 7, 9, 12 |
| L4-I-071-BH-003 | The system shall permit academic evaluation only for an evaluator authorized for the applicable assessment scope. | L3-I-071 §§6–7, 9, 12, 15 |
| L4-I-071-BH-004 | The system shall preserve the approved marking scheme/evaluation policy context used to determine awarded marks. | L3-I-071 §§7, 9, 12 |
| L4-I-071-BH-005 | The system shall associate collected marks with the authoritative candidate, assessment and evaluator context before submission. | L3-I-071 §§3, 9, 12 |
| L4-I-071-BH-006 | The system shall prevent `Marks Submitted` while required marks for the applicable collection scope remain incomplete or unattributable. | L3-I-071 §§9, 11, 12, 16 |
| L4-I-071-BH-007 | The system shall keep submitted marks provisional and shall not perform Marks Validation, Marks Approval, Final Approved Marks finalisation or Result Processing in this scenario. | L3-I-071 §§3–4, 12–13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-071-BH-008 | The system shall recognize the applicable exact conduct-completion event (`Practical Examination Completed`, `Theory Examination Completed`, or `Practical Assessment Completed`) as authoritative readiness for marks collection when that event is applicable. | L3-I-071 §§4, 14; L2-I08 §12 |
| L4-I-071-BH-009 | Where theory-script evidence is applicable, the system shall recognize `Answer Books Collected` as authoritative source evidence from Theory Examination Conduct. | L3-I-071 §§4, 14; L2-I08 §12 |
| L4-I-071-BH-010 | When required assessment evidence is successfully established in the evaluation context, the system shall publish `Assessment Evidence Received`. | L3-I-071 §§9, 14; L2-I08 §11 |
| L4-I-071-BH-011 | When the authorized evaluator completes academic evaluation, the system shall publish `Evaluation Completed`. | L3-I-071 §§9, 14; L2-I08 §11 |
| L4-I-071-BH-012 | When collected marks are submitted for downstream validation, the system shall publish `Marks Submitted`; this event is the scenario completion event. | L3-I-071 §§4, 9, 14, 16; L2-I08 §11 |

## 6. Failure / Rejection / Continuation

- If completion evidence cannot be verified, the system shall prevent marks collection.
- If required assessment evidence is missing or disputed, the system shall prevent affected evaluation until the authoritative evidence condition is resolved.
- If evaluator authority is invalid or the evaluator becomes unavailable, the system shall preserve the last valid collection position and permit only a source-supported replacement/continuation path.
- If required marks remain incomplete, the system shall prevent submission.

## 7. Audit & Evidence Behavior

The system shall preserve the completion source fact, candidate/assessment identity, assessment evidence, evaluator assignment/authority, applicable marking scheme/policy, awarded marks, collection completeness, submission actor/time and the `Marks Submitted` completion fact.

## 8. Observable Completion Conditions

The behavior is complete when authoritative completion evidence exists, evaluation is attributable to an authorized evaluator, required marks are collected and complete for the applicable scope, `Marks Submitted` is published, and no validation/approval/finalisation/result-processing outcome has been absorbed.

## 9. Traceability & Provenance

**Parent:** L3-I-071 — Marks Collection for Completed Examination Activities  
**Primary verification source:** L2-I08 — Marks Collection & Evaluation Workflows  

Every BH derives from the corrected locked L3-I-071 parent. L2-I08 verifies exact events, evaluation rules and the submitted-marks boundary.

## 10. Open Issues / Deferred Design

No material L4 issue remains after event/completeness correction. Architecture Approval was granted by Vikram Desai, Enterprise Architecture Governance Lead, and recorded on 2026-09-03. Repository lock remains a separate governance action.

API/interface design, persistence, event transport, technical access control, infrastructure and UI/UX remain deferred.
