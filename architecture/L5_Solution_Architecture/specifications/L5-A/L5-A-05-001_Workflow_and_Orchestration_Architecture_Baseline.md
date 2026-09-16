# Workflow & Orchestration Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-A-05-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-A — Core Platform & Application Architecture`  
**Major Area:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Canonical Technical Responsibility:** Reusable workflow/orchestration capability; business domains retain workflow rules and decisions.  
**Technical Architecture Owner:** `L5-A-05 — Workflow & Orchestration Architecture`  
**Assigned Author:** Devang  
**Version:** `1.0.0`  
**Status:** `Review`  
**Last Updated:** `2026-09-12`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105 v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; L5 Dependency-Gated Parallel Authoring Execution Timeline; `L5_Devang_Stage2_Architecture_Remediation_and_Approval_Decision_2026-09-12.md`; `L5-D-02-001` through `L5-D-02-007`; `L5-C-01-001` through `L5-C-01-003`; `L5-C-02-001` through `L5-C-02-003`; `L5-A-04-001/002`; `L5-B-02-002/003`; `L5-D-01-002/003`; `L5-D-03-001`; applicable `L5-D-04`; `L2-A05_Workflow_Engine_Workflows.md`; `L4-A-014`; `L4-A-015`; `L4-A-016`; `L4-A-017`.  
**Referenced By:** `L5-A-05-002`; `L5-A-05-003`; business-domain workflow consumers; `L5-A-08`; `L5-C-01`; `L5-C-02`; `L5-D-02`; `L5-D-04`; operational/observability architecture.  
**Applicable ADRs:** `ADR-L5-0003` Contract-Driven Modular Platform with Selective Isolation; `ADR-L5-0004` Tenant and Institution Isolation Enforcement Pattern; `ADR-L5-0006` Owner-Local Transaction and Optimistic Concurrency Strategy; `ADR-L5-0007` Atomic State-to-Publication Intent Handoff Pattern; `ADR-L5-0009` Non-Human Principal Authorization Binding Pattern; `ADR-L5-0010` Durable Event Delivery, Subscription, Ordering & Recovery Pattern; `ADR-L5-0011` Event Contract Versioning & Consumer Compatibility Pattern. No A05-specific workflow engine/runtime product ADR is allocated by this specification.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the solution-wide L5 baseline for SutraERP workflow and orchestration.

It translates the locked Workflow Engine responsibility into a reusable technical architecture that can:

- execute approved workflow definitions;
- coordinate long-running business processes;
- route work without owning business decisions;
- invoke authoritative domain capabilities through governed contracts;
- preserve Tenant, identity, authorization, correlation and workflow-version context;
- persist workflow execution state durably;
- publish canonical Workflow Engine events without transferring semantic ownership to messaging infrastructure;
- survive technical failures without rewriting completed business outcomes;
- support human and automated execution without duplicating IAM or authorization;
- remain product-neutral and deployment-neutral.

The baseline establishes the family-wide execution model.

Detailed specialization is intentionally split:

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

### 2.2 Core Architectural Position

```text
Business Domain
owns business rule / decision / state / outcome
        ↓ governed orchestration request
C01 Contract Boundary
        ↓
A05 Workflow & Orchestration
owns workflow definition/runtime/orchestration state
        ↓
D02 Authorization
        ↓
owning domain command / decision contract
        ↓
authoritative domain result
        ↓
A05 routing / wait / continuation
        ↓
workflow-owned lifecycle outcome
        ↓
C02 event distribution + D04 evidence
```

A05 is an orchestration capability.

It is **not** a generic business-rules engine and is **not** a canonical owner of the business processes it coordinates.

### 2.3 In Scope

This baseline owns or constrains:

- workflow/orchestration family boundaries;
- Workflow Definition / Version / Instance / Runtime technical separation;
- workflow initiation boundary;
- workflow execution context;
- Tenant and Institution context propagation;
- human and technical-principal attribution;
- authorization handoff at protected workflow actions;
- synchronous domain-call orchestration through C01;
- asynchronous workflow event handoff through C02;
- durable workflow runtime-state ownership;
- workflow-version binding;
- workflow-instance state isolation;
- workflow task coordination at baseline level;
- waiting/suspension/resumption semantics at baseline level;
- runtime checkpoints;
- failure/recovery architectural boundary;
- technical retries versus business retries;
- cancellation boundary;
- cross-domain transaction boundary;
- technical compensation boundary;
- idempotency and duplicate-initiation boundary;
- concurrency ownership;
- workflow telemetry/evidence handoff;
- long-running execution;
- extension points;
- deployment neutrality;
- product neutrality.

### 2.4 Out of Scope / Non-Goals

This baseline does not own:

- business rules;
- business decisions;
- business eligibility;
- business state machines;
- business approval authority;
- source-domain commands/events;
- institutional policy content;
- Role/Permission/SoD semantics;
- authentication/session/credential internals;
- Notification delivery;
- document/media storage;
- business-domain persistence;
- canonical event transport mechanics;
- event-contract schema/versioning;
- exact Workflow Definition schema;
- exact task/timer/escalation mechanics;
- exact compensation algorithm;
- exact workflow migration mechanics;
- UI workflow designer;
- work-inbox screen design;
- dashboard/navigation design;
- BPMN, DMN, BPEL or another notation selection;
- a workflow-engine product;
- a scheduler product;
- a database product;
- a message broker;
- a programming language/runtime;
- numeric retry counts, timer precision, retention durations or performance targets.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Responsibility

