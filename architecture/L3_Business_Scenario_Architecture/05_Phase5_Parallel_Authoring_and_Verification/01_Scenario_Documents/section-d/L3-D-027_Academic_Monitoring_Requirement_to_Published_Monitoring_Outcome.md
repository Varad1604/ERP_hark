# Academic Monitoring Requirement to Published Monitoring Outcome

**Scenario ID:** L3-D-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Monitoring  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D05_Academic_Monitoring_Workflows.md`  
**Referenced By:** None yet  
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

The monitoring outcome is published, `AcademicMonitoringOutcomePublished` is available and risk evaluation is triggered where applicable.

## 4.3 In Scope

- Create case, define scope, plan, rules and authorities, then activate the cycle.
- Collect references to authoritative evidence from D01–D04 and Section C.
- Create immutable evaluation context, execute evaluation, assess health, snapshot and publish outcome.

## 4.4 Out of Scope

- Copying or modifying source-domain truth.
- Automatic governance decision or intervention.
- Student registration ownership.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D05 | Owns monitoring case, plan, rules, evaluation context, health assessment, snapshots and outcome. |
| D01–D04 and Section C | Own source academic and registration facts consumed as immutable evidence references. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Monitoring Authority | Initiates the case, owns monitoring execution and reviews the outcome. | Initiates and executes monitoring. |
| Academic Governance Authority / Governance Authority | Approves the monitoring requirement and governed configuration/decision where required. | Decides governance approval. |
| D05 Monitoring Engine | Collects references, evaluates rules and produces governed records. | Automated execution only. |
| Reviewer | Performs independent review where required. | No self-approval. |

# 7. Preconditions and Entry Criteria

- Monitoring requirement, scope and source evidence availability are identifiable.
- Approved rules, policy versions and assigned authorities exist before activation.

# 8. Trigger

An approved academic monitoring requirement becomes due or an authorized monitoring condition occurs.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the Academic Monitoring Case and define scope. | Monitoring Authority / D05 | Case and authoritative scope exist. |
| 2 | Create plan, configure rules, assign authorities and activate cycle. | D05 with Governance Authority | Governed monitoring configuration is Active. |
| 3 | Initiate execution and collect references to authoritative evidence. | D05 Monitoring Engine | Traceable evidence set exists without ownership duplication. |
| 4 | Create immutable evaluation context and execute evaluation. | D05 | Evaluation result is reproducible and policy-versioned. |
| 5 | Generate Academic Health Assessment and identify risk candidates. | D05 Monitoring Engine / Monitoring Authority | Evidence-based health result exists. |
| 6 | Create immutable monitoring snapshot. | D05 | Historical monitoring truth preserved. |
| 7 | Validate completeness and publish monitoring outcome. | Monitoring Authority / D05 | `AcademicMonitoringOutcomePublished`; risk evaluation triggered. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Academic Monitoring |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Academic Monitoring |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Academic Monitoring |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Academic Monitoring | Prevents cross-domain ownership leakage. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Academic Monitoring | Prevents governance bypass. | `L2-D05_Academic_Monitoring_Workflows.md` |
| Published or historical business facts remain immutable. | Academic Monitoring | Preserves auditability and lineage. | `L2-D05_Academic_Monitoring_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Academic Monitoring Case | Requirement Identified | Set up, activate and execute monitoring through publication. | Created → Scoped → Planned → Configured → Activated → Executing → Outcome Published | D05 | L2-D05 §§10–11 |
| Monitoring Snapshot | No snapshot | Capture evidence, context, evaluation and health state. | Immutable snapshot created | D05 | L2-D05 §11.8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AcademicMonitoringStarted / MonitoringExecutionContextCreated / MonitoringEvidenceCollected / EvaluationContextCreated / AcademicEvaluationCompleted / AcademicHealthAssessmentCreated / MonitoringSnapshotCreated / AcademicMonitoringOutcomePublished | D05 | D05 governance workflows, Section C and authorized consumers | Records setup/execution completion and published monitoring outcome. | L2-D05 §§10–11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D01–D04 and Section C | Academic Monitoring | Own source academic and registration facts consumed as immutable evidence references. | Preserve ownership while enabling the scenario. | `L2-D05_Academic_Monitoring_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiates the case, owns monitoring execution and reviews the outcome. | Monitoring Authority | Initiates and executes monitoring. | Academic Monitoring |
| Approves the monitoring requirement and governed configuration/decision where required. | Academic Governance Authority / Governance Authority | Decides governance approval. | Academic Monitoring |
| Collects references, evaluates rules and produces governed records. | D05 Monitoring Engine | Automated execution only. | Academic Monitoring |
| Performs independent review where required. | Reviewer | No self-approval. | Academic Monitoring |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Academic Monitoring |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Academic Monitoring |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Academic Monitoring |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A complete, evidence-backed monitoring outcome and immutable snapshot are published for governance consumption without modifying any source-domain record.
- All required authority, validation and audit evidence is retained.
- Applicable completed business facts are published exactly once in business effect.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical decisions and prior states remain immutable.
- Downstream domains consume facts without modifying the owning aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D05_Academic_Monitoring_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
