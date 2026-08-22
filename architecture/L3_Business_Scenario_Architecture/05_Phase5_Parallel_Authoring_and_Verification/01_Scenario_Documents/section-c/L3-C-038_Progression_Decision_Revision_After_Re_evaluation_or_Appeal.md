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
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-22  

## 1. Scenario Metadata

This document preserves the frozen Stage 3 identity and semantics for **L3-C-038 — Progression Decision Revision After Re-evaluation or Appeal**. No ownership, participating-domain, type or priority reclassification is introduced.

## 2. Scenario Purpose

Re-evaluate an existing Academic Progression decision when an authoritative re-evaluation result, supplementary result, or other institutionally governed review outcome changes the academic evidence and institutional policy permits a fresh progression evaluation.

The scenario does not create or own an Examination appeal workflow. It consumes authoritative changed academic evidence and preserves Academic Progression as the sole owner of the resulting progression decision.

## 3. Business Outcome

A fresh authoritative progression evaluation is completed where permitted by policy. The previous progression decision remains immutable, a subsequent progression decision is established, and the applicable canonical Academic Progression outcome is published.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when Academic Progression receives authoritative changed academic evidence, including `RevaluationResultPublished` or `SupplementaryResultPublished`, or another governed review outcome that is authoritative for academic evidence and is permitted to trigger a fresh progression evaluation.

### 4.2 End Boundary

The scenario ends when the fresh progression evaluation establishes the new authoritative progression outcome while preserving the historical decision.

### 4.3 In Scope

- Consume authoritative changed academic evidence.
- Verify that institutional policy permits a fresh progression evaluation.
- Re-evaluate progression using the applicable approved policy and authoritative academic data.
- Establish the resulting progression decision.
- Preserve the prior progression decision as immutable history.
- Publish the applicable canonical Academic Progression outcome.

### 4.4 Out of Scope

- Changing Examination-owned results.
- Defining or owning an Examination appeal workflow.
- Inventing an appeal authority, appeal state, or appeal event.
- Editing or deleting the previous progression decision.
- Defining Policy Management ownership where upstream architecture leaves it unresolved.
- Technical replay, retry, API, database, queue, or implementation mechanics.

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
| Academic Progression (C) | Owns the fresh progression evaluation, progression decision, academic standing impact, and resulting progression outcome. |
| Examination (I) | Owns authoritative Examination results and publishes the re-evaluation or supplementary-result facts consumed by Academic Progression. |

Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Examination responsibility | Establishes and publishes authoritative changed Examination evidence. | Source authority for Examination-owned results. |
| Academic Progression | Determines whether the changed evidence may trigger a fresh progression evaluation and establishes the revised progression decision. | Canonical progression decision authority. |
| Authorized institutional review responsibility | May provide an authoritative review/appeal outcome only where such authority is established by the applicable upstream architecture or policy. | No specific role title is invented by this L3 scenario. |

## 7. Preconditions and Entry Criteria

- A prior progression decision exists.
- Authoritative changed academic evidence exists.
- Institutional policy permits a fresh progression evaluation for that evidence.
- Required academic data is complete and authoritative.
- Any review/appeal outcome used as evidence has already been established by its canonical owning process.

## 8. Trigger

The canonical L2-C04 changed-result triggers include:

- `RevaluationResultPublished` from Examination; or
- `SupplementaryResultPublished` from Examination.

A governed appeal/review outcome may trigger this scenario only when an authoritative upstream process establishes that outcome and applicable policy permits a fresh progression evaluation. This L3 does not invent that upstream appeal event.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Receive authoritative changed academic evidence. | Academic Progression consumes; source domain retains ownership. |
| 2 | Verify that the evidence is authoritative and policy permits fresh progression evaluation. | Academic Progression |
| 3 | Re-evaluate applicable credits, standing, and progression criteria using authoritative data. | Academic Progression |
| 4 | Apply the approved progression policy. | Academic Progression |
| 5 | Establish the new progression decision. | Academic Progression |
| 6 | Preserve the historical progression decision unchanged. | Academic Progression |
| 7 | Publish the applicable canonical Academic Progression outcome. | Academic Progression |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Changed evidence is not eligible to trigger re-evaluation | Do not revise progression. | Existing decision remains authoritative. |
| Changed evidence alters progression eligibility | Complete a fresh progression evaluation. | New progression decision established. |
| Fresh evaluation reaches the same conclusion | Record the fresh evaluation without rewriting historical evidence. | Business conclusion remains unchanged; traceability is preserved. |
| A governed review outcome changes the authoritative academic position | Use that outcome only after its upstream authority is established. | Fresh progression evaluation proceeds where policy permits. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Changed academic evidence is incomplete or not authoritative | Do not establish a revised progression decision. |
| Institutional policy does not permit fresh evaluation | Preserve the existing progression decision. |
| Upstream appeal/review authority is unclear | Do not invent an L3 appeal authority or event; retain the matter as an upstream architecture issue. |
| L2 event terminology is internally inconsistent | Do not silently rename canonical outcomes in L3; use only unambiguous canonical events and retain the conflict in Section 17. |

