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
**Version:** 1.1  
**Last Updated:** 2026-08-22  
**Exact Filename:** `L3-C-027_Student_Academic_Progression_Decision.md`  
**Applicable L2:** `L2-C04_Student_Academic_Progression_Workflows.md` as amended by `L2-C04_Student_Academic_Progression_Workflows_Amendment_2026-08-22.md` under `ADR-STU-PROG-EVT-001`

## 1. Scenario Metadata

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

Frozen Stage 3/4 identity is preserved. The 2026-08-22 L2-C04 amendment changes only canonical event vocabulary; it does not reclassify this scenario.

## 2. Scenario Purpose

Govern the institution's authoritative decision on whether a Student progresses academically after an evaluation cycle. Academic Progression owns the progression decision and Academic Standing. Examination provides officially published academic outcomes without acquiring progression ownership.

## 3. Business Outcome

One authoritative, policy-governed progression decision exists for the Student and evaluation cycle. Applicable outcomes are:

- Progressed;
- Conditional Progression;
- Academic Probation;
- Detained; or
- Qualification Eligible where programme-completion requirements are satisfied.

The outcome is reproducible from approved policy and authoritative evidence and remains auditable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario starts when an authoritative progression-evaluation trigger occurs, including officially published examination results, an evaluation window, permitted manual institutional evaluation, approved re-evaluation, governed appeal outcome, or supplementary result publication.

### 4.2 End Boundary
The scenario ends when the authoritative progression decision/state is established and applicable canonical Academic Progression event(s) are published. Qualification Eligible hands control to Qualification Completion & Student Exit.

### 4.3 In Scope
- Validate authoritative academic inputs.
- Evaluate earned credits and progression eligibility.
- Apply one approved progression policy.
- Determine Academic Standing and progression outcome.
- Establish conditional promotion, probation or detention where applicable.
- Publish canonical progression outcomes.
- Preserve audit evidence.

### 4.4 Out of Scope
- Examination evaluation/result creation.
- Academic Registration creation/modification.
- Curriculum ownership.
- Finance clearance ownership.
- Qualification conferment.
- Technical policy engines, APIs, databases, queues, UI or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership Boundary |
|---|---|---|
| Academic Progression | Evaluates academic evidence and establishes progression and Academic Standing. | Canonical owner. |
| Student Lifecycle (C) | Provides/consumes Student context and consequences where required. | Does not independently promote, place on probation, or detain. |
| Examination (I) | Supplies officially published examination outcomes. | Retains examination/result ownership. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Academic Progression | Performs progression evaluation and records the decision. | Sole progression-decision owner. |
| Authorized academic/progression authority | Provides institutional approval where policy requires it. | Cannot redefine approved progression rules locally. |
| Examination | Publishes authoritative examination outcomes. | Does not make progression decisions. |
| Student Lifecycle | Consumes progression consequences. | No progression-decision authority. |

## 7. Preconditions and Entry Criteria

- Student is Active where required by the governing workflow.
- Academic Registration is complete.
- Applicable examination outcomes are officially published.
- Curriculum and academic requirements are available.
- One approved progression policy is active.
- Academic calendar permits evaluation.

## 8. Trigger

An authoritative progression-evaluation business trigger occurs. Only authoritative business facts may initiate the evaluation; implementation mechanisms are outside L3.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Confirm the Student and completed registration context. | Academic Progression consuming authoritative context | Evaluation context established. |
| 2 | Collect officially published academic outcomes. | Examination supplies; Academic Progression consumes | Authoritative evidence established. |
| 3 | Identify curriculum requirements and the active approved progression policy. | Academic Progression using authoritative sources | Governing criteria established. |
| 4 | Evaluate earned credits and progression eligibility. | Academic Progression | Eligibility established. |
| 5 | Determine Academic Standing and progression outcome. | Academic Progression | Authoritative decision established. |
| 6 | Record the decision with policy/evidence traceability. | Academic Progression | Auditable decision preserved. |
| 7 | Publish the applicable canonical progression event(s). | Academic Progression | Downstream consumers can respond. |

## 10. Alternate Business Flows

| Outcome | Business Path |
|---|---|
| Progressed | Establish Progressed and publish `StudentProgressed`. |
| Conditional Progression | Establish Conditional Progression and publish `ConditionalPromotionGranted`. |
| Academic Probation | Establish Academic Probation and publish `AcademicProbationAssigned`. |
| Detained | Establish Detained and publish `AcademicDetentionApplied`. |
| Qualification Eligible | Establish qualification eligibility and publish the applicable qualification-eligibility outcome before handing control to L2-C05. |
| Re-evaluation / supplementary outcome | Perform a fresh evaluation where policy permits; preserve historical decisions. |

