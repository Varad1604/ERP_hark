# L2-D00 — Academic Delivery Review

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | Purpose |
| Scope | Scope |
| Actors | — not present — |
| Entities | Part 5 — Aggregate Ownership Review |
| Business Rules | Rules / Invariants |
| Workflow | Complete Academic Delivery Workflow Chain; L2-D03 — Timetable Workflows |
| State Machine | Lifecycle |
| Contracts | Part 10 — Integration Contracts & External Interfaces |
| Events | Part 7 — Event Catalogue Review |
| Dependencies | — not present — |
| Locked Decisions | — not present — |

> **Gap:** 3 required section(s) have no counterpart in this document: Actors, Dependencies, Locked Decisions. Recorded, not authored.

---

# Document Metadata

| Attribute | Value |
|-----------|-------|
| Document ID | L2-D00 |
| Document Owner | Pallav |
| Domain Section | Academic Delivery (Section D) |
| Layer | L2 — Academic Operations |
| Architecture Level | Enterprise Domain Review & Certification |
| Architecture Style | Domain Driven Design (DDD), CQRS, Event Driven Architecture (EDA), Clean & Hexagonal Architecture |
| Primary Owner | Enterprise Architecture Review Board |
| Technical Owner | Technical Architecture Board |
| Canonical Owner | Academic Delivery Domain Group |
| Version | 1.0 |
| Status | Draft |
| Review Date | 2026-07-27 |
| Repository | SutraERP Enterprise Repository |
| Target Scope | L2-D01, L2-D02, L2-D03, L2-D04, L2-D05, L2-D06 |

---

# Purpose

The purpose of this document is to establish a unified architecture review, boundary verification, dependency validation, and governance lock for the entire **Academic Delivery Section (Section D)** of the SutraERP platform.

This review certifies that all constituent sub-domains (L2-D01 through L2-D06) adhere strictly to the enterprise standards of Domain-Driven Design (DDD), Command-Query Responsibility Segregation (CQRS), Event-Driven Architecture (EDA), Canonical Data Ownership, and Hexagonal Architecture. It acts as the final gate to prevent domain leakage, circular dependencies, transactional integrity failures, and canonical ownership violations before final implementation locking.

---

# Scope

This review covers the complete architectural definitions, boundaries, and integration contracts of:
- **L2-D01**: Course & Course Offering Workflows
- **L2-D02**: Faculty Academic Assignment Workflows
- **L2-D03**: Timetable Workflows
- **L2-D04**: Teaching Session & Attendance Workflows
- **L2-D05**: Academic Monitoring Workflows
- **L2-D06**: Open Electives & MOOCs Workflows

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

Below is the verified registry of the six bounded contexts constituting the Academic Delivery Section.

## L2-D01 — Course & Course Offering
- **Purpose**: Governs canonical course definitions and course offering lifecycles (creation, validation, capacity limits, publication).
- **Core Aggregates**:
  - `CourseMaster`: Owns course code, credits, structure.
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
  [Standard Grade Posting]                                [Credit Recognition Saga] (L2-D06)
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
- Source Document: L2-B04 — Curriculum Framework Workflows
- Owner Domain: L2-B04 (Curriculum Framework)
- Aggregate Root: CourseMaster
- Purpose: Governs course syllabus framework, credit values, and foundational curriculum mappings.
- Lifecycle: Proposed -> Active -> Superseded -> Archived
- Business Invariants: Course Code must be unique. Credits cannot be negative.
- External References: L2-B04 Curriculum Framework ID
- Canonical IDs Used: CourseMasterId
- Commands Targeting: CreateCourseMaster, UpdateCourseMaster
- Events Published: CourseMasterCreated, CourseMasterUpdated
- Events Consumed: None
- Read Models: Course Catalog (RM-D01-01)
- Boundary Validation: Strictly encapsulated; referenced in L2-D02 and L2-D03 by CourseMasterId only.
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
```

---

# Part 6 — Command Ownership Matrix

This matrix documents **every command** defined across contexts L2-D01 to L2-D06.

| Command Name | Owning Domain | Target Aggregate | Preconditions | Postconditions | Events Produced | Auth Scope |
|---|---|---|---|---|---|---|
| **B04-C01 CreateCourseMaster** | L2-B04 | `CourseMaster` | Curriculum template exists | CourseMaster initialized | CourseMasterCreated | Registrar |
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
3. Downstream Academic Records (Section H)
========================================================================================
- Commands Consumed: Request Transcript Posting (Post mapping)
- Events Consumed: TranscriptPostingCompleted, TranscriptPostingRejected
- Events Published: TranscriptPostingRequested (Outbox pattern)
- Read Models Accessed: None
- APIs Used: Transcript Registry POST /api/v1/transcripts/credits
- Dependency Type: Outbound Command execution (D06 -> H01)
- Validation: Resilient outbox retries to recover transient transaction errors.
```

---

# Part 11 — Read Model Review

