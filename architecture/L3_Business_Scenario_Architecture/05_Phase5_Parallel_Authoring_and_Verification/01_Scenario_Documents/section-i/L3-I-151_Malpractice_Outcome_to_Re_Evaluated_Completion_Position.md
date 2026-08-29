# Malpractice Outcome to Re-Evaluated Completion Position

**Scenario ID:** L3-I-151  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-C05 Qualification Completion & Student Exit Workflows; applicable locked Examination malpractice/result workflows; L2-I09 Result Processing & Computation; L2-I10 Result Publication  
**Referenced By:** Downstream completion, student-lifecycle, and evidence consumers as applicable

## 1. Scenario Metadata

This scenario preserves the frozen Stage 3 semantics, final L3 identity, canonical owner, participating domains, Scenario Type, and Priority. Phase 5 author assignment does not transfer canonical ownership.

## 2. Scenario Purpose

Ensure that an authoritative examination malpractice outcome that changes examination evidence is reflected in the student's qualification-completion position without transferring malpractice or examination-result ownership to Academic Completion & Completion Evidence.

## 3. Business Outcome

The current Examination evidence position arising from the authoritative malpractice outcome is consumed by Academic Completion & Completion Evidence, the affected completion requirements are re-evaluated, and one current completion position is established. Any qualification consequence remains a completion-domain decision.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authoritative malpractice outcome has affected examination evidence relevant to an existing or completed qualification-completion evaluation.

**In Scope:** consume the current authoritative examination evidence position; determine its impact on completion requirements; re-evaluate completion; establish the current completion position; preserve prior completion history.

**Out of Scope:** malpractice adjudication, penalty determination, examination-result computation, publication of examination results, or direct modification of Student Lifecycle state.

**End Boundary:** The affected qualification-completion position reflects the authoritative examination evidence.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns qualification eligibility, qualification approval, degree conferment and qualification consequences.

Examination owns malpractice adjudication and examination-result/evidence facts.

Student Lifecycle owns Student Context and student academic lifecycle state.

Cross-domain consumption does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

Only source-supported roles are stated. The Examination authority/committee establishes the authoritative malpractice outcome and its examination consequence. The completion authority evaluates the qualification consequence. Student Lifecycle supplies authoritative student context. Exact named malpractice roles are not asserted where the accessible L2 source does not define them.

## 7. Preconditions and Entry Criteria

- The malpractice outcome is authoritative under the Examination workflow.
- The resulting examination evidence position has been established by Examination.
- The affected student and qualification-completion scope are identifiable.
- The prior completion position, if any, is traceable.

## 8. Trigger

An authoritative malpractice outcome changes or confirms examination evidence relevant to qualification completion.

## 9. Main Business Flow

1. Identify the affected student and qualification-completion scope.
2. Consume the authoritative examination evidence position after Examination has completed its own malpractice/result disposition.
3. Preserve the prior examination and completion decision history.
4. Determine which completion requirements are affected.
5. Re-evaluate qualification completion using the current examination evidence and other authoritative completion inputs.
6. If completion eligibility is unchanged, retain the current completion position and record the re-evaluation.
7. If eligibility changes, establish the new authoritative completion position through the existing completion lifecycle.
8. If a qualification has already been conferred, determine through the completion workflow whether correction, revocation, or no change is required.
9. Publish applicable completion-domain business facts through the canonical completion owner.
10. End when the current completion position is authoritative.

## 10. Alternate Business Flows

**No completion impact:** the malpractice outcome is recorded as having no effect on the completion decision.

**Completion eligibility changes:** progress through the existing completion governance and approval lifecycle.

**Qualification already conferred:** Academic Completion & Completion Evidence alone determines the qualification consequence after consuming the current Examination evidence position.

**Examination evidence is restored:** re-evaluate completion against the restored authoritative evidence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Malpractice outcome is not yet authoritative | Completion cannot rely on it as a final examination basis | Wait for the Examination-owned authoritative outcome | Examination |
| Examination evidence remains unresolved | Completion determination remains unresolved | Consume the current evidence only after Examination disposition | Examination / Academic Completion |
| Conflicting source facts remain | Competing completion positions cannot be finalized | Resolve through the canonical owner before completion determination | Canonical source owner |
| Qualification consequence is required | Prior institutional qualification position may be affected | Apply the existing L2-C05 correction/revocation/no-change path | Academic Completion & Completion Evidence |

## 12. Business Rules and Decision Constraints

- Examination owns malpractice decisions and examination consequences.
- Completion consumes authoritative examination evidence and does not adjudicate malpractice.
- A change in authoritative examination evidence may require completion re-evaluation.
- A qualification correction or revocation remains owned by Academic Completion & Completion Evidence.
- Historical examination and completion decisions remain traceable.
- No local malpractice severity, penalty, or qualification-consequence rule is invented.

## 13. State and Lifecycle Impact

The scenario uses the existing qualification-completion lifecycle. No malpractice-specific completion state is introduced.

L2-C05 establishes that when Examination-originated evidence changes, Examination first re-evaluates/dispositions its own completion evidence and then supplies the superseding current evidence position through the existing `ExaminationCompletionEvidencePrepared` event. Academic Completion then determines the qualification consequence. No lifecycle state is renamed or created here.

## 14. Business Events and Cross-Domain Collaboration

The canonical cross-domain boundary explicitly identified by L2-C05 is `ExaminationCompletionEvidencePrepared`, published by the Examination completion-evidence workflow after Examination has established its current evidence position.

The completion domain publishes its own canonical qualification/completion facts. No new malpractice-to-completion event is invented.

## 15. Business Authorization, Approval and Audit Obligations

Malpractice adjudication and examination consequences remain subject to Examination authority. Qualification completion decisions and any post-conferment correction/revocation remain subject to Academic Completion authority.

The decision basis, contributing examination evidence, prior position, current position, and any qualification consequence must remain auditable. No technical authorization model is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** the malpractice outcome is authoritative; Examination has established the current evidence position; completion requirements have been re-evaluated; one current completion position is authoritative; and any qualification consequence has been processed through the existing completion lifecycle.

**Postconditions:** the completion position reflects the current authoritative examination evidence; Examination retains ownership of examination facts; Academic Completion retains ownership of the qualification decision; historical positions remain traceable.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-C05 explicitly states the approved post-conferment ordering: Examination first dispositions its own completion evidence, then publishes the existing `ExaminationCompletionEvidencePrepared` event; Academic Completion alone determines correction, revocation, or no change.
2. The accessible sources do not expose a separate malpractice-to-completion event; none is invented.
3. Exact malpractice role names and penalty rules remain upstream-owned where not explicitly defined.
