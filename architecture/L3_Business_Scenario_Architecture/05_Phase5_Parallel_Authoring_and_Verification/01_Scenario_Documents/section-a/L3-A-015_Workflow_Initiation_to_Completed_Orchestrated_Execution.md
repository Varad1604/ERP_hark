# Workflow Initiation to Completed Orchestrated Execution

# 1. Scenario Metadata

**Scenario ID:** L3-A-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workflow Engine  
**Participating Domains:** A, VAR:Workflow-Owning Domain  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-014 — Workflow Definition to Activated Orchestration Availability  
**Referenced By:** Workflow-owning domains requiring governed orchestration and L3-A-016 — Workflow Execution Failure to Controlled Recovery Outcome

---

# 2. Scenario Purpose

Execute an approved published Workflow Version for a workflow-owning domain, coordinating domain-owned activities and decisions to a governed completion or cancellation outcome without transferring business ownership to the Workflow Engine.

---

# 3. Business Outcome

The workflow instance executes to a governed terminal state of Completed or Cancelled and returns its defined output contract, without the Workflow Engine assuming ownership of the participating domains' business decisions.

The authoritative L2 runtime lifecycle, contracts, events and publisher are sufficient for this Draft. The scenario is classified `PASS WITH ISSUES` because L2-A05 omits named actors and contains overlapping lifecycle-event terminology and a later archival transition beyond the frozen terminal wording.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an owning business domain initiates an approved published workflow with valid execution context and inputs.

## 4.2 End Boundary

The scenario ends when the Workflow Instance reaches `Completed` or `Cancelled`, its governed outcome and defined output contract are returned to the owning domain, canonical lifecycle events are published, and execution evidence is auditable.

## 4.3 In Scope

- Validation of Tenant, Workflow Version, execution context and input contract.
- Creation and start of one Workflow Instance against one immutable version.
- Orchestration of automated, human, event, decision, timer, parallel and sub-workflow steps defined by that version.
- Calls to domain-owned commands and receipt of domain-owned outcomes.
- Waiting, suspension and authorized resumption where defined by the workflow.
- Completion or governed cancellation.
- Return of the defined output contract.
- Publication and audit of canonical lifecycle events.

## 4.4 Out of Scope

- Definition, approval, publication or activation of the Workflow Version.
- Owning or evaluating the participating domains' business rules or decisions.
- Changing domain aggregate state except through the owning domain's governed commands.
- Failure recovery after the instance enters `Failed`; that belongs to L3-A-016.
- Workflow-version migration; that belongs to L3-A-017.
- APIs, database structures, queues, threads, deployment or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-014 — Workflow Definition to Activated Orchestration Availability | Supplies the approved immutable Workflow Version and contracts. |
| Exception | L3-A-016 — Workflow Execution Failure to Controlled Recovery Outcome | Owns the controlled path after execution failure. |
| Related | L3-A-017 — Workflow Change to Governed New Version Availability | Governs successor versions without silently changing this instance. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Workflow Engine / Section A | Primary owner. Owns Workflow Instance lifecycle, orchestration state, task routing, waiting, suspension, resumption, completion, cancellation and workflow lifecycle events. |
| VAR:Workflow-Owning Domain | Initiates the workflow, owns its business process and decisions, executes domain commands, publishes domain events and consumes the defined output. |
| Platform & Tenant / Section A | Supplies Tenant context, time and configuration needed by governed execution. |
| Person & Identity / Section A | Supplies canonical actor and assignee references for human participation. |
| Authentication / Section A | Supplies authenticated context where a human action requires it. |
| Authorization & Policy / Section A | Supplies Authorization Decisions and policy outcomes; the Workflow Engine does not make them. |
| Event Infrastructure / Section A | Distributes canonical workflow and domain events without owning their facts. |
| Audit Infrastructure / Section A | Owns immutable evidence derived from canonical events. |

The frozen Participating Domains value remains exactly `A, VAR:Workflow-Owning Domain`.

---

# 6. Business Actors, Roles and Responsibilities

L2-A05 records its Actors section as absent. The canonical initiator at this level is the Workflow-Owning Domain through the governed `Start Workflow` contract. Human task assignees, administrators and other participants may be resolved by the active Workflow Version and referenced policies, but L2-A05 does not name universal business actors for this scenario. None are invented here.

