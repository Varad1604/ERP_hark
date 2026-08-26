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
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern institutional review of a validated H08 reporting snapshot and produce the authoritative package decision required before external regulatory submission.

# 3. Business Outcome

The reporting package is approved, rejected, or returned for correction with immutable decision evidence

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a complete validated snapshot is ready for institutional approval.

## 4.2 End Boundary

The frozen end boundary requires an approved, rejected, or correction-returned reporting package. Locked H08 is internally inconsistent about whether the Submission Package exists before institutional approval or is created only after approval completes.

## 4.3 In Scope

- Preserve the validated and locked Reporting Snapshot as the immutable reporting basis.
- Confirm required validation and mandatory evidence conditions before approval.
- Conduct the H08 institutional approval decision using the configured approval hierarchy.
- Record approval, rejection, or return-for-correction outcomes where defined.
- Preserve immutable decision evidence and package/snapshot lineage.

## 4.4 Out of Scope

- External regulatory submission or acknowledgement.
- Rewriting the locked Reporting Snapshot after approval review.
- Inventing an order between H08-W07 Institutional Approval and H08-W08 Submission Package Generation.
- Inventing a distinct `ReturnedForCorrection` Submission Package state absent from the H08 state machine.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| NEP, NAAC & AISHE Reporting / H08 | Owns Reporting Snapshot, Validation Result, Compliance Evidence Mapping, Submission Package, reporting approval state, and approval history. |

# 6. Business Actors, Roles and Responsibilities

H08-W07 lists possible institutional approvers including Reporting Coordinator, Department Head, Dean, Registrar, Controller of Examinations, Finance Officer, Principal, Vice Chancellor, and Institutional Compliance Officer. The actual approval hierarchy remains configurable; L3 does not select one approver locally.

# 7. Preconditions and Entry Criteria

- The Reporting Snapshot is locked.
- Validation has succeeded.
- Mandatory evidence is available and evidence mapping has been approved where required.
- The applicable H08 approval hierarchy is established.

# 8. Trigger

A complete validated snapshot is ready for institutional approval

# 9. Main Business Flow

The source-grounded approval activities are:

1. H08 initiates institutional approval against the reporting material governed by the validated snapshot.
2. The applicable configured institutional authority reviews the reporting package context and validation findings.
3. The authority records an approval, rejection, or return-for-correction decision.
4. H08 preserves the approval history and corresponding decision evidence.

A complete canonical sequence cannot be authored because H08-W07 makes `Submission Package` the primary aggregate for approval, while H08-W08 states that `Create Submission Package` occurs only after institutional approval has completed.

# 10. Alternate Business Flows

- Approved outcome: the Submission Package is eligible to progress toward `ReadyForSubmission` under H08 governance.
- Rejected outcome: the package follows the H08 rejection/correction path and cannot be submitted.
- Returned-for-correction outcome: H08 defines the decision and event, but its Submission Package state machine contains no distinct `ReturnedForCorrection` state.

# 11. Failure, Interruption and Business Recovery

Missing validation success or mandatory evidence blocks approval under H08 rules. Rejection or return for correction preserves the existing reporting history and routes the business matter to correction rather than altering the locked snapshot in place.

# 12. Business Rules and Decision Constraints

- BR-007 requires validation failures to prevent institutional approval.
- BR-011 requires institutional approval before submission.
- BR-012 requires approval workflows to remain configurable.
- BR-013 routes rejected submissions to correction.
- BR-016 requires missing mandatory evidence to block submission.
- BR-017 permits submission only for approved Submission Packages.
- Every submission must reference exactly one immutable Reporting Snapshot.

# 13. State and Lifecycle Impact

The H08 Submission Package state machine is `Draft → ApprovalPending → Approved/Rejected`, with `Rejected → Draft`; `Approved → ReadyForSubmission → Submitted → Accepted/RejectedByAuthority → Archived`. H08 also defines a `SubmissionReturnedForCorrection` event but no distinct `ReturnedForCorrection` package state. The source additionally places package creation after institutional approval in H08-W08, conflicting with H08-W07 approval of the package.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H08 defines `ComplianceReportApprovalInitiated`, `SubmissionApproved`, `SubmissionRejected`, and `SubmissionReturnedForCorrection` for institutional approval. The same event name `SubmissionRejected` is also used by H08-W10 for regulatory rejection, so L3 does not redefine its semantics.

## 14.2 Business Contracts and Cross-Domain Hand-offs

This scenario is H08-owned. The reporting snapshot and evidence references originate from prior governed H08/source-domain collaboration; no new source ownership is transferred during institutional approval.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The configured H08 approval hierarchy determines which of the upstream-listed institutional authorities may decide the package outcome. L3 does not promote any one possible approver into a universal authority.

## 15.2 Separation of Duties, Overrides and Exceptions

Approval cannot bypass successful validation or mandatory evidence requirements. A correction decision does not authorize in-place modification of a locked Reporting Snapshot.

## 15.3 Business Audit and Evidence Obligations

Preserve the snapshot reference, validation result, evidence status, approval initiation, reviewer/decision authority as resolved by the configured hierarchy, decision outcome, correction return where applicable, and complete approval history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen completion criterion requires an authoritative package decision with immutable evidence. Locked H08 defines the decision outcomes but does not define a coherent package-creation/approval sequence that L3 can safely apply.

## 16.2 Postconditions

No local package-creation ordering or correction state is invented. The validated snapshot remains immutable and H08 retains ownership of the approval outcome.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**Material workflow/boundary conflict:** H08-W07 Institutional Approval names `Submission Package` as its primary aggregate and approves/rejects that package. H08-W08 then states that institutional approval completion triggers **creation** of the Submission Package. The locked source therefore does not define a coherent order for creating and approving the package.

**Lifecycle gap:** H08 defines `SubmissionReturnedForCorrection`, but the Submission Package state machine has no distinct `ReturnedForCorrection` state or explicit transition for that outcome.

**Event semantic ambiguity:** `SubmissionRejected` is used for both institutional approval rejection and external regulatory rejection in different H08 workflows.

**Architecture impact:** package boundary, lifecycle sequencing, and one rejection-event meaning cannot be resolved at L3 without redesigning H08.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Architecture change control must establish when the Submission Package is created relative to institutional approval, map return-for-correction to the canonical package lifecycle, and disambiguate institutional versus regulatory rejection events.
