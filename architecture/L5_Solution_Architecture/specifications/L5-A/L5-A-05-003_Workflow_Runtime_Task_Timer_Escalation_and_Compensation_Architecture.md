# Workflow Runtime, Task, Timer, Escalation & Compensation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-05-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-A — Core Platform & Application Architecture`  
**Major Area:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Canonical Technical Responsibility:** Runtime task/timer/escalation/technical compensation mechanics without inventing business recovery policy.  
**Technical Architecture Owner:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Assigned Author:** Devang  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-12`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105 v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-A-05-001_Workflow_and_Orchestration_Architecture_Baseline.md`; `L5-A-05-002_Workflow_Definition_Versioning_and_Policy_Resolution_Architecture.md`; `L5-D-02-001` through `L5-D-02-007`; `L5-C-01-001` through `L5-C-01-003`; `L5-C-02-001` through `L5-C-02-003`; `L5-A-04-001/002`; `L5-B-02-002/003`; applicable `L5-D-04`; applicable `L5-A-08`; `L2-A05_Workflow_Engine_Workflows.md`; `L4-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md`; `L4-A-016_Workflow_Execution_Failure_to_Controlled_Recovery_Outcome.md`; `L4-A-058_Human_Decision_Requirement_to_Completed_or_Escalated_Task_Resolution.md`; supporting `L4-A-017`.  
**Referenced By:** workflow-consuming business domains; workflow administration; human-task experiences; background/timer execution; operational monitoring; D04 evidence; E01/E05 resilience and operations architecture.  
**Applicable ADRs:** `ADR-L5-0003` Contract-Driven Modular Platform with Selective Isolation; `ADR-L5-0004` Tenant and Institution Isolation Enforcement Pattern; `ADR-L5-0006` Owner-Local Transaction and Optimistic Concurrency Strategy; `ADR-L5-0007` Atomic State-to-Publication Intent Handoff Pattern; `ADR-L5-0009` Non-Human Principal Authorization Binding Pattern; `ADR-L5-0010` Durable Event Delivery, Subscription, Ordering & Recovery Pattern; `ADR-L5-0011` Event Contract Versioning & Consumer Compatibility Pattern. No runtime engine, scheduler, lock service, task inbox, timer service, queue, cache, database or observability product is selected here.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the L5 technical runtime architecture that executes an immutable Workflow Version after A05-002 has made it eligible for use.

It realizes:

- Workflow Instance runtime;
- execution position and token/state coordination;
- Human Task lifecycle mechanics;
- candidate resolution and claim concurrency;
- wait/resume behavior;
- timer and timeout mechanics;
- SLA/escalation execution;
- checkpoints;
- technical retries;
- controlled recovery;
- compensation orchestration;
- cancellation;
- runtime idempotency and concurrency;
- long-running execution;
- runtime event/evidence handoff.

The primary runtime principle is:

> A05 owns workflow execution state and coordination, while every business domain remains owner of its business decisions, business state, business actions and compensating business behavior.

### 2.2 Runtime Flow

```text
Activated immutable Workflow Version
        ↓
validated start request
        ↓
Workflow Instance created
        ↓
Created → Running
        ↓
Runtime Coordinator
  ├─ invoke domain capability
  ├─ create/await Human Task
  ├─ wait for event
  ├─ wait for timer
  ├─ evaluate source-owned policy result
  ├─ checkpoint
  ├─ retry safe technical work
  └─ invoke source-defined compensation
        ↓
Running / Waiting / Suspended
        ↓
Completed / Cancelled
or
Failed → Recovering → Running / Cancelled
```

### 2.3 In Scope

This specification owns or constrains:

- Workflow Instance runtime state realization;
- execution token/position coordination;
- Workflow Runtime Context;
- runtime variables;
- durable execution state;
- checkpoint creation/use;
- domain invocation sequencing;
- wait/resume mechanics;
- event correlation;
- Human Task runtime objects;
- candidate-resolution handoff;
- claim atomicity;
- current-claimant enforcement;
- task completion handoff;
- task escalation mechanics;
- SLA/time-condition consumption;
- workflow timers;
- timeout detection;
- technical retry scheduling;
- retry eligibility boundary;
- recovery continuation;
- compensation sequencing;
- cancellation effects on workflow runtime;
- runtime concurrency;
- idempotency;
- event publication intent;
- evidence and observability;
- long-running execution;
- A05↔A08 timer/background execution boundary.

### 2.4 Out of Scope / Non-Goals

This specification does not own:

- Workflow Definition creation/versioning — A05-002;
- business rule evaluation;
- business decisions;
- business-state mutation;
- business retry policy;
- business compensation semantics;
- identity ownership;
- Role/Permission/Authorization Policy;
- candidate authority semantics;
- SLA policy content;
- escalation policy content;
- business calendar ownership;
- notification delivery;
- exact recovery approver;
- exact cancellation authority;
- exact compensation authority;
- exact task-event canonical naming while upstream conflict remains unresolved;
- UI work-inbox design;
- timer/scheduler product;
- workflow engine product;
- exact retry counts;
- exact timeouts;
- exact SLA durations;
- database/lock implementation;
- distributed transaction coordinator.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Frozen L5 Responsibility

The approved corpus defines `L5-A-05-003` as:

> Runtime task/timer/escalation/technical compensation mechanics without inventing business recovery policy.

This boundary is mandatory.

A05-003 may implement mechanics.

It may not create business recovery policy.

### 3.2 Locked L2-A05 Runtime Ownership

Locked L2-A05 assigns the Workflow Engine ownership of:

- Workflow Instances;
- Workflow Runtime;
- Workflow Tokens;
- Workflow Tasks;
- Workflow Execution;
- Runtime Context;
- Workflow Variables;
- Workflow State Transitions;
- Workflow Scheduling;
- Workflow Checkpoints;
- Workflow Compensation;
- Workflow Retry Management;
- Workflow Timeout Management;
- Workflow Execution History.

Business domains remain owners of business behavior.

### 3.3 Locked Runtime Principles

The following are upstream facts:

- workflow execution is durable;
- running instances remain version-bound;
- only A05 owns workflow runtime state;
- tasks remain Workflow Engine-owned;
- identity/authorization remain peer-owned;
- retry execution is policy driven;
- retry configuration remains externalized;
- timeout behavior is policy driven;
- timeout classes include node, Human Task, external service and workflow timeout;
- Runtime Checkpoints are created throughout execution;
- checkpoint frequency is policy driven;
- recovery resumes from a valid persisted checkpoint;
- compensation is Saga-based;
- compensation does not rely on distributed database rollback;
- Human Task claim must admit only one effective claimant;
- claim operation must be atomic;
- escalation is policy/SLA driven;
- Workflow Engine consumes SLA policy but does not define it.

