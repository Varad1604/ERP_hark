# L2-H08 — NEP, NAAC & AISHE Reporting Workflows — Authority and Lifecycle Amendment

| Attribute | Value |
|---|---|
| Target | L2-H08 — NEP, NAAC & AISHE Reporting Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-REGREPORT-GOV-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H08 architecture. It fills the recorded actor/authority gap while preserving H08 as the owner of reporting metadata, reporting lifecycle, validation, snapshots and submission governance only.

## 1. Regulatory Reporting Governance Authority

The **Regulatory Reporting Governance Authority** is the canonical institutional business authority for:

- approving or rejecting Reporting Framework Definitions and material versions;
- approving a governed Reporting Collection Plan for an initiated Reporting Cycle;
- approving or returning a validated Submission Package before external submission;
- authorizing retirement/supersession of reporting definitions where required.

This authority governs H08 reporting decisions only. It does not approve or modify the underlying operational facts owned by source domains.

## 2. Reporting Framework Definition Lifecycle

Canonical Reporting Framework Definition lifecycle:

`Draft → Under Review → Approved → Active → Superseded / Retired`

Branch:

`Under Review → Rejected`

A material regulatory change creates a successor Draft version. An Active predecessor becomes `Superseded` only when the successor becomes Active. Historical framework versions remain immutable and traceable.

Canonical events added:

- `ReportingFrameworkDefinitionApproved`
- `ReportingFrameworkDefinitionRejected`
- `ReportingFrameworkDefinitionActivated`
- `ReportingFrameworkDefinitionSuperseded`
- `ReportingFrameworkDefinitionRetired`

## 3. Reporting Collection Plan Lifecycle

For each governed Reporting Cycle, H08 owns a Reporting Collection Plan describing the required source-domain collection scope and evidence obligations without taking ownership of the collected source facts.

Canonical lifecycle:

`Draft → Under Review → Approved → Active → Completed`

Branches:

- `Under Review → Revision Required → Draft`
- `Active → Suspended → Active`
- `Active / Suspended → Cancelled` where the reporting cycle is formally cancelled.

Canonical events added:

- `ReportingCollectionPlanApproved`
- `ReportingCollectionPlanRevisionRequired`
- `ReportingCollectionPlanActivated`
- `ReportingCollectionPlanCompleted`
- `ReportingCollectionPlanCancelled`

## 4. Submission Package Approval Lifecycle

A Submission Package may be presented for institutional approval only after mandatory validation has succeeded against an immutable Reporting Snapshot.

Canonical lifecycle:

`Prepared → Validated → Under Approval → Approved → Submitted → Acknowledged → Archived`

Branches:

- `Under Approval → Returned for Correction → Prepared`
- `Under Approval → Rejected`
- `Submitted → Resubmission Required → Prepared`

A correction or resubmission creates a successor package version linked to the previous package; prior submitted/acknowledged versions are never overwritten.

Canonical events added:

- `SubmissionPackageApprovalRequested`
- `SubmissionPackageApproved`
- `SubmissionPackageReturnedForCorrection`
- `SubmissionPackageRejected`
- `SubmissionPackageResubmissionRequired`

## 5. Source-Data Ownership

- H08 consumes authoritative operational facts only through approved reporting contracts/projections.
- Source domains remain canonical owners of Student, Academic, Workforce, Finance, Examination, Research, Campus and other operational facts.
- H08 validation, snapshot and submission decisions do not mutate those source records.

## 6. Downstream Effect

This amendment resolves the authority/lifecycle gaps identified by L3-H-055, L3-H-056 and L3-H-058.

## 7. Locked Decisions

The authority, lifecycles, immutable versioning and source-data boundary in §§1–5 are Locked L2-H08 decisions under ADR-H-REGREPORT-GOV-001.
