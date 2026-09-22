# Reporting, Analytics & Data Projection Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-B-06-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-06 — Reporting, Analytics & Data Projection Architecture  
**Canonical Technical Responsibility:** Governed reporting/analytics/projection architecture; source domains remain owners of operational facts.  
**Technical Architecture Owner:** L5-B-06 — Reporting, Analytics & Data Projection Architecture  
**Assigned Author:** Chetan  
**Version:** `1.2.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Stage 3 Final Closure & Stage 4 Proceed Decision 2026-09-15; `L5-B-02-003`; `L5-D-02-004`; `L5-C-02-001`; `L5-A-04-002`; `L5-D-04-001`; applicable source-domain L4/L2 architecture.  
**Referenced By:** `L5-B-06-002`; `L5-B-06-003`; authorized reporting/dashboard consumers.  
**Applicable ADRs:** `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0036`; `ADR-L5-0037`; `ADR-L5-0045`; `ADR-L5-0046`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the common L5 technical architecture for read-oriented reporting, analytics and data projections across SutraERP while preserving canonical ownership of every operational fact, KPI definition, reporting rule and business event.

It establishes how authorized canonical source information may be transformed into governed read-side technical products without turning B06 into a second system of record.

### In Scope

- source-owned fact consumption through approved contracts/events;
- read-side projection identity and source-lineage preservation;
- projection build/refresh coordination;
- technical freshness/consistency metadata;
- projection persistence through B02-owned persistence mechanisms;
- authorization-aware projection access;
- source-to-projection traceability;
- failure/staleness/rebuild boundaries;
- technical handoffs to operational read models, analytical datasets, dashboards and statutory reporting data.

### Out of Scope / Non-Goals

This specification does not own:

- operational business facts, transactions, lifecycles or canonical events;
- business KPI definitions/calculation meaning;
- source-domain reporting rules;
- H07 dashboard composition, widgets, layouts, publication or audience assignment;
- H08 Reporting Cycle, Reporting Snapshot, Submission Package or regulatory approval lifecycle;
- source-domain Financial Report/KPI/Analytical Model ownership such as F13;
- generic authorization policy — D02;
- event transport — C02;
- physical database/product selection — B02;
- reporting/BI/warehouse/OLAP/search/cache product selection;
- L6 presentation.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Status | Exact Reference | B06-001 Effect |
|---|---|---|---|
| L5 Candidate Corpus Register 105 | Architecture Approved | `L5-B-06-001` | B06 owns governed reporting/analytics/projection technical architecture while source domains retain operational facts. |
| Stage 3 Closure / Stage 4 Proceed Decision | Locked | Stage 4 table: `L5-B-06`, dependency `B02 + D02 + C02`, Proceed `YES` | Authorizes B06 authoring against stabilized peer boundaries. |
| L2-H07 Enterprise Dashboard | Locked v1.0 | §§1.1–1.8, 2.5, 3–4 | Dashboards consume authoritative Query APIs/read models/projections and do not own business facts/KPIs. |
| L2-H08 Regulatory Reporting | Locked v1.0 | §§1.7–1.8, 2.4–2.5, 3 | Reporting consumes canonical data through Query APIs/read models/reporting projections without becoming operational system of record. |
| L2-F13 Financial Reporting & Analytics | Locked v1.0.0 | §§1–6, 9–16 | F13 owns financial reports/KPI definitions/analytical models/dashboard-ready financial projections; operational Finance retains transaction ownership. |
| L4-H-048 | Draft v1.0.0 | `BH-001`, `BH-003`–`BH-007` | H07 uses authoritative source projections and may not recompute/re-own source KPI/facts. Relation is **Specified** only. |
| L4-H-057 | Draft v1.0.1 | `BH-001`, `BH-002`, `BH-007`, `BH-010`, `BH-013` | Regulatory reporting collects source-owned facts by reference, preserves immutable snapshot lineage and source ownership. Relation is **Specified** only. |
| L4-F-077 | Draft v1.0.0 | `BH-002`, `BH-007`, `BH-008` | Completed Finance-owned KPI/projections may be supplied to H07 without transferring ownership; source/configuration lineage is preserved. Relation is **Specified** only. |
| L5-B-02-003 | Draft v1.0.1 | §§3–6 | Owner-local transaction/concurrency; no distributed ACID or cross-owner persistence mutation. |
| L5-D-02-004 | Architecture Approved v1.0.0 | §§2–5 | Protected reads/queries require enforceable current authorization and scope context. |
| L5-C-02-001 | Architecture Approved v1.0.3 | §§2–6 | Source-domain events remain source-owned; C02 supplies durable delivery and consumer isolation. |

### 3.2 Authoritative Constraints

1. Source domains remain canonical owners of operational facts and business events.
2. B06 projections are read-side technical representations, not authoritative replacements for source aggregates.
3. B06 shall not invent KPI definitions, business calculations, report approval rules, lifecycle states or publishers.
4. H07 consumes projections; it does not transfer dashboard composition ownership into B06.
5. H08 reporting artifacts/lifecycles remain H08-owned even when B06 supplies technical dataset/projection support.
6. Protected projection reads require D02 authorization and validated Tenant/Institution scope.
7. Cross-owner projection refresh shall not use direct source-database writes or distributed ACID.
8. Draft L4 relations used here remain `Specified`, not `Validated`.

### 3.3 Assumptions

No product, storage engine, BI platform, refresh interval, latency target, retention period or dimensional model is assumed.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

B06-001 owns the shared technical architecture for:

- Projection Registration Context;
- Source Binding & Lineage Context;
- Projection Build/Refresh Boundary;
- Projection Freshness Context;
- Projection Query Boundary;
- Projection Rebuild/Reconciliation Boundary;
- common authorization, isolation and evidence handoffs for B06.

These are technical architecture constructs, not new business aggregates.

### Consumes / Collaborates With

| L5 Spec | Supplies / Owns | B06 Use |
|---|---|---|
| `L5-B-02-*` | persistence ownership, transactions, concurrency | Stores B06-owned projection state without direct cross-owner writes. |
| `L5-D-02-*` | authorization/scope policy | Protects projection queries and administrative operations. |
| `L5-C-01-*` | synchronous contracts | Owner-governed query/snapshot acquisition where required. |
| `L5-C-02-*` | event delivery | Event-driven projection updates while preserving source publisher ownership. |
| `L5-A-04-*` | Tenant/Institution isolation | Scope binding for projection build/query. |
| `L5-D-04-*` | audit/evidence | Receives material projection lineage/rebuild/access evidence. |
| `L5-B-05-*` | privacy/classification controls | Constrains projection content/display where applicable. |
| `L5-E-05-*` | operational telemetry | Observes refresh lag, failures and rebuild health. |

### Must Not Own

B06 shall not:

- become a canonical business publisher merely because it consumes source events;
- mutate operational owners to "repair" a projection;
- make projection freshness equivalent to source transactional correctness;
- compute a business KPI unless an upstream owner provides the governed definition/meaning and this B06 contribution is only technical execution/materialization;
- create dashboard/reporting business lifecycles.

---

## 5. Solution Architecture

### 5.1 Baseline Flow

```text
Canonical source owner
    ├─ governed query/read contract via C01
    └─ canonical committed event via C02
                ↓
      B06 Source Admission Boundary
                ↓
  validate source identity + Tenant/Institution scope
                ↓
 Projection Build / Incremental Update
                ↓
 B02-owned physical persistence mechanisms
                ↓
 Projection Freshness + Lineage Context
                ↓
 D02-protected Projection Query Boundary
        ├─ operational consumers → B06-002
        └─ analytical/reporting consumers → B06-003
