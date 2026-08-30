# Closed Financial Information to Approved Financial Statements

## 1. Specification Metadata

**Specification ID:** L4-F-073  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Financial Reporting & Analytics  
**Realizes Scenario:** L3-F-073  
**Primary Business Owner:** Financial Reporting & Analytics  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-073_Closed_Financial_Information_to_Approved_Financial_Statements.md`; L2-F09 — Financial Period & Closing; L2-F13 — Financial Reporting & Analytics Workflows; `L2-F13_Financial_Reporting_and_Analytics_Workflows_Amendment_2026-08-26.md`; ADR-FIN-REPORT-001  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall produce an authoritative Financial Report from governed closed-period Finance information, require source-defined validation before publication, support the governed pre-publication correction path, and preserve the published report and its source/configuration lineage.

The behavior begins when the applicable Financial Period is `Closed`, canonical Finance information and a governed Report Definition are available, and the applicable validation/approval authority can be resolved. It ends when the report reaches `Published` as the authoritative statement with required lifecycle/event evidence preserved. L4 does not reopen the Financial Period, alter source-owned Finance facts, edit a Published report in place, or perform statutory external submission.

## 3. Required Business Context

- The applicable Financial Period is authoritatively `Closed` under F09.
- Required canonical Finance facts remain owned by their source Finance capabilities and are available to F13 read-only.
- One governed Report Definition and reporting-period context apply to the report.
- The Financial Report Approval Authority is resolved under institutional Finance policy for the final validation/approval decision.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-073-BH-001 | The system shall prevent authoritative statement generation when the applicable Financial Period is not `Closed`, required canonical Finance information is unavailable, or the governed Report Definition/reporting-period context cannot be resolved. | L3-F-073 §§7, 9, 12–13 |
| L4-F-073-BH-002 | When generation is eligible, the system shall use canonical Finance information without changing source-owned financial state and shall associate the Financial Report with its governed reporting period and Report Definition. | L3-F-073 §§4–5, 9, 12 |
| L4-F-073-BH-003 | On successful generation, the system shall place the Financial Report at `Generated`. | L3-F-073 §§9, 13 |
| L4-F-073-BH-004 | The system shall prevent transition from `Generated` to `Validated` unless the source-defined final validation/approval decision has succeeded under the applicable Finance approval hierarchy. | L3-F-073 §§6–7, 9, 12, 15 |
| L4-F-073-BH-005 | When the Financial Report Approval Authority returns a Generated report for correction, the system shall place it at `Correction Required` and prevent publication. | L3-F-073 §§9, 11–13, 15 |
| L4-F-073-BH-006 | After governed pre-publication correction, the system shall permit only the source-defined continuation `Correction Required → Generating → Generated` before another validation decision. | L3-F-073 §§11–13 |
| L4-F-073-BH-007 | The system shall permit publication only from `Validated` and shall place the report at `Published` only after successful publication. | L3-F-073 §§9, 12–13, 16 |
| L4-F-073-BH-008 | The system shall preserve a Published report as immutable; any post-publication content correction shall require a distinct report version rather than transition back to `Correction Required`. | L3-F-073 §§4, 11–13, 16 |
| L4-F-073-BH-009 | The system shall preserve the Published report's reporting period, Report Definition/configuration context and canonical-source lineage so the authoritative statement remains reproducible. | L3-F-073 §§4, 9, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-073-BH-010 | On successful generation, the system shall publish `FinancialReportGenerated` as the Financial Report / F13 fact. | L3-F-073 §§9, 14 |
| L4-F-073-BH-011 | When a Generated report is conclusively returned for pre-publication correction, the system shall publish `FinancialReportReturnedForCorrection` as the Financial Report / F13 fact. | L3-F-073 §§11, 14 |
| L4-F-073-BH-012 | On successful validation/approval for publication, the system shall publish `FinancialReportValidated` as the Financial Report / F13 fact. | L3-F-073 §§9, 12, 14, 16 |
| L4-F-073-BH-013 | On successful publication, the system shall publish `FinancialReportPublished` as the Financial Report / F13 fact. | L3-F-073 §§9, 14, 16 |

F09 retains ownership of Financial Period state and `FinancialPeriodClosed`. Other Finance capabilities retain ownership of source financial facts. F13 consumes those facts without ownership transfer.

## 6. Failure / Rejection / Continuation

If generation cannot validly complete, the system shall not expose a successful report-generation outcome. If validation fails, the report shall remain outside `Published`; where the source-defined return-for-correction decision applies, continuation is limited to `Correction Required → Generating → Generated` before re-review. If a source-owned Finance fact is defective, F13 shall not alter it; publication remains blocked until the canonical owner supplies the governed corrected fact and the report is regenerated.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-073-BH-014 | The system shall preserve attributable business evidence for report version/execution, reporting period, Report Definition/configuration, source references, return-for-correction reason where applicable, validation/approval authority and outcome, lifecycle transitions and canonical event publication. | L3-F-073 §§15–16 |

## 8. Observable Completion Conditions

Completion requires the Financial Period to remain `Closed`; the Financial Report to have reached `Generated`, then `Validated`, then `Published`; `FinancialReportValidated` and `FinancialReportPublished` to have been published; the Published version to remain immutable; and its source/configuration lineage to remain available. A `Correction Required` position alone is not successful completion.

## 9. Traceability & Provenance

**Parent:** L3-F-073 — Closed Financial Information to Approved Financial Statements  
**Primary verification sources:** L2-F13 — Financial Reporting & Analytics Workflows; L2-F13 Locked Amendment 2026-08-26  
**Additional verification source:** L2-F09 — Financial Period & Closing

The L2-F13 amendment verifies the actor mapping, pre-publication correction lifecycle, `FinancialReportReturnedForCorrection` publisher and approval semantics already incorporated by the Locked parent.

## 10. Open Issues / Deferred Design

No L4 correctness issue remains. External statutory submission is outside this parent. API/interface design, persistence, event transport, technical access control, UI/UX and other implementation mechanisms remain deferred to downstream architecture.