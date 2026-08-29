# Duplicate Completion Evidence to Single Authoritative Evidence Position

**Scenario ID:** L3-I-159
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C, H, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I13 — Academic Completion, CGPA & Completion Evidence Workflows; L2-C05 — Qualification Completion & Student Exit Workflows; applicable Section H architecture for downstream institutional evidence use  
**Referenced By:** Downstream qualification and institutional evidence consumers as applicable

## 1. Scenario Metadata

This Draft preserves the frozen Stage 3 canonical title, Primary Owner, participating domains, Scenario Type, Priority and final L3 identity. Cross-domain participation does not transfer ownership of qualification, examination, or student lifecycle facts.

## 2. Scenario Purpose

Correct a completion-evidence association when the evidence is linked to the wrong or inconsistent student identity/context, so that the authoritative evidence is associated only with the correct candidate without altering the underlying academic or qualification ownership.

## 3. Business Outcome

The completion evidence is associated with the correct candidate and authoritative student context, with the erroneous association preserved as history and prevented from producing an incorrect institutional outcome.

## 4. Scenario Boundary and Scope

**Start Boundary:** A completion-evidence position is identified whose student identity/context association is incorrect, inconsistent, or otherwise not authoritative.

**End Boundary:** The evidence is authoritatively associated with the correct candidate and completion scope.

**In Scope:** identify the mismatch; verify authoritative student context; correct the evidence association through the owning evidence lifecycle; preserve history; prevent incorrect downstream use.

**Out of Scope:** changing student identity itself, changing examination results, or making a Qualification Completion decision.

**Related Scenarios:** Student identity correction remains owned by the canonical identity/student-lifecycle architecture; this scenario only addresses the evidence association.

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

- A completion-evidence position exists.
- A mismatch in candidate association is identified.
- Student identity/context can be verified through its canonical owner.
- The evidence provenance and completion scope can be established.
- The evidence owner remains Academic Completion & Completion Evidence.

## 8. Trigger

A mismatch is identified between the candidate associated with completion evidence and the authoritative student context.

## 9. Main Business Flow

1. Identify the completion-evidence position and the associated candidate context.
2. Verify the authoritative student identity/context through its canonical owner.
3. Determine whether the mismatch is in the evidence association or in the underlying student record.
4. If the student record is authoritative and unchanged, correct the evidence association through the I13 evidence lifecycle.
5. If the student record itself is incorrect, resolve that upstream condition before finalizing evidence association.
6. Preserve the erroneous association and correction basis as historical evidence.
7. Establish one authoritative evidence-to-candidate association.
8. Make the corrected evidence available for governed downstream use.
9. End when the evidence is associated with the correct candidate.

## 10. Alternate Business Flows

**Student identity itself is incorrect:** do not correct identity within I13; resolve it through the canonical Student Lifecycle/identity owner, then reassess the evidence association.

**Evidence belongs to another candidate:** do not merge records or transfer academic facts locally; resolve the evidence ownership/association through the canonical workflow.

**Mismatch is only contextual:** correct the evidence association while preserving the underlying Examination facts unchanged.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Student context is not authoritative | Correct association cannot be established | Resolve student context through its canonical owner first | Student Lifecycle |
| Evidence provenance cannot be established | Association correction may corrupt academic history | Keep evidence unresolved and escalate through I13 governance | Academic Completion & Completion Evidence |
| Evidence is actually for another candidate | Incorrect transfer could create a false academic record | Do not reassign underlying academic facts locally; resolve through canonical ownership | Academic Completion / Examination |
| Downstream consumer has used incorrect evidence | An institutional process may have relied on wrong candidate context | Establish the corrected authoritative evidence and follow applicable downstream correction governance | Academic Completion & Completion Evidence / participating domain |

## 12. Business Rules and Decision Constraints

- Student identity/context is canonical upstream information.
- Completion evidence must be associated with the correct candidate before it is treated as authoritative.
- Correcting evidence association does not change Examination-owned academic results.
- Correcting evidence association does not itself decide Qualification Completion.
- Historical incorrect association and correction basis remain traceable.
- No local identity-matching rule is invented where upstream identity architecture is authoritative.

## 13. State and Lifecycle Impact

The evidence lifecycle is affected only through the correction of its candidate association.

No new student-identity or completion lifecycle state is introduced. If the authoritative student context changes, that change remains within the canonical Student Lifecycle/identity lifecycle.

## 14. Business Events and Cross-Domain Collaboration

Student Lifecycle/identity architecture remains the publisher/authority for student context. Academic Completion & Completion Evidence remains the owner of its evidence position.

The existing `ExaminationCompletionEvidencePrepared` event remains the applicable evidence boundary where current evidence is supplied to L2-C05. No new identity-mismatch event is invented.

## 15. Business Authorization, Approval and Audit Obligations

Candidate association correction must be supported by authoritative student context and evidence provenance. The mismatch, verification basis, correction, and resulting authoritative association must be auditable.

Student identity authority and qualification authority remain separate from evidence ownership. No technical IAM design is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** authoritative student context is verified; the mismatch is resolved through the correct owner; one evidence-to-candidate association is authoritative; historical mismatch evidence is preserved; and downstream use can rely on the corrected association.

**Postconditions:** completion evidence is associated with the correct candidate; Examination facts remain owned by Examination; Student Lifecycle retains identity and qualification authority; downstream consumers have a single authoritative association.

## 17. Open Issues, Assumptions and Deferred Questions

The locked architecture clearly separates Student Lifecycle ownership of student context/qualification from I13 ownership of examination-derived completion evidence. The accessible L2-I13 source does not define a dedicated identity-mismatch event or separate evidence state, so neither is invented.

If the mismatch is caused by an upstream identity correction rather than an evidence-association error, the upstream correction must be completed before this scenario can reach its end boundary.
