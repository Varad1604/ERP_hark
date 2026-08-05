# L2-I06 — Practical & Viva Examination Conduct Workflows

**Section:** I — Examination  
**Status:** Draft  
**Document Owner:** Examinations — Practical & Viva Conduct

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
| Workflow | §8 Practical & Viva Examination Lifecycle Workflows; §17 Workflow Completion Criteria; L2-I06 — Practical & Viva Examination Conduct Workflows |
| State Machine | §20 Assessment Conduct State Model |
| Contracts | — not present — |
| Events | §11 Events Published; §12 Events Consumed; §4.9 Event-Driven Examination Conduct |
| Dependencies | §19 Assessment Dependency Matrix |
| Locked Decisions | — not present — |

> **Gap:** 3 required section(s) have no counterpart in this document: Entities, Contracts, Locked Decisions. Recorded, not authored.

---

# 1. Purpose

This document defines the canonical business workflows governing the conduct of all non-theory examinations within the SutraERP Examination domain.

It establishes the enterprise framework for managing practical examinations, laboratory examinations, viva voce, project demonstrations, project viva, studio and jury assessments, clinical assessments, field assessments, performance assessments, skill assessments, and other institutionally approved non-theory examination activities.

The document governs examination conduct beginning with candidate participation in an authorised assessment and concluding with completion of the assessment, after which the assessment becomes available for Marks Collection & Evaluation workflows.

This document standardises examination conduct while allowing institutions to configure operational policies without altering the canonical repository architecture.

---

# 2. Scope

The Practical & Viva Examination Conduct domain governs the business processes required to execute authorised non-theory examinations.

The scope includes:

- Practical Examination Conduct
- Laboratory Examination Conduct
- Viva Voce Conduct
- Project Demonstration Conduct
- Project Viva Conduct
- Studio and Jury Assessments
- Clinical Assessments
- Skill-Based Assessments
- Performance Assessments
- Field Assessments
- Internship and Industrial Assessments (where governed as examinations)
- Candidate Check-in
- Hall Ticket Validation
- Examiner Assignment
- Examination Batch Management
- Examination Attendance
- Assessment Execution
- Conduct Completion
- Conduct Exceptions
- Conduct Lifecycle Governance

This document does not govern:

- Examination Policy Management
- Examination Session Management
- Examination Planning & Scheduling
- Examination Registration
- Hall Ticket Management
- Theory Examination Conduct
- Marks Collection & Evaluation
- Result Processing
- Revaluation & Rechecking
- Supplementary Examination Processing
- Academic Completion
- Qualification Completion Processing

Ownership of these business capabilities remains with their respective workflow documents.

---

# 3. Repository Context

The Practical & Viva Examination Conduct domain is positioned after Hall Ticket Management within the Examination repository.

It consumes authorised examination participation established by previous examination workflows and governs the operational execution of non-theory examinations.

This document consumes business information from:

- L2-I01 — Examination Policy & Configuration Workflows
- L2-I02 — Examination Session Management Workflows
- L2-I03 — Examination Planning & Scheduling Workflows
- L2-I04 — Examination Registration Workflows
- L2-I05 — Hall Ticket Management Workflows

It collaborates with:

- Person & Identity Domain
- Organisation Domain
- Academic Calendar Domain
- Platform Foundation Services

It produces authoritative examination conduct records consumed by:

- L2-I08 — Marks Collection & Evaluation Workflows

This document owns examination conduct only and does not own evaluation or academic outcome processing.

---

# 4. Architectural Principles

The following architectural principles govern Practical & Viva Examination Conduct across the SutraERP repository.

---

## 4.1 Authorised Participation Before Conduct

Every practical or viva examination shall be conducted only for candidates possessing valid examination authorisation established through Hall Ticket Management.

---

## 4.2 Conduct Before Evaluation

Assessment activities shall be completed before Marks Collection & Evaluation begins.

Completion of examination conduct establishes readiness for downstream evaluation workflows but does not perform evaluation itself.

---

## 4.3 Unified Assessment Conduct Framework

All non-theory examination activities shall follow a common business workflow regardless of assessment type.

Institution-specific assessment types remain configurable while adhering to the canonical conduct lifecycle.

---

## 4.4 Examiner-Governed Assessment

Every assessment shall be conducted under institutionally authorised examiners.

Examiner assignment shall follow institutional governance policies while remaining independent of evaluation ownership.

---

## 4.5 Attendance as Part of Examination Conduct

Candidate attendance for practical and viva examinations forms part of examination conduct.

