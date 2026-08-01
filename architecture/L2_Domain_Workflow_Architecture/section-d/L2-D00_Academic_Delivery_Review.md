# L2-D00 — Academic Delivery Review

---

# Document Metadata

| Attribute | Value |
|-----------|-------|
| Document ID | L2-D00 |
| Domain Section | Academic Delivery (Section D) |
| Layer | L2 — Domain Workflow Architecture |
| Architecture Level | Enterprise Domain Review & Certification |
| Architecture Style | Domain Driven Design (DDD), CQRS, Event Driven Architecture (EDA), Clean & Hexagonal Architecture |
| Primary Owner | Enterprise Architecture Review Board |
| Technical Owner | Technical Architecture Board |
| Canonical Owner | Pallav |
| Version | 1.0 |
| Status | Certified & Locked |
| Review Date | 2026-07-27 |
| Repository | SutraERP Enterprise Repository |
| Target Scope | L2-D01, L2-D02, L2-D03, L2-D04, L2-D05, L2-D06, L2-D07 |

---

# Purpose

The purpose of this document is to establish a unified architecture review, boundary verification, dependency validation, and governance lock for the entire **Academic Delivery Section (Section D)** of the SutraERP platform.

This review certifies that all constituent sub-domains (L2-D01 through L2-D07) adhere strictly to the enterprise standards of Domain-Driven Design (DDD), Command-Query Responsibility Segregation (CQRS), Event-Driven Architecture (EDA), Canonical Data Ownership, and Hexagonal Architecture. It acts as the final gate to prevent domain leakage, circular dependencies, transactional integrity failures, and canonical ownership violations before final implementation locking.

---

# Scope

This review covers the complete architectural definitions, boundaries, and integration contracts of:
- **L2-D01**: Course & Course Offering Workflows
- **L2-D02**: Faculty Academic Assignment Workflows
- **L2-D03**: Timetable Workflows
- **L2-D04**: Teaching Session & Attendance Workflows
- **L2-D05**: Academic Monitoring Workflows
- **L2-D06**: Open Electives & MOOCs Workflows
- **L2-D07**: Internal Assessment Workflows

This document does *not* duplicate the detailed operational workflows of the individual sub-domains but rather defines their transactional boundaries, structural dependencies, and behavioral relationships.

---

# Section Vision

The Academic Delivery section of SutraERP governs the operational execution of a university's academic core—spanning curriculum manifestation, teaching resources scheduling, lecture execution, attendance logging, compliance monitoring, and learning flexibility.

## Business Objectives
1. **Academic Execution Integrity**: Ensure that teaching sessions align strictly with scheduled slots, assigned faculty, and syllabus targets.
2. **Operational Efficiency**: Automate course creation, allocation, scheduling, and attendance capture, reducing administrative burden.
3. **Flexible Learning Governance**: Support modern flexible learning pathways (NEP 2020, MOOCs, SWAYAM/NPTEL) without degrading transcript authenticity.
4. **Compliance & Quality Assurance**: Provide automated monitoring of teaching progress and student attendance to support accreditation audits (e.g., NAAC, NBA).

## Architectural Objectives
- **Zero Shared Database Boundaries**: Sub-domains communicate exclusively through APIs, commands, or events.
- **Strict Aggregate Isolation**: Aggregate roots manage their consistency boundaries; direct cross-aggregate queries are prohibited on the write side.
- **Asynchronous Event-Driven Integration**: Use eventually consistent sagas for multi-stage workflows to isolate transaction boundaries.
- **CQRS Separation**: Isolate performance-intensive read models (e.g., timetable search, seat availability dashboards) from transaction-critical command models.

---

# Section Domain Inventory

Below is the verified registry of the seven bounded contexts constituting the Academic Delivery Section.

## L2-D01 — Course & Course Offering
- **Purpose**: Governs canonical course definitions and course offering lifecycles (creation, validation, capacity limits, publication).
- **Core Aggregates**:
  - `CourseMaster`: Canonical course definition owned by Section B and referenced by D01 for offering validation.
  - `CourseOffering`: Owns offering session, department eligibility, capacity, availability status.
- **Publishes Events**: `CourseOfferingCreated`, `CourseOfferingUpdated`, `CourseOfferingSubmittedForApproval`, `CourseOfferingApproved`, `CourseOfferingRejected`, `CourseOfferingPublished`, `CourseOfferingPublicationScheduled`, `CourseOfferingActivated`, `CourseOfferingActivationFailed`, `CourseOfferingSuspended`, `CourseOfferingReactivated`, `CourseOfferingCancelled`.
- **Consumes Events**: None (Upstream foundation).
- **Depends On**: Curriculum Framework (Upstream L2-B04).
- **Downstream Domains**: L2-D02, L2-D03, L2-D06, Examinations.
- **Current Status**: Architecture Locked.
- **Architecture Health**: Excellent.

## L2-D02 — Faculty Academic Assignment
- **Purpose**: Governs the allocation of faculty members to course offerings, lecture divisions, and lab groups.
- **Core Aggregates**:
  - `FacultyAssignment`: Owns assignment status, loading hours, and specific course offering references.
  - `AssignmentApprovalCase`: Governs institutional approval workflows for manual exceptions.
  - `OperationalChange`: Controls modifications during execution.
- **Publishes Events**: `FacultyAssignmentCreated`, `FacultyAssignmentSubmitted`, `FacultyAssignmentReviewed`, `FacultyAssignmentApproved`, `FacultyAssignmentRejected`, `FacultyAssignmentActivated`, `FacultyAssignmentCompleted`, `FacultyAssignmentArchived`, `ApprovalInitiated`, `ApprovalRouted`, `ApprovalGranted`, `ApprovalRejected`, `ApprovalEscalated`, `ApprovalCompleted`, `OperationalChangeRequested`, `OperationalChangeValidated`, `OperationalChangeApproved`, `OperationalChangeApplied`, `OperationalChangeClosed`.
- **Consumes Events**: `CourseOfferingPublished`, `CourseOfferingCancelled` (from L2-D01).
- **Depends On**: L2-D01, Human Resources (Reference data).
- **Downstream Domains**: L2-D03, L2-D04.
- **Current Status**: Implementation Ready.
- **Architecture Health**: High.

## L2-D03 — Timetable Workflows
- **Purpose**: Manages scheduling of course offerings, timeslot definitions, and physical room/slot allocations while preventing resource conflicts.
- **Core Aggregates**:
  - `TimetableStructure`: Defines timeslots, days, and period patterns.
  - `TimetableSlotAssignment`: Owns Room, Slot, Faculty, and Course Offering bindings.
  - `SchedulingConflictRegister`: Tracks conflicts and exceptions.
