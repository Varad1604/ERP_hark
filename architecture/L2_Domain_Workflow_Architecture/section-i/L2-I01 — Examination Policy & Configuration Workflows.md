# L2-I01 — Examination Policy & Configuration Workflows

**Level:** L2 – Business Workflow Architecture

**Domain:** I – Examination

**Status:** Draft

**Owner:** Examination Domain

---

# Document Control

| Item | Value |
|------|-------|
| Document ID | L2-I01 |
| Document Name | Examination Policy & Configuration Workflows |
| Domain | Examination |
| Level | L2 – Business Workflow Architecture |
| Owner | Examination Domain |
| Depends On | A01 Platform Foundation, A02 Person & Identity, A03 Authentication, A04 Authorization, H02 Notification Workflows, A06 Event Infrastructure, A07 Audit Infrastructure, B01 Institution, B02 Organisation, B04 Curriculum Framework, B05 Academic Calendar, F00 Finance Domain Model & Foundation |
| Consumed By | L2-I02 through L2-I16 Examination Workflows |
| Canonical Owner | Examination Domain |

---

# 1. Purpose

This document defines the configurable business policies governing examination operations within SutraERP.

Rather than embedding institution-specific examination rules into operational workflows, SutraERP centralises examination governance through configurable policy definitions that are referenced by all downstream examination workflows.

The objective is to ensure that institutions with differing academic regulations, grading philosophies, examination structures, eligibility criteria, and result processing rules can operate using a common workflow architecture while maintaining complete policy flexibility.

This document establishes the business ownership of examination policies and defines how policy configuration influences examination planning, registration, evaluation, result processing, graduation eligibility, and academic governance.

This document defines business workflows only.

No implementation, database design, API specification, user interface behaviour, storage model, or technology architecture is included.

---

# 2. Scope

This document includes:

- Examination policy lifecycle
- Examination policy configuration
- Examination policy activation
- Examination policy version management
- Examination grading policy
- Examination eligibility policy
- Examination attempt policy
- Passing criteria configuration
- Grace mark policy
- Result publication policy
- Revaluation policy
- Examination fee policy definition
- Examination governance policies
- Cross-domain policy dependencies
- Policy audit requirements

This document excludes:

- Examination scheduling
- Examination registration
- Hall ticket generation
- Examination conduct
- Practical examinations
- Marks entry
- Result computation
- Finance payment processing
- Academic curriculum management
- Student lifecycle management

These are defined in subsequent Examination workflow documents or their respective domain architectures.

---

# 3. Repository Context

L2-I01 serves as the policy foundation for the Examination domain.

Every operational examination workflow references policy definitions established in this document rather than maintaining independent rule sets.

This ensures:

- consistent institutional governance,
- reusable business workflows,
- configurable academic regulations,
- auditability,
- policy versioning,
- institution-specific flexibility without workflow redesign.

This document collaborates with:

- Platform Foundation for identity, authorization, notifications, events, and audit.
- Institution and Organisation domains for organisational governance.
- Curriculum Framework for programme and course structures.
- Academic Calendar for examination windows and academic periods.
- Finance Domain Model & Foundation for examination fee payment processing.
- Subsequent Examination workflows for operational execution.

L2-I01 does not perform operational examination activities.

It defines the governing policies consumed by those workflows.

# 4. Architectural Principles

The Examination Policy & Configuration workflows are governed by the following business principles.

These principles ensure that examination governance remains configurable, institution-independent, auditable, and reusable across all supported education models.

---

## 4.1 Policy-Driven Governance

All examination operations shall be governed through configurable business policies.

Operational workflows must never embed institution-specific examination rules directly.

Instead, every workflow shall reference approved policy definitions maintained by the Examination Domain.

Examples include:

- Examination eligibility
- Passing criteria
- Attempt limitations
- Grace mark rules
- Grading methodology
- Result publication policies
- Revaluation policies
- Graduation requirements

---

## 4.2 Institution Independence

The policy framework shall support different educational systems without requiring workflow redesign.

Examples include:

- Universities
- Autonomous Colleges
- Affiliated Colleges
- Engineering Institutions
- Medical Institutions
- Pharmacy Colleges
- Polytechnic Institutions
- Professional Education Institutions
- Schools (where applicable)

Business workflows remain identical while institutional policies vary through configuration.

