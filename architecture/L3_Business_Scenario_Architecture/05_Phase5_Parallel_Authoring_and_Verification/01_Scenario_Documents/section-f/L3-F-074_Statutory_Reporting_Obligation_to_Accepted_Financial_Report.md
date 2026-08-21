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
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics; L2-H08 — Regulatory Reporting & Compliance; L2-A06 — Event Infrastructure; L2-A07 — Audit Infrastructure; L3-F-073  
**Referenced By:** Tax, regulatory, funding, legal, compliance and audit reporting processes

---

# 2. Scenario Purpose

Produce and approve an authoritative Finance-owned report, hand it to H08 for governed submission, and retain external acceptance, rejection, correction or resubmission history without transferring ownership of financial facts.

---

# 3. Business Outcome

Finance produces, approves and submits the authoritative financial report and records acceptance, rejection, correction, or resubmission

Source assessment: `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A tax, regulatory, audit, funding, or legal reporting obligation becomes due.

## 4.2 End Boundary

An approved financial report is submitted through H08 and the regulatory acknowledgement is recorded as Accepted/Accepted With Remarks, or a rejected outcome enters governed correction and resubmission with immutable history.

## 4.3 In Scope

- Obligation, deadline, framework and scope resolution.
- Finance-owned report generation and validation.
- Immutable H08 Reporting Snapshot and Submission Package.
- Institutional approval, submission and acknowledgement.
- Correction through successor snapshot and resubmission.
- Complete approval and submission audit trail.

## 4.4 Out of Scope

- Ownership or modification of canonical financial transactions.
- External authority decision-making.
- Vendor/portal technology, transport, document storage or UI.
- Non-financial sections of a wider regulatory submission except as H08 context.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Upstream | L3-F-073 — Approved Financial Statements | May supply the authoritative Finance report. |
| Related | H08 Regulatory Reporting scenarios | H08 owns snapshots, packages, submission and acknowledgement lifecycle. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Financial Reporting & Analytics / F | Primary owner of the authoritative financial report and its values. |
| Regulatory Reporting & Compliance / H | Owns Reporting Snapshot, Submission Package, Submission Record, acknowledgement and resubmission lifecycle. |
| Event Infrastructure / A | Distributes canonical F13 and H08 events. |
| Audit Infrastructure / A | Preserves immutable evidence. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|
| Finance Reviewer / Publisher | Validates and releases the authoritative financial report. | Does not own H08 submission state. |
| Institutional Submission Authority | Approves the Submission Package. | Exact actor is not named in H08. |
| Regulatory Authority | Accepts, remarks on or rejects the submission. | External to SutraERP ownership. |
| Reporting Administrator | Executes governed submission and records acknowledgement. | Does not alter financial facts. |

---

# 7. Preconditions and Entry Criteria

- Tenant, institution, obligation, framework and submission window are resolved.
- Required financial information is available from canonical Finance sources.
- Applicable report definition, regulatory template, validation and evidence rules are active.
- Authorized submission mechanism is available.
- Approval and resubmission policies can be evaluated.

---

# 8. Trigger

A tax, regulatory, audit, funding, or legal reporting obligation becomes due

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Register the due obligation, framework, period, deadline and authority. | H08; Finance | Reporting scope is established. |
| 2 | Generate the authoritative financial report from canonical Finance facts. | F13 | `FinancialReportGenerated` is published. |
| 3 | Validate and approve the report for publication. | F13 | `FinancialReportValidated` and, on release, `FinancialReportPublished` are published. |
| 4 | Supply the immutable Finance report/projection to H08 without transferring financial ownership. | F13 → H08 | H08 receives authoritative financial input. |
| 5 | Generate and lock the H08 Reporting Snapshot with evidence references. | H08 | Submission data becomes immutable. |
| 6 | Create, validate, approve, seal and mark the Submission Package ready. | H08 | Package reaches `ReadyForSubmission`; package events are published. |
| 7 | Execute `SubmitReport` or governed upload and record the Submission Record. | H08 | `ReportSubmitted` and submission history are recorded. |
| 8 | Receive and validate the regulatory acknowledgement. | H08; Regulatory Authority | Accepted, remarks, Rejected or Resubmission Required is identified. |
| 9 | Record acceptance through `RecordSubmissionAcceptance`. | H08 | `SubmissionAccepted` is published and acceptance history retained. |
| 10 | If rejected, record rejection and initiate correction using a new Reporting Snapshot version. | H08; F13 where financial correction is needed | Prior snapshot/report remains immutable. |
| 11 | Approve and submit the corrected package; preserve every resubmission and acknowledgement. | H08 | `ReportResubmitted` and later terminal acknowledgement are recorded. |
| 12 | Preserve report, snapshot, package, submission, authority and correlation evidence. | F13; H08; A07 | Complete statutory history is auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|
| Accepted with remarks | Authority accepts but comments. | Record acceptance and `RegulatoryRemarksReceived`. | Submission is accepted with permanent remarks. |
| Regulatory rejection | Authority rejects. | Record rejection, create successor snapshot/package and resubmit. | History remains immutable. |
| Submission transport failure | Authority does not receive package. | Apply H08 idempotent retry policy. | No duplicate Submission Record is created. |
| Institutional rejection | Approval fails before submission. | Return package for correction. | No external submission occurs. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Financial report is unvalidated. | Package cannot be authoritative. | Stop before H08 approval. | F13 |
| Mandatory evidence is missing. | Submission is blocked. | Complete evidence mapping and revalidate. | H08 |
| Submission window is closed. | External submission is invalid. | Do not submit; follow authority policy. | H08 |
| Acknowledgement is delayed. | Outcome remains pending. | Retain Submitted state and use governed callback/polling contract. | H08 |
| Correction changes financial facts. | Reporting cannot mutate source facts. | Correct through owning Finance domain, then generate successor report/snapshot. | Finance owner; F13; H08 |

---

# 12. Business Rules and Decision Constraints

| Rule | Effect | Source |
|---|---|---|
| Finance remains owner of financial information. | H08 consumes approved projections only. | L2-H08 §3.9; L2-F13 |
| Every submission uses one immutable Reporting Snapshot. | Historical submission is reproducible. | L2-H08 O-004/BR-010 |
| Only approved packages may be submitted. | Institutional approval precedes external delivery. | L2-H08 BR-011/017 |
| Rejected submissions return to correction. | Correction uses successor snapshots. | L2-H08 BR-013, W11 |
| Acknowledgements remain permanently available. | Acceptance/rejection is auditable. | L2-H08 BR-019 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Financial Report | Defined/Scheduled | Generate, validate and publish. | Immutable authoritative report. |
| Reporting Snapshot | Building | `Building → Validated → Locked → Submitted`. | Immutable submission basis. |
| Submission Package | Draft | `Draft → ApprovalPending → Approved → ReadyForSubmission → Submitted → Accepted`. | Accepted statutory package. |
| Rejected Package | Submitted | `Submitted → RejectedByAuthority → Draft` for successor correction. | Governed resubmission path. |
| Financial facts | Existing | Read only. | Unchanged. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

`FinancialReportGenerated`, `FinancialReportValidated`, `FinancialReportPublished`, `SubmissionPackageCreated`, `SubmissionPackageReady`, `ReportSubmitted`, `SubmissionAccepted`, `SubmissionRejected`, `RegulatoryRemarksReceived`, `ResubmissionRequested`, `SnapshotVersionCreated`, and `ReportResubmitted` publish only after their owning transitions succeed.

## 14.2 Contracts and Hand-offs

| Contract | Provider | Consumer | Outcome |
|---|---|---|---|
| Authoritative financial report | F13 | H08 | Approved financial information and traceability. |
| Submission Package | H08 | Regulatory Authority | Approved sealed statutory report. |
| Acknowledgement | Regulatory Authority | H08 | Acceptance, remarks, rejection or resubmission. |
| Audit input | F13/H08 | A07 | Complete report and submission lineage. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Report publication, package approval, submission, correction and acknowledgement recording require scoped authorization.

## 15.2 Approval and Separation of Duty

Institutional approval is mandatory before submission. H08 leaves approval workflow configurable and does not name actors; no role is made canonical here.

## 15.3 Audit Obligations

Retain obligation, deadline, report and source versions, snapshot, evidence, approval, package, submission attempts, responses, correction lineage, actors, timestamps and correlation.

---

# 16. Completion Criteria and Postconditions

- Finance produced and approved the authoritative report.
- H08 locked an immutable snapshot and approved package.
- Submission and external response were recorded.
- Acceptance completed the normal path, or rejection initiated successor correction/resubmission.
- All versions and acknowledgements remain immutable and auditable.
- Ownership of financial facts remained with Finance.

---

# 17. Open Issues, Assumptions and Deferred Questions

No open issue prevents this scenario from proceeding to Architecture Approval. H08 explicitly records Actors as absent, so actor labels remain logical responsibilities rather than invented canonical roles.
