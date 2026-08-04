# L2-H04 — Placement Workflows

**Document ID:** L2-H04  
**Domain:** Section H — Institutional Services & Intelligence  
**Module:** Placement Management  
**Document Type:** Level 2 Enterprise Workflow Specification  
**Architecture Style:** Domain-Driven Design (DDD), CQRS, Event-Driven Architecture (EDA)  
**Version:** 1.0  
**Status:** Draft  
**Owner:** Enterprise Architecture Team  
**Document Owner:** Dhruv  

---

# 1. Purpose

This document defines the complete enterprise workflow architecture for the Placement Management domain within SutraERP.

The objective of this module is to provide a centralized, configurable, and auditable platform for managing institutional placement activities throughout the complete placement lifecycle. The module enables educational institutions to coordinate employers, placement drives, student participation, internship opportunities, selection processes, offer management, joining confirmation, alumni placement tracking, and placement analytics while maintaining complete traceability and policy compliance.

This document establishes:

- Functional boundaries of the Placement domain
- Business capabilities
- Aggregate ownership
- Workflow orchestration
- Commands
- Events
- Read models
- Integration contracts
- Security responsibilities
- Audit requirements
- Extension points

This specification serves as the authoritative architectural reference for implementation teams, integration architects, QA teams, and future Level 3 workflow decomposition.

---

# 2. Objectives

The Placement Management module is designed to achieve the following enterprise objectives:

- Digitize the complete institutional placement lifecycle.
- Standardize employer engagement processes.
- Enable configurable placement policies for different institutions.
- Automate student eligibility verification and screening.
- Support configurable multi-stage recruitment workflows.
- Provide complete auditability of every placement decision.
- Maintain transparent employer–institution collaboration.
- Support internship and final placement processes within a unified architecture.
- Provide accurate institutional placement statistics.
- Enable event-driven integrations across enterprise domains.
- Ensure scalability for institutions conducting thousands of placement activities annually.

---

# 3. Scope

The Placement Management domain owns workflows related to institutional placement operations from employer onboarding until placement drive closure and post-placement tracking.

The domain includes:

- Employer Management
- Company Verification
- Placement Drive Management
- Position Management
- Eligibility Configuration
- Student Registration for Placement Drives
- Resume and Candidate Profile Management
- AI-Assisted Resume Screening
- Manual Candidate Screening
- Student Shortlisting
- Pre-Placement Talk (PPT) Management
- Assessment Round Management
- Group Discussion Management
- Technical Interview Management
- HR Interview Management
- Offer Management
- Offer Acceptance and Rejection
- Joining Confirmation
- Internship Management
- Alumni Placement Tracking
- Placement Statistics
- Placement Reporting
- Placement Notifications
- Placement Audit History

---

# 4. Out of Scope

The Placement domain intentionally excludes ownership of the following business capabilities.

These capabilities remain owned by their respective enterprise domains.

| Capability | Owning Section |
|------------|----------------|
| Student Master | Section C |
| Student Admissions | Section C |
| Academic Records | Section D |
| Examination Results | Section D |
| Attendance | Section D |
| Faculty Management | Section E |
| Employee Management | Section E |
| Authentication | Section A |
| Authorization | Section A |
| Enterprise Notifications | Section H02 |
| Enterprise Search | Section H03 |
| Enterprise Dashboards | Section H07 |
| Financial Accounting | Section K |
| Fee Management | Section K |
| Procurement | Section J |

The Placement module consumes information from these domains through enterprise APIs, read models, and published events without assuming ownership of their business entities.

---

# 5. Architectural Position

Within the SutraERP enterprise architecture, Placement Management is classified as an **Institutional Service Domain**.

Unlike academic or administrative domains that own canonical institutional records, Placement Management orchestrates placement-related business processes by consuming authoritative information published by multiple enterprise domains.

The module acts as an orchestration layer responsible for coordinating employer engagement, student participation, recruitment workflows, and placement outcomes while maintaining strict separation from academic ownership.

Employer-originated business actions are initiated outside the system and recorded by authorized Placement Officers or Placement Coordinators; no employer authentication or employer user account is assumed.

Placement Management therefore:

- Consumes student information from the Student domain.
- Consumes academic eligibility information from Academic Management.
- Consumes notifications through the Enterprise Notification service.
- Publishes placement events for downstream analytics and reporting.
- Exposes placement-specific read models for dashboards and institutional intelligence.

This architectural separation prevents duplication of canonical records while ensuring enterprise-wide consistency.

---

# 6. Guiding Architectural Principles

The Placement Management module follows the core architectural principles established for the SutraERP platform.

## 6.1 Domain-Driven Design

Business capabilities are organized into clearly defined aggregates with explicit ownership, lifecycle boundaries, and transactional consistency.

---

## 6.2 CQRS

Commands modify business state.

Queries retrieve optimized read models.

Command processing and read projections remain independently scalable.

---

## 6.3 Event-Driven Architecture

Every significant business action produces immutable domain events.

Events become the authoritative mechanism for inter-domain communication, workflow orchestration, audit generation, notification triggering, analytics, and reporting.

---

## 6.4 Configurable Business Policies

Institution-specific placement rules shall not be hardcoded.

Policies such as:

- Dream company eligibility
- Super Dream policies
- Maximum applications
- Multiple offer rules
- Internship conversion
- Company-specific restrictions

shall be configurable through institutional policy definitions.

---

## 6.5 Complete Auditability

Every business decision affecting employers, placement drives, candidate progression, interviews, offers, internships, or joining confirmation shall generate immutable audit records.

Historical records must remain permanently traceable.

---

## 6.6 Workflow Configurability

Recruitment workflows vary between employers.

The system shall support configurable recruitment templates containing arbitrary combinations of:

- Resume Screening
- Aptitude Tests
- Coding Tests
- Group Discussions
- Technical Interviews
- HR Interviews
- Custom Evaluation Rounds

without requiring software customization.

---

## 6.7 Loose Coupling

Placement Management shall communicate with other enterprise domains exclusively through:

- Published Events
- Enterprise Commands
- Query APIs
- Read Models

Direct database coupling between domains is prohibited.

---

# 7. Business Vision

The long-term vision of Placement Management is to establish a unified enterprise platform that enables institutions to efficiently coordinate employers, students, placement officers, recruiters, alumni, and institutional leadership throughout the entire placement ecosystem.

The platform shall provide transparent recruitment processes, configurable institutional policies, AI-assisted candidate evaluation, comprehensive auditability, real-time operational visibility, and accurate placement intelligence while remaining extensible for evolving recruitment practices and enterprise integrations.

# 8. Domain Overview

Placement Management is the enterprise capability responsible for orchestrating institutional recruitment activities between employers and students while ensuring compliance with institutional placement policies.

The domain manages the complete lifecycle of placement and internship activities beginning with employer engagement and continuing through recruitment planning, candidate evaluation, offer management, joining confirmation, drive closure, and post-placement reporting.

Rather than owning academic or student information, Placement Management consumes authoritative data from upstream enterprise domains and coordinates business workflows using commands, events, and read models.

The domain is designed to support institutions ranging from small colleges conducting a few annual drives to universities coordinating hundreds of employers and thousands of students simultaneously.

---

# 9. Business Capabilities

The Placement Management domain provides the following enterprise business capabilities.

## 9.1 Employer Management

Responsible for maintaining employer organizations participating in institutional recruitment activities.

Capabilities include:

- Employer Registration
- Employer Verification
- Employer Classification
- Industry Categorization
- Employer Contact Management
- Employer Communication History
- Employer Status Management
- Employer Engagement Tracking

---

## 9.2 Placement Drive Management

Responsible for planning, executing, monitoring, and closing recruitment drives.

Capabilities include:

- Drive Creation
- Drive Scheduling
- Position Assignment
- Venue Planning
- Online Drive Configuration
- Recruitment Timeline Management
- Drive Progress Monitoring
- Drive Closure

---

## 9.3 Position Management

Responsible for managing employment opportunities offered by employers.

Capabilities include:

- Position Creation
- Position Modification
- Compensation Definition
- Employment Type
- Internship Definition
- Hiring Targets
- Job Description Management
- Position Status Tracking

---

## 9.4 Eligibility Management

Responsible for defining institution-specific and employer-specific eligibility requirements.

Capabilities include:

- Academic Eligibility
- CGPA Requirements
- Percentage Requirements
- Active Backlog Validation
- Branch Eligibility
- Program Eligibility
- Qualification Completion Year Validation
- Skills Validation
- Certification Validation
- Resume Completeness Validation
- Employer-Specific Rules
- Institutional Placement Policy Validation

---

## 9.5 Student Registration

Responsible for student participation in placement opportunities.

Capabilities include:

- Registration
- Application Withdrawal
- Resume Submission
- Resume Updates
- Candidate Profile Submission
- Consent Management
- Registration Validation

---

## 9.6 Candidate Screening

Responsible for evaluating candidates before recruitment stages.

Capabilities include:

- Automated Screening
- AI Resume Screening
- Manual Screening
- Eligibility Verification
- Position Requirement Matching
- Shortlisting
- Candidate Ranking
- Candidate Reconsideration

---

## 9.7 Recruitment Process Management

Responsible for executing configurable recruitment workflows.

Capabilities include:

- Recruitment Templates
- Assessment Scheduling
- Coding Tests
- Aptitude Tests
- Group Discussions
- Technical Interviews
- HR Interviews
- Custom Evaluation Rounds
- Candidate Progress Tracking

---

## 9.8 Offer Management

Responsible for managing employment offers.

Capabilities include:

- Offer Generation
- Offer Distribution
- Offer Acceptance
- Offer Rejection
- Offer Withdrawal
- Offer Expiry
- Offer Audit

---

## 9.9 Internship Management

Responsible for internship-specific recruitment activities.

Capabilities include:

- Internship Positions
- Internship Applications
- Internship Selection
- Internship Completion Tracking
- Internship Conversion
- Internship Reporting

---

## 9.10 Joining Management

Responsible for confirming successful placement completion.

Capabilities include:

- Joining Confirmation
- Employer Confirmation
- Student Confirmation
- Joining Status Updates
- Placement Completion

---

## 9.11 Placement Analytics

Responsible for enterprise reporting and operational intelligence.

Capabilities include:

- Placement Statistics
- Department Reports
- Company Reports
- Package Analysis
- Recruitment Trend Analysis
- Student Success Metrics
- Internship Analytics
- Alumni Placement Analytics

---

# 10. Bounded Context

The Placement Management bounded context owns business processes related to employer engagement and institutional recruitment activities.

It is responsible for maintaining placement-specific aggregates while consuming authoritative information from other enterprise domains.

The bounded context includes:

- Employer
- Placement Drive
- Position
- Candidate Application
- Recruitment Workflow
- Placement Offer
- Internship
- Joining Confirmation

These aggregates collectively define all transactional boundaries within the Placement domain.

---

# 11. Core Responsibilities

The Placement Management domain is responsible for:

