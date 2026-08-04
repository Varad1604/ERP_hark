# L2-I14 — Malpractice & Examination Committee Workflows

---

# Document Control

| Attribute | Value |
|------------|-------|
| Document ID | L2-I14 |
| Document Name | Malpractice & Examination Committee Workflows |
| Domain | Examination Domain |
| Architecture Level | L2 – Canonical Domain Workflow |
| Owner | Examination Domain |
| Document Owner | Devang |
| Repository | SutraERP Enterprise Architecture Repository |
| Status | Draft |
| Depends On | L1 Business Domain Architecture, L2-A01–A08, L2-B01–B05, L2-I01–L2-I13 |
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

The Malpractice & Examination Committee domain defines the canonical business architecture governing the identification, investigation, adjudication, and resolution of examination malpractice across the SutraERP platform.

This domain ensures that every allegation of examination misconduct follows a standardized, policy-driven, auditable, and institution-approved workflow while preserving fairness, due process, academic integrity, and regulatory compliance.

The architecture provides a single authoritative business process for managing malpractice cases from initial reporting through investigation, committee review, decision, penalty enforcement, appeal handling (where permitted), and archival.

This document is implementation-independent and defines business ownership, workflows, responsibilities, events, and validation rules for the Malpractice & Examination Committee bounded context.

---

## 1.2 Objectives

The objectives of this domain are to:

- Establish a single authoritative malpractice management workflow.
- Standardize examination disciplinary processes.
- Support configurable institutional malpractice policies.
- Ensure due process for all accused candidates.
- Enable evidence-based committee decision making.
- Maintain complete auditability of disciplinary actions.
- Support regulatory and accreditation compliance.
- Prevent ownership overlap with other Examination domain workflows.
- Publish canonical malpractice events for downstream domains.

---

# 2. Scope

## 2.1 In Scope

This domain governs:

### Malpractice Case Management

- Malpractice allegation registration
- Incident reporting
- Case creation
- Case classification
- Case assignment
- Case tracking
- Case closure

### Investigation Management

- Evidence collection
- Evidence verification
- Invigilator reports
- Student explanations
- Witness statements
- Investigation workflow

### Examination Committee

- Committee constitution
- Committee scheduling
- Agenda preparation
- Case review
- Hearing management
- Recommendation preparation
- Decision recording

### Penalty Management

- Penalty recommendation
- Penalty approval
- Penalty publication
- Result impact recommendation
- Suspension recommendation
- Debarment recommendation
- Warning issuance

### Appeals

- Appeal registration
- Appeal review
- Appeal committee recommendation
- Final decision recording

### Governance

- Policy validation
- Due process validation
- Regulatory compliance
- Audit traceability
- Historical preservation

---

## 2.2 Out of Scope

The following responsibilities are owned by other canonical domains.

| Capability | Owning Domain |
|------------|---------------|
| Student Identity | L2-A02 |
| Authentication | L2-A03 |
| Authorization | L2-A04 |
| Workflow Engine | L2-A05 |
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

---

# 3. Repository Context

## 3.1 Position within the Repository

The Malpractice & Examination Committee domain is a governance domain within the Examination bounded context.

It operates after examination conduct and may influence downstream workflows such as result processing, result publication, supplementary examinations, and qualification completion by issuing authoritative disciplinary decisions.

No other domain may independently investigate or adjudicate examination malpractice.

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
- L2-B04 Curriculum Framework
- L2-B05 Academic Calendar
- L2-I01–L2-I13 Examination Domains

### Downstream

- Result Processing
- Result Publication
- Academic Completion
- Student Lifecycle
- Reporting & Analytics
- Audit Infrastructure
- Regulatory Reporting

---

# 4. Architectural Principles

The Malpractice & Examination Committee domain shall adhere to the following principles:

1. Single Source of Truth for malpractice cases.
2. Policy-driven disciplinary governance.
3. Separation of investigation and adjudication responsibilities.
4. Due process for every accused candidate.
5. Immutable audit history.
6. Event-driven integration.
7. Domain ownership without overlap.
8. Implementation independence.
9. Configurable institutional policies.
10. Regulatory compliance by design.

# 5. Domain Overview

## 5.1 Overview

The Malpractice & Examination Committee domain governs the complete disciplinary lifecycle associated with examination-related misconduct.

It provides a structured, policy-driven framework for recording suspected malpractice, conducting investigations, evaluating evidence, facilitating committee proceedings, issuing disciplinary decisions, managing appeals, and preserving permanent institutional records.

This domain acts as the authoritative governance layer responsible for maintaining academic integrity throughout the examination ecosystem.

The domain does not conduct examinations, evaluate answer scripts, calculate results, or determine academic completion. Instead, it governs disciplinary actions that may subsequently influence those downstream academic processes.

---

## 5.2 Business Objectives

The domain shall:

- Protect examination integrity.
- Ensure fairness and due process.
- Standardize malpractice investigations.
- Enable evidence-based disciplinary decisions.
- Support institution-specific malpractice regulations.
- Maintain complete traceability of disciplinary proceedings.
- Prevent unauthorized modification of committee decisions.
- Publish authoritative malpractice outcomes to dependent domains.
- Preserve historical disciplinary records for audit and regulatory purposes.

---

## 5.3 Core Responsibilities

