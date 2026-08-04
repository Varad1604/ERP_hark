# L2-I08 — Marks Collection & Evaluation Workflows

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
| Workflow | §8 Evaluation Lifecycle Workflows; §17 Workflow Completion Criteria; L2-I08 — Marks Collection & Evaluation Workflows |
| State Machine | §20 Marks Collection & Evaluation State Model |
| Contracts | — not present — |
| Events | §11 Events Published; §12 Events Consumed; §4.9 Event-Driven Evaluation |
| Dependencies | §19 Marks Collection & Evaluation Dependency Matrix |
| Locked Decisions | — not present — |

> **Gap:** 3 required section(s) have no counterpart in this document: Entities, Contracts, Locked Decisions. Recorded, not authored.

---

# 1. Purpose

This document defines the canonical business workflows governing Marks Collection and Evaluation within the SutraERP Examination domain.

It establishes the enterprise framework for collecting assessment evidence, assigning evaluators, conducting academic evaluation, validating awarded marks, performing moderation, approving evaluated marks, and preparing final approved marks for downstream Result Processing workflows.

The document governs evaluation activities beginning with receipt of completed assessment artifacts from examination conduct workflows and concluding with publication of institutionally approved marks for Result Processing & Computation.

This document standardises academic evaluation while allowing institutions to configure evaluation methodologies, moderation policies, marking schemes, and approval processes without modifying the canonical repository architecture.

---

# 2. Scope

The Marks Collection & Evaluation domain governs the complete academic evaluation lifecycle following examination conduct.

The scope includes:

- Marks Collection
- Assessment Evidence Collection
- Answer Script Management
- Evaluator Assignment
- Evaluation Planning
- Manual Evaluation
- Digital Evaluation
- Anonymous Evaluation
- Single Evaluation
- Double Evaluation
- Multiple Evaluation
- Internal Assessment Marks Collection
- Practical Assessment Marks Collection
- Theory Assessment Marks Collection
- Moderation
- Marks Validation
- Marks Approval
- Marks Finalisation
- Evaluation Lifecycle Governance

This document does not govern:

- Examination Policy Management
- Examination Session Management
- Examination Planning & Scheduling
- Examination Registration
- Hall Ticket Management
- Practical & Viva Examination Conduct
- Theory Examination Conduct
- Result Processing & Computation
- Result Publication
- Grade Calculation
- SGPA / CGPA Calculation
- Revaluation & Rechecking
- Academic Completion

Ownership of these business capabilities remains with their respective workflow documents.

---

# 3. Repository Context

The Marks Collection & Evaluation domain is positioned immediately after Examination Conduct within the Examination repository.

It consumes completed assessment evidence generated through Practical & Viva Examination Conduct and Theory Examination Conduct and transforms that evidence into institutionally approved marks.

This document consumes business information from:

- L2-I01 — Examination Policy & Configuration Workflows
- L2-I02 — Examination Session Management Workflows
- L2-I03 — Examination Planning & Scheduling Workflows
- L2-I04 — Examination Registration Workflows
- L2-I05 — Hall Ticket Management Workflows
- L2-I06 — Practical & Viva Examination Conduct Workflows
- L2-I07 — Theory Examination Conduct Workflows

It collaborates with:

- Person & Identity Domain
- Organisation Domain
- Academic Calendar Domain
- Platform Foundation Services

It produces authoritative Final Approved Marks consumed by:

- L2-I09 — Result Processing & Computation Workflows

This document owns academic evaluation only.

It does not own examination conduct, academic result computation, grade calculation, or result publication.

---

# 4. Architectural Principles

The following architectural principles govern Marks Collection & Evaluation across the SutraERP repository.

---

## 4.1 Conduct Before Evaluation

Academic evaluation shall begin only after authorised examination conduct has been completed.

Evaluation consumes completed assessment evidence without modifying examination conduct records.

---

## 4.2 Evaluation Before Result Processing

Result Processing shall consume only institutionally approved marks.

Result computation shall never operate on provisional, incomplete, or unapproved evaluation outcomes.

---

## 4.3 Separation of Conduct and Evaluation

Examination conduct and academic evaluation represent separate business responsibilities.

Operational examination execution remains outside this document.

---

## 4.4 Configurable Evaluation Models

Institutions may configure evaluation methodologies while preserving repository-standard workflows.

Supported models may include:

- Manual Evaluation
- Digital Evaluation
- Anonymous Evaluation
- Single Evaluation
- Double Evaluation
- Multiple Evaluation
- Committee Evaluation

The evaluation framework remains institution configurable.

---

## 4.5 Independent Evaluator Governance

Academic evaluation shall be performed only by authorised evaluators.

Evaluator assignment shall comply with institutional policies regarding qualification, independence, confidentiality, and conflict-of-interest requirements.

---

## 4.6 Controlled Moderation

Moderation shall occur before marks become final.

Moderation improves consistency, fairness, and institutional compliance without altering ownership of academic evaluation.

---

## 4.7 Approved Marks as the Authoritative Outcome

The authoritative output of this domain is Final Approved Marks.

These marks become the sole academic input for Result Processing & Computation.

