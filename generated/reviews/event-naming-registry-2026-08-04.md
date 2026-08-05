# Event Naming Registry — Collision Resolution Proposal

**Finding:** INT-016 (Type 1 — Terminology, High)
**Date:** 2026-08-04
**Status:** APPROVED AND APPLIED 2026-08-04
**248 publisher-side and 22 consumer-side occurrences were renamed. 10 references are held pending your decision — see section 9.**

---

## 1. Why this exists

`L2-A06_Event_Infrastructure_Workflows.md` (lines 780-804, 832, 1102-1104) requires
that a canonical event name have exactly one semantic definition and be **globally
unique**. Today several names are published by two or more documents for different
aggregates, so event routing, schemas, subscriptions, observability and replay
cannot infer which fact a name represents.

## 2. What counts as a collision

A collision is **two or more documents publishing the same event name for different
aggregates**.

A name appearing in many documents is *not* automatically a collision. The common
and correct pattern is one publisher and many consumers — for example
`InstitutionCreated` is published only by B01 and referenced by A02, A10, F09 and
J00. **Renaming that would introduce a defect, not fix one.** Every name below was
checked against this distinction.

## 3. Method and its limits

Event names were extracted from all 107 L2 documents and attributed to the document
that declares them, then classified by the heading they sit under (published /
consumed / other).

Two adjustments were necessary:

- **Section review documents (`L2-X00`) were excluded as publishers.** They
  aggregate their whole section's events, which otherwise produced false collisions
  such as `TimetablePublished` appearing to be published by both D00 and D03.
- **Heading classification is imperfect.** 198 documents use an ambiguous
  `Domain Events` heading that mixes published and consumed events, and 16 documents
  have unbalanced code fences. Where the 2026-08-03 review verified a collision by
  hand that automated classification filed as uncertain, the review's judgement was
  taken. Those rows are marked `review` in the Source column.

**Consequence: the table in section 5 is a proposal to be read, not a script to be
run.** 17 of the 53 names come from manual review rather than machine confirmation.

## 4. Naming convention

Proposed names use **full words**, qualified by the owning aggregate or domain:
`EmployeeLeaveApproved`, `HostelLeaveApproved`, `LibraryReservationCreated`.

Abbreviated forms (`EmpLeaveApproved`) were considered. They are shorter, but they
require a governed abbreviation dictionary of their own — Emp, Lib, Infra, Sync,
Exam — and an ungoverned abbreviation set is how the current terminology drift
began. Length is cheap in generated code; ambiguity is not.

**If you prefer the abbreviated form, say so and the whole table can be regenerated
in one pass.** The convention is a single decision, not 111 decisions.

## 5. Proposed renames