Attendance establishes examination participation and shall be completed before assessment execution.

Attendance records do not represent evaluation outcomes.

---

## 4.6 Controlled Conduct Lifecycle

Every assessment shall progress through a governed lifecycle from preparation to completion.

Business state transitions shall be controlled, auditable, and governed by institutional policies.

---

## 4.7 Configuration Before Customisation

Institutions may configure examination conduct policies, assessment categories, examiner allocation models, batch management approaches, attendance rules, and operational procedures without modifying the canonical workflow architecture.

---

## 4.8 Multi-Domain Collaboration

Practical & Viva Examination Conduct collaborates with Registration, Hall Ticket Management, Examination Planning, Academic Calendar, Identity, Organisation, and Platform Foundation domains while preserving clear ownership boundaries.

---

## 4.9 Event-Driven Examination Conduct

Significant business transitions during examination conduct shall publish business events for downstream workflows.

Examples include:

- Candidate Checked In
- Attendance Recorded
- Assessment Started
- Assessment Completed
- Conduct Closed

---

## 4.10 Complete Auditability

Every significant business activity performed during examination conduct shall generate auditable records supporting institutional governance, compliance, and historical traceability.

---

# 5. Business Capabilities

The Practical & Viva Examination Conduct domain provides the following canonical business capabilities.

These capabilities govern the execution of non-theory examinations while maintaining clear ownership boundaries with other Examination workflows.

---

## 5.1 Assessment Preparation

The domain shall prepare authorised assessment activities before examination commencement.

Preparation may include:

- Assessment Readiness
- Candidate Readiness
- Examiner Readiness
- Venue Readiness
- Assessment Resource Verification
- Batch Preparation

Assessment Preparation establishes operational readiness without modifying examination schedules.

---

## 5.2 Candidate Check-in

The domain shall manage candidate participation at the commencement of assessment activities.

Candidate Check-in may include:

- Candidate Arrival
- Identity Verification
- Hall Ticket Verification
- Batch Confirmation
- Examination Entry Authorization

Successful Check-in establishes candidate eligibility to participate in the scheduled assessment.

---

## 5.3 Hall Ticket Validation

The domain shall validate Hall Ticket authorization before allowing assessment participation.

Validation may confirm:

- Hall Ticket Validity
- Candidate Authorization
- Examination Session
- Scheduled Assessment
- Institutional Eligibility

Hall Ticket ownership remains with L2-I05.

---

## 5.4 Examiner Assignment

The domain shall govern examiner allocation for assessment activities.

Examiner Assignment may include:

- Internal Examiner Allocation
- External Examiner Allocation
- Observer Assignment
- Panel Formation
- Assessment Responsibility Allocation

Assignment policies remain institution configurable.

---

## 5.5 Assessment Batch Management

The domain shall manage operational batches for assessment execution.

Batch Management may include:

- Batch Creation
- Candidate Allocation
- Batch Capacity Management
- Batch Scheduling
- Batch Completion

Batch Management supports operational execution without altering examination planning.

---

## 5.6 Assessment Attendance

The domain shall record candidate participation during assessment activities.

Attendance management may include:

- Candidate Presence
- Candidate Absence
- Late Arrival
- Approved Exception
- Attendance Finalization

Attendance establishes participation only and does not represent academic performance.

---

## 5.7 Assessment Conduct

The domain shall govern execution of authorised assessment activities.

Assessment Conduct may include:

- Practical Examination
- Viva Voce
- Project Demonstration
- Clinical Assessment
- Studio Assessment
- Jury Assessment
- Skill Assessment
- Performance Assessment
- Field Assessment
- Internship Assessment

Assessment execution follows institutionally approved examination procedures.

---

## 5.8 Assessment Completion

The domain shall manage completion of assessment activities.

Completion includes:

- Assessment Closure
- Attendance Finalization
- Examiner Confirmation
- Conduct Completion
- Readiness for Evaluation

Completion establishes the official conclusion of examination conduct.

---

## 5.9 Conduct Exception Management

The domain shall manage exceptional situations occurring during assessment execution.

Examples include:

- Candidate Absence
- Examiner Unavailability
- Venue Issues
- Equipment Failure
- Assessment Cancellation
- Emergency Suspension
- Administrative Override

Exception handling shall preserve institutional governance and auditability.

---

## 5.10 Conduct Lifecycle Governance

The domain shall govern the complete lifecycle of assessment conduct.

Lifecycle governance includes:

- Preparation
- Check-in
- Validation
- Attendance
- Assessment Execution
- Completion
- Closure
- Archival

