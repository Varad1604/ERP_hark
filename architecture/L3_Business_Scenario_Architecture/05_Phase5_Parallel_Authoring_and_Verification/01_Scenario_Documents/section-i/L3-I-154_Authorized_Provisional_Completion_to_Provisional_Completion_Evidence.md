# Authorized Provisional Completion to Provisional Completion Evidence

**Scenario ID:** L3-I-154  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C, H, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I13 — Academic Completion, CGPA & Completion Evidence Workflows; L2-C05 — Qualification Completion & Student Exit Workflows; applicable Section H participating-domain architecture  
**Referenced By:** Downstream academic completion, qualification and institutional evidence consumers as applicable

## 1. Scenario Metadata

This Draft preserves the frozen final L3 identity, canonical title, Primary Owner, participating domains, Scenario Type and Priority. Phase 5 author assignment does not transfer canonical ownership.

## 2. Scenario Purpose

Provide a governed examination-owned provisional completion-evidence outcome when provisional academic completion is authorized, while preserving Student Lifecycle as the sole authority for the institutional Qualification Completion decision.

## 3. Business Outcome

An authorized provisional completion position results in corresponding provisional completion evidence that is clearly treated as evidence rather than as a final institutional qualification decision.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authorized provisional completion basis exists under the applicable academic-completion governance.

**In Scope:** establish the provisional examination-owned evidence position, distinguish it from final Qualification Completion, and make the governed evidence available to participating downstream domains.

**Out of Scope:** recalculating marks/grades/CGPA, changing Examination-owned results, or conferring a qualification.

**End Boundary:** Provisional completion evidence is established and governed for its permitted institutional use.

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

- The academic outcomes consumed by L2-I13 are authoritative for the information available to completion-evidence preparation.
- The applicable provisional-completion authorization exists under the upstream completion workflow.
- Student identity and programme context are authoritative.
- The final Qualification Completion decision remains with L2-C05.

## 8. Trigger

An authorized provisional completion position is established and requires a provisional completion-evidence outcome.

## 9. Main Business Flow

1. Identify the affected student and completion scope.
2. Confirm the authoritative academic outcome basis available to L2-I13.
3. Confirm that the provisional completion condition is authorized under the applicable upstream governance.
4. Evaluate the examination-owned completion evidence against the permitted provisional basis.
5. Establish the provisional completion-evidence position without representing it as a final qualification decision.
6. Make the governed provisional evidence available to participating downstream domains for their authorized use.
7. Preserve the distinction between provisional evidence and the final Student Lifecycle qualification decision.
8. End when the provisional completion-evidence position is authoritative within its permitted scope.

## 10. Alternate Business Flows

**Provisional basis is sufficient only for evidence:** establish provisional evidence while leaving final Qualification Completion to Student Lifecycle.

**Later authoritative evidence changes the basis:** re-evaluate the examination-owned evidence through the applicable completion-evidence lifecycle.

**Final qualification decision is made:** the provisional evidence is treated only as an academic input to the Student Lifecycle decision.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required academic evidence is not authoritative | Provisional evidence cannot be responsibly established | Keep the completion-evidence position unresolved until the authoritative input exists | Academic Completion & Completion Evidence |
| Provisional authorization is absent or no longer applicable | The provisional outcome cannot be used | Do not establish or continue the provisional evidence position | Academic Completion & Completion Evidence |
| Provisional evidence conflicts with an authoritative Examination fact | Evidence position is not reliable | Re-evaluate against the current Examination-owned fact | Examination / Academic Completion |
| Qualification decision is required | Provisional evidence alone cannot confer qualification | Route the evidence to the existing Student Lifecycle Qualification Completion governance | Student Lifecycle |

## 12. Business Rules and Decision Constraints

- L2-I13 owns examination-derived completion evidence; L2-C05 owns the institutional Qualification Completion decision.
- Provisional evidence must not be represented as a qualification award.
- Examination-owned academic facts are consumed rather than recomputed.
- No local provisional-completion rule or authority is invented.

## 13. State and Lifecycle Impact

The examination-owned completion-evidence lifecycle is affected; no new lifecycle state is introduced here.

The provisional evidence position remains subordinate to the final Student Lifecycle Qualification Completion lifecycle.

## 14. Business Events and Cross-Domain Collaboration

No new event is invented.

Where the evidence position is communicated across the Examination/Student Lifecycle boundary, L2-I13's existing `ExaminationCompletionEvidencePrepared` event remains the canonical Examination-owned evidence event. L2-C05 consumes that current evidence position and retains qualification ownership.

Section H participation is preserved, but no unsupported H-specific event is asserted.

## 15. Business Authorization, Approval and Audit Obligations

The provisional-completion authorization must be supported by the authoritative upstream completion workflow. The resulting evidence must remain auditable as an examination-owned evidence position.

Student Lifecycle retains authority for the final qualification decision. No technical access control or access-control design is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** the authorized provisional basis is established; the examination-owned provisional evidence position is authoritative; its provisional nature is preserved; and it is available for its permitted downstream use.

**Postconditions:** provisional completion evidence exists as an examination-owned fact; it does not itself confer qualification; Student Lifecycle retains final Qualification Completion authority.

## 17. Open Issues, Assumptions and Deferred Questions

The frozen Stage 3 semantics establish the scenario and its C/H/I participation. The accessible L2-I13 source establishes examination-owned completion evidence and the existing `ExaminationCompletionEvidencePrepared` event, but does not expose a dedicated provisional-evidence event name. Therefore no new event is invented.

No scenario-specific assumption is used to override the locked ownership boundary.