| # | Current name | Publisher | Proposed name | Source |
|---:|---|---|---|---|
| 1 | `AllocationUpdated` | L2-G04 | `TransportAllocationUpdated` | review |
| 2 | `AllocationUpdated` | L2-G10 | `InfrastructureAllocationUpdated` | review |
| 3 | `AlumniTransitionCompleted` | L2-C08 | `AlumniTransitionCompleted` | scan |
| 4 | `AlumniTransitionCompleted` | L2-H06 | `AlumniTransitionCompleted` | scan |
| 5 | `AlumniTransitionInitiated` | L2-C05 | `QualificationAlumniTransitionInitiated` | scan |
| 6 | `AlumniTransitionInitiated` | L2-C08 | `AlumniTransitionInitiated` | scan |
| 7 | `ApprovalCompleted` | L2-C08 | `AlumniTransitionApprovalCompleted` | scan |
| 8 | `ApprovalCompleted` | L2-D02 | `FacultyAssignmentApprovalCompleted` | scan |
| 9 | `ApprovalInitiated` | L2-D02 | `FacultyAssignmentApprovalInitiated` | scan |
| 10 | `ApprovalInitiated` | L2-H08 | `ComplianceReportApprovalInitiated` | scan |
| 11 | `AuditRecorded` | L2-A02 | `PersonAuditRecorded` | scan |
| 12 | `AuditRecorded` | L2-C01 | `AdmissionAuditRecorded` | scan |
| 13 | `AuthenticationFailed` | L2-A02 | `PersonAuthenticationFailed` | scan |
| 14 | `AuthenticationFailed` | L2-H10 | `ExternalProviderAuthenticationFailed` | scan |
| 15 | `CommitteeRecommendationRecorded` | L2-I14 | `ExamCommitteeCommitteeRecommendationRecorded` | review |
| 16 | `CommitteeRecommendationRecorded` | L2-J01 | `ProcurementCommitteeRecommendationRecorded` | review |
| 17 | `ComplaintRegistered` | L2-G01 | `FacilityComplaintRegistered` | review |
| 18 | `ComplaintRegistered` | L2-G04 | `TransportComplaintRegistered` | review |
| 19 | `ConflictDetected` | L2-B04 | `CurriculumConflictDetected` | scan |
| 20 | `ConflictDetected` | L2-D03 | `TimetableConflictDetected` | scan |
| 21 | `CourseOfferingPublished` | L2-D01 | `CourseOfferingPublished` | scan |
| 22 | `CourseOfferingPublished` | L2-D06 | `ElectiveCourseOfferingPublished` | scan |
| 23 | `CredentialExpired` | L2-A03 | `AuthCredentialExpired` | scan |
| 24 | `CredentialExpired` | L2-H10 | `ExternalProviderCredentialExpired` | scan |
| 25 | `EmergencyContactUpdated` | L2-C06 | `GuardianEmergencyContactUpdated` | scan |
| 26 | `EmergencyContactUpdated` | L2-E07 | `EmployeeEmergencyContactUpdated` | scan |
| 27 | `EnrollmentRejected` | L2-D06 | `ElectiveEnrollmentRejected` | scan |
| 28 | `EnrollmentRejected` | L2-H10 | `ExternalProviderEnrollmentRejected` | scan |
| 29 | `ExceptionCaseClosed` | L2-D06 | `ElectiveExceptionCaseClosed` | scan |
| 30 | `ExceptionCaseClosed` | L2-I15 | `ExamExceptionExceptionCaseClosed` | scan |
| 31 | `ExceptionResolved` | L2-B04 | `CurriculumExceptionResolved` | scan |
| 32 | `ExceptionResolved` | L2-I15 | `ExamExceptionExceptionResolved` | scan |
| 33 | `IncidentReported` | L2-G02 | `LaboratoryIncidentReported` | review |
| 34 | `IncidentReported` | L2-G07 | `SecurityIncidentReported` | review |
| 35 | `IncidentResolved` | L2-G02 | `LaboratoryIncidentResolved` | review |
| 36 | `IncidentResolved` | L2-G07 | `SecurityIncidentResolved` | review |
| 37 | `InspectionCompleted` | L2-G01 | `FacilityInspectionCompleted` | review |
| 38 | `InspectionCompleted` | L2-G02 | `LaboratoryInspectionCompleted` | review |
| 39 | `InspectionCompleted` | L2-G10 | `InfrastructureInspectionCompleted` | review |
| 40 | `InspectionScheduled` | L2-G01 | `FacilityInspectionScheduled` | review |
| 41 | `InspectionScheduled` | L2-G02 | `LaboratoryInspectionScheduled` | review |
| 42 | `InspectionScheduled` | L2-G10 | `InfrastructureInspectionScheduled` | review |
| 43 | `InspectionStarted` | L2-G01 | `FacilityInspectionStarted` | review |
| 44 | `InspectionStarted` | L2-G02 | `LaboratoryInspectionStarted` | review |
| 45 | `InspectionStarted` | L2-G10 | `InfrastructureInspectionStarted` | review |
| 46 | `InvestigationStarted` | L2-G07 | `SecurityInvestigationStarted` | review |
| 47 | `InvestigationStarted` | L2-I14 | `ExamCommitteeInvestigationStarted` | review |
| 48 | `LeaveApproved` | L2-E05 | `EmployeeLeaveLeaveApproved` | review |
| 49 | `LeaveApproved` | L2-G03 | `HostelLeaveApproved` | review |
| 50 | `LeaveRequested` | L2-E05 | `EmployeeLeaveLeaveRequested` | review |
| 51 | `LeaveRequested` | L2-G03 | `HostelLeaveRequested` | review |
| 52 | `MigrationCompleted` | L2-A02 | `PersonMigrationCompleted` | scan |
| 53 | `MigrationCompleted` | L2-B04 | `CurriculumMigrationCompleted` | scan |
| 54 | `MigrationStarted` | L2-A02 | `PersonMigrationStarted` | scan |
| 55 | `MigrationStarted` | L2-B04 | `CurriculumMigrationStarted` | scan |
| 56 | `PersonUpdated` | L2-A02 | `PersonUpdated` | scan |
| 57 | `PersonUpdated` | L2-H06 | `AlumniPersonUpdated` | scan |
| 58 | `PolicyUpdated` | L2-A02 | `PersonPolicyUpdated` | scan |
| 59 | `PolicyUpdated` | L2-C08 | `AlumniTransitionPolicyUpdated` | scan |
| 60 | `RegistrationApproved` | L2-B04 | `CurriculumRegistrationApproved` | scan |
| 61 | `RegistrationApproved` | L2-C03 | `StudentRegistrationRegistrationApproved` | scan |
| 62 | `RegistrationCancelled` | L2-B04 | `CurriculumRegistrationCancelled` | scan |
| 63 | `RegistrationCancelled` | L2-C03 | `StudentRegistrationRegistrationCancelled` | scan |
| 64 | `ReservationCreated` | L2-G05 | `LibraryReservationCreated` | review |
| 65 | `ReservationCreated` | L2-G10 | `InfrastructureReservationCreated` | review |
| 66 | `ReservationExpired` | L2-G05 | `LibraryReservationExpired` | review |
| 67 | `ReservationExpired` | L2-G10 | `InfrastructureReservationExpired` | review |
| 68 | `RetryCompleted` | L2-A02 | `PersonRetryCompleted` | scan |
| 69 | `RetryCompleted` | L2-B04 | `CurriculumRetryCompleted` | scan |
| 70 | `RetryScheduled` | L2-A02 | `PersonRetryScheduled` | scan |
| 71 | `RetryScheduled` | L2-B04 | `CurriculumRetryScheduled` | scan |
| 72 | `SessionCreated` | L2-A02 | `PersonSessionCreated` | scan |
| 73 | `SessionCreated` | L2-A03 | `AuthSessionCreated` | scan |
| 74 | `SessionExpired` | L2-A02 | `PersonSessionExpired` | scan |
| 75 | `SessionExpired` | L2-A03 | `AuthSessionExpired` | scan |
| 76 | `StockIssued` | L2-J06 | `InventoryStockIssued` | scan |
| 77 | `StockIssued` | L2-J07 | `StockIssued` | scan |
| 78 | `StockReturned` | L2-J06 | `InventoryStockReturned` | scan |
| 79 | `StockReturned` | L2-J07 | `StockReturned` | scan |
| 80 | `StudentCreated` | L2-C01 | `AdmissionStudentCreated` | scan |
| 81 | `StudentCreated` | L2-C02 | `StudentCreated` | scan |
| 82 | `StudentStatusChanged` | L2-C02 | `StudentStatusChanged` | scan |
| 83 | `StudentStatusChanged` | L2-C04 | `ProgressionStudentStatusChanged` | scan |
| 84 | `StudentStatusChanged` | L2-D06 | `ElectiveStudentStatusChanged` | scan |
| 85 | `SynchronizationCompleted` | L2-A02 | `PersonSynchronizationCompleted` | scan |
| 86 | `SynchronizationCompleted` | L2-H01 | `ApaarSynchronizationCompleted` | scan |
| 87 | `SynchronizationCompleted` | L2-H10 | `ExternalProviderSynchronizationCompleted` | scan |
| 88 | `SynchronizationStarted` | L2-A02 | `PersonSynchronizationStarted` | scan |
| 89 | `SynchronizationStarted` | L2-H10 | `ExternalProviderSynchronizationStarted` | scan |
| 90 | `VerificationExpired` | L2-B04 | `CurriculumVerificationExpired` | scan |
| 91 | `VerificationExpired` | L2-H10 | `ExternalProviderVerificationExpired` | scan |
| 92 | `VerificationFailed` | L2-B04 | `CurriculumVerificationFailed` | scan |
| 93 | `VerificationFailed` | L2-H10 | `ExternalProviderVerificationFailed` | scan |
| 94 | `VerificationRequested` | L2-B04 | `CurriculumVerificationRequested` | scan |
| 95 | `VerificationRequested` | L2-H10 | `ExternalProviderVerificationRequested` | scan |
| 96 | `VerificationStarted` | L2-B04 | `CurriculumVerificationStarted` | scan |
| 97 | `VerificationStarted` | L2-H10 | `ExternalProviderVerificationStarted` | scan |
| 98 | `VisitorApproved` | L2-G03 | `HostelVisitorApproved` | review |
| 99 | `VisitorApproved` | L2-G07 | `SecurityVisitorApproved` | review |
| 100 | `VisitorCheckedIn` | L2-G03 | `HostelVisitorCheckedIn` | review |
| 101 | `VisitorCheckedIn` | L2-G07 | `SecurityVisitorCheckedIn` | review |
| 102 | `VisitorCheckedOut` | L2-G03 | `HostelVisitorCheckedOut` | review |
| 103 | `VisitorCheckedOut` | L2-G07 | `SecurityVisitorCheckedOut` | review |
| 104 | `VisitorRegistered` | L2-G03 | `HostelVisitorRegistered` | review |
| 105 | `VisitorRegistered` | L2-G07 | `SecurityVisitorRegistered` | review |
| 106 | `WorkflowCancelled` | L2-A05 | `WorkflowCancelled` | scan |
| 107 | `WorkflowCancelled` | L2-B04 | `CurriculumWorkflowCancelled` | scan |
| 108 | `WorkflowCompleted` | L2-A05 | `WorkflowCompleted` | scan |
| 109 | `WorkflowCompleted` | L2-B04 | `CurriculumWorkflowCompleted` | scan |
| 110 | `WorkflowSuspended` | L2-A05 | `WorkflowSuspended` | scan |
| 111 | `WorkflowSuspended` | L2-B04 | `CurriculumWorkflowSuspended` | scan |

