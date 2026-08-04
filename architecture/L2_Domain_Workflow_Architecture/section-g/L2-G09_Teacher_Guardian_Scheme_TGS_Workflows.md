# L2-G09 — Teacher Guardian Scheme (TGS) Workflows

**Document Version:** 1.0  
**Repository Level:** L2 Business Workflow Architecture  
**Domain Code:** G09  
**Domain Name:** Teacher Guardian Scheme (TGS)  
**Architecture Level:** Enterprise Business Workflow  
**Status:** Draft  
**Document Owner:** Devang

**Upstream Dependencies**

- Section A — Platform Foundation
- Section B — Institution & Structure
- Section C — Student Lifecycle
- Section D — Academic Delivery
- Section E — People & Workforce
- Section F — Finance
- G03 — Hostel Management
- G05 — Library Management
- G06 — Institutional Health Services

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §2 Domain Scope; §4 Explicitly Out of Scope; Audit Scope |
| Actors | §5 Domain Actors; Primary Actors; Secondary Actors |
| Entities | §9 Aggregate Model; Primary Aggregate |
| Business Rules | §19 Business Rules |
| Workflow | §11 Workflow 1 — Mentorship Relationship Activation; §12 Workflow 2 — Scheduled Mentorship Session; §13 Workflow 3 — Student Mentoring Discussion |
| State Machine | §18 Mentorship Case State Model |
| Contracts | §22 Service Contracts; Command Contracts; Query Contracts |
| Events | §21 Domain Events; Relationship Events; Discussion Events |
| Dependencies | §6 Enterprise Dependencies |
| Locked Decisions | §30 Architecture Decision Records (ADR) |

---

# 1. Purpose

The Teacher Guardian Scheme (TGS) domain governs the enterprise workflows that support structured faculty mentorship for students throughout an academic session.

The domain provides a centralized mentoring environment where assigned Teacher Guardians can conduct mentoring interactions, record discussions, provide guidance, maintain mentoring history, coordinate follow-up activities, and refer students to appropriate institutional departments whenever required.

The Teacher Guardian Scheme does **not** own academic, financial, hostel, medical, library, examination, or administrative operations. Instead, it consumes enterprise information from canonical domains to provide Teacher Guardians with sufficient context during mentoring interactions.

The primary objective of this domain is to strengthen faculty–student mentorship while maintaining strict enterprise ownership boundaries across SutraERP.

---

# 2. Domain Scope

The Teacher Guardian Scheme domain governs the complete lifecycle of mentoring interactions between assigned Teacher Guardians and students.

The domain includes:

- Mentorship relationship lifecycle
- Student mentoring sessions
- Mentoring discussion records
- Guidance and recommendations
- Parent interaction records
- Referral management
- Student follow-up management
- Mentorship history
- Mentorship case closure
- Read-only enterprise student context for mentoring purposes

The domain provides a consolidated mentoring workspace while ensuring that operational ownership always remains within the respective canonical domains.

---

# 3. Canonical Responsibilities

The Teacher Guardian Scheme domain is responsible for:

- Maintaining mentorship relationships for assigned students
- Recording mentoring discussions
- Recording observations during mentoring interactions
- Recording guidance provided by Teacher Guardians
- Recording recommendations issued during mentoring
- Managing parent interaction records
- Managing referrals to enterprise domains
- Managing mentoring follow-up activities
- Maintaining complete mentoring history
- Providing Teacher Guardians with a consolidated read-only student context
- Preserving complete mentoring audit history

---

# 4. Explicitly Out of Scope

The Teacher Guardian Scheme domain does **not** own or modify enterprise data belonging to other domains.

The following responsibilities remain outside this domain.

## Academic Domain

- Teacher Guardian assignment
- Academic session
- Timetable
- TGS timetable periods
- Attendance
- Internal assessment
- Academic performance
- Semester management

---

## Student Domain

- Student master
- Student profile
- Student admission
- Student academic lifecycle

---

## Faculty Domain

- Faculty master
- Faculty employment
- Faculty profile

---

## Finance Domain

- Fee assessment
- Fee collection
- Payments
- Scholarships
- Financial accounting

---

## Hostel Domain

- Hostel administration
- Room allocation
- Hostel discipline
- Hostel operations

---

## Medical Domain

- Medical records
- Diagnosis
- Treatment
- Medical consultations

