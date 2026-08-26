# L2-H07 — Enterprise Dashboard Workflows — Governance Lifecycle Amendment

| Attribute | Value |
|---|---|
| Target | L2-H07 — Enterprise Dashboard Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-DASHBOARD-GOV-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H07 architecture. It fills the recorded business-authority gap and completes dashboard/widget retirement semantics without changing H07's presentation-only boundary.

## 1. Dashboard Governance Authority

The **Dashboard Governance Authority** is the canonical institutional business authority for approval, rejection, publication, retirement and restoration decisions on governed institutional dashboards and governed widget definitions.

This is a business authority, not a technical platform role. Section A continues to provide authorization/audit capability without owning H07 decisions.

## 2. Dashboard Lifecycle Completion

Canonical governed Dashboard lifecycle:

`Draft → Under Review → Approved → Published`

Branches:

- `Under Review → Rejected`
- `Published → Retired`
- `Retired → Restored → Published` only where the retired dashboard remains policy-valid and its source projections/contracts remain valid.
- when restoration is not allowed, a successor Dashboard version is created instead of mutating immutable historical evidence.

A Rejected Dashboard may be revised only through a successor Draft version. A Retired Dashboard is unavailable for normal institutional use while its history remains preserved.

Canonical H07 events added:

- `DashboardApproved`
- `DashboardRejected`
- `DashboardPublished`
- `DashboardRetired`
- `DashboardRestored`

## 3. Widget Definition Lifecycle

The H07 Widget Catalog/definition governance lifecycle is:

`Draft → Under Review → Approved → Available`

Branches:

- `Under Review → Rejected`
- `Available → Suspended → Available`
- `Available / Suspended → Retired`

`Suspended` temporarily prevents governed use while retaining configuration/history. `Retired` is the terminal business state for that Widget definition version; a future replacement is a successor definition/version.

Canonical H07 events added:

- `WidgetDefinitionApproved`
- `WidgetDefinitionAvailable`
- `WidgetDefinitionRejected`
- `WidgetDefinitionSuspended`
- `WidgetDefinitionRestored`
- `WidgetDefinitionRetired`

## 4. Source-Fact Boundary

H07 remains presentation/composition only.

- It shall not own or recompute source-domain business facts or KPIs.
- Dashboard publication/retirement affects H07 presentation availability only.
- Widget lifecycle events do not change the lifecycle of any source-domain data.
- Source information continues to be consumed through approved projections/read contracts.

## 5. Downstream Effect

This amendment resolves the authority, rejection, retirement/restoration and widget-lifecycle gaps identified by L3-H-048, L3-H-053 and L3-H-054.

## 6. Locked Decisions

The authority, lifecycles, events and source-fact boundary in §§1–4 are Locked L2-H07 decisions under ADR-H-DASHBOARD-GOV-001.