The frozen 105-specification corpus defines `L5-A-05-001` as:

> Reusable workflow/orchestration capability; business domains retain workflow rules and decisions.

This baseline therefore owns **technical orchestration mechanics**, not business meaning.

### 3.2 Locked L2-A05 Constraints

The following locked L2-A05 facts are treated as upstream constraints rather than local A05 decisions:

- Workflow Engine owns Workflow Definitions, Versions, Instances, Runtime, Tokens, Tasks, execution state, scheduling, checkpoints, compensation, timeout/retry management, execution history and orchestration.
- Business domains own business rules, decisions, state machines, commands, events and business outcomes.
- Business domains shall not implement independent workflow runtimes.
- Published Workflow Versions are immutable.
- Running Workflow Instances remain bound to their originating Workflow Version unless an explicitly governed migration succeeds.
- Workflow execution is durable.
- Workflow execution remains Tenant-isolated.
- Workflow execution integrates through published commands, APIs, events and approved platform contracts.
- Workflow Engine does not deliver Notifications.
- Workflow Engine does not own Person, Identity, Account, Role, Permission or Authorization Decision.
- Workflow Engine stores references to business/domain resources rather than taking ownership of those resources.

### 3.3 Stage-3 Dependency Gate

The locked execution timeline requires:

```text
L5-A-05
minimum dependency
=
D02 + C01
```

The canonical Stage-2 remediation decision dated 2026-09-12 establishes D02 as Architecture Approved and ready for Stage-3 consumption.

This baseline consumes D02 as authoritative L5 peer architecture for:

- human and technical-principal authority;
- Permission/capability evaluation;
- Tenant/Institution/resource scope;
- centralized policy decisions;
- Delegation/Temporary Access;
- privileged access;
- SoD/maker-checker/emergency controls.

C01 supplies the ownership-preserving synchronous interaction boundary.

### 3.4 Exact L4 → L5 Baseline Traceability

This baseline contributes materially to the following L4 BHs.

Detailed realization is refined by A05-002 and A05-003.

| L4 BH | A05-001 Material Technical Contribution | Detailed Owner |
|---|---|---|
| `L4-A-014-BH-001` | Definition admission requires resolved owning domain, Tenant and business-process boundary before technical workflow definition handling. | A05-002 |
| `L4-A-014-BH-003` | Defines strict orchestration-reference boundary so business rules/decisions/state/events remain source-owned. | A05-002 |
| `L4-A-014-BH-004` | Establishes mandatory structural/dependency/policy/event/compatibility validation boundary before technical publication. | A05-002 |
| `L4-A-014-BH-006` | Requires immutable published version realization and workflow-owned publication handoff. | A05-002 |
| `L4-A-014-BH-008` | Makes an Activated version resolvable for new Workflow Instances and exposes governed input/output contracts. | A05-002 |
| `L4-A-014-BH-009` | Separates definition activation from instance execution. | A05-001/002 |
| `L4-A-015-BH-001` | Defines initiation admission: Tenant, version, authority, context and input contract must be valid. | A05-001/003 |
| `L4-A-015-BH-002` | Defines durable Workflow Instance creation bound to immutable version. | A05-003 |
| `L4-A-015-BH-004` | Enforces orchestration-only ownership and prohibits direct mutation of participating-domain business state. | A05-001/003 |
| `L4-A-015-BH-005` | Requires authoritative owning-domain result through governed contract before workflow continues. | A05-001/003 |
| `L4-A-015-BH-006` | Establishes durable Waiting boundary and governed resumption input. | A05-003 |
| `L4-A-015-BH-007` | Establishes governed suspension/resumption boundary without version substitution. | A05-003 |
| `L4-A-015-BH-008` | Defines runtime completion boundary and Workflow Output Contract handoff. | A05-003 |
| `L4-A-015-BH-009` | Defines authorized cancellation boundary. | A05-003 |
| `L4-A-015-BH-010` | Separates normal execution from failure-recovery architecture. | A05-003 |
| `L4-A-015-BH-011` | Requires correlated workflow execution evidence and authorization provenance. | A05-001/003 |
| `L4-A-016-BH-001` | Separates workflow technical failure classification from domain-owned business failure. | A05-003 |
| `L4-A-016-BH-003` | Requires Tenant/instance/version/correlation and completed-domain outcome preservation across recovery. | A05-001/003 |
| `L4-A-016-BH-004` | Requires source-defined recovery eligibility before technical continuation. | A05-003 |
| `L4-A-016-BH-005` | Requires D02 authorization before Failed → Recovering. | A05-003 |
| `L4-A-016-BH-006` | Requires a valid continuation position and prohibits rewriting completed domain outcomes or silently changing version. | A05-003 |
| `L4-A-016-BH-009` | Prevents terminal Completed/Cancelled instances from being resumed through recovery. | A05-003 |
| `L4-A-016-BH-010` | Requires attributable recovery evidence. | A05-003 |
| `L4-A-017-BH-001` | Requires immutable published versions and independent successor Drafts. | A05-002 |
| `L4-A-017-BH-002` | Preserves owning-domain business semantics while versioning only orchestration structure/references. | A05-002 |
| `L4-A-017-BH-003` | Requires successor validation before approval. | A05-002 |
| `L4-A-017-BH-005` | Realizes immutable published successor version. | A05-002 |
| `L4-A-017-BH-006` | Makes Activated successor available only to new instances by default. | A05-002 |
| `L4-A-017-BH-007` | Preserves existing instance → originating version binding. | A05-001/002 |
| `L4-A-017-BH-008` | Defines technical migration gate without inventing approval authority. | A05-002/003 |
| `L4-A-017-BH-009` | Unsafe/unauthorized migration leaves instance on originating version. | A05-002/003 |
| `L4-A-017-BH-010` | Prohibits invented migration lifecycle state/event. | A05-001/002 |