---

## 4.3 Separation of Policy and Operations

Policy definition and operational execution are separate business responsibilities.

Policy workflows determine **what rules apply**.

Operational workflows determine **how examination activities are executed**.

Example:

This document defines the examination eligibility policy.

L2-I04 Examination Registration evaluates students against that policy.

---

## 4.4 Canonical Ownership

The Examination Domain is the canonical owner of all examination-related policies.

No external domain may independently create or modify examination governance policies.

Other domains consume approved examination policies where required.

Examples include:

- Registration
- Evaluation
- Result Processing
- Graduation
- Academic Completion

---

## 4.5 Policy Versioning

Policy changes shall not invalidate historical examination records.

Each examination session shall execute using the policy version approved for that session.

Future sessions may adopt newer policy versions without altering completed examination cycles.

Historical examination outcomes must remain reproducible using the policy version originally applied.

---

## 4.6 Effective Date Governance

Every policy shall define its business applicability.

Policies may become effective:

- Immediately
- From a future academic session
- From a specific academic year
- From a defined semester
- From a configured examination cycle

Institutions shall not be required to migrate active examinations when future policies are introduced.

---

## 4.7 Configurable Academic Regulations

Academic regulations differ significantly across institutions.

Therefore, the architecture shall support configurable rules including, but not limited to:

- Minimum passing marks
- Separate theory/practical passing requirements
- Internal assessment contribution
- External assessment contribution
- Credit completion requirements
- CGPA methodology
- Grace mark eligibility
- Improvement examination eligibility
- Supplementary examination eligibility
- Result classification methodology
- Graduation requirements

No regulation shall be hardcoded into examination workflows.

---

## 4.8 Cross-Domain Collaboration

Although examination policies are owned by the Examination Domain, they depend upon information owned by other domains.

Examples include:

| Domain | Business Dependency |
|---------|---------------------|
| Institution | Institutional examination regulations |
| Organisation | Administrative governance |
| Curriculum | Programme, Course and Credit Structure |
| Academic Calendar | Examination Windows and Academic Sessions |
| Finance | Examination Fee Policies and Payment Status |
| Identity | Student and Faculty Identity |
| Authorization | Administrative Permissions |
| Audit | Policy Traceability |

Ownership remains unchanged.

Each collaborating domain remains the canonical owner of its own business information.

---

## 4.9 Complete Auditability

Every examination policy shall maintain complete business traceability.

Audit information includes:

- Policy creation
- Policy modification
- Policy approval
- Policy activation
- Policy retirement
- Version history
- Effective dates
- Administrative ownership
- Reason for policy change

Audit responsibilities are implemented through Platform Audit Infrastructure.

---

## 4.10 Configuration Before Customisation

Institutional variation shall be achieved through configuration wherever possible.

Custom workflow implementation shall only be considered when business requirements cannot be satisfied through configurable policy definitions.

This principle ensures:

- repository consistency,
- simplified upgrades,
- reduced maintenance,
- predictable business behaviour,
- reusable examination architecture.

# 5. Business Capabilities

The Examination Policy & Configuration domain provides the foundational governance capabilities that regulate all examination operations within SutraERP.

These capabilities define institutional examination rules that are subsequently consumed by operational examination workflows.

---

## 5.1 Examination Policy Definition

The system shall support the creation and maintenance of configurable examination policies.

Policy definitions establish the institutional rules governing examination operations without embedding business logic into operational workflows.

Typical policy categories include:

- Examination Structure
- Eligibility
- Attempts
- Passing Criteria
- Grading
- Result Processing
- Revaluation
- Graduation
- Examination Governance

---

## 5.2 Examination Policy Version Management

The platform shall support multiple versions of examination policies.

Each version shall preserve:

- Business validity
- Effective period
- Academic applicability
- Version history
- Administrative ownership

Historical examination sessions shall always reference the policy version applicable at the time of execution.

---

## 5.3 Examination Type Configuration

Institutions shall configure supported examination types.

Examples include:

- Regular Examination
- Semester Examination
- Annual Examination
- Practical Examination
- Viva Examination
- Oral Examination
- Supplementary Examination
- Backlog Examination
- Improvement Examination
- Makeup Examination
- Special Examination

Additional examination types may be configured without requiring workflow modifications.

---