Lifecycle governance ensures consistent examination conduct across institutions.

---

# 6. Business Actors

The following business actors participate in Practical & Viva Examination Conduct workflows.

| Business Actor | Responsibility |
|----------------|----------------|
| Candidate | Participates in authorised assessment activities |
| Internal Examiner | Conducts and supervises assessments |
| External Examiner | Participates where institutional policy requires external assessment |
| Chief Examiner | Oversees assessment governance where applicable |
| Examination Administrator | Coordinates assessment operations |
| Controller of Examination | Governs examination conduct policies |
| Department Administration | Supports departmental examination activities |
| Laboratory / Assessment Coordinator | Manages operational readiness of assessment venues |
| Observer | Monitors assessment activities where required |
| Person & Identity Domain | Provides candidate identity information |
| Hall Ticket Management Domain | Provides examination entry authorization |
| Examination Planning Domain | Provides approved examination schedules |
| Audit Infrastructure | Maintains conduct audit history |

Business actors collaborate while ownership remains with their respective domains.

---

# 7. Practical & Viva Examination Framework

The Practical & Viva Examination Framework establishes the canonical business structure governing execution of all non-theory examinations.

The framework standardises assessment conduct while allowing institutions to configure assessment models according to academic requirements.

---

## 7.1 Assessment Conduct Objectives

The framework enables institutions to:

- Conduct authorised assessments.
- Verify candidate participation.
- Allocate examiners.
- Execute assessment activities.
- Record attendance.
- Complete assessment operations.
- Prepare assessments for evaluation.

Assessment Conduct governs operational execution only.

---

## 7.2 Assessment Types Framework

The framework supports multiple institutionally approved assessment categories.

Examples include:

- Practical Examination
- Laboratory Assessment
- Viva Voce
- Project Demonstration
- Project Viva
- Clinical Assessment
- Studio Assessment
- Jury Assessment
- Skill Assessment
- Performance Assessment
- Field Assessment
- Internship Assessment

Additional assessment types may be configured by institutions without modifying the canonical architecture.

---

## 7.3 Candidate Participation Framework

Candidate participation is governed through authorised examination entry.

Participation requires:

- Valid Hall Ticket
- Approved Examination Registration
- Scheduled Assessment
- Institutional Eligibility

Only authorised candidates may participate in assessment activities.

---

## 7.4 Examiner Framework

Assessment activities shall be conducted by authorised examiners.

Institutions may configure:

- Single Examiner
- Multiple Examiners
- Internal Assessment
- External Assessment
- Examination Panels

Examiner governance remains institution configurable.

---

## 7.5 Assessment Batch Framework

Assessment activities may be organised into operational batches.

Batch management supports:

- Candidate Distribution
- Resource Allocation
- Examiner Allocation
- Session Management
- Operational Efficiency

Batch structures do not modify academic schedules.

---

## 7.6 Attendance Framework

Attendance forms an integral part of assessment conduct.

Attendance establishes:

- Candidate Participation
- Candidate Absence
- Assessment Eligibility
- Examination Presence

Attendance ownership remains within examination conduct.

---

## 7.7 Assessment Completion Framework

Assessment completion establishes the successful conclusion of operational examination activities.

Completion confirms:

- Assessment Conduct Finished
- Attendance Finalized
- Examination Activity Closed
- Ready for Marks Collection & Evaluation

Assessment completion does not perform evaluation or determine academic outcomes.

# 8. Practical & Viva Examination Lifecycle Workflows

Every assessment activity progresses through a governed business lifecycle.

The lifecycle ensures authorised, controlled, auditable, and standardised execution of all non-theory examinations.

---

## 8.1 Lifecycle Overview

The standard Practical & Viva Examination Conduct lifecycle consists of the following stages:

1. Assessment Preparation
2. Candidate Check-in
3. Hall Ticket Validation
4. Examiner Assignment
5. Batch Allocation
6. Attendance Recording
7. Assessment Commencement
8. Assessment Execution
9. Assessment Completion
10. Conduct Verification
11. Ready for Marks Collection & Evaluation
12. Archival

Each stage represents a controlled business transition.

---

## 8.2 Assessment Preparation

Assessment Preparation establishes operational readiness before examination activities begin.

Preparation may include:

- Assessment Schedule Confirmation
- Venue Readiness
- Laboratory Readiness
- Equipment Availability
- Assessment Material Readiness
- Examiner Availability
- Candidate Allocation
- Batch Preparation

Preparation does not modify examination planning.

---

## 8.3 Candidate Check-in

