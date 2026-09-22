# Analytical, Dashboard & Statutory Reporting Data Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-06-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-06 — Reporting, Analytics & Data Projection Architecture  
**Canonical Technical Responsibility:** Analytical/statutory datasets and dashboard projections without transferring KPI/fact ownership.  
**Technical Architecture Owner:** L5-B-06 — Reporting, Analytics & Data Projection Architecture  
**Assigned Author:** Chetan  
**Version:** `1.0.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-B-06-001`; `L5-B-06-002`; `L5-B-02-003`; `L5-D-02-004`; `L5-C-01-*`; `L5-C-02-*`; `L5-A-04-002`; `L5-D-04-001`; `L5-B-05-*`; L2-H07; L2-H08; applicable source reporting domains such as L2-F13.  
**Referenced By:** H07 dashboard composition; H08 regulatory reporting; authorized analytical/reporting consumers.  
**Applicable ADRs:** `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0036`; `ADR-L5-0037`; `ADR-L5-0045`; `ADR-L5-0046`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the technical data architecture for analytical datasets, dashboard-ready projections and statutory/reporting datasets while ensuring that the business meaning of facts, KPIs, reporting definitions and regulatory lifecycles stays with its canonical source/reporting owner.

### In Scope

- analytical dataset materialization from governed source projections/facts;
- dashboard-ready technical projections;
- historical/period-oriented reporting datasets;
- technical support for immutable H08 Reporting Snapshot data;
- source/KPI/model/configuration lineage;
- reconciliation/completeness metadata;
- authorized analytical/report query;
- privacy/masking constraints;
- dataset versioning required for reproducibility;
- data handoff to H07/H08 and source-owned reporting domains.

### Out of Scope

- KPI business definition/approval;
- analytical-model business meaning;
- source-domain report lifecycle;
- H07 Dashboard lifecycle/composition/publication;
- H08 Reporting Cycle/Validation Result/Reporting Snapshot/Submission Package business lifecycle;
- statutory approval/submission authority;
- operational fact ownership;
- BI/warehouse/lake/OLAP product selection.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Source Matrix

| Source | Status | Exact Reference | B06-003 Effect |
|---|---|---|---|
| Candidate Corpus 105 | Architecture Approved | `L5-B-06-003` | Owns analytical/statutory datasets and dashboard projections without KPI/fact ownership transfer. |
| L2-H07 | Locked | §§1–4 | H07 is composition/presentation only; KPI computation/reporting logic remains with owners; consumes authoritative projections. |
| L2-H08 | Locked | §§1–3 | H08 owns regulatory reporting metadata/lifecycle/snapshots and consumes authoritative source information without operational ownership. |
| L2-F13 | Locked | §§1–6, 9–16 | F13 owns financial KPIs, analytical models and dashboard-ready financial projections while operational Finance owns facts. |
| L4-H-048 | Draft | `BH-001`, `BH-003`, `BH-005`–`BH-007` | H07 composes authoritative source projections and retains dashboard governance only. **Specified**. |
| L4-H-057 | Draft v1.0.1 | `BH-001`–`BH-003`, `BH-006`, `BH-007`, `BH-010`, `BH-013` | H08 creates immutable validated/locked Reporting Snapshot from authoritative source facts with lineage. **Specified**. |
| L4-F-077 | Draft | `BH-001`–`BH-011` | Finance-owned KPI/projection generation is governed by F13 definitions/models/config; H07 is only a consumer. **Specified**. |
| B06-001/002 | Review | owned B06 baseline/read model boundaries | Supplies source binding, freshness, rebuild and query controls. |
| D02-004 | Architecture Approved | query/policy enforcement | Protects analytical/report dataset access. |

### 3.2 Constraints

1. An analytical dataset may contain derived values without becoming the canonical owner of their business meaning.
2. Every authoritative KPI/result/projection must point to the source-owned governed definition/model/configuration that gives it meaning.
3. Dashboard data supplied to H07 does not transfer KPI/fact ownership.
4. H08 Reporting Snapshot lifecycle remains H08-owned; B06 may provide its technical data materialization and lineage only.
5. Locked H08 snapshots are not altered in place; later correction uses the H08-governed successor snapshot/version path.
6. Statutory dataset materialization shall not assert completeness/validation/approval beyond actual H08 outcomes.
7. Protected analytics/reporting access remains D02/A04-controlled.
8. Draft L4 mappings remain `Specified`.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Analytical Dataset technical representation;
- Dashboard Projection technical representation;
- Reporting Dataset / Snapshot Data technical representation;
- Dataset Lineage Context;
- Dataset Build/Refresh/Reconciliation Boundary;
- Authorized Analytical/Reporting Query Boundary.