## 11. Failure, Interruption and Business Recovery

| Condition | Resolution |
|---|---|
| Official examination outcome unavailable | Progression remains unresolved until Examination publishes authoritative evidence. |
| Mandatory academic data incomplete | Resolve the missing authoritative information before completion. |
| No active approved progression policy | Do not establish an authoritative decision. |
| Student/registration precondition not satisfied | Resolve through the canonical owning Student Lifecycle/Registration workflow. |
| Re-evaluation changes authoritative evidence | Create a fresh progression evaluation where policy permits; do not mutate history. |

No technical retry or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

- Only officially published examination results participate in authoritative progression evaluation.
- Exactly one approved institutional progression policy applies to an evaluation.
- Mandatory academic evidence must be complete.
- Historical progression decisions are immutable.
- Only successfully earned and institutionally approved credits contribute as defined by upstream rules.
- Every active Student has the applicable authoritative Academic Standing.
- Manual overrides require authorized institutional approval and audit evidence.

These rules remain upstream-owned; L3 applies but does not re-own them.

## 13. State and Lifecycle Impact

The governing C04 lifecycle remains:

`Created → Awaiting Evaluation → Evaluation Running → {Progressed | Conditional Progression | Academic Probation | Detained}`

Further governed transitions include Conditional Progression to Progressed or Academic Probation, Academic Probation to Progressed or Detained, Detained to Evaluation Running after approved appeal where applicable, Detained to Progressed after governed requirements completion, and Progressed to Qualification Eligible when programme-completion requirements are satisfied.

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

| Business Outcome / Input | Canonical Event | Canonical Publisher |
|---|---|---|
| Evaluation completed | `ProgressionEvaluated` | Academic Progression |
| Student progressed | `StudentProgressed` | Academic Progression |
| Conditional promotion approved | `ConditionalPromotionGranted` | Academic Progression |
| Academic probation assigned | `AcademicProbationAssigned` | Academic Progression |
| Academic detention finalized | `AcademicDetentionApplied` | Academic Progression |
| Qualification eligibility updated | `QualificationEligibilityUpdated` | Academic Progression |
| Official examination result | Applicable Examination-owned published result event | Examination |

`ADR-STU-PROG-EVT-001` supersedes `StudentConditionallyProgressed`, `StudentPlacedOnProbation`, `StudentOnProbation`, and `StudentDetained` as canonical publication names. They are not parallel business events.

## 15. Business Authorization, Approval and Audit Obligations

Progression decisions must remain policy-driven, reproducible and auditable. Manual intervention follows upstream-authorized institutional approval. Historical decisions and the policy/evidence basis remain preserved. No technical IAM/RBAC design is introduced.

## 16. Completion Criteria and Postconditions

The scenario is complete when mandatory authoritative data is available, one approved progression policy has been applied, Academic Standing and the progression decision are established, the applicable canonical event(s) are published by Academic Progression, and audit evidence exists.

Downstream domains consume the decision without redefining it.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C027-01 — Policy Management ownership:** Locked L2-C04 identifies Policy Management ownership/publisher as unresolved. C027 consumes an already-approved policy and does not assign a new owner.

**OI-C027-02 — Institution-specific approval routing:** Exact approval routing remains configuration-driven where upstream architecture does not establish a universal named role.

**Resolved upstream issue:** Competing conditional-promotion, probation and detention event names are resolved by `ADR-STU-PROG-EVT-001` and the locked L2-C04 amendment.

**Assumptions:** Examination owns official examination results; Academic Progression owns all progression decisions and canonical progression-outcome publications; historical decisions remain immutable.

**Deferred:** Policy owner/contract, institution-specific thresholds, detailed policy-version lifecycle and approval routing.

## Final Authoring / Architecture-Safety Validation

- [x] Frozen L3 identity preserved.
- [x] Explicit mandatory Section 1 restored.
- [x] L2-C04 amendment consumed.
- [x] One canonical event name used per progression outcome.
- [x] Academic Progression publisher ownership preserved.
- [x] No implementation-level design introduced.
- [x] All 17 mandatory sections present.
- [x] Status remains Draft pending independent architecture approval.
