# Progression Decision Revision After Re-evaluation or Appeal

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-038  
**Canonical Title:** Progression Decision Revision After Re-evaluation or Appeal  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Draft  
**Version:** 1.1  
**Author:** Varad  
**Last Updated:** 2026-08-22  
**Applicable L2:** `L2-C04_Student_Academic_Progression_Workflows.md` as amended by `L2-C04_Student_Academic_Progression_Workflows_Amendment_2026-08-22.md` under `ADR-STU-PROG-EVT-001`

## 1. Scenario Metadata

This document preserves the frozen Stage 3 identity and semantics for **L3-C-038 — Progression Decision Revision After Re-evaluation or Appeal**. No ownership, participating-domain, type or priority reclassification is introduced.

## 2. Scenario Purpose

Re-evaluate an existing Academic Progression decision when authoritative re-evaluation, supplementary, or other institutionally governed review evidence changes the academic position and approved policy permits a fresh progression evaluation.

This scenario does not create or own an Examination appeal workflow. It consumes authoritative changed evidence and preserves Academic Progression as the sole owner of the resulting progression decision.

## 3. Business Outcome

A fresh authoritative progression evaluation is completed where permitted. The prior decision remains immutable, a subsequent progression decision is established, and the applicable canonical Academic Progression outcome is published.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins when Academic Progression receives authoritative changed academic evidence such as `RevaluationResultPublished` or `SupplementaryResultPublished`, or another governed review outcome established by its canonical upstream process.

### 4.2 End Boundary
The scenario ends when fresh evaluation establishes the authoritative subsequent progression outcome while preserving historical decisions.

### 4.3 In Scope
- Consume authoritative changed academic evidence.
- Confirm policy permits fresh evaluation.
- Re-evaluate progression using authoritative data.
- Establish and publish the applicable canonical progression outcome.
- Preserve historical decisions.

### 4.4 Out of Scope
- Changing Examination-owned results.
- Defining/owning an Examination appeal workflow.
- Inventing appeal authority/state/event.
- Deleting or rewriting historical decisions.
- Resolving Policy Management ownership locally.
- Technical replay, APIs, databases, queues or infrastructure.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Predecessor | C027 — Student Academic Progression Decision |
| Related | C029 — Conditional Promotion Condition Fulfilment |
| Related | C030 — Academic Probation Management and Resolution |
| Related | C031 — Academic Detention and Repeat Determination |
| Related | C036 — Qualification Eligibility Determination |

## 5. Ownership and Participating Domains

| Domain | Canonical Responsibility |
|---|---|
| Academic Progression (C) | Owns fresh progression evaluation, decision, Academic Standing impact and progression events. |
| Examination (I) | Owns authoritative Examination results and changed-result publications consumed by Academic Progression. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Examination responsibility | Establishes/publishes authoritative changed Examination evidence. | Examination-result authority only. |
| Academic Progression | Determines whether fresh evaluation is permitted and establishes the new progression decision. | Canonical progression authority. |
| Authorized institutional review responsibility | May supply a governed review/appeal outcome when established upstream. | No specific role is invented here. |

## 7. Preconditions and Entry Criteria

- Prior progression decision exists.
- Authoritative changed academic evidence exists.
- Approved policy permits fresh progression evaluation.
- Required evidence is complete.
- Any appeal/review outcome used as evidence has already been established by its owning process.

## 8. Trigger

Canonical changed-result triggers include `RevaluationResultPublished` and `SupplementaryResultPublished` from Examination. A governed appeal/review outcome may also trigger only when an authoritative upstream process establishes it and policy permits fresh evaluation.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Receive authoritative changed academic evidence. | Academic Progression consumes; source retains ownership. |
| 2 | Verify evidence authority and policy permission. | Academic Progression |
| 3 | Re-evaluate credits, standing and progression criteria. | Academic Progression |
| 4 | Apply approved progression policy. | Academic Progression |
| 5 | Establish the subsequent progression decision. | Academic Progression |
| 6 | Preserve the historical decision unchanged. | Academic Progression |
| 7 | Publish the applicable canonical progression outcome. | Academic Progression |

## 10. Alternate Business Flows

