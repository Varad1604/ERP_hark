# Workflow Execution Failure to Controlled Recovery Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-A-016  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workflow Engine  
**Participating Domains:** A, VAR:Workflow-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-015 — Workflow Initiation to Completed Orchestrated Execution  
**Referenced By:** Workflow-owning domains requiring governed failed-execution outcomes

---

# 2. Scenario Purpose

Govern the business-visible lifecycle of a failed Workflow Instance so recovery preserves Tenant isolation, originating-version integrity, domain ownership, correlation and auditability, and results in either resumed orchestration or an explicitly governed closure.

---

# 3. Business Outcome

Execution is recovered from a valid persisted checkpoint and either resumes toward completion or reaches an explicitly governed terminal outcome without losing workflow integrity.

The frozen outcome requires a persisted-checkpoint recovery mechanism that is below normal L3 business-scenario detail, while L2-A05 does not define a canonical generic recovery command or the authority for retry, recovery and terminal resolution. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an active workflow instance fails because of a runtime, integration, timeout, policy or other recoverable execution condition.

## 4.2 End Boundary

The intended end boundary is reached when the failed instance either returns to `Running` under an authorized governed recovery outcome or reaches `Cancelled` as the explicitly governed execution-terminal outcome, with canonical events and immutable evidence preserved.

## 4.3 In Scope

- Classification of the failed execution condition using L2-A05 failure categories.
- Preservation of the failed instance, originating Workflow Version, Tenant, correlation and domain-owned outcomes.
- `Running → Failed → Recovering` lifecycle governance.
- Validation that recovery is permitted and that a valid prior execution position exists.
- Authorized resumption to `Running` or governed cancellation.
- Canonical failure, recovery, resumption and cancellation events.
- Audit evidence for failure classification, authority, recovery decision and final outcome.

## 4.4 Out of Scope

- Defining infrastructure retries, persistence technology, queues, transaction rollback, thread/process restart or deployment recovery.
- Re-evaluating or overriding a participating domain's business decision.
- Changing the originating Workflow Version.
- Silently migrating the instance to a successor version.
- Reopening a `Completed` or `Cancelled` instance.
- Inventing an administrator, recovery approver, command, policy or terminal state.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-015 — Workflow Initiation to Completed Orchestrated Execution | Supplies the active instance and normal execution boundary. |
| Related | L3-A-017 — Workflow Change to Governed New Version Availability | A successor version does not automatically repair or migrate this failed instance. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Workflow Engine / Section A | Primary owner. Owns failure classification, Workflow Instance lifecycle, recovery governance, orchestration state and workflow lifecycle events. |
| VAR:Workflow-Owning Domain | Owns its business process, decisions, commands, business failures and outcomes; supplies authoritative business facts used by recovery policy. |
| Platform & Tenant / Section A | Supplies Tenant context, time and relevant configuration. |
| Person & Identity / Section A | Supplies canonical actor references where human intervention is authorized. |
| Authentication / Section A | Supplies authenticated context for protected recovery actions. |
| Authorization & Policy / Section A | Supplies Authorization Decisions and policy outcomes for recovery, resumption or cancellation. |
| Event Infrastructure / Section A | Distributes canonical failure and recovery events without owning them. |
| Audit Infrastructure / Section A | Owns immutable evidence derived from canonical events and correlated recovery facts. |

The frozen Participating Domains value remains exactly `A, VAR:Workflow-Owning Domain`.

---

# 6. Business Actors, Roles and Responsibilities

L2-A05 records Actors as absent. It does not name the actor authorized to request retry, approve recovery, resume a failed instance or choose cancellation. `RetryWorkflow` appears only as a representative runtime command, without a complete authority contract. No workflow administrator, support operator, approver or recovery authority is invented.

---

# 7. Preconditions and Entry Criteria

- One Workflow Instance exists within exactly one Tenant.
- The instance is bound to one immutable originating Workflow Version.
- The instance was active and has reached `Failed` through a recognized failure condition.
- The failure, current lifecycle position, correlation and completed domain outcomes are preserved.
- Applicable recovery, authorization and cancellation policies can be resolved.
- The Workflow-Owning Domain remains authoritative for its business decisions and results.
- No recovery request attempts to alter historical events or completed domain outcomes.

---

