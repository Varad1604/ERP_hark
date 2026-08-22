# Student Academic Progression Decision

**Scenario ID:** L3-C-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Derivation:** KEEP  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Last Updated:** 2026-08-20  
**Exact Filename:** `L3-C-027_Student_Academic_Progression_Decision.md`

> **Production-authoring note:** This document is authored to production-quality L3 standards and is intended for architecture review. Per Phase 5 governance, a newly authored scenario must remain **Status: Draft** until independently approved; “production-ready” does not mean self-approved or locked.

---


| Attribute | Value |
|---|---|
| Final L3 ID | `L3-C-027` |
| Canonical Title | Student Academic Progression Decision |
| Primary Owner | Academic Progression |
| Participating Domains | C, I |
| Scenario Type | Critical |
| Priority | P0 |
| Derivation | KEEP |
| Assigned Author | Varad |
| Status | Draft |

The frozen Section C register identifies C027 exactly with Academic Progression as owner, C and I as participating domains, Critical type, P0 priority and KEEP derivation.

**Applicable authoritative L2:** `L2-C04_Student_Academic_Progression_Workflows.md`.

---

## 2. Scenario Purpose

This scenario governs the institution's authoritative decision on whether a Student progresses academically after an evaluation cycle.

Academic Progression is the canonical owner of the progression decision and Academic Standing. Examination provides officially published academic outcomes; Academic Registration provides the completed registration context; Curriculum and policy sources provide the applicable requirements.

The scenario converts verified academic evidence into one authoritative progression outcome without modifying examination results or registration ownership.

---

## 3. Business Outcome

The institution has produced an authoritative, policy-governed progression decision for the Student for the applicable academic evaluation cycle.

The decision may establish:

- Progressed;
- Conditional Progression;
- Academic Probation;
- Detained; or
- Qualification Eligible where the progression requirements are fully satisfied and the L2-C04 qualification-eligibility workflow is reached.

The decision is reproducible from the approved policy and authoritative academic information and is permanently auditable.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an authoritative progression-evaluation trigger occurs, including:

- `ExaminationResultPublished`;
- academic evaluation window opening;
- manual institutional evaluation;
- approved re-evaluation;
- appeal decision; or
- supplementary result publication.

## 4.2 End Boundary

The scenario ends when the authoritative progression state/decision has been established and its canonical progression outcomes have been published.

If the Student becomes **Qualification Eligible**, responsibility proceeds to L2-C05 for qualification completion/exit processing.

## 4.3 In Scope

- Collection of authoritative academic inputs.
- Validation of progression eligibility.
- Earned-credit evaluation.
- Application of one approved progression policy.
- Academic standing determination.
- Progression decision.
- Conditional progression/probation/detention outcomes where applicable.
- Progression events.
- Audit evidence.

## 4.4 Out of Scope

- Examination evaluation or result creation.
- Academic registration creation/modification.
- Curriculum ownership.
- Finance clearance ownership.
- Qualification conferment.
- Alumni transition.
- Technical policy engines, APIs, databases, queues or infrastructure.

---

## 5. Ownership and Participating Domains

| Domain | Participation | Ownership |
|---|---|---|
| **Academic Progression** | Evaluates academic evidence and owns the authoritative progression decision and standing. | **Canonical owner** |
| **Student Lifecycle (C)** | Provides Student lifecycle context and consumes progression outcomes where required. | Retains Student lifecycle ownership. |
| **Examination (I)** | Provides officially published examination outcomes. | Retains examination/result ownership. |

L2-C04 explicitly states that Academic Progression consumes official examination outcomes and owns progression decisions and academic standing. It does not perform examination evaluation or modify examination results.

---

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Academic Progression | Executes progression evaluation and records the authoritative decision. | Sole owner of progression decision. |
| Academic administration / authorised progression authority | Provides required institutional approval where policy requires it. | Cannot redefine the progression rules outside approved policy. |
| Examination domain | Publishes official examination outcomes. | Does not make progression decisions. |
| Student Lifecycle | Consumes progression outcomes for Student lifecycle consequences where applicable. | Does not independently promote/detain the Student. |

No additional actor is invented beyond the L2 architecture's policy/approval model.

---