## 6. Names needing your judgement before any rename

These have one declared publisher plus at least one other document mentioning the
name outside a recognised publisher or consumer heading. Each is either a genuine
second publisher or a consumer reference that is simply not filed under a
`Consumed Events` heading. **Machine classification cannot tell these apart.**

| Event name | Declared publisher | Also appears in |
|---|---|---|
| `AcademicStandingUpdated` | L2-C04 | L2-F11 |
| `AdmissionCancelled` | L2-C01 | L2-F11 |
| `AlumniMembershipActivated` | L2-H06 | L2-C08 |
| `ApprovalRejected` | L2-D02 | L2-B04,L2-F11 |
| `ApprovalReturned` | L2-E02 | L2-B04 |
| `AssetAssigned` | L2-J08 | L2-F12 |
| `AssetRegistered` | L2-J08 | L2-F12 |
| `AssignmentCompleted` | L2-B04 | L2-C02 |
| `AssignmentCreated` | L2-B04 | L2-B05,L2-E02 |
| `AssignmentRejected` | L2-B04 | L2-E02 |
| `AttendanceRecorded` | L2-H04 | L2-D04 |
| `AuthenticationStarted` | L2-A02 | L2-A03 |
| `BudgetValidationCompleted` | L2-F14 | L2-J01 |
| `CampusActivated` | L2-B03 | L2-A10 |
| `CampusCreated` | L2-B03 | L2-A10 |
| `ClearanceCompleted` | L2-C08 | L2-G05 |
| `CommitteeRecommendationRecorded` | L2-I14 | L2-J01 |
| `ConflictResolved` | L2-D03 | L2-D05 |
| `ConnectorActivated` | L2-H01 | L2-H10 |
| `ConnectorRegistered` | L2-H10 | L2-H01 |
| `CreditTransferApproved` | L2-H10 | L2-C04 |
| `CurriculumUpdated` | L2-D06 | L2-C04 |
| `DisposalApproved` | L2-B04 | L2-F12,L2-J10 |
| `EligibilityRejected` | L2-C07 | L2-D06 |
| `EnrollmentCancelled` | L2-H10 | L2-D06 |
| `EnrollmentRequested` | L2-H10 | L2-D06 |
| `EventCreated` | L2-H06 | L2-E02 |
| `FinancialPeriodClosed` | L2-F09 | L2-F10 |
| `FinancialPeriodOpened` | L2-F09 | L2-F10 |
| `InfrastructureArchived` | L2-B03 | L2-G10 |
| `InvestigationStarted` | L2-I14 | L2-G07 |
| `ItemCreated` | L2-J06 | L2-J01 |
| `LearningProgressUpdated` | L2-D06 | L2-H10 |
| `LeaveApproved` | L2-E05 | L2-E04,L2-G03 |
| `LeaveCancelled` | L2-E05 | L2-E04 |
| `LeaveRejected` | L2-E05 | L2-E04 |
| `LeaveRequested` | L2-E05 | L2-E04,L2-G03 |
| `PositionAssignmentClosed` | L2-E02 | L2-E01 |
| `PositionAssignmentCreated` | L2-E02 | L2-E01 |
| `ProviderVerified` | L2-D06 | L2-H10 |
| `PublicationArchived` | L2-H05 | L2-B04 |
| `PurchaseRequisitionApproved` | L2-J03 | L2-J01 |
| `RecoveryCompleted` | L2-A02 | L2-H10 |
| `RecoveryStarted` | L2-A02 | L2-H10 |
| `RenewalRejected` | L2-C07 | L2-F11 |
| `ResponsibilityAssigned` | L2-E01 | L2-E02 |
| `RetryStarted` | L2-B04 | L2-H02 |
| `StudentActivated` | L2-C02 | L2-C01,L2-C04,L2-D06,L2-F11 |
| `StudentRegistered` | L2-D06 | L2-A06,L2-F11 |
| `StudentSuspended` | L2-C02 | L2-D06 |
| `SynchronizationFailed` | L2-H10 | L2-H01 |
| `VendorInvoiceApproved` | L2-F14 | L2-F08 |
| `VerificationCompleted` | L2-B04 | L2-G05 |
| `WorkflowArchived` | L2-B04 | L2-A05 |

