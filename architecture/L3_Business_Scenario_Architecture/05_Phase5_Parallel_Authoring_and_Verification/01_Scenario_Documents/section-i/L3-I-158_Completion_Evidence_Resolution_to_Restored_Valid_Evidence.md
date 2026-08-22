# Completion Evidence Resolution to Restored Valid Evidence

**Scenario ID:** L3-I-158
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C, H, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I13 — Academic Completion, CGPA & Completion Evidence Workflows; L2-C05 — Qualification Completion & Student Exit Workflows; applicable Section H architecture for downstream institutional evidence use  
**Referenced By:** Downstream qualification and institutional evidence consumers as applicable

## 1. Scenario Metadata

This Draft preserves the frozen Stage 3 canonical title, Primary Owner, participating domains, Scenario Type, Priority and final L3 identity. Cross-domain participation does not transfer ownership of qualification, examination, or student lifecycle facts.

## 2. Scenario Purpose

Ensure that multiple completion-evidence positions for the same student and completion scope do not create competing current institutional evidence, while preserving legitimate evidence history.

## 3. Business Outcome

One current completion-evidence position is authoritative for the affected student and scope; duplicate or superseded evidence remains historically traceable and cannot compete as the current position.

## 4. Scenario Boundary and Scope

**Start Boundary:** Multiple materially equivalent completion-evidence positions exist or are presented for the same student and completion scope.

**End Boundary:** A single current completion-evidence position is authoritative.

**In Scope:** identify duplicate evidence; distinguish legitimate historical/reissued evidence from competing current evidence; establish the authoritative current position; preserve history.

**Out of Scope:** changing underlying examination results, changing Qualification Completion, or deleting historical evidence.

**Related Scenarios:** Reissue scenarios remain distinct where replacement evidence is legitimate and completion remains unchanged.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns the examination-derived completion-evidence lifecycle and its authoritative evidence position. Student Lifecycle (C) remains the canonical owner of the institutional Qualification Completion decision. Section H participates in governed downstream institutional use where applicable; participation does not transfer ownership. Examination-owned academic outcomes remain owned by Examination.

## 6. Business Actors, Roles and Responsibilities

- **Academic Completion & Completion Evidence authority:** determines and governs the examination-owned completion-evidence position.
- **Student Lifecycle authority:** retains authority for the institutional Qualification Completion decision.
- **Student:** is the subject of the completion evidence.
- **Section H participating business authority/consumer:** uses governed evidence for its own downstream institutional purpose where defined by its architecture.
- **Examination authority:** remains responsible for any underlying examination result or academic fact that is consumed as evidence.

No additional actor or authority is introduced without an authoritative upstream definition.

## 7. Preconditions and Entry Criteria

- Multiple evidence positions can be associated with the same student and completion scope.
- The evidence provenance and scope can be established.
- The canonical evidence owner is identifiable.
- Underlying Examination facts remain authoritative and traceable.

## 8. Trigger

A duplicate or competing completion-evidence position is identified for the same student and completion scope.

## 9. Main Business Flow

1. Identify all evidence positions associated with the affected student and scope.
2. Determine whether each position is a legitimate historical, corrected, or replacement evidence position.
3. Identify the position that represents the current authoritative evidence under the existing I13 lifecycle.
4. Ensure obsolete or duplicate positions are no longer treated as competing current evidence, without destroying required history.
5. Preserve provenance and the basis for the authoritative current position.
6. Make the current authoritative evidence position available to governed downstream consumers.
7. End when one evidence position is authoritative for current use.

## 10. Alternate Business Flows

**Legitimate reissue:** retain the reissued evidence as current and preserve prior evidence history.

**Identical duplicate with no substantive distinction:** consolidate current use around the canonical evidence position while retaining required history.

**Underlying academic facts differ:** do not select a winner locally; resolve the underlying Examination fact through its canonical owner before finalizing current evidence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| No authoritative current evidence can be identified | Competing evidence remains possible | Do not finalize a current position; resolve through I13 governance | Academic Completion & Completion Evidence |
| Underlying Examination facts conflict | Evidence cannot reliably establish one current position | Resolve the underlying fact through Examination first | Examination |
| Student identity/context differs | Evidence may belong to different scopes rather than being duplicate | Resolve student context and reassess the evidence association | Student Lifecycle |
| Downstream consumer has a superseded position | Institutional use may rely on obsolete evidence | Use the current authoritative evidence position and preserve history | Academic Completion & Completion Evidence |

## 12. Business Rules and Decision Constraints

- One current completion-evidence position must be authoritative for a given student and completion scope.
- Legitimate reissue/correction is distinct from duplicate current evidence.
- Historical evidence must remain traceable.
- Evidence ownership remains with Academic Completion & Completion Evidence.
- No local precedence rule is invented where upstream evidence governance is silent.

## 13. State and Lifecycle Impact

The existing completion-evidence lifecycle is used to distinguish the current authoritative position from historical/superseded evidence.

No new `Duplicate` or `Consolidated` state is introduced unless already defined by L2-I13.

## 14. Business Events and Cross-Domain Collaboration

The existing `ExaminationCompletionEvidencePrepared` boundary remains applicable to the current examination-owned evidence position.

No new duplicate-resolution event is invented. Cross-domain consumers use the current canonical evidence position rather than selecting among competing evidence positions independently.

## 15. Business Authorization, Approval and Audit Obligations

The current evidence position and the basis for treating other positions as historical/superseded must be auditable. Any approval requirement is the one defined by I13; downstream consumers do not acquire evidence ownership.

No technical authorization model is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** all competing evidence positions are identified; legitimate historical/replacement evidence is distinguished from duplicate current evidence; one current position is authoritative; provenance is preserved; and downstream use is aligned to the current position.

**Postconditions:** exactly one current evidence position is authoritative for the evaluated scope; historical evidence remains traceable; underlying academic and qualification ownership is unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

The locked architecture establishes canonical ownership and the existing evidence boundary but does not define a dedicated duplicate-evidence event. The scenario therefore uses existing lifecycle behaviour and records any unresolved precedence issue rather than inventing a new rule.