### 3.4 L4-A-015 Exact Traceability

| L4 BH | A05-003 Material Technical Contribution |
|---|---|
| `L4-A-015-BH-001` | Runtime admission validates Tenant, selected immutable version, initiation authority, execution context and input contract before instance creation. |
| `L4-A-015-BH-002` | Creates one durable Workflow Instance bound to the selected version and realizes `Created → Running`. |
| `L4-A-015-BH-003` | Establishes workflow-owned `WorkflowStarted` publication intent when governed execution begins. |
| `L4-A-015-BH-004` | Runtime Coordinator executes only version-defined steps/routes/conditions and never directly mutates participating-domain state. |
| `L4-A-015-BH-005` | Domain Invocation Adapter obtains authoritative owner result before runtime progression. |
| `L4-A-015-BH-006` | Wait subsystem persists `Waiting` state/condition correlation and resumes only on valid source-defined condition. |
| `L4-A-015-BH-007` | Suspension subsystem preserves same instance/version and admits governed resumption only. |
| `L4-A-015-BH-008` | Completion subsystem realizes `Completed`, Workflow Output Contract and `WorkflowCompleted` publication handoff. |
| `L4-A-015-BH-009` | Cancellation subsystem realizes authorized `Cancelled` and `WorkflowCancelled` without assuming business rollback. |
| `L4-A-015-BH-010` | Failure boundary moves normal progression into A016 recovery architecture and publishes `WorkflowFailed`. |
| `L4-A-015-BH-011` | Runtime evidence preserves Tenant, version, actor, transitions, domain outcomes, waits, authorization and terminal result. |

### 3.5 L4-A-016 Exact Traceability

| L4 BH | A05-003 Material Technical Contribution |
|---|---|
| `L4-A-016-BH-001` | Failure classifier distinguishes runtime/infrastructure failure from source-domain business failure. |
| `L4-A-016-BH-002` | Failed transition persists owner-local state and workflow-owned `WorkflowFailed` publication intent. |
| `L4-A-016-BH-003` | Recovery context preserves Tenant, instance, originating version, correlation and completed authoritative domain outcomes. |
| `L4-A-016-BH-004` | Recovery Eligibility Gate checks source-defined version/policy/failure-category/current-state eligibility. |
| `L4-A-016-BH-005` | Recovery authorization is re-evaluated through D02 before `Failed → Recovering`. |
| `L4-A-016-BH-006` | Recovery requires a valid continuation/checkpoint position and forbids version substitution or domain-result rewrite. |
| `L4-A-016-BH-007` | Successful recovery resumes `Recovering → Running` and emits only source-supported recovery/resumption events. |
| `L4-A-016-BH-008` | Authorized non-resumable recovery can terminate in `Cancelled` without inventing another terminal state. |
| `L4-A-016-BH-009` | Recovery engine rejects attempts to resume `Completed` or `Cancelled`. |
| `L4-A-016-BH-010` | Recovery evidence includes category, checkpoint/continuation, authority, events and final status. |

### 3.6 L4-A-058 Exact Traceability

| L4 BH | A05-003 Material Technical Contribution |
|---|---|
| `L4-A-058-BH-001` | Creates A05-owned Human Task linked to Workflow Instance, workflow node and Task-Owning Domain. |
| `L4-A-058-BH-002` | Candidate Resolver consumes D01/D02 identity/responsibility/authorization facts without taking candidate ownership. |
| `L4-A-058-BH-003` | Valid candidate resolution permits Human Task `Available`. |
| `L4-A-058-BH-004` | Atomic Claim Boundary ensures one effective eligible claimant and realizes `Claimed`. |
| `L4-A-058-BH-005` | Current-Claimant Guard permits `In Progress` only for the valid current claimant under current authorization. |
| `L4-A-058-BH-006` | Task Decision Adapter records only the domain-owned decision result; A05 never derives substantive business decision. |
| `L4-A-058-BH-007` | Valid completion persists `Completed` and makes authoritative domain-owned result available to workflow continuation. |
| `L4-A-058-BH-008` | SLA/Escalation Boundary applies source-defined escalation rather than silently completing overdue work. |
| `L4-A-058-BH-009` | Re-open-to-Available escalation preserves prior task/escalation history rather than rewriting it. |
| `L4-A-058-BH-010` | Task evidence preserves workflow context, candidate basis, claimant, domain result and escalation outcome. |

### 3.7 Human Task Event Naming Gap

L4-A-058 records an unresolved upstream naming conflict between:

```text
WorkflowTaskCreated / WorkflowTaskAssigned /
WorkflowTaskClaimed / WorkflowTaskCompleted /
WorkflowTaskEscalated
```

and:

```text
TaskCreated / TaskAssigned /
TaskClaimed / TaskCompleted /
TaskEscalated
```

A05-003 shall **not** select one family as canonical until repository governance resolves the nomenclature.

Technical task state transitions do not require inventing a third event family.

### 3.8 Retry Configuration Constraint

Locked L2-A05 permits policy-driven retry configuration including:

- Maximum Attempts;
- Retry Interval;
- Exponential Backoff;
- Immediate Retry;
- Scheduled Retry.

A05-003 models these as policy-resolved runtime parameters.

It does not choose numeric values.

### 3.9 Timeout Constraint

Locked L2-A05 recognizes timeout categories:

- Node timeout;
- Human Task timeout;
- External service timeout;
- Workflow timeout.

Timeout occurrence is a runtime fact.

The business consequence of the timeout remains source-policy/workflow-definition governed.

### 3.10 Compensation Constraint

Locked L2-A05 establishes Saga-based compensation supporting:

- node compensation;
- sub-workflow compensation;
- parent workflow compensation;
- external integration compensation.

Compensation shall not rely on distributed database rollback.

This is treated as an upstream architectural constraint.

A05-003 does not invent the business compensating command or outcome.

### 3.11 Recovery Authority Gap

L4-A016 leaves unresolved:

- universal recovery initiator/approver;
- terminal-resolution authority;
- complete generic recovery contract.

A05-003 therefore defines technical recovery admission and execution mechanics only.

It does not name those business actors.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 A05-003 Owns

- Workflow Instance runtime;
- runtime execution position;
- workflow token coordination;
- Human Task technical lifecycle;
- candidate/claim technical state;
- timer records;
- timeout detection;
- SLA/escalation execution mechanics;
- runtime checkpoints;
- technical retry scheduling;
- recovery continuation mechanics;
- Saga compensation sequencing;
- runtime cancellation mechanics;
- runtime variables;
- runtime history;
- owner-local concurrency;
- workflow runtime event handoff;
- runtime evidence/telemetry.

