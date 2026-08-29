# Revised Result to Re Evaluated Completion Eligibility

**Scenario ID:** L3-I-150
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-C05 Qualification Completion & Student Exit; applicable locked Examination Malpractice workflow; L2-I09 Result Processing & Computation; Examination completion-evidence workflow

## 1. Scenario Metadata

The canonical Stage 3 semantics and Stage 4 final identity are preserved. The scenario remains owned by Academic Completion & Completion Evidence. Cross-domain participation does not transfer ownership of examination or student lifecycle facts.

## 2. Scenario Purpose

Re-evaluate a student's qualification completion position when an authoritative examination malpractice outcome changes or materially affects the examination evidence used by completion evaluation.

## 3. Business Outcome

The authoritative malpractice outcome is reflected in the examination evidence consumed by completion, and the completion position is re-evaluated. Any qualification consequence is determined by Academic Completion & Completion Evidence rather than by the malpractice workflow.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authoritative malpractice outcome is established and it affects examination evidence relevant to a student's completion position.

**In Scope:** consume the authoritative examination/malpractice consequence; establish the current examination evidence position; re-evaluate completion; preserve prior completion history; apply any completion consequence through L2-C05.

**Out of Scope:** adjudicating malpractice, determining penalties, changing examination results locally, or making the qualification decision within Examination.

## 5. Ownership and Participating Domains

Malpractice & Examination Committee/Examination owns malpractice adjudication and its examination consequences.

Examination owns examination evidence and result facts.

Academic Completion & Completion Evidence owns the completion decision and any qualification correction/revocation.

Student Lifecycle remains authoritative for student context.

## 6. Business Actors, Roles and Responsibilities

- Malpractice authority/committee: establishes the authoritative malpractice outcome.
- Examination/result authority: establishes the resulting examination evidence position.
- Completion authority: re-evaluates qualification completion.
- Student Lifecycle: provides authoritative student context.

## 7. Preconditions and Entry Criteria

- A malpractice case involving examination evidence has an authoritative outcome.
- The affected student and completion scope can be identified.
- The malpractice consequence has been reflected in the canonical Examination evidence/result workflow before completion re-evaluation.
- Prior completion history is traceable.

## 8. Trigger

An authoritative malpractice outcome changes or confirms examination evidence relevant to a student's completion position.

## 9. Main Business Flow

1. Identify the affected student and qualification-completion scope.
2. Consume the authoritative malpractice/examination consequence after it has been established by the owning Examination workflow.
3. Establish the current examination evidence position available to completion.
4. Preserve the prior completion and examination evidence history.
5. Determine whether the malpractice outcome changes a completion requirement.
6. Re-evaluate qualification completion using the current authoritative evidence.
7. If completion eligibility changes, establish the new completion position through L2-C05.
8. If a qualification has already been conferred, determine through L2-C05 whether correction, revocation or no change is required.
9. Publish the applicable completion-domain outcome.
10. End when the current completion position is authoritative.

## 10. Alternate Business Flows

**Malpractice outcome has no completion impact:** retain the completion position after documented re-evaluation.

**Outcome invalidates examination evidence:** consume the Examination-owned invalid/superseding evidence position and re-evaluate completion.

**Outcome restores examination evidence:** consume the restored Examination evidence position and re-evaluate completion.

**Qualification already conferred:** L2-C05 makes the qualification consequence decision after receiving the current Examination evidence; Examination does not revoke the qualification.

## 11. Failure, Interruption and Business Recovery

**Malpractice decision is not final:** no completion consequence is treated as authoritative.

**Examination evidence not yet dispositioned:** completion waits for the Examination-owned evidence position.

**Conflicting malpractice decisions:** Examination malpractice governance establishes the authoritative outcome before completion re-evaluation.

**Student context conflict:** Student Lifecycle remains authoritative.

## 12. Business Rules and Decision Constraints

- Malpractice adjudication and penalty ownership remain with Examination.
- Completion consumes the resulting authoritative examination evidence and does not adjudicate malpractice.
- A malpractice outcome may trigger completion re-evaluation when it changes authoritative examination evidence.
- Qualification correction/revocation remains solely a completion-domain decision.
- Historical examination and completion decisions remain traceable.

## 13. State and Lifecycle Impact

The scenario reuses the existing completion lifecycle. No new `MalpracticeAffected` completion state is created.

Where Examination withdraws or restores its own completion evidence, L2-C05 consumes the resulting `ExaminationCompletionEvidencePrepared` current evidence position and determines the qualification consequence.

## 14. Business Events and Cross-Domain Collaboration

The malpractice workflow and Examination evidence workflow publish their own canonical events. L2-C05 explicitly identifies `ExaminationCompletionEvidencePrepared` as the canonical evidence boundary into completion.

No new malpractice-to-completion event is invented.

## 15. Business Authorization, Approval and Audit Obligations

- Malpractice adjudication follows the authority defined by Examination.
- Completion re-evaluation and any qualification consequence follow L2-C05 authority.
- Decisions must be auditable across the malpractice outcome, examination evidence disposition and completion decision.
- Separation of duties between malpractice adjudication and qualification decision is preserved.

## 16. Completion Criteria and Postconditions

**Complete when:** the malpractice outcome is authoritative; Examination has established the current evidence position; completion has re-evaluated the affected requirements; one current completion position is authoritative; and any qualification consequence is processed through L2-C05.

**Postconditions:** the completion position reflects the authoritative malpractice-derived examination evidence; examination ownership remains intact; qualification decisions remain owned by Academic Completion & Completion Evidence.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-C05 explicitly establishes that Examination-derived completion evidence is an input, not the qualification decision, and that post-conferment Examination changes are processed by Examination first.
2. The accessible repository retrieval did not expose the complete malpractice L2 document/event catalogue, so no malpractice-specific event name is asserted.
3. No local penalty, malpractice severity or qualification consequence rule is invented; those remain upstream-owned.
