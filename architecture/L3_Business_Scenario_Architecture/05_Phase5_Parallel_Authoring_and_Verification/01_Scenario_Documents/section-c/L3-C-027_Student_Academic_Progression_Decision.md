# Student Academic Progression Decision

**Scenario ID:** L3-C-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.3  
**Owner:** Varad  
**Last Updated:** 2026-08-23  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; `L2-C04_Student_Academic_Progression_Workflows_Amendment_2026-08-22.md` under `ADR-STU-PROG-EVT-001`  
**Referenced By:** None yet

## 1. Scenario Metadata

Frozen Stage 3/4 identity is preserved: `L3-C-027`, **Student Academic Progression Decision**, Primary Owner **Academic Progression**, Participating Domains **C, I**, Scenario Type **Critical**, Priority **P0**.

The locked 2026-08-22 amendment changes canonical event vocabulary only; it does not reclassify this scenario.

## 2. Scenario Purpose

Govern the institution's authoritative academic progression decision after an evaluation cycle using officially published academic evidence and an approved progression policy.

Academic Progression owns the progression decision and Academic Standing. Examination owns and publishes examination outcomes used as authoritative inputs.

## 3. Business Outcome

One authoritative, policy-governed progression decision is established for the Student and evaluation cycle. Applicable outcomes are:

- `Progressed`;
- `Conditional Progression`;
- `Academic Probation`;
- `Detained`; or
- `Qualification Eligible` where programme-completion requirements are satisfied.

The decision remains reproducible and auditable from the governing policy and authoritative academic evidence.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when an authoritative progression-evaluation trigger occurs, including `ExaminationResultPublished`, a permitted evaluation window, governed manual evaluation, approved re-evaluation, appeal outcome, or supplementary result publication.

### 4.2 End Boundary

The scenario ends when the authoritative progression decision and applicable owned lifecycle state are established, required approval has been satisfied where applicable, and the canonical progression event(s) are published.

Qualification eligibility hands control to Qualification Completion & Student Exit.

### 4.3 In Scope

- Validate authoritative Student, registration, curriculum and examination context.
- Apply one approved progression policy.
- Evaluate earned credits and progression eligibility.
- Determine Academic Standing and progression outcome.
- Perform upstream-supported review/approval where required.
- Publish exact canonical progression events.
- Preserve decision and evidence traceability.

### 4.4 Out of Scope

- Examination evaluation or result creation.
- Academic Registration creation/modification.
- Curriculum ownership.
- Finance clearance ownership.
- Qualification conferment.
- Institution-specific technical routing or technical RBAC/IAM design.
- APIs, databases, queues, UI, services, deployment or infrastructure.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Academic Progression / Section C | Owns progression evaluation, progression decision, Academic Standing outcome and canonical progression publications. |
| Examination / Section I | Owns and publishes official examination outcomes consumed as progression evidence. |
| Student Lifecycle / Section C | Provides authoritative Student lifecycle context and consumes applicable progression consequences without owning the progression decision. |

Participation does not transfer ownership of examination results or progression decisions.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Programme Coordinator | Reviews progression cases and may execute/approve progression actions allowed by the L2-C04 permission model. | Review / permitted progression decision participation. |
| Department Administrator | Initiates authorized progression workflows and may execute/approve progression actions permitted upstream. | Initiates / performs / approves within configured policy. |
| Controller of Examination | Publishes authoritative examination outcomes used as progression inputs. | Supplies official examination evidence; no progression-decision ownership. |
| Dean | Approves exceptional progression decisions where the governing institutional process requires it. | Exception approval only where applicable. |
| Registrar | Authorizes institutional progression decisions and may perform progression capabilities permitted by L2-C04. | Institutional progression authority. |
| Academic Council | Approves policy exceptions and participates in authorized manual-override governance where applicable. | Policy-exception / override authority. |

Actual permissions and routing remain configurable institutional policy as stated by L2-C04. This scenario uses only roles already established upstream.

## 7. Preconditions and Entry Criteria

- The Student has the lifecycle status required by the governing progression workflow.
- Academic Registration for the relevant period is complete where required.
- Official examination outcomes required for the decision are published.
- Applicable curriculum requirements and academic context are available.
- One approved progression policy is active for the evaluation.
- Any required academic-calendar condition is satisfied.

## 8. Trigger

