# L2-G08 — Housekeeping Operations & Service Governance Workflows

**Repository:** SutraERP  
**Section:** G — Campus Operations  
**Domain:** G08 — Housekeeping Operations & Service Governance  
**Document ID:** L2-G08  
**Version:** 1.0  
**Status:** Architecture Draft  
**Classification:** Level 2 Domain Architecture  
**Owner:** Campus Operations Domain

**Upstream Dependencies**

- Section A — Platform Foundation
- Section B — Institution & Academic Structure
- G01 — Facilities Management
- G02 — Laboratory Management
- G03 — Hostel Management
- G06 — Institutional Health Services
- Finance Domain

**Downstream Consumers**

- Student Portal
- Employee Portal
- Institutional Reporting
- Notification Services
- Analytics
- Audit

---

# Table of Contents

1. Purpose
2. Architectural Position
3. Scope
4. Out of Scope
5. Domain Responsibilities
6. Design Principles
7. Domain Terminology
8. Aggregate Design
9. Bounded Context Relationships
10. Domain Model
11. Lifecycle Overview
12. Workflow Catalogue
13. Business Invariants
14. Architectural Constraints
15. Repository Principles
16. Housekeeping Zone Management
17. Cleaning Operations
18. Cleaning Requests & Complaint Management
19. Hygiene Inspection & Waste Management
20. Vendor Service Governance
21. Notifications
22. Domain Events
23. Service Contracts
24. State Transition Matrix
25. Authorization Model
26. Reporting
27. Configuration
28. Audit
29. Exception Handling
30. Performance & Scalability
31. Glossary

---

# 1. Purpose

The Housekeeping Operations & Service Governance domain governs operational housekeeping services throughout institutional campuses.

The domain manages housekeeping zones, cleaning schedules, cleaning requests, hygiene inspections, washroom management, waste collection, consumable monitoring, service quality, SLA monitoring, vendor operational coordination, and housekeeping reporting.

The architecture is intentionally designed to support institutions operating with:

- Internal Housekeeping Teams
- Outsourced Housekeeping Agencies
- Hybrid Housekeeping Operations

through enterprise configuration without architectural redesign.

Housekeeping Operations governs service delivery.

It does **not** own housekeeping employees, vendor contracts, payroll, procurement, maintenance, or facility assets.

Finance owns vendor payments and contracts.

Section E owns employee identities.

G01 owns maintenance and physical assets.

This document defines:

- Housekeeping Zones
- Cleaning Schedules
- Cleaning Requests
- Complaint Management
- Hygiene Inspections
- Washroom Operations
- Waste Collection
- Consumable Monitoring
- Service Quality
- SLA Monitoring
- Vendor Operational Coordination
- Operational Reporting
- Domain Events
- Integration Contracts

Implementation technologies, IoT platforms, robotics, procurement systems, APIs, databases, deployment architecture, and UI behaviour remain outside the scope of this document.

---

# 2. Architectural Position

Housekeeping Operations represents the operational housekeeping layer within SutraERP.

Repository Position

```text
Platform Foundation
        │
        ▼
Campus Operations
        │
        ▼
Housekeeping Operations
        │
 ┌────────────┼──────────────┬──────────────┐
 ▼            ▼              ▼
Facilities  Notifications  Institutional Reporting
```

Housekeeping Operations governs operational service delivery.

It does **not** replace:

- Vendor Management
- Procurement Systems
- HR Systems
- Facility Maintenance
- Asset Management
- Building Management Systems

---

# 3. Scope

Housekeeping Operations owns:

- Housekeeping Zones
- Cleaning Schedules
- Cleaning Requests
- Complaint Management
- Cleaning Activities
- Deep Cleaning
- Event Cleaning
- Washroom Operations
- Hygiene Inspections
- Quality Inspections
- Waste Collection
- Consumable Monitoring
- Housekeeping Incidents
- SLA Monitoring
- Service Quality
- Operational Dashboards
- Operational Audit

The domain serves as the canonical operational housekeeping engine for the institution.

---

# 4. Out of Scope

The following responsibilities remain outside this domain.

---

## Housekeeping Personnel Management

Owned by Section E or outsourced housekeeping agencies.

Examples:

- Recruitment
- Leave
- Attendance
- Payroll
- Performance Reviews
- Workforce Planning

Housekeeping Operations performs operational assignments only.

---

## Vendor Contract Management

Owned outside this domain.

Examples:

- Vendor Contracts
- SLA Commercial Agreements
- Vendor Billing
- Contract Renewals

Housekeeping Operations references vendors operationally.

---

## Maintenance Management

Owned by G01.

Examples:

- Electrical Repairs
- Plumbing
- Civil Maintenance
- Carpentry
- Painting
- Equipment Repair