## 7. Execution plan once approved

1. You approve, amend, or reject each row in section 5, and confirm the convention
   in section 4.
2. Renames are applied **per name, publisher-side and consumer-side together**. A
   consumer reference must move with its publisher or the contract breaks.
3. After each name: assert zero occurrences of the old name remain outside this
   registry.
4. A canonical event registry keyed by name is added under Section A, per the
   INT-016 resolution, so future additions are checked against it.
5. Section 6 items are resolved individually; none are renamed on assumption.

## 8. Scope note

This registry covers canonical event *names*. It does not resolve the underlying
**ownership** conflicts that produced some of them — for example `StockIssued`
published by both J06 and J07 is a symptom of INT-014, and `AlumniTransitionCompleted`
published by both C08 and H06 is a symptom of INT-010. Those findings remain open and
should be settled first, because settling them may remove the collision rather than
rename it.

---

## 9. What was applied, and what was held

Approved by the repository owner and applied on 2026-08-04.

### Applied

| Scope | Count |
|---|---:|
| Colliding names resolved | 53 |
| Publisher-side occurrences renamed | 248 across 29 documents |
| Consumer-side references rewritten | 22 |
| Consumer references needing no change (bare name retained by one publisher) | 23 |

Diff was +265 / -265 across 33 files: every rename was in place, so no line was
added or removed from the corpus.

