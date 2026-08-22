# Management Reporting Request to Governed Financial Report

# 1. Scenario Metadata

**Scenario ID:** L3-F-075  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Authorized institutional management reporting and dashboard consumption

# 2. Scenario Purpose

Produce a governed and reproducible management financial report from authoritative Finance information for an authorized leadership need, while preserving source-domain ownership and keeping any dashboard composition within H07.

# 3. Business Outcome

A reproducible governed financial report is produced from authoritative Finance data

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Authorized leadership requires budget, receivable, payable, cash, aid, asset, or performance information.

## 4.2 End Boundary

The scenario ends when a governed Financial Report has been produced from canonical Finance information with its reporting period, definition/configuration context and source traceability preserved. Where the output is officially released, it follows the canonical validation and publication path.

## 4.3 In Scope

- Resolution of the management reporting requirement.
- Selection of the governed Report Definition and reporting period.
- Read-only consumption of authoritative Finance facts.
- Report generation and, where applicable, validation/publication.
- Preservation of reproducibility and report lineage.
- H07 consumption where dashboard presentation is required.

## 4.4 Out of Scope

- Changing any operational Finance fact.
- Defining dashboard composition, layout, widgets, audience or publication.
- Inventing a canonical leadership requester, reviewer or publisher role where F13/H07 do not define one.
- Statutory submission and regulatory acknowledgement.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / F | Owns Financial Report generation, validation, publication, reporting definitions and report reproducibility. |
| Applicable Finance capabilities / F | Retain ownership of budget, receivable, payable, treasury, aid, asset and other canonical financial facts. |
| Enterprise Dashboard / H07 | May consume governed Finance projections/reports for dashboard presentation; retains dashboard composition ownership. |

# 6. Business Actors, Roles and Responsibilities

The frozen trigger identifies **Authorized leadership** as the initiating business category. L2-F13 and L2-H07 both record their canonical Actors sections as absent, so no more specific requester, reviewer, publisher, or dashboard actor is established here. This source gap is recorded in Section 17.

# 7. Preconditions and Entry Criteria

- The management reporting requirement identifies the financial subject and reporting period needed.
- The requested information is available from canonical Finance owners.
- An applicable governed Report Definition exists.
- H07 involvement, if any, is limited to downstream presentation of governed Finance information.

# 8. Trigger

Authorized leadership requires budget, receivable, payable, cash, aid, asset, or performance information

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the management reporting requirement, scope and reporting period. | Financial Reporting & Analytics / F | The required financial information product is identified. |
| 2 | Resolve the applicable Report Definition and reporting configuration. | Financial Reporting & Analytics / F | A reproducible report context is established. |
| 3 | Consume the required authoritative Finance facts from their owning capabilities without modifying them. | Financial Reporting & Analytics / F | Canonical reporting inputs are available. |
| 4 | Generate the governed Financial Report. | Financial Reporting & Analytics / F | The report reaches `Generated`; `FinancialReportGenerated` records successful generation. |
| 5 | Where the report is to become an official published information product, complete business validation. | Financial Reporting & Analytics / F | The report reaches `Validated`; `FinancialReportValidated` records approval for publication. |
| 6 | Where official publication is required, publish the validated report. | Financial Reporting & Analytics / F | The report reaches `Published`; `FinancialReportPublished` records official release. |
| 7 | Preserve the report's period, definition/configuration and canonical-source lineage so the result is reproducible. | Financial Reporting & Analytics / F | The frozen business outcome is achieved. |
| 8 | If dashboard presentation is needed, make the governed Finance output available to H07 without transferring financial ownership. | F → H07 | H07 may compose the presentation under its own lifecycle. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Governed report required without official publication | The management need is satisfied by a generated governed report and publication is not required by policy | Stop after the governed report is produced and traceability is preserved | Reproducible management report available | F |
| Dashboard presentation required | Leadership needs the governed result in a dashboard context | H07 consumes the Finance-owned information under its dashboard workflows | Dashboard presentation without ownership transfer | H07 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required canonical Finance information is incomplete or unavailable | A reproducible report cannot be produced | Do not publish an authoritative report until source information is complete | Applicable Finance owner; F13 |
| Report generation fails | No governed report outcome exists | Preserve the unsuccessful reporting outcome; no terminal success event is published | F13 |
| Validation fails where publication is required | The report cannot become official | Keep the report outside `Published`; F13 does not define a dedicated correction-return state | F13 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial Reporting consumes canonical Finance information read-only | F13 / Finance source owners | Reporting does not change operational financial truth | L2-F13 FRP-01, FRP-02 |
| Every report is associated with one reporting period and one Report Definition | F13 | The management report remains reproducible | L2-F13 FRI-02, FRI-03 |
| Only validated reports may be published | F13 | Official publication cannot bypass validation | L2-F13 FRI-05 |
| Historical reports preserve the configuration context under which they were generated | F13 | Reproduction remains possible after configuration changes | L2-F13 CP-04 |
| H07 owns dashboard composition while Finance retains financial report/KPI meaning | F13/H07 | Dashboard consumption does not transfer financial ownership | L2-F13 ADR boundary; L2-H07 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Report | Defined / Scheduled | Generate governed management report | Generated; optionally Validated → Published when official release is required | F13 | L2-F13 §§21–22 |
| Canonical Finance facts | Existing source-owned state | Consumed read-only | Unchanged | Applicable Finance owners | L2-F13 ownership principles |
| Dashboard | H07-owned context, if used | Consumes governed Finance output | H07 controls its own lifecycle | H07 | L2-H07 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| FinancialReportGenerated | Financial Report / F13 | Authorized reporting consumers | Records successful report generation | L2-F13 §29 |
| FinancialReportValidated | Financial Report / F13 | Publication/compliance consumers | Records successful business validation and approval for publication | L2-F13 §29 |
| FinancialReportPublished | Financial Report / F13 | Authorized report/dashboard consumers | Records official publication when used | L2-F13 §29 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Applicable Finance owners | F13 | Canonical financial information | Produces the report without ownership transfer | L2-F13 §§13, 38 |
| F13 | H07 | Governed financial report/projection/read model | Enables dashboard presentation while Finance retains report meaning | DEPENDENCY_MAP; L2-F13/H07 ownership boundary |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The frozen trigger requires an authorized leadership need. The canonical leadership role and detailed authorization authority are not defined by F13/H07 and are therefore not invented.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override, or exception rule is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

F13 requires every reporting lifecycle transition to generate immutable audit evidence and historical reports to retain their evaluated configuration context.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A governed Financial Report has been generated from authoritative Finance information.
- The report is associated with its reporting period and Report Definition/configuration context.
- Source financial ownership remains unchanged.
- Where publication is required, the report has completed validation before publication.

## 16.2 Postconditions

The resulting management report is reproducible. H07 may present governed Finance information but does not own the report, financial facts or reporting calculations.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| F13 and H07 have no canonical Actors sections, while the frozen trigger refers to Authorized leadership | The specific initiating leadership role and any report-review authority cannot be stated canonically | Clarify the applicable business actors/roles upstream if required before locking |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