Housekeeping Operations may publish maintenance requests.

---

## Procurement & Inventory

Owned by Finance / Procurement.

Examples:

- Purchase Orders
- Vendor Procurement
- Inventory Purchasing
- Consumable Procurement

Housekeeping Operations only monitors operational availability.

---

## Authentication & Workflow

Authentication, authorization, workflow execution, notifications, audit infrastructure, and document management remain Platform responsibilities.

---

# 5. Domain Responsibilities

Housekeeping Operations owns:

- Zone Management
- Cleaning Operations
- Complaint-Based Cleaning
- Hygiene Inspection
- Washroom Inspection
- Waste Collection
- Consumable Monitoring
- Service Quality
- SLA Monitoring
- Vendor Operational Coordination
- Operational Dashboards
- Operational Audit

The domain maintains complete housekeeping operational history.

---

# 6. Design Principles

## Operational Ownership

Housekeeping Operations owns housekeeping service delivery.

Personnel ownership remains external.

---

## Configuration over Customisation

Cleaning schedules, cleaning frequencies, inspection policies, complaint workflows, SLA rules, hygiene standards, and notification templates shall remain configuration driven.

Institutions with outsourced housekeeping shall require no architectural modification.

---

## Manual First Operations

Housekeeping activities remain operationally human-driven.

The ERP records, coordinates, governs, and audits housekeeping operations without replacing field execution.

---

## Event Driven Architecture

Every significant housekeeping activity shall publish immutable domain events.

Downstream systems consume events without modifying housekeeping state.

---

## Audit First

Every housekeeping operation shall produce immutable audit history.

Operational housekeeping history shall remain permanently traceable.

---

## Enterprise Scalability

The architecture shall support:

- Multi Institution
- Multi Campus
- Internal Housekeeping
- Outsourced Housekeeping
- Hybrid Operations
- Future IoT Integration
- Future Smart Washrooms
- Future Robotic Cleaning
- Future AI Cleaning Optimisation

without architectural redesign.

# 7. Domain Terminology

| Term | Definition |
|------|------------|
| Housekeeping Zone | Operational area assigned for housekeeping planning, scheduling, monitoring, and governance. |
| Cleaning Schedule | Planned recurring housekeeping activity executed according to institutional policies. |
| Cleaning Request | Operational request generated due to complaints, inspections, or ad-hoc requirements. |
| Cleaning Task | Individual housekeeping activity assigned for execution. |
| Hygiene Inspection | Operational verification of cleanliness and hygiene standards. |
| Washroom Inspection | Inspection of washroom cleanliness, consumables, and operational condition. |
| Housekeeping Incident | Operational issue observed during housekeeping activities requiring attention. |
| Service Level Agreement (SLA) | Configured operational service expectations for housekeeping delivery. |
| Quality Inspection | Verification of housekeeping work quality before operational closure. |
| Consumable Monitoring | Operational monitoring of housekeeping consumables without procurement ownership. |
| Waste Collection | Operational collection and movement of institutional waste. |
| Vendor Reference | Operational reference to outsourced housekeeping service provider. |

---

# 8. Aggregate Design

## Aggregate Roots

Housekeeping Operations consists of the following aggregate roots:

- Housekeeping Zone
- Cleaning Schedule
- Cleaning Request
- Cleaning Task
- Hygiene Inspection
- Waste Collection
- Housekeeping Incident
- Service Quality Assessment

Each aggregate owns its complete operational lifecycle.

---

## Aggregate Responsibilities

### Housekeeping Zone

Owns:

- Zone Information
- Zone Hierarchy
- Assigned Cleaning Policies
- Operational Status

---

### Cleaning Schedule

Owns:

- Cleaning Frequency
- Scheduled Activities
- Cleaning Templates
- Schedule Lifecycle

---

### Cleaning Request

Owns:

- Complaint Registration
- Request Priority
- Request Assignment
- Request Resolution

---

### Cleaning Task

Owns:

- Operational Assignment
- Completion
- Cleaning Checklist
- Execution History

---

### Hygiene Inspection

Owns:

- Inspection Checklist
- Hygiene Score
- Washroom Inspection
- Corrective Actions

---

### Waste Collection

Owns:

- Waste Collection Schedule
- Waste Categories
- Collection History
- Disposal References

Hazardous waste ownership remains external.

---

### Housekeeping Incident

Owns:

- Incident Registration
- Incident Category
- Severity
- Escalation
- Resolution

---

### Service Quality Assessment

Owns:

- Quality Inspection
- SLA Compliance
- Re-clean Requirement
- Quality Score

---

## Aggregate Identity

Every aggregate shall possess:

- Immutable System Identifier
- Configurable Business Code
- Human Readable Reference

Aggregate identity shall remain immutable throughout its lifecycle.