- Managing employer organizations.
- Registering and verifying companies.
- Managing recruitment drives.
- Managing employment and internship positions.
- Defining eligibility criteria.
- Registering student applications.
- Validating institutional placement policies.
- Screening candidates.
- Publishing shortlisted candidates.
- Executing configurable recruitment workflows.
- Recording recruitment outcomes.
- Managing employment offers.
- Managing internship offers.
- Recording joining confirmations.
- Producing placement statistics.
- Publishing placement events for downstream enterprise consumers.

---

# 12. Aggregate Ownership

The Placement domain owns the following aggregates.

| Aggregate | Description |
|------------|-------------|
| Employer | Registered recruiting organization |
| PlacementDrive | Recruitment campaign conducted by an employer |
| Position | Employment or internship opportunity offered during a drive |
| CandidateApplication | Student participation in a placement opportunity |
| RecruitmentWorkflow | Configurable recruitment stages for a position |
| PlacementOffer | Employment or internship offer issued to a candidate |
| Internship | Internship engagement lifecycle |
| JoiningConfirmation | Confirmation of successful joining after offer acceptance |

## 12.1 Aggregate Ownership Matrix

| Aggregate | Aggregate Root | Owner | Primary Commands | Primary Events | Primary Read Models |
|-----------|----------------|-------|------------------|----------------|---------------------|
| Employer | Employer | Placement Officer | RegisterEmployer, CreateEmployer, ReviewEmployerVerification, ApproveEmployer, RejectEmployer, UpdateEmployerStatus | EmployerRegistrationInitiated, EmployerRegistered, EmployerVerificationRequested, EmployerVerified, EmployerVerificationRejected, EmployerStatusUpdated | Employer Directory, Employer Verification Queue, Verified Employer List, Employer Dashboard |
| PlacementDrive | PlacementDrive | Placement Officer / Placement Coordinator | CreatePlacementDrive, SavePlacementDrive, ConfigureDriveSchedule, AssignEmployerToDrive, ReviewPlacementDriveCompletion, GeneratePlacementStatistics, ArchivePlacementDrive, ClosePlacementDrive | PlacementDriveCreated, DriveScheduleConfigured, EmployerAssignedToDrive, PlacementStatisticsGenerated, PlacementDriveArchived, PlacementDriveClosed | Placement Drive Dashboard, Upcoming Placement Drives, Employer Drive History, Historical Placement Drives, Institutional Placement Statistics |
| Position | Position | Placement Officer / Placement Coordinator | CreatePosition, SavePosition, ConfigureCompensation, ConfigureHiringCapacity, ConfigureEligibility, AddAcademicCriteria, AddEmployerCriteria, PublishEligibilityRules, PublishPosition, ClosePosition | PositionCreated, CompensationConfigured, HiringCapacityDefined, EligibilityConfigured, AcademicCriteriaAdded, EmployerCriteriaAdded, EligibilityPublished, PositionPublished, PositionClosed | Position Catalogue, Drive Positions, Employer Position Summary, Position Screening Summary |
| CandidateApplication | CandidateApplication | Placement Officer / Placement Coordinator | RegisterCandidate, ValidateEligibility, ValidateAcademicEligibility, ValidatePlacementPolicy, CreateCandidateApplication, InitiateCandidateScreening, ExecuteAIResumeScreening, MatchPositionRequirements, PerformManualScreening, FinalizeScreening, GenerateCandidateRanking, ApplyShortlistingCriteria, ApproveShortlist, PublishShortlist, RequestCandidateReconsideration, ReviewCandidateElimination, ReinstateCandidate, ReopenCandidateEligibility, UpdateCandidateOfferStatus | CandidateRegistrationInitiated, CandidateEligibilityValidated, CandidateRegistered, CandidateApplicationCreated, CandidateScreeningStarted, AcademicEligibilityValidated, PlacementPolicyValidated, AIResumeScreeningCompleted, PositionRequirementsMatched, ManualScreeningCompleted, CandidateScreened, CandidateRankingGenerated, CandidateShortlisted, ShortlistApproved, ShortlistPublished, CandidateReconsiderationRequested, CandidateReinstated, CandidateEligibilityReopened | Candidate Applications, Position Applicants, Student Placement Dashboard, Screening Dashboard, Candidate Ranking View, Eligible Candidate List, Shortlisted Candidates, Candidate Status Dashboard, Placement Offer Dashboard |
| RecruitmentWorkflow | RecruitmentWorkflow | Placement Officer / Placement Coordinator | StartRecruitmentRound, RecordEvaluation, SubmitRoundResults, AdvanceCandidate, EliminateCandidate, ResumeRecruitmentWorkflow | RecruitmentRoundStarted, CandidateEvaluated, RoundResultsSubmitted, RecruitmentRoundCompleted, RecruitmentWorkflowResumed | Recruitment Progress Dashboard, Candidate Progress View, Recruitment Round Results, Employer Recruitment Dashboard |
| PlacementOffer | PlacementOffer | Placement Officer | FinalizeCandidateSelection, GeneratePlacementOffer, ValidateOfferDetails, PublishOffer, AcceptPlacementOffer, RejectPlacementOffer, RequestOfferWithdrawal, ApproveOfferWithdrawal, WithdrawPlacementOffer, NotifyEmployer, NotifyCandidate | CandidateSelected, PlacementOfferGenerated, PlacementOfferPublished, CandidateOfferStatusUpdated, PlacementOfferAccepted, PlacementOfferRejected, OfferWithdrawalRequested, PlacementOfferWithdrawn, EmployerNotified, CandidateNotified | Placement Offer Dashboard, Candidate Offer View, Employer Offer Summary, Offer Dashboard, Employer Recruitment Dashboard |
| Internship | Internship | Placement Officer | PublishInternshipPosition, RegisterInternshipCandidate, GenerateInternshipOffer, StartInternship, CompleteInternship, ConvertInternshipToPlacement | InternshipPublished, InternshipCandidateRegistered, InternshipOfferGenerated, InternshipStarted, InternshipCompleted, InternshipConvertedToPlacement | Internship Dashboard, Internship Progress View, Internship Analytics, Placement Conversion Dashboard |
| JoiningConfirmation | JoiningConfirmation | Placement Officer | RecordStudentJoining, RecordEmployerConfirmation, VerifyJoining, CompletePlacement | StudentJoiningRecorded, EmployerJoiningConfirmed, JoiningVerified, PlacementCompleted | Placement Completion Dashboard, Alumni Placement Dashboard, Employer Hiring Dashboard, Institutional Placement Statistics |

No other enterprise domain shall directly modify these aggregates.

---

# 13. External Dependencies

Placement Management depends on multiple enterprise domains to execute its workflows.

| Domain | Purpose |
|---------|----------|
| Section A | Authentication, Authorization, Identity |
| Section C | Student Master Data |
| Section D | Academic Records, Results, Eligibility |
| Section E | Faculty and Placement Coordinator Information |
| Section H02 | Enterprise Notifications |
| Section H03 | Enterprise Directory and Search |
| Section H07 | Enterprise Dashboards |
| External Services | LinkedIn, AI Resume Screening, Email, SMS, WhatsApp |

These dependencies are consumed through enterprise APIs, CQRS read models, and published domain events. The Placement domain shall never directly manipulate the data owned by these external domains.

---

# 14. Upstream Events Consumed

Placement Management consumes enterprise events published by upstream domains.

Examples include:

- StudentCreated
- StudentProfileUpdated
- AcademicResultPublished
- SemesterCompleted
- BacklogUpdated
- QualificationAwarded
- StudentStatusChanged
- UserActivated
- UserDeactivated
- NotificationDeliveryCompleted

These events update placement read models and trigger workflow evaluations where applicable.

---

# 15. Downstream Events Published

Placement Management publishes business events consumed by other enterprise domains.

Representative events include:

- EmployerRegistered
- EmployerVerified
- PlacementDriveCreated
- PlacementDrivePublished
- DriveRegistrationOpened
- PositionCreated
- EligibilityConfigured
- CandidateRegistered
- CandidateScreened
- CandidateShortlisted
- RecruitmentRoundCompleted
- PlacementOfferGenerated
- PlacementOfferAccepted
- PlacementOfferRejected
- PlacementOfferWithdrawn
- InternshipCompleted
- PlacementCompleted
- PlacementDriveClosed

These events enable notifications, dashboards, analytics, institutional reporting, and enterprise integrations while maintaining loose coupling across the SutraERP ecosystem.

## 15.1 Event Producer Consumer Matrix

| Event | Producer Aggregate | Internal Consumers | External Consumers |
|-------|--------------------|--------------------|--------------------|
| EmployerRegistered | Employer | PlacementDrive, CandidateApplication, Read Model projections | Section H02, Section H03, Section H07 |
| PlacementDriveCreated | PlacementDrive | Position, CandidateApplication, RecruitmentWorkflow | Section H02, Section H03, Section H07 |
| PlacementDrivePublished | PlacementDrive | Position, CandidateApplication, RecruitmentWorkflow | Section H02, Section H03, Section H07 |
| CandidateShortlisted | CandidateApplication | RecruitmentWorkflow, PlacementOffer, Read Model projections | Section H02, Section H07 |
| PlacementOfferGenerated | PlacementOffer | CandidateApplication, JoiningConfirmation | Section H02, Section H07 |
| PlacementOfferAccepted | PlacementOffer | CandidateApplication, JoiningConfirmation, PlacementDrive | Section H02, Section H07 |
| PlacementCompleted | JoiningConfirmation | Alumni analytics, PlacementDrive closure, institutional reporting | Section H03, Section H07 |
| InternshipCompleted | Internship | PlacementDrive, PlacementOffer, Placement analytics | Section H02, Section H07 |

# 16. Domain Aggregate Architecture

The Placement Management domain is composed of multiple aggregates that collectively implement the complete placement lifecycle while preserving transactional consistency and clear ownership boundaries.

Each aggregate encapsulates its own business rules, lifecycle, commands, events, and state transitions. Aggregates communicate exclusively through published domain events and enterprise commands.

The domain intentionally avoids large transactional boundaries. Every aggregate remains independently scalable and independently auditable.

---

# 17. Aggregate Specifications

## 17.1 Employer Aggregate

### Purpose

Represents an organization that recruits students through the institution.

The Employer aggregate is responsible for maintaining employer identity, organizational information, verification status, recruitment history, and active participation within institutional placement activities.

---

### Responsibilities

- Register employer organizations
- Maintain employer profile
- Store company information
- Manage verification status
- Maintain employer contacts
- Record employer engagement history
- Track employer participation
- Maintain employer lifecycle

---

### Aggregate Root

```
Employer
```

---

### Child Entities

- Employer Contact
- Company Address
- Communication Record
- Verification Record
- Industry Classification
- Employer Notes

---

### Value Objects

- Employer Code
- Company Name
- Company Type
- Industry
- Website
- Contact Details
- Verification Status

---

### Invariants

