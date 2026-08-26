# Statutory Reporting Obligation to Accepted Financial Report

# 1. Scenario Metadata

**Scenario ID:** L3-F-074  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics Workflows; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** Regulatory reporting, compliance and external financial submission processes

# 2. Scenario Purpose

Coordinate a Finance-owned authoritative financial report with H08-owned institutional submission governance so a statutory reporting obligation can reach an accepted, rejected, corrected or resubmitted external outcome without transferring ownership of financial facts.

# 3. Business Outcome

Finance produces, approves and submits the authoritative financial report and records acceptance, rejection, correction, or resubmission

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A tax, regulatory, audit, funding, or legal reporting obligation becomes due.

## 4.2 End Boundary

The scenario ends when the authoritative Finance report has been validated for publication, incorporated into H08 reporting governance, submitted, and the external acknowledgement outcome has been recorded; rejection or resubmission follows H08's governed correction lifecycle.

## 4.3 In Scope

- Finance report generation and validation-for-publication.
- H08 reporting snapshot and submission-package governance.
- Institutional package approval.
- External submission and acknowledgement.
- Rejection, correction and resubmission under H08.
- Preservation of Finance ownership of financial information.

## 4.4 Out of Scope

- Modification of source financial transactions by F13 or H08.
- Inventing a Finance actor for F13 validation approval.
- Implementation mechanisms for external transmission or acknowledgement.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / F | Owns the authoritative financial report, report lifecycle, validation-for-publication and financial reporting meaning. |
| Regulatory Reporting & Compliance / H08 | Owns Reporting Snapshot, Submission Package, Submission Record, institutional submission approval, external submission, acknowledgement, correction and resubmission. |

H08 consumes Finance information through approved reporting boundaries and does not become owner of financial facts.

# 6. Business Actors, Roles and Responsibilities

H08 defines possible institutional approvers including Reporting Coordinator, Department Head, Dean, Registrar, Controller of Examinations, Finance Officer, Principal, Vice Chancellor and Institutional Compliance Officer; its approval hierarchy is configurable. F13 itself has no canonical Actors section, so no separate Finance-report approver is invented.

# 7. Preconditions and Entry Criteria

- The statutory obligation, framework and reporting period are known.
- Canonical Finance information required for the report is available.
- The F13 report can progress through generation and validation-for-publication.
- H08 can create and govern the required reporting snapshot and submission package.

# 8. Trigger