## 5.4 Eligibility Policy Configuration

The Examination Domain shall define the business rules determining examination eligibility.

Eligibility policies may reference criteria such as:

- Student Academic Status
- Programme Registration
- Course Registration
- Attendance Completion
- Internal Assessment Completion
- Examination Fee Payment Status
- Academic Standing
- Institutional Restrictions

Eligibility evaluation is performed by operational workflows.

This document defines only the governing policy.

---

## 5.5 Attempt Policy Configuration

Institutions may configure examination attempt policies.

Examples include:

- Maximum Attempts
- Unlimited Attempts
- Improvement Attempt Eligibility
- Supplementary Attempt Eligibility
- Attempt Restrictions
- Special Approval Requirements

Attempt policies remain configurable according to institutional regulations.

---

## 5.6 Passing Criteria Configuration

The platform shall support configurable passing criteria.

Examples include:

- Overall Passing Marks
- Theory Passing Marks
- Practical Passing Marks
- Internal Assessment Threshold
- External Assessment Threshold
- Component-wise Passing
- Aggregate Passing
- Credit-based Completion

Passing methodologies shall remain institution configurable.

---

## 5.7 Grading Policy Configuration

The Examination Domain shall support configurable grading methodologies.

Examples include:

- Percentage-Based Evaluation
- Grade-Based Evaluation
- Grade Point Evaluation
- Credit-Based Evaluation
- CGPA-Based Evaluation
- SGPA-Based Evaluation
- Letter Grade Systems

Multiple grading methodologies may coexist across different academic programmes where permitted by institutional regulations.

---

## 5.8 Result Governance Policy

Institutions shall define policies governing examination result processing.

Examples include:

- Result Approval Requirements
- Result Publication Rules
- Result Withholding Rules
- Provisional Result Rules
- Final Result Rules
- Administrative Review Requirements

Operational result processing shall reference these policies during execution.

---

## 5.9 Revaluation Policy Configuration

The platform shall support configurable post-result evaluation policies.

Examples include:

- Revaluation Eligibility
- Rechecking Eligibility
- Application Windows
- Maximum Subjects
- Approval Requirements
- Processing Sequence

The operational execution of revaluation is defined within L2-I11.

---

## 5.10 Examination Fee Policy Definition

The Examination Domain shall define examination fee policies.

These policies specify:

- Fee applicability
- Fee categories
- Fee calculation basis
- Payment prerequisites
- Late fee applicability
- Fee exemptions
- Refund eligibility rules

Business ownership of payment processing remains with the Finance Domain.

The Examination Domain consumes payment status during operational workflows.

---

## 5.11 Graduation Policy Configuration

Institutions shall configure examination policies governing academic completion.

Examples include:

- Credit Completion
- CGPA Requirements
- Mandatory Course Completion
- Graduation Eligibility
- Academic Classification
- Merit Eligibility
- Rank Determination

Operational academic completion evidence preparation is performed within L2-I13, while qualification completion remains owned by Student Lifecycle.

---

# 6. Business Actors

The following actors participate in Examination Policy & Configuration workflows.

| Actor | Responsibility |
|--------|----------------|
| Examination Administrator | Configures and maintains examination policies |
| Controller of Examination | Reviews, approves, activates and governs institutional examination policies |
| Academic Administration | Provides academic regulations influencing examination policies |
| Finance Domain | Provides examination fee payment status consumed by examination workflows |
| Academic Calendar Domain | Provides academic sessions and examination windows referenced by policies |
| Curriculum Domain | Provides programme, course and credit structures used during policy definition |
| Institution Administration | Establishes institution-specific examination regulations |
| Platform Administration | Maintains platform-level configuration capabilities without owning institutional examination policies |
| Audit Infrastructure | Records policy lifecycle activities for governance and compliance |

No student or faculty member directly modifies examination governance policies.

Policy administration remains a controlled institutional administrative responsibility.

# 7. Policy Classification Framework

The Examination Domain organises examination governance into independent policy categories.

Each policy category governs a specific aspect of examination operations while remaining reusable across all examination workflows.

Operational workflows consume these policy definitions without redefining business rules.

---

## 7.1 Examination Structure Policies

These policies define the overall structure of institutional examinations.

Examples include:

- Examination Models
- Examination Types
- Academic Examination Cycles
- Examination Components
- Assessment Structures
- Theory and Practical Relationships
- Semester or Annual Examination Models