---

## Library Domain

- Membership
- Circulation
- Fine management

---

## Examination Domain

- Examination registration
- Hall tickets
- Results
- Grade processing

---

## Notification Domain

- Notification delivery
- Email
- SMS
- Push notifications

---

# 5. Domain Actors

The Teacher Guardian Scheme domain involves the following enterprise actors.

## Primary Actors

- Teacher Guardian
- Student

## Secondary Actors

- Parent / Guardian
- Head of Department
- Department Administration
- Student Welfare Office
- Hostel Administration
- Finance Office
- Medical Unit
- Library Administration
- Placement Office

Secondary actors participate only when referrals, consultations, or mentoring coordination are required.

---

# 6. Enterprise Dependencies

The Teacher Guardian Scheme consumes enterprise information from multiple canonical domains while preserving ownership boundaries.

## Student Domain

Consumes:

- Student Profile
- Program
- Semester
- Division
- Batch
- Student Status

Ownership remains with Student Management.

---

## Faculty Domain

Consumes:

- Faculty Profile
- Teacher Guardian Details

Ownership remains with Faculty Management.

---

## Academic Domain

Consumes:

- Teacher Guardian Assignment
- Academic Session
- TGS Timetable Schedule
- Attendance Summary
- Academic Performance Summary

Academic remains the canonical owner of all scheduling and assignment activities.

---

## Finance Domain

Consumes:

- Fee Summary
- Outstanding Amount
- Installment Status
- Financial Hold Status

Financial ownership remains with Finance.

---

## Hostel Domain

Consumes:

- Hostel Residency Status
- Hostel Summary
- Hostel Discipline Summary

Operational ownership remains with Hostel Management.

---

## Medical Domain

Consumes:

- Medical Referral Status
- Fitness Advisory Status

No confidential medical records are owned or maintained by the Teacher Guardian Scheme.

---

## Library Domain

Consumes:

- Library Membership Status
- Outstanding Library Obligations

Operational ownership remains with Library Management.

---

# 7. Canonical Data Ownership

The Teacher Guardian Scheme owns only mentoring-related business information.

## Owned by Teacher Guardian Scheme

- Mentorship Relationship
- Student Mentorship Case
- Mentoring Discussion
- Discussion Notes
- Guidance Records
- Recommendations
- Parent Interaction Records
- Referral Records
- Follow-up Records
- Mentorship History
- Mentorship Case Status

---

## Consumed as Read-Only

- Student Information
- Faculty Information
- Academic Summary
- Attendance Summary
- Academic Performance Summary
- Fee Summary
- Hostel Summary
- Library Summary
- Medical Referral Summary

These records are never modified by the Teacher Guardian Scheme.

---

# 8. Core Architectural Principles

The Teacher Guardian Scheme follows the enterprise architectural principles adopted throughout SutraERP.

## Canonical Ownership

Every operational record remains owned by its canonical enterprise domain.

Teacher Guardian Scheme consumes enterprise information but never duplicates or modifies external business records.

---

## Relationship-Centric Architecture

The central business capability of the domain is the mentorship relationship between Teacher Guardian and Student.

All workflows revolve around mentoring interactions rather than operational transactions.

---

## Read-Only Enterprise Context

Teacher Guardians receive consolidated enterprise information solely for mentoring purposes.

Operational decisions continue to be performed within their respective enterprise domains.

---

## Guidance-Oriented Workflows

Teacher Guardian Scheme records:

- Discussions
- Guidance
- Recommendations
- Follow-ups
- Referrals

The domain does not execute operational activities on behalf of other domains.

---

## Enterprise Coordination

The Teacher Guardian Scheme coordinates student support by connecting students with the appropriate enterprise departments whenever required while preserving complete business ownership boundaries.
# 9. Aggregate Model

The Teacher Guardian Scheme domain is centered around a single primary business aggregate.

## Primary Aggregate

**Student Mentorship Case**

A Student Mentorship Case represents the complete mentoring relationship between an assigned Teacher Guardian and a student for a specific academic assignment period.

The Mentorship Case serves as the enterprise container for every mentoring interaction conducted throughout the assignment lifecycle.

The Mentorship Case may contain:

- Teacher Guardian Reference
- Student Reference
- Assignment Reference
- Mentorship Status
- Mentoring Discussion History
- Parent Interaction Records
- Guidance Records
- Recommendation Records
- Referral Records
- Follow-up Records
- Case Closure Information
- Audit History

The Mentorship Case never stores operational data owned by other enterprise domains.

Instead, it references enterprise read models whenever contextual information is required.

---

# 10. Mentorship Relationship Lifecycle

The mentorship relationship begins only after Academic publishes the Teacher Guardian assignment.

Teacher Guardian Scheme does not create assignments.

Teacher Guardian Scheme activates mentoring operations based upon Academic ownership.

Lifecycle Overview:

```text
Academic Assignment Published

↓

Mentorship Relationship Activated

↓

Mentoring Sessions

↓

Discussions Recorded

↓

Guidance Provided

↓

Recommendations

↓

Referrals (If Required)

↓

Follow-up Activities

↓

Relationship Completed

↓

Mentorship Case Closed
```

Each mentorship relationship maintains complete historical continuity throughout its lifecycle.

---

# 11. Workflow 1 — Mentorship Relationship Activation

## Purpose

Activate the mentoring relationship after Academic assigns a Teacher Guardian to a student.

Assignment ownership remains entirely within the Academic domain.

Teacher Guardian Scheme simply establishes the mentoring workspace required for subsequent mentoring activities.

---

## Trigger

Academic publishes a valid Teacher Guardian assignment.

---

## Preconditions

The following information must already exist.

- Student
- Teacher Guardian
- Academic Session
- Guardian Assignment

All of the above remain owned by their canonical domains.

---

## Workflow

```text
Academic

↓

Teacher Guardian Assignment Published

↓

Teacher Guardian Scheme Receives Assignment

↓

Mentorship Relationship Activated

↓

Student Mentorship Case Created

↓

Mentorship History Initialized

↓

Relationship Available for Mentoring
```

---

## Postconditions

The following become available.

- Active Mentorship Relationship
- Student Mentorship Case
- Mentorship Timeline
- Discussion Workspace
- Follow-up Workspace

---

## Business Rules

- Assignment cannot originate from Teacher Guardian Scheme.
- Only Academic may create or modify assignments.
- Only one active mentorship relationship may exist for the same assignment period.
- Historical mentorship relationships remain immutable.
- Assignment changes create a new mentorship relationship while preserving historical mentoring records.

---

# 12. Workflow 2 — Scheduled Mentorship Session

## Purpose

Record mentoring activities conducted during scheduled Teacher Guardian sessions.

Scheduling remains owned by the Academic Timetable domain.

Teacher Guardian Scheme records only the mentoring interaction.

---

## Trigger

Scheduled Teacher Guardian period begins.

or

A mentoring interaction occurs outside the scheduled period.

---

## Preconditions

- Active Mentorship Relationship
- Assigned Teacher Guardian
- Active Student

---

## Workflow

```text
Teacher Guardian

↓

Opens Assigned Student

↓

Reviews Student Context

↓

Conducts Mentoring Session

↓

Discussion Recorded

↓

Guidance Recorded

↓

Recommendations Recorded

↓

Session Completed
```

---

## Student Context Available

Teacher Guardian may review consolidated read-only information including:

Academic

- Attendance Summary
- Academic Performance Summary

Finance

- Fee Summary
- Outstanding Balance

Hostel

- Residency Status
- Hostel Summary

Library

- Membership Summary
- Outstanding Obligations

Medical

- Referral Status
- Fitness Advisory

The above information is never modified by Teacher Guardian Scheme.

---

## Business Rules

- Student context remains read-only.
- Teacher Guardian cannot modify external domain records.
- Multiple discussions may occur within a single mentoring session.
- Every mentoring session becomes part of permanent mentorship history.

---

# 13. Workflow 3 — Student Mentoring Discussion

## Purpose

Capture structured mentoring discussions between the Teacher Guardian and student.

The discussion becomes the central business record of the mentoring process.

---

## Trigger

Teacher Guardian initiates a discussion.

or

Student requests guidance.

---

## Workflow

```text
Teacher Guardian

↓

Student Discussion Begins

↓

Discussion Topics Selected

↓

Discussion Notes Recorded

↓

Guidance Provided

↓

Recommendations Recorded

↓

Referral Required?

↓

Yes → Referral Workflow

↓

No

↓

Follow-up Required?

↓

Yes → Follow-up Workflow

↓

No

↓

Discussion Closed
```