---

# 9. Bounded Context Relationships

## Upstream Domains

### Platform Foundation

Provides:

- Authentication
- Authorization
- Workflow Engine
- Notification Framework
- Audit Framework
- Configuration Framework
- Document Management
- Event Bus

---

### G01 — Facilities Management

Provides:

- Buildings
- Floors
- Rooms
- Facility Assets
- Maintenance Services

Housekeeping Operations consumes facility information.

Maintenance ownership remains external.

---

### G02 — Laboratory Management

Provides:

- Laboratory Information
- Laboratory Availability
- Laboratory Operational Restrictions

Housekeeping Operations consumes laboratory references.

---

### G03 — Hostel Management

Provides:

- Hostel Buildings
- Hostel Floors
- Hostel Rooms
- Hostel Operational Information

Housekeeping Operations coordinates hostel cleaning.

---

### G06 — Institutional Health Services

Provides:

- Biomedical Waste References
- Infection Control Guidance
- Health Alerts

Biomedical waste ownership remains external.

---

### Finance

Provides:

- Vendor References
- Commercial SLA References
- Procurement References

Finance owns vendor contracts and procurement.

---

## Downstream Consumers

Housekeeping Operations publishes operational information to:

### Institutional Reporting

Consumes:

- Cleaning Statistics
- Hygiene Scores
- SLA Reports
- Complaint Reports

---

### Notification Services

Consumes:

- Cleaning Alerts
- Complaint Notifications
- Hygiene Alerts
- Escalation Notifications

---

### Analytics

Consumes:

- Cleaning Performance
- Vendor Performance
- Zone Cleanliness
- Quality Trends

---

# 10. Domain Model

Housekeeping Operations governs operational service delivery throughout the campus.

```text
Housekeeping Zone
        │
Cleaning Schedule
        │
Cleaning Task
        │
Completion
        │
Quality Inspection
        │
Operational Closure
```

Complaint-driven workflow

```text
Complaint
     │
Cleaning Request
     │
Assignment
     │
Execution
     │
Inspection
     │
Closure
```

Supporting operational model

```text
Housekeeping Operations
          │
          ├── Zones
          ├── Cleaning Schedules
          ├── Cleaning Requests
          ├── Hygiene Inspections
          ├── Waste Collection
          ├── Quality Assessment
          ├── SLA Monitoring
          └── Audit
```

---

# 11. Lifecycle Overview

## Cleaning Schedule Lifecycle

```text
Draft
   │
Approved
   │
Active
   │
Executed
   │
Completed
```

---

## Cleaning Request Lifecycle

```text
Reported
     │
Validated
     │
Assigned
     │
Completed
     │
Verified
     │
Closed
```

---

## Hygiene Inspection Lifecycle

```text
Scheduled
     │
Inspection
     │
Evaluation
     │
Corrective Action (Optional)
     │
Closed
```

---

## Waste Collection Lifecycle

```text
Scheduled
     │
Collected
     │
Verified
     │
Closed
```

---

## Housekeeping Incident Lifecycle

```text
Reported
     │
Assessment
     │
Resolution
     │
Closed
```

---

# 12. Workflow Catalogue

Housekeeping Operations governs the following workflow families.

## Planning Operations

- Housekeeping Zone Management
- Cleaning Schedule Planning
- Deep Cleaning Planning
- Event Cleaning Planning

---

## Cleaning Operations

- Routine Cleaning
- Complaint-Based Cleaning
- Emergency Cleaning
- Washroom Cleaning
- Common Area Cleaning

---

## Quality Operations

- Hygiene Inspection
- Washroom Inspection
- Quality Inspection
- Re-clean Verification

---

## Waste Operations

- Waste Collection
- Waste Segregation
- Waste Escalation
- Disposal References

---

## Governance Operations

- SLA Monitoring
- Vendor Operational Monitoring
- Complaint Resolution
- Performance Monitoring

---

# 13. Business Invariants

The following invariants shall always remain true.

- Every Cleaning Task shall belong to one Housekeeping Zone.
- Every Cleaning Request shall reference one operational location.
- Every Hygiene Inspection shall reference one completed cleaning activity or scheduled inspection.
- Every Quality Inspection shall reference one Cleaning Task.
- Every Housekeeping Incident shall remain permanently auditable.
- Consumable monitoring shall never own procurement.
- Maintenance ownership remains with G01.
- Vendor ownership remains outside this domain.
- Every lifecycle transition shall be audited.

Violation of business invariants shall reject the transaction.

---

# 14. Architectural Constraints

Housekeeping Operations shall not:

- Replace Maintenance Management
- Replace Procurement Systems
- Replace Vendor Management
- Replace HR Systems
- Replace Inventory Management
- Own Building Assets
- Own Vendor Contracts
- Own Housekeeping Personnel