An authoritative progression-evaluation condition occurs, most commonly `ExaminationResultPublished`; governed re-evaluation and supplementary-result paths use the exact upstream examination events applicable to those cases.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the Student and completed registration context required for evaluation. | Academic Progression | Evaluation context is established. |
| 2 | Consume officially published examination evidence. | Controller of Examination / Examination supplies; Academic Progression consumes | Authoritative academic evidence is established. |
| 3 | Identify applicable curriculum requirements and the approved progression policy. | Academic Progression | Governing criteria are established. |
| 4 | Evaluate earned credits and progression eligibility. | Programme Coordinator / Department Administrator / Academic Progression as permitted upstream | Evaluation result is established. |
| 5 | Determine Academic Standing and the progression outcome under the approved policy. | Academic Progression | Authoritative progression outcome is established. |
| 6 | Obtain the applicable upstream-supported review or authorization where required by policy. | Programme Coordinator / Department Administrator / Dean / Registrar / Academic Council as applicable | Required institutional authority is satisfied. |
| 7 | Preserve the decision with policy and evidence traceability. | Academic Progression | Auditable progression history is established. |
| 8 | Publish the exact canonical progression event(s) for the actual outcome. | Academic Progression | Downstream consumers can respond to the completed business fact. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Conditional progression | Policy permits progression subject to conditions. | Establish `Conditional Progression` and complete the required upstream approval. | Publish `ConditionalPromotionGranted`. | Academic Progression |
| Academic probation | Governing policy requires probation. | Establish the Academic Probation lifecycle outcome under its canonical aggregate. | Publish `AcademicProbationAssigned`. | Academic Progression |
| Academic detention | Governing policy requires detention. | Establish the Detained lifecycle outcome under its canonical aggregate after required approval. | Publish `AcademicDetentionApplied`. | Academic Progression |
| Qualification eligibility | Programme-completion requirements are satisfied. | Establish `Qualification Eligible`. | Publish `QualificationEligibilityUpdated` and hand control to L2-C05. | Academic Progression |
| Re-evaluation / supplementary result | Authoritative evidence changes under a governed re-evaluation or supplementary path. | Perform a fresh progression evaluation; preserve historical decisions. | A new authoritative progression decision is established. | Academic Progression |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Official examination outcome unavailable | Authoritative progression cannot complete. | Wait for the applicable Examination-owned published outcome. | Examination / Academic Progression boundary |
| Mandatory academic data incomplete | Evaluation cannot complete. | Resolve the missing authoritative information through its owning domain. | Relevant owner / Academic Progression |
| No approved progression policy is active | No authoritative decision may be established. | Resolve the policy-governance dependency before completion. | Policy governance / Academic Progression |
| Required institutional approval is not satisfied | Progression outcome cannot be finalized where approval is mandatory. | Obtain approval through an upstream-supported authorized role. | Academic Progression |
| Student/registration precondition is not satisfied | Evaluation cannot proceed normally. | Resolve through the canonical Student Lifecycle / Academic Registration workflow. | Owning Section C domain |
| Re-evaluation changes authoritative evidence | Prior history must not be overwritten. | Create a fresh governed progression evaluation. | Academic Progression |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only officially published examination outcomes may drive authoritative progression. | Academic Progression consuming Examination facts | Draft/provisional/unpublished results cannot produce the final decision. | L2-C04 AP-02; §2.8 |
| Every progression decision references an approved institutional progression policy. | Academic Progression / unresolved Policy Management dependency | Decision cannot complete without a governing policy. | L2-C04 AP-01; §2.8 |
| Historical progression decisions are preserved and not overwritten. | Academic Progression | Re-evaluation produces new governed history rather than mutation. | L2-C04 §2.8 |
| Academic standing changes are auditable. | Academic Progression | Decision/evidence history must be preserved. | L2-C04 AP-06; §2.8 |
| Manual overrides require authorized institutional approval and complete audit history. | Academic Progression | Override cases require Registrar / Academic Council authority as defined upstream. | L2-C04 §9.12; §11.5 |
| Downstream domains do not independently promote, place on probation, or detain a Student. | Academic Progression | Progression ownership remains singular. | L2-C04 AP-03 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Academic Progression | `Created`, `Awaiting Evaluation` | Govern the Student into active evaluation and, where applicable, the final progressed/qualification-eligible position. | `Awaiting Evaluation`, `Progressed`, or later `Qualification Eligible` as governed | Academic Progression | L2-C04 §4.4, §4.6 |
| Progression Evaluation | `Awaiting Evaluation` | Execute the policy-driven progression evaluation. | `Evaluation Running` | Progression Evaluation | L2-C04 §4.4, §4.6 |
| Academic Progression | `Evaluation Running` | Establish conditional progression where policy permits. | `Conditional Progression` | Academic Progression | L2-C04 §4.4, §4.6 |
| Academic Probation | `Evaluation Running` | Establish probation outcome where policy requires it. | `Academic Probation` | Academic Probation | L2-C04 §4.4, §4.6 |
| Academic Detention | `Evaluation Running` | Establish detention outcome where policy requires it. | `Detained` | Academic Detention | L2-C04 §4.4, §4.6 |

