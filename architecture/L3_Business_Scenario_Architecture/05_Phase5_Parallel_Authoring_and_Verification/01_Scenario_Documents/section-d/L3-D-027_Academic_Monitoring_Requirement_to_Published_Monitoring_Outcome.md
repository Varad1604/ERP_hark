# L3-D-027 — Academic Monitoring Requirement to Published Monitoring Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-D-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-24  
**Depends On:** `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D05_Academic_Monitoring_Workflows.md`; `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md`; `governance/adr/ADR-D-MON-PURITY-001.md`  
**Frozen Stage 3 Source:** P2W-D-0747; P2W-D-0748 / CAN-L3-D-027; CAN-L3-D-028 (merged final L3-D-027)

---

# 2. Scenario Purpose

Establish and execute an evidence-based Academic Monitoring Case from an approved monitoring requirement through governed publication of its monitoring outcome.

# 3. Business Outcome

A complete, evidence-backed monitoring outcome and immutable snapshot are published for governance consumption without modifying any source-domain record.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Monitoring Authority identifies a monitoring requirement that the Academic Governance Authority has approved.

## 4.2 End Boundary

The monitoring outcome is published, `AcademicMonitoringOutcomePublished` is available, and risk evaluation is triggered where applicable.

## 4.3 In Scope

- Create the monitoring case and establish governed scope.
- Create the monitoring plan, rules and responsible authorities.
- Activate monitoring and collect references to authoritative evidence from D01–D04 and Section C.
- Create immutable evaluation context, evaluate the evidence, assess academic health, preserve a monitoring snapshot and publish the monitoring outcome.
- Preserve business continuity after interruption without creating duplicate institutional outcomes.

## 4.4 Out of Scope

- Copying or modifying source-domain truth.
- Automatic governance approval or governance decision.
- Student registration ownership.
- APIs, queues, locks, persistence, transactions, technical retry design, technical deduplication or orchestration mechanisms.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D05 — Academic Monitoring | Owns monitoring case, plan, rules, evaluation context, health assessment, snapshots and monitoring outcome. |
| D01–D04 and Section C | Own source academic and registration facts consumed as authoritative evidence references. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Monitoring Authority | Initiates monitoring, oversees monitoring execution and reviews the monitoring outcome. | May initiate monitoring within authorized scope; does not acquire ownership of source-domain facts. |
| Academic Governance Authority / Governance Authority | Approves monitoring requirements and makes governance decisions where required by authoritative policy. | Holds governance decision authority where the governing workflow requires approval. |
| Reviewer | Performs independent review where required. | Reviews evidence and outcome within the authorized review scope; no self-approval where separation is required. |

Automated evaluation activity may calculate, evaluate or publish business facts where upstream architecture permits it, but automated capabilities are not institutional business actors and do not hold governance decision authority.

# 7. Preconditions and Entry Criteria

- Monitoring requirement, scope and source-evidence availability are identifiable.
- Approved rules, policy versions and assigned authorities exist before activation.
- Source-domain facts required for evaluation remain owned by their canonical domains.

# 8. Trigger

An approved academic monitoring requirement becomes due, or an authorized monitoring condition occurs.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the Academic Monitoring Case and define its governed scope. | Monitoring Authority / D05 | Case and authoritative monitoring scope exist. |
| 2 | Create the monitoring plan, configure monitoring rules, assign responsible authorities and activate the cycle. | D05 with Governance Authority where approval is required | Governed monitoring configuration becomes active. |
| 3 | Establish the monitoring execution context and collect references to authoritative source-domain evidence without copying ownership. | D05 | Traceable evidence references exist. |
| 4 | Create the immutable evaluation context and evaluate the evidence under the applicable monitoring rules and policy versions. | D05 | A reproducible evaluation result exists. |
| 5 | Generate the Academic Health Assessment and identify any risk candidates. | D05 / Monitoring Authority | Evidence-based health result exists. |
| 6 | Create an immutable monitoring snapshot. | D05 | Historical monitoring truth is preserved. |
| 7 | Validate outcome completeness and publish the monitoring outcome. | Monitoring Authority / D05 | `AcademicMonitoringOutcomePublished` becomes available and risk evaluation may proceed. |

# 10. Alternate Business Flows

