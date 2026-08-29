# Human Decision Requirement to Completed or Escalated Task Resolution

# 1. Scenario Metadata

**Scenario ID:** L3-A-058  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workflow Engine  
**Participating Domains:** A, VAR:Task-Owning Domain  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Business workflows containing governed Human Task nodes

---

# 2. Scenario Purpose

Coordinate a workflow Human Task from creation and eligible-candidate assignment through claim and work to a completed business decision or policy-governed escalation, without the Workflow Engine owning identity, authorization or the business decision.

---

# 3. Business Outcome

The task is assigned to resolved eligible candidates, claimed and worked, and reaches a completed decision or a governed escalation outcome under its applicable SLA policy, returning control to workflow orchestration without the Workflow Engine owning the business decision

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An executing workflow reaches a node requiring a human decision within a business process.

## 4.2 End Boundary

The Human Task reaches `Completed` and returns its domain-owned decision to orchestration, or an SLA/policy trigger produces a governed `Escalated` outcome and corresponding orchestration path.

## 4.3 In Scope

- Human Task creation and lifecycle.
- Candidate resolution through Authorization contracts.
- Assignment, claim and work execution.
- SLA tracking, timeout and escalation.
- Completion or governed escalation outcome.
- Workflow continuation and audit history.

## 4.4 Out of Scope

- Owning or deciding the business-domain decision.
- Identity, position, role or delegation ownership.
- Notification delivery.
- UI, work-queue implementation, APIs or persistence.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Embedded in | L3-A-015 — Workflow Initiation to Completed Orchestrated Execution | A058 governs only the Human Task node. |
| Related | L3-A-011 — Responsibility Delegation | Authorization delegation may inform candidate resolution but is not Task Delegation. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Workflow Engine / Section A | Owns Human Task lifecycle, assignment/claim state, escalation state and execution history. |
| Authorization & Policy / Section A | Resolves eligible candidates and authorization decisions. |
| Person & Identity / Section A | Supplies canonical identity references. |
| Event Infrastructure / Section A | Distributes stable Task lifecycle events. |
| Audit Infrastructure / Section A | Preserves immutable long-term evidence. |
| VAR:Task-Owning Domain | Owns business meaning, decision rules and resulting business outcome. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Eligible Candidate | May claim the available task. | Eligibility is resolved by Authorization, not Workflow. |
| Task Performer | Works and completes the claimed task. | Owns the human action, not Workflow policy. |
| Business Decision Owner | Defines and owns decision meaning. | Belongs to `VAR:Task-Owning Domain`. |
| Workflow Engine | Coordinates state and orchestration. | Does not make the business decision. |

---

# 7. Preconditions and Entry Criteria

- A valid running Workflow Instance reaches a Human Task node.
- Tenant, workflow runtime and correlation context are available.
- The task definition identifies candidate-resolution and SLA policy references.
- Authorization can resolve eligible identities.
- The business decision inputs are available or can be referenced.

---

# 8. Trigger

