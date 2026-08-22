# Conflicting Subsequent Examination Results to Single Valid Result Position

**Scenario ID:** L3-I-139  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I09 Result Processing & Computation; L2-I10 Result Publication; L2-I12 Supplementary / Backlog / Improvement Examination Execution  
**Referenced By:** Downstream examination and academic workflows consuming the authoritative outcome

## 1. Scenario Metadata

The frozen Stage 3 row fixes the canonical title, owner, participating domains, Type and Priority shown above. No reclassification is introduced.

## 2. Scenario Purpose

Resolve conflicting Supplementary, Backlog or Improvement result positions so that one authoritative current result remains available to downstream academic workflows. The scenario applies the existing result-processing and publication ownership rather than creating local result-computation rules.

## 3. Business Outcome

A single authoritative current result position is established for the affected candidate and subsequent-examination scope. Conflicting or superseded result positions remain historically traceable and do not compete as current academic facts.

## 4. Scenario Boundary and Scope

**Start Boundary:** More than one subsequent-examination result position is identified as conflicting for the same candidate and examination scope.

**In Scope:** identify candidate, attempt, subject and result scope; distinguish legitimate historical attempt outcomes from a genuine current conflict; route genuine conflicts to the canonical result-processing workflow; preserve history; and make the authoritative result available downstream.

**Out of Scope:** changing Student academic status directly, recomputing marks locally, independently publishing results, changing completion status, or technical reconciliation.

**End Boundary:** One current result position is authoritative for the affected scope and downstream consumers have a clear result basis.

## 5. Ownership and Participating Domains

**Supplementary, Backlog & Improvement Examination:** owns the subsequent-examination attempt context and coordinates the case.

**Result Processing & Computation (L2-I09):** owns result computation, consolidation, validation and the authoritative computed result.

**Result Publication (L2-I10):** owns publication of the authoritative result.

**Student Lifecycle (C):** consumes the authoritative examination result and remains owner of the student's academic record.

Participation does not transfer result or student ownership.

## 6. Business Actors, Roles and Responsibilities

- Supplementary Examination Officer: identifies and coordinates the conflicting result case.
- Examination Administrator: coordinates the business resolution.
- Controller of Examination: provides examination governance where required.
- Result Processing authority: determines the authoritative result through L2-I09/L2-I10.
- Student Lifecycle: consumes the resulting authoritative academic fact.

No new approval authority is introduced.

## 7. Preconditions and Entry Criteria

- A subsequent-examination attempt exists.
- Result information exists for the affected scope.
- More than one result position or conflicting result evidence has been identified.
- Candidate and examination context can be established.
- The canonical result-processing workflow can determine the authoritative result.

## 8. Trigger

A conflict is identified between result positions for the same candidate and subsequent-examination scope.

## 9. Main Business Flow

1. Identify candidate, examination category, subject scope and attempt history.
2. Identify every result position contributing to the conflict.
3. Establish which facts are authoritative inputs and which are derived or superseded.
4. Determine whether multiple results are legitimate historical attempts or a genuine current conflict.
5. Route a genuine conflict to L2-I09 for authoritative result determination.
6. Preserve the affected history and resolution basis.
7. Complete required review/approval under the canonical result workflow.
8. Establish one authoritative current result position.
9. Publish the result-processing/publication events through their canonical owners.
10. Make the authoritative result available to Student Lifecycle and downstream academic consumers.
11. Close the conflicting-result case after the authoritative result position is established.

## 10. Alternate Business Flows

**Legitimate attempt history:** distinct historical attempt outcomes remain preserved while current-result selection follows existing policy.

**Computation correction:** L2-I09 performs the authoritative recomputation; this scenario does not calculate marks.

**Publication correction:** L2-I10 governs correction of an incorrectly published result.

**Completion impact:** if the result changes completion eligibility, the Academic Completion workflow consumes the updated authoritative result.

## 11. Failure, Interruption and Business Recovery

**Authoritative result cannot be determined:** no local compromise is invented; the case remains unresolved pending the owning result workflow.

**Candidate context conflict:** Student Lifecycle remains authoritative for its own facts.

**Multiple legitimate attempts:** histories remain distinct; only the policy-defined current result is treated as current.

**Correction required:** any correction follows L2-I09/L2-I10; no direct overwrite is performed.

## 12. Business Rules and Decision Constraints

- Result computation/consolidation is owned by L2-I09.
- Result publication is owned by L2-I10.
- Subsequent-examination attempts remain uniquely identifiable and traceable.
- Historical result outcomes must not be destroyed or confused with the current result.
- A current result consumer must have one authoritative result position.
- Student Lifecycle consumes the authoritative result but does not determine result computation.
- No local mark/result selection rule is invented.

## 13. State and Lifecycle Impact

No new result state is introduced. The subsequent-examination attempt remains within the L2-I12 lifecycle while result computation and publication continue through L2-I09 and L2-I10. The conflict is resolved through existing result-processing outcomes rather than a new `Conflicted` or `Resolved` state.

## 14. Business Events and Cross-Domain Collaboration

L2-I12 provides the subsequent-examination attempt context and its canonical attempt lifecycle events. L2-I09/L2-I10 remain the canonical publishers for result computation and publication milestones.

No `SubsequentResultConflictResolved` event is invented. Student Lifecycle consumes the authoritative result after the canonical result workflow establishes it.

## 15. Business Authorization, Approval and Audit Obligations

- Conflicting-result cases follow existing result-processing governance.
- Exceptional result decisions use the authority defined by L2-I09/L2-I10.
- Result history, correction basis and final authoritative position are auditable.
- Original and superseded evidence remain traceable.
- Student academic record updates remain owned by Student Lifecycle.

## 16. Completion Criteria and Postconditions

**Complete when:** the conflict is classified; authoritative result inputs are established; L2-I09/L2-I10 determine and publish the single current result; history is preserved; and downstream consumers have one valid result position.

**Postconditions:** one authoritative current result exists for the scope; Student Lifecycle can consume it; completion workflows may use it; no competing result is treated as current.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I09/L2-I10 remain the canonical owners of result computation/publication; this scenario does not redefine their rules.
2. The accessible L2-I12 material does not define a dedicated result-conflict event; none is invented.
3. The exact institutional rule for choosing among legitimate historical attempt outcomes remains policy-owned by the result-processing architecture.
