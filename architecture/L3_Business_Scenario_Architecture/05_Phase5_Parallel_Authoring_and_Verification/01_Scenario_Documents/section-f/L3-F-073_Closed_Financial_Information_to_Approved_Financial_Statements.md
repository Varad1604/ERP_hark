# Closed Financial Information to Approved Financial Statements

# 1. Scenario Metadata

**Scenario ID:** L3-F-073  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F09 — Financial Period & Closing; L2-F13 — Financial Reporting & Analytics Workflows; L2-F13 Locked Amendment 2026-08-26; ADR-FIN-REPORT-001  
**Referenced By:** Statutory, management, correction and historical financial-reporting scenarios

# 2. Scenario Purpose

Produce authoritative institutional financial statements from governed closed-period Finance information, including governed pre-publication correction and regeneration where necessary, while preserving source Finance ownership and report traceability.

# 3. Business Outcome

Finance produces, validates, approves and publishes an authoritative financial statement for the governed reporting period.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governed reporting period is ready for institutional financial statements.

## 4.2 End Boundary

The scenario ends when the Financial Report is validated/approved and published as the authoritative statement.

## 4.3 In Scope

- Closed-period eligibility and canonical Finance source references.
- Financial statement generation under the governed Report Definition.
- Review, validation/approval and publication.
- Governed pre-publication return for correction and regeneration where required.
- Report version, source traceability, events and audit evidence.

## 4.4 Out of Scope

- Reopening or changing Financial Period state.
- Modifying ledger balances or source-owned Finance facts inside F13.
- Editing a published report in place.
- Statutory external submission and acknowledgement.

# 5. Ownership and Participating Domains

| Domain / Capability | Responsibility |
|---|---|
| Financial Reporting & Analytics / F13 | Owns Financial Report generation, correction state, validation/approval, publication, versioning and reporting events. |
| Financial Period & Closing / F09 | Owns Financial Period state and supplies authoritative closed-period eligibility. |
| Other Finance capabilities | Retain canonical ownership of source financial facts consumed read-only by F13. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Authority | Responsibility |
|---|---|
| Accounts Officer | Prepares or coordinates governed statement generation and authorized pre-publication correction. |
| Finance Manager | Performs financial-report review and validation where assigned by policy. |
| Financial Report Approval Authority | Makes the final validation/approval decision and either approves for publication or returns the report for correction. |
| Internal Auditor | Examines report evidence and lineage in a read-only capacity. |

`Financial Report Approval Authority` is institution-independent and maps to the applicable level of the existing Finance approval hierarchy.

# 7. Preconditions and Entry Criteria

- The applicable Financial Period is `Closed`.
- Required canonical Finance information is available from its owning capabilities.
- A governed Report Definition and reporting-period context exist.
- The applicable approval authority can be resolved under institutional Finance policy.

# 8. Trigger

A governed reporting period is ready for institutional financial statements.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm closed-period eligibility and authoritative source references. | Accounts Officer / F13 | Statement preparation is eligible to begin. |
| 2 | Resolve the applicable Report Definition and reporting inputs without changing source facts. | Accounts Officer / F13 | Governed statement context is established. |
| 3 | Generate the statement. | F13 | Financial Report reaches `Generated`; `FinancialReportGenerated` is published. |
| 4 | Review the generated statement against reporting policy and source traceability. | Finance Manager / Financial Report Approval Authority | A validation decision is ready. |
| 5 | Approve a valid statement for publication. | Financial Report Approval Authority | Financial Report reaches `Validated`; `FinancialReportValidated` is published. |
| 6 | Publish the validated statement. | F13 | Financial Report reaches `Published`; `FinancialReportPublished` is published. |
| 7 | Preserve the immutable published version and its source/configuration lineage. | F13 | The authoritative statement remains traceable and historically preserved. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture that bypasses or replaces the same successful published-statement outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Generated statement cannot be approved in its current form. | The report cannot progress to publication and enters `Correction Required`; `FinancialReportReturnedForCorrection` is published. | The report is corrected and re-enters `Generating`, then must reach `Generated` again before re-review and approval. | F13 / Financial Report Approval Authority |
| A reporting-definition, configuration or assembly defect exists within F13. | The current report remains unpublishable. | Accounts Officer coordinates governed F13 correction; the report is regenerated and re-reviewed. | F13 |
| A source-owned Finance fact is defective. | F13 cannot validly publish a statement based on the defective source fact. | The owning Finance capability corrects its authoritative fact through its own workflow; F13 then regenerates from the corrected source. | Canonical source Finance owner with F13 consuming the corrected fact |
| Statement generation fails. | No publishable report is produced. | The report remains incomplete until the governed reporting condition is corrected and generation can validly resume. | F13 |

A published report is never rolled back into correction. Post-publication correction requires a new report version under the upstream immutability rule.

# 12. Business Rules and Decision Constraints

- Reports consume canonical Finance facts without modifying operational financial state.
- Every report belongs to one reporting period and one Report Definition.
- Only validated reports may be published.
- `FinancialReportValidated` means the report is approved for publication.
- `Correction Required` is pre-publication only and is an interruption/recovery position, not successful scenario completion.
- Published reports are immutable; later correction requires a new version.
- Critical financial reporting decisions remain subject to the configured Finance approval hierarchy and applicable SoD controls.

# 13. State and Lifecycle Impact

| Concept | Lifecycle Impact | Owner |
|---|---|---|
| Financial Period | Remains `Closed`; referenced only. | F09 |
| Financial Report | Successful path: `Defined → Scheduled → Generating → Generated → Validated → Published → Archived`; recovery path: `Generated → Correction Required → Generating → Generated`. | F13 |

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Meaning |
|---|---|---|
| `FinancialPeriodClosed` | F09 | Reporting period is authoritatively closed. |
| `FinancialReportGenerated` | Financial Report / F13 | Report generation completed. |
| `FinancialReportReturnedForCorrection` | Financial Report / F13 | Generated report requires pre-publication correction. |
| `FinancialReportValidated` | Financial Report / F13 | Report is approved for publication. |
| `FinancialReportPublished` | Financial Report / F13 | Report is officially released. |

# 15. Business Authorization, Approval and Audit Obligations

- Financial Report Approval Authority owns the final F13 validation/approval decision.
- Institutional policy maps that authority to the applicable Finance approval level.
- Return-for-correction, regeneration, approval and publication decisions must remain auditable.
- F13 cannot use approval authority to alter source-owned financial facts.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The applicable Financial Period remains `Closed`.
- The Financial Report reaches `Generated`, is successfully validated/approved, and reaches `Published`.
- `FinancialReportValidated` and `FinancialReportPublished` are published by F13 as applicable.
- The published version and governing source/configuration lineage are preserved.

## 16.2 Postconditions

- The institution has an authoritative published financial statement for the governed reporting period.
- The published report is immutable.
- Source-owned Finance facts retain their canonical owners.
- Any pre-publication `Correction Required` history remains preserved as recovery history and does not satisfy successful completion by itself.

# 17. Open Issues, Assumptions and Deferred Questions

**Open Issues:** None.  
**Assumptions:** None.  
**Deferred Questions:** None.