These policies establish the institutional examination framework.

---

## 7.2 Eligibility Policies

Eligibility policies determine whether a student may participate in an examination.

Policy definitions may include:

- Programme Eligibility
- Course Eligibility
- Attendance Requirements
- Internal Assessment Requirements
- Academic Standing
- Examination Fee Payment Requirement
- Administrative Restrictions

Eligibility evaluation occurs during Examination Registration.

---

## 7.3 Attempt Policies

Attempt policies regulate examination opportunities available to students.

Examples include:

- Maximum Attempts
- Unlimited Attempts
- Improvement Attempts
- Supplementary Attempts
- Backlog Attempts
- Special Examination Attempts
- Attempt Cooling Periods
- Administrative Approval Requirements

Attempt validation is performed during operational workflows.

---

## 7.4 Evaluation Policies

Evaluation policies define how academic performance is assessed.

Examples include:

- Theory Weightage
- Practical Weightage
- Internal Assessment Contribution
- External Assessment Contribution
- Component-wise Evaluation
- Aggregate Evaluation
- Credit-based Evaluation

Evaluation policies remain configurable according to institutional regulations.

---

## 7.5 Passing Policies

Passing policies determine successful completion of examination requirements.

Examples include:

- Minimum Passing Marks
- Minimum Grade
- Practical Passing Threshold
- Theory Passing Threshold
- Internal Assessment Threshold
- Aggregate Passing
- Component-wise Passing
- Credit Completion Requirements

Passing rules are referenced during Result Processing.

---

## 7.6 Grading Policies

Grading policies define academic performance representation.

Examples include:

- Percentage
- Letter Grade
- Grade Point
- SGPA
- CGPA
- Credit Grade
- Classification Rules

Multiple grading systems may coexist where institution policies permit.

---

## 7.7 Result Governance Policies

These policies regulate examination result management.

Examples include:

- Result Approval
- Result Verification
- Result Publication
- Result Withholding
- Provisional Results
- Final Results
- Administrative Corrections

Operational workflows execute these rules during Result Processing and Publication.

---

## 7.8 Revaluation Policies

These policies govern post-result academic review.

Examples include:

- Revaluation Eligibility
- Rechecking Eligibility
- Application Windows
- Subject Limits
- Administrative Approval
- Revaluation Result Acceptance

Operational processing is defined within L2-I11.

---

## 7.9 Graduation Policies

Graduation policies define examination-related academic completion requirements.

Examples include:

- Credit Completion
- Programme Completion
- CGPA Requirement
- Mandatory Course Completion
- Degree Classification
- Merit Rules
- Rank Rules
- Graduation Eligibility

These policies support Academic Completion workflows.

---

## 7.10 Governance Policies

Governance policies regulate institutional examination administration.

Examples include:

- Policy Approval Authority
- Policy Activation Authority
- Policy Review Frequency
- Policy Audit Requirements
- Policy Retirement
- Policy Exception Approval
- Compliance Requirements

These policies ensure institutional governance and regulatory compliance.

---

# 8. Policy Lifecycle Workflows

Every examination policy follows a controlled business lifecycle.

This lifecycle ensures policy consistency, governance, auditability, and controlled adoption across examination operations.

---

## 8.1 Policy Creation Workflow

### Objective

Establish a new examination policy for institutional use.

### Workflow

1. Policy requirement is identified.
2. Examination Administrator prepares policy definition.
3. Supporting academic regulations are reviewed.
4. Policy category is assigned.
5. Initial policy version is created.
6. Policy enters Draft status.

### Outcome

A draft examination policy becomes available for institutional review.

---

## 8.2 Policy Review Workflow

### Objective

Validate policy correctness before institutional approval.

### Workflow

1. Draft policy is submitted for review.
2. Institutional academic regulations are verified.
3. Cross-domain dependencies are validated.
4. Policy conflicts are identified.
5. Required revisions are incorporated.
6. Policy becomes eligible for approval.

### Outcome

Policy is approved for governance review.

---

## 8.3 Policy Approval Workflow

### Objective

Authorize institutional use of an examination policy.

### Workflow

1. Reviewed policy is submitted for approval.
2. Controller of Examination evaluates compliance.
3. Institutional governance requirements are verified.
4. Approval decision is recorded.
5. Policy status changes to Approved.