An executing workflow reaches a node requiring a human decision within a business process

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the Human Task from the active workflow node. | Workflow Engine | Task enters `Created` and its creation fact is published. |
| 2 | Request candidate resolution using Roles, hierarchy, assignments and applicable policies. | Workflow Engine; Authorization & Policy | Authorized candidate identities are returned. |
| 3 | Make the task `Available` to resolved candidates and record assignment. | Workflow Engine | Eligible candidates can discover the task. |
| 4 | Start the applicable response and completion SLA timers. | Workflow Engine | Warning and escalation thresholds are governed. |
| 5 | Receive `ClaimTask` from an eligible authenticated candidate and atomically validate eligibility. | Task Performer; Workflow Engine | Task reaches `Claimed`. |
| 6 | Begin work and transition the task to `In Progress`. | Task Performer; Workflow Engine | Human decision activity is underway. |
| 7 | Apply the Task-owning domain's decision rules and capture its decision result. | Task Performer; VAR:Task-Owning Domain | The business decision remains domain-owned. |
| 8 | Execute `CompleteTask` and transition to `Completed`. | Task Performer; Workflow Engine | Completion fact is published. |
| 9 | Return the decision outcome to Workflow orchestration and continue the configured path. | Workflow Engine; Task-Owning Domain | Control returns to the workflow without ownership transfer. |
| 10 | Preserve candidate, assignment, claim, SLA, decision reference, state and correlation history. | Workflow Engine; Audit Infrastructure | The task is auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome |
|---|---|---|---|
| SLA escalation | Warning or escalation threshold is reached before completion. | Apply policy action: notify, delegate, reassign, alternate path, compensate or cancel. | Task reaches `Escalated` and may return to `Available` under policy. |
| Delegation | Claimed/available work is delegated by applicable policy. | Record original and delegate identities and return to available assignment. | Execution responsibility changes; Workflow ownership does not. |
| Reassignment | Candidate eligibility changes or assignment is corrected. | Re-resolve candidates and record complete history. | Task remains governable. |
| Cancellation | Workflow or policy terminates the task. | Transition a valid non-terminal state to `Cancelled`. | No completed business decision is asserted. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| No eligible candidates. | Task cannot be assigned. | Suspend, retry, escalate or cancel according to runtime policy. | Workflow Engine; Authorization & Policy |
| Unauthorized claim. | Task ownership would be invalid. | Reject claim and audit the attempt. | Workflow Engine; Authorization & Policy |
| Concurrent claim. | Multiple performers could act. | Enforce atomic claim; only one succeeds. | Workflow Engine |
| SLA policy unavailable. | Escalation behaviour is undefined. | Apply configured policy-failure response. | Workflow Engine |
| Completion is interrupted. | Decision and Task state may diverge. | Recover from the runtime checkpoint without re-owning the business decision. | Workflow Engine |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| Workflow Engine exclusively owns Human Task lifecycle. | Business domains cannot directly mutate Task state. | L2-A05 §§61–64 |
| Authorization resolves candidates. | Workflow does not evaluate roles or hierarchy. | L2-A05 §§65–66 |
| Only eligible candidates may claim and claim is atomic. | Concurrent or unauthorized claims fail. | L2-A05 §67 |
| SLA and escalation remain policy-driven. | Workflow follows returned decisions. | L2-A05 §§70–71, 126 |
| Workflow does not own the business decision. | Domain semantics stay with `VAR:Task-Owning Domain`. | L2-A05 Human Task principles |
| Completed tasks are retained according to policy. | Execution history remains available. | L2-A05 §62 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Workflow Task | None | `Created → Available → Claimed → In Progress → Completed`. | Completed terminal task. |
| Escalated Task | Available | `Available → Escalated → Available` or configured alternate outcome. | Governed escalation. |
| Workflow Instance | Running at Human Task node | Wait, then continue after completion/escalation. | Orchestration resumes. |
| Business object | Awaiting human decision | Apply domain-owned decision only on valid completion. | Domain-governed outcome. |
| Identity / Authorization | Existing | Referenced/evaluated only. | Unchanged. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Business Events

| Event family | Publisher | Condition |
|---|---|---|
| `WorkflowTaskCreated` / `TaskCreated` | Workflow Task | Task is generated. |
| `WorkflowTaskAssigned` / `TaskAssigned` | Workflow Task | Candidate assignment is established. |
| `WorkflowTaskClaimed` / `TaskClaimed` | Workflow Task | Atomic claim succeeds. |
| `WorkflowTaskCompleted` / `TaskCompleted` | Workflow Task | Task reaches Completed. |
| `WorkflowTaskEscalated` / `TaskEscalated` | Workflow Task | Policy escalation occurs. |

The paired names reflect an upstream catalogue inconsistency recorded in Section 17; no new event name is selected by this Draft.

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Candidate resolution | Authorization & Policy | Workflow Engine | Eligible canonical identities. |
| Identity references | Person & Identity | Authorization & Workflow | Stable candidate/performer references. |
| Business decision rules | Task-Owning Domain | Task Performer | Domain-owned decision meaning. |
| Completed decision | Workflow Task | Workflow Instance | Decision result and continuation signal. |
| Audit input | Workflow Engine | Audit Infrastructure | Complete Task execution history. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Task visibility, claim and completion require authenticated, authorized context.
- Work queues are views and never grant assignment or access.

## 15.2 Approval and Separation of Duty

- Any approval meaning belongs to the Task-owning domain.
- Assignment, delegation and SoD remain policy-driven through Authorization.

## 15.3 Audit Obligations

Record Task, workflow, candidates, policy, assignee, claim, delegation/reassignment, SLA, escalation, decision reference, state, actor, timestamp and correlation.

---

# 16. Completion Criteria and Postconditions

- Task was created by a valid running Workflow Instance.
- Eligible candidates were resolved externally.
- Claim was authorized and atomic.
- The domain-owned decision was completed and returned, or policy escalation occurred.
- Workflow orchestration continued through the configured path.
- Complete execution history and events were retained.

Postconditions: Workflow owns Task state, Authorization owns eligibility, and `VAR:Task-Owning Domain` owns the business decision.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|
| A058-ISSUE-01 | Non-blocking | L2-A05's early published-event list uses `WorkflowTaskCreated`, `WorkflowTaskAssigned`, `WorkflowTaskClaimed`, `WorkflowTaskCompleted` and `WorkflowTaskEscalated`, while later canonical examples and notification sections use `TaskCreated`, `TaskAssigned`, `TaskClaimed`, `TaskCompleted` and `TaskEscalated`. | Preserve both source families transparently; select one canonical naming set and publishers before locking/L4. |
| A058-ISSUE-02 | Non-blocking | The lifecycle contains `Available` but not an `Assigned` state although an assignment event is expected. | Treat assignment as metadata/result establishing availability; clarify whether assignment is a state transition before L4. |

No business decision, identity, authorization rule or implementation mechanism is assigned to the Workflow Engine by inference.
