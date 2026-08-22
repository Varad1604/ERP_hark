# Examination Exception Resolution to Re-Evaluated Completion Position

**Scenario ID:** L3-I-152  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-C05 Qualification Completion & Student Exit Workflows; applicable locked Examination Exception & Contingency workflows; L2-I09 Result Processing & Computation; L2-I10 Result Publication  
**Referenced By:** Downstream completion, student-lifecycle, and evidence consumers as applicable

## 1. Scenario Metadata

This scenario preserves the frozen Stage 3 semantics, final L3 identity, canonical owner, participating domains, Scenario Type, and Priority. Phase 5 author assignment does not transfer canonical ownership.

## 2. Scenario Purpose

Ensure that an authoritative resolution of an examination exception affecting examination evidence is reflected in the student's qualification-completion position through governed re-evaluation.

## 3. Business Outcome

The resolved examination exception has an authoritative resulting examination evidence/result position, and Academic Completion re-evaluates the affected qualification-completion position using that current evidence.

## 4. Scenario Boundary and Scope

**Start Boundary:** An examination exception has been authoritatively resolved and its resolution changes or confirms examination evidence relevant to completion.

**In Scope:** consume the authoritative post-resolution examination position; identify completion impact; re-evaluate completion; preserve prior decisions; establish the current completion position.

**Out of Scope:** adjudicating or resolving the examination exception itself, changing Examination-owned results, or independently publishing examination results.

**End Boundary:** The qualification-completion position reflects the authoritative post-resolution examination evidence.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns qualification eligibility and qualification consequences.

Examination Exception & Contingency and other Examination workflows own exception resolution and examination outcomes.

Student Lifecycle owns Student Context and student academic lifecycle state.

Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- Examination exception authority: establishes the authoritative exception resolution and its examination consequence.
- Examination result/evidence authority: establishes the resulting current examination evidence.
- Completion authority: re-evaluates qualification completion and determines any qualification consequence.
- Student Lifecycle: provides authoritative student context.

The exact named exception role is not asserted where not defined by the accessible L2 source.

## 7. Preconditions and Entry Criteria

- The exception resolution is authoritative.
- Any resulting Examination evidence/result position is authoritative under Examination ownership.
- The affected student and completion scope are known.
- Prior completion decisions remain traceable.

## 8. Trigger

An authoritative examination exception resolution changes or confirms examination evidence relevant to qualification completion.

## 9. Main Business Flow

1. Identify the affected student and completion scope.
2. Consume the authoritative Examination position established after exception resolution.
3. Preserve the prior examination and completion history.
4. Identify completion requirements affected by the resolved exception.
5. Re-evaluate qualification completion using the current Examination evidence and all other authoritative completion inputs.
6. Retain the existing completion position if the outcome is unchanged, while recording the re-evaluation.
7. Establish a revised completion position through L2-C05 if eligibility changes.
8. Where a qualification has already been conferred, apply the existing L2-C05 correction/revocation/no-change determination.
9. Publish applicable completion-domain facts through the canonical completion owner.
10. End when the current completion position is authoritative.

## 10. Alternate Business Flows

**Exception resolution has no academic impact:** retain the completion position after the governed re-evaluation.

**Exception resolution changes examination evidence:** use the new authoritative evidence and re-evaluate completion.

**Qualification already conferred:** L2-C05 alone determines the institutional qualification consequence.

**Other completion input remains unresolved:** retain the governed unresolved/blocked completion position until that canonical input is resolved.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Exception resolution is not final | Completion cannot rely on a provisional Examination position | Wait for the authoritative Examination outcome | Examination |
| Examination evidence remains unresolved after exception handling | Completion cannot be conclusively determined from that evidence | Consume the evidence only after its owning workflow reaches an authoritative position | Examination |
| Another mandatory completion input is unresolved | Completion cannot be finalized | Resolve through its canonical owner, then re-evaluate | Relevant source owner / Academic Completion |
| Prior qualification position requires correction | Existing institutional outcome may no longer be valid | Apply the L2-C05 correction/revocation/no-change governance | Academic Completion & Completion Evidence |

## 12. Business Rules and Decision Constraints

- Examination exception resolution remains an Examination-owned business responsibility.
- Completion consumes the authoritative post-resolution Examination position.
- Completion does not recalculate Examination results.
- A changed authoritative academic input may trigger completion re-evaluation.
- No local exception classification, severity, remedy, or completion consequence is invented.

## 13. State and Lifecycle Impact

The scenario reuses the existing completion lifecycle. No new `ExceptionResolved` completion state or other local state is introduced.

Where the exception changes Examination completion evidence, the L2-C05 ordering remains authoritative: Examination establishes its own current evidence position first, then Academic Completion consumes it and determines the qualification consequence.

## 14. Business Events and Cross-Domain Collaboration

The cross-domain evidence boundary explicitly identified by L2-C05 is `ExaminationCompletionEvidencePrepared`. Examination remains the publisher of its examination evidence position; Academic Completion publishes its own qualification/completion outcome.

No new `ExaminationExceptionResolvedForCompletion` event is invented.

## 15. Business Authorization, Approval and Audit Obligations

Exception resolution is governed by Examination authority. Completion re-evaluation and any qualification correction/revocation are governed by Academic Completion.

The exception resolution, resulting Examination evidence, completion inputs, re-evaluation basis, and final completion position must be auditable. No technical authorization design is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** the exception resolution is authoritative; the resulting Examination position is authoritative; completion has re-evaluated the affected requirements; one current completion position is established; and any required qualification consequence is governed through L2-C05.

**Postconditions:** completion reflects the authoritative post-exception Examination position; Examination retains examination ownership; Academic Completion retains qualification ownership; historical decisions remain traceable.

## 17. Open Issues, Assumptions and Deferred Questions

1. The accessible Phase 4 row defines the outcome but does not itself define the exception-resolution lifecycle.
2. The dedicated Examination Exception & Contingency L2 document was not available through the current repository retrieval; exact exception states/events are therefore not invented.
3. L2-C05 explicitly defines `ExaminationCompletionEvidencePrepared` as the existing examination-to-completion evidence boundary; no replacement event is introduced.