---

## Discussion Categories

One or more categories may be associated with each mentoring discussion.

Examples include:

- Academic Guidance
- Attendance Discussion
- Financial Discussion
- Hostel Discussion
- Career Guidance
- Personal Discussion
- Parent Interaction
- Medical Referral
- Library Discussion
- General Mentoring

The categories classify the discussion only.

They do not transfer ownership of enterprise business records.

---

## Discussion Record

Each mentoring discussion may include:

- Discussion Date
- Teacher Guardian
- Student
- Discussion Categories
- Discussion Notes
- Guidance Provided
- Recommendations
- Referral Indicator
- Follow-up Indicator
- Supporting Remarks

---

## Discussion Outcomes

A discussion may conclude with one or more outcomes.

Examples include:

- Guidance Provided
- Recommendation Issued
- Referral Created
- Parent Interaction Required
- Follow-up Required
- No Further Action

These outcomes describe the mentoring interaction only.

They do not initiate operational activities within external enterprise domains.

---

## Business Rules

- Discussion history is immutable after completion.
- Corrections require enterprise audit.
- Every discussion belongs to exactly one Student Mentorship Case.
- Discussion records become part of permanent mentorship history.
- Teacher Guardian Scheme records mentoring interactions only and never modifies operational records owned by other domains.
# 14. Workflow 4 — Parent Interaction

## Purpose

Record structured interactions between the Teacher Guardian and the student's parent or guardian whenever parent involvement becomes necessary during the mentoring process.

The Teacher Guardian Scheme records the interaction, guidance exchanged, and agreed follow-up actions.

The domain does not manage official parent communication channels, notifications, or institutional correspondence.

---

## Trigger

A parent interaction may be initiated when:

- Teacher Guardian determines parent involvement is beneficial.
- Student requests parent involvement.
- Parent requests interaction through institutional processes.
- Previous mentoring discussion recommends parent consultation.

---

## Workflow

```text
Teacher Guardian

↓

Parent Interaction Initiated

↓

Parent Discussion Conducted

↓

Discussion Summary Recorded

↓

Guidance Provided

↓

Recommendations Recorded

↓

Further Follow-up Required?

↓

Yes

↓

Follow-up Workflow

↓

No

↓

Parent Interaction Closed
```

---

## Parent Interaction Record

Each interaction may include:

- Interaction Date
- Student
- Parent / Guardian
- Teacher Guardian
- Interaction Type
- Discussion Summary
- Guidance Provided
- Recommendations
- Follow-up Requirement
- Additional Remarks

---

## Business Rules

- Parent interaction belongs to one Student Mentorship Case.
- Multiple parent interactions may exist during a mentorship lifecycle.
- Parent interaction history remains immutable.
- Teacher Guardian Scheme records only mentoring-related parent interactions.

---

# 15. Workflow 5 — Referral Workflow

## Purpose

Allow the Teacher Guardian to recommend that a student consult an appropriate institutional department whenever an issue requires specialized attention.

Teacher Guardian Scheme records the referral.

Operational responsibility remains with the receiving domain.

---

## Trigger

A mentoring discussion identifies an issue requiring assistance from another enterprise domain.

---

## Referral Targets

Typical referral destinations include:

- Academic Department
- Finance Office
- Hostel Administration
- Library
- Medical Unit
- Student Welfare Office
- Placement Office
- Head of Department

Additional departments may be configured by the institution.

---

## Workflow

```text
Mentoring Discussion

↓

Issue Identified

↓

Teacher Guardian Selects Referral Department

↓

Referral Recorded

↓

Student Advised

↓

Referral Status Updated

↓

Referral Closed
```

---

## Referral Record

Each referral may include:

- Referral Date
- Student
- Teacher Guardian
- Referral Department
- Referral Reason
- Supporting Remarks
- Follow-up Indicator
- Referral Status

---

## Business Rules

- Referral is advisory.
- Referral does not automatically create work in another domain.
- Receiving domains maintain complete operational ownership.
- Teacher Guardian Scheme maintains referral history only.
- Multiple referrals may originate from a single mentoring discussion.

---

# 16. Workflow 6 — Follow-up Workflow

## Purpose

Track mentoring follow-up activities after discussions, referrals, or parent interactions.

Follow-up represents continued mentoring engagement rather than operational task management.

