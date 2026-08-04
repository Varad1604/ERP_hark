# L2-I15 — Examination Exception & Contingency Workflows

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §2 Scope; §2.1 In Scope; §2.2 Out of Scope |
| Actors | §7 Business Actors; §7.1 Internal Actors; §7.2 External Actors |
| Entities | §10 Aggregate Model; §10.1 Aggregate Overview; §10.2 Aggregate Root |
| Business Rules | §13 Business Rules |
| Workflow | §11 Workflow Catalogue; §12 Workflow Specifications; §11.1 Workflow Overview |
| State Machine | §21 State Model; §21.3 State Definitions; §21.4 Valid State Transitions |
| Contracts | — not present — |
| Events | §16 Events Published; §17 Events Consumed; §16.2 Published Events |
| Dependencies | §22 Dependency Matrix; §3.2 Repository Dependencies; §22.1 Upstream Dependencies |
| Locked Decisions | §18.9 Decision Validation |

> **Gap:** 1 required section(s) have no counterpart in this document: Contracts. Recorded, not authored.

---

# Document Control

| Attribute | Value |
|------------|-------|
| Document ID | L2-I15 |
| Document Name | Examination Exception & Contingency Workflows |
| Domain | Examination Domain |
| Architecture Level | L2 – Canonical Domain Workflow |
| Owner | Examination Domain |
| Document Owner | Devang |
| Repository | SutraERP Enterprise Architecture Repository |
| Status | Draft |
| Depends On | L1 Business Domain Architecture, L2-A01–A08, L2-B01–B05, L2-I01–L2-I14 |
| Version | 0.1 |

---

# Revision History

| Version | Date | Author | Description |
|----------|------|--------|-------------|
| 0.1 | YYYY-MM-DD | Enterprise Architecture Team | Initial Draft |

---

# Table of Contents

1. Purpose
2. Scope
3. Repository Context
4. Architectural Principles
5. Domain Overview
6. Business Capabilities
7. Business Actors
8. Ubiquitous Language
9. Domain Ownership
10. Aggregate Model
11. Workflow Catalogue
12. Workflow Specifications
13. Business Rules
14. Exception Scenarios
15. Integration Model
16. Events Published
17. Events Consumed
18. Validation Framework
19. Audit Requirements
20. Security & Compliance
21. State Model
22. Dependency Matrix
23. Appendices

---

# 1. Purpose

## 1.1 Overview

The Examination Exception & Contingency domain defines the canonical business architecture governing exceptional situations that disrupt, delay, suspend, postpone, cancel, or otherwise affect scheduled examination activities.

The domain provides a standardized institutional framework for identifying, assessing, approving, executing, and resolving examination contingencies while preserving academic integrity, fairness, operational continuity, and regulatory compliance.

It establishes a single authoritative workflow for managing examination exceptions arising from operational failures, environmental conditions, technical incidents, institutional emergencies, regulatory directives, and unforeseen circumstances.

This document is implementation-independent and defines business ownership, workflows, responsibilities, events, validation rules, and governance for the Examination Exception & Contingency bounded context.

---

## 1.2 Objectives

The objectives of this domain are to:

- Establish a single enterprise workflow for examination exceptions.
- Standardize contingency planning across institutions.
- Ensure business continuity during examination disruptions.
- Support timely operational decision-making.
- Preserve fairness for all affected candidates.
- Enable controlled examination recovery.
- Maintain complete auditability of exceptional decisions.
- Support institution-specific contingency policies.
- Publish canonical exception events for downstream domains.

---

# 2. Scope

## 2.1 In Scope

This domain governs:

### Exception Management

- Exception registration
- Exception classification
- Exception validation
- Exception escalation
- Exception ownership
- Exception tracking
- Exception closure

### Contingency Management

- Contingency planning
- Emergency response
- Operational recovery
- Examination postponement
- Examination rescheduling
- Examination suspension
- Examination cancellation
- Examination continuation
- Examination restart

### Impact Assessment

- Candidate impact analysis
- Examination impact assessment
- Venue impact assessment
- Infrastructure impact assessment
- Staff availability assessment
- Regulatory impact assessment

### Emergency Governance

- Emergency approvals
- Exceptional authorization
- Decision recording
- Regulatory notifications
- Stakeholder communication

### Recovery Management

- Examination recovery planning
- Recovery execution
- Operational verification
- Recovery completion
- Exception closure

### Enterprise Integration

- Domain commands
- Domain events
- Workflow integration
- Audit integration
- Notification integration

---

## 2.2 Out of Scope

The following responsibilities belong to other canonical domains.

| Capability | Owning Domain |
|------------|---------------|
| Student Identity | L2-A02 |
| Authentication | L2-A03 |
| Authorization | L2-A04 |
| Workflow Infrastructure | L2-A05 |
| Audit Infrastructure | L2-A07 |
| Examination Scheduling | L2-I03 |
| Hall Ticket Management | L2-I05 |
| Examination Conduct | L2-I06 |
| Evaluation | L2-I08 |
| Result Processing | L2-I09 |
| Result Publication | L2-I10 |
| Revaluation | L2-I11 |
| Supplementary Examination | L2-I12 |
| Academic Completion | L2-I13 |
| Malpractice Investigation | L2-I14 |

---

# 3. Repository Context

## 3.1 Position within the Repository

The Examination Exception & Contingency domain provides operational resilience for the Examination bounded context.