# 8. Trigger

An active workflow instance fails because of a runtime, integration, timeout, policy or other recoverable execution condition.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Recognize that the active Workflow Instance cannot continue under its current execution condition. | Workflow Engine; applicable participating domain | A governed failure condition is established. |
| 2 | Classify the condition as business, integration, runtime, infrastructure, timeout or policy failure without changing domain ownership. | Workflow Engine; VAR:Workflow-Owning Domain where it owns the business failure | The failure category and owner are explicit. |
| 3 | Complete the permitted transition to `Failed` and publish `WorkflowFailed`. | Workflow Engine | Normal execution stops and the failure becomes a canonical fact. |
| 4 | Preserve Tenant, instance, originating version, correlation, completed activity outcomes and the latest valid execution position. | Workflow Engine | Workflow integrity and traceability are retained. |
| 5 | Determine whether recovery is allowed by the originating version, applicable policy, failure category and current lifecycle state. | Workflow Engine; Authorization & Policy; VAR:Workflow-Owning Domain supplies its rules | A governed recovery-eligibility outcome is established. |
| 6 | Obtain the required authorization for retry or recovery. | Canonical recovery authority undefined; Authorization & Policy | Recovery may proceed only when authorized. |
| 7 | Complete `Failed → Recovering` while retaining the originating Workflow Version. | Workflow Engine | The instance enters the governed recovery lifecycle. |
| 8 | Validate that the recovery point is usable without changing completed domain outcomes or replaying business decisions without authority. | Workflow Engine; participating domains | The intended resumption preserves business integrity. |
| 9 | If recovery succeeds, complete `Recovering → Running`, publish `WorkflowRecovered` and `WorkflowResumed` according to their canonical transition conditions, and return control to A015. | Workflow Engine | The same instance resumes toward its defined outcome. |
| 10 | If governed recovery cannot or must not resume, obtain the required terminal-resolution authority and complete `Recovering → Cancelled`. | Canonical terminal-resolution authority undefined; Workflow Engine | The failed execution reaches its defined execution-terminal outcome. |
| 11 | Publish `WorkflowCancelled` only after cancellation succeeds and return the governed outcome to the Workflow-Owning Domain. | Workflow Engine | Consumers receive the canonical closure fact. |
| 12 | Preserve immutable evidence of failure, classification, authorization, recovery decision, lifecycle transitions and final outcome. | Workflow Engine; Audit Infrastructure | The recovery outcome is fully traceable. |

Steps 6, 8 and 10 cannot be finalized as exact L3 business-authority behaviour because L2-A05 does not define the universal authority or complete recovery contract. The mechanics of persisted-checkpoint restoration are deliberately not specified at L3.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Recoverable participating-domain condition | The domain confirms that its prior outcome remains valid and continuation is permitted. | Preserve that authoritative outcome and resume only the orchestration activity allowed by policy. | The same instance returns to `Running`. | Workflow Engine; VAR:Workflow-Owning Domain |
| Recovery requires new domain action | The failed action may be requested again under the originating workflow and domain policy. | Obtain authorization and issue a new governed domain command correlated to the failed activity. | The domain returns a new authoritative outcome; historical outcomes remain immutable. | Workflow Engine; VAR:Workflow-Owning Domain |
| Recovery not permitted | Policy, state or domain ownership prevents resumption. | Do not resume; follow the governed cancellation path if authorized. | The instance reaches `Cancelled` or remains unresolved pending authority. | Workflow Engine; Authorization & Policy |
| Recovery interrupted | The instance is in `Recovering` but no valid resolution has completed. | Preserve `Recovering` and its evidence; do not claim resumption or cancellation. | The scenario remains incomplete. | Workflow Engine |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Failure cannot be classified. | Recovery policy and ownership cannot be determined. | Keep the instance `Failed`; do not initiate recovery. | Workflow Engine |
| Tenant, instance or originating version cannot be resolved. | Integrity of the execution boundary is not established. | Stop recovery and preserve the failure record. | Platform & Tenant; Workflow Engine |
| Recovery is not authorized. | The instance cannot enter or continue recovery. | Retain `Failed` or apply an independently authorized cancellation outcome. | Authorization & Policy; Workflow Engine |
| No valid recovery position can be established. | Resumption could duplicate or corrupt business effects. | Do not resume; seek the governed terminal resolution. | Workflow Engine; participating domains |
| Participating-domain outcome is ambiguous. | Orchestration cannot safely determine what already occurred. | Ask the owning domain for an authoritative outcome; do not infer or overwrite it. | VAR:Workflow-Owning Domain; Workflow Engine |
| Recovery action fails. | The instance cannot return to `Running`. | Preserve `Recovering` or follow authorized cancellation; do not publish `WorkflowRecovered` or `WorkflowResumed`. | Workflow Engine |
| Cancellation is not authorized. | No governed terminal outcome can be reached. | Preserve the unresolved lifecycle state and escalate through an upstream-defined authority when available. | Authorization & Policy; Workflow Engine |
| Event publication condition is not satisfied. | Consumers cannot rely on an incomplete recovery fact. | Do not publish the corresponding success or cancellation event. | Workflow Engine; Event Infrastructure |

