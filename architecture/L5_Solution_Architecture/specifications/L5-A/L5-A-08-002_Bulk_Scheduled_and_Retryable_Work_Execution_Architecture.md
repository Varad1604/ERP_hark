# L5-A-08-002 — Bulk, Scheduled & Retryable Work Execution Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-08-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-08 — Background Processing & Job Execution Architecture  
**Canonical Technical Responsibility:** Long-running, bulk, scheduled and technically retryable work execution using durable work-unit and attempt state while preserving upstream business/workflow authority.  
**Technical Architecture Owner:** L5-A-08 — Background Processing & Job Execution Architecture  
**Assigned Author:** Varad  
**Version:** `1.3.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Technology Selection & ADR Methodology; L5 ADR Register; `L5-A-08-001`; `L5-A-01-001`; `L5-C-02-001`; `L5-C-02-002`; `L5-C-02-003`; `L5-E-04-001`; `L5-A-05-003`; `L5-B-02-003`; `L5-D-02-*`; `L4-A-015`; `L4-A-016`.  
**Referenced By:** L5 consumers requiring reusable bulk/scheduled/technical-retry execution.  
**Applicable ADRs:** `ADR-L5-0006`; `ADR-L5-0007`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0013`; `ADR-L5-0019`; `ADR-L5-0022`; `ADR-L5-0041`; `ADR-L5-0042`.

---

## 2. Purpose, Scope & Non-Goals

This specification extends `L5-A-08-001` for work that must be partitioned, delayed, scheduled, resumed or technically retried. It defines durable Work Unit and Attempt state, release gating, bounded retry, duplicate-execution protection and aggregate technical completion.

It does not own workflow timers/business deadlines, business retry/recovery, business compensation, canonical business events, product-specific schedulers/queues, or another owner’s authoritative state.

---

## 3. Authoritative Constraints & Assumptions

1. `L5-A-08-001` owns common Job identity/context/admission/lifecycle; this specification shall not redefine it.
2. A05 owns workflow-specific timer semantics, wait conditions and Workflow Instance transitions. A08 consumes only a generic technical release request.
3. A retry creates a new technical `attempt_id`; it never silently creates a new business request.
4. Retry is permitted only for explicitly technical-retryable failure and only while upstream business validity remains in force.
5. Unknown or business-ambiguous failure is not retryable by default.
6. C02 is present and approved on current `main`; this specification consumes C02 rather than treating it as unavailable.
7. Stage-3 dependency gate is explicitly consumed: A01 + C02 + E04, in addition to A08-001 and A05 where workflow initiated.

**AS-001:** Upstream request contains a stable Job/source-work reference and indicates whether the operation is duplicate-safe/idempotent where repeated technical execution is possible.  
**AS-002:** The failure source provides a technical classification sufficient to distinguish retryable, non-retryable, business-invalid and unresolved outcomes. When it cannot, classification is `Unresolved` and automatic retry is prohibited.  
**AS-003:** Numeric retry count, backoff and release windows are governed configuration and may vary by workload class; no canonical numeric values are fixed here.

---

## 4. Technical Architecture

### 4.1 Logical Components

| Component | Responsibility |
|---|---|
| Work Planner | validates and materializes bounded Work Units from an admitted Job |
| Release Scheduler | evaluates technical release eligibility; does not own business calendars |
| Attempt Controller | creates durable Attempt identity and applies retry bounds |
| Claim Coordinator | prevents conflicting simultaneous execution of the same exclusive Work Unit |
| Progress Aggregator | derives technical aggregate status from durable unit outcomes |
| Recovery Handoff | returns unresolved/business-invalid conditions to the owning workflow/domain |

### 4.2 Durable State Model

Each Work Unit records at minimum:

- `unit_id`, parent `job_id`, partition/source reference;
- Tenant/Institution/correlation context inherited from Job;
- release condition/window reference;
- current unit status;
- dependency/order reference where units are not independent;
- retry-policy/config reference;
- latest attempt/outcome reference;
- optimistic version/claim state.

Each Attempt records at minimum:

- `attempt_id`, `unit_id`/`job_id`;
- start/end timestamps or equivalent ordering evidence;
- executor/authority reference;
- claim/version reference;
- failure classification;
- retry decision and reason;
- terminal outcome/evidence reference.

### 4.3 Unit Lifecycle

```text
Planned -> Eligible -> Running -> Completed
                     |          -> FailedRetryable -> Eligible
                     |          -> FailedTerminal
                     |          -> Cancelled
                     -> Cancelled
```

A retry transition always closes the prior Attempt and creates a new Attempt. The Unit identity remains stable.

### 4.4 Partitioning

Partitioning shall be deterministic or durably recorded so restart does not silently create a different unit set for the same admitted Job. Partitioning shall not mix unrelated Tenant/Institution scope. Dependencies/order constraints among units must be explicit where correctness requires them.

### 4.5 Release Eligibility

Before release, validate:

- parent Job is still admitted and non-terminal;
- Unit is not completed/cancelled/terminally failed;
- A05/domain-owned business validity has not been revoked;
- technical release time/condition has been reached;
- required dependencies are complete where applicable;
- authority/context is still valid;
- safe capacity/health permits execution;
- no conflicting valid claim exists.

A05 may request a technical `not-before` or equivalent release condition, but A08 shall not interpret that as ownership of workflow timer semantics.

### 4.6 Retry Decision

A new Attempt may be created only when all are true:

1. previous Attempt is durably closed;
2. failure class = `TechnicalRetryable`;
3. retry budget/config permits another Attempt;
4. Unit/Job remains non-terminal;
5. business validity/revocation state permits continuation;
6. authority/context remains valid;
7. any required delay/release condition is satisfied.

`BusinessInvalid`, `TechnicalNonRetryable` and `Unresolved` do not auto-retry.

### 4.7 Duplicate Execution Protection

Exclusive Work Units use the A08-001 claim/version invariant. A late/stale Attempt cannot commit a terminal state after losing its claim. Where owner-side effects are repeatable only through an idempotency key, the stable Job/Unit/source reference is supplied to the owner contract.

### 4.8 Aggregate Technical Completion

Parent Job progress is derived from durable Work Unit states. At minimum the implementation can distinguish:

- all required Units completed;
- Units remain pending/eligible/running;
- one or more terminal technical failures;
- cancelled work;
- unresolved/business-invalid handoff required.

Aggregate technical completion shall not be exposed as a business outcome unless explicitly mapped by the owner contract.

### 4.9 Pause / Resume / Cancellation

Pause prevents release of new Attempts; it does not erase prior Attempts. Resume revalidates eligibility before any new Attempt. Cancellation prevents future execution where possible, invalidates outstanding release eligibility/claims as governed, and preserves completed history. It does not reverse business effects already committed by owners.

### 4.10 Persistence / Publication / Runtime Boundaries

- Work Unit/Attempt state obeys B02 owner-local transaction and concurrency rules.
- Required signals use durable publication intent then C02 delivery.
- Worker authority consumes D02/D01 non-human principal binding.
- E04 owns runtime packaging/scaling/deployment.

---

## 5. A05 ↔ A08 Operational Boundary

### A05 owns

- workflow-specific timer identity;
- wait/due condition;
- Workflow Instance state/version;
- whether workflow continuation is business-valid;
- workflow recovery/compensation decision.

### A08 owns

- generic technical Job/Unit/Attempt execution;
- technical `not-before`/release enforcement supplied by caller;
- durable execution state and retry mechanics;
- technical failure classification and evidence.

### Contract

A05 submits a generic execution request containing source workflow reference, work identity, scope/context, permitted operation, release condition, cancellation/revocation reference and authority binding. A08 returns Job/Unit/Attempt status and outcome/evidence reference only. A08 never directly transitions Workflow Instance state.

---

## 6. Failure Model

| Condition | Required disposition |
|---|---|
| retryable infrastructure/transient technical failure | close Attempt; schedule next eligible Attempt within governed bound |
| non-retryable technical failure | terminal Unit failure; handoff evidence |
| business validity revoked | no new Attempt; controlled cancellation/handoff |
| failure cannot be classified | `Unresolved`; no automatic retry |
| scheduler/process restart | recover from durable Job/Unit/Attempt state |
| duplicate/stale claimant | reject stale commit; preserve winning claim evidence |
| C02 unavailable | keep durable publication intent; C02 recovery owns delivery |

---

## 7. Conformance Requirements

| ID | Requirement |
|---|---|
| CR-001 | Every Work Unit has stable identity bound to one admitted Job. |
| CR-002 | Partitioning is scope-safe and restart-stable/durably recorded. |
| CR-003 | Every retry creates a new durable Attempt identity. |
| CR-004 | Automatic retry occurs only for explicitly `TechnicalRetryable` failure. |
| CR-005 | `Unresolved` or business-invalid failure cannot auto-retry. |
| CR-006 | Retry is bounded by governed configuration; unbounded immediate loops are prohibited. |
| CR-007 | A05 workflow state/timer semantics are never mutated/redefined by A08. |
| CR-008 | Exclusive Units cannot have two valid concurrent execution owners. |
| CR-009 | Pause/resume/cancel preserves prior execution evidence and revalidates future execution. |
| CR-010 | Aggregate technical completion is distinguishable from business completion. |
| CR-011 | Stage-3 A01+C02+E04 dependencies are explicitly consumed. |
| CR-012 | Durable state and downstream publication follow B02/C02 handoff rules. |

---

## 8. L4 Traceability

| L4 BH ID | Relationship | L5 contribution / boundary |
|---|---|---|
| `L4-A-015-BH-006` | Supporting | A08 enforces technical deferred release only after the caller-owned wait/release condition; workflow semantics remain upstream. |
| `L4-A-015-BH-007` | Supporting | pause/resume preserves execution identity/context; Workflow Instance/version remain upstream. |
| `L4-A-016-BH-001` | Supporting | A08 records technical failure classification without taking business recovery ownership. |
| `L4-A-016-BH-003` | Supporting | Tenant/correlation/execution context survives across Attempts. |
| `L4-A-016-BH-006` | Supporting | every recovery/retry attempt revalidates continuation eligibility; business recovery authority remains upstream. |
| `L4-A-016-BH-010` | Direct | unit/attempt/retry evidence supports reconstruction of recovery execution. |

---

## 9. Architecture Decisions

**AD-001:** `ADR-L5-0013` governs the reusable Job/Work Unit/Attempt and bounded technical-retry pattern.  
**AD-002:** `ADR-L5-0006`/`0007` govern owner-local concurrency and state-to-publication handoff.  
**AD-003:** `ADR-L5-0010`/`0011` govern durable message delivery and compatibility after A08 handoff.  
**AD-004:** `ADR-L5-0009` governs non-human execution authority.

---

## 10. Open / Deferred Design

**OD-001:** Workload-specific numeric retry/backoff/parallelism values remain governed configuration.  
**OD-002:** **Resolved architecture:** A08 Rust workers execute technical work, JetStream carries durable work delivery, and Flowable owns workflow-instance timers only. Numeric/profile limits remain governed configuration. See `ADR-L5-0041` and `ADR-L5-0042`.  
**OD-003:** Concrete persistence schema and indexes are implementation design under the state/concurrency invariants above.

## 2026-09-22 Backend Readiness Group 4 Synchronization

**Applicable locked decision:** `ADR-L5-0041`  
Dedicated Rust workers execute technical/background work; NATS JetStream supplies durable work delivery; owner-local persistence holds job/work truth; retries are classified/bounded with exponential backoff+jitter; failed work is recoverable and never silently dropped; A08 owns technical schedules; no default global distributed lock.


## 2026-09-23 Post-R8 Lifecycle Closure

This specification is promoted from Review to Architecture Approved after the post-R8 upstream authority remediation made its material A016/A057 parent behavior Architecture Approved and the specification's existing HF/open-decision profile was rechecked. No new ownership, business lifecycle, event publisher, or implementation technology is introduced by this promotion. Final corpus lock remains separate governance.