---

## Trigger

A mentoring interaction recommends future review.

---

## Workflow

```text
Discussion Completed

↓

Follow-up Required

↓

Follow-up Created

↓

Teacher Guardian Meets Student

↓

Progress Reviewed

↓

Additional Guidance

↓

Further Follow-up Required?

↓

Yes

↓

Repeat

↓

No

↓

Follow-up Closed
```

---

## Follow-up Record

Each follow-up may include:

- Follow-up Date
- Related Discussion
- Student
- Teacher Guardian
- Review Summary
- Guidance Provided
- Recommendations
- Current Status
- Closure Remarks

---

## Business Rules

- Follow-up belongs to one Mentorship Case.
- Follow-up may reference discussions, referrals, or parent interactions.
- Multiple follow-ups may exist.
- Follow-up history remains immutable after completion.

---

# 17. Workflow 7 — Mentorship Case Closure

## Purpose

Complete the mentorship lifecycle after the Teacher Guardian assignment concludes.

Closure preserves the complete mentoring history for future reference while preventing further modifications.

---

## Trigger

One of the following occurs:

- Academic assignment period completes.
- Academic publishes new Teacher Guardian assignment.
- Student exits the institution.
- Administrative closure.

---

## Workflow

```text
Assignment Period Completed

↓

Pending Activities Reviewed

↓

Open Follow-ups Closed

↓

Mentorship History Finalized

↓

Case Archived

↓

Mentorship Case Closed
```

---

## Postconditions

The following remain permanently available:

- Discussion History
- Parent Interaction History
- Referral History
- Follow-up History
- Guidance History
- Complete Audit Trail

No further operational changes are permitted after closure.

---

## Business Rules

- Closed mentorship cases are immutable.
- Historical records remain searchable.
- New academic assignments create new mentorship cases.
- Previous mentorship history is never overwritten.

---

# 18. Mentorship Case State Model

```text
Relationship Activated

↓

Active

↓

Mentoring In Progress

↓

Follow-up Pending

↓

Under Review

↓

Completed

↓

Closed
```

State transitions occur only through valid business workflows.

Historical states remain preserved.

---

# 19. Business Rules

The Teacher Guardian Scheme follows these enterprise business rules.

## Mentorship

- Every mentorship case belongs to one student.
- Every mentorship case references one active Teacher Guardian assignment.
- Teacher Guardian assignment remains owned by the Academic domain.

---

## Discussions

- Every discussion belongs to one mentorship case.
- Discussion history is permanent.
- Discussion categories classify conversations only.

---

## Enterprise Context

Teacher Guardians may view read-only information from:

- Academic
- Finance
- Hostel
- Library
- Medical

Teacher Guardian Scheme never modifies these records.

---

## Referrals

- Referrals are recommendations.
- Operational ownership remains with receiving domains.

---

## Follow-ups

- Follow-up activities support mentoring continuity.
- Multiple follow-ups may exist for a mentorship case.

---

## Audit

Every significant mentoring activity must remain completely auditable throughout the mentorship lifecycle.
# 20. Validation Rules

The Teacher Guardian Scheme enforces business validation to maintain consistency, integrity, and enterprise alignment throughout the mentoring lifecycle.

---

## Mentorship Relationship Validation

The system shall ensure:

- A valid Teacher Guardian assignment exists before activating a mentorship relationship.
- Student reference is valid.
- Teacher Guardian reference is valid.
- Academic Session is active.
- Duplicate active mentorship relationships for the same assignment are not permitted.

---

## Discussion Validation

Before a discussion is recorded:

- Mentorship Relationship must be Active.
- Student must belong to the assigned Teacher Guardian.
- Discussion Date must be valid.
- At least one discussion category must be selected.
- Discussion summary must be recorded.
- Guidance section cannot be empty.

---

## Parent Interaction Validation

Before recording a parent interaction:

- Student must belong to the mentorship relationship.
- Parent interaction date must be valid.
- Interaction summary must be recorded.
- Guidance or recommendation must be captured.

---

## Referral Validation

Before creating a referral:

- Referral destination must exist.
- Referral reason must be provided.
- Mentorship Case must remain active.

---

## Follow-up Validation

Before creating a follow-up:

- A previous mentoring activity must exist.
- Follow-up date must be valid.
- Student must belong to the mentorship relationship.

---

## Closure Validation

