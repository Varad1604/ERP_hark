# Progression Decision Revision After Re-evaluation or Appeal

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** C038  
**Canonical Title:** Progression Decision Revision After Re-evaluation or Appeal  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata

This document preserves the frozen Stage 3 identity and semantics for **C038 — Progression Decision Revision After Re-evaluation or Appeal**. No ownership, participating-domain, type or priority reclassification is introduced.

## 2. Scenario Purpose

Re-evaluate an existing Academic Progression decision when an authoritative re-evaluation, supplementary result or appeal outcome changes the academic evidence permitted by institutional policy.

## 3. Business Outcome

The Student receives a new authoritative progression evaluation where the upstream result/appeal is permitted to trigger reevaluation. The historical progression decision remains immutable; a subsequent decision is established and the corresponding progression state/outcome is published.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins when an authoritative re-evaluation, supplementary result or approved appeal outcome is published and policy permits a fresh progression evaluation.

### 4.2 End Boundary
The scenario ends when the fresh progression evaluation establishes the new authoritative business outcome, while preserving the historical decision.

### 4.3 In Scope
- Consume the authoritative changed academic outcome.
- Verify that a fresh progression evaluation is permitted.
- Re-evaluate progression using the applicable academic data and policy.
- Establish the new progression outcome.
- Preserve the historical decision.
- Publish the canonical resulting progression event(s).

### 4.4 Out of Scope
- Changing examination results directly.
- Editing or deleting the previous progression decision.
- Creating a new local appeal authority.
- Technical replay/retry mechanics.

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
| Academic Progression (C) | Owns the progression decision revision after re-evaluation or appeal business decision and outcome within the L2-C04 boundary. |
| Examination / Assessment (I) | Publishes authoritative re-evaluation/supplementary/appeal-related academic outcomes consumed by Academic Progression. |

Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Examination / Assessment | Owns the authoritative academic assessment outcome. | Source authority for changed result. |
| Academic Progression | Determines the revised progression decision using the changed authoritative outcome. | Canonical progression decision authority. |

No actor, role or authority not supported by the upstream architecture is invented.

## 7. Preconditions and Entry Criteria

 - A prior progression decision exists.
- An authoritative changed academic outcome has been published.
- Institutional policy permits a fresh progression evaluation.
- Required academic data is complete.

## 8. Trigger

One or more of the L2-C04 triggers occurs: `RevaluationResultPublished`, `SupplementaryResultPublished`, or an approved Appeal Decision that permits fresh progression evaluation.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Receive the authoritative changed academic outcome. | Academic Progression |
| 2 | Verify that policy permits fresh progression evaluation. | Academic Progression |
| 3 | Recalculate applicable earned credits and academic standing using the authoritative revised evidence. | Academic Progression |
| 4 | Apply the applicable progression policy. | Academic Progression |
| 5 | Establish the new progression decision. | Academic Progression |
| 6 | Preserve the historical progression decision as immutable. | Academic Progression |
| 7 | Publish the resulting canonical progression outcome and standing event(s). | Academic Progression |

The flow stops when the institutional business outcome defined in Section 3 is reached. Downstream consequences remain owned by their canonical domains.

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Changed result is not eligible to trigger re-evaluation | Do not revise progression. | Existing decision remains authoritative. |
| Revised result changes eligibility | Execute a fresh progression evaluation. | New progression decision established. |
| Revised evaluation still produces the same outcome | Record the new evaluation without changing the business conclusion. | Historical and current evaluation remain traceable. |
| Appeal outcome changes the permitted academic position | Re-evaluate using the authoritative appeal outcome. | New progression decision established if warranted. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Authoritative changed academic data is incomplete | Do not establish a revised progression decision until required data is complete. |
| Re-evaluation is not permitted by policy | Preserve the existing decision and do not create a local exception. |
| Revised evaluation changes standing | Establish the new authoritative standing through the governed progression workflow while preserving history. |
| Revised evaluation makes the Student eligible for qualification | Publish the applicable qualification-eligibility outcome only after the revised progression requirements are satisfied. |

No technical retry, queue, rollback, infrastructure failure or implementation exception model is introduced.

## 12. Business Rules and Decision Constraints

| Rule | Effect |
|---|---|
| **BR-PROG-006** Re-evaluation or supplementary examination outcomes trigger a fresh progression evaluation where institutional policy permits. | Governs when revision may occur. |
| **BR-PROG-007** Historical progression decisions remain immutable; policy-approved revisions create new progression records. | Prevents silent mutation of history. |
| **BR-PROG-001** Only officially published examination results participate in progression evaluation. | Ensures authoritative academic input. |
| **BR-PROG-002** Each progression evaluation references exactly one approved institutional progression policy. | Preserves reproducibility. |

Rules remain owned by L2-C04 or their upstream canonical owners; this L3 scenario does not create local rules.

## 13. State and Lifecycle Impact

| Concept / Aggregate | State Impact | Owner |
|---|---|---|
| Prior Progression Decision | Historical record remains unchanged. | Academic Progression |
| New Progression Evaluation | Fresh evaluation is performed using the changed authoritative academic evidence. | Progression Evaluation within Academic Progression |
| Academic Standing | May change only through the new approved progression workflow. | Academic Standing within Academic Progression |

No new lifecycle state or transition is introduced by this scenario.

## 14. Business Events and Cross-Domain Collaboration

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `RevaluationResultPublished` | Examination / Assessment (I) | Authoritative changed result that may trigger fresh evaluation. |
| `SupplementaryResultPublished` | Examination / Assessment (I) | Authoritative supplementary result that may trigger fresh evaluation. |
| `ProgressionEvaluationStarted` | Academic Progression | Indicates fresh evaluation has begun. |
| `ProgressionEvaluated` | Academic Progression | Indicates the revised evaluation completed. |
| `StudentProgressed` / `StudentConditionallyProgressed` / `StudentPlacedOnProbation` / `StudentDetained` | Canonical owning aggregate within Academic Progression | Communicates the resulting progression outcome where applicable. |
| `AcademicStandingUpdated` | Academic Standing | Communicates the resulting standing change where applicable. |

Events remain distinct from synchronous business hand-offs. Canonical publishers are preserved exactly as defined upstream.

## 15. Business Authorization, Approval and Audit Obligations

The changed examination/appeal outcome remains owned by I. Academic Progression owns the resulting progression decision. Historical decisions are immutable and every revised decision remains auditable and traceable to the authoritative changed academic outcome and applicable policy.

No technical IAM/RBAC model is introduced.

## 16. Completion Criteria and Postconditions

- Authoritative changed academic outcome received.
- Policy permission verified.
- Fresh evaluation completed where permitted.
- Historical decision preserved.
- New progression decision established where required.
- Canonical outcome event(s) published.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C038-01:** The exact appeal workflow/authority in the I domain was not retrievable from the available repository evidence during this authoring pass. This document therefore does not invent an appeal role, appeal state or appeal event.

**OI-C038-02:** The exact policy owner is unresolved in L2-C04: its `AcademicPolicyUpdated` source explicitly notes that Policy Management has no defined repository owner/publisher. This scenario therefore does not introduce a policy-management dependency beyond applying the already-approved progression policy required by C04.

**Assumption:** I remains the authoritative source of the changed examination/assessment outcome, while C04 remains the sole progression decision owner.

All unresolved upstream matters remain explicit rather than being silently resolved.

## Production Verification Record

- Frozen Stage 3 identity preserved.
- Primary Owner, Participating Domains, Scenario Type and Priority preserved.
- L2-C04 ownership and dependency boundaries applied.
- Canonical business events and publishers preserved.
- No invented actors, rules, states, events or authority.
- No implementation-level mechanics.
- Business recovery only.
- Seventeen mandatory L3 sections present.
- Material source gaps are recorded in Section 17.
- Lifecycle status remains **Draft** because repository governance requires newly authored L3 scenarios to begin as Draft; this is a production-quality authoring artifact, not an approval/lock claim.