### 4.2 A05-003 Does Not Own

- business-domain decision;
- policy definitions;
- authorization decision;
- workflow definition/version;
- identity;
- notification delivery;
- business compensation command semantics;
- business retry permission;
- source-domain persistence;
- cross-owner transaction;
- event transport;
- audit storage.

### 4.3 A05-001 Boundary

A05-001 is the family architecture baseline.

A05-003 conforms to its:

- orchestration-only ownership;
- durable execution;
- no cross-owner write;
- C01/C02 integration;
- D02 authorization;
- Tenant isolation;
- owner-local transaction boundaries.

### 4.4 A05-002 Boundary

A05-003 consumes:

- one exact immutable Workflow Version;
- validated node/connector structure;
- contract/event/policy references;
- version lineage;
- optional migration admission result.

Runtime never silently edits or upgrades the version.

### 4.5 D02 Boundary

D02 owns:

- candidate authorization;
- task-action authorization;
- cancellation/recovery authorization;
- SoD/maker-checker;
- privileged workflow administration;
- policy evaluation.

A05 runtime consumes D02 decisions.

Task assignment/claim is not permission creation.

### 4.6 D01 Boundary

D01 owns human and non-human principal identity/authentication.

A05 stores identity references only.

A05 does not create a Person/User Account because a task needs a claimant.

### 4.7 B02 Boundary

B02 provides owner-local persistence, concurrency and transaction patterns.

A05-003 owns the workflow runtime data.

A05 runtime writes only A05-owned state.

### 4.8 C01 Boundary

C01 supplies synchronous contract mechanics, timeout/idempotency semantics and provider-result boundaries.

A05 decides when the immutable Workflow Version requires the call.

The provider domain decides the business result.

### 4.9 C02 Boundary

C02 transports workflow events and consumed external events.

A05 remains semantic publisher of workflow-owned events.

C02 does not own Workflow Instance or Human Task lifecycle.

### 4.10 A08 Boundary

A05 owns workflow-specific scheduling semantics.

A08 may provide generic technical background execution.

A05 defines:

- due condition;
- owning Workflow Instance;
- timer identity;
- workflow action made eligible by timer.

A08, if used, executes generic technical work and must not mutate workflow state outside A05 contracts.

### 4.11 C04 Boundary

Escalation may require notifications.

A05 emits the source-governed trigger/context.

C04 owns notification delivery.

A05 does not send notification channels directly.

---

## 5. Solution Architecture

### 5.1 Runtime Component Model

```text
Immutable Workflow Version
        ↓
Instance Admission
        ↓
Workflow Instance Store
        ↓
Runtime Coordinator
 ┌──────────────────────────────────────────────┐
 │ Execution Position / Token Coordinator       │
 │ Runtime Context Manager                      │
 │ Domain Invocation Adapter                    │
 │ Event Wait Correlator                        │
 │ Human Task Manager                           │
 │ Timer / Timeout Manager                      │
 │ SLA / Escalation Coordinator                 │
 │ Checkpoint Manager                           │
 │ Retry Coordinator                            │
 │ Recovery Coordinator                         │
 │ Compensation Coordinator                     │
 │ Cancellation Coordinator                     │
 │ Workflow Event Publication Handoff           │
 │ Evidence / Telemetry Handoff                 │
 └──────────────────────────────────────────────┘
        ↓
A05-owned durable runtime persistence
```

### 5.2 Conceptual Workflow Instance Runtime Model

```text
WorkflowInstanceRuntime
{
  tenant_reference
  workflow_instance_reference
  workflow_definition_reference
  workflow_version_reference

  lifecycle_state
  execution_position_reference
  active_token_reference(s)

  runtime_context_reference
  runtime_variables
  waiting_condition_reference(s)

  active_task_reference(s)
  active_timer_reference(s)

  latest_valid_checkpoint_reference
  retry_context_reference
  recovery_context_reference
  compensation_context_reference

  initiating_subject_reference
  current_actor / technical_executor_reference

  correlation_reference
  causation_reference
  business_reference(s)

  concurrency_version
  evidence_reference
}
```

This is conceptual.

No physical schema is prescribed.

### 5.3 Runtime Lifecycle

A05-003 realizes only source-defined lifecycle states.

For normal execution:

```text
Created
  ↓
Running
  ↔ Waiting
  ↔ Suspended
  ↓
Completed

or

Running / Waiting / Suspended
  ↓
Cancelled

or

active execution
  ↓
Failed
  ↓
Recovering
  ├─→ Running
  └─→ Cancelled
```

No new canonical workflow lifecycle state is added.

### 5.4 Instance Admission

A Workflow Instance is created only when:

- Tenant is valid;
- exact Workflow Version is eligible;
- input contract is valid;
- initiating context is valid;
- protected initiation authorization exists where required;
- idempotency/concurrency checks permit one authoritative start.

Invalid admission shall not create a running instance.

### 5.5 Created → Running

The owner-local start transaction may include:

- Workflow Instance record;
- originating version binding;
- initial execution position;
- runtime context;
- `Created → Running`;
- durable publication intent for `WorkflowStarted`.

The transaction excludes participating-domain state.

### 5.6 Execution Position

The Runtime Coordinator tracks a technical execution position sufficient to continue from durable state.

The position can reference:

- current node;
- active branch/token;
- prior completed orchestration steps;
- pending task/timer/event;
- last authoritative domain result reference;
- checkpoint.

Execution position is workflow runtime state.

It is not business state.

### 5.7 Workflow Tokens

Where the immutable version permits concurrent branches, A05 may use technical Workflow Tokens to represent independent active execution paths.

Token mechanics shall:

- remain inside A05;
- retain Tenant/version/instance identity;
- avoid duplicating domain commands accidentally;
- merge only under version-defined orchestration semantics.

No specific token algorithm is prescribed.

### 5.8 Runtime Variables

Runtime variables may hold:

- orchestration control values;
- correlation values;
- source-result references;
- values explicitly returned through workflow/domain contracts.

They shall not become shadow canonical business records.

A source-domain fact that can change authoritatively should be re-resolved when current truth is required.

### 5.9 Domain Invocation

For each domain action:

```text
runtime step
   ↓
resolve provider contract
   ↓
D02 authorization where required
   ↓
C01 request
   ↓
provider domain performs business behavior
   ↓
authoritative result
   ↓
runtime records result reference/outcome
   ↓
continue only along immutable-version path
```

### 5.10 Domain Rejection

A business rejection is not an infrastructure retry condition.

The runtime may route it only if the Workflow Version defines a path for that authoritative outcome.

A05 shall not convert `NotEligible`, `Rejected` or equivalent source result into a retryable technical failure.

