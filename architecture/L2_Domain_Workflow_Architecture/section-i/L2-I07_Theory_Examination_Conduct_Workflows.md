# L2-I07 — Theory Examination Conduct Workflows

**Section:** I — Examination  
**Status:** Draft  
**Document Owner:** Devang

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §2 Scope |
| Actors | §6 Business Actors |
| Entities | — not present — |
| Business Rules | §9 Business Rules |
| Workflow | §8 Theory Examination Lifecycle Workflows; §17 Workflow Completion Criteria; L2-I07 — Theory Examination Conduct Workflows |
| State Machine | §20 Theory Examination Conduct State Model |
| Contracts | — not present — |
| Events | §11 Events Published; §12 Events Consumed; §4.9 Event-Driven Examination Conduct |
| Dependencies | §19 Theory Examination Dependency Matrix |
| Locked Decisions | — not present — |

> **Gap:** 3 required section(s) have no counterpart in this document: Entities, Contracts, Locked Decisions. Recorded, not authored.

---

# 1. Purpose

This document defines the canonical business workflows governing the conduct of written and theory examinations within the SutraERP Examination domain.

It establishes the enterprise framework for managing examination centre operations, candidate entry, seating management, invigilator management, attendance, question paper distribution, examination supervision, answer book collection, conduct completion, and operational governance of theory examinations.

The document governs theory examination conduct beginning with candidate participation in an authorised examination and concluding with completion of examination conduct, after which answer books become available for Marks Collection & Evaluation workflows.

This document standardises theory examination operations while allowing institutions to configure operational policies without altering the canonical repository architecture.

---

# 2. Scope

The Theory Examination Conduct domain governs the operational execution of authorised written examinations.

The scope includes:

- Theory Examination Conduct
- Examination Centre Operations
- Examination Hall Management
- Room Allocation
- Seating Plan Management
- Seat Assignment
- Candidate Check-in
- Examination Entry Verification
- Invigilator Assignment
- Attendance Management
- Question Paper Distribution
- Answer Book Distribution
- Examination Supervision
- Candidate Movement Management
- Answer Book Collection
- Conduct Completion
- Conduct Exception Management
- Conduct Lifecycle Governance

This document does not govern:

- Examination Policy Management
- Examination Session Management
- Examination Planning & Scheduling
- Examination Registration
- Hall Ticket Management
- Practical & Viva Examination Conduct
- Question Paper Creation
- Question Paper Moderation
- Question Paper Printing
- Marks Collection & Evaluation
- Result Processing
- Revaluation & Rechecking
- Academic Completion

Ownership of these business capabilities remains with their respective workflow documents.

---

# 3. Repository Context

The Theory Examination Conduct domain is positioned after Hall Ticket Management within the Examination repository.

It consumes authorised examination participation established by previous examination workflows and governs the operational execution of written examinations.

This document consumes business information from:

- L2-I01 — Examination Policy & Configuration Workflows
- L2-I02 — Examination Session Management Workflows
- L2-I03 — Examination Planning & Scheduling Workflows
- L2-I04 — Examination Registration Workflows
- L2-I05 — Hall Ticket Management Workflows

It collaborates with:

- Person & Identity Domain
- Organisation Domain
- Campus & Infrastructure Domain
- Academic Calendar Domain
- Platform Foundation Services

It produces authoritative theory examination conduct records consumed by:

- L2-I08 — Marks Collection & Evaluation Workflows

This document owns operational theory examination conduct only and does not own evaluation or academic outcome processing.

---

# 4. Architectural Principles

The following architectural principles govern Theory Examination Conduct across the SutraERP repository.

---

## 4.1 Authorised Participation Before Conduct

Every theory examination shall be conducted only for candidates possessing valid examination authorisation established through Hall Ticket Management.

---

## 4.2 Conduct Before Evaluation

Theory examination conduct shall be completed before Marks Collection & Evaluation begins.

Completion of examination conduct establishes readiness for downstream evaluation workflows but does not perform evaluation itself.

---

## 4.3 Controlled Examination Environment

Every theory examination shall be conducted within an institutionally authorised examination environment.

The examination environment includes authorised examination centres, examination halls, seating arrangements, invigilators, and approved operational procedures.

---

## 4.4 One Candidate, One Assigned Seat

Each authorised candidate shall be assigned an examination seat for every scheduled theory examination according to institutional policies.

Seat assignment governs operational examination execution and does not modify examination scheduling or registration.

---

## 4.5 Invigilator-Governed Examination

Every theory examination shall be supervised by institutionally authorised invigilators.

Invigilators govern examination conduct and preserve examination integrity without participating in academic evaluation.

---

## 4.6 Attendance as Part of Examination Conduct

Candidate attendance forms part of theory examination conduct.

Attendance establishes examination participation and shall be completed before examination completion.

Attendance records do not represent academic performance.

---

## 4.7 Controlled Conduct Lifecycle

Every theory examination shall progress through a governed operational lifecycle from preparation through completion.

Business state transitions shall be controlled, auditable, and governed by institutional policies.

---

## 4.8 Multi-Domain Collaboration

Theory Examination Conduct collaborates with Registration, Hall Ticket Management, Examination Planning, Campus & Infrastructure, Academic Calendar, Identity, Organisation, and Platform Foundation domains while preserving ownership boundaries.

---

## 4.9 Event-Driven Examination Conduct

Significant business transitions during theory examination conduct shall publish business events for downstream workflows.

Examples include:

- Candidate Checked In
- Seat Assigned
- Attendance Recorded
- Examination Started
- Examination Completed
- Answer Books Collected
- Conduct Closed

---

## 4.10 Complete Auditability

Every significant business activity performed during theory examination conduct shall generate auditable records supporting institutional governance, compliance, and historical traceability.

---

# 5. Business Capabilities

The Theory Examination Conduct domain provides the following canonical business capabilities.

These capabilities govern the operational execution of written examinations while maintaining clear ownership boundaries with other Examination workflows.

---

## 5.1 Examination Centre Operations

The domain shall govern operational management of authorised examination centres.

Examination Centre Operations may include:

- Centre Readiness
- Hall Readiness
- Resource Verification
- Operational Coordination
- Centre Activation

Examination Centre Operations establish operational readiness without modifying examination schedules.

---

## 5.2 Examination Hall Management

The domain shall manage authorised examination halls for theory examination conduct.

Hall Management may include:

- Hall Allocation
- Hall Capacity Management
- Hall Preparation
- Examination Environment Readiness
- Hall Closure

Hall ownership remains with the Campus & Infrastructure domain.

---

## 5.3 Seating Plan Management

The domain shall govern seating arrangements for theory examinations.

Seating Plan Management may include:

- Seating Plan Creation
- Seating Optimisation
- Hall-wise Allocation
- Candidate Distribution
- Seating Validation

Seating plans support examination integrity and operational efficiency.

---

## 5.4 Seat Assignment

The domain shall assign authorised examination seats to candidates.

Seat Assignment may include:

- Candidate Seat Allocation
- Seat Verification
- Seat Reassignment
- Reserved Seating
- Exceptional Seating Allocation

Seat Assignment shall follow institutional examination policies.

---

## 5.5 Candidate Check-in

The domain shall manage candidate participation before examination commencement.

Candidate Check-in may include:

- Candidate Arrival
- Examination Entry Verification
- Hall Ticket Verification
- Seat Confirmation
- Examination Entry Authorization

Successful Check-in establishes operational readiness for examination participation.

---

## 5.6 Invigilator Management

The domain shall govern allocation of authorised invigilators.

Invigilator Management may include:

- Invigilator Assignment
- Chief Invigilator Assignment
- Relief Invigilator Assignment
- Hall Allocation
- Invigilation Responsibility Allocation

Invigilator governance remains institution configurable.

---

## 5.7 Attendance Management

The domain shall record candidate participation during theory examinations.

Attendance Management may include:

- Candidate Presence
- Candidate Absence
- Late Arrival
- Approved Exception
- Attendance Finalisation

Attendance establishes examination participation only.

---

## 5.8 Theory Examination Conduct

The domain shall govern operational execution of written examinations.

Theory Examination Conduct may include:

- Examination Commencement
- Question Paper Distribution
- Answer Book Distribution
- Examination Supervision
- Candidate Movement Management
- Examination Monitoring
- Examination Completion

Operational conduct shall comply with institutional examination policies.

---

## 5.9 Answer Book Collection

The domain shall govern controlled collection of answer books after examination completion.

Answer Book Collection may include:

- Collection Verification
- Candidate Submission
- Invigilator Confirmation
- Collection Finalisation
- Transfer Readiness

Collection establishes readiness for Marks Collection & Evaluation.

---

## 5.10 Conduct Lifecycle Governance

The domain shall govern the complete lifecycle of theory examination conduct.

Lifecycle governance includes:

- Preparation
- Centre Operations
- Candidate Check-in
- Seating
- Attendance
- Examination Conduct
- Answer Book Collection
- Conduct Completion
- Archival

Lifecycle governance ensures standardised theory examination operations.

---

# 6. Business Actors

The following business actors participate in Theory Examination Conduct workflows.

| Business Actor | Responsibility |
|----------------|----------------|
| Candidate | Participates in authorised theory examinations |
| Invigilator | Supervises examination conduct |
| Chief Invigilator | Governs examination hall operations |
| Examination Superintendent | Oversees examination centre operations |
| Examination Administrator | Coordinates examination activities |
| Controller of Examination | Governs institutional examination conduct |
| Centre Coordinator | Manages examination centre readiness |
| Hall Supervisor | Supports examination hall operations |
| Person & Identity Domain | Provides candidate identity information |
| Hall Ticket Management Domain | Provides examination entry authorisation |
| Examination Planning Domain | Provides approved examination schedules |
| Campus & Infrastructure Domain | Provides examination halls and facilities |
| Audit Infrastructure | Maintains examination conduct audit history |

Business actors collaborate while ownership remains with their respective domains.

---

# 7. Theory Examination Framework

The Theory Examination Framework establishes the canonical business structure governing operational execution of written examinations.

The framework standardises theory examination conduct while allowing institutions to configure operational models according to institutional requirements.

---

## 7.1 Theory Examination Objectives

The framework enables institutions to:

- Conduct authorised written examinations.
- Manage examination centres.
- Allocate examination halls.
- Assign candidate seating.
- Allocate invigilators.
- Record attendance.
- Supervise examination conduct.
- Collect answer books.
- Complete examination operations.

Theory Examination Conduct governs operational execution only.

---

## 7.2 Examination Centre Framework

Theory examinations shall be conducted only within institutionally authorised examination centres.

The framework governs:

- Centre Activation
- Centre Readiness
- Hall Readiness
- Operational Coordination
- Centre Closure

Centre ownership remains outside this domain.

---

## 7.3 Seating Framework

Every authorised candidate shall receive an examination seat.

The framework governs:

- Seating Plans
- Seat Allocation
- Seat Validation
- Exceptional Seating
- Seating Integrity

Seat allocation supports operational examination execution.

---

## 7.4 Candidate Participation Framework

Candidate participation is governed through authorised examination entry.

Participation requires:

- Valid Hall Ticket
- Approved Examination Registration
- Assigned Examination Seat
- Scheduled Examination
- Institutional Eligibility

Only authorised candidates may participate in theory examinations.

---

## 7.5 Invigilation Framework

Theory examinations shall be supervised by authorised invigilators.

Institutions may configure:

- Single Invigilator
- Multiple Invigilators
- Chief Invigilator
- Relief Invigilator
- Examination Superintendent

Invigilation governance remains institution configurable.

---

## 7.6 Attendance Framework

Attendance forms an integral part of theory examination conduct.

Attendance establishes:

- Candidate Participation
- Candidate Absence
- Examination Presence
- Examination Eligibility

Attendance ownership remains within examination conduct.

---

## 7.7 Answer Book Management Framework

Answer books represent the official examination response submitted by candidates.

The framework governs:

- Answer Book Distribution
- Candidate Submission
- Collection Verification
- Secure Collection
- Transfer Readiness

Marks Collection & Evaluation begins only after successful completion of Answer Book Collection.

# 8. Theory Examination Lifecycle Workflows

Every theory examination progresses through a governed operational lifecycle.

The lifecycle ensures secure, standardised, auditable, and institutionally governed execution of written examinations.

---

## 8.1 Lifecycle Overview

The standard Theory Examination Conduct lifecycle consists of the following stages:

1. Examination Centre Preparation
2. Examination Hall Preparation
3. Invigilator Assignment
4. Seating Allocation
5. Candidate Check-in
6. Examination Entry Verification
7. Attendance Recording
8. Examination Commencement
9. Theory Examination Execution
10. Answer Book Collection
11. Conduct Verification
12. Ready for Marks Collection & Evaluation
13. Archival

Each stage represents a controlled business transition.

---

## 8.2 Examination Centre Preparation

Theory examination conduct begins by preparing authorised examination centres.

Preparation may include:

- Centre Readiness
- Hall Readiness
- Resource Availability
- Operational Readiness
- Administrative Preparation

Preparation establishes operational readiness without altering examination schedules.

---

## 8.3 Examination Hall Preparation

Authorised examination halls shall be prepared before candidate entry.

Preparation may include:

- Seating Arrangement
- Hall Identification
- Resource Placement
- Invigilator Allocation
- Examination Environment Readiness

Hall preparation ensures examination readiness.

---

## 8.4 Invigilator Assignment

Authorised invigilators shall be assigned before examination commencement.

Assignment may include:

- Chief Invigilator
- Hall Invigilators
- Relief Invigilators
- Examination Superintendent
- Observer Assignment

Invigilator assignment shall comply with institutional policies.

---

## 8.5 Seating Allocation

Candidates shall be assigned examination seats before examination entry.

Seating Allocation supports:

- Examination Integrity
- Candidate Distribution
- Capacity Management
- Exceptional Seating Requirements
- Operational Efficiency

Seat allocation does not modify examination schedules or registrations.

---

## 8.6 Candidate Check-in

Candidates report to the assigned examination centre.

Candidate Check-in may include:

- Candidate Arrival
- Hall Ticket Presentation
- Examination Entry Verification
- Seat Confirmation
- Examination Hall Entry

Successful Check-in authorises operational participation.

---

## 8.7 Examination Entry Verification

Before examination commencement, candidate eligibility shall be verified.

Verification confirms:

- Valid Hall Ticket
- Scheduled Examination
- Assigned Seat
- Institutional Eligibility

Verification consumes information from Hall Ticket Management without modifying Hall Ticket ownership.

---

## 8.8 Attendance Recording

Attendance shall be recorded before or during examination commencement according to institutional policy.

Attendance outcomes may include:

- Present
- Absent
- Late
- Excused
- Approved Exception

Attendance establishes examination participation only.

---

## 8.9 Examination Commencement

Examination Commencement marks the official start of the written examination.

Commencement confirms:

- Invigilators Present
- Candidates Authorised
- Attendance Completed
- Examination Environment Ready
- Question Papers Distributed
- Answer Books Distributed

Official examination timing begins after commencement.

---

## 8.10 Theory Examination Execution

Theory Examination Execution represents the operational conduct of the written examination.

Execution includes:

- Examination Supervision
- Candidate Monitoring
- Candidate Movement Management
- Operational Announcements
- Examination Discipline
- Examination Closure

Execution shall comply with institutional examination regulations.

---

## 8.11 Answer Book Collection

Upon examination completion, answer books shall be collected under controlled procedures.

Collection includes:

- Candidate Submission
- Collection Verification
- Invigilator Confirmation
- Secure Collection
- Transfer Readiness

Answer Book Collection concludes candidate participation.

---

## 8.12 Conduct Verification

Institutional verification confirms successful completion of theory examination conduct.

Verification may include:

- Attendance Verification
- Answer Book Verification
- Invigilator Confirmation
- Administrative Review

Verification establishes operational integrity before downstream evaluation.

---

## 8.13 Ready for Marks Collection & Evaluation

Successfully completed theory examinations become available for Marks Collection & Evaluation.

Operational responsibility transitions to L2-I08.

This document concludes responsibility once examination conduct has been successfully completed.

---

## 8.14 Archival

Completed conduct records enter archival status after institutional closure.

Archived records remain available for:

- Audit
- Compliance
- Institutional Reporting
- Historical Reference
- Regulatory Review

Archival preserves examination history without altering business outcomes.

---

# 9. Business Rules

The following business rules govern Theory Examination Conduct across all institutions.

These rules establish consistent governance while allowing institutional configuration.

---

## 9.1 Authorised Participation Rules

Only candidates possessing valid Hall Tickets shall participate in scheduled theory examinations.

Unauthorised candidates shall not participate unless governed through approved exception workflows.

---

## 9.2 Seating Rules

Every authorised candidate shall receive an institutionally assigned examination seat.

Seat assignments shall:

- Support examination integrity.
- Prevent allocation conflicts.
- Follow approved seating policies.

Seat assignments shall remain unique for each scheduled examination.

---

## 9.3 Invigilation Rules

Every theory examination shall be supervised by authorised invigilators.

Invigilator responsibilities shall be assigned before examination commencement.

Only authorised personnel may supervise examination activities.

---

## 9.4 Attendance Rules

Attendance shall be recorded for every scheduled theory examination.

Attendance shall accurately represent candidate participation.

Attendance records shall not represent academic performance.

---

## 9.5 Question Paper Distribution Rules

Question papers shall be distributed only after authorised examination commencement.

Distribution shall follow institutional examination procedures.

Question paper governance remains outside this document.

---

## 9.6 Answer Book Collection Rules

Answer books shall be collected from every participating candidate.

Collection shall:

- Preserve examination integrity.
- Ensure complete accountability.
- Establish readiness for Marks Collection & Evaluation.

---

## 9.7 Examination Conduct Rules

Theory examinations shall:

- Follow approved examination schedules.
- Be conducted within authorised examination centres.
- Maintain institutional examination integrity.
- Preserve fairness for all candidates.

---

## 9.8 Exception Handling Rules

Exceptional situations shall follow institutionally approved procedures.

Examples include:

- Candidate Late Arrival
- Candidate Illness
- Invigilator Replacement
- Hall Change
- Emergency Evacuation
- Examination Suspension

Exception handling shall preserve complete auditability.

---

## 9.9 Audit Rules

Every significant theory examination activity shall generate audit records.

Examples include:

- Candidate Check-in
- Seat Assignment
- Attendance Recording
- Examination Commencement
- Answer Book Collection
- Conduct Verification
- Administrative Override

All business transitions shall remain historically traceable.

---

# 10. Exception Scenarios

The Theory Examination Conduct domain shall support controlled handling of exceptional business situations.

---

## 10.1 Candidate Absent

If a candidate does not report for the examination:

- Attendance shall record absence.
- Examination participation shall not occur.
- Appropriate downstream workflows shall receive the attendance outcome.

---

## 10.2 Invalid Hall Ticket

If examination entry verification fails:

- Candidate entry shall be denied.
- Examination participation shall not commence.
- Institutional exception procedures may apply.

---

## 10.3 Seat Reassignment

Where operational circumstances require seat reassignment:

- An authorised seat shall be allocated.
- Examination integrity shall be maintained.
- Audit records shall capture the reassignment.

---

## 10.4 Invigilator Unavailable

Where an assigned invigilator becomes unavailable:

- Authorised replacement procedures may be initiated.
- Examination supervision shall continue only after institutional approval.

---

## 10.5 Examination Hall Change

Operational circumstances may require examination hall changes.

Hall changes shall:

- Preserve examination integrity.
- Notify affected participants.
- Maintain complete audit records.

---

## 10.6 Candidate Late Arrival

Institutions may define configurable policies governing late arrival.

Late participation shall comply with institutional examination regulations.

---

## 10.7 Examination Interrupted

Theory examinations interrupted by emergencies may be:

- Suspended
- Resumed
- Rescheduled
- Cancelled

Institutional authorities shall govern continuation decisions.

---

## 10.8 Administrative Override

Authorised officers may perform controlled administrative overrides under exceptional circumstances.

Every override shall:

- Require appropriate authority.
- Record business justification.
- Preserve complete audit history.
- Generate appropriate business events.

# 11. Events Published

The Theory Examination Conduct domain publishes business events representing significant milestones during written examination execution.

These events communicate authoritative business state transitions to downstream workflows while preserving ownership boundaries.

| Event | Published When | Primary Consumers |
|-------|----------------|-------------------|
| Examination Centre Ready | Centre preparation completed | Examination Administration |
| Examination Hall Ready | Hall preparation completed | Examination Operations |
| Invigilator Assigned | Invigilator allocation completed | Audit |
| Seat Assigned | Candidate seat allocation completed | Examination Operations |
| Candidate Checked In | Candidate successfully entered examination | Attendance, Audit |
| Examination Entry Verified | Candidate authorised for examination | Examination Conduct |
| Attendance Recorded | Attendance finalised | L2-I08 |
| Theory Examination Commenced | Examination officially started | Audit |
| Answer Books Distributed | Answer books issued | Audit |
| Theory Examination Completed | Examination concluded | L2-I08 |
| Answer Books Collected | Collection completed | L2-I08 |
| Conduct Verified | Operational verification completed | L2-I08 |
| Conduct Closed | Examination conduct lifecycle completed | Audit |
| Conduct Archived | Conduct records archived | Reporting, Audit |

Business events represent authoritative theory examination conduct lifecycle transitions.

---

# 12. Events Consumed

The Theory Examination Conduct workflow consumes business events published by collaborating domains.

Ownership of business information remains with the publishing domain.

| Event | Published By |
|-------|---------------|
| Examination Policy Updated | L2-I01 |
| Examination Session Activated | L2-I02 |
| Examination Schedule Published | L2-I03 |
| Examination Registration Confirmed | L2-I04 |
| Hall Ticket Published | L2-I05 |
| Hall Ticket Withdrawn | L2-I05 |
| Hall Ticket Cancelled | L2-I05 |
| Candidate Information Updated | Person & Identity Domain |
| Academic Calendar Updated | Academic Calendar Domain |
| Examination Centre Available | Campus & Infrastructure Domain |
| Administrative Approval Granted | Governance Processes |
| Audit Request Raised | Audit Infrastructure |

Consumed events influence examination conduct without transferring ownership.

---

# 13. Multi-Domain Interactions

The Theory Examination Conduct domain collaborates with multiple business domains to execute authorised written examinations.

Business ownership remains with the originating domains.

---

## 13.1 Examination Policy Domain

L2-I01 governs institutional examination policies.

Business interactions include:

- Theory Examination Policies
- Invigilation Policies
- Attendance Policies
- Examination Integrity Policies
- Exception Handling Policies

Theory Examination Conduct consumes policies but does not modify them.

---

## 13.2 Examination Session Management Domain

L2-I02 governs the Examination Session.

Business interactions include:

- Active Examination Sessions
- Session Validity
- Session Lifecycle

Theory examinations shall occur only within active Examination Sessions.

---

## 13.3 Examination Planning & Scheduling Domain

L2-I03 provides approved examination schedules.

Business interactions include:

- Examination Dates
- Time Slots
- Examination Centres
- Hall Allocations
- Published Timetables

Theory Examination Conduct consumes schedules without modifying them.

---

## 13.4 Examination Registration Domain

L2-I04 provides authorised examination participation.

Business interactions include:

- Confirmed Registration
- Registered Theory Subjects
- Candidate Eligibility

Registration ownership remains external to theory examination conduct.

---

## 13.5 Hall Ticket Management Domain

L2-I05 provides examination entry authorisation.

Business interactions include:

- Hall Ticket Validation
- Examination Entry Authorisation
- Hall Ticket Status

Theory Examination Conduct consumes Hall Ticket information but does not modify Hall Ticket governance.

---

## 13.6 Person & Identity Domain

The Person & Identity domain provides candidate identity information.

Business interactions include:

- Candidate Identity
- Identity Verification
- Identity Status

Identity ownership remains outside the Examination domain.

---

## 13.7 Campus & Infrastructure Domain

The Campus & Infrastructure domain provides physical examination facilities.

Business interactions include:

- Examination Centres
- Examination Halls
- Seating Capacity
- Physical Infrastructure

Infrastructure ownership remains external to Theory Examination Conduct.

---

## 13.8 Organisation Domain

The Organisation domain provides institutional organisational structures.

Business interactions include:

- Departments
- Faculties
- Examination Administration
- Administrative Units

Organisation ownership remains external to Theory Examination Conduct.

---

## 13.9 Platform Foundation

Platform Foundation provides enterprise infrastructure supporting theory examination execution.

Supporting services include:

- Authentication
- Authorization
- Notification Workflows
- Event Infrastructure
- Audit Infrastructure

Platform services support business workflows without owning examination conduct.

---

# 14. Ownership Boundaries

The Theory Examination Conduct domain owns the operational execution of authorised written examinations.

It governs examination operations from examination centre preparation until answer books become available for Marks Collection & Evaluation.

---

## Owned by L2-I07

The following business capabilities are owned by this domain:

- Examination Centre Operations
- Examination Hall Management
- Seating Plan Management
- Seat Assignment
- Candidate Check-in
- Examination Entry Verification
- Invigilator Management
- Attendance Management
- Theory Examination Conduct
- Answer Book Collection
- Conduct Verification
- Conduct Lifecycle Governance

---

## Not Owned by L2-I07

The following business capabilities belong to other workflow documents.

| Business Capability | Owning Document |
|---------------------|-----------------|
| Examination Policy Management | L2-I01 |
| Examination Session Management | L2-I02 |
| Examination Planning & Scheduling | L2-I03 |
| Examination Registration | L2-I04 |
| Hall Ticket Management | L2-I05 |
| Practical & Viva Examination Conduct | L2-I06 |
| Marks Collection & Evaluation | L2-I08 |
| Result Processing & Computation | L2-I09 |
| Result Publication | L2-I10 |
| Revaluation & Rechecking | L2-I11 |
| Supplementary / Backlog / Improvement Examinations | L2-I12 |
| Academic Completion & Completion Evidence | L2-I13 |
| Malpractice & Examination Committee | L2-I14 |
| Examination Exception & Contingency Management | L2-I15 |
| Examination Session Closure & Archival | L2-I16 |

This separation maintains clear business ownership across the Examination repository.

---

# 15. Configurable Theory Examination Matrix

Institutions may configure theory examination conduct without modifying the canonical workflow architecture.

| Configuration Area | Examples |
|--------------------|----------|
| Examination Centre Allocation | Fixed, Dynamic |
| Hall Allocation | Manual, Capacity-Based |
| Seating Strategy | Sequential, Random, Alternate |
| Invigilator Allocation | Single, Multiple, Relief |
| Attendance Policy | Mandatory, Grace Period, Exceptions |
| Late Entry Policy | Allowed, Restricted, Prohibited |
| Answer Book Distribution | Institution Configurable |
| Candidate Movement Policy | Restricted, Approval Required |
| Conduct Verification | Mandatory, Optional |
| Notification Policy | Candidate, Invigilator, Administrator |

Configuration changes operational behaviour while preserving repository-standard business workflows.

# 16. Audit Requirements

Every Theory Examination Conduct activity shall generate auditable business records.

Auditability supports institutional governance, regulatory compliance, operational transparency, dispute resolution, and historical traceability.

---

## Auditable Activities

The following business activities shall be auditable:

- Examination Centre Preparation
- Examination Hall Preparation
- Invigilator Assignment
- Seating Plan Generation
- Seat Assignment
- Candidate Check-in
- Examination Entry Verification
- Attendance Recording
- Theory Examination Commencement
- Question Paper Distribution
- Answer Book Distribution
- Examination Supervision
- Answer Book Collection
- Conduct Verification
- Administrative Override
- Conduct Closure
- Conduct Archival

Every significant business transition shall be recorded.

---

## Audit Information

Audit records may include:

- Examination Identifier
- Candidate Identifier
- Examination Session
- Hall Ticket Reference
- Examination Centre
- Examination Hall
- Assigned Seat
- Invigilator Information
- Attendance Status
- Business Activity
- Previous Business State
- New Business State
- Responsible Business Actor
- Business Justification
- Date and Time
- Approval Reference

Audit implementation is provided by the Platform Audit Infrastructure.

---

# 17. Workflow Completion Criteria

A Theory Examination Conduct workflow is considered complete when:

- Examination Centre preparation has been completed.
- Examination Hall preparation has been completed.
- Candidate Check-in has concluded.
- Examination Entry Verification has been completed.
- Attendance has been finalised.
- Theory Examination has been successfully conducted.
- Answer Books have been collected.
- Conduct Verification has been completed.
- Conduct completion events have been published.
- Audit records have been generated.
- Answer Books are available for Marks Collection & Evaluation.

Completion establishes the authoritative operational record of written examination execution.

---

# 18. Theory Examination Conduct Catalogue

The Theory Examination Conduct Catalogue defines the canonical business capabilities owned by this domain.

These capabilities are consumed by downstream examination workflows without redefining theory examination operations.

| Business Capability | Business Purpose | Primary Consumers |
|---------------------|------------------|-------------------|
| Examination Centre Operations | Establish operational readiness | I07 |
| Examination Hall Management | Prepare authorised examination halls | I07 |
| Seating Plan Management | Allocate examination seating | I07 |
| Seat Assignment | Assign candidate seating | I07 |
| Candidate Check-in | Manage examination participation | I07 |
| Examination Entry Verification | Verify examination authorisation | I07 |
| Invigilator Management | Allocate examination supervisors | I07 |
| Attendance Management | Record candidate participation | I08 |
| Theory Examination Conduct | Execute written examinations | I08 |
| Answer Book Collection | Complete examination execution | I08 |
| Conduct Verification | Verify operational completion | I08 |
| Conduct Lifecycle Governance | Govern examination lifecycle | Audit |

Additional capabilities may be introduced through institutional configuration without modifying the canonical workflow architecture.

---

# 19. Theory Examination Dependency Matrix

The Theory Examination Conduct domain depends upon business information owned by collaborating domains.

Ownership remains with the originating business domains.

| Business Capability | Depends Upon |
|---------------------|--------------|
| Examination Centre Operations | Examination Planning (L2-I03), Campus & Infrastructure Domain |
| Hall Management | Campus & Infrastructure Domain |
| Seating Plan Management | Approved Examination Schedule |
| Seat Assignment | Candidate Registration (L2-I04), Hall Ticket (L2-I05) |
| Candidate Check-in | Hall Ticket (L2-I05) |
| Examination Entry Verification | Hall Ticket Management (L2-I05), Person & Identity Domain |
| Invigilator Management | Organisation Domain |
| Attendance Management | Candidate Participation |
| Theory Examination Conduct | Approved Examination Schedule |
| Answer Book Collection | Completed Theory Examination |
| Conduct Verification | Attendance & Answer Book Collection |
| Conduct Archival | Audit Infrastructure |

The dependency matrix establishes business sequencing while preserving ownership boundaries.

---

# 20. Theory Examination Conduct State Model

Every theory examination progresses through a governed operational lifecycle.

| State | Description |
|--------|-------------|
| Planned | Examination prepared for execution |
| Centre Ready | Examination Centre operationally ready |
| Hall Ready | Examination Hall prepared |
| Seating Ready | Seating allocation completed |
| Check-in | Candidate entry in progress |
| Verified | Examination entry successfully verified |
| In Progress | Theory examination currently underway |
| Suspended | Examination temporarily interrupted |
| Completed | Examination successfully concluded |
| Verified for Closure | Conduct verification completed |
| Closed | Operational lifecycle completed |
| Archived | Historical conduct record retained |

Business rules governing state transitions:

- Every examination begins in **Planned**.
- Prepared examinations become **Centre Ready**.
- Hall preparation establishes **Hall Ready**.
- Seating allocation establishes **Seating Ready**.
- Candidate participation begins during **Check-in**.
- Successful examination entry establishes **Verified**.
- Active examinations remain **In Progress** until operational completion.
- Interrupted examinations may enter **Suspended** before resuming.
- Successfully completed examinations become **Completed**.
- Administrative verification establishes **Verified for Closure**.
- Verified examinations become **Closed**.
- Closed examinations progress to **Archived**.

---

# 21. Configurable Parameters Catalogue

Institutions may configure Theory Examination Conduct while preserving repository-standard workflows.

| Configuration Category | Examples |
|------------------------|----------|
| Examination Centre Allocation | Fixed, Dynamic |
| Hall Allocation | Capacity-Based, Department-Based |
| Seating Strategy | Sequential, Random, Alternate |
| Seat Reservation | Special Accommodation Policies |
| Invigilator Allocation | Single, Multiple, Relief |
| Attendance Rules | Mandatory, Grace Period, Exceptions |
| Late Entry Policy | Allowed, Restricted, Prohibited |
| Candidate Movement | Restricted, Approval Required |
| Conduct Verification | Mandatory, Optional |
| Notification Behaviour | Candidate, Invigilator, Administrator Alerts |

Institution-specific values may vary without changing workflow architecture.

---

# 22. Domain Responsibility Summary

The Theory Examination Conduct domain governs the operational execution of authorised written examinations.

It establishes the authoritative operational record of theory examination participation before Marks Collection & Evaluation begins.

| Business Capability | Owned by L2-I07 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Examination Centre Operations | ✓ | Campus & Infrastructure provides facilities |
| Examination Hall Management | ✓ | Campus & Infrastructure owns halls |
| Seating Plan Management | ✓ | — |
| Seat Assignment | ✓ | — |
| Candidate Check-in | ✓ | — |
| Examination Entry Verification | ✓ | L2-I05 owns Hall Ticket lifecycle |
| Invigilator Management | ✓ | — |
| Attendance Management | ✓ | — |
| Theory Examination Conduct | ✓ | — |
| Answer Book Collection | ✓ | — |
| Conduct Verification | ✓ | — |
| Marks Collection & Evaluation | — | L2-I08 |
| Result Processing | — | L2-I09 |
| Result Publication | — | L2-I10 |
| Revaluation & Rechecking | — | L2-I11 |
| Supplementary / Backlog / Improvement Examinations | — | L2-I12 |
| Academic Completion & Completion Evidence | — | L2-I13 |
| Malpractice Management | — | L2-I14 |
| Examination Exception Management | — | L2-I15 |
| Examination Session Closure & Archival | — | L2-I16 |

The Theory Examination Conduct domain governs operational execution only and does not own academic evaluation, marks processing, or examination outcomes.

---

# 23. Repository Integration Summary

L2-I07 establishes the canonical business workflows governing Theory Examination Conduct within the SutraERP Examination domain.

Every written examination shall be conducted through the workflows defined in this document.

The relationship between Examination workflow documents is as follows:

| Document | Responsibility |
|----------|----------------|
| L2-I01 | Examination Policy & Configuration |
| L2-I02 | Examination Session Management |
| L2-I03 | Examination Planning & Scheduling |
| L2-I04 | Examination Registration |
| L2-I05 | Hall Ticket Management |
| L2-I06 | Practical & Viva Examination Conduct |
| **L2-I07** | **Theory Examination Conduct** |
| L2-I08 | Marks Collection & Evaluation |
| L2-I09 | Result Processing & Computation |
| L2-I10 | Result Publication |
| L2-I11 | Revaluation & Rechecking |
| L2-I12 | Supplementary / Backlog / Improvement Examination Execution |
| L2-I13 | Academic Completion, CGPA & Completion Evidence Workflows |
| L2-I14 | Malpractice & Examination Committee |
| L2-I15 | Examination Exception & Contingency Management |
| L2-I16 | Examination Session Closure & Archival |

L2-I07 consumes authorised examination participation established by L2-I05 and produces authoritative theory examination conduct records consumed by L2-I08.

The domain governs written examination conduct only and does not own question paper creation, evaluation, marks processing, or academic outcome determination.

---

# Appendix A — Theory Examination Conduct Business Reference Model

This appendix provides the canonical business reference model for Theory Examination Conduct.

The appendix defines the conceptual structure of written examination operations without introducing additional business ownership beyond the main document.

---

# A.1 Theory Examination Conduct Business Model

The Theory Examination Conduct domain governs the operational execution of authorised written examinations.

The canonical business model is illustrated below.

```text
Examination Session
        │
        ▼
Approved Examination Schedule
        │
        ▼
Authorised Hall Ticket
        │
        ▼
Examination Centre
        │
        ▼
Examination Hall
        │
        ▼
Seating Plan
        │
        ▼
Candidate Check-in
        │
        ▼
Examination Entry Verification
        │
        ▼
Attendance Recording
        │
        ▼
Theory Examination Conduct
        │
        ▼
Answer Book Collection
        │
        ▼
Answer Book Verification
        │
        ▼
Ready for Marks Collection & Evaluation
```

The business model represents operational examination execution only.

---

# A.2 Examination Centre & Hall Framework

Theory examinations shall be conducted only within authorised examination facilities.

The operational framework consists of:

```text
Examination Centre
        │
        ├── Building (Optional)
        │
        ├── Floor (Optional)
        │
        ├── Examination Hall
        │
        ├── Seating Capacity
        │
        └── Operational Resources
```

Institutions may organise centres differently while preserving the canonical workflow.

This document governs operational usage of examination facilities.

Ownership of physical infrastructure remains with the Campus & Infrastructure domain.

---

# A.3 Seating & Candidate Participation Framework

Candidate participation follows a controlled operational sequence.

```text
Authorised Hall Ticket
        │
        ▼
Candidate Check-in
        │
        ▼
Seat Assignment
        │
        ▼
Examination Entry Verification
        │
        ▼
Attendance Recording
        │
        ▼
Theory Examination Participation
```

Each authorised candidate shall occupy one assigned examination seat for each scheduled theory examination unless institutionally approved exceptions apply.

Seat assignment supports examination integrity and operational governance.

---

# A.4 Invigilation Framework

Theory examinations shall operate under authorised supervision.

The canonical operational framework is:

```text
Examination Superintendent
        │
        ▼
Chief Invigilator
        │
        ▼
Hall Invigilators
        │
        ▼
Relief Invigilators (Optional)
        │
        ▼
Observers (Optional)
```

Institutions may configure invigilation structures according to operational requirements.

This document governs invigilation during examination conduct only.

---

# A.5 Answer Book Operational Framework

Answer books represent the official written responses submitted during theory examinations.

The operational lifecycle is:

```text
Answer Book Distribution
        │
        ▼
Candidate Response
        │
        ▼
Answer Book Submission
        │
        ▼
Answer Book Collection
        │
        ▼
Collection Verification
        │
        ▼
Packaging & Sealing (Institution Policy)
        │
        ▼
Handover for Marks Collection & Evaluation
```

This document governs operational custody until formal handover.

Academic evaluation begins only after successful handover to L2-I08.

---

# A.6 Ownership Summary

The Theory Examination Conduct domain owns:

- Examination Centre Operations
- Examination Hall Operations
- Seating Plan Management
- Seat Assignment
- Candidate Check-in
- Examination Entry Verification
- Invigilator Management
- Attendance Management
- Theory Examination Execution
- Answer Book Collection
- Answer Book Verification
- Operational Handover
- Conduct Completion
- Conduct Lifecycle Governance

The domain does **not** own:

- Examination Policies
- Examination Sessions
- Examination Scheduling
- Examination Registration
- Hall Ticket Lifecycle
- Question Paper Authoring
- Question Paper Moderation
- Question Paper Printing
- Marks Collection
- Answer Evaluation
- Result Processing
- Academic Decisions

---

# A.7 Governing Principles

The Theory Examination Conduct domain is governed by the following principles:

- Only authorised candidates may participate.
- Every examination shall occur within an active Examination Session.
- Every candidate shall receive an authorised examination seat.
- Examination entry shall be verified before participation.
- Attendance forms part of examination conduct.
- Every theory examination shall be supervised by authorised invigilators.
- Answer books shall remain under controlled operational custody until formal handover.
- Marks Collection & Evaluation begins only after operational handover.
- Operational conduct and academic evaluation remain separate business responsibilities.
- Every significant business activity shall remain fully auditable.

---

**Appendix A completes the business reference model for L2-I07 and aligns it structurally with L2-I04, L2-I05, and L2-I06.**

# End of Document







