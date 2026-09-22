# ADR-L5-0045 — Measured Performance, Capacity & Workload Profile Standard

**ADR ID:** `ADR-L5-0045`  
**Title:** Measured Performance, Capacity & Workload Profile Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-E-02 — Performance & Capacity Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-E-02-001`; `L5-E-02-002`; constrains A08/C01/C02/B02/E04/F02 capacity-sensitive realization  
**Related ADRs:** `ADR-L5-0014`; `ADR-L5-0036`; `ADR-L5-0037`; `ADR-L5-0041`; `ADR-L5-0044`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

SutraERP now has concrete runtime, persistence, cache, messaging and worker technology choices, but repository-wide numeric latency, throughput, concurrency, queue, pool and autoscaling values would be guesses without representative workload evidence.

## 2. Decision

1. **No universal numeric performance target:** L5 shall not invent one repository-wide API latency, throughput, concurrency, connection-pool, queue-depth, batch-size, memory, CPU or autoscaling threshold.
2. **Workload classes:** capacity profiles classify workload at minimum as:
   - Interactive / synchronous request;
   - Asynchronous / background;
   - Batch / bulk;
   - Integration / connector;
   - Reporting / analytics.
   Additional bounded classes may be introduced by E02 without changing business ownership.
3. **Measured targets:** service objectives and resource envelopes are set from representative workload tests, institutional/business expectations where authoritative, dependency limits and observed operating evidence.
4. **API SLOs:** synchronous latency/availability objectives are contract/profile-specific; no universal millisecond target is canonical.
5. **Database capacity:** SQLx pool sizes, PgBouncer limits, PostgreSQL connection budgets and storage/IO envelopes are deployment-profile values based on measured workload and safe database capacity.
6. **Worker capacity:** A08 worker concurrency, prefetch/batch parameters and parallelism are bounded profile values derived from CPU/memory, PostgreSQL, broker and downstream dependency capacity.
7. **Backlog health:** asynchronous backlog evaluation considers at least age/lag, count/depth and processing/arrival rate. Raw message count alone is not a universal health threshold.
8. **Autoscaling:** scaling policies may consume workload-appropriate signals such as CPU, memory, latency, active concurrency, queue lag/backlog age or domain-appropriate technical pressure. Autoscaling shall not exceed downstream/admission safety bounds.
9. **Performance verification:** representative performance/capacity tests are required before production readiness for workload classes material to the deployment; test evidence feeds E02 profile approval.
10. **Backpressure first:** saturation causes governed defer/throttle/reject/degrade behavior rather than unbounded concurrency or silent dropping.

## 3. Profile Evidence

A governed capacity profile identifies at minimum:

- deployment/environment profile;
- workload class and representative workload assumptions;
- request/job/event mix where material;
- measured latency/throughput/concurrency/resource evidence;
- dependency constraints;
- admission/backpressure behavior;
- scaling bounds and triggers;
- known bottlenecks/margins;
- test evidence/version;
- approval/change history.

## 4. Conformance Rules

- A benchmark result from one environment shall not silently become a universal production SLO.
- Capacity targets are versioned and may change when measured workload/evidence changes.
- Scaling does not bypass Tenant/Institution isolation, D02 authorization, B02 ownership or dependency protection.
- Higher concurrency is not automatically better if it increases downstream saturation or recovery risk.
- Capacity testing uses representative data shape/volume while preserving privacy/security controls.

## 5. Consequences

Backend implementation can proceed with deterministic measurement/governance even though final production numbers are intentionally deployment- and workload-specific.

## 6. Approval Record

**Decision selected:** workload-class and evidence-driven performance/capacity governance; no invented universal numbers; profile-driven DB/worker/scaling/backlog limits; mandatory representative capacity testing before production readiness.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