### 5.11 Wait Architecture

A wait is an A05-owned durable runtime condition.

Conceptually:

```text
WorkflowWaitContext
{
  instance_reference
  workflow_version_reference
  execution_position
  wait_type
  expected_source_reference
  correlation_reference
  tenant_reference
  created_at
  satisfaction_reference
}
```

Possible wait types include:

- canonical event;
- Human Task;
- time condition;
- authoritative domain callback/result;
- source-governed external completion.

### 5.12 Waiting

If the source-defined condition is unsatisfied:

- instance remains `Waiting`;
- execution position is persisted;
- no busy loop is required;
- no business decision is invented.

A valid condition later re-admits the same instance/version.

### 5.13 Event Wait Correlation

Event-driven continuation requires:

- correct Tenant;
- expected canonical event identity;
- compatible event contract;
- workflow correlation;
- expected business/workflow reference;
- event eligibility for current execution position.

Wrong-Tenant, stale, duplicate or unrelated events shall not advance the instance.

### 5.14 Duplicate Event Input

If the same event is delivered repeatedly, only the first valid state-changing application may advance the workflow.

Subsequent duplicates must be idempotently ignored or treated as already applied according to the runtime contract.

### 5.15 Suspension

Authorized suspension persists:

- same instance;
- same originating version;
- current execution position;
- current tasks/timers/waits according to source behavior.

Suspension does not create a successor instance.

### 5.16 Resumption

Resumption requires:

- current lifecycle permits it;
- required authorization exists;
- same originating version is retained;
- continuation position remains valid.

A successor Activated version is irrelevant unless a separately governed migration succeeds.

### 5.17 Human Task Creation

When a human-decision node is reached, A05 creates one Human Task associated with:

- Tenant;
- Workflow Instance;
- Workflow Version;
- node/step;
- Task-Owning Domain;
- required substantive decision reference;
- candidate-resolution context;
- SLA/escalation policy references where applicable;
- correlation/evidence.

A05 does not make the substantive business decision.

### 5.18 Human Task Technical Model

```text
HumanTask
{
  task_reference
  tenant_reference

  workflow_instance_reference
  workflow_version_reference
  node_reference

  task_owning_domain_reference
  decision_contract_reference

  lifecycle_state
  candidate_context_reference
  current_claimant_reference

  sla_policy_reference
  escalation_policy_reference
  due / timer_reference(s)

  task_result_reference
  escalation_history_reference(s)

  concurrency_version
  correlation_reference
}
```

### 5.19 Human Task Lifecycle

For A058 behavior, the relevant task progression is:

```text
Created
  ↓
Available
  ↓
Claimed
  ↓
In Progress
  ↓
Completed
```

Additional source-governed cancellation/escalation transitions may apply.

A05-003 does not create an `Assigned` canonical lifecycle state because L4-A058 explicitly records that assignment may be metadata/result rather than a lifecycle state.

### 5.20 Candidate Resolution

Candidate resolution is delegated to canonical identity/authorization architecture.

A05 supplies:

- task;
- Tenant/scope;
- required responsibility/Permission context;
- workflow/process/resource context.

D01/D02 return authoritative candidate/access facts.

A05 stores candidate-resolution provenance but does not own candidate identity or Role hierarchy.

### 5.21 Available Task

A Task becomes `Available` only after valid candidate resolution.

If no eligible candidates exist:

- the Task remains outside `Completed`;
- runtime follows only source-governed continuation/escalation behavior;
- A05 shall not invent a fallback claimant.

### 5.22 Atomic Claim

A valid claim requires:

- Task is claimable;
- claimant is eligible;
- claimant is currently authorized;
- one effective claimant wins the concurrency race.

The claim update shall be atomic at the owner-local Task boundary.

Conceptually:

```text
Task state = Available
AND expected task version = V
AND claimant eligible
        ↓ atomic compare/update
Task state = Claimed
current claimant = X
version = V+1
```

Exact locking primitive is B02 implementation detail.

### 5.23 Competing Claims

If two actors claim concurrently:

- exactly one may become effective;
- loser receives a concurrency/claim conflict outcome;
- no duplicate Task is created;
- no second effective claimant is recorded.

### 5.24 Claimed → In Progress

Only the valid current claimant under current authorization context may progress the task.

A stale claimant reference or expired authority shall not preserve execution rights.

### 5.25 Task Decision

The Task-Owning Domain owns the decision.

The workflow may:

- invoke a decision contract;
- accept a domain-approved decision outcome;
- store the result reference;
- route according to immutable Workflow Version.

It may not calculate the business answer itself.

### 5.26 Task Completion

Valid completion requires:

- correct current claimant where source behavior requires claim;
- current authorization;
- valid domain-owned decision result;
- task still in eligible state;
- concurrency check succeeds.

Then:

- Task becomes `Completed`;
- result is preserved;
- workflow continuation receives the result;
- task history remains immutable.

### 5.27 Human Task Event Naming

Runtime may create publication intents for source-defined task lifecycle events only after the repository resolves which naming family is canonical.

Until then:

- internal lifecycle/evidence remains valid;
- no new event name is invented;
- implementations shall not publish both families as if both were canonical unless governance explicitly defines equivalence.

### 5.28 Timer Architecture

A workflow timer is an A05-owned durable due-condition record.

Conceptually:

```text
WorkflowTimer
{
  timer_reference
  tenant_reference
  workflow_instance_reference
  workflow_version_reference
  node / task_reference
  timer_type
  due_condition
  policy_reference
  lifecycle / processing metadata
  correlation_reference
}
```

The exact storage/scheduler mechanism is not prescribed.

### 5.29 Timer Types

The runtime may require time-based handling for:

- workflow wait;
- node timeout;
- Human Task timeout;
- external service timeout;
- workflow timeout;
- retry schedule;
- SLA warning/escalation;
- source-defined delayed execution.

Timer existence and semantic consequence must come from the immutable Workflow Version or source-governed policy.

### 5.30 Governed Time Source

Timer decisions use a governed server/platform time source.

Client device time is not authoritative for:

- task due;
- timeout;
- escalation;
- retry eligibility;
- workflow completion.

Exact clock technology is outside scope.

### 5.31 Timer Durability

A timer must survive process/node restart.

A timer shall not exist only as in-memory sleep state for long-running workflow correctness.

If generic execution is delegated to A08, the authoritative timer semantics/state remain A05-owned.

### 5.32 Timer Firing

A timer firing is a technical wake-up signal.

Before state progression, A05 revalidates:

- timer is still active/current;
- instance/task is still in expected state;
- Tenant/version match;
- due condition actually holds;
- no earlier completion/cancellation invalidated it;
- applicable policy is current where required.