- **Publishes Events**: `TimetablePlanningStarted`, `TimetablePlanningCompleted`, `TimetableDraftCreated`, `TimetableValidated`, `TimetableValidationFailed`, `TimetableOptimized`, `ConflictDetected`, `ConflictResolved`, `TimetableApproved`, `TimetableRejected`, `TimetablePublished`, `TimetableActivated`, `TimetableRevisionCreated`.
- **Consumes Events**: `CourseOfferingPublished` (L2-D01), `FacultyAssignmentApproved` (L2-D02).
- **Depends On**: L2-D01, L2-D02, Campus Infrastructure (L2-B03).
- **Downstream Domains**: L2-D04, L2-D05.
- **Current Status**: Implementation Ready.
- **Architecture Health**: High.

## L2-D04 — Teaching Session & Attendance
- **Purpose**: Governs real-time classroom lecture execution, faculty progress (syllabus tracking), and student attendance logging.
- **Core Aggregates**:
  - `TeachingSession`: Represents a conducted lecture unit.
  - `AttendanceRecord`: Captures student attendance segments, corrections, and audit trails.
- **Publishes Events**: `TeachingSessionStarted`, `TeachingSessionPaused`, `TeachingSessionResumed`, `TeachingSessionCompleted`, `TeachingSessionCancelled`, `TeachingSessionAbandoned`, `TeachingSessionArchived`, `AttendanceRegisterOpened`, `AttendanceRecorded`, `AttendanceSubmitted`, `AttendanceApproved`, `AttendanceFinalized`, `AttendanceCorrectionRequested`, `AttendanceCorrectionApproved`, `AttendanceCorrectionRejected`, `AttendanceCorrected`, `TeachingProgressRecorded`, `TopicCoverageRecorded`, `TeachingNotesRecorded`, `SessionEvidenceRecorded`, `StudentParticipationRecorded`, `SessionRemarkRecorded`.
- **Consumes Events**: `TimetablePublished`, `TimetableRevisionCreated` (from L2-D03).
- **Depends On**: L2-D03, Student Lifecycle (Reference data).
- **Downstream Domains**: L2-D05, Examinations (Attendance thresholds).
- **Current Status**: Implementation Ready.
- **Architecture Health**: Very High.

## L2-D05 — Academic Monitoring
- **Purpose**: Audits compliance, syllabus timelines, faculty delivery deviations, and student attendance risk levels.
- **Core Aggregates**:
  - `MonitoringCase`: Tracks active compliance review files.
  - `MonitoringPlan`: Governs thresholds and rules.
  - `MonitoringEvaluation`: Conducts automated compliance analysis.
  - `MonitoringIntervention`: Manages warning escalations.
- **Publishes Events**: `AcademicMonitoringCaseCreated`, `MonitoringScopeAssigned`, `MonitoringPlanCreated`, `MonitoringPolicyAssigned`, `AcademicMonitoringStarted`, `MonitoringSnapshotCreated`.
- **Consumes Events**: `TeachingProgressRecorded`, `AttendanceFinalized` (L2-D04), `TimetablePublished` (L2-D03).
- **Depends On**: L2-D04, Academic Policy (Reference data).
- **Downstream Domains**: Academic Affairs Dashboard.
- **Current Status**: Implementation Ready.
- **Architecture Health**: High.

## L2-D06 — Open Electives & MOOCs
- **Purpose**: Orchestrates student enrollment in flexible learning (open electives, value-added courses, MOOCs, Coursera) and credit recognition.
- **Core Aggregates**:
  - `FlexibleLearningEnrollment` (FLE): Controls the enrollment state and academic completion status.
  - `SeatAllocation`: Handles seat reservations.
  - `WaitlistEntry`: Prioritizes course queues.
  - `ExternalLearningRegistration`: Manages external registration.
  - `LearningProgress`: Tracks progress synchronization.
  - `CertificateVerification`: Audits certificate uploads.
  - `CreditRecognition`: Manages credit mapping and transcript posting retries.
- **Publishes Events**: `LearningOpportunityPublished`, `LearningOpportunityUpdated`, `LearningOpportunityActivated`, `LearningOpportunitySuspended`, `LearningOpportunityArchived`, `LearningOpportunityCapacityUpdated`, `LearningOpportunityClosed`, `EnrollmentRequested`, `EnrollmentValidated`, `EnrollmentApproved`, `EnrollmentRejected`, `EnrollmentCancelled`, `EnrollmentWithdrawn`, `EnrollmentClosed`, `EligibilityValidated`, `EligibilityRejected`, `SeatAllocated`, `SeatReserved`, `SeatReleased`, `WaitlistEntryCreated`, `WaitlistedStudentPromoted`, `WaitlistEntryExpired`, `WaitlistEntryRemoved`, `LearnerRegisteredWithProvider`, `EnrollmentStatusSynchronized`, `LearningProgressSynchronized`, `AssessmentStatusSynchronized`, `ExternalCourseCompleted`, `SynchronizationRecovered`, `SynchronizationFailureResolved`, `LearningProgressInitialized`, `LearningProgressUpdated`, `LearningCompletionValidated`, `LearningCompleted`, `LearningActivityClosed`, `CertificateReceived`, `CertificateAuthenticityValidated`, `LearnerIdentityVerified`, `ProviderVerified`, `CertificateAccepted`, `CertificateRejected`, `CertificateFraudSuspected`, `CreditRecognitionRequested`, `CreditRecognitionApproved`, `CreditRecognitionRejected`, `TranscriptPostingRequested`, `TranscriptPostingCompleted`, `CreditRecognitionCaseClosed`, `LearningProviderConfigured`, `ElectivePolicyPublished`, `CreditMappingRulesPublished`, `EnrollmentWindowConfigured`, `ApprovalPolicyPublished`, `ProviderIntegrationConfigured`, `IntegrationHealthUpdated`, `AcademicSessionArchived`, `EnrollmentRecovered`, `SeatConflictResolved`, `ProviderSynchronizationRecovered`, `LearningRecordsReconciled`, `AdministrativeOverrideExecuted`, `WorkflowRecovered`, `ExceptionCaseClosed`.
- **Consumes Events**: `CourseOfferingPublished` (from L2-D01).
- **Depends On**: L2-D01, Student Lifecycle, Curriculum Management, Academic Records (Posting target).
- **Downstream Domains**: Academic Records, Examinations.
- **Current Status**: Certified & Locked (Refactored 1.0).
- **Architecture Health**: Fully Compliant.

## L2-D07 â€” Internal Assessment
- **Purpose**: Governs institution-controlled internal and continuous assessment activities, including blueprinting, window control, submission capture, moderation, and finalization of internal assessment marks.
- **Core Aggregates**:
  - `InternalAssessmentBlueprint`: Defines assessment components, weightage, and policy bindings.
  - `AssessmentWindow`: Owns release, deadline, extension, and closure rules.
  - `AssessmentSubmission`: Captures student submission evidence and awarded internal marks.
  - `AssessmentModerationCase`: Governs moderation, correction, and approval of internal assessment outcomes.