No lifecycle ownership is transferred or collapsed by this L3 scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ExaminationResultPublished` | Examination | Academic Progression | Primary authoritative trigger/input for progression evaluation. | L2-C04 §7.3 / §23.3 |
| `RevaluationResultPublished` | Examination | Academic Progression | Recalculate progression after governed revaluation where this L2 event form applies. | L2-C04 §7.3 |
| `SupplementaryResultPublished` | Examination | Academic Progression | Recalculate progression after supplementary result publication. | L2-C04 §7.3 |
| `ProgressionEvaluated` | Academic Progression | Interested downstream domains | Evaluation completed. | L2-C04 event architecture |
| `StudentProgressed` | Academic Progression | Downstream consumers | Student progressed. | L2-C04 event architecture |
| `ConditionalPromotionGranted` | Academic Progression | Downstream consumers | Conditional promotion approved. | Locked L2-C04 amendment `ADR-STU-PROG-EVT-001` |
| `AcademicProbationAssigned` | Academic Progression | Downstream consumers | Academic probation assigned. | Locked L2-C04 amendment `ADR-STU-PROG-EVT-001` |
| `AcademicDetentionApplied` | Academic Progression | Downstream consumers | Academic detention finalized. | Locked L2-C04 amendment `ADR-STU-PROG-EVT-001` |
| `QualificationEligibilityUpdated` | Academic Progression | Qualification Completion & Student Exit | Qualification eligibility updated. | L2-C04 existing event semantics |

L2-C04 also contains `AssessmentRevaluationCompleted` in a separate consumed-event catalogue for re-evaluation. Because the source uses both `RevaluationResultPublished` and `AssessmentRevaluationCompleted`, this scenario does not declare them synonyms; see `OI-C027-03`.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Examination | Academic Progression | Official published examination outcome | Supply authoritative academic evidence without transferring Examination ownership. | L2-C04 domain inputs / event architecture |
| Academic Progression | Qualification Completion & Student Exit | Qualification-eligibility outcome | Hand off a completed eligibility decision for downstream completion processing. | L2-C04 / L2-C05 boundary |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Review progression cases | Programme Coordinator | Permitted progression-review participation. | Academic Progression |
| Initiate / execute authorized progression workflow | Department Administrator | Permitted by L2-C04 role/permission architecture and institutional policy. | Academic Progression |
| Authorize institutional progression decision | Registrar | Canonical institutional progression authorization role in L2-C04. | Academic Progression |
| Approve exceptional progression decision | Dean | Applies to governed exceptional decisions. | Academic Progression |
| Approve policy exception / authorized manual override | Academic Council | Applies where policy exception or override governance requires it. | Academic Progression |
| Publish examination inputs | Controller of Examination | Publishes examination-owned authoritative outcomes; cannot decide progression. | Examination |

### 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Manual override | Registrar / Academic Council | Only authorized institutional processes may override; the decision remains auditable and policy-governed. |
| Exceptional progression decision | Dean / Registrar as applicable | Exception handling must remain within roles and authority established by L2-C04 and institutional policy. |

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Every progression decision must be reproducible and auditable. | Preserved progression decision with governing policy version and authoritative academic evidence. | Academic Progression |
| Manual override / exception must be auditable. | Preserved authorized decision and progression history required by L2-C04. | Academic Progression |
| Historical decisions must not be overwritten. | Immutable progression history showing prior and subsequent governed evaluations. | Academic Progression |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Mandatory authoritative Student, registration, curriculum and examination information is available.
- One approved progression policy has been applied.
- Any required upstream-supported approval has been satisfied.
- Academic Standing and the authoritative progression outcome are established under their canonical lifecycle owners.
- The applicable exact canonical progression event(s) are published by Academic Progression.
- Required audit/evidence history exists.

### 16.2 Postconditions

- Downstream domains consume the progression outcome without redefining it.
- Qualification-eligible cases are handed to Qualification Completion & Student Exit.
- Historical progression decisions remain preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

**OI-C027-01 — Policy Management ownership/publisher:** Locked L2-C04 identifies Policy Management as unresolved/no owner for progression-policy publication. C-027 consumes an already-approved policy and does not assign a new owner. **Impact:** this dependency must be resolved upstream before any architecture requiring a canonical policy publisher can be locked.

**OI-C027-02 — Institution-specific routing:** L2-C04 establishes canonical participating roles and a permission matrix but explicitly allows actual permissions/routing to be configured by institutional policy. C-027 therefore names only upstream-defined roles and does not prescribe institution-specific technical routing.

**OI-C027-03 — Re-evaluation event vocabulary inconsistency:** L2-C04 §7.3 identifies `RevaluationResultPublished` as an Examination event for progression recalculation, while §23.3 identifies `AssessmentRevaluationCompleted` for re-evaluation. C-027 does not silently merge or choose between these names. **Impact:** primary progression via `ExaminationResultPublished` and supplementary progression via `SupplementaryResultPublished` are unambiguous; exact re-evaluation event selection remains an upstream L2 issue where material.

### 17.2 Assumptions

- Examination owns official examination results.
- Academic Progression owns the final progression decision and canonical progression-outcome publications defined by the locked amendment.
- Historical progression decisions remain immutable.

### 17.3 Deferred Questions

- Resolve Policy Management ownership/publisher upstream.
- Resolve the exact canonical re-evaluation event vocabulary where a re-evaluation-specific L3 scenario depends on it.
- Institution-specific thresholds and routing remain governed configuration rather than new L3 architecture.