- Employer code must be unique.
- Verified employers cannot be deleted.
- Every employer must have at least one primary contact.
- Employer verification history is immutable.
- Closed employers cannot create new placement drives.

---

### Lifecycle

```
Registered
      │
      ▼
Verification Pending
      │
      ▼
Verified
      │
      ├────────► Suspended
      │
      ▼
Active
      │
      ▼
Inactive
```

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Registered | SubmitEmployerForVerification | Verification Pending |
| Verification Pending | ApproveEmployer | Verified |
| Verification Pending | RejectEmployer | Inactive |
| Verified | UpdateEmployerStatus | Suspended |
| Suspended | UpdateEmployerStatus | Active |
| Active | UpdateEmployerStatus | Inactive |

---

## 17.2 Placement Drive Aggregate

### Purpose

Represents a recruitment campaign organized for one employer.

A placement drive contains one or more employment or internship positions together with eligibility rules, recruitment workflow, participating candidates, and recruitment outcomes.

---

### Responsibilities

- Create drives
- Configure recruitment schedules
- Manage participating positions
- Configure eligibility
- Publish drives
- Open registrations
- Execute recruitment
- Close drives

---

### Aggregate Root

```
PlacementDrive
```

---

### Child Entities

- Drive Schedule
- Venue
- Recruitment Timeline
- Drive Documents
- Drive Coordinator
- Drive Announcement

---

### Value Objects

- Drive Code
- Drive Name
- Drive Type
- Campus Type
- Recruitment Mode
- Registration Window
- Drive Status

---

### Invariants

- Every drive belongs to exactly one employer.
- Every drive contains at least one position.
- Closed drives cannot be modified.
- Published drives cannot remove positions already accepting registrations.
- Registration dates cannot overlap incorrectly.

---

### Lifecycle

```
Draft
   │
   ▼
Published
   │
   ▼
Registration Open
   │
   ▼
Recruitment Active
   │
   ▼
Offer Processing
   │
   ▼
Completed
   │
   ▼
Closed
```

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Draft | SavePlacementDrive | Draft |
| Draft | PublishPlacementDrive | Published |
| Published | OpenDriveRegistration | Registration Open |
| Registration Open | CloseDriveRegistration | Recruitment Active |
| Recruitment Active | ReviewPlacementDriveCompletion | Completed |
| Completed | ArchivePlacementDrive | Closed |
| Completed | ClosePlacementDrive | Closed |

---

## 17.3 Position Aggregate

### Purpose

Represents a job or internship opportunity offered within a placement drive.

The Position aggregate owns compensation details, hiring targets, eligibility requirements, recruitment template assignment, and recruitment progress.

---

### Responsibilities

- Create positions
- Define hiring targets
- Configure compensation
- Configure eligibility
- Assign recruitment workflow
- Publish opportunities
- Track hiring progress

---

### Aggregate Root

```
Position
```

---

### Child Entities

- Compensation Structure
- Hiring Target
- Eligibility Rule
- Required Skills
- Required Certifications
- Recruitment Template Assignment

---

### Value Objects

- Position Code
- Job Title
- Employment Type
- Internship Type
- Package
- Work Location
- Hiring Capacity

---

### Invariants

- Every position belongs to one placement drive.
- Hiring capacity cannot be negative.
- Eligibility rules must exist before publication.
- Closed positions cannot accept registrations.
- Recruitment workflow must be assigned before registrations begin.

---

### Lifecycle

```
Draft
   │
   ▼
Configured
   │
   ▼
Published
   │
   ▼
Open
   │
   ▼
Recruitment Active
   │
   ▼
Filled
   │
   ▼
Closed
```

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Draft | SavePosition | Draft |
| Draft | ConfigureEligibility | Configured |
| Configured | PublishPosition | Published |
| Published | ClosePosition | Closed |

---

## 17.4 Candidate Application Aggregate

### Purpose

Represents a student's application to a specific position.

The aggregate manages application lifecycle, eligibility validation, screening outcomes, recruitment progression, interview results, and final placement outcome.
Canonical resume ownership remains in the Student domain; Placement stores only an immutable resume snapshot reference used during recruitment.

---

### Responsibilities

- Register applications
- Validate eligibility
- Store resume snapshot references
- Record screening decisions
- Track recruitment progress
- Record interview outcomes
- Manage placement status

---

### Aggregate Root

```
CandidateApplication
```

---

### Child Entities

- Resume Snapshot Reference
- Screening Record
- Assessment Result
- Interview Result
- Evaluation Notes
- Selection History

---

### Value Objects

- Application Number
- Candidate Status
- Registration Timestamp
- Selection Score
- Recruitment Round
- Ranking

---

### Invariants

- One application belongs to one student and one position.
- Applications cannot bypass configured recruitment stages.
- Eliminated candidates cannot progress without authorized reconsideration.
- Offer generation requires successful completion of all mandatory recruitment rounds.
- Every application maintains complete historical progression.

---

### Lifecycle

```
Registered
     │
     ▼
Eligibility Verified
     │
     ▼
Screened
     │
     ▼
Shortlisted
     │
     ▼
Recruitment In Progress
     │
     ├────────► Eliminated
     │
     ▼
Selected
     │
     ▼
Offer Issued
```

### Lifecycle (continued)

```
Offer Issued
      │
      ├────────► Offer Declined
      │
      ▼
Offer Accepted
      │
      ▼
Joining Confirmed
```

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Registered | CreateCandidateApplication | Registered |
| Registered | ValidateEligibility | Eligibility Verified |
| Eligibility Verified | InitiateCandidateScreening | Screened |
| Screened | ApproveShortlist | Shortlisted |
| Shortlisted | StartRecruitmentRound | Recruitment In Progress |
| Recruitment In Progress | EliminateCandidate | Eliminated |
| Recruitment In Progress | AdvanceCandidate | Selected |
| Selected | FinalizeCandidateSelection | Offer Issued |
| Offer Issued | AcceptPlacementOffer | Offer Accepted |
| Offer Issued | RejectPlacementOffer | Offer Declined |
| Offer Accepted | VerifyJoining | Joining Confirmed |

---

## 17.5 Recruitment Workflow Aggregate

### Purpose

Represents the configurable recruitment process assigned to a position.

This aggregate enables institutions to define reusable recruitment templates that employers can adopt without requiring application changes. Every workflow consists of an ordered sequence of evaluation stages through which candidates progress.

---

### Responsibilities

- Maintain recruitment templates
- Define recruitment stages
- Configure stage sequencing
- Configure mandatory and optional rounds
- Configure elimination rules
- Configure advancement criteria
- Track workflow execution

---

### Aggregate Root

```
RecruitmentWorkflow
```

---

### Child Entities

- Recruitment Stage
- Stage Configuration
- Evaluation Criteria
- Passing Rule
- Workflow Sequence
- Workflow Version

---

### Value Objects

- Workflow Name
- Workflow Type
- Stage Order
- Stage Duration
- Passing Threshold
- Evaluation Method

---

### Invariants

- Every workflow contains at least one recruitment stage.
- Stage order must remain unique.
- The first stage must always be defined.
- Circular workflow definitions are prohibited.
- Candidates cannot skip mandatory stages.
- Completed workflow versions become immutable.

---

### Typical Recruitment Workflow

```
Resume Screening
        │
        ▼
Aptitude Test
        │
        ▼
Coding Test (Optional)
        │
        ▼
Group Discussion
        │
        ▼
Technical Interview
        │
        ▼
HR Interview
        │
        ▼
Offer Generation
```

Institutions may configure alternative workflows depending on employer requirements.

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Draft | StartRecruitmentRound | In Progress |
| In Progress | RecordEvaluation | In Progress |
| In Progress | SubmitRoundResults | Completed |
| Completed | ResumeRecruitmentWorkflow | In Progress |

---

## 17.6 Placement Offer Aggregate

### Purpose

Represents an employment or internship offer issued by an employer to a selected candidate.

The aggregate manages offer creation, communication, acceptance, rejection, withdrawal, expiry, and final placement status.

---

### Responsibilities

- Generate offers
- Maintain offer details
- Track offer lifecycle
- Record candidate decisions
- Record employer withdrawals
- Manage offer validity
- Publish placement outcomes

---

### Aggregate Root

```
PlacementOffer
```

---

### Child Entities

- Offer Document
- Compensation Details
- Offer Conditions
- Acceptance Record
- Withdrawal Record

---

### Value Objects

- Offer Number
- Offer Type
- Compensation Package
- Joining Date
- Offer Expiry Date
- Offer Status

---

### Invariants

- Every offer belongs to one candidate application.
- Only selected candidates may receive offers.
- Accepted offers cannot return to pending status.
- Withdrawn offers remain permanently auditable.
- Expired offers cannot be accepted.

---

### Lifecycle

```
Draft
   │
   ▼
Generated
   │
   ▼
Published
   │
   ├────────► Withdrawn
   │
   ├────────► Expired
   │
   ▼
Accepted
   │
   ├────────► Declined
   │
   ▼
Joining Confirmed
```

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Draft | GeneratePlacementOffer | Generated |
| Generated | PublishOffer | Published |
| Published | AcceptPlacementOffer | Accepted |
| Published | RejectPlacementOffer | Declined |
| Published | WithdrawPlacementOffer | Withdrawn |
| Accepted | VerifyJoining | Joining Confirmed |

---

## 17.7 Internship Aggregate

### Purpose

Represents internship opportunities offered through institutional placement activities.

The Internship aggregate supports the complete internship lifecycle from opportunity publication through internship completion and potential conversion into full-time employment.

---

### Responsibilities

- Manage internship opportunities
- Track internship applications
- Record internship selection
- Monitor internship progress
- Record internship completion
- Manage internship conversion

---

### Aggregate Root

```
Internship
```

---

### Child Entities

- Internship Assignment
- Internship Duration
- Mentor Assignment
- Evaluation Record
- Completion Record

---

### Value Objects

- Internship Type
- Duration
- Stipend
- Department
- Work Mode
- Completion Status

---

### Invariants

- Internship belongs to one employer.
- Internship assignment belongs to one student.
- Completed internships cannot be modified.
- Conversion history remains immutable.
- Internship completion precedes conversion.

---

### Lifecycle

```
Created
    │
    ▼
Published
    │
    ▼
Applications Open
    │
    ▼
Selected
    │
    ▼
In Progress
    │
    ▼
Completed
    │
    ├────────► Converted to Placement
    │
    ▼
Closed
```

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Created | PublishInternshipPosition | Published |
| Published | RegisterInternshipCandidate | Applications Open |
| Applications Open | GenerateInternshipOffer | Selected |
| Selected | StartInternship | In Progress |
| In Progress | CompleteInternship | Completed |
| Completed | ConvertInternshipToPlacement | Closed |

---

## 17.8 Joining Confirmation Aggregate

### Purpose

Represents the final confirmation that a selected student has successfully joined the employer.

This aggregate closes the placement lifecycle by recording employer confirmation, student confirmation, and institutional verification.