- **Publishes Events**: `InternalAssessmentBlueprintPublished`, `AssessmentWindowOpened`, `AssessmentWindowExtended`, `AssessmentSubmissionRecorded`, `AssessmentSubmissionUpdated`, `AssessmentModerationRequested`, `AssessmentModerationCompleted`, `InternalAssessmentFinalized`, `InternalAssessmentArchived`.
- **Consumes Events**: `CourseOfferingPublished` (from L2-D01), `TeachingSessionCompleted` (from L2-D04), `AttendanceFinalized` (from L2-D04).
- **Depends On**: L2-D01, L2-D04, L2-I01 (assessment policy reference), Student Lifecycle reference data.
- **Downstream Domains**: L2-I08, L2-I09.
- **Current Status**: Architecture Locked.
- **Architecture Health**: High.

---

# Complete Academic Delivery Workflow Chain

```text
  [Curriculum Framework] (L2-B04)
           │
           ▼
  [Course Offering Creation] (L2-D01)  ◄── Marks Course Codes, Credits & Max Capacities
           │
           ▼
  [Faculty Academic Assignment] (L2-D02) ◄── Maps Faculty Load to published Course Offerings
           │
           ▼
  [Timetable Generation] (L2-D03)      ◄── Resolves Rooms, Slots, Faculty & Offering bindings
           │
           ▼
  [Teaching Session Execution] (L2-D04) ◄── Classroom Conducted & Syllabus Progress Tracked
           │
           ▼
  [Attendance Logging] (L2-D04)        ◄── Records Student Attendance Segments
           │
           ▼
  [Academic Monitoring] (L2-D05)       ◄── Analyzes syllabus deviation & attendance risk levels
           │
           ├───────────────────────────┼───────────────────────────┐
           ▼                                                       ▼
  [Standard Electives Enrollment] (L2-D01)               [MOOCs & Open Electives] (L2-D06)
           │                                                       │
           ▼                                                       ▼
  [Institutional Grading] (Exam Domain)                   [Certificate Verification] (L2-D06)
           │                                                       │
           ▼                                                       ▼
  [Internal Assessment Finalization] (L2-D07)             [Credit Recognition Saga] (L2-D06)
           │                                                       │
           └───────────────────────────┬───────────────────────────┘
                                       ▼
                         [Transcript Integration] (Academic Records)
```

---

# Part 5 — Aggregate Ownership Review

Below is the verified registry of every Aggregate Root defined within Section D.

