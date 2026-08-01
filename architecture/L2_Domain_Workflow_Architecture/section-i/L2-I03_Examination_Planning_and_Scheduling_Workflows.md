# L2-I03 — Examination Planning & Scheduling Workflows

**Level:** L2 – Business Workflow Architecture

**Domain:** I – Examination

**Status:** Draft

**Owner:** Examination Domain

**Document Owner:** Devang

---

# Document Control

| Item | Value |
|------|-------|
| Document ID | L2-I03 |
| Document Name | Examination Planning & Scheduling Workflows |
| Domain | Examination |
| Level | L2 – Business Workflow Architecture |
| Owner | Examination Domain |
| Depends On | L2-I01 Examination Policy & Configuration, L2-I02 Examination Session Management, A01 Platform Foundation, A02 Person & Identity, A03 Authentication, A04 Authorization, H02 Notification Workflows, A06 Event Infrastructure, A07 Audit Infrastructure, B01 Institution, B02 Organisation, B03 Campus & Infrastructure, B04 Curriculum Framework, B05 Academic Calendar |
| Consumed By | L2-I04 through L2-I16 Examination Workflows |
| Canonical Owner | Examination Domain |

---

# 1. Purpose

This document defines the business workflows governing Examination Planning and Scheduling within SutraERP.

Examination Planning transforms an approved Examination Session into an executable examination schedule by determining examination dates, time slots, venues, resources, and operational sequencing.

This document establishes how examinations are planned, validated, approved, published, revised, and governed before candidate registration and examination execution begin.

Operational workflows such as candidate registration, hall ticket generation, examination conduct, evaluation, and result processing consume approved examination schedules produced by this document.

This document defines business workflows only.

No implementation, database design, scheduling algorithms, optimisation techniques, APIs, user interface behaviour, storage models, or technology architecture are included.

---

# 2. Scope

This document includes:

- Examination planning
- Examination timetable planning
- Examination scheduling
- Examination slot planning
- Venue planning
- Room allocation planning
- Invigilator planning
- Resource planning
- Schedule validation
- Clash detection
- Schedule approval
- Schedule publication
- Schedule revision
- Schedule cancellation
- Schedule version management

This document excludes:

- Examination Session creation
- Examination Policy management
- Candidate registration
- Hall ticket generation
- Practical and viva conduct
- Theory examination execution
- Attendance management
- Marks collection
- Evaluation
- Result processing
- Graduation processing

These capabilities are defined in their respective Examination workflow documents.

---

# 3. Repository Context

L2-I03 establishes the canonical business workflows for Examination Planning and Scheduling.

Every operational examination activity requiring dates, venues, time slots, and resource allocation shall consume an approved Examination Schedule produced by this document.

Examination Planning consumes approved Examination Sessions from L2-I02 and transforms them into institutionally approved operational schedules.

This document collaborates with:

- L2-I01 for examination policies.
- L2-I02 for approved examination sessions.
- Academic Calendar for examination windows.
- Curriculum Framework for programme and course structures.
- Campus & Infrastructure for venue and facility information.
- Organisation for administrative ownership.
- Platform Foundation for identity, authorization, notifications, events, and audit.

Operational workflows consume approved schedules but shall not modify planning governance.

# 4. Architectural Principles

The Examination Planning & Scheduling workflows are governed by the following business principles.

These principles ensure examination schedules remain governed, conflict-free, configurable, auditable, and independent of examination execution.

---

## 4.1 Session-Driven Planning

Every Examination Plan shall belong to one approved Examination Session.

Planning activities shall not exist independently of an approved Examination Session defined in L2-I02.

All examination schedules inherit their business governance from the associated Examination Session.

---

## 4.2 Planning Before Operations

Operational examination activities shall only commence after examination planning has been completed and approved.

This includes:

- Candidate Registration
- Hall Ticket Generation
- Practical Examination
- Theory Examination
- Marks Collection
- Result Processing

Planning establishes the operational blueprint consumed by downstream workflows.

---

## 4.3 Single Source of Schedule

Each examination schedule shall represent the authoritative operational timetable for the applicable Examination Session.

All downstream examination workflows shall consume the approved schedule rather than maintaining independent scheduling information.

---

## 4.4 Conflict-Free Scheduling

The planning process shall minimise operational conflicts before schedule approval.

Business validation may include:

- Student examination clashes
- Faculty examination clashes
- Venue conflicts
- Room capacity conflicts
- Infrastructure availability
- Academic Calendar conflicts

Conflict resolution remains part of planning governance.

---

## 4.5 Resource-Oriented Planning

Examination planning shall consider institutional resources including:

- Examination Venues
- Examination Rooms
- Laboratories
- Computer Centres
- Invigilators
- Supervisors
- Support Staff

Resource ownership remains with collaborating domains.

---

## 4.6 Institutional Flexibility

The planning architecture shall support different institutional scheduling models through configuration.

Examples include:

- Single Shift Examinations
- Double Shift Examinations
- Multiple Daily Sessions
- Centralised Scheduling
- Department-Level Scheduling
- Campus-Specific Scheduling

Operational workflows remain unchanged while planning configuration remains institution-specific.

---

## 4.7 Controlled Schedule Lifecycle

Every Examination Schedule shall progress through a governed business lifecycle.

Typical lifecycle stages include:

- Draft
- Under Review
- Approved
- Published
- Active
- Revised
- Completed
- Archived

Only approved schedules may be published for operational use.

---

## 4.8 Version Integrity

Schedule revisions shall preserve historical examination records.

Previously published schedules remain available for audit and historical reference even after subsequent revisions are approved.

Version management ensures operational consistency throughout the examination lifecycle.

---

## 4.9 Multi-Domain Collaboration

Examination Planning collaborates with multiple business domains.

Examples include:

| Domain | Business Dependency |
|---------|---------------------|
| Examination Session | Governing examination cycle |
| Academic Calendar | Examination windows |
| Curriculum Framework | Courses and programme structures |
| Campus & Infrastructure | Venues and rooms |
| Organisation | Administrative ownership |
| Person & Identity | Faculty and staff |
| Notification Workflows | Schedule publication |
| Audit Infrastructure | Planning audit trail |

Business ownership remains with the originating domains.

---

## 4.10 Complete Auditability

Every planning activity shall generate auditable business records.

Examples include:

- Schedule Creation
- Timetable Changes
- Venue Changes
- Resource Allocation
- Approval
- Publication
- Revision
- Cancellation

Audit implementation is provided through the Platform Audit Infrastructure.

---

## 4.11 Configuration Before Customisation

Institution-specific planning behaviour shall be achieved through configurable planning parameters.

Business workflows shall remain repository standard across all supported institutions.

Custom workflow implementation shall only be considered where institutional requirements cannot be satisfied through configuration.

---

# 5. Business Capabilities

The Examination Planning & Scheduling domain provides the business capabilities required to transform approved Examination Sessions into executable operational examination schedules.

---

## 5.1 Examination Planning

The platform shall support preparation of examination plans for approved Examination Sessions.

Planning establishes the operational framework required for examination execution.

---

## 5.2 Timetable Planning

The platform shall support preparation of examination timetables.

Planning may include:

- Examination Dates
- Examination Days
- Time Slots
- Examination Duration
- Shift Planning

Timetable planning establishes the chronological execution sequence.

---

## 5.3 Venue Planning

The platform shall support allocation of examination venues.

Examples include:

- Buildings
- Examination Halls
- Classrooms
- Laboratories
- Computer Centres
- External Examination Centres

Venue ownership remains within the Campus & Infrastructure domain.

---

## 5.4 Room Allocation Planning

Examinations may be distributed across one or more rooms.

Planning considers:

- Room Capacity
- Infrastructure Availability
- Accessibility
- Examination Requirements

Operational seating arrangements are outside the scope of this document.

---

## 5.5 Resource Planning

The platform shall support planning of examination resources.

Resources may include:

- Invigilators
- Supervisors
- Coordinators
- Technical Staff
- Support Personnel
- Examination Materials

Assignment governance is established during planning while operational execution occurs during examination conduct.

---

## 5.6 Conflict Validation

Planning workflows shall support validation of scheduling conflicts before approval.

Typical validations include:

- Student timetable conflicts
- Faculty conflicts
- Venue conflicts
- Resource conflicts
- Academic Calendar conflicts

Conflict resolution remains a planning responsibility.

---

## 5.7 Schedule Approval

Completed examination plans shall undergo institutional review.

Approval confirms:

- Academic correctness
- Resource availability
- Institutional compliance
- Policy compliance
- Operational readiness

Only approved schedules proceed to publication.

---

## 5.8 Schedule Publication

Approved examination schedules may be published for institutional consumption.

Publication authorises downstream workflows including:

- Candidate Registration
- Hall Ticket Generation
- Examination Conduct
- Operational Coordination

Publication represents the official examination timetable.

---

## 5.9 Schedule Revision

Institutions may revise approved examination schedules when necessary.

Revision may occur due to:

- Administrative Decisions
- Academic Changes
- Emergency Situations
- Regulatory Directives
- Infrastructure Constraints

Revision shall preserve historical schedule versions.

---

## 5.10 Schedule Cancellation

Institutions may cancel planned examination schedules under approved governance.

Cancellation terminates the affected schedule while preserving complete audit history.

Alternative planning workflows may subsequently be initiated.

# 6. Business Actors

The following business actors participate in Examination Planning & Scheduling workflows.

| Actor | Responsibility |
|--------|----------------|
| Examination Administrator | Prepares and manages examination plans and schedules |
| Controller of Examination | Reviews, approves and governs examination schedules |
| Academic Administration | Validates academic applicability and examination sequencing |
| Department Administration | Coordinates departmental examination planning |
| Campus Administration | Coordinates venue availability and campus resources |
| Infrastructure Administration | Provides examination venues and facilities |
| Faculty Administration | Coordinates invigilator and supervisory resource planning |
| Organisation Domain | Provides organisational ownership and governance |
| Academic Calendar Domain | Provides examination windows and academic periods |
| Curriculum Framework Domain | Provides programme, course and curriculum applicability |
| Audit Infrastructure | Maintains complete audit records of planning activities |

Students are consumers of published examination schedules but do not participate in schedule planning or approval.

---

# 7. Examination Planning Framework

The Examination Planning Framework establishes the business structure used to prepare institutionally governed examination schedules.

Planning transforms approved Examination Sessions into executable operational schedules.

---

## 7.1 Planning Objectives

The framework enables institutions to:

- Prepare examination schedules.
- Organise examinations across academic programmes.
- Allocate institutional resources.
- Prevent operational conflicts.
- Maintain examination governance.
- Support schedule revisions.
- Ensure institutional auditability.

Planning defines operational readiness but does not execute examinations.

---

## 7.2 Planning Dimensions

Every Examination Plan shall consider one or more planning dimensions.

| Planning Dimension | Purpose |
|--------------------|---------|
| Examination Session | Governing examination cycle |
| Academic Period | Examination window |
| Programme Applicability | Participating academic programmes |
| Course Applicability | Courses being examined |
| Examination Mode | Theory, Practical, Viva or Combined |
| Schedule Structure | Dates, slots and sequence |
| Venue Allocation | Buildings, halls, laboratories and centres |
| Resource Allocation | Invigilators, supervisors and support staff |
| Administrative Ownership | Responsible academic authorities |

Multiple planning dimensions combine to produce a complete examination schedule.

---

## 7.3 Timetable Planning

Timetable planning determines when examinations occur.

Planning activities include:

- Examination Dates
- Examination Days
- Time Slots
- Shift Allocation
- Examination Duration
- Examination Sequence

Detailed examination execution remains outside this document.

---

## 7.4 Venue Planning Framework

Venue planning identifies institutional locations where examinations are conducted.

Venue planning may include:

- Campus
- Building
- Examination Hall
- Classroom
- Laboratory
- Computer Centre
- External Examination Centre

Venue ownership remains within the Campus & Infrastructure domain.

---

## 7.5 Resource Planning Framework

Resource planning determines institutional resources required for examination execution.

Examples include:

- Invigilators
- Chief Superintendent
- Observers
- Supervisors
- Technical Staff
- Laboratory Assistants
- Support Personnel

Operational deployment occurs during examination conduct.

---

## 7.6 Examination Mode Planning

Planning shall support multiple examination modes.

Examples include:

| Examination Mode | Planning Characteristics |
|------------------|--------------------------|
| Theory | Written examination timetable |
| Practical | Laboratory scheduling |
| Viva | Examiner availability scheduling |
| Project Evaluation | Evaluation schedule |
| Combined | Multiple coordinated examination modes |

Operational execution remains outside planning.

---

## 7.7 Schedule Publication Readiness

An Examination Schedule shall be considered ready for publication only after:

- Planning is complete.
- Business validation succeeds.
- Resource availability is confirmed.
- Institutional approval is obtained.
- Conflict resolution is completed.

Publication authorises downstream operational workflows.

---

# 8. Examination Scheduling Lifecycle

Every Examination Schedule progresses through a governed business lifecycle.

The lifecycle establishes planning governance before examination execution begins.

---

## 8.1 Lifecycle Overview

The standard Examination Scheduling lifecycle consists of the following stages:

1. Planning Initiation
2. Timetable Preparation
3. Resource Planning
4. Conflict Validation
5. Administrative Review
6. Schedule Approval
7. Schedule Publication
8. Operational Consumption
9. Schedule Revision (Optional)
10. Schedule Cancellation (Optional)
11. Schedule Completion
12. Schedule Archival

Each stage represents a governed business transition.

---

## 8.2 Planning Initiation

Planning begins after an Examination Session becomes Active.

Business activities include:

- Identifying examination scope
- Determining participating programmes
- Identifying applicable courses
- Defining planning boundaries
- Assigning planning responsibility

No candidate-facing activities occur during this stage.

---

## 8.3 Timetable Preparation

The examination timetable is prepared by determining:

- Examination dates
- Time slots
- Examination duration
- Examination sequence
- Shift allocation

Timetable preparation establishes the operational examination calendar.

---

## 8.4 Resource Planning

Required institutional resources are identified and planned.

Planning may include:

- Venues
- Rooms
- Laboratories
- Invigilators
- Supervisors
- Technical Staff

Planning confirms operational readiness without assigning execution responsibilities.

---

## 8.5 Conflict Validation

Prepared schedules undergo institutional validation.

Validation may identify:

- Student examination clashes
- Faculty scheduling conflicts
- Venue overlaps
- Room capacity issues
- Academic Calendar inconsistencies
- Resource shortages

Detected conflicts shall be resolved before approval.

---

## 8.6 Administrative Review

Completed examination schedules undergo institutional review.

Review verifies:

- Academic correctness
- Operational feasibility
- Resource readiness
- Institutional compliance
- Policy alignment

Only reviewed schedules may proceed for approval.

---

## 8.7 Schedule Approval

Institutional approval confirms that the examination schedule is authorised for publication.

Approval establishes the official operational examination timetable.

---

## 8.8 Schedule Publication

Approved schedules become available for downstream examination workflows.

Publication authorises:

- Candidate Registration
- Hall Ticket Generation
- Operational Examination Coordination

Published schedules become the authoritative institutional timetable.

---

## 8.9 Schedule Revision

Institutions may revise published schedules under approved governance.

Typical reasons include:

- Administrative changes
- Infrastructure issues
- Regulatory directives
- Emergency situations
- Academic revisions

Every revision shall preserve complete historical traceability.

---

## 8.10 Schedule Cancellation

An examination schedule may be cancelled under institutional authority.

Cancellation terminates future operational use while preserving complete audit history.

---

## 8.11 Schedule Completion

An examination schedule is considered complete after all planned examinations have concluded.

Completion confirms the end of planning responsibilities.

---

## 8.12 Schedule Archival

Completed schedules enter archival status.

Archived schedules remain available for:

- Historical reference
- Regulatory compliance
- Institutional reporting
- Audit
- Long-term record retention

Archived schedules remain immutable from a business governance perspective.

# 9. Business Rules

The following business rules govern Examination Planning & Scheduling across all institutions using SutraERP.

Business rules establish planning governance while remaining independent of implementation.

---

## 9.1 Planning Eligibility Rules

- Examination Planning shall begin only after an Examination Session becomes Active.
- Every Examination Plan shall belong to exactly one Examination Session.
- Planning shall be performed only within approved academic periods.
- Examination Planning shall comply with the governing Examination Policy.
- Unapproved Examination Sessions shall not enter the planning workflow.

---

## 9.2 Timetable Rules

Every examination included within an Examination Plan shall define:

- Examination Date
- Examination Time Slot
- Examination Duration
- Applicable Programme
- Applicable Course
- Examination Mode

Incomplete timetable definitions shall not proceed for approval.

---

## 9.3 Resource Allocation Rules

Every planned examination shall reference appropriate institutional resources.

Examples include:

- Examination Venue
- Examination Room
- Laboratory
- Computer Centre
- Invigilators
- Supervisors

Resource ownership remains with collaborating domains.

---

## 9.4 Conflict Validation Rules

Before approval, examination schedules shall be validated for operational conflicts.

Validation may include:

- Student examination clashes
- Faculty conflicts
- Venue conflicts
- Room allocation conflicts
- Infrastructure conflicts
- Academic Calendar conflicts
- Examination Session conflicts

Unresolved conflicts shall prevent schedule approval.

---

## 9.5 Approval Rules

An Examination Schedule may be approved only when:

- Planning activities are complete.
- Conflict validation succeeds.
- Resource availability is confirmed.
- Institutional governance requirements are satisfied.
- Administrative review has concluded.

Only approved schedules may be published.

---

## 9.6 Publication Rules

Published examination schedules become the official institutional timetable.

Following publication:

- Candidate Registration may begin.
- Hall Ticket workflows may consume the schedule.
- Operational examination workflows may reference published schedules.

Publication shall preserve schedule version integrity.

---

## 9.7 Revision Rules

Schedule revisions shall follow controlled governance.

Revision may occur due to:

- Academic changes
- Administrative decisions
- Emergency situations
- Infrastructure constraints
- Regulatory directives

Historical schedule versions shall remain available for audit purposes.

---

## 9.8 Cancellation Rules

Cancelled schedules:

- Shall not participate in future operational workflows.
- Shall preserve complete historical records.
- Shall remain available for institutional audit.

Cancellation shall not remove previously published historical information.

---

## 9.9 Audit Rules

Every significant planning activity shall generate audit records.

Examples include:

- Plan Creation
- Timetable Changes
- Venue Changes
- Resource Changes
- Conflict Resolution
- Approval
- Publication
- Revision
- Cancellation

---

# 10. Exception Scenarios

The Examination Planning & Scheduling workflows shall support controlled handling of exceptional business situations.

---

## 10.1 Missing Examination Session

Planning shall not begin if no approved Examination Session exists.

Planning remains blocked until an eligible session becomes Active.

---

## 10.2 Academic Calendar Conflict

If proposed examination dates conflict with the Academic Calendar:

- Planning review shall identify the conflict.
- Schedule approval shall not proceed.
- Administrative resolution shall be required.

---

## 10.3 Resource Unavailability

Where required resources become unavailable:

Examples include:

- Examination Hall
- Laboratory
- Computer Centre
- Invigilator
- Technical Staff

Planning shall require reassignment before approval.

---

## 10.4 Venue Capacity Conflict

If planned candidate volume exceeds venue capacity:

- Capacity validation shall fail.
- Alternative planning shall be required.
- Schedule approval shall remain pending.

---

## 10.5 Examination Clash

Conflict validation may identify:

- Student timetable clashes
- Faculty timetable clashes
- Programme scheduling conflicts
- Shared resource conflicts

Conflicts shall be resolved before schedule publication.

---

## 10.6 Approval Rejection

Institutional reviewers may reject an Examination Schedule.

Rejected schedules may:

- Return for modification.
- Undergo replanning.
- Be cancelled.
- Be replaced by a revised schedule.

Rejected schedules shall not be published.

---

## 10.7 Emergency Rescheduling

Approved schedules may require revision due to:

- Natural disasters
- Government directives
- Institutional emergencies
- Public health situations
- Infrastructure failures

Emergency revisions shall follow institutional governance.

---

## 10.8 Publication Withdrawal

Institutions may withdraw a published schedule.

Withdrawal requires:

- Administrative approval.
- Audit recording.
- Controlled communication to affected stakeholders.

Subsequent planning shall generate a replacement schedule where required.

---

## 10.9 Revision After Publication

Published schedules may undergo controlled revisions.

Revised schedules shall:

- Preserve historical versions.
- Notify downstream operational workflows.
- Maintain complete audit continuity.

---

# 11. Events Published

The Examination Planning & Scheduling domain publishes business events for downstream workflow consumption.

| Event | Published When | Primary Consumers |
|-------|----------------|-------------------|
| Examination Planning Initiated | Planning begins | I04 |
| Examination Schedule Created | Initial schedule prepared | I04, I05 |
| Schedule Updated | Planning changes completed | I04, I05 |
| Conflict Validation Completed | Validation process concludes | Governance Workflows |
| Schedule Approved | Institutional approval granted | I04 |
| Schedule Published | Official timetable released | I04–I10 |
| Schedule Revised | Approved revision completed | I04–I10 |
| Schedule Cancelled | Schedule withdrawn | All Operational Examination Workflows |
| Schedule Archived | Planning lifecycle completed | Reporting, Audit |

Business events represent significant planning milestones.

---

# 12. Events Consumed

The Examination Planning & Scheduling workflows consume business events published by collaborating domains.

| Event | Published By |
|-------|---------------|
| Examination Session Activated | L2-I02 |
| Examination Policy Updated | L2-I01 |
| Academic Calendar Updated | Academic Calendar Domain |
| Curriculum Updated | Curriculum Framework Domain |
| Campus Infrastructure Updated | Campus & Infrastructure Domain |
| Resource Availability Updated | Organisation / Infrastructure Domains |
| Administrative Approval Granted | Governance Processes |
| Audit Request Raised | Audit Infrastructure |

Consumed events influence planning decisions while ownership remains with the originating business domain.

# 13. Multi-Domain Interactions

The Examination Planning & Scheduling domain collaborates with multiple business domains to produce institutionally governed examination schedules.

Ownership of business information always remains with the originating domain.

---

## 13.1 Examination Policy Domain

L2-I01 provides the policies governing examination planning.

Business interactions include:

- Examination planning regulations
- Examination mode policies
- Scheduling constraints
- Institutional examination rules
- Examination governance

The Planning domain consumes policies but does not modify them.

---

## 13.2 Examination Session Management Domain

L2-I02 provides the approved Examination Session within which planning occurs.

Business interactions include:

- Active Examination Sessions
- Session applicability
- Session lifecycle
- Session governance
- Session validity

Planning cannot exist independently of an approved Examination Session.

---

## 13.3 Academic Calendar Domain

The Academic Calendar defines the academic periods and examination windows used during planning.

Business interactions include:

- Academic Year
- Semester
- Trimester
- Examination Window
- Institutional Holidays
- Academic Events

The Examination Planning domain consumes calendar information without owning it.

---

## 13.4 Curriculum Framework Domain

The Curriculum Framework provides the academic structure required for examination planning.

Business interactions include:

- Programmes
- Courses
- Curriculum Versions
- Semester Structures
- Credit Structures

Curriculum ownership remains external to the Examination domain.

---

## 13.5 Campus & Infrastructure Domain

The Campus & Infrastructure domain provides examination facilities.

Business interactions include:

- Examination Centres
- Buildings
- Examination Halls
- Classrooms
- Laboratories
- Computer Centres
- Seating Capacity

Facility ownership remains within the Campus & Infrastructure domain.

---

## 13.6 Organisation Domain

The Organisation domain provides administrative ownership for planning activities.

Business interactions include:

- Faculty
- Departments
- Schools
- Administrative Units
- Examination Authorities

Organisation structures remain externally governed.

---

## 13.7 Person & Identity Domain

Person & Identity provides business actors participating in examination planning.

Business interactions include:

- Examination Administrators
- Controllers of Examination
- Faculty Members
- Invigilators
- Supervisors
- Technical Staff

Personnel information remains owned by the Identity domain.

---

## 13.8 Platform Foundation

Platform Foundation provides enterprise infrastructure supporting planning workflows.

Collaborating services include:

- Authentication
- Authorization
- Notification Workflows
- Event Infrastructure
- Audit Infrastructure

Platform services provide infrastructure without owning examination planning business logic.

---

# 14. Ownership Boundaries

The Examination Planning & Scheduling domain owns examination planning governance.

Operational examination execution remains outside the scope of this document.

---

## Owned by L2-I03

The following business capabilities are owned by this domain:

- Examination Planning
- Timetable Planning
- Schedule Preparation
- Examination Slot Planning
- Venue Planning
- Room Allocation Planning
- Resource Planning
- Conflict Validation
- Schedule Review
- Schedule Approval
- Schedule Publication
- Schedule Revision
- Schedule Cancellation
- Schedule Version Management

---

## Not Owned by L2-I03

The following business capabilities belong to other Examination workflow documents.

| Business Capability | Owning Document |
|---------------------|-----------------|
| Examination Policy Governance | L2-I01 |
| Examination Session Management | L2-I02 |
| Candidate Registration | L2-I04 |
| Hall Ticket Management | L2-I05 |
| Practical & Viva Examination Conduct | L2-I06 |
| Theory Examination Conduct | L2-I07 |
| Marks Collection & Evaluation | L2-I08 |
| Result Processing | L2-I09 |
| Result Publication | L2-I10 |
| Revaluation & Rechecking | L2-I11 |
| Supplementary & Improvement Examinations | L2-I12 |
| Academic Completion & Completion Evidence | L2-I13 |
| Malpractice Management | L2-I14 |
| Examination Exception Management | L2-I15 |
| Examination Session Closure & Archival | L2-I16 |

This separation maintains clear business ownership across the Examination domain.

---

# 15. Configurable Planning Matrix

Institutions may configure Examination Planning according to their academic regulations while preserving the standard workflow architecture.

| Configuration Area | Examples |
|--------------------|----------|
| Planning Model | Centralised, Faculty-wise, Department-wise |
| Timetable Structure | Single Shift, Double Shift, Multiple Daily Sessions |
| Examination Duration | Institution-specific durations |
| Slot Configuration | Morning, Afternoon, Evening |
| Venue Allocation | Campus, Building, Hall, Laboratory |
| Resource Planning | Invigilator Planning Rules, Supervisor Requirements |
| Conflict Validation | Student, Faculty, Venue, Resource Rules |
| Approval Workflow | Single-Level or Multi-Level Approval |
| Publication Rules | Immediate or Scheduled Publication |
| Revision Rules | Institution-specific Change Management Policies |

Configuration affects planning behaviour while preserving the repository-standard workflow.

---

# 16. Audit Requirements

Every Examination Planning activity shall generate auditable business records.

Auditability supports institutional governance, regulatory compliance, and historical traceability.

---

## Auditable Activities

The following business activities shall be auditable:

- Planning Initiation
- Timetable Preparation
- Venue Allocation
- Room Allocation
- Resource Planning
- Conflict Validation
- Administrative Review
- Schedule Approval
- Schedule Publication
- Schedule Revision
- Schedule Cancellation
- Schedule Archival
- Administrative Override

---

## Audit Information

Audit records may include:

- Business Activity
- Date and Time
- Responsible Business Actor
- Examination Session
- Examination Schedule Identifier
- Previous Business State
- New Business State
- Approval Reference
- Business Justification

Audit implementation is provided by the Platform Audit Infrastructure.

---

# 17. Workflow Completion Criteria

An Examination Planning & Scheduling workflow is considered complete when:

- An approved Examination Session has been consumed.
- Examination planning has been completed.
- Timetable preparation has been finalised.
- Venue and resource planning have been completed.
- Conflict validation has succeeded.
- Administrative review has concluded.
- Institutional approval has been obtained.
- Examination Schedule has been published or formally cancelled.
- All planning activities have been audited.
- The approved Examination Schedule is available for downstream operational workflows.

Completion of this workflow establishes the authoritative operational examination timetable for the associated Examination Session.

# 18. Examination Planning Catalogue

The Examination Planning Catalogue defines the canonical planning capabilities owned by the Examination Planning & Scheduling domain.

Operational examination workflows consume these planning artefacts without redefining planning responsibilities.

| Planning Capability | Business Purpose | Primary Consumers |
|---------------------|------------------|-------------------|
| Examination Timetable | Defines official examination dates and timings | I04–I10 |
| Examination Slot Plan | Defines examination shifts and time slots | I04–I10 |
| Venue Plan | Defines examination centres and venues | I05, I06, I07 |
| Room Allocation Plan | Defines room-level examination allocation | I05, I06, I07 |
| Resource Plan | Defines examination personnel requirements | I06, I07 |
| Invigilator Planning | Plans invigilator requirements | I07 |
| Laboratory Planning | Plans laboratory examinations | I06 |
| Computer Centre Planning | Plans computer-based examinations | I06, I07 |
| Schedule Revision Plan | Governs approved timetable revisions | I04–I10 |
| Schedule Cancellation Plan | Governs withdrawal of examination schedules | I04–I10 |

Additional planning capabilities may be introduced through configuration without modifying the workflow architecture.

---

# 19. Planning Dependency Matrix

The Examination Planning & Scheduling domain depends upon business information owned by collaborating domains.

Ownership remains unchanged.

| Planning Capability | Depends Upon |
|---------------------|--------------|
| Examination Planning | Examination Session (L2-I02) |
| Timetable Planning | Academic Calendar |
| Programme Planning | Curriculum Framework |
| Course Planning | Curriculum Framework |
| Venue Planning | Campus & Infrastructure |
| Room Allocation | Campus & Infrastructure |
| Resource Planning | Organisation, Person & Identity |
| Conflict Validation | Academic Calendar, Curriculum, Campus & Infrastructure |
| Schedule Approval | Institution Governance |
| Schedule Publication | Approved Examination Schedule |
| Schedule Revision | Approved Published Schedule |
| Schedule Cancellation | Institution Administration |
| Schedule Archival | Audit Infrastructure |

This dependency matrix establishes business sequencing while preserving ownership boundaries.

---

# 20. Examination Planning State Model

Every Examination Schedule progresses through a governed planning lifecycle.

| State | Description |
|--------|-------------|
| Draft | Initial examination planning is in progress |
| Under Review | Schedule undergoing institutional validation |
| Approved | Schedule approved for publication |
| Published | Official examination timetable released |
| Active | Schedule currently governing examination operations |
| Revised | Updated schedule approved and published |
| Cancelled | Schedule withdrawn from operational use |
| Completed | All scheduled examinations have concluded |
| Archived | Historical schedule retained for audit and reference |

Business rules governing state transitions:

- Every Examination Schedule begins in **Draft**.
- Only reviewed schedules may become **Approved**.
- Approved schedules may be **Published**.
- Published schedules become **Active** according to the approved examination timeline.
- Active schedules may be **Revised** under institutional governance.
- Active schedules may be **Cancelled** where exceptional circumstances require.
- Completed schedules progress to **Archived**.
- Archived schedules remain immutable for historical reference.

---

# 21. Configurable Parameters Catalogue

Institutions may configure Examination Planning behaviour without modifying the standard workflow architecture.

| Configuration Category | Examples |
|------------------------|----------|
| Planning Model | Centralised, Decentralised, Department-wise |
| Timetable Structure | Single Shift, Double Shift, Multiple Sessions |
| Examination Duration | Institution-specific duration rules |
| Time Slots | Morning, Afternoon, Evening |
| Venue Allocation | Campus, Building, Hall, Laboratory |
| Room Allocation | Capacity-based or Manual Allocation |
| Resource Planning | Invigilator Ratios, Supervisor Requirements |
| Conflict Validation | Student, Faculty, Venue, Resource Rules |
| Approval Workflow | Single-Level or Multi-Level Approval |
| Publication Rules | Immediate or Scheduled Publication |
| Revision Policy | Amendment Windows, Approval Requirements |
| Cancellation Policy | Institutional Cancellation Procedures |

Configuration values remain institution-specific while workflow architecture remains repository standard.

---

# 22. Domain Responsibility Summary

The Examination Planning & Scheduling domain governs preparation and publication of institutional examination schedules.

Operational execution remains the responsibility of subsequent Examination workflow documents.

| Business Capability | Owned by L2-I03 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Examination Planning | ✓ | — |
| Timetable Planning | ✓ | — |
| Venue Planning | ✓ | — |
| Room Allocation Planning | ✓ | — |
| Resource Planning | ✓ | — |
| Conflict Validation | ✓ | — |
| Schedule Approval | ✓ | — |
| Schedule Publication | ✓ | — |
| Schedule Revision | ✓ | — |
| Candidate Registration | — | L2-I04 |
| Hall Ticket Management | — | L2-I05 |
| Practical & Viva Examination Conduct | — | L2-I06 |
| Theory Examination Conduct | — | L2-I07 |
| Marks Collection & Evaluation | — | L2-I08 |
| Result Processing | — | L2-I09 |
| Result Publication | — | L2-I10 |
| Revaluation & Rechecking | — | L2-I11 |
| Supplementary & Improvement Examinations | — | L2-I12 |
| Academic Completion & Completion Evidence | — | L2-I13 |
| Malpractice Management | — | L2-I14 |
| Examination Exception Management | — | L2-I15 |
| Examination Session Closure & Archival | — | L2-I16 |

The Examination Planning & Scheduling domain owns planning governance only. It does not execute examination operations.

---

# 23. Repository Integration Summary

L2-I03 establishes the canonical business workflows for Examination Planning & Scheduling within the SutraERP Examination domain.

Every downstream operational examination workflow shall consume an approved Examination Schedule produced by this document.

The relationship between the foundational Examination documents is as follows:

| Document | Responsibility |
|----------|----------------|
| L2-I01 | Defines Examination Policies and Configuration |
| L2-I02 | Defines Examination Session lifecycle and governance |
| L2-I03 | Defines Examination Planning and Scheduling |
| L2-I04 | Registers eligible candidates for scheduled examinations |
| L2-I05 | Generates Hall Tickets using approved schedules |
| L2-I06 | Conducts Practical and Viva examinations according to approved schedules |
| L2-I07 | Conducts Theory examinations according to approved schedules |
| L2-I08 | Collects and evaluates examination marks |
| L2-I09 | Processes examination results |
| L2-I10 | Publishes approved examination results |
| L2-I11 | Manages revaluation and rechecking workflows |
| L2-I12 | Conducts supplementary, backlog, and improvement examinations |
| L2-I13 | Determines academic completion and completion evidence |
| L2-I14 | Manages malpractice investigations |
| L2-I15 | Handles examination exceptions and contingencies |
| L2-I16 | Closes examination sessions and archives examination records |

L2-I03 governs examination planning and scheduling only.

Operational examination activities consume approved schedules but do not modify planning governance.

---