It governs exceptional situations that interrupt or significantly alter the normal examination lifecycle and coordinates institution-approved contingency actions.

The domain does not own routine examination workflows but orchestrates approved responses to extraordinary operational situations.

---

## 3.2 Repository Dependencies

### Upstream

- L2-A01 Platform & Tenant Foundation
- L2-A02 Person & Identity
- L2-A03 Authentication
- L2-A04 Authorization
- L2-A05 Workflow Infrastructure
- L2-A06 Event Infrastructure
- L2-A07 Audit Infrastructure
- L2-B01 Institution
- L2-B02 Organisation
- L2-B03 Campus & Infrastructure
- L2-B05 Academic Calendar
- L2-I01–L2-I14 Examination Domains

### Downstream

- Examination Conduct
- Result Processing
- Result Publication
- Supplementary Examination
- Academic Completion
- Student Lifecycle
- Notification Services
- Reporting & Analytics

---

# 4. Architectural Principles

The Examination Exception & Contingency domain shall adhere to the following principles:

1. Single Source of Truth for examination exceptions.
2. Policy-driven contingency governance.
3. Controlled operational recovery.
4. Fairness for affected candidates.
5. Explicit institutional approvals.
6. Event-driven integration.
7. Immutable audit history.
8. Clear ownership boundaries.
9. Implementation independence.
10. Regulatory compliance by design.

# 5. Domain Overview

## 5.1 Overview

The Examination Exception & Contingency domain governs the complete lifecycle of exceptional events that may impact the planning, execution, continuity, or completion of examination activities.

It provides the institutional governance required to assess operational disruptions, determine appropriate contingency actions, obtain necessary approvals, execute recovery strategies, communicate with stakeholders, and restore examination operations while preserving academic integrity and fairness.

The domain acts as the operational resilience layer of the Examination bounded context.

It neither owns routine examination execution nor academic decision-making. Instead, it governs institutional responses whenever normal examination workflows cannot proceed as planned.

---

## 5.2 Business Objectives

The domain shall:

- Ensure examination continuity whenever reasonably possible.
- Standardize institutional contingency planning.
- Minimize disruption to candidates.
- Support rapid operational decision-making.
- Preserve fairness across affected candidates.
- Maintain complete auditability of emergency actions.
- Support institution-specific contingency policies.
- Enable controlled examination recovery.
- Protect examination integrity during exceptional situations.

---

## 5.3 Core Responsibilities

The Examination Exception & Contingency domain owns responsibility for:

- Registering examination exceptions.
- Classifying exception categories.
- Assessing operational impact.
- Coordinating contingency planning.
- Managing emergency approvals.
- Recording exceptional institutional decisions.
- Coordinating examination postponement.
- Managing examination cancellation.
- Coordinating examination rescheduling.
- Managing examination recovery.
- Recording stakeholder communications.
- Closing exception cases.
- Publishing exception events.
- Maintaining complete audit history.

---

## 5.4 Architectural Characteristics

| Characteristic | Description |
|----------------|-------------|
| Domain Type | Operational Governance Domain |
| Ownership Model | Single Aggregate Ownership |
| Architecture Style | Domain-Driven Design |
| Integration Style | Event-Driven |
| Consistency Model | Strong Consistency |
| Auditability | Mandatory |
| Security Classification | Confidential |
| Policy Driven | Yes |
| Business Continuity | Mandatory |

---

# 6. Business Capabilities

## 6.1 Capability Overview

The domain consists of the following canonical business capabilities.

| Capability ID | Capability | Description |
|---------------|------------|-------------|
| EC-01 | Exception Registration | Register examination exceptions. |
| EC-02 | Exception Classification | Categorize operational disruptions. |
| EC-03 | Impact Assessment | Assess operational impact. |
| EC-04 | Contingency Planning | Prepare contingency strategies. |
| EC-05 | Emergency Approval | Obtain institutional approvals. |
| EC-06 | Contingency Execution | Execute approved contingency plans. |
| EC-07 | Examination Recovery | Restore examination operations. |
| EC-08 | Stakeholder Communication | Notify affected stakeholders. |
| EC-09 | Exception Resolution | Resolve examination exceptions. |
| EC-10 | Exception Closure | Close completed exception cases. |
| EC-11 | Audit & Compliance | Maintain audit and compliance records. |
| EC-12 | Event Publication | Publish canonical exception events. |

---

## 6.2 Capability Relationships

```text
Exception Reported
        │
        ▼
Exception Registration
        │
        ▼
Classification
        │
        ▼
Impact Assessment
        │
        ▼
Contingency Planning
        │
        ▼
Emergency Approval
        │
        ▼
Contingency Execution
        │
        ▼
Recovery
        │
        ▼
Resolution
        │
        ▼
Closure
```

---

# 7. Business Actors

## 7.1 Internal Actors

| Actor | Responsibility |
|--------|----------------|
| Examination Controller | Overall exception governance. |
| Registrar | Institutional emergency approvals. |
| Chief Superintendent | Reports examination disruptions. |
| Examination Administrator | Coordinates contingency activities. |
| Centre Superintendent | Manages examination centre operations. |
| IT Operations Team | Resolves technology-related disruptions. |
| Infrastructure Manager | Resolves facility-related disruptions. |
| Emergency Response Committee | Makes strategic contingency decisions. |

---

## 7.2 External Actors