Housekeeping Operations shall communicate with external domains exclusively through published business contracts.

---

# 15. Repository Principles

Housekeeping Operations conforms to the following enterprise principles.

- Domain-Driven Design
- Event-Driven Architecture
- CQRS
- Canonical Domain Ownership
- Configuration over Customisation
- Manual First Operations
- Effective-Dated Configuration
- Immutable Audit History
- Multi Campus Support
- Multi Institution Support
- Internal / Outsourced / Hybrid Housekeeping Support
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Future Smart Housekeeping Ready
```

# 16. Housekeeping Zone Management

## Purpose

Housekeeping Zone Management governs operational planning and execution areas for housekeeping services.

The zone is the primary operational planning unit for housekeeping activities.

Cleaning activities may occur at room level, but planning, scheduling, reporting, SLA monitoring, and governance shall occur at the zone level.

---

## Zone Hierarchy

The architecture supports the following hierarchy:

```text
Institution
      │
Campus
      │
Building
      │
Floor
      │
Housekeeping Zone
      │
Room / Area
```

Zone hierarchy shall remain configurable.

---

## Zone Types

Examples include:

- Academic Zone
- Laboratory Zone
- Hostel Zone
- Administrative Zone
- Sports Zone
- Library Zone
- Washroom Zone
- Cafeteria Zone
- Outdoor Zone
- Parking Zone

Zone categories shall remain configurable.

---

## Zone Workflow

```text
Zone Created
      │
Configuration
      │
Cleaning Policies Assigned
      │
Inspection Policies Assigned
      │
Zone Activated
```

---

## Zone Information

Typical information includes:

- Zone Code
- Zone Name
- Campus
- Building
- Floor
- Zone Type
- Cleaning Frequency
- Operational Priority
- Operational Status

Additional attributes shall remain configurable.

---

## Zone Priorities

Examples include:

- Critical
- High
- Medium
- Normal
- Low

Priority definitions shall remain configurable.

---

# 17. Cleaning Operations

## Purpose

Cleaning Operations govern planned housekeeping activities across institutional facilities.

Routine cleaning remains independent from complaint-driven cleaning.

---

## Cleaning Categories

Examples include:

- Routine Cleaning
- Deep Cleaning
- Event Cleaning
- Emergency Cleaning
- Special Cleaning
- Preventive Cleaning

Cleaning categories shall remain configurable.

---

## Scheduled Cleaning Workflow

```text
Cleaning Schedule
        │
Task Generation
        │
Operational Assignment
        │
Cleaning Performed
        │
Quality Inspection
        │
Completed
```

---

## Cleaning Tasks

Cleaning tasks may include:

- Classroom Cleaning
- Laboratory Cleaning
- Office Cleaning
- Hostel Cleaning
- Washroom Cleaning
- Corridor Cleaning
- Sports Facility Cleaning
- Cafeteria Cleaning
- Outdoor Cleaning

Task definitions shall remain configurable.

---

## Cleaning Checklists

Each cleaning task may reference:

- Standard Checklist
- Area Specific Checklist
- Consumable Checklist
- Safety Checklist

Checklist templates shall remain configurable.

---

## Deep Cleaning

Deep cleaning operations may support:

- Academic Break Cleaning
- Hostel Vacation Cleaning
- Event Preparation
- Laboratory Reset Cleaning
- Administrative Cleaning

Deep cleaning workflows remain configurable.

---

## Event Cleaning

Special cleaning may support:

- Convocation
- Cultural Events
- Sports Events
- Conferences
- Examinations

Event policies shall remain configurable.

---

# 18. Cleaning Requests & Complaint Management

## Purpose

Cleaning Requests govern reactive housekeeping activities initiated through complaints or operational observations.

Complaint-based cleaning remains independent from scheduled cleaning.

---

## Complaint Sources

Cleaning requests may originate from:

- Students
- Employees
- Hostel Administration
- Laboratory Administration
- Security Operations
- Housekeeping Supervisors
- Automated Inspection (Future)

Complaint sources shall remain configurable.

---

## Complaint Workflow

```text
Complaint Registered
        │
Validation
        │
Priority Assignment
        │
Operational Assignment
        │
Cleaning Completed
        │
Inspection
        │