No technical retry schedule, rollback, persistence implementation or infrastructure-recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only Workflow Engine may transition Workflow Instance state. | Workflow Engine | Participating domains cannot directly mark recovery or cancellation. | L2-A05 instance lifecycle rules |
| `Failed → Recovering` is the governed recovery entry path. | Workflow Engine | A failed instance cannot jump directly to normal execution. | L2-A05 lifecycle transition matrix |
| `Recovering → Running` or `Recovering → Cancelled` are the defined recovery outcomes. | Workflow Engine | No additional terminal state is invented. | L2-A05 lifecycle transition matrix |
| Completed and Cancelled executions cannot resume. | Workflow Engine | Recovery is prohibited after execution closure. | L2-A05 lifecycle rules |
| The instance retains its originating immutable version. | Workflow Engine | Failure cannot silently select a successor version. | L2-A05 versioning and migration rules |
| Business decisions and outcomes remain domain-owned. | VAR:Workflow-Owning Domain | Recovery cannot rewrite a domain result or assume business authority. | L2-A05 WEP-02 and WEP-03 |
| Tenant isolation and correlation must be preserved. | Workflow Engine; Platform & Tenant | Recovery cannot cross Tenant or instance boundaries. | L2-A05 WEP-08 and runtime context |
| Canonical events publish only after their transitions succeed. | Workflow Engine; Event Infrastructure | Failed recovery cannot publish a success fact. | L2-A05 event principles; L2-A06 |
| Recovery activity must be auditable. | Workflow Engine; Audit Infrastructure | Failure cause, authority, decisions and outcomes remain immutable evidence. | L2-A05 audit rules; L2-A07 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Workflow Version | Originating immutable version | Remains the sole definition governing recovery. | Unchanged. | Workflow Engine |
| Workflow Instance | `Running` or another active execution position | Failure stops normal progression. | `Failed` | Workflow Engine |
| Workflow Instance | `Failed` and recovery permitted | Enter controlled recovery. | `Recovering` | Workflow Engine |
| Workflow Instance | `Recovering` and recovery succeeds | Resume the originating orchestration. | `Running` | Workflow Engine |
| Workflow Instance | `Recovering` and governed closure is authorized | End execution without completion. | `Cancelled` | Workflow Engine |
| Participating-domain aggregate | Existing domain-owned outcome | Preserve or change only through its owning domain's governed command. | Domain-defined state. | VAR:Workflow-Owning Domain |

No `Recovered`, `Retrying` or `Abandoned` state is introduced because L2-A05 defines none.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Publication Condition | Business Effect / Consumers |
|---|---|---|---|
| `WorkflowFailed` | Workflow Engine | The Workflow Instance reaches `Failed`. | Owning domain, monitoring and audit receive the failure fact. |
| `WorkflowCheckpointCreated` | Workflow Engine | L2-A05's governed checkpoint condition completes. | Recovery and audit consumers receive the checkpoint fact; this Draft does not define persistence mechanics. |
| `WorkflowRecovered` | Workflow Engine | Recovery succeeds and the instance is eligible to return to execution. | Owning domain and audit receive the recovery fact. |
| `WorkflowResumed` | Workflow Engine | The instance completes its governed transition back to `Running`. | Normal orchestration under A015 continues. |
| `WorkflowCancelled` | Workflow Engine | The governed cancellation transition succeeds. | Owning domain and audit receive the execution-closure fact. |

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Contract / Hand-off | Business Information / Outcome |
|---|---|---|---|
| Workflow Engine | VAR:Workflow-Owning Domain | Failure notification and context | Instance, version, correlation, failure category and affected business reference. |
| VAR:Workflow-Owning Domain | Workflow Engine | Authoritative business outcome | Whether a domain action completed, failed, may be requested again or requires a business path. |
| Authorization & Policy | Workflow Engine | Recovery, resumption or cancellation decision | Governed allow or deny result for the protected lifecycle action. |
| Workflow Engine | VAR:Workflow-Owning Domain | Recovery or terminal outcome | Resumed execution or cancellation result. |
| Workflow Engine | Audit Infrastructure | Canonical failure and recovery events | Immutable correlated evidence. |

