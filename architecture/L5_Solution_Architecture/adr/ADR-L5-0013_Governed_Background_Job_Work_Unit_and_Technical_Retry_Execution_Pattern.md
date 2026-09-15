# ADR-L5-0013 — Governed Background Job, Work-Unit & Technical Retry Execution Pattern

**ADR ID:** `ADR-L5-0013`  
**Title:** Governed Background Job, Work-Unit & Technical Retry Execution Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-A-08 — Background Processing & Job Execution Architecture  
**Assigned Author:** Architecture remediation for Varad Stage-3 corpus  
**Reviewers:** Controlled L5 architecture remediation review  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-15  
**Affected L5 Specifications:** `L5-A-08-001`; `L5-A-08-002`; constrains consumers including `L5-A-05-003`  
**Related ADRs:** `ADR-L5-0003`; `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0011`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

SutraERP requires a reusable asynchronous execution architecture for background work, long-running work, scheduled release, work partitioning and bounded technical retry. These semantics cross multiple specifications and downstream consumers and therefore cannot remain an implicit local design choice.

The decision must preserve business ownership, workflow ownership, Tenant/Institution scope, non-human authority, durable technical state, independent execution attempts and reliable event/evidence handoffs without selecting a specific queue, scheduler, worker framework, database or cloud product.

## 2. Scope & Affected Architecture

This ADR governs the reusable technical execution pattern owned by A08. It establishes the canonical relationship among admitted job, work unit, execution attempt, technical scheduling/release, retry and terminal technical outcome.

It does not own business workflow state, business retry/recovery policy, business deadlines, canonical business events, business transactions, product selection or deployment topology.

Where A05 initiates background work, A05 owns workflow-specific scheduling semantics, workflow state and business continuation. A08 owns only the generic technical execution mechanism exposed through an explicit handoff contract.

## 3. Authoritative Constraints

1. Every background execution originates from a governed admission request with stable technical identity and authoritative execution context.
2. Job identity, work-unit identity and execution-attempt identity are distinct and durable where the corresponding concept exists.
3. Tenant, Institution, correlation, initiating authority and source-work references are immutable execution-context attributes except where the authoritative owner explicitly issues a new request.
4. A08 shall not create, change or infer business authority merely because work is asynchronous.
5. A05 retains workflow-specific timers, wait conditions, Workflow Instance state and recovery decisions; A08 never mutates workflow state outside A05-owned contracts.
6. Technical retry is bounded, failure-class aware and creates a new attempt; it is not a new business request.
7. Unknown or business-ambiguous failure shall fail closed rather than trigger autonomous technical retry.
8. Duplicate simultaneous execution of the same exclusive work unit shall be prevented through durable state/coordination semantics.
9. State transitions that claim Running, Completed, Failed, Cancelled or Terminated shall be durably recorded before they are exposed as authoritative technical status.
10. When a state transition requires downstream publication, the owner-local persistence/handoff architecture shall preserve atomic state-to-publication intent semantics where applicable; event delivery then follows C02.
11. Non-human execution authority shall consume the canonical D02/D01 binding model and shall not fabricate a Person/Role Assignment.
12. Runtime packaging/deployment shall remain E04-owned and provider-neutral/on-premises-primary capable.
13. No queue, broker, scheduler, worker runtime, locking product or database technology is selected by this ADR.

## 4. Decision Criteria

The decision must preserve ownership, support crash/restart recovery, allow controlled concurrency, make retries observable and bounded, isolate failures, remain technology-neutral, integrate with C02 event delivery, and support both modular-core and selectively isolated execution permitted by A01/E04.

## 5. Alternatives Considered

### A — Fire-and-forget asynchronous execution
Rejected because execution identity, restart safety, failure visibility and durable completion evidence are insufficient for ERP workloads.

### B — Workflow engine owns all background execution mechanics
Rejected because it conflates workflow-specific business/orchestration semantics with reusable technical background execution and creates ownership duplication with A08.

### C — Durable admitted job/work-unit/attempt model with governed technical retry and explicit owner handoffs
Selected.

### D — Repository-wide exactly-once execution guarantee
Rejected because end-to-end exactly-once behavior cannot be guaranteed across independent owner transactions and external effects without excessive coupling; idempotency and duplicate-safe handoffs are required instead.

## 6. Decision

SutraERP shall use a durable governed background-execution model with the following canonical concepts:

1. **Job** — the admitted technical execution request and immutable context envelope.
2. **Work Unit** — an optional bounded partition of the admitted job used for bulk/long-running execution.
3. **Attempt** — one concrete execution try for a job or work unit; retries always create a new attempt.
4. **Release Eligibility** — a technical gate determining whether an admitted unit may start now; it cannot override upstream business validity.
5. **Execution Lease/Claim** — an implementation-neutral exclusivity record preventing conflicting concurrent execution where required.
6. **Technical Outcome** — Completed, Failed, Cancelled/Terminated or equivalent governed technical terminal status; never silently equated with a business outcome.

The canonical flow is:

```text
Owner Request
  -> A08 Admission + Context Validation
  -> Durable Job/Unit State
  -> Release Eligibility
  -> Exclusive Attempt Claim
  -> Running Attempt
  -> owner-local operation / delegated work
  -> durable technical outcome
  -> publication/evidence handoff where required
```

Technical retry is permitted only when all are true:

- failure classification is explicitly technical and retryable;
- the job/unit remains non-terminal;
- upstream business validity has not been revoked;
- retry budget/policy remains available;
- required authority/context remains valid;
- a new attempt identity is created.

Retry delay/backoff values remain governed configuration, not canonical architecture constants.

## 7. Required Handoff Contracts

### 7.1 A05 → A08 Generic Execution Request

At minimum: source workflow reference, stable requested-work identity, Tenant/Institution context, correlation context, permitted technical operation class, earliest release condition if applicable, cancellation/revocation reference, and required non-human authority binding.

A08 returns only technical execution status and attributable outcome/evidence references. It does not set Workflow Instance state.

### 7.2 A08 → B02 Persistence Boundary

Durable job/unit/attempt state belongs to the owning A08 persistence responsibility but must obey B02 persistence ownership, transaction and concurrency rules. Competing claims use owner-local concurrency control; direct cross-owner persistence writes are prohibited.

### 7.3 A08 → C02 Event Boundary

Where technical status publication is required, A08 produces an owner-local publication intent and C02 owns durable delivery, subscription, acknowledgement, ordering and recovery. A08 does not redefine canonical business events.

### 7.4 A08 → D02/D01 Authority Boundary

Workers execute as non-human principals bound to an allowed operation/scope. The binding must remain traceable to the initiating request and must not widen authority.

### 7.5 A08 → E04 Runtime Boundary

A08 defines logical execution isolation and concurrency requirements; E04 decides packaging, process/container/host placement, scaling and deployment topology.

## 8. Consequences

Positive: restart-safe execution, explicit retry semantics, clear A05/A08 ownership, durable diagnostics, controlled duplicate execution and consistent downstream integration.  
Cost: implementations require durable state, claim/concurrency control, explicit handoff contracts, attempt history and recovery tooling.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Technical retry repeats business effects | Require owner-defined idempotency/business-validity boundary; retry creates only a technical attempt. |
| Worker gains excess authority | Consume non-human principal binding and immutable execution scope. |
| Duplicate workers race | Durable owner-local claim/version check before Running transition. |
| Crash after side effect but before status update | Require owner contract/idempotency and atomic owner-local state/publication intent where applicable. |
| A08 redefines workflow timers | Explicit A05 ownership; A08 only consumes a technical release request. |
| Queue/scheduler implementation becomes canonical | Keep product/topology outside ADR and under E04/A02 governance. |

## 10. Migration / Exit Position

Any implementation may use different products or runtime topology provided the durable identity, state, claim, retry, ownership and handoff invariants remain intact. A future incompatible execution model must supersede this ADR through canonical ADR governance.

## 11. Traceability

Primary realization: `L5-A-08-001` and `L5-A-08-002`.  
Mandatory Stage-3 dependencies: A08 consumes A01, C02 and E04.  
Workflow boundary: `L5-A-05-003`.  
Persistence/concurrency: `ADR-L5-0005`, `ADR-L5-0006`, `ADR-L5-0007`.  
Non-human authority: `ADR-L5-0009`.  
Event delivery/compatibility: `ADR-L5-0010`, `ADR-L5-0011`.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The A08 asynchronous execution and retry pattern is reusable across specifications and consumers, satisfying mandatory ADR criteria. The user explicitly authorized remediation of the production-audit findings on 2026-09-15. Approval does not select a technology and does not imply Locked status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| A08 technical decision owner | Recommended | 2026-09-15 | A08 Stage-3 remediation architecture |
| L5 architecture remediation reviewer | PASS | 2026-09-15 | Production audit corrective action |
| Architecture remediation authorization | Approved | 2026-09-15 | Explicit user request to fix highlighted issues |