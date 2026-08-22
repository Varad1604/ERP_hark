# Validated Regulatory Snapshot to Approved Submission Package

# 1. Scenario Metadata

**Scenario ID:** L3-H-058  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** Regulatory submission, acknowledgement, correction, resubmission, and reporting-closure scenarios

# 2. Scenario Purpose

Govern institutional review and decision-making for a validated H08 regulatory reporting outcome so that the official Submission Package reaches an approved, rejected, or returned-for-correction result with immutable decision history.

# 3. Business Outcome

The reporting package is approved, rejected, or returned for correction with immutable decision evidence

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A complete validated snapshot is ready for institutional approval.

## 4.2 End Boundary

The frozen outcome requires a Submission Package to exist for institutional approval. H08-W07 names Submission Package as the approval aggregate and its state machine begins at `Draft`, but H08-W08 states that the Submission Package is generated only after institutional approval completes. H08 therefore does not provide one authoritative entry path from the validated snapshot to the package that is being approved.

## 4.3 In Scope

- H08 institutional approval governance for reporting submissions.
- Review of the locked Reporting Snapshot, validation outcome, and mandatory evidence.
- Approved, rejected, or returned-for-correction decision outcomes where a valid Submission Package exists.
- Preservation of immutable approval decision history.

## 4.4 Out of Scope

- Inventing creation of a Draft Submission Package before H08 says package generation occurs.
- Approving a non-existent business aggregate.
- Reordering W07 and W08 locally.
- Changing the validated Reporting Snapshot or source-domain business facts during approval.
- External regulatory submission or acknowledgement.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| NEP, NAAC & AISHE Reporting / H | Owns Reporting Snapshot references, Validation Result, Compliance Evidence Mapping, Submission Package, reporting approval state, approval history, and correction routing. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Institutional Approver | Provides the institutional submission decision where authorized. | Approves / rejects / returns for correction |
| Reporting Coordinator | Participates in the H08 approval workflow and reporting-package review where configured. | Coordinates / reviews |
| Institutional Compliance Officer | May participate as an approver where the configured H08 approval hierarchy requires it. | Reviews / approves where configured |

H08-W07 also lists Department Head, Dean, Registrar, Controller of Examinations, Finance Officer, Principal, and Vice Chancellor as possible approvers. The approval hierarchy is explicitly configurable; this Draft does not invent a universal mandatory sequence.

# 7. Preconditions and Entry Criteria

- The Reporting Snapshot is finalized and locked according to H08 requirements.
- Required reporting validation has succeeded.
- Mandatory evidence is available and the required evidence mapping is approved.
- The institutional approval decision must be made through H08's configured approval governance.
- A valid Submission Package entry state is required, but H08's creation/approval sequence is unresolved.

# 8. Trigger

A complete validated snapshot is ready for institutional approval

# 9. Main Business Flow

1. H08 identifies the validated/locked Reporting Snapshot and its completed validation and mandatory evidence context.
2. H08 determines that the reporting outcome is ready to enter institutional approval governance.
3. The source-defined approval workflow requires review of the reporting package, validation findings, and evidence before an approval decision.
4. The H08 Submission Package state machine requires the package to exist in `Draft` before entering `ApprovalPending`.
5. Stop before creating or approving the Submission Package because H08-W08 defines package generation as occurring only after institutional approval has completed.
6. No approved, rejected, or returned-for-correction package outcome is asserted until the upstream entry sequence is reconciled.

# 10. Alternate Business Flows

If a valid Submission Package is established by an upstream-corrected architecture, H08 supports three institutional decision outcomes: `Approved`, `Rejected`, and `Returned For Correction`. Rejected or returned outcomes route back into correction governance. This Draft does not fabricate the missing package-creation entry path.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Validation is not successful | Institutional approval cannot proceed | Return to the source-defined validation/correction path | NEP, NAAC & AISHE Reporting |
| Mandatory evidence is missing | Submission must not proceed | Complete the required evidence governance before later approval | NEP, NAAC & AISHE Reporting |
| Submission Package entry state cannot be established | Approval outcome cannot be recorded authoritatively | Escalate W07/W08 ordering conflict for upstream resolution | NEP, NAAC & AISHE Reporting |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Validation failure prevents institutional approval | H08 | Only successfully validated reporting can reach approval | L2-H08 BR-007 |
| Missing mandatory evidence blocks submission | H08 | Approval readiness requires mandatory evidence | L2-H08 BR-016; H08-W07 preconditions |
| Submission requires institutional approval | H08 | No external submission may precede the governed institutional decision | L2-H08 BR-011 |
| Approval workflows are configurable | H08 | No single universal approval chain is invented | L2-H08 BR-012; H08-W07 |
| Rejected submissions return to correction | H08 | Rejection does not erase history | L2-H08 BR-013 |
| Approved Submission Packages become immutable completion records | H08 | Approval decision evidence must be preserved | L2-H08 §15.6; §6.11 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Submission Package | Draft | Institutional approval should move package into ApprovalPending | Approved or Rejected; returned-for-correction behavior routes back under H08 governance | H08 | L2-H08 §§6.11, 18.3; H08-W07 |
| Reporting Cycle | SnapshotCreated | Institutional approval phase | ApprovalPending → Approved or Rejected | H08 | L2-H08 §§6.6, 18.1 |

The lifecycle itself is defined, but the authoritative creation of the Draft Submission Package before approval is not.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ComplianceReportApprovalInitiated | H08 | H08 approval participants | Records start of institutional approval | L2-H08 §10.8 |
| SubmissionApproved | H08 | Downstream submission governance | Records institutional approval | L2-H08 §10.8 |
| SubmissionRejected | H08 | H08 correction governance | Records institutional rejection | L2-H08 §10.8 |
| SubmissionReturnedForCorrection | H08 | H08 correction governance | Records return for correction | L2-H08 §10.8 |

These events are source-defined, but L3 does not publish them against an invented package-entry sequence.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No ownership hand-off is required. H08 reviews its own immutable reporting outcome and evidence references; the source facts referenced by that evidence remain with their canonical owners.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H08 defines a configurable institutional approval hierarchy and lists possible approvers. `Institutional Approver` is a source-listed role. No universal approver order is imposed by this scenario.

## 15.2 Separation of Duties, Overrides and Exceptions

The configured H08 approval hierarchy governs the required approvals. No local bypass is permitted for failed validation or missing mandatory evidence.

## 15.3 Business Audit and Evidence Obligations

Preserve the Reporting Snapshot reference, validation result, evidence completeness, approval initiation, each approval/rejection/return decision, decision authority, and immutable approval history once the package-entry conflict is resolved upstream.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The complete frozen outcome cannot be claimed until H08 defines how a Submission Package exists in `Draft`/`ApprovalPending` before the institutional approval workflow that H08-W08 currently says precedes package generation.

## 16.2 Postconditions

The validated snapshot and evidence remain unchanged and traceable. No approval outcome is fabricated against an undefined Submission Package entry path.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H08-W07 makes **Submission Package** the primary aggregate of institutional approval and reviews that package before approving/rejecting it.
- The Submission Package state machine begins `Draft → ApprovalPending → Approved/Rejected`.
- H08-W08 says **Institutional approval completed** is the trigger for creating the Submission Package and marks package generation as occurring after approval.
- Therefore the authoritative source simultaneously requires the Submission Package before approval and creates it after approval.

This materially affects aggregate entry, approval authority application, and the frozen approved/rejected/returned final outcome.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H08 architecture must reconcile W07 and W08 and define the canonical point at which a Submission Package is created relative to institutional approval.
