# L2-H07 — Dashboard Event Vocabulary Normalization — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H07 — Enterprise Dashboard Workflows |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-002 |

## 1. Dashboard Template retirement

The canonical Dashboard Template lifecycle remains:

`Draft → Configured → Validated → Approved → Published → Deprecated → Retired`

`DashboardTemplateRetired` is the canonical H07 event for the transition to `Retired`.

`DashboardTemplateArchived` in the historical event catalogue is superseded vocabulary for this lifecycle fact and shall not be used as a competing canonical event.

## 2. Dashboard Version publication

The canonical Dashboard Version lifecycle remains:

`Candidate → Validated → Approved → Published → Locked → Superseded → Archived`

Event semantics are normalized as follows:

- `DashboardVersionPublished` — canonical Dashboard Version publication fact.
- `DashboardVersionLocked` — records that the published version has become immutable under version governance; it is not the publication event.
- `DashboardPublished` — Dashboard aggregate publication fact; it does not replace `DashboardVersionPublished` for a Dashboard Version.
- `DashboardVersionRolledBack` — canonical governed rollback fact and remains unchanged.

## 3. Publisher and ownership

H07 remains the canonical publisher for these Dashboard/Template/Version facts. A04 continues to own authorization decisions. Source-domain facts and KPI definitions remain owned by their canonical domains.

## 4. Downstream effect

This amendment resolves the event-vocabulary conflicts recorded by L3-H-049 and L3-H-052.