Before closing a mentorship case:

- Active mentoring relationship must exist.
- Pending follow-ups should be reviewed.
- Closure reason must be recorded.
- Complete mentoring history must be preserved.

---

# 21. Domain Events

The Teacher Guardian Scheme publishes business events whenever significant mentoring activities occur.

These events communicate business state changes without transferring ownership of enterprise data.

---

## Relationship Events

- MentorshipRelationshipActivated
- MentorshipRelationshipClosed

---

## Discussion Events

- MentoringDiscussionRecorded
- GuidanceRecorded
- RecommendationRecorded

---

## Parent Interaction Events

- ParentInteractionRecorded

---

## Referral Events

- ReferralRecorded
- ReferralClosed

---

## Follow-up Events

- FollowUpCreated
- FollowUpCompleted

---

## Lifecycle Events

- MentorshipCaseClosed

---

## Event Principles

Business events are:

- Immutable
- Business meaningful
- Auditable
- Time ordered
- Enterprise consumable

Events never modify records owned by external domains.

---

# 22. Service Contracts

The Teacher Guardian Scheme exposes canonical service contracts for governed mentoring interactions.

These contracts provide the only supported synchronous integration surface for external domains.

---

## Command Contracts

- ActivateMentorshipRelationship
- RecordMentoringDiscussion
- RecordParentInteraction
- RecordReferral
- CreateFollowUp
- CloseMentorshipCase

---

## Query Contracts

- GetAssignedStudents
- GetStudentMentorshipProfile
- GetMentoringDiscussionHistory
- GetReferralHistory
- GetFollowUpHistory
- GetMentorshipTimeline

---

## Contract Principles

- Service Contracts shall be versioned.
- Service Contracts shall preserve mentorship ownership boundaries.
- External domains shall not modify mentorship aggregates directly.

---

# 23. Read Models

Teacher Guardian Scheme exposes optimized read models for mentoring operations.

Read models are optimized for information consumption and never become transactional ownership models.

---

## Assigned Students View

Displays:

- Student
- Program
- Semester
- Division
- Batch
- Mentorship Status
- Last Discussion Date
- Last Follow-up Date

---

## Student Mentorship Profile

Displays:

- Student Profile
- Teacher Guardian
- Academic Summary
- Attendance Summary
- Fee Summary
- Hostel Summary
- Library Summary
- Medical Referral Summary
- Mentorship History

This becomes the primary mentoring workspace.

---

## Mentoring Discussion History

Displays:

- Discussion Date
- Categories
- Guidance
- Recommendations
- Follow-up Status

---

## Parent Interaction History

Displays:

- Parent Interaction Timeline
- Discussion Summary
- Recommendations

---

## Referral History

Displays:

- Referral Date
- Department
- Referral Reason
- Current Status

---

## Follow-up History

Displays:

- Follow-up Timeline
- Review Summary
- Guidance
- Current Status

---

## Mentorship Timeline

Displays the complete chronological mentoring journey.

Examples include:

- Relationship Activated
- Discussion Recorded
- Parent Interaction
- Referral
- Follow-up
- Case Closed

---

# 24. Reporting

Teacher Guardian Scheme provides operational reporting to assist institutional mentoring activities.

Reporting remains analytical.

Operational ownership remains within canonical domains.

---

## Teacher Guardian Reports

Examples include:

- Assigned Students
- Mentoring Sessions Conducted
- Discussions Recorded
- Parent Interactions
- Referrals
- Follow-ups
- Closed Mentorship Cases

---

## Student Reports

Examples include:

- Mentoring History
- Guidance History
- Parent Interaction History
- Referral History
- Follow-up History

---

## Department Reports

Examples include:

- Teacher Guardian Workload
- Mentorship Coverage
- Student Participation
- Referral Distribution
- Follow-up Completion

---

## Institutional Reports

Examples include:

- Mentorship Activity by Academic Session
- Department-wise Mentorship Statistics
- Parent Interaction Statistics
- Referral Statistics
- Follow-up Statistics

Reports remain configurable according to institutional policies.

---

# 25. Authorization Model

Authorization follows enterprise role-based access principles.

---

## Teacher Guardian

May:

- View assigned students
- View mentoring context
- Record discussions
- Record guidance
- Record recommendations
- Record parent interactions
- Record referrals
- Record follow-ups