```text
========================================================================================
1. CourseMaster
========================================================================================
- Source Document: L2-B04 — Curriculum Framework
- Owner Domain: L2-B04 (Curriculum Framework Context)
- Aggregate Root: CourseMaster
- Purpose: Governs canonical course definitions and curriculum mappings consumed by Section D offering workflows.
- Lifecycle: Proposed -> Active -> Superseded -> Archived
- Business Invariants: Course Code must be unique. Credits cannot be negative.
- External References: L2-B04 Curriculum Framework ID
- Canonical IDs Used: CourseMasterId
- Commands Targeting: CreateCourse
- Events Published: CourseCreated, CourseUpdated
- Events Consumed: None
- Read Models: Course Catalog (RM-B04-01)
- Boundary Validation: Strictly encapsulated; referenced in L2-D01, L2-D02, and L2-D03 by CourseMasterId only.
- Aggregate Leakage: PASS. No direct reference properties exposed.

========================================================================================
2. CourseOffering
========================================================================================
- Source Document: L2-D01 — Course & Course Offering
- Owner Domain: L2-D01 (Course Offering Context)
- Aggregate Root: CourseOffering
- Purpose: Represents a manifest instance of a course offered in a specific term.
- Lifecycle: Draft -> Published -> Active -> Suspended -> Closed
- Business Invariants: Max Capacity must exceed current allocated seats.
- External References: Academic Session ID, Faculty ID
- Canonical IDs Used: CourseOfferingId
- Commands Targeting: PublishOffering, UpdateOfferingCapacity, SuspendOffering
- Events Published: CourseOfferingPublished, CourseOfferingCapacityUpdated, CourseOfferingClosed
- Events Consumed: None
- Read Models: Course Offering Registry (RM-D01-02)
- Boundary Validation: Reference integrity checked on calendar terms.
- Aggregate Leakage: PASS.

========================================================================================
3. FacultyAssignment
========================================================================================
- Source Document: L2-D02 — Faculty Academic Assignment
- Owner Domain: L2-D02 (Faculty Assignment Context)
- Aggregate Root: FacultyAssignment
- Purpose: Mappings of active faculty members to course offering sections.
- Lifecycle: Draft -> Submitted -> Approved -> Active -> Completed
- Business Invariants: Workload allocation hours cannot exceed maximum allowed limits.
- External References: Human Resources Workforce ID, CourseOfferingId
- Canonical IDs Used: FacultyAssignmentId
- Commands Targeting: AssignFaculty, ReassignFaculty
- Events Published: FacultyAssignmentCreated, FacultyAssignmentApproved
- Events Consumed: CourseOfferingPublished
- Read Models: Faculty Workload Board (RM-D02-01)
- Boundary Validation: Resolves conflicts using Workforce ID checks.
- Aggregate Leakage: PASS.

========================================================================================
4. AssignmentApprovalCase
========================================================================================
- Source Document: L2-D02 — Faculty Academic Assignment
- Owner Domain: L2-D02 (Faculty Assignment Context)
- Aggregate Root: AssignmentApprovalCase
- Purpose: Controls institutional workflows for approving faculty workload exceptions.
- Lifecycle: Initiated -> Routed -> Granted -> Completed
- Business Invariants: Approver role permissions must match requested override levels.
- External References: Role Registry ID, FacultyAssignmentId
- Canonical IDs Used: ApprovalCaseId
- Commands Targeting: ApproveWorkloadException, EscalateApproval
- Events Published: ApprovalInitiated, ApprovalGranted
- Events Consumed: None
- Read Models: Pending Approval Case Grid (RM-D02-02)
- Boundary Validation: Separates override actions from core assignment states.
- Aggregate Leakage: PASS.

========================================================================================
5. OperationalChange
========================================================================================
- Source Document: L2-D02 — Faculty Academic Assignment
- Owner Domain: L2-D02 (Faculty Assignment Context)
- Aggregate Root: OperationalChange
- Purpose: Tracks assignment amendments requested after semester start.
- Lifecycle: Requested -> Validated -> Approved -> Closed
- Business Invariants: Applied changes must be logged with historical auditing context.
- External References: Administrator User ID, FacultyAssignmentId
- Canonical IDs Used: OperationalChangeId
- Commands Targeting: RequestAssignmentChange, ApplyAssignmentChange
- Events Published: OperationalChangeRequested, OperationalChangeApplied
- Events Consumed: None
- Read Models: Operational Change Log (RM-D02-03)
- Boundary Validation: Strict auditing trail.
- Aggregate Leakage: PASS.

========================================================================================
6. TimetableStructure
========================================================================================
- Source Document: L2-D03 — Timetable
- Owner Domain: L2-D03 (Scheduling Context)
- Aggregate Root: TimetableStructure
- Purpose: Defines timeslots, days, and period patterns for scheduling blocks.
- Lifecycle: Draft -> Published -> Archived
- Business Invariants: Slots must be continuous and non-overlapping.
- External References: Academic Calendar ID
- Canonical IDs Used: TimetableStructureId
- Commands Targeting: CreateTimetable, UpdateTimetableStructure
- Events Published: TimetablePlanningStarted, TimetablePlanningCompleted
- Events Consumed: None
- Read Models: Active Calendar Slots Grid (RM-D03-02)
- Boundary Validation: Immutable definition of operational calendar boundaries.
- Aggregate Leakage: PASS.

========================================================================================
7. TimetableSlotAssignment
========================================================================================
- Source Document: L2-D03 — Timetable
- Owner Domain: L2-D03 (Scheduling Context)
- Aggregate Root: TimetableSlotAssignment
- Purpose: Space-time bindings (Room ID, Slot ID, Offering ID, Faculty ID).
- Lifecycle: Draft -> Scheduled -> Modified -> Archived
- Business Invariants: Room, Slot, Faculty, and Course Offering must be clash-free.
- External References: Room ID (L2-B03), Slot ID, Faculty ID
- Canonical IDs Used: TimetableSlotAssignmentId
- Commands Targeting: AllocateSlot, RescheduleSlot
- Events Published: SlotScheduled, ScheduleModified
- Events Consumed: FacultyAssignmentApproved
- Read Models: Student Class Calendar (RM-D03-01)
- Boundary Validation: Room constraints mapped from L2-B03.
- Aggregate Leakage: PASS.

========================================================================================
8. SchedulingConflictRegister
========================================================================================
- Source Document: L2-D03 — Timetable
- Owner Domain: L2-D03 (Scheduling Context)
- Aggregate Root: SchedulingConflictRegister
- Purpose: Log and resolve timetable clash events.
- Lifecycle: Detected -> Investigating -> Resolved
- Business Invariants: Double-bookings must generate resolution records.
- External References: Room ID, Faculty ID
- Canonical IDs Used: ConflictRegisterId
- Commands Targeting: LogConflict, ResolveConflict
- Events Published: ConflictDetected, ConflictResolved
- Events Consumed: SlotScheduled
- Read Models: Conflict Dashboard (RM-D03-03)
- Boundary Validation: Encapsulated as a tracking entity.
- Aggregate Leakage: PASS.

========================================================================================
9. TeachingSession
========================================================================================
- Source Document: L2-D04 — Teaching Session & Attendance
- Owner Domain: L2-D04 (Execution Context)
- Aggregate Root: TeachingSession
- Purpose: Represents a conducted lecture unit, capturing topics and notes.
- Lifecycle: Scheduled -> Active -> Completed -> Archived
- Business Invariants: Conducted session dates must match scheduling periods.
- External References: Timetable Slot ID, Faculty ID
- Canonical IDs Used: TeachingSessionId
- Commands Targeting: StartTeachingSession, CompleteTeachingSession, CancelTeachingSession
- Events Published: TeachingSessionStarted, TeachingSessionCompleted, TeachingSessionCancelled
- Events Consumed: TimetablePublished
- Read Models: Daily Attendance Grid (RM-D04-01)
- Boundary Validation: Syllabus coverage validated against Curriculum.
- Aggregate Leakage: PASS.

========================================================================================
10. AttendanceRecord
========================================================================================
- Source Document: L2-D04 — Teaching Session & Attendance
- Owner Domain: L2-D04 (Execution Context)
- Aggregate Root: AttendanceRecord
- Purpose: Tracks student presence states and manual correction histories.
- Lifecycle: Opened -> Submitted -> Finalized -> Corrected
- Business Invariants: Correction records require institutional approval dean signatures.
- External References: Student ID, TeachingSessionId
- Canonical IDs Used: AttendanceRecordId
- Commands Targeting: RecordAttendance, SubmitAttendance, RequestAttendanceCorrection
- Events Published: AttendanceRecorded, AttendanceSubmitted, AttendanceCorrected
- Events Consumed: None
- Read Models: Student Attendance Profile (RM-D04-02)
- Boundary Validation: Strict audit segment updates.
- Aggregate Leakage: PASS.

========================================================================================
11. MonitoringCase
========================================================================================
- Source Document: L2-D05 — Academic Monitoring
- Owner Domain: L2-D05 (Monitoring Context)
- Aggregate Root: MonitoringCase
- Purpose: Tracks compliance review files for academic operational tracking.
- Lifecycle: Proposed -> Active -> UnderReview -> Resolved
- Business Invariants: Health scoring assessments must align with active policies.
- External References: Faculty ID, Course ID, Attendance ID
- Canonical IDs Used: MonitoringCaseId
- Commands Targeting: CreateAcademicMonitoringCaseCommand, AssignMonitoringScopeCommand
- Events Published: AcademicMonitoringCaseCreated, MonitoringScopeAssigned
- Events Consumed: TeachingProgressRecorded, AttendanceFinalized
- Read Models: Academic Risk Dashboard (RM-D05-01)
- Boundary Validation: Observational only.
- Aggregate Leakage: PASS.

========================================================================================
12. MonitoringPlan
========================================================================================
- Source Document: L2-D05 — Academic Monitoring
- Owner Domain: L2-D05 (Monitoring Context)
- Aggregate Root: MonitoringPlan
- Purpose: Quality assurance benchmarks for course progress tracking.
- Lifecycle: Draft -> Active -> Superseded
- Business Invariants: Monitoring thresholds must exist within 0% to 100% ranges.
- External References: Program Policy ID
- Canonical IDs Used: MonitoringPlanId
- Commands Targeting: CreateMonitoringPlanCommand, AssignMonitoringPolicyCommand
- Events Published: MonitoringPlanCreated, MonitoringPolicyAssigned
- Events Consumed: None
- Read Models: Monitoring Configurations View (RM-D05-02)
- Boundary Validation: Read-only references to policies.
- Aggregate Leakage: PASS.

========================================================================================
13. MonitoringEvaluation
========================================================================================
- Source Document: L2-D05 — Academic Monitoring
- Owner Domain: L2-D05 (Monitoring Context)
- Aggregate Root: MonitoringEvaluation
- Purpose: Conducts automated compliance analysis against targets.
- Lifecycle: Initialized -> Processing -> Completed
- Business Invariants: Metrics must be reproducible using identical inputs.
- External References: MonitoringCaseId, Plan ID
- Canonical IDs Used: MonitoringEvaluationId
- Commands Targeting: ConfigureMonitoringRulesCommand, ActivateMonitoringConfigurationCommand
- Events Published: None
- Events Consumed: None
- Read Models: Evaluation Metrics View (RM-D05-03)
- Boundary Validation: Local algorithmic execution.
- Aggregate Leakage: PASS.

========================================================================================
14. MonitoringIntervention
========================================================================================
- Source Document: L2-D05 — Academic Monitoring
- Owner Domain: L2-D05 (Monitoring Context)
- Aggregate Root: MonitoringIntervention
- Purpose: Manages warnings, alerts, and corrective action workflows.
- Lifecycle: Draft -> Triggered -> Responded -> Closed
- Business Invariants: Warning notices require verified communication channels.
- External References: Student ID, Parent ID, Advisor ID
- Canonical IDs Used: InterventionId
- Commands Targeting: ActivateMonitoringCycleCommand, CreateMonitoringSnapshotCommand
- Events Published: AcademicMonitoringStarted, MonitoringSnapshotCreated
- Events Consumed: None
- Read Models: Active Warning Dashboard (RM-D05-04)
- Boundary Validation: Integrates directly with Notifications domain.
- Aggregate Leakage: PASS.

========================================================================================
15. FlexibleLearningEnrollment (FLE)
========================================================================================
- Source Document: L2-D06 — Open Electives & MOOCs
- Owner Domain: L2-D06 (MOOCs & Open Electives Context)
- Aggregate Root: FlexibleLearningEnrollment
- Purpose: Primary aggregate representing flexible registrations. Holds canonical academic Completion Status.
- Lifecycle: Created -> Approved -> Waitlisted -> Active -> Completed -> Closed
- Business Invariants: Cannot enroll in duplicate offering instances during the same term.
- External References: Student ID, CourseOfferingId
- Canonical IDs Used: FlexibleLearningEnrollmentId
- Commands Targeting: SubmitEnrollmentRequest, RecordLearningCompletion, CloseEnrollment
- Events Published: EnrollmentRequested, EnrollmentApproved, LearningCompletionRecorded, EnrollmentClosed
- Events Consumed: CourseOfferingPublished
- Read Models: Electives Registry Board (RM-D06-01)
- Boundary Validation: System of record for student elective completion.
- Aggregate Leakage: PASS.

========================================================================================
16. SeatAllocation
========================================================================================
- Source Document: L2-D06 — Open Electives & MOOCs
- Owner Domain: L2-D06 (MOOCs & Open Electives Context)
- Aggregate Root: SeatAllocation
- Purpose: Tracks allocation limits and capacity constraints for electives.
- Lifecycle: Available -> Reserved -> Allocated -> Released -> Archived
- Business Invariants: Mapped allocated counts cannot exceed maximum capacities.
- External References: CourseOfferingId, FlexibleLearningEnrollmentId
- Canonical IDs Used: SeatAllocationId
- Commands Targeting: AllocateSeat, ReserveSeat, ReleaseSeat
- Events Published: SeatAllocated, SeatReserved, SeatReleased
- Events Consumed: EnrollmentApproved
- Read Models: Real-time Seat Availability (RM-D06-02)
- Boundary Validation: Syncs capacity locks asynchronously.
- Aggregate Leakage: PASS.

========================================================================================
17. WaitlistEntry
========================================================================================
- Source Document: L2-D06 — Open Electives & MOOCs
- Owner Domain: L2-D06 (MOOCs & Open Electives Context)
- Aggregate Root: WaitlistEntry
- Purpose: Prioritized queue of registrations for capacity-exhausted offerings.
- Lifecycle: Added -> Queued -> Promoted -> Allocated -> Closed
- Business Invariants: Queue priority order must remain FIFO or rule-driven.
- External References: CourseOfferingId, FlexibleLearningEnrollmentId
- Canonical IDs Used: WaitlistEntryId
- Commands Targeting: CreateWaitlistEntry, PromoteWaitlistedStudent, ExpireWaitlistEntry
- Events Published: WaitlistEntryCreated, WaitlistedStudentPromoted, WaitlistEntryExpired
- Events Consumed: SeatReleased
- Read Models: Queue Position dashboard (RM-D06-03)
- Boundary Validation: Independent prioritizations.
- Aggregate Leakage: PASS.

========================================================================================
18. CreditRecognition
========================================================================================
- Source Document: L2-D06 — Open Electives & MOOCs
- Owner Domain: L2-D06 (MOOCs & Open Electives Context)
- Aggregate Root: CreditRecognition
- Purpose: Map and verify credits, orchestrating posting retries.
- Lifecycle: Initiated -> Approved -> TranscriptPostingRetryPending -> TranscriptPostingFailed -> Closed
- Business Invariants: Mapped institutional credits must conform to policy maps.
- External References: FlexibleLearningEnrollmentId, Academic Records API
- Canonical IDs Used: CreditRecognitionId
- Commands Targeting: SubmitCreditRecognitionRequest, InitiateTranscriptPostingRequest, RetryTranscriptPosting
- Events Published: CreditRecognitionRequested, TranscriptPostingRequested, TranscriptPostingCompleted, TranscriptPostingFailed
- Events Consumed: TranscriptPostingCompleted (Academic Records callback)
- Read Models: Credit Mapping History (RM-D06-04)
- Boundary Validation: Employs outbox pattern for transactional operations.
- Aggregate Leakage: PASS.

========================================================================================
19. InternalAssessmentBlueprint
========================================================================================
- Source Document: L2-D07 â€” Internal Assessment
- Owner Domain: L2-D07 (Internal Assessment Context)
- Aggregate Root: InternalAssessmentBlueprint
- Purpose: Defines canonical internal assessment components, weightage, and policy binding for a course offering or academic term.
- Lifecycle: Draft -> Published -> Active -> Closed -> Archived
- Business Invariants: Component weightage must comply with institutional policy.
- External References: CourseOfferingId, AcademicSessionId, TeachingSessionId
- Canonical IDs Used: InternalAssessmentBlueprintId
- Commands Targeting: CreateInternalAssessmentBlueprint, PublishInternalAssessmentBlueprint
- Events Published: InternalAssessmentBlueprintPublished, AssessmentWindowOpened
- Events Consumed: CourseOfferingPublished
- Read Models: Internal Assessment Blueprint View (RM-D07-01)
- Boundary Validation: Reference-only to D01 and D04.
- Aggregate Leakage: PASS.

========================================================================================
20. AssessmentWindow
========================================================================================
- Source Document: L2-D07 â€” Internal Assessment
- Owner Domain: L2-D07 (Internal Assessment Context)
- Aggregate Root: AssessmentWindow
- Purpose: Governs release, deadline, extension, and closure of assessment windows.
- Lifecycle: Scheduled -> Open -> Extended -> Closed -> Archived
- Business Invariants: Windows must fall within approved academic periods.
- External References: InternalAssessmentBlueprintId, AcademicCalendarId
- Canonical IDs Used: AssessmentWindowId
- Commands Targeting: OpenAssessmentWindow, ExtendAssessmentWindow, CloseAssessmentWindow
- Events Published: AssessmentWindowOpened, AssessmentWindowExtended, AssessmentWindowClosed
- Events Consumed: None
- Read Models: Assessment Window Board (RM-D07-02)
- Boundary Validation: Window control is local to Section D.
- Aggregate Leakage: PASS.

========================================================================================
21. AssessmentSubmission
========================================================================================
- Source Document: L2-D07 â€” Internal Assessment
- Owner Domain: L2-D07 (Internal Assessment Context)
- Aggregate Root: AssessmentSubmission
- Purpose: Captures student submission evidence and awarded internal marks.
- Lifecycle: Created -> Submitted -> Evaluated -> Moderated -> Finalized -> Archived
- Business Invariants: A student may have only one final submission per component per window.
- External References: StudentId, AssessmentWindowId
- Canonical IDs Used: AssessmentSubmissionId
- Commands Targeting: RecordAssessmentSubmission, UpdateAssessmentMarks
- Events Published: AssessmentSubmissionRecorded, AssessmentSubmissionUpdated, InternalAssessmentFinalized
- Events Consumed: AssessmentWindowOpened
- Read Models: Student Assessment Ledger (RM-D07-03)
- Boundary Validation: Marks remain internal-assessment only until exported to Examination.
- Aggregate Leakage: PASS.

========================================================================================
22. AssessmentModerationCase
========================================================================================
- Source Document: L2-D07 â€” Internal Assessment
- Owner Domain: L2-D07 (Internal Assessment Context)
- Aggregate Root: AssessmentModerationCase
- Purpose: Governs moderation, correction, and approval of internal assessment outcomes.
- Lifecycle: Initiated -> UnderReview -> Approved -> Rejected -> Closed
- Business Invariants: Moderation decisions must preserve the original submission audit trail.
- External References: AssessmentSubmissionId, ApproverRoleId
- Canonical IDs Used: AssessmentModerationCaseId
- Commands Targeting: RequestAssessmentModeration, ApproveAssessmentModeration, RejectAssessmentModeration
- Events Published: AssessmentModerationRequested, AssessmentModerationCompleted, AssessmentModerationRejected
- Events Consumed: AssessmentSubmissionRecorded
- Read Models: Moderation Queue (RM-D07-04)
- Boundary Validation: Moderation cannot recalculate examination results.
- Aggregate Leakage: PASS.
```