No technical recovery design is introduced.

## 12. Business Rules and Decision Constraints

| Rule | Effect |
|---|---|
| **BR-PROG-001** Only officially published examination results participate in progression evaluation. | Prevents draft or unofficial results from becoming authoritative progression inputs. |
| **BR-PROG-002** Each progression evaluation references one approved institutional progression policy. | Preserves reproducibility. |
| **BR-PROG-006** Re-evaluation or supplementary outcomes trigger a fresh progression evaluation where institutional policy permits. | Governs when revision may occur. |
| **BR-PROG-007** Historical progression decisions remain immutable; approved revisions create new progression records. | Prevents silent historical mutation. |

These rules remain owned by locked L2-C04 or their canonical upstream owners. L3 only applies them.

## 13. State and Lifecycle Impact

| Concept / Aggregate | State Impact | Owner |
|---|---|---|
| Prior Progression Decision | Remains immutable historical evidence. | Academic Progression |
| Progression Evaluation | A fresh evaluation is performed when permitted. | Academic Progression |
| Academic Standing | May change only through the resulting governed progression decision. | Academic Progression domain / applicable owned aggregate |

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Unambiguous Upstream Inputs

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `RevaluationResultPublished` | Examination | Signals an authoritative changed Examination result requiring progression recalculation where policy permits. |
| `SupplementaryResultPublished` | Examination | Signals an authoritative supplementary result requiring progression recalculation where policy permits. |

### 14.2 Academic Progression Outcomes

Locked L2-C04 clearly establishes Academic Progression as the business owner of progression decisions and contains canonical progression events. However, L2-C04 also contains an internal naming inconsistency between its earlier event-ownership catalogue and its later canonical-publisher table for conditional promotion, probation, and detention outcomes.

Accordingly, this L3 scenario does not select one conflicting name as a silent replacement for the other. Unambiguous events such as `ProgressionEvaluationStarted`, `ProgressionEvaluated`, `StudentProgressed`, and `AcademicStandingUpdated` may be used where applicable; disputed outcome naming remains governed by the upstream resolution recorded in Section 17.

No event publisher is transferred to Examination merely because Examination supplies the changed result.

## 15. Business Authorization, Approval and Audit Obligations

- Examination retains authority over Examination-owned changed results.
- Academic Progression retains authority over progression decisions.
- Every fresh evaluation and resulting decision must remain traceable to the changed authoritative evidence and approved policy.
- Historical progression decisions remain immutable.
- No new appeal authority or Policy Management owner is created at L3.

No technical IAM/RBAC design is introduced.

## 16. Completion Criteria and Postconditions

- Authoritative changed academic evidence received.
- Policy permission verified.
- Fresh progression evaluation completed where permitted.
- Historical progression decision preserved.
- New progression decision established where required.
- Applicable unambiguous canonical event(s) published.
- No Examination result or upstream appeal decision is re-owned by Academic Progression.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C038-01 — Appeal authority remains upstream-unresolved for this scenario:** Direct review of locked L2-C04 confirms support for audit evidence relating to academic appeals, but L2-C04 does not itself establish a dedicated Examination appeal decision workflow, canonical appeal authority, appeal state, or appeal event for this scenario. L3 therefore must not invent them. If a dedicated locked Examination workflow exists and is intended to govern this scenario, it must be explicitly cross-referenced before Architecture Approval.

**OI-C038-02 — Policy Management owner remains unresolved upstream:** L2-C04 explicitly identifies `AcademicPolicyUpdated` with Policy Management as an unresolved publisher/owner. This scenario applies an already-approved progression policy but does not assign ownership of Policy Management.

**OI-C038-03 — L2-C04 event-name inconsistency:** L2-C04's earlier event-ownership catalogue uses `StudentConditionallyProgressed`, `StudentPlacedOnProbation`, and `StudentDetained`, while its later canonical-publisher table uses `ConditionalPromotionGranted`, `AcademicProbationAssigned`, and `AcademicDetentionApplied`. This is an upstream locked-document consistency issue and must not be silently resolved in L3. The canonical L2 source should be remediated or formally clarified before C038 is approved/locked.

**Assumption:** Examination remains authoritative for changed examination results; Academic Progression remains the sole owner of the resulting progression decision.

## Production Verification Record

- Frozen canonical identity expressed in required `L3-C-038` format.
- Frozen owner, participating domains, type, priority, and title preserved.
- Direct L2-C04 source verification completed.
- `RevaluationResultPublished` and `SupplementaryResultPublished` preserved as Examination-published inputs.
- No appeal authority or appeal event invented.
- Upstream Policy Management ownership ambiguity retained explicitly.
- L2-C04 event naming conflict escalated rather than silently normalized.
- No implementation-level mechanics introduced.
- Seventeen mandatory L3 sections present.
- Status remains **Draft** pending architecture review and upstream clarification where required.
