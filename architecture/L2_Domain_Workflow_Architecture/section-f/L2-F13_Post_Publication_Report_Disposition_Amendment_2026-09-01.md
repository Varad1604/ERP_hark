# L2-F13 — Post-Publication Financial Report Disposition — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F13 — Financial Reporting & Analytics Workflows and locked 2026-08-26 amendment |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

This amendment resolves the post-publication disposition gap without weakening the existing rule that a Published Financial Report is immutable.

## 1. Canonical disposition semantics

A Published report is never edited in place. If content must change, F13 creates a distinct successor report version and processes it through the normal generation, validation and publication lifecycle.

The following post-publication dispositions are canonical:

- `Published → Superseded` when an approved successor report becomes the authoritative current report for the same governed reporting purpose.
- `Published → Withdrawn` when the Financial Report Approval Authority determines that the report must cease current institutional use and no successor is required at that decision point.
- `Published / Superseded / Withdrawn → Archived` when archival eligibility is satisfied.

`Superseded` and `Withdrawn` preserve the original content, approval evidence and historical lineage. Neither state permits modification of the report version.

## 2. Authority

The existing **Financial Report Approval Authority** is the canonical authority for:

- approving a successor for publication;
- confirming supersession of its predecessor;
- authorizing withdrawal from current use;
- authorizing archival where institutional policy requires an approval decision.

## 3. Events

F13 additionally publishes:

- `FinancialReportSuperseded` — a Published report has been replaced as the authoritative current report by an approved Published successor.
- `FinancialReportWithdrawn` — a Published report has been removed from current institutional use by authorized decision.

Existing `FinancialReportPublished` and `FinancialReportArchived` remain canonical.

## 4. Lineage rules

A supersession shall identify the predecessor and successor report versions. Withdrawal shall preserve the withdrawal reason and authority. Historical consumers may reconstruct every published, superseded, withdrawn and archived version without rewriting source Finance facts.

## 5. Downstream effect

This amendment resolves the upstream blocker recorded by L3-F-078.