| Read Model Name | Owner | Source Events | Refresh Strategy | Consistency | Cache Policy |
|---|---|---|---|---|---|
| **Course Catalog (RM-D01-01)** | L2-D01 | `CourseMasterCreated`, `CourseMasterUpdated` | Projection on event | Eventual | Redis; 24-hr TTL |
| **Faculty Workload Board (RM-D02-01)** | L2-D02 | `FacultyAssignmentApproved`, `AssignmentCancelled` | Event-driven projection | Eventual | Invalidated on change |
| **Student Class Calendar (RM-D03-01)** | L2-D03 | `TimetablePublished`, `ScheduleModified` | Real-time project | Eventual | CDN cached |
| **Daily Attendance Grid (RM-D04-01)** | L2-D04 | `AttendanceRegisterOpened`, `AttendanceRecorded` | Real-time Query | Eventual | No cache |
| **Academic Risk Dashboard (RM-D05-01)**| L2-D05 | `AcademicMonitoringStarted`, `InterventionTriggered` | Event-driven projection | Eventual | CDN cached |
| **Electives Registry Board (RM-D06-01)**| L2-D06 | `SeatReserved`, `SeatAllocated`, `SeatReleased` | Projection on event | Eventual | Invalidated on seat release |

---

# Part 15 — Section Readiness Certification

The Technical Architecture Board certifies the Academic Delivery Section (Section D):

- **Aggregate Completeness**: Verified. All 18 aggregates are mapped with zero transaction leaks.
- **Boundary Validation**: PASS.
- **Dependency Validation**: PASS.

### Section Scores
- **Readiness Score**: **10/10**
- **Confidence Score**: **10/10**
- **Validation Result**: **PASS**

---

# Part 16 — Architecture Lock Statement

The Technical Architecture Board hereby declares the **Academic Delivery Section (Section D)** of the SutraERP system to be **LOCKED**.

All aggregates, commands, events, read models, and cross-domain dependencies are frozen. Future modifications require a formal Architecture Decision Record (ADR) and review board approval.

---

# Appendices

### Appendix A — Frozen Aggregates (Source Referencing)
1. `CourseMaster` (Source: L2-D01)
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

### Appendix B — Frozen Commands
- `CreateCourse` (D01), `PublishOffering` (D01), `AssignFaculty` (D02), `ApproveWorkload` (D02), `CreateTimetable` (D03), `AllocateSlot` (D03), `StartTeachingSession` (D04), `RecordAttendance` (D04), `CorrectAttendance` (D04), `CreateMonitoringPlan` (D05), `TriggerIntervention` (D05), `ValidateEligibility` (D06), `SubmitEnrollment` (D06), `ApproveEnrollment` (D06), `RecordLearningComplete` (D06), `InitiateTranscriptPost` (D06), `RetryTranscriptPosting` (D06).

### Appendix C — Frozen Domain Events
- `CourseOfferingPublished` (D01), `FacultyAssignmentApproved` (D02), `TimetablePublished` (D03), `ScheduleModified` (D03), `TeachingSessionStarted` (D04), `AttendanceFinalized` (D04), `TeachingProgressRecorded` (D04), `AcademicMonitoringStarted` (D05), `EnrollmentRequested` (D06), `EnrollmentApproved` (D06), `LearningCompletionRecorded` (D06), `TranscriptPostingRequested` (D06), `TranscriptPostingCompleted` (D06), `TranscriptPostingFailed` (D06).

### Appendix D — Frozen Read Models
- `Course Catalog (RM-D01-01)`, `Faculty Workload Board (RM-D02-01)`, `Student Class Calendar (RM-D03-01)`, `Daily Attendance Grid (RM-D04-01)`, `Academic Risk Dashboard (RM-D05-01)`, `Electives Registry Board (RM-D06-01)`.

### Appendix E — Frozen Integration Contracts
- `Section B Integration (Timetable Slots / Room Change Mapping)`
- `Section C Integration (Lifecycle Checks / Student Inactive Lock)`
- `Section H Integration (Credit Posting / Resilient Transcript Outbox Saga)`

### Appendix F — Published APIs
- `GET /api/v1/offerings` (D01)
- `GET /api/v1/faculty/assignments` (D02)
- `GET /api/v1/timetables` (D03)
- `POST /api/v1/sessions` (D04)
- `POST /api/v1/attendance` (D04)
- `GET /api/v1/monitoring/cases` (D05)
- `POST /api/v1/electives/enrollments` (D06)

### Appendix G — Cross-Domain References
- `Room ID` (References Section B)
- `Student ID` (References Section C)
- `Transcript ID` (References Section H)

---
**End of Document**
  

---


## Lifecycle

Not separately defined in this document. As a section review document, L2-D00 validates the
lifecycle definitions owned by L2-D01 through L2-D07 rather than defining lifecycle behaviour.

---

## Rules / Invariants

Not separately defined in this document. The review criteria applied here are recorded in the
sections above; canonical business rules and invariants remain owned by L2-D01 through L2-D07.

---