Closure
```

---

## Complaint Priorities

Examples include:

- Critical
- High
- Medium
- Normal
- Low

Priority policies shall remain configurable.

---

## Complaint Categories

Examples include:

- Dirty Area
- Washroom Issue
- Waste Overflow
- Consumable Shortage
- Odour Complaint
- Hygiene Concern
- Emergency Cleaning
- Miscellaneous

Categories shall remain configurable.

---

## Escalation

Escalation policies may consider:

- SLA Breach
- Critical Zone
- Repeat Complaint
- Hygiene Risk
- Administrative Escalation

Escalation rules shall remain configurable.

---

## Re-clean Requests

Quality inspection may initiate:

- Partial Re-clean
- Complete Re-clean
- Supervisor Review
- Additional Inspection

Re-clean workflows shall remain configurable.

---

# 19. Hygiene Inspection & Waste Management

## Purpose

Hygiene Inspection governs operational verification of institutional cleanliness.

Waste Management governs operational waste collection while ownership of specialised waste remains external.

---

## Hygiene Inspection Workflow

```text
Inspection Scheduled
        │
Inspection Conducted
        │
Checklist Evaluation
        │
Score Assigned
        │
Corrective Action (If Required)
        │
Inspection Closed
```

---

## Inspection Types

Examples include:

- Routine Inspection
- Surprise Inspection
- Complaint Inspection
- Washroom Inspection
- Event Inspection
- Post-clean Inspection

Inspection types shall remain configurable.

---

## Hygiene Checklists

Inspection may evaluate:

- Floor Cleanliness
- Surface Cleanliness
- Dust Level
- Waste Disposal
- Odour
- Consumables
- Safety
- Hygiene Compliance

Checklist items shall remain configurable.

---

## Washroom Inspection

Inspection may include:

- Floor Condition
- Basin Condition
- Toilet Condition
- Soap Availability
- Tissue Availability
- Sanitizer Availability
- Water Availability
- Odour Check
- Overall Hygiene Score

Inspection standards shall remain configurable.

---

## Waste Collection

Supported waste categories may include:

- General Waste
- Recyclable Waste
- Organic Waste
- E-Waste (Reference)
- Biomedical Waste (Reference)
- Hazardous Waste (Reference)

Specialised waste ownership remains outside this domain.

---

## Housekeeping Incidents

Housekeeping staff may report:

- Water Leakage
- Electrical Issue
- Broken Furniture
- Unsafe Area
- Damaged Infrastructure
- Pest Observation

Infrastructure ownership remains with G01.

Incident events may publish maintenance requests.

---

# 20. Vendor Service Governance

## Purpose

Vendor Service Governance measures operational housekeeping performance regardless of whether services are performed by internal teams or outsourced agencies.

The domain governs service quality rather than workforce management.

---

## Operational Models

Supported operational models include:

- Internal Housekeeping
- Outsourced Housekeeping
- Hybrid Operations

All operational models shall function through enterprise configuration.

---

## SLA Monitoring

Operational SLA monitoring may evaluate:

- Response Time
- Completion Time
- Inspection Completion
- Complaint Resolution
- Escalation Time

Commercial SLA ownership remains outside this domain.

---

## Service Quality

Quality assessment may include:

- Cleaning Quality
- Inspection Score
- Re-clean Percentage
- Complaint Frequency
- Zone Cleanliness Score
- Washroom Hygiene Score

Quality policies shall remain configurable.

---

## Vendor References

Operational references may include:

- Vendor
- Service Area
- Operational Coverage
- Assigned Zones
- Operational Status

Vendor contracts remain outside this domain.

---

## Operational Dashboards

Supervisor dashboards may include:

- Pending Cleaning Tasks
- Active Cleaning Requests
- SLA Breaches
- Hygiene Inspection Status
- Re-clean Requests
- Critical Complaints
- Zone Performance

Management dashboards may include:

- Campus Cleanliness
- Vendor Performance
- SLA Compliance
- Complaint Trends
- Hygiene Trends
- Service Quality Trends

Dashboard composition shall remain configurable.

---

# 21. Operational Notifications

## Purpose

Operational Notifications inform housekeeping stakeholders regarding significant housekeeping activities.

Notification delivery remains within Section H.

Housekeeping Operations publishes business events only.

---

## Notification Sources

Notifications may originate from:

- Cleaning Schedule
- Cleaning Request
- Complaint Registration
- Hygiene Inspection
- SLA Breach
- Washroom Inspection
- Re-clean Request
- Housekeeping Incident
- Consumable Alert
- Waste Collection Exception

---

## Typical Notifications

Examples include:

- Cleaning Assignment
- Cleaning Completed
- Complaint Assigned
- SLA Warning
- Hygiene Failure
- Washroom Alert
- Consumable Low
- Re-clean Required
- Maintenance Required

Notification templates shall remain configurable.

---

## Notification Workflow

```text
Business Event
      │
Domain Event Published
      │
Notification Framework
      │
Recipient Resolution
      │
