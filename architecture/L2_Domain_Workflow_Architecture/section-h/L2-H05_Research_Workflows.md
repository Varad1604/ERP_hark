# L2-H05_Research_Workflows

**Document ID:** L2-H05  
**Domain:** Section H — Institutional Services & Intelligence  
**Module:** Research Management  
**Document Type:** Level 2 Enterprise Workflow Specification  
**Architecture Style:** Domain-Driven Design (DDD), CQRS, Event-Driven Architecture (EDA)  
**Version:** 1.0  
**Status:** Draft – Architecture Baseline  
**Owner:** Enterprise Architecture Team  

---

# 1. Purpose

This document defines the complete enterprise workflow architecture for the Research Management domain within SutraERP.

The objective of this module is to provide a centralized, configurable, auditable, and policy-driven platform for managing institutional research throughout its complete lifecycle. The module enables educational institutions to manage research proposals, funded and non-funded research projects, grants, ethics approvals, publications, patents, intellectual property, consultancy engagements, collaborations, and institutional research analytics while maintaining complete traceability, governance, and regulatory compliance.

This document establishes:

- Functional boundaries of the Research domain
- Business capabilities
- Aggregate ownership
- Workflow orchestration
- Commands
- Events
- Read models
- Integration contracts
- Security responsibilities
- Audit requirements
- Extension points

This specification serves as the authoritative architectural reference for implementation teams, integration architects, QA teams, and future Level 3 workflow decomposition.

---

# 2. Objectives

The Research Management module is designed to achieve the following enterprise objectives:

- Digitize the complete institutional research lifecycle.
- Standardize research governance across the institution.
- Support configurable institutional research policies.
- Enable structured research proposal evaluation and approval.
- Manage funded and non-funded research projects.
- Support government, industry, consultancy, CSR, international, and internal research funding.
- Enable configurable ethics approval workflows.
- Track publications, patents, copyrights, software assets, datasets, and technical reports.
- Maintain complete research portfolio history for faculty and researchers.
- Support multidisciplinary and multi-investigator research projects.
- Enable secure collaboration with external organizations and researchers.
- Provide complete auditability of research activities and decisions.
- Support institution-specific approval workflows.
- Generate research KPIs and institutional analytics.
- Enable event-driven integration across enterprise domains.
- Ensure scalability for institutions managing thousands of research activities simultaneously.

---

# 3. Scope

The Research Management domain owns workflows related to institutional research execution and governance from proposal initiation through project completion, research output management, and institutional research reporting.

The domain includes:

- Research Proposal Management
- Proposal Review and Approval
- Research Project Management
- Grant Management
- Funding Management
- Ethics Approval Management
- Research Collaboration Management
- Consultancy Project Management
- Publication Management
- Patent Management
- Intellectual Property Registration
- Research Repository Metadata Management
- Research Portfolio Management
- Principal Investigator Management
- Co-Investigator Management
- Research Team Management
- Research Milestone Tracking
- Research KPI Management
- Research Reporting
- Research Notifications
- Research Audit History

---

# 4. Out of Scope

The Research domain intentionally excludes ownership of the following business capabilities.

These capabilities remain owned by their respective enterprise domains.

| Capability | Owning Section |
|------------|----------------|
| Faculty Master | L2-E03 Faculty Workforce |
| Employee Master | Section E |
| Faculty Workload & Capacity | L2-E03 Faculty Workforce |
| Academic Assignments | Section D |
| Student Master | Section C |
| Student Academic Lifecycle | Section C |
| Authentication | Section A |
| Authorization | Section A |
| Workflow Engine | Section A |
| Event Infrastructure | Section A |
| Audit Infrastructure | Section A |
| Enterprise Notifications | Section H02 |
| Enterprise Directory & Search | Section H03 |
| Enterprise Dashboards | Section H07 |
| Financial Accounting | Section F |
| Budget & Payments | Section F |
| Enterprise Document Management | Platform Shared Service |

The Research Management module consumes information from these domains through enterprise APIs, read models, and published events without assuming ownership of their business entities.

Research Management SHALL consume Faculty Research Capability from L2-E03 Faculty Workforce.
Research Management SHALL consume Faculty Availability from L2-E03 Faculty Workforce.
Research Management SHALL consume Faculty Capacity information from L2-E03 Faculty Workforce.
Research Management SHALL consume Faculty Eligibility decisions from L2-E03 Faculty Workforce.
Research Management SHALL NOT determine researcher eligibility independently.
Research Management SHALL NOT own Faculty Capability, Faculty Status, Faculty Profile, or Faculty Workforce information.

Research Management owns research metadata and workflow state only. It does not own faculty profile, faculty status, faculty records, faculty capability, faculty availability, faculty capacity, faculty eligibility, organizational assignment, financial ledgers, academic assignments, or document storage.

---

# 5. Architectural Position

Within the SutraERP enterprise architecture, Research Management is classified as an **Institutional Service Domain**.

Unlike academic and workforce domains that own canonical institutional records, Research Management orchestrates institutional research processes by consuming authoritative information published by multiple enterprise domains.

Research Management is responsible for governing research execution rather than maintaining the institutional master records upon which research activities depend.

The domain owns:

- Research lifecycle execution
- Proposal governance
- Research project governance
- Grant lifecycle
- Ethics approval lifecycle
- Publication lifecycle
- Patent lifecycle
- Collaboration lifecycle
- Consultancy research lifecycle
- Research portfolio metadata
- Institutional research analytics

The domain consumes:

- Faculty identity from L2-E03 Faculty Workforce
- Faculty Research Capability from L2-E03 Faculty Workforce
- Faculty Availability from L2-E03 Faculty Workforce
- Faculty Capacity information from L2-E03 Faculty Workforce
- Faculty workload signals from L2-E03 Faculty Workforce for downstream research participation tracking
- Academic assignment information from Section D
- Student information from Section C
- Financial transactions from Section F
- Notifications from H02
- Enterprise search indexing from H03
- Dashboard presentation through H07
- Authentication, authorization, workflow, event, and audit services from Section A

Research Management never assumes ownership of upstream master data and does not duplicate canonical records maintained by other enterprise domains.

---

# 6. Enterprise Design Principles

The Research Management domain adheres to the following enterprise architectural principles.

## 6.1 Single Source of Truth

Each business entity has exactly one authoritative owning domain.

Research Management owns research business processes but consumes faculty, student, financial, and organizational information from their authoritative domains.

Faculty workload planning remains exclusively owned by L2-E03 Faculty Workforce.
Research Management SHALL publish research participation and workload consumption events so L2-E03 Faculty Workforce can update workload projections asynchronously.
Research Management SHALL consume faculty capacity projections from L2-E03 Faculty Workforce.

---

## 6.2 Domain Ownership

Research Management owns only research-related aggregates and business rules.

Cross-domain updates occur exclusively through published events and enterprise service contracts.

---

## 6.3 Event-Driven Architecture

Every successful business transaction producing a significant business fact publishes immutable domain events.

Downstream domains react to these events asynchronously without introducing direct dependencies.

---

## 6.4 CQRS

Commands modify research aggregates.

Queries consume optimized read models.

Operational workloads remain isolated from analytical workloads.

---

## 6.5 Audit by Design

Every research decision, approval, rejection, funding activity, publication, patent, collaboration, and project milestone shall generate immutable audit evidence.

---

## 6.6 Institution Configurability

Approval workflows, funding rules, ethics policies, research categories, publication classifications, patent policies, and reporting requirements shall be configurable without requiring source code modifications.

---

## 6.7 Loose Coupling

Research Management communicates with external domains exclusively through published events, APIs, and read models.

No direct database dependencies are permitted.

---

## 6.8 Extensibility

The architecture shall support future integration with research repositories, funding agencies, citation platforms, AI-assisted proposal evaluation, technology transfer systems, and additional institutional research services without requiring redesign of the core domain.

# 7. Domain Boundaries

Research Management is an execution and governance domain responsible for managing the complete institutional research lifecycle. It governs research-related business processes while consuming authoritative information from enterprise domains that own institutional master data.

The domain owns research execution, research governance, research outputs, and institutional research portfolio management. It does not own faculty identity, faculty capability, faculty availability, faculty capacity, student identity, financial accounting, academic assignments, or enterprise document storage.

The architectural objective is to maintain strict Domain-Driven Design (DDD) boundaries where each business capability has exactly one authoritative owner.

---

## 7.1 Domain Responsibilities

Research Management owns the following enterprise capabilities.

| Business Capability | Ownership |
|---------------------|-----------|
| Research Proposal Management | Research Domain |
| Proposal Review | Research Domain |
| Proposal Approval Workflow | Research Domain |
| Research Project Lifecycle | Research Domain |
| Research Grant Management | Research Domain |
| Research Funding Tracking | Research Domain |
| Ethics Approval Lifecycle | Research Domain |
| Publication Management | Research Domain |
| Patent Management | Research Domain |
| Intellectual Property Metadata | Research Domain |
| Research Collaboration | Research Domain |
| Consultancy Research Projects | Research Domain |
| Research Portfolio | Research Domain |
| Research KPI Generation | Research Domain |
| Institutional Research Reporting | Research Domain |
| Research Workflow Audit | Research Domain |

---

## 7.2 Business Capabilities Consumed

The Research domain consumes business capabilities from other enterprise domains without assuming ownership.

| Capability | Source Domain | Usage |
|------------|---------------|------|
| Faculty Master | L2-E03 Faculty Workforce | Principal Investigator and Research Team |
| Faculty Research Capability | L2-E03 Faculty Workforce | Research participation reference |
| Faculty Availability | L2-E03 Faculty Workforce | Research scheduling reference |
| Faculty Capacity | L2-E03 Faculty Workforce | Research participation reference |
| Faculty Position | L2-E03 Faculty Workforce | Eligibility Validation |
| Faculty Workload | L2-E03 Faculty Workforce | Capacity Verification |
| Faculty Assignment | Section D | Academic Context |
| Student Master | Section C | Student Researchers |
| Authentication | Section A | Identity Verification |
| Authorization | Section A | Access Control |
| Workflow Engine | Section A | Workflow Orchestration |
| Event Infrastructure | Section A | Domain Events |
| Audit Infrastructure | Section A | Audit Evidence |
| Finance | Section F | Budget & Grant Financials |
| Notification | H02 | Alerts |
| Directory | H03 | Enterprise Search |
| Dashboard | H07 | Analytics Presentation |

---

## 7.3 Domain Ownership Rules

The following ownership rules are mandatory throughout the enterprise.

| Rule | Description |
|------|-------------|
| Faculty records remain owned by L2-E03 Faculty Workforce. |
| Faculty capability, availability, capacity, eligibility, and workload planning remain owned by L2-E03 Faculty Workforce. |
| Student records remain owned by Section C. |
| Academic assignments remain owned by Section D. |
| Finance remains the authoritative owner of financial accounting. |
| Finance remains the authoritative owner of budgets, accounting, ledgers, payment processing, procurement, taxation, and financial compliance. |
| Enterprise documents remain owned by the Enterprise Document Management Platform. |
| Research owns only research metadata and workflow state. |
| Research never duplicates authoritative master data. |
| Cross-domain updates occur only through APIs or Domain Events. |

---

## 7.4 Faculty Workload Ownership

Faculty Workload planning remains exclusively owned by L2-E03 Faculty Workforce.

Research Management SHALL consume workload information, faculty capacity, faculty availability, and faculty eligibility as authoritative inputs for research participation validation.
Research Management SHALL NOT assign teaching workload.
Research Management SHALL NOT plan faculty workload.
Research Management SHALL NOT calculate faculty capacity.
Research Management SHALL NOT manage faculty availability.
Research Management SHALL NOT override faculty eligibility.
Research Management SHALL only consume workload decisions from L2-E03 Faculty Workforce.

---

## 7.5 Research Financial Ownership Boundary

Research Management SHALL consume financial references and funding status from Finance, while Finance remains the authoritative owner of accounting and posting decisions.

| Research OWNS | Description |
|--------------|-------------|
| Grant lifecycle | Research grant application, award, allocation reference, utilization tracking, and closure state |
| Funding metadata | Research-specific funding metadata linked to grants and projects |
| Funding approvals | Research-side approval state for funding requests |
| Grant utilization tracking | Research tracking of how approved funding is used |
| Project financial status | Research project-level financial status references |
| Budget references | Research references to approved budgets and funding envelopes |
| Funding source metadata | Research metadata describing sponsor, scheme, and source context |
| Research expenditure metadata | Research metadata describing expenditure categories and references |
| Financial reporting metadata | Research reporting metadata for research finance visibility |

| Finance (Section F) OWNS | Description |
|--------------------------|-------------|
| Ledger | Canonical accounting ledger |
| Journal Entries | Accounting journal entries |
| Accounts | Financial accounts and chart of accounts |
| Payments | Payment execution and settlement |
| Vendor Payments | Vendor payment processing |
| Procurement | Procurement workflows and controls |
| Budget Accounting | Budget accounting and accounting controls |
| Cost Centers | Cost center ownership and allocation |
| Taxation | Tax calculation and compliance |
| Financial Posting | Financial posting and accounting entries |
| Revenue Recognition | Revenue recognition accounting |
| General Accounting | General ledger and general accounting rules |

Research Management SHALL NEVER:

- create accounting entries
- execute payments
- calculate taxes
- maintain ledgers
- manage procurement
- approve financial postings

Research Management SHALL communicate with Finance ONLY through published events and service contracts.

---

# 8. Bounded Context Definition

Research Management represents a single bounded context responsible for institutional research governance and execution.

The context maintains complete ownership of research aggregates while referencing upstream entities using immutable identifiers rather than duplicating business data.

The Research bounded context guarantees that all research workflows remain independent from the implementation details of Faculty, Student, Finance, Academic Delivery, and Platform Foundation domains.
The Research bounded context SHALL consume faculty research capability, faculty availability, faculty capacity, faculty eligibility, faculty status, and faculty profile information from L2-E03 Faculty Workforce.

---

## 8.1 Core Responsibilities

The bounded context is responsible for:

- Managing research proposals.
- Governing proposal review and approvals.
- Managing research projects.
- Managing grants and funding.
- Managing ethics approvals.
- Managing publications.
- Managing patents.
- Managing collaborations.
- Managing consultancy research.
- Managing institutional research portfolios.
- Publishing research domain events.
- Providing optimized read models for reporting.

Research Management records research participation and publishes events that L2-E03 Faculty Workforce consumes to update workload projections, availability, and capacity.

---

## 8.2 External Responsibilities

The following capabilities remain external to the Research bounded context.

| External Capability | Owning Domain |
|---------------------|---------------|
| Faculty Lifecycle | L2-E03 Faculty Workforce |
| Student Lifecycle | Section C |
| Academic Structure | Section D |
| Payroll | L2-E03 Faculty Workforce |
| Financial Ledger | Section F |
| Authentication | Section A |
| Authorization | Section A |
| Workflow Engine | Section A |
| Notifications | H02 |
| Enterprise Search | H03 |
| Enterprise Dashboards | H07 |

---

## 8.3 Integration Philosophy

Research Management communicates with external domains using the following mechanisms.

- Published Domain Events
- Enterprise APIs
- CQRS Read Models
- Event Subscriptions
- Workflow Engine
- Enterprise Authorization Services

Direct database access across domains is strictly prohibited.

---

# 9. Business Capabilities

The Research Management domain provides the following enterprise business capabilities.

| Capability ID | Business Capability | Description |
|--------------|---------------------|-------------|
| RM-01 | Research Proposal Management | Manage proposal lifecycle from creation to approval |
| RM-02 | Research Project Management | Execute and monitor research projects |
| RM-03 | Grant Management | Manage institutional research grants |
| RM-04 | Funding Management | Track research funding utilization |
| RM-05 | Ethics Management | Govern ethics approval lifecycle |
| RM-06 | Publication Management | Manage journals, conferences, books, and reports |
| RM-07 | Patent Management | Manage patent filing and lifecycle |
| RM-08 | Intellectual Property Management | Maintain institutional IP metadata |
| RM-09 | Collaboration Management | Manage research collaborations |
| RM-10 | Consultancy Management | Govern consultancy research engagements |
| RM-11 | Research Repository Registration | Register research outputs for institutional repositories |
| RM-12 | Research Portfolio Management | Maintain faculty and institutional portfolios |
| RM-13 | Research KPI Management | Calculate institutional research KPIs |
| RM-14 | Research Reporting | Generate operational and analytical reports |
| RM-15 | Research Notifications | Generate workflow notifications |
| RM-16 | Research Audit | Maintain immutable audit history |

---

# 10. Ownership Matrix

The following matrix defines ownership boundaries across enterprise domains.

| Business Object | Owner | Research Access |
|----------------|-------|-----------------|
| Faculty | L2-E03 Faculty Workforce | Read Only |
| Faculty Workload | L2-E03 Faculty Workforce | Read Only |
| Student | Section C | Read Only |
| Academic Assignment | Section D | Read Only |
| Research Proposal | Research | Full Ownership |
| Research Project | Research | Full Ownership |
| Grant | Research | Full Ownership |
| Funding Allocation Metadata | Research | Full Ownership |
| Financial Ledger | Section F | Read Only |
| Ethics Approval | Research | Full Ownership |
| Publication | Research | Full Ownership |
| Patent | Research | Full Ownership |
| Intellectual Property Metadata | Research | Full Ownership |
| Collaboration | Research | Full Ownership |
| Consultancy Project | Research | Full Ownership |
| Research Portfolio | Research | Full Ownership |
| Research KPI | Research | Full Ownership |
| Notification | H02 | Consumed |
| Search Index | H03 | Published |
| Dashboard | H07 | Published Read Models |
| Audit Record | Section A | Consumed Service |
| Workflow Instance | Section A | Consumed Service |

---

## Architecture Decision

Research Management is an orchestration and governance domain rather than a master data domain.

Its responsibility is to execute, monitor, govern, and report institutional research while relying on authoritative enterprise domains for identity, workforce, academic structure, finance, security, workflow execution, and enterprise infrastructure.

# 11. Domain Model

The Research Management domain is composed of a collection of business aggregates that collectively manage the complete institutional research lifecycle. Each aggregate owns its own state, business rules, lifecycle transitions, and domain events.

Aggregates communicate through published domain events and never modify each other's state directly.

Each aggregate is transactionally consistent within its own boundary.

---

# 12. Aggregate Catalogue

The following aggregates constitute the Research Management bounded context.

| Aggregate ID | Aggregate | Purpose |
|--------------|-----------|---------|
| RA-01 | Research Proposal | Manages proposal lifecycle from creation through approval or rejection |
| RA-02 | Research Project | Manages approved research projects and execution lifecycle |
| RA-03 | Research Grant | Manages grant applications, awards, funding, and utilization |
| RA-04 | Ethics Approval | Manages ethics committee review and approvals |
| RA-05 | Publication | Manages publications and publication lifecycle |
| RA-06 | Patent | Manages patents, copyrights, software registrations, and IP filings |
| RA-07 | Collaboration | Manages institutional and external research collaborations |
| RA-08 | Consultancy Project | Manages consultancy-based research engagements |
| RA-09 | Research Portfolio | Maintains researcher and institutional research portfolio |
| RA-10 | Research Repository Registration | Manages repository metadata and external repository registrations |

---

## 12.1 Aggregate Ownership Principles

Every aggregate follows the enterprise ownership model.

- Each aggregate owns its own lifecycle.
- Aggregates are modified only through commands.
- Successful state changes publish immutable domain events.
- Aggregates reference one another using identifiers.
- Cross-aggregate consistency is maintained through eventual consistency.
- Business rules remain encapsulated inside aggregate boundaries.

---