### Must Not Own

- canonical KPI Definition;
- business Analytical Model;
- financial/statutory reporting rules;
- H08 Validation Result or lifecycle;
- H07 Dashboard;
- source business facts/events;
- regulatory submission/approval authority.

---

## 5. Solution Architecture

### 5.1 Common Dataset Flow

```text
Canonical source facts / governed source projections
        ↓
source-owner + scope + definition/model/version resolution
        ↓
B06 analytical/reporting build
        ↓
validation of technical completeness/lineage
        ↓
B02-backed dataset/projection persistence
        ↓
authorized consumption
   ├─ H07 Dashboard
   ├─ H08 Reporting
   └─ source-owned analytical/reporting domain
```

"Technical completeness" does not replace source-domain or H08 business/regulatory validation.

### 5.2 Analytical Dataset

An analytical dataset is a technical read product that may consolidate/derive information under an upstream-governed analytical/KPI definition.

Conceptually it preserves:

```text
AnalyticalDatasetContext
{
  dataset identity/version
  Tenant / Institution scope
  source owner/reference set
  governing KPI Definition / Analytical Model reference where applicable
  governing configuration/version reference
  source period/snapshot/progress references
  lineage/reconciliation evidence
}
```

B06 may execute technical transformation/materialization but shall not invent the formula or semantic meaning.

### 5.3 Dashboard Projection

A Dashboard Projection is a provider-independent, presentation-consumable data projection for H07.

Rules:

- projection content is supplied by its actual owner/reporting domain;
- B06 preserves owner/KPI/model references;
- H07 may filter/present/compose according to H07 governance but does not become KPI owner;
- B06 does not own dashboard layout, widgets, workspace, publication or audience;
- unsuccessful/stale projection does not become a new source fact.

For F13, `FinancialKPICalculated` and `FinancialDashboardProjectionUpdated` remain F13-owned canonical facts/events; B06/C02 may materialize/deliver them technically without publisher transfer.

### 5.4 Statutory / Regulatory Reporting Dataset

For H08, B06-003 provides the technical dataset layer needed to assemble an H08 Reporting Snapshot from source-owned facts.

```text
source-domain references
       ↓
B06 governed collection/materialization
       ↓
dataset completeness + lineage context
       ↓
H08 validation
       ↓
H08 Reporting Snapshot
Building → Validated → Locked
```

The state machine above is H08-owned and is shown only to define the handoff.

B06 must not independently mark an H08 snapshot `Validated` or `Locked`.

Once H08 locks a Reporting Snapshot:

- B06 shall preserve the exact dataset/source lineage used;
- the locked snapshot data basis is immutable for that version;
- correction creates a new H08-governed snapshot/version rather than modifying history.

### 5.5 Reproducibility and Lineage

For an analytical/reporting outcome to be reproducible, B06 preserves references to the materially applicable:

- canonical source owner/data references;
- source reporting period/snapshot/progress basis;
- KPI Definition / Analytical Model / Report Definition reference;
- configuration/version context;
- dataset/projection version;
- transformation/build version reference where material;
- validation/reconciliation evidence reference.

This does not authorize B06 to invent missing business definitions.

### 5.6 Query & Disclosure Control

D02 authorization is applied to the actual requested analytical/reporting resource and scope.

B05 controls masking, minimization, purpose/disclosure restrictions where applicable.

Aggregated data is not automatically non-sensitive; protection follows authoritative classification/privacy policy.

### 5.7 Failure Boundaries

| Failure | Required Outcome |
|---|---|
| Missing canonical source | Dataset/projection cannot claim complete/current authority. |
| Missing KPI/model/report definition | Do not produce authoritative derived meaning. |
| Source reconciliation failure | Preserve discrepancy and block affected authoritative publication/snapshot handoff as upstream requires. |
| H08 validation failure | B06 retains technical data/evidence; H08 controls correction/recollection lifecycle. |
| Stale dashboard projection | Expose freshness context; H07 decides presentation per upstream rules, not B06 business logic. |
| Authorization/privacy failure | Deny or apply canonical masking/minimization; never permissive fallback. |