Delivery
```

Housekeeping Operations shall never implement notification delivery.

# 22. Domain Events

Housekeeping Operations publishes immutable operational domain events.

Every significant business transition shall publish a canonical domain event.

---

## Housekeeping Zone Events

| Event | Description |
|--------|-------------|
| HousekeepingZoneCreated | Housekeeping zone created |
| HousekeepingZoneActivated | Zone activated |
| HousekeepingZoneUpdated | Zone configuration updated |
| HousekeepingZoneDeactivated | Zone retired or deactivated |

---

## Cleaning Schedule Events

| Event | Description |
|--------|-------------|
| CleaningScheduleCreated | Cleaning schedule created |
| CleaningScheduleActivated | Schedule activated |
| CleaningTaskGenerated | Cleaning task generated |
| CleaningScheduleCompleted | Scheduled cleaning completed |

---

## Cleaning Request Events

| Event | Description |
|--------|-------------|
| CleaningRequestRegistered | Complaint/request registered |
| CleaningRequestAssigned | Request assigned |
| CleaningRequestEscalated | SLA escalation triggered |
| CleaningRequestCompleted | Cleaning completed |
| CleaningRequestClosed | Request verified and closed |

---

## Hygiene Inspection Events

| Event | Description |
|--------|-------------|
| HygieneInspectionScheduled | Inspection scheduled |
| HygieneInspectionCompleted | Inspection completed |
| HygieneInspectionFailed | Hygiene standards not met |
| CorrectiveActionRequested | Corrective cleaning requested |

---

## Waste Management Events

| Event | Description |
|--------|-------------|
| WasteCollectionScheduled | Waste collection scheduled |
| WasteCollected | Waste collection completed |
| WasteCollectionException | Waste collection exception recorded |

---

## Housekeeping Incident Events

| Event | Description |
|--------|-------------|
| HousekeepingIncidentReported | Operational incident reported |
| MaintenanceRequested | Maintenance request published to G01 |
| HousekeepingIncidentResolved | Incident resolved |

---

## Service Quality Events

| Event | Description |
|--------|-------------|
| QualityInspectionCompleted | Quality inspection completed |
| RecleanRequested | Re-clean required |
| SLAWarningGenerated | SLA warning generated |
| SLABreached | SLA breach recorded |

---

# 23. Service Contracts

Housekeeping Operations exposes operational housekeeping services.

---

## Zone Services

- Create Housekeeping Zone
- Configure Zone
- Activate Zone
- Query Zone Information

---

## Cleaning Schedule Services

- Create Cleaning Schedule
- Generate Cleaning Tasks
- Update Cleaning Frequency
- Complete Scheduled Cleaning

---

## Cleaning Request Services

- Register Cleaning Request
- Assign Cleaning Request
- Escalate Cleaning Request
- Close Cleaning Request

---

## Hygiene Services

- Schedule Inspection
- Perform Inspection
- Record Hygiene Score
- Record Corrective Action

---

## Waste Management Services

- Schedule Waste Collection
- Register Waste Collection
- Record Collection Exception
- Query Waste History

---

## Service Quality Services

- Record Quality Inspection
- Record SLA
- Initiate Re-clean
- Query Service Performance

---

## Incident Services

- Register Housekeeping Incident
- Escalate Incident
- Publish Maintenance Request
- Close Incident

All interactions shall occur exclusively through published business service contracts.

---

# 24. State Transition Matrix

| Aggregate | Primary Lifecycle |
|-----------|-------------------|
| Housekeeping Zone | Draft → Configured → Active → Closed |
| Cleaning Schedule | Draft → Approved → Active → Completed |
| Cleaning Request | Reported → Assigned → Completed → Verified → Closed |
| Cleaning Task | Generated → Assigned → Executing → Completed |
| Hygiene Inspection | Scheduled → Inspection → Evaluated → Closed |
| Waste Collection | Scheduled → Collected → Verified → Closed |
| Housekeeping Incident | Reported → Assessment → Resolved → Closed |
| Service Quality Assessment | Initiated → Evaluated → Approved → Closed |

Lifecycle extensions shall remain configuration driven.

---

# 25. Authorization Model

## Purpose

Housekeeping Operations shall enforce enterprise authorization through the Platform Authorization Framework.

Authentication remains the responsibility of the Platform.

Housekeeping Operations governs operational permissions only.

---

## Authorization Principles

Authorization shall support:

- Role Based Access Control (RBAC)
- Campus Scoped Authorization
- Zone Scoped Authorization
- Delegated Responsibility
- Temporary Delegation
- Effective-Dated Permissions
- Multi-Institution Isolation

---

## Typical Operational Roles

Examples include:

- Housekeeping Administrator
- Housekeeping Manager
- Housekeeping Supervisor
- Housekeeping Operator
- Quality Inspector
- Hygiene Inspector
- Vendor Coordinator
- Read Only Auditor

Operational roles may reference:

- Internal Employees
- Outsourced Housekeeping Personnel
- Hybrid Housekeeping Teams

Personnel ownership remains external.

---

## Authorization Matrix

| Activity | Typical Authorization |
|----------|-----------------------|
| Configure Zones | Housekeeping Administrator |
| Create Cleaning Schedule | Housekeeping Manager |
| Assign Cleaning Tasks | Housekeeping Supervisor |
| Complete Cleaning Tasks | Housekeeping Operator |
| Conduct Hygiene Inspection | Hygiene Inspector |
| Record Quality Inspection | Quality Inspector |
| Register Housekeeping Incident | Housekeeping Supervisor |
| View Reports | Management |

Authorization policies shall remain configurable.

---

## Delegation

Operational authority may be delegated.

Delegation shall include:

- Delegate User
- Effective Start Date
- Effective End Date
- Delegation Reason
- Automatic Expiry

Delegation history shall remain immutable.

---

# 26. Reporting Responsibilities

## Purpose

Housekeeping Operations provides operational reporting supporting institutional housekeeping governance.

Analytical processing remains outside this domain.

---

## Operational Reports

The domain shall support reporting for:

- Cleaning Schedules
- Cleaning Requests
- Cleaning Completion
- Hygiene Inspections
- Washroom Inspections
- Waste Collection
- Housekeeping Incidents
- SLA Monitoring
- Re-clean Requests
- Consumable Monitoring

---

## Management Reports

Examples include:

- Zone Cleanliness Summary
- Hygiene Compliance
- Washroom Performance
- Complaint Resolution
- SLA Compliance
- Vendor Operational Performance
- Cleaning Productivity
- Inspection Statistics
- Waste Collection Summary

---

## Executive Dashboards

Executive dashboards may include:

- Active Cleaning Tasks
- Pending Complaints
- Critical Hygiene Alerts
- SLA Breaches
- Washroom Status
- Zone Performance
- Vendor Performance
- Campus Cleanliness Index

Dashboard composition shall remain configurable.

# 27. Configuration Management

## Purpose

Housekeeping Operations shall support enterprise configuration without requiring source code modification.

The architecture shall support:

- Internal Housekeeping
- Outsourced Housekeeping
- Hybrid Housekeeping

through configuration alone.

---

## Configurable Elements

Institutions may configure:

### Housekeeping Zones

- Zone Types
- Zone Hierarchy
- Cleaning Policies
- Inspection Policies
- Operational Priorities

---

### Cleaning Operations

- Cleaning Categories
- Cleaning Frequencies
- Cleaning Templates
- Deep Cleaning Policies
- Event Cleaning Policies

---

### Cleaning Requests

- Complaint Categories
- Priority Matrix
- Escalation Rules
- Assignment Policies
- Re-clean Policies

---

### Hygiene Management

- Inspection Checklists
- Hygiene Standards
- Washroom Standards
- Inspection Frequencies
- Hygiene Threshold Scores

---

### Waste Management

- Waste Categories
- Collection Frequencies
- Collection Routes
- Escalation Rules

Specialised waste ownership remains outside this domain.

---

### SLA & Quality

- Response Time Targets
- Completion Targets
- Inspection Targets
- Quality Score Thresholds
- Escalation Matrix

Commercial SLA ownership remains outside this domain.

---

### Notifications

- Notification Templates
- Reminder Policies
- Escalation Policies
- Recipient Rules

---

## Configuration Principles

Configuration shall support:

- Version Control
- Effective Dating
- Institution Isolation
- Rollback
- Complete Audit History

---

# 28. Audit Responsibilities

## Purpose

Housekeeping Operations shall maintain immutable operational audit history.

Every significant housekeeping activity shall be auditable.

---

## Audit Coverage

Audit records shall include:

- User
- Timestamp
- Business Action
- Previous State
- New State
- Zone Reference
- Cleaning Request Reference
- Inspection Reference
- Business Reason
- Source System

---

## Audited Activities

Examples include:

- Zone Configuration
- Cleaning Schedule Creation
- Cleaning Assignment
- Cleaning Completion
- Complaint Registration
- Hygiene Inspection
- Quality Inspection
- Waste Collection
- Housekeeping Incident
- Configuration Changes

---

## Audit Principles

Audit history shall:

- Never be deleted
- Never be modified
- Support operational investigations
- Support compliance reviews
- Support accreditation
- Support historical reconstruction

Operational housekeeping history shall remain permanently available according to institutional retention policies.

---

# 29. Exception Handling

## Purpose

Housekeeping Operations shall manage operational housekeeping exceptions while preserving business consistency.

---

## Typical Exceptions

Examples include:

- Cleaning Request Validation Failure
- Zone Not Available
- SLA Breach
- Hygiene Failure
- Re-clean Required
- Consumable Shortage
- Waste Collection Failure
- Housekeeping Incident Escalation
- Authorization Failure
- Configuration Error

---

## Exception Workflow

```text
Business Exception
        │