May not:

- Modify academic records
- Modify attendance
- Modify fees
- Modify hostel records
- Modify medical records
- Modify library records

---

## Student

May:

- View permitted mentoring history
- View guidance
- View recommendations

Student visibility is governed by institutional policy.

---

## Department Administration

May:

- View departmental mentoring activities
- View reports
- Review mentorship history

Administrative permissions remain configurable.

---

## Institutional Administration

May:

- Access institution-wide mentoring reports
- Review audit history
- Review historical mentorship records

---

# 26. Audit Architecture

Every significant mentoring activity shall be permanently auditable.

Audit supports institutional governance while preserving historical mentoring continuity.

---

## Audit Scope

Audit captures:

- Relationship Activation
- Discussion Creation
- Parent Interaction
- Referral
- Follow-up
- Case Closure

---

## Audit Attributes

Each audit record may include:

- Event Identifier
- Business Event
- Timestamp
- User
- Student
- Teacher Guardian
- Academic Session
- Previous State
- New State

---

## Audit Principles

Audit records are:

- Immutable
- Chronological
- Complete
- Historically searchable
- Enterprise compliant

Audit records are never physically modified after creation.

They become part of the permanent institutional mentoring history.
# 27. Security Considerations

The Teacher Guardian Scheme manages mentoring information that may contain sensitive personal discussions.

Accordingly, security controls shall prioritize confidentiality, integrity, accountability, and controlled information sharing while respecting institutional policies.

---

## Confidentiality

Mentoring records shall be accessible only to authorized users.

Access shall follow institutional authorization policies.

Confidential mentoring information shall never be exposed outside authorized business workflows.

---

## Least Privilege

Users receive only the minimum permissions necessary for their responsibilities.

Examples:

- Teacher Guardians access only assigned students.
- Students access only institution-approved mentoring records.
- Department administrators access departmental reports.
- Institutional administrators access organization-wide analytics according to authorization policies.

---

## Multi-Domain Protection

Teacher Guardian Scheme consumes enterprise information from other domains as read-only contextual information.

The domain shall never:

- Modify academic records
- Modify financial records
- Modify medical records
- Modify hostel records
- Modify library records

Operational ownership always remains with the canonical domain.

---

## Sensitive Discussion Protection

Mentoring discussions may contain sensitive information.

Institutions may configure access policies for:

- Student visibility
- Parent visibility
- Department visibility
- Administrative review
- Historical access

The Teacher Guardian Scheme enforces policy but does not define institutional privacy rules.

---

## Audit Protection

Audit history shall be:

- Immutable
- Tamper resistant
- Historically preserved
- Available only to authorized reviewers

---

# 28. Performance & Scalability

The Teacher Guardian Scheme is a relationship-centric business domain with relatively low transaction volume and high historical value.

Architecture shall therefore optimize for rapid information retrieval, long-term history preservation, and efficient reporting.

---

## Performance Objectives

The system should efficiently support:

- Student mentoring profile retrieval
- Assigned student listing
- Mentoring history retrieval
- Parent interaction history
- Referral history
- Follow-up history
- Institutional mentoring reports

---

## Read Optimization

Frequently accessed information shall be optimized through read models.

Examples include:

- Assigned Students View
- Student Mentorship Profile
- Mentorship Timeline
- Follow-up Dashboard
- Referral Dashboard

Read optimization shall never compromise canonical ownership.

---

## Historical Preservation

Mentoring history forms part of the institutional student lifecycle.

Historical records shall remain available for:

- Academic review
- Institutional reporting
- Administrative audit
- Historical reference

Historical records shall never be overwritten.

---

## Scalability Principles

The architecture shall support:

- Multiple academic sessions
- Multiple institutions
- Large student populations
- Long-term mentoring history
- Enterprise reporting

Scalability shall remain independent of operational domains.

---

# 29. Extension Points

The Teacher Guardian Scheme has been intentionally designed to support future institutional expansion without changing its core architecture.

Extensions shall complement mentoring workflows while preserving canonical ownership.

---

## Additional Discussion Categories

Institutions may introduce additional mentoring categories.

Examples:

- Entrepreneurship Guidance
- Innovation Activities
- Research Guidance
- Community Engagement
- Alumni Interaction
- International Programs

Discussion categories remain configurable.

---

## Additional Referral Destinations

Institutions may define additional referral destinations.

Examples include:

- Entrepreneurship Cell
- Innovation Center
- Research Cell
- International Relations Office
- Alumni Office
- Career Development Center

Teacher Guardian Scheme records referrals without assuming operational ownership.

---

## Additional Reporting

Institutions may introduce custom mentoring reports.

Examples include:

- Department Mentoring Effectiveness
- Faculty Mentoring Participation
- Session-wise Mentoring Trends
- Referral Distribution Analysis

Reporting extensions do not modify transactional workflows.

---

## Institutional Configuration

Institutions may configure:

- Parent interaction policies
- Student visibility policies
- Discussion categories
- Referral categories
- Follow-up practices
- Reporting preferences

Configuration shall never alter canonical ownership boundaries.

---

# 30. Architecture Decision Records (ADR)

## ADR-G09-001

**Decision**

Teacher Guardian Assignment is owned by the Academic domain.

**Reason**

Assignments are directly related to academic session planning, faculty workload, and timetable generation.

Teacher Guardian Scheme consumes assignments only.

---

## ADR-G09-002

**Decision**

Teacher Guardian Scheme does not own TGS timetable scheduling.

**Reason**

Scheduling belongs to Academic Timetable management.

Teacher Guardian Scheme records mentoring interactions only.

---

## ADR-G09-003

**Decision**

Teacher Guardian Scheme consumes enterprise information using read-only contextual views.

**Reason**

Maintains strict canonical ownership while providing Teacher Guardians with complete mentoring context.

---

## ADR-G09-004

**Decision**

Teacher Guardian Scheme is a relationship-centric domain.

**Reason**

The primary business capability is mentoring and student guidance rather than operational administration.

---

## ADR-G09-005

**Decision**

Teacher Guardian Scheme records guidance, recommendations, referrals, and follow-ups but never executes operational activities belonging to other domains.

**Reason**

Preserves clear enterprise responsibility boundaries.

---

## ADR-G09-006

**Decision**

Each Teacher Guardian–Student relationship is represented through a Student Mentorship Case.

**Reason**

Provides a complete historical record of mentoring interactions while maintaining lifecycle continuity.

---

# 31. Glossary

| Term | Definition |
|------|------------|
| Teacher Guardian | Faculty member assigned by the Academic domain to mentor students. |
| Student Mentorship Case | Primary business aggregate representing the mentoring relationship. |
| Mentorship Relationship | Active mentoring association between a Teacher Guardian and a student. |
| Mentoring Session | Structured mentoring interaction conducted by the Teacher Guardian. |
| Discussion Record | Permanent record of a mentoring discussion. |
| Guidance | Advice or mentoring provided during a discussion. |
| Recommendation | Suggested course of action arising from mentoring. |
| Parent Interaction | Recorded communication between the Teacher Guardian and a student's parent or guardian. |
| Referral | Recommendation for the student to consult another institutional department. |
| Follow-up | Subsequent mentoring interaction reviewing previous discussions or recommendations. |
| Read Model | Optimized, read-only view of enterprise information consumed by the Teacher Guardian Scheme. |
| Canonical Ownership | Enterprise principle stating that each business record has exactly one owning domain. |
| Academic Assignment | Teacher Guardian assignment published by the Academic domain. |

---

# 32. Domain Summary

The Teacher Guardian Scheme domain provides the enterprise mentoring capability within SutraERP.

It establishes and maintains structured mentoring relationships between Teacher Guardians and students while preserving complete separation of operational ownership across enterprise domains.

The domain owns:

- Student Mentorship Cases
- Mentoring Discussions
- Guidance Records
- Recommendations
- Parent Interactions
- Referrals
- Follow-ups
- Mentorship History

The domain consumes read-only contextual information from:

- Academic
- Student
- Faculty
- Finance
- Hostel
- Library
- Medical

The Teacher Guardian Scheme never modifies operational records owned by other domains.

Instead, it enables Teacher Guardians to understand the complete student context, conduct meaningful mentoring discussions, provide guidance, coordinate referrals, and maintain a permanent institutional mentoring history.

This architecture positions G09 as a **relationship-centric enterprise domain**, fully aligned with SutraERP's principles of Domain-Driven Design, Canonical Ownership, Event-Driven Architecture, CQRS, Effective-Dated Governance, Immutable Audit, and Configuration over Customization.