| Actor | Responsibility |
|--------|----------------|
| Student | Receives contingency notifications. |
| Parent / Guardian | Receives communication where institutional policy permits. |
| Regulatory Authority | Receives statutory notifications where required. |
| Service Provider | Restores outsourced examination services. |
| Government Authority | Issues emergency directives where applicable. |

---

## 7.3 System Actors

| System | Interaction |
|---------|-------------|
| Identity Domain | User verification. |
| Authorization Domain | Permission validation. |
| Workflow Engine | Executes contingency workflows. |
| Notification Domain | Sends emergency communications. |
| Audit Infrastructure | Stores immutable audit history. |
| Event Infrastructure | Publishes exception events. |

---

# 8. Ubiquitous Language

The following terminology shall be used consistently throughout the repository.

| Term | Definition |
|------|------------|
| Examination Exception | Any unexpected event affecting examination operations. |
| Contingency | Approved response to an examination exception. |
| Impact Assessment | Evaluation of operational consequences. |
| Recovery Plan | Approved strategy for restoring examination operations. |
| Emergency Approval | Institutional authorization for exceptional actions. |
| Exception Case | Aggregate representing an operational disruption. |
| Operational Recovery | Restoration of examination activities. |
| Exception Resolution | Completion of all contingency activities. |
| Exception Closure | Formal completion of the exception lifecycle. |
| Force Majeure | Extraordinary event beyond institutional control. |

---

# 9. Domain Ownership

## 9.1 Owns

The Examination Exception & Contingency domain exclusively owns:

- Examination exception cases.
- Exception classifications.
- Operational impact assessments.
- Contingency plans.
- Emergency approvals.
- Recovery plans.
- Exception communications.
- Exception resolution.
- Exception closure.
- Exception audit metadata.
- Exception domain events.

No other domain may create, modify, or close Examination Exception Cases.

---

## 9.2 Does Not Own

The domain explicitly does not own:

- Examination schedules.
- Candidate registrations.
- Hall tickets.
- Examination attendance.
- Question papers.
- Evaluation.
- Result processing.
- Result publication.
- Malpractice investigations.
- Academic completion.
- Degree certification.

Ownership of these capabilities remains with their respective canonical domains.

# 10. Aggregate Model

## 10.1 Aggregate Overview

The Examination Exception & Contingency domain is modelled as a single transactional aggregate centred around the **Examination Exception Case**.

An Examination Exception Case represents the authoritative business entity governing every operational disruption affecting examination activities.

All modifications relating to impact assessment, contingency planning, emergency approvals, recovery execution, stakeholder communication, and exception closure shall occur within the governance boundaries of the Examination Exception Case aggregate.

No external domain may directly modify aggregate state.

---

## 10.2 Aggregate Root

**Aggregate Root**

> Examination Exception Case

The aggregate root is responsible for:

- Maintaining exception lifecycle integrity.
- Enforcing institutional contingency policies.
- Coordinating operational recovery.
- Managing emergency approvals.
- Recording exceptional decisions.
- Managing examination continuity.
- Publishing domain events.
- Maintaining complete audit traceability.

---

## 10.3 Aggregate Components

```text
Examination Exception Case
│
├── Exception Report
├── Classification
├── Impact Assessment
├── Risk Assessment
├── Contingency Plan
├── Emergency Approval
├── Recovery Plan
├── Stakeholder Communication
├── Resolution
├── Closure
└── Audit Metadata
```

---

## 10.4 Aggregate Invariants

The aggregate shall enforce the following invariants.

- Every exception case originates from a reported operational incident.
- Every contingency plan belongs to exactly one exception case.
- Impact assessment shall be completed before contingency approval.
- Recovery activities shall follow an approved contingency plan.
- Exception closure shall occur only after all recovery activities are completed.
- Archived exception cases shall be immutable.
- Every operational decision shall be fully auditable.
- Aggregate consistency shall be preserved throughout every state transition.

---

# 11. Workflow Catalogue

## 11.1 Workflow Overview

The domain consists of the following canonical workflows.

| Workflow ID | Workflow Name |
|--------------|---------------|
| EW-01 | Register Examination Exception |
| EW-02 | Classify Exception |
| EW-03 | Assess Operational Impact |
| EW-04 | Prepare Contingency Plan |
| EW-05 | Obtain Emergency Approval |
| EW-06 | Execute Contingency Plan |
| EW-07 | Notify Stakeholders |
| EW-08 | Postpone Examination |
| EW-09 | Reschedule Examination |
| EW-10 | Suspend Examination |
| EW-11 | Resume Examination |
| EW-12 | Cancel Examination |
| EW-13 | Close Exception Case |
| EW-14 | Archive Exception Case |

---

# 12. Workflow Specifications

## EW-01 Register Examination Exception

### Purpose

Register an operational event that may significantly affect examination activities.

### Trigger

- Examination centre disruption.
- Infrastructure failure.
- Technology outage.
- Natural disaster.
- Security incident.
- Public emergency.
- Regulatory directive.
- Institutional emergency.

### Preconditions

- Examination session exists.
- Reporting authority is authorized.
- Operational disruption verified.

### Workflow

1. Receive exception report.
2. Validate reporting authority.
3. Capture operational details.
4. Capture examination session information.
5. Assign preliminary exception category.
6. Generate exception reference.
7. Publish exception registration event.

### Postconditions

- Exception registered.
- Exception reference created.
- Impact assessment initiated.

---

## EW-02 Classify Exception

### Purpose

Categorize the operational disruption according to institutional contingency policies.