Validation
        │
Recovery Strategy
        │
Audit Recorded
        │
Domain Event Published
        │
Notification Published
        │
Operational Continuation
```

---

## Recovery Principles

Recovery actions may include:

- Administrative Override
- Supervisor Assignment
- Alternative Housekeeping Team
- Emergency Cleaning
- Re-clean Assignment
- Maintenance Request (G01)
- Hygiene Escalation
- Vendor Escalation

Recovery behaviour shall remain configuration driven.

---

# 30. Performance & Scalability

Housekeeping Operations shall support enterprise-scale educational institutions.

The architecture shall support:

- Multi Institution
- Multi Campus
- Multiple Housekeeping Offices
- Thousands of Housekeeping Zones
- Internal Housekeeping Teams
- Outsourced Housekeeping Agencies
- Hybrid Operations
- High Complaint Volumes
- Large Cleaning Histories
- Large Inspection Histories
- Future Smart Washrooms
- Future IoT Cleaning Sensors
- Future Occupancy-Based Cleaning
- Future Robotic Cleaning
- Future AI Cleaning Optimisation
- Future Digital Twin Integration

The architecture shall remain horizontally scalable without domain redesign.

---

# 31. Glossary

| Term | Definition |
|------|------------|
| Housekeeping Zone | Operational planning and governance area for housekeeping activities. |
| Cleaning Schedule | Planned recurring housekeeping activity. |
| Cleaning Request | Reactive housekeeping request initiated through complaints or operational observations. |
| Cleaning Task | Individual housekeeping activity executed within a housekeeping zone. |
| Hygiene Inspection | Operational evaluation of institutional cleanliness and hygiene. |
| Service Quality Assessment | Evaluation of housekeeping quality and SLA compliance. |
| Housekeeping Incident | Operational issue identified during housekeeping activities. |
| Waste Collection | Operational process for institutional waste collection. |
| SLA | Configured operational service level expectation for housekeeping delivery. |

---

# Appendix A — Canonical Operational Flow

## Scheduled Housekeeping

```text
Housekeeping Zone
        │