Two naming defects were caught during application and corrected before writing:
qualifier and name overlapped, which would have produced
`EmployeeLeaveLeaveApproved` and `ExamExceptionExceptionCaseClosed`. The
qualifier merge now collapses the overlap, giving `EmployeeLeaveApproved` and
`ExamExceptionCaseClosed`.

### Held — 10 references still carry an ambiguous name

These are third-party documents citing a name published by two or more sources,
where nothing in the text says which one is meant. **Renaming them on a guess
would silently rewire an event contract**, so they were left alone.

| Event name | Referencing document | Occurrences | Why held |
|---|---|---:|---|
| `AuthenticationFailed` | L2-A03 | 6 | publisher attribution disputed: scan says A02, review says A03 |
| `AuthenticationFailed` | L2-D04 | 1 | publishers L2-A02,L2-H10 |
| `ConflictDetected` | L2-E02 | 2 | publishers L2-B04,L2-D03 |
| `LeaveApproved` | L2-H07 | 1 | publishers L2-E05,L2-G03 |
| `PolicyUpdated` | L2-F11 | 1 | publishers L2-A02,L2-C08 |
| `PolicyUpdated` | L2-I13 | 2 | publishers L2-A02,L2-C08 |
| `RetryCompleted` | L2-H02 | 1 | publishers L2-A02,L2-B04 |
| `VerificationFailed` | L2-E03 | 13 | publishers L2-B04,L2-H10 |
| `VerificationStarted` | L2-G05 | 1 | publishers L2-B04,L2-H10 |
| `VerificationStarted` | L2-J00 | 1 | publishers L2-B04,L2-H10 |