### 5.8 Events and Handoffs

B06 consumes C02-delivered source events but does not rename or republish source facts under B06 ownership.

Where a source-owned reporting domain publishes a completion event, B06 may update a projection and make it queryable.

H07/H08 events remain published by their canonical owners.

### 5.9 Evidence & Observability

D04 receives lineage/build/reconciliation/access evidence as materially required.

E05 receives operational build/refresh/failure/lag telemetry.

Neither B06 dataset telemetry nor projection state becomes audit truth or operational business truth.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Position |
|---|---|---|
| `AD-001` | Semantic ownership | Dataset materialization does not transfer fact/KPI/reporting ownership. |
| `AD-002` | Dashboard boundary | B06 supplies authoritative projections; H07 owns composition/presentation/governance. |
| `AD-003` | Regulatory boundary | B06 supplies technical dataset/lineage; H08 owns Reporting Snapshot lifecycle, validation and submission governance. |
| `AD-004` | Derivation | Derived values require an upstream-governed definition/model/configuration reference. |
| `AD-005` | Historical reproducibility | Published/locked reporting outcomes retain dataset/source/version lineage sufficient for reconstruction. |
| `AD-006` | Security | D02/A04/B05 controls apply to analytical/reporting data; aggregation is not an authorization bypass. |
| `AD-007` | Technology neutrality | No BI, warehouse, lake, OLAP, semantic-layer or visualization product is selected. |

---

## 7. Security, Privacy, Quality & Operational Impact

- Analytical/statutory datasets preserve Tenant/Institution isolation.
- Cross-scope institutional reporting requires explicit authoritative scope/authorization.
- B05 restrictions apply through projection and export/report consumption.
- Historical reproducibility may require preserving lineage/version references even when source operational state later changes.
- Dataset build/refresh capacity targets are deferred to E02/operational profiles.
- H07 widget/display controls are not a substitute for D02 enforcement.

---

## 8. Architecture Conformance Requirements

- `CR-001`: Every derived KPI/projection value shall retain a reference to the upstream owner/definition/model that gives it business meaning.
- `CR-002`: B06 shall not redefine KPI formulas, report rules, statutory validation rules or source business facts.
- `CR-003`: Dashboard projections supplied to H07 shall preserve source/KPI ownership.
- `CR-004`: B06 shall not own H07 dashboard lifecycle/composition/publication.
- `CR-005`: B06 shall not own H08 Reporting Cycle, Validation Result, Reporting Snapshot or submission lifecycle.
- `CR-006`: Data underlying a locked H08 Reporting Snapshot version shall not be modified in place.
- `CR-007`: Historical analytical/reporting outcomes shall preserve sufficient source/configuration/model/version lineage for governed reconstruction.
- `CR-008`: Protected analytical/reporting queries shall enforce D02 authorization and applicable B05 controls.
- `CR-009`: Source event publishers shall remain unchanged when B06 materializes their facts.
- `CR-010`: No BI/warehouse/OLAP/data-platform product or numeric target is canonicalized here.

---

## 9. Open Decisions & Deferred Design

| ID | Item | Blocking? |
|---|---|---|
| `OD-001` | `L4-H-048`, `L4-H-057` and `L4-F-077` remain Draft; mappings are `Specified`. | **Blocks Architecture Approved; not Draft authoring.** |
| `OD-002` | Exact analytical/reporting physical platform and semantic/query technology remain unresolved. | No. |
| `OD-003` | Dataset retention, refresh frequency and performance targets require authoritative policy/operational profiles. | No for architecture. |
| `OD-004` | Additional domain-specific KPI/report definitions must remain with their respective canonical owners and be integrated only when authoritative sources exist. | No; ongoing controlled extension. |

---

## 10. Traceability Summary

- Canonical responsibility: Candidate Corpus `L5-B-06-003`.
- Material L4: `L4-H-048-BH-001/003/005/006/007`; `L4-H-057-BH-001/002/003/006/007/010/013`; `L4-F-077-BH-001`–`BH-011`.
- Material L2: H07 Enterprise Dashboard; H08 Regulatory Reporting; F13 Financial Reporting & Analytics.
- Material L5: B06-001/002, B02, D02, C01/C02, A04, B05, D04, E05.