Candidates report for their scheduled assessment.

Candidate Check-in may include:

- Candidate Arrival
- Identity Verification
- Hall Ticket Presentation
- Batch Verification
- Examination Entry Confirmation

Successful Check-in authorises participation in the scheduled assessment.

---

## 8.4 Hall Ticket Validation

Before assessment begins, Hall Ticket authorisation shall be validated.

Validation confirms:

- Hall Ticket Validity
- Candidate Authorisation
- Examination Session
- Scheduled Assessment
- Institutional Eligibility

Only valid Hall Tickets may proceed to assessment execution.

---

## 8.5 Examiner Assignment

Authorised examiners shall be assigned before assessment commencement.

Assignment may include:

- Internal Examiner
- External Examiner
- Assessment Panel
- Observer
- Assessment Coordinator

Examiner assignment shall comply with institutional policies.

---

## 8.6 Batch Allocation

Candidates may be organised into operational batches.

Batch Allocation supports:

- Laboratory Capacity
- Equipment Availability
- Examiner Workload
- Time Slot Management
- Operational Efficiency

Batch allocation shall not alter the approved Examination Schedule.

---

## 8.7 Attendance Recording

Attendance shall be recorded before or during assessment commencement according to institutional policy.

Attendance outcomes may include:

- Present
- Absent
- Late
- Excused
- Special Approval

Attendance establishes examination participation only.

---

## 8.8 Assessment Commencement

Assessment Commencement marks the official start of examination activities.

Commencement confirms:

- Assessment Environment Ready
- Candidates Authorised
- Examiners Present
- Assessment Resources Available

Assessment officially begins after commencement.

---

## 8.9 Assessment Execution

Assessment Execution represents the operational conduct of the examination.

Execution varies according to assessment type.

Examples include:

- Practical Experiment
- Laboratory Exercise
- Viva Voce
- Project Demonstration
- Clinical Examination
- Studio Assessment
- Jury Assessment
- Skill Demonstration
- Performance Assessment
- Field Assessment

Assessment execution shall comply with institutional examination policies.

---

## 8.10 Assessment Completion

Assessment Completion concludes examination activities.

Completion confirms:

- Candidate Participation Finished
- Assessment Activities Completed
- Examiner Responsibilities Concluded
- Operational Conduct Closed

Completion establishes readiness for post-conduct processing.

---

## 8.11 Conduct Verification

Institutional verification confirms successful completion of examination conduct.

Verification may include:

- Attendance Verification
- Assessment Completion Confirmation
- Examiner Confirmation
- Administrative Review

Verification ensures assessment integrity before downstream processing.

---

## 8.12 Ready for Marks Collection & Evaluation

Successfully completed assessments become available for Marks Collection & Evaluation.

Ownership transfers operationally to L2-I08.

This document concludes responsibility once examination conduct has been successfully completed.

---

## 8.13 Archival

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

The following business rules govern Practical & Viva Examination Conduct across all institutions.

These rules establish consistent governance while allowing institutional configuration.

---

## 9.1 Authorised Participation Rules

Only candidates possessing valid Hall Tickets shall participate in assessment activities.

Unauthorised candidates shall not participate unless governed through approved exception workflows.

---

## 9.2 Examiner Assignment Rules

Every assessment shall be conducted by authorised examiners.

Institutions may configure:

- Single Examiner
- Multiple Examiners
- Internal Examiner
- External Examiner
- Assessment Panels

Only authorised examiners may supervise assessment activities.

---

## 9.3 Attendance Rules

Attendance shall be recorded for every scheduled assessment.

Attendance shall accurately represent candidate participation.

Attendance shall not represent academic performance.

---

## 9.4 Batch Management Rules

Operational batches shall:

- Follow approved examination schedules.
- Respect institutional capacity.
- Support efficient examination conduct.

Batch changes shall not alter examination planning.

---

## 9.5 Assessment Conduct Rules

Assessment execution shall:

- Follow institutional policies.
- Use authorised assessment procedures.
- Preserve fairness.
- Maintain examination integrity.

Assessment conduct shall remain independent of evaluation.

---

## 9.6 Assessment Completion Rules

Assessment shall be completed only after:

- Candidate participation concludes.
- Attendance is finalised.
- Examiner responsibilities conclude.
- Operational verification completes.

Completed assessments become eligible for Marks Collection & Evaluation.

---

## 9.7 Exception Handling Rules

Exceptional situations shall follow institutionally approved procedures.

Examples include:

- Candidate Absence
- Examiner Replacement
- Equipment Failure
- Venue Change
- Emergency Suspension

Exception handling shall preserve complete auditability.

---

## 9.8 Audit Rules

Every significant examination conduct activity shall generate audit records.

Examples include:

- Candidate Check-in
- Hall Ticket Validation
- Examiner Assignment
- Attendance Recording
- Assessment Commencement
- Assessment Completion
- Administrative Override
- Conduct Closure

All business transitions shall remain historically traceable.

---

# 10. Exception Scenarios

The Practical & Viva Examination Conduct domain shall support controlled handling of exceptional business situations.

---

## 10.1 Candidate Absent

If a candidate does not report for assessment:

- Attendance shall record absence.
- Assessment participation shall not occur.
- Downstream workflows shall receive the appropriate business status.

---

## 10.2 Invalid Hall Ticket

If Hall Ticket validation fails:

- Candidate participation shall be denied.
- Assessment shall not commence.
- Institutional exception procedures may apply.

---

## 10.3 Examiner Unavailable

Where an assigned examiner becomes unavailable:

- Authorised replacement procedures may be initiated.
- Assessment shall continue only after institutional approval.

---

## 10.4 Equipment or Laboratory Failure

Operational failures affecting assessment conduct may require:

- Temporary suspension
- Batch rescheduling
- Venue reassignment
- Administrative intervention

Institutional policies govern recovery procedures.

---

## 10.5 Venue Change

Assessment venues may change under authorised circumstances.

Venue changes shall:

- Preserve examination integrity.
- Notify affected participants.
- Maintain complete audit records.

---

## 10.6 Candidate Late Arrival

Institutions may define configurable policies for late arrival.

Late participation shall comply with institutional examination regulations.

---

## 10.7 Assessment Interrupted

Assessment activities interrupted by emergencies may be:

- Suspended
- Rescheduled
- Restarted
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

The Practical & Viva Examination Conduct domain publishes business events representing significant milestones during assessment execution.

These events communicate authoritative business state transitions to downstream workflows while preserving ownership boundaries.

| Event | Published When | Primary Consumers |
|-------|----------------|-------------------|
| Assessment Prepared | Assessment readiness completed | Examination Administration |
| Candidate Checked In | Candidate successfully checked in | Attendance, Audit |
| Hall Ticket Validated | Candidate authorised for assessment | Examination Conduct |
| Examiner Assigned | Examiner allocation completed | Audit |
| Batch Allocated | Candidate assigned to assessment batch | Examination Operations |
| Attendance Recorded | Candidate attendance finalised | L2-I08 |
| Assessment Commenced | Assessment officially started | Audit |
| Assessment Completed | Assessment execution completed | L2-I08 |
| Conduct Verified | Examination conduct verified | L2-I08 |
| Conduct Closed | Conduct lifecycle completed | Audit |
| Conduct Archived | Assessment archived | Reporting, Audit |

Business events represent authoritative conduct lifecycle transitions.

---

# 12. Events Consumed

The Practical & Viva Examination Conduct workflow consumes business events published by collaborating domains.

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
| Administrative Approval Granted | Governance Processes |
| Audit Request Raised | Audit Infrastructure |

Consumed events influence assessment execution without transferring ownership.

---

# 13. Multi-Domain Interactions

The Practical & Viva Examination Conduct domain collaborates with multiple business domains to execute authorised assessments.

Business ownership remains with the originating domains.

---

## 13.1 Examination Policy Domain

L2-I01 governs institutional examination policies.

Business interactions include:

- Assessment Policies
- Attendance Policies
- Examiner Policies
- Exception Policies
- Assessment Governance

Conduct workflows consume policies but do not modify them.

---

## 13.2 Examination Session Management Domain

L2-I02 governs the Examination Session.

Business interactions include:

- Active Examination Sessions
- Session Validity
- Session Lifecycle

Assessment activities shall occur only within active Examination Sessions.

---

## 13.3 Examination Planning & Scheduling Domain

L2-I03 provides approved assessment schedules.

Business interactions include:

- Assessment Dates
- Time Slots
- Assessment Venues
- Published Timetables
- Batch Scheduling Inputs

Conduct workflows consume schedules without modifying them.

---

## 13.4 Examination Registration Domain

L2-I04 provides authorised examination participation.

Business interactions include:

- Confirmed Registration
- Registered Assessment Components
- Candidate Eligibility

Registration ownership remains external to examination conduct.

---

## 13.5 Hall Ticket Management Domain

L2-I05 provides examination entry authorisation.

Business interactions include:

- Hall Ticket Validation
- Examination Entry Authorisation
- Hall Ticket Status

Assessment conduct consumes Hall Ticket information but does not modify Hall Ticket governance.

---

## 13.6 Person & Identity Domain

The Person & Identity domain provides candidate and examiner identity information.

Business interactions include:

- Candidate Identity
- Examiner Identity
- Identity Verification

Identity ownership remains outside the Examination domain.

---

## 13.7 Organisation Domain

The Organisation domain provides institutional organisational structures.

Business interactions include:

- Departments
- Faculties
- Assessment Centres
- Administrative Units

Organisation ownership remains external to examination conduct.

---

## 13.8 Platform Foundation

Platform Foundation provides enterprise infrastructure supporting assessment execution.

Supporting services include:

- Authentication
- Authorization
- Notification Workflows
- Event Infrastructure
- Audit Infrastructure

Platform services support business workflows without owning examination conduct.

---

# 14. Ownership Boundaries

The Practical & Viva Examination Conduct domain owns operational execution of authorised non-theory examinations.

It governs assessment execution from candidate check-in until assessment completion.

---

## Owned by L2-I06

The following business capabilities are owned by this domain:

- Assessment Preparation
- Candidate Check-in
- Hall Ticket Validation During Conduct
- Examiner Assignment
- Assessment Batch Management
- Assessment Attendance
- Assessment Conduct
- Assessment Completion
- Conduct Exception Management
- Conduct Lifecycle Governance

---

## Not Owned by L2-I06

The following business capabilities belong to other workflow documents.

| Business Capability | Owning Document |
|---------------------|-----------------|
| Examination Policy Management | L2-I01 |
| Examination Session Management | L2-I02 |
| Examination Planning & Scheduling | L2-I03 |
| Examination Registration | L2-I04 |
| Hall Ticket Management | L2-I05 |
| Theory Examination Conduct | L2-I07 |
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

# 15. Configurable Assessment Matrix

Institutions may configure assessment conduct without modifying the canonical workflow architecture.

| Configuration Area | Examples |
|--------------------|----------|
| Assessment Types | Practical, Viva, Clinical, Project, Jury |
| Examiner Model | Single, Multiple, Internal, External |
| Batch Strategy | Fixed, Dynamic, Capacity-Based |
| Attendance Policy | Mandatory, Grace Period, Exceptions |
| Identity Verification | Mandatory, Institution-Specific |
| Assessment Duration | Institution Configurable |
| Resource Allocation | Laboratory, Equipment, Assessment Room |
| Exception Handling | Administrative Approval Rules |
| Conduct Verification | Mandatory, Optional |
| Notification Policy | Candidate, Examiner, Administrator |

Configuration changes operational behaviour while preserving repository-standard business workflows.

# 16. Audit Requirements

Every Practical & Viva Examination Conduct activity shall generate auditable business records.

Auditability supports institutional governance, regulatory compliance, operational transparency, dispute resolution, and historical traceability.

---

## Auditable Activities

The following business activities shall be auditable:

- Assessment Preparation
- Candidate Check-in
- Hall Ticket Validation
- Examiner Assignment
- Batch Allocation
- Attendance Recording
- Assessment Commencement
- Assessment Execution
- Assessment Completion
- Conduct Verification
- Administrative Override
- Conduct Closure
- Conduct Archival

Every significant business transition shall be recorded.

---

## Audit Information

Audit records may include:

- Assessment Identifier
- Candidate Identifier
- Examination Session
- Examination Registration Reference
- Hall Ticket Reference
- Assessment Type
- Examiner Information
- Batch Information
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

A Practical & Viva Examination Conduct workflow is considered complete when:

- Candidate participation has concluded.
- Hall Ticket validation has been successfully completed.
- Attendance has been finalised.
- Assessment activities have been completed.
- Examiner responsibilities have concluded.
- Conduct verification has been completed.
- Assessment completion events have been published.
- Audit records have been generated.
- Assessment is available for Marks Collection & Evaluation.

Completion establishes the authoritative operational record of assessment execution.

---

# 18. Assessment Conduct Catalogue

The Assessment Conduct Catalogue defines the canonical business capabilities owned by the Practical & Viva Examination Conduct domain.

These capabilities are consumed by downstream examination workflows without redefining examination conduct.