### 3.5 Canonical Workflow Events Consumed from L4

A05 does not invent workflow business events.

Where exact upstream conditions are satisfied, the Workflow Engine remains semantic publisher of source-defined workflow events including:

- `WorkflowDefinitionCreated`;
- `WorkflowDraftUpdated`;
- `WorkflowVersionPublished`;
- `WorkflowVersionActivated`;
- source-governed `WorkflowVersionRetired`;
- `WorkflowStarted`;
- `WorkflowCompleted`;
- `WorkflowCancelled`;
- `WorkflowFailed`;
- source-governed recovery events such as `WorkflowCheckpointCreated`, `WorkflowRecovered`, `WorkflowResumed`.

C02 distributes these events but does not become their semantic publisher.

### 3.6 Explicit Assumptions

No material business assumption is required for the baseline.

The following implementation questions remain deliberately unresolved:

- exact workflow notation/definition format;
- exact workflow execution product;
- exact runtime persistence product;
- exact timer/scheduler implementation;
- exact worker-execution topology;
- exact human-task inbox implementation;
- exact workflow migration representation;
- exact compensation execution technology.

They are not silently assumed.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 A05-001 Owns

This baseline owns the family-wide architecture for:

- Workflow Engine technical bounded context;
- orchestration-only execution principle;
- definition/runtime separation;
- version-bound execution;
- workflow instance technical identity;
- workflow execution context categories;
- cross-owner orchestration boundary;
- workflow runtime-state ownership;
- transaction isolation principle;
- domain-result consumption boundary;
- authorization integration;
- event publication handoff;
- durability and checkpoint principle;
- execution evidence/correlation;
- extension boundaries;
- family decomposition into A05-002/A05-003.

### 4.2 A05-001 Does Not Own

- detailed definition/version lifecycle realization — A05-002;
- detailed task/timer/escalation/compensation runtime — A05-003;
- identity/authentication — D01;
- authorization/policy/SoD — D02;
- trust/cryptography/secrets — D03;
- audit storage/evidence model — D04;
- synchronous-contract infrastructure — C01;
- event delivery/schema infrastructure — C02;
- domain persistence — B02 owner-specific;
- generic background-job infrastructure — A08;
- notification delivery — C04;
- business rules/decisions/state/events — owning business domains.

### 4.3 Cross-L5 Collaboration Matrix

| L5 Owner | Owns | A05 Consumption / Boundary |
|---|---|---|
| `L5-A-04` | Tenant/Institution isolation/context | A05 execution context carries validated scope but does not redefine it. |
| `L5-B-02` | Persistence ownership, transactions, concurrency | A05 persists only A05-owned workflow runtime state; no cross-owner direct write. |
| `L5-C-01` | synchronous contract boundaries | A05 invokes participating domains through governed owner contracts. |
| `L5-C-02` | event publication/delivery/schema mechanics | A05 remains semantic publisher of workflow-owned events; C02 distributes them. |
| `L5-C-04` | notifications/communications | A05 may emit source-defined notification trigger/handoff; C04 owns delivery. |
| `L5-D-01` | human/non-human identity/authentication context | A05 records attributable actor/executor references. |
| `L5-D-02` | authorization/policy/Permission/scope/SoD/privilege | A05 requests authorization decisions before protected workflow operations. |
| `L5-D-03` | trust, cryptography, secrets | A05 consumes protected technical boundaries and credentials without owning them. |
| `L5-D-04` | audit/evidence | A05 supplies correlated workflow provenance/evidence. |
| `L5-A-08` | generic background/scheduled job execution | A05 retains workflow timer/job semantics; A08 may execute generic technical work under A05-owned intent. |
| `L5-E-01` | resilience | A05 runtime must survive failure without changing business semantics. |
| `L5-E-05` | observability/operations | A05 exposes runtime health/lag/failure telemetry without making telemetry authoritative state. |

### 4.4 Business-Domain Boundary

A participating business domain owns:

- whether a business command is valid;
- whether a business rule passes;
- whether a business state transition occurs;
- business transaction data;
- business outcome;
- business event.

A05 owns:

- which orchestration step is current;
- which Workflow Version defines the path;
- which external result is awaited;
- whether the workflow is Running/Waiting/Suspended/Failed/etc. according to upstream workflow lifecycle;
- task/timer/checkpoint/runtime state;
- orchestration progression after authoritative outcomes are supplied.

### 4.5 No Cross-Owner Write Authority

A05 shall never:

- directly update Finance, Examination, HR, Admissions, Library or other domain persistence;
- emulate a domain command by writing domain rows;
- directly change another owner's business state;
- treat workflow runtime state as canonical business state.

Cross-owner mutation occurs only through the owning capability's governed contract.

---

## 5. Solution Architecture

### 5.1 Logical Architecture