### Typical Categories

- Infrastructure failure.
- Technology failure.
- Power outage.
- Network outage.
- Natural disaster.
- Weather disruption.
- Public health emergency.
- Security threat.
- Examination centre closure.
- Staff shortage.
- Question paper compromise.
- Regulatory intervention.
- Force majeure.
- Other institution-defined exception.

### Postconditions

- Exception classification completed.
- Applicable contingency policy determined.

---

## EW-03 Assess Operational Impact

### Purpose

Assess the operational consequences of the examination exception.

### Assessment Areas

- Candidate impact.
- Examination schedule.
- Examination centres.
- Infrastructure availability.
- Technology availability.
- Staff availability.
- Regulatory obligations.
- Recovery feasibility.

### Outputs

- Operational impact report.
- Risk classification.
- Recommended contingency approach.

---

## EW-04 Prepare Contingency Plan

### Purpose

Develop an institutional contingency plan appropriate to the identified operational disruption.

### Workflow

1. Review impact assessment.
2. Identify recovery options.
3. Evaluate institutional policies.
4. Prepare contingency plan.
5. Estimate implementation timeline.
6. Submit for approval.

### Outputs

- Contingency plan completed.
- Approval request initiated.

---

## EW-05 Obtain Emergency Approval

### Purpose

Obtain institutional approval for contingency execution.

### Preconditions

- Impact assessment completed.
- Contingency plan prepared.
- Required documentation available.

### Workflow

1. Review contingency proposal.
2. Verify institutional authority.
3. Validate regulatory compliance.
4. Record emergency approval.
5. Authorize execution.

### Outputs

- Emergency approval granted or rejected.
- Approval record created.

---

## EW-06 Execute Contingency Plan

### Purpose

Execute the approved contingency plan.

### Possible Actions

- Continue examination.
- Suspend examination.
- Postpone examination.
- Cancel examination.
- Shift examination venue.
- Switch delivery mode.
- Extend examination duration.
- Conduct emergency recovery activities.

### Postconditions

- Approved contingency executed.
- Operational status updated.

---

## EW-07 Notify Stakeholders

### Purpose

Notify all affected stakeholders regarding contingency decisions.

### Stakeholders

- Students.
- Parents / Guardians (where permitted).
- Examination staff.
- Academic departments.
- Institution management.
- Regulatory authorities.
- External service providers.

### Notification Channels

- ERP Notification.
- Email.
- SMS.
- Student Portal.
- Institutional Website.
- Official Circular.

### Outputs

- Notifications delivered.
- Communication history recorded.

---

## EW-08 Postpone Examination

### Purpose

Postpone an examination when continuation is not feasible.

### Workflow

1. Verify postponement approval.
2. Suspend scheduled examination.
3. Reserve future scheduling.
4. Notify stakeholders.
5. Publish postponement event.

### Postconditions

- Examination postponed.
- Rescheduling initiated.

---

## EW-09 Reschedule Examination

### Purpose

Assign a new examination schedule following postponement.

### Workflow

1. Identify available schedule.
2. Validate institutional calendar.
3. Allocate examination resources.
4. Confirm examination schedule.
5. Notify stakeholders.

### Outputs

- Examination rescheduled.
- Updated schedule published.

---

## EW-10 Suspend Examination

### Purpose

Temporarily suspend an ongoing examination due to operational disruption.

### Typical Reasons

- Security incident.
- Medical emergency.
- Infrastructure failure.
- Technology outage.
- Force majeure.
- Institutional directive.

### Postconditions

- Examination suspended.
- Recovery planning initiated.

---

## EW-11 Resume Examination

### Purpose

Resume a previously suspended examination following operational recovery.

### Preconditions

- Operational issues resolved.
- Recovery approved.
- Examination environment verified.

### Workflow

1. Verify recovery readiness.
2. Obtain authorization.
3. Resume examination.
4. Notify stakeholders.
5. Publish resumption event.

### Postconditions

- Examination resumed.
- Operational continuity restored.

---

## EW-12 Cancel Examination

### Purpose

Permanently cancel an examination where recovery is not feasible.

### Preconditions

- Exceptional approval obtained.
- Institutional policy satisfied.
- Regulatory obligations reviewed.

### Workflow

1. Review cancellation request.
2. Obtain final authorization.
3. Record cancellation.
4. Notify stakeholders.
5. Initiate follow-up actions.

### Postconditions

- Examination cancelled.
- Recovery workflow terminated.

---

## EW-13 Close Exception Case

### Purpose

Formally conclude the exception management lifecycle.

### Preconditions

- Recovery completed.
- Communications completed.
- Outstanding actions resolved.
- Approvals finalized.

### Workflow

1. Verify completion checklist.
2. Confirm operational restoration.
3. Close exception case.
4. Publish closure event.

### Postconditions

- Exception case closed.
- Audit finalized.

---

## EW-14 Archive Exception Case

### Purpose

Archive completed examination exception cases for institutional retention.

### Archival Contents

- Exception report.
- Impact assessment.
- Contingency plan.
- Approval records.
- Recovery records.
- Communication history.
- Resolution records.
- Audit history.

### Business Rules