---

### Responsibilities

- Record joining confirmations
- Track joining status
- Record joining evidence
- Publish placement completion
- Maintain historical joining records

---

### Aggregate Root

```
JoiningConfirmation
```

---

### Child Entities

- Employer Confirmation
- Student Confirmation
- Verification Record
- Supporting Documents

---

### Value Objects

- Joining Date
- Confirmation Status
- Verification Timestamp
- Verification Method

---

### Invariants

- Joining confirmation requires an accepted offer.
- Confirmation records become immutable after verification.
- Every confirmation references exactly one placement offer.
- Withdrawn offers cannot generate joining confirmations.

---

### Lifecycle

```
Pending
    │
    ▼
Student Confirmed
    │
    ▼
Employer Confirmed
    │
    ▼
Institution Verified
    │
    ▼
Completed
```

### Transition Table

| Current State | Command | Next State |
|---------------|---------|------------|
| Pending | RecordStudentJoining | Student Confirmed |
| Student Confirmed | RecordEmployerConfirmation | Employer Confirmed |
| Employer Confirmed | VerifyJoining | Institution Verified |
| Institution Verified | CompletePlacement | Completed |

---

# 18. Aggregate Relationships

The Placement Management aggregates collaborate while preserving aggregate independence.

```
Employer
    │
    ▼
Placement Drive
    │
    ▼
Position
    │
    ├──────────────► Recruitment Workflow
    │
    ▼
Candidate Application
    │
    ▼
Placement Offer
    │
    ▼
Joining Confirmation

Position
    │
    ▼
Internship
```

Each aggregate owns its internal consistency boundary. Communication between aggregates occurs through commands and published domain events rather than direct state manipulation.

# 19. Workflow Specifications

This section defines the enterprise workflows supported by the Placement Management domain.

Each workflow follows the CQRS and Event-Driven Architecture adopted across SutraERP.

Every workflow specifies:

- Objective
- Preconditions
- Primary Flow
- Commands
- Domain Events
- Read Model Updates
- Business Rules
- Failure Scenarios
- Audit Requirements

---

# 19.1 Employer Registration Workflow

## Objective

Register a new employer organization for institutional placement activities.

---

## Preconditions

- Placement Officer is authenticated.
- Employer does not already exist.
- Mandatory organization information is available.

---

## Primary Workflow

1. Placement Officer initiates employer registration.
2. Employer information is captured.
3. Contact person information is recorded.
4. Organization details are validated.
5. Employer record is created.
6. Verification request is initiated.
7. Employer enters Verification Pending state.

---

## Commands

- RegisterEmployer
- ValidateEmployerInformation
- CreateEmployer
- SubmitEmployerForVerification

---

## Domain Events

- EmployerRegistrationInitiated
- EmployerRegistered
- EmployerVerificationRequested

---

## Read Models Updated

- Employer Directory
- Employer Verification Queue
- Employer Dashboard

---

## Business Rules

- Employer code must be unique.
- Mandatory contact information is required.
- Duplicate organizations are prohibited.
- Verification is mandatory before recruitment activities.

---

## Failure Scenarios

- Duplicate employer detected.
- Mandatory information missing.
- Invalid organization information.
- Verification request creation failed.

---

## Audit Requirements

Audit log shall capture:

- Registration initiator
- Registration timestamp
- Employer information
- Verification request identifier

---

# 19.2 Employer Verification Workflow

## Objective

Verify employer authenticity before participation in placement activities.

---

## Preconditions

- Employer exists.
- Employer status is Verification Pending.

---

## Primary Workflow

1. Placement Officer reviews employer details.
2. Supporting documents are verified.
3. Employer authenticity is confirmed.
4. Employer status is updated.
5. Employer becomes eligible for placement drives.

---

## Commands

- ReviewEmployerVerification
- ApproveEmployer
- RejectEmployer
- UpdateEmployerStatus

---

## Domain Events

- EmployerVerified
- EmployerVerificationRejected
- EmployerStatusUpdated

---

## Read Models Updated

- Employer Directory
- Verified Employer List
- Employer Dashboard

---

## Business Rules

- Only authorized officers may verify employers.
- Rejected employers cannot create drives.
- Verification decisions are immutable.

---

## Failure Scenarios

- Verification evidence incomplete.
- Employer documents invalid.
- Verification authorization failed.

---

## Audit Requirements

Audit trail records:

- Reviewing officer
- Verification outcome
- Decision timestamp
- Supporting documents

---

# 19.3 Placement Drive Creation Workflow

## Objective

Create a recruitment drive for one verified employer.

---

## Preconditions

- Employer is verified.
- Placement Officer is authorized.
- Recruitment schedule is available.

---

## Primary Workflow

1. Placement Officer creates placement drive.
2. Employer association is recorded by authorized placement staff.
3. Drive information is entered.
4. Schedule is configured.
5. Venue or online mode is selected.
6. Initial drive record is created.
7. Drive enters Draft state.

---

## Commands

- CreatePlacementDrive
- ConfigureDriveSchedule
- AssignEmployerToDrive
- SavePlacementDrive

---

## Domain Events

- PlacementDriveCreated
- DriveScheduleConfigured
- EmployerAssignedToDrive

---

## Read Models Updated

- Placement Drive Dashboard
- Employer Drive History
- Upcoming Placement Drives

---

## Business Rules

- Every drive belongs to one employer.
- Draft drives are editable.
- Published drives require at least one position.

---

## Failure Scenarios

- Employer inactive.
- Schedule conflict.
- Missing mandatory information.

---

## Audit Requirements

Record:

- Drive creator
- Employer
- Schedule
- Creation timestamp

---

# 19.4 Position Creation Workflow

## Objective

Create employment or internship positions within a placement drive.

---

## Preconditions

- Placement drive exists.
- Drive status allows modification.

---

## Primary Workflow

1. Placement Officer creates position.
2. Position details are entered.
3. Compensation is configured.
4. Hiring capacity is defined.
5. Employment type is selected.
6. Position is saved.

---

## Commands

- CreatePosition
- ConfigureCompensation
- ConfigureHiringCapacity
- SavePosition

---

## Domain Events

- PositionCreated
- CompensationConfigured
- HiringCapacityDefined

---

## Read Models Updated

- Position Catalogue
- Drive Positions
- Employer Position Summary

---

## Business Rules

- Position belongs to one drive.
- Hiring capacity must be positive.
- Position code is unique within a drive.

---

## Failure Scenarios

- Invalid compensation.
- Invalid hiring capacity.
- Duplicate position.

---

## Audit Requirements

Capture:

- Position creator
- Position details
- Compensation
- Timestamp

---

# 19.5 Eligibility Configuration Workflow

## Objective

Define academic and institutional eligibility criteria for a position.

---

## Preconditions

- Position exists.
- Recruitment has not started.

---

## Primary Workflow

1. Placement Officer selects position.
2. Eligibility criteria are configured.
3. Academic rules are added.
4. Employer-specific rules are added.
5. Institutional placement policies are linked.
6. Eligibility configuration is published.

---

## Commands

- ConfigureEligibility
- AddAcademicCriteria
- AddEmployerCriteria
- PublishEligibilityRules

---

## Domain Events

- EligibilityConfigured
- AcademicCriteriaAdded
- EmployerCriteriaAdded
- EligibilityPublished

---

## Read Models Updated

- Eligibility Rules
- Student Eligibility View
- Position Summary

---

## Business Rules

Eligibility may include:

- CGPA
- Percentage
- Active Backlogs
- Branch
- Program
- Qualification Completion Year
- Skills
- Certifications
- Resume Completeness
- Institution Placement Policies
- Employer Custom Rules

Institution-specific placement policy validation shall be executed before student registration opens.

---

## Failure Scenarios

- Invalid eligibility rule.
- Conflicting criteria.
- Missing mandatory criteria.

---

## Audit Requirements

Audit records include:

- Configured rules
- Policy version
- Configuring officer
- Configuration timestamp

---

# 19.6 Student Registration Workflow

## Objective

Allow eligible students to register for a placement position.

---

## Preconditions

- Registration window is open.
- Position is published.
- Student is active.
- Eligibility rules are available.

---

## Primary Workflow

1. Student selects a position.
2. Resume is submitted or selected.
3. Eligibility validation is executed.
4. Placement policy validation is executed.
5. Registration is accepted.
6. Candidate application is created.

---

## Commands

- RegisterCandidate
- ValidateEligibility
- ValidatePlacementPolicy
- CreateCandidateApplication

---

## Domain Events

- CandidateRegistrationInitiated
- CandidateEligibilityValidated
- CandidateRegistered
- CandidateApplicationCreated

---

## Read Models Updated

- Candidate Applications
- Position Applicants
- Student Placement Dashboard

---

## Business Rules

- Registration is permitted only during the configured registration period.
- Students must satisfy all mandatory eligibility criteria.
- Institutional placement policies must be enforced before registration.
- One candidate application represents one student applying for one position.

---

## Failure Scenarios

- Student not eligible.
- Registration closed.
- Placement policy violation.
- Resume incomplete.

---

## Audit Requirements

Capture:

- Student
- Position
- Registration timestamp
- Eligibility evaluation result
- Policy validation outcome

# 19.7 Candidate Screening Workflow

## Objective

Evaluate registered candidates against employer requirements and institutional placement policies before recruitment begins.

---

## Preconditions

- Registration window has closed or screening has been initiated.
- Candidate applications exist.
- Eligibility rules are published.
- Recruitment workflow is assigned to the position.

---

## Primary Workflow

1. Screening process is initiated.
2. Academic eligibility is verified.
3. Institutional placement policy is evaluated.
4. AI resume screening is executed (if enabled).
5. Position requirement matching is performed.
6. Manual screening is conducted where required.
7. Candidate screening score is finalized.
8. Candidate status is updated.

---

## Commands

- InitiateCandidateScreening
- ValidateAcademicEligibility
- ValidatePlacementPolicy
- ExecuteAIResumeScreening
- MatchPositionRequirements
- PerformManualScreening
- FinalizeScreening

---

## Domain Events

- CandidateScreeningStarted
- AcademicEligibilityValidated
- PlacementPolicyValidated
- AIResumeScreeningCompleted
- PositionRequirementsMatched
- ManualScreeningCompleted
- CandidateScreened

---

## Read Models Updated

- Screening Dashboard
- Candidate Ranking View
- Eligible Candidate List
- Position Screening Summary

---

## Business Rules

- Every registered candidate must undergo screening.
- AI screening supplements but does not replace institutional decision-making.
- Manual screening may override AI recommendations.
- Screening criteria must be traceable.
- Screening decisions are auditable.

---

## Failure Scenarios

- Academic data unavailable.
- Resume missing.
- AI screening unavailable.
- Required screening criteria missing.

---

## Audit Requirements

Capture:

- Screening evaluator
- Screening criteria
- AI recommendation
- Manual decision
- Screening score
- Timestamp