```text
                    Workflow-Owning Domain
                            │
                            │ governed start/change request
                            ▼
                C01 Contract Admission Boundary
                            │
                            ▼
                 Workflow Orchestration Boundary
        ┌────────────────────────────────────────────┐
        │ Definition / Version Resolver              │
        │ Instance Admission                         │
        │ Runtime Coordinator                        │
        │ Execution Context                          │
        │ Task / Wait / Timer Coordination           │
        │ Domain Invocation Adapter                  │
        │ Authorization Adapter                      │
        │ Runtime State / Checkpoint Boundary        │
        │ Workflow Event Publication Handoff         │
        │ Evidence / Telemetry Handoff               │
        └────────────────────────────────────────────┘
             │             │              │
             ▼             ▼              ▼
         D02 Auth       C01 Domain      B02 A05-owned
         Decision       Contracts       runtime state
             │             │              │
             └──────┬──────┘              │
                    ▼                     │
              Participating Domain        │
                    │                     │
                    └── authoritative outcome
                               │
                               ▼
                        Runtime Coordinator
                               │
                  ┌────────────┴────────────┐
                  ▼                         ▼
              C02 Workflow Event        D04 / E05
              Distribution              Evidence/Telemetry
```

### 5.2 Baseline Technical Components

| Component | Responsibility | Must Not Own |
|---|---|---|
| Workflow Definition Boundary | Accepts/locates governed definition/version artifacts for runtime use. | Business rules or policy content. |
| Workflow Version Resolver | Resolves the exact immutable version for definition activation or new execution. | Business-version authority. |
| Instance Admission Boundary | Validates Tenant, version, input contract, context and initiating authority before instance creation. | Authentication or authorization internals. |
| Workflow Instance Registry | Maintains A05-owned instance identity and originating-version relationship. | Business record identity. |
| Runtime Coordinator | Advances workflow-owned execution positions based only on definition structure and authoritative outcomes. | Domain decisions. |
| Execution Context Manager | Preserves Tenant, Institution, actor/executor, correlation, causation and business-reference context. | Canonical identity/scope ownership. |
| Authorization Adapter | Requests/consumes D02 decisions for protected workflow actions. | Role/Permission/policy logic. |
| Domain Invocation Adapter | Invokes owner contracts and captures authoritative outcomes. | Participating-domain state. |
| Wait/Resume Boundary | Persists source-defined wait position and resumes only on valid governed condition. | Inventing business conditions. |
| Runtime Persistence Boundary | Durably persists only A05-owned runtime state/checkpoints. | Other owner persistence. |
| Workflow Event Publication Handoff | Publishes workflow-owned facts through B02/C02 governed publication architecture. | Event transport ownership. |
| Evidence Handoff | Supplies attributable, correlated workflow evidence to D04. | Audit storage. |
| Telemetry Handoff | Supplies runtime metrics/traces/health signals to E05. | Canonical workflow truth. |

### 5.3 Definition vs Runtime Separation

The baseline requires a hard conceptual boundary:

```text
Workflow Definition / Version
design-time orchestration artifact
        ↓ activation/resolution
Workflow Instance
runtime execution artifact
```

A definition/version can exist without starting an instance.

A Workflow Instance can execute only against an eligible immutable Workflow Version.

A05-002 owns the detailed definition/version architecture.

A05-003 owns detailed runtime behavior.

### 5.4 Workflow Version Binding

Each Workflow Instance is bound to exactly one originating Workflow Version at initiation.

The binding is preserved through:

- Running;
- Waiting;
- Suspended;
- Failed;
- Recovering;
- resumption;
- completion/cancellation;

unless a separately governed migration satisfies the source-defined migration safeguards.

Activation of a successor version does not silently move existing instances.

### 5.5 Workflow Execution Context

Conceptually:

```text
WorkflowExecutionContext
{
  tenant_reference
  institution / lower_scope_reference(s) where applicable

  workflow_definition_reference
  workflow_version_reference
  workflow_instance_reference

  initiating_subject_reference
  current_actor_reference
  technical_executor_reference where applicable

  correlation_reference
  causation_reference
  business_reference(s)

  authorization_context_reference(s)
  input_contract_reference
  current_execution_position_reference
}
```

This is a conceptual contract, not a prescribed API payload or persistence schema.

### 5.6 Human and Non-Human Execution

Workflow execution can involve:

- human actors;
- service principals;
- workload principals;
- background/job principals;
- integration principals.

Human and non-human actors remain attributable.

For non-human execution, A05 consumes D01 principal identity and D02 Technical Principal Authority Binding governed by `ADR-L5-0009`.

A05 shall not fabricate a Person/Role Assignment for a machine.

### 5.7 Workflow Initiation

Baseline initiation:

```text
Start request
    ↓
C01 contract validation
    ↓
resolve Tenant / Institution context
    ↓
resolve eligible Activated Workflow Version
    ↓
validate Workflow Input Contract
    ↓
resolve initiating actor / technical principal
    ↓
D02 authorization where required
    ↓
create one A05-owned Workflow Instance
    ↓
bind immutable version
    ↓
persist Created / Running progression as source-defined
    ↓
publish WorkflowStarted when exact upstream condition is satisfied
```

Invalid initiation shall not create a Running instance.

### 5.8 Synchronous Domain Invocation

For a domain-owned business action:

```text
A05 runtime step
    ↓
resolve owning capability contract
    ↓
D02 authorization for protected action
    ↓
C01 governed request
    ↓
owning domain validates and executes its business behavior
    ↓
authoritative domain result
    ↓
A05 records result reference / outcome category
    ↓
A05 chooses only the workflow path already defined by the immutable version
```