- Archived exception cases shall remain immutable.
- Archive retention shall follow institutional policy.
- Retrieval shall be fully audited.
- Archived records shall remain searchable by authorized users.
```

# 13. Business Rules

## 13.1 General Rules

| Rule ID | Business Rule |
|----------|---------------|
| BR-001 | Every examination exception shall result in a uniquely identifiable Exception Case. |
| BR-002 | Every Exception Case shall originate from a verified operational incident. |
| BR-003 | Every Exception Case shall have exactly one owning institution. |
| BR-004 | Impact assessment shall be completed before contingency approval. |
| BR-005 | Contingency execution shall require institutional authorization. |
| BR-006 | Emergency approvals shall comply with institutional delegation policies. |
| BR-007 | Examination postponement shall update all dependent examination activities. |
| BR-008 | Examination rescheduling shall comply with Academic Calendar constraints. |
| BR-009 | Examination cancellation shall notify all affected stakeholders. |
| BR-010 | Operational recovery shall be verified before examination resumption. |
| BR-011 | Closed Exception Cases shall not return to execution unless officially reopened. |
| BR-012 | Archived Exception Cases shall remain immutable. |
| BR-013 | Every state transition shall generate an audit record. |
| BR-014 | Every contingency decision shall be attributable to an authorized authority. |
| BR-015 | All exception workflows shall comply with institutional examination regulations. |

---

# 14. Exception Scenarios

## 14.1 Overview

The Examination Exception & Contingency domain exists specifically to manage exceptional operational circumstances affecting examination activities.

Every exception shall be managed through standardized, auditable, policy-driven workflows while preserving institutional governance, fairness, and examination integrity.

---

## 14.2 Exception Catalogue

| Exception ID | Scenario | Resolution |
|--------------|----------|------------|
| EX-001 | Power failure | Suspend or relocate examination according to contingency policy. |
| EX-002 | Network outage | Switch to approved offline or backup examination mode where applicable. |
| EX-003 | Examination server failure | Initiate technology recovery workflow. |
| EX-004 | Question paper compromise | Cancel affected examination and initiate emergency replacement workflow. |
| EX-005 | Examination centre inaccessible | Activate alternate examination centre. |
| EX-006 | Natural disaster | Execute institutional emergency contingency plan. |
| EX-007 | Public health emergency | Apply institution-approved emergency examination policy. |
| EX-008 | Security threat | Suspend examination and activate security protocol. |
| EX-009 | Candidate medical emergency | Apply institution-approved candidate contingency procedure. |
| EX-010 | Invigilator shortage | Assign replacement personnel. |
| EX-011 | Regulatory directive received | Execute regulatory compliance workflow. |
| EX-012 | Force majeure | Escalate for institutional emergency decision. |

---

## 14.3 Technology Exceptions

Technology-related disruptions include:

- Computer-Based Test platform failure.
- Authentication service outage.
- Identity verification failure.
- Network disruption.
- Internet outage.
- Storage failure.
- Database outage.
- Examination software malfunction.
- Backup system activation.

Technology exceptions shall follow institution-approved recovery procedures.

---

## 14.4 Infrastructure Exceptions

Infrastructure disruptions include:

- Power outage.
- Fire.
- Flood.
- Building evacuation.
- Examination hall damage.
- HVAC failure.
- Seating capacity reduction.
- Access restrictions.

Institutional contingency policies shall determine examination continuation, relocation, postponement, or cancellation.

---

## 14.5 Operational Exceptions

Operational exceptions include:

- Examiner unavailable.
- Invigilator unavailable.
- Staff strike.
- Transport disruption.
- Mass candidate absence.
- Candidate identity issue.
- Incorrect examination materials.
- Scheduling conflicts.

Operational exceptions shall be resolved according to approved institutional procedures.

---

# 15. Integration Model

## 15.1 Overview

The Examination Exception & Contingency domain integrates with surrounding domains exclusively through canonical commands, published events, and business contracts.

Direct database dependencies between domains are prohibited.

---

## 15.2 Upstream Integrations

| Domain | Purpose |
|---------|---------|
| Identity | Candidate verification. |
| Authentication | User authentication. |
| Authorization | Permission validation. |
| Academic Calendar | Schedule validation. |
| Examination Scheduling | Scheduled examination context. |
| Examination Conduct | Operational disruption source. |
| Workflow Infrastructure | Workflow execution. |
| Audit Infrastructure | Immutable audit storage. |

---

## 15.3 Downstream Integrations

| Domain | Purpose |
|---------|---------|
| Examination Conduct | Resume or suspend examination execution. |
| Result Processing | Handle cancelled or postponed examinations. |
| Result Publication | Delay publication where necessary. |
| Supplementary Examination | Determine additional examination requirements. |
| Student Lifecycle | Update candidate examination status. |
| Notification Services | Stakeholder communication. |
| Reporting & Analytics | Operational reporting and KPIs. |

---

## 15.4 Integration Principles

The domain shall adhere to the following principles:

- Event-driven communication.
- Strong ownership boundaries.
- Explicit dependency management.
- No shared mutable state.
- Idempotent event consumption.
- Immutable published events.
- Versioned integration contracts.

---

# 16. Events Published

## 16.1 Overview

The domain publishes canonical business events representing operational exception lifecycle transitions.

Events are immutable and published only after successful completion of the associated business transaction.

---

## 16.2 Published Events

| Event ID | Event |
|-----------|-------|
| EVT-001 | ExaminationExceptionRegistered |
| EVT-002 | ExceptionClassified |
| EVT-003 | OperationalImpactAssessed |
| EVT-004 | ContingencyPlanPrepared |
| EVT-005 | EmergencyApprovalGranted |
| EVT-006 | ContingencyExecutionStarted |
| EVT-007 | ExaminationPostponed |
| EVT-008 | ExaminationRescheduled |
| EVT-009 | ExaminationSuspended |
| EVT-010 | ExaminationResumed |
| EVT-011 | ExaminationCancelled |
| EVT-012 | StakeholdersNotified |
| EVT-013 | ExceptionResolved |
| EVT-014 | ExceptionCaseClosed |
| EVT-015 | ExceptionCaseArchived |

---

## 16.3 Event Publication Rules

- Events shall be immutable.
- Events shall contain aggregate identifiers.
- Events shall include occurrence timestamps.
- Events shall include correlation identifiers where applicable.
- Events shall support downstream idempotency.
- Events shall not expose confidential operational information.

---

# 17. Events Consumed

## 17.1 Overview

The domain consumes canonical business events from upstream domains to initiate exception handling and contingency workflows.

Consumed events shall never modify upstream domain ownership.

---

## 17.2 Consumed Events

| Publishing Domain | Event |
|-------------------|-------|
| Examination Scheduling | ExaminationScheduled |
| Examination Conduct | ExaminationStarted |
| Examination Conduct | OperationalDisruptionDetected |
| Academic Calendar | AcademicCalendarUpdated |
| Identity | IdentityServiceUnavailable |
| Workflow Infrastructure | WorkflowStarted |
| Workflow Infrastructure | WorkflowCompleted |
| Authorization | AuthorizationGranted |
| Authorization | AuthorizationRevoked |

---

## 17.3 Event Consumption Rules

- Consumed events shall be validated before processing.
- Duplicate events shall be ignored.
- Invalid events shall be rejected.
- Failed processing shall generate audit records.
- Event replay shall not create duplicate business state.
```