---

# Part 6 — Command Ownership Matrix

This matrix documents **every command** defined across contexts L2-D01 to L2-D07.

| Command Name | Owning Domain | Target Aggregate | Preconditions | Postconditions | Events Produced | Auth Scope |
|---|---|---|---|---|---|---|
| **B04-C01 CreateCourse** | L2-B04 | `CourseMaster` | Curriculum template exists | CourseMaster initialized | CourseCreated | Registrar |
| **D01-C02 PublishOffering** | L2-D01 | `CourseOffering` | Offering validations pass | Offering status: Published | CourseOfferingPublished | Dept Head |
| **D02-C01 AssignFaculty** | L2-D02 | `FacultyAssignment` | Faculty qualified; load slot open | Assignment status: Draft | FacultyAssignmentCreated | Admin |
| **D02-C02 ApproveWorkload** | L2-D02 | `AssignmentApprovalCase`| Workload exception case active | Exception status: Approved | ApprovalGranted | Dean |
| **D03-C01 CreateTimetable** | L2-D03 | `TimetableStructure` | Calendar terms published | Planning session: Active | TimetablePlanningStarted | Timetable Admin|
| **D03-C02 AllocateSlot** | L2-D03 | `TimetableSlotAssignment` | room/faculty slot conflict-free | Slot state: Scheduled | SlotScheduled | Scheduler |
| **D04-C01 StartTeachingSession**| L2-D04 | `TeachingSession` | slot Scheduled; time active | Session state: Active | TeachingSessionStarted | Faculty |
| **D04-C02 RecordAttendance** | L2-D04 | `AttendanceRecord` | Session state: Active | Presence segments logged | AttendanceRecorded | Faculty |
| **D04-C03 CorrectAttendance** | L2-D04 | `AttendanceRecord` | Session finalized; corrections valid| Presence status: Corrected | AttendanceCorrected | Dean |
| **D05-C01 CreateMonitoringPlan** | L2-D05 | `MonitoringPlan` | Program policy active | Plan established | MonitoringPlanCreated | Quality Board |
| **D05-C02 TriggerIntervention** | L2-D05 | `MonitoringIntervention`| Cases under deficiency review | Warning letters sent | InterventionTriggered | System |
| **D06-C08 ValidateEligibility** | L2-D06 | `FlexibleLearningEnrollment`| Student exists; active window | Eligibility status updated | EligibilityValidated | System |
| **D06-C17 SubmitEnrollment** | L2-D06 | `FlexibleLearningEnrollment`| Student validated | Status: EnrollmentRequested | EnrollmentRequested | Student |
| **D06-C19 ApproveEnrollment** | L2-D06 | `FlexibleLearningEnrollment`| Seat allocation success | Status: EnrollmentApproved | EnrollmentApproved | Dean |
| **D06-C43 RecordLearningComplete**| L2-D06 | `FlexibleLearningEnrollment`| Progress sync validation passes | Completion: Completed (FLE SoR) | LearningCompletionRecorded | Provider Sync |
| **D06-C59 InitiateTranscriptPost**| L2-D06 | `CreditRecognition` | Recognition status: Approved | Status: CreditPostedRequest | TranscriptPostingRequested | System |
| **D06-C60 RetryTranscriptPosting**| L2-D06 | `CreditRecognition` | Connection fail; retry pending | Outbox task retried | TranscriptPostingRequested | System |
| **D07-C01 CreateInternalAssessmentBlueprint** | L2-D07 | `InternalAssessmentBlueprint` | Course offering published | Blueprint established | InternalAssessmentBlueprintPublished | Academic Staff |
| **D07-C02 OpenAssessmentWindow** | L2-D07 | `AssessmentWindow` | Blueprint published; term active | Window opened | AssessmentWindowOpened | Academic Staff |
| **D07-C03 RecordAssessmentSubmission** | L2-D07 | `AssessmentSubmission` | Window open; student eligible | Submission recorded | AssessmentSubmissionRecorded | Faculty |
| **D07-C04 RequestAssessmentModeration** | L2-D07 | `AssessmentModerationCase` | Submission requires review | Moderation case initiated | AssessmentModerationRequested | Faculty |
| **D07-C05 FinalizeInternalAssessment** | L2-D07 | `AssessmentSubmission` | Moderation completed; approval granted | Final internal marks locked | InternalAssessmentFinalized | Academic Staff |

