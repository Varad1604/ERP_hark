# Workflow Runtime, Task, Timer, Escalation & Compensation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-05-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-A — Core Platform & Application Architecture`  
**Major Area:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Canonical Technical Responsibility:** Runtime task/timer/escalation/technical compensation mechanics without inventing business recovery policy.  
**Technical Architecture Owner:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Assigned Author:** Devang  
**Version:** `1.2.1`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105 v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-A-05-001_Workflow_and_Orchestration_Architecture_Baseline.md`; `L5-A-05-002_Workflow_Definition_Versioning_and_Policy_Resolution_Architecture.md`; `L5-D-02-001` through `L5-D-02-007`; `L5-C-01-001` through `L5-C-01-003`; `L5-C-02-001` through `L5-C-02-003`; `L5-A-04-001/002`; `L5-B-02-002/003`; applicable `L5-D-04`; applicable `L5-A-08`; `L2-A05_Workflow_Engine_Workflows.md`; `L4-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md`; `L4-A-016_Workflow_Execution_Failure_to_Controlled_Recovery_Outcome.md`; `L4-A-058_Human_Decision_Requirement_to_Completed_or_Escalated_Task_Resolution.md`; supporting `L4-A-017`.  
**Referenced By:** workflow-consuming business domains; workflow administration; human-task experiences; background/timer execution; operational monitoring; D04 evidence; E01/E05 resilience and operations architecture.  
**Applicable ADRs:** `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0007`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0023`; `ADR-L5-0041`; `ADR-L5-0042`. Flowable OSS 8.x is the canonical workflow runtime; A08 Rust workers/JetStream own generic background execution; exact runtime persistence schema and numeric operational values remain separately governed.
**Governance Closure:** Stage-3 final closure authorizes this specification as architecture-complete while preserving unresolved upstream event nomenclature and recovery-actor authority as explicit non-L5 decisions. This specification shall not choose or invent those semantics.

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
- universal recovery initiator/approver identity;
- terminal-resolution authority;
- canonical resolution of the upstream Human Task event-name conflict;
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

### 3.7 Human Task Event Naming Boundary

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

Stage-3 closure does not resolve this upstream nomenclature conflict. A05-003 is architecture-complete without selecting one family because event-name ownership is upstream. Until repository governance resolves the conflict, A05 implementations and downstream specifications shall reference the source-defined semantic event by canonical source reference rather than inventing a third name or declaring either conflicting family canonical.

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

### 3.11 Recovery Authority Boundary

L4-A016 leaves unresolved:

- universal recovery initiator/approver;
- terminal-resolution authority;
- complete generic recovery contract.

Stage-3 closure classifies these as upstream business/governance authorities, not missing A05 technical architecture. A05-003 therefore closes its L5 responsibility by requiring a valid D02/source-governed authorization result before protected recovery action while refusing to invent the actor or authority. Any later upstream decision shall be consumed through normal impact review without reopening A05 technical ownership.

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

A05 decides when the immutable Workflow Version requires the call but does not redefine the owner contract.

### 4.9 C02 Boundary

C02 owns event transport, durable delivery, subscription, schema/version compatibility and replay mechanics.

A05 remains semantic publisher of A05-owned workflow events and preserves source-defined event identity.

### 4.10 D04 Boundary

D04 owns audit/evidence architecture. A05 supplies attributable workflow runtime evidence and does not become audit-storage owner.

### 4.11 A08 Boundary

A08 may execute generic technical background/timer work. A05 retains workflow timer meaning, Workflow Instance state and business/workflow continuation semantics. A08 receives only generic execution requests and returns technical status/evidence.

---

## 5. Solution Architecture

### 5.1 Runtime Control Model

The runtime architecture shall provide logical control points for:

- instance admission;
- execution coordination;
- durable wait/resume;
- human-task creation/claim/completion;
- timer registration/firing;
- escalation execution;
- checkpoint creation;
- technical retry scheduling;
- recovery eligibility/authorization;
- compensation sequencing;
- cancellation;
- runtime evidence and publication handoff.

These are logical responsibilities, not mandated deployable services.

### 5.2 Runtime Persistence

A05-owned runtime state is durable and owner-local. Transaction/concurrency realization consumes B02 patterns. Cross-owner distributed transactions are prohibited.

### 5.3 Human Task Claim Concurrency

Claim shall be atomic or semantically equivalent under B02 concurrency architecture. At most one effective claimant may own a claim at a time. Stale claimants cannot complete a task after their claim/authorization is invalidated.

### 5.4 Timer and Background Execution

Workflow timer meaning remains A05-owned. Where A08 is used, A05 submits generic technical execution with stable workflow/timer reference and release condition; A08 returns technical outcome only. A08 shall not transition Workflow Instance state directly.

### 5.5 Recovery

Recovery requires:

1. durable Failed state and attributable failure evidence;
2. preserved Tenant/instance/version/correlation context;
3. valid continuation/checkpoint position;
4. current recovery eligibility under source policy;
5. D02/source-governed authorization for protected recovery action;
6. preservation of already-completed authoritative domain outcomes;
7. no silent Workflow Version substitution.

### 5.6 Compensation

Compensation is technical orchestration of source-defined compensating actions. A05 sequences compensation but does not invent compensating business commands/outcomes or assume database rollback.

---

## 6. Failure Model

| Failure | Required response |
|---|---|
| invalid runtime context/version | reject before instance progression |
| task claim conflict | one effective claimant; stale claim rejected |
| timer/background executor failure | preserve durable wait/timer context; recover through governed A08/A05 boundary |
| technical retry exhaustion | preserve explicit technical failure; do not fabricate business outcome |
| recovery unauthorized/ineligible | remain Failed/contained; do not progress |
| checkpoint invalid | no recovery continuation from invalid checkpoint |
| compensation action failure | preserve failure/evidence and follow source-defined compensation policy |
| event transport unavailable after durable intent | retain owner-local publication intent; C02 owns delivery recovery |

---

## 7. Conformance Requirements

| ID | Requirement |
|---|---|
| CR-001 | Workflow Instances execute one immutable Workflow Version unless a source-governed migration succeeds. |
| CR-002 | A05 mutates only A05-owned workflow runtime state. |
| CR-003 | Human Task claims are concurrency-safe and admit at most one effective claimant. |
| CR-004 | D02/source-governed authorization is re-evaluated for protected task/recovery/cancellation actions. |
| CR-005 | Workflow waits/timers are durable and preserve Tenant/instance/version/correlation context. |
| CR-006 | Technical retry never becomes business retry policy. |
| CR-007 | Recovery never rewrites completed authoritative domain outcomes or silently substitutes Workflow Version. |
| CR-008 | Compensation sequencing invokes source-defined compensating behavior and does not rely on distributed database rollback. |
| CR-009 | A08 may execute generic technical work but never owns Workflow Instance state/timer meaning. |
| CR-010 | Workflow-owned events use durable publication handoff and C02 delivery semantics. |
| CR-011 | Runtime evidence remains attributable to Tenant, Workflow Instance, Workflow Version, actor/executor, authorization and outcome. |
| CR-012 | Upstream event-name or business-authority gaps are not resolved by inventing L5 semantics. |

---

## 8. Architecture Decisions

**AD-001:** A05 runtime is durable, owner-local and contract-driven; cross-owner direct writes are prohibited.  
**AD-002:** Technical retry/recovery mechanics do not transfer business retry/recovery authority into A05.  
**AD-003:** A05↔A08 interaction uses generic technical execution while preserving A05 workflow semantics.  
**AD-004:** Stage-3 closure does not resolve upstream Human Task event nomenclature or universal recovery actor/authority; those remain source-governed and are consumed when canonically resolved.

---

## 9. Open / Deferred Design

| Item ID | Open / Deferred Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| `OD-001` | **Resolved:** Flowable OSS 8.x. | Technology | Locked | `ADR-L5-0023`; runtime behavior refined by `ADR-L5-0042`. |
| `OD-002` | **Resolved boundary:** Flowable owns workflow timers; A08 owns recurring technical/background scheduling. | Technical implementation | Locked | `ADR-L5-0041`; `ADR-L5-0042`. |
| `OD-003` | Exact runtime persistence schema/indexes | Implementation design | No | Downstream technical design under B02 invariants. |
| `OD-004` | Numeric retry/timeout/SLA/checkpoint values | Operational configuration | No | Governed profiles/policies; not universal L5 constants. |
| `OD-005` | Human Task event naming conflict in upstream L4 | Upstream nomenclature governance | **No for A05/Stage-3 closure** | Preserve source semantic reference and consume later canonical naming decision. |
| `OD-006` | Universal recovery initiator/approver and terminal-resolution authority | Upstream business/governance authority | **No for A05/Stage-3 closure** | Require source/D02 authorization result; do not invent actor/authority. |

No deferred item transfers ownership, changes lifecycle semantics, or blocks Stage-3 architecture closure.

## 2026-09-22 Backend Readiness Group 4 Synchronization

**Applicable locked decision:** `ADR-L5-0042`  
Flowable stores orchestration/runtime state only; canonical business state remains source-owned. Human task assignment is not authorization and D02 re-evaluates authority at action time. Workflow timers stay A05-owned, running instances are version-pinned by default, migration is explicit, and compensation invokes only owner-provided actions.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Removed the obsolete claim that no runtime/scheduler product was selected. Flowable, A08 Rust workers and the timer/scheduling responsibility split now follow `ADR-L5-0023`, `ADR-L5-0041` and `ADR-L5-0042`; unresolved upstream event nomenclature/actor authority remains intentionally preserved.
