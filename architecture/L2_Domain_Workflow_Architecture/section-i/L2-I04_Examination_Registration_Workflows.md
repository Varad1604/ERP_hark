# L2-I04 — Examination Registration Workflows

**Level:** L2 – Business Workflow Architecture

**Domain:** I – Examination

**Status:** Draft

**Owner:** Examination Domain

**Document Owner:** Examinations — Registration

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
| Workflow | §17 Workflow Completion Criteria; L2-I04 — Examination Registration Workflows |
| State Machine | §20 Registration State Model |
| Contracts | — not present — |
| Events | §11 Events Published; §12 Events Consumed; §4.9 Event-Driven Registration |
| Dependencies | §19 Registration Dependency Matrix |
| Locked Decisions | — not present — |

> **Gap:** 3 required section(s) have no counterpart in this document: Entities, Contracts, Locked Decisions. Recorded, not authored.

---

# Document Control

| Item | Value |
|------|-------|
| Document ID | L2-I04 |
| Document Name | Examination Registration Workflows |
| Domain | Examination |
| Level | L2 – Business Workflow Architecture |
| Owner | Examination Domain |
| Depends On | L2-I01 Examination Policy & Configuration, L2-I02 Examination Session Management, L2-I03 Examination Planning & Scheduling, A01 Platform Foundation, A02 Person & Identity, A03 Authentication, A04 Authorization, H02 Notification Workflows, A06 Event Infrastructure, A07 Audit Infrastructure, B01 Institution, B02 Organisation, B04 Curriculum Framework, B05 Academic Calendar, F00 Finance Domain Model & Foundation |
| Consumed By | L2-I05 through L2-I16 Examination Workflows |
| Canonical Owner | Examination Domain |

---

# 1. Purpose

This document defines the business workflows governing Examination Registration within SutraERP.

Examination Registration determines which eligible candidates are authorised to participate in a scheduled Examination Session by registering for applicable examinations.

This document establishes how examination registrations are initiated, validated, approved, amended, confirmed, withdrawn, and governed before hall tickets are generated and examinations are conducted.

Operational workflows such as Hall Ticket Management, Practical & Viva Examination, Theory Examination Conduct, Marks Collection, and Result Processing consume approved examination registrations produced by this document.

This document defines business workflows only.

No implementation, database design, APIs, user interface behaviour, storage models, payment processing, or technology architecture are included.

---

# 2. Scope

This document includes:

- Examination registration
- Registration eligibility validation
- Subject registration
- Regular examination registration
- Supplementary examination registration
- Backlog examination registration
- Improvement examination registration
- Late registration
- Registration approval
- Registration amendment
- Registration withdrawal
- Registration confirmation
- Registration locking
- Registration governance

This document excludes:

- Student admission
- Academic programme enrolment
- Semester registration
- Examination planning
- Hall ticket generation
- Examination attendance
- Examination conduct
- Marks collection
- Result processing
- Finance transaction processing

These capabilities are governed by their respective domains and workflow documents.

---

# 3. Repository Context

L2-I04 establishes the canonical business workflows for Examination Registration.

Every candidate appearing for an examination shall possess an approved Examination Registration associated with an active Examination Session and an approved Examination Schedule.

Examination Registration consumes:

- Examination Policies from L2-I01.
- Approved Examination Sessions from L2-I02.
- Approved Examination Schedules from L2-I03.
- Academic structures from the Curriculum Framework.
- Academic periods from the Academic Calendar.
- Financial eligibility supplied by the Finance domain.

This document collaborates with multiple business domains but owns only the governance of Examination Registration.

Approved Examination Registrations become the authoritative business input for Hall Ticket generation and all subsequent operational Examination workflows.

# 4. Architectural Principles

The Examination Registration domain operates according to the following architectural principles.

These principles establish consistent registration governance across all institutions while allowing institution-specific configuration.

---

## 4.1 Registration After Planning

Examination Registration shall begin only after:

- Examination Policies are established.
- An Examination Session is Active.
- An Examination Schedule has been officially published.

Registration shall not precede examination planning.

---

## 4.2 Eligibility Before Registration

Only eligible candidates may register for examinations.

Eligibility shall be determined using institutional policies and collaborating business domains.

Eligibility validation may consider:

- Academic status
- Programme applicability
- Course applicability
- Examination rules
- Financial eligibility
- Institutional restrictions

The Examination Registration domain consumes eligibility information but does not own all eligibility rules.

---

## 4.3 Registration is Candidate-Centric

Every Examination Registration belongs to a single candidate.

A candidate may register for one or more examinations within the same Examination Session according to institutional policy.

Registration governance is always maintained at the candidate level.

---

## 4.4 Registration Follows Curriculum

Examination Registration shall align with the applicable academic structure.

Registration shall reference:

- Programme
- Batch
- Academic Period
- Curriculum Version
- Course
- Subject
- Examination Type

Curriculum ownership remains with the Curriculum Framework domain.

---

## 4.5 Controlled Registration Lifecycle

Every Examination Registration shall progress through a governed lifecycle.

Business state transitions shall be controlled, auditable, and institutionally governed.

No registration shall bypass defined lifecycle stages.

---

## 4.6 Configurable Registration Policies

Institutions may configure registration behaviour without changing the canonical workflow.

Examples include:

- Registration windows
- Late registration rules
- Approval requirements
- Subject registration policies
- Improvement examination rules
- Backlog registration policies

Workflow architecture remains repository standard.

---

## 4.7 Registration Before Hall Ticket Generation

Hall Ticket generation depends upon approved Examination Registrations.

Only valid registrations may participate in downstream examination workflows.

Registration serves as the authoritative source for candidate examination participation.

---

## 4.8 Multi-Domain Governance

Examination Registration collaborates with multiple business domains.

These include:

- Examination Policy
- Examination Session
- Examination Planning
- Curriculum Framework
- Academic Calendar
- Student
- Finance
- Person & Identity

Ownership of business information remains with the originating domain.

---

## 4.9 Event-Driven Registration

Significant registration activities shall publish business events.

Examples include:

- Registration Created
- Registration Approved
- Registration Confirmed
- Registration Withdrawn
- Registration Locked

Downstream workflows consume these events while preserving ownership boundaries.

---

## 4.10 Complete Auditability

Every significant registration activity shall generate audit records.

Audit history shall support:

- Regulatory compliance
- Institutional governance
- Historical traceability
- Administrative accountability

Audit implementation is provided by the Platform Audit Infrastructure.

---

## 4.11 Configuration Before Customisation

Institution-specific registration requirements shall be implemented through configuration wherever possible.

Repository-standard registration workflows shall remain unchanged.

---

# 5. Business Capabilities

The Examination Registration domain provides the following business capabilities.

---

## 5.1 Registration Eligibility Validation

Determines whether a candidate is eligible to register for examinations.

---

## 5.2 Examination Registration

Creates official Examination Registration records for eligible candidates.

---

## 5.3 Subject Registration

Registers applicable examination subjects within an Examination Session.

---

## 5.4 Regular Examination Registration

Supports registration for standard scheduled examinations.

---

## 5.5 Backlog Examination Registration

Supports registration for previously unsuccessful examinations requiring reappearance.

---

## 5.6 Supplementary Examination Registration

Supports registration for supplementary examination opportunities governed by institutional policy.

---

## 5.7 Improvement Examination Registration

Supports registration for examinations intended to improve previously awarded performance.

---

## 5.8 Late Registration

Supports controlled registration after the normal registration period where institutional policies permit.

---

## 5.9 Registration Approval

Provides institutional review and approval of examination registrations where required.

---

## 5.10 Registration Amendment

Supports controlled modification of approved or pending examination registrations according to institutional policy.

---

## 5.11 Registration Withdrawal

Supports candidate or institutional withdrawal of examination registrations before defined deadlines.

---

## 5.12 Registration Confirmation

Confirms successful completion of the examination registration process.

---

## 5.13 Registration Locking

Prevents further modification of examination registrations after institutional cut-off points.

This establishes the final candidate list for downstream examination operations.

# 6. Business Actors

The following business actors participate in Examination Registration workflows.

| Actor | Responsibility |
|--------|----------------|
| Candidate (Student) | Initiates examination registration for eligible examinations |
| Examination Administrator | Manages examination registration operations |
| Controller of Examination | Governs examination registration policies and approvals |
| Department Administration | Verifies departmental registration requirements |
| Academic Administration | Validates academic eligibility where applicable |
| Finance Domain | Provides financial eligibility and payment status |
| Curriculum Framework Domain | Provides programme, course and subject applicability |
| Academic Calendar Domain | Provides registration windows and academic periods |
| Student Domain | Provides candidate academic status and enrolment information |
| Audit Infrastructure | Maintains complete registration audit history |

Business actors consume or contribute business information while ownership remains within their respective domains.

---

# 7. Examination Registration Framework

The Examination Registration Framework establishes the canonical business structure governing candidate registration for examinations.

Registration authorises eligible candidates to participate in approved examinations within an active Examination Session.

---

## 7.1 Registration Objectives

The framework enables institutions to:

- Register eligible candidates.
- Validate examination eligibility.
- Register applicable examination subjects.
- Support multiple examination categories.
- Govern registration approvals.
- Produce authoritative candidate examination lists.
- Support downstream examination operations.

Registration establishes examination participation but does not authorise examination entry. Hall Ticket Management performs operational authorisation.

---

## 7.2 Registration Dimensions

Every Examination Registration shall consider one or more registration dimensions.

| Registration Dimension | Purpose |
|------------------------|---------|
| Candidate | Person appearing for examinations |
| Examination Session | Governing examination cycle |
| Academic Period | Applicable semester, trimester or academic term |
| Programme | Applicable academic programme |
| Curriculum Version | Governing curriculum |
| Subject Registration | Subjects being registered for examination |
| Examination Category | Regular, Backlog, Supplementary, Improvement |
| Registration Window | Applicable registration period |
| Registration Status | Current business state |

These dimensions collectively define a complete Examination Registration.

---

## 7.3 Registration Categories

Institutions may support multiple examination registration categories.

Examples include:

| Registration Category | Business Purpose |
|-----------------------|------------------|
| Regular | Standard examination registration |
| Backlog | Registration for previously unsuccessful subjects |
| Supplementary | Registration for supplementary examination opportunities |
| Improvement | Registration to improve previously awarded performance |
| Special | Institution-specific examination opportunities |

Available categories shall be governed by institutional Examination Policies.

---

## 7.4 Subject Registration Framework

Candidates register for examination subjects rather than entire programmes.

Subject registration may include:

- Mandatory Subjects
- Elective Subjects
- Practical Subjects
- Viva Subjects
- Project Evaluation Subjects

Subject applicability is determined by the Curriculum Framework.

---

## 7.5 Registration Eligibility Framework

Eligibility shall be evaluated before registration proceeds.

Eligibility may consider:

- Candidate academic status
- Programme applicability
- Subject applicability
- Examination policy compliance
- Examination Session applicability
- Financial eligibility
- Institutional restrictions

Eligibility determination consumes business information from collaborating domains.

---

## 7.6 Registration Windows

Institutions may define controlled registration periods.

Typical registration windows include:

- Standard Registration
- Late Registration
- Extended Registration
- Special Registration

Registration outside approved windows shall follow institutional policy.

---

## 7.7 Registration Confirmation

Registration becomes operationally valid only after successful completion of all required institutional validations.

Confirmed registrations become eligible for Hall Ticket generation.

---

# 8. Examination Registration Lifecycle

Every Examination Registration progresses through a governed business lifecycle.

The lifecycle ensures that only validated and approved registrations participate in downstream examination workflows.

---

## 8.1 Lifecycle Overview

The standard Examination Registration lifecycle consists of the following stages:

1. Registration Initiated
2. Eligibility Validation
3. Subject Registration
4. Registration Review
5. Registration Approval
6. Registration Confirmation
7. Registration Locking
8. Operational Consumption
9. Registration Amendment (Optional)
10. Registration Withdrawal (Optional)
11. Registration Completion
12. Registration Archival

Each stage represents a governed business transition.

---

## 8.2 Registration Initiation

Registration begins when an eligible candidate initiates examination registration during an approved registration window.

Business activities include:

- Identifying the Examination Session
- Identifying applicable subjects
- Selecting examination category
- Creating an initial registration request

Registration initiation does not confirm examination participation.

---

## 8.3 Eligibility Validation

Institutional rules determine whether the candidate may proceed.

Validation may include:

- Academic eligibility
- Curriculum applicability
- Examination policy compliance
- Registration window validation
- Financial eligibility

Failed validations prevent registration from progressing.

---

## 8.4 Subject Registration

Eligible candidates register for applicable examination subjects.

Registration may include:

- Mandatory subjects
- Elective subjects
- Practical examinations
- Viva examinations
- Improvement subjects
- Backlog subjects

Subject registration establishes the examination scope for the candidate.

---

## 8.5 Registration Review

Institutional review verifies registration correctness before approval.

Review may include:

- Subject applicability
- Examination category correctness
- Policy compliance
- Administrative verification

Only reviewed registrations may proceed to approval.

---

## 8.6 Registration Approval

Approved registrations become institutionally authorised.

Approval confirms that the candidate is permitted to participate in the registered examinations, subject to downstream operational requirements.

# 8.7 Registration Confirmation

Following institutional approval, the Examination Registration is confirmed.

Confirmation establishes the registration as the authoritative business record for the candidate's participation in the registered examinations.

Confirmation activities include:

- Finalising registration details
- Recording confirmation status
- Publishing registration events
- Making the registration available for downstream workflows

Only confirmed registrations may proceed toward Hall Ticket generation.

---

## 8.8 Registration Locking

After the institutional registration deadline, Examination Registrations may be locked.

Registration locking:

- Prevents further modifications.
- Establishes the final examination candidate list.
- Preserves registration integrity.
- Supports downstream operational workflows.

Locked registrations may only be changed through approved exceptional workflows.

---

## 8.9 Registration Amendment

Institutions may permit controlled amendments before registration locking.

Typical amendments include:

- Subject addition
- Subject withdrawal
- Registration category correction
- Administrative corrections
- Approved academic changes

Every amendment shall preserve complete audit history.

---

## 8.10 Registration Withdrawal

Candidates or authorised institutional administrators may withdraw an Examination Registration where permitted.

Withdrawal may apply to:

- Individual subjects
- Selected examination categories
- Entire examination registrations

Withdrawal shall follow institutional policy and applicable deadlines.

---

## 8.11 Registration Completion

An Examination Registration is considered complete when:

- Registration has been confirmed.
- Registration has been locked.
- Candidate information is available for downstream operational workflows.

Completion marks the end of registration governance.

---

## 8.12 Registration Archival

Completed registrations eventually enter archival status.

Archived registrations remain available for:

- Regulatory compliance
- Institutional reporting
- Historical reference
- Audit investigations
- Examination record retention

Archived registrations remain immutable from a business governance perspective.

---

# 9. Business Rules

The following business rules govern Examination Registration across all institutions.

Business rules establish consistent governance while allowing configurable institutional policies.

---

## 9.1 Registration Eligibility Rules

A candidate may register only when:

- An Examination Session is Active.
- An Examination Schedule has been published.
- The candidate satisfies institutional eligibility requirements.
- Registration occurs within an approved registration window.
- Applicable institutional restrictions have been satisfied.

Ineligible candidates shall not proceed with registration.

---

## 9.2 Subject Registration Rules

Every Examination Registration shall include one or more valid examination subjects.

Subject registration shall comply with:

- Programme applicability
- Curriculum version
- Academic period
- Examination category
- Examination policy

Subject ownership remains within the Curriculum Framework domain.

---

## 9.3 Examination Category Rules

Every registration shall belong to an approved examination category.

Examples include:

- Regular
- Backlog
- Supplementary
- Improvement
- Special

Category availability is determined by institutional Examination Policies.

---

## 9.4 Registration Window Rules

Registrations shall occur only within approved institutional registration windows.

Examples include:

- Standard Registration
- Late Registration
- Extended Registration
- Special Registration

Registrations outside approved windows require authorised institutional exceptions.

---

## 9.5 Approval Rules

Where institutional approval is required:

- Registration review shall be completed.
- Eligibility validation shall succeed.
- Applicable policy requirements shall be satisfied.
- Required approvals shall be obtained.

Only approved registrations may become confirmed registrations.

---

## 9.6 Amendment Rules

Registration amendments shall:

- Follow institutional policy.
- Occur before registration locking unless exceptional approval is granted.
- Preserve complete audit history.

Unauthorised amendments shall not be permitted.

---

## 9.7 Withdrawal Rules

Registration withdrawal shall comply with institutional policies governing:

- Withdrawal eligibility
- Withdrawal deadlines
- Administrative approval
- Subject-specific withdrawal
- Complete registration withdrawal

Withdrawn registrations shall not participate in downstream examination workflows unless subsequently reinstated through approved processes.

---

## 9.8 Registration Locking Rules

After registration locking:

- Registration details become operationally final.
- Routine amendments shall not be permitted.
- Downstream examination workflows shall consume the locked registration data.

Exceptional administrative actions remain governed by institutional policies.

---

## 9.9 Audit Rules

Every significant registration activity shall generate audit records.

Examples include:

- Registration creation
- Eligibility validation
- Subject registration
- Approval
- Confirmation
- Amendment
- Withdrawal
- Locking
- Administrative override

---

# 10. Exception Scenarios

The Examination Registration workflow shall support controlled handling of exceptional business situations.

---

## 10.1 Ineligible Candidate

If eligibility validation fails:

- Registration shall not proceed.
- Failure reasons shall be recorded.
- The candidate may reattempt registration after eligibility requirements are satisfied.

---

## 10.2 Registration Window Closed

If registration is attempted after the approved registration period:

- Registration shall be rejected.
- Institutional late registration policies may apply where configured.

---

## 10.3 Subject Eligibility Failure

If one or more subjects are not eligible for examination registration:

- Invalid subjects shall not be registered.
- Eligible subjects may continue according to institutional policy.

---

## 10.4 Financial Eligibility Failure

Where financial eligibility is required and not satisfied:

- Registration shall remain pending or be rejected according to institutional policy.
- Financial ownership remains with the Finance domain.

---

## 10.5 Duplicate Registration

If a duplicate Examination Registration is detected:

- Duplicate registration shall be prevented.
- Existing valid registration shall remain authoritative.

---

## 10.6 Administrative Rejection

Institutional administrators may reject an Examination Registration.

Rejected registrations may:

- Return for correction.
- Be resubmitted.
- Be withdrawn.
- Be cancelled.

Rejected registrations shall not progress to confirmation.

---

## 10.7 Amendment After Locking

Requests to modify locked registrations shall require exceptional institutional approval.

Unauthorised modifications shall not be permitted.

---

## 10.8 Registration Withdrawal After Approval

Approved registrations may be withdrawn according to institutional withdrawal policies.

Withdrawal shall preserve complete audit history and notify downstream workflows where applicable.

---

## 10.9 Exceptional Administrative Override

Authorised institutional officers may perform controlled administrative overrides in exceptional circumstances.

Every override shall:

- Require appropriate authority.
- Record business justification.
- Generate complete audit records.
- Preserve historical traceability.

# 11. Events Published

The Examination Registration domain publishes business events that notify downstream workflows of significant registration lifecycle changes.

These events communicate business state changes while preserving ownership boundaries.

| Event | Published When | Primary Consumers |
|-------|----------------|-------------------|
| Registration Initiated | Registration request created | Audit, Notification |
| Eligibility Validated | Eligibility verification completed | Registration Workflow |
| Registration Submitted | Candidate completes registration submission | Examination Administration |
| Registration Approved | Institutional approval granted | L2-I05 Hall Ticket |
| Registration Rejected | Registration rejected | Notification, Candidate Services |
| Registration Confirmed | Registration becomes operationally valid | L2-I05, L2-I06, L2-I07 |
| Registration Amended | Registration modification approved | L2-I05, Audit |
| Registration Withdrawn | Registration withdrawn | L2-I05, L2-I06, L2-I07 |
| Registration Locked | Registration frozen for examination operations | L2-I05–L2-I10 |
| Registration Archived | Registration lifecycle completed | Reporting, Audit |

Business events represent authoritative registration milestones.

---

# 12. Events Consumed

The Examination Registration workflow consumes business events published by collaborating domains.

Ownership of business information remains with the publishing domain.

| Event | Published By |
|-------|---------------|
| Examination Policy Updated | L2-I01 |
| Examination Session Activated | L2-I02 |
| Examination Schedule Published | L2-I03 |
| Academic Calendar Updated | Academic Calendar Domain |
| Curriculum Updated | Curriculum Framework Domain |
| Student Academic Status Updated | Student Domain |
| Financial Eligibility Updated | Finance Domain |
| Authentication Completed | Platform Authentication |
| Audit Request Raised | Audit Infrastructure |

Consumed events influence registration decisions without transferring ownership.

---

# 13. Multi-Domain Interactions

The Examination Registration domain collaborates with multiple business domains to determine candidate eligibility and establish authoritative examination registrations.

Business ownership remains with the originating domains.

---

## 13.1 Examination Policy Domain

L2-I01 governs the policies controlling examination registration.

Business interactions include:

- Registration policies
- Registration windows
- Registration categories
- Eligibility rules
- Amendment policies
- Withdrawal policies

The Registration domain consumes policies but does not modify them.

---

## 13.2 Examination Session Management Domain

L2-I02 provides the Examination Session governing registration.

Business interactions include:

- Active Examination Sessions
- Session applicability
- Session lifecycle
- Session validity

Registration cannot exist independently of an active Examination Session.

---

## 13.3 Examination Planning & Scheduling Domain

L2-I03 provides the approved Examination Schedule.

Business interactions include:

- Published examination timetable
- Scheduled examination subjects
- Examination dates
- Examination slots
- Examination centres

Only published schedules are eligible for examination registration.

---

## 13.4 Curriculum Framework Domain

The Curriculum Framework determines the academic applicability of examination registration.

Business interactions include:

- Programmes
- Curriculum Versions
- Courses
- Subjects
- Academic Structures

Curriculum ownership remains external to the Examination domain.

---

## 13.5 Academic Calendar Domain

The Academic Calendar defines registration periods and academic timelines.

Business interactions include:

- Academic Year
- Semester
- Registration Window
- Examination Period
- Institutional Holidays

Calendar ownership remains outside the Registration domain.

---

## 13.6 Student Domain

The Student domain provides candidate academic information.

Business interactions include:

- Student Identity
- Academic Status
- Programme Association
- Semester Status
- Academic Progress

The Registration domain consumes student information but does not manage student records.

---

## 13.7 Finance Domain

The Finance domain provides financial eligibility required for examination registration where institutional policy requires it.

Business interactions include:

- Financial Clearance
- Outstanding Obligations
- Examination Fee Status
- Financial Holds

Financial ownership remains entirely within the Finance domain.

---

## 13.8 Person & Identity Domain

The Person & Identity domain provides institutional users participating in registration governance.

Business interactions include:

- Candidates
- Examination Administrators
- Academic Administrators
- Controllers of Examination

Identity management remains outside the Examination domain.

---

## 13.9 Platform Foundation

Platform Foundation provides enterprise infrastructure supporting Examination Registration.

Supporting services include:

- Authentication
- Authorization
- Notification Workflows
- Event Infrastructure
- Audit Infrastructure

Platform services provide infrastructure without owning registration business logic.

---

# 14. Ownership Boundaries

The Examination Registration domain owns the governance of candidate examination registration.

Subsequent examination workflows consume approved registrations without modifying registration governance.

---

## Owned by L2-I04

The following business capabilities are owned by this domain:

- Registration Eligibility Validation
- Examination Registration
- Subject Registration
- Registration Categories
- Registration Approval
- Registration Confirmation
- Registration Amendment
- Registration Withdrawal
- Registration Locking
- Registration Governance

---

## Not Owned by L2-I04

The following business capabilities belong to other workflow documents.

| Business Capability | Owning Document |
|---------------------|-----------------|
| Examination Policy Governance | L2-I01 |
| Examination Session Management | L2-I02 |
| Examination Planning & Scheduling | L2-I03 |
| Hall Ticket Management | L2-I05 |
| Practical & Viva Examination Conduct | L2-I06 |
| Theory Examination Conduct | L2-I07 |
| Marks Collection & Evaluation | L2-I08 |
| Result Processing | L2-I09 |
| Result Publication | L2-I10 |
| Revaluation & Rechecking | L2-I11 |
| Supplementary & Improvement Examination Execution | L2-I12 |
| Academic Completion & Completion Evidence | L2-I13 |
| Malpractice Management | L2-I14 |
| Examination Exception Management | L2-I15 |
| Examination Session Closure & Archival | L2-I16 |

This separation maintains clear ownership across the Examination repository.

---

# 15. Configurable Registration Matrix

Institutions may configure Examination Registration behaviour without modifying the canonical workflow architecture.

| Configuration Area | Examples |
|--------------------|----------|
| Registration Windows | Standard, Late, Extended, Special |
| Registration Categories | Regular, Backlog, Supplementary, Improvement |
| Subject Registration Policy | Mandatory Only, Mandatory + Electives |
| Eligibility Validation | Academic, Financial, Administrative Rules |
| Approval Workflow | Automatic, Single-Level, Multi-Level |
| Amendment Policy | Allowed Until Locking, Restricted Changes |
| Withdrawal Policy | Subject-Level, Registration-Level |
| Registration Lock Date | Institution-defined Cut-off |
| Financial Requirement | Mandatory, Optional, Institution-specific |
| Notification Policy | Registration Status Notifications |

Configuration affects business behaviour while preserving the repository-standard workflow architecture.

# 16. Audit Requirements

Every Examination Registration activity shall generate auditable business records.

Auditability supports regulatory compliance, institutional governance, dispute resolution, and historical traceability.

---

## Auditable Activities

The following business activities shall be auditable:

- Registration Initiation
- Eligibility Validation
- Subject Registration
- Registration Review
- Registration Approval
- Registration Confirmation
- Registration Amendment
- Registration Withdrawal
- Registration Locking
- Administrative Override
- Registration Archival

Every significant business transition shall be recorded.

---

## Audit Information

Audit records may include:

- Registration Identifier
- Candidate Identifier
- Examination Session
- Examination Category
- Registered Subjects
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

An Examination Registration workflow is considered complete when:

- An active Examination Session has been identified.
- A published Examination Schedule has been consumed.
- Candidate eligibility has been successfully validated.
- Applicable examination subjects have been registered.
- Required institutional approvals have been completed.
- Registration has been confirmed.
- Registration has been locked or otherwise finalised according to institutional policy.
- All registration events have been published.
- Audit records have been generated.
- Approved registration data is available for downstream examination workflows.

Completion establishes the authoritative candidate examination registration for the associated Examination Session.

---

# 18. Registration Catalogue

The Registration Catalogue defines the canonical registration capabilities owned by the Examination Registration domain.

These capabilities are consumed by downstream examination workflows without redefining registration responsibilities.

| Registration Capability | Business Purpose | Primary Consumers |
|-------------------------|------------------|-------------------|
| Eligibility Validation | Determines candidate eligibility | I04 |
| Examination Registration | Creates official examination participation record | I05–I10 |
| Subject Registration | Registers examination subjects | I05–I10 |
| Regular Registration | Standard examination participation | I05–I10 |
| Backlog Registration | Reappearance for failed subjects | I05–I12 |
| Supplementary Registration | Supplementary examination participation | I05–I12 |
| Improvement Registration | Performance improvement examinations | I05–I12 |
| Registration Amendment | Controlled registration modification | I05 |
| Registration Withdrawal | Controlled registration cancellation | I05 |
| Registration Locking | Finalises examination participation | I05–I10 |

Additional registration capabilities may be introduced through institutional configuration without modifying the canonical workflow.

---

# 19. Registration Dependency Matrix

The Examination Registration domain depends upon business information owned by collaborating domains.

Ownership remains with the originating business domains.

| Registration Capability | Depends Upon |
|--------------------------|--------------|
| Eligibility Validation | Examination Policy (L2-I01), Student Domain, Finance Domain |
| Examination Registration | Examination Session (L2-I02) |
| Subject Registration | Curriculum Framework |
| Registration Windows | Academic Calendar |
| Registration Approval | Institutional Governance |
| Registration Confirmation | Successful Validation & Approval |
| Registration Locking | Registration Deadline |
| Registration Amendment | Institutional Registration Policy |
| Registration Withdrawal | Institutional Withdrawal Policy |
| Registration Archival | Audit Infrastructure |

The dependency matrix establishes business sequencing while preserving ownership boundaries.

---

# 20. Registration State Model

Every Examination Registration progresses through a governed business lifecycle.

| State | Description |
|--------|-------------|
| Draft | Registration initiated but not submitted |
| Pending Validation | Eligibility verification in progress |
| Pending Review | Awaiting institutional review |
| Approved | Institutionally approved |
| Confirmed | Operationally valid registration |
| Locked | Registration frozen for examination operations |
| Amended | Approved registration modified |
| Withdrawn | Registration withdrawn according to policy |
| Completed | Registration lifecycle concluded |
| Archived | Historical registration retained for audit |

Business rules governing state transitions:

- Every registration begins in **Draft**.
- Successful validation moves the registration to **Pending Review** where applicable.
- Approved registrations become **Confirmed**.
- Confirmed registrations may become **Locked** after registration deadlines.
- Confirmed registrations may become **Amended** before locking according to institutional policy.
- Confirmed or Locked registrations may become **Withdrawn** through authorised institutional processes.
- Completed registrations progress to **Archived**.
- Archived registrations remain immutable for historical reference.

---

# 21. Configurable Parameters Catalogue

Institutions may configure Examination Registration behaviour while preserving repository-standard workflows.

| Configuration Category | Examples |
|------------------------|----------|
| Registration Window | Opening and closing dates |
| Registration Categories | Regular, Backlog, Supplementary, Improvement |
| Eligibility Rules | Academic, Financial, Administrative |
| Subject Registration Rules | Mandatory, Elective, Practical, Viva |
| Approval Workflow | Automatic, Single-Level, Multi-Level |
| Amendment Rules | Allowed Periods, Approval Requirements |
| Withdrawal Rules | Deadlines, Approval Policies |
| Registration Locking | Automatic or Administrative |
| Financial Clearance | Mandatory, Optional |
| Notification Behaviour | Registration Status Notifications |

Institution-specific values may vary without changing workflow architecture.

---

# 22. Domain Responsibility Summary

The Examination Registration domain governs candidate participation in scheduled examinations.

It establishes the authoritative registration records consumed by downstream operational workflows.

| Business Capability | Owned by L2-I04 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Eligibility Validation | ✓ | — |
| Examination Registration | ✓ | — |
| Subject Registration | ✓ | — |
| Registration Approval | ✓ | — |
| Registration Confirmation | ✓ | — |
| Registration Amendment | ✓ | — |
| Registration Withdrawal | ✓ | — |
| Registration Locking | ✓ | — |
| Hall Ticket Generation | — | L2-I05 |
| Practical & Viva Conduct | — | L2-I06 |
| Theory Examination Conduct | — | L2-I07 |
| Marks Collection & Evaluation | — | L2-I08 |
| Result Processing | — | L2-I09 |
| Result Publication | — | L2-I10 |
| Revaluation & Rechecking | — | L2-I11 |
| Supplementary Examination Execution | — | L2-I12 |
| Academic Completion & Completion Evidence | — | L2-I13 |
| Malpractice Management | — | L2-I14 |
| Exception Management | — | L2-I15 |
| Session Closure & Archival | — | L2-I16 |

The Examination Registration domain governs registration only and does not execute examination operations.

---

# 23. Repository Integration Summary

L2-I04 establishes the canonical business workflows for Examination Registration within the SutraERP Examination domain.

Every operational examination workflow shall consume approved Examination Registrations produced by this document.

The relationship between Examination workflow documents is as follows:

| Document | Responsibility |
|----------|----------------|
| L2-I01 | Examination Policy & Configuration |
| L2-I02 | Examination Session Management |
| L2-I03 | Examination Planning & Scheduling |
| **L2-I04** | **Examination Registration** |
| L2-I05 | Hall Ticket Management |
| L2-I06 | Practical & Viva Examination Conduct |
| L2-I07 | Theory Examination Conduct |
| L2-I08 | Marks Collection & Evaluation |
| L2-I09 | Result Processing & Computation |
| L2-I10 | Result Publication |
| L2-I11 | Revaluation & Rechecking |
| L2-I12 | Supplementary / Backlog / Improvement Examination Execution |
| L2-I13 | Academic Completion, CGPA & Completion Evidence Workflows |
| L2-I14 | Malpractice & Examination Committee |
| L2-I15 | Examination Exception & Contingency Management |
| L2-I16 | Examination Session Closure & Archival |

L2-I04 produces the authoritative examination registration records that form the basis for Hall Ticket generation and all subsequent examination operations.

---

# Appendix A – Examination Registration Architectural Reference

> **Status:** Informative Architectural Appendix
>
> This appendix supplements the Examination Registration workflows by defining the canonical business concepts consumed by downstream Examination workflow documents. It does not introduce additional workflow stages or modify the ownership boundaries defined in this document.

---

# A.1 Registration Business Model

An **Examination Registration** represents a candidate's participation in a specific Examination Session.

Each Examination Registration:

- Belongs to exactly one Candidate.
- Belongs to exactly one Examination Session.
- Contains one or more Registration Items.
- Progresses through a single Registration Lifecycle.
- Acts as the authoritative business record for examination participation.

An Examination Registration does not represent academic enrolment, semester registration, or programme admission. Those responsibilities belong to their respective business domains.

---

# A.2 Registration Item Framework

A **Registration Item** represents an individual examination subject or assessment component within an Examination Registration.

Each Registration Item identifies a single examination component for which the candidate intends to appear.

Examples include:

- Theory Subject
- Practical Examination
- Viva Examination
- Laboratory Assessment
- Project Evaluation
- Dissertation Evaluation

Every Registration Item remains governed by its parent Examination Registration while serving as the operational unit consumed by downstream examination workflows.

---

# A.3 Registration Category Applicability

Registration categories apply at the **Registration Item** level rather than the overall Examination Registration.

This allows a single Examination Registration to contain multiple subject registrations belonging to different examination categories where permitted by institutional policy.

Example:

| Registration Item | Examination Category |
|-------------------|----------------------|
| Mathematics | Regular |
| Physics | Backlog |
| Chemistry Practical | Improvement |
| Project Viva | Supplementary |

This model supports mixed examination registrations while maintaining a single Examination Registration for the candidate within an Examination Session.

---

# A.4 Downstream Consumption Model

The Examination Registration and Registration Item business concepts are consumed by subsequent Examination workflow documents.

| Workflow | Business Unit Consumed |
|----------|------------------------|
| L2-I05 Hall Ticket Management | Registration Item |
| L2-I06 Practical & Viva Examination | Registration Item |
| L2-I07 Theory Examination Conduct | Registration Item |
| L2-I08 Marks Collection & Evaluation | Registration Item |
| L2-I09 Result Processing & Computation | Registration Item |
| L2-I10 Result Publication | Registration Item |
| L2-I11 Revaluation & Rechecking | Registration Item |
| L2-I12 Supplementary / Backlog / Improvement Examination Execution | Registration Item |

The parent Examination Registration provides governance and lifecycle management, while Registration Items provide the operational scope for examination execution.

---

# A.5 Ownership Clarification

The Examination Registration domain owns only examination participation governance.

It does **not** own:

- Student Admission
- Programme Enrolment
- Semester Registration
- Curriculum Ownership
- Academic Eligibility Determination
- Financial Clearance
- Fee Assessment
- Fee Collection
- Hall Ticket Generation
- Examination Attendance
- Marks Evaluation
- Result Processing

These capabilities are consumed from their respective business domains.

The Examination Registration domain consumes business outcomes such as **Academic Eligibility** and **Financial Clearance** but does not determine or modify them.

---

# End of Appendix A