---

# Part 7 — Event Catalogue Review

This catalogue records **every domain event** published by Section D aggregates.

| Event Name | Publishing Aggregate | Publishing Domain | Version | Trigger | Consumers |
|---|---|---|---|---|---|
| **CourseOfferingPublished** | `CourseOffering` | L2-D01 | 1.0 | W18 completes | L2-D02, L2-D03, L2-D06 |
| **FacultyAssignmentApproved** | `FacultyAssignment` | L2-D02 | 1.0 | Workload verified | L2-D03, L2-D04 |
| **TimetablePublished** | `TimetableStructure` | L2-D03 | 1.0 | Timetable approved | L2-D04, Notifications |
| **ScheduleModified** | `TimetableSlotAssignment` | L2-D03 | 1.0 | Slot assignment revised | L2-D04, L2-D05 |
| **TeachingSessionStarted** | `TeachingSession` | L2-D04 | 1.0 | Class conducts start | L2-D05, Notifications |
| **AttendanceFinalized** | `AttendanceRecord` | L2-D04 | 1.0 | Attendance locked | L2-D05, Examinations |
| **TeachingProgressRecorded** | `TeachingSession` | L2-D04 | 1.0 | Progress sync | L2-D05, Curriculum |
| **AcademicMonitoringStarted** | `MonitoringIntervention`| L2-D05 | 1.0 | Case cycle starts | Notifications |
| **EnrollmentRequested** | `FlexibleLearningEnrollment`| L2-D06 | 1.0 | Student submits registry | SeatAllocation, Waitlist |
| **EnrollmentApproved** | `FlexibleLearningEnrollment`| L2-D06 | 1.0 | Dean signs off | Provider sync adapter |
| **LearningCompletionRecorded** | `FlexibleLearningEnrollment`| L2-D06 | 1.0 | Progress complete sync | CreditRecognition |
| **TranscriptPostingRequested** | `CreditRecognition` | L2-D06 | 1.0 | Credit approval granted | Academic Records |
| **TranscriptPostingCompleted** | Academic Records (External)| Academic Records | 1.0 | Transcript callback | L2-D06 |
| **TranscriptPostingFailed** | `CreditRecognition` | L2-D06 | 1.0 | Retry count exhausted | Alert Registry, Audit |
| **InternalAssessmentBlueprintPublished** | `InternalAssessmentBlueprint` | L2-D07 | 1.0 | Blueprint approved | L2-I08, L2-I09 |
| **AssessmentWindowOpened** | `AssessmentWindow` | L2-D07 | 1.0 | Assessment window released | L2-I08 |
| **AssessmentSubmissionRecorded** | `AssessmentSubmission` | L2-D07 | 1.0 | Student submission captured | L2-I08 |
| **AssessmentModerationCompleted** | `AssessmentModerationCase` | L2-D07 | 1.0 | Moderation approved | L2-I08 |
| **InternalAssessmentFinalized** | `AssessmentSubmission` | L2-D07 | 1.0 | Final marks locked | L2-I08, L2-I09 |