### Outcome

Policy is authorised for future activation.

---

## 8.4 Policy Activation Workflow

### Objective

Make an approved policy available for operational examination workflows.

### Workflow

1. Effective academic period is selected.
2. Applicable examination sessions are identified.
3. Policy activation schedule is confirmed.
4. Policy becomes Active.
5. Operational workflows begin consuming the active policy version.

### Outcome

Institutional examination operations execute using the approved policy.

---

## 8.5 Policy Modification Workflow

### Objective

Introduce controlled changes to existing examination policies.

### Workflow

1. Change request is initiated.
2. Existing active policy remains unchanged.
3. New policy version is created.
4. Revised policy follows review and approval lifecycle.
5. Approved version becomes effective according to institutional policy.

### Outcome

Policy evolution occurs without affecting completed examination sessions.

---

## 8.6 Policy Retirement Workflow

### Objective

Withdraw obsolete examination policies.

### Workflow

1. Retirement request is initiated.
2. Active examination dependencies are verified.
3. Future operational impact is evaluated.
4. Retirement is approved.
5. Policy status changes to Retired.
6. Historical examination records continue referencing retired policy versions.

### Outcome

Policy is withdrawn while preserving historical examination integrity.

# 9. Business Rules

The following business rules govern Examination Policy & Configuration workflows.

These rules establish consistent governance while allowing institutional flexibility through configuration.

---

## BR-001 Policy Ownership

The Examination Domain shall remain the canonical owner of all examination policies.

No other domain may independently create, modify, approve, activate, or retire examination policies.

---

## BR-002 Policy Approval Requirement

Only approved examination policies may become operational.

Draft, rejected, expired, or retired policies shall never be consumed by operational examination workflows.

---

## BR-003 Single Active Version

Only one active version of a policy shall exist for the same policy category, institution, and effective academic period.

Future versions may coexist but remain inactive until their configured effective period.

---

## BR-004 Historical Integrity

Policy modifications shall never alter completed examination sessions.

Historical examination records shall continue referencing the policy version under which they were executed.

---

## BR-005 Effective Date Compliance

Every policy shall define an effective business period.

Operational workflows shall consume only the policy version applicable to the corresponding examination session.

---

## BR-006 Configurable Academic Regulations

Institution-specific academic regulations shall be implemented through policy configuration.

Operational workflows shall not contain institution-specific business logic.

---

## BR-007 Cross-Domain Ownership

Examination policies may reference information owned by other domains.

However, ownership shall remain unchanged.

Examples include:

- Academic Calendar
- Curriculum
- Finance
- Institution
- Organisation

Each collaborating domain remains the canonical owner of its own information.

---

## BR-008 Examination Fee Governance

The Examination Domain defines examination fee applicability and policy.

Payment collection, accounting, refunds, and financial reconciliation remain the responsibility of the Finance Domain.

Operational examination workflows consume payment status published by Finance.

---

## BR-009 Audit Compliance

Every policy lifecycle activity shall be recorded through the Platform Audit Infrastructure.

Audit records shall remain immutable.

---

## BR-010 Policy Retirement

Retired policies shall not participate in future examination operations.

Historical examination records shall remain fully accessible using retired policy definitions.

---

# 10. Exception Scenarios

The Examination Policy & Configuration workflows shall support controlled handling of exceptional business situations.

---

## EX-001 Policy Approval Rejected

If a policy fails institutional review or governance approval:

- Policy remains in Draft status.
- Operational workflows continue using the currently active policy.
- Revisions may be submitted for subsequent approval.

---

## EX-002 Policy Conflict Detected

If multiple policies define conflicting business rules for the same effective period:

- Policy activation shall be prevented.
- Administrative review shall be required.
- Conflict resolution shall occur before activation.

---

## EX-003 Effective Period Overlap

If two active policy versions overlap for the same policy category:

- Activation shall be rejected.
- Existing active policy remains effective.
- Administrative correction shall be required.

---

## EX-004 External Dependency Changed

If dependent academic regulations change before policy activation:

Examples include:

- Curriculum revision
- Academic Calendar revision
- Institutional regulation update

The policy shall be revalidated before activation.

---

## EX-005 Policy Retirement During Active Examination Cycle

