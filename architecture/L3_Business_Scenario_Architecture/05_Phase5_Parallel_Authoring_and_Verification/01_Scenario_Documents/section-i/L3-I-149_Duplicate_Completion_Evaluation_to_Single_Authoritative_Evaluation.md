# Duplicate Completion Evaluation to Single Authoritative Evaluation

**Scenario ID:** L3-I-149
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-C05 Qualification Completion & Student Exit; L2-I09 Result Processing & Computation; L2-I10 Result Publication; Examination completion-evidence ownership

## 1. Scenario Metadata

The canonical Stage 3 semantics and Stage 4 final identity are preserved. The scenario remains owned by Academic Completion & Completion Evidence. Cross-domain participation does not transfer ownership of examination or student lifecycle facts.

## 2. Scenario Purpose

Re-evaluate qualification completion eligibility when an authoritative examination result is revised, ensuring that the completion decision uses the current examination evidence without transferring result ownership to the completion domain.

## 3. Business Outcome

The revised examination result is consumed as authoritative input and completion eligibility is re-evaluated. The resulting completion position is authoritative, while the prior completion decision and superseded examination evidence remain historically traceable.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authoritative revised examination result affects a student for whom completion eligibility has been evaluated or finalized.

**In Scope:** consume the revised result/evidence; determine its impact on completion requirements; re-evaluate completion eligibility; update the completion position through L2-C05; preserve prior history.

**Out of Scope:** recalculating the revised examination result, changing examination states, publishing examination results, or independently revoking/conferment-changing a qualification outside completion authority.

## 5. Ownership and Participating Domains

Examination remains authoritative for result computation and publication.

L2-I13/examination completion-evidence ownership remains responsible for dispositioning examination completion evidence before the revised evidence is consumed by completion.

L2-C05 owns the completion decision and determines whether qualification status/evidence requires change.

Student Lifecycle remains authoritative for student context.

## 6. Business Actors, Roles and Responsibilities

- Examination/result authority: establishes the revised examination outcome.
- Examination completion-evidence authority: establishes the current examination completion-evidence position.
- Completion authority: re-evaluates qualification completion.
- Student Lifecycle: supplies authoritative student context and consumes resulting lifecycle facts.

## 7. Preconditions and Entry Criteria

- A prior examination result/evidence position exists.
- The result revision is authoritative under Examination governance.
- The affected student and qualification-completion scope are identifiable.
- The completion lifecycle can consume a superseding examination evidence position.

## 8. Trigger

An authoritative examination result revision is published/established and affects a student's completion basis.

## 9. Main Business Flow

1. Identify the affected student and completion scope.
2. Consume the current authoritative examination result/evidence position.
3. Preserve the prior examination and completion history.
4. Determine which completion requirements are affected by the revised result.
5. Re-evaluate qualification completion eligibility using the current examination evidence and all other authoritative completion inputs.
6. If eligibility remains unchanged, retain the current completion decision while recording the re-evaluation.
7. If eligibility changes, establish the new authoritative completion position through L2-C05.
8. Where the prior qualification outcome requires correction, revocation or another governed consequence, apply the existing completion-governance workflow.
9. Publish the applicable completion-domain outcome.
10. End when the current completion eligibility/qualification position is authoritative.

## 10. Alternate Business Flows

**Eligibility unchanged:** preserve the existing qualification position and record the re-evaluation basis.

**Eligibility becomes satisfied:** progress through the existing completion approval/conferment lifecycle.

**Eligibility becomes unsatisfied:** establish the applicable non-eligible/affected completion position under L2-C05.

**Qualification already conferred:** L2-C05 determines whether correction/revocation/no-change is required; Examination does not make the qualification decision.

## 11. Failure, Interruption and Business Recovery

**Revised result not authoritative:** do not re-evaluate completion from a provisional result.

**Examination evidence still unresolved:** wait for the Examination-owned current evidence position.

**Other completion inputs conflict:** resolve those inputs through their canonical owners before finalizing completion.

**Qualification correction required:** L2-C05 owns the institutional qualification decision; no direct modification is made by Examination.

## 12. Business Rules and Decision Constraints

- Final Approved/authoritative examination results are inputs to completion; completion does not recompute them.
- The current examination completion evidence supersedes prior evidence only when established by its canonical owner.
- Completion eligibility may change when authoritative academic inputs change.
- Prior decisions and evidence remain historically traceable.
- Qualification correction/revocation remains owned by L2-C05.

## 13. State and Lifecycle Impact

The scenario does not create a new completion state. It causes re-evaluation through the existing L2-C05 Qualification Completion lifecycle.

The approved post-conferment ordering in L2-C05 is preserved: Examination first disposes its own completion evidence, then L2-C05 alone determines the qualification consequence.

## 14. Business Events and Cross-Domain Collaboration

L2-I09 remains the canonical publisher for result-processing facts; L2-I10 remains the canonical publisher for result-publication facts. L2-I13's existing `ExaminationCompletionEvidencePrepared` event is the established cross-domain evidence boundary identified by L2-C05.

No new `RevisedResultToCompletion` event is invented. L2-C05 publishes the authoritative completion consequence through its own event model.

## 15. Business Authorization, Approval and Audit Obligations

- Result revision follows Examination authority and approval.
- Completion re-evaluation follows L2-C05 completion authority.
- Any post-conferment correction/revocation is auditable and institutionally governed.
- The prior and current completion bases must remain traceable.

## 16. Completion Criteria and Postconditions

**Complete when:** the authoritative revised examination evidence is established; completion re-evaluation is performed; one current completion position is authoritative; any required qualification consequence is processed through L2-C05; and history is preserved.

**Postconditions:** completion eligibility reflects the current examination evidence; Examination retains ownership of examination facts; downstream consumers receive the authoritative completion outcome.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-C05 explicitly defines the approved post-conferment ordering: Examination completion evidence is first re-evaluated by Examination, then L2-C05 consumes `ExaminationCompletionEvidencePrepared` and alone determines qualification correction/revocation/no-change.
2. Exact result-revision event names beyond those explicitly identified by L2-C05 are not invented.
3. L2-I09 explicitly excludes Academic Completion, so no completion decision is made there.