The Malpractice & Examination Committee domain owns responsibility for:

- Registering malpractice incidents.
- Creating malpractice cases.
- Classifying malpractice categories.
- Assigning investigation ownership.
- Managing evidence collection.
- Recording candidate explanations.
- Recording witness statements.
- Scheduling committee hearings.
- Managing committee deliberations.
- Recording disciplinary decisions.
- Assigning institutional penalties.
- Managing appeals.
- Closing disciplinary cases.
- Publishing malpractice events.
- Maintaining permanent disciplinary history.

---

## 5.4 Architectural Characteristics

| Characteristic | Description |
|----------------|-------------|
| Domain Type | Governance Domain |
| Ownership Model | Single Aggregate Ownership |
| Architecture Style | Domain Driven Design |
| Integration Style | Event Driven |
| Consistency Model | Strong Consistency |
| Auditability | Mandatory |
| Security Classification | Highly Confidential |
| Policy Driven | Yes |
| Regulatory Sensitive | Yes |

---

# 6. Business Capabilities

## 6.1 Capability Overview

The domain is composed of the following canonical business capabilities.

| Capability ID | Capability | Description |
|---------------|------------|-------------|
| MC-01 | Incident Registration | Register suspected malpractice incidents. |
| MC-02 | Case Management | Create and manage malpractice cases. |
| MC-03 | Investigation Management | Conduct investigations and collect evidence. |
| MC-04 | Evidence Management | Maintain documentary and digital evidence. |
| MC-05 | Committee Administration | Manage examination committee activities. |
| MC-06 | Hearing Management | Conduct disciplinary hearings. |
| MC-07 | Decision Management | Record committee verdicts. |
| MC-08 | Penalty Management | Apply institutional penalties. |
| MC-09 | Appeal Management | Process appeals where permitted. |
| MC-10 | Case Closure | Close disciplinary proceedings. |
| MC-11 | Audit & Compliance | Preserve audit history and compliance evidence. |
| MC-12 | Event Publication | Publish canonical malpractice events. |

---

## 6.2 Capability Relationships

```text
Incident Report
        │
        ▼
Incident Registration
        │
        ▼
Case Management
        │
        ▼
Investigation Management
        │
        ▼
Evidence Collection
        │
        ▼
Committee Review
        │
        ▼
Hearing
        │
        ▼
Committee Decision
        │
        ▼
Penalty Assignment
        │
        ▼
Appeal (Optional)
        │
        ▼
Case Closure
```

---

# 7. Business Actors

## 7.1 Internal Actors

| Actor | Responsibility |
|--------|----------------|
| Examination Controller | Overall disciplinary governance. |
| Chief Superintendent | Reports examination malpractice. |
| Invigilator | Reports examination misconduct. |
| Investigation Officer | Conducts malpractice investigation. |
| Committee Chairperson | Leads committee proceedings. |
| Committee Member | Reviews evidence and participates in decisions. |
| Examination Administrator | Coordinates committee operations. |
| Registrar | Approves institution-level disciplinary actions where required. |

---

## 7.2 External Actors

| Actor | Responsibility |
|--------|----------------|
| Student | Responds to allegations and participates in hearings. |
| Parent / Guardian | Receives communication where institutional policy permits. |
| External Expert | Participates in investigations if required. |
| Regulatory Authority | Reviews disciplinary compliance during audits. |

---

## 7.3 System Actors

| System | Interaction |
|---------|-------------|
| Identity Domain | User identity verification. |
| Authorization Domain | Access control validation. |
| Workflow Engine | Executes disciplinary workflows. |
| Notification Domain | Sends disciplinary notifications. |
| Audit Infrastructure | Stores immutable audit records. |
| Event Infrastructure | Publishes malpractice events. |

---

# 8. Ubiquitous Language

The following terminology shall be used consistently throughout the repository.

| Term | Definition |
|------|------------|
| Malpractice Incident | A suspected breach of examination regulations. |
| Malpractice Case | The official disciplinary record created from an incident. |
| Investigation | Formal examination of the reported incident. |
| Evidence | Material supporting or refuting allegations. |
| Committee | Authorized body responsible for disciplinary decisions. |
| Hearing | Formal session allowing parties to present information. |
| Recommendation | Committee proposal prior to final approval where applicable. |
| Decision | Official disciplinary outcome. |
| Penalty | Institutional disciplinary action imposed after a decision. |
| Appeal | Formal request to review a disciplinary decision. |
| Case Closure | Completion of all disciplinary activities. |

---

# 9. Domain Ownership

## 9.1 Owns

The Malpractice & Examination Committee domain exclusively owns:

- Malpractice incidents
- Malpractice cases
- Investigation lifecycle
- Evidence lifecycle
- Committee proceedings
- Hearing records
- Committee decisions
- Penalty recommendations
- Appeal lifecycle
- Case closure
- Malpractice audit metadata
- Malpractice domain events

No other domain may create, modify, or close malpractice cases.

---

## 9.2 Does Not Own

The domain explicitly does not own:

- Student master data
- Examination schedules
- Hall tickets
- Attendance
- Question papers
- Answer scripts
- Marks evaluation
- Grade calculation
- Result publication
- Degree completion
- Certificate generation

Ownership of these capabilities remains with their respective canonical domains.

# 10. Aggregate Model

