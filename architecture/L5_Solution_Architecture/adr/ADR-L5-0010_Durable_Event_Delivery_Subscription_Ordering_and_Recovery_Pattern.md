# ADR-L5-0010 — Durable Event Delivery, Subscription, Ordering & Recovery Pattern

**ADR ID:** `ADR-L5-0010`  
**Title:** Durable Event Delivery, Subscription, Ordering & Recovery Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-C-02 — Event & Messaging Architecture  
**Assigned Author:** Architecture remediation for Pallav Stage-2 corpus  
**Reviewers:** SAOS architecture review; remediation explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-12  
**Affected L5 Specifications:** `L5-C-02-001`; `L5-C-02-002`  
**Related ADRs:** `ADR-L5-0007`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

The C02 family requires one reusable technical decision for durable event routing and consumer delivery after the B02-owned Durable Publication Intent handoff. The decision affects publication topology, independent subscriptions, acknowledgement, duplicate tolerance, ordering, retry, dead-message isolation and replay. These are reusable family-wide technical semantics and therefore require ADR governance rather than remaining only as local specification decisions.

## 2. Scope & Affected Architecture

This ADR governs technical messaging behavior after the `ADR-L5-0007` handoff exists. It does not define canonical business-event names, event meaning, publisher authority, owner-local state transactions, consumer-domain business outcomes, broker products, protocols, numeric retry schedules, retention periods or service targets.

`L5-C-02-001` owns baseline publication, routing, subscription and consumer-isolation architecture. `L5-C-02-002` owns durable delivery attempts, acknowledgement, retry/recovery, ordering scope, dead-message handling and replay.

## 3. Authoritative Constraints

1. C02 begins delivery responsibility only after the B02-owned durable publication handoff exists.
2. Canonical business-event publisher ownership remains upstream and unchanged.
3. Critical delivery shall use durable capability and shall tolerate duplicate delivery.
4. Critical consumers shall be idempotent where repetition is possible.
5. Consumer acknowledgement shall occur only after successful owned processing outcome is established.
6. Consumer subscriptions shall be independently recoverable so one consumer failure/backlog does not block unrelated consumers.
7. Ordering shall be scoped only where correctness requires it; no repository-wide global ordering guarantee exists.
8. Retry shall distinguish recoverable from non-recoverable failure and shall be bounded; numeric policy values remain governed configuration/operational-profile concerns.
9. Exhausted or non-recoverable delivery shall be isolated without mutating the original published event.
10. Replay shall be privileged, Tenant-aware where applicable, observable, rate-controlled and shall preserve normal validation/idempotency boundaries.
11. Exactly-once processing is not a platform-wide guarantee and shall not be inferred from transport behavior.
12. No broker, queue, protocol or vendor product is selected by this ADR.

## 4. Decision Criteria

Preserve canonical event ownership, prevent single-consumer failure propagation, support restart/recovery, provide deterministic acknowledgement boundaries, avoid global serialization, preserve replay safety, remain technology-neutral and remain compatible with `ADR-L5-0007`.

## 5. Alternatives Considered

### A — Best-effort publication with ephemeral consumer delivery
Rejected for critical ERP facts because restart or consumer unavailability can lose required delivery.

### B — Broker-level exactly-once as a repository-wide guarantee
Rejected because end-to-end business processing spans consumer-owned state and cannot be guaranteed solely by transport semantics without materially stronger coupling.

### C — Durable at-least-once-compatible delivery with independent subscriptions, idempotent consumers and scoped ordering
Selected.

### D — One globally ordered event stream
Rejected because unrelated business streams would be unnecessarily serialized and availability/scalability would become coupled.

## 6. Evaluation

| Criterion | A | B | C | D |
|---|---|---|---|---|
| Critical durability | Weak | Strong | Strong | Strong |
| Consumer isolation | Weak | Acceptable | Strong | Weak |
| End-to-end correctness clarity | Weak | Weak/ambiguous | Strong | Acceptable |
| Recovery control | Weak | Acceptable | Strong | Acceptable |
| Scalability | Strong | Acceptable | Strong | Weak |
| Technology neutrality | Strong | Weak | Strong | Strong |

## 7. Decision

SutraERP C02 shall use a durable, at-least-once-compatible event-delivery model for critical event flows, implemented through independently managed consumer subscriptions and idempotent consumer processing.

The family-wide semantics are:

1. topic-oriented publish/subscribe is the logical routing model; physical broker topology remains product-specific and unselected;
2. each eligible durable consumer has an independent delivery outcome;
3. acknowledgement follows successful owned consumer processing;
4. duplicate delivery is permitted and must be safe;
5. ordering is explicit and scoped to a declared business/aggregate stream where required, never global by default;
6. retry is bounded and failure-class aware;
7. unrecoverable/exhausted delivery is isolated as dead-message/quarantine state for the affected consumer only;
8. replay preserves event identity plus replay provenance and re-enters normal authorization, validation and idempotency controls;
9. one slow or failed consumer shall not block unrelated consumers;
10. operational telemetry is emitted to the E05-owned observability architecture without transferring business or audit ownership.

## 8. Consequences

Positive: durable critical delivery, restart safety, independent consumer failure handling, explicit processing boundary, controlled replay and scalable ordering.  
Cost: consumers require idempotency, duplicate-aware processing, durable subscription state and governed recovery operations.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Duplicate delivery causes repeated business effects | Require consumer idempotency and consumer-owned processing controls. |
| Replay bypasses normal controls | Replay re-enters normal authorization, validation and idempotency paths. |
| Ordering is over-applied | Require explicit scoped ordering; prohibit global ordering assumption. |
| Dead-message store leaks sensitive data | Apply D03 security/classification controls and least-privilege access. |
| Retry storms amplify failure | Bound retries, classify failure and rate-control recovery. |
| Product-specific semantics leak into canonical architecture | Keep broker/protocol/product selection outside this ADR until separately governed. |

## 10. Migration / Exit Position

No product is selected. Any future messaging platform or replacement shall preserve the invariants in this ADR or explicitly supersede it through governed ADR change.

## 11. Traceability

Primary realization: `L5-C-02-001_Event_and_Messaging_Architecture_Baseline.md` and `L5-C-02-002_Durable_Event_Delivery_Ordering_and_Failure_Handling_Architecture.md`.  
Upstream boundary: `ADR-L5-0007`.  
Authoritative L4 behavior: `L4-A-019` and `L4-A-020` under the current L5 validation authority assumption.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** Shared C02 delivery semantics materially affect multiple C02 specifications and downstream consumers, satisfying mandatory ADR criteria. Controlled Stage-2 remediation was explicitly authorized on 2026-09-12. This approval does not select a messaging product and does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| C02 technical decision owner | Recommended | 2026-09-12 | C02-001/C02-002 architecture |
| SAOS architecture reviewer | PASS | 2026-09-12 | Pallav Stage-2 production architecture review |
| Architecture remediation authorization | Approved | 2026-09-12 | Explicit user instruction to fix highlighted issues |
