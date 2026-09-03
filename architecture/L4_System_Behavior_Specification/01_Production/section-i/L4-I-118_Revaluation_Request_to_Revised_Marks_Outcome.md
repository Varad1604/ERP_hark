# Revaluation Request to Revised Marks Outcome

## 1. Specification Metadata

**Specification ID:** L4-I-118  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Revaluation & Rechecking  
**Realizes Scenario:** L3-I-118  
**Primary Business Owner:** Revaluation & Rechecking  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Architecture Approval:** Approved — Vikram Desai, Enterprise Architecture Governance Lead  
**Approval Recorded:** 2026-09-03  
**Owner:** Pallav  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-118 — Locked canonical parent; L2-I11 — Locked primary verification source; L4 Governance and Architecture Standard; L4 Validation Standard  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP must govern a Revaluation Request against an officially published result through eligibility, request approval, authorized academic reassessment, outcome verification and revised-marks approval where marks change, while preserving Finance, identity and assessment-evidence ownership and stopping before downstream result recomputation or publication.

## 3. Required Business Context

- Officially published result.
- Revaluation eligibility and approved application-window context.
- Candidate/evaluator/authority identity information.
- Finance-owned payment confirmation where a review fee is required.
- Authorized assessment evidence.
- Source-supported request approval, evaluator and revised-marks approval authority.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-118-BH-001 | The system shall prevent revaluation from proceeding against an unpublished, provisional or draft examination result. | L3-I-118 §§4, 7, 12 |
| L4-I-118-BH-002 | The system shall evaluate the Revaluation Request against the applicable institutional eligibility rules before accepting it for review. | L3-I-118 §§7, 9, 12 |
| L4-I-118-BH-003 | The system shall enforce the applicable review window and permit an out-of-window request only through a source-supported authorized exception path. | L3-I-118 §§7, 10, 12 |
| L4-I-118-BH-004 | Where a review fee is required, the system shall use authoritative Finance payment confirmation without creating or altering that Finance-owned fact. | L3-I-118 §§5, 7, 9, 14 |
| L4-I-118-BH-005 | The system shall prevent academic reassessment from beginning until the Revaluation Request is approved under the source-supported institutional review policy. | L3-I-118 §§9, 12, 15 |
| L4-I-118-BH-006 | The system shall make authorized assessment evidence available for revaluation without modifying the source assessment record. | L3-I-118 §§4, 9, 14 |
| L4-I-118-BH-007 | The system shall permit revaluation only to a source-supported authorized evaluator. | L3-I-118 §§6, 9, 12, 15 |
| L4-I-118-BH-008 | The system shall require institutional verification of the completed review outcome before revised marks may proceed to approval. | L3-I-118 §§9, 12 |
| L4-I-118-BH-009 | The system shall prevent revised marks from becoming authoritative until source-supported institutional approval is completed. | L3-I-118 §§3, 9, 12, 15–16 |
| L4-I-118-BH-010 | The system shall preserve a verified no-change outcome without creating revised marks when reassessment confirms the existing marks. | L3-I-118 §§10, 13, 16 |
| L4-I-118-BH-011 | The system shall not compute grades, grade points, credit points, SGPA, CGPA, standing or classification in this scenario. | L3-I-118 §§4, 12, 16 |
| L4-I-118-BH-012 | The system shall not publish or republish the official examination result in this scenario. | L3-I-118 §§4, 12, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-118-BH-013 | The system shall recognize `Published Result Available` as the authoritative published-result context when that event establishes review eligibility. | L3-I-118 §14; L2-I11 §12 |
| L4-I-118-BH-014 | When a governed Revaluation Request is registered, the system shall publish `Revaluation Requested`. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-015 | When eligibility verification completes successfully, the system shall publish `Review Eligibility Verified`. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-016 | Where a fee applies and authoritative Finance payment confirmation is verified, the system shall publish `Review Fee Verified`. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-017 | When the request is institutionally approved to proceed, the system shall publish `Review Request Approved`. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-018 | When authorized academic reassessment begins, the system shall publish `Revaluation Started`. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-019 | When institutional verification of the review outcome completes, the system shall publish `Review Outcome Verified`. | L3-I-118 §14; L2-I11 §11 |
| L4-I-118-BH-020 | When revised marks become authorized, the system shall publish `Revised Marks Approved`. | L3-I-118 §§14, 16; L2-I11 §11 |
| L4-I-118-BH-021 | When approved revised marks require downstream result revision, the system shall publish `Result Revision Initiated` without performing Result Processing or Result Publication. | L3-I-118 §§14, 16; L2-I11 §11 |

## 6. Failure / Rejection / Continuation

- If mandatory eligibility fails, the system shall prevent revaluation unless a source-supported authorized exception applies.
- If a required fee cannot be authoritatively verified, the system shall retain the request in the source-supported pending/rejected position rather than infer payment.
- If assessment evidence or an eligible evaluator is unavailable, the system shall preserve the last valid review position until the business condition is resolved.
- Unapproved revised marks shall not become authoritative or alter downstream academic outcomes.

## 7. Audit & Evidence Behavior

The system shall preserve the published-result reference, request identity, eligibility and window outcomes, applicable fee verification, request approval, assessment evidence, evaluator/review evidence, outcome verification, prior/revised marks where applicable, approval authority, final review outcome and downstream result-revision initiation.

## 8. Observable Completion Conditions

The behavior is complete when the request reaches a source-supported completed/terminal outcome, review outcome verification is complete, changed marks are authoritative only if approved, required review events/evidence are preserved, and any required downstream revision has only been initiated rather than performed.

## 9. Traceability & Provenance

**Parent:** L3-I-118 — Revaluation Request to Revised Marks Outcome  
**Primary verification source:** L2-I11 — Revaluation & Rechecking Workflows  

Every BH derives from corrected locked L3-I-118. L2-I11 verifies exact actors, rules, events, publishers and downstream ownership boundaries.

## 10. Open Issues / Deferred Design

No material L4 issue remains after completeness correction. Architecture Approval was granted by Vikram Desai, Enterprise Architecture Governance Lead, and recorded on 2026-09-03. Locking remains a separate governance action.

API/interface design, persistence, event transport, technical access control, notification implementation, infrastructure and UI/UX remain deferred.
