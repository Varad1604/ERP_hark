# Upstream Examination Dependency to Blocked Academic Completion

**Scenario ID:** L3-I-161
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

Reflect an authoritative Examination result change in examination-owned completion evidence by withdrawing evidence that is no longer valid, while preserving its history and leaving the institutional qualification consequence to Student Lifecycle.

## 3. Business Outcome

The changed Examination outcome is authoritative, the affected completion evidence is withdrawn from active use with its history preserved, and the superseding current evidence position is made available to L2-C05 for the institutional qualification consequence.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authoritative Examination result change has been established and can affect completion evidence previously prepared from the earlier outcome.

**End Boundary:** Completion evidence that is no longer valid is withdrawn from active use, the current evidence position reflects the changed Examination outcome, and the evidence position is available to L2-C05 for any required qualification determination.

**In Scope:** consume the authoritative changed Examination outcome; re-evaluate I13-owned completion evidence; withdraw invalid evidence; preserve history; prepare the superseding current evidence position.

**Out of Scope:** changing the Examination result itself, deciding whether a qualification is corrected/revoked, or independently modifying Student Lifecycle state.

**Related Scenarios:** I160 covers pre-release evidence correction; this scenario specifically covers an authoritative Examination result change that makes existing completion evidence invalid.

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

- The Examination result change is authoritative under the canonical Examination workflow.
- The affected student and completion scope are identifiable.
- Prior completion evidence is traceable.
- I13 is responsible for evaluating the consequence for its own completion evidence.
- L2-C05 remains responsible for any qualification consequence.

## 8. Trigger

An authoritative Examination result change is established and affects previously prepared completion evidence.

## 9. Main Business Flow

1. Identify the affected student, changed Examination outcome and existing completion-evidence position.
2. Consume the authoritative Examination result change without recomputing or modifying the result within I13.
3. Re-evaluate the I13-owned completion evidence against the changed Examination outcome.
4. Determine whether the prior completion evidence remains valid.
5. Where it is no longer valid, withdraw it from active use through the existing I13 evidence lifecycle while preserving its history.
6. Prepare the superseding current evidence position based only on authoritative current Examination outcomes and applicable completion evidence rules.
7. Publish the superseding evidence position through the existing canonical I13 evidence boundary.
8. L2-C05 consumes the current evidence and alone determines whether the institutional Qualification Completion consequence is correction, revocation, or no change.
9. End when the evidence position is current and authoritative for the changed academic basis.

## 10. Alternate Business Flows

**Result change does not affect completion evidence:** retain the existing evidence position after the governed re-evaluation.

**Result change invalidates evidence:** withdraw the prior evidence and establish the superseding current evidence position.

**Qualification already conferred:** I13 does not revoke the qualification; it supplies the current evidence position and L2-C05 determines correction, revocation, or no change.

**Further Examination evidence is unresolved:** do not publish a final superseding evidence position until the Examination-owned condition is authoritative.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Result change is not authoritative | Existing evidence cannot be judged against a final new basis | Wait for the canonical Examination result outcome | Examination |
| Changed result invalidates prior evidence | Prior completion evidence must not remain active | Withdraw prior evidence with history preserved and establish the superseding position | Academic Completion & Completion Evidence |
| Examination condition remains unresolved | A superseding evidence position cannot be finalized | Keep evidence unresolved until the Examination-owned condition is resolved | Examination / Academic Completion |
| Qualification consequence is required | Qualification may no longer correspond to current evidence | L2-C05 alone determines correction, revocation, or no change | Student Lifecycle |
| Result later restored/re-established | The evidence basis may change again | Re-evaluate I13-owned evidence after the Examination-owned condition is authoritatively resolved | Examination / Academic Completion |

## 12. Business Rules and Decision Constraints

- The Examination result change is owned and established by Examination.
- I13 consumes the changed result and governs only its own completion evidence.
- Invalid evidence is withdrawn from active use with history preserved.
- Evidence may be restored or re-established only after the Examination-owned condition is authoritatively resolved.
- L2-C05 alone determines any qualification correction, revocation, or no-change consequence.
- No local rule is created to decide qualification status from the result change.

## 13. State and Lifecycle Impact

This scenario uses the existing I13 evidence lifecycle and does not introduce a new `Withdrawn` or `Invalid` state. The locked I13 architecture explicitly requires evidence that is no longer valid to be withdrawn from active use with history preserved.

The exact state labels remain those of the locked L2 lifecycle.

## 14. Business Events and Cross-Domain Collaboration

The locked post-conferment decision explicitly establishes `ExaminationCompletionEvidencePrepared` as the I13-to-C05 evidence boundary after I13 has evaluated the current Examination outcome.

The underlying Examination result change uses its existing canonical Examination event, such as `ExaminationResultPublished` or `RevaluationCompleted`, where applicable. No new result-change or evidence-withdrawal event is invented.

## 15. Business Authorization, Approval and Audit Obligations

The Examination result change must be authoritative under Examination governance. I13 evidence withdrawal/re-establishment follows I13 authority. L2-C05 alone determines qualification correction, revocation, or no change.

The original evidence, changed Examination outcome, withdrawal basis, superseding evidence, and any qualification decision must be auditable. No technical access control/IAM design is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** the authoritative Examination result change is consumed; I13 has re-evaluated its own evidence; any invalid prior evidence has been withdrawn with history preserved; the current evidence position is authoritative; and L2-C05 can determine any institutional qualification consequence.

**Postconditions:** obsolete completion evidence is not active; the superseding evidence position reflects the current Examination outcome; qualification ownership remains exclusively with Student Lifecycle; historical evidence remains reconstructable.

## 17. Open Issues, Assumptions and Deferred Questions

The locked I13 ownership-resolution decision is explicit: when an authoritative Examination outcome changes, I13 consumes the existing Examination fact, first re-evaluates its own completion evidence, withdraws evidence that is no longer valid with history preserved, restores/re-establishes only after the Examination-owned condition is resolved, and then publishes the superseding evidence position through `ExaminationCompletionEvidencePrepared`. L2-C05 alone determines qualification correction, revocation, or no change. No new event or canonical owner is introduced.