A stale timer shall not advance an already completed/cancelled task/instance.

### 5.33 Timer Idempotency

Repeated timer delivery for the same due occurrence shall produce at most one effective runtime transition.

### 5.34 Timeout Detection

Timeout manager records a timeout fact when an applicable source-defined limit is crossed.

A timeout fact does not itself define the business consequence.

The immutable Workflow Version / policy determines whether to:

- retry;
- escalate;
- remain waiting;
- fail;
- cancel;
- choose an alternate source-defined path.

### 5.35 SLA Architecture

Each Human Task may reference an SLA policy.

L2-A05 supports attributes including:

- response time;
- completion time;
- warning threshold;
- escalation threshold;
- business calendar.

A05 consumes the policy.

It does not define the SLA.

### 5.36 Business Calendar Boundary

If SLA evaluation depends on working hours/business calendar:

- A05 consumes an authoritative calendar/policy source;
- A05 does not invent institution holidays or working-day rules;
- cached calendar facts must be fresh enough for the decision.

### 5.37 Escalation Detection

Escalation occurs only when a source-governed SLA/policy condition becomes conclusive before completion.

Examples from locked L2 include:

- SLA expiration;
- response timeout;
- approval delay;
- organizational escalation;
- administrative escalation.

These are categories, not hard-coded universal rules.

### 5.38 Escalation Actions

Source policy may permit actions such as:

- notification trigger;
- task delegation/reassignment;
- reopening candidate handling;
- alternate workflow path.

A05 executes only the source-defined action.

It does not invent a supervisor or administrator.

### 5.39 Notification During Escalation

Where escalation requires notification:

```text
A05 escalation fact/context
        ↓
C04 Notification & Communication
        ↓
delivery
```

A05 does not directly send email/SMS/push/WhatsApp.

### 5.40 Re-open to Available

Where source-governed escalation returns the Task to `Available`:

- prior claim/in-progress/escalation history is preserved;
- candidate resolution is re-run as required;
- the previous history is not overwritten;
- the Task is not marked Completed.

### 5.41 Escalation and Authorization

Reassignment/delegation does not automatically grant Permission.

The newly eligible actor must still be authorized through D02.

### 5.42 Checkpoint Architecture

A checkpoint is an A05-owned durable execution snapshot/reference sufficient to support safe continuation.

Conceptually:

```text
WorkflowCheckpoint
{
  checkpoint_reference
  tenant_reference
  workflow_instance_reference
  workflow_version_reference
  execution_position
  completed_step_references[]
  authoritative_domain_outcome_references[]
  pending_task / timer / wait references[]
  runtime_context_reference
  created_at
  correlation_reference
}
```

### 5.43 Checkpoint Placement

Locked L2-A05 identifies checkpoint creation around sensitive boundaries including:

- before Human Tasks;
- after transaction commits;
- before external integrations;
- before compensation boundaries.

Checkpoint frequency remains policy driven.

A05-003 does not invent numeric frequency.

### 5.44 Checkpoint Integrity

A valid checkpoint shall:

- belong to the same Tenant;
- belong to the same Workflow Instance/version;
- correspond to a valid prior execution position;
- preserve completed source-domain outcomes;
- not contain rewritten business state;
- be durably persisted before being relied on for recovery.

### 5.45 Recovery Entry

Recovery starts only from canonical `Failed`.

`Completed` and `Cancelled` are terminal for A016 recovery.

### 5.46 Failure Classification

The runtime distinguishes at least:

- technical execution failure;
- dependency/transport failure;
- timeout;
- concurrency conflict;
- invalid/stale runtime context;
- source-domain business rejection/failure.

A source-domain business failure remains domain-owned.

### 5.47 Failed Transition

When the source-defined failure transition succeeds:

- persist `Failed`;
- preserve instance/version/correlation;
- preserve completed domain outcomes;
- record failure category;
- create `WorkflowFailed` publication intent;
- stop normal A015 progression.

### 5.48 Recovery Eligibility

Before `Failed → Recovering`, validate:

- current state is Failed;
- originating version permits applicable recovery;
- failure category is recoverable;
- source policy permits recovery;
- valid continuation/checkpoint exists;
- D02 authorization outcome exists where required.

### 5.49 Recovering

During `Recovering`:

- original Workflow Instance is preserved;
- original Workflow Version is preserved;
- completed domain outcomes remain authoritative;
- continuation starts only at a valid source-supported position;
- no predecessor business action is silently replayed unless safe and permitted.

### 5.50 Recovery Success

Successful recovery may transition:

```text
Recovering → Running
```

and publish only source-supported recovery/resumption events whose conditions are actually satisfied.

No invented recovery event is emitted.

### 5.51 Recovery Cancellation

If governed recovery cannot resume and authorized cancellation exists:

```text
Recovering → Cancelled
```

with `WorkflowCancelled`.

If neither resume nor cancellation authority exists, preserve the unresolved valid lifecycle position.

### 5.52 Technical Retry Architecture

Retry is a technical execution mechanism, not business-policy ownership.

A technical retry is appropriate only when:

- source/contract permits retry;
- repeated execution is safe or idempotent;
- business outcome is not already conclusively known;
- retry policy authorizes another attempt;
- current runtime state remains compatible.

### 5.53 Retry Context

Conceptually:

```text
RetryContext
{
  operation_reference
  workflow_instance_reference
  attempt_metadata
  retry_policy_reference
  previous_failure_category
  next_eligible_time
  idempotency_reference
  authoritative_outcome_status
}
```

No numeric attempt limit is defined here.

### 5.54 Retry Policy

The Retry Coordinator consumes source-configured policy for:

- maximum attempts;
- retry interval;
- exponential backoff;
- immediate retry;
- scheduled retry.

The architecture supports these forms without choosing values.

### 5.55 Unknown Outcome Safety

If a remote request outcome is ambiguous:

- A05 shall not blindly invoke the business action again;
- C01 idempotency/outcome-recovery semantics must determine whether safe retry is possible;
- authoritative provider outcome may need resolution first.

### 5.56 Retry Exhaustion

When retry policy is exhausted:

- do not invent additional attempts;
- classify/fail according to source workflow/recovery behavior;
- preserve evidence of attempts/failure;
- do not synthesize business success.

### 5.57 Retry vs Recovery

Retry and recovery are distinct:

```text
Retry
= repeat a retry-safe technical operation within governed policy

Recovery
= move a failed Workflow Instance through source-defined
  Failed → Recovering → Running/Cancelled behavior
```

A retry may occur during normal execution or recovery depending on source policy.

### 5.58 Saga Compensation Architecture

Locked L2-A05 requires Saga-based compensation.

At L5, this means:

- each participating owner commits its own transaction;
- A05 coordinates source-defined compensating actions;
- no distributed database rollback spans owners;
- compensation progression is durable and attributable;
- a compensation failure does not erase the original completed outcome.

### 5.59 Compensation Eligibility

Compensation occurs only when the Workflow Version / recovery policy identifies a compensatable completed activity and the owning domain provides an approved compensating capability.

A05 does not infer reversibility.

### 5.60 Compensation Scope

Compensation may coordinate:

- node compensation;
- sub-workflow compensation;
- parent workflow compensation;
- external integration compensation.

The exact business effect remains owned by the provider of the compensating capability.

### 5.61 Compensation Invocation

```text
A05 identifies source-defined compensatable activity
        ↓
resolve compensation contract
        ↓
D02 authorization where required
        ↓
C01 / approved asynchronous invocation
        ↓
owning domain executes compensation
        ↓
authoritative compensation result
        ↓
A05 records progress and chooses only source-defined continuation
```

### 5.62 Compensation Is Not Rollback

A05 shall never:

- open a global transaction across domains;
- directly undo provider rows;
- pretend a committed business event never happened;
- rewrite domain audit history.

Compensation creates new authoritative domain behavior where the domain supports it.

### 5.63 Compensation Ordering

Where multiple completed actions require compensation, order is determined by the immutable Workflow Version/source recovery policy.

A05 may coordinate reverse order or another explicit sequence only when defined.

No universal order is invented here.

### 5.64 Compensation Failure

If compensation fails:

- preserve original completed outcomes;
- preserve attempted compensation result;
- follow source-defined retry/recovery/escalation path;
- do not mark compensation successful;
- do not invent a new business correction.

### 5.65 Compensation Idempotency

Repeated technical delivery of one compensation request must not create duplicate business reversal where the provider contract requires idempotency.

C01/provider contract defines safe duplicate semantics.

### 5.66 Cancellation

Cancellation of a Workflow Instance is an A05 lifecycle action.

It does not mean:

- every Human Task was Completed;
- every business transaction was reversed;
- every external action was undone.

Any compensating business action must be explicitly source-defined.

### 5.67 Cancellation and Active Tasks

On authorized workflow cancellation, A05 may make active A05-owned tasks/timers no longer executable according to source runtime semantics.

It shall preserve their history/evidence.

It shall not delete completed or historical task records merely because the workflow was cancelled.

### 5.68 Runtime Concurrency

Potential concurrent operations include:

- two Task claims;
- Task completion and escalation timer;
- workflow cancellation and domain callback;
- timer and event continuation;
- retry completion and recovery action;
- duplicate event delivery;
- two recovery requests.

A05 runtime must produce one valid authoritative owner-local result.

### 5.69 Optimistic Concurrency

A compliant design may use owner-local version/compare-and-set semantics constrained by B02/ADR-L5-0006.

Exact database locking primitive is not prescribed.

### 5.70 Task Completion vs Escalation Race

If task completion and escalation become concurrent:

- state/version checks determine which valid operation commits first;
- the second operation revalidates current state;
- a completed task shall not subsequently be escalated by a stale timer;
- an escalation that reopens/reroutes the task invalidates stale completion assumptions.

### 5.71 Cancellation vs External Result Race

If an external result arrives concurrently with cancellation:

- both operations validate expected current workflow position/version;
- one owner-local transition wins according to source semantics;
- stale delivery cannot resurrect a Cancelled instance.

### 5.72 Event Publication

Workflow-owned runtime events use owner-local state-to-publication intent:

```text
runtime state transition
        +
canonical workflow event publication intent
        ↓ one A05 owner-local transaction
commit
        ↓
C02 durable publication / delivery
```

A C02 delivery failure shall not roll back a committed A05 runtime transition.

### 5.73 Runtime Events

A05 may publish only canonical source-supported workflow events such as:

- `WorkflowStarted`;
- `WorkflowCompleted`;
- `WorkflowCancelled`;
- `WorkflowFailed`;
- source-supported `WorkflowCheckpointCreated`;
- `WorkflowRecovered`;
- `WorkflowResumed`.

Human Task event names remain unresolved as noted in §3.7.

### 5.74 Runtime Evidence

A05 runtime evidence shall preserve references to:

- Tenant;
- Workflow Definition/Version/Instance;
- actors/executors;
- state transitions;
- task lifecycle;
- claim winner;
- candidate-resolution basis;
- waits/timers;
- SLA/escalation;
- domain outcomes;
- authorization decisions;
- retries;
- checkpoints;
- recovery;
- compensation;
- terminal outcome;
- workflow-owned events.

D04 owns evidence storage/retention.

### 5.75 Runtime Observability

Operational telemetry may include:

- Running/Waiting/Suspended/Failed/Recovering counts;
- task backlog;
- task age;
- claim conflicts;
- timer backlog;
- timer lag;
- timeout counts;
- SLA warning/breach counts;
- escalation counts;
- retry attempts/exhaustion;
- recovery attempts/outcomes;
- compensation attempts/outcomes;
- dependency latency/failure;
- duplicate input detections;
- concurrency conflicts.

Telemetry is read-only with respect to canonical workflow state.

### 5.76 Runtime Privacy

Workflow runtime shall minimize copied business payload.

Where references suffice, store references.

Sensitive decision payload/evidence access remains policy-controlled.

Credentials/secrets must never be stored in workflow variables, task metadata, timers, retry context or checkpoints.

### 5.77 Failure Model

| Failure / Condition | Required Runtime Response |
|---|---|
| invalid start context | Do not create running instance. |
| duplicate start | Resolve idempotently; avoid duplicate instance. |
| domain business rejection | Preserve domain result; follow only version-defined route. |
| dependency unavailable | Preserve runtime position; apply retry/failure policy without fabricating outcome. |
| task has no eligible candidate | Do not complete; follow source-governed escalation/continuation. |
| competing task claims | Exactly one effective claimant. |
| stale task claimant | Re-authorize/revalidate; reject stale action. |
| stale task escalation timer | No effect if task state no longer eligible. |
| wrong-Tenant event/timer/task request | Reject/fail closed. |
| duplicate event | At most one effective state transition. |
| timer processing duplicated | At most one effective state transition. |
| retry unsafe | Do not retry. |
| retry exhausted | Enter source-defined failure/recovery path. |
| failure unclassified | Do not return to Running. |
| recovery authorization denied | Remain outside Recovering/Running. |
| checkpoint invalid | Do not resume from it. |
| provider outcome ambiguous | Resolve safely before replaying side effect. |
| compensation unsupported | Do not invent reversal. |
| compensation fails | Preserve outcomes and follow source-defined recovery. |
| cancellation unauthorized | Preserve current valid state. |
| publication transport failure | Preserve committed runtime state and durable publication intent. |
| persistence failure | Do not claim transition success. |
| concurrency conflict | Re-read/revalidate; do not overwrite authoritative state. |

