# Completion Evidence Identity Mismatch to Correct Candidate Association

**Scenario ID:** L3-I-160
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C, H, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I13 — Academic Completion, CGPA & Completion Evidence Workflows; L2-C05 — Qualification Completion & Student Exit Workflows; applicable Examination result/revaluation workflows and participating Section H architecture  
**Referenced By:** Downstream qualification, completion-evidence and institutional-service consumers as applicable

## 1. Scenario Metadata

This Draft preserves the frozen canonical scenario semantics and final L3 identity. Academic Completion & Completion Evidence remains the canonical owner of examination-derived completion evidence. Cross-domain participation does not transfer ownership of Examination results, Student Lifecycle qualification decisions, or downstream service responsibilities.

## 2. Scenario Purpose

Correct examination-owned completion evidence before its downstream institutional use is treated as final, ensuring that the current evidence position accurately reflects authoritative academic outcomes and remains distinct from the institutional Qualification Completion decision.

## 3. Business Outcome

One corrected and authoritative completion-evidence position is established for the affected student and scope before final downstream use, with the prior evidence position retained as history and without changing qualification ownership.

## 4. Scenario Boundary and Scope

**Start Boundary:** An examination-owned completion-evidence position exists but a material correction is required before final downstream use.

**End Boundary:** The corrected evidence position is authoritative and available for its governed downstream purpose.

**In Scope:** identify the evidence discrepancy; determine whether the discrepancy is evidence-only or caused by an upstream Examination change; apply the applicable correction/re-evaluation path; preserve history; establish the corrected evidence position.

**Out of Scope:** recalculating Examination results, changing Qualification Completion, or inventing a downstream evidence lifecycle.

**Related Scenarios:** I161 addresses an authoritative Examination result change that withdraws invalid completion evidence; this scenario addresses pre-release correction of the evidence position.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns the examination-derived completion-evidence lifecycle and its authoritative evidence position. Student Lifecycle (C) remains the sole owner of the institutional Qualification Completion decision under L2-C05. Examination (I) owns examination results and any underlying academic outcome changes. Section H participates in governed downstream use where applicable and does not acquire ownership of qualification or examination evidence.

## 6. Business Actors, Roles and Responsibilities

- **Academic Completion & Completion Evidence authority:** evaluates and governs the examination-owned completion-evidence position.
- **Examination authority:** establishes the authoritative examination result or revised examination outcome when the underlying academic fact changes.
- **Student Lifecycle authority:** determines the institutional Qualification Completion consequence.
- **Student:** is the subject of the academic completion evidence.
- **Section H participating business authority/consumer:** uses governed completion evidence for its defined institutional purpose where applicable.

No additional actor, approval authority, or responsibility is introduced beyond authoritative upstream architecture.

## 7. Preconditions and Entry Criteria

- An examination-owned completion-evidence position exists.
- The affected student and completion scope are identifiable.
- Underlying Examination facts are authoritative unless separately identified as changed.
- The correction is within the existing I13 evidence-governance boundary.

## 8. Trigger

A material discrepancy in completion evidence is identified before final downstream use.

## 9. Main Business Flow

1. Identify the affected completion-evidence position, student and completion scope.
2. Determine whether the discrepancy is confined to the evidence position or originates from an underlying Examination fact.
3. If the underlying Examination fact has not changed, apply the existing I13 evidence correction/disposition path.
4. If the underlying Examination fact has changed, consume the newly authoritative Examination outcome and re-evaluate the completion evidence.
5. Preserve the prior evidence position and correction basis as history.
6. Establish one corrected current completion-evidence position through the existing I13 lifecycle.
7. Make the corrected position available to the participating downstream domains for authorized use.
8. Do not treat evidence correction as a new Qualification Completion decision.
9. End when the corrected evidence position is authoritative.

## 10. Alternate Business Flows

**Evidence-only correction:** correct the I13-owned evidence position while leaving underlying Examination results unchanged.

**Underlying result changed:** stop treating the case as evidence-only correction and consume the canonical Examination result outcome before re-evaluating evidence.

**No material correction required:** retain the current evidence position and close the correction concern without creating a new business outcome.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Underlying Examination fact is not authoritative | Corrected evidence cannot be finalized | Resolve the Examination fact first | Examination |
| Evidence correction cannot be completed | Current evidence remains unsuitable for final downstream use | Preserve history and retain the governed current position until correction completes | Academic Completion & Completion Evidence |
| Student context is inconsistent | Evidence may be associated with the wrong student | Resolve context through Student Lifecycle before finalizing evidence | Student Lifecycle |
| Downstream consumer has the earlier evidence | An obsolete position may continue to be used | Establish the corrected authoritative position and follow applicable downstream correction governance | Academic Completion & Completion Evidence / participating domain |

## 12. Business Rules and Decision Constraints

- I13 owns examination-derived completion evidence; it does not own the underlying Examination result.
- Evidence correction must use authoritative upstream facts.
- Evidence correction does not itself confer, revoke, or redefine qualification.
- Historical evidence positions remain traceable.
- No local correction threshold or unsupported precedence rule is invented.

## 13. State and Lifecycle Impact

The existing I13 completion-evidence lifecycle is used. No new `Corrected` lifecycle state is created unless already defined upstream.

Where an underlying Examination outcome changes, the evidence lifecycle follows the locked post-conferment ordering: Examination establishes the current outcome first; I13 re-evaluates its own evidence; Student Lifecycle alone determines any qualification consequence.

## 14. Business Events and Cross-Domain Collaboration

The existing `ExaminationCompletionEvidencePrepared` event remains the canonical I13 evidence boundary to L2-C05 where the corrected current evidence is supplied for qualification review.

No new `CompletionEvidenceCorrected` event is invented. Existing upstream event names and publishers remain authoritative.

## 15. Business Authorization, Approval and Audit Obligations

Correction authority remains within the existing I13 evidence governance. Where the underlying academic result changes, Examination retains its own approval/result authority. Any institutional Qualification Completion consequence remains with L2-C05.

The discrepancy, source facts, correction basis, current evidence and downstream consequence must remain auditable. No technical authorization mechanism is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** the discrepancy is resolved through the correct canonical owner; the current completion evidence is corrected and authoritative; prior evidence history is preserved; and the corrected position is available for its governed downstream use.

**Postconditions:** one current evidence position is authoritative; Examination facts remain owned by Examination; Student Lifecycle retains Qualification Completion authority; downstream consumers can use the corrected evidence.

## 17. Open Issues, Assumptions and Deferred Questions

L2-I13 explicitly governs examination-owned completion evidence and distinguishes it from Qualification Completion. It does not expose a dedicated pre-release correction event, so none is invented. If a correction materially changes an Examination result, this scenario must hand back to the canonical Examination result workflow before evidence is re-evaluated.