Cleaning Schedule
        │
Cleaning Task
        │
Execution
        │
Quality Inspection
        │
Completion
        │
Audit History
```

---

## Complaint-Based Housekeeping

```text
Complaint
     │
Cleaning Request
     │
Assignment
     │
Cleaning
     │
Inspection
     │
Closure
```

---

## Hygiene Governance

```text
Inspection Schedule
        │
Inspection
        │
Evaluation
        │
Corrective Action
        │
Compliance
```

---

# Appendix B — Canonical Ownership Matrix

| Business Concept | Canonical Owner |
|------------------|-----------------|
| Housekeeping Zone | G08 Housekeeping Operations |
| Cleaning Schedule | G08 Housekeeping Operations |
| Cleaning Request | G08 Housekeeping Operations |
| Cleaning Task | G08 Housekeeping Operations |
| Hygiene Inspection | G08 Housekeeping Operations |
| Service Quality Assessment | G08 Housekeeping Operations |
| Waste Collection | G08 Housekeeping Operations |
| Housekeeping Incident | G08 Housekeeping Operations |
| Cleaning SLA Monitoring | G08 Housekeeping Operations |
| Buildings | G01 Facilities Management |
| Maintenance | G01 Facilities Management |
| Laboratory Master Data | G02 Laboratory Management |
| Hostel Master Data | G03 Hostel Management |
| Biomedical Waste | G06 Institutional Health Services |
| Vendor Contracts | Finance / Procurement |
| Housekeeping Personnel | Section E / External Vendor |
| Authentication | Section A |
| Notifications | Section H |

---

# Appendix C — Enterprise Design Principles

Housekeeping Operations follows the enterprise architectural principles established by SutraERP.

- Domain-Driven Design (DDD)
- Event-Driven Architecture (EDA)
- CQRS
- Canonical Domain Ownership
- Configuration over Customisation
- Manual First Operations
- Effective-Dated Configuration
- Immutable Audit History
- Multi-Campus Architecture
- Multi-Institution Architecture
- Internal Housekeeping Support
- Outsourced Housekeeping Support
- Hybrid Housekeeping Support
- Audit First
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Future Smart Housekeeping Ready
- Future IoT Integration Ready
- Future Occupancy-Based Cleaning Ready
- Future Robotic Cleaning Ready
- Future AI Cleaning Optimisation Ready
- Future Digital Twin Integration Ready

---

**End of Document**

**L2-G08 — Housekeeping Operations & Service Governance Workflows**

**Version:** 1.0  
**Status:** Architecture Draft (Ready for Architectural Review)