# Conditional Promotion Condition Fulfilment

**Scenario ID:** L3-C-029  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; upstream Student Lifecycle and Academic Registration context consumed by L2-C04; official Examination outcomes where applicable  
**Referenced By:** L3-C-027, L3-C-028

## 1. Scenario Metadata

This scenario is the canonical L3 outcome for fulfilling conditions attached to an already-authorized **Conditional Progression** decision.

Frozen identity: C029 — Conditional Promotion Condition Fulfilment; Primary Owner Academic Progression; Participating Domains C and I; Critical; P1; KEEP.

---

## 2. Scenario Purpose

The scenario governs the institutional resolution of conditions attached to Conditional Progression.

Its purpose is to determine whether the Student satisfies the policy-defined conditions for conversion from **Conditional Progression** to **Progressed**, or whether the Student must move to the governed **Academic Probation** path when those conditions are not satisfied.

L2-C04 defines Conditional Promotion as a policy-governed progression path and identifies condition monitoring followed by a satisfied/not-satisfied decision.

---

## 3. Business Outcome

The Student's Conditional Progression conditions have been authoritatively evaluated and the Student is either:

- moved to **Progressed** because the applicable conditions are satisfied; or
- moved to **Academic Probation** because the conditions are not satisfied.

The outcome remains owned by Academic Progression.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins after a valid Conditional Progression outcome exists and the policy-defined conditions become due for evaluation.

### 4.2 End Boundary

The scenario ends when the conditions are authoritatively determined satisfied or failed and the corresponding C04 lifecycle transition is established.

### 4.3 In Scope

- Evaluation of Conditional Progression conditions.
- Verification of required academic evidence.
- Determination of condition satisfaction.
- Transition to Progressed where conditions are satisfied.
- Transition to Academic Probation where conditions fail.
- Applicable academic audit/evidence.

### 4.4 Out of Scope

- The original progression decision that placed the Student into Conditional Progression.
- Creation or correction of examination results.
- Definition of new institutional progression rules.
- Independent Student Lifecycle status changes.
- Technical implementation of monitoring or reminders.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-C-027 Student Academic Progression Decision | Establishes the Conditional Progression outcome. |
| Related | L3-C-030 Academic Probation Management and Resolution | Receives the governed failure outcome. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Academic Progression | Owns Conditional Progression conditions, their evaluation, and the resulting progression lifecycle transition. |
| Student Lifecycle (C) | Provides/consumes Student lifecycle context where required by the authoritative C04 workflow. It does not own the progression decision. |
| Examination (I) | Provides official academic outcomes where they are evidence for condition satisfaction. It does not determine progression. |

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Academic Progression | Evaluates whether policy-defined conditions are satisfied and establishes the resulting progression state. | Canonical decision owner. |
| Examination | Provides official examination outcomes when applicable to the condition. | Source of official academic outcome; not progression authority. |
| Student Lifecycle | Consumes the resulting progression consequence where required. | Downstream participant; does not decide progression. |

No additional scenario-specific approving role is invented because the available C04 source does not define one universally for this scenario.

---

## 7. Preconditions and Entry Criteria

- The Student is in **Conditional Progression**.
- The Conditional Progression conditions are already established by the authoritative progression workflow.
- The evidence required to assess those conditions is available or its absence can be determined.
- Where examination outcomes are relevant, only officially published outcomes are considered.
- The applicable institutional progression policy remains authoritative.

---

## 8. Trigger

**Condition review becomes due under the applicable Conditional Progression policy.**

The trigger is business-level and does not specify a technical scheduler or implementation mechanism.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Student's outstanding Conditional Progression conditions. | Academic Progression | Conditions to be evaluated are established. |
| 2 | Gather authoritative academic evidence relevant to those conditions. | Academic Progression / Examination | Evidence is available for evaluation. |
| 3 | Evaluate the conditions against the applicable approved progression policy. | Academic Progression | Condition result is determined. |
| 4 | Where all required conditions are satisfied, establish the **Progressed** outcome. | Academic Progression | Student returns to normal progression. |
| 5 | Record the resulting progression outcome and audit evidence. | Academic Progression | Conditional Promotion fulfilment is institutionally recorded. |

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | Conditions are not satisfied | Establish the governed Academic Probation path. | Student enters Academic Probation. | Academic Progression |
| A2 | A condition depends on a later official Examination outcome | Evaluate only after the authoritative outcome is available. | Condition remains unresolved until valid evidence exists. | Academic Progression / Examination |