---

# 19.8 Candidate Shortlisting Workflow

## Objective

Publish shortlisted candidates eligible to participate in recruitment rounds.

---

## Preconditions

- Screening has been completed.
- Candidate ranking is finalized.

---

## Primary Workflow

1. Shortlisting process begins.
2. Candidates are ranked.
3. Cut-off criteria are applied.
4. Placement Officer reviews shortlisted candidates.
5. Shortlist is approved.
6. Shortlist is published.

---

## Commands

- GenerateCandidateRanking
- ApplyShortlistingCriteria
- ApproveShortlist
- PublishShortlist

---

## Domain Events

- CandidateRankingGenerated
- CandidateShortlisted
- ShortlistApproved
- ShortlistPublished

---

## Read Models Updated

- Shortlisted Candidates
- Recruitment Dashboard
- Student Placement Dashboard

---

## Business Rules

- Only screened candidates may be shortlisted.
- Published shortlists become immutable.
- Candidate reconsideration requires explicit authorization.

---

## Failure Scenarios

- No eligible candidates.
- Ranking inconsistency.
- Approval rejected.

---

## Audit Requirements

Capture:

- Approval authority
- Shortlisted candidates
- Ranking
- Publication timestamp

---

# 19.9 Pre-Placement Talk (PPT) Workflow

## Objective

Coordinate employer presentations before recruitment activities.

---

## Preconditions

- Placement drive is active.
- PPT has been scheduled.

---

## Primary Workflow

1. PPT schedule is created.
2. Venue or virtual meeting is configured.
3. Students are notified.
4. Employer conducts presentation.
5. Attendance is recorded.
6. PPT is marked complete.

---

## Commands

- SchedulePrePlacementTalk
- NotifyParticipants
- RecordAttendance
- CompletePrePlacementTalk

---

## Domain Events

- PrePlacementTalkScheduled
- ParticipantsNotified
- AttendanceRecorded
- PrePlacementTalkCompleted

---

## Read Models Updated

- PPT Schedule
- Student Calendar
- Employer Activity Dashboard

---

## Business Rules

- PPT is optional unless configured as mandatory.
- Attendance may be mandatory for specific employers.
- Completion must be recorded before recruitment proceeds if required.

---

## Failure Scenarios

- Venue unavailable.
- Meeting cancelled.
- Employer absent.

---

## Audit Requirements

Capture:

- Schedule
- Attendance
- Completion status
- Conducting officer

---

# 19.10 Recruitment Round Workflow

## Objective

Execute configurable recruitment stages for shortlisted candidates.

---

## Preconditions

- Recruitment workflow exists.
- Candidate is shortlisted.
- Previous mandatory stage is completed.

---

## Primary Workflow

1. Recruitment round begins.
2. Candidates participate.
3. Evaluations are recorded.
4. Results are submitted.
5. Advancement criteria are evaluated.
6. Candidate status is updated.
7. Next stage becomes available or candidate is eliminated.

---

## Commands

- StartRecruitmentRound
- RecordEvaluation
- SubmitRoundResults
- AdvanceCandidate
- EliminateCandidate

---

## Domain Events

- RecruitmentRoundStarted
- CandidateEvaluated
- RoundResultsSubmitted
- CandidateAdvanced
- CandidateEliminated
- RecruitmentRoundCompleted

---

## Read Models Updated

- Recruitment Progress Dashboard
- Candidate Progress View
- Recruitment Round Results
- Employer Recruitment Dashboard

---

## Business Rules

- Recruitment stages execute sequentially.
- Mandatory stages cannot be skipped.
- Employer-defined workflow governs progression.
- Eliminated candidates exit the current recruitment process.
- Reconsideration requires authorized approval.

---

## Failure Scenarios

- Evaluation incomplete.
- Stage configuration missing.
- Invalid advancement decision.

---

## Audit Requirements

Capture:

- Stage
- Evaluator
- Evaluation scores
- Advancement decision
- Timestamp

---

# 19.11 Candidate Reconsideration Workflow

## Objective

Allow authorized reinstatement of candidates previously eliminated from a recruitment process.

---

## Preconditions

- Candidate status is Eliminated.
- Employer requests reconsideration or Placement Officer initiates reconsideration.
- Reconsideration authority is available.

---

## Primary Workflow

1. Reconsideration request is submitted.
2. Authorization is verified.
3. Previous evaluation is reviewed.
4. Candidate is reinstated.
5. Recruitment workflow resumes from the appropriate stage.
6. Candidate status is updated.

---

## Commands

- RequestCandidateReconsideration
- ReviewCandidateElimination
- ReinstateCandidate
- ResumeRecruitmentWorkflow

---

## Domain Events

- CandidateReconsiderationRequested
- CandidateReinstated
- RecruitmentWorkflowResumed

---

## Read Models Updated

- Candidate Status Dashboard
- Recruitment Dashboard
- Audit Dashboard

---

## Business Rules

- Only authorized personnel may approve reconsideration.
- Reinstatement preserves complete historical records.
- Previous elimination records remain immutable.

---

## Failure Scenarios

- Unauthorized reconsideration.
- Recruitment already closed.
- Candidate already finalized.

---

## Audit Requirements

Capture:

- Requesting authority
- Approval authority
- Reason for reconsideration
- Previous status
- New status
- Timestamp

# 19.12 Offer Generation Workflow

## Objective

Generate employment or internship offers for candidates successfully completing the configured recruitment workflow.

---

## Preconditions

- Candidate has successfully completed all mandatory recruitment stages.
- Employer has finalized selection decisions.
- Position hiring capacity is available.

---

## Primary Workflow

1. Employer finalizes selected candidates.
2. Placement Officer reviews selection input received from the employer outside the system.
3. Offer details are generated.
4. Compensation package is verified.
5. Offer validity period is configured.
6. Offer is published to the candidate.
7. Candidate application status is updated.

---

## Commands

- FinalizeCandidateSelection
- GeneratePlacementOffer
- ValidateOfferDetails
- PublishOffer
- UpdateCandidateOfferStatus

---

## Domain Events

- CandidateSelected
- PlacementOfferGenerated
- PlacementOfferPublished
- CandidateOfferStatusUpdated

---

## Read Models Updated

- Placement Offer Dashboard
- Candidate Offer View
- Employer Offer Summary
- Placement Statistics

---

## Business Rules

- Offers may only be generated for selected candidates.
- Offer validity period is mandatory.
- Hiring capacity shall not be exceeded.
- Offer identifiers shall be unique.

---

## Failure Scenarios

- Position capacity exceeded.
- Compensation package incomplete.
- Candidate no longer eligible.
- Offer generation failure.

---

## Audit Requirements

Capture:

- Employer
- Position
- Candidate
- Offer details
- Compensation package
- Offer generation timestamp

---

# 19.13 Offer Acceptance Workflow

## Objective

Record the student's response to a placement offer.

---

## Preconditions

- Offer has been published.
- Offer has not expired.
- Offer has not been withdrawn.

---

## Primary Workflow

1. Student reviews offer.
2. Student accepts or declines the offer.
3. Institutional placement policy is evaluated.
4. Offer status is updated.
5. Candidate application status is updated.
6. Employer is notified.

---

## Commands

- AcceptPlacementOffer
- RejectPlacementOffer
- ValidatePlacementPolicyAfterAcceptance
- UpdateOfferStatus
- NotifyEmployer

---

## Domain Events

- PlacementOfferAccepted
- PlacementOfferRejected
- PlacementPolicyEvaluated
- EmployerNotified

---

## Read Models Updated

- Offer Dashboard
- Student Placement Dashboard
- Employer Recruitment Dashboard
- Placement Analytics Dashboard

---

## Business Rules

- Offers cannot be accepted after expiry.
- Institution placement policy shall be enforced after acceptance.
- Accepted offers become immutable.
- Rejected offers remain permanently auditable.

---

## Failure Scenarios

- Offer expired.
- Offer withdrawn.
- Placement policy violation.
- Invalid offer status.

---

## Audit Requirements

Capture:

- Student decision
- Decision timestamp
- Policy evaluation
- Offer status

---

# 19.14 Offer Withdrawal Workflow

## Objective

Process employer withdrawal of previously issued placement offers.

---

## Preconditions

- Offer exists.
- Offer has not reached joining confirmation.

---

## Primary Workflow

1. Employer requests withdrawal.
2. Placement Officer validates the withdrawal request recorded outside the system.
3. Offer status is updated.
4. Student is notified.
5. Student becomes eligible for future placement drives according to institutional policy.
6. Placement records are updated.

---

## Commands

- RequestOfferWithdrawal
- ApproveOfferWithdrawal
- WithdrawPlacementOffer
- ReopenCandidateEligibility
- NotifyCandidate

---

## Domain Events

- OfferWithdrawalRequested
- PlacementOfferWithdrawn
- CandidateEligibilityReopened
- CandidateNotified

---

## Read Models Updated

- Offer Dashboard
- Candidate Status Dashboard
- Employer Dashboard
- Placement Analytics Dashboard

---

## Business Rules

- Withdrawn offers remain permanently auditable.
- Candidate eligibility reopening follows institutional placement policy.
- Employer withdrawal reason shall be recorded.

---

## Failure Scenarios

- Joining already confirmed.
- Unauthorized withdrawal.
- Invalid offer status.

---

## Audit Requirements

Capture:

- Employer
- Withdrawal reason
- Approval authority
- Candidate notification
- Timestamp

---

# 19.15 Joining Confirmation Workflow

## Objective

Record successful joining of a selected student.

---

## Preconditions

- Offer has been accepted.
- Student has joined the employer.

---

## Primary Workflow

1. Student confirms joining.
2. Employer confirmation is recorded by authorized placement staff from an external employer submission.
3. Placement Officer verifies evidence.
4. Joining record is finalized.
5. Placement completion is published.

---

## Commands

- RecordStudentJoining
- RecordEmployerConfirmation
- VerifyJoining
- CompletePlacement

---

## Domain Events

- StudentJoiningRecorded
- EmployerJoiningConfirmed
- JoiningVerified
- PlacementCompleted

---

## Read Models Updated

- Placement Completion Dashboard
- Alumni Placement Dashboard
- Employer Hiring Dashboard
- Institutional Placement Statistics

---

## Business Rules

- Joining confirmation requires an accepted offer.
- Employer confirmation is mandatory.
- Verification permanently closes the placement lifecycle.

---

## Failure Scenarios

- Employer confirmation missing.
- Joining evidence invalid.
- Verification rejected.

---

## Audit Requirements

Capture:

- Student
- Employer
- Joining date
- Verification officer
- Supporting evidence

---

# 19.16 Placement Drive Closure Workflow

## Objective

Complete and archive a placement drive after all recruitment activities have concluded.

---

## Preconditions

- All recruitment activities completed.
- Outstanding offers resolved.
- Pending recruitment rounds completed.

---

## Primary Workflow