The two authentication rows need a prior decision: this scan attributes platform
`AuthenticationFailed` to A02 (Person and Identity), while the 2026-08-03 review
frames the collision as platform authentication (A03) versus external-provider
authentication (H10). Settle which document owns platform authentication events
before renaming either.

### Still recommended

`InventoryStockIssued` / `StockIssued` and `AlumniTransitionCompleted` were
renamed, but the underlying ownership conflicts (INT-014 for J06/J07 stock,
INT-010 for C08/H06 alumni) remain open. Resolving those may remove one side of
the pair entirely, at which point the qualified name can be simplified.
---

## 10. Addendum — Result Publication Event (2026-08-05)

**Finding:** INT-008 of `review-repository-integrity-2026-08-05.md` (Type 1/7/8, High)
**Status:** APPROVED AND APPLIED 2026-08-05

The official result-publication fact was carried under **five** distinct spellings.
The 2026-08-05 review identified four; a fifth (`AcademicResultPublished`) was found
during remediation.

| Former spelling | Where it appeared |
|---|---|
| `ResultPublished` | L1 locked architecture, Volume 2, Volume 3, L2-I10, REPO_UNDERSTANDING |
| `ResultsPublished` | L1 locked architecture, L2-I16 |
| `ExaminationResultsPublished` | L2-C04 |
| `Results Published` / `Official Examination Results Published` | L2-C04 prose and diagram labels |
| `AcademicResultPublished` | L2-F11, L2-H04 |

### Canonical name

**`ExaminationResultPublished`** — singular `Result`, full word `Examination`.

Rationale: matches the unabbreviated, singular-subject convention already set by
`StudentQualificationCompleted`, and matches the owning document's own title
(`L2-I10 — Result Publication Workflows`). Publisher is **L2-I10**; all other
documents are consumers.

### Sibling events normalized for convention consistency

`RevaluationResultsPublished` -> `RevaluationResultPublished`
`SupplementaryResultsPublished` -> `SupplementaryResultPublished`

These are genuinely distinct events, not collisions. Only their plural form changed.

### Correction — `FinalResultPublished` was a sixth variant (2026-08-05, same day)

This section originally recorded `FinalResultPublished` as "deliberately not renamed"
on the grounds that it was a distinct fact — the consolidated final result. **That was
wrong**, and it was wrong because it accepted `L2-I13`'s own "Published By" claim
without checking the named publisher.

Verified evidence:

- `FinalResultPublished` occurs in exactly one document, `L2-I13` (`:1365`, `:2283`).
- `L2-I13` declares it "Published By: L2-I10 — Result Publication".
- `L2-I10` contains **zero** occurrences of it, and publishes exactly one publication
  event, `ExaminationResultPublished` (`:239`, `:1062`).
- `L2-I10:64` states Result Publication *consumes* finalized academic outcomes and does
  not modify computation results — it has no separate "final" fact to publish.

`FinalResultPublished` was therefore a **sixth spelling** of the same fact, and a
dangling consumer subscription: I13 waited on an event no publisher emitted, so
Academic Completion processing would never have triggered.

Both occurrences in `L2-I13` are now `ExaminationResultPublished`. No document
references `FinalResultPublished` any longer except the explanatory note in I13.

**Total variants collapsed by INT-008: six.** The 2026-08-05 review identified four;
`AcademicResultPublished` and `FinalResultPublished` were found by grepping the corpus
rather than by reading the review.