## 7. Preconditions and Entry Criteria

The L2-C04 APW-01 workflow requires:

- Student is Active.
- Academic Registration is completed.
- Examination is officially published.
- Curriculum is available.
- Progression policy is active.
- Academic calendar permits evaluation.

Only official published examination outcomes may participate in authoritative progression.

---

## 8. Trigger

The canonical progression-evaluation triggers are:

- `ExaminationResultPublished`;
- Academic Evaluation Window Opens;
- Manual Institutional Evaluation;
- Approved Re-evaluation;
- Appeal Decision;
- Supplementary Result Publication.

The trigger is a business fact, not a technical implementation event.

---

## 9. Main Business Flow

1. A canonical progression-evaluation trigger occurs.
2. Academic Progression confirms that the Student is Active.
3. Academic Registration completion is confirmed.
4. Official examination results are collected.
5. Curriculum and applicable academic requirements are established.
6. The active progression policy is identified.
7. Earned credits are calculated/validated.
8. Progression eligibility is evaluated.
9. Academic standing is determined.
10. The authoritative progression outcome is established.
11. The progression decision is recorded with the applicable policy reference.
12. The applicable canonical progression event(s) are published.
13. The decision is audited.
14. The scenario ends at the resulting progression state.

The L2-C04 APW-01 workflow explicitly follows: collect academic data → validate eligibility → calculate earned credits → apply progression policies → determine academic standing → publish progression decision → update Student progression.

---

## 10. Alternate Business Flows

## 10.1 Progressed

The Student satisfies progression requirements and enters **Progressed**.

## 10.2 Conditional Progression

Where policy permits, the Student enters **Conditional Progression** subject to defined academic conditions.

Typical conditions in L2-C04 include backlog subjects, credit deficiency, improvement examination, mandatory credit recovery and department approval.

## 10.3 Academic Probation

Where conditional requirements are not satisfied or policy directly assigns probation, the Student enters **Academic Probation**.

## 10.4 Detention

Where progression requirements remain unsatisfied, the Student may enter **Detained**.

## 10.5 Qualification Eligibility

Where progression and completion requirements are satisfied, the Student may reach **Qualification Eligible**, handing control to L2-C05.

## 10.6 Re-evaluation / Supplementary Outcome

A permitted re-evaluation or supplementary result may trigger a fresh progression evaluation rather than modifying the historical decision.

---

## 11. Failure, Interruption and Business Recovery

| Condition | Business Outcome | Resolution |
|---|---|---|
| Official examination outcome unavailable | Progression remains blocked. | Await/resolve official publication through Examination. |
| Mandatory academic data incomplete | Evaluation cannot complete. | Resolve missing authoritative academic information. |
| No active progression policy | Evaluation cannot complete. | Activate/resolve the applicable institutional policy. |
| Student not Active | Progression cannot be established through APW-01. | Resolve Student lifecycle condition through C02. |
| Academic Registration incomplete | Progression cannot complete. | Resolve registration lifecycle through C03. |
| Re-evaluation changes official results | Fresh evaluation occurs where policy permits. | Preserve historical decision; create a new decision record. |
| Appeal changes progression basis | Re-evaluate under the authoritative appeal outcome. | Preserve historical progression history. |

Technical retry and infrastructure recovery are excluded.

---

## 12. Business Rules and Decision Constraints

The authoritative C04 rules include:

- only officially published examination results may participate;
- exactly one approved institutional progression policy applies to an evaluation;
- one authoritative progression decision exists for each academic evaluation cycle;
- all mandatory academic evaluations must be complete;
- progression remains blocked while mandatory data is incomplete;
- re-evaluation/supplementary outcomes may trigger a fresh evaluation where policy permits;
- historical progression decisions are immutable;
- only successfully earned credits contribute to progression;
- transferred credits contribute only after institutional approval;
- duplicate academic credits are prohibited;
- credit calculations follow the applicable curriculum structure;
- every active Student has exactly one active Academic Standing;
- standing changes only follow an approved progression workflow;
- manual overrides require authorised institutional approval.

These rules remain owned by Academic Progression and are not duplicated into C or I.

---

## 13. State and Lifecycle Impact

The locked C04 progression lifecycle is:

```text
Created
  ↓
Awaiting Evaluation
  ↓
Evaluation Running
  ├──► Progressed
  ├──► Conditional Progression
  ├──► Academic Probation
  └──► Detained
```

Further transitions include:

- Conditional Progression → Progressed when conditions are satisfied.
- Conditional Progression → Academic Probation when conditions fail.
- Academic Probation → Progressed when review passes.
- Academic Probation → Detained when review fails.
- Detained → Evaluation Running after approved appeal.
- Detained → Progressed when requirements are completed.
- Progressed → Qualification Eligible when programme completion requirements are satisfied.

No lifecycle state outside the locked C04 state model is introduced.

---

## 14. Business Events and Cross-Domain Collaboration

L2-C04 APW-01 explicitly defines these published progression outcomes:

- `ProgressionEvaluated`
- `StudentProgressed`
- `StudentDetained`
- `StudentOnProbation`
- `QualificationEligibilityUpdated`

**Publisher:** Academic Progression.

`ExaminationResultPublished` is consumed as an evaluation trigger from Examination.

The Student Lifecycle domain consumes progression outcomes where its own Student lifecycle requires an update. Examination remains the canonical owner of examination outcomes.

No C027-specific event is invented beyond the locked C04 event catalogue.

---

## 15. Business Authorization, Approval and Audit Obligations

Progression decisions are institutional records and must be deterministic, policy-driven and auditable.

The L2 architecture requires:

- approved institutional policy;
- authorised manual intervention;
- immutable progression history;
- policy-version reference;
- audit record for every transition;
- authorised approval for manual overrides.

The decision authority is recorded as part of the progression business record where applicable.

No technical access control/IAM design is introduced.

---

## 16. Completion Criteria and Postconditions

The scenario is complete when:

- all mandatory academic data is available;
- official examination results are used;
- the active progression policy is applied;
- earned credits are evaluated;
- academic standing is determined;
- an authoritative progression decision exists;
- the applicable progression state is established;
- the corresponding progression event is published;
- audit evidence exists.

Postconditions:

- one authoritative progression decision exists for the evaluation cycle;
- Academic Standing is authoritative;
- historical progression history remains immutable;
- downstream domains consume the decision without redefining it.

If **Qualification Eligible** is reached, control transfers to L2-C05.

---

## 17. Open Issues, Assumptions and Deferred Questions

## Open Issues

**OI-C027-01 — Policy Management ownership is unresolved in L2-C04.**  
The source metadata identifies “Policy Management — unresolved, no owner” as an input. C027 therefore treats the progression policy as an authoritative prerequisite but does not invent a policy-owner domain.

**OI-C027-02 — Exact institution-specific approval routing is configuration-driven.**  
No universal approver is invented.

## Assumptions

1. Examination remains the canonical owner of official examination results.
2. Academic Progression owns all progression decisions and Academic Standing.
3. Student Lifecycle consumes consequences but does not independently promote/detain.
4. Historical decisions are immutable.

## Deferred Questions

- Exact owner and contract for institutional progression policy.
- Exact manual-override authority.
- Detailed policy-version lifecycle.
- Institution-specific progression thresholds.


## Final Authoring / Architecture-Safety Validation

- [x] Frozen L3 identity preserved.
- [x] Frozen title, owner, participating domains, Type and Priority preserved.
- [x] Assigned author is Varad for Section C.
- [x] Exact filename preserved.
- [x] Applicable locked L2 workflow source(s) used.
- [x] Ownership remains with the canonical owner.
- [x] Cross-domain participation does not transfer aggregate/lifecycle ownership.
- [x] Only source-supported actors, rules, lifecycle states, events and publishers are used.
- [x] Business events are kept distinct from synchronous business contracts/hand-offs.
- [x] Failure/recovery remains business-level; technical retries/queues/rollback are excluded.
- [x] No API, database, Business Data Structure, class, code, UI, infrastructure or deployment design is introduced.
- [x] All 17 mandatory sections are present.
- [x] Material upstream ambiguity is explicitly recorded in Section 17.
- [x] Status remains `Draft` pending independent architecture approval.

**Final Classification: PASS WITH ISSUES — Draft**
