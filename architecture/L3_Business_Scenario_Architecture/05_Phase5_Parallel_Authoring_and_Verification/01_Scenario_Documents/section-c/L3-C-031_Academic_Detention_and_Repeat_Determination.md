# Academic Detention and Repeat Determination

**Scenario ID:** L3-C-031  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; `L2-C03_Student_Academic_Registration_Workflows.md` for registration implications; applicable Student Lifecycle context; official Examination outcomes  
**Referenced By:** L3-C-027, L3-C-029, L3-C-030, L3-C-024

## 1. Scenario Metadata

Frozen identity: C031 — Academic Detention and Repeat Determination; Primary Owner Academic Progression; Participating Domains C and I; Critical; P1; KEEP.

---

## 2. Scenario Purpose

This scenario governs the institutional determination that a Student must not progress because applicable academic requirements remain unsatisfied, together with the governed determination of the academic repeat consequence where policy provides one.

L2-C04 defines Academic Detention as the state representing prevention from academic progression and identifies repeat semester/repeat academic year as possible outcomes.

---

## 3. Business Outcome

The institution has authoritatively determined that the Student is **Detained** and, where the applicable policy requires a repeat determination, has established the governed repeat outcome.

Academic Progression remains the canonical owner of the detention decision. Academic Registration subsequently applies the resulting registration consequence through its own registration lifecycle; it does not make the detention decision.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when Academic Progression has sufficient authoritative evidence to determine that progression requirements are not satisfied and the canonical Detention path is applicable.

This may follow progression evaluation or a failed Academic Probation review.

### 4.2 End Boundary

The scenario ends when the Detained state is established and the applicable repeat determination, where required by policy, is recorded.

The scenario does not continue into the subsequent academic registration transaction.

### 4.3 In Scope

- Determination of academic detention.
- Evaluation of the academic basis for detention.
- Determination of repeat semester/repeat academic year where policy requires it.
- Recording detention and repeat outcome.
- Publication of the canonical detention outcome.
- Audit/evidence.

### 4.4 Out of Scope

- Original progression evaluation.
- Academic Probation management.
- Examination evaluation/result creation.
- Academic Registration execution after detention.
- Appeal processing beyond the C04-defined re-entry to Evaluation Running.
- Technical retry, queue, rollback or infrastructure behaviour.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-C-027 Student Academic Progression Decision | May establish detention directly. |
| Preceded by | L3-C-030 Academic Probation Management and Resolution | Failed probation review may transition to Detained. |
| Related | L3-C-024 Backlog and Repeat Course Registration | Registration consequences may require repeat registration after the detention decision. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Academic Progression | Owns the Detention decision, Detained lifecycle and policy-governed repeat determination. |
| Student Lifecycle (C) | Consumes Student lifecycle consequences where applicable; does not own academic detention. |
| Examination (I) | Provides official examination outcomes used as academic evidence. |
| Academic Registration (C03) | Participates only after the detention outcome where registration must reflect the repeat consequence; it retains registration ownership. |

Although C03 may be involved in the downstream repeat-registration consequence, the frozen C031 participating-domain list is C and I. The C03 dependency is therefore treated as an upstream architectural dependency, not as an added participating domain.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Academic Progression | Determines detention and the applicable repeat outcome. | Canonical decision owner. |
| Examination | Supplies official academic outcomes. | Academic evidence source. |
| Academic Registration | Applies the downstream registration consequence when repeat registration is required. | Owns registration, not detention. |
| Student Lifecycle | Consumes relevant Student lifecycle consequence. | Does not determine detention. |

No new detention-specific actor or approver is invented.

---

## 7. Preconditions and Entry Criteria

- Student is subject to an Academic Progression evaluation or failed probation review.
- Official examination outcomes relevant to the decision are available.
- Applicable academic progression policy is active.
- Required academic evidence is complete enough to establish the detention decision.
- Where repeat determination depends on registration/curriculum conditions, those authoritative conditions are available.

---

## 8. Trigger

**Academic Progression determines that the Student has failed the applicable progression requirements and the Detention path is permitted by policy.**

A failed Academic Probation review is an explicit C04 transition into Detained.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Evaluate the authoritative academic basis for non-progression. | Academic Progression | Detention basis is established. |
| 2 | Apply the approved institutional progression policy. | Academic Progression | Detention is confirmed where policy requires it. |
| 3 | Establish the Student's **Detained** lifecycle state. | Academic Progression | Student is prevented from progressing. |
| 4 | Determine the governed repeat consequence where applicable, such as repeat semester or repeat academic year. | Academic Progression | Repeat outcome is established where policy requires one. |
| 5 | Record the detention and repeat outcome and preserve audit evidence. | Academic Progression | Institutional detention decision is complete. |
| 6 | Publish the canonical detention outcome for downstream business consumption. | Academic Progression | Registration/other consumers can act within their own ownership boundaries. |

The Main Business Flow stops once detention and the applicable repeat determination are established.

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | Detention is established after failed probation review | Use the C04 Academic Probation → Detained transition. | Student becomes Detained. | Academic Progression |
| A2 | Detention basis is satisfied after an authoritative requirements-completion outcome | Use the canonical Detained → Progressed path defined by C04. | Student returns to Progressed. | Academic Progression |