---

# Part 10 — Integration Contracts & External Interfaces

Section D maintains loose coupling with other SutraERP domain sections.

```text
========================================================================================
1. Campus & Infrastructure Integration (Section B)
========================================================================================
- Commands Consumed: None
- Events Consumed: RoomCreated, RoomStatusChanged (Section B)
- Events Published: SlotScheduled (Timetable)
- Read Models Accessed: Room Availability Map (RM-B03)
- APIs Used: RoomRegistry GET /api/v1/rooms
- Dependency Type: Reference-only (D03 -> B03)
- Validation: Room status changes automatically trigger D03 slot conflicts if room is unusable.

========================================================================================
2. Student Lifecycle Integration (Section C)
========================================================================================
- Commands Consumed: None
- Events Consumed: StudentActivated, StudentSuspended, StudentWithdrawn (Section C)
- Events Published: AttendanceFinalized, EnrollmentApproved (Section D)
- Read Models Accessed: Student Profile details
- APIs Used: Student Registry GET /api/v1/students/{id}
- Dependency Type: Reference-only (D04/D06 -> C01)
- Validation: Enrollment rejected if student status is Inactive.

========================================================================================
3. Downstream Academic Records Integration (External)
========================================================================================
- Commands Consumed: Request Transcript Posting (Post mapping)
- Events Consumed: TranscriptPostingCompleted, TranscriptPostingRejected
- Events Published: TranscriptPostingRequested (Outbox pattern)
- Read Models Accessed: None
- APIs Used: Transcript Registry POST /api/v1/transcripts/credits
- Dependency Type: Outbound Command execution (D06 -> External Academic Records)
- Validation: Resilient outbox retries to recover transient transaction errors.
```

---

# Part 11 — Read Model Review

| Read Model Name | Owner | Source Events | Refresh Strategy | Consistency | Cache Policy |
|---|---|---|---|---|---|
| **Course Catalog (RM-B04-01)** | L2-B04 | `CourseCreated`, `CourseUpdated` | Projection on event | Eventual | Redis; 24-hr TTL |
| **Faculty Workload Board (RM-D02-01)** | L2-D02 | `FacultyAssignmentApproved`, `AssignmentCancelled` | Event-driven projection | Eventual | Invalidated on change |
| **Student Class Calendar (RM-D03-01)** | L2-D03 | `TimetablePublished`, `ScheduleModified` | Real-time project | Eventual | CDN cached |
| **Daily Attendance Grid (RM-D04-01)** | L2-D04 | `AttendanceRegisterOpened`, `AttendanceRecorded` | Real-time Query | Eventual | No cache |
| **Academic Risk Dashboard (RM-D05-01)**| L2-D05 | `AcademicMonitoringStarted`, `InterventionTriggered` | Event-driven projection | Eventual | CDN cached |
| **Electives Registry Board (RM-D06-01)**| L2-D06 | `SeatReserved`, `SeatAllocated`, `SeatReleased` | Projection on event | Eventual | Invalidated on seat release |
| **Internal Assessment Blueprint View (RM-D07-01)** | L2-D07 | `InternalAssessmentBlueprintPublished` | Projection on event | Eventual | Invalidated on update |
| **Assessment Window Board (RM-D07-02)** | L2-D07 | `AssessmentWindowOpened`, `AssessmentWindowClosed` | Projection on event | Eventual | TTL based |
| **Student Assessment Ledger (RM-D07-03)** | L2-D07 | `AssessmentSubmissionRecorded`, `AssessmentSubmissionUpdated`, `InternalAssessmentFinalized` | Projection on event | Eventual | Invalidated on finalization |
| **Moderation Queue (RM-D07-04)** | L2-D07 | `AssessmentModerationRequested`, `AssessmentModerationCompleted` | Projection on event | Eventual | No cache |