A05 does not:

- calculate domain eligibility;
- force a domain state transition;
- infer success from transport success;
- reinterpret a business rejection as technical success.

### 5.9 Asynchronous Outcome / Event Participation

A workflow may wait for an upstream-defined event/fact.

The event's semantic publisher remains its source owner.

C02 delivers the event.

A05:

- subscribes only through governed event contracts;
- correlates the event to an eligible waiting instance;
- validates Tenant and required workflow context;
- treats the delivered fact as an orchestration input;
- does not become publisher/owner of the source fact.

### 5.10 Workflow-Owned Event Publication

When exact L4 conditions occur, A05 is the semantic publisher of workflow-owned events.

Technical publication follows:

```text
A05 owner-local workflow state
    +
workflow event publication intent
        ↓
B02 owner-local atomic state/publication handoff
        ↓
C02 publication/delivery
```

`ADR-L5-0007` governs state-to-publication intent consistency.

`ADR-L5-0010` governs durable delivery/subscription/ordering/recovery.

`ADR-L5-0011` governs event contract compatibility.

### 5.11 Transaction Boundary

A Workflow Engine transaction is owner-local.

It may atomically cover:

- A05 runtime-state transition;
- A05-owned checkpoint/state metadata;
- A05-owned workflow event publication intent.

It shall not atomically include:

- participating-domain persistence;
- Finance transactions;
- Examination transactions;
- HR/Admissions business state;
- remote service transaction state.

`ADR-L5-0006` prohibits propagating a caller transaction across canonical owners.

### 5.12 Long-Running Execution

A Workflow Instance may outlive:

- one API request;
- one process;
- one application node;
- one Authentication Session;
- one deployment/restart.

Therefore workflow correctness is based on durable A05 runtime state and authoritative current context, not process memory.

### 5.13 Durable Execution

A05 runtime state required for continuation must survive:

- application restart;
- node failure;
- temporary dependency failure;
- runtime relocation.

Durability does not mean A05 owns downstream business data.

The exact persistence technology remains B02/technology-governed.

### 5.14 Checkpoints

A checkpoint is an A05-owned recoverable execution position.

It may reference:

- workflow version;
- execution position;
- completed orchestration steps;
- authoritative domain outcomes already observed;
- pending wait/task/timer context;
- correlation/provenance.

A checkpoint shall not copy/overwrite canonical business state.

Detailed checkpoint realization belongs A05-003.

### 5.15 Wait Conditions

A Waiting instance remains durable until the source-defined condition is satisfied.

A wait condition can be driven by governed inputs such as:

- authoritative domain response;
- canonical event;
- time condition;
- human-task completion;
- policy-governed continuation input.

A05 does not invent the underlying business condition.

### 5.16 Suspension and Resumption

Suspension:

- preserves instance identity;
- preserves originating version;
- preserves completed domain outcomes;
- stops normal progression according to source behavior.

Resumption continues the same governed instance/version.

It does not restart as a new version unless a separately governed migration/start behavior exists.

### 5.17 Failure Classification Boundary

A05 distinguishes:

```text
Workflow technical/execution failure
vs
Participating-domain business rejection/failure
```

A domain business rejection remains a domain-owned outcome.

A05 may route according to the immutable workflow definition.

It shall not reclassify a business rejection as infrastructure failure merely to retry it.

### 5.18 Controlled Recovery

When an instance reaches upstream-defined `Failed`:

- normal A015 progression stops;
- A016 recovery governs;
- the originating version is preserved;
- completed domain outcomes are preserved;
- D02 authorization is required for protected recovery progression;
- a valid continuation position is required;
- recovery never rewrites completed domain results.

Detailed retry/recovery mechanics belong A05-003.

### 5.19 Technical Retry vs Business Retry

A technical retry is permissible only where it repeats an operation without changing business meaning and the interaction contract supports safe retry/idempotency.

A business retry means attempting the business action again.

A05 shall not convert one into the other.

Examples:

```text
network timeout before outcome known
→ technical retry may be possible through C01-003 semantics

domain says "Not Eligible"
→ business outcome, not a technical retry condition
```

### 5.20 Idempotency

A05 consumes C01-003 idempotency semantics for retryable synchronous requests.

A05-owned instance initiation and transition processing must prevent duplicate technical delivery from producing duplicate workflow instances or duplicate transition effects where the same governed request is being retried.

The exact idempotency-key field/encoding is not defined here.

### 5.21 Concurrency

Concurrent inputs may target the same Workflow Instance:

- duplicate task completion;
- event plus cancellation;
- timer plus human completion;
- recovery plus revocation;
- simultaneous administrative actions.

A05 shall serialize or conditionally validate conflicting owner-local state transitions so one authoritative workflow position results.

The exact optimistic-concurrency realization is B02-governed and constrained by `ADR-L5-0006`.

### 5.22 Compensation Boundary

Compensation is orchestration of already-approved/source-defined compensating actions.

A05 may:

- invoke a source-domain compensation command when the immutable workflow/recovery policy permits;
- track compensation progression;
- preserve evidence.

A05 may not:

- invent a business reversal;
- directly undo another domain's data;
- assume all completed actions are reversible;
- equate database rollback with business compensation.

Detailed compensation mechanics belong A05-003.

### 5.23 Cancellation Boundary

Cancellation requires source-permitted and authorized behavior.

A05 may move its Workflow Instance to upstream-defined `Cancelled` only when the exact governing conditions are satisfied.

A05 cannot infer that cancelling a workflow automatically reverses already-completed domain outcomes.

### 5.24 Authorization Enforcement

A05 uses D02 before protected workflow actions such as:

- start where initiation authority is required;
- human task completion;
- cancellation;
- suspension/resumption;
- recovery;
- privileged administration;
- version migration administration;
- protected domain command invocation.

Workflow assignment alone is not authorization.

Task ownership alone is not authorization.

A05 shall re-evaluate current authorization where the protected operation occurs rather than freezing a permanent Allow at workflow start.

### 5.25 SoD / Maker-Checker

A05 provides workflow/process context to D02-007.

D02 decides whether maker/checker or SoD requirements are satisfied.

A05 does not define:

- which process requires maker-checker;
- who is a valid checker;
- which responsibilities conflict;
- emergency override authority.

When D02 denies, A05 shall not bypass the denial through workflow routing.

### 5.26 Tenant / Institution Isolation

Every Workflow Definition/Version/Instance and execution context is resolved within a Tenant-rooted boundary.

A05 shall not:

- start a Workflow Instance without Tenant context;
- move an instance to another Tenant;
- accept cross-Tenant task completion;
- correlate an event from another Tenant;
- reuse an authorization decision across Tenant boundaries.

Institution/lower-scope context remains subordinate to A04/D02 rules.

### 5.27 Business Reference Handling

A05 stores/references business identifiers required for orchestration.

These references:

- do not transfer ownership;
- do not permit direct business persistence writes;
- must be Tenant/scope compatible;
- are resolved through owning contracts when current authoritative state is needed.

### 5.28 Workflow Variables

Workflow variables are A05-owned runtime values only when they represent orchestration/runtime context.

They shall not become a shadow copy of canonical domain state.

Where current business state is authoritative, A05 resolves it from the owner rather than trusting stale copied variables.

### 5.29 Workflow Output

A Workflow Output Contract may contain:

- workflow status/outcome;
- correlation;
- references to authoritative business results;
- output values explicitly defined by the activated Workflow Version.

A05 shall not fabricate a participating-domain business success merely because the workflow terminal state is `Completed`.

### 5.30 Human Task Baseline

A human task is an A05 runtime coordination object.

It may reference:

- Workflow Instance;
- workflow step;
- candidate/assigned actor or responsibility reference;
- required action;
- due/escalation context;
- domain resource reference.

Task assignment is not a Role Assignment and does not grant Permission.

D02 remains final authorization authority.

Detailed task lifecycle belongs A05-003.

### 5.31 Timer / Scheduling Baseline

Workflow timer semantics belong A05.

Generic background execution infrastructure may be provided by A08.

Boundary:

```text
A05 owns:
what workflow timer exists,
which Workflow Instance it belongs to,
what due condition means,
what transition becomes eligible.

A08 may own:
generic durable background execution mechanism
used to awaken/process the due technical work.
```

A08 shall not become owner of workflow lifecycle.

### 5.32 Notification Boundary

Workflow events or source-defined communication triggers may cause Notifications.

A05 does not send email/SMS/push/WhatsApp directly.

L5-C-04 owns notification/communication technical delivery.

### 5.33 Workflow Extension Points

A05 may expose controlled extension boundaries for:

- domain invocation adapters;
- event wait adapters;
- technical task execution;
- policy/authorization handoff;
- timer execution;
- telemetry/evidence hooks.

Extensions shall not:

- modify Workflow Engine core invariants;
- directly access another owner’s persistence;
- introduce business logic into A05;
- bypass D02;
- bypass Tenant isolation.

### 5.34 Deployment Independence

The Workflow Engine may be:

- co-located within the modular application runtime;
- selectively isolated as a deployable component;
- evolved later according to approved platform/deployment architecture.

Its contract, ownership and state semantics must not change merely because runtime placement changes.

This is constrained by `ADR-L5-0003`.

### 5.35 No Mandatory Workflow Product

This baseline does not decide between:

- a custom runtime;
- BPM/workflow platform;
- orchestration framework;
- state-machine framework;
- durable-execution product.

Any normative repository-wide or family product/runtime choice requires the L5 technology/ADR process.

### 5.36 Failure Model

| Failure / Condition | Baseline Architectural Response |
|---|---|
| Tenant or scope context missing | Reject/fail closed; do not start or advance protected execution. |
| No eligible activated version | Do not create a Running instance. |
| Input contract invalid | Reject initiation before execution. |
| Initiator/actor not authorized | D02 Deny; no protected transition. |
| Participating-domain rejects business action | Preserve domain outcome; route only through defined workflow path. |
| Participating-domain unavailable | Preserve current A05 position; do not fabricate domain outcome. |
| Workflow runtime persistence failure | Do not claim transition success until owner-local durable state is established. |
| Workflow event publication transport failure | Preserve durable publication intent; C02 recovery handles delivery. |
| Duplicate start request | Apply C01/A05 idempotency semantics; no duplicate instance for same governed retry context. |
| Concurrent transition conflict | Revalidate/serialize through B02 owner-local concurrency control. |
| Wait correlation mismatch | Do not resume instance. |
| Event from wrong Tenant | Reject as incompatible execution input. |
| Authorization context stale | Re-evaluate through D02. |
| Workflow Version changed after start | Existing instance remains on originating version unless governed migration succeeds. |
| Technical retry exhausted | Move only through source-defined workflow failure/recovery behavior; do not invent business outcome. |
| Recovery not authorized | Preserve failed/unresolved governed lifecycle position. |
| Compensation unavailable | Preserve completed outcomes and expose source-defined recovery failure; do not fabricate reversal. |

### 5.37 Observability

A05 runtime observability may expose:

- active instance counts;
- state distribution;
- waiting duration;
- task backlog;
- timer backlog;
- transition latency;
- domain-call latency/failure;
- authorization denial count;
- recovery attempts;
- compensation attempts;
- publication lag;
- version distribution;
- stuck-instance diagnostics.

Metrics/logs/traces are operational projections, not canonical workflow state.

Exact SLOs/thresholds are not defined here.

---

## 6. Architecture & Technology Decisions

Only A05-owned technical choices are recorded here.

Upstream business rules and approved peer-ADR constraints are not re-declared as local decisions.

| Decision ID | Decision | Rationale / Authority |
|---|---|---|
| `AD-001` | A05 is one reusable orchestration bounded context with explicit design-time and runtime sub-boundaries rather than domain-specific workflow engines. | Frozen A05 responsibility + locked L2-A05 ownership. |
| `AD-002` | Cross-domain orchestration uses governed owner contracts and authoritative results; A05 never writes another owner’s persistence. | C01 + canonical ownership + ADR-L5-0006. |
| `AD-003` | Workflow Instance state is durable A05-owned state and remains separate from participating-domain business state. | L2-A05 WEP-06/WEP-07 + L4-A-015/A016. |
| `AD-004` | Workflow Instance → originating Workflow Version binding is immutable by default; migration is an explicit separately governed exception. | L4-A-015-BH-002 + L4-A-017-BH-007/008/009. |
| `AD-005` | A05 owner-local state transition and workflow-event publication intent use the canonical state-to-publication handoff rather than a distributed transaction with C02. | ADR-L5-0007. |
| `AD-006` | Human and non-human workflow execution preserve actual actor/executor attribution; machine authority uses D02 Technical Principal Authority Binding. | ADR-L5-0009. |
| `AD-007` | Workflow timer/business-wait semantics remain A05-owned even if generic technical background execution is delegated to A08. | Prevents hidden runtime ownership transfer. |
| `AD-008` | Workflow product/runtime/notation remains unselected in A05-001. | Vendor independence; technology selection requires governed decision if made normative. |

---

## 7. Security, Privacy, Quality & Operational Impact

### 7.1 Security / Trust

- Every protected workflow operation uses D02.
- Tenant scope is mandatory.
- Machine execution is explicitly authorized and attributable.
- Workflow task assignment does not grant Permission.
- Workflow context never substitutes for authentication or authorization.
- Internal placement does not imply trusted authorization.
- Cross-owner service invocation uses C01 and D03-governed trust.
- Secrets/credentials are not persisted as workflow variables.

### 7.2 Privacy

Workflow runtime stores only orchestration information necessary to coordinate the process.

Sensitive domain payloads should remain with canonical owners where references are sufficient.

A05 does not become a shadow data warehouse for business records.

### 7.3 Reliability

Durable owner-local runtime state permits restart/recovery.

A05 does not rely on in-memory state for long-running correctness.

C02 durable delivery supports asynchronous workflow events.

### 7.4 Consistency

- A05 workflow state is strongly governed within the A05 owner-local transaction boundary.
- Cross-domain consistency is coordinated, not distributed-transactional.
- Completed domain outcomes remain authoritative and are not rewritten by workflow rollback.
- Compensation is explicit business-aware orchestration, not implicit database rollback.

### 7.5 Operational Impact

Operations must be able to distinguish:

- active;
- waiting;
- suspended;
- failed;
- recovering;
- terminal workflow instances;

according to upstream lifecycle semantics.

