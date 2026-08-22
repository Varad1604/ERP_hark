# Closed Financial Information to Approved Financial Statements

# 1. Scenario Metadata

**Scenario ID:** L3-F-073  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F09 — Financial Period & Closing; L2-F13 — Financial Reporting & Analytics Workflows  
**Referenced By:** Statutory, management, correction and historical financial-reporting scenarios

# 2. Scenario Purpose

Produce authoritative institutional financial statements from governed closed-period Finance information while preserving operational Finance ownership, the canonical Financial Report lifecycle, report immutability, and reporting traceability.

# 3. Business Outcome

Finance calculates, validates, approves and publishes authoritative statements or returns them for correction

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governed reporting period is ready for institutional financial statements.

## 4.2 End Boundary

The intended end boundary is reached when the statement has completed the frozen calculation, validation, approval and publication outcome, or has been returned for correction. Locked F13 supports generation, validation-as-approval-for-publication and publication, but it does not define the required return-for-correction branch; therefore exact completion is blocked.

## 4.3 In Scope

- Consumption of the authoritative closed-period fact and canonical Finance information.
- Financial statement generation under a governed Report Definition.
- Business validation and publication under the Financial Report lifecycle.
- Preservation of report version, source traceability and audit evidence.
- Identification of the unresolved correction-return path.

## 4.4 Out of Scope

- Reopening or changing Financial Period state.
- Modifying ledger balances or any operational Finance fact.
- Inventing a `Returned`, `Correction Required`, or equivalent Financial Report state/event.
- Statutory external submission and acknowledgement.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-F-074 — Statutory Reporting Obligation to Accepted Financial Report | May consume an authoritative Finance report for regulatory submission. |
| Related | L3-F-078 — Published Financial Report to Correction, Supersession, or Archival | Governs later published-report disposition. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / F | Owns Financial Report generation, validation, publication, versioning and archival. |
| Financial Period & Closing / F | Owns Financial Period state and supplies the authoritative closed-period fact. |
| Other Finance capabilities / F | Retain ownership of canonical financial facts consumed read-only by reporting. |

# 6. Business Actors, Roles and Responsibilities

L2-F13 explicitly has no canonical Actors section. No scenario-specific human reviewer, approver, publisher, or correction authority is invented. The Financial Reporting & Analytics domain owns the supported reporting transitions; the unresolved correction-return authority is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- The applicable Financial Period has reached the source-defined `Closed` state.
- The required canonical Finance information is available from its owning Finance capabilities.
- A governed Report Definition and reporting-period context exist.
- The Financial Report has not bypassed any canonical lifecycle state.

# 8. Trigger

A governed reporting period is ready for institutional financial statements

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Consume the authoritative closed-period fact and establish the reporting period. | Financial Reporting & Analytics / F | Statement preparation is eligible to begin. |
| 2 | Resolve the applicable Report Definition and canonical Finance inputs without changing source facts. | Financial Reporting & Analytics / F | Governed statement context is established. |
| 3 | Generate the financial statement under the Financial Report lifecycle. | Financial Reporting & Analytics / F | The report reaches `Generated`; `FinancialReportGenerated` may be published by Financial Report. |
| 4 | Perform the source-defined business validation. | Financial Reporting & Analytics / F | Successful validation reaches `Validated`; `FinancialReportValidated` records that the report is approved for publication. |
| 5 | Publish the validated statement. | Financial Reporting & Analytics / F | The report reaches `Published`; `FinancialReportPublished` records official release. |
| 6 | Preserve the immutable published version and its source/configuration lineage. | Financial Reporting & Analytics / F | The published statement remains authoritative and traceable. |

The alternative frozen path that returns a statement for correction cannot be completed canonically because F13 defines no such lifecycle transition, event, or business recovery outcome.

# 10. Alternate Business Flows

No scenario-specific alternate business flow toward the same successful published outcome is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Generation does not complete successfully | No statement is available for validation or publication | Do not publish a terminal success event | Financial Reporting & Analytics / F |
| Validation does not succeed | The report cannot be published | F13 requires the report to remain outside `Published`, but does not define the frozen return-for-correction path | Financial Reporting & Analytics / F |
| Source Finance information requires correction | Reporting cannot alter the source fact | Correction remains with the canonical Finance owner before a new report version is generated | Applicable Finance owner |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Reports consume canonical Finance facts without modifying operational state | Financial Reporting & Analytics / F | Statement generation is read-only with respect to source financial facts | L2-F13 FRP-01, FRP-02 |
| Every report belongs to one reporting period and one Report Definition | Financial Reporting & Analytics / F | Statement identity and meaning remain reproducible | L2-F13 FRI-02, FRI-03 |
| Only validated reports may be published | Financial Reporting & Analytics / F | Validation is mandatory before official release | L2-F13 FRI-05 |
| `FinancialReportValidated` means the report is approved for publication | Financial Reporting & Analytics / F | The source defines approval-for-publication as the successful validation business fact | L2-F13 Domain Event Catalog |
| Published reports are immutable; changes require a new report version | Financial Reporting & Analytics / F | Published statements cannot be edited in place | L2-F13 FRI-06; ADR-03 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Period | Closed | Referenced as reporting eligibility; not modified | Closed | Financial Period & Closing / F | L2-F09 |
| Financial Report | Defined / Scheduled | Progresses through supported generation and validation | Generated → Validated → Published | Financial Reporting & Analytics / F | L2-F13 §§21–22 |

No `Returned for Correction` Financial Report state or transition is defined upstream.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| FinancialPeriodClosed | Financial Period | Financial Reporting & Analytics | Establishes authoritative closed-period availability | L2-F09 |
| FinancialReportGenerated | Financial Report | Finance review/audit consumers | Records successful report generation | L2-F13 §29 |
| FinancialReportValidated | Financial Report | Publication/compliance consumers | Records successful validation and approval for publication | L2-F13 §29 |
| FinancialReportPublished | Financial Report | Authorized reporting consumers | Records official publication | L2-F13 §29 |

No canonical correction-return event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Within Section F, Financial Reporting consumes stable business information from the owning Finance capabilities and the closed-period governance fact from F09. No ownership is transferred.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

F13 defines the approval-for-publication business fact through successful validation, but its canonical crosswalk records Actors as absent. The specific business actor or institutional authority performing that approval is therefore unresolved and is not invented here.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override, or exception rule is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

F13 requires every reporting lifecycle transition and reporting configuration change to generate immutable audit evidence. Published statements retain their reporting period, definition/configuration context and source traceability.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen scenario cannot be fully certified complete until the return-for-correction business path and its authority are defined upstream. The supported successful path requires a closed period, generated statement, successful validation/approval-for-publication, and publication.

## 16.2 Postconditions

A successfully published statement is immutable and traceable to canonical Finance information. A failed validation remains outside `Published`; its exact frozen correction-return outcome is unresolved.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| F13 defines no canonical return-for-correction lifecycle state, transition, event, or business recovery for a financial statement that does not pass validation | The frozen alternative outcome cannot be represented exactly | Define the authoritative correction-return semantics in L2-F13 |
| F13 has no canonical Actors section for the approval-for-publication decision | The institutional actor/authority for a P0 financial-statement approval cannot be established | Define the source-supported approval authority in L2-F13 |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