| Condition | Path | Result |
|---|---|---|
| Changed evidence cannot trigger re-evaluation | Do not revise progression. | Existing decision remains authoritative. |
| Changed evidence alters eligibility | Perform fresh evaluation. | New decision established. |
| Fresh evaluation reaches same conclusion | Record fresh evaluation without mutating history. | Decision conclusion unchanged; traceability preserved. |
| Governed review outcome changes academic position | Consume only after upstream authority establishes it. | Fresh evaluation proceeds where permitted. |

## 11. Failure, Interruption and Business Recovery

| Situation | Resolution |
|---|---|
| Evidence incomplete/not authoritative | Do not establish a revised decision. |
| Policy does not permit fresh evaluation | Preserve existing decision. |
| Appeal/review authority unclear | Do not invent an L3 authority/event; retain as upstream/open dependency. |

The previous event-name conflict is no longer a failure condition because it is resolved by the approved L2-C04 amendment.

## 12. Business Rules and Decision Constraints

- `BR-PROG-001`: only officially published examination results participate in progression evaluation.
- `BR-PROG-002`: each evaluation references one approved progression policy.
- `BR-PROG-006`: re-evaluation/supplementary outcomes trigger fresh evaluation where policy permits.
- `BR-PROG-007`: historical progression decisions remain immutable; approved revisions create subsequent records.

L3 applies but does not re-own these rules.

## 13. State and Lifecycle Impact

| Concept | Impact | Owner |
|---|---|---|
| Prior Progression Decision | Remains immutable historical evidence. | Academic Progression |
| Progression Evaluation | Fresh evaluation performed where permitted. | Academic Progression |
| Academic Standing | May change through the governed subsequent progression decision. | Academic Progression |

No new state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Changed Academic Evidence

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `RevaluationResultPublished` | Examination | Authoritative changed result input. |
| `SupplementaryResultPublished` | Examination | Authoritative supplementary result input. |

### 14.2 Canonical Academic Progression Outcomes

| Outcome | Canonical Event | Canonical Publisher |
|---|---|---|
| Evaluation completed | `ProgressionEvaluated` | Academic Progression |
| Progressed | `StudentProgressed` | Academic Progression |
| Conditional promotion | `ConditionalPromotionGranted` | Academic Progression |
| Probation | `AcademicProbationAssigned` | Academic Progression |
| Detention | `AcademicDetentionApplied` | Academic Progression |
| Standing update | `AcademicStandingUpdated` | Academic Progression |

`ADR-STU-PROG-EVT-001` resolves the former competing names. `StudentConditionallyProgressed`, `StudentPlacedOnProbation`, `StudentOnProbation`, and `StudentDetained` are superseded aliases, not parallel canonical events.

## 15. Business Authorization, Approval and Audit Obligations

Examination retains authority over changed Examination results; Academic Progression retains authority over progression decisions. Each fresh evaluation remains traceable to authoritative evidence and approved policy. Historical decisions remain immutable. No appeal authority or Policy Management owner is invented.

## 16. Completion Criteria and Postconditions

- Authoritative changed evidence received.
- Policy permission verified.
- Fresh progression evaluation completed where permitted.
- Historical decision preserved.
- Subsequent progression decision established where required.
- Applicable canonical event(s) published by the correct owner.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C038-01 — Appeal authority:** L2-C04 does not itself establish a dedicated Examination appeal-decision workflow, canonical appeal authority, appeal state or appeal event for this scenario. L3 therefore continues to consume only an upstream-authorized review outcome where one exists.

**OI-C038-02 — Policy Management owner:** L2-C04 identifies `AcademicPolicyUpdated` with Policy Management as an unresolved publisher/owner. This scenario consumes an approved policy without assigning that ownership.

**Resolved:** The former L2-C04 event-name inconsistency is resolved by `ADR-STU-PROG-EVT-001` and the locked amendment effective 2026-08-22.

**Assumption:** Examination remains authoritative for changed examination results and Academic Progression remains sole owner of the resulting progression decision.

## Production Verification Record

- Frozen identity preserved.
- Approved L2-C04 event amendment consumed.
- Changed-result events remain Examination-owned inputs.
- Canonical progression event vocabulary standardized.
- Appeal and Policy Management gaps not invented locally.
- No implementation mechanics introduced.
- All 17 mandatory sections present.
- Status remains Draft pending normal architecture review.