## 10.1 Aggregate Overview

The Malpractice & Examination Committee domain is modelled as a single transactional aggregate centred around the **Malpractice Case**.

A Malpractice Case acts as the authoritative business entity governing every disciplinary activity associated with an examination misconduct incident.

All modifications to investigations, evidence, hearings, committee decisions, penalties, appeals, and case closure shall occur within the governance boundaries of the Malpractice Case aggregate.

No external domain may directly modify aggregate state.

---

## 10.2 Aggregate Root

**Aggregate Root**

> Malpractice Case

The aggregate root is responsible for:

- Maintaining disciplinary lifecycle integrity.
- Enforcing institutional malpractice policies.
- Managing investigation progression.
- Managing committee proceedings.
- Recording committee decisions.
- Applying disciplinary outcomes.
- Coordinating appeals.
- Publishing domain events.
- Maintaining complete audit traceability.

---

## 10.3 Aggregate Components

```text
Malpractice Case
│
├── Incident
├── Investigation
├── Evidence Collection
├── Candidate Response
├── Witness Statements
├── Committee Session
├── Hearing
├── Committee Recommendation
├── Committee Decision
├── Penalty
├── Appeal
├── Case Closure
└── Audit Metadata
```

---

## 10.4 Aggregate Invariants

The aggregate shall enforce the following invariants.

- Every malpractice case originates from a registered incident.
- Every investigation belongs to exactly one malpractice case.
- Committee hearings cannot occur before investigation completion unless institutional policy explicitly permits.
- Committee decisions require an authorised committee session.
- A closed case cannot return to investigation.
- Penalties cannot exist without an approved committee decision.
- Appeals may only be initiated against eligible decisions.
- Every disciplinary action shall be auditable.
- Aggregate consistency shall be preserved throughout every state transition.

---

# 11. Workflow Catalogue

## 11.1 Workflow Overview

The domain consists of the following canonical workflows.

| Workflow ID | Workflow Name |
|--------------|---------------|
| MW-01 | Register Malpractice Incident |
| MW-02 | Create Malpractice Case |
| MW-03 | Classify Incident |
| MW-04 | Assign Investigation |
| MW-05 | Conduct Investigation |
| MW-06 | Collect Evidence |
| MW-07 | Record Candidate Explanation |
| MW-08 | Record Witness Statements |
| MW-09 | Schedule Committee Meeting |
| MW-10 | Conduct Committee Hearing |
| MW-11 | Record Committee Recommendation |
| MW-12 | Record Committee Decision |
| MW-13 | Assign Penalty |
| MW-14 | Notify Stakeholders |
| MW-15 | Register Appeal |
| MW-16 | Review Appeal |
| MW-17 | Close Case |
| MW-18 | Archive Case |

---

# 12. Workflow Specifications

## MW-01 Register Malpractice Incident

### Purpose

Register an official allegation of examination malpractice.

### Trigger

- Invigilator report
- Chief Superintendent report
- Digital examination monitoring
- Evaluation irregularity
- Identity verification failure
- Institutional complaint

### Preconditions

- Examination session exists.
- Candidate exists.
- Reporter is authorised.
- Examination event has occurred.

### Workflow

1. Receive malpractice report.
2. Validate reporting authority.
3. Capture incident information.
4. Capture examination details.
5. Capture candidate details.
6. Assign preliminary category.
7. Generate incident reference.
8. Publish incident registration event.

### Postconditions

- Incident registered.
- Incident reference created.
- Investigation pending.

---

## MW-02 Create Malpractice Case

### Purpose

Convert a validated incident into a formal disciplinary case.

### Preconditions

- Incident registered.
- Initial validation complete.

### Workflow

1. Review incident.
2. Create malpractice case.
3. Allocate case identifier.
4. Assign investigation status.
5. Assign responsible authority.
6. Record audit metadata.

### Postconditions

- Case established.
- Investigation initiated.

---

## MW-03 Classify Incident

### Purpose

Categorise malpractice according to institutional regulations.

### Typical Categories

- Copying
- Possession of prohibited material
- Mobile device usage
- Identity impersonation
- Candidate impersonation
- Answer sheet substitution
- Collusion
- External assistance
- Digital examination misconduct
- Invigilator misconduct
- Evaluation malpractice
- Other policy-defined misconduct

### Postconditions

- Classification assigned.
- Applicable institutional policy determined.

---

## MW-04 Assign Investigation

### Purpose

Assign responsibility for disciplinary investigation.

### Workflow

1. Select investigation officer.
2. Verify conflict of interest.
3. Allocate case.
4. Notify investigator.
5. Record assignment.

### Outputs

- Investigation assignment completed.

---

## MW-05 Conduct Investigation

### Purpose

Perform formal investigation into the malpractice allegation.

### Investigation Activities

- Review incident report.
- Interview reporting personnel.
- Interview candidate.
- Collect documentary evidence.
- Verify examination records.
- Review surveillance evidence.
- Evaluate digital examination logs.
- Prepare investigation findings.

### Outputs

- Investigation report.
- Supporting evidence.
- Recommendation for committee review.

---

## MW-06 Collect Evidence

### Evidence Types

- Answer book
- Seating plan
- Invigilator report
- Candidate statement
- CCTV recording
- Digital examination logs
- Attendance sheet
- Identity verification records
- Photographs
- Electronic devices (where institution policy permits)

