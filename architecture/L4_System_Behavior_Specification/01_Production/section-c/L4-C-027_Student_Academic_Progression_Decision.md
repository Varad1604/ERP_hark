# L4-C-027 — Student Academic Progression Decision

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-027` — Student Academic Progression Decision  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad

## 1. Purpose
Define behavior for establishing one authoritative, policy-governed academic progression decision from official academic evidence.

## 2. Preconditions and Trigger
Student lifecycle and registration context are valid; official Examination outcomes are published; applicable curriculum context and one approved progression policy are available. Primary trigger is `ExaminationResultPublished`; governed re-evaluation/supplementary paths use only upstream-supported events.

## 3. Main Behavior
1. Establish evaluation context.
2. Consume officially published examination evidence.
3. Identify applicable curriculum requirements and approved progression policy.
4. Evaluate earned credits and progression eligibility.
5. Determine Academic Standing and progression outcome.
6. Obtain required review/authorization where upstream policy requires it.
7. Preserve the decision and evidence traceability.
8. Publish the exact canonical progression event for the actual outcome.

## 4. Outcomes
Supported outcomes are `Progressed`, `Conditional Progression`, `Academic Probation`, `Detained`, and `Qualification Eligible` where requirements are satisfied.

## 5. Rules
- Only official examination outcomes may drive authoritative progression.
- Every decision references an approved progression policy.
- Historical decisions remain preserved.
- Manual overrides require authorised institutional governance.
- Downstream domains do not independently promote, place on probation or detain a Student.

## 6. Lifecycle
Academic Progression evaluation proceeds through the states defined by L2-C04, including `Awaiting Evaluation`, `Evaluation Running`, `Conditional Progression`, `Academic Probation`, `Detained`, `Progressed`, and `Qualification Eligible` where applicable. No new state is introduced.

## 7. Canonical Events
Examination publishes `ExaminationResultPublished`, `RevaluationResultPublished` or `SupplementaryResultPublished` where those exact upstream events apply. Academic Progression publishes established outcome events including `ProgressionEvaluated`, `StudentProgressed`, `ConditionalPromotionGranted`, `AcademicProbationAssigned`, `AcademicDetentionApplied`, and `QualificationEligibilityUpdated` as applicable. `ADR-STU-PROG-EVT-001` governs amended canonical vocabulary.

## 8. Authorization and Audit
Roles and authority remain those defined by L2-C04, including Programme Coordinator, Department Administrator, Dean, Registrar and Academic Council where applicable. Decision history, policy basis, evidence and exception/override authority are auditable.

## 9. Business Failure Handling
Unavailable official results, incomplete mandatory academic context, absent approved policy, unsatisfied approval or invalid Student/registration context prevents authoritative completion. Re-evaluation creates a fresh governed decision while preserving history.

## 10. Traceability
L3-C-027; L2-C04 Student Academic Progression Workflows and its locked 2026-08-22 amendment; `ADR-STU-PROG-EVT-001`.

## 11. Open Issues
Policy Management ownership/publisher remains unresolved upstream. Re-evaluation vocabulary has an upstream inconsistency; L4 does not silently reconcile it.

## 12. Implementation Boundary
No API, database, class, service, UI, queue, infrastructure or technical routing is defined.

**Classification: PASS WITH ISSUES — Draft**