The C04 workflow explicitly shows the satisfied path to Progressed and the unsatisfied path to Probation.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required official academic evidence is unavailable | Condition cannot yet be authoritatively satisfied. | Await the authoritative business outcome before determining fulfilment. | Academic Progression / Examination |
| Condition remains unsatisfied | Conditional Progression cannot convert to Progressed. | Follow the C04 Academic Probation transition. | Academic Progression |
| Evidence is disputed or corrected through an authoritative academic process | Existing determination may require a governed fresh evaluation. | Re-evaluate using the authoritative outcome; do not silently rewrite historical decisions. | Academic Progression |

No technical retry or rollback behaviour is defined.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Conditional Promotion occurs only where explicitly permitted by institutional policy. | Academic Progression | Conditions may only be evaluated within an approved Conditional Progression path. | L2-C04 — Conditional Promotion rules |
| Progression policy governs permitted conditions. | Academic Progression | Satisfaction is evaluated against the applicable policy. | L2-C04 — APW-04 |
| Historical progression decisions remain immutable. | Academic Progression | A later evaluation does not erase the historical decision. | L2-C04 — progression rules |
| Only officially published examination outcomes may participate in progression evaluation. | Examination / Academic Progression boundary | Examination evidence is valid only after official publication. | L2-C04 — BR-PROG-001 |

No new local business rule is introduced.

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Academic Progression | Conditional Progression | Conditions are evaluated. | Progressed when satisfied. | Academic Progression | L2-C04 §4.4 |
| Academic Progression / Academic Probation | Conditional Progression | Conditions fail. | Academic Probation. | Academic Progression | L2-C04 §4.4; APW-04/APW-05 |

The scenario does not create a new lifecycle state.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Applicable official examination result event | Examination | Academic Progression | Provides academic evidence where relevant. | L2-C04 APW-01 trigger semantics |
| `StudentProgressed` | Academic Progression | Downstream Student Lifecycle / relevant consumers | Communicates the resulting Progressed business outcome. | L2-C04 APW-01 |

No new `ConditionalPromotionConditionsFulfilled` event is invented because the available C04 event catalogue does not establish one.

### 14.2 Synchronous / Business Hand-off

Where an authoritative business fact must be obtained from Examination or Student Lifecycle, the hand-off remains within those domains' ownership boundaries. L3 does not define technical contract mechanics.

---

## 15. Business Authorization, Approval and Audit Obligations

The condition decision must remain within the approved progression policy and Academic Progression authority.

The resulting lifecycle transition is auditable. Historical progression decisions remain immutable.

Where a manual intervention or exception is permitted by institutional policy, it requires the authorized institutional approval defined upstream; this scenario does not invent an approver role.

---

## 16. Completion Criteria and Postconditions

### Complete — Conditions Satisfied

- all applicable conditions have been evaluated;
- satisfaction is established under the approved policy;
- Student transitions from Conditional Progression to Progressed;
- the progression history is auditable.

### Complete — Conditions Failed

- failure is authoritatively established;
- Student enters Academic Probation;
- the transition is auditable.

The Main Business Flow stops at this resulting lifecycle outcome. Probation management belongs to C030.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

**OI-C029-01 — Exact condition catalogue and review cadence are not fully enumerated in the available C04 excerpt.**  
C04 identifies backlog subjects, credit deficiency, improvement examination and mandatory credit recovery as typical conditions, but L3 does not invent a complete institutional condition matrix.

**OI-C029-02 — Exact actor/approval authority for condition fulfilment is not universally defined.**  
No new role is introduced.

**OI-C029-03 — No dedicated conditional-fulfilment domain event is established in the available C04 event catalogue.**  
C029 therefore uses the established progression outcome event(s) only.

### Assumptions

1. Conditional Progression has already been authoritatively established by C027.
2. Academic Progression remains the sole owner of the condition decision and lifecycle transition.
3. Examination remains the source of official examination outcomes.

### Deferred Questions

- Exact condition catalogue per institution.
- Exact review timing and authority.
- Whether a dedicated canonical event is required.


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