### Business Rules

- Evidence shall remain immutable after submission.
- Evidence chain-of-custody shall be maintained.
- Every evidence item shall be uniquely identifiable.
- Evidence access shall be fully audited.

---

## MW-07 Record Candidate Explanation

Candidates shall be provided an opportunity to submit an explanation within institution-defined timelines.

The explanation shall become part of the permanent disciplinary record.

Failure to respond shall not invalidate disciplinary proceedings unless institutional policy explicitly requires candidate participation.

---

## MW-08 Record Witness Statements

Witness statements may be recorded from:

- Invigilators
- Supervisors
- Examination staff
- Technical personnel
- External observers
- Other authorised witnesses

Each statement shall be timestamped, attributable, and immutable after submission.

## MW-09 Schedule Committee Meeting

### Purpose

Schedule an official Examination Malpractice Committee meeting for review of one or more malpractice cases.

### Preconditions

- Investigation completed.
- Investigation report approved.
- Evidence package finalized.
- Committee members available.
- Required quorum achievable.

### Workflow

1. Identify eligible committee members.
2. Validate conflict of interest.
3. Verify committee quorum.
4. Prepare meeting agenda.
5. Attach investigation reports.
6. Attach supporting evidence.
7. Schedule meeting.
8. Notify committee members.
9. Notify candidate where institutional policy requires.
10. Publish committee schedule.

### Postconditions

- Committee meeting scheduled.
- Agenda finalized.
- Hearing preparation completed.

---

## MW-10 Conduct Committee Hearing

### Purpose

Conduct a formal disciplinary hearing before the Examination Malpractice Committee.

### Preconditions

- Committee meeting scheduled.
- Required quorum available.
- Investigation completed.
- Evidence package available.

### Workflow

1. Open committee session.
2. Verify committee quorum.
3. Present investigation summary.
4. Review evidence.
5. Hear candidate explanation.
6. Hear witness statements.
7. Seek clarifications.
8. Record committee observations.
9. Close hearing.
10. Forward case for deliberation.

### Postconditions

- Hearing completed.
- Hearing minutes recorded.
- Case ready for committee decision.

---

## MW-11 Record Committee Recommendation

### Purpose

Record the committee's recommendation prior to the final disciplinary decision where institutional policy distinguishes recommendation from approval.

### Possible Recommendations

- No malpractice established.
- Warning.
- Subject cancellation.
- Examination cancellation.
- Semester cancellation.
- Debarment.
- Re-examination.
- Suspension.
- Withhold result.
- Escalation to higher authority.

### Outputs

- Committee recommendation recorded.
- Recommendation published internally.

---

## MW-12 Record Committee Decision

### Purpose

Record the official disciplinary decision issued by the competent authority.

### Preconditions

- Committee deliberation completed.
- Required approvals obtained.
- Institutional policy satisfied.

### Workflow

1. Review committee recommendation.
2. Validate institutional rules.
3. Approve final decision.
4. Record disciplinary outcome.
5. Record effective date.
6. Record approving authority.
7. Lock decision against modification.
8. Publish decision event.

### Postconditions

- Official decision finalized.
- Decision becomes immutable.
- Downstream domains notified.

---

## MW-13 Assign Penalty

### Purpose

Apply disciplinary penalties resulting from committee decisions.

### Typical Penalties

- Written warning.
- Examination cancellation.
- Subject cancellation.
- Semester cancellation.
- Result withholding.
- Zero marks.
- Fine (institution policy).
- Temporary suspension.
- Debarment for specified examination sessions.
- Permanent debarment.
- Referral to institutional disciplinary authority.

### Business Rules

- Penalties shall originate only from an approved committee decision.
- Multiple penalties may be applied where institutional regulations permit.
- Penalties shall include an effective period where applicable.
- Penalties shall be fully auditable.
- Revoked penalties shall preserve historical records.

---

## MW-14 Notify Stakeholders

### Purpose

Notify all relevant stakeholders of disciplinary outcomes.

### Stakeholders

- Student
- Parent / Guardian (where permitted)
- Examination Controller
- Registrar
- Academic Department
- Result Processing Domain
- Student Lifecycle Domain
- Regulatory Authority (where required)

### Notification Channels

- ERP Notification
- Email
- SMS
- Official Letter
- Student Portal
- Institution Dashboard

### Outputs

- Notifications dispatched.
- Delivery status recorded.
- Audit trail updated.

---

## MW-15 Register Appeal

### Purpose

Register an appeal against a disciplinary decision where institutional regulations permit.

### Preconditions

- Appeal window open.
- Decision appealable.
- Student eligible.
- Appeal submitted within prescribed timeline.

### Workflow

1. Receive appeal.
2. Validate eligibility.
3. Register appeal.
4. Assign appeal identifier.
5. Notify appeal authority.
6. Schedule appeal review.

### Postconditions

- Appeal registered.
- Appeal workflow initiated.

---

## MW-16 Review Appeal

### Purpose

Conduct an independent review of the disciplinary decision.

### Workflow

1. Review original case.
2. Review committee proceedings.
3. Review appeal grounds.
4. Review additional evidence.
5. Conduct appeal hearing if required.
6. Record appeal decision.
7. Notify stakeholders.

### Possible Outcomes