---

## 6. Architecture & Technology Decisions

Only A05-003-owned technical choices are recorded.

| Decision ID | Decision | Rationale / Authority |
|---|---|---|
| `AD-001` | A05 runtime is a durable stateful coordinator whose authoritative state is owner-local and version-bound rather than process-memory-bound. | L2 WEP-05/WEP-06 + L4-A015/A016. |
| `AD-002` | Human Task claim uses owner-local atomic concurrency so one eligible claimant becomes effective. | L4-A058-BH-004 + locked L2 claim requirement. |
| `AD-003` | Workflow timers are durable A05-owned semantic records; generic wake-up/background execution may be delegated to A08 without transferring workflow lifecycle ownership. | Frozen A05 responsibility + ownership safety. |
| `AD-004` | Timer firing, event delivery, retry delivery and other wake-up signals are treated as revalidated triggers, not direct authority to mutate runtime state. | Prevents stale/duplicate progression. |
| `AD-005` | Runtime recovery resumes only from a valid version-bound continuation/checkpoint position and never reconstructs by rewriting completed domain outcomes. | L4-A016. |
| `AD-006` | Technical retry is separated from business retry and is allowed only where the provider/workflow contract proves retry safety or idempotency. | Ownership + C01 idempotency semantics. |
| `AD-007` | Compensation uses the locked L2 Saga-based pattern: owner-local transactions plus source-defined compensating actions, never distributed database rollback. | Locked L2-A05 §56. |
| `AD-008` | Runtime state transition and required workflow-event publication intent use owner-local atomic handoff. | ADR-L5-0007. |
| `AD-009` | No runtime engine, scheduler, lock service, queue, database, retry library or numeric policy value is selected here. | Technology governance / vendor independence. |

---

## 7. Security, Privacy, Quality & Operational Impact

### 7.1 Authorization

D02 authorization is re-evaluated at protected execution points, including:

- task claim;
- task start;
- task completion;
- workflow cancellation;
- suspension/resumption;
- recovery;
- privileged runtime administration;
- protected compensation where required.

An Allow at workflow initiation is not a permanent Allow for later actions.

### 7.2 Tenant Isolation

Every runtime object is Tenant-bound:

- Workflow Instance;
- Human Task;
- Timer;
- Checkpoint;
- Retry Context;
- Recovery Context;
- Compensation Context.

Cross-Tenant correlation is prohibited.

### 7.3 Actor Attribution

Human and machine execution retain actual actor/executor identity.

A05 does not collapse a machine worker into a human task claimant.

### 7.4 Privacy

Human Task context, workflow variables and checkpoints must avoid unnecessary sensitive payload duplication.

### 7.5 Reliability

Durability, checkpointing, idempotency, concurrency control and safe retry are the core runtime reliability mechanisms.

### 7.6 Operational Safety

Operational tools may inspect/runtime-administer only through governed privileged contracts.

