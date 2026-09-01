# L2-H04 — Placement Outcome Reporting Clarification — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H04 — Placement Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Reporting scope

H04 Placement outcome reporting is limited to Placement statistics and institutional reporting information derivable from authoritative Placement outcomes and permitted source-domain references already supported by H04.

`PlacementStatisticsGenerated` is the canonical fact that the governed Placement statistics projection is available for authorized institutional use.

This architecture does not introduce:

- a universal diversity framework or new diversity dimensions;
- a separate institutional Placement-report approval/publication lifecycle;
- a generic reporting-period aggregate.

Where an institution later requires additional diversity analytics, those semantics require separate architecture change control identifying source owners and governing rules.

## 2. Reporting boundary

The canonical trigger for the scenario represented by L3-H-028 is closure of a Placement cycle/drive scope supported by H04. Existing completed Placement, offer, joining, internship and employer facts remain authoritative; C/E source facts remain source-owned.

## 3. Downstream effect

L3-H-028 shall be narrowed to this supported H04 reporting outcome. The prior diversity/publication/reporting-period gaps are resolved by scope correction rather than invention of new business architecture.