- Decision upheld.
- Penalty modified.
- Penalty reduced.
- Penalty revoked.
- Case reopened.
- Appeal rejected.

---

## MW-17 Close Case

### Purpose

Complete all disciplinary activities and formally close the malpractice case.

### Preconditions

- Final decision issued.
- Appeal resolved or appeal period expired.
- All notifications completed.
- Penalties recorded.

### Workflow

1. Verify completion checklist.
2. Confirm outstanding activities.
3. Close case.
4. Publish closure event.
5. Archive disciplinary records.

### Postconditions

- Case permanently closed.
- Workflow completed.
- Historical records preserved.

---

## MW-18 Archive Case

### Purpose

Archive completed malpractice cases for long-term institutional retention.

### Archival Contents

- Incident record.
- Investigation report.
- Evidence inventory.
- Candidate explanation.
- Witness statements.
- Hearing minutes.
- Committee recommendations.
- Committee decision.
- Penalty records.
- Appeal records.
- Audit history.

### Business Rules

- Archived cases shall remain read-only.
- Archive retention shall follow institutional and regulatory policies.
- Archived records shall remain searchable by authorized users.
- Archived records shall never be physically deleted except under approved retention policies.
- Archive retrieval shall be fully audited.

---

# 13. Business Rules

## 13.1 General Rules

| Rule ID | Business Rule |
|----------|---------------|
| BR-001 | Every malpractice allegation shall result in a uniquely identifiable incident record. |
| BR-002 | A malpractice case shall originate from exactly one registered incident. |
| BR-003 | Investigation shall commence only after case creation. |
| BR-004 | Every investigation shall have an assigned responsible officer. |
| BR-005 | All evidence shall maintain chain-of-custody. |
| BR-006 | Committee hearings shall satisfy institutional quorum requirements. |
| BR-007 | Committee decisions shall be recorded before penalties are applied. |
| BR-008 | Appeals shall only be accepted within institution-defined timelines. |
| BR-009 | Closed cases shall not return to investigation unless officially reopened. |
| BR-010 | Every state transition shall generate an audit record. |
| BR-011 | Every disciplinary decision shall be attributable to an authorized authority. |
| BR-012 | Malpractice records shall be retained according to institutional retention policies. |
| BR-013 | Committee decisions shall become immutable after final approval. |
| BR-014 | Unauthorized users shall never access confidential disciplinary records. |
| BR-015 | All malpractice workflows shall comply with institutional examination regulations. |

# 14. Exception Scenarios

## 14.1 Overview

The Malpractice & Examination Committee domain shall support standardized handling of exceptional disciplinary scenarios to ensure fairness, regulatory compliance, and institutional governance.

Exception workflows shall preserve auditability, maintain process integrity, and prevent unauthorized disciplinary outcomes.

---

## 14.2 Exception Catalogue

| Exception ID | Scenario | Resolution |
|--------------|----------|------------|
| EX-001 | Duplicate malpractice report | Merge or reject duplicate according to institutional policy. |
| EX-002 | False allegation | Close case with no disciplinary action. |
| EX-003 | Reporter not authorized | Reject incident registration. |
| EX-004 | Evidence missing | Keep investigation pending until evidence is available or investigation concludes. |
| EX-005 | Evidence tampering suspected | Escalate for independent investigation. |
| EX-006 | Committee quorum unavailable | Reschedule committee meeting. |
| EX-007 | Committee member conflict of interest | Replace conflicted member before hearing. |
| EX-008 | Candidate absent from hearing | Proceed according to institutional regulations. |
| EX-009 | Appeal submitted after deadline | Reject unless exceptional approval exists. |
| EX-010 | Penalty conflicts with institutional regulations | Escalate for administrative review. |
| EX-011 | Decision approval withdrawn | Return decision for committee reconsideration. |
| EX-012 | Archived case requested for review | Reopen only through authorized workflow. |

---

## 14.3 Investigation Exceptions

Investigation may be interrupted when:

- Required evidence cannot be obtained.
- Witnesses become unavailable.
- Identity verification fails.
- Examination records are incomplete.
- Digital examination logs are corrupted.
- External regulatory investigation is initiated.

Such interruptions shall generate investigation exception events and remain fully auditable.

---

## 14.4 Hearing Exceptions

Committee hearings may encounter:

- Quorum failure.
- Chairperson unavailable.
- Candidate absence.
- Witness absence.
- New evidence submitted during hearing.
- Hearing adjournment.
- Hearing cancellation.

Institutional policy shall determine whether proceedings continue or are rescheduled.

---

## 14.5 Decision Exceptions

Committee decisions may require reconsideration when:

- Procedural violations are identified.
- Material evidence was unavailable.
- Regulatory authority intervenes.
- Administrative approval is withheld.
- Successful appeal overturns the decision.

Every reconsideration shall preserve the original decision as immutable historical data.

---

# 15. Integration Model

## 15.1 Overview

The Malpractice & Examination Committee domain integrates with surrounding domains exclusively through canonical commands, events, and published business contracts.

Direct database dependencies between domains are prohibited.

---

## 15.2 Upstream Integrations

