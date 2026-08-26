# ADR-FIN-REPORT-001 — Financial Statement Approval and Correction Governance

| Field | Value |
|---|---|
| ADR ID | ADR-FIN-REPORT-001 |
| Change Type | L2 business-lifecycle and authority clarification |
| Target Locked Document | L2-F13 — Financial Reporting & Analytics Workflows |
| Blocked Downstream Scenario | L3-F-073 — Closed Financial Information to Approved Financial Statements |
| Status | Approved — Incorporated |
| Decision Effect | Effective through the locked L2-F13 amendment dated 2026-08-26 |
| Domain | Section F — Finance / Financial Reporting & Analytics |
| Date | 2026-08-26 |

---

## 1. Need

Locked L2-F13 defines Financial Report generation, validation, publication, immutability, and the events `FinancialReportGenerated`, `FinancialReportValidated`, and `FinancialReportPublished`. It does not define the business path for a generated financial statement that is reviewed and returned for correction before publication, and its canonical crosswalk records that Actors are absent.

This prevents L3-F-073 from representing its frozen outcome — publish authoritative financial statements or return them for correction — without inventing a lifecycle state, event, or decision authority.

## 2. Architecture Constraints

The resolution shall:

- preserve Financial Reporting & Analytics ownership of the Financial Report aggregate and reporting lifecycle;
- preserve ownership of accounting, ledger, period, budget, receivable, payable, treasury, scholarship, asset, and other source facts in their existing Finance owners;
- preserve `FinancialReportValidated` as the business fact that a report is approved for publication;
- preserve published-report immutability and versioned correction after publication;
- remain compatible with the configurable Finance approval hierarchy and SoD principles already defined in L2-F00;
- introduce no API, database, UI, technical RBAC, queue, retry, deployment, or implementation design.

## 3. Decision

### 3.1 Canonical Reporting Actors

The following business responsibilities apply to F13 reporting workflows:

| Business role / authority | Canonical responsibility |
|---|---|
| Accounts Officer | Prepares or coordinates generation of governed financial reports within authorized reporting scope and coordinates pre-publication corrections. |
| Finance Manager | Performs financial-report review and validation activities where assigned by institutional policy. |
| Financial Report Approval Authority | Makes the final business validation/approval decision for an official financial statement and may approve it for publication or return it for correction. |
| Internal Auditor | Examines report evidence and lineage in a read-only capacity. |

`Financial Report Approval Authority` is an abstract business authority, not a universal job title. Institutional policy maps that authority to the applicable level of the existing L2-F00 Finance approval hierarchy, which may include Finance Manager, Registrar, Principal / Director, Trust / Governing Authority, or an additional institution-configured approval level.

### 3.2 Pre-Publication Correction Path

The Financial Report lifecycle is extended only for pre-publication correction:

```text
Defined → Scheduled → Generating → Generated
Generated → Validated → Published → Archived
Generated → Correction Required → Generating → Generated
```

`Correction Required` means the generated report has not received successful business validation/approval and has been returned for correction. It is not a published state and does not alter source-owned financial facts.

A report may repeat the `Correction Required → Generating → Generated` path until it either satisfies validation or is otherwise concluded according to institutional policy. No transition from `Published` back to `Correction Required` is permitted.

### 3.3 Source-Fact Correction Boundary

If the defect exists in F13-owned report definition, reporting configuration, assembly, or presentation, F13 corrects that reporting concern before regeneration.

If the defect exists in a canonical source financial fact, F13 shall not modify it. The owning Finance domain performs the authorized correction under its own lifecycle; F13 then consumes the corrected authoritative fact and regenerates the report.

### 3.4 Canonical Correction Event

`FinancialReportReturnedForCorrection` is the canonical business event published by the Financial Report aggregate when a generated report is conclusively returned for pre-publication correction.

The event means only that the report requires governed correction before validation can succeed. It does not transfer ownership of any source financial fact and does not imply that a Financial Period has been reopened.

Existing events remain unchanged:

- `FinancialReportGenerated`
- `FinancialReportValidated`
- `FinancialReportPublished`
- `FinancialReportArchived`

No separate `FinancialReportApproved` event is introduced because `FinancialReportValidated` already means approved for publication in locked F13.

### 3.5 Approval and SoD

Final validation/approval follows the institution-configured Finance approval hierarchy. Existing L2-F00 segregation-of-duties requirements continue to apply to critical financial activities. This ADR does not weaken or replace those controls and does not hard-code a universal local job title.

## 4. Impact Analysis

### L0 / L1

No change required. Finance ownership remains unchanged.

### Dependency Map

No dependency change required.

### L2

A locked L2-F13 amendment incorporates the actors, correction lifecycle, correction event, and ownership boundary defined here.

### L3

L3-F-073 may now express both the successful publish path and the return-for-correction path without local invention. No scenario ID, title, owner, participants, priority, or canonical publisher is changed.

## 5. Rejected Alternatives

1. Add `Approved` as a separate lifecycle state — rejected because locked F13 already defines successful validation as approval for publication.
2. Return a `Published` report to an earlier state — rejected because F13 requires published reports to remain immutable.
3. Let F13 repair source accounting facts — rejected because reporting is read-only with respect to operational Finance truth.
4. Hard-code one universal CFO, Registrar, Principal, or Board approver — rejected because L2-F00 defines an institution-configurable approval hierarchy.
5. Leave correction semantics to L3 — rejected because L3 cannot invent an upstream lifecycle state, event, or business authority.

## 6. Acceptance Criteria

1. Generated financial statements have a canonical pre-publication correction path.
2. `Correction Required` has one owner: Financial Report / F13.
3. `FinancialReportReturnedForCorrection` has one publisher: Financial Report / F13.
4. `FinancialReportValidated` remains the approval-for-publication business fact.
5. Published reports remain immutable and corrections after publication require a new version.
6. Source Finance ownership remains unchanged.
7. L3-F-073 can complete without an upstream blocker.
8. No L4/L5 implementation architecture is introduced.

## 7. Decision Status

**Approved and incorporated on 2026-08-26 under the explicit repository-change authorization recorded in the architecture working session.**