# 18. Validation Framework

## 18.1 Overview

The Validation Framework defines the canonical validation rules governing every business operation within the Examination Exception & Contingency domain.

Validation ensures that every operational exception is processed using authorized, policy-compliant, auditable, and institution-approved workflows.

Every command modifying the Examination Exception Case Aggregate shall successfully complete all applicable validation stages before business execution begins.

Validation failures shall prevent state modification and return standardized business validation errors.

---

## 18.2 Validation Layers

```text
Incoming Command
        │
        ▼
Identity Validation
        │
        ▼
Authorization Validation
        │
        ▼
Business Rule Validation
        │
        ▼
Aggregate Validation
        │
        ▼
Operational Policy Validation
        │
        ▼
Workflow State Validation
        │
        ▼
Execution
```

---

## 18.3 Identity Validation

Identity validation verifies that every participating actor is uniquely identifiable.

### Validation Rules

- User identity shall exist.
- User account shall be active.
- User shall belong to the institution.
- Responsible authority shall exist.
- Emergency approver shall be authorized.
- Committee members shall be valid where applicable.
- External agencies shall be validated where required.

Failure shall terminate processing.

---

## 18.4 Authorization Validation

Every contingency operation requires authorization validation.

### Examples

| Operation | Required Authorization |
|------------|-----------------------|
| Register Exception | Authorized Reporter |
| Classify Exception | Examination Controller |
| Approve Contingency | Competent Authority |
| Suspend Examination | Examination Controller |
| Cancel Examination | Registrar / Competent Authority |
| Resume Examination | Examination Controller |
| Close Exception | Examination Controller |
| Archive Exception | Records Administrator |

Unauthorized requests shall be rejected.

---

## 18.5 Aggregate Validation

Aggregate consistency shall always be preserved.

Validation includes:

- Aggregate exists.
- Aggregate not archived.
- Aggregate state valid.
- Parent references valid.
- Duplicate operations prevented.
- Immutable records protected.

Aggregate invariant violations shall reject execution.

---

## 18.6 Workflow Validation

Workflow validation ensures lifecycle integrity.

Examples include:

- Impact assessment cannot begin before exception registration.
- Contingency planning requires completed impact assessment.
- Emergency approval requires completed contingency plan.
- Examination resumption requires successful recovery.
- Exception closure requires completion of recovery activities.
- Archived exception cases shall not be modified.

---

## 18.7 Operational Policy Validation

Institutional contingency policies shall be validated before execution.

Examples include:

- Delegation of emergency authority.
- Examination cancellation policy.
- Examination postponement policy.
- Alternate centre activation policy.
- Force majeure procedures.
- Recovery authorization.
- Regulatory notification requirements.
- Business continuity procedures.

Policy violations shall terminate execution.

---

## 18.8 Recovery Validation

Recovery activities shall satisfy:

- Recovery plan approved.
- Infrastructure restored.
- Technology operational.
- Examination venue available.
- Staff available.
- Examination materials verified.
- Candidate notifications completed.
- Required approvals obtained.

---

## 18.9 Decision Validation

Before contingency decisions become effective:

- Required authority verified.
- Institutional policy satisfied.
- Regulatory obligations completed.
- Stakeholder communication prepared.
- Effective timestamp recorded.
- Decision attributable to authorized authority.

---

## 18.10 Validation Failure Behaviour

Validation failures shall:

- Prevent state transition.
- Preserve aggregate consistency.
- Generate audit records.
- Produce standardized business validation errors.
- Never partially execute business commands.

---

# 19. Audit Requirements

## 19.1 Overview

Every operational exception shall produce a complete, immutable, and traceable audit history.

The audit record shall support:

