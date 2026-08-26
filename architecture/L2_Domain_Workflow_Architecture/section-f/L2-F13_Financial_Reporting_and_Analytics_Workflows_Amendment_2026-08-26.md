# L2-F13 — Financial Reporting & Analytics Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F13 — Financial Reporting & Analytics Workflows v1.0.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-FIN-REPORT-001 |
| Authority | Explicit repository-change authorization recorded 2026-08-26 |

This amendment is part of the canonical L2-F13 architecture. Where this amendment conflicts with L2-F13 v1.0.0, this amendment is authoritative. Unaffected L2-F13 content remains unchanged.

## 1. Canonical Business Actors

The former canonical-section gap for Actors is closed as follows.

| Business role / authority | Responsibility in F13 |
|---|---|
| Accounts Officer | Prepares or coordinates generation of governed financial reports and coordinates authorized pre-publication corrections. |
| Finance Manager | Performs financial-report review and validation where assigned by institutional policy. |
| Financial Report Approval Authority | Makes the final business validation/approval decision for an official financial statement and may approve it for publication or return it for correction. |
| Internal Auditor | Examines report evidence and lineage in a read-only capacity. |

`Financial Report Approval Authority` is an institution-independent business authority. It is mapped by institutional policy to the applicable level of the existing L2-F00 Finance approval hierarchy; it is not a new platform role and does not transfer authorization-policy ownership from Platform Foundation.

## 2. Canonical Pre-Publication Correction Lifecycle

The Financial Report lifecycle is clarified and extended as follows:

```text
Defined → Scheduled → Generating → Generated
Generated → Validated → Published → Archived
Generated → Correction Required → Generating → Generated
```

Rules:

1. `Correction Required` is a pre-publication Financial Report state owned by F13.
2. A generated report enters `Correction Required` when the Financial Report Approval Authority determines that the statement cannot be approved for publication in its current form.
3. Correction may address F13-owned reporting content or configuration, or may wait for a source Finance owner to correct an authoritative source fact through that source owner's own workflow.
4. F13 never modifies source-owned financial truth.
5. After correction, the report re-enters `Generating` and must reach `Generated` again before another validation decision.
6. A report in `Correction Required` cannot be published.
7. `Published` remains immutable and cannot transition to `Correction Required`; a post-publication correction requires a new report version under the existing F13 immutability rule.
8. Existing canonical states and transitions remain unchanged except where this amendment explicitly adds the correction path.

## 3. Canonical Correction Event

F13 publishes the following additional event:

### `FinancialReportReturnedForCorrection`

**Publisher:** Financial Report Aggregate / F13  
**Trigger:** A generated Financial Report is conclusively returned for correction before successful validation.  
**Business meaning:** The current generated report is not approved for publication and requires governed correction before validation can succeed.

The event does not reopen a Financial Period, modify a ledger balance, or transfer ownership of any source Finance fact.

Existing events remain canonical, including:

- `FinancialReportGenerated`
- `FinancialReportValidated`
- `FinancialReportPublished`
- `FinancialReportArchived`

No `FinancialReportApproved` event is added. `FinancialReportValidated` continues to mean that business validation has succeeded and the report is approved for publication.

## 4. Approval, SoD and Audit

- The final validation/approval decision follows the institution-configured Finance approval hierarchy defined by L2-F00.
- Existing L2-F00 segregation-of-duties principles remain applicable to critical financial reporting decisions.
- Return-for-correction, regeneration, validation, approval and publication decisions shall remain auditable.
- Audit evidence shall identify the report version/execution, reporting period, governing report definition, decision authority, reason for correction where applicable, source references, and final outcome at business level.

## 5. Ownership Preservation

This amendment changes no canonical ownership outside F13.

- F13 owns Financial Report lifecycle, report versions, reporting definitions, reporting validation, publication and the new correction state/event.
- F09 retains Financial Period and closing ownership.
- F05 and other operational Finance domains retain their accounting and financial-fact ownership.
- A correction request from F13 is not permission for F13 to edit another aggregate.

## 6. Downstream Effect

L3-F-073 shall consume this amendment and ADR-FIN-REPORT-001. Its former upstream blocker concerning return-for-correction semantics and approval authority is resolved.

## 7. Locked Decisions

The actor mapping, pre-publication correction lifecycle, `FinancialReportReturnedForCorrection` publisher, approval-authority abstraction, and ownership boundaries in §§1–6 are Locked L2 decisions under ADR-FIN-REPORT-001.