L2-A05 lists `RetryWorkflow` only as a representative command. It does not define a canonical generic recovery request/approval contract or its authorized initiator.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Recovery, resumption and cancellation are protected Workflow Instance lifecycle actions.
- Authentication alone does not authorize them.
- Authorization & Policy supplies decisions; Workflow Engine owns the lifecycle transition.
- The Workflow-Owning Domain remains authoritative for its business effects.

## 15.2 Approval and Separation of Duty

- L2-A05 does not define a universal recovery approver, terminal-resolution authority or scenario-specific SoD rule.
- No administrator or override authority is inferred from the representative `RetryWorkflow` command.
- Any policy-specific approval must be explicit before it can be used by this scenario.

## 15.3 Audit Obligations

The following must be auditable: Tenant; instance and originating version; correlation; failure time and category; failed orchestration position; relevant domain outcome; recovery eligibility; actor and authorization references; lifecycle transitions; recovery, resumption or cancellation event; final status; and any unresolved outcome. Sensitive error detail must not be exposed beyond its governed audit purpose.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- the failure was classified and `WorkflowFailed` was published;
- Tenant, instance, originating version, correlation and prior authoritative outcomes were preserved;
- recovery eligibility and authorization were established;
- the instance entered `Recovering` through its canonical transition;
- a valid persisted checkpoint was used as required by the frozen outcome, without specifying its implementation at L3;
- the instance either returned to `Running` with the appropriate recovery/resumption events or reached authorized `Cancelled` with `WorkflowCancelled`;
- the Workflow-Owning Domain received the outcome; and
- immutable evidence was preserved.

These criteria cannot yet be validated as a complete canonical business outcome because the recovery authority, terminal-resolution authority and complete generic recovery contract are undefined, and the frozen checkpoint wording crosses the L3 technical-detail boundary.

Postconditions:

- A resumed instance continues under A015 and the same originating version.
- A cancelled instance cannot resume.
- Completed domain outcomes and historical events remain immutable.
- No version migration or new business decision ownership is created.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A016-BLOCKER-01:** The frozen outcome mandates recovery from a “valid persisted checkpoint,” while L3 validation rules prohibit technical retry, persistence and recovery-mechanism design. | The outcome must be preserved verbatim, but its implementation-level mechanism cannot be specified or validated at L3. | Reframe the frozen outcome as a governed recovery position or explicitly authorize checkpoint semantics at L3 while deferring persistence design to L4/L5. |
| **A016-BLOCKER-02:** L2-A05 lists `RetryWorkflow` as a representative command but defines no complete canonical command/contract for entering recovery, selecting a recovery position, approving resumption or choosing terminal cancellation. | The state path exists, but its authoritative business operations and inputs are incomplete. | Define canonical recovery commands and their lifecycle preconditions and outcomes. |
| **A016-BLOCKER-03:** L2-A05 records Actors as absent and names no retry initiator, recovery approver, resumption authority or terminal-resolution authority. | Sections 6, 9 and 15 cannot allocate the required business authority without invention. | Define the canonical recovery actors and authority boundaries. |
| **A016-ISSUE-01:** The frozen outcome permits an “explicitly governed terminal outcome,” while the L2 recovery transition matrix provides only `Recovering → Cancelled` before later archival. | This Draft treats `Cancelled` as the only defined execution-terminal recovery outcome. | Confirm whether any other governed terminal outcome is intended and, if so, add its state and transition. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