No downstream workflow shall modify evaluation history maintained by this domain.

---

## 4.8 Multi-Domain Collaboration

Marks Collection & Evaluation collaborates with Examination Conduct, Registration, Hall Ticket Management, Examination Policy, Academic Calendar, Organisation, Identity, and Platform Foundation domains while preserving ownership boundaries.

---

## 4.9 Event-Driven Evaluation

Significant evaluation lifecycle transitions shall publish business events.

Examples include:

- Evaluation Assigned
- Evaluation Started
- Evaluation Completed
- Moderation Completed
- Marks Approved
- Marks Finalised

---

## 4.10 Complete Auditability

Every significant evaluation activity shall generate auditable business records supporting institutional governance, academic integrity, regulatory compliance, and historical traceability.

---
# 5. Business Capabilities

The Marks Collection & Evaluation domain provides the following canonical business capabilities.

These capabilities govern the complete academic evaluation lifecycle while maintaining clear ownership boundaries with Examination Conduct and Result Processing workflows.

---

## 5.1 Assessment Evidence Collection

The domain shall govern collection of completed assessment evidence produced by examination conduct workflows.

Assessment Evidence Collection may include:

- Theory Answer Books
- Practical Assessment Records
- Viva Assessment Records
- Project Assessment Records
- Internal Assessment Submissions
- Digital Assessment Artefacts

Collection establishes readiness for academic evaluation.

---

## 5.2 Answer Script Management

The domain shall manage assessment scripts throughout the evaluation lifecycle.

Answer Script Management may include:

- Script Registration
- Script Identification
- Script Allocation
- Script Tracking
- Script Return
- Script Closure

Script management ensures complete accountability throughout evaluation.

---

## 5.3 Evaluator Assignment

The domain shall govern allocation of authorised evaluators.

Evaluator Assignment may include:

- Subject-wise Allocation
- Internal Evaluator Assignment
- External Evaluator Assignment
- Chief Evaluator Assignment
- Evaluation Panel Assignment
- Workload Distribution

Evaluator allocation shall comply with institutional policies.

---

## 5.4 Evaluation Planning

The domain shall coordinate academic evaluation activities.

Evaluation Planning may include:

- Evaluation Scheduling
- Evaluation Centre Allocation
- Evaluation Batch Planning
- Evaluator Capacity Planning
- Workload Balancing

Planning supports timely and consistent evaluation.

---

## 5.5 Academic Evaluation

The domain shall govern academic assessment of submitted examination responses.

Academic Evaluation may include:

- Manual Evaluation
- Digital Evaluation
- On-Screen Evaluation
- Anonymous Evaluation
- Rubric-Based Evaluation
- Criteria-Based Assessment

Evaluation determines candidate performance according to approved academic standards.

---

## 5.6 Multiple Evaluation Models

The domain shall support configurable evaluation methodologies.

Supported models may include:

- Single Evaluation
- Double Evaluation
- Multiple Evaluation
- Blind Evaluation
- Independent Evaluation
- Committee Evaluation

Institutions may configure evaluation models according to academic regulations.

---

## 5.7 Marks Collection

The domain shall collect marks awarded through authorised evaluation processes.

Marks Collection may include:

- Theory Marks
- Practical Marks
- Viva Marks
- Project Marks
- Internal Assessment Marks
- Continuous Assessment Marks

Collected marks remain provisional until institutional approval.

---

## 5.8 Moderation

The domain shall govern moderation activities performed after academic evaluation.

Moderation may include:

- Statistical Moderation
- Manual Moderation
- Exceptional Review
- Evaluation Consistency Review
- Borderline Case Review

Moderation improves fairness and consistency before marks become final.

---

## 5.9 Marks Validation

The domain shall validate evaluated marks before institutional approval.

Validation may include:

- Marks Completeness
- Marks Consistency
- Policy Compliance
- Calculation Verification
- Evaluation Integrity

Validation ensures readiness for approval.

---

## 5.10 Marks Approval

The domain shall govern institutional approval of validated marks.

Approval may include:

- Evaluator Approval
- Chief Evaluator Approval
- Department Approval
- Controller Approval
- Institutional Approval

Only approved marks may become final.

---

## 5.11 Marks Finalisation

The domain shall establish Final Approved Marks.

Marks Finalisation includes:

- Final Marks Confirmation
- Evaluation Closure
- Final Approval Recording
- Downstream Publication Readiness

Final Approved Marks become the authoritative academic output of this domain.

---

## 5.12 Evaluation Lifecycle Governance

The domain shall govern the complete lifecycle of academic evaluation.

Lifecycle governance includes:

- Assessment Collection
- Evaluator Assignment
- Evaluation
- Moderation
- Validation
- Approval
- Finalisation
- Archival

Lifecycle governance ensures consistent academic evaluation across all institutions.

---

# 6. Business Actors

The following business actors participate in Marks Collection & Evaluation workflows.

| Business Actor | Responsibility |
|----------------|----------------|
| Candidate | Receives evaluated marks |
| Evaluator | Performs academic evaluation |
| Chief Evaluator | Supervises evaluation quality |
| External Evaluator | Performs independent evaluation |
| Internal Evaluator | Performs institutional evaluation |
| Evaluation Coordinator | Coordinates evaluation operations |
| Department Head | Reviews evaluation activities |
| Controller of Examination | Governs institutional evaluation |
| Examination Administrator | Coordinates evaluation processes |
| Moderation Committee | Reviews moderated evaluations |
| Person & Identity Domain | Provides evaluator and candidate identity |
| Organisation Domain | Provides academic organisational structure |
| Audit Infrastructure | Maintains evaluation audit history |

Business actors collaborate while ownership remains within their respective domains.

---

# 7. Evaluation Framework

The Evaluation Framework establishes the canonical business structure governing academic evaluation within the Examination domain.

The framework standardises evaluation while allowing institution-specific academic regulations.

---

## 7.1 Evaluation Objectives

The framework enables institutions to:

- Collect assessment evidence.
- Assign authorised evaluators.
- Evaluate academic performance.
- Apply institutional moderation.
- Validate awarded marks.
- Approve evaluated marks.
- Produce Final Approved Marks.

Evaluation concludes before Result Processing begins.

---

## 7.2 Assessment Evidence Framework

Academic evaluation begins only after assessment evidence has been received.

Assessment evidence may originate from:

- Theory Examination Conduct
- Practical Examination Conduct
- Viva Examination Conduct
- Project Assessment
- Internal Assessment

Evidence remains immutable throughout evaluation.

---

## 7.3 Evaluation Assignment Framework

Evaluation activities shall be assigned to authorised evaluators.

Assignment considers:

- Subject Expertise
- Academic Responsibility
- Institutional Policy
- Evaluation Capacity
- Conflict of Interest

Only authorised evaluators may perform academic evaluation.

---

## 7.4 Evaluation Model Framework

Institutions may configure one or more evaluation methodologies.

Supported evaluation models include:

- Single Evaluation
- Double Evaluation
- Multiple Evaluation
- Blind Evaluation
- Anonymous Evaluation
- Committee Evaluation

Evaluation models remain institution configurable.

---

## 7.5 Marks Governance Framework

Marks progress through a controlled academic lifecycle.

The lifecycle consists of:

- Awarded Marks
- Moderated Marks
- Validated Marks
- Approved Marks
- Final Approved Marks

Only Final Approved Marks are released to downstream Result Processing.

---

## 7.6 Moderation Framework

Moderation improves academic consistency without replacing evaluator responsibility.

Moderation may include:

- Random Sampling
- Borderline Review
- Exceptional Review
- Statistical Review
- Academic Committee Review

Institutional moderation policies remain configurable.

---

## 7.7 Approval Framework

Evaluation concludes only after authorised institutional approval.

Approval confirms:

- Evaluation Completion
- Moderation Completion
- Validation Success
- Institutional Acceptance

Approved marks become the authoritative academic record for Result Processing.

# 8. Evaluation Lifecycle Workflows

Every academic evaluation progresses through a governed lifecycle.

The lifecycle ensures consistent, transparent, auditable, and institutionally governed academic assessment.

---

## 8.1 Lifecycle Overview

The standard Marks Collection & Evaluation lifecycle consists of the following stages:

1. Assessment Evidence Collection
2. Evaluation Planning
3. Evaluator Assignment
4. Assessment Distribution
5. Academic Evaluation
6. Marks Collection
7. Moderation
8. Marks Validation
9. Marks Approval
10. Marks Finalisation
11. Ready for Result Processing
12. Archival

Each stage represents a controlled business transition.

---

## 8.2 Assessment Evidence Collection

Evaluation begins after authorised assessment evidence has been received from Examination Conduct workflows.

Assessment evidence may include:

- Theory Answer Books
- Practical Assessment Records
- Viva Assessment Records
- Project Assessments
- Internal Assessment Records
- Digital Assessment Artefacts

Assessment evidence establishes readiness for academic evaluation.

---

## 8.3 Evaluation Planning

Institutional evaluation activities shall be planned before evaluation begins.

Planning may include:

- Evaluation Scheduling
- Evaluation Centre Planning
- Workload Distribution
- Evaluation Batch Planning
- Resource Allocation

Planning supports timely completion of academic evaluation.

---

## 8.4 Evaluator Assignment

Authorised evaluators shall be assigned according to institutional policies.

Assignment may include:

- Internal Evaluators
- External Evaluators
- Chief Evaluators
- Subject Experts
- Evaluation Panels

Evaluator assignment shall preserve academic integrity and institutional governance.

---

## 8.5 Assessment Distribution

Assessment evidence shall be distributed to authorised evaluators.

Distribution may include:

- Physical Script Allocation
- Digital Script Allocation
- Anonymous Script Distribution
- Evaluation Batch Allocation

Distribution establishes evaluator responsibility without transferring business ownership.

---

## 8.6 Academic Evaluation

Authorised evaluators perform academic assessment of submitted examination responses.

Evaluation activities may include:

- Answer Assessment
- Rubric-Based Assessment
- Criteria-Based Evaluation
- Practical Assessment Review
- Viva Assessment Review
- Project Assessment Review

Evaluation results remain provisional until institutional approval.

---

## 8.7 Marks Collection

Awarded marks shall be collected from authorised evaluators.

Marks Collection may include:

- Theory Marks
- Practical Marks
- Viva Marks
- Project Marks
- Internal Assessment Marks
- Continuous Assessment Marks

Collected marks establish the basis for moderation and validation.

---

## 8.8 Moderation

Moderation improves consistency and fairness before institutional approval.

Moderation activities may include:

- Statistical Moderation
- Manual Moderation
- Borderline Review
- Exceptional Review
- Academic Committee Review

Moderation remains part of academic evaluation.

---

## 8.9 Marks Validation

Validated marks shall satisfy institutional academic policies.

Validation confirms:

- Completeness
- Consistency
- Academic Policy Compliance
- Evaluation Integrity
- Calculation Accuracy

Only successfully validated marks proceed for approval.

---

## 8.10 Marks Approval

Validated marks shall undergo institutional approval.

Approval may occur through:

- Evaluator Approval
- Chief Evaluator Approval
- Department Approval
- Controller Approval
- Institutional Approval

Only approved marks may become final.

---

## 8.11 Marks Finalisation

Institutionally approved marks become Final Approved Marks.

Finalisation confirms:

- Evaluation Completion
- Moderation Completion
- Validation Completion
- Institutional Approval

Final Approved Marks represent the authoritative academic outcome of this domain.

---

## 8.12 Ready for Result Processing

Final Approved Marks become available for Result Processing & Computation.

Operational responsibility transitions to L2-I09.

This document concludes responsibility once Final Approved Marks have been established.

---

## 8.13 Archival

Completed evaluation records enter archival status after institutional closure.

Archived records remain available for:

- Academic Audit
- Regulatory Compliance
- Historical Reference
- Institutional Reporting
- Quality Assurance

Archival preserves academic evaluation history without modifying approved marks.

---

# 9. Business Rules

The following business rules govern Marks Collection & Evaluation across all institutions.

These rules establish consistent academic governance while allowing institutional configuration.

---

## 9.1 Evaluation Eligibility Rules

Academic evaluation shall begin only after authorised examination conduct has been completed.

Incomplete or unauthorised assessment evidence shall not enter evaluation.

---

## 9.2 Evaluator Assignment Rules

Only authorised evaluators shall perform academic evaluation.

Evaluator assignments shall:

- Follow institutional policies.
- Maintain evaluator independence.
- Prevent unauthorised evaluation.

---

## 9.3 Assessment Integrity Rules

Assessment evidence shall remain protected throughout evaluation.

Assessment artefacts shall:

- Remain traceable.
- Preserve academic integrity.
- Prevent unauthorised modification.
- Support complete auditability.

---

## 9.4 Evaluation Rules

Academic evaluation shall comply with approved institutional evaluation policies.

Evaluation shall:

- Follow approved marking schemes.
- Apply authorised evaluation methodologies.
- Produce academically defensible outcomes.

---

## 9.5 Moderation Rules

Moderation shall occur before institutional approval whenever required by institutional policy.

Moderation shall:

- Improve evaluation consistency.
- Preserve academic fairness.
- Maintain evaluation transparency.

---

## 9.6 Validation Rules

All evaluated marks shall undergo validation before approval.

Validation shall verify:

- Marks Completeness
- Marks Accuracy
- Policy Compliance
- Evaluation Consistency

Incomplete validation shall prevent marks approval.

---

## 9.7 Approval Rules

Only authorised academic authorities may approve evaluated marks.

Approval shall establish institutional acceptance of academic evaluation.

Unapproved marks shall remain provisional.

---

## 9.8 Finalisation Rules

Only institutionally approved marks shall become Final Approved Marks.

Final Approved Marks shall become the authoritative academic input for Result Processing & Computation.

---

## 9.9 Audit Rules

Every significant academic evaluation activity shall generate audit records.

Examples include:

- Evaluator Assignment
- Evaluation Start
- Evaluation Completion
- Marks Submission
- Moderation
- Validation
- Approval
- Administrative Override
- Finalisation

All evaluation transitions shall remain historically traceable.

---

# 10. Exception Scenarios

The Marks Collection & Evaluation domain shall support controlled handling of exceptional academic situations.

---

## 10.1 Missing Assessment Evidence

If required assessment evidence is unavailable:

- Evaluation shall not begin.
- Responsible authorities shall be notified.
- Evaluation shall resume only after evidence becomes available.

---

## 10.2 Evaluator Unavailable

If an assigned evaluator becomes unavailable:

- A replacement evaluator may be assigned.
- Evaluation continuity shall follow institutional policy.
- Complete audit records shall be maintained.

---

## 10.3 Evaluation Discrepancy

Where significant differences occur between multiple evaluations:

- Institutional discrepancy resolution procedures shall apply.
- Additional evaluation may be initiated where required.

---

## 10.4 Moderation Required

Where institutional policies require moderation:

- Evaluation shall remain provisional.
- Final approval shall occur only after moderation is completed.

---

## 10.5 Validation Failure

If validation identifies inconsistencies:

- Marks shall return for correction.
- Approval shall not proceed until validation succeeds.

---

## 10.6 Approval Rejected

Where approval is denied:

- Evaluation shall return for review.
- Corrective academic actions shall follow institutional procedures.

---

## 10.7 Assessment Evidence Compromised

Where assessment integrity is questioned:

- Evaluation may be suspended.
- Institutional investigation procedures shall apply.
- Academic authorities shall determine further action.

---

## 10.8 Administrative Override

Authorised officers may perform controlled administrative overrides under exceptional circumstances.

Every override shall:

- Require authorised approval.
- Record business justification.
- Preserve complete audit history.
- Publish appropriate business events.

# 11. Events Published

The Marks Collection & Evaluation domain publishes business events representing significant academic evaluation lifecycle transitions.

These events communicate authoritative evaluation outcomes to downstream workflows while preserving business ownership.

| Event | Published When | Primary Consumers |
|-------|----------------|-------------------|
| Assessment Evidence Received | Assessment evidence successfully collected | Evaluation Operations |
| Evaluator Assigned | Evaluator allocation completed | Audit |
| Evaluation Started | Academic evaluation commenced | Audit |
| Evaluation Completed | Evaluation finished | Moderation |
| Marks Submitted | Evaluator submitted marks | Validation |
| Moderation Started | Moderation initiated | Audit |
| Moderation Completed | Moderation completed | Validation |
| Marks Validated | Validation completed successfully | Approval |
| Marks Approved | Institutional approval completed | Result Processing |
| Final Approved Marks Available | Marks finalised | L2-I09 |
| Evaluation Closed | Evaluation lifecycle completed | Audit |
| Evaluation Archived | Evaluation records archived | Reporting, Audit |

Business events represent authoritative academic evaluation lifecycle transitions.

---

# 12. Events Consumed

The Marks Collection & Evaluation workflow consumes business events published by collaborating domains.

Ownership of business information remains with the publishing domain.

| Event | Published By |
|-------|---------------|
| Examination Policy Updated | L2-I01 |
| Examination Session Activated | L2-I02 |
| Examination Schedule Published | L2-I03 |
| Examination Registration Confirmed | L2-I04 |
| Hall Ticket Published | L2-I05 |
| Practical Examination Completed | L2-I06 |
| Theory Examination Completed | L2-I07 |
| Answer Books Collected | L2-I07 |
| Practical Assessment Completed | L2-I06 |
| Candidate Information Updated | Person & Identity Domain |
| Academic Calendar Updated | Academic Calendar Domain |
| Administrative Approval Granted | Governance Processes |
| Audit Request Raised | Audit Infrastructure |

Consumed events influence evaluation without transferring ownership.

---

# 13. Multi-Domain Interactions

The Marks Collection & Evaluation domain collaborates with multiple business domains while preserving ownership boundaries.

Business ownership always remains with the originating domain.

---

## 13.1 Examination Policy Domain

L2-I01 governs institutional examination policies.

Business interactions include:

- Evaluation Policies
- Marking Policies
- Moderation Policies
- Approval Policies
- Academic Integrity Policies

Evaluation consumes institutional policies without modifying them.

---

## 13.2 Examination Session Management Domain

L2-I02 governs Examination Sessions.

Business interactions include:

- Active Examination Sessions
- Session Validity
- Session Governance

Evaluation occurs only within authorised Examination Sessions.

---

## 13.3 Examination Planning & Scheduling Domain

L2-I03 provides approved examination schedules.

Business interactions include:

- Subject Schedules
- Assessment Timing
- Evaluation Windows
- Academic Calendars

Evaluation consumes scheduling information without modifying planning.

---

## 13.4 Examination Registration Domain

L2-I04 provides authorised examination participation.

Business interactions include:

- Registered Candidates
- Registered Subjects
- Candidate Eligibility

Registration ownership remains external to evaluation.

---

## 13.5 Hall Ticket Management Domain

L2-I05 provides examination participation authorisation.

Business interactions include:

- Candidate Authorisation
- Examination Participation
- Assessment Eligibility

Hall Ticket ownership remains outside this document.

---

## 13.6 Practical & Viva Examination Conduct

L2-I06 provides completed practical assessment evidence.

Business interactions include:

- Practical Assessment Records
- Viva Assessment Records
- Project Assessment Records
- Completed Practical Conduct

Evaluation consumes completed assessment evidence only.

---

## 13.7 Theory Examination Conduct

L2-I07 provides completed theory examination evidence.

Business interactions include:

- Answer Books
- Attendance Confirmation
- Completed Examination Conduct
- Operational Handover

Evaluation begins after operational responsibility has concluded.

---

## 13.8 Person & Identity Domain

The Person & Identity domain provides identity information.

Business interactions include:

- Candidate Identity
- Evaluator Identity
- Academic Staff Identity

Identity ownership remains outside the Examination domain.

---

## 13.9 Organisation Domain

The Organisation domain provides academic organisational structures.

Business interactions include:

- Faculties
- Departments
- Academic Units
- Evaluation Authorities

Organisation ownership remains external.

---

## 13.10 Platform Foundation

Platform Foundation provides enterprise infrastructure supporting evaluation workflows.

Supporting services include:

- Authentication
- Authorization
- Notification Workflows
- Event Infrastructure
- Audit Infrastructure

Platform services support business workflows without owning academic evaluation.

---

# 14. Ownership Boundaries

The Marks Collection & Evaluation domain owns academic evaluation from receipt of completed assessment evidence until Final Approved Marks become available for Result Processing.

---

## Owned by L2-I08

The following business capabilities are owned by this domain:

- Assessment Evidence Collection
- Answer Script Management
- Evaluation Planning
- Evaluator Assignment
- Assessment Distribution
- Academic Evaluation
- Marks Collection
- Moderation
- Marks Validation
- Marks Approval
- Marks Finalisation
- Evaluation Lifecycle Governance

---

## Not Owned by L2-I08

The following business capabilities belong to other workflow documents.

| Business Capability | Owning Document |
|---------------------|-----------------|
| Examination Policy Management | L2-I01 |
| Examination Session Management | L2-I02 |
| Examination Planning & Scheduling | L2-I03 |
| Examination Registration | L2-I04 |
| Hall Ticket Management | L2-I05 |
| Practical & Viva Examination Conduct | L2-I06 |
| Theory Examination Conduct | L2-I07 |
| Result Processing & Computation | L2-I09 |
| Result Publication | L2-I10 |
| Revaluation & Rechecking | L2-I11 |
| Supplementary / Backlog / Improvement Examinations | L2-I12 |
| Academic Completion & Completion Evidence | L2-I13 |
| Malpractice & Examination Committee | L2-I14 |
| Examination Exception & Contingency Management | L2-I15 |
| Examination Session Closure & Archival | L2-I16 |

This separation maintains clear ownership across the Examination repository.

---

# 15. Configurable Evaluation Matrix

Institutions may configure academic evaluation without modifying the canonical repository architecture.

| Configuration Area | Examples |
|--------------------|----------|
| Evaluation Model | Single, Double, Multiple, Committee |
| Evaluation Method | Manual, Digital, On-Screen |
| Evaluator Assignment | Manual, Automatic, Panel-Based |
| Anonymous Evaluation | Enabled, Disabled |
| Moderation Policy | Statistical, Manual, Committee |
| Validation Rules | Mandatory, Institution Configurable |
| Approval Hierarchy | Evaluator, Chief Evaluator, Controller |
| Internal Assessment Integration | Enabled, Disabled |
| Practical Assessment Integration | Enabled, Disabled |
| Marks Finalisation Policy | Immediate, Multi-Level Approval |

Configuration affects operational behaviour while preserving canonical workflow architecture.

# 16. Audit Requirements

Every Marks Collection & Evaluation activity shall generate auditable business records.

Auditability supports academic integrity, institutional governance, regulatory compliance, quality assurance, dispute resolution, and historical traceability.

---

## Auditable Activities

The following business activities shall be auditable:

- Assessment Evidence Collection
- Answer Script Registration
- Evaluator Assignment
- Assessment Distribution
- Evaluation Commencement
- Evaluation Completion
- Marks Submission
- Moderation
- Marks Validation
- Marks Approval
- Marks Finalisation
- Administrative Override
- Evaluation Closure
- Evaluation Archival

Every significant business transition shall be recorded.

---

## Audit Information

Audit records may include:

- Examination Identifier
- Examination Session
- Subject
- Candidate Identifier
- Assessment Identifier
- Answer Script Reference
- Evaluation Batch
- Evaluator Identifier
- Evaluation Method
- Awarded Marks
- Moderation Outcome
- Validation Status
- Approval Status
- Previous Business State
- New Business State
- Responsible Business Actor
- Business Justification
- Date and Time
- Approval Reference

Audit implementation is provided by the Platform Audit Infrastructure.

---

# 17. Workflow Completion Criteria

A Marks Collection & Evaluation workflow is considered complete when:

- Assessment evidence has been successfully collected.
- Evaluation planning has been completed.
- Evaluators have been assigned.
- Assessment evidence has been evaluated.
- Marks have been collected.
- Moderation has been completed where applicable.
- Validation has been successfully completed.
- Institutional approval has been obtained.
- Final Approved Marks have been established.
- Evaluation events have been published.
- Audit records have been generated.
- Final Approved Marks are available for Result Processing & Computation.

Completion establishes the authoritative academic evaluation record.

---

# 18. Marks Collection & Evaluation Catalogue

The Marks Collection & Evaluation Catalogue defines the canonical business capabilities owned by this domain.

These capabilities are consumed by downstream workflows without redefining academic evaluation.

| Business Capability | Business Purpose | Primary Consumers |
|---------------------|------------------|-------------------|
| Assessment Evidence Collection | Receive completed assessment evidence | I08 |
| Answer Script Management | Govern assessment artefacts | I08 |
| Evaluation Planning | Organise evaluation activities | I08 |
| Evaluator Assignment | Allocate authorised evaluators | I08 |
| Assessment Distribution | Allocate assessment evidence | I08 |
| Academic Evaluation | Evaluate academic performance | I08 |
| Marks Collection | Collect awarded marks | I08 |
| Moderation | Improve evaluation consistency | I08 |
| Marks Validation | Verify evaluation correctness | I08 |
| Marks Approval | Obtain institutional approval | I08 |
| Marks Finalisation | Produce Final Approved Marks | I09 |
| Evaluation Lifecycle Governance | Govern complete evaluation lifecycle | Audit |

Additional capabilities may be introduced through institutional configuration without modifying the canonical workflow architecture.

---

# 19. Marks Collection & Evaluation Dependency Matrix

The Marks Collection & Evaluation domain depends upon business information owned by collaborating domains.

Ownership remains with the originating business domains.

| Business Capability | Depends Upon |
|---------------------|--------------|
| Assessment Evidence Collection | L2-I06, L2-I07 |
| Answer Script Management | Completed Examination Conduct |
| Evaluation Planning | Examination Schedule (L2-I03) |
| Evaluator Assignment | Organisation Domain |
| Assessment Distribution | Evaluation Planning |
| Academic Evaluation | Assessment Evidence Collection |
| Marks Collection | Completed Academic Evaluation |
| Moderation | Marks Collection |
| Marks Validation | Moderated Marks |
| Marks Approval | Validated Marks |
| Marks Finalisation | Approved Marks |
| Evaluation Archival | Audit Infrastructure |

The dependency matrix establishes business sequencing while preserving ownership boundaries.

---

# 20. Marks Collection & Evaluation State Model

Every academic evaluation progresses through a governed business lifecycle.

| State | Description |
|--------|-------------|
| Pending | Assessment evidence awaiting evaluation |
| Planned | Evaluation activities prepared |
| Assigned | Evaluator allocation completed |
| In Evaluation | Academic evaluation underway |
| Evaluated | Evaluation completed |
| Moderation | Moderation in progress |
| Validated | Marks successfully validated |
| Approved | Institutionally approved |
| Finalised | Final Approved Marks established |
| Closed | Evaluation lifecycle completed |
| Archived | Historical evaluation record retained |

Business rules governing state transitions:

- Every evaluation begins in **Pending**.
- Planned evaluations become **Assigned**.
- Assigned evaluations enter **In Evaluation**.
- Completed evaluations become **Evaluated**.
- Where required, evaluations proceed through **Moderation**.
- Successfully moderated evaluations become **Validated**.
- Validated evaluations become **Approved**.
- Approved evaluations become **Finalised**.
- Finalised evaluations progress to **Closed**.
- Closed evaluations become **Archived**.

---

# 21. Configurable Parameters Catalogue

Institutions may configure Marks Collection & Evaluation while preserving repository-standard workflows.

| Configuration Category | Examples |
|------------------------|----------|
| Evaluation Model | Single, Double, Multiple, Blind |
| Evaluation Method | Manual, Digital, On-Screen |
| Script Allocation | Manual, Automatic |
| Evaluator Assignment | Manual, Rule-Based |
| Moderation Strategy | Statistical, Manual, Committee |
| Validation Rules | Mandatory, Configurable |
| Approval Workflow | Single-Level, Multi-Level |
| Internal Assessment Integration | Enabled, Disabled |
| Practical Assessment Integration | Enabled, Disabled |
| Marks Finalisation Policy | Immediate, Scheduled |

Institution-specific values may vary without changing workflow architecture.

---

# 22. Domain Responsibility Summary

The Marks Collection & Evaluation domain governs academic evaluation from receipt of completed assessment evidence until Final Approved Marks are produced.

| Business Capability | Owned by L2-I08 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Assessment Evidence Collection | ✓ | Assessment evidence produced by I06/I07 |
| Answer Script Management | ✓ | — |
| Evaluation Planning | ✓ | — |
| Evaluator Assignment | ✓ | Organisation provides academic personnel |
| Academic Evaluation | ✓ | — |
| Marks Collection | ✓ | — |
| Moderation | ✓ | — |
| Marks Validation | ✓ | — |
| Marks Approval | ✓ | — |
| Marks Finalisation | ✓ | I09 consumes final marks |
| Result Processing | — | L2-I09 |
| Result Publication | — | L2-I10 |
| Revaluation & Rechecking | — | L2-I11 |
| Academic Completion | — | L2-I13 |
| Malpractice Management | — | L2-I14 |

The Marks Collection & Evaluation domain governs academic evaluation only and does not own result computation, grading, publication, or academic completion.

---

# 23. Repository Integration Summary

L2-I08 establishes the canonical business workflows governing Marks Collection & Evaluation within the SutraERP Examination domain.

Every assessment shall be evaluated through the workflows defined in this document.

The relationship between Examination workflow documents is as follows:

| Document | Responsibility |
|----------|----------------|
| L2-I01 | Examination Policy & Configuration |
| L2-I02 | Examination Session Management |
| L2-I03 | Examination Planning & Scheduling |
| L2-I04 | Examination Registration |
| L2-I05 | Hall Ticket Management |
| L2-I06 | Practical & Viva Examination Conduct |
| L2-I07 | Theory Examination Conduct |
| **L2-I08** | **Marks Collection & Evaluation** |
| L2-I09 | Result Processing & Computation |
| L2-I10 | Result Publication |
| L2-I11 | Revaluation & Rechecking |
| L2-I12 | Supplementary / Backlog / Improvement Examination Execution |
| L2-I13 | Academic Completion, CGPA & Completion Evidence Workflows |
| L2-I14 | Malpractice & Examination Committee |
| L2-I15 | Examination Exception & Contingency Management |
| L2-I16 | Examination Session Closure & Archival |

L2-I08 consumes completed assessment evidence from L2-I06 and L2-I07 and produces Final Approved Marks consumed by L2-I09.

The domain governs academic evaluation only and does not own result computation, grading, academic decisions, or result publication.

---

# Appendix A — Marks Collection & Evaluation Business Reference Model

This appendix provides the canonical business reference model for Marks Collection & Evaluation.

The appendix defines the conceptual structure governing academic evaluation while preserving the ownership boundaries established in the main document.

---

# A.1 Marks Collection & Evaluation Business Model

The Marks Collection & Evaluation domain governs the academic assessment lifecycle following examination conduct.

The canonical business model is illustrated below.

```text
Completed Assessment Evidence
        │
        ▼
Assessment Evidence Collection
        │
        ▼
Answer Script Management
        │
        ▼
Evaluator Assignment
        │
        ▼
Academic Evaluation
        │
        ▼
Marks Collection
        │
        ▼
Moderation
        │
        ▼
Marks Validation
        │
        ▼
Institutional Approval
        │
        ▼
Final Approved Marks
        │
        ▼
Ready for Result Processing
```

This business model governs academic evaluation only.

---

# A.2 Assessment Evidence Framework

Academic evaluation begins with completed assessment evidence received from authorised examination conduct workflows.

Assessment evidence may originate from:

- Theory Examination Conduct
- Practical Examination Conduct
- Viva Examination Conduct
- Project Assessment
- Internal Assessment
- Continuous Assessment
- Digital Assessment

Assessment evidence remains immutable throughout the evaluation lifecycle.

Ownership of examination conduct remains with L2-I06 and L2-I07.

---

# A.3 Evaluation Framework

Academic evaluation follows a controlled institutional process.

```text
Assessment Evidence
        │
        ▼
Evaluator Assignment
        │
        ▼
Evaluation
        │
        ▼
Awarded Marks
        │
        ▼
Moderation
        │
        ▼
Validation
        │
        ▼
Approval
        │
        ▼
Final Approved Marks
```

Institutions may configure evaluation methodologies while preserving this canonical workflow.

---

# A.4 Evaluation Model Framework

The repository supports multiple academic evaluation models.

Supported models include:

- Single Evaluation
- Double Evaluation
- Multiple Evaluation
- Blind Evaluation
- Anonymous Evaluation
- Committee Evaluation
- Digital Evaluation
- Manual Evaluation

Institutions may configure one or more evaluation models according to academic regulations.

Evaluation models do not alter downstream Result Processing workflows.

---

# A.5 Marks Governance Framework

Marks progress through controlled business states.

```text
Awarded Marks
        │
        ▼
Moderated Marks
        │
        ▼
Validated Marks
        │
        ▼
Approved Marks
        │
        ▼
Final Approved Marks
```

Only **Final Approved Marks** become authoritative inputs for Result Processing & Computation.

No downstream workflow modifies the academic evaluation history maintained by this domain.

---

# A.6 Ownership Summary

The Marks Collection & Evaluation domain owns:

- Assessment Evidence Collection
- Answer Script Management
- Evaluation Planning
- Evaluator Assignment
- Assessment Distribution
- Academic Evaluation
- Marks Collection
- Moderation
- Marks Validation
- Marks Approval
- Marks Finalisation
- Evaluation Lifecycle Governance

The domain does **not** own:

- Examination Conduct
- Examination Scheduling
- Hall Ticket Lifecycle
- Examination Registration
- Result Processing
- Grade Calculation
- SGPA / CGPA Calculation
- Result Publication
- Revaluation Decisions
- Academic Completion

---

# A.7 Downstream Consumption

Final Approved Marks produced by this domain are consumed by:

- L2-I09 — Result Processing & Computation

Subsequent examination workflows consume processed academic outcomes rather than evaluation activities.

This preserves the separation between **academic evaluation** and **academic result computation**.

---

# A.8 Governing Principles

The Marks Collection & Evaluation domain is governed by the following principles:

- Evaluation begins only after authorised examination conduct.
- Assessment evidence remains protected throughout evaluation.
- Only authorised evaluators may perform academic evaluation.
- Institutions may configure evaluation methodologies without changing repository architecture.
- Moderation occurs before institutional approval.
- Only validated marks may be approved.
- Only Final Approved Marks are transferred to Result Processing.
- Academic evaluation and academic result computation remain separate business responsibilities.
- Every significant evaluation activity shall remain fully auditable.

---

**Appendix A completes the business reference model for L2-I08 and aligns its structure with L2-I04, L2-I05, L2-I06, and L2-I07.**

# End of Document