- Institutional governance
- Operational review
- Regulatory inspection
- Business continuity review
- Internal audit
- Historical reconstruction

Audit records are immutable.

---

## 19.2 Auditable Operations

The following operations shall always generate audit entries.

| Operation | Audit Required |
|------------|----------------|
| Exception Registration | Yes |
| Exception Classification | Yes |
| Impact Assessment | Yes |
| Contingency Plan Creation | Yes |
| Emergency Approval | Yes |
| Contingency Execution | Yes |
| Examination Suspension | Yes |
| Examination Resumption | Yes |
| Examination Cancellation | Yes |
| Examination Postponement | Yes |
| Exception Closure | Yes |
| Archive Retrieval | Yes |

---

## 19.3 Audit Attributes

Each audit record shall contain:

- Audit Identifier
- Aggregate Identifier
- Operation
- Timestamp
- Performing User
- User Role
- Institution
- Correlation Identifier
- Previous State
- New State
- Business Outcome
- Source System

---

## 19.4 Audit Principles

The audit subsystem shall guarantee:

- Immutability.
- Chronological ordering.
- Tamper detection.
- Complete traceability.
- Regulatory retention.
- Historical reconstruction.
- Read-only historical access.

---

# 20. Security & Compliance

## 20.1 Overview

Exception management contains operationally sensitive institutional information.

Access shall be governed using institutional authorization policies and the Principle of Least Privilege.

---

## 20.2 Security Classification

| Information | Classification |
|--------------|----------------|
| Exception Reports | Confidential |
| Impact Assessments | Confidential |
| Contingency Plans | Restricted |
| Emergency Approvals | Restricted |
| Recovery Plans | Restricted |
| Communication Records | Internal |
| Operational Decisions | Restricted |
| Audit History | Confidential |

---

## 20.3 Security Principles

The domain shall enforce:

- Role-Based Access Control.
- Attribute-Based Authorization where configured.
- Least Privilege.
- Segregation of Duties.
- Confidentiality by Default.
- Immutable Audit History.
- Secure Communication.
- Encryption in Transit.
- Regulatory Compliance.

---

## 20.4 Compliance Requirements

The domain shall support compliance with:

- Institutional examination regulations.
- University statutes.
- Business continuity policies.
- Disaster recovery policies.
- Information security policies.
- Data retention policies.
- Applicable privacy regulations.

The architecture remains regulation-agnostic and allows institution-specific contingency policies to be configured without modifying the canonical domain model.

# 21. State Model

## 21.1 Overview

The Examination Exception Case Aggregate progresses through a well-defined lifecycle to ensure that every operational disruption is assessed, approved, managed, resolved, and archived in a controlled, auditable, and policy-compliant manner.

Each state transition shall be initiated only through authorized business commands and shall satisfy all applicable validation rules.

State transitions are immutable once committed and shall generate corresponding domain events.

---

## 21.2 Examination Exception Lifecycle

```text
Exception Reported
        │
        ▼
Exception Registered
        │
        ▼
Exception Classified
        │
        ▼
Impact Assessment In Progress
        │
        ▼
Impact Assessment Completed
        │
        ▼
Contingency Planning
        │
        ▼
Pending Approval
        │
        ▼
Approved
        │
        ▼
Contingency Executing
        │
        ├───────────────┐
        │               │
        ▼               ▼
Examination         Examination
Rescheduled         Suspended
        │               │
        └──────┬────────┘
               ▼
Recovery In Progress
               │
               ▼
Recovery Completed
               │
               ▼
Exception Closed
               │
               ▼
Archived
```

---

## 21.3 State Definitions

| State | Description |
|---------|-------------|
| Exception Reported | Operational disruption identified but not yet formally registered. |
| Exception Registered | Exception officially recorded. |
| Exception Classified | Operational category assigned. |
| Impact Assessment In Progress | Operational impact evaluation underway. |
| Impact Assessment Completed | Operational assessment finalized. |
| Contingency Planning | Recovery strategy being prepared. |
| Pending Approval | Awaiting institutional authorization. |
| Approved | Contingency authorized for execution. |
| Contingency Executing | Approved recovery activities underway. |
| Examination Rescheduled | Examination assigned a revised schedule. |
| Examination Suspended | Examination temporarily halted. |
| Recovery In Progress | Operational recovery activities executing. |
| Recovery Completed | Examination environment restored. |
| Exception Closed | Operational disruption fully resolved. |
| Archived | Historical record retained for audit and compliance. |

---

## 21.4 Valid State Transitions

| Current State | Command | Next State |
|---------------|---------|------------|
| Exception Reported | Register Exception | Exception Registered |
| Exception Registered | Classify Exception | Exception Classified |
| Exception Classified | Start Impact Assessment | Impact Assessment In Progress |
| Impact Assessment In Progress | Complete Assessment | Impact Assessment Completed |
| Impact Assessment Completed | Prepare Contingency Plan | Contingency Planning |
| Contingency Planning | Submit for Approval | Pending Approval |
| Pending Approval | Approve Contingency | Approved |
| Approved | Execute Contingency | Contingency Executing |
| Contingency Executing | Reschedule Examination | Examination Rescheduled |
| Contingency Executing | Suspend Examination | Examination Suspended |
| Examination Suspended | Start Recovery | Recovery In Progress |
| Examination Rescheduled | Start Recovery | Recovery In Progress |
| Recovery In Progress | Complete Recovery | Recovery Completed |
| Recovery Completed | Close Exception | Exception Closed |
| Exception Closed | Archive Exception | Archived |