| Domain | Purpose |
|---------|---------|
| Identity | Candidate verification. |
| Authentication | User authentication. |
| Authorization | Permission validation. |
| Examination Conduct | Examination incident source. |
| Evaluation | Evaluation-related malpractice reports. |
| Workflow Infrastructure | Workflow execution. |
| Audit Infrastructure | Immutable audit storage. |

---

## 15.3 Downstream Integrations

| Domain | Purpose |
|---------|---------|
| Result Processing | Apply disciplinary impact on results. |
| Result Publication | Withhold or publish results according to disciplinary outcome. |
| Supplementary Examination | Determine eligibility where required. |
| Academic Completion | Apply qualification completion restrictions where applicable. |
| Student Lifecycle | Update disciplinary history. |
| Notification | Inform stakeholders. |
| Analytics | Institutional reporting. |

---

## 15.4 Integration Principles

The domain shall adhere to the following integration principles:

- Event-driven communication.
- No shared mutable data.
- Strong ownership boundaries.
- Idempotent event consumption.
- Immutable published events.
- Versioned integration contracts.
- Backward-compatible event evolution.

---

# 16. Events Published

## 16.1 Overview

The domain publishes canonical business events representing significant disciplinary lifecycle transitions.

Events are immutable and shall be published only after successful completion of the corresponding business transaction.

---

## 16.2 Published Events

| Event ID | Event |
|-----------|-------|
| EVT-001 | MalpracticeIncidentRegistered |
| EVT-002 | MalpracticeCaseCreated |
| EVT-003 | InvestigationAssigned |
| EVT-004 | InvestigationStarted |
| EVT-005 | InvestigationCompleted |
| EVT-006 | EvidenceCollected |
| EVT-007 | CandidateExplanationRecorded |
| EVT-008 | WitnessStatementRecorded |
| EVT-009 | CommitteeMeetingScheduled |
| EVT-010 | CommitteeHearingCompleted |
| EVT-011 | CommitteeRecommendationRecorded |
| EVT-012 | CommitteeDecisionRecorded |
| EVT-013 | PenaltyAssigned |
| EVT-014 | StakeholdersNotified |
| EVT-015 | AppealRegistered |
| EVT-016 | AppealReviewed |
| EVT-017 | CaseClosed |
| EVT-018 | CaseArchived |

---

## 16.3 Event Publication Rules

- Events shall be immutable.
- Events shall contain aggregate identifiers.
- Events shall include occurrence timestamps.
- Events shall contain correlation identifiers where applicable.
- Events shall never expose confidential evidence.
- Events shall support downstream idempotency.

---

# 17. Events Consumed

## 17.1 Overview

The domain consumes business events published by upstream canonical domains.

Consumed events trigger business workflows but shall never modify upstream domain ownership.

---

## 17.2 Consumed Events

| Publishing Domain | Event |
|-------------------|-------|
| Examination Conduct | ExaminationCompleted |
| Examination Conduct | CandidateReported |
| Evaluation | EvaluationIrregularityDetected |
| Identity | IdentityVerified |
| Identity | IdentityMismatchDetected |
| Workflow Infrastructure | WorkflowStarted |
| Workflow Infrastructure | WorkflowCompleted |
| Authorization | AuthorizationGranted |
| Authorization | AuthorizationRevoked |

---

## 17.3 Event Consumption Rules