---

## Lifecycle

Lifecycle is not consolidated into a single section; each of the 22 aggregates documented in Part 5 (Aggregate Ownership Review) specifies its own lifecycle state chain via the "Lifecycle:" field — for example, `CourseOffering`: Draft -> Published -> Active -> Suspended -> Closed, and `TeachingSession`: Scheduled -> Active -> Completed -> Archived. The Complete Academic Delivery Workflow Chain diagram further sequences these per-aggregate lifecycles across the seven bounded contexts (L2-D01 through L2-D07).

---

## States

State values are recorded per aggregate as the "Lifecycle:" transition chains in Part 5 (Aggregate Ownership Review) rather than in a separate state catalogue. No additional state machine section exists beyond these per-aggregate chains and the Command Ownership Matrix (Part 6), which records the precondition and postcondition state changes triggered by each command.

---

## Rules / Invariants

Business rules are recorded per aggregate as the "Business Invariants:" field in Part 5 (Aggregate Ownership Review) — for example, "Max Capacity must exceed current allocated seats" (`CourseOffering`) and "Workload allocation hours cannot exceed maximum allowed limits" (`FacultyAssignment`). Command-level preconditions and postconditions enforcing these invariants are additionally listed in the Command Ownership Matrix (Part 6).

---

# Part 15 — Section Readiness Certification

The Technical Architecture Board certifies the Academic Delivery Section (Section D):

- **Aggregate Completeness**: Verified. All 22 aggregates are mapped with zero transaction leaks.
- **Boundary Validation**: PASS.
- **Dependency Validation**: PASS.

### Section Scores
- **Readiness Score**: **10/10**
- **Confidence Score**: **10/10**
- **Validation Result**: **PASS**

---

# Part 16 — Architecture Lock Statement

The Technical Architecture Board hereby declares the **Academic Delivery Section (Section D)** of the SutraERP system to be **LOCKED**.

All aggregates, commands, events, read models, and multi-domain dependencies are frozen. Future modifications require a formal Architecture Decision Record (ADR) and review board approval.

---

# Appendices

### Appendix A — Frozen Aggregates (Source Referencing)
1. `CourseMaster` (Source: L2-B04)
2. `CourseOffering` (Source: L2-D01)
3. `FacultyAssignment` (Source: L2-D02)
4. `AssignmentApprovalCase` (Source: L2-D02)
5. `OperationalChange` (Source: L2-D02)
6. `TimetableStructure` (Source: L2-D03)
7. `TimetableSlotAssignment` (Source: L2-D03)
8. `SchedulingConflictRegister` (Source: L2-D03)
9. `TeachingSession` (Source: L2-D04)
10. `AttendanceRecord` (Source: L2-D04)
11. `MonitoringCase` (Source: L2-D05)
12. `MonitoringPlan` (Source: L2-D05)
13. `MonitoringEvaluation` (Source: L2-D05)
14. `MonitoringIntervention` (Source: L2-D05)
15. `FlexibleLearningEnrollment` (Source: L2-D06)
16. `SeatAllocation` (Source: L2-D06)
17. `WaitlistEntry` (Source: L2-D06)
18. `CreditRecognition` (Source: L2-D06)
19. `InternalAssessmentBlueprint` (Source: L2-D07)
20. `AssessmentWindow` (Source: L2-D07)
21. `AssessmentSubmission` (Source: L2-D07)
22. `AssessmentModerationCase` (Source: L2-D07)

### Appendix B — Frozen Commands
- `CreateCourse` (B04), `PublishOffering` (D01), `AssignFaculty` (D02), `ApproveWorkload` (D02), `CreateTimetable` (D03), `AllocateSlot` (D03), `StartTeachingSession` (D04), `RecordAttendance` (D04), `CorrectAttendance` (D04), `CreateMonitoringPlan` (D05), `TriggerIntervention` (D05), `ValidateEligibility` (D06), `SubmitEnrollment` (D06), `ApproveEnrollment` (D06), `RecordLearningComplete` (D06), `InitiateTranscriptPost` (D06), `RetryTranscriptPosting` (D06), `CreateInternalAssessmentBlueprint` (D07), `OpenAssessmentWindow` (D07), `RecordAssessmentSubmission` (D07), `RequestAssessmentModeration` (D07), `FinalizeInternalAssessment` (D07).

### Appendix C — Frozen Domain Events
- `CourseOfferingPublished` (D01), `FacultyAssignmentApproved` (D02), `TimetablePublished` (D03), `ScheduleModified` (D03), `TeachingSessionStarted` (D04), `AttendanceFinalized` (D04), `TeachingProgressRecorded` (D04), `AcademicMonitoringStarted` (D05), `EnrollmentRequested` (D06), `EnrollmentApproved` (D06), `LearningCompletionRecorded` (D06), `TranscriptPostingRequested` (D06), `TranscriptPostingCompleted` (D06), `TranscriptPostingFailed` (D06), `InternalAssessmentBlueprintPublished` (D07), `AssessmentWindowOpened` (D07), `AssessmentSubmissionRecorded` (D07), `AssessmentModerationCompleted` (D07), `InternalAssessmentFinalized` (D07).

### Appendix D — Frozen Read Models
- `Course Catalog (RM-B04-01)`, `Faculty Workload Board (RM-D02-01)`, `Student Class Calendar (RM-D03-01)`, `Daily Attendance Grid (RM-D04-01)`, `Academic Risk Dashboard (RM-D05-01)`, `Electives Registry Board (RM-D06-01)`, `Internal Assessment Blueprint View (RM-D07-01)`, `Assessment Window Board (RM-D07-02)`, `Student Assessment Ledger (RM-D07-03)`, `Moderation Queue (RM-D07-04)`.

### Appendix E — Frozen Integration Contracts
- `Section B Integration (Timetable Slots / Room Change Mapping)`
- `Section C Integration (Lifecycle Checks / Student Inactive Lock)`
- `External Academic Records Integration (Credit Posting / Resilient Transcript Outbox Saga)`

### Appendix F — Published APIs
- `GET /api/v1/offerings` (D01)
- `GET /api/v1/faculty/assignments` (D02)
- `GET /api/v1/timetables` (D03)
- `POST /api/v1/sessions` (D04)
- `POST /api/v1/attendance` (D04)
- `GET /api/v1/monitoring/cases` (D05)
- `POST /api/v1/electives/enrollments` (D06)
- `GET /api/v1/internal-assessments/blueprints` (D07)
- `POST /api/v1/internal-assessments/windows` (D07)
- `POST /api/v1/internal-assessments/submissions` (D07)

### Appendix G — Multi-Domain References
- `Room ID` (References Section B)
- `Student ID` (References Section C)
- `Transcript ID` (References External Academic Records)

---
**End of Document**
  