# 13. Aggregate Relationships

The aggregates collaborate throughout the research lifecycle while maintaining transactional independence.

```text
                 Research Proposal
                         │
          Proposal Approved Event
                         │
                         ▼
                 Research Project
          ┌──────────┼──────────┐
          │          │          │
          ▼          ▼          ▼
   Research Grant  Ethics   Collaboration
          │                     │
          ▼                     ▼
   Funding Updates      Consultancy Project
          │                     │
          └──────────┬──────────┘
                     ▼
             Research Portfolio
                     │
          ┌──────────┴──────────┐
          ▼                     ▼
      Publication          Patent/IP
          │                     │
          └──────────┬──────────┘
                     ▼
      Repository Registration
```

---

## 13.1 Aggregate Dependency Rules

| Parent Aggregate | Child Aggregate | Dependency Type |
|------------------|-----------------|-----------------|
| Research Proposal | Research Project | Proposal approval creates project |
| Research Project | Research Grant | Project may receive funding |
| Research Project | Ethics Approval | Ethics approval may be mandatory |
| Research Project | Collaboration | Projects may include collaborators |
| Research Project | Consultancy Project | Consultancy may execute under project |
| Research Project | Publication | Publications originate from projects |
| Research Project | Patent | Patents originate from projects |
| Publication | Research Portfolio | Portfolio updated through events |
| Patent | Research Portfolio | Portfolio updated through events |
| Repository Registration | Publication | Repository indexes publication metadata |
| Repository Registration | Patent | Repository indexes IP metadata |

---

## 13.2 Aggregate Reference Rules

The following reference rules apply throughout the domain.

| Reference | Rule |
|-----------|------|
| Faculty | Referenced by Faculty ID only |
| Student | Referenced by Student ID only |
| Department | Referenced by Department ID only |
| Institution | Referenced by Institution ID only |
| Grant Agency | Referenced by Agency ID |
| External Organization | Referenced by Organization ID |
| Repository | Referenced by Repository ID |

Research aggregates shall never embed authoritative copies of external business entities.

---

# 14. Entity Definitions

Entities exist within aggregate boundaries and possess independent identity.

---

## 14.1 Research Proposal Aggregate

### Purpose

Represents an institutional research proposal before approval.

### Primary Entity

Research Proposal

### Child Entities

- Proposal Investigator
- Co-Investigator
- Funding Request
- Review Record
- Approval Decision
- Proposal Attachment Metadata

---

## 14.2 Research Project Aggregate

### Purpose

Represents an approved research initiative.

### Child Entities

- Project Team
- Milestone
- Deliverable
- Funding Allocation Metadata
- Progress Report
- Risk Register

---

## 14.3 Research Grant Aggregate

### Purpose

Represents funding associated with research projects.

### Child Entities

- Funding Source
- Grant Installment
- Budget Allocation
- Utilization Record
- Grant Condition

---

## 14.4 Ethics Approval Aggregate

### Purpose

Represents institutional ethics review activities.

### Child Entities

- Ethics Submission
- Committee Review
- Approval Condition
- Renewal Record
- Compliance Observation

---

## 14.5 Publication Aggregate

### Purpose

Represents research publications.

### Child Entities

- Author
- Publication Venue
- Citation Information
- DOI Metadata
- Publication Version

---

## 14.6 Patent Aggregate

### Purpose

Represents institutional intellectual property.

### Child Entities

- Inventor
- Patent Filing
- Patent Office
- Registration Record
- Legal Status

---

## 14.7 Collaboration Aggregate

### Purpose

Represents research collaborations.

### Child Entities

- Collaborating Organization
- Collaborator
- Agreement Metadata
- Collaboration Activity

---

## 14.8 Consultancy Project Aggregate

### Purpose

Represents consultancy engagements performed as research activities.

### Child Entities

- Client Organization
- Consultancy Agreement
- Deliverable
- Revenue Metadata

---

## 14.9 Research Portfolio Aggregate

### Purpose

Represents cumulative institutional research achievements.

### Child Entities

- Publication Summary
- Patent Summary
- Grant Summary
- Citation Metrics
- KPI Snapshot

---

## 14.10 Repository Registration Aggregate

### Purpose

Maintains metadata registered with institutional and external repositories.

### Child Entities

- Repository Metadata
- Repository Identifier
- Registration Status
- Synchronization History

---

# 15. Value Objects

The Research domain uses immutable value objects shared across aggregates.

| Value Object | Purpose |
|--------------|---------|
| Research Identifier | Unique research identifier |
| Proposal Number | Proposal reference |
| Project Number | Project identifier |
| Grant Number | Grant reference |
| Publication Identifier | Publication identifier |
| DOI | Digital Object Identifier |
| Patent Number | Patent reference |
| ORCID | Researcher identifier |
| Funding Amount | Approved funding value |
| Project Duration | Planned execution period |
| Ethics Category | Ethics classification |
| Research Category | Institutional research classification |
| Collaboration Type | Collaboration classification |
| Consultancy Type | Consultancy classification |
| Approval Decision | Approval outcome |
| Review Score | Proposal evaluation result |
| KPI Value | Research performance metric |

Value objects are immutable and may be reused across multiple aggregates.

---

# 16. Lifecycle States

Each aggregate owns its own lifecycle.

| Aggregate | Initial State | Terminal State |
|-----------|---------------|----------------|
| Research Proposal | Draft | Approved / Rejected / Withdrawn |
| Research Project | Initiated | Closed |
| Research Grant | Proposed | Closed |
| Ethics Approval | Submitted | Expired / Closed |
| Publication | Draft | Published / Archived |
| Patent | Draft | Granted / Expired / Closed |
| Collaboration | Proposed | Completed / Terminated |
| Consultancy Project | Proposed | Completed |
| Research Portfolio | Active | Archived |
| Repository Registration | Pending | Registered / Failed |

State transitions shall occur exclusively through validated commands and successful transitions shall publish corresponding immutable domain events.

# 17. Workflow Catalogue

The Research Management domain orchestrates the complete institutional research lifecycle through a collection of governed business workflows.

Each workflow owns a specific business objective, operates on one or more aggregates, and communicates with other enterprise domains exclusively through commands, domain events, enterprise APIs, and CQRS read models.

The workflow catalogue serves as the authoritative inventory for all Level-2 Research workflows.

---

# 18. Workflow Classification

Research workflows are organized into logical capability groups based on business responsibility.

| Workflow Category | Description |
|-------------------|-------------|
| Proposal Management | Research proposal preparation, review, approval, and closure |
| Project Management | Research project execution and governance |
| Funding Management | Grants, funding allocation, and utilization |
| Ethics Management | Ethics committee approval and compliance |
| Publication Management | Research publication lifecycle |
| Intellectual Property Management | Patents, copyrights, software, and IP lifecycle |
| Collaboration Management | Institutional and external research collaborations |
| Consultancy Management | Consultancy-based research projects |
| Portfolio Management | Research portfolio and KPI management |
| Repository Management | Institutional and external repository registration |

---

# 19. Complete Workflow List

## 19.1 Proposal Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-01 | Create Research Proposal |
| RM-02 | Submit Research Proposal |
| RM-03 | Review Research Proposal |
| RM-04 | Approve Research Proposal |
| RM-05 | Request Proposal Revision |
| RM-06 | Reject Research Proposal |
| RM-07 | Withdraw Research Proposal |

---

## 19.2 Project Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-08 | Initiate Research Project |
| RM-09 | Assign Research Team |
| RM-10 | Manage Research Milestones |
| RM-11 | Submit Progress Report |
| RM-12 | Update Project Status |
| RM-13 | Complete Research Project |
| RM-14 | Close Research Project |

---

## 19.3 Grant & Funding Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-15 | Apply for Research Grant |
| RM-16 | Record Grant Award |
| RM-17 | Allocate Research Funding |
| RM-18 | Record Funding Utilization |
| RM-19 | Close Research Grant |

---

## 19.4 Ethics Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-20 | Submit Ethics Application |
| RM-21 | Review Ethics Application |
| RM-22 | Approve Ethics Request |
| RM-23 | Reject Ethics Request |
| RM-24 | Renew Ethics Approval |

---

## 19.5 Publication Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-25 | Register Publication |
| RM-26 | Verify Publication |
| RM-27 | Publish Research Output |
| RM-28 | Archive Publication |

---

## 19.6 Intellectual Property Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-29 | Register Patent |
| RM-30 | Submit Patent Filing |
| RM-31 | Update Patent Status |
| RM-32 | Close Patent Lifecycle |

---

## 19.7 Collaboration Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-33 | Create Collaboration |
| RM-34 | Approve Collaboration |
| RM-35 | Manage Collaboration Activities |
| RM-36 | Complete Collaboration |

---

## 19.8 Consultancy Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-37 | Register Consultancy Project |
| RM-38 | Execute Consultancy Project |
| RM-39 | Complete Consultancy Project |

---

## 19.9 Repository & Portfolio Management

| Workflow ID | Workflow |
|-------------|----------|
| RM-40 | Register Repository Metadata |
| RM-41 | Synchronize Repository |
| RM-42 | Update Research Portfolio |
| RM-43 | Generate Research KPIs |
| RM-44 | Generate Research Reports |

---

# 20. Workflow Dependency Matrix

The following matrix defines workflow dependencies within the Research Management domain.

| Parent Workflow | Dependent Workflow | Dependency |
|-----------------|-------------------|------------|
| Create Proposal | Submit Proposal | Proposal must exist |
| Submit Proposal | Review Proposal | Proposal submitted |
| Review Proposal | Approve Proposal | Review completed |
| Approve Proposal | Initiate Project | Approved proposal required |
| Initiate Project | Assign Research Team | Project created |
| Assign Research Team | Manage Milestones | Team established |
| Initiate Project | Apply Grant | Eligible project required |
| Initiate Project | Submit Ethics Application | Ethics-required projects only |
| Ethics Approval | Execute Research Activities | Mandatory for regulated research |
| Project Execution | Register Publication | Research output generated |
| Project Execution | Register Patent | Patentable output generated |
| Publication | Update Portfolio | Publication accepted |
| Patent | Update Portfolio | Patent filed or granted |
| Portfolio Update | Generate KPIs | Portfolio refreshed |
| Repository Registration | Synchronize Repository | Registration successful |

---

## 20.1 Cross-Domain Dependencies

Research workflows depend upon enterprise services provided by other bounded contexts.

| External Domain | Dependency |
|-----------------|------------|
| Section A | Authentication, Authorization, Workflow Engine, Event Infrastructure, Audit |
| Section C | Student Research Participants |
| Section D | Academic Assignment Context |
| L2-E03 Faculty Workforce | Faculty, Researchers, Workforce Information |
| Section F | Grant Accounting and Financial Transactions |
| H02 | Notifications |
| H03 | Enterprise Search |
| H07 | Dashboard Read Models |

Research Management consumes these services without assuming ownership of their business entities.

---

# 21. Workflow Interaction Diagram

The following diagram illustrates the end-to-end orchestration of the Research lifecycle.

```text
Create Proposal
        │
        ▼
Submit Proposal
        │
        ▼
Review Proposal
        │
        ▼
Approve Proposal
        │
        ▼
Create Research Project
        │
 ┌──────┼─────────────┐
 ▼      ▼             ▼
Grant  Ethics   Research Team
 │       │             │
 └───────┴─────────────┘
             │
             ▼
      Project Execution
             │
   ┌─────────┼──────────┐
   ▼         ▼          ▼
Publication Patent Collaboration
   │         │          │
   └─────────┼──────────┘
             ▼
      Research Portfolio
             │
             ▼
      KPI Generation
             │
             ▼
 Research Reports & Dashboards
```

---

# 22. Workflow Orchestration Principles

All workflows within the Research Management domain shall adhere to the following enterprise orchestration principles.

| Principle | Description |
|------------|-------------|
| Command Driven | Every state transition is initiated through a validated command. |
| Event Driven | Every successful business transaction publishes immutable domain events. |
| Aggregate Isolation | Workflows never directly modify another aggregate's state. |
| Eventual Consistency | Cross-aggregate consistency is achieved asynchronously through domain events. |
| Policy Driven | Institution-specific approval policies are configurable without source code changes. |
| Idempotent Processing | Commands and event handlers shall safely handle duplicate execution. |
| Audit by Default | Every workflow transition generates immutable audit evidence. |
| Authorization First | Authorization is validated before any business operation is executed. |
| Transaction Boundary | Each aggregate maintains its own transactional consistency. |
| Loose Coupling | Interactions with external domains occur only through enterprise APIs, read models, or published events. |

---

# 23. Workflow Execution Rules

The following execution rules apply uniformly across all Research workflows.

1. Every workflow starts with a validated command.

2. Business validations execute before aggregate state changes.

3. Successful state transitions generate immutable domain events.

4. Domain events trigger downstream workflows asynchronously.

5. Workflow failures never leave aggregates in partially committed states.

6. Long-running workflows are orchestrated through the enterprise workflow engine.

7. Every workflow execution generates complete audit evidence.

8. Institution-specific approval routing is resolved dynamically from configuration.

9. All workflow identifiers remain immutable throughout execution.

10. Research Management never directly updates external domain data; it requests changes through published events or enterprise service contracts.

# 24. Workflow Specifications

This section defines the detailed workflow specifications for Proposal Management.

Proposal Management governs the lifecycle of a research proposal from initial drafting until approval, rejection, or withdrawal. It is the mandatory entry point for every institutional research project.

All proposal workflows operate on the **Research Proposal Aggregate**.

---

# 24.1 RM-01 — Create Research Proposal

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Create a new research proposal for institutional review. |
| **Business Objective** | Capture complete research proposal information before formal submission. |
| **Actors** | Faculty Member, Research Staff, PhD Scholar (through Supervisor) |
| **Trigger** | User initiates proposal creation. |
| **Preconditions** | User is authenticated and authorized to create proposals. |
| **Inputs** | Proposal title, abstract, research category, department, investigators, funding requirement, duration, keywords, attachments. |
| **Outputs** | Draft Research Proposal Aggregate. |
| **Business Rules** | Proposal number is generated automatically. Draft proposals remain editable until submission. |
| **Authorization Requirements** | CreateResearchProposal permission. |
| **Validation Rules** | Mandatory fields completed. Principal Investigator eligibility shall be derived from L2-E03 Faculty Workforce. Institution policy validations satisfied. |
| **Main Flow** | Validate request → Generate Proposal Number → Create Proposal Aggregate → Save Draft → Publish Event |
| **Alternative Flows** | Import proposal from institutional template. |
| **Failure Flows** | Reject invalid proposal or insufficient permissions. |
| **Rollback Rules** | Remove transient proposal if persistence fails. |
| **Post Conditions** | Proposal exists in Draft state. |
| **Published Domain Events** | ResearchProposalCreated |
| **Consumed Events** | FacultyVerified, DepartmentValidated |
| **Audit Requirements** | Capture creator, timestamp, originating department, proposal metadata. |
| **Security Considerations** | Proposal visible only to authorized participants while in Draft. |
| **Performance Considerations** | Proposal creation shall complete within acceptable transactional latency. |
| **Architecture Constraints** | Proposal owns research metadata only. Faculty information remains external. |

```text
Researcher
      │
      ▼
Create Proposal
      │
Validate
      │
Generate Proposal Number
      │
Persist Aggregate
      │
Publish ResearchProposalCreated
```

---

# 24.2 RM-02 — Submit Research Proposal

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Submit a completed proposal into the institutional approval workflow. |
| **Business Objective** | Initiate configurable institutional proposal review. |
| **Actors** | Principal Investigator |
| **Trigger** | Proposal submission request. |
| **Preconditions** | Proposal exists in Draft state. |
| **Inputs** | Proposal Identifier. |
| **Outputs** | Submitted proposal. |
| **Business Rules** | Submitted proposals become read-only except through revision workflow. |
| **Authorization Requirements** | SubmitResearchProposal permission. |
| **Validation Rules** | Mandatory information complete. Required documents attached. Eligibility satisfied. |
| **Main Flow** | Validate Proposal → Lock Draft → Submit → Determine Approval Workflow → Publish Event |
| **Alternative Flows** | Institution-specific routing. |
| **Failure Flows** | Reject incomplete proposal. |
| **Rollback Rules** | Restore Draft status if submission transaction fails. |
| **Post Conditions** | Proposal enters Submitted state. |
| **Published Domain Events** | ResearchProposalSubmitted |
| **Consumed Events** | None |
| **Audit Requirements** | Record submitter and submission timestamp. |
| **Security Considerations** | Editing disabled after submission. |
| **Performance Considerations** | Approval routing shall not block submission transaction. |
| **Architecture Constraints** | Approval routing configured externally. |

```text
Draft Proposal
       │
Validate
       │
Lock Proposal
       │
Resolve Approval Policy
       │
Submitted
       │
Publish ResearchProposalSubmitted
```

---

# 24.3 RM-03 — Review Research Proposal

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Perform institutional review of a submitted proposal. |
| **Business Objective** | Evaluate proposal quality before approval decision. |
| **Actors** | Reviewer, Research Committee Member |
| **Trigger** | Proposal assigned for review. |
| **Preconditions** | Proposal submitted. |
| **Inputs** | Proposal, evaluation comments, review scores. |
| **Outputs** | Review record. |
| **Business Rules** | Multiple reviews may be configured by institution. |
| **Authorization Requirements** | ReviewResearchProposal permission. |
| **Validation Rules** | Reviewer assignment valid. Review criteria configured. |
| **Main Flow** | Load Proposal → Record Review → Store Evaluation → Publish Event |
| **Alternative Flows** | Multiple parallel reviewers. Sequential review. Blind review. |
| **Failure Flows** | Reject unauthorized review. |
| **Rollback Rules** | Remove incomplete review transaction. |
| **Post Conditions** | Proposal contains review evidence. |
| **Published Domain Events** | ResearchProposalReviewed |
| **Consumed Events** | ResearchProposalSubmitted |
| **Audit Requirements** | Preserve reviewer identity, scoring, comments, timestamps. |
| **Security Considerations** | Reviews visible only according to institutional policy. |
| **Performance Considerations** | Independent reviewer processing. |
| **Architecture Constraints** | Review does not modify proposal ownership. |

```text
Submitted Proposal
        │
Assign Reviewer
        │
Evaluate Proposal
        │
Record Review
        │
Publish ResearchProposalReviewed
```

---

# 24.4 RM-04 — Approve Research Proposal

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Approve a research proposal. |
| **Business Objective** | Authorize creation of an institutional research project. |
| **Actors** | Research Committee, Dean, Research Director, Institution-specific Approver |
| **Trigger** | Approval decision. |
| **Preconditions** | Review completed. |
| **Inputs** | Approval decision. |
| **Outputs** | Approved proposal. |
| **Business Rules** | Approval hierarchy is institution configurable. |
| **Authorization Requirements** | ApproveResearchProposal permission. |
| **Validation Rules** | Review completed. Required approvals satisfied. |
| **Main Flow** | Validate Approval → Update Proposal State → Publish Approval Event |
| **Alternative Flows** | Multi-level approval. Committee approval. External approval. |
| **Failure Flows** | Reject invalid approval request. |
| **Rollback Rules** | Restore previous proposal state if approval persistence fails. |
| **Post Conditions** | Proposal Approved. Project initiation becomes eligible. |
| **Published Domain Events** | ResearchProposalApproved |
| **Consumed Events** | ResearchProposalReviewed |
| **Audit Requirements** | Preserve approver, approval route, decision timestamp. |
| **Security Considerations** | Only authorized approvers may approve. |
| **Performance Considerations** | Approval routing supports asynchronous workflow. |
| **Architecture Constraints** | Approval does not automatically create project inside transaction. |