- Events shall be validated before processing.
- Duplicate events shall be ignored.
- Invalid events shall be rejected.
- Processing failures shall be auditable.
- Event replay shall not create duplicate business state.
```

# 18. Validation Framework

## 18.1 Overview

The Validation Framework defines the canonical validation rules governing every business operation within the Malpractice & Examination Committee domain.

Validation ensures that only complete, authorized, policy-compliant, and auditable disciplinary actions are performed.

Every command modifying the Malpractice Case Aggregate shall successfully pass all applicable validation stages before business execution begins.

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
Policy Validation
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

- User identity must exist.
- User account must be active.
- User must belong to the institution.
- Candidate identity must exist.
- Committee members must be registered.
- Investigation officer must exist.
- Witness identity shall be verifiable where applicable.

Failure shall terminate processing.

---

## 18.4 Authorization Validation

Every disciplinary operation requires authorization validation.

### Examples

| Operation | Required Authorization |
|------------|-----------------------|
| Register Incident | Authorized Reporter |
| Assign Investigation | Examination Controller |
| Upload Evidence | Investigation Officer |
| Schedule Committee | Committee Administrator |
| Conduct Hearing | Committee Chairperson |
| Record Decision | Authorized Committee |
| Apply Penalty | Competent Authority |
| Close Case | Examination Controller |
| Archive Case | Records Administrator |

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
- Immutable data protected.

Aggregate invariant violations shall reject the command.

---

## 18.6 Workflow Validation

Workflow validation ensures lifecycle integrity.

Examples include:

- Investigation cannot begin before case creation.
- Hearing cannot occur before investigation completion.
- Decision cannot be recorded before hearing completion.
- Penalty cannot be applied before decision approval.
- Appeal cannot begin before decision publication.
- Case cannot close while appeal remains pending.

---

## 18.7 Policy Validation

Institutional policies shall be validated before execution.

Examples include:

- Penalty matrix validation.
- Appeal eligibility.
- Appeal submission period.
- Committee quorum.
- Required approvals.
- Maximum disciplinary authority.
- Delegation rules.
- Regulatory compliance.

Policy violations shall terminate execution.

---

## 18.8 Evidence Validation

Evidence submitted during investigations shall satisfy:

- Evidence type supported.
- Source identified.
- Upload completed.
- Hash verification successful (for digital evidence).
- Chain-of-custody maintained.
- Duplicate evidence detected.
- File integrity preserved.
- Mandatory metadata captured.

---

## 18.9 Decision Validation

Before disciplinary decisions become final:

- Committee quorum verified.
- Required members participated.
- Investigation completed.
- Recommendation recorded (where applicable).
- Required approvals obtained.
- Penalty permitted by institutional regulations.
- Decision timestamp recorded.
- Approving authority verified.

---

## 18.10 Validation Failure Behaviour

Validation failures shall:

- Prevent state transition.
- Preserve aggregate consistency.
- Generate audit records.
- Produce standardized business errors.
- Never partially execute commands.

---

# 19. Audit Requirements

## 19.1 Overview

Every disciplinary action shall produce a complete, immutable, and traceable audit history.

The audit record shall support:

- Institutional governance
- Internal audit
- Regulatory inspection
- Legal proceedings
- Accreditation reviews
- Historical reconstruction

Audit records are immutable.

---

## 19.2 Auditable Operations

The following operations shall always generate audit entries.

| Operation | Audit Required |
|------------|----------------|
| Incident Registration | Yes |
| Case Creation | Yes |
| Investigation Assignment | Yes |
| Evidence Upload | Yes |
| Evidence Modification Attempt | Yes |
| Hearing Scheduling | Yes |
| Committee Attendance | Yes |
| Committee Decision | Yes |
| Penalty Assignment | Yes |
| Appeal Registration | Yes |
| Appeal Decision | Yes |
| Case Closure | Yes |
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

- Immutability
- Chronological ordering
- Tamper detection
- Complete traceability
- Regulatory retention
- Historical reconstruction
- Read-only historical access

---

# 20. Security & Compliance

## 20.1 Overview

Malpractice cases contain highly confidential disciplinary information.

Access shall be governed using institutional authorization policies and the Principle of Least Privilege.

---

## 20.2 Security Classification

| Information | Classification |
|--------------|----------------|
| Incident Reports | Confidential |
| Investigation Reports | Highly Confidential |
| Candidate Statements | Highly Confidential |
| Witness Statements | Highly Confidential |
| Committee Minutes | Restricted |
| Committee Decisions | Restricted |
| Penalty Records | Restricted |
| Appeal Records | Restricted |
| Audit History | Confidential |

---

## 20.3 Security Principles

The domain shall enforce:

- Role-based access control.
- Attribute-based authorization where configured.
- Least privilege.
- Segregation of duties.
- Confidentiality by default.
- Immutable audit history.
- Secure evidence storage.
- Encrypted communication.
- Regulatory compliance.

---

## 20.4 Compliance Requirements

The domain shall support compliance with:

- Institutional examination regulations.
- University statutes.
- Accreditation requirements.
- National education regulations.
- Information security policies.
- Data retention policies.
- Privacy regulations applicable to the institution.

The architecture remains regulation-agnostic and allows institution-specific compliance policies to be configured without changing the canonical domain model.

# 21. State Model

## 21.1 Overview

The Malpractice Case Aggregate progresses through a well-defined lifecycle to ensure disciplinary actions are executed in a controlled, auditable, and policy-compliant manner.

Each state transition shall be initiated only through valid business commands and shall satisfy all applicable validation rules.

State transitions are immutable once committed and shall generate corresponding domain events.

---

## 21.2 Malpractice Case Lifecycle

```text
Incident Reported
        │
        ▼
Incident Registered
        │
        ▼
Case Created
        │
        ▼
Investigation Assigned
        │
        ▼
Investigation In Progress
        │
        ▼
Investigation Completed
        │
        ▼
Committee Scheduled
        │
        ▼
Hearing Conducted
        │
        ▼
Decision Pending
        │
        ▼
Decision Recorded
        │
        ▼
Penalty Assigned
        │
        ▼
Appeal Window Open
        │
        ├───────────────┐
        ▼               │
Appeal Registered       │
        │               │
        ▼               │
Appeal Reviewed         │
        │               │
        └──────┬────────┘
               ▼
Case Closed
        │
        ▼
