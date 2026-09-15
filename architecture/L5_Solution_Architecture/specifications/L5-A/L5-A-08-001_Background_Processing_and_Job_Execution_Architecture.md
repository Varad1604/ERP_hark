# L5-A-08-001 — Background Processing & Job Execution Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-08-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-08 — Background Processing & Job Execution Architecture  
**Canonical Technical Responsibility:** Trusted background/asynchronous execution, durable job identity, execution-context preservation and governed technical job lifecycle without redefining business ownership.  
**Technical Architecture Owner:** L5-A-08 — Background Processing & Job Execution Architecture  
**Assigned Author:** Varad  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-15`  
**Depends On:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Technology Selection & ADR Methodology; L5 ADR Register; `L5-A-01-001`; `L5-C-02-001`; `L5-C-02-002`; `L5-C-02-003`; `L5-E-04-001`; `L5-B-02-001`; `L5-B-02-003`; `L5-D-02-*`; `L5-D-04-*`; `L4-A-015`; `L4-A-016`.  
**Referenced By:** `L5-A-08-002`; workflow/orchestration and other L5 consumers requiring generic background execution.  
**Applicable ADRs:** `ADR-L5-0003`, `ADR-L5-0005`, `ADR-L5-0006`, `ADR-L5-0007`, `ADR-L5-0009`, `ADR-L5-0010`, `ADR-L5-0011`, `ADR-L5-0013`.  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable L5 technical boundary for controlled background/asynchronous execution. It establishes how approved work becomes a durable technical Job, how authoritative execution context is bound, how one execution attempt is exclusively claimed, how technical lifecycle state is persisted, and how completion/failure evidence is handed to canonical persistence, event-delivery and observability boundaries.

It does not create a second business authority. The originating domain or workflow retains ownership of business validity, business lifecycle, business retry/recovery and business outcome.

### 2.2 In Scope

- governed background-work admission;
- stable Job identity and immutable source/context references;
- durable Job lifecycle state;
- execution eligibility and non-human authority validation;
- exclusive execution claim and concurrency protection;
- execution Attempt identity;
- technical completion/failure/termination recording;
- owner-local state-to-publication/evidence handoff;
- restart/recovery of non-terminal technical execution;
- A05-to-A08 generic execution contract;
- integration with C02, B02, D02/D04 and E04.

### 2.3 Non-Goals

- workflow-specific timers, wait conditions, Workflow Instance state or workflow recovery — A05;
- bulk partitioning, scheduled release and bounded retry policy — A08-002;
- business retry/compensation/recovery;
- canonical business-event semantics;
- cross-owner persistence writes;
- runtime packaging/deployment topology — E04;
- worker/queue/scheduler/database/vendor selection;
- L6 UI behavior.

---

## 3. Authoritative Constraints & Assumptions

### 3.1 Ownership Constraints

1. Background execution shall not change canonical business ownership.
2. A Job may execute an operation only through the owning domain/workflow contract.
3. A08 shall not mutate Workflow Instance state. Where A05 is the caller, A05 owns workflow-specific scheduling semantics and consumes only A08 technical status/outcome.
4. Cross-owner direct database writes are prohibited.
5. Non-human worker authority shall consume the canonical D02/D01 binding model; no Person/Role Assignment is fabricated.
6. C02 owns durable event delivery, acknowledgement, ordering and recovery after A08 establishes any required owner-local publication intent.
7. E04 owns physical runtime placement and deployment realization.

### 3.2 Stage-3 Dependency Gate

This specification explicitly consumes the Stage-3 minimum gate:

| Required dependency | Consumed responsibility |
|---|---|
| `L5-A-01-001` | application/execution boundary model and selective isolation constraints |
| `L5-C-02-*` | durable technical-event/message delivery and compatibility after owner handoff |
| `L5-E-04-001` | provider-neutral/on-premises-primary-capable runtime packaging and deployment realization |

Additional B02/D02/D04 dependencies govern persistence, concurrency, execution authority and evidence.

### 3.3 Assumptions

**AS-001:** The caller supplies a stable source-work reference and enough authoritative scope to validate execution.  
**AS-002:** The caller remains responsible for business validity and can revoke/cancel future technical execution through an explicit contract.  
**AS-003:** Where business side effects may be repeated after infrastructure failure, the owning operation supplies an idempotency or duplicate-safety contract; A08 does not infer one.

---

## 4. Technical Architecture

### 4.1 Logical Components

| Component | Responsibility |
|---|---|
| Job Admission Controller | validates request shape, scope, source reference and permitted operation class |
| Job State Store | durable owner-local Job and Attempt state under B02 constraints |
| Eligibility Gate | validates lifecycle, authority/context, cancellation and runtime eligibility before start |
| Execution Claim Coordinator | establishes exclusive claim/version before one Attempt becomes Running |
| Background Executor | invokes only the declared owner contract using bound execution context |
| Outcome Recorder | persists technical success/failure/termination and attempt evidence |
| Publication/Evidence Handoff | creates governed downstream intent/evidence references without redefining business events |

These are logical responsibilities, not mandated deployable services.

### 4.2 Canonical Job Record

A durable Job shall include at minimum:

- `job_id` — stable technical identity;
- `source_work_ref` — caller-owned source identity/reference;
- `tenant_id` and Institution/scope reference where applicable;
- correlation/causation references;
- allowed technical operation class;
- non-human authority binding reference;
- admission timestamp/order evidence;
- lifecycle status;
- cancellation/revocation status;
- optimistic version or equivalent concurrency token;
- latest technical outcome/evidence reference.

Tenant/Institution/source-work identity shall not be silently rewritten after admission.

### 4.3 Technical Lifecycle

Canonical conceptual lifecycle:

```text
Admitted -> Ready -> Running -> Completed
                    |          -> Failed
                    |          -> Terminated
                    -> Terminated
```

`Completed`, `Failed` and `Terminated` are technical states. They do not imply business completion, business failure or compensation.

Allowed transitions shall be checked against the durable current state/version. A terminal Job cannot silently re-enter Running. New work requires an explicit new Job or an A08-002-governed new Attempt when retry semantics apply.

### 4.4 Admission Transaction

Admission shall atomically establish the Job identity, immutable source/context binding and initial durable state. Admission is rejected when:

- required Tenant/Institution/source scope cannot be resolved;
- requested operation is not allowed for the supplied technical principal/context;
- caller source reference is invalid or structurally incomplete;
- a caller-required uniqueness key conflicts with an already-admitted equivalent Job;
- mandatory persistence is unavailable and durable admission cannot be proven.

No request is considered admitted before durable state exists.

### 4.5 Eligibility Gate

Before every start/restart, validate:

1. Job is non-terminal and start-compatible;
2. required execution context remains valid;
3. non-human authority binding remains permitted;
4. cancellation/revocation has not been asserted;
5. prerequisites declared by the caller are satisfied;
6. runtime capacity/health permits safe execution;
7. no conflicting active claim exists.

Failure of the gate leaves the Job non-running or moves it to an explicit technical failure/termination state as appropriate; it never fabricates success.

### 4.6 Exclusive Execution Claim

The transition to Running shall use an owner-local atomic compare-and-set/versioned update, lease/claim record or semantically equivalent concurrency mechanism. The invariant is:

```text
For an exclusive Job identity, at most one valid active Attempt may own execution authority at a time.
```

A stale claim holder shall not commit a newer technical state after its claim/version becomes invalid.

### 4.7 Attempt Identity

Every concrete run has an `attempt_id` associated with one `job_id`. A08-001 requires at least one Attempt for execution; A08-002 extends this to multiple bounded attempts.

Attempt evidence records start, end, executor identity, claim/version, technical failure class and outcome reference. Attempt identity is never a business-request identity.

### 4.8 Owner-Contract Invocation

The executor invokes only a declared owner contract. Inputs include bound Job context and stable idempotency/source references where applicable. The owner remains responsible for its transaction, authoritative state and business result.

A08 shall not read/write another owner’s authoritative tables directly.

### 4.9 Durable Outcome and Publication Handoff

A Job shall not expose `Completed`, `Failed` or `Terminated` as authoritative until that technical state and required evidence references are durably recorded.

Where a technical status signal/event must be delivered:

1. A08 records the state change and publication intent using the applicable B02/ADR-0007 owner-local handoff pattern;
2. C02 owns durable delivery, subscription, acknowledgement, ordering and recovery;
3. the signal remains a technical execution signal unless an upstream owner explicitly defines a canonical business event.

### 4.10 Crash / Restart Recovery

On process/host restart, the recovery controller examines durable non-terminal Jobs/Attempts. A previously Running Attempt whose claim is expired/invalid is not assumed successful. The system determines a technical recovery disposition from durable evidence:

- re-establish safe execution eligibility;
- mark technical failure/unresolved state; or
- hand off to A08-002 retry rules where the work is retry-capable.

Unknown business side-effect status must not be converted to automatic success or retry without the owner’s duplicate-safety contract.

### 4.11 Security & Trust

- worker execution uses least-authority non-human principal binding;
- Tenant/Institution context is mandatory where the source work is scoped;
- secrets/keys remain D03-owned;
- executor logs/evidence must not expose protected values beyond D03/D04 policy;
- cancellation, manual requeue/recovery and privileged execution operations require explicit authorization.

### 4.12 Runtime Boundary

A08 defines logical executor isolation, concurrency and restart requirements. E04 decides whether those logical responsibilities are co-located, separate processes, containers or hosts. The realization must remain on-premises-primary capable and provider-neutral unless separately governed.

---

## 5. Integration Contracts

### 5.1 A05 -> A08 Generic Execution Request

Minimum logical fields:

- caller/source workflow reference;
- stable requested-work identity;
- Tenant/Institution context;
- correlation/causation context;
- allowed technical operation class;
- earliest technical release condition if one exists;
- cancellation/revocation reference;
- authority-binding reference;
- idempotency/duplicate-safety reference where required.

A08 returns `job_id`, technical status and outcome/evidence references. A05 alone changes Workflow Instance state.

### 5.2 A08 -> B02

A08 owns its Job/Attempt records while obeying B02 persistence topology, transaction ownership and optimistic concurrency. No cross-owner database mutation is permitted.

### 5.3 A08 -> C02

C02 begins after A08 has a durable publication intent. A08 consumes C02 delivery semantics; it does not create a separate delivery subsystem.

### 5.4 A08 -> E04

E04 realizes deployment, scaling, restart and placement. A08 supplies logical requirements only.

---

## 6. Technical Failure Model

| Failure | Required behavior |
|---|---|
| invalid context/authority | reject or fail closed before owner invocation |
| concurrent duplicate claim | one claimant wins; stale claimant cannot commit |
| executor crash before owner invocation | recover from durable state; never mark Completed |
| crash during/after owner side effect | use owner evidence/idempotency contract; unresolved state is explicit |
| persistence unavailable | no new durable admission/start that cannot be proven |
| C02 unavailable after durable publication intent | Job state remains durable; C02 recovery owns delivery |
| capacity unavailable | remain Ready/deferred or explicit technical failure; never silently drop |

---

## 7. Conformance Requirements

| ID | Requirement |
|---|---|
| CR-001 | Every admitted background execution has a durable stable `job_id`. |
| CR-002 | Source work, Tenant/Institution scope, correlation and authority binding are recorded before execution. |
| CR-003 | A Job cannot become Running without an atomic exclusive claim/version check. |
| CR-004 | A stale or competing executor cannot commit a newer technical lifecycle state. |
| CR-005 | Terminal Job state is durably recorded before being exposed as authoritative. |
| CR-006 | Technical completion is not equated with business completion unless the owner contract explicitly defines that relation. |
| CR-007 | A08 never mutates Workflow Instance state or another owner’s authoritative persistence directly. |
| CR-008 | Worker authority consumes the canonical non-human principal binding architecture. |
| CR-009 | Required publication uses owner-local durable handoff then C02 delivery semantics. |
| CR-010 | Crash/restart recovery uses durable Job/Attempt evidence and never assumes success from process disappearance. |
| CR-011 | Runtime realization remains E04-owned and provider-neutral/on-premises-primary capable. |
| CR-012 | Implementation evidence can reconstruct Job, Attempt, context, state transitions, claim ownership and terminal disposition. |

---

## 8. L4 Traceability

For this validation program the referenced canonical L4 behaviors are treated as locked behavioral authority despite repository metadata still showing Draft.

| L4 BH ID | Relationship | L5 contribution / ownership boundary |
|---|---|---|
| `L4-A-015-BH-001` | Supporting | A08 preserves validated execution context/admission; workflow-initiation authority remains A05/L4. |
| `L4-A-015-BH-006` | Supporting | A08 can execute deferred technical work only after the A05-owned wait/release condition is satisfied. |
| `L4-A-015-BH-007` | Supporting | Job context/identity survives technical suspension/resumption; workflow state/version remains A05-owned. |
| `L4-A-015-BH-010` | Supporting | technical failure remains explicit and cannot be represented as workflow success. |
| `L4-A-015-BH-011` | Direct | Job/Attempt evidence supplies attributable technical execution evidence. |
| `L4-A-016-BH-003` | Supporting | Tenant, correlation and execution context are preserved across technical failure/recovery boundaries. |
| `L4-A-016-BH-010` | Direct | execution/recovery evidence remains attributable and reconstructable. |

No claim is made that A08 owns the business/workflow behavior represented by these BHs.

---

## 9. Architecture Decisions

**AD-001 — Governed Background Execution Pattern:** `ADR-L5-0013` is the canonical shared decision for Job/Unit/Attempt identity, durable execution state, technical retry boundary and A05/A08 handoff.  
**AD-002 — Owner-Local Persistence & Concurrency:** A08 consumes `ADR-L5-0005`, `0006` and `0007`; it does not invent alternate persistence ownership or cross-owner transactions.  
**AD-003 — Non-Human Authority:** A08 consumes `ADR-L5-0009`.  
**AD-004 — Durable Delivery:** required technical signals consume `ADR-L5-0010`/`0011` after durable handoff.

---

## 10. Open / Deferred Design

**OD-001:** Concrete Job/Attempt storage schema and indexes are L6/implementation design under the invariants above.  
**OD-002:** Numeric lease duration, worker limits and timeout values are governed configuration/operational profile, not fixed by this specification.  
**OD-003:** Product selection for workers/queues/schedulers remains separate technology/ADR governance if material.  

No open item blocks the ownership, lifecycle, persistence, claim, retry-boundary or dependency architecture defined here.