If retirement is requested while examination operations are in progress:

- Retirement shall be deferred.
- Current examination sessions continue using the active policy.
- Retirement becomes effective after operational completion.

---

## EX-006 Invalid Policy Configuration

If mandatory policy information is incomplete or inconsistent:

- Approval shall be prevented.
- Policy remains in Draft status.
- Administrative correction shall be required before resubmission.

---

# 11. Events Published

Upon successful completion of policy lifecycle activities, the Examination Domain publishes business events for downstream consumers.

| Event | Description | Primary Consumers |
|---------|------------|------------------|
| Examination Policy Created | New policy definition established | Audit Infrastructure |
| Examination Policy Submitted | Policy submitted for governance review | Audit Infrastructure |
| Examination Policy Approved | Policy approved for activation | Examination Workflows |
| Examination Policy Activated | Policy becomes operational | L2-I02 to L2-I16 |
| Examination Policy Modified | New policy version created | Audit Infrastructure |
| Examination Policy Retired | Policy withdrawn from future use | Examination Workflows |
| Examination Policy Version Published | New version available | Operational Examination Workflows |

---

# 12. Events Consumed

The Examination Policy Domain consumes business events produced by collaborating domains.

| Event | Publishing Domain | Purpose |
|---------|------------------|---------|
| Academic Calendar Updated | Academic Calendar | Validate policy applicability |
| Curriculum Updated | Curriculum | Validate academic structure dependencies |
| Institution Regulation Updated | Institution | Review policy compliance |
| Examination Fee Policy Updated | Finance | Synchronise fee policy definitions where applicable |
| Organisation Structure Updated | Organisation | Validate governance applicability |

Policy workflows consume business events only.

They do not assume ownership of external business information.

# 13. Cross-Domain Interactions

The Examination Policy & Configuration Domain collaborates with multiple business domains while maintaining clear ownership boundaries.

Each collaborating domain remains the canonical owner of its own business information.

The Examination Domain consumes business information only where necessary to define examination governance.

---

## 13.1 Institution Domain

### Business Purpose

Institution-specific academic regulations establish the overall examination governance framework.

### Information Consumed

- Institutional Examination Regulations
- Academic Governance Structure
- Examination Authorities
- Institutional Policies

### Information Published

- Examination Policy Availability
- Examination Governance Policies

---

## 13.2 Organisation Domain

### Business Purpose

Organisation defines administrative units responsible for examination governance.

### Information Consumed

- Organisational Structure
- Administrative Units
- Governance Hierarchy

### Information Published

- Examination Policy Responsibilities
- Examination Governance Assignments

---

## 13.3 Curriculum Domain

### Business Purpose

Curriculum provides the academic structure upon which examination policies operate.

### Information Consumed

- Programme Structure
- Course Structure
- Credit Framework
- Assessment Components
- Academic Requirements

### Information Published

None.

Curriculum remains the canonical owner of academic structure.

---

## 13.4 Academic Calendar Domain

### Business Purpose

Academic Calendar determines the academic periods in which examination policies become effective.

### Information Consumed

- Academic Sessions
- Semesters
- Examination Windows
- Academic Years

### Information Published

None.

---

## 13.5 Finance Domain

### Business Purpose

Finance governs financial processing related to examinations.

### Information Consumed

None for policy definition.

Operational workflows consume Finance events.

### Information Published

Examination Fee Policies

These define:

- Fee applicability
- Fee categories
- Fee exemptions
- Late fee rules
- Refund eligibility

Finance remains responsible for:

- Fee Collection
- Receipts
- Refund Processing
- Financial Accounting
- Financial Audit

---

## 13.6 Platform Foundation

Platform services provide shared enterprise capabilities.

### Services Utilised

- Identity
- Authentication
- Authorization
- Notification
- Event Infrastructure
- Audit Infrastructure

These services support examination governance without owning examination policies.

---

# 14. Ownership Boundaries

Clear ownership boundaries ensure that examination governance remains consistent across the repository.

---

## Examination Domain Owns

The Examination Domain is the canonical owner of:

- Examination Policies
- Examination Governance
- Eligibility Policies
- Attempt Policies
- Passing Policies
- Grading Policies
- Result Policies
- Revaluation Policies
- Graduation Policies
- Examination Fee Policy Definitions
- Examination Policy Lifecycle

---

## Examination Domain Does NOT Own

The Examination Domain does not own:

- Student Identity
- Faculty Identity
- Institution Structure
- Organisation Structure
- Programme Structure
- Course Structure
- Academic Calendar
- Fee Collection
- Accounting
- Payments
- Authentication
- Authorization
- Notifications
- Audit Infrastructure

These responsibilities remain within their respective domains.

---

# 15. Configurable Policy Matrix

The following examination policy categories are institution configurable.

| Policy Category | Configurable | Mandatory | Referenced By |
|-----------------|--------------|-----------|---------------|
| Examination Types | Yes | Yes | I02, I03 |
| Eligibility Rules | Yes | Yes | I04 |
| Attempt Rules | Yes | Yes | I04, I12 |
| Passing Criteria | Yes | Yes | I09 |
| Evaluation Rules | Yes | Yes | I08, I09 |
| Grading Methodology | Yes | Yes | I09 |
| Result Governance | Yes | Yes | I09, I10 |
| Revaluation Rules | Yes | No | I11 |
| Examination Fee Policies | Yes | Yes | I04 |
| Graduation Rules | Yes | Yes | I13 |
| Merit & Rank Rules | Yes | No | I13 |
| Grace Mark Policies | Yes | No | I09 |
| Administrative Approval Rules | Yes | No | Multiple Examination Workflows |

Configuration replaces workflow customisation wherever possible.

---

# 16. Audit Requirements

Every examination policy lifecycle activity shall be fully traceable.

The Platform Audit Infrastructure records audit events generated by Examination Policy workflows.

Audit records include:

- Policy Creation
- Policy Modification
- Policy Review
- Policy Approval
- Policy Activation
- Policy Version Changes
- Policy Retirement
- Administrative Actions
- Effective Date Changes
- Governance Decisions

Audit records are retained according to institutional and regulatory requirements.

The Examination Domain publishes audit events but does not implement audit storage or retention mechanisms.

---

# 17. Workflow Completion Criteria

An Examination Policy workflow is considered complete when all of the following conditions are satisfied:

- Policy definition has been completed.
- Required institutional information has been validated.
- Governance review has been completed.
- Policy approval has been granted.
- Effective period has been established.
- Policy version has been activated.
- Dependent examination workflows can consume the active policy.
- Audit events have been published.
- Historical policy integrity has been preserved.
- Ownership boundaries remain consistent with repository architecture.

---

# 18. Examination Policy Catalogue

The Examination Policy Catalogue defines the canonical policy artefacts owned by the Examination Domain.

Operational examination workflows consume these policies without redefining business rules.

| Policy | Purpose | Primary Consumers |
|---------|---------|-------------------|
| Examination Type Policy | Defines supported examination categories and structures | I02, I03 |
| Examination Session Policy | Governs examination sessions and cycles | I02 |
| Examination Eligibility Policy | Defines eligibility requirements for examination registration | I04 |
| Examination Registration Policy | Governs registration windows and registration rules | I04 |
| Hall Ticket Policy | Governs hall ticket generation and eligibility | I05 |
| Practical Examination Policy | Defines practical and viva examination regulations | I06 |
| Theory Examination Policy | Defines theory examination governance | I07 |
| Marks Evaluation Policy | Governs marks entry, verification and approval | I08 |
| Result Computation Policy | Defines grading, SGPA, CGPA and result computation rules | I09 |
| Result Publication Policy | Governs approval and publication of examination results | I10 |
| Revaluation Policy | Defines revaluation and rechecking regulations | I11 |
| Supplementary Examination Policy | Governs supplementary, backlog and improvement examinations | I12 |
| Graduation Policy | Defines academic completion and completion evidence requirements | I13 |
| Merit & Rank Policy | Governs merit list and rank computation | I13 |
| Malpractice Policy | Defines malpractice investigation and disciplinary actions | I14 |
| Examination Exception Policy | Governs exceptional examination situations | I15 |
| Examination Closure Policy | Governs examination completion and archival | I16 |

All examination workflow documents shall consume the appropriate policy definitions from this catalogue.

---

# 19. Policy Dependency Matrix

Examination policies depend upon business information owned by collaborating domains.

Ownership remains unchanged.

| Examination Policy | Depends Upon |
|--------------------|--------------|
| Examination Type Policy | Institution |
| Examination Session Policy | Academic Calendar |
| Eligibility Policy | Curriculum, Academic Calendar, Finance |
| Registration Policy | Eligibility Policy, Academic Calendar |
| Hall Ticket Policy | Registration Policy |
| Practical Examination Policy | Curriculum |
| Theory Examination Policy | Academic Calendar |
| Marks Evaluation Policy | Curriculum |
| Result Computation Policy | Curriculum, Grading Policy |
| Result Publication Policy | Result Computation Policy |
| Revaluation Policy | Result Publication Policy |
| Supplementary Examination Policy | Result Computation Policy |
| Graduation Policy | Curriculum, Result Computation Policy |
| Merit & Rank Policy | Graduation Policy |
| Malpractice Policy | Institution |
| Examination Exception Policy | Institution |
| Examination Closure Policy | All Operational Examination Workflows |

This dependency matrix establishes the business sequencing of examination governance without altering domain ownership.

---

# 20. Examination Policy State Model

Every examination policy follows a controlled lifecycle.

| State | Description |
|--------|-------------|
| Draft | Initial policy definition under preparation |
| Under Review | Policy undergoing institutional review |
| Approved | Policy approved for institutional adoption |
| Scheduled | Policy approved with a future effective period |
| Active | Policy currently governing examination operations |
| Superseded | Policy replaced by a newer approved version |
| Retired | Policy withdrawn from future operational use |
| Archived | Historical policy retained for audit and reference purposes |

Business rules governing policy transitions:

- Policies begin in **Draft**.
- Only approved policies may become **Scheduled** or **Active**.
- Active policies may be superseded by newer approved versions.
- Superseded policies remain available for historical examination sessions.
- Retired policies shall not participate in future examination operations.
- Archived policies remain accessible for governance and audit purposes.

---

# 21. Configurable Parameters Catalogue

Institutions may configure examination behaviour through business parameters without modifying operational workflows.

The following configuration categories are supported.

| Configuration Category | Examples |
|------------------------|----------|
| Examination Structure | Semester, Annual, Modular, Trimester |
| Examination Types | Regular, Practical, Viva, Supplementary, Improvement, Makeup |
| Registration | Registration Window, Late Registration, Administrative Approval |
| Eligibility | Attendance Percentage, Internal Assessment Requirement, Fee Payment Requirement |
| Attempts | Maximum Attempts, Improvement Attempts, Supplementary Attempts |
| Evaluation | Internal Weightage, External Weightage, Practical Weightage |
| Passing Rules | Theory Passing Marks, Practical Passing Marks, Aggregate Passing |
| Grading | Percentage, Grade, Grade Point, SGPA, CGPA |
| Result Processing | Grace Marks, Moderation Rules, Classification |
| Result Publication | Publication Date, Publication Approval, Withheld Results |
| Revaluation | Application Window, Subject Limits, Approval Requirements |
| Graduation | Credit Completion, CGPA Requirement, Degree Classification |
| Merit & Rank | Rank Methodology, Merit Eligibility |
| Governance | Approval Authority, Review Frequency, Policy Versioning |

Configuration values remain institution specific while workflow architecture remains repository standard.

---

# 22. Domain Responsibility Summary

The Examination Policy & Configuration Domain defines examination governance.

Operational execution remains the responsibility of subsequent Examination workflow documents.

| Business Capability | Owned by L2-I01 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Examination Policy Definition | ✓ | — |
| Policy Governance | ✓ | — |
| Policy Versioning | ✓ | — |
| Examination Planning | — | L2-I03 |
| Examination Registration | — | L2-I04 |
| Hall Ticket Generation | — | L2-I05 |
| Practical Examination | — | L2-I06 |
| Theory Examination | — | L2-I07 |
| Marks Collection | — | L2-I08 |
| Result Processing | — | L2-I09 |
| Result Publication | — | L2-I10 |
| Revaluation | — | L2-I11 |
| Supplementary & Improvement | — | L2-I12 |
| Qualification Completion & Academic Completion | — | L2-I13 |
| Malpractice Management | — | L2-I14 |
| Exception Management | — | L2-I15 |
| Examination Closure & Archival | — | L2-I16 |

L2-I01 establishes the governance foundation consumed by every operational Examination workflow within the SutraERP repository.