Archived
```

---

## 21.3 State Definitions

| State | Description |
|---------|-------------|
| Incident Reported | Suspected malpractice has been reported but not yet formally registered. |
| Incident Registered | Incident has been officially recorded. |
| Case Created | Formal malpractice case established. |
| Investigation Assigned | Investigation responsibility allocated. |
| Investigation In Progress | Evidence collection and inquiry underway. |
| Investigation Completed | Investigation finalized. |
| Committee Scheduled | Committee meeting planned. |
| Hearing Conducted | Formal hearing completed. |
| Decision Pending | Committee deliberation in progress. |
| Decision Recorded | Official disciplinary decision finalized. |
| Penalty Assigned | Institutional penalty applied. |
| Appeal Window Open | Candidate eligible to submit appeal. |
| Appeal Registered | Appeal accepted for review. |
| Appeal Reviewed | Appeal decision completed. |
| Case Closed | All disciplinary activities completed. |
| Archived | Case preserved for historical retention. |

---

## 21.4 Valid State Transitions

| Current State | Command | Next State |
|---------------|---------|------------|
| Incident Reported | Register Incident | Incident Registered |
| Incident Registered | Create Case | Case Created |
| Case Created | Assign Investigation | Investigation Assigned |
| Investigation Assigned | Start Investigation | Investigation In Progress |
| Investigation In Progress | Complete Investigation | Investigation Completed |
| Investigation Completed | Schedule Committee | Committee Scheduled |
| Committee Scheduled | Conduct Hearing | Hearing Conducted |
| Hearing Conducted | Record Decision | Decision Pending |
| Decision Pending | Finalize Decision | Decision Recorded |
| Decision Recorded | Assign Penalty | Penalty Assigned |
| Penalty Assigned | Open Appeal Window | Appeal Window Open |
| Appeal Window Open | Register Appeal | Appeal Registered |
| Appeal Registered | Complete Appeal Review | Appeal Reviewed |
| Appeal Window Open | Close Case | Case Closed |
| Appeal Reviewed | Close Case | Case Closed |
| Case Closed | Archive Case | Archived |

---

## 21.5 Invalid State Transitions

The following transitions are prohibited:

- Investigation before case creation.
- Committee hearing before investigation completion.
- Decision before hearing.
- Penalty before approved decision.
- Appeal after archival.
- Archive before case closure.
- Modification of archived cases.
- Reopening a closed case without authorized review.
- Deletion of malpractice cases.

Attempts to perform invalid transitions shall fail validation and generate audit records.

---

## 21.6 Terminal States

The aggregate contains the following terminal state:

| State | Description |
|---------|-------------|
| Archived | Final immutable state of the malpractice case. |

Archived aggregates become read-only and may only be accessed by authorized users.

---

# 22. Dependency Matrix

## 22.1 Upstream Dependencies

| Domain | Dependency | Type |
|---------|------------|------|
| L2-A01 Platform Foundation | Platform services | Mandatory |
| L2-A02 Person & Identity | Candidate identity | Mandatory |
| L2-A03 Authentication | User authentication | Mandatory |
| L2-A04 Authorization | Permission validation | Mandatory |
| L2-A05 Workflow Infrastructure | Workflow execution | Mandatory |
| L2-A06 Event Infrastructure | Event publishing | Mandatory |
| L2-A07 Audit Infrastructure | Immutable audit storage | Mandatory |
| L2-B01 Institution | Institutional policies | Mandatory |
| L2-B02 Organisation | Organizational hierarchy | Mandatory |
| L2-B03 Campus & Infrastructure | Examination locations | Optional |
| L2-B05 Academic Calendar | Examination session context | Mandatory |
| L2-I06 Examination Conduct | Incident generation | Mandatory |
| L2-I08 Evaluation | Evaluation irregularities | Optional |

---

## 22.2 Downstream Dependencies

| Domain | Dependency | Purpose |
|---------|------------|---------|
| L2-I09 Result Processing | Penalty impact | Result modification decisions |
| L2-I10 Result Publication | Publication restrictions | Withhold or publish results |
| L2-I12 Supplementary Examination | Eligibility | Determine supplementary eligibility |
| L2-I13 Academic Completion | qualification eligibility | Restrict academic completion where applicable |
| Student Lifecycle | Disciplinary history | Student profile updates |
| Notification Services | Stakeholder communication | Notifications |
| Reporting & Analytics | Institutional reporting | KPIs and dashboards |

---

## 22.3 Dependency Principles

The Malpractice & Examination Committee domain shall adhere to the following dependency principles:

- Dependencies shall be explicit.
- Circular dependencies are prohibited.
- Domain ownership shall not be violated.
- Integration shall occur through canonical contracts.
- Multi-domain communication shall be event-driven.
- Downstream domains shall not modify malpractice aggregate state.
- Published events shall remain backward compatible.

---

# 23. Appendices

## Appendix A — Malpractice Categories

Typical institutional malpractice categories include:

- Copying from another candidate.
- Possession of unauthorized study material.
- Mobile phone usage.
- Electronic device usage.
- Candidate impersonation.
- Identity fraud.
- Answer script substitution.
- Communication during examination.
- Collusion.
- External assistance.
- Leakage of question papers.
- Digital examination misconduct.
- Invigilator misconduct.
- Evaluation malpractice.
- Other institution-defined disciplinary violations.

---

## Appendix B — Typical Penalties

Examples of institutional penalties include:

- Warning.
- Marks deduction.
- Subject cancellation.
- Examination cancellation.
- Semester cancellation.
- Result withholding.
- Fine.
- Temporary suspension.
- Debarment for one or more examination sessions.
- Permanent debarment.
- Referral to institutional disciplinary committee.

Institutions may define additional penalties according to their governing regulations.

---

## Appendix C — Design Principles

The architecture follows the enterprise principles established throughout the SutraERP repository:

- Domain-Driven Design (DDD).
- Single Source of Truth.
- Event-Driven Architecture.
- CQRS compatibility.
- Strong aggregate consistency.
- Explicit ownership boundaries.
- Immutable audit history.
- Policy-driven governance.
- Security by design.
- Regulatory compliance by design.

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
**L2-I14 — Malpractice & Examination Committee Workflows**
**Status:** Draft
**Next Document:** **L2-I15 — Examination Exception & Contingency Workflows**



