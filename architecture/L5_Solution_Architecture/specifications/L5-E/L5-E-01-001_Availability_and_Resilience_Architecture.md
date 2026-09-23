# L5-E-01-001 — Availability & Resilience Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-01-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-01 — Availability & Resilience Architecture  
**Canonical Technical Responsibility:** Repository-wide technical availability model, resilience principles, scoped health states, redundancy expectations and restoration gates without redefining business lifecycle or deployment ownership.  
**Technical Architecture Owner:** L5-E-01 — Availability & Resilience Architecture  
**Assigned Author:** Varad  
**Version:** `1.1.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Technology Selection & ADR Methodology; L5 ADR Register; `L5-A-01-001`; `L5-E-04-001`; `L5-E-01-002`; `L5-E-02-*`; `L5-A-08-*`; applicable B02/C02/D03/D04 architecture; `L4-A-015`; `L4-A-016`; `L4-A-057`.  
**Referenced By:** L5 reliability-aware consumers, `L5-E-01-002`, `L5-E-02-*`.  
**Applicable ADRs:** `ADR-L5-0003`; `ADR-L5-0014`; `ADR-L5-0028`; `ADR-L5-0043`; `ADR-L5-0044`; `ADR-L5-0045`.

---

## 2. Purpose, Scope & Non-Goals

This specification defines the shared technical model for availability and resilience across SutraERP. It establishes scoped technical health (`Healthy`, `Degraded`, `Unavailable`), resilience requirements, failure-domain separation, redundancy expectations and the conditions under which a technical capability may be returned to service.

It does not define business SLA/SLO/RTO/RPO values, business continuity policy, workflow recovery, database replication technology, cloud services, deployment topology or product-specific failover.

---

## 3. Authoritative Constraints & Dependency Gate

1. Health state is technical and shall not redefine business lifecycle state.
2. A capability may be `Degraded` only when a declared degradation profile identifies what remains safe.
3. No degraded path may fabricate success, freshness or authoritative availability.
4. Restoration requires positive integrity/health validation; disappearance of an error is insufficient.
5. E04 owns physical packaging, placement, orchestration and deployment topology.
6. E02 owns capacity/performance mechanisms; E01 consumes their health/capacity signal where relevant.
7. A08 owns job/retry mechanics; C02 owns event delivery/recovery; B02 owns persistence/concurrency.
8. The architecture remains on-premises-primary capable, cloud-capable and provider-neutral.

The Stage-3 minimum dependency gate is explicitly consumed:

| Dependency | Consumption |
|---|---|
| `L5-A-01-001` | logical application/execution boundary and selective-isolation model |
| `L5-E-04-001` | concrete runtime packaging/deployment realization and portability constraints |

---

## 4. Technical Architecture

### 4.1 Scoped Health Model

Every health assessment is bound to a declared technical scope such as capability, execution unit or dependency edge.

- **Healthy** — required operation and authoritative prerequisites are valid.
- **Degraded** — operation is intentionally available with a declared limitation.
- **Unavailable** — required operation cannot be provided safely.

Health inheritance is not automatically global: one unhealthy unit shall not mark unrelated units unavailable.

### 4.2 Resilience Control Points

| Control point | Responsibility |
|---|---|
| Health Evaluator | evaluates declared health evidence for one scope |
| Dependency Guard | prevents an operation when a mandatory dependency cannot satisfy its contract |
| Failure-Domain Boundary | limits propagation across unrelated execution/state owners |
| Degradation Controller | activates only a predeclared safe degradation profile |
| Recovery Validator | verifies state/dependency integrity before return to service |
| Health Publisher | exposes scoped technical health to observability/consumers without redefining business state |

### 4.3 Degradation Profile

A valid profile declares:

- affected capability/scope;
- permitted operations;
- blocked operations;
- authoritative/staleness limits;
- caller-visible technical signal;
- required dependencies that may be bypassed or substituted safely;
- exit/recovery conditions.

If no such profile exists for a material failure, the capability shall fail closed as `Unavailable`.

### 4.4 Redundancy Pattern Vocabulary

Where redundancy is required, logical patterns may include active-passive, active-active with state coordination, N+1 or intentionally degraded singleton operation. Pattern choice must preserve state ownership and consistency and is realized by E04/B02 as applicable. This vocabulary does not mandate a product or node count.

### 4.5 Failure-Domain Isolation

A failure shall be contained to the smallest safe scope consistent with correctness. Isolation boundaries shall preserve:

- Tenant/Institution separation;
- owner-local persistence authority;
- independent consumer/job execution where possible;
- security/trust boundaries;
- independent recovery of unrelated capabilities.

### 4.6 Restoration Gate

A scope may transition back to `Healthy` only after applicable validation establishes:

1. required dependencies are healthy enough for normal operation;
2. authoritative state is consistent/integral under the owning architecture;
3. unresolved recovery work does not make normal operation unsafe;
4. required security/context authority is valid;
5. stale degradation overrides/traffic restrictions are removed in a controlled sequence;
6. health evidence is recorded and observable.

### 4.7 Runtime Boundary

E01 supplies resilience requirements to E04: isolation need, restart/failover eligibility, redundancy intent and restoration criteria. E04 chooses processes/containers/hosts/orchestration and deployment topology.

---

## 5. Failure / Recovery Concerns

| Condition | Required behavior |
|---|---|
| optional dependency failure with approved profile | enter `Degraded`; expose limitation |
| mandatory dependency failure | affected operation/scope becomes `Unavailable` |
| state integrity uncertain | block restoration until owner validation succeeds |
| one consumer/worker failure | isolate where correctness permits; do not fail unrelated scopes |
| recovery action needs owner-state mutation | invoke owner contract; E01 performs no direct cross-owner write |
| runtime instance loss | E04 realizes restart/failover; E01 restoration gate remains authoritative |

---

## 6. Conformance Requirements

| ID | Requirement |
|---|---|
| CR-001 | Every reported health state is scoped and has attributable evidence. |
| CR-002 | `Degraded` is used only with an explicit degradation profile. |
| CR-003 | Degraded operation cannot fabricate authoritative freshness, success or business completion. |
| CR-004 | Failure is contained to the smallest safe scope where ownership/correctness permit. |
| CR-005 | E01 does not directly mutate another owner’s authoritative state during recovery. |
| CR-006 | Return to `Healthy` requires explicit dependency/state/integrity validation. |
| CR-007 | Physical topology/deployment realization remains E04-owned. |
| CR-008 | A08/C02/B02 technical ownership is preserved. |
| CR-009 | Architecture is provider-neutral and on-premises-primary capable. |
| CR-010 | No business SLA/SLO/RTO/RPO is invented by this specification. |

---

## 7. L4 Traceability

| L4 BH ID | Relationship | L5 contribution / boundary |
|---|---|---|
| `L4-A-015-BH-002` | Supporting | technical execution availability must not cause a second Workflow Instance or duplicate ownership. |
| `L4-A-015-BH-006` | Supporting | unavailable/degraded technical dependencies must not bypass the workflow-owned wait condition. |
| `L4-A-016-BH-001` | Supporting | technical failure health classification may support containment while business recovery ownership remains upstream. |
| `L4-A-016-BH-003` | Supporting | resilience/recovery preserves Tenant, execution and correlation context. |
| `L4-A-057-BH-006` | Supporting | restoration after controlled platform change requires post-change integrity validation. |
| `L4-A-057-BH-007` | Supporting | normal technical availability returns only after change+validation completes. |
| `L4-A-057-BH-008` | Direct | failed/unresolved change must not be represented as restored availability. |

`L4-A-019-BH-001` and `L4-A-057-BH-001` are intentionally not claimed because they do not establish the availability/restoration responsibilities previously attributed to them.

---

## 8. Architecture Decisions

**AD-001:** `ADR-L5-0014` is the canonical repository-level resilience/health/degradation/recovery decision.  
**AD-002:** `ADR-L5-0003` constrains logical application/isolation structure; E01 does not redefine it.

---

## 9. Open / Deferred Design

**OD-001:** Numeric availability objectives, thresholds and alert windows require separate governed operational/SLO decisions.  
**OD-002:** **Partially resolved:** OCI/Kubernetes clustered deployment, Podman non-clustered deployment and E04 runtime lifecycle/health conventions are Locked. Exact failover/replication topology and profile-specific managed-service choices remain E04/B02/E03 deployment governance.  
**OD-003:** Capability-specific degradation profiles are defined by the owning L5 capability under `ADR-L5-0014` and must be reviewed before use.

## 2026-09-23 Post-R8 Lifecycle Closure

This specification is promoted from Review to Architecture Approved after the post-R8 upstream authority remediation made its material A016/A057 parent behavior Architecture Approved and the specification's existing HF/open-decision profile was rechecked. No new ownership, business lifecycle, event publisher, or implementation technology is introduced by this promotion. Final corpus lock remains separate governance.
