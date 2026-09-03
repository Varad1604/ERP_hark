# Upstream Examination Dependency to Blocked Academic Completion

**Scenario ID:** L3-I-161
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L2-I13 — Academic Completion, CGPA & Completion Evidence Workflows; L2-C05 — Qualification Completion & Student Exit Workflows; applicable Examination result publication, revaluation/rechecking, malpractice and supplementary/backlog/improvement workflows  
**Referenced By:** Downstream qualification and completion-evidence consumers as applicable

## 1. Scenario Metadata

This scenario realizes the frozen canonical `CAN-L3-I-166` semantics under final identity `L3-I-161`. Academic Completion & Completion Evidence remains the canonical owner of examination-derived completion evidence. Examination-owned outcomes remain with their canonical Examination owners, and Student Lifecycle retains the institutional Qualification Completion decision.

## 2. Scenario Purpose

Govern the academic-completion position when a required Examination-owned outcome is not yet authoritative, so that completion evidence does not become conclusive from incomplete or unresolved examination facts and normal completion evaluation resumes only after the upstream Examination condition is resolved.

## 3. Business Outcome

Academic completion remains authoritatively blocked while a required Examination outcome is unpublished or unresolved. Once the applicable Examination owner establishes the authoritative outcome, Academic Completion & Completion Evidence can resume its normal evidence evaluation using that outcome without recomputing or owning it.

## 4. Scenario Boundary and Scope

**Start Boundary:** Academic completion evaluation requires an Examination-owned outcome that is not yet authoritative because the result is unpublished, revaluation/rechecking is pending, a malpractice matter affecting the outcome is unresolved, or a required subsequent-examination outcome is pending.

**End Boundary:** Either the completion-evidence position remains authoritatively blocked because the required Examination outcome is still unresolved, or the authoritative Examination outcome becomes available and normal I13 completion evaluation can resume.

**In Scope:** identify the material upstream Examination dependency; preserve the completion context while the dependency is unresolved; prevent conclusive completion-evidence progression from incomplete examination facts; consume the eventual authoritative Examination outcome; resume I13 evaluation when the dependency clears.

**Out of Scope:** publishing or changing Examination results; deciding revaluation/rechecking; adjudicating malpractice; conducting supplementary/backlog/improvement examinations; inventing provisional academic outcomes; deciding institutional Qualification Completion; or defining technical dependency orchestration.

**Related Scenarios:** Result-change evidence disposition, evidence restoration, duplicate evidence resolution, and identity-association correction remain separate canonical scenarios.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns only the examination-derived completion-evidence position and its evaluation under L2-I13. Examination owns the upstream result, revaluation/rechecking, malpractice and subsequent-examination outcomes through their respective canonical workflows. Student Lifecycle (C) remains the sole owner of the institutional Qualification Completion decision under L2-C05.

Cross-domain participation does not transfer ownership. I13 consumes authoritative Examination facts; it does not create, infer, revise or approve them.

## 6. Business Actors, Roles and Responsibilities

- **Academic Completion & Completion Evidence authority:** identifies when required Examination evidence is insufficient for conclusive completion evaluation and governs the I13-owned completion-evidence position.
- **Applicable Examination authority:** resolves and establishes the authoritative upstream Examination outcome under its canonical workflow.
- **Student Lifecycle authority:** determines the institutional Qualification Completion consequence when authoritative completion evidence is available.
- **Student:** is the subject of the completion evaluation.

No additional approval authority or cross-domain responsibility is introduced.

## 7. Preconditions and Entry Criteria

- The student and programme/completion scope are identifiable.
- Academic completion evaluation has reached a point at which a specific Examination-owned outcome is materially required.
- The required upstream outcome is not yet authoritative.
- The unresolved condition belongs to its canonical Examination owner.
- No conclusive completion-evidence position may be derived by assuming the missing outcome.

## 8. Trigger

Academic completion evaluation encounters a required Examination-owned outcome that is unpublished or unresolved.

## 9. Main Business Flow

1. Identify the affected student, completion scope and required Examination-owned outcome.
2. Determine that the required outcome is not yet authoritative.
3. Classify the dependency according to the applicable upstream Examination workflow without transferring ownership to I13.
4. Retain the I13 completion-evidence position as non-conclusive and prevent progression that would require the missing authoritative outcome.
5. Preserve the dependency, affected completion context and reason progression cannot conclude.
6. Wait for the canonical Examination owner to establish the authoritative outcome through its existing workflow.
7. Consume the authoritative Examination fact when it becomes available; do not recompute or reinterpret it inside I13.
8. Resume normal I13 completion-evidence evaluation using the now-authoritative examination basis.
9. If completion evidence is subsequently prepared, make it available to L2-C05 through the existing canonical evidence boundary; L2-C05 alone determines the institutional Qualification Completion outcome.
10. End when the dependency remains explicitly unresolved and completion remains blocked, or when the authoritative outcome is available and completion evaluation has validly resumed.

