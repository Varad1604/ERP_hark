# Authorized Result Change to Withdrawn Invalid Completion Evidence

**Scenario ID:** L3-I-157
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

Restore a completion-evidence position to valid active use after the condition that made the evidence unusable has been authoritatively resolved, while preserving the evidence lifecycle and ownership boundaries.

## 3. Business Outcome

The examination-owned completion evidence is authoritatively restored to a valid current position and can again support its governed downstream use without changing the institutional Qualification Completion decision unless an authoritative academic outcome also changes.

## 4. Scenario Boundary and Scope

**Start Boundary:** Examination-owned completion evidence is not currently valid for active use and the underlying condition has been authoritatively resolved.

**End Boundary:** One restored, valid completion-evidence position is authoritative for the affected student and completion scope.

**In Scope:** confirm resolution of the condition; verify the evidence basis; restore the evidence position; preserve historical disposition; make the current evidence available for governed downstream use.

**Out of Scope:** recalculating examination results, changing qualification ownership, or creating a new downstream evidence lifecycle.

**Related Scenarios:** This scenario is distinct from evidence correction where the evidence itself must be changed, and from result-change scenarios where the underlying Examination fact changes.

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

- The prior evidence disposition is known and traceable.
- The condition preventing active evidence use has an authoritative resolution.
- The affected student and completion scope are established.
- Any underlying Examination fact required for restoration is authoritative.
- Student Lifecycle remains the owner of the institutional Qualification Completion decision.

## 8. Trigger

An authoritative resolution makes previously invalid, withdrawn, or otherwise inactive completion evidence eligible for restoration.

## 9. Main Business Flow

1. Identify the affected completion-evidence position and the condition that caused its prior invalid/inactive status.
2. Confirm that the responsible upstream condition has been authoritatively resolved.
3. Confirm the underlying academic evidence remains valid for the affected completion scope.
4. Reassess the examination-owned completion evidence against the resolved condition.
5. Restore the evidence to the applicable valid current position through the existing I13 lifecycle.
6. Preserve the prior invalid/inactive history and the basis for restoration.
7. Make the restored current evidence available to the governed downstream consumers.
8. Do not reinterpret the restored evidence as a new Qualification Completion decision.
9. End when one restored valid evidence position is authoritative.

## 10. Alternate Business Flows

**Underlying Examination result also changed:** do not restore from the old basis; consume the new authoritative Examination outcome and re-evaluate completion evidence.

**Resolution has no effect on evidence validity:** retain the existing disposition rather than restoring evidence without sufficient basis.

**Qualification was already conferred:** restoration of examination evidence does not itself alter the qualification; any qualification consequence remains with L2-C05 if an authoritative academic change requires one.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Resolution is not authoritative | Evidence cannot safely be restored | Retain the existing disposition until the canonical owner establishes resolution | Relevant source owner |
| Underlying Examination fact changed | Previous evidence basis may no longer be valid | Consume the current authoritative Examination fact and re-evaluate | Examination / Academic Completion |
| Student context is uncertain | Evidence may be restored to the wrong student | Resolve context through Student Lifecycle before restoration | Student Lifecycle |
| Downstream consumer still has the former disposition | Obsolete evidence may remain in use | Establish the restored current evidence as authoritative and preserve history | Academic Completion & Completion Evidence |

## 12. Business Rules and Decision Constraints

- Restoration requires an authoritative resolution of the condition that prevented active evidence use.
- Restoration does not create a new qualification decision.
- Examination-owned academic facts remain owned by Examination.
- Historical evidence dispositions remain traceable.
- No local rule is invented to override an upstream evidence disposition.

## 13. State and Lifecycle Impact

The scenario uses the existing I13 completion-evidence lifecycle. It does not introduce a new `Restored` state unless that state is already defined by the locked L2 architecture.

The business outcome is a restored valid evidence position; exact lifecycle labels remain upstream-controlled.

## 14. Business Events and Cross-Domain Collaboration

L2-I13 identifies `ExaminationCompletionEvidencePrepared` as the existing Examination-owned evidence boundary to L2-C05. Where the restored current evidence is supplied through that boundary, the existing event remains authoritative.

No new `CompletionEvidenceRestored` event is invented.

## 15. Business Authorization, Approval and Audit Obligations

Restoration must follow the business authority defined by the existing I13 evidence governance. The basis for restoration, prior disposition, current evidence, and any consequential qualification review must be auditable.

No technical access control or access-control mechanism is defined.

## 16. Completion Criteria and Postconditions

**Complete when:** the resolution is authoritative; the evidence basis is valid; the evidence is restored through the existing lifecycle; the current restored position is authoritative; and prior history remains traceable.

**Postconditions:** the restored evidence can support its governed downstream purpose; Student Lifecycle retains Qualification Completion authority; Examination retains ownership of underlying academic facts.

## 17. Open Issues, Assumptions and Deferred Questions

The accessible locked L2-I13 source explicitly states that evidence may be restored or re-established only after the Examination-owned condition is authoritatively resolved. It also prohibits direct cross-domain modification. The source does not define a dedicated restoration event or universally named restoration state, so neither is invented at L3.