```

No step transfers business-fact ownership.

### 5.2 Source Admission Boundary

A projection contribution is admitted only when its source can be tied to an authoritative owner/contract/event.

Conceptual context:

```text
ProjectionSourceContext
{
  source-owner reference
  source contract/event reference
  Tenant / Institution scope
  source version/event-position reference where available
  correlation / causation reference where applicable
  projection-definition reference
}
```

These are conceptual categories, not mandated wire fields.

Unknown/unattributable source data is not silently promoted into a governed projection.

### 5.3 Projection Definition Boundary

B06 requires a technical definition sufficient to determine:

- projection identity/purpose;
- source-owner bindings;
- transformation/materialization rule references;
- authorized consumer class/scope;
- freshness requirement class where upstream defines one;
- rebuild/reconciliation basis.

The definition shall reference upstream-owned business meaning rather than duplicate it.

A projection definition does not become owner of a KPI/reporting rule merely because it describes how to materialize the output.

### 5.4 Refresh Modes

B06 supports only governed refresh mechanisms whose source authority is known:

- event-driven update from C02-delivered committed facts;
- owner-governed query/snapshot refresh through C01;
- controlled rebuild from canonical sources.

Exact schedules, frequencies and lag targets remain configuration/operational decisions unless authoritative upstream architecture defines them.

### 5.5 Consistency and Freshness

Projection correctness is expressed as:

```text
source authority
+ identifiable projection rule/version
+ known applied source position/snapshot
+ explicit freshness state/context
```

B06 shall expose enough freshness context for a consumer to determine whether the projection satisfies its upstream-defined need.

A stale projection:

- remains non-authoritative operationally;
- shall not be misrepresented as current;
- shall not be repaired by writing to source owners;
- may require rebuild/reconciliation.

B06 does not invent numeric freshness SLAs.

### 5.6 Authorization and Isolation

Every protected query is evaluated through D02 using the actual resource/scope context.

Dashboard visibility, report availability or possession of a projection identifier does not itself authorize underlying information.

Projection storage/query paths preserve Tenant/Institution scope and shall not merge data across scope boundaries unless an authoritative cross-scope reporting requirement and authorization exist.

### 5.7 Failure and Recovery

| Failure | Required Outcome |
|---|---|
| Source unavailable | Preserve prior projection with explicit freshness limitation; do not fabricate updates. |
| Event delivery repeats | Projection consumer behavior is idempotent where C02 delivery can repeat. |
| Projection update conflict | Use B02 conflict detection; no silent overwrite. |
| Projection logic/source-version mismatch | Stop affected update and preserve diagnostic/evidence context. |
| Projection drift/reconciliation mismatch | Mark affected projection non-current/needs reconciliation; rebuild from canonical source basis. |
| Authorization/scope uncertainty | Deny protected query/update. |

### 5.8 Evidence and Observability

Material evidence includes projection identity, source-owner/reference, governing projection definition/version, refresh/rebuild result, source position/snapshot reference and freshness status where applicable.

D04 owns enterprise evidence persistence. E05 owns operational telemetry such as refresh lag/failure/rebuild health.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Position | Basis |
|---|---|---|---|
| `AD-001` | Ownership | Projections are derived technical read products; source domains remain canonical owners. | Corpus + H07/H08/F13 |
| `AD-002` | Source acquisition | Use governed query/read contracts and/or canonical committed events; no direct source DB ownership bypass. | H07/H08 + C01/C02 |
| `AD-003` | Refresh | Event-driven, governed query/snapshot and controlled rebuild are supported patterns; exact cadence is not fixed here. | H07/H08 |
| `AD-004` | Consistency | Projection freshness is explicit and distinct from source transactional truth. | Corpus B06-002 boundary + H07 |
| `AD-005` | Security | Projection access is resource/scope authorized through D02 and A04. | D02/A04 |
| `AD-006` | Product neutrality | No warehouse, lake, OLAP, BI, database, cache or stream product is selected. | L5 governance |

---

## 7. Security, Privacy, Quality & Operational Impact

- Projection access never bypasses D02 because data is "read-only."
- B05 controls applicable masking/minimization/disclosure constraints.
- Tenant/Institution scope is carried through build, persistence and query.
- Event-driven projection updates tolerate at-least-once-compatible C02 delivery.
- Numeric lag, throughput and capacity targets remain with authoritative operational profiles/E02.
- Projection telemetry is not business audit truth.

---

## 8. Architecture Conformance Requirements

- `CR-001`: Every projection shall identify its canonical source owner(s) and governing source contract/event basis.
- `CR-002`: B06 shall not become owner of operational facts, KPI meaning or source-domain business events.
- `CR-003`: Projection updates shall not directly mutate source-domain persistence.
- `CR-004`: Event-driven updates shall preserve canonical publisher identity and be safe under governed repeated delivery.
- `CR-005`: Protected projection reads shall require D02 authorization and validated scope.
- `CR-006`: Projection freshness/currentness shall be explicit enough for consumers to avoid treating stale data as current.
- `CR-007`: Rebuild/reconciliation shall use canonical source basis and preserve lineage.
- `CR-008`: Cross-owner distributed ACID shall not be introduced for projection refresh.
- `CR-009`: H07 dashboard composition and H08 reporting lifecycle remain outside B06 ownership.
- `CR-010`: No technology product or numeric operational target becomes canonical through this specification.

---

## 9. Open Decisions & Deferred Design

| ID | Item | Blocking? |
|---|---|---|
| `OD-001` | Material L4 sources `L4-H-048`, `L4-H-057` and `L4-F-077` remain Draft; mappings are `Specified`, not `Validated`. | **Blocks Architecture Approved; not Draft authoring.** |
| `OD-002` | Exact reporting/analytics physical platform/product remains unselected. | No for technology-neutral Draft. |
| `OD-003` | Numeric freshness/lag/retention/capacity targets remain source/operational-profile decisions. | No for baseline. |

---

## 10. Traceability Summary

- Canonical responsibility: Candidate Corpus Register 105 `L5-B-06-001`.
- Stage authorization: Stage 4 Proceed Decision, B06 = `YES`.
- Material L4: `L4-H-048-BH-001/003/004/006/007`; `L4-H-057-BH-001/002/007/010/013`; `L4-F-077-BH-002/007/008`.
- Material L2: H07 Enterprise Dashboard; H08 Regulatory Reporting; F13 Financial Reporting & Analytics.
- Core L5 dependencies: B02, D02, C01/C02, A04, D04, B05, E05.


## 2026-09-22 Backend Readiness Group 2 Synchronization

**Applicable locked decision:** `ADR-L5-0036`  
Cross-domain/read-heavy query needs shall use governed B06 projections/read models rather than direct joins across authoritative owner tables. Projections remain derived, non-authoritative and rebuildable.


## 2026-09-22 Backend Readiness Group 2 Synchronization

**Applicable locked decision:** `ADR-L5-0037`  
Valkey may accelerate projection/query access where freshness policy permits, but it shall not become the projection's or source domain's canonical state.