- If required evidence is incomplete, monitoring remains at the last valid business position until authoritative evidence is available or a governed terminal decision is reached.
- If a configured approval or review path applies, the scenario follows that authoritative path before publication.
- If a prior monitoring action must be corrected, withdrawn or reversed, only an explicitly governed correction, withdrawal or reversal path may alter the current business position; prior history remains preserved.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or evidence-reference failure | Scenario cannot proceed to the affected outcome. | Reject, return for correction, or wait for authoritative evidence without changing the prior authoritative state. | Academic Monitoring and evidence-owning domain |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior authoritative state and preserve the decision basis. | Academic Monitoring |
| Monitoring activity is interrupted | Completion is not yet established. | Continue from the last valid authoritative business position or use an explicitly governed restart/correction path while preserving business evidence and historical continuity. | Academic Monitoring |
| The same authoritative business fact is received or reconsidered more than once | Duplicate institutional consequences could otherwise arise. | Preserve one authoritative business effect; repeated receipt does not create duplicate monitoring outcomes or duplicate governance consequences. | Academic Monitoring |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Academic Monitoring | Prevents cross-domain ownership leakage. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Monitoring decisions require authoritative evaluation context and traceable evidence. | Academic Monitoring | Prevents unsupported governance conclusions. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Published or historical monitoring facts remain immutable and historically traceable. | Academic Monitoring | Preserves auditability and lineage. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Repeated receipt of the same authoritative business fact shall not create duplicate institutional monitoring outcomes or governance consequences. | Academic Monitoring | Expresses one authoritative business effect without defining a technical deduplication mechanism. | `L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md` |
| Interruption recovery preserves the last authoritative business position and governed context. | Academic Monitoring | Keeps recovery at business level and defers technical retry/orchestration design. | `L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Academic Monitoring Case | Monitoring requirement identified | Establish, scope, plan, activate and execute monitoring through publication. | Monitoring Case progresses through the existing L2-D05 monitoring lifecycle to a published monitoring outcome. | D05 | L2-D05 §§10–11 |
| Monitoring Snapshot | No snapshot for the current evaluated position | Capture evidence context, evaluation and health state. | Immutable snapshot created | D05 | L2-D05 §11.8 |

No technical execution state is introduced by this scenario.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The following exact D05 events are applicable to this scenario where their corresponding L2-D05 workflow step occurs:

| Business Event | Canonical Publisher | Purpose | Source |
|---|---|---|---|
| `AcademicMonitoringCaseCreated` | D05 | Records creation of the monitoring case. | L2-D05 §10.3 |
| `MonitoringScopeDefined` | D05 | Records confirmation of the monitoring scope. | L2-D05 §10.4 |
| `MonitoringPlanCreated` | D05 | Records establishment of the monitoring plan. | L2-D05 §10.5 |
| `MonitoringRulesConfigured` | D05 | Records the governed monitoring rules applied to the plan. | L2-D05 §10.6 |
| `MonitoringAuthorityAssigned` | D05 | Records assignment of monitoring authority. | L2-D05 §10.7 |
| `AcademicMonitoringStarted` | D05 | Records activation of monitoring. | L2-D05 §10.8 |
| `MonitoringExecutionContextCreated` | D05 | Records establishment of monitoring execution context. | L2-D05 §11.3 |
| `MonitoringEvidenceCollected` | D05 | Records availability of the required evidence references. | L2-D05 §11.4 |
| `EvaluationContextCreated` | D05 | Records creation of the immutable evaluation context. | L2-D05 §11.5 |
| `AcademicEvaluationCompleted` | D05 | Records completion of academic monitoring evaluation. | L2-D05 §11.6 |
| `AcademicHealthAssessmentCreated` | D05 | Records creation of the academic health assessment. | L2-D05 §11.7 |
| `MonitoringSnapshotCreated` | D05 | Records creation of the immutable monitoring snapshot. | L2-D05 §11.8 |
| `AcademicMonitoringOutcomePublished` | D05 | Communicates the completed monitoring outcome for governed consumption. | L2-D05 §11.9 |

Cross-domain source events remain published by their canonical source domains. This scenario does not rename or republish them.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D01–D04 and Section C | Academic Monitoring | Canonically owned academic and registration facts supplied as authoritative evidence references. | Preserve source ownership while enabling monitoring evaluation. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Academic Monitoring | Authorized governance consumers | Published monitoring outcome and supporting authoritative monitoring facts. | Enable downstream governance use without transferring D05 ownership. | `L2-D05_Academic_Monitoring_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiate monitoring | Monitoring Authority | Must operate within the assigned monitoring authority and governed scope. | Academic Monitoring |
| Approve monitoring requirement or governance decision where required | Academic Governance Authority / Governance Authority | Approval applies only where required by the governing policy/workflow. | Academic Monitoring |
| Perform independent review where required | Reviewer | Must remain independent where the authoritative architecture requires separation. | Academic Monitoring |

Automated execution performs no governance approval or exception decision.

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever authoritative policy or source architecture requires separation. Any override must be source-supported, authorized, justified and historically traceable.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Responsible actor/authority, reason, prior business position, validation evidence, decision and resulting outcome. | Academic Monitoring |
| Source and cross-domain traceability | Source identifiers, versions, evidence references and consumed source facts. | Academic Monitoring and source-domain owner |
| Interruption and recovery | Last valid business position, missing condition, governed continuation/correction decision and resulting business effect. | Academic Monitoring |
| Duplicate-effect prevention | Evidence that repeated receipt or reconsideration did not create a second institutional outcome. | Academic Monitoring |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A complete, evidence-backed monitoring outcome and immutable snapshot are published for governance consumption without modifying any source-domain record.
- All required authority, validation and audit evidence is retained.
- Applicable D05 events use the exact names defined by L2-D05.
- Repeated receipt or reconsideration of the same authoritative fact has not created duplicate institutional outcomes.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical monitoring decisions and prior states remain traceable.
- Downstream domains consume facts without modifying the owning aggregate.
- Technical recovery, transport and execution mechanisms remain deferred to later architecture levels.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking scenario-specific architecture issue remains from the Phase 5 review. The former L3 purity issue concerning automated actors and implementation-level idempotency/recovery terminology was resolved through `ADR-D-MON-PURITY-001` and `L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md`.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

APIs, queues, persistence, transactions, locks, orchestration, technical retries, deployment and other implementation mechanisms are deferred to later architecture levels.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D05_Academic_Monitoring_Workflows.md`; `L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