A2 is a subsequent governed resolution of an existing detention state; it is included only because C04 explicitly defines **Detained → Requirements Completed → Progressed**.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required official examination evidence is incomplete | Detention cannot be authoritatively finalized solely on incomplete evidence. | Resolve the authoritative academic evidence first. | Academic Progression / Examination |
| Student remains Detained because requirements are incomplete | Normal progression remains blocked. | When institutional requirements are completed, apply the canonical C04 transition to Evaluation Running or Progressed as defined by the governing workflow. | Academic Progression |
| Approved appeal changes the detention basis | Existing detention outcome may require fresh evaluation. | C04 permits **Detained → Appeal Approved → Evaluation Running**. Re-evaluate without silently rewriting history. | Academic Progression |
| Repeat consequence cannot be determined from authoritative academic/curriculum information | Detention may be established but repeat determination remains unresolved if policy permits separation. | Resolve the governing academic condition before establishing the repeat outcome. | Academic Progression |

Technical recovery is excluded.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Academic Detention prevents academic progression. | Academic Progression | Student cannot advance while Detained. | L2-C04 APW-06 |
| Detention may result from failed progression policy, minimum credit deficiency, academic regulations or institutional decisions. | Academic Progression | Establishes valid business bases for detention. | L2-C04 APW-06 |
| Only officially published examination outcomes may participate in progression evaluation. | Academic Progression / Examination | Examination evidence must be authoritative. | L2-C04 BR-PROG-001 |
| Historical progression decisions remain immutable. | Academic Progression | New appeal/re-evaluation outcomes do not erase history. | L2-C04 progression rules |
| Manual overrides require authorized institutional approval and complete audit history. | Academic Progression | Exceptional detention decisions remain governed and auditable. | L2-C04 business constraints |

No local detention rule is created.

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Academic Progression | Evaluation Running | Detention approved. | Detained. | Academic Progression | L2-C04 §4.4 |
| Academic Probation / Academic Progression | Academic Probation | Review fails. | Detained. | Academic Progression | L2-C04 §4.4 |
| Academic Progression | Detained | Approved appeal. | Evaluation Running. | Academic Progression | L2-C04 §4.4 |
| Academic Progression | Detained | Requirements completed. | Progressed. | Academic Progression | L2-C04 §4.4 |

The repeat-semester/repeat-year consequence does not create a new progression state.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Applicable official examination result event | Examination | Academic Progression | Provides authoritative academic evidence. | L2-C04 APW-01 trigger semantics |
| `StudentDetained` | Academic Progression | Registration / relevant downstream consumers | Communicates the authoritative detention outcome. | L2-C04 APW-01 |

C04 also defines `StudentProgressed` for the later requirements-completed resolution, but it is not the primary completion event of the detention establishment path.

No new `RepeatSemesterRequired` or `RepeatAcademicYearRequired` event is invented.

### 14.2 Cross-Domain Business Hand-off

Academic Registration may consume the detention outcome to determine subsequent registration consequences under its own C03 ownership. Academic Registration does not make or alter the Academic Detention decision.

This preserves the canonical dependency direction:

**Academic Progression decision → downstream Academic Registration consequence.**

---

## 15. Business Authorization, Approval and Audit Obligations

Detention is an institutional academic decision and must remain governed by approved progression policy.

The detention decision, its business basis, repeat determination where applicable, and any approved exception/appeal outcome must be auditable.

Manual overrides require authorized institutional approval under the upstream architecture.

No technical IAM/RBAC structure is defined.

---

## 16. Completion Criteria and Postconditions

### Completion

- detention basis is established under approved policy;
- Student is in **Detained** state;
- repeat consequence is established where required and supported by authoritative policy;
- `StudentDetained` is published by Academic Progression;
- audit evidence is complete.

### Postconditions

- Student cannot progress normally while Detained.
- Downstream Academic Registration may apply its own registration rules using the authoritative detention outcome.
- Historical detention evidence remains preserved.
- A later approved appeal or completed requirement may re-enter the canonical C04 resolution path.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

**OI-C031-01 — Exact repeat-determination rules are not fully enumerated in the available C04 excerpt.**  
C04 identifies Repeat Semester and Repeat Academic Year as detention outcomes, but L3 does not invent the policy thresholds selecting between them.

**OI-C031-02 — Exact repeat-registration hand-off contract is not defined here.**  
C031 identifies the business dependency on Academic Registration but leaves detailed contract semantics to the applicable L2/L4 architecture.

**OI-C031-03 — Detention-specific approval authority is not defined as a universal named business role.**  
No role is invented.

### Assumptions

1. Academic Progression owns detention and repeat determination.
2. Academic Registration owns any subsequent registration action.
3. Examination owns official examination outcomes.
4. An appeal does not directly erase detention; C04 governs the transition back to Evaluation Running.

### Deferred Questions

- Exact repeat semester/year policy criteria.
- Exact downstream registration hand-off.
- Institution-specific appeal authority and evidence requirements.


## Production Authoring / Verification Checklist

- [x] Final L3 ID and canonical title preserved from Stage 3 / Stage 4.
- [x] Primary Owner, Participating Domains, Scenario Type and Priority preserved.
- [x] Exact Phase 5 filename follows the locked naming rule.
- [x] Applicable locked L2-C04 source governs workflow, ownership, rules, states and events.
- [x] Academic Progression remains canonical owner of the progression outcome and related lifecycle.
- [x] Student Lifecycle and Examination remain participating domains without ownership transfer.
- [x] No new actor, authority, rule, state, event publisher or dependency is invented.
- [x] Business failure/recovery is expressed only at business level.
- [x] No APIs, database design, services, classes, Business Data Structures, UI, infrastructure or technical access control/IAM are introduced.
- [x] Cross-domain events remain distinct from synchronous business hand-offs.
- [x] Material upstream ambiguity is explicitly recorded in Section 17.
- [x] All 17 locked template sections are present.
- [x] Status is `Draft` because Phase 5 authoring cannot self-approve or self-lock a scenario.
