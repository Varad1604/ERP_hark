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
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F05 — Accounting & General Ledger; L2-F09 — Financial Period & Closing; L2-F13 — Financial Reporting & Analytics; L2-A06 — Event Infrastructure; L2-A07 — Audit Infrastructure  
**Referenced By:** Institutional leadership, Finance, audit and statutory-reporting consumers

---

# 2. Scenario Purpose

Generate authoritative institutional financial statements from closed, canonical Finance information and govern them through validation, approval-equivalent validation and publication without modifying operational financial facts.

---

# 3. Business Outcome

Finance calculates, validates, approves and publishes authoritative statements or returns them for correction

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governed reporting period is ready for institutional financial statements.

## 4.2 End Boundary

Validated statements are officially Published as immutable financial information products, or validation failure returns the generated statement for governed correction without publication.

## 4.3 In Scope

- Closed-period and reporting-definition validation.
- Trial Balance, Balance Sheet, Income & Expenditure and Cash Flow statement generation.
- Traceability to canonical Finance facts.
- Business validation, publication and distribution eligibility.
- Version, event and audit preservation.

## 4.4 Out of Scope

- Posting or correcting accounting entries.
- Closing or reopening the Financial Period.
- Statutory submission and external acknowledgement.
- Presentation technology, BI tooling, APIs or storage.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Upstream | Financial Period Closing | Supplies `FinancialPeriodClosed`; Reporting owns no period state. |
| Followed by | L3-F-074 — Statutory Financial Report | May use approved statements in an H08 submission. |
| Related | L3-F-078 — Report Correction/Supersession/Archival | Governs later published-report lifecycle changes. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Financial Reporting & Analytics / F | Owns Financial Report, statement generation, validation, publication and archival. |
| Financial Period & Closing / F | Owns period closure and publishes the closed-period fact. |
| General Ledger and other Finance domains / F | Supply canonical read-only financial facts. |
| Event Infrastructure / A | Distributes canonical reporting events. |
| Audit Infrastructure / A | Preserves immutable evidence. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|
| Authorized Finance Reviewer | Validates statement correctness under institutional policy. | Does not modify source financial facts. |
| Authorized Finance Publisher | Releases a validated statement. | Exact role names and approval matrix are not defined in L2-F13. |
| Reporting Service | Generates and versions the Financial Report. | Owns reporting artifacts only. |

---

# 7. Preconditions and Entry Criteria

- Tenant, institution and reporting period are resolved.
- The Financial Period is governed as `Closed` or otherwise approved for statement reporting.
- Canonical Ledger and Finance facts are available through read contracts.
- An active Report Definition and template exist.
- Validation, approval and distribution policies can be resolved.

---

# 8. Trigger

A governed reporting period is ready for institutional financial statements

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the statement-generation requirement for the governed period. | Financial Reporting | A report execution is scheduled. |
| 2 | Validate `FinancialPeriodClosed`, definition, template, scope and authorization. | F09; F13 | Generation prerequisites are established. |
| 3 | Resolve canonical Ledger, cash, receivable, payable and other required Finance facts read-only. | Finance domains; F13 | Authoritative inputs are fixed. |
| 4 | Generate the configured financial statements and preserve input/version references. | Financial Report Aggregate | Report reaches `Generated`; `FinancialReportGenerated` is published. |
| 5 | Reconcile calculations and validate completeness, consistency, period and traceability. | Authorized Finance Reviewer | Valid statement reaches `Validated`; `FinancialReportValidated` is published. |
| 6 | If validation fails, return the non-Published version for correction and regeneration. | Finance Reviewer; Financial Reporting | No official statement is released. |
| 7 | Publish the validated statement for authorized institutional consumers. | Authorized Finance Publisher; Financial Report | Report reaches `Published`; `FinancialReportPublished` is published. |
| 8 | Preserve report version, definition, source facts, validation, publication and correlation evidence. | F13; Audit Infrastructure | Statement is authoritative and auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|
| Validation failure | Statement is incomplete or inconsistent. | Retain outside Published and regenerate after source-governed correction. | No terminal publication event. |
| Period reopened | F09 reopens the period before publication. | Stop publication and regenerate after renewed governance. | Stale statement does not become authoritative. |
| Multiple statement types | The reporting package requires several statements. | Generate each under the same period and policy context. | Every published statement remains independently traceable. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Period is not eligible. | Statement cannot be authoritative. | Reject or wait for period governance. | F09; F13 |
| Canonical input is incomplete. | Calculation is unreliable. | Stop generation and evidence the deficiency. | Owning Finance domain; F13 |
| Generation fails. | No report is available for validation. | Do not publish terminal events; restart through governed execution. | F13 |
| Publication fails. | Statement is not officially released. | Preserve Validated state and recover without regenerating financial facts. | F13 |

---

# 12. Business Rules and Decision Constraints

| Rule | Effect | Source |
|---|---|---|
| Reports consume canonical Finance facts read-only. | Reporting cannot change accounting state. | L2-F13 FRP-01/02 |
| Every report references one definition and period. | Statement meaning and scope are fixed. | L2-F13 FRI-02/03 |
| Validation precedes publication. | Unvalidated statements cannot become official. | L2-F13 FRI-05 |
| Published reports are immutable. | Corrections require a successor version. | L2-F13 FRI-06 |
| Every value is traceable to canonical records. | Audit reconstruction remains possible. | L2-F13 FRP-05 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Financial Period | Closed | Referenced only. | Unchanged. |
| Financial Report | Defined/Scheduled | `Scheduled → Generating → Generated → Validated → Published`. | Immutable official statement. |
| Operational Finance facts | Closed canonical facts | Read only. | Unchanged. |
| Failed statement version | Generated | Retain outside Published pending regeneration. | Non-authoritative evidence. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|
| `FinancialPeriodClosed` | Financial Period Aggregate | Upstream period closure succeeds. |
| `FinancialReportGenerated` | Financial Report Aggregate | Generation succeeds. |
| `FinancialReportValidated` | Financial Report Aggregate | Business validation succeeds. |
| `FinancialReportPublished` | Financial Report Aggregate | Official publication succeeds. |

## 14.2 Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Closed-period fact | F09 | F13 | Reporting eligibility. |
| Canonical financial read models | Finance domains | F13 | Ledger and subsidiary facts. |
| Published statement | F13 | Authorized consumers | Immutable authoritative report. |
| Audit input | F13 | A07 | Generation-to-publication evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Generation, validation and publication require Tenant- and institution-scoped authorization.

## 15.2 Approval and Separation of Duty

L2-F13 equates successful validation with “approved for publication” but does not name approvers or require separate reviewer/publisher roles; none are made canonical here.

## 15.3 Audit Obligations

Record period, definition/template versions, input references, calculations, validation, publication, actors, timestamps and correlation.

---

# 16. Completion Criteria and Postconditions

- Reporting period and canonical inputs were valid.
- Statements were generated and fully traceable.
- Successful validation made them eligible for publication.
- Published statements are immutable and events were emitted after successful transitions.
- Failed validation produced no publication.
- Audit evidence is complete.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|
| F073-ISSUE-01 | Non-blocking | L2-F13 has `Validated` and states its meaning is “approved for publication,” but defines no separate approval state, command, event or named authority. | Treat validation as the source-defined approval-equivalent; define any distinct approval contract before locking if required. |
| F073-ISSUE-02 | Non-blocking | Return-for-correction is part of the frozen outcome but is not a canonical Report lifecycle state/event in F13. | Retain failed output outside Published and regenerate; do not invent a Returned state/event. |

No accounting mutation, approval actor, correction state or technical mechanism is invented.
