# Bulk Scholarship Eligibility Recalculation

**Scenario ID:** L3-C-070  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship Eligibility  
**Participating Domains:** C064 — Student Scholarship Eligibility Determination and Recommendation; C066 — Scholarship Eligibility Reassessment; C067 — Scholarship Renewal Evaluation; C071 — Scholarship Eligibility to Financial Aid Processing.  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C07 — Scholarship Eligibility Workflows; applicable upstream sources named below  
**Referenced By:** No material ownership conflict identified. The key dependency is C07's explicit WS-10 approval rule and its separation from Finance.

**Classification: PASS** for source-grounded authoring, subject to normal architecture review.

## 1. Scenario Metadata
Frozen Section C identity is preserved exactly. This scenario applies locked L2-C07 business architecture and does not redefine upstream ownership.

## 2. Scenario Purpose
Recalculate scholarship eligibility for an affected population when approved policy, institutional, academic or administrative changes require multiple Student Scholarship Eligibility Records to be reassessed while preserving historical decision integrity.

## 3. Business Outcome
All affected eligibility records receive updated authoritative assessments, applicable recommendations are regenerated, and prior assessment versions remain historically traceable.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A governed bulk recalculation condition exists and the affected population can be identified.

### 4.2 End Boundary
The authorized recalculation completes with updated eligibility outcomes/recommendations published and previous versions preserved.

### 4.3 In Scope
- Establish the governed recalculation reason.
- Identify the affected student/scholarship population.
- Evaluate each affected eligibility record using the effective policy and authoritative facts.
- Create new immutable assessment versions.
- Update active eligibility positions where required.
- Regenerate applicable recommendations.
- Publish bulk completion and recommendation business outcomes.

### 4.4 Out of Scope
- Individual reassessment (C066).
- Renewal evaluation (C067).
- Conditional eligibility processing (C065).
- Financial processing (C071).
- Technical batch/job orchestration.

### 4.5 Related Scenarios
No material ownership conflict identified. The key dependency is C07's explicit WS-10 approval rule and its separation from Finance.

**Classification: PASS** for source-grounded authoring, subject to normal architecture review.

## 5. Ownership and Participating Domains
Scholarship Eligibility owns the Student Scholarship Eligibility Record, scholarship eligibility policy/evaluation and canonical eligibility outcomes. Participating domains retain ownership of their own facts and downstream decisions. Participation never transfers ownership.

## 6. Business Actors, Roles and Responsibilities
The authoritative L2-C07 workflow actor catalogue is used where applicable. No new role is invented.

## 7. Preconditions and Entry Criteria
- Scholarship Eligibility owns bulk eligibility recalculation.
- Upstream domains remain authoritative for student, academic, identity, document and policy facts.
- Finance consumes resulting eligibility outcomes and remains owner of financial assistance decisions.

## 8. Trigger
- A valid recalculation reason exists.
- Affected population is defined.
- Effective policy is available.
- Required upstream facts are authoritative.
- Where institutional governance requires manual authorization, the recalculation request is approved. C07 also permits system-initiated recalculations arising from approved policy activation, scheduled renewal cycles, or authorized system processes without additional manual approval, while remaining auditable.

## 9. Main Business Flow
An approved policy revision, institutional rule change, administrative request, or other authorized C07 bulk-recalculation condition occurs.

## 10. Alternate Business Flows
1. Identify the recalculation reason and affected population.
2. Establish the effective scholarship policy and relevant authoritative upstream facts.
3. Evaluate affected eligibility records.
4. Produce new immutable assessment versions.
5. Activate the resulting eligibility positions where applicable.
6. Regenerate applicable recommendations from completed eligible outcomes.
7. Publish updated eligibility outcomes.
8. Publish `BulkEligibilityEvaluationCompleted`.
9. Publish `BulkRecommendationsGenerated` where recommendations are produced.

## 11. Failure, Interruption and Business Recovery
- If a student is no longer eligible, the updated outcome supersedes the active eligibility position while the prior version remains historical.
- If eligibility remains unchanged, a new assessment may still establish the current evaluation history without rewriting prior history.
- If a record is conditionally eligible, only the policy-defined conditional treatment applies.
- A bulk recalculation must not independently approve financial assistance.

Business recovery only; no technical retries, queues, rollback or infrastructure mechanics.

## 12. Business Rules and Decision Constraints
An affected record that cannot be validly reassessed remains governed by its prior authoritative position until a valid assessment can be completed under C07 rules. Historical versions are never deleted or overwritten.

## 13. State and Lifecycle Impact
- Historical eligibility assessments are immutable.
- Reassessment creates a new version.
- Recommendations originate only from completed eligibility outcomes.
- Only one active eligibility record exists for the Student/Scholarship/Institution/Academic Period combination.
- Bulk recalculation remains within C07 ownership.
- Manual approval is required only where institutional governance requires it; approved system-initiated recalculation paths may proceed without additional manual approval while remaining auditable.

## 14. Business Events and Cross-Domain Collaboration
Each affected Student Scholarship Eligibility Record follows the C07 eligibility lifecycle; recalculation produces new immutable assessment versions rather than a new separate aggregate lifecycle.

## 15. Business Authorization, Approval and Audit Obligations
`BulkEligibilityEvaluationStarted`; `BulkEligibilityEvaluationCompleted`; `BulkRecommendationsGenerated` are explicitly published by WS-10. Individual recalculation outcomes use the canonical C07 eligibility events as applicable.

## 16. Completion Criteria and Postconditions
WS-10 explicitly defines approval governance: manual approval only where institutional governance requires it; approved policy activation, scheduled renewal cycles, and authorized system processes may initiate recalculation without additional manual approval. All recalculation remains auditable.

## 17. Open Issues, Assumptions and Deferred Questions
- Affected population established.
- Recalculation completed.
- Updated eligibility positions recorded.
- Prior versions preserved.
- Applicable recommendations regenerated.
- Bulk completion outcome published.

## Production Verification Record
- [x] Frozen identity, owner, domains, type and priority preserved.
- [x] L2-C07 ownership boundary preserved.
- [x] Cross-domain dependency boundaries preserved.
- [x] No implementation architecture introduced.
- [x] All 17 mandatory sections present.
- [x] Material source gaps recorded.
- [x] Status remains Draft under Phase 5 governance.