1. Drive completion review begins.
2. Recruitment statistics are generated.
3. Hiring summary is finalized.
4. Drive is archived.
5. Drive status becomes Closed.

---

## Commands

- ReviewPlacementDriveCompletion
- GeneratePlacementStatistics
- ArchivePlacementDrive
- ClosePlacementDrive

---

## Domain Events

- PlacementStatisticsGenerated
- PlacementDriveArchived
- PlacementDriveClosed

---

## Read Models Updated

- Historical Placement Drives
- Employer Recruitment History
- Institutional Analytics
- Executive Dashboard

---

## Business Rules

- Closed drives become immutable.
- Historical records cannot be deleted.
- Archived drives remain searchable.

---

## Failure Scenarios

- Pending offers exist.
- Recruitment still active.
- Archive operation failed.

---

## Audit Requirements

Capture:

- Closing authority
- Drive summary
- Hiring statistics
- Closure timestamp

---

# 19.17 Internship Management Workflow

## Objective

Manage internship recruitment, execution, completion, and conversion into permanent employment where applicable.

---

## Preconditions

- Internship position exists.
- Internship recruitment is active.

---

## Primary Workflow

1. Internship position is published.
2. Students register.
3. Screening and recruitment are completed.
4. Internship offer is issued.
5. Internship begins.
6. Internship progress is monitored.
7. Internship completes.
8. Internship may be converted into full-time employment.

---

## Commands

- PublishInternshipPosition
- RegisterInternshipCandidate
- GenerateInternshipOffer
- StartInternship
- CompleteInternship
- ConvertInternshipToPlacement

---

## Domain Events

- InternshipPublished
- InternshipCandidateRegistered
- InternshipOfferGenerated
- InternshipStarted
- InternshipCompleted
- InternshipConvertedToPlacement

---

## Read Models Updated

- Internship Dashboard
- Internship Progress View
- Internship Analytics
- Placement Conversion Dashboard

---

## Business Rules

- Internship workflows follow configured recruitment templates.
- Internship conversion is optional.
- Conversion history remains permanently auditable.

---

## Failure Scenarios

- Internship cancelled.
- Candidate withdraws.
- Employer cancels internship.
- Conversion denied.

---

## Audit Requirements

Capture:

- Internship lifecycle
- Employer
- Student
- Conversion decision
- Completion details

# 20. Commands Catalogue

The Placement Management domain exposes commands that initiate business operations on Placement aggregates.

Commands are imperative, validated before execution, and processed by aggregate roots. Successful command execution results in one or more immutable domain events.

---

# 20.1 Employer Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| RegisterEmployer | Employer | Register a new employer organization |
| CreateEmployer | Employer | Create an employer record |
| ValidateEmployerInformation | Employer | Validate employer details |
| SubmitEmployerForVerification | Employer | Send employer for verification |
| ReviewEmployerVerification | Employer | Review employer verification request |
| ApproveEmployer | Employer | Approve employer verification |
| RejectEmployer | Employer | Reject employer verification |
| UpdateEmployerStatus | Employer | Change employer lifecycle status |

---

# 20.2 Placement Drive Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| CreatePlacementDrive | PlacementDrive | Create a new placement drive |
| SavePlacementDrive | PlacementDrive | Persist placement drive details |
| ConfigureDriveSchedule | PlacementDrive | Configure recruitment schedule |
| AssignEmployerToDrive | PlacementDrive | Associate employer with drive |
| PublishPlacementDrive | PlacementDrive | Publish placement drive |
| OpenDriveRegistration | PlacementDrive | Open candidate registration |
| CloseDriveRegistration | PlacementDrive | Close candidate registration |
| SchedulePrePlacementTalk | PlacementDrive | Schedule pre-placement talk |
| NotifyParticipants | PlacementDrive | Notify participants for a placement activity |
| RecordAttendance | PlacementDrive | Record participant attendance |
| CompletePrePlacementTalk | PlacementDrive | Complete pre-placement talk |
| ReviewPlacementDriveCompletion | PlacementDrive | Review drive completion |
| GeneratePlacementStatistics | PlacementDrive | Generate placement statistics |
| ArchivePlacementDrive | PlacementDrive | Archive completed drive |
| ClosePlacementDrive | PlacementDrive | Close drive lifecycle |

---

# 20.3 Position Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| CreatePosition | Position | Create employment position |
| SavePosition | Position | Persist position details |
| ConfigureCompensation | Position | Configure salary package |
| ConfigureHiringCapacity | Position | Define hiring capacity |
| ConfigureEligibility | Position | Configure eligibility rules |
| AddAcademicCriteria | Position | Add academic eligibility criteria |
| AddEmployerCriteria | Position | Add employer-specific criteria |
| PublishEligibilityRules | Position | Publish eligibility rules |
| PublishPosition | Position | Publish position |
| ClosePosition | Position | Close position |

---

# 20.4 Candidate Application Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| RegisterCandidate | CandidateApplication | Register candidate |
| ValidateEligibility | CandidateApplication | Validate academic eligibility |
| ValidateAcademicEligibility | CandidateApplication | Validate academic eligibility against student records |
| ValidatePlacementPolicy | CandidateApplication | Validate institutional placement policy |
| ValidatePlacementPolicyAfterAcceptance | CandidateApplication | Validate placement policy after offer acceptance |
| CreateCandidateApplication | CandidateApplication | Create application |
| InitiateCandidateScreening | CandidateApplication | Begin screening |
| ExecuteAIResumeScreening | CandidateApplication | Perform AI resume evaluation |
| MatchPositionRequirements | CandidateApplication | Evaluate employer requirements |
| PerformManualScreening | CandidateApplication | Manual screening |
| FinalizeScreening | CandidateApplication | Finalize screening |
| GenerateCandidateRanking | CandidateApplication | Rank candidates |
| ApplyShortlistingCriteria | CandidateApplication | Apply shortlist criteria |
| ApproveShortlist | CandidateApplication | Approve shortlist |
| PublishShortlist | CandidateApplication | Publish shortlisted candidates |
| RequestCandidateReconsideration | CandidateApplication | Request candidate reinstatement |
| ReviewCandidateElimination | CandidateApplication | Review candidate elimination |
| ReinstateCandidate | CandidateApplication | Reinstate eliminated candidate |
| ReopenCandidateEligibility | CandidateApplication | Reopen candidate eligibility after withdrawal |
| UpdateCandidateOfferStatus | CandidateApplication | Update candidate offer status |

---

# 20.5 Recruitment Workflow Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| StartRecruitmentRound | RecruitmentWorkflow | Begin recruitment stage |
| RecordEvaluation | RecruitmentWorkflow | Record candidate evaluation |
| SubmitRoundResults | RecruitmentWorkflow | Submit evaluation outcome |
| AdvanceCandidate | RecruitmentWorkflow | Advance candidate |
| EliminateCandidate | RecruitmentWorkflow | Eliminate candidate |
| ResumeRecruitmentWorkflow | RecruitmentWorkflow | Resume workflow after reinstatement |

---

# 20.6 Placement Offer Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| FinalizeCandidateSelection | PlacementOffer | Finalize selected candidates |
| GeneratePlacementOffer | PlacementOffer | Generate offer |
| ValidateOfferDetails | PlacementOffer | Validate offer details |
| PublishOffer | PlacementOffer | Publish offer |
| UpdateOfferStatus | PlacementOffer | Update lifecycle status |
| AcceptPlacementOffer | PlacementOffer | Accept offer |
| RejectPlacementOffer | PlacementOffer | Reject offer |
| RequestOfferWithdrawal | PlacementOffer | Request withdrawal |
| ApproveOfferWithdrawal | PlacementOffer | Approve offer withdrawal |
| WithdrawPlacementOffer | PlacementOffer | Withdraw offer |
| NotifyEmployer | PlacementOffer | Notify employer of offer outcome |
| NotifyCandidate | PlacementOffer | Notify candidate of offer outcome |

---

# 20.7 Internship Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| PublishInternshipPosition | Internship | Publish internship |
| RegisterInternshipCandidate | Internship | Register internship applicant |
| GenerateInternshipOffer | Internship | Generate internship offer |
| StartInternship | Internship | Begin internship |
| CompleteInternship | Internship | Complete internship |
| ConvertInternshipToPlacement | Internship | Convert internship to full-time placement |

---

# 20.8 Joining Commands

| Command | Aggregate | Description |
|----------|-----------|-------------|
| RecordStudentJoining | JoiningConfirmation | Record student joining |
| RecordEmployerConfirmation | JoiningConfirmation | Record employer confirmation |
| VerifyJoining | JoiningConfirmation | Verify joining |
| CompletePlacement | JoiningConfirmation | Complete placement lifecycle |

---

# 21. Domain Events Catalogue

Events represent immutable business facts that occurred within the Placement domain.

---

# 21.1 Employer Events

| Event |
|--------|
| EmployerRegistrationInitiated |
| EmployerRegistered |
| EmployerVerificationRequested |
| EmployerVerified |
| EmployerVerificationRejected |
| EmployerStatusUpdated |

---

# 21.2 Placement Drive Events

| Event |
|--------|
| PlacementDriveCreated |
| DriveScheduleConfigured |
| EmployerAssignedToDrive |
| PlacementDrivePublished |
| DriveRegistrationOpened |
| DriveRegistrationClosed |
| PrePlacementTalkScheduled |
| ParticipantsNotified |
| AttendanceRecorded |
| PrePlacementTalkCompleted |
| PlacementStatisticsGenerated |
| PlacementDriveArchived |
| PlacementDriveClosed |

---

# 21.3 Position Events

| Event |
|--------|
| PositionCreated |
| CompensationConfigured |
| HiringCapacityDefined |
| EligibilityConfigured |
| AcademicCriteriaAdded |
| EmployerCriteriaAdded |
| EligibilityPublished |
| PositionPublished |
| PositionClosed |

---

# 21.4 Candidate Application Events

| Event |
|--------|
| CandidateRegistrationInitiated |
| CandidateEligibilityValidated |
| CandidateRegistered |
| CandidateApplicationCreated |
| AcademicEligibilityValidated |
| PlacementPolicyValidated |
| CandidateScreeningStarted |
| AIResumeScreeningCompleted |
| PositionRequirementsMatched |
| ManualScreeningCompleted |
| CandidateScreened |
| CandidateRankingGenerated |
| CandidateShortlisted |
| ShortlistApproved |
| ShortlistPublished |
| CandidateReconsiderationRequested |
| CandidateEligibilityReopened |
| CandidateReinstated |

---

# 21.5 Recruitment Events

| Event |
|--------|
| RecruitmentRoundStarted |
| CandidateEvaluated |
| RoundResultsSubmitted |
| CandidateAdvanced |
| CandidateEliminated |
| RecruitmentRoundCompleted |
| RecruitmentWorkflowResumed |

---

# 21.6 Placement Offer Events