Direct persistence mutation is prohibited as normal runtime administration.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Requirement |
|---|---|
| `CR-001` | Every Workflow Instance shall remain bound to one originating immutable Workflow Version unless a separately governed migration succeeds. |
| `CR-002` | A05 runtime shall not directly mutate participating-domain business state. |
| `CR-003` | A domain-owned decision/result shall be consumed as authoritative orchestration input and shall not be re-derived by A05. |
| `CR-004` | Long-running Workflow Instance state required for continuation shall be durably persisted. |
| `CR-005` | Waiting execution shall not resume until the exact source-defined condition is validly satisfied. |
| `CR-006` | Suspension/resumption shall preserve the same Workflow Instance and originating version. |
| `CR-007` | Workflow completion/cancellation/failure transitions shall create only source-supported workflow-event publication intents. |
| `CR-008` | Runtime state and required workflow-event publication intent shall follow owner-local atomic handoff. |
| `CR-009` | Human Tasks shall remain A05-owned runtime objects while identity, candidate authority and authorization remain D01/D02-owned. |
| `CR-010` | Candidate resolution shall not transfer Person/Role/Permission ownership into A05. |
| `CR-011` | A Human Task shall reach `Available` only after valid candidate resolution where A058 applies. |
| `CR-012` | Task claim shall admit at most one effective eligible claimant through owner-local atomic concurrency control. |
| `CR-013` | Only the valid current claimant under current authorization shall progress a claimed task to protected work/completion. |
| `CR-014` | A05 shall not derive the substantive business decision represented by a Human Task. |
| `CR-015` | Task completion shall preserve the authoritative Task-Owning Domain decision result for workflow continuation. |
| `CR-016` | Escalation shall occur only through applicable source-governed SLA/policy behavior and shall not silently mark an overdue task Completed. |
| `CR-017` | Re-opening a task through escalation shall preserve prior claim/work/escalation history. |
| `CR-018` | A05 shall not invent a canonical Human Task event family while the upstream naming conflict remains unresolved. |
| `CR-019` | Workflow timers shall be durable and Tenant/version/instance bound. |
| `CR-020` | A timer wake-up shall revalidate current runtime/task state before applying an effective transition. |
| `CR-021` | Duplicate timer delivery shall not produce duplicate effective transitions. |
| `CR-022` | Timeout consequence shall be resolved from immutable workflow/policy semantics rather than hard-coded globally by A05. |
| `CR-023` | SLA attributes and business-calendar rules shall be consumed from authoritative policy/calendar sources rather than invented by A05. |
| `CR-024` | Runtime checkpoints shall preserve the originating version, execution position and completed authoritative domain outcomes. |
| `CR-025` | Recovery shall not resume from an invalid or incompatible checkpoint/continuation position. |
| `CR-026` | `Failed → Recovering` shall require source-defined eligibility and required current authorization. |
| `CR-027` | Recovery shall never resume a `Completed` or `Cancelled` instance. |
| `CR-028` | Recovery shall not silently substitute a successor Workflow Version. |
| `CR-029` | Technical retry shall not be used for a conclusive source-domain business rejection. |
| `CR-030` | Side-effecting technical retry shall require safe retry/idempotency semantics from the applicable contract. |
| `CR-031` | Retry policy values shall remain externalized and shall not be embedded as universal numeric constants in A05-003. |
| `CR-032` | Retry exhaustion shall not fabricate success or unauthorized further attempts. |
| `CR-033` | Compensation shall use source-defined compensating capabilities and shall not directly reverse another owner's persistence. |
| `CR-034` | Compensation shall not rely on a distributed database rollback across canonical owners. |
| `CR-035` | Compensation failure shall preserve original completed outcomes and the failed compensation attempt. |
| `CR-036` | Workflow cancellation shall not be interpreted as automatic business rollback. |
| `CR-037` | Duplicate/replayed event, task, timer, retry or compensation inputs shall be idempotently constrained to at most one effective transition/effect under the applicable contract. |
| `CR-038` | Concurrent runtime operations shall use owner-local concurrency control and shall not overwrite newer authoritative state. |
| `CR-039` | Wrong-Tenant runtime inputs shall be rejected. |
| `CR-040` | Credentials/secrets shall not be stored in workflow variables, Human Tasks, timers, retry contexts or checkpoints. |
| `CR-041` | Runtime telemetry shall remain a projection and shall not mutate canonical workflow state. |
| `CR-042` | A05 shall not deliver notifications directly; escalation notification delivery shall use C04. |
| `CR-043` | Generic A08 background execution shall not become canonical owner of workflow timer/task/retry lifecycle. |
| `CR-044` | No workflow runtime product, scheduler, lock service, queue, database, retry library, cache or numeric policy target shall be inferred as canonical from this specification. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| `OD-001` | Canonical Human Task event naming family remains inconsistent upstream. | Upstream nomenclature | Blocking for canonical task-event publication names; non-blocking for internal runtime lifecycle. | Repository governance must select/formally relate the naming families. |
| `OD-002` | Exact runtime engine / durable-execution product. | Technology | Non-blocking for product-neutral architecture; blocking before product lock. | Apply L5 Technology Selection & ADR Methodology. |
| `OD-003` | Exact timer/scheduler implementation and A05↔A08 technical execution contract. | A05/A08 implementation | Non-blocking for architecture; blocking before shared scheduler implementation. | Define without transferring timer semantics to A08. |
| `OD-004` | Exact task/runtime persistence schema and concurrency primitives. | B02/A05 implementation | Non-blocking. | Resolve under B02/ADR-L5-0006. |
| `OD-005` | Exact checkpoint persistence format and checkpoint compaction/retention strategy. | Runtime implementation | Non-blocking. | Define under B02/D04/operational governance. |
| `OD-006` | Universal recovery initiator/approver and terminal-resolution authority remain undefined. | Upstream authority gap | Feature-blocking for concrete recovery UI/workflow requiring named actors. | Do not invent in A05. |
| `OD-007` | Exact generic recovery request/response contract. | A05/C01 | Non-blocking for conceptual architecture; blocking before generic recovery API. | Define after source authority is resolved. |
| `OD-008` | Exact retry policy values and business-specific retryability rules. | Policy/domain | Non-blocking. | Source workflow/domain policy supplies them. |
| `OD-009` | Exact SLA policy values/business-calendar source for each process. | Policy/domain | Non-blocking framework; process-specific. | Resolve through canonical policy/calendar owners. |
| `OD-010` | Exact compensation commands/order for each business process. | Business-domain policy | Non-blocking framework; feature-specific. | Workflow-owning domain + participating domain define source-approved compensation semantics. |
| `OD-011` | Whether runtime realization requires a dedicated A05 ADR after engine/scheduler/state alternatives are evaluated. | ADR governance | Non-blocking now. | Apply ADR-necessity test before normative technology/pattern lock. |
| `OD-012` | Exact runtime state archival/retention behavior beyond A015/A016 terminal boundary. | A05/D04/records governance | Non-blocking for execution architecture. | Resolve under later retention/archive architecture without redefining terminal execution semantics. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-A-05-003` realizes:

> Runtime task/timer/escalation/technical compensation mechanics without inventing business recovery policy.

### 10.2 Primary L4 Runtime Realization

**L4-A-015**

- `BH-001` initiation admission;
- `BH-002` instance/version binding;
- `BH-003` WorkflowStarted;
- `BH-004` version-defined orchestration;
- `BH-005` authoritative domain outcomes;
- `BH-006` Waiting;
- `BH-007` Suspended/resume;
- `BH-008` Completed/output;
- `BH-009` Cancelled;
- `BH-010` Failed boundary;
- `BH-011` runtime evidence.

### 10.3 Primary Recovery Realization

**L4-A-016**

- `BH-001` failure classification;
- `BH-002` Failed + WorkflowFailed;
- `BH-003` preserve Tenant/version/outcomes;
- `BH-004` recovery eligibility;
- `BH-005` recovery authorization;
- `BH-006` valid continuation position;
- `BH-007` Running recovery;
- `BH-008` recovery cancellation;
- `BH-009` terminal-state prohibition;
- `BH-010` recovery evidence.

### 10.4 Primary Human Task Realization

**L4-A-058**

- `BH-001` Human Task creation;
- `BH-002` candidate resolution;
- `BH-003` Available;
- `BH-004` atomic Claim;
- `BH-005` current claimant / In Progress;
- `BH-006` domain-owned decision;
- `BH-007` Completed / continuation result;
- `BH-008` escalation;
- `BH-009` re-open history;
- `BH-010` evidence.

### 10.5 Locked L2 Runtime Mechanics

A05-003 technically realizes locked L2-A05 ownership of:

- checkpoints;
- retry management;
- timeout management;
- SLA/escalation coordination;
- workflow scheduling;
- Human Task lifecycle;
- Saga-based compensation.

### 10.6 Cross-L5 Runtime Flow

```text
A05-002
immutable activated Workflow Version
        ↓
A05-003 Runtime
        ├── D01 identity / technical principal
        ├── D02 authorization / task candidate / SoD
        ├── C01 domain calls
        ├── C02 event delivery
        ├── A08 generic background execution where used
        ├── C04 escalation notification delivery
        ├── B02 persistence/concurrency
        └── D04 evidence
        ↓
Completed / Cancelled
or governed Failed → Recovering outcome
```

### 10.7 Applicable Approved ADRs

- `ADR-L5-0003`
- `ADR-L5-0004`
- `ADR-L5-0006`
- `ADR-L5-0007`
- `ADR-L5-0009`
- `ADR-L5-0010`
- `ADR-L5-0011`

### 10.8 Stage-3 Family Completion

```text
L5-A-05-001
Workflow & Orchestration Architecture Baseline
        ↓
L5-A-05-002
Workflow Definition, Versioning & Policy Resolution Architecture
        ↓
L5-A-05-003
Workflow Runtime, Task, Timer, Escalation & Compensation Architecture
```

Together these three specifications establish Devang's Stage-3 Workflow & Orchestration architecture family.

### 10.9 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-A/
            └── L5-A-05-003_Workflow_Runtime_Task_Timer_Escalation_and_Compensation_Architecture.md
```

No `L5-A/L5-A-05/` major-area subfolder is introduced.