## 10. Alternate Business Flows

**Unpublished result:** completion remains blocked until the official result is published through the canonical result-publication workflow.

**Pending revaluation/rechecking:** completion remains blocked where the pending review materially affects the required examination basis; evaluation resumes after the authoritative revaluation/rechecking outcome is established.

**Unresolved malpractice matter:** completion remains blocked where the unresolved malpractice outcome can affect the required examination basis; evaluation resumes only after the canonical malpractice authority establishes the applicable outcome.

**Pending subsequent examination:** completion remains blocked where completion depends on a supplementary, backlog or improvement examination outcome; evaluation resumes after the canonical subsequent-examination outcome is authoritative.

**Dependency does not affect completion:** if authoritative Examination evidence establishes that the unresolved matter is not material to the completion basis, I13 continues evaluation using the authoritative available facts without inventing a new outcome.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required result is unpublished | Completion evidence cannot conclude | Remain blocked until official result publication establishes the required outcome | Examination / Academic Completion |
| Revaluation or rechecking remains unresolved | Current result basis may not be final for completion | Remain blocked where material until the canonical revaluation/rechecking outcome is established | Examination / Academic Completion |
| Malpractice outcome affecting examination basis is unresolved | Completion basis is not authoritative | Remain blocked until the malpractice authority establishes the applicable outcome | Examination / Academic Completion |
| Required subsequent-examination outcome is pending | Completion requirement remains unresolved | Remain blocked until the supplementary/backlog/improvement outcome is authoritative | Examination / Academic Completion |
| Conflicting or non-authoritative examination facts are presented | I13 cannot safely conclude completion evidence | Use only the canonical authoritative Examination outcome and retain the position as unresolved until that outcome is established | Examination / Academic Completion |
| Authoritative dependency becomes available | The blockage condition is cleared | Resume normal I13 completion-evidence evaluation using the authoritative outcome | Academic Completion & Completion Evidence |

## 12. Business Rules and Decision Constraints

- Academic completion evaluation shall not conclude from unpublished, provisional or otherwise non-authoritative Examination outcomes.
- I13 consumes Examination outcomes; it does not calculate, publish, revise or approve them.
- An unresolved upstream Examination condition blocks only the completion-evidence progression that materially depends on that condition.
- Clearing the dependency does not itself establish Qualification Completion; it only permits I13 evaluation to resume.
- Student Lifecycle under L2-C05 alone determines the institutional Qualification Completion decision.
- No local precedence rule, provisional-result rule, exception waiver, or substitute Examination outcome is introduced.

## 13. State and Lifecycle Impact

This scenario uses the existing L2-I13 lifecycle and source-defined blocked business outcome. It does not introduce a new canonical `Blocked` lifecycle state.

While a material upstream Examination dependency is unresolved, the completion-evidence position remains non-conclusive within the existing lifecycle. When the authoritative dependency is resolved, normal I13 evaluation resumes through the existing valid transitions.

## 14. Business Events and Cross-Domain Collaboration

I13 consumes existing canonical Examination facts/events as applicable to the resolved dependency, including:

- `ExaminationResultPublished` for authoritative published results;
- `RevaluationCompleted` for completed revaluation/rechecking where applicable; and
- `SupplementaryExaminationCompleted` for subsequent-examination completion where applicable.

Any malpractice-related outcome remains governed by the canonical malpractice workflow and its existing authoritative outcome/event model; this scenario does not invent a new malpractice event.

No `CompletionDependencyBlocked`, `CompletionDependencyResolved`, or equivalent new business event is introduced.

After the upstream dependency clears and I13 subsequently prepares authoritative completion evidence, the existing `ExaminationCompletionEvidencePrepared` event remains the I13-to-C05 evidence boundary.

## 15. Business Authorization, Approval and Audit Obligations

Each upstream Examination outcome must be established by its canonical authority. I13 has no authority to waive, infer, substitute or approve a missing Examination outcome.

The student/completion context, unresolved dependency, authoritative source owner, reason completion could not conclude, eventual authoritative Examination outcome, resumption of completion evaluation, and resulting evidence position must remain auditable.

No technical access-control or implementation mechanism is defined.

## 16. Completion Criteria and Postconditions

**Blocked terminal condition:** the required Examination outcome remains unresolved, the completion-evidence position is explicitly non-conclusive, and no downstream qualification decision is derived from the missing outcome.

**Continuation condition:** the required Examination owner establishes the authoritative outcome and I13 resumes its normal completion-evidence evaluation using that fact.

**Postconditions:** Examination ownership is preserved; I13 has not fabricated or recomputed the missing outcome; Student Lifecycle retains Qualification Completion authority; and the dependency and its resolution remain traceable.

## 17. Open Issues, Assumptions and Deferred Questions

No new architecture decision is required for this scenario after the 2026-09-03 controlled semantic amendment. Exact technical orchestration, persistence, user-interface handling, retry behavior and transport mechanisms remain outside L3.
