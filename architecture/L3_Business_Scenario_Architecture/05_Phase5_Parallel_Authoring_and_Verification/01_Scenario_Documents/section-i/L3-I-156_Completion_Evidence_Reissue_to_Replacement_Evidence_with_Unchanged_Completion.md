# Completion Evidence Reissue to Replacement Evidence with Unchanged Completion

**Scenario ID:** L3-I-156  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C, H, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I13 — Academic Completion, CGPA & Completion Evidence Workflows; L2-C05 — Qualification Completion & Student Exit Workflows; applicable Section H participating-domain architecture  
**Referenced By:** Downstream academic completion, qualification and institutional evidence consumers as applicable

## 1. Scenario Metadata

This Draft preserves the frozen final L3 identity, canonical title, Primary Owner, participating domains, Scenario Type and Priority. Phase 5 author assignment does not transfer canonical ownership.

## 2. Scenario Purpose

Reissue completion evidence when the evidence representation must be replaced while the underlying academic completion position remains unchanged, preserving the distinction between evidence replacement and a new qualification decision.

## 3. Business Outcome

A replacement completion-evidence outcome is established with unchanged academic completion, while prior evidence history remains traceable and the qualification decision is not reinterpreted.

## 4. Scenario Boundary and Scope

**Start Boundary:** An existing completion-evidence outcome remains substantively valid, but its evidence must be reissued/replaced under governed completion-evidence rules.

**In Scope:** confirm unchanged completion, replace the evidence position through the existing evidence lifecycle, preserve history, and make the replacement evidence available to participating domains.

**Out of Scope:** changing academic results, re-evaluating qualification eligibility from a changed academic basis, or conferring/revoking qualification.

**End Boundary:** Replacement completion evidence is authoritative and the underlying completion position remains unchanged.

## 5. Ownership and Participating Domains

- **Academic Completion & Completion Evidence:** owns examination-derived completion evidence and its lifecycle.
- **Student Lifecycle:** owns the institutional Qualification Completion decision; it does not become an owner of Examination completion evidence.
- **Institutional Services & Intelligence:** participates where governed downstream evidence/service use is applicable; participation does not transfer ownership.
- Examination-owned academic facts remain owned by Examination.

## 6. Business Actors, Roles and Responsibilities

- **Academic Completion & Completion Evidence authority:** performs the examination-owned completion-evidence evaluation and evidence disposition.
- **Student Lifecycle authority:** remains responsible for the institutional Qualification Completion decision.
- **Student:** is the subject of the academic completion evidence.
- **Section H participating service/consumer:** receives or uses governed completion evidence where its authoritative workflow requires it.
- Exact additional approval roles are not asserted where the accessible authoritative sources do not define them.

## 7. Preconditions and Entry Criteria

- Existing completion evidence is authoritative enough to establish the current completion position.
- The reissue condition is governed by the existing completion-evidence workflow.
- No underlying academic result change requires re-evaluation.
- Student and programme context remain authoritative.

## 8. Trigger

A governed need to reissue completion evidence is identified while the underlying completion position remains unchanged.

## 9. Main Business Flow

1. Identify the existing completion-evidence position and the student context.
2. Confirm that the underlying academic completion position has not changed.
3. Determine that the required action is evidence replacement rather than academic re-evaluation.
4. Preserve the existing evidence history and completion position.
5. Establish the replacement evidence through the existing I13 evidence lifecycle.
6. Confirm that the replacement evidence represents the same completion position.
7. Make the replacement evidence available to participating downstream domains for authorized use.
8. End when the replacement evidence is authoritative and the completion position remains unchanged.

## 10. Alternate Business Flows

**Underlying completion unchanged:** replace evidence without reopening the completion determination.

**Underlying academic result changes:** stop treating the case as evidence-only reissue and re-evaluate completion evidence under the applicable result-change scenario.

**Reissue request does not satisfy the governed evidence condition:** retain the existing authoritative evidence position until the applicable business requirement is resolved.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Underlying academic outcome changes | The unchanged-completion premise no longer holds | Re-evaluate completion evidence against the new authoritative Examination outcome | Examination / Academic Completion |
| Existing evidence is not valid for replacement | Replacement cannot safely be established | Apply the existing evidence resolution path rather than inventing a new reissue state | Academic Completion & Completion Evidence |
| Student association is uncertain | Replacement evidence could be associated incorrectly | Resolve student context through its canonical owner | Student Lifecycle |
| Downstream consumer has an older evidence position | Consumer may rely on superseded evidence | Use the current authoritative replacement evidence; preserve historical traceability | Academic Completion & Completion Evidence |

## 12. Business Rules and Decision Constraints

- Evidence replacement does not by itself change academic completion.
- The underlying completion position remains governed by the authoritative academic evidence.
- Academic Completion owns completion evidence; Student Lifecycle owns Qualification Completion.
- Historical evidence must remain distinguishable from the current replacement position.
- No local reissue rule or new lifecycle state is invented.

## 13. State and Lifecycle Impact

The evidence lifecycle advances to the replacement evidence position while the underlying completion position remains unchanged.

No new qualification lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

The existing `ExaminationCompletionEvidencePrepared` event remains the canonical Examination-owned evidence boundary where applicable.

No new `CompletionEvidenceReissued` or equivalent event is invented because the accessible authoritative architecture does not establish such an event. The replacement evidence is distinguished as the current evidence position through the existing evidence lifecycle.

## 15. Business Authorization, Approval and Audit Obligations

The business authority for evidence replacement must follow the existing L2-I13 governance. The unchanged completion position must be demonstrable from authoritative evidence.

Evidence replacement and its unchanged-completion basis must remain auditable. No technical authorization or access-control mechanism is defined.

## 16. Completion Criteria and Postconditions

**Complete when:** the unchanged completion position is confirmed; replacement evidence is established through the existing evidence lifecycle; the replacement is authoritative; prior evidence history is preserved; and downstream use can rely on the current evidence.

**Postconditions:** replacement evidence is current; the underlying completion position is unchanged; Student Lifecycle retains qualification authority; historical evidence remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

The frozen Stage 3 semantics explicitly include Section H participation, but the accessible authoritative L2 sources do not define a dedicated H-specific evidence-reissue event. No such event is invented.

If downstream Section H processing requires a specific business contract or event beyond the existing I13 evidence boundary, that definition belongs upstream and should be resolved before architecture approval.