| Event |
|--------|
| CandidateSelected |
| PlacementOfferGenerated |
| PlacementOfferPublished |
| CandidateOfferStatusUpdated |
| PlacementOfferAccepted |
| PlacementOfferRejected |
| PlacementPolicyEvaluated |
| EmployerNotified |
| OfferWithdrawalRequested |
| PlacementOfferWithdrawn |
| CandidateEligibilityReopened |
| CandidateNotified |

---

# 21.7 Internship Events

| Event |
|--------|
| InternshipPublished |
| InternshipCandidateRegistered |
| InternshipOfferGenerated |
| InternshipStarted |
| InternshipCompleted |
| InternshipConvertedToPlacement |

---

# 21.8 Joining Events

| Event |
|--------|
| StudentJoiningRecorded |
| EmployerJoiningConfirmed |
| JoiningVerified |
| PlacementCompleted |

---

# 22. Read Models

The Placement domain maintains optimized read models to support query performance, dashboards, search, reporting, and operational visibility.

| Read Model | Purpose |
|------------|---------|
| Employer Directory | Registered employers |
| Employer Verification Queue | Employers awaiting verification |
| Verified Employer List | Approved employers |
| Employer Dashboard | Employer operational summary |
| Employer Activity Dashboard | Employer-facing operational summary |
| Placement Drive Dashboard | Active placement drives |
| Upcoming Placement Drives | Future recruitment schedule |
| Employer Drive History | Historical drive participation |
| Position Catalogue | Published positions |
| Position Summary | Position operational summary |
| Drive Positions | Positions within a specific drive |
| Employer Position Summary | Position summary by employer |
| Position Screening Summary | Screening summary by position |
| Eligibility Rules | Published eligibility rules |
| Student Eligibility View | Eligibility outcome for a student |
| Position Applicants | Applications by position |
| Student Placement Dashboard | Student placement progress |
| Candidate Applications | Candidate application summary |
| Candidate Status Dashboard | Candidate lifecycle status |
| Screening Dashboard | Screening results |
| Candidate Ranking View | Ranked candidates |
| Eligible Candidate List | Screening-qualified candidates |
| Shortlisted Candidates | Published shortlist |
| Recruitment Dashboard | Recruitment operations summary |
| Recruitment Progress Dashboard | Recruitment stage tracking |
| Candidate Progress View | Individual candidate progression |
| Recruitment Round Results | Evaluation outcomes |
| Student Calendar | Student placement schedule |
| PPT Schedule | Pre-placement talk schedule |
| Placement Offer Dashboard | Offer management |
| Candidate Offer View | Candidate-facing offer details |
| Employer Offer Summary | Offer summary by employer |
| Offer Dashboard | Offer lifecycle summary |
| Employer Recruitment Dashboard | Employer recruitment summary |
| Internship Dashboard | Internship operations |
| Internship Progress View | Internship lifecycle |
| Internship Analytics | Internship analytical summary |
| Placement Completion Dashboard | Completed placements |
| Alumni Placement Dashboard | Alumni placement records |
| Employer Hiring Dashboard | Employer hiring summary |
| Institutional Placement Statistics | Executive reporting |
| Placement Analytics Dashboard | Analytical reporting |
| Placement Statistics | Placement aggregate statistics |
| Historical Placement Drives | Archived recruitment history |
| Employer Recruitment History | Employer-specific drive history |
| Institutional Analytics | Institution-level analytical summary |
| Executive Dashboard | Executive placement overview |
| Audit Dashboard | Placement audit and compliance |

## 22.1 Projection Catalogue

| Projection | Source Events | Refresh Strategy | Projection Owner |
|------------|---------------|------------------|------------------|
| EmployerDirectoryProjection | EmployerRegistered, EmployerVerified, EmployerStatusUpdated | Event-driven incremental update | Placement Read Model Team |
| PlacementDriveProjection | PlacementDriveCreated, PlacementDrivePublished, DriveRegistrationOpened, DriveRegistrationClosed, PlacementDriveClosed | Event-driven incremental update | Placement Read Model Team |
| CandidateStatusProjection | CandidateRegistered, CandidateScreened, CandidateShortlisted, CandidateReinstated, CandidateOfferStatusUpdated, PlacementCompleted | Event-driven incremental update | Placement Read Model Team |
| PlacementOfferProjection | CandidateSelected, PlacementOfferGenerated, PlacementOfferPublished, PlacementOfferAccepted, PlacementOfferRejected, PlacementOfferWithdrawn | Event-driven incremental update | Placement Read Model Team |
| InternshipProjection | InternshipPublished, InternshipStarted, InternshipCompleted, InternshipConvertedToPlacement | Event-driven incremental update | Placement Read Model Team |
| PlacementStatisticsProjection | PlacementCompleted, PlacementDriveClosed, InternshipCompleted | Scheduled and event-driven refresh | Analytics Read Model Team |
| AuditProjection | All placement events | Continuous append-only projection | Compliance Read Model Team |

# 23. Integration Architecture

The Placement Management domain participates in the enterprise event ecosystem by consuming authoritative information from upstream domains and publishing placement-related business events for downstream consumers.

The domain never directly modifies data owned by other bounded contexts. All multi-domain interactions occur through enterprise APIs, commands, read models, or published events.

---

# 23.1 Internal Enterprise Integrations

## Section A — Identity & Access Management

### Consumed Services

- Authentication
- Authorization
- Role Resolution
- User Identity
- Session Validation

### Consumed Events

- UserCreated
- UserActivated
- UserDeactivated
- RoleAssigned
- RoleRevoked

### Published Events

None

---

## Section C — Student Management

### Consumed Services

- Student Master Information
- Student Profile
- Student Contact Information
- Student Resume Profile
- Academic Program Information

### Consumed Events

- StudentCreated
- StudentUpdated
- QualificationAwarded
- StudentStatusChanged

### Published Events

- PlacementCompleted
- InternshipCompleted

---

## Section D — Academic Management

### Consumed Services

- CGPA
- Percentage
- Semester Results
- Active Backlogs
- Program
- Branch
- Academic Standing

### Consumed Events

- AcademicResultPublished
- BacklogUpdated
- SemesterCompleted
- AcademicStandingUpdated

### Published Events

None

---

## Section E — People & Workforce

### Consumed Services

- Placement Officer Information
- Placement Coordinator Information
- Department Coordinator Assignment

### Consumed Events

- EmployeeAssigned
- EmployeeUpdated
- EmployeeTransferred

### Published Events

None

---

## Section H02 — Enterprise Notification

### Consumed Services

- Email Delivery
- SMS Delivery
- WhatsApp Delivery
- Push Notification Delivery

### Notification Triggers

- Employer Registration
- Employer Verification
- Drive Publication
- Registration Opening
- Registration Closing
- Screening Completion
- Shortlist Publication
- Interview Scheduling
- Offer Publication
- Offer Withdrawal
- Offer Acceptance
- Joining Confirmation

---

## Section H03 — Enterprise Directory & Search

### Published Search Objects

- Employers
- Placement Drives
- Positions
- Internship Opportunities
- Placement Statistics

---

## Section H07 — Enterprise Dashboards

### Published Metrics

- Active Employers
- Active Drives
- Registered Students
- Shortlisted Students
- Selected Students
- Offers Generated
- Offers Accepted
- Internship Statistics
- Joining Statistics
- Placement Percentage
- Employer Participation
- Average Compensation
- Highest Compensation

---

# 23.2 External Integrations

## LinkedIn Integration

Purpose:

- Employer discovery
- Organization validation
- Employer profile enrichment

Capabilities

- Organization lookup
- Company profile synchronization
- Industry information retrieval

---

## AI Resume Screening

Purpose

Provide AI-assisted candidate evaluation against employer requirements.

Capabilities

- Resume parsing
- Skill extraction
- Resume scoring
- Position matching
- Candidate ranking assistance

Business Rule

AI recommendations support human decision-making and shall never replace authorized institutional approval.

---

## Email Integration

Purpose

Official institutional communication.

Examples

- Employer communication
- Registration confirmation
- Interview schedule
- Offer publication
- Joining reminders

---

## SMS Integration

Purpose

Time-sensitive candidate notifications.

Examples

- Registration deadlines
- Interview reminders
- Offer alerts

---

## WhatsApp Integration

Purpose

Operational communication.

Examples

- Placement reminders
- Schedule updates
- Emergency announcements

---

## 23.3 Enterprise API Catalogue

### Command APIs

The Placement domain exposes command-oriented APIs for authoritative write-side operations that are executed by authorized institutional users or downstream orchestration services.

| API | Architectural Purpose |
|-----|-----------------------|
| POST /employers | Register employer records and verification requests |
| POST /placement-drives | Create and maintain placement drives |
| POST /positions | Create and maintain positions within a drive |
| POST /applications | Register candidate participation and screening state |
| POST /offers | Generate and manage placement offers |
| POST /joining-confirmations | Record joining confirmation workflows |

### Query APIs

The Placement domain exposes query-oriented APIs for optimized read access to placement dashboards, operational summaries, and reporting views.

| API | Architectural Purpose |
|-----|-----------------------|
| GET /placement-drives | Retrieve placement drive summaries |
| GET /positions | Retrieve position summaries |
| GET /applications | Retrieve candidate application summaries |
| GET /offers | Retrieve offer summaries |
| GET /statistics | Retrieve placement statistics and analytics |

### External Integration APIs

The Placement domain integrates with external services through well-defined enterprise contracts.

| External Integration | Architectural Purpose |
|----------------------|-----------------------|
| LinkedIn | Employer discovery, organization validation, and profile enrichment |
| AI Resume Screening | Candidate evaluation support for screening workflows |
| Notification Service | Delivery of placement notifications across enterprise channels |

---

# 24. Notification Matrix

| Business Event | Recipient | Channel |
|---------------|-----------|---------|
| EmployerRegistered | Placement Officer | Email |
| EmployerVerified | Employer | Email |
| PlacementDrivePublished | Eligible Students | Email, SMS, WhatsApp |
| DriveRegistrationOpened | Eligible Students | Email, SMS |
| DriveRegistrationClosed | Placement Officer | Email |
| CandidateScreened | Placement Officer | Email |
| ShortlistPublished | Candidates | Email, SMS, WhatsApp |
| PrePlacementTalkScheduled | Candidates | Email, SMS |
| RecruitmentRoundStarted | Candidates | Email, SMS, WhatsApp |
| RecruitmentRoundCompleted | Placement Officer | Email |
| PlacementOfferGenerated | Candidate | Email, SMS, WhatsApp |
| PlacementOfferWithdrawn | Candidate | Email, SMS, WhatsApp |
| PlacementOfferAccepted | Employer | Email |
| PlacementCompleted | Employer, Placement Officer | Email |
| PlacementDriveClosed | Placement Officer | Email |

---

# 25. Authorization Matrix

