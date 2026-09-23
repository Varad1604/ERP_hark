# L5-E-02-001 — Performance & Capacity Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-02-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-02 — Performance & Capacity Architecture  
**Canonical Technical Responsibility:** Performance and capacity dimension model, admission/measurement boundaries and capacity-planning architecture without redefining business admission, persistence ownership or runtime deployment.  
**Technical Architecture Owner:** L5-E-02 — Performance & Capacity Architecture  
**Assigned Author:** Varad  
**Version:** `1.3.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Technology Selection & ADR Methodology; L5 ADR Register; `L5-A-01-001`; `L5-B-02-001`; `L5-B-02-002`; `L5-B-02-003`; `L5-E-04-001`; `L5-E-01-*`; `L5-A-08-*`; applicable C02/E05 architecture; `L4-A-015`; `L4-A-016`.  
**Referenced By:** `L5-E-02-002`; L5 capabilities that consume capacity/performance constraints.  
**Applicable ADRs:** `ADR-L5-0014`; `ADR-L5-0029`; `ADR-L5-0043`; `ADR-L5-0045`.

---

## 2. Purpose, Scope & Non-Goals

This specification defines the repository performance/capacity architecture: the dimensions that must be measured and planned, the logical capacity envelope attached to a technical scope, and the admission/feedback boundary used by owning capabilities when capacity is constrained.

It does not invent business eligibility, business queue priority, SLA/SLO targets, database products, cloud autoscaling, runtime topology, cache design or user-interface performance targets.

---

## 3. Constraints & Stage-3 Dependency Gate

The Stage-3 minimum dependency set is explicitly consumed:

| Dependency | Consumption |
|---|---|
| `L5-A-01-001` | logical application/module/execution boundaries to which capacity is attributed |
| `L5-B-02-*` | persistence ownership, topology and concurrency constraints that materially affect capacity |
| `L5-E-04-001` | runtime/deployment realization and scaling/placement constraints |

Additional constraints:

1. E02 constrains technical capacity; it does not decide whether a business action is allowed.
2. Capacity signals may cause an owning admission boundary to defer/reject/throttle work, but the owner remains authoritative for the resulting business/workflow behavior.
3. E01 owns health/degradation semantics when capacity exhaustion affects availability.
4. A08 owns Job/Unit execution and retry; C02 owns event-delivery backlogs/recovery.
5. Numeric thresholds are governed workload profiles/configuration, not fabricated universal constants.

---

## 4. Capacity Model

### 4.1 Capacity Dimensions

Each relevant technical scope declares only the dimensions that materially constrain it. Canonical dimension classes include:

- concurrent interactive/session workload;
- synchronous request throughput;
- persisted data volume and growth;
- database connection/query/transaction concurrency;
- background Job/Work Unit parallelism;
- event/message ingress, delivery and backlog;
- external integration concurrency;
- computation workload density;
- search/query workload;
- cache memory/entry pressure where caching is used.

These are technical dimension classes, not fixed targets.

### 4.2 Capacity Envelope

A capacity envelope for a scope contains:

- scope identity and owner;
- applicable dimensions;
- observed demand signals;
- safe operating limits or governed limit references;
- dependency constraints;
- headroom/pressure state;
- admission response available to the owning capability;
- evidence/telemetry source.

### 4.3 Logical Control Points

| Control point | Responsibility |
|---|---|
| Demand Meter | records workload demand using attributable scope/dimension |
| Capacity Evaluator | compares demand/pressure to governed envelope |
| Admission Signal | returns `Accept`, `Defer`, `Throttle` or `RejectTechnical` to the owning boundary |
| Pressure Publisher | exposes capacity state to E01/E05 and owning capability |
| Planning Aggregator | uses observed demand and growth evidence for forward capacity planning |

The Admission Signal is advisory/technical to the owner; it does not itself alter a business lifecycle.

### 4.4 Capacity State

Conceptual technical states:

- `Normal` — within governed safe envelope;
- `Pressured` — approaching or exceeding preferred headroom but safe operation remains available;
- `Saturated` — additional work cannot be safely admitted without defer/throttle/reject action.

If saturation makes safe capability operation impossible, E01 may represent the affected scope as `Degraded` or `Unavailable` under `ADR-L5-0014`.

### 4.5 Admission Interaction

At an owning admission point:

```text
Business/technical request
 -> owner validates business eligibility
 -> E02 capacity evaluation
 -> Accept | Defer | Throttle | RejectTechnical
 -> owner applies the permitted outcome without changing ownership
```

E02 does not make business approval, workflow or publication decisions.

### 4.6 Persistence Capacity

Database/persistence capacity assessment shall consume B02 topology/ownership rather than assume shared cross-domain tables. Relevant signals may include transaction concurrency, query latency, connection saturation, storage growth and owner-local contention. Any remediation that changes topology/partitioning remains B02/E04/ADR governed.

### 4.7 Runtime Capacity

E04 owns concrete process/container/host placement and scaling. E02 supplies load/pressure requirements and capacity evidence. No cloud autoscaler or orchestration platform is mandated.

---

## 5. Failure / Saturation Concerns

| Condition | Required behavior |
|---|---|
| transient pressure | throttle/defer where owner contract permits |
| sustained saturation | stop unsafe additional admission; expose pressure and planning evidence |
| persistence bottleneck | identify owning persistence scope; do not bypass B02 ownership |
| A08 worker saturation | A08 keeps durable work deferred/ready; E02 supplies pressure signal |
| C02 backlog growth | C02 retains delivery ownership; E02 measures capacity/backlog pressure |
| capacity data unavailable | do not fabricate headroom; use conservative owner-defined fallback |

---

## 6. Conformance Requirements

| ID | Requirement |
|---|---|
| CR-001 | Every capacity limit/signal is bound to a declared technical scope and dimension. |
| CR-002 | E02 does not redefine business eligibility or business lifecycle. |
| CR-003 | Saturation produces explicit technical admission behavior; work is not silently dropped. |
| CR-004 | Persistence capacity consumes B02 ownership/topology rather than bypassing it. |
| CR-005 | Runtime realization/scaling remains E04-owned. |
| CR-006 | A08/C02 retain ownership of job execution and event-delivery mechanics. |
| CR-007 | Numeric thresholds/targets are externally governed workload profiles/configuration. |
| CR-008 | Capacity evidence is observable and attributable to the constrained scope. |
| CR-009 | Saturation-to-health mapping consumes E01 rather than inventing new health semantics. |
| CR-010 | Stage-3 A01+B02+E04 dependencies are explicitly consumed. |

---

## 7. L4 Traceability

| L4 BH ID | Relationship | L5 contribution / boundary |
|---|---|---|
| `L4-A-015-BH-001` | Supporting | capacity may technically constrain initiation admission after business/context validation; workflow admission authority remains upstream. |
| `L4-A-015-BH-002` | Supporting | capacity controls may constrain technical execution-instance admission without redefining Workflow Instance semantics. |
| `L4-A-016-BH-003` | Supporting | capacity-constrained recovery must preserve Tenant/execution/correlation context. |

`L4-A-019-BH-001` and `L4-A-057-BH-001` are intentionally removed from traceability because they do not establish event-throughput or restoration-capacity responsibilities.

---

## 8. Architecture Decisions

**AD-001:** Capacity state integrates with the repository resilience model in `ADR-L5-0014` when pressure affects technical availability.  
**AD-002:** Persistence capacity respects `ADR-L5-0005`/`0006`; no alternate cross-owner persistence model is introduced.

---

## 9. Open / Deferred Design

**OD-001:** **Governance resolved by `ADR-L5-0045`:** workload/service classes and evidence-driven profile method are Locked; concrete numeric thresholds, forecast horizons and service objectives remain deployment/workload-profile values.  
**OD-002:** **Partially resolved:** Kubernetes is the clustered production baseline and Podman the non-clustered profile; workload-specific scaling bounds/triggers and provider topology remain E02/E04 profile decisions.  
**OD-003:** **Resolved implementation family:** OpenTelemetry/OTLP + Collector, Prometheus, Loki, Tempo and Grafana are canonical under `ADR-L5-0029`; E02 continues to own workload/capacity dimensions and numeric profile evidence.

## 2026-09-22 Backend Readiness Group 6 Synchronization

**Applicable locked decision:** `ADR-L5-0045`  
Capacity uses Interactive, Asynchronous/Background, Batch/Bulk, Integration and Reporting/Analytics workload classes with measured profile-specific SLO/resource envelopes. No universal latency, concurrency, pool, backlog or autoscaling numbers are canonical.


## 2026-09-23 Post-R8 Lifecycle Closure

This specification is promoted from Review to Architecture Approved after the post-R8 upstream authority remediation made its material A016/A057 parent behavior Architecture Approved and the specification's existing HF/open-decision profile was rechecked. No new ownership, business lifecycle, event publisher, or implementation technology is introduced by this promotion. Final corpus lock remains separate governance.