---

## 21.5 Invalid State Transitions

The following transitions are prohibited:

- Recovery before contingency approval.
- Recovery before operational execution.
- Examination resumption without recovery completion.
- Exception closure before operational recovery.
- Archive before exception closure.
- Modification of archived Exception Cases.
- Direct transition from Registered to Closed.
- Deletion of Exception Cases.

Attempts to perform invalid transitions shall fail validation and generate audit records.

---

## 21.6 Terminal States

The aggregate contains the following terminal state:

| State | Description |
|---------|-------------|
| Archived | Final immutable state of the Examination Exception Case. |

Archived aggregates become read-only and may only be accessed by authorized users.

---

# 22. Dependency Matrix

## 22.1 Upstream Dependencies

| Domain | Dependency | Type |
|---------|------------|------|
| L2-A01 Platform Foundation | Platform services | Mandatory |
| L2-A02 Person & Identity | User identity | Mandatory |
| L2-A03 Authentication | Authentication | Mandatory |
| L2-A04 Authorization | Permission validation | Mandatory |
| L2-A05 Workflow Infrastructure | Workflow execution | Mandatory |
| L2-A06 Event Infrastructure | Event publishing | Mandatory |
| L2-A07 Audit Infrastructure | Immutable audit storage | Mandatory |
| L2-B01 Institution | Institutional policies | Mandatory |
| L2-B02 Organisation | Governance hierarchy | Mandatory |
| L2-B03 Campus & Infrastructure | Examination facilities | Mandatory |
| L2-B05 Academic Calendar | Academic schedule | Mandatory |
| L2-I03 Examination Scheduling | Scheduled examination context | Mandatory |
| L2-I06 Examination Conduct | Operational examination events | Mandatory |
| L2-I14 Malpractice & Examination Committee | Regulatory or disciplinary dependencies | Optional |

---

## 22.2 Downstream Dependencies

| Domain | Dependency | Purpose |
|---------|------------|---------|
| L2-I06 Examination Conduct | Resume, suspend or cancel examination operations |
| L2-I09 Result Processing | Manage cancelled or postponed examination consequences |
| L2-I10 Result Publication | Delay publication where required |
| L2-I12 Supplementary Examination | Determine supplementary examination necessity |
| L2-I13 Academic Completion | Handle academic progression impact |
| Student Lifecycle | Update examination status |
| Notification Services | Candidate and stakeholder communication |
| Reporting & Analytics | Operational resilience reporting |

---

## 22.3 Dependency Principles

The Examination Exception & Contingency domain shall adhere to the following dependency principles:

- Dependencies shall be explicit.
- Circular dependencies are prohibited.
- Domain ownership shall never be violated.
- Communication shall occur through canonical business contracts.
- Multi-domain integration shall be event-driven.
- Downstream domains shall not modify Exception Case aggregates.
- Published events shall remain backward compatible.

---

# 23. Appendices

## Appendix A — Typical Examination Exception Categories

Typical operational exception categories include:

- Power outage.
- Network outage.
- Internet disruption.
- Computer-Based Test platform failure.
- Examination server failure.
- Examination centre closure.
- Fire.
- Flood.
- Earthquake.
- Public health emergency.
- Security threat.
- Bomb threat.
- Law and order situation.
- Candidate medical emergency.
- Invigilator shortage.
- Staff strike.
- Question paper compromise.
- Force majeure.
- Government directive.
- Other institution-defined operational exceptions.

---

## Appendix B — Typical Contingency Actions

Examples of approved contingency actions include:

- Continue examination.
- Extend examination duration.
- Shift examination venue.
- Switch to backup infrastructure.
- Switch examination delivery mode.
- Suspend examination.
- Resume examination.
- Postpone examination.
- Reschedule examination.
- Cancel examination.
- Activate alternate examination centre.
- Initiate emergency recovery.

Institutions may define additional contingency actions according to institutional regulations.

---

## Appendix C — Design Principles

The architecture follows the enterprise principles established throughout the SutraERP repository:

- Domain-Driven Design (DDD).
- Event-Driven Architecture.
- Single Source of Truth.
- Strong Aggregate Consistency.
- CQRS Compatibility.
- Explicit Ownership Boundaries.
- Policy-Driven Governance.
- Immutable Audit History.
- Operational Resilience by Design.
- Security and Compliance by Design.

---

# Architecture Completion Checklist

| Item | Status |
|------|--------|
| Purpose Defined | ✓ |
| Scope Defined | ✓ |
| Repository Context Established | ✓ |
| Domain Ownership Defined | ✓ |
| Aggregate Model Defined | ✓ |
| Workflow Catalogue Complete | ✓ |
| Workflow Specifications Complete | ✓ |
| Business Rules Defined | ✓ |
| Exception Scenarios Defined | ✓ |
| Integration Model Defined | ✓ |
| Published Events Defined | ✓ |
| Consumed Events Defined | ✓ |
| Validation Framework Complete | ✓ |
| Audit Requirements Complete | ✓ |
| Security & Compliance Complete | ✓ |
| State Model Complete | ✓ |
| Dependency Matrix Complete | ✓ |
| Appendices Complete | ✓ |

---

# End of Document

**L2-I15 — Examination Exception & Contingency Workflows**

**Status:** Draft

**Next Document:** **L2-I16 — Examination Session Closure & Archival Workflows**