| Assessment Capability | Business Purpose | Primary Consumers |
|------------------------|------------------|-------------------|
| Assessment Preparation | Establish operational readiness | I06 |
| Candidate Check-in | Authorise candidate participation | I06 |
| Hall Ticket Validation | Validate examination entry | I06 |
| Examiner Assignment | Allocate authorised examiners | I06 |
| Assessment Batch Management | Organise assessment execution | I06 |
| Assessment Attendance | Record candidate participation | I08 |
| Assessment Conduct | Execute examination activity | I08 |
| Assessment Completion | Complete operational assessment | I08 |
| Conduct Verification | Verify assessment integrity | I08 |
| Conduct Lifecycle Governance | Govern assessment lifecycle | Audit |

Additional capabilities may be introduced through institutional configuration without modifying the canonical workflow.

---

# 19. Assessment Dependency Matrix

The Practical & Viva Examination Conduct domain depends upon business information owned by collaborating domains.

Ownership remains with the originating business domains.

| Assessment Capability | Depends Upon |
|------------------------|--------------|
| Assessment Preparation | Examination Planning (L2-I03), Examination Policy (L2-I01) |
| Candidate Check-in | Hall Ticket (L2-I05) |
| Hall Ticket Validation | Hall Ticket Management (L2-I05) |
| Examiner Assignment | Organisation Domain |
| Batch Management | Examination Planning (L2-I03) |
| Attendance Recording | Candidate Participation |
| Assessment Conduct | Approved Assessment Schedule |
| Assessment Completion | Completed Assessment Activity |
| Conduct Verification | Attendance & Assessment Completion |
| Conduct Archival | Audit Infrastructure |

The dependency matrix establishes business sequencing while preserving ownership boundaries.

---

# 20. Assessment Conduct State Model

Every assessment progresses through a governed business lifecycle.

| State | Description |
|--------|-------------|
| Planned | Assessment prepared for execution |
| Ready | Operational readiness confirmed |
| Check-in | Candidate verification in progress |
| Validated | Hall Ticket successfully validated |
| In Progress | Assessment currently being conducted |
| Suspended | Assessment temporarily interrupted |
| Completed | Assessment execution finished |
| Verified | Conduct successfully verified |
| Closed | Operational lifecycle completed |
| Archived | Historical conduct record retained |

Business rules governing state transitions:

- Every assessment begins in **Planned**.
- Prepared assessments become **Ready**.
- Candidate participation begins during **Check-in**.
- Successful verification establishes the **Validated** state.
- Active assessments remain **In Progress** until operational completion.
- Interrupted assessments may enter **Suspended** before resuming.
- Successfully completed assessments become **Completed**.
- Administrative verification establishes the **Verified** state.
- Verified assessments become **Closed**.
- Closed assessments progress to **Archived** for historical retention.

---

# 21. Configurable Parameters Catalogue

Institutions may configure Practical & Viva Examination Conduct while preserving repository-standard workflows.

| Configuration Category | Examples |
|------------------------|----------|
| Assessment Types | Practical, Viva, Project, Clinical, Jury |
| Examiner Allocation | Internal, External, Mixed Panels |
| Batch Configuration | Fixed Size, Dynamic Allocation |
| Attendance Rules | Mandatory, Grace Period, Exception Approval |
| Hall Ticket Validation | Mandatory, Institution-Specific |
| Assessment Duration | Institution Configurable |
| Venue Allocation | Laboratory, Clinical Centre, Assessment Room |
| Identity Verification | Institution Configurable |
| Conduct Verification | Mandatory, Optional |
| Notification Behaviour | Candidate, Examiner, Administrator Alerts |

Institution-specific values may vary without changing workflow architecture.

---

# 22. Domain Responsibility Summary

The Practical & Viva Examination Conduct domain governs the operational execution of authorised non-theory examinations.

It establishes the authoritative record of examination participation before evaluation begins.

| Business Capability | Owned by L2-I06 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Assessment Preparation | ✓ | — |
| Candidate Check-in | ✓ | — |
| Hall Ticket Validation During Conduct | ✓ | L2-I05 owns Hall Ticket lifecycle |
| Examiner Assignment | ✓ | — |
| Assessment Batch Management | ✓ | — |
| Assessment Attendance | ✓ | — |
| Assessment Conduct | ✓ | — |
| Assessment Completion | ✓ | — |
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

The Practical & Viva Examination Conduct domain governs operational assessment execution only and does not perform academic evaluation or determine examination outcomes.

---

# 23. Repository Integration Summary

L2-I06 establishes the canonical business workflows governing Practical & Viva Examination Conduct within the SutraERP Examination domain.

Every non-theory examination shall be conducted through the workflows defined in this document.

The relationship between Examination workflow documents is as follows:

| Document | Responsibility |
|----------|----------------|
| L2-I01 | Examination Policy & Configuration |
| L2-I02 | Examination Session Management |
| L2-I03 | Examination Planning & Scheduling |
| L2-I04 | Examination Registration |
| L2-I05 | Hall Ticket Management |
| **L2-I06** | **Practical & Viva Examination Conduct** |
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

L2-I06 consumes authorised examination participation established by L2-I05 and produces authoritative assessment conduct records consumed by L2-I08.

The domain governs examination conduct only and does not own marks collection, evaluation, result computation, or academic outcome determination.

---

# Appendix A. Practical & Viva Examination Conduct Business Model

This appendix defines the canonical business model governing the operational execution of non-theory examinations.

It establishes the reusable business concepts consumed by downstream examination workflows while preserving the ownership boundaries defined in this document.

---

## A.1 Practical & Viva Conduct Business Model

Practical & Viva Examination Conduct represents the institution's authorised execution of non-theory examination activities.

Examination Conduct begins after successful Hall Ticket authorisation and concludes when assessment activities are completed and become available for Marks Collection & Evaluation.

Conceptually:

```
Examination Session
          │
          ▼
     Hall Ticket
          │
          ▼
Assessment Activity
          │
          ▼
Assessment Execution
          │
          ▼
Completed Assessment
```

This document governs operational examination execution only.

---

## A.2 Assessment Type Framework

The Practical & Viva Examination Conduct workflow supports multiple institutionally approved assessment types through a common business framework.

Supported assessment types include:

- Practical Examination
- Laboratory Examination
- Viva Voce
- Project Demonstration
- Project Viva
- Clinical Assessment
- Studio Assessment
- Jury Assessment
- Skill Assessment
- Performance Assessment
- Field Assessment
- Internship / Industrial Assessment (where governed as an examination)

Additional assessment types may be configured without changing the canonical workflow architecture.

---

## A.3 Operational Participation Framework

Every assessment activity follows a common operational participation model.

```
Authorised Hall Ticket
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
Assessment Execution
          │
          ▼
Assessment Completion
          │
          ▼
Ready for Marks Collection & Evaluation
```

This operational sequence applies regardless of assessment type.

---

## A.4 Examiner & Batch Framework

Assessment execution is governed through authorised examiner allocation and operational batch management.

The framework supports:

- Internal Examiners
- External Examiners
- Assessment Panels
- Observers
- Batch-wise Assessment Execution
- Resource-Based Batch Allocation

Operational batches are temporary execution groups created solely for conducting examination activities.

They do **not** represent academic batches, student cohorts, programmes, or organisational structures.

---

## A.5 Ownership Clarification

The Practical & Viva Examination Conduct domain owns:

- Assessment Preparation
- Candidate Check-in
- Examination Entry Verification
- Examiner Assignment
- Operational Batch Management
- Attendance Recording
- Assessment Execution
- Assessment Completion
- Conduct Verification
- Conduct Lifecycle

The Practical & Viva Examination Conduct domain does **not** own:

- Candidate Identity
- Hall Ticket Lifecycle
- Examination Registration
- Examination Scheduling
- Marks Collection
- Evaluation
- Result Processing
- Academic Decisions

Ownership of these capabilities remains with their respective workflow documents.

---

## A.6 Downstream Consumption

The outputs of Practical & Viva Examination Conduct are consumed by downstream Examination workflows.

| Workflow | Business Consumption |
|----------|----------------------|
| L2-I07 Theory Examination Conduct | Common examination governance principles where applicable |
| L2-I08 Marks Collection & Evaluation | Completed assessment records, attendance status and examiner completion |
| L2-I09 Result Processing & Computation | Evaluated assessment outcomes |
| L2-I10 Result Publication | Published assessment results |

Practical & Viva Examination Conduct concludes once operational assessment execution has been successfully completed and responsibility transitions to Marks Collection & Evaluation.

---

## A.7 Governing Business Principles

The following principles govern Practical & Viva Examination Conduct across the SutraERP repository:

- Every assessment shall originate from an authorised Hall Ticket.
- Every assessment shall be conducted within an active Examination Session.
- Attendance forms part of examination conduct.
- Examination entry verification shall precede assessment execution.
- Assessment completion shall precede Marks Collection & Evaluation.
- Practical & Viva Examination Conduct governs operational execution only and shall not perform academic evaluation or determine examination outcomes.

This appendix serves as the canonical operational business model for Practical & Viva Examination Conduct throughout the SutraERP Examination repository.

# End of Document