Operational tooling may inspect/control runtime only through governed privileged contracts and D02 controls.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Requirement |
|---|---|
| `CR-001` | Business domains shall not create independent workflow runtimes for behavior covered by canonical A05 orchestration. |
| `CR-002` | A05 shall never evaluate or become canonical owner of participating-domain business rules, decisions or business state. |
| `CR-003` | Every Workflow Instance shall be Tenant-bound and shall never change Tenant during its lifecycle. |
| `CR-004` | A Workflow Instance shall execute against one explicit originating immutable Workflow Version unless a separately governed migration succeeds. |
| `CR-005` | A05 shall invoke cross-owner business capability only through governed C01 contracts or approved asynchronous contracts and shall never write another owner’s persistence directly. |
| `CR-006` | A domain-owned action result used for workflow progression shall come from the authoritative owning domain and shall not be synthesized by A05. |
| `CR-007` | A05 owner-local transaction boundaries shall not propagate across canonical business owners. |
| `CR-008` | A05 runtime state required for long-running continuation shall be durably recoverable and shall not depend solely on process memory. |
| `CR-009` | Workflow-owned event publication shall preserve the Workflow Engine as semantic publisher while C02 owns distribution. |
| `CR-010` | Workflow state and workflow-event publication intent shall conform to the canonical owner-local state/publication handoff when durable publication is required. |
| `CR-011` | Human task assignment or workflow assignment shall not itself create authorization Permission. |
| `CR-012` | Protected task completion, cancellation, suspension, recovery, migration or privileged workflow operation shall consume a current D02 authorization result. |
| `CR-013` | Non-human workflow execution shall use an attributable D01 technical principal plus D02-governed technical authority binding and shall not fabricate a Person identity. |
| `CR-014` | A05 shall preserve correlation and sufficient causation/provenance across workflow initiation, domain invocation, wait/resume and terminal outcome. |
| `CR-015` | Duplicate/replayed technical inputs shall not produce duplicate workflow transitions when they represent the same governed operation. |
| `CR-016` | Concurrent inputs targeting the same Workflow Instance shall be resolved through owner-local concurrency control so conflicting authoritative states are not committed. |
| `CR-017` | Workflow cancellation shall not be treated as automatic reversal of completed participating-domain outcomes. |
| `CR-018` | Compensation shall invoke source-owned compensating behavior where defined and shall never directly rewrite another owner’s business state. |
| `CR-019` | A05 shall reject cross-Tenant event/task/domain-result correlation. |
| `CR-020` | Workflow runtime variables shall not become an authoritative shadow copy of canonical business state where the owning domain remains source of truth. |
| `CR-021` | Workflow definition activation and Workflow Instance initiation shall remain separate technical operations. |
| `CR-022` | Activating a successor Workflow Version shall not silently change an existing Workflow Instance’s originating version. |
| `CR-023` | Workflow Engine technical failure and participating-domain business rejection shall remain distinguishable. |
| `CR-024` | A05 shall not silently retry a source-owned business action merely because a technical dependency failed unless the governing contract/recovery policy makes the retry safe and permitted. |
| `CR-025` | A05 shall not deliver Notifications directly; notification delivery shall use the canonical C04 capability. |
| `CR-026` | Workflow timer semantics shall remain A05-owned even when generic technical execution uses A08. |
| `CR-027` | No workflow-engine vendor, notation, programming language, persistence product, broker, scheduler or numeric runtime target shall be inferred as canonical from this baseline. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| `OD-001` | Exact workflow definition representation / notation. | Technology / representation | No for baseline; yes before a normative notation is mandated. | A05-002 evaluates under technology/ADR governance if material. |
| `OD-002` | Exact workflow runtime/engine product or custom implementation. | Technology | No for product-neutral architecture; yes before product lock. | Evaluate under L5 technology/ADR methodology. |
| `OD-003` | Exact A05 runtime persistence representation/schema. | B02/A05 implementation | No for baseline. | A05-003 + B02 realization. |
| `OD-004` | Exact timer/scheduler execution mechanism and A05↔A08 runtime handoff. | A05/A08 technical boundary | No for baseline; yes before implementation of shared timer execution. | Resolve in A05-003/A08 without transferring workflow semantics. |
| `OD-005` | Exact workflow-instance migration technical mechanism. | A05-002/A05-003 | No for baseline; feature-blocking when migration is implemented. | Must preserve L4-A-017 safeguards; no invented state/event. |
| `OD-006` | Exact generic human-task assignment representation. | A05-003/D02 | No for baseline. | Define without treating task assignment as authorization. |
| `OD-007` | Exact technical compensation execution strategy. | A05-003 | No for baseline; feature-blocking for compensation-capable flows. | Preserve source-owned compensation policy/commands. |
| `OD-008` | C01 family remains a live peer architecture and may evolve before final lock. | Peer dependency | Non-blocking for controlled Stage-3 authoring while current contract boundaries remain stable. | Perform impact review if C01 changes materially. |
| `OD-009` | Whether a new A05-specific ADR is required for workflow runtime realization after A05-002/A05-003 expose concrete alternatives. | ADR governance | Non-blocking now. | Apply ADR-necessity test before selecting a normative engine/runtime pattern. |

No unresolved upstream business-authority gap is silently repaired here.

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-A-05-001` establishes the reusable workflow/orchestration architecture baseline while business domains retain ownership of business rules and decisions.

### 10.2 Primary Upstream Sources

- `L2-A05_Workflow_Engine_Workflows.md`
- `L4-A-014_Workflow_Definition_to_Activated_Orchestration_Availability.md`
- `L4-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md`
- `L4-A-016_Workflow_Execution_Failure_to_Controlled_Recovery_Outcome.md`
- `L4-A-017_Workflow_Change_to_Governed_New_Version_Availability.md`

### 10.3 Primary Cross-L5 Dependencies

```text
A04  Tenant / Institution context
D01  Human + technical principal identity
D02  Authorization / scope / SoD / privileged controls
C01  Synchronous owner contracts
B02  Owner-local persistence / transaction / concurrency
C02  Workflow-event publication delivery and compatibility
D04  Evidence
A08  Optional generic background execution
C04  Notification delivery
```

### 10.4 Applicable Approved ADRs

- `ADR-L5-0003`
- `ADR-L5-0004`
- `ADR-L5-0006`
- `ADR-L5-0007`
- `ADR-L5-0009`
- `ADR-L5-0010`
- `ADR-L5-0011`

### 10.5 Family Specialization Handoff

```text
L5-A-05-001
Baseline invariants, ownership, cross-L5 contracts,
durable/version-bound orchestration model
        ↓
L5-A-05-002
definition structure, validation, immutable versioning,
activation, policy/effective definition resolution,
successor/migration safeguards
        ↓
L5-A-05-003
instance runtime, task, wait, timer, escalation,
checkpoint, failure, retry, compensation and recovery mechanics
```

### 10.6 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-A/
            └── L5-A-05-001_Workflow_and_Orchestration_Architecture_Baseline.md
```

No major-area subfolder such as `L5-A/L5-A-05/` is introduced.
