# Academic Probation Management and Resolution

**Scenario ID:** L3-C-030  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; applicable Student Lifecycle context; official Examination outcomes where probation review requires them  
**Referenced By:** L3-C-027, L3-C-028

## 1. Scenario Metadata

Frozen identity: C030 — Academic Probation Management and Resolution; Primary Owner Academic Progression; Participating Domains C and I; Critical; P1; KEEP.

---

## 2. Scenario Purpose

This scenario governs the institutional lifecycle of a Student placed on **Academic Probation**, including establishment of probation conditions, review of academic progress, and authoritative resolution of the probation outcome.

L2-C04 defines Academic Probation as a policy-governed monitoring lifecycle and permits resolution through return to Progressed or escalation to Detained.

---

## 3. Business Outcome

The Student's Academic Probation has been authoritatively reviewed and resolved as one of the governed outcomes:

- **Progressed** after a successful review; or
- **Detained** after an unsuccessful review.

The probation record and its resolution remain part of the Student's auditable academic progression history.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when Academic Progression authoritatively assigns the Student to **Academic Probation**.

### 4.2 End Boundary

The scenario ends when the probation review establishes either successful progression or detention.

### 4.3 In Scope

- Establishing probation conditions.
- Academic monitoring at the business level.
- Periodic review.
- Evaluation of probation conditions.
- Return to Progressed.
- Escalation to Detained.
- Probation history and evidence.

### 4.4 Out of Scope

- The progression evaluation that initially assigned probation.
- Creation/correction of examination results.
- Definition of probation policy.
- Detention workflow after escalation.
- Technical monitoring, scheduling, notifications or retry mechanisms.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-C-027 Student Academic Progression Decision | May establish Academic Probation. |
| May be preceded by | L3-C-029 Conditional Promotion Condition Fulfilment | Failed Conditional Progression conditions may lead to Probation. |
| Followed by | L3-C-031 Academic Detention and Repeat Determination | Receives the unsuccessful-review outcome. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Academic Progression | Owns Academic Probation, its conditions, review and resolution. |
| Student Lifecycle (C) | Provides/consumes Student lifecycle context as required; does not own probation. |
| Examination (I) | Provides official academic outcomes where they are relevant to probation review. |

Academic Progression remains the canonical owner.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Academic Progression | Establishes probation conditions, conducts the governed review, and records resolution. | Canonical owner. |
| Examination | Provides official academic outcomes where applicable. | Source of academic outcome. |
| Student Lifecycle | Consumes resulting Student lifecycle consequence where applicable. | Participant; not progression decision owner. |

The authoritative source does not establish a universal probation-specific role name, so none is invented.

---

## 7. Preconditions and Entry Criteria

- Student is in **Academic Probation**.
- Probation conditions have been established.
- The applicable progression policy governs the review.
- Relevant official academic evidence is available where required.
- The Student's academic history remains available for review.

---

## 8. Trigger

**Probation review becomes due under the applicable institutional policy.**

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the active probation conditions and review basis. | Academic Progression | Review scope is established. |
| 2 | Evaluate current authoritative academic evidence against the probation conditions. | Academic Progression / Examination | Evidence is assessed. |
| 3 | Determine whether the Student has satisfied the probation requirements. | Academic Progression | Review decision is established. |
| 4 | Where the review passes, transition the Student to **Progressed**. | Academic Progression | Probation is successfully resolved. |
| 5 | Record the review outcome and preserve probation history. | Academic Progression | Institutional evidence is complete. |

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | Review fails | Transition the Student to **Detained** under the C04 state transition. | Detention established. | Academic Progression |
| A2 | Required academic evidence is not yet authoritative | Defer the review outcome until valid evidence exists. | Probation remains unresolved. | Academic Progression |

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required official examination evidence is unavailable | Review cannot establish a final academic outcome. | Continue the governed probation lifecycle until authoritative evidence is available, where policy permits. | Academic Progression / Examination |
| Probation requirements are not satisfied | Student cannot return to Progressed. | Transition to Detained under the canonical C04 lifecycle. | Academic Progression |
| Authoritative academic evidence changes through an approved process | Previous review basis may no longer be sufficient. | Conduct the governed fresh evaluation where policy permits; preserve historical records. | Academic Progression |

No technical recovery behaviour is specified.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Probation is policy governed. | Academic Progression | Conditions and review must follow the applicable institutional policy. | L2-C04 APW-05 |
| Probation may include academic counselling, credit targets and performance monitoring. | Academic Progression | Review evaluates the policy-defined probation conditions. | L2-C04 APW-05 |
| A failed probation review transitions to Detained. | Academic Progression | Unsuccessful resolution leaves the probation lifecycle through the canonical Detained path. | L2-C04 §4.4 |
| Historical progression decisions remain immutable. | Academic Progression | Review does not erase historical decisions. | L2-C04 progression rules |

No new probation rule is introduced.

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Academic Probation | Academic Probation | Review passes. | Progressed. | Academic Progression | L2-C04 §4.4 |
| Academic Probation / Academic Detention | Academic Probation | Review fails. | Detained. | Academic Progression | L2-C04 §4.4 |

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Applicable official examination result event | Examination | Academic Progression | Supplies official academic evidence where required. | L2-C04 APW-01 trigger semantics |
| `StudentProgressed` | Academic Progression | Downstream consumers | Communicates successful resolution to Progressed. | L2-C04 APW-01 |
| `StudentDetained` | Academic Progression | Registration / relevant downstream consumers | Communicates unsuccessful probation resolution. | L2-C04 APW-01 |

No dedicated `ProbationResolved` event is invented.

### 14.2 Synchronous / Business Hand-off

Student Lifecycle and Examination contribute authoritative business information; they do not modify Academic Probation ownership.

---

## 15. Business Authorization, Approval and Audit Obligations

Probation decisions remain governed by institutional progression policy.

The review outcome must be auditable, including the business basis for the decision and the resulting lifecycle transition.

Any manual intervention must use the upstream authorized institutional approval model. This scenario does not invent a role or technical access-control structure.

---

## 16. Completion Criteria and Postconditions

### Successful Resolution

- probation conditions have been reviewed;
- review passes under the applicable policy;
- Student transitions to Progressed;
- probation history is preserved and auditable.

### Unsuccessful Resolution

- review failure is established;
- Student transitions to Detained;
- detention processing becomes the responsibility of C031.

The scenario stops at the resulting lifecycle state.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

**OI-C030-01 — Exact probation review cadence is not fully specified in the available C04 source.**

**OI-C030-02 — Exact probation authority/actor is not defined as a universal named role.** No role is invented at L3.

**OI-C030-03 — No dedicated probation-resolution event is established in the available C04 event catalogue.** Existing progression outcome events are used without renaming or inventing events.

### Assumptions

1. Academic Progression owns the probation lifecycle.
2. A Student already in Academic Probation is the entry condition.
3. Examination remains authoritative for official academic outcomes.
4. Detention escalation is handled by C031.

### Deferred Questions

- Institution-specific probation review schedule.
- Exact condition catalogue.
- Exact approval authority.
- Whether a dedicated probation-resolution event should be introduced through architecture change control.


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