A tax, regulatory, audit, funding, or legal reporting obligation becomes due

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Generate the authoritative financial report from canonical Finance information. | Financial Reporting & Analytics / F | `FinancialReportGenerated` records successful generation. |
| 2 | Complete F13 business validation. | Financial Reporting & Analytics / F | `FinancialReportValidated` records that the report is approved for publication. |
| 3 | Make the validated/published Finance report available to H08 under the governed reporting relationship. | F → H08 | H08 receives authoritative financial information without ownership transfer. |
| 4 | Build, validate and lock the H08 Reporting Snapshot. | Regulatory Reporting & Compliance / H | An immutable submission basis is established. |
| 5 | Complete H08 institutional approval for the Submission Package. | Configured H08 approval hierarchy | Package becomes eligible for submission or is rejected/returned for correction. |
| 6 | Submit the approved package and record the Submission Record. | H08 | The statutory submission is recorded. |
| 7 | Record the regulatory acknowledgement. | H08 | Accepted, Accepted With Remarks, Rejected or Resubmission Required is captured. |
| 8 | Where rejected or resubmission is required, use H08 correction and resubmission workflows while preserving prior history. | H08; F only if Finance report content itself must change | A corrected/resubmitted statutory outcome is governed without rewriting history. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Accepted with remarks | Authority accepts with comments | Record acknowledgement and remarks | Accepted statutory outcome with remarks retained | H08 |
| Rejected | Authority rejects | Record rejection and enter H08 correction path | Rejected outcome with preserved history | H08 |
| Resubmission required | Authority requests another submission | Create corrected snapshot/package and resubmit | Resubmitted outcome | H08 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| F13 report does not validate | Finance report is not approved for publication | Do not advance the Finance report to publication/H08 submission | F13 |
| H08 package approval is rejected or returned for correction | External submission cannot proceed | Apply H08's defined correction path | H08 |
| External authority rejects submission | Submission is not accepted | Record rejection and follow H08 correction/resubmission | H08 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| `FinancialReportValidated` means approved for publication | F13 | Finance report may proceed toward official use | L2-F13 Domain Event Catalog |
| Published reports are immutable | F13 | Finance corrections require a new report version | L2-F13 FRI-06; ADR-03 |
| Operational data remains owned by source domains | F/H | Neither F13 nor H08 may re-own operational facts | L2-F13; L2-H08 ownership principles |
| H08 validates before institutional approval and supports Approved, Rejected, Returned For Correction | H08 | Submission-package approval is governed upstream | L2-H08 H08-W07 |
| H08 acknowledgement outcomes include Accepted, Accepted With Remarks, Rejected, Resubmission Required | H08 | External response remains explicit and traceable | L2-H08 H08-W10 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Report | Generated | Validate and, where released, publish | Validated / Published | F13 | L2-F13 §§21–22 |
| Reporting Snapshot | Building | Validate and lock for submission | Validated → Locked → Submitted | H08 | L2-H08 §18.2 |
| Submission Package | Draft | Approval and submission governance | Approved/ReadyForSubmission → Submitted → Accepted or RejectedByAuthority | H08 | L2-H08 §18.3 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| FinancialReportGenerated | Financial Report / F13 | H08, audit, Finance | Finance report generation fact | L2-F13 §29 |
| FinancialReportValidated | Financial Report / F13 | H08, compliance | Approval-for-publication fact | L2-F13 §29 |
| FinancialReportPublished | Financial Report / F13 | H08 and authorized consumers | Official Finance report release | L2-F13 §29 |
| SubmissionApproved | Submission Package / H08 | H08 submission workflow | Institutional package approval | L2-H08 H08-W07 / event catalog |
| SubmissionRejected | H08 submission governance | H08 correction workflow | Rejection outcome | L2-H08 |
| SubmissionReturnedForCorrection | H08 submission governance | H08 correction workflow | Return-for-correction outcome | L2-H08 |
| SubmissionAccepted | Submission Record / H08 | Institutional consumers | External acceptance outcome | L2-H08 |
| ResubmissionRequested | Submission Record / H08 | H08 correction workflow | External resubmission requirement | L2-H08 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| F13 | H08 | Authoritative financial report / financial reporting projection | Supplies Finance-owned information for statutory reporting | L2-H08 §3.9; L2-F13 ownership boundary |
| H08 | Regulatory Authority | Approved Submission Package | Governed statutory submission | L2-H08 H08-W08/W09 |
| Regulatory Authority | H08 | Regulatory acknowledgement | Records accepted/rejected/resubmission outcome | L2-H08 H08-W10 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H08 provides the canonical institutional approval workflow and possible approvers; its hierarchy remains configurable. F13's human actor for Finance report validation remains undefined upstream, so no actor is added locally.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty requirement beyond H08's governed approval workflow is defined by the authoritative sources.

## 15.3 Business Audit and Evidence Obligations

Preserve F13 report lifecycle evidence and H08 snapshot, approval, submission, acknowledgement, correction and resubmission history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Finance report generation and validation-for-publication are complete.
- H08 institutional package approval is complete before submission.
- The submission and external acknowledgement are recorded.
- Rejected/correction/resubmission outcomes follow H08's defined lifecycle.

## 16.2 Postconditions

Finance retains ownership of the financial report and source facts; H08 retains ownership of statutory submission and acknowledgement history.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| F13 has no canonical Actors section for the human validation/approval-for-publication responsibility | The specific Finance business actor cannot be stated | Clarify F13 business actor authority upstream |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