---

# 7. Preconditions and Entry Criteria

- Exactly one Tenant context is resolved.
- The requested Workflow Definition and immutable Workflow Version exist.
- The version is approved, `Published` and available for execution.
- The Workflow-Owning Domain is authorized to initiate the workflow.
- The execution context identifies the Tenant, correlation, initiator and applicable business references.
- Required input values satisfy the Workflow Input Contract.
- Referenced domain, authorization, event and assignment contracts are available.
- No request attempts to substitute a Draft or retired version for the selected executable version.

---

# 8. Trigger

An owning business domain initiates an approved published workflow with valid execution context and inputs.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive `Start Workflow` with the requested Workflow Version, Tenant context, correlation, business references and input payload. | VAR:Workflow-Owning Domain; Workflow Engine | A governed initiation request is established. |
| 2 | Validate Tenant isolation, initiating authority, version eligibility, execution context and the Workflow Input Contract. | Workflow Engine; Platform & Tenant; Authorization & Policy | The request is eligible to create an instance. |
| 3 | Create one Workflow Instance bound permanently to the selected immutable Workflow Version. | Workflow Engine | The instance enters `Created` without changing the definition. |
| 4 | Complete `Created → Running` and publish `WorkflowStarted`. | Workflow Engine | Governed orchestration begins. |
| 5 | Determine the next eligible step from the fixed workflow structure and current execution context. | Workflow Engine | A defined orchestration action is selected. |
| 6 | For a domain action or decision, invoke the owning domain's governed contract and accept only its authoritative business result. | Workflow Engine; VAR:Workflow-Owning Domain | Business ownership remains with the participating domain. |
| 7 | For a human, event, timer, parallel or sub-workflow step, coordinate the defined participation and preserve instance state while awaiting the governed result. | Workflow Engine; applicable participant | The instance advances or enters `Waiting` without inventing a business decision. |
| 8 | Apply routing and transition conditions defined by the immutable Workflow Version using domain-provided outcomes and policy decisions. | Workflow Engine | The next valid orchestration position is established. |
| 9 | Repeat governed step coordination until a completion or cancellation condition is reached. | Workflow Engine; participating domains | Every executed step remains traceable to the originating version. |
| 10 | When all completion conditions succeed, complete `Running → Completed`, finalize the defined output and publish `WorkflowCompleted`. | Workflow Engine | The successful governed outcome is established. |
| 11 | When an authorized cancellation condition applies, complete the permitted transition to `Cancelled`, finalize the defined cancellation outcome and publish `WorkflowCancelled`. | Workflow Engine | The governed cancellation outcome is established. |
| 12 | Return the Workflow Output Contract and final status to the Workflow-Owning Domain. | Workflow Engine | The owning domain receives the defined orchestration result. |
| 13 | Preserve immutable lifecycle and correlation evidence. | Workflow Engine; Audit Infrastructure | The execution is auditable end to end. |

Only one of Steps 10 or 11 completes for a given instance.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Waiting for governed input | A defined event, timer, external result or human action is not yet available. | Move `Running → Waiting`, preserve the lifecycle position and resume only when the defined condition is satisfied. | Execution remains governed and incomplete. | Workflow Engine |
| Authorized suspension | A permitted request or policy pauses execution. | Move `Running → Suspended`; resume through the governed `Resume Workflow` contract when authorization and resumption conditions succeed. | The same instance and originating version continue. | Workflow Engine; Authorization & Policy |
| Human task | The next step requires assigned human participation. | Resolve the assignee under canonical identity and policy contracts and await the governed task outcome. | The task result feeds orchestration without transferring the underlying business decision. | Workflow Engine; Person & Identity; Authorization & Policy |
| Parallel path | The version defines concurrent branches. | Coordinate all required branches and apply the defined join condition. | Execution advances only when the version's join rule is satisfied. | Workflow Engine |
| Sub-workflow | The version invokes another governed workflow. | Start and correlate the defined child workflow and consume its output contract. | Parent orchestration proceeds according to the version. | Workflow Engine; applicable Workflow-Owning Domain |

Failure recovery is not an alternate A015 path after `Failed`; it is owned by A016.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant, version or input contract is invalid. | A governed instance cannot start. | Reject initiation without treating the workflow as started. | Workflow Engine |
| Initiating authority is denied. | The owning domain cannot start this execution. | Return the denial outcome; do not create a running instance. | Authorization & Policy; Workflow Engine |
| A participating domain rejects a command on business grounds. | The defined path cannot assume success. | Apply only the alternate or terminal path explicitly defined by the immutable version. | VAR:Workflow-Owning Domain; Workflow Engine |
| Required waiting condition is not met. | Execution cannot advance. | Remain `Waiting`; do not fabricate an event, decision or task result. | Workflow Engine |
| Cancellation is not authorized or not permitted from the current state. | The requested terminal outcome is invalid. | Retain the current state and return the governed rejection. | Workflow Engine; Authorization & Policy |
| Execution enters `Failed`. | Normal A015 progression stops. | Publish `WorkflowFailed` and transfer the outcome boundary to L3-A-016. | Workflow Engine |
| Completion or cancellation event condition is not satisfied. | A terminal success fact cannot be asserted. | Do not publish the corresponding event. | Workflow Engine; Event Infrastructure |

No technical retry, rollback or infrastructure recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only published Workflow Versions may execute. | Workflow Engine | Draft or invalid versions cannot create a running instance. | L2-A05 RE-01 and WD-04 |
| Each instance remains bound to its originating immutable Workflow Version. | Workflow Engine | A later version cannot silently alter current execution. | L2-A05 WEP-05 and versioning rules |
| Workflow Engine owns orchestration, not business decisions. | Workflow Engine; VAR:Workflow-Owning Domain | Domain commands and decisions remain authoritative. | L2-A05 WEP-01–WEP-03 |
| Execution is isolated within one Tenant. | Workflow Engine; Platform & Tenant | Cross-Tenant orchestration is prohibited. | L2-A05 WEP-08 |
| Inputs and outputs must satisfy the version's explicit contracts. | Workflow Engine | The instance cannot accept or return undefined business content. | L2-A05 Workflow Input and Output Contracts |
| Only Workflow Engine may transition Workflow Instance state. | Workflow Engine | Participating domains supply facts; they do not mutate orchestration state. | L2-A05 instance lifecycle rules |
| Authorization decisions remain owned by Authorization & Policy. | Authorization & Policy | The Workflow Engine consumes decisions without becoming the policy owner. | L2-A04; L2-A05 WEP-03 |
| Completed instances are immutable. | Workflow Engine | A completed execution cannot resume or change outcome. | L2-A05 lifecycle rules |
| Workflow events are published once after their business transition. | Workflow Engine; Event Infrastructure | Consumers receive canonical completed facts. | L2-A05 event rules; L2-A06 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Workflow Version | `Published` and executable | Referenced immutably by the new instance; no version transition. | Unchanged. | Workflow Engine |
| Workflow Instance | Does not exist | Create and begin execution. | `Created → Running` | Workflow Engine |
| Workflow Instance | `Running` | Coordinate defined steps and domain outcomes. | Remains `Running`, may enter `Waiting` or `Suspended`, or reaches a governed end state. | Workflow Engine |
| Workflow Instance | `Waiting` | Resume when the defined wait condition is satisfied. | `Waiting → Running` | Workflow Engine |
| Workflow Instance | `Suspended` | Resume through an authorized governed request. | `Suspended → Running` | Workflow Engine |
| Workflow Instance | `Running` | Complete all required workflow conditions. | `Completed` | Workflow Engine |
| Workflow Instance | Permitted cancellable state | Apply authorized cancellation. | `Cancelled` | Workflow Engine |
| Participating-domain aggregates | Domain-defined states | Change only through their owning domain's commands. | Domain-defined outcome. | VAR:Workflow-Owning Domain |

`Failed` transfers the scenario boundary to A016. L2-A05 later allows `Completed` and `Cancelled` to move to `Archived`; archival is outside the frozen A015 completion boundary.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Publication Condition | Business Effect / Consumers |
|---|---|---|---|
| `WorkflowStarted` | Workflow Engine | The instance is created against the selected version and begins governed execution. | Owning domain, monitoring and audit receive the start fact. |
| `WorkflowCompleted` | Workflow Engine | All completion conditions succeed and the instance reaches `Completed`. | The owning domain consumes the successful output; audit records completion. |
| `WorkflowCancelled` | Workflow Engine | A permitted cancellation transition completes. | The owning domain consumes the cancellation outcome; audit records closure. |
| `WorkflowFailed` | Workflow Engine | Execution reaches `Failed`. | A015 stops and A016 recovery governance may begin. |

Participating domains publish their own business events. Event Infrastructure distributes all canonical events without becoming their publisher.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Contract / Hand-off | Business Information / Outcome |
|---|---|---|---|
| VAR:Workflow-Owning Domain | Workflow Engine | `Start Workflow` | Workflow identifier/version, Tenant, correlation, business references and valid input. |
| Workflow Engine | Participating domain | Domain-owned command request | Orchestration request made without transferring business ownership. |
| Participating domain | Workflow Engine | Domain business outcome or event | Authoritative fact used for routing and progress. |
| Authorization & Policy | Workflow Engine | Authorization Decision | Governed allow or deny result for protected execution actions. |
| Person & Identity | Workflow Engine | Actor and assignment references | Canonical identity context for human participation. |
| Workflow Engine | VAR:Workflow-Owning Domain | Workflow status and Workflow Output Contract | Final `Completed` or `Cancelled` result and defined output. |
| Workflow Engine | Audit Infrastructure | Canonical lifecycle events | Correlated immutable execution evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- The initiating domain must be authorized to start the selected Workflow Version in the Tenant context.
- Human and administrative actions require their applicable Authorization Decisions.
- The Workflow Engine consumes policy outcomes and never grants business permission itself.
- Authentication establishes identity context but does not authorize workflow or domain actions.

## 15.2 Approval and Separation of Duty

- A015 introduces no universal approval role or scenario-specific SoD rule.
- Any approval, separation-of-duty or assignment requirement is referenced by the immutable Workflow Version and remains owned by the applicable business or policy domain.
- The Workflow Engine coordinates required outcomes but does not approve the underlying business decision.

## 15.3 Audit Obligations

The following must be auditable: Tenant, instance and version identity; initiator reference; correlation; input-contract validation; state transitions; domain commands and correlated outcomes; waiting, suspension and resumption; authorization outcomes; final output contract; published lifecycle events; timestamps and final status.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when:

- initiation was accepted within one Tenant;
- the instance was bound to one approved published immutable version;
- the Workflow Input Contract and execution context were valid;
- domain-owned activities and decisions remained with their owning domains;
- every orchestration transition followed the originating version;
- the instance reached `Completed` or `Cancelled`;
- the corresponding canonical event was published by the Workflow Engine;
- the defined Workflow Output Contract and final status were returned to the owning domain; and
- immutable correlated evidence was preserved.

Postconditions:

- The originating Workflow Version is unchanged.
- A `Completed` instance is immutable; a `Cancelled` instance cannot resume.
- Participating-domain outcomes remain owned by those domains.
- No successor version or migration was created.
- Later archival, where applicable, is outside this scenario.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A015-ISSUE-01:** L2-A05 records Actors as absent and does not name universal human initiation, suspension, resumption or cancellation actors. | Domain contracts and Authorization Decisions are sufficient for the L3 flow, but human responsibility labels cannot be made universal. | Define actors only where L2 governance intends universal roles; otherwise preserve workflow-version-specific assignment. |
| **A015-ISSUE-02:** L2-A05 uses `WorkflowStarted` in its aggregate event catalogue and canonical event contracts, while a later lifecycle catalogue also uses `WorkflowCreated` and `WorkflowActivated`. | Start-event vocabulary overlaps, though `WorkflowStarted` has the clearest start contract and is used here. | Reconcile the lifecycle catalogue and canonical runtime event contracts. |
| **A015-ISSUE-03:** The frozen outcome calls `Completed` and `Cancelled` terminal, while L2-A05 later permits both to transition to `Archived`. | A015 ends at the frozen business outcome; later archival is excluded. | Clarify whether “terminal” means execution-terminal rather than final archival state. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
