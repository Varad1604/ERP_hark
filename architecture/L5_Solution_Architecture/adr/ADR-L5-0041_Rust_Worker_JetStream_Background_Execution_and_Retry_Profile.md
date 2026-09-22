# ADR-L5-0041 — Rust Worker, JetStream Background Execution & Retry Profile

**ADR ID:** `ADR-L5-0041`  
**Title:** Rust Worker, JetStream Background Execution & Retry Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-A-08 — Background Processing & Job Execution Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-A-08-001`; `L5-A-08-002`; constrains `L5-C-02-001/002`, `L5-C-01-001`, and A05/A08 handoffs  
**Related ADRs:** `ADR-L5-0010`; `ADR-L5-0013`; `ADR-L5-0019`; `ADR-L5-0022`; `ADR-L5-0037`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

A08 already owns technical background execution, work-unit identity, attempts and retry mechanics. Flowable is the canonical workflow engine and NATS JetStream is the canonical durable messaging transport. Backend implementation still requires a deterministic division of responsibility among workflow orchestration, technical workers, durable work delivery and retry/failure handling.

## 2. Decision

1. **Dedicated worker runtime:** technical/background work executes in dedicated **Rust worker processes/runtimes** built on the canonical Rust/Tokio platform. Flowable does not become the generic compute or batch-execution engine.
2. **Work delivery:** durable asynchronous work delivery uses **NATS JetStream** through C02-governed transport semantics. Technical work messages use A08/C02 contracts and shall not be confused with canonical business events merely because they share the broker.
3. **Authoritative job/work state:** the owning A08/job capability persists authoritative work-unit/job state through B02 owner-local persistence. JetStream delivery state is transport state, not the canonical job/business record.
4. **Stable execution identity:** every durable/retryable work unit has a stable technical identity and an idempotency boundary at the owning effect.
5. **Retry classification:** transient technical failures may retry using bounded exponential backoff with jitter; permanent validation/configuration/business failures do not blindly retry. No universal retry count or delay is locked.
6. **Failure/dead-letter handling:** work exhausted under the active retry policy is preserved in a governed failed/dead-letter state or equivalent recoverable failure path with attributable reason, attempt history and an explicit manual/automated recovery decision. Failed work is not silently dropped.
7. **Scheduling split:** recurring technical/maintenance/background jobs are scheduled through A08-owned scheduling mechanics. Workflow-owned timers/escalations remain A05/Flowable-owned.
8. **Distributed locking:** no global distributed lock service is canonical. Prefer owner-local transactions, optimistic concurrency, stable idempotency keys, leases/claims and single-owner work partitioning. A distributed lock may be introduced only for a demonstrated technical need under separate governance.
9. **Ordering:** no global job ordering exists. Ordering is preserved only within the smallest required owner/work partition and must be explicitly justified.
10. **Long-running requests:** work that cannot safely/appropriately complete inside a synchronous request is accepted asynchronously and returns a governed work/reference identity through C01, with separate status/result retrieval or event completion as applicable.
11. **Concurrency/backpressure:** worker concurrency, prefetch, batch size and scaling are bounded by E02 capacity/admission profiles. No unlimited autoscaling and no universal numeric limit are established here.
12. **Valkey boundary:** Valkey may support non-authoritative acceleration where E02 permits it, but it is not the canonical job queue, durable work store, scheduler or global lock authority.

## 3. Conformance Rules

- Retry shall not replay an already-committed business effect without owner idempotency protection.
- Technical work delivery acknowledgement occurs only after the applicable durable effect/state transition is safely established according to the owner contract.
- A worker may call owner contracts but shall not bypass owner persistence boundaries.
- Dead-letter/failure state shall preserve enough non-secret evidence to diagnose and govern re-drive.
- Re-drive uses the same stable work identity or an explicitly linked successor; it shall not fabricate a new unrelated business request.
- Capacity controls fail/defer explicitly rather than creating unbounded queues or concurrency.

## 4. Consequences

SutraERP gains one clear asynchronous execution model: Rust workers perform technical work; JetStream transports work; B02 persists owner state; Flowable orchestrates workflows; E02 governs capacity. This prevents a second queue product and avoids turning Flowable, Valkey or the broker into business authority.

## 5. Approval Record

**Decision selected:** dedicated Rust workers; JetStream work delivery; owner-persisted job/work truth; classified bounded retries with jitter; recoverable dead-letter handling; A08 technical scheduler; no default global distributed lock; stable idempotent work identity; scoped ordering; asynchronous work-reference pattern; bounded profile-driven worker concurrency.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
