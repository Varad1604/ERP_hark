# L2-I02 — Examination Session Management Workflows

**Level:** L2 – Business Workflow Architecture

**Domain:** I – Examination

**Status:** Draft

**Owner:** Examination Domain

**Document Owner:** Devang

---

# Document Control

| Item | Value |
|------|-------|
| Document ID | L2-I02 |
| Document Name | Examination Session Management Workflows |
| Domain | Examination |
| Level | L2 – Business Workflow Architecture |
| Owner | Examination Domain |
| Depends On | L2-I01 Examination Policy & Configuration, A01 Platform Foundation, A02 Person & Identity, A03 Authentication, A04 Authorization, H02 Notification Workflows, A06 Event Infrastructure, A07 Audit Infrastructure, B01 Institution, B02 Organisation, B04 Curriculum Framework, B05 Academic Calendar |
| Consumed By | L2-I03 through L2-I16 Examination Workflows |
| Canonical Owner | Examination Domain |

---

# 1. Purpose

This document defines the business workflows governing the lifecycle of Examination Sessions within SutraERP.

An Examination Session represents a governed academic examination cycle conducted by an institution for one or more academic programmes, courses, semesters, or examination categories.

This document establishes how examination sessions are created, configured, activated, managed, suspended, completed, and closed before operational examination activities begin.

Operational workflows such as examination planning, registration, hall ticket generation, marks processing, result publication, and qualification completion consume approved examination sessions defined in this document.

This document defines business workflows only.

No implementation, database design, APIs, scheduling algorithms, user interface behaviour, storage models, or technology architecture are included.

---

# 2. Scope

This document includes:

- Examination Session lifecycle
- Examination Session creation
- Session configuration
- Session classification
- Session activation
- Session publication
- Session suspension
- Session reopening
- Session closure
- Session governance
- Session version management
- Multi-domain session dependencies

This document excludes:

- Examination timetable planning
- Venue allocation
- Invigilator allocation
- Hall ticket generation
- Examination registration
- Practical examinations
- Theory examination conduct
- Marks entry
- Result processing
- qualification completion processing

These activities are defined in subsequent Examination workflow documents.

---

# 3. Repository Context

L2-I02 establishes the canonical business lifecycle of Examination Sessions.

Every operational examination workflow shall execute within an approved Examination Session.

Examination Sessions provide the business context that connects institutional examination policies, academic calendars, curriculum structures, examination planning, student registration, evaluation, result processing, and examination completion.

This document collaborates with:

- L2-I01 for examination policy governance.
- Academic Calendar for academic periods and examination windows.
- Curriculum Framework for programme and course applicability.
- Institution and Organisation domains for examination governance.
- Platform Foundation for identity, authorization, events, notifications, and audit.

Operational workflows consume approved examination sessions but do not manage the session lifecycle itself.

# 4. Architectural Principles

The Examination Session Management workflows are governed by the following business principles.

These principles ensure that examination sessions remain governed, reusable, auditable, and independent of operational examination activities.

---

## 4.1 Session-Driven Examination Operations

Every operational examination activity shall execute within an approved Examination Session.

No examination planning, registration, hall ticket generation, evaluation, or result processing shall occur outside an active Examination Session.

The Examination Session serves as the business boundary for the complete examination lifecycle.

---

## 4.2 Separation of Session and Planning

An Examination Session defines **what examination cycle exists**.

Examination Planning defines **how that session will be executed**.

Examples:

- Session Creation
- Session Activation
- Session Governance
- Session Closure

are managed within this document.

Examples:

- Examination Timetable
- Venue Planning
- Faculty Allocation
- Invigilator Assignment

are managed by L2-I03.

---

## 4.3 Policy Governed Sessions

Every Examination Session shall be governed by an approved Examination Policy defined within L2-I01.

The session itself shall not redefine:

- Eligibility Rules
- Passing Rules
- Grading Rules
- Result Policies
- Revaluation Rules
- Qualification Completion Policies

These policies are consumed from the Examination Policy Domain.

---

## 4.4 Institution Independence

The Examination Session architecture shall support different institutional examination models through configuration.

Examples include:

- Universities
- Autonomous Colleges
- Affiliated Colleges
- Engineering Institutions
- Medical Institutions
- Polytechnic Institutions
- Professional Institutions

Operational workflows remain identical while institutional examination sessions remain configurable.

---

## 4.5 Multiple Concurrent Sessions

The platform shall support multiple independent Examination Sessions operating simultaneously.

Examples include:

- Regular Examination
- Supplementary Examination
- Improvement Examination
- Makeup Examination
- Special Examination

Each session operates independently while remaining governed by institutional examination policies.

---

## 4.6 Controlled Session Lifecycle

Every Examination Session shall follow a controlled business lifecycle.

Sessions shall progress through governed business states before becoming operational.

Only Active sessions may be consumed by downstream examination workflows.

---

## 4.7 Session Version Integrity

Changes made to an Examination Session shall not invalidate completed examination activities.

Where significant changes are required after operational activities have commenced, controlled versioning or governed amendments shall be followed according to institutional regulations.

Historical examination records shall preserve the session context under which they were executed.

---

## 4.8 Multi-Domain Collaboration

Examination Sessions consume business information from collaborating domains without assuming ownership.

Examples include:

| Domain | Business Dependency |
|---------|---------------------|
| Institution | Institutional examination governance |
| Organisation | Administrative ownership |
| Curriculum | Programme and course applicability |
| Academic Calendar | Academic sessions and examination windows |
| Examination Policy | Session governing policies |
| Platform Foundation | Identity, Authorization, Events, Notifications and Audit |

Ownership remains with the originating domain.

---

## 4.9 Complete Auditability

Every Examination Session lifecycle activity shall be fully traceable.

Audit records include:

- Session Creation
- Session Modification
- Session Approval
- Session Activation
- Session Suspension
- Session Reopening
- Session Closure
- Administrative Decisions
- Version Changes

Audit implementation is provided through the Platform Audit Infrastructure.

---

## 4.10 Configuration Before Customisation

Institution-specific examination session behaviour shall be achieved through configuration.

Business workflows shall remain repository standard across all supported institutions.

Custom workflow implementation shall only be considered when institutional requirements cannot be satisfied through configurable session definitions.

---

# 5. Business Capabilities

The Examination Session Management domain provides the foundational capabilities required to establish and govern institutional examination sessions.

---

## 5.1 Examination Session Definition

The platform shall support the creation of Examination Sessions representing institutional examination cycles.

Each session establishes the business context within which operational examination activities are executed.

---

## 5.2 Session Classification

Institutions shall configure supported examination session categories.

Examples include:

- Regular Examination Session
- Supplementary Examination Session
- Improvement Examination Session
- Makeup Examination Session
- Special Examination Session

Additional session categories may be introduced without requiring workflow redesign.

---

## 5.3 Session Configuration

The platform shall support configuration of business attributes governing examination sessions.

Examples include:

- Session Name
- Session Type
- Academic Year
- Semester
- Applicable Programmes
- Applicable Courses
- Governing Examination Policy
- Effective Academic Period

---

## 5.4 Session Activation

Approved Examination Sessions may be activated for operational use.

Only Active sessions may be consumed by:

- Examination Planning
- Registration
- Hall Ticket Generation
- Marks Processing
- Result Processing

---

## 5.5 Session Suspension

Institutions may temporarily suspend Examination Sessions due to exceptional business circumstances.

Examples include:

- Natural Disasters
- Government Directives
- Institutional Emergencies
- Academic Interruptions

Suspended sessions remain historically valid but temporarily unavailable for operational activities.

---

## 5.6 Session Reopening

Previously suspended sessions may be reopened following institutional approval.

Operational workflows resume only after the session returns to the Active state.

---

## 5.7 Session Closure

Following completion of all examination activities, Examination Sessions may be formally closed.

Closed sessions become read-only from a business perspective while remaining available for reporting, audit, and historical reference.

---

## 5.8 Session Version Management

Where institutional regulations require controlled changes, Examination Sessions shall support governed version management.

Versioning preserves:

- Historical Integrity
- Audit Traceability
- Operational Consistency
- Business Governance

---

# 6. Business Actors

The following business actors participate in Examination Session Management workflows.

| Actor | Responsibility |
|--------|----------------|
| Examination Administrator | Creates and manages Examination Sessions |
| Controller of Examination | Approves, activates, suspends and closes Examination Sessions |
| Academic Administration | Provides academic governance inputs for session applicability |
| Institution Administration | Defines institutional examination governance |
| Curriculum Domain | Provides programme and course applicability |
| Academic Calendar Domain | Provides academic periods and examination windows |
| Platform Administration | Maintains platform capabilities supporting session management |
| Audit Infrastructure | Records Examination Session lifecycle events |

Students and faculty members do not directly administer Examination Sessions.

Session governance remains an institutional administrative responsibility.

# 7. Examination Session Classification Framework

Examination Sessions are classified according to their academic purpose, institutional regulations, and operational characteristics.

The classification framework provides a standardized business model while allowing institution-specific configuration.

---

## 7.1 Classification Objectives

The Examination Session Classification Framework enables institutions to:

- Organize examination cycles consistently.
- Differentiate examination purposes.
- Apply appropriate examination policies.
- Support multiple concurrent examination sessions.
- Maintain governance across diverse academic structures.
- Preserve auditability throughout the examination lifecycle.

Classification defines business characteristics only and does not determine operational scheduling.

---

## 7.2 Classification Dimensions

Every Examination Session shall be classified using one or more business dimensions.

| Classification Dimension | Purpose |
|--------------------------|---------|
| Session Type | Identifies the business purpose of the examination session |
| Academic Period | Defines the academic year, semester, trimester or term |
| Programme Applicability | Identifies participating academic programmes |
| Curriculum Applicability | Identifies applicable curriculum versions |
| Institution Scope | Defines participating institutions or campuses |
| Examination Mode | Identifies theory, practical or combined examination cycles |
| Governing Policy | Identifies the Examination Policy governing the session |

Multiple dimensions may coexist without creating duplicate session definitions.

---

## 7.3 Session Type Classification

Institutions may configure one or more supported Examination Session types.

Typical session types include:

| Session Type | Business Purpose |
|--------------|------------------|
| Regular | Standard scheduled examination cycle |
| Supplementary | Additional examination opportunity for eligible students |
| Improvement | Examination conducted for grade improvement |
| Makeup | Examination for approved exceptional circumstances |
| Special | Institution-specific examination cycle |
| Entrance | Admission-related examinations (where applicable) |
| Certification | Internal certification examinations |
| Exit Examination | Programme completion assessment |

Additional institutional session types may be configured without changing workflow architecture.

---

## 7.4 Academic Period Classification

Every Examination Session shall be associated with an academic period defined by the Academic Calendar domain.

Examples include:

- Academic Year
- Semester
- Trimester
- Quarter
- Annual Programme
- Modular Programme

Academic periods remain owned by the Academic Calendar domain.

---

## 7.5 Programme Applicability

An Examination Session may apply to:

- One Programme
- Multiple Programmes
- Programme Groups
- Institution-wide Academic Programmes

Programme ownership remains with the Curriculum domain.

---

## 7.6 Curriculum Applicability

Examination Sessions may reference one or more curriculum versions.

Examples include:

- Current Curriculum
- Legacy Curriculum
- Revised Curriculum
- CBCS
- NEP-based Curriculum
- Institution-specific Curriculum Structures

Curriculum ownership remains outside the Examination domain.

---

## 7.7 Organisational Scope

Depending upon institutional governance, an Examination Session may be applicable to:

- Entire Institution
- Campus
- Faculty
- School
- Department
- Programme Group

Organisational structures are owned by the Organisation domain.

---

## 7.8 Examination Mode Classification

An Examination Session may govern different examination modes.

Examples include:

| Mode | Description |
|------|-------------|
| Theory | Written examinations |
| Practical | Laboratory or practical examinations |
| Viva | Oral examinations |
| Project | Project evaluations |
| Combined | Multiple examination modes within one session |

Operational conduct of each examination mode is defined in subsequent workflow documents.

---

## 7.9 Governing Policy Association

Every Examination Session shall reference an approved Examination Policy defined in L2-I01.

Policy association ensures that all operational workflows execute under approved institutional regulations.

Session definitions shall not duplicate policy definitions.

---

# 8. Examination Session Lifecycle Workflows

Every Examination Session progresses through a governed lifecycle before becoming available for operational examination activities.

The lifecycle ensures consistency, governance, and auditability across all examination cycles.

---

## 8.1 Lifecycle Overview

The standard Examination Session lifecycle consists of the following stages:

1. Session Initiation
2. Session Configuration
3. Policy Association
4. Review
5. Approval
6. Activation
7. Operational Consumption
8. Suspension (Optional)
9. Reopening (Optional)
10. Closure
11. Archival

Each stage represents a controlled business transition.

---

## 8.2 Session Initiation

The lifecycle begins with the creation of a proposed Examination Session.

Business activities include:

- Identifying examination purpose
- Selecting session type
- Identifying academic period
- Determining organisational scope
- Establishing initial governance

No operational examination activities may begin during this stage.

---

## 8.3 Session Configuration

Following initiation, the Examination Session is configured with its governing business attributes.

Configuration includes:

- Academic applicability
- Programme applicability
- Curriculum applicability
- Examination policy association
- Session classification
- Administrative ownership

Configuration remains business-oriented and independent of operational planning.

---

## 8.4 Policy Association

Before approval, the Examination Session shall reference an approved Examination Policy.

Policy association determines:

- Eligibility governance
- Registration governance
- Result governance
- Evaluation governance
- Qualification Completion governance

The session consumes policy definitions rather than creating new business rules.

---

## 8.5 Review and Approval

Configured Examination Sessions undergo institutional review.

The review process verifies:

- Policy compliance
- Academic applicability
- Administrative approval
- Institutional governance
- Multi-domain consistency

Only approved sessions may proceed to activation.

---

## 8.6 Session Activation

Approved Examination Sessions become Active.

Activation authorises downstream workflows including:

- Examination Planning
- Examination Registration
- Hall Ticket Generation
- Examination Conduct
- Marks Processing
- Result Processing

Activation marks the official commencement of the examination cycle.

---

## 8.7 Operational Consumption

During the Active state, operational examination workflows consume the Examination Session as their governing business context.

Operational workflows do not modify session governance.

They execute within the approved session until completion.

---

## 8.8 Session Suspension

Institutions may temporarily suspend an Active Examination Session.

Typical reasons include:

- Regulatory directives
- Institutional emergencies
- Natural disasters
- Academic disruptions
- Legal requirements

Suspension pauses operational workflows without terminating the session.

---

## 8.9 Session Reopening

A suspended Examination Session may be reopened following institutional approval.

Upon reopening:

- The session returns to the Active state.
- Operational workflows may resume.
- Historical audit continuity is preserved.

---

## 8.10 Session Closure

Following successful completion of all examination activities, the Examination Session may be formally closed.

Closure confirms that:

- Operational workflows are complete.
- Pending governance actions are resolved.
- Institutional approvals are finalised.
- Historical records are preserved.

Closed sessions shall not accept new operational activities.

---

## 8.11 Session Archival

Closed Examination Sessions enter the archival stage.

Archived sessions remain available for:

- Historical reporting
- Regulatory compliance
- Institutional audit
- Academic reference
- Long-term record retention

Archived sessions remain immutable from a business governance perspective.

# 9. Business Rules

The following business rules govern Examination Session Management across all institutions using SutraERP.

Business rules establish governance for examination sessions while remaining independent of implementation.

---

## 9.1 Session Creation Rules

- Every Examination Session shall belong to a single Institution.
- Every Examination Session shall have a unique business identity.
- Every Examination Session shall define its business purpose before approval.
- Every Examination Session shall reference an approved academic period.
- Examination Sessions shall not exist without institutional ownership.

---

## 9.2 Policy Association Rules

- Every Examination Session shall reference one approved Examination Policy.
- Policies shall be active before a session may be activated.
- Policy changes shall not automatically alter historical Examination Sessions.
- Sessions shall consume policy definitions without modifying policy ownership.

---

## 9.3 Academic Applicability Rules

Every Examination Session shall define its academic applicability.

Applicability may include:

- Academic Year
- Semester
- Trimester
- Programme
- Course
- Curriculum Version
- Department
- Faculty
- Campus

Applicability definitions shall be consistent with collaborating academic domains.

---

## 9.4 Activation Rules

An Examination Session may become Active only when:

- Institutional approval is complete.
- Examination Policy has been associated.
- Academic applicability has been validated.
- Administrative ownership has been assigned.
- Required governance activities have been completed.

Sessions failing validation shall remain unavailable for operational workflows.

---

## 9.5 Operational Consumption Rules

Only Active Examination Sessions may be consumed by downstream examination workflows.

Operational workflows include:

- Examination Planning
- Registration
- Hall Ticket Generation
- Practical Examination
- Theory Examination
- Marks Processing
- Result Processing
- Revaluation
- Supplementary Examination

Inactive sessions shall not participate in operational processing.

---

## 9.6 Suspension Rules

Suspension temporarily pauses operational activities without invalidating the Examination Session.

Suspended sessions:

- Preserve historical information.
- Prevent new operational activities.
- Retain audit history.
- Remain available for authorised administrative review.

---

## 9.7 Closure Rules

An Examination Session may be closed only after:

- Examination activities are completed.
- Result processing is complete.
- Outstanding governance actions are resolved.
- Required approvals are obtained.

Closed sessions become read-only from a business perspective.

---

## 9.8 Version Management Rules

Controlled version management shall preserve:

- Historical consistency
- Business auditability
- Regulatory compliance
- Institutional governance

Historical operational records shall remain associated with the Examination Session version under which they were executed.

---

## 9.9 Audit Rules

Every significant Examination Session activity shall generate audit records.

Examples include:

- Session Creation
- Configuration Changes
- Approval
- Activation
- Suspension
- Reopening
- Closure
- Administrative Override
- Version Creation

---

# 10. Exception Scenarios

The Examination Session Management workflows shall support controlled handling of exceptional business situations.

---

## 10.1 Duplicate Session Creation

If an administrator attempts to create a duplicate Examination Session for the same governed scope:

- Duplicate detection shall occur.
- Administrative review may be initiated.
- Institutional approval may be required before continuation.

---

## 10.2 Invalid Academic Period

If the referenced academic period is unavailable, inactive, or inconsistent:

- Session approval shall be prevented.
- Appropriate administrative action shall be initiated.

---

## 10.3 Missing Examination Policy

An Examination Session without an approved governing Examination Policy shall not proceed to activation.

The session remains in a pre-operational state until compliance is achieved.

---

## 10.4 Incomplete Configuration

Where mandatory configuration is incomplete:

Examples include:

- Missing Programme
- Missing Curriculum
- Missing Academic Period
- Missing Administrative Owner

The session shall remain unavailable for approval.

---

## 10.5 Approval Rejection

Institutional reviewers may reject an Examination Session.

Rejected sessions may:

- Return for modification.
- Be cancelled.
- Be replaced by a newly proposed session.

Operational workflows shall not consume rejected sessions.

---

## 10.6 Activation Failure

If activation prerequisites are not satisfied:

- Activation shall not proceed.
- Operational examination workflows remain blocked.
- Administrative review shall be required.

---

## 10.7 Suspension During Operations

An Active Examination Session may require emergency suspension.

Possible reasons include:

- Government directives
- Natural disasters
- Legal orders
- Institutional emergencies
- Academic disruptions

Operational workflows pause until authorised reopening.

---

## 10.8 Reopening Restrictions

Only authorised institutional administrators may reopen suspended Examination Sessions.

Reopened sessions shall preserve complete historical audit continuity.

---

## 10.9 Closure Validation Failure

If pending examination activities remain incomplete:

Examples include:

- Pending evaluations
- Pending result approvals
- Outstanding governance actions

Session closure shall be prevented until completion.

---

# 11. Events Published

The Examination Session Management domain publishes business events for downstream workflow consumption.

| Event | Published When | Primary Consumers |
|-------|----------------|-------------------|
| Examination Session Created | Session successfully initiated | I03–I16 |
| Examination Session Updated | Configuration modified | I03, I04 |
| Examination Session Submitted | Review initiated | Governance Workflows |
| Examination Session Approved | Institutional approval completed | I03 |
| Examination Session Activated | Session becomes operational | I03–I12 |
| Examination Session Suspended | Operational pause initiated | All Active Examination Workflows |
| Examination Session Reopened | Suspended session restored | I03–I12 |
| Examination Session Closed | Operational completion confirmed | I16 |
| Examination Session Archived | Historical retention begins | Reporting, Audit |

These events represent business milestones rather than technical implementation events.

---

# 12. Events Consumed

The Examination Session Management workflows consume business events published by collaborating domains.

| Event | Published By |
|-------|---------------|
| Academic Period Published | Academic Calendar Domain |
| Curriculum Updated | Curriculum Framework Domain |
| Programme Structure Changed | Curriculum Framework Domain |
| Examination Policy Approved | L2-I01 |
| Institution Updated | Institution Domain |
| Organisation Structure Updated | Organisation Domain |
| Administrative Approval Granted | Governance Processes |
| Audit Request Raised | Audit Infrastructure |

Consumed events influence session governance while ownership remains with the originating domain.

# 13. Multi-Domain Interactions

The Examination Session Management domain collaborates with multiple business domains to establish and govern examination sessions.

Ownership of business information always remains with the originating domain.

---

## 13.1 Institution Domain

The Institution Domain provides the institutional context within which Examination Sessions are established.

Business interactions include:

- Institutional ownership
- Examination governance framework
- Institutional regulations
- Academic identity

The Examination Session domain does not manage institutional information.

---

## 13.2 Organisation Domain

The Organisation Domain provides the organisational structure responsible for examination administration.

Business interactions include:

- Campus applicability
- Faculty ownership
- School ownership
- Department applicability
- Administrative responsibility

Organisation structures remain owned by the Organisation domain.

---

## 13.3 Curriculum Framework Domain

The Curriculum Framework determines the academic entities governed by an Examination Session.

Business interactions include:

- Programme applicability
- Course applicability
- Curriculum versions
- Credit structures
- Academic regulations

Curriculum ownership remains external to the Examination domain.

---

## 13.4 Academic Calendar Domain

The Academic Calendar defines the academic periods within which Examination Sessions operate.

Business interactions include:

- Academic Year
- Semester
- Trimester
- Academic Terms
- Examination Windows

The Examination Session domain consumes academic periods but does not define them.

---

## 13.5 Examination Policy Domain

L2-I01 provides the governance policies applicable to every Examination Session.

Business interactions include:

- Eligibility Policies
- Registration Policies
- Evaluation Policies
- Result Policies
- Qualification Completion Policies
- Revaluation Policies

The Examination Session domain applies approved policies but does not create or modify them.

---

## 13.6 Platform Foundation

Platform Foundation services provide enterprise infrastructure supporting Examination Session workflows.

Collaborating services include:

- Identity Management
- Authentication
- Authorization
- Notification Workflows
- Event Infrastructure
- Audit Infrastructure

These services provide platform capabilities while business ownership remains within the Examination domain.

---

# 14. Ownership Boundaries

The Examination Session Management domain owns the governance of Examination Sessions.

Operational examination activities remain outside the scope of this document.

---

## Owned by L2-I02

The following business capabilities are owned by this domain:

- Examination Session Definition
- Session Classification
- Session Lifecycle
- Session Configuration
- Session Approval
- Session Activation
- Session Suspension
- Session Reopening
- Session Closure
- Session Version Management
- Session Governance

---

## Not Owned by L2-I02

The following capabilities belong to other Examination workflow documents:

| Business Capability | Owning Document |
|---------------------|-----------------|
| Examination Planning & Scheduling | L2-I03 |
| Examination Registration | L2-I04 |
| Hall Ticket Management | L2-I05 |
| Practical & Viva Examination | L2-I06 |
| Theory Examination Conduct | L2-I07 |
| Marks Collection & Evaluation | L2-I08 |
| Result Processing | L2-I09 |
| Result Publication | L2-I10 |
| Revaluation & Rechecking | L2-I11 |
| Supplementary & Improvement Examinations | L2-I12 |
| Academic Completion & Completion Evidence | L2-I13 |
| Malpractice Management | L2-I14 |
| Examination Exception Management | L2-I15 |
| Session Closure & Archival Operations | L2-I16 |

This separation ensures clear ownership across the Examination domain and prevents overlap between workflow documents.

---

# 15. Configurable Session Matrix

Institutions may configure Examination Sessions according to their academic regulations without altering the standard workflow architecture.

| Configuration Area | Examples |
|--------------------|----------|
| Session Types | Regular, Supplementary, Improvement, Makeup, Special |
| Academic Structure | Semester, Annual, Trimester, Modular |
| Applicable Programmes | Institution-defined Programme Selection |
| Applicable Curriculum | Multiple Curriculum Versions |
| Organisation Scope | Institution, Campus, Faculty, Department |
| Examination Modes | Theory, Practical, Viva, Project, Combined |
| Activation Rules | Administrative Approval Requirements |
| Suspension Rules | Institution-specific Emergency Policies |
| Closure Rules | Result Completion, Approval Requirements |
| Versioning Rules | Amendment and Revision Policies |

Configuration affects business behaviour while preserving the standard Examination Session lifecycle.

---

# 16. Audit Requirements

Every Examination Session lifecycle activity shall generate auditable business records.

Auditability supports institutional governance, regulatory compliance, and historical traceability.

---

## Auditable Activities

The following business activities shall be auditable:

- Session Creation
- Configuration Changes
- Administrative Review
- Approval Decisions
- Policy Association
- Activation
- Suspension
- Reopening
- Closure
- Archival
- Version Creation
- Administrative Overrides

---

## Audit Information

Audit records may include:

- Business Activity
- Date and Time
- Responsible Business Actor
- Institutional Context
- Session Identifier
- Previous Business State
- New Business State
- Approval Reference
- Business Justification

Audit implementation is provided by the Platform Audit Infrastructure.

---

# 17. Workflow Completion Criteria

An Examination Session Management workflow is considered complete when:

- Examination Session has been created.
- Business configuration has been completed.
- Academic applicability has been established.
- Governing Examination Policy has been associated.
- Institutional review has been completed.
- Administrative approval has been obtained.
- Session has been activated or appropriately closed.
- All lifecycle activities have been audited.
- Business ownership has been established.
- Session is available for downstream Examination workflows or has been formally archived.

Completion of this workflow establishes the governed business context required for all subsequent Examination operations.

# 18. Examination Session Catalogue

The Examination Session Catalogue defines the canonical business session types supported by the Examination domain.

Operational workflows consume these session definitions without redefining their business purpose.

| Session Type | Business Purpose | Primary Consumers |
|--------------|------------------|-------------------|
| Regular Examination Session | Standard institutional examination cycle | I03–I10 |
| Supplementary Examination Session | Additional opportunity for eligible students | I03, I04, I12 |
| Improvement Examination Session | Grade improvement examination cycle | I03, I04, I12 |
| Makeup Examination Session | Examination for approved exceptional cases | I03, I04, I15 |
| Special Examination Session | Institution-specific examination cycle | I03–I10 |
| Entrance Examination Session | Admission-related examinations (where applicable) | Institution-specific |
| Certification Examination Session | Internal certification examinations | Institution-specific |
| Exit Examination Session | Programme completion examinations | I13 |

Additional session types may be introduced through configuration without altering the workflow architecture.

---

# 19. Session Dependency Matrix

The Examination Session Management domain depends upon business information owned by collaborating domains.

Ownership remains unchanged.

| Session Capability | Depends Upon |
|--------------------|--------------|
| Session Creation | Institution, Organisation |
| Session Classification | Examination Policy |
| Academic Applicability | Academic Calendar, Curriculum |
| Programme Applicability | Curriculum Framework |
| Organisational Scope | Organisation |
| Policy Association | L2-I01 Examination Policy |
| Session Approval | Institution Governance |
| Session Activation | Approved Policy, Academic Calendar |
| Session Suspension | Institution Administration |
| Session Reopening | Institution Administration |
| Session Closure | Completion of Operational Examination Workflows |
| Session Archival | Audit Infrastructure |

The dependency matrix establishes business sequencing while preserving clear ownership boundaries.

---

# 20. Examination Session State Model

Every Examination Session follows a governed business lifecycle.

| State | Description |
|--------|-------------|
| Draft | Session is being defined and configured |
| Under Review | Session is undergoing institutional review |
| Approved | Session has received required approvals |
| Scheduled | Session is approved for a future operational period |
| Active | Session is available for operational examination workflows |
| Suspended | Operational activities are temporarily paused |
| Reopened | Previously suspended session restored to operational state |
| Closed | Operational examination activities have concluded |
| Archived | Historical session retained for audit and reference |

Business rules governing state transitions:

- Every session begins in **Draft**.
- Only reviewed sessions may become **Approved**.
- Approved sessions may enter the **Scheduled** state.
- Scheduled sessions become **Active** at the appropriate academic period.
- Active sessions may be **Suspended** or **Closed**.
- Suspended sessions may be **Reopened**.
- Closed sessions progress to **Archived**.
- Archived sessions remain immutable for historical reference.

---

# 21. Configurable Parameters Catalogue

Institutions may configure Examination Session behaviour without modifying the standard workflow architecture.

| Configuration Category | Examples |
|------------------------|----------|
| Session Types | Regular, Supplementary, Improvement, Makeup, Special |
| Academic Structure | Semester, Annual, Trimester, Quarter |
| Academic Applicability | Academic Year, Semester, Programme |
| Curriculum Applicability | Curriculum Version Selection |
| Organisation Scope | Institution, Campus, Faculty, Department |
| Examination Modes | Theory, Practical, Viva, Combined |
| Session Activation | Approval Workflow, Activation Date |
| Session Suspension | Emergency Policies, Administrative Controls |
| Session Closure | Completion Conditions, Approval Requirements |
| Version Management | Amendment Rules, Version Policies |
| Governance | Review Authority, Approval Hierarchy |

Configuration values remain institution-specific while workflow architecture remains repository standard.

---

# 22. Domain Responsibility Summary

The Examination Session Management domain governs the lifecycle of Examination Sessions.

Operational execution remains the responsibility of subsequent Examination workflow documents.

| Business Capability | Owned by L2-I02 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Session Definition | ✓ | — |
| Session Classification | ✓ | — |
| Session Configuration | ✓ | — |
| Session Lifecycle | ✓ | — |
| Session Approval | ✓ | — |
| Session Activation | ✓ | — |
| Session Suspension | ✓ | — |
| Session Reopening | ✓ | — |
| Session Closure | ✓ | — |
| Examination Planning | — | L2-I03 |
| Examination Registration | — | L2-I04 |
| Hall Ticket Generation | — | L2-I05 |
| Practical & Viva Examination | — | L2-I06 |
| Theory Examination Conduct | — | L2-I07 |
| Marks Collection & Evaluation | — | L2-I08 |
| Result Processing | — | L2-I09 |
| Result Publication | — | L2-I10 |
| Revaluation & Rechecking | — | L2-I11 |
| Supplementary & Improvement Examinations | — | L2-I12 |
| Academic Completion & Completion Evidence | — | L2-I13 |
| Malpractice Management | — | L2-I14 |
| Examination Exception Management | — | L2-I15 |
| Examination Session Closure & Archival Operations | — | L2-I16 |

---

# 23. Repository Integration Summary

L2-I02 establishes the canonical business governance for Examination Sessions within the SutraERP Examination domain.

Every operational Examination workflow shall execute within an approved and active Examination Session governed by this document.

The relationship between the foundational Examination documents is as follows:

| Document | Responsibility |
|----------|----------------|
| L2-I01 | Defines Examination Policies and Configuration governing examination operations |
| L2-I02 | Defines the lifecycle, governance, and management of Examination Sessions |
| L2-I03 | Consumes approved Examination Sessions to plan and schedule examinations |
| L2-I04 | Registers eligible candidates within an active Examination Session |
| L2-I05 | Generates Hall Tickets for registered candidates |
| L2-I06 | Conducts Practical and Viva examinations |
| L2-I07 | Conducts Theory examinations |
| L2-I08 | Collects and evaluates examination marks |
| L2-I09 | Processes examination results |
| L2-I10 | Publishes approved examination results |
| L2-I11 | Manages revaluation and rechecking workflows |
| L2-I12 | Conducts supplementary, backlog, and improvement examinations |
| L2-I13 | Determines academic completion and completion evidence |
| L2-I14 | Manages malpractice investigations and disciplinary actions |
| L2-I15 | Handles examination exceptions and contingencies |
| L2-I16 | Closes examination sessions and archives historical records |

L2-I02 does not perform operational examination activities.

Its responsibility is to establish the governed business context within which all Examination workflows operate.

---

## Entities / Aggregates

This document does not use "Entity" or "Aggregate" terminology directly. Its canonical business object is the Examination Session, whose supported types are catalogued in the Examination Session Catalogue (Section 18) — Regular, Supplementary, Improvement, Makeup, Special, Entrance, Certification, and Exit Examination Sessions. Ownership of the Examination Session business capability is further defined under Ownership Boundaries (Section 14).

---

# End of Document