| Operation | Placement Officer | Placement Coordinator | Student |
|-----------|-------------------|-----------------------|---------|
| Register Employer | ✓ | ✓ | — |
| Verify Employer | ✓ | — | — |
| Create Placement Drive | ✓ | ✓ | — |
| Configure Position | ✓ | ✓ | — |
| Configure Eligibility | ✓ | ✓ | — |
| Publish Placement Drive | ✓ | ✓ | — |
| Register for Position | — | — | ✓ |
| Upload Resume | — | — | ✓ |
| Participate in Recruitment | — | — | ✓ |
| Record Screening | ✓ | ✓ | — |
| Publish Shortlist | ✓ | ✓ | — |
| Record Interview Results | ✓ | ✓ | — |
| Generate Offer | ✓ | ✓ | — |
| Withdraw Offer | ✓ | — | — |
| Approve Candidate Reconsideration | ✓ | — | — |
| Override Eligibility | ✓ | — | — |
| Verify Joining | ✓ | ✓ | — |
| Close Placement Drive | ✓ | — | — |

---

# 26. Security Requirements

The Placement Management domain shall comply with the enterprise security architecture.

## Authentication

All users shall authenticate through the enterprise Identity and Access Management platform.

---

## Authorization

Role-Based Access Control (RBAC) shall govern every operation.

No business operation may execute without explicit authorization.

---

## Data Protection

Sensitive information including:

- Compensation packages
- Candidate evaluations
- Interview feedback
- Offer documents
- Personal information

shall be protected according to institutional security policies.

---

## Audit Protection

Audit records shall be immutable.

Audit history shall never be modified or deleted through operational workflows.

---

## Privacy

Candidate information shall only be visible to authorized institutional users participating in recruitment activities.

Employer-specific confidential information shall remain isolated from other employers.

---

# 27. Validation Rules

Representative validation rules include:

- Employer must be verified before creating a placement drive.
- Every placement drive shall contain at least one position.
- Every position shall define eligibility before publication.
- Students shall satisfy all mandatory eligibility criteria before registration.
- Registration shall occur only during the configured registration window.
- Recruitment stages shall execute in configured order.
- Mandatory recruitment stages shall not be skipped.
- Offers shall only be generated for selected candidates.
- Joining confirmation requires an accepted offer.
- Closed placement drives become immutable.

---

# 28. Error Handling Principles

Business errors shall return meaningful domain validation messages.

Examples include:

- Employer already exists.
- Employer not verified.
- Position capacity exceeded.
- Candidate not eligible.
- Registration closed.
- Recruitment stage incomplete.
- Offer expired.
- Joining already confirmed.
- Placement drive already closed.

System failures shall generate operational events for monitoring while preserving transactional consistency.

# 29. Reporting & Analytics

The Placement Management domain provides operational, strategic, and executive reporting through optimized read models and analytical projections.

Reports shall be generated without impacting transactional workloads and shall consume CQRS read models.

---

## 29.1 Operational Reports

Operational reports support daily activities of Placement Officers and Placement Coordinators.

### Reports

- Active Placement Drives
- Upcoming Placement Drives
- Employer Registration Status
- Employer Verification Pending
- Student Registrations
- Eligible Candidate List
- Shortlisted Candidates
- Recruitment Round Progress
- Offer Status
- Joining Confirmation Status
- Internship Status

---

## 29.2 Employer Reports

Employer-specific reports include:

- Recruitment History
- Positions Published
- Applications Received
- Screening Statistics
- Selection Ratio
- Offer Acceptance Rate
- Joining Percentage
- Internship Conversion Rate

---

## 29.3 Student Reports

Student reports include:

- Registered Drives
- Recruitment Progress
- Interview History
- Offer History
- Placement Status
- Internship History
- Joining Status

---

## 29.4 Institutional Reports

Institution-level reports include:

- Placement Percentage
- Department-wise Placement
- Program-wise Placement
- Branch-wise Placement
- Highest Package
- Average Package
- Median Package
- Company-wise Recruitment
- Internship Statistics
- Alumni Placement Summary

---

## 29.5 Executive Dashboards

Executive dashboards expose real-time KPIs including:

- Active Employers
- Active Placement Drives
- Open Positions
- Students Registered
- Students Shortlisted
- Students Selected
- Offers Generated
- Offers Accepted
- Joining Confirmations
- Placement Percentage
- Internship Conversion Rate
- Average Salary Package
- Highest Salary Package
- Employer Participation
- Recruitment Pipeline Status

---

# 30. Performance & Scalability

The Placement Management domain shall support enterprise-scale deployment.

## Performance Objectives

- Support concurrent placement drives.
- Support thousands of simultaneous student registrations.
- Support concurrent screening operations.
- Support bulk candidate processing.
- Support bulk notification delivery.
- Support real-time dashboard updates.

---

## Scalability Principles

The domain shall scale horizontally using independent services for:

- Employer Management
- Placement Drive Management
- Candidate Management
- Recruitment Workflow
- Offer Management
- Internship Management
- Analytics

Read models shall scale independently from transactional aggregates.

---

## Event Processing

Long-running operations shall execute asynchronously.

Examples include:

- AI Resume Screening
- Candidate Ranking
- Bulk Notification Delivery
- Dashboard Projection Updates
- Report Generation
- Placement Analytics Dashboard

---

## 30.1 Distributed System Considerations

The Placement Management domain shall apply the following distributed systems controls across commands, events, APIs, and projections:

- Correlation ID for end-to-end workflow tracing
- Causation ID for linking follow-up actions to their initiating commands or events
- Idempotent command handling for retried submissions
- Event versioning for safe evolution of published contracts
- Retry policy for transient integration and projection failures
- Dead Letter Queue handling for poison messages and repeated failures
- Event ordering guarantees where aggregate sequencing matters
- Optimistic concurrency for aggregate consistency under concurrent updates
- Distributed tracing across internal and external enterprise boundaries

## Availability

The Placement Management domain shall remain operational during high-volume campus recruitment activities.

Failure of reporting or analytical projections shall not affect transactional workflows.

---

# 31. Audit & Compliance

Every business operation shall produce immutable audit records.

Audit history shall support institutional governance, compliance reviews, dispute resolution, and historical analysis.

---

## Audit Information

Each audit record shall capture:

- Aggregate Identifier
- Aggregate Type
- Business Operation
- Command Executed
- Event Generated
- Previous State
- Current State
- User
- User Role
- Timestamp
- Source System
- Correlation Identifier
- Reason (where applicable)

---

## Auditable Operations

The following operations shall always be audited:

- Employer Registration
- Employer Verification
- Placement Drive Creation
- Position Publication
- Eligibility Configuration
- Student Registration
- Candidate Screening
- Shortlisting
- Recruitment Evaluations
- Candidate Reconsideration
- Offer Generation
- Offer Acceptance
- Offer Withdrawal
- Internship Assignment
- Internship Conversion
- Joining Confirmation
- Placement Drive Closure

---

## Compliance Principles

The Placement Management domain shall ensure:

- Complete historical traceability
- Immutable audit history
- Policy enforcement
- Role-based authorization
- Data confidentiality
- Employer data isolation
- Candidate privacy
- Event traceability across enterprise domains

---

# 32. Extension Points

The architecture intentionally provides extension points for future institutional requirements.

Future enhancements may include:

- Employer Self-Service Portal
- Online Assessment Platform Integration
- Video Interview Platform Integration
- AI Interview Evaluation
- Campus Ambassador Programs
- Alumni Referral Programs
- International Placement Support
- Multi-Campus Recruitment
- Employer Rating System
- Placement Fee Management
- Offer Negotiation Workflow
- Industry Partnership Management
- Campus Recruitment Calendar Synchronization
- Placement Mobile Application

These enhancements shall integrate through commands, events, APIs, and read models without modifying existing aggregate boundaries.

---

# 33. Architecture Assumptions

This specification is based on the following enterprise assumptions:

- Student master records are owned by Section C.
- Academic eligibility is owned by Section D.
- Identity and access management are owned by Section A.
- Placement Officers and Coordinators are managed by Section E.
- Notifications are delegated to Section H02.
- Enterprise search capabilities are delegated to Section H03.
- Dashboards are provided by Section H07.
- Employer-originated business actions are recorded by authorized Placement Officers or Placement Coordinators; no employer user account or employer portal is assumed.
- Canonical resume ownership remains in Section C, while Placement stores only immutable resume snapshot references used during recruitment.
- Placement policies are institution-configurable.
- AI recommendations assist but do not replace institutional decisions.
- Recruitment workflows are configurable and employer-specific.

---

# 34. Future Evolution

Future versions of this specification may introduce:

- AI-driven candidate recommendation engines
- Predictive placement analytics
- Employer API integrations
- Skill-gap analysis
- Learning recommendation engines
- Industry competency mapping
- National placement exchange integration
- Government employment portal integration
- Advanced workforce intelligence
- Cross-institution recruitment collaboration

All future capabilities shall preserve aggregate boundaries, event-driven communication, and enterprise ownership principles.

---

# 35. Glossary

| Term | Definition |
|------|------------|
| Employer | Organization participating in institutional recruitment |
| Placement Drive | Recruitment campaign conducted by an employer |
| Position | Employment or internship opportunity offered during a placement drive |
| Candidate Application | Student application for a specific position |
| Recruitment Workflow | Configurable sequence of recruitment stages |
| Screening | Evaluation of candidate eligibility before recruitment |
| Shortlisting | Selection of candidates for recruitment rounds |
| Placement Offer | Employment or internship offer issued to a candidate |
| Internship | Temporary industry engagement managed through the placement process |
| Joining Confirmation | Verification that the candidate has successfully joined the employer |
| Placement Policy | Institution-configurable rules governing placement eligibility and participation |
| CQRS | Command Query Responsibility Segregation architecture |
| Domain Event | Immutable business fact published after successful command execution |
| Aggregate | Transactional consistency boundary in Domain-Driven Design |
| Read Model | Optimized query model used for reporting and dashboards |

---

# 36. Conclusion

The Placement Management domain establishes a comprehensive enterprise architecture for managing institutional placement and internship activities within the SutraERP platform.

The architecture adheres to Domain-Driven Design, CQRS, and Event-Driven Architecture principles while maintaining clear ownership boundaries, immutable auditability, configurable institutional policies, and scalable workflow orchestration.

By separating transactional aggregates from query projections and integrating with upstream enterprise domains through commands, events, APIs, and read models, the module provides a robust foundation for employer engagement, student recruitment, placement governance, internship management, and institutional placement intelligence.

This document serves as the authoritative Level 2 specification for the Placement Management bounded context and forms the baseline for subsequent Level 3 workflow decomposition, service implementation, API contracts, and user interface design.

---

## States

State machines for each aggregate (Employer, PlacementDrive, Position, CandidateApplication, RecruitmentWorkflow, PlacementOffer, Internship, JoiningConfirmation) are defined via the per-aggregate Lifecycle diagrams and Transition Tables in Section 17 Aggregate Specifications. Each Transition Table enumerates the aggregate's valid states, the command that triggers each transition, and the resulting next state. No separate unified state-machine catalogue exists outside these per-aggregate definitions.