```text
Reviewed Proposal
        │
Validate Approval
        │
Approved
        │
Publish ResearchProposalApproved
        │
Downstream Project Creation
```

---

# 24.5 RM-05 — Request Proposal Revision

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Return proposal to investigators for modification. |
| **Business Objective** | Improve proposal quality without rejection. |
| **Actors** | Reviewer, Research Committee |
| **Trigger** | Revision requested during review. |
| **Preconditions** | Proposal under review. |
| **Inputs** | Revision comments. |
| **Outputs** | Proposal returned for revision. |
| **Business Rules** | Revision history shall remain immutable. |
| **Authorization Requirements** | RequestProposalRevision permission. |
| **Validation Rules** | Review completed. Revision reason mandatory. |
| **Main Flow** | Record Comments → Return Proposal → Publish Event |
| **Alternative Flows** | Multiple revision cycles. |
| **Failure Flows** | Reject invalid revision request. |
| **Rollback Rules** | Restore review state if update fails. |
| **Post Conditions** | Proposal returns to Draft Revision state. |
| **Published Domain Events** | ResearchProposalRevisionRequested |
| **Consumed Events** | ResearchProposalReviewed |
| **Audit Requirements** | Preserve reviewer comments and revision history. |
| **Security Considerations** | Revision comments visible only to proposal participants. |
| **Performance Considerations** | Lightweight workflow. |
| **Architecture Constraints** | Previous proposal versions remain immutable. |

---

# 24.6 RM-06 — Reject Research Proposal

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Reject a research proposal. |
| **Business Objective** | Formally terminate proposal evaluation. |
| **Actors** | Authorized Approver |
| **Trigger** | Rejection decision. |
| **Preconditions** | Review completed. |
| **Inputs** | Rejection reason. |
| **Outputs** | Rejected proposal. |
| **Business Rules** | Rejection reason mandatory. Proposal becomes immutable. |
| **Authorization Requirements** | RejectResearchProposal permission. |
| **Validation Rules** | Authorized decision maker. |
| **Main Flow** | Record Decision → Reject Proposal → Publish Event |
| **Alternative Flows** | None. |
| **Failure Flows** | Reject unauthorized action. |
| **Rollback Rules** | Restore previous review state if persistence fails. |
| **Post Conditions** | Proposal lifecycle completed as Rejected. |
| **Published Domain Events** | ResearchProposalRejected |
| **Consumed Events** | ResearchProposalReviewed |
| **Audit Requirements** | Preserve rejection rationale permanently. |
| **Security Considerations** | Decision immutable after completion. |
| **Performance Considerations** | Standard transactional operation. |
| **Architecture Constraints** | Rejected proposals cannot directly transition to Approved. A new proposal or governed revision workflow is required. |

# 24.7 RM-07 — Withdraw Research Proposal

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Withdraw a submitted research proposal before approval. |
| **Business Objective** | Allow investigators to terminate proposal evaluation voluntarily. |
| **Actors** | Principal Investigator |
| **Trigger** | Withdrawal request. |
| **Preconditions** | Proposal is in Submitted, Under Review, or Revision Requested state. |
| **Inputs** | Proposal Identifier, Withdrawal Reason |
| **Outputs** | Withdrawn Proposal |
| **Business Rules** | Approved proposals cannot be withdrawn. Withdrawal reason is mandatory. |
| **Authorization Requirements** | WithdrawResearchProposal permission |
| **Validation Rules** | Requestor must be the Principal Investigator or delegated authority. |
| **Main Flow** | Validate Request → Record Withdrawal Reason → Update Proposal State → Publish Event |
| **Alternative Flows** | Institutional administrator withdrawal under exceptional circumstances. |
| **Failure Flows** | Reject unauthorized withdrawal or withdrawal after approval. |
| **Rollback Rules** | Restore previous proposal state if transaction fails. |
| **Post Conditions** | Proposal lifecycle ends in Withdrawn state. |
| **Published Domain Events** | ResearchProposalWithdrawn |
| **Consumed Events** | ResearchProposalSubmitted, ResearchProposalReviewed |
| **Audit Requirements** | Preserve withdrawal reason, actor, timestamp, and previous workflow stage. |
| **Security Considerations** | Only authorized investigators may withdraw proposals. |
| **Performance Considerations** | Standard transactional workflow. |
| **Architecture Constraints** | Withdrawn proposals cannot be resubmitted. A new proposal must be created. |

```text
Principal Investigator
         │
         ▼
Withdraw Proposal
         │
Validate
         │
Record Reason
         │
Withdraw Proposal
         │
Publish ResearchProposalWithdrawn
```

---

# 25. Proposal Lifecycle State Machine

## 25.1 State Transition Matrix

| Current State | Command | Validation Required | Next State | Domain Event |
|---------------|---------|--------------------|------------|--------------|
| None | CreateResearchProposal | User Authorized | Draft | ResearchProposalCreated |
| Draft | SubmitResearchProposal | Proposal Complete | Submitted | ResearchProposalSubmitted |
| Submitted | AssignReviewer | Review Routing Complete | Under Review | ProposalReviewStarted |
| Under Review | RecordProposalReview | Reviewer Authorized | Under Review | ResearchProposalReviewed |
| Under Review | RequestProposalRevision | Revision Required | Revision Requested | ResearchProposalRevisionRequested |
| Revision Requested | UpdateProposal | Investigator Authorized | Draft | ResearchProposalUpdated |
| Under Review | ApproveResearchProposal | Approval Criteria Satisfied | Approved | ResearchProposalApproved |
| Under Review | RejectResearchProposal | Rejection Decision Recorded | Rejected | ResearchProposalRejected |
| Submitted | WithdrawResearchProposal | Investigator Authorized | Withdrawn | ResearchProposalWithdrawn |
| Under Review | WithdrawResearchProposal | Investigator Authorized | Withdrawn | ResearchProposalWithdrawn |

---

## 25.2 Invalid State Transitions

| Current State | Invalid Action | Reason |
|---------------|---------------|--------|
| Draft | Approve Proposal | Proposal not submitted |
| Draft | Reject Proposal | Review not initiated |
| Approved | Edit Proposal | Proposal immutable after approval |
| Approved | Withdraw Proposal | Approved proposals cannot be withdrawn |
| Rejected | Approve Proposal | Requires new proposal submission |
| Withdrawn | Submit Proposal | Lifecycle completed |
| Withdrawn | Edit Proposal | Proposal permanently closed |

---

## 25.3 Proposal Lifecycle Diagram

```text
              Draft
                │
                ▼
           Submitted
                │
                ▼
          Under Review
      ┌──────┼─────────┐
      ▼      ▼         ▼
Revision  Approved  Rejected
   │
   ▼
 Draft

Submitted ─────────► Withdrawn

Under Review ──────► Withdrawn
```

---

# 25.4 Proposal Business Rules

| Rule ID | Rule |
|----------|------|
| PR-01 | Proposal Number shall be unique within the institution. |
| PR-02 | Every proposal must have exactly one Principal Investigator. |
| PR-03 | Multiple Co-Investigators are permitted. |
| PR-04 | Approval workflow shall be institution configurable. |
| PR-05 | Ethics approval requirements depend upon research category. |
| PR-06 | Funding information may be optional for self-funded research. |
| PR-07 | Proposal attachments are stored in Enterprise Document Management; only metadata is maintained by Research Management. |
| PR-08 | Every approval decision shall generate immutable audit evidence. |
| PR-09 | Every successful state transition shall publish a domain event. |
| PR-10 | Proposal ownership cannot be transferred after submission without governed approval. |

---

# 25.5 Proposal Validation Rules

| Validation | Description |
|------------|-------------|
| Investigator Validation | Principal Investigator must exist in Faculty domain. |
| Department Validation | Department must be active. |
| Funding Validation | Funding category must exist in master configuration. |
| Ethics Validation | Ethics category required when applicable. |
| Research Category Validation | Category must be institution configured. |
| Duplicate Proposal Validation | Institution-defined duplicate detection policy shall execute before submission. |
| Attachment Validation | Mandatory documents must be available in Enterprise DMS. |
| Approval Policy Validation | Appropriate approval workflow must be configured before submission. |

---

# 25.6 Events Published by Proposal Aggregate

| Event | Trigger |
|--------|---------|
| ResearchProposalCreated | Proposal created |
| ResearchProposalUpdated | Proposal modified |
| ResearchProposalSubmitted | Proposal submitted |
| ProposalReviewStarted | Review initiated |
| ResearchProposalReviewed | Review completed |
| ResearchProposalRevisionRequested | Revision requested |
| ResearchProposalApproved | Proposal approved |
| ResearchProposalRejected | Proposal rejected |
| ResearchProposalWithdrawn | Proposal withdrawn |

---

# 25.7 Events Consumed by Proposal Aggregate

| Event | Source |
|--------|--------|
| FacultyVerified | L2-E03 Faculty Workforce |
| DepartmentValidated | Section B |
| ResearchCategoryConfigured | Master Data |
| ApprovalPolicyConfigured | Configuration Domain |
| FundingSchemeUpdated | Finance Domain |

---

# 25.8 Aggregate Consistency Rules

The Research Proposal aggregate shall enforce the following consistency guarantees.

- Proposal state transitions are atomic.
- Every successful command produces at most one state transition.
- Aggregate version shall increment after every successful update.
- Concurrent modifications shall use optimistic concurrency control.
- Cross-domain validations shall occur before aggregate persistence.
- Cross-aggregate updates shall occur asynchronously using published domain events.
- Proposal history shall remain immutable after lifecycle completion.
- Every proposal shall maintain complete audit traceability from creation through closure.

---
```

# 26. Workflow Specifications — Research Project Management

Research Project Management governs the lifecycle of approved research proposals after institutional approval.

Projects represent the execution phase of research and coordinate investigators, milestones, funding, ethics compliance, publications, patents, collaborations, consultancy activities, and project closure.

All workflows operate on the **Research Project Aggregate**.

---

# 26.1 RM-08 — Initiate Research Project

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Create a research project from an approved proposal. |
| **Business Objective** | Begin institutional execution of approved research. |
| **Actors** | Research Office, Research Administrator |
| **Trigger** | ResearchProposalApproved event. |
| **Preconditions** | Proposal approved. |
| **Inputs** | Approved Proposal Identifier |
| **Outputs** | Research Project Aggregate |
| **Business Rules** | Exactly one project shall be created from one approved proposal unless institution configuration explicitly permits multiple implementation projects. |
| **Authorization Requirements** | InitiateResearchProject permission |
| **Validation Rules** | Proposal approved. PI active. Approval completed. |
| **Main Flow** | Validate Approval → Generate Project Number → Create Aggregate → Publish Event |
| **Alternative Flows** | Multi-phase projects when institution configuration allows. |
| **Failure Flows** | Reject duplicate project creation. |
| **Rollback Rules** | Remove partially created aggregate upon transaction failure. |
| **Post Conditions** | Project enters Initiated state. |
| **Published Domain Events** | ResearchProjectCreated |
| **Consumed Events** | ResearchProposalApproved |
| **Audit Requirements** | Preserve originating proposal reference. |
| **Security Considerations** | Project accessible only to authorized research participants. |
| **Performance Considerations** | Event-driven asynchronous initiation supported. |
| **Architecture Constraints** | Project references proposal by identifier only. |

```text
Approved Proposal
        │
        ▼
Validate
        │
Generate Project Number
        │
Create Project
        │
Publish ResearchProjectCreated
```

---

# 26.2 RM-09 — Assign Research Team

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Assign investigators and project participants. |
| **Business Objective** | Establish official research execution team. |
| **Actors** | Principal Investigator, Research Administrator |
| **Trigger** | Team assignment request. |
| **Preconditions** | Project initiated. |
| **Inputs** | Faculty IDs, Student IDs, External Collaborators |
| **Outputs** | Updated Project Team |
| **Business Rules** | Exactly one Principal Investigator. Multiple Co-Investigators permitted. External collaborators subject to institutional approval. |
| **Authorization Requirements** | AssignResearchTeam permission |
| **Validation Rules** | Faculty active. Students active. Collaborators valid. |
| **Main Flow** | Validate Members → Assign Roles → Update Aggregate → Publish Event |
| **Alternative Flows** | Team modification during project execution. |
| **Failure Flows** | Reject duplicate or invalid participant. |
| **Rollback Rules** | Restore previous team composition. |
| **Post Conditions** | Team assignment completed. |
| **Published Domain Events** | ResearchTeamAssigned |
| **Consumed Events** | FacultyVerified, StudentVerified |
| **Audit Requirements** | Preserve role assignment history. |
| **Security Considerations** | Project permissions derived from assigned roles. |
| **Performance Considerations** | Team validation may execute asynchronously. |
| **Architecture Constraints** | Team members referenced only through enterprise identifiers. |

---

# 26.3 RM-10 — Manage Research Milestones

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Plan and maintain project milestones. |
| **Business Objective** | Track research execution progress. |
| **Actors** | Principal Investigator |
| **Trigger** | Milestone creation or update. |
| **Preconditions** | Project active. |
| **Inputs** | Milestone information |
| **Outputs** | Updated milestone schedule |
| **Business Rules** | Milestones may be added, updated, completed, or cancelled. |
| **Authorization Requirements** | ManageResearchMilestones permission |
| **Validation Rules** | Milestone dates within project duration. |
| **Main Flow** | Validate → Update Milestone → Publish Event |
| **Alternative Flows** | Milestone rescheduling. |
| **Failure Flows** | Reject invalid dates. |
| **Rollback Rules** | Restore previous milestone version. |
| **Post Conditions** | Project schedule updated. |
| **Published Domain Events** | ProjectMilestoneUpdated |
| **Consumed Events** | ResearchProjectCreated |
| **Audit Requirements** | Preserve milestone history. |
| **Security Considerations** | Updates limited to authorized project members. |
| **Performance Considerations** | Standard transactional operation. |
| **Architecture Constraints** | Milestones remain internal to Project Aggregate. |

---

# 26.4 RM-11 — Submit Progress Report

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Record project progress. |
| **Business Objective** | Maintain institutional visibility into research execution. |
| **Actors** | Principal Investigator |
| **Trigger** | Progress reporting schedule or manual submission. |
| **Preconditions** | Active project. |
| **Inputs** | Progress report, achievements, issues, deliverables |
| **Outputs** | Progress record |
| **Business Rules** | Reporting frequency configurable by institution. |
| **Authorization Requirements** | SubmitProgressReport permission |
| **Validation Rules** | Project active. Reporting period valid. |
| **Main Flow** | Validate → Store Report → Update Status → Publish Event |
| **Alternative Flows** | Ad-hoc reporting. |
| **Failure Flows** | Reject invalid reporting period. |
| **Rollback Rules** | Remove incomplete report. |
| **Post Conditions** | Progress available for monitoring. |
| **Published Domain Events** | ProjectProgressReported |
| **Consumed Events** | None |
| **Audit Requirements** | Preserve submitted reports permanently. |
| **Security Considerations** | Reports visible according to institutional policy. |
| **Performance Considerations** | Attachments managed by Enterprise DMS. |
| **Architecture Constraints** | Research stores report metadata only. |

---

# 26.5 RM-12 — Update Project Status

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Update project execution state. |
| **Business Objective** | Reflect actual project lifecycle progression. |
| **Actors** | Principal Investigator, Research Administrator |
| **Trigger** | Project status change. |
| **Preconditions** | Project exists. |
| **Inputs** | New status |
| **Outputs** | Updated project |
| **Business Rules** | Status transitions follow defined lifecycle. |
| **Authorization Requirements** | UpdateProjectStatus permission |
| **Validation Rules** | Valid state transition. |
| **Main Flow** | Validate Transition → Update Aggregate → Publish Event |
| **Alternative Flows** | Temporary suspension. |
| **Failure Flows** | Reject invalid transition. |
| **Rollback Rules** | Restore previous state. |
| **Post Conditions** | Project status updated. |
| **Published Domain Events** | ResearchProjectStatusChanged |
| **Consumed Events** | None |
| **Audit Requirements** | Record previous and new states. |
| **Security Considerations** | Restricted administrative operation. |
| **Performance Considerations** | Lightweight transaction. |
| **Architecture Constraints** | State transitions governed by aggregate rules. |

---

# 26.6 RM-13 — Complete Research Project

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Mark research execution as complete. |
| **Business Objective** | Complete operational research activities prior to closure. |
| **Actors** | Principal Investigator |
| **Trigger** | Completion request. |
| **Preconditions** | Major deliverables completed. |
| **Inputs** | Completion summary |
| **Outputs** | Completed project |
| **Business Rules** | Outstanding mandatory milestones must be resolved before completion. |
| **Authorization Requirements** | CompleteResearchProject permission |
| **Validation Rules** | Required deliverables complete. |
| **Main Flow** | Validate Completion → Update State → Publish Event |
| **Alternative Flows** | Partial completion when institution permits. |
| **Failure Flows** | Reject incomplete project. |
| **Rollback Rules** | Restore Active state. |
| **Post Conditions** | Project enters Completed state. |
| **Published Domain Events** | ResearchProjectCompleted |
| **Consumed Events** | ProjectMilestoneUpdated |
| **Audit Requirements** | Preserve completion summary. |
| **Security Considerations** | Authorized completion only. |
| **Performance Considerations** | Standard transaction. |
| **Architecture Constraints** | Completion does not archive project. |

---

# 26.7 RM-14 — Close Research Project

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Permanently close a completed research project. |
| **Business Objective** | Finalize project lifecycle and archive operational activity. |
| **Actors** | Research Office |
| **Trigger** | Closure approval. |
| **Preconditions** | Project completed. |
| **Inputs** | Closure approval |
| **Outputs** | Closed project |
| **Business Rules** | Financial, publication, and compliance obligations should be resolved before closure according to institutional policy. |
| **Authorization Requirements** | CloseResearchProject permission |
| **Validation Rules** | Completion confirmed. Required closure checks satisfied. |
| **Main Flow** | Validate → Close Project → Archive → Publish Event |
| **Alternative Flows** | Administrative closure. |
| **Failure Flows** | Reject premature closure. |
| **Rollback Rules** | Restore Completed state. |
| **Post Conditions** | Project enters Closed state. |
| **Published Domain Events** | ResearchProjectClosed |
| **Consumed Events** | ResearchProjectCompleted |
| **Audit Requirements** | Permanent archival audit record. |
| **Security Considerations** | Closed projects become read-only. |
| **Performance Considerations** | Archive asynchronously where supported. |
| **Architecture Constraints** | Closed projects remain queryable through CQRS read models. |

# 26.8 Research Project Lifecycle State Machine

## 26.8.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | InitiateResearchProject | Proposal Approved | Initiated | ResearchProjectCreated |
| Initiated | AssignResearchTeam | Team Valid | Active | ResearchTeamAssigned |
| Active | ManageResearchMilestones | Authorized | Active | ProjectMilestoneUpdated |
| Active | SubmitProgressReport | Reporting Period Valid | Active | ProjectProgressReported |
| Active | UpdateProjectStatus | Transition Allowed | Active / Suspended | ResearchProjectStatusChanged |
| Suspended | UpdateProjectStatus | Resume Approved | Active | ResearchProjectStatusChanged |
| Active | CompleteResearchProject | Completion Criteria Met | Completed | ResearchProjectCompleted |
| Completed | CloseResearchProject | Closure Validation Passed | Closed | ResearchProjectClosed |

---

## 26.8.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| Initiated | Complete Project | Execution not started |
| Initiated | Close Project | Project not completed |
| Completed | Assign Team | Team frozen after completion |
| Completed | Submit Progress Report | Execution completed |
| Closed | Modify Milestones | Closed projects are immutable |
| Closed | Assign Team | Lifecycle completed |
| Closed | Resume Project | Closed projects cannot be reopened unless institutional archival recovery policy explicitly permits it |

---

## 26.8.3 Project Lifecycle Diagram

```text
          Initiated
               │
               ▼
            Active
        ┌─────┴──────┐
        ▼            ▼
   Suspended     Completed
        │            │
        └──────┬─────┘
               ▼
             Closed
```

---

# 26.9 Project Business Rules

| Rule ID | Rule |
|----------|------|
| PJ-01 | Every project originates from an approved proposal. |
| PJ-02 | Every project shall have exactly one Principal Investigator. |
| PJ-03 | Multiple Co-Principal Investigators may be assigned according to institutional configuration. |
| PJ-04 | Research team members shall be referenced from authoritative enterprise domains. |
| PJ-05 | Project duration shall remain within approved limits unless extension approval is granted. |
| PJ-06 | Milestones may be modified while the project is Active. |
| PJ-07 | Progress reporting frequency is institution configurable. |
| PJ-08 | Completion requires mandatory deliverables to be satisfied. |
| PJ-09 | Closure requires institutional closure validation. |
| PJ-10 | Every successful state transition publishes an immutable domain event. |

---

# 26.10 Project Validation Rules

| Validation | Description |
|------------|-------------|
| Proposal Validation | Proposal must be approved. |
| Investigator Validation | Principal Investigator must remain active. |
| Team Validation | Every assigned participant shall exist in enterprise master data. |
| Milestone Validation | Planned dates shall fall within project duration. |
| Funding Validation | Funding references shall exist when funding is assigned. |
| Ethics Validation | Ethics approval shall exist where mandatory. |
| Duration Validation | Project end date shall not precede start date. |
| Completion Validation | Mandatory deliverables shall be completed before project completion. |

---

# 26.11 Events Published by Project Aggregate

| Event | Description |
|--------|-------------|
| ResearchProjectCreated | Project initiated |
| ResearchTeamAssigned | Team assigned |
| ProjectMilestoneUpdated | Milestone created or updated |
| ProjectProgressReported | Progress submitted |
| ResearchProjectStatusChanged | Project status modified |
| ResearchProjectCompleted | Project execution completed |
| ResearchProjectClosed | Project officially closed |

---

# 26.12 Events Consumed by Project Aggregate

| Event | Source |
|--------|--------|
| ResearchProposalApproved | Proposal Aggregate |
| FacultyVerified | L2-E03 Faculty Workforce |
| StudentVerified | Section C |
| EthicsApprovalGranted | Ethics Aggregate |
| GrantAwarded | Grant Aggregate |
| FundingAllocated | Grant Aggregate |

---

# 26.13 Aggregate Consistency Rules

The Research Project aggregate shall enforce the following consistency guarantees.

- A project shall always reference a valid approved proposal.
- Project numbering shall remain immutable.
- Team modifications shall increment aggregate version.
- Milestone updates shall be atomic.
- Progress reports shall never overwrite historical reports.
- Aggregate concurrency shall use optimistic locking.
- Project closure shall be irreversible under normal operating conditions.
- Cross-domain synchronization shall occur through published domain events only.

---

# 27. Workflow Specifications — Grant & Funding Management

Grant and Funding Management governs the acquisition, allocation, utilization, monitoring, and closure of research funding.

The aggregate supports internal funding, government grants, CSR funding, industry-sponsored projects, international funding, and self-funded research.

Financial accounting remains the responsibility of the Finance domain. Research Management owns only research-specific funding metadata and workflow state.
Finance remains the authoritative owner of budgets, accounting, ledgers, payment processing, procurement, taxation, and financial compliance.
Research Management owns research grant lifecycle management, grant application tracking, approval tracking, utilization metadata, and research funding status.

---

# 27.1 RM-15 — Apply for Research Grant

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Apply for research funding. |
| **Business Objective** | Initiate institutional grant application. |
| **Actors** | Principal Investigator |
| **Trigger** | Grant application request. |
| **Preconditions** | Project initiated or approved proposal eligible for grant application according to institutional policy. |
| **Inputs** | Grant Scheme, Budget, Proposal, Supporting Documents |
| **Outputs** | Grant Application |
| **Business Rules** | Funding source shall be institution-configured. |
| **Authorization Requirements** | ApplyResearchGrant permission |
| **Validation Rules** | Investigator eligibility shall be derived from L2-E03 Faculty Workforce. Funding scheme active. Budget complete. |
| **Main Flow** | Validate → Create Grant → Submit → Publish Event |
| **Alternative Flows** | Internal grant application. External funding portal submission. |
| **Failure Flows** | Reject incomplete application. |
| **Rollback Rules** | Remove incomplete grant record. |
| **Post Conditions** | Grant enters Proposed state. |
| **Published Domain Events** | ResearchGrantApplied |
| **Consumed Events** | ResearchProjectCreated |
| **Audit Requirements** | Preserve submitted budget and supporting evidence. |
| **Security Considerations** | Funding information restricted to authorized users. |
| **Performance Considerations** | External submission may execute asynchronously. |
| **Architecture Constraints** | Finance remains the authoritative owner of financial transactions. |

---

# 27.2 RM-16 — Record Grant Award

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Record successful grant approval. |
| **Business Objective** | Register awarded research funding. |
| **Actors** | Research Office |
| **Trigger** | Funding agency award notification. |
| **Preconditions** | Grant application exists. |
| **Inputs** | Award Reference, Approved Amount, Award Date |
| **Outputs** | Awarded Grant |
| **Business Rules** | Grant identifiers shall remain immutable after award. |
| **Authorization Requirements** | RecordGrantAward permission |
| **Validation Rules** | Funding agency verified. Award details complete. |
| **Main Flow** | Validate → Update Aggregate → Publish Event |
| **Alternative Flows** | Partial funding award. |
| **Failure Flows** | Reject duplicate award registration. |
| **Rollback Rules** | Restore previous grant status. |
| **Post Conditions** | Grant enters Awarded state. |
| **Published Domain Events** | GrantAwarded |
| **Consumed Events** | ResearchGrantApplied |
| **Audit Requirements** | Preserve official award documentation reference. |
| **Security Considerations** | Award details available according to institutional authorization policy. |
| **Performance Considerations** | Standard transactional workflow. |
| **Architecture Constraints** | Award registration does not perform accounting entries. |

# 27.3 RM-17 — Allocate Research Funding

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Allocate approved research funding to a project. |
| **Business Objective** | Distribute awarded funding for project execution. |
| **Actors** | Research Office, Finance Officer |
| **Trigger** | Grant award recorded or internal funding approved. |
| **Preconditions** | Grant awarded or institutional funding approved. |
| **Inputs** | Grant Identifier, Allocation Amount, Allocation Schedule |
| **Outputs** | Funding Allocation |
| **Business Rules** | Allocation shall not exceed approved funding amount. Allocation schedules are institution configurable. Faculty workload planning remains exclusively owned by L2-E03 Faculty Workforce. |
| **Authorization Requirements** | AllocateResearchFunding permission |
| **Validation Rules** | Funding available. Allocation within approved limits. Project active. |
| **Main Flow** | Validate Funding → Allocate Amount → Update Aggregate → Publish Event |
| **Alternative Flows** | Multi-phase funding allocation. |
| **Failure Flows** | Reject over-allocation. |
| **Rollback Rules** | Reverse allocation metadata update if transaction fails. |
| **Post Conditions** | Funding allocation recorded. |
| **Published Domain Events** | FundingAllocated |
| **Consumed Events** | GrantAwarded |
| **Audit Requirements** | Preserve allocation history and approving authority. |
| **Security Considerations** | Allocation data restricted to authorized financial and research personnel. |
| **Performance Considerations** | Financial synchronization may execute asynchronously. |
| **Architecture Constraints** | Finance remains the authoritative owner of financial ledger updates. |

```text
Grant Awarded
       │
       ▼
Validate Allocation
       │
Allocate Funding
       │
Update Aggregate
       │
Publish FundingAllocated
```

---

# 27.4 RM-18 — Record Funding Utilization

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Record utilization of allocated research funding. |
| **Business Objective** | Monitor research expenditure against allocated funding. |
| **Actors** | Principal Investigator, Research Office |
| **Trigger** | Funding utilization update. |
| **Preconditions** | Funding allocated. |
| **Inputs** | Utilization Amount, Category, Supporting Reference |
| **Outputs** | Funding Utilization Record |
| **Business Rules** | Research domain stores utilization metadata only. Financial accounting remains within Finance. |
| **Authorization Requirements** | RecordFundingUtilization permission |
| **Validation Rules** | Utilization shall not exceed available allocation according to institutional policy. |
| **Main Flow** | Validate → Record Utilization → Update Aggregate → Publish Event |
| **Alternative Flows** | Batch utilization import from Finance. |
| **Failure Flows** | Reject invalid utilization entry. |
| **Rollback Rules** | Restore previous utilization summary. |
| **Post Conditions** | Funding utilization updated. |
| **Published Domain Events** | FundingUtilizationRecorded |
| **Consumed Events** | FundingAllocated |
| **Audit Requirements** | Preserve utilization references and audit history. |
| **Security Considerations** | Financial metadata visible according to authorization policy. |
| **Performance Considerations** | Batch synchronization supported. |
| **Architecture Constraints** | No accounting journal entries are created by Research Management. |

---

# 27.5 RM-19 — Close Research Grant

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Close the lifecycle of a research grant. |
| **Business Objective** | Finalize grant administration after project completion. |
| **Actors** | Research Office |
| **Trigger** | Grant closure request. |
| **Preconditions** | Grant obligations completed. |
| **Inputs** | Closure Summary |
| **Outputs** | Closed Grant |
| **Business Rules** | Outstanding funding obligations shall be resolved before closure according to institutional policy. |
| **Authorization Requirements** | CloseResearchGrant permission |
| **Validation Rules** | Project completed where applicable. Grant reporting complete. |
| **Main Flow** | Validate Closure → Update Aggregate → Publish Event |
| **Alternative Flows** | Administrative closure. |
| **Failure Flows** | Reject premature closure. |
| **Rollback Rules** | Restore Awarded/Active state if transaction fails. |
| **Post Conditions** | Grant enters Closed state. |
| **Published Domain Events** | ResearchGrantClosed |
| **Consumed Events** | ResearchProjectCompleted |
| **Audit Requirements** | Preserve closure evidence and reporting references. |
| **Security Considerations** | Closed grants become read-only. |
| **Performance Considerations** | Archive processing may execute asynchronously. |
| **Architecture Constraints** | Historical grant records remain queryable through CQRS read models. |

---

# 27.6 Grant Lifecycle State Machine

## 27.6.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | ApplyResearchGrant | Eligibility Verified | Proposed | ResearchGrantApplied |
| Proposed | RecordGrantAward | Award Verified | Awarded | GrantAwarded |
| Awarded | AllocateResearchFunding | Allocation Valid | Active | FundingAllocated |
| Active | RecordFundingUtilization | Utilization Valid | Active | FundingUtilizationRecorded |
| Active | CloseResearchGrant | Closure Approved | Closed | ResearchGrantClosed |

---

## 27.6.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| Proposed | Allocate Funding | Grant not awarded |
| Proposed | Close Grant | Funding not finalized |
| Awarded | Close Grant | Utilization process incomplete |
| Closed | Allocate Funding | Closed grants are immutable |
| Closed | Record Utilization | Lifecycle completed |
| Closed | Modify Award | Award information immutable after closure |

---

## 27.6.3 Grant Lifecycle Diagram

```text
         Proposed
             │
             ▼
         Awarded
             │
             ▼
          Active
             │
             ▼
           Closed
```

---

# 27.7 Grant Business Rules

| Rule ID | Rule |
|----------|------|
| GR-01 | Funding source shall belong to an institution-configured funding category. |
| GR-02 | One grant may support one or more research projects if permitted by institutional configuration. |
| GR-03 | Allocation shall never exceed awarded funding. |
| GR-04 | Utilization shall reference approved allocations. |
| GR-05 | Financial accounting remains owned by the Finance domain. |
| GR-06 | Grant numbers remain immutable after award. |
| GR-07 | Grant closure requires completion of institutional reporting obligations. |
| GR-08 | Every grant lifecycle transition publishes a domain event. |

---

# 27.8 Grant Validation Rules

| Validation | Description |
|------------|-------------|
| Funding Scheme Validation | Funding source exists and is active. |
| Budget Validation | Requested budget satisfies institutional rules. |
| Award Validation | Award information complete and verified. |
| Allocation Validation | Allocation within approved limits. |
| Utilization Validation | Utilization references valid allocation. |
| Project Validation | Associated project exists and is eligible. |
| Closure Validation | Reporting and compliance requirements satisfied. |

---

# 27.9 Events Published by Grant Aggregate

| Event | Description |
|--------|-------------|
| ResearchGrantApplied | Grant application created |
| GrantAwarded | Grant approved |
| FundingAllocated | Funding allocated |
| FundingUtilizationRecorded | Utilization recorded |
| ResearchGrantClosed | Grant lifecycle completed |

---

# 27.10 Events Consumed by Grant Aggregate

| Event | Source |
|--------|--------|
| ResearchProjectCreated | Project Aggregate |
| ResearchProjectCompleted | Project Aggregate |
| FundingSchemeUpdated | Finance Domain |
| FundingAgencyVerified | External Integration |
| BudgetApproved | Finance Domain |

---

# 27.11 Aggregate Consistency Rules

- Every grant shall reference an approved funding source.
- Award amounts remain immutable after award registration unless amended through a governed amendment workflow.
- Funding allocations shall never exceed awarded funding.
- Utilization history is immutable.
- Aggregate version shall increment after every successful update.
- Cross-domain financial synchronization shall occur asynchronously through enterprise events or APIs. Finance remains the authoritative owner of accounting and posting decisions.
- Grant closure permanently freezes administrative modifications while preserving read-model accessibility.

# 28. Workflow Specifications — Ethics Management

Ethics Management governs institutional ethics review, approval, monitoring, renewal, and compliance for research activities.

Ethics approval is applicable only for research categories configured by the institution. The approval workflow, committee structure, review stages, and renewal policies are fully configurable.

All workflows operate on the **Ethics Approval Aggregate**.

---

# 28.1 RM-20 — Submit Ethics Application

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Submit a research project for institutional ethics review. |
| **Business Objective** | Initiate ethics approval for research requiring ethical clearance. |
| **Actors** | Principal Investigator |
| **Trigger** | Ethics application submission. |
| **Preconditions** | Active research project. Ethics required by research category. |
| **Inputs** | Project Identifier, Ethics Category, Supporting Documents |
| **Outputs** | Ethics Application |
| **Business Rules** | Ethics requirements are determined by institution configuration. |
| **Authorization Requirements** | SubmitEthicsApplication permission |
| **Validation Rules** | Project exists. Ethics required. Mandatory documents available. |
| **Main Flow** | Validate → Create Ethics Application → Submit → Publish Event |
| **Alternative Flows** | Expedited review, exempt review, full committee review. |
| **Failure Flows** | Reject incomplete application. |
| **Rollback Rules** | Remove incomplete submission upon transaction failure. |
| **Post Conditions** | Ethics application enters Submitted state. |
| **Published Domain Events** | EthicsApplicationSubmitted |
| **Consumed Events** | ResearchProjectCreated |
| **Audit Requirements** | Preserve application version and supporting evidence references. |
| **Security Considerations** | Ethics documents restricted to authorized reviewers. |
| **Performance Considerations** | Document storage delegated to Enterprise DMS. |
| **Architecture Constraints** | Research stores only metadata and document references. |

```text
Research Project
        │
        ▼
Submit Ethics Application
        │
Validate
        │
Create Application
        │
Publish EthicsApplicationSubmitted
```

---

# 28.2 RM-21 — Review Ethics Application

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Review ethics application. |
| **Business Objective** | Evaluate ethical compliance before approval. |
| **Actors** | Ethics Committee Member |
| **Trigger** | Application assigned for review. |
| **Preconditions** | Ethics application submitted. |
| **Inputs** | Review Comments, Recommendations |
| **Outputs** | Ethics Review Record |
| **Business Rules** | Committee composition and review stages are institution configurable. |
| **Authorization Requirements** | ReviewEthicsApplication permission |
| **Validation Rules** | Reviewer authorized. Committee assignment valid. |
| **Main Flow** | Assign Review → Evaluate → Record Findings → Publish Event |
| **Alternative Flows** | Multiple reviewers. External reviewer participation. |
| **Failure Flows** | Reject unauthorized review. |
| **Rollback Rules** | Remove incomplete review transaction. |
| **Post Conditions** | Review completed. |
| **Published Domain Events** | EthicsApplicationReviewed |
| **Consumed Events** | EthicsApplicationSubmitted |
| **Audit Requirements** | Preserve reviewer observations permanently. |
| **Security Considerations** | Confidential review access enforced. |
| **Performance Considerations** | Independent reviewer processing supported. |
| **Architecture Constraints** | Review records remain immutable after submission. |

---

# 28.3 RM-22 — Approve Ethics Request

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Approve ethics application. |
| **Business Objective** | Authorize ethical execution of research. |
| **Actors** | Ethics Committee Chair, Authorized Approver |
| **Trigger** | Approval decision. |
| **Preconditions** | Review completed. |
| **Inputs** | Approval Decision, Approval Conditions |
| **Outputs** | Approved Ethics Clearance |
| **Business Rules** | Approval validity period is institution configurable. |
| **Authorization Requirements** | ApproveEthicsRequest permission |
| **Validation Rules** | Required committee approvals completed. |
| **Main Flow** | Validate → Approve → Update Aggregate → Publish Event |
| **Alternative Flows** | Conditional approval. |
| **Failure Flows** | Reject invalid approval request. |
| **Rollback Rules** | Restore previous review state. |
| **Post Conditions** | Ethics application enters Approved state. |
| **Published Domain Events** | EthicsApprovalGranted |
| **Consumed Events** | EthicsApplicationReviewed |
| **Audit Requirements** | Preserve approval authority and validity period. |
| **Security Considerations** | Approval certificates accessible according to authorization policy. |
| **Performance Considerations** | Standard transactional workflow. |
| **Architecture Constraints** | Ethics approval does not modify Research Project aggregate directly. |

---

# 28.4 RM-23 — Reject Ethics Request

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Reject ethics application. |
| **Business Objective** | Prevent non-compliant research from proceeding. |
| **Actors** | Ethics Committee Chair |
| **Trigger** | Rejection decision. |
| **Preconditions** | Review completed. |
| **Inputs** | Rejection Reason |
| **Outputs** | Rejected Ethics Application |
| **Business Rules** | Rejection reason mandatory. |
| **Authorization Requirements** | RejectEthicsRequest permission |
| **Validation Rules** | Authorized committee decision. |
| **Main Flow** | Record Decision → Reject → Publish Event |
| **Alternative Flows** | None. |
| **Failure Flows** | Reject unauthorized operation. |
| **Rollback Rules** | Restore previous review state. |
| **Post Conditions** | Application enters Rejected state. |
| **Published Domain Events** | EthicsApplicationRejected |
| **Consumed Events** | EthicsApplicationReviewed |
| **Audit Requirements** | Preserve rejection rationale permanently. |
| **Security Considerations** | Decision immutable after completion. |
| **Performance Considerations** | Standard transaction. |
| **Architecture Constraints** | Rejected applications require new submission or governed resubmission process. |

---

# 28.5 RM-24 — Renew Ethics Approval

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Renew ethics approval for continuing research. |
| **Business Objective** | Maintain ethics compliance throughout project execution. |
| **Actors** | Principal Investigator |
| **Trigger** | Renewal request before approval expiry. |
| **Preconditions** | Existing ethics approval. |
| **Inputs** | Renewal Application, Updated Documents |
| **Outputs** | Renewed Ethics Approval |
| **Business Rules** | Renewal timing and eligibility are institution configurable. |
| **Authorization Requirements** | RenewEthicsApproval permission |
| **Validation Rules** | Existing approval valid. Renewal window satisfied. |
| **Main Flow** | Validate → Review Renewal → Approve Renewal → Publish Event |
| **Alternative Flows** | Administrative extension where policy permits. |
| **Failure Flows** | Reject expired or invalid renewal request. |
| **Rollback Rules** | Restore existing approval record if renewal fails. |
| **Post Conditions** | Ethics approval validity extended. |
| **Published Domain Events** | EthicsApprovalRenewed |
| **Consumed Events** | EthicsApprovalGranted |
| **Audit Requirements** | Preserve renewal history and validity periods. |
| **Security Considerations** | Renewal records inherit original security controls. |
| **Performance Considerations** | Lightweight transactional workflow. |
| **Architecture Constraints** | Renewal creates a new approval version while preserving historical approvals. |

---

# 28.6 Ethics Lifecycle State Machine

## 28.6.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | SubmitEthicsApplication | Project Eligible | Submitted | EthicsApplicationSubmitted |
| Submitted | ReviewEthicsApplication | Reviewer Assigned | Under Review | EthicsApplicationReviewed |
| Under Review | ApproveEthicsRequest | Committee Approval Complete | Approved | EthicsApprovalGranted |
| Under Review | RejectEthicsRequest | Decision Recorded | Rejected | EthicsApplicationRejected |
| Approved | RenewEthicsApproval | Renewal Eligible | Approved | EthicsApprovalRenewed |
| Approved | ExpireApproval | Validity Period Ends | Expired | EthicsApprovalExpired |

---

## 28.6.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| Submitted | Renew Approval | Approval not granted |
| Rejected | Approve Application | Requires governed resubmission |
| Expired | Approve Application | Renewal or resubmission required |
| Approved | Submit New Application | Existing approval active unless policy requires new application |
| Expired | Review Existing Application | Lifecycle completed |

---

## 28.6.3 Ethics Lifecycle Diagram

```text
          Submitted
               │
               ▼
         Under Review
         ┌─────┴─────┐
         ▼           ▼
     Approved     Rejected
         │
         ▼
     Renewed
         │
         ▼
      Expired
```

# 28.7 Ethics Business Rules

| Rule ID | Rule |
|----------|------|
| ET-01 | Ethics approval is mandatory only for institution-configured research categories. |
| ET-02 | Ethics committee structure shall be configurable. |
| ET-03 | Approval validity period shall be configurable. |
| ET-04 | Conditional approvals may define mandatory compliance conditions. |
| ET-05 | Ethics approval shall be linked to a single research project. |
| ET-06 | Renewal history shall remain immutable. |
| ET-07 | Rejection reason is mandatory. |
| ET-08 | Every successful lifecycle transition publishes a domain event. |
| ET-09 | Ethics approval does not replace statutory approvals required by external regulatory bodies. |
| ET-10 | Ethics documentation shall be stored in Enterprise DMS with only metadata maintained by Research Management. |

---

# 28.8 Ethics Validation Rules

| Validation | Description |
|------------|-------------|
| Project Validation | Associated research project shall exist. |
| Research Category Validation | Ethics requirement determined from configured research category. |
| Committee Validation | Appropriate ethics committee shall be configured. |
| Reviewer Validation | Reviewer shall be authorized committee member. |
| Document Validation | Mandatory ethics documents shall be available. |
| Renewal Validation | Renewal request shall be submitted within the configured renewal window. |
| Approval Validation | Required committee approvals shall be completed. |
| Validity Validation | Approval period shall comply with institutional policy. |

---

# 28.9 Events Published by Ethics Aggregate

| Event | Description |
|--------|-------------|
| EthicsApplicationSubmitted | Ethics application submitted |
| EthicsApplicationReviewed | Ethics review completed |
| EthicsApprovalGranted | Ethics approval granted |
| EthicsApplicationRejected | Ethics application rejected |
| EthicsApprovalRenewed | Ethics approval renewed |
| EthicsApprovalExpired | Ethics approval expired |

---

# 28.10 Events Consumed by Ethics Aggregate

| Event | Source |
|--------|--------|
| ResearchProjectCreated | Research Project Aggregate |
| ResearchProjectCompleted | Research Project Aggregate |
| ResearchCategoryConfigured | Configuration Domain |
| CommitteeConfigurationUpdated | Configuration Domain |
| ResearchProjectClosed | Research Project Aggregate |

---

# 28.11 Aggregate Consistency Rules

- Every ethics application references exactly one research project.
- Ethics approval identifiers remain immutable.
- Every review shall be permanently preserved.
- Approval validity shall never overlap for the same approval version.
- Renewal creates a new approval version while preserving previous versions.
- Aggregate version increments after every successful update.
- Cross-domain synchronization occurs through domain events only.
- Historical ethics decisions shall never be modified.

---

# 29. Workflow Specifications — Publication Management

Publication Management governs institutional research outputs including journals, conference papers, books, book chapters, technical reports, datasets, and other scholarly publications.

The module records publication metadata, verifies institutional ownership, synchronizes with external scholarly platforms, and contributes to researcher portfolios and institutional KPIs.

Research Management does not replace institutional repositories or publisher platforms. It maintains authoritative institutional publication metadata.

All workflows operate on the **Publication Aggregate**.

---

# 29.1 RM-25 — Register Publication

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Register a research publication within the institution. |
| **Business Objective** | Capture institutional publication metadata. |
| **Actors** | Faculty Member, Principal Investigator, Research Administrator |
| **Trigger** | Publication registration request. |
| **Preconditions** | Publication exists or has been accepted according to institutional policy. |
| **Inputs** | Publication Metadata, Authors, DOI, Venue, Project Reference |
| **Outputs** | Publication Aggregate |
| **Business Rules** | Publication types shall be institution configurable. |
| **Authorization Requirements** | RegisterPublication permission |
| **Validation Rules** | Author exists. Publication type valid. Duplicate publication validation completed. |
| **Main Flow** | Validate → Register Publication → Generate Identifier → Publish Event |
| **Alternative Flows** | Bulk publication import. |
| **Failure Flows** | Reject duplicate publication. |
| **Rollback Rules** | Remove incomplete publication record. |
| **Post Conditions** | Publication enters Draft state. |
| **Published Domain Events** | PublicationRegistered |
| **Consumed Events** | ResearchProjectCompleted |
| **Audit Requirements** | Preserve registration metadata. |
| **Security Considerations** | Editing restricted to authorized contributors. |
| **Performance Considerations** | Bulk import supported asynchronously. |
| **Architecture Constraints** | Publication documents remain in Enterprise DMS or external repositories. |

```text
Research Output
        │
        ▼
Validate
        │
Register Publication
        │
Generate Publication ID
        │
Publish PublicationRegistered
```

---

# 29.2 RM-26 — Verify Publication

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Verify publication authenticity and institutional ownership. |
| **Business Objective** | Ensure publication quality and institutional accuracy. |
| **Actors** | Research Office |
| **Trigger** | Verification request. |
| **Preconditions** | Publication registered. |
| **Inputs** | DOI, Publication Metadata, External References |
| **Outputs** | Verified Publication |
| **Business Rules** | Verification rules are institution configurable. |
| **Authorization Requirements** | VerifyPublication permission |
| **Validation Rules** | DOI valid where applicable. Author affiliations verified. |
| **Main Flow** | Validate Metadata → Verify Ownership → Update Aggregate → Publish Event |
| **Alternative Flows** | Manual verification when external validation unavailable. |
| **Failure Flows** | Mark publication as verification failed. |
| **Rollback Rules** | Restore previous verification status. |
| **Post Conditions** | Publication verification completed. |
| **Published Domain Events** | PublicationVerified |
| **Consumed Events** | PublicationRegistered |
| **Audit Requirements** | Preserve verification evidence. |
| **Security Considerations** | Verification actions restricted to authorized officers. |
| **Performance Considerations** | External verification may execute asynchronously. |
| **Architecture Constraints** | External scholarly databases remain authoritative for citation metadata. |

---

# 29.3 RM-27 — Publish Research Output

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Mark a verified publication as officially published. |
| **Business Objective** | Record completed scholarly output for institutional reporting and researcher portfolios. |
| **Actors** | Research Office |
| **Trigger** | Publication confirmation. |
| **Preconditions** | Publication verified. |
| **Inputs** | Publication Identifier |
| **Outputs** | Published Publication |
| **Business Rules** | Publication date becomes immutable after publishing. |
| **Authorization Requirements** | PublishResearchOutput permission |
| **Validation Rules** | Verification completed. Required metadata available. |
| **Main Flow** | Validate → Publish → Update Portfolio → Publish Domain Event |
| **Alternative Flows** | Automatic synchronization from trusted scholarly databases. |
| **Failure Flows** | Reject publication without verification. |
| **Rollback Rules** | Restore verified state if transaction fails. |
| **Post Conditions** | Publication enters Published state. |
| **Published Domain Events** | ResearchOutputPublished |
| **Consumed Events** | PublicationVerified |
| **Audit Requirements** | Preserve publication timestamp and publishing authority. |
| **Security Considerations** | Published records become institutionally visible according to policy. |
| **Performance Considerations** | Portfolio updates occur asynchronously. |
| **Architecture Constraints** | Published outputs feed Research Portfolio through domain events only. |

# 29.4 RM-28 — Archive Publication

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Archive a publication after its active institutional lifecycle. |
| **Business Objective** | Preserve publication records while preventing operational modifications. |
| **Actors** | Research Office |
| **Trigger** | Archive request. |
| **Preconditions** | Publication published or administratively closed. |
| **Inputs** | Publication Identifier, Archive Reason |
| **Outputs** | Archived Publication |
| **Business Rules** | Archived publications remain searchable but become read-only. |
| **Authorization Requirements** | ArchivePublication permission |
| **Validation Rules** | Publication exists. Archive policy satisfied. |
| **Main Flow** | Validate → Archive Publication → Publish Event |
| **Alternative Flows** | Bulk archival. |
| **Failure Flows** | Reject invalid archive request. |
| **Rollback Rules** | Restore Published state if transaction fails. |
| **Post Conditions** | Publication enters Archived state. |
| **Published Domain Events** | PublicationArchived |
| **Consumed Events** | ResearchOutputPublished |
| **Audit Requirements** | Preserve archival history permanently. |
| **Security Considerations** | Archived publications are read-only. |
| **Performance Considerations** | Bulk archival supported asynchronously. |
| **Architecture Constraints** | Archived records remain available through CQRS read models. |

---

# 29.5 Publication Lifecycle State Machine

## 29.5.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | RegisterPublication | Metadata Valid | Draft | PublicationRegistered |
| Draft | VerifyPublication | Verification Successful | Verified | PublicationVerified |
| Verified | PublishResearchOutput | Approval Complete | Published | ResearchOutputPublished |
| Published | ArchivePublication | Archive Policy Satisfied | Archived | PublicationArchived |

---

## 29.5.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| Draft | Publish Publication | Verification required |
| Draft | Archive Publication | Publication not published |
| Verified | Archive Publication | Publication must first be published |
| Archived | Edit Publication | Archived publications are immutable |
| Archived | Publish Publication | Lifecycle completed |

---

## 29.5.3 Publication Lifecycle Diagram

```text
          Draft
            │
            ▼
         Verified
            │
            ▼
        Published
            │
            ▼
         Archived
```

---

# 29.6 Publication Business Rules

| Rule ID | Rule |
|----------|------|
| PB-01 | Publication types shall be institution configurable. |
| PB-02 | Every publication shall have at least one institutional author. |
| PB-03 | Publications may reference one or more research projects. |
| PB-04 | DOI shall be unique where applicable. |
| PB-05 | Duplicate publications shall not be registered. |
| PB-06 | Publication documents remain outside the Research domain. |
| PB-07 | Publication metadata contributes to Research Portfolio KPIs. |
| PB-08 | Every publication lifecycle transition publishes a domain event. |
| PB-09 | Citation metrics are synchronized from external scholarly platforms. |
| PB-10 | Archived publications remain available for reporting. |

---

# 29.7 Publication Validation Rules

| Validation | Description |
|------------|-------------|
| Author Validation | Institutional author shall exist. |
| DOI Validation | DOI format and uniqueness validated where applicable. |
| Publication Type Validation | Publication type configured. |
| Venue Validation | Publication venue valid. |
| Project Validation | Referenced project exists where linked. |
| Duplicate Validation | Institution duplicate policy executed. |
| Verification Validation | External verification completed when required. |
| Archive Validation | Publication eligible for archival. |

---

# 29.8 Events Published by Publication Aggregate

| Event | Description |
|--------|-------------|
| PublicationRegistered | Publication registered |
| PublicationVerified | Publication verified |
| ResearchOutputPublished | Publication officially published |
| PublicationArchived | Publication archived |

---

# 29.9 Events Consumed by Publication Aggregate

| Event | Source |
|--------|--------|
| ResearchProjectCompleted | Project Aggregate |
| ResearchProjectClosed | Project Aggregate |
| RepositorySynchronizationCompleted | Repository Aggregate |
| ExternalPublicationImported | Integration Layer |

---

# 29.10 Aggregate Consistency Rules

- Publication identifiers remain immutable.
- DOI uniqueness shall be enforced where applicable.
- Published metadata shall remain historically traceable.
- Verification history is immutable.
- Aggregate version increments after every successful update.
- Portfolio updates occur asynchronously.
- External citation updates never directly modify aggregate state without validation.
- Historical publication records shall never be deleted.

---

# 30. Workflow Specifications — Intellectual Property Management

The Intellectual Property Management domain governs patents, copyrights, software registrations, designs, trademarks (if institutionally managed), and other forms of research-generated intellectual property.

The domain maintains institutional IP metadata, filing lifecycle, ownership, and status while integrating with external patent authorities.

All workflows operate on the **Patent Aggregate**.

---

# 30.1 RM-29 — Register Patent

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Register a patent or intellectual property record. |
| **Business Objective** | Capture institutional IP before formal filing. |
| **Actors** | Principal Investigator, Inventor, Research Office |
| **Trigger** | Patent registration request. |
| **Preconditions** | Patentable research output exists. |
| **Inputs** | Patent Metadata, Inventors, Project Reference |
| **Outputs** | Patent Aggregate |
| **Business Rules** | IP ownership follows institution-configured ownership policy. |
| **Authorization Requirements** | RegisterPatent permission |
| **Validation Rules** | Inventors valid. Project exists. Ownership policy satisfied. |
| **Main Flow** | Validate → Register Patent → Generate Identifier → Publish Event |
| **Alternative Flows** | Administrative registration. |
| **Failure Flows** | Reject duplicate registration. |
| **Rollback Rules** | Remove incomplete patent record. |
| **Post Conditions** | Patent enters Draft state. |
| **Published Domain Events** | PatentRegistered |
| **Consumed Events** | ResearchOutputPublished |
| **Audit Requirements** | Preserve inventor declarations and ownership references. |
| **Security Considerations** | Patent information restricted until disclosure is permitted. |
| **Performance Considerations** | Standard transactional workflow. |
| **Architecture Constraints** | Supporting documents remain within Enterprise DMS. |

---

# 30.2 RM-30 — Submit Patent Filing

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Submit patent for external filing. |
| **Business Objective** | Initiate statutory patent filing process. |
| **Actors** | Research Office |
| **Trigger** | Filing approval. |
| **Preconditions** | Patent registered. |
| **Inputs** | Filing Details, Patent Office |
| **Outputs** | Filed Patent |
| **Business Rules** | Filing authority is institution configurable. |
| **Authorization Requirements** | SubmitPatentFiling permission |
| **Validation Rules** | Required documentation complete. Filing authority selected. |
| **Main Flow** | Validate → Submit Filing → Update Aggregate → Publish Event |
| **Alternative Flows** | Electronic filing through external integration. |
| **Failure Flows** | Reject incomplete filing package. |
| **Rollback Rules** | Restore Draft state if filing fails before submission completion. |
| **Post Conditions** | Patent enters Filed state. |
| **Published Domain Events** | PatentFiled |
| **Consumed Events** | PatentRegistered |
| **Audit Requirements** | Preserve filing reference numbers and timestamps. |
| **Security Considerations** | Filing details visible only to authorized personnel. |
| **Performance Considerations** | External filing may execute asynchronously. |
| **Architecture Constraints** | External patent office remains authoritative for statutory filing status. |
# 30.3 RM-31 — Update Patent Status

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Update the lifecycle status of a patent or intellectual property record. |
| **Business Objective** | Maintain accurate institutional visibility into the statutory IP lifecycle. |
| **Actors** | Research Office |
| **Trigger** | Patent office notification or administrative update. |
| **Preconditions** | Patent filing exists. |
| **Inputs** | Patent Identifier, New Status, External Reference |
| **Outputs** | Updated Patent Record |
| **Business Rules** | Patent status shall reflect the latest verified statutory status. |
| **Authorization Requirements** | UpdatePatentStatus permission |
| **Validation Rules** | Patent exists. Status transition valid. External reference verified where applicable. |
| **Main Flow** | Validate → Update Status → Publish Event |
| **Alternative Flows** | Automatic synchronization from Patent Office integration. |
| **Failure Flows** | Reject invalid transition. |
| **Rollback Rules** | Restore previous status if update fails. |
| **Post Conditions** | Patent status updated. |
| **Published Domain Events** | PatentStatusUpdated |
| **Consumed Events** | PatentFiled |
| **Audit Requirements** | Preserve status history permanently. |
| **Security Considerations** | Status changes restricted to authorized personnel. |
| **Performance Considerations** | External synchronization may execute asynchronously. |
| **Architecture Constraints** | External Patent Office remains the statutory authority. |

---

# 30.4 RM-32 — Close Patent Lifecycle

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Close institutional management of a patent lifecycle. |
| **Business Objective** | Complete operational administration of institutional IP. |
| **Actors** | Research Office |
| **Trigger** | Patent granted, expired, abandoned, or administratively closed. |
| **Preconditions** | Patent lifecycle completed. |
| **Inputs** | Closure Information |
| **Outputs** | Closed Patent Record |
| **Business Rules** | Closure reason is mandatory. |
| **Authorization Requirements** | ClosePatentLifecycle permission |
| **Validation Rules** | Patent lifecycle eligible for closure. |
| **Main Flow** | Validate → Close Patent → Archive → Publish Event |
| **Alternative Flows** | Administrative closure. |
| **Failure Flows** | Reject premature closure. |
| **Rollback Rules** | Restore previous lifecycle state. |
| **Post Conditions** | Patent enters Closed state. |
| **Published Domain Events** | PatentLifecycleClosed |
| **Consumed Events** | PatentStatusUpdated |
| **Audit Requirements** | Preserve complete lifecycle history. |
| **Security Considerations** | Closed patent records become read-only. |
| **Performance Considerations** | Archive operations may execute asynchronously. |
| **Architecture Constraints** | Historical patent records remain available through CQRS read models. |

---

# 30.5 Patent Lifecycle State Machine

## 30.5.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | RegisterPatent | Metadata Valid | Draft | PatentRegistered |
| Draft | SubmitPatentFiling | Filing Ready | Filed | PatentFiled |
| Filed | UpdatePatentStatus | Valid Status | Filed / Under Examination / Granted / Rejected | PatentStatusUpdated |
| Granted | ClosePatentLifecycle | Closure Approved | Closed | PatentLifecycleClosed |
| Rejected | ClosePatentLifecycle | Closure Approved | Closed | PatentLifecycleClosed |
| Expired | ClosePatentLifecycle | Closure Approved | Closed | PatentLifecycleClosed |

---

## 30.5.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| Draft | Close Patent | Filing not initiated |
| Draft | Update Status | Patent not filed |
| Filed | Register Patent | Already registered |
| Closed | Modify Patent | Lifecycle completed |
| Closed | Submit Filing | Closed patents are immutable |

---

## 30.5.3 Patent Lifecycle Diagram

```text
          Draft
            │
            ▼
          Filed
            │
            ▼
   Under Examination
      ┌────┼─────┐
      ▼    ▼     ▼
 Granted Rejected Expired
      └────┼─────┘
           ▼
         Closed
```

---

# 30.6 Patent Business Rules

| Rule ID | Rule |
|----------|------|
| IP-01 | IP ownership follows institution-configured ownership policy. |
| IP-02 | Every patent shall reference at least one inventor. |
| IP-03 | Multiple inventors are permitted. |
| IP-04 | Patent identifiers remain immutable. |
| IP-05 | Filing authority is institution configurable. |
| IP-06 | External patent office status is authoritative. |
| IP-07 | Historical filing information shall never be modified. |
| IP-08 | Every successful lifecycle transition publishes a domain event. |
| IP-09 | Supporting documents remain within Enterprise DMS. |
| IP-10 | Closed patent records remain queryable for reporting and compliance. |

---

# 30.7 Patent Validation Rules

| Validation | Description |
|------------|-------------|
| Inventor Validation | Inventors shall exist in institutional records or approved external collaborator records. |
| Ownership Validation | Ownership policy shall be satisfied. |
| Filing Validation | Filing package complete. |
| Patent Office Validation | Patent office configured. |
| Status Validation | Status transition valid. |
| Duplicate Validation | Duplicate patent registration prohibited. |
| Project Validation | Referenced research project exists where applicable. |
| Closure Validation | Patent lifecycle eligible for closure. |

---

# 30.8 Events Published by Patent Aggregate

| Event | Description |
|--------|-------------|
| PatentRegistered | Patent registered |
| PatentFiled | Patent filing submitted |
| PatentStatusUpdated | Patent status changed |
| PatentLifecycleClosed | Patent lifecycle closed |

---

# 30.9 Events Consumed by Patent Aggregate

| Event | Source |
|--------|--------|
| ResearchOutputPublished | Publication Aggregate |
| ResearchProjectCompleted | Project Aggregate |
| RepositorySynchronizationCompleted | Repository Aggregate |
| ExternalPatentStatusUpdated | Integration Layer |

---

# 30.10 Aggregate Consistency Rules

- Patent identifiers remain immutable.
- Every patent references one ownership policy.
- Filing history shall never be modified.
- Status history remains immutable.
- Aggregate version increments after every successful update.
- Patent office synchronization occurs asynchronously.
- Historical IP records are retained permanently.
- Patent lifecycle changes shall never directly modify external statutory records.

---

# 31. Workflow Specifications — Collaboration Management

Collaboration Management governs research collaborations with universities, industries, government agencies, research institutes, international organizations, and other approved partners.

The domain maintains collaboration agreements, participating organizations, researchers, collaboration activities, and governance metadata.

All workflows operate on the **Collaboration Aggregate**.

---

# 31.1 RM-33 — Create Collaboration

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Register a research collaboration. |
| **Business Objective** | Establish an institutional collaboration agreement. |
| **Actors** | Research Office |
| **Trigger** | Collaboration initiation request. |
| **Preconditions** | Participating organizations identified. |
| **Inputs** | Collaboration Metadata, Partner Organization, Agreement Details |
| **Outputs** | Collaboration Aggregate |
| **Business Rules** | Collaboration types are institution configurable. |
| **Authorization Requirements** | CreateCollaboration permission |
| **Validation Rules** | Organization verified. Agreement complete. |
| **Main Flow** | Validate → Register Collaboration → Publish Event |
| **Alternative Flows** | Multi-institution collaborations. |
| **Failure Flows** | Reject duplicate collaboration. |
| **Rollback Rules** | Remove incomplete collaboration record. |
| **Post Conditions** | Collaboration enters Proposed state. |
| **Published Domain Events** | CollaborationCreated |
| **Consumed Events** | None |
| **Audit Requirements** | Preserve agreement metadata and participating organizations. |
| **Security Considerations** | Collaboration data accessible according to institutional policy. |
| **Performance Considerations** | Standard transactional workflow. |
| **Architecture Constraints** | Legal agreements remain stored within Enterprise DMS. |
# 31.2 RM-34 — Approve Collaboration

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Approve an institutional research collaboration. |
| **Business Objective** | Authorize collaboration commencement. |
| **Actors** | Research Director, Authorized Institutional Approver |
| **Trigger** | Collaboration approval request. |
| **Preconditions** | Collaboration registered. |
| **Inputs** | Approval Decision |
| **Outputs** | Approved Collaboration |
| **Business Rules** | Approval workflow is institution configurable. |
| **Authorization Requirements** | ApproveCollaboration permission |
| **Validation Rules** | Agreement complete. Partner organization verified. |
| **Main Flow** | Validate → Approve Collaboration → Publish Event |
| **Alternative Flows** | Multi-level institutional approval. |
| **Failure Flows** | Reject unauthorized approval. |
| **Rollback Rules** | Restore Proposed state. |
| **Post Conditions** | Collaboration enters Active state. |
| **Published Domain Events** | CollaborationApproved |
| **Consumed Events** | CollaborationCreated |
| **Audit Requirements** | Preserve approval authority and approval history. |
| **Security Considerations** | Collaboration visibility follows institutional authorization policy. |
| **Performance Considerations** | Workflow routing supported asynchronously. |
| **Architecture Constraints** | Approval does not modify external partner systems directly. |

---

# 31.3 RM-35 — Manage Collaboration Activities

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Record and manage collaboration activities. |
| **Business Objective** | Monitor collaboration execution throughout its lifecycle. |
| **Actors** | Principal Investigator, Research Office |
| **Trigger** | Collaboration activity update. |
| **Preconditions** | Collaboration active. |
| **Inputs** | Activity Details, Milestones, Deliverables |
| **Outputs** | Updated Collaboration Record |
| **Business Rules** | Activities remain linked to the collaboration agreement. |
| **Authorization Requirements** | ManageCollaborationActivities permission |
| **Validation Rules** | Collaboration active. Activity valid. |
| **Main Flow** | Validate → Record Activity → Publish Event |
| **Alternative Flows** | Joint activities involving multiple organizations. |
| **Failure Flows** | Reject invalid activity. |
| **Rollback Rules** | Restore previous activity record. |
| **Post Conditions** | Collaboration history updated. |
| **Published Domain Events** | CollaborationActivityRecorded |
| **Consumed Events** | CollaborationApproved |
| **Audit Requirements** | Preserve complete collaboration activity history. |
| **Security Considerations** | Activities visible only to authorized participants. |
| **Performance Considerations** | Large activity imports supported asynchronously. |
| **Architecture Constraints** | Activity documents stored within Enterprise DMS. |

---

# 31.4 RM-36 — Complete Collaboration

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Complete an institutional collaboration. |
| **Business Objective** | Close collaboration after agreed objectives have been achieved. |
| **Actors** | Research Office |
| **Trigger** | Collaboration completion request. |
| **Preconditions** | Collaboration active. |
| **Inputs** | Completion Summary |
| **Outputs** | Completed Collaboration |
| **Business Rules** | Completion evidence shall be preserved permanently. |
| **Authorization Requirements** | CompleteCollaboration permission |
| **Validation Rules** | Mandatory collaboration obligations completed. |
| **Main Flow** | Validate → Complete Collaboration → Publish Event |
| **Alternative Flows** | Administrative termination. |
| **Failure Flows** | Reject incomplete collaboration. |
| **Rollback Rules** | Restore Active state. |
| **Post Conditions** | Collaboration enters Completed state. |
| **Published Domain Events** | CollaborationCompleted |
| **Consumed Events** | CollaborationActivityRecorded |
| **Audit Requirements** | Preserve completion evidence and summary. |
| **Security Considerations** | Completed collaborations become read-only. |
| **Performance Considerations** | Standard transactional workflow. |
| **Architecture Constraints** | Historical collaborations remain queryable through CQRS read models. |

---

# 31.5 Collaboration Lifecycle State Machine

## 31.5.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | CreateCollaboration | Agreement Valid | Proposed | CollaborationCreated |
| Proposed | ApproveCollaboration | Approval Complete | Active | CollaborationApproved |
| Active | ManageCollaborationActivities | Authorized | Active | CollaborationActivityRecorded |
| Active | CompleteCollaboration | Completion Criteria Met | Completed | CollaborationCompleted |

---

## 31.5.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| Proposed | Record Activity | Collaboration not approved |
| Proposed | Complete Collaboration | Collaboration not active |
| Active | Create Collaboration | Already exists |
| Completed | Modify Collaboration | Lifecycle completed |
| Completed | Approve Collaboration | Already completed |

---

## 31.5.3 Collaboration Lifecycle Diagram

```text
        Proposed
            │
            ▼
          Active
            │
            ▼
        Completed
```

---

# 31.6 Collaboration Business Rules

| Rule ID | Rule |
|----------|------|
| CL-01 | Collaboration types shall be institution configurable. |
| CL-02 | Every collaboration shall include at least one external organization. |
| CL-03 | Multiple institutional and external partners are permitted. |
| CL-04 | Approval workflow shall be configurable. |
| CL-05 | Legal agreements remain outside the Research domain. |
| CL-06 | Collaboration history shall remain immutable. |
| CL-07 | Every lifecycle transition publishes a domain event. |
| CL-08 | Completed collaborations remain available for institutional reporting. |

---

# 31.7 Collaboration Validation Rules

| Validation | Description |
|------------|-------------|
| Organization Validation | Partner organization verified. |
| Agreement Validation | Collaboration agreement available. |
| Approval Validation | Required institutional approvals completed. |
| Activity Validation | Collaboration active before recording activities. |
| Completion Validation | Required obligations completed. |
| Participant Validation | Participants exist in institutional or approved external records. |

---

# 31.8 Events Published by Collaboration Aggregate

| Event | Description |
|--------|-------------|
| CollaborationCreated | Collaboration registered |
| CollaborationApproved | Collaboration approved |
| CollaborationActivityRecorded | Collaboration activity recorded |
| CollaborationCompleted | Collaboration completed |

---

# 31.9 Events Consumed by Collaboration Aggregate

| Event | Source |
|--------|--------|
| ResearchProjectCreated | Project Aggregate |
| ResearchProjectCompleted | Project Aggregate |
| ExternalOrganizationVerified | Integration Layer |
| AgreementRegistered | Enterprise DMS |

---

# 31.10 Aggregate Consistency Rules

- Collaboration identifiers remain immutable.
- Every collaboration references one or more verified organizations.
- Agreement history remains immutable.
- Activity history shall never be deleted.
- Aggregate version increments after successful updates.
- Cross-domain synchronization occurs through enterprise events.
- Completed collaborations become immutable.
- Historical collaboration data remains permanently queryable.

---

# 32. Workflow Specifications — Consultancy Management

Consultancy projects are treated as a specialized category of research projects within the Research Management domain.

Consultancy workflows govern industry-sponsored advisory work, technical consulting, professional services, feasibility studies, technology transfer, and similar institutional engagements.

Financial transactions remain owned by the Finance domain. Research Management maintains consultancy governance, metadata, deliverables, and lifecycle state.
Finance remains the authoritative owner of budgets, accounting, ledgers, payment processing, procurement, taxation, and financial compliance.

All workflows operate on the **Consultancy Project Aggregate**.

---

# 32.1 RM-37 — Register Consultancy Project

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Register a consultancy engagement. |
| **Business Objective** | Establish an institutional consultancy project. |
| **Actors** | Research Office, Principal Investigator |
| **Trigger** | Consultancy registration request. |
| **Preconditions** | Consultancy proposal approved according to institutional policy. |
| **Inputs** | Client Organization, Agreement Metadata, Project Details |
| **Outputs** | Consultancy Project Aggregate |
| **Business Rules** | Consultancy type shall be institution configurable. |
| **Authorization Requirements** | RegisterConsultancyProject permission |
| **Validation Rules** | Client organization verified. Agreement available. |
| **Main Flow** | Validate → Register Consultancy → Publish Event |
| **Alternative Flows** | Multi-client consultancy projects. |
| **Failure Flows** | Reject duplicate consultancy registration. |
| **Rollback Rules** | Remove incomplete consultancy record. |
| **Post Conditions** | Consultancy project enters Proposed state. |
| **Published Domain Events** | ConsultancyProjectRegistered |
| **Consumed Events** | ResearchProjectCreated |
| **Audit Requirements** | Preserve client references and agreement metadata. |
| **Security Considerations** | Client information protected according to institutional policy. |
| **Performance Considerations** | Standard transactional workflow. |
| **Architecture Constraints** | Consultancy contracts remain stored within Enterprise DMS. |
# 32.2 RM-38 — Execute Consultancy Project

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Execute an approved consultancy engagement. |
| **Business Objective** | Deliver consultancy services according to institutional commitments. |
| **Actors** | Principal Investigator, Consultancy Team |
| **Trigger** | Consultancy execution begins. |
| **Preconditions** | Consultancy project registered and approved. |
| **Inputs** | Work Plan, Deliverables, Milestones |
| **Outputs** | Updated Consultancy Project |
| **Business Rules** | Deliverables shall align with consultancy agreement. |
| **Authorization Requirements** | ExecuteConsultancyProject permission |
| **Validation Rules** | Consultancy active. Team assigned. Client agreement valid. |
| **Main Flow** | Validate → Record Activities → Update Progress → Publish Event |
| **Alternative Flows** | Phase-wise execution. Multi-client execution where permitted. |
| **Failure Flows** | Reject execution before approval. |
| **Rollback Rules** | Restore previous execution state. |
| **Post Conditions** | Consultancy execution progress updated. |
| **Published Domain Events** | ConsultancyProjectProgressUpdated |
| **Consumed Events** | ConsultancyProjectRegistered |
| **Audit Requirements** | Preserve execution history and deliverable references. |
| **Security Considerations** | Client-specific information accessible only to authorized participants. |
| **Performance Considerations** | Progress reporting supports asynchronous processing. |
| **Architecture Constraints** | Deliverable documents remain stored in Enterprise DMS. |

---

# 32.3 RM-39 — Complete Consultancy Project

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Complete consultancy engagement. |
| **Business Objective** | Close consultancy after contractual obligations are fulfilled. |
| **Actors** | Research Office |
| **Trigger** | Completion request. |
| **Preconditions** | Consultancy deliverables completed. |
| **Inputs** | Completion Report |
| **Outputs** | Completed Consultancy Project |
| **Business Rules** | Completion evidence shall be preserved permanently. |
| **Authorization Requirements** | CompleteConsultancyProject permission |
| **Validation Rules** | Required deliverables completed. Client acceptance recorded where applicable. |
| **Main Flow** | Validate → Complete Consultancy → Publish Event |
| **Alternative Flows** | Administrative closure. |
| **Failure Flows** | Reject incomplete consultancy. |
| **Rollback Rules** | Restore Active state. |
| **Post Conditions** | Consultancy project enters Completed state. |
| **Published Domain Events** | ConsultancyProjectCompleted |
| **Consumed Events** | ConsultancyProjectProgressUpdated |
| **Audit Requirements** | Preserve completion report and closure evidence. |
| **Security Considerations** | Completed consultancy becomes read-only. |
| **Performance Considerations** | Archive processing supported asynchronously. |
| **Architecture Constraints** | Historical consultancy projects remain queryable through CQRS read models. |

---

# 32.4 Consultancy Lifecycle State Machine

## 32.4.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | RegisterConsultancyProject | Registration Valid | Proposed | ConsultancyProjectRegistered |
| Proposed | ExecuteConsultancyProject | Approval Complete | Active | ConsultancyProjectProgressUpdated |
| Active | ExecuteConsultancyProject | Progress Valid | Active | ConsultancyProjectProgressUpdated |
| Active | CompleteConsultancyProject | Completion Criteria Met | Completed | ConsultancyProjectCompleted |

---

## 32.4.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| Proposed | Complete Consultancy | Execution not started |
| Proposed | Record Progress | Consultancy inactive |
| Completed | Execute Consultancy | Lifecycle completed |
| Completed | Modify Deliverables | Completed projects immutable |

---

## 32.4.3 Consultancy Lifecycle Diagram

```text
        Proposed
            │
            ▼
          Active
            │
            ▼
        Completed
```

---

# 32.5 Consultancy Business Rules

| Rule ID | Rule |
|----------|------|
| CS-01 | Consultancy is treated as a research project category. |
| CS-02 | Consultancy types shall be institution configurable. |
| CS-03 | Client organization shall be verified. |
| CS-04 | Deliverables shall align with consultancy agreement. |
| CS-05 | Finance remains the authoritative owner of consultancy revenue accounting. |
| CS-06 | Consultancy contracts remain outside the Research domain. |
| CS-07 | Every lifecycle transition publishes a domain event. |
| CS-08 | Completed consultancy projects remain available for institutional reporting. |

---

# 32.6 Consultancy Validation Rules

| Validation | Description |
|------------|-------------|
| Client Validation | Client organization verified. |
| Agreement Validation | Consultancy agreement available. |
| Team Validation | Consultancy team assigned. |
| Deliverable Validation | Deliverables satisfy contractual obligations. |
| Completion Validation | Completion evidence available. |
| Project Validation | Consultancy project exists and is active. |

---

# 32.7 Events Published by Consultancy Aggregate

| Event | Description |
|--------|-------------|
| ConsultancyProjectRegistered | Consultancy registered |
| ConsultancyProjectProgressUpdated | Consultancy progress updated |
| ConsultancyProjectCompleted | Consultancy completed |

---

# 32.8 Events Consumed by Consultancy Aggregate

| Event | Source |
|--------|--------|
| ResearchProjectCreated | Research Project Aggregate |
| CollaborationApproved | Collaboration Aggregate |
| ClientOrganizationVerified | Integration Layer |
| AgreementRegistered | Enterprise DMS |

---

# 32.9 Aggregate Consistency Rules

- Consultancy identifiers remain immutable.
- Every consultancy references one verified client organization.
- Consultancy agreements remain immutable after approval.
- Progress history shall never overwrite historical records.
- Aggregate version increments after every successful update.
- Financial synchronization occurs asynchronously with the Finance domain. Finance remains the authoritative owner of accounting and posting decisions.
- Completed consultancy projects become immutable.
- Historical consultancy records remain permanently queryable.

---

# 33. Workflow Specifications — Repository & Portfolio Management

Repository and Portfolio Management governs institutional research repositories, researcher portfolios, institutional research metrics, KPI generation, and enterprise reporting.

This capability aggregates information from Publications, Patents, Grants, Consultancy, and Research Projects to produce consolidated institutional research intelligence.

The Repository & Portfolio aggregates are **read-optimized** and are updated primarily through published domain events rather than direct transactional workflows.

H05 stores repository metadata only.
Enterprise Document Management owns binary files.
PDFs, datasets, images, office documents, source code archives, supplementary files, patents, and publication files SHALL NOT be stored inside H05.
H05 SHALL reference enterprise document identifiers only.
Metadata synchronization SHALL occur through integration events.

# 33.1 RM-40 — Register Repository Metadata

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Register institutional repository metadata for a research output. |
| **Business Objective** | Enable institutional and external repository indexing of research outputs. |
| **Actors** | Research Office |
| **Trigger** | Publication or patent becomes eligible for repository registration. |
| **Preconditions** | Research output published or institutionally approved. |
| **Inputs** | Repository Identifier, Research Output Metadata |
| **Outputs** | Repository Registration |
| **Business Rules** | Repository types shall be institution configurable. |
| **Authorization Requirements** | RegisterRepositoryMetadata permission |
| **Validation Rules** | Output exists. Repository configured. Metadata complete. |
| **Main Flow** | Validate → Register Metadata → Publish Event |
| **Alternative Flows** | Automatic registration through repository integration. |
| **Failure Flows** | Reject duplicate registration. |
| **Rollback Rules** | Remove incomplete registration record. |
| **Post Conditions** | Repository registration enters Pending state. |
| **Published Domain Events** | RepositoryMetadataRegistered |
| **Consumed Events** | ResearchOutputPublished, PatentFiled |
| **Audit Requirements** | Preserve registration history. |
| **Security Considerations** | Repository metadata follows institutional publication policies. |
| **Performance Considerations** | Bulk repository registration supported asynchronously. |
| **Architecture Constraints** | Repository systems remain external authoritative storage locations. |

---

# 33.2 RM-41 — Synchronize Repository

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Synchronize institutional repository metadata with external repositories. |
| **Business Objective** | Keep institutional repository information aligned with external platforms. |
| **Actors** | System Integration Service |
| **Trigger** | Synchronization schedule or repository event. |
| **Preconditions** | Repository metadata registered. |
| **Inputs** | Repository Identifier |
| **Outputs** | Synchronized Repository Record |
| **Business Rules** | Synchronization frequency shall be institution configurable. |
| **Authorization Requirements** | System Integration Authorization |
| **Validation Rules** | Repository reachable. Metadata valid. |
| **Main Flow** | Load Metadata → Synchronize → Update Status → Publish Event |
| **Alternative Flows** | Retry failed synchronization automatically. |
| **Failure Flows** | Record synchronization failure for retry. |
| **Rollback Rules** | Preserve previous synchronization state. |
| **Post Conditions** | Repository synchronization completed. |
| **Published Domain Events** | RepositorySynchronizationCompleted |
| **Consumed Events** | RepositoryMetadataRegistered |
| **Audit Requirements** | Preserve synchronization history. |
| **Security Considerations** | Secure API integration with external repositories. |
| **Performance Considerations** | Batch synchronization supported. |
| **Architecture Constraints** | Repository synchronization is eventually consistent. |

---

# 33.3 RM-42 — Update Research Portfolio

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Update researcher and institutional research portfolios. |
| **Business Objective** | Maintain consolidated research achievements. |
| **Actors** | System |
| **Trigger** | Publication, Patent, Grant, Consultancy, or Project domain event. |
| **Preconditions** | Source aggregate event published. |
| **Inputs** | Domain Event |
| **Outputs** | Updated Portfolio |
| **Business Rules** | Portfolio updates occur asynchronously. |
| **Authorization Requirements** | System Event Handler |
| **Validation Rules** | Event valid. Referenced entities exist. |
| **Main Flow** | Consume Event → Update Read Model → Publish Event |
| **Alternative Flows** | Portfolio rebuild from historical events. |
| **Failure Flows** | Queue retry through event infrastructure. |
| **Rollback Rules** | Event replay restores consistency. |
| **Post Conditions** | Portfolio reflects latest research achievements. |
| **Published Domain Events** | ResearchPortfolioUpdated |
| **Consumed Events** | ResearchOutputPublished, PatentFiled, GrantAwarded, ConsultancyProjectCompleted, ResearchProjectCompleted |
| **Audit Requirements** | Portfolio rebuild history maintained. |
| **Security Considerations** | Portfolio visibility determined by institutional policy. |
| **Performance Considerations** | CQRS read model optimized for reporting. |
| **Architecture Constraints** | Portfolio is a derived read model and never modifies source aggregates. |

---

# 33.4 RM-43 — Generate Research KPIs

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Calculate institutional research performance indicators. |
| **Business Objective** | Produce enterprise research analytics. |
| **Actors** | System |
| **Trigger** | Scheduled execution or administrator request. |
| **Preconditions** | Portfolio data available. |
| **Inputs** | Portfolio Read Models |
| **Outputs** | KPI Snapshot |
| **Business Rules** | KPI definitions shall be institution configurable. |
| **Authorization Requirements** | GenerateResearchKPIs permission |
| **Validation Rules** | Required source data available. |
| **Main Flow** | Load Portfolio → Calculate KPIs → Publish Event |
| **Alternative Flows** | Department-level KPI calculation. |
| **Failure Flows** | Log calculation failure and retry. |
| **Rollback Rules** | Previous KPI snapshot retained until successful recalculation. |
| **Post Conditions** | Latest KPI snapshot available. |
| **Published Domain Events** | ResearchKPIsGenerated |
| **Consumed Events** | ResearchPortfolioUpdated |
| **Audit Requirements** | Preserve KPI calculation history. |
| **Security Considerations** | KPI access governed by institutional authorization. |
| **Performance Considerations** | Batch processing supported. |
| **Architecture Constraints** | KPI calculations operate on read models only. |

---

# 33.5 RM-44 — Generate Research Reports

| Field | Enterprise Specification |
|-------|--------------------------|
| **Purpose** | Produce operational and analytical research reports. |
| **Business Objective** | Support institutional governance and decision making. |
| **Actors** | Research Office, Management |
| **Trigger** | Report request or scheduled reporting cycle. |
| **Preconditions** | Required read models available. |
| **Inputs** | Report Parameters |
| **Outputs** | Research Reports |
| **Business Rules** | Report templates shall be institution configurable. |
| **Authorization Requirements** | GenerateResearchReports permission |
| **Validation Rules** | User authorized. Parameters valid. |
| **Main Flow** | Validate → Query Read Models → Generate Report |
| **Alternative Flows** | Scheduled report distribution. |
| **Failure Flows** | Return report generation failure. |
| **Rollback Rules** | Not Applicable (read-only operation). |
| **Post Conditions** | Report available for download or dashboard presentation. |
| **Published Domain Events** | ResearchReportGenerated |
| **Consumed Events** | ResearchKPIsGenerated |
| **Audit Requirements** | Record report generation requests. |
| **Security Considerations** | Reports filtered according to authorization scope. |
| **Performance Considerations** | Long-running reports executed asynchronously. |
| **Architecture Constraints** | Reports query CQRS read models only; transactional aggregates are never queried directly. |

---

# 33.6 Repository Lifecycle State Machine

## 33.6.1 State Transition Matrix

| Current State | Command | Validation | Next State | Published Event |
|---------------|---------|------------|------------|-----------------|
| None | RegisterRepositoryMetadata | Metadata Valid | Pending | RepositoryMetadataRegistered |
| Pending | SynchronizeRepository | Repository Available | Registered | RepositorySynchronizationCompleted |
| Registered | SynchronizeRepository | Synchronization Required | Registered | RepositorySynchronizationCompleted |
| Failed | SynchronizeRepository | Retry Successful | Registered | RepositorySynchronizationCompleted |

---

## 33.6.2 Invalid State Transitions

| Current State | Invalid Operation | Reason |
|---------------|------------------|--------|
| None | Synchronize Repository | Registration required |
| Registered | Register Metadata | Already registered |
| Failed | Generate Portfolio | Repository synchronization incomplete where mandatory |
| Registered | Delete Registration | Historical registration retained |

---

## 33.6.3 Repository Lifecycle Diagram

```text
         Pending
        /       \
       ▼         ▼
 Registered    Failed
       ▲         │
       └─────────┘
```

---

# 33.7 Repository & Portfolio Business Rules

| Rule ID | Rule |
|----------|------|
| RP-01 | Repository registration occurs only after eligible research outputs exist. |
| RP-02 | Repository synchronization is asynchronous. |
| RP-03 | Research Portfolio is a derived read model. |
| RP-04 | Portfolio shall never become the system of record. |
| RP-05 | KPI calculations operate only on read models. |
| RP-06 | Reporting shall never directly access transactional aggregates. |
| RP-07 | Repository metadata shall remain immutable after synchronization except through governed update workflows. |
| RP-08 | Every successful synchronization publishes a domain event. |
| RP-09 | Portfolio rebuild shall be supported through event replay. |
| RP-10 | Repository integrations shall use secure authenticated APIs. |
# 33.8 Repository & Portfolio Validation Rules

| Validation | Description |
|------------|-------------|
| Repository Validation | Repository shall be configured and active. |
| Metadata Validation | Mandatory metadata fields shall be complete. |
| Publication Validation | Referenced publication shall exist where applicable. |
| Patent Validation | Referenced patent shall exist where applicable. |
| Portfolio Validation | Referenced researcher and institution shall exist. |
| KPI Validation | KPI definitions shall be configured before calculation. |
| Report Validation | Requested report parameters shall be valid. |
| Synchronization Validation | Repository endpoint shall be reachable before synchronization. |

---

# 33.9 Events Published by Repository & Portfolio Aggregates

| Event | Description |
|--------|-------------|
| RepositoryMetadataRegistered | Repository metadata registered |
| RepositorySynchronizationCompleted | Repository synchronization successful |
| RepositorySynchronizationFailed | Repository synchronization failed |
| ResearchPortfolioUpdated | Portfolio refreshed |
| ResearchKPIsGenerated | KPI snapshot generated |
| ResearchReportGenerated | Research report generated |

---

# 33.10 Events Consumed by Repository & Portfolio Aggregates

| Event | Source |
|--------|--------|
| ResearchOutputPublished | Publication Aggregate |
| PatentFiled | Patent Aggregate |
| PatentStatusUpdated | Patent Aggregate |
| GrantAwarded | Grant Aggregate |
| FundingUtilizationRecorded | Grant Aggregate |
| ConsultancyProjectCompleted | Consultancy Aggregate |
| ResearchProjectCompleted | Research Project Aggregate |
| ResearchProjectClosed | Research Project Aggregate |

---

# 33.11 Aggregate Consistency Rules

- Repository registrations reference immutable research output identifiers.
- Repository synchronization never modifies source aggregates.
- Portfolio data is derived exclusively from domain events.
- KPI snapshots are immutable after generation.
- Report generation is a read-only operation.
- Aggregate versions increment after successful repository updates.
- Event replay shall be supported for complete portfolio reconstruction.
- Repository failures shall not impact transactional research workflows.

---

# 34. Commands Catalogue

The Research Management domain follows the Command Query Responsibility Segregation (CQRS) architectural pattern.

Commands express business intent, mutate aggregate state, execute validations, enforce business rules, and publish immutable domain events upon successful completion.

Commands are always executed against a single aggregate boundary.

---

# 34.1 Proposal Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| CreateResearchProposal | Research Proposal | Create draft proposal |
| UpdateResearchProposal | Research Proposal | Update proposal details |
| SubmitResearchProposal | Research Proposal | Submit proposal for review |
| ReviewResearchProposal | Research Proposal | Record proposal review |
| ApproveResearchProposal | Research Proposal | Approve proposal |
| RejectResearchProposal | Research Proposal | Reject proposal |
| RequestProposalRevision | Research Proposal | Return proposal for revision |
| WithdrawResearchProposal | Research Proposal | Withdraw proposal |

---

# 34.2 Research Project Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| InitiateResearchProject | Research Project | Create project |
| AssignResearchTeam | Research Project | Assign investigators |
| UpdateResearchTeam | Research Project | Modify project team |
| CreateProjectMilestone | Research Project | Add milestone |
| UpdateProjectMilestone | Research Project | Modify milestone |
| SubmitProgressReport | Research Project | Record project progress |
| UpdateProjectStatus | Research Project | Change project status |
| CompleteResearchProject | Research Project | Complete project |
| CloseResearchProject | Research Project | Close project |

---

# 34.3 Grant Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| ApplyResearchGrant | Research Grant | Apply for funding |
| RecordGrantAward | Research Grant | Register awarded grant |
| AllocateResearchFunding | Research Grant | Allocate project funding |
| RecordFundingUtilization | Research Grant | Record utilization |
| CloseResearchGrant | Research Grant | Close grant |

---

# 34.4 Ethics Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| SubmitEthicsApplication | Ethics Approval | Submit ethics request |
| ReviewEthicsApplication | Ethics Approval | Record committee review |
| ApproveEthicsRequest | Ethics Approval | Grant ethics approval |
| RejectEthicsRequest | Ethics Approval | Reject ethics request |
| RenewEthicsApproval | Ethics Approval | Renew approval |
| ExpireEthicsApproval | Ethics Approval | Expire approval |

---

# 34.5 Publication Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| RegisterPublication | Publication | Register publication |
| VerifyPublication | Publication | Verify publication |
| PublishResearchOutput | Publication | Mark publication published |
| ArchivePublication | Publication | Archive publication |

---

# 34.6 Intellectual Property Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| RegisterPatent | Patent | Register patent |
| SubmitPatentFiling | Patent | Submit filing |
| UpdatePatentStatus | Patent | Update patent status |
| ClosePatentLifecycle | Patent | Close patent |

---

# 34.7 Collaboration Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| CreateCollaboration | Collaboration | Register collaboration |
| ApproveCollaboration | Collaboration | Approve collaboration |
| RecordCollaborationActivity | Collaboration | Record activity |
| CompleteCollaboration | Collaboration | Complete collaboration |

---

# 34.8 Consultancy Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| RegisterConsultancyProject | Consultancy Project | Register consultancy |
| ExecuteConsultancyProject | Consultancy Project | Record consultancy execution |
| CompleteConsultancyProject | Consultancy Project | Complete consultancy |

---

# 34.9 Repository & Portfolio Commands

| Command | Aggregate | Purpose |
|----------|-----------|---------|
| RegisterRepositoryMetadata | Repository Registration | Register repository metadata |
| SynchronizeRepository | Repository Registration | Synchronize repositories |
| RebuildResearchPortfolio | Research Portfolio | Replay events and rebuild portfolio |
| GenerateResearchKPIs | Research Portfolio | Generate KPI snapshot |
| GenerateResearchReports | Research Portfolio | Generate operational reports |
# 35. Domain Events Catalogue

Research Management is an event-driven domain.

Every successful command produces one immutable domain event. Events represent facts that have already occurred and are consumed by downstream bounded contexts.

Events shall never be modified after publication.

---

# 35.1 Proposal Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| ResearchProposalCreated | CreateResearchProposal | Proposal created | Workflow Engine |
| ResearchProposalUpdated | UpdateResearchProposal | Proposal modified | Workflow Engine |
| ResearchProposalSubmitted | SubmitResearchProposal | Proposal submitted | Approval Engine |
| ResearchProposalReviewed | ReviewResearchProposal | Proposal reviewed | Approval Engine |
| ResearchProposalApproved | ApproveResearchProposal | Proposal approved | Research Project Aggregate |
| ResearchProposalRejected | RejectResearchProposal | Proposal rejected | Notifications |
| ProposalRevisionRequested | RequestProposalRevision | Revision requested | Notifications |
| ResearchProposalWithdrawn | WithdrawResearchProposal | Proposal withdrawn | Reporting |

---

# 35.2 Research Project Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| ResearchProjectCreated | InitiateResearchProject | Project created | Finance, Collaboration |
| ResearchTeamAssigned | AssignResearchTeam | Team assigned | HR, Notifications |
| ResearchTeamUpdated | UpdateResearchTeam | Team updated | HR |
| ProjectMilestoneCreated | CreateProjectMilestone | Milestone created | Dashboard |
| ProjectMilestoneUpdated | UpdateProjectMilestone | Milestone updated | Dashboard |
| ResearchProgressReported | SubmitProgressReport | Progress submitted | KPI Engine |
| ResearchProjectStatusUpdated | UpdateProjectStatus | Status updated | Dashboard |
| ResearchProjectCompleted | CompleteResearchProject | Project completed | Publication, Repository |
| ResearchProjectClosed | CloseResearchProject | Project closed | Reporting |

---

# 35.3 Grant Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| GrantApplicationSubmitted | ApplyResearchGrant | Grant application submitted | Finance |
| GrantAwarded | RecordGrantAward | Grant awarded | Finance, Project |
| FundingAllocated | AllocateResearchFunding | Funding allocated | Finance |
| FundingUtilizationRecorded | RecordFundingUtilization | Utilization updated | Finance Dashboard |
| ResearchGrantClosed | CloseResearchGrant | Grant closed | Reporting |

---

# 35.4 Ethics Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| EthicsApplicationSubmitted | SubmitEthicsApplication | Ethics submitted | Ethics Committee |
| EthicsApplicationReviewed | ReviewEthicsApplication | Review completed | Approval Engine |
| EthicsApprovalGranted | ApproveEthicsRequest | Approval granted | Project Aggregate |
| EthicsApprovalRejected | RejectEthicsRequest | Approval rejected | Notifications |
| EthicsApprovalRenewed | RenewEthicsApproval | Approval renewed | Dashboard |
| EthicsApprovalExpired | ExpireEthicsApproval | Approval expired | Notifications |

---

# 35.5 Publication Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| PublicationRegistered | RegisterPublication | Publication registered | Repository |
| PublicationVerified | VerifyPublication | Publication verified | Repository |
| ResearchOutputPublished | PublishResearchOutput | Research output published | Repository, Portfolio |
| PublicationArchived | ArchivePublication | Publication archived | Reporting |

---

# 35.6 Intellectual Property Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| PatentRegistered | RegisterPatent | Patent registered | Repository |
| PatentFiled | SubmitPatentFiling | Patent filed | Repository |
| PatentStatusUpdated | UpdatePatentStatus | Patent status changed | Portfolio |
| PatentLifecycleClosed | ClosePatentLifecycle | Patent closed | Reporting |

---

# 35.7 Collaboration Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| CollaborationCreated | CreateCollaboration | Collaboration created | Dashboard |
| CollaborationApproved | ApproveCollaboration | Collaboration activated | Project Aggregate |
| CollaborationActivityRecorded | RecordCollaborationActivity | Activity recorded | Dashboard |
| CollaborationCompleted | CompleteCollaboration | Collaboration completed | Reporting |

---

# 35.8 Consultancy Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| ConsultancyProjectRegistered | RegisterConsultancyProject | Consultancy registered | Finance |
| ConsultancyProjectProgressUpdated | ExecuteConsultancyProject | Progress updated | Dashboard |
| ConsultancyProjectCompleted | CompleteConsultancyProject | Consultancy completed | Finance, Portfolio |

---

# 35.9 Repository & Portfolio Events

| Event | Triggering Command | Description | Primary Consumers |
|--------|-------------------|-------------|-------------------|
| RepositoryMetadataRegistered | RegisterRepositoryMetadata | Metadata registered | Integration Layer |
| RepositorySynchronizationCompleted | SynchronizeRepository | Repository synchronized | Portfolio |
| RepositorySynchronizationFailed | SynchronizeRepository | Synchronization failed | Monitoring |
| ResearchPortfolioUpdated | RebuildResearchPortfolio | Portfolio rebuilt | Dashboard |
| ResearchKPIsGenerated | GenerateResearchKPIs | KPI snapshot generated | Enterprise Dashboard |
| ResearchReportGenerated | GenerateResearchReports | Report produced | Management |

---

# 35.10 Faculty Workforce Integration Events

Research Management publishes the following events to L2-E03 Faculty Workforce.

| Event | Description |
|-------|-------------|
| ResearchParticipationStarted | Communicates that a faculty member has started participating in a research activity and workload projection must reflect the active participation. |
| ResearchParticipationEnded | Communicates that a faculty member has ended participation in a research activity and workload projection may be reduced. |
| ResearchTeamMemberAssigned | Communicates assignment of a faculty member to a research team and that workforce availability must be recalculated. |
| ResearchTeamMemberRemoved | Communicates removal of a faculty member from a research team and that workforce availability may be released. |
| PrincipalInvestigatorAssigned | Communicates PI assignment for a research activity and that ownership-related workload projection must be updated. |
| CoInvestigatorAssigned | Communicates co-investigator assignment for a research activity and that workforce projection must include the assigned participant. |
| ResearchWorkloadConsumptionRecorded | Communicates the amount of research workload consumed for workforce planning and capacity projection. |
| ResearchCapacityReleased | Communicates that research participation capacity has been released after a role change or completion. |

Faculty Workforce consumes these events to update workload projections, availability, and capacity.
Research Management never updates faculty workload directly.

---

# 36. CQRS Read Models

The Research Management bounded context exposes optimized read models for dashboards, reporting, analytics, operational monitoring, and external integrations.

Read models are generated asynchronously from domain events and never participate in transactional command execution.

---

# 36.1 Research Proposal Read Model

## Purpose

Provides searchable proposal information.

## Primary Consumers

- Research Office
- Principal Investigators
- Department Heads
- Management Dashboard

## Fields

- Proposal ID
- Proposal Title
- Research Type
- Principal Investigator
- Department
- Funding Type
- Proposal Status
- Submission Date
- Approval Date
- Current Approver
- Funding Agency
- Proposal Score
- Last Updated

---

# 36.2 Research Project Read Model

## Purpose

Provides operational visibility of research projects.

## Fields

- Project ID
- Project Title
- PI
- Co-PI List
- Department
- Funding Agency
- Budget
- Start Date
- End Date
- Current Status
- Milestone Completion %
- Active Team Members
- Publications Produced
- Patents Produced
- Last Progress Date

---

# 36.3 Grant Read Model

## Fields

- Grant ID
- Funding Agency
- Grant Type
- Award Amount
- Currency
- Principal Investigator
- Project
- Grant Status
- Budget Utilized
- Remaining Budget
- Start Date
- End Date

---

# 36.4 Ethics Read Model

## Fields

- Ethics Approval ID
- Proposal
- Committee
- Approval Status
- Approval Date
- Expiry Date
- Renewal Status
- Current Review Stage

---

# 36.5 Publication Read Model

## Fields

- Publication ID
- Title
- Publication Type
- Authors
- Journal
- Conference
- DOI
- Publication Date
- Verification Status
- Citation Count
- Repository Status

---

# 36.6 Patent Read Model

## Fields

- Patent ID
- Patent Title
- Inventors
- Filing Number
- Filing Authority
- Filing Date
- Current Status
- Grant Date
- Ownership Policy
- Repository Status

---

# 36.7 Collaboration Read Model

## Fields

- Collaboration ID
- Partner Organization
- Collaboration Type
- Start Date
- End Date
- Current Status
- Active Projects
- Principal Investigator
- Agreement Status

---

# 36.8 Consultancy Read Model

## Fields

- Consultancy ID
- Client Organization
- Consultancy Type
- Principal Investigator
- Revenue
- Current Status
- Deliverables Completed
- Completion Percentage
- End Date

---

# 36.9 Research Portfolio Read Model

## Fields

- Researcher ID
- Publications
- Citations
- H-Index
- Grants Awarded
- Total Funding
- Patents
- Consultancy Projects
- Active Research Projects
- Collaborations
- Research Score
- Last Updated
# 36.10 Repository Read Model

## Fields

- Repository Record ID
- Research Output ID
- Repository Type
- Repository Name
- Repository Identifier
- DOI
- External Repository Identifier
- Synchronization Status
- Synchronization Date
- Repository URL
- Last Synchronization Timestamp

---

# 36.11 Research KPI Read Model

## Fields

- Institution
- Academic Year
- Publications
- Journal Publications
- Conference Publications
- Books
- Book Chapters
- Patents Filed
- Patents Granted
- Copyrights
- Software Registrations
- Datasets Published
- Active Research Projects
- Completed Research Projects
- Total Research Funding
- Consultancy Revenue
- Citation Count
- Institutional H-Index
- International Collaborations
- Industry Collaborations
- Government Sponsored Projects
- Faculty Participation %
- Student Participation %
- Last Calculated Timestamp

---

# 37. External Integrations

The Research Management domain exchanges information with multiple internal and external enterprise systems.

All integrations follow asynchronous event-driven architecture wherever possible.

Research Management never owns external master data.

---

# 37.1 Internal Enterprise Integrations

| Domain | Integration Purpose | Direction |
|---------|--------------------|-----------|
| People & Workforce | Faculty, Researchers, Staff, PhD Scholars | Consume |
| Student Lifecycle | Student Researchers | Consume |
| Academic Structure | Departments, Schools, Centers | Consume |
| L2-E03 Faculty Workforce | Faculty Profile, Faculty Status, Faculty Capability, Research Capability, Faculty Availability, Faculty Capacity, Faculty Eligibility, Organizational Assignment | Consume |
| Finance | Grants, Budget, Utilization, Consultancy Revenue | Bi-directional |
| Document Management | Agreements, Publications, Ethics Documents, Contracts | Consume |
| Notification | Workflow Notifications | Publish |
| Enterprise Dashboard | KPIs | Publish |
| Enterprise Search | Research Metadata | Publish |
| Identity & Access | Authentication and Authorization | Consume |

Finance remains the authoritative owner of budgets, accounting, ledgers, payment processing, procurement, taxation, and financial compliance. Research Management exchanges grant and consultancy metadata with Finance without assuming those responsibilities.
Document Management remains the owner of binary files and managed document content. Research Management exchanges only document identifiers and metadata references.

## 37.1.1 Integration with L2-E03 Faculty Workforce

Research Management SHALL consume the following faculty workforce data from L2-E03 Faculty Workforce:

- Faculty Profile
- Faculty Status
- Faculty Capability
- Research Capability
- Faculty Availability
- Faculty Capacity
- Teaching / Research Eligibility
- Academic Rank
- Organizational Assignment

### APIs Consumed

- GetFacultyProfile
- GetFacultyCapability
- GetResearchCapability
- GetFacultyAvailability
- GetFacultyCapacity
- CheckResearchEligibility

### Events Consumed

- FacultyCapabilityUpdated
- FacultyAvailabilityChanged
- FacultyCapacityUpdated
- ResearchCapabilityUpdated
- FacultyTransferred
- FacultySuspended
- FacultyReactivated

### Ownership Boundary

L2-E03 Faculty Workforce remains the authoritative owner of faculty profile, faculty status, capability, capacity, availability, eligibility, and workforce decisions.

Research Management SHALL NEVER calculate workload, maintain faculty capability, determine availability, modify faculty profile, or override faculty eligibility.

Research Management SHALL only consume these decisions for research participation validation and research coordination.

## 37.1.2 Events Published to Faculty Workforce

Research Management publishes the following events so L2-E03 Faculty Workforce can update workload projections, availability, and capacity asynchronously:

- ResearchParticipationStarted
- ResearchParticipationEnded
- ResearchTeamMemberAssigned
- ResearchTeamMemberRemoved
- PrincipalInvestigatorAssigned
- CoInvestigatorAssigned
- ResearchWorkloadConsumptionRecorded
- ResearchCapacityReleased

Faculty Workforce consumes these events to update workload projections, availability, and capacity.
Research Management never updates faculty workload directly.

---

# 37.2 External Integrations

| External System | Purpose |
|-----------------|---------|
| ORCID | Researcher identity synchronization |
| Scopus | Publication indexing |
| Web of Science | Citation indexing |
| CrossRef | DOI verification |
| Google Scholar | Citation aggregation |
| Government Funding Portals | Grant synchronization |
| CSR Funding Portals | Funding integration |
| International Funding Agencies | Grant management |
| Patent Office Systems | Patent lifecycle synchronization |
| Institutional Repositories | Repository synchronization |

---

# 37.3 Integration Principles

- External systems remain the authoritative source for externally owned information.
- Synchronization is asynchronous unless real-time validation is mandatory.
- Integration failures shall never block transactional workflows.
- Retry mechanisms shall follow enterprise integration policies.
- Every integration transaction shall be auditable.
- Authentication shall follow enterprise security standards.
- Personally identifiable information shall only be exchanged when authorized.
- All integrations shall support versioned APIs.

---

# 38. Configuration Matrix

All business behavior that differs across universities shall be configurable through master configuration rather than code customization.

---

# 38.1 Institution Configuration

| Configuration | Description |
|--------------|-------------|
| Research Types | Institution-defined research categories |
| Proposal Approval Workflow | Single or multi-level approval |
| Ethics Requirement | Mandatory or optional by research type |
| Funding Types | Institution-configurable funding sources |
| Consultancy Types | Institution-defined consultancy categories |
| Publication Types | Institution-supported publication categories |
| Patent Authorities | Supported patent offices |
| Collaboration Types | Institutional collaboration classifications |
| Repository Types | Supported repository platforms |
| KPI Definitions | Institution-specific KPI formulas |

---

# 38.2 Workflow Configuration

| Configuration | Description |
|--------------|-------------|
| Proposal Approval Matrix | Configurable approval routing |
| Ethics Committee Workflow | Institution-specific review workflow |
| Grant Approval Workflow | Funding approval routing |
| Publication Verification Workflow | Verification rules |
| Patent Approval Workflow | IP governance workflow |
| Collaboration Approval Workflow | Institutional collaboration approvals |
| Consultancy Approval Workflow | Consultancy authorization workflow |
| Repository Synchronization Schedule | Batch synchronization frequency |

---

# 38.3 Integration Configuration

| Configuration | Description |
|--------------|-------------|
| ORCID API Configuration | API credentials and endpoints |
| Scopus Configuration | API integration settings |
| CrossRef Configuration | DOI verification configuration |
| Patent Office Configuration | Supported patent office APIs |
| Repository Configuration | Repository endpoints |
| Funding Portal Configuration | External funding APIs |
| Notification Templates | Event notification templates |

---

# 39. Security Model

Research Management processes confidential research, funding, intellectual property, and collaboration information.

Access control shall follow enterprise Role-Based Access Control (RBAC) with fine-grained authorization.

---

# 39.1 Security Principles

- Least privilege access.
- Role-based authorization.
- Department-aware visibility.
- Institution-aware segregation.
- Immutable audit history.
- Secure API authentication.
- Encrypted integration channels.
- Sensitive research data masking where required.
- Complete traceability for every state transition.

---

# 39.2 Major Roles

| Role | Responsibilities |
|------|------------------|
| Research Administrator | Domain administration |
| Research Director | Strategic approvals |
| Principal Investigator | Research project ownership |
| Co-Principal Investigator | Project participation |
| Research Staff | Operational activities |
| Ethics Committee Member | Ethics reviews |
| Finance Officer | Grant financial monitoring |
| Department Head | Department oversight |
| Institutional Administrator | Enterprise governance |
| System Integration Service | Automated synchronization |

---

# 39.3 Permission Groups

## Proposal Management

- CreateResearchProposal
- UpdateResearchProposal
- SubmitResearchProposal
- ReviewResearchProposal
- ApproveResearchProposal
- RejectResearchProposal
- WithdrawResearchProposal

## Project Management

- InitiateResearchProject
- AssignResearchTeam
- ManageProjectMilestones
- SubmitProgressReport
- CompleteResearchProject
- CloseResearchProject

## Grant Management

- ApplyResearchGrant
- AwardGrant
- AllocateFunding
- RecordFundingUtilization
- CloseGrant

## Ethics Management

- SubmitEthicsApplication
- ReviewEthicsApplication
- ApproveEthicsRequest
- RenewEthicsApproval

## Publication Management

- RegisterPublication
- VerifyPublication
- PublishResearchOutput
- ArchivePublication

## Intellectual Property Management

- RegisterPatent
- SubmitPatentFiling
- UpdatePatentStatus
- ClosePatentLifecycle

## Collaboration Management

- CreateCollaboration
- ApproveCollaboration
- RecordCollaborationActivity
- CompleteCollaboration

## Consultancy Management

- RegisterConsultancyProject
- ExecuteConsultancyProject
- CompleteConsultancyProject

## Repository & Analytics

- RegisterRepositoryMetadata
- SynchronizeRepository
- GenerateResearchKPIs
- GenerateResearchReports
- ViewResearchPortfolio

# 40. Audit & Compliance

Research Management maintains complete, immutable audit trails for every business transaction to support institutional governance, accreditation, statutory reporting, funding audits, and intellectual property compliance.

All audit records are append-only and retained according to institutional retention policies.

---

# 40.1 Audit Principles

- Every successful command generates an audit record.
- Every state transition is traceable.
- Historical values are never overwritten.
- User identity shall be captured for every manual action.
- System-generated actions shall record the originating service.
- Every integration transaction shall be auditable.
- Audit history shall remain immutable.
- Deleted business records shall never remove audit history.
- Event replay shall reproduce historical state accurately.

---

# 40.2 Audit Information Captured

| Audit Item | Description |
|------------|-------------|
| Audit Identifier | Unique audit record identifier |
| Aggregate Identifier | Aggregate instance identifier |
| Aggregate Type | Proposal, Project, Grant, Publication, Patent, etc. |
| Command Executed | Business command executed |
| Event Published | Resulting domain event |
| Previous State | Aggregate state before execution |
| Current State | Aggregate state after execution |
| Executed By | User or system |
| Execution Timestamp | Date and time |
| Source Channel | Web, Mobile, API, Integration |
| Correlation Identifier | Enterprise correlation identifier |
| Transaction Identifier | Distributed transaction identifier |

---

# 40.3 Compliance Coverage

| Compliance Area | Coverage |
|-----------------|----------|
| Research Governance | Complete |
| Funding Agency Reporting | Complete |
| Ethics Committee Compliance | Complete |
| Publication Compliance | Complete |
| Intellectual Property Compliance | Complete |
| Consultancy Compliance | Complete |
| Institutional Audit | Complete |
| Accreditation Reporting | Complete |

---

# 41. Reporting Catalogue

The Research Management domain provides operational, analytical, executive, and statutory reporting.

Reports consume CQRS read models only.

---

# 41.1 Operational Reports

| Report | Description |
|---------|-------------|
| Active Research Projects | Current projects |
| Proposal Pipeline | Proposal lifecycle |
| Grant Status Report | Grant monitoring |
| Ethics Pending Cases | Pending ethics approvals |
| Publication Verification Queue | Publications awaiting verification |
| Patent Status Report | Patent lifecycle |
| Consultancy Status Report | Consultancy monitoring |
| Collaboration Status Report | Active collaborations |

---

# 41.2 Executive Reports

| Report | Description |
|---------|-------------|
| Institutional Research Dashboard | Overall research performance |
| Funding Summary | Funding received |
| Publication Analytics | Publications by period |
| Citation Analytics | Citation statistics |
| Patent Portfolio | Institutional IP portfolio |
| Consultancy Revenue | Consultancy analytics |
| Research KPI Dashboard | Enterprise KPIs |
| Department Research Ranking | Department comparison |

---

# 41.3 Statutory Reports

| Report | Description |
|---------|-------------|
| Government Funding Reports | Funding utilization |
| Accreditation Research Reports | NAAC/NBA/Institutional reporting |
| Annual Research Report | Institutional annual report |
| Intellectual Property Report | Patent statistics |
| Ethics Committee Report | Ethics compliance |
| Sponsored Research Report | Sponsored projects |
| Consultancy Report | Consultancy summary |

---

# 42. Enterprise Search Metadata

Research Management publishes searchable metadata to the Enterprise Search bounded context.

Only metadata is indexed.

Research documents remain managed by the Enterprise Document Management domain.

---

# 42.1 Searchable Entities

| Entity | Search Attributes |
|---------|------------------|
| Research Proposal | Title, PI, Department, Keywords |
| Research Project | Project Code, Title, PI |
| Grant | Grant Number, Funding Agency |
| Publication | Title, DOI, Authors |
| Patent | Patent Number, Inventors |
| Collaboration | Partner Organization |
| Consultancy | Client Organization |
| Researcher Portfolio | Researcher Name |

---

# 43. Enterprise Dashboard Metrics

The Research Management domain continuously publishes research metrics to the Enterprise Dashboard bounded context.

Metrics are generated asynchronously through event-driven processing.

---

# 43.1 Institutional KPIs

| KPI | Description |
|-----|-------------|
| Total Research Projects | Active and completed projects |
| Active Projects | Currently active |
| Completed Projects | Successfully completed |
| Proposal Approval Rate | Proposal success percentage |
| Total Funding Received | Institutional funding |
| Funding Utilization | Utilized funding |
| Publications | Total publications |
| Journal Publications | Journal count |
| Conference Publications | Conference count |
| Patents Filed | Filed patents |
| Patents Granted | Granted patents |
| Consultancy Revenue | Total consultancy income |
| Active Collaborations | Institutional collaborations |
| Citation Count | Institutional citations |
| Institutional H-Index | H-index |
| Repository Synchronization Rate | Repository success percentage |

---

# 43.2 Department KPIs

| KPI | Description |
|-----|-------------|
| Department Projects |
| Department Publications |
| Department Funding |
| Department Citations |
| Department Patents |
| Department Consultancy |
| Department Collaborations |
| Faculty Participation Rate |

---

# 43.3 Faculty KPIs

| KPI | Description |
|-----|-------------|
| Publications |
| Citations |
| H-Index |
| Research Projects |
| Grants |
| Consultancy |
| Patents |
| Research Score |

---

# 44. Non-Functional Requirements

| Category | Requirement |
|----------|-------------|
| Availability | Enterprise high availability |
| Scalability | Horizontal scalability |
| Performance | CQRS optimized reads |
| Consistency | Eventual consistency for read models |
| Reliability | Guaranteed event delivery |
| Security | Enterprise RBAC |
| Auditability | Complete immutable audit history |
| Observability | Centralized monitoring and tracing |
| Extensibility | Configuration-first architecture |
| Integration | API-first and event-driven |
| Disaster Recovery | Enterprise backup and recovery policies |
| Multi-Tenancy | Institution-aware logical isolation |

---

# 45. Architectural Constraints

- Research Management owns only research-domain transactional data.
- Documents are owned exclusively by the Enterprise Document Management domain.
- Finance remains the authoritative owner of financial accounting.
- Employee master data is owned by People & Workforce.
- Student master data is owned by Student Lifecycle.
- Authentication and authorization are owned by Identity & Access Management.
- Dashboards consume CQRS read models only.
- Cross-domain communication occurs through immutable domain events.
- External systems are never updated synchronously unless explicitly required by institutional policy.
- Aggregate boundaries shall never be violated by direct database access.
- Every state mutation shall occur only through validated commands.
- Every successful command shall publish exactly one business domain event.
- Read models shall never be updated directly.
- Repository synchronization follows eventual consistency.
- Historical research records shall never be physically deleted.

---

# 46. Section Summary

The Research Management bounded context provides complete lifecycle management for:

- Research Proposals
- Research Projects
- Research Grants
- Ethics Management
- Publications
- Intellectual Property
- Research Collaborations
- Consultancy Projects
- Repository Management
- Research Portfolio
- Institutional Research Analytics

The architecture conforms to the enterprise standards defined for SutraERP:

- Domain-Driven Design (DDD)
- Command Query Responsibility Segregation (CQRS)
- Event-Driven Architecture (EDA)
- Immutable Domain Events
- Aggregate-Centric Consistency
- Enterprise Configuration Framework
- Role-Based Access Control (RBAC)
- Eventual Consistency
- API-First Integration
- Enterprise Audit & Compliance
- Read-Optimized Reporting Architecture
- Multi-Institution Enterprise Deployment
