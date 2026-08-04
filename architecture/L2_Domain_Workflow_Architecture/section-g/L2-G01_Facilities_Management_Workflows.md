# L2-G01 — Facilities Management Workflows

**Repository:** SutraERP  
**Section:** G — Campus Operations  
**Domain:** G01 — Facilities Management  
**Document ID:** L2-G01  
**Version:** 1.0  
**Status:** Draft  
**Classification:** Level 2 Domain Architecture  
**Owner:** Campus Operations Domain  
**Document Owner:** Devang

**Upstream Dependencies**

- Section A — Platform Foundation
- Section B — Institution & Structure
- Section E — People & Workforce
- G10 — Infrastructure Management

**Downstream Consumers**

- Academic Delivery
- Student Operations
- Hostel Management
- Library Management
- Laboratory Management
- Security Management
- Medical Centre
- Examination Management

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
16. Facilities Service Request Workflows
17. Complaint Management Workflows
18. Work Order Management
19. Technician Assignment
20. Preventive Maintenance
21. Corrective Maintenance
22. Emergency Maintenance
23. Vendor Service Management
24. AMC Management
25. Utility Operations
26. Inspection Management
27. SLA Management
28. Escalation Management
29. Verification & Closure
30. Facility Asset Consumption
31. Domain Events
32. Service Contracts
33. State Transition Matrix
34. Authorization Model
35. Reporting
36. Configuration
37. Audit
38. Exception Handling
39. Performance & Scalability
40. Glossary

---

# 1. Purpose

The Facilities Management Domain governs the operational maintenance, servicing, inspection, utility operations, vendor coordination, preventive maintenance, corrective maintenance, emergency response, and service execution required to keep institutional infrastructure operational.

Facilities Management is the enterprise operational maintenance engine of SutraERP.

The domain owns operational execution.

It does not own physical infrastructure.

Physical infrastructure ownership remains within **G10 — Infrastructure Management**.

Facilities Management consumes infrastructure published by G10 and performs operational activities upon that infrastructure.

The domain provides a unified maintenance platform used by all institutional departments including academic operations, hostels, libraries, laboratories, transport, medical centres, administration, and common campus infrastructure.

This document defines:

- Facility service requests
- Complaint management
- Work orders
- Technician assignment
- Preventive maintenance
- Corrective maintenance
- Emergency maintenance
- Vendor management
- AMC governance
- Utility operations
- Inspection workflows
- SLA management
- Escalation
- Verification
- Operational reporting
- Business rules
- Domain events
- Integration contracts

Implementation technologies, APIs, databases, UI design, and deployment architecture remain outside the scope of this document.

---

# 2. Architectural Position

Facilities Management represents the operational execution layer responsible for maintaining institutional infrastructure.

The domain consumes infrastructure from G10 and performs maintenance activities upon that infrastructure.

Repository Position

```text
Platform Foundation
        │
        ▼
Institution & Structure
        │
        ▼
G10 Infrastructure Management
        │
        ▼
G01 Facilities Management
        │
 ┌──────┼─────────┬─────────┬─────────┬──────────┐
 ▼      ▼         ▼         ▼         ▼
Hostel Library Laboratory Security Examination
```

Facilities does not create infrastructure.

Facilities operates infrastructure.

---

# 3. Scope

Facilities Management owns the operational lifecycle of institutional maintenance and facility services.

The domain includes:

- Service Requests
- Facility Complaints
- Work Orders
- Technician Assignment
- Maintenance Scheduling
- Preventive Maintenance
- Corrective Maintenance
- Emergency Maintenance
- Vendor Coordination
- Annual Maintenance Contracts
- Utility Operations
- Facility Inspections
- SLA Monitoring
- Escalations
- Completion Verification
- Operational Cost Tracking
- Consumable Usage
- Facility Audit
- Operational Notifications
- Operational Reporting

Facilities serves as the single operational maintenance engine for the institution.

---

# 4. Out of Scope

The following responsibilities are intentionally excluded.

## Infrastructure Ownership

Owned by G10.

Examples:

- Campus
- Building
- Floor
- Space
- Capacity
- Availability
- Reservation
- Infrastructure Lifecycle

Facilities references infrastructure.

It never owns infrastructure.

---

## Employee Management

Owned by Section E.

Facilities assigns technicians.

Facilities does not own employee records.

---

## Procurement

Owned operationally by Section J - Enterprise Resource Management, with monetary consequences owned by Section F - Finance.

Facilities may request services or materials.

Facilities does not execute purchasing activities.

---

## Financial Accounting

Owned by Finance.

Facilities records operational costs.

Accounting remains outside this domain.

---

## Inventory Ownership

Facilities records consumption of materials used during maintenance activities.

Enterprise inventory ownership remains outside this domain.

---

## Workflow Engine

Owned by Platform Foundation.

Facilities publishes business events.

Workflow execution remains platform responsibility.

---

# 5. Domain Responsibilities

Facilities Management owns:

- Service Request Processing
- Complaint Processing
- Maintenance Planning
- Maintenance Execution
- Work Order Lifecycle
- Technician Assignment
- Vendor Assignment
- AMC Execution
- Inspection Planning
- Inspection Recording
- Utility Monitoring
- SLA Monitoring
- Escalation
- Completion Verification
- Consumable Tracking
- Operational Cost Recording
- Maintenance History
- Audit History
- Operational Dashboards

Facilities shall maintain complete operational history throughout the lifecycle of every maintenance activity.

---

# 6. Design Principles

## Operational Ownership

Facilities owns operational maintenance.

Infrastructure ownership remains external.

---

## Single Maintenance Engine

All maintenance activities within the institution shall execute through Facilities Management regardless of the consuming business domain.

Examples include:

- Hostel Maintenance
- Library Maintenance
- Laboratory Maintenance
- Classroom Maintenance
- Office Maintenance
- Auditorium Maintenance
- Sports Facility Maintenance
- Medical Centre Maintenance

This ensures consistent operational governance across the institution.

---

## Configuration over Customisation

Approval workflows, SLA rules, priorities, categories, technician groups, vendor policies, and escalation policies shall remain configuration-driven.

Institution-specific behaviour shall never require application customization.

---

## Event Driven Architecture

Every significant operational transition shall publish immutable domain events.

Downstream systems consume events.

They shall never manipulate Facilities state directly.

---

## Audit First

Every operational activity shall produce immutable audit history.

Operational history shall never be deleted.

---

## Mobile First Operations

The architecture shall support mobile-first execution.

Technicians shall be capable of:

- Receiving assignments
- Updating work orders
- Uploading photographs
- Completing inspections
- Recording consumable usage
- Capturing verification evidence

using supported mobile applications.

---

## Enterprise Scalability

The architecture shall support:

- Multi-campus institutions
- Multi-institution deployments
- Thousands of concurrent work orders
- Vendor execution
- Outsourced maintenance
- Future predictive maintenance

without architectural redesign.

# 7. Domain Terminology

| Term | Definition |
|------|------------|
| Facility Service Request | A formal request for facility-related assistance initiated by an authorised requester. |
| Complaint | A reported issue affecting institutional infrastructure, utilities, or facility services requiring operational attention. |
| Work Order | The operational execution unit responsible for resolving one or more facility requests or maintenance activities. |
| Maintenance | Planned or unplanned activities performed to restore, preserve, or improve operational infrastructure. |
| Preventive Maintenance | Planned maintenance performed before failure occurs. |
| Corrective Maintenance | Maintenance performed after a defect has been identified. |
| Emergency Maintenance | Immediate maintenance required to eliminate safety risks or major operational disruption. |
| Breakdown Maintenance | Unplanned maintenance resulting from infrastructure or equipment failure. |
| Technician | Workforce member assigned to perform maintenance or inspection activities. |
| Technician Team | Logical grouping of technicians based on skills, departments, or operational responsibilities. |
| Vendor | External service provider authorised to perform maintenance activities. |
| AMC | Annual Maintenance Contract governing scheduled vendor-supported maintenance services. |
| Inspection | Operational assessment of infrastructure condition, compliance, or maintenance quality. |
| SLA | Service Level Agreement defining expected operational response and resolution times. |
| Escalation | Workflow process triggered when operational commitments are not satisfied. |
| Verification | Confirmation that maintenance has been completed satisfactorily. |
| Consumable | Materials consumed during maintenance execution. |
| Utility | Operational services including electricity, water, HVAC, internet, gas, generators, solar systems, and related infrastructure. |
| Facility Incident | Operational event requiring immediate maintenance intervention. |
| Root Cause Analysis | Investigation identifying the underlying reason for recurring or significant failures. |

---

# 8. Aggregate Design

## Aggregate Roots

Facilities Management consists of the following aggregate roots:

- Service Request
- Complaint
- Work Order
- Preventive Maintenance Plan
- Maintenance Schedule
- Inspection
- Vendor Assignment
- AMC Contract
- Utility Meter
- Utility Reading
- SLA Policy
- Escalation
- Verification Record

Each aggregate owns its operational lifecycle.

---

## Aggregate Responsibilities

### Service Request

Owns:

- Request Registration
- Request Classification
- Request Priority
- Request Assignment
- Request Lifecycle

---

### Complaint

Owns:

- Complaint Registration
- Complaint Categorisation
- Severity
- Attachments
- Related Infrastructure
- Complaint Lifecycle

---

### Work Order

Owns:

- Operational Execution
- Technician Assignment
- Vendor Assignment
- Maintenance Activities
- Completion Evidence
- Operational Status

---

### Preventive Maintenance Plan

Owns:

- Schedule Definition
- Frequency
- Applicable Infrastructure
- Maintenance Templates
- Checklist References

---

### Inspection

Owns:

- Inspection Planning
- Findings
- Recommendations
- Verification
- Closure

---

### Utility Meter

Owns:

- Meter Information
- Meter Location
- Meter Configuration
- Utility Classification

---

### Utility Reading

Owns:

- Reading Values
- Reading History
- Consumption
- Operational Alerts

---

## Aggregate Identity

Every aggregate shall possess:

- Immutable System Identifier
- Configurable Business Code
- Human Readable Name or Reference

Aggregate identity shall never change after creation.

Business codes may change through authorised administrative processes.

---

# 9. Dependencies & Integration (Bounded Context Relationships)

## Upstream Domains

### Platform Foundation

Provides:

- Authentication
- Authorization
- Workflow Engine
- Notification Framework
- Audit Framework
- Event Bus
- Document Management
- Configuration Framework

---

### G10 Infrastructure Management

Provides:

- Campus
- Building
- Floor
- Space
- Capacity
- Availability
- Infrastructure Lifecycle

Facilities consumes infrastructure.

Infrastructure ownership always remains within G10.

---

### People & Workforce

Provides:

- Employees
- Technicians
- Supervisors
- Vendor Workforce
- Organisational Assignment

Facilities assigns work.

People domain owns personnel.

---

## Downstream Consumers

Facilities publishes operational information to:

### Academic Delivery

Consumes:

- Classroom Maintenance Status
- Operational Availability
- Emergency Closures

---

### Hostel Management

Consumes:

- Hostel Maintenance
- Room Maintenance Status

---

### Library Management

Consumes:

- Library Maintenance
- Reading Area Availability

---

### Laboratory Management

Consumes:

- Laboratory Maintenance
- Equipment Maintenance References

---

### Security

Consumes:

- Facility Incidents
- Emergency Maintenance
- Safety Notifications

---

### Examination

Consumes:

- Examination Venue Readiness
- Emergency Maintenance Notifications

---

# 10. Domain Model

The Facilities domain models operational maintenance activities performed upon infrastructure owned by G10.

```text
Infrastructure (G10)
        │
        ▼
Service Request
        │
        ▼
Complaint
        │
        ▼
Work Order
        │
 ┌──────┼───────────┬───────────┐
 ▼      ▼           ▼
Technician Vendor Inspection
        │
        ▼
Verification
        │
        ▼
Closure
```

Supporting aggregates:

```text
Work Order
     │
     ├── Checklist
     ├── SOP
     ├── Attachments
     ├── Consumables
     ├── Cost
     ├── Time Log
     ├── SLA
     └── Audit
```

Utility Operations:

```text
Utility
    │
Meter
    │
Reading
    │
Consumption
    │
Alert
```

---

# 11. Lifecycle Overview

## Service Request Lifecycle

```text
Draft
   │
Submitted
   │
Validated
   │
Accepted
   │
Converted to Complaint
```

---

## Complaint Lifecycle

```text
Open
   │
Assigned
   │
Work Order Created
   │
In Progress
   │
Resolved
   │
Verified
   │
Closed
```

---

## Work Order Lifecycle

```text
Created
    │
Assigned
    │
Accepted
    │
In Progress
    │
Paused
    │
Completed
    │
Verified
    │
Closed
```

Work Orders may reopen following authorised verification failure.

---

## Preventive Maintenance Lifecycle

```text
Planned
     │
Scheduled
     │
Generated
     │
Executed
     │
Verified
     │
Completed
```

---

## Inspection Lifecycle

```text
Scheduled
      │
Assigned
      │
Inspection
      │
Findings
      │
Approved
      │
Closed
```

---

# 12. Workflow Catalogue

Facilities Management governs the following workflow families.

## Service Management

- Service Request Registration
- Complaint Registration
- Complaint Assignment
- Complaint Escalation
- Complaint Closure

---

## Work Management

- Work Order Creation
- Technician Assignment
- Vendor Assignment
- Work Execution
- Verification
- Closure
- Reopening

---

## Maintenance Management

- Preventive Maintenance
- Corrective Maintenance
- Breakdown Maintenance
- Emergency Maintenance
- Scheduled Maintenance

---

## Vendor Management

- Vendor Assignment
- Vendor Performance
- Vendor SLA
- AMC Execution
- Vendor Verification

---

## Utility Management

- Utility Monitoring
- Utility Reading
- Utility Alerts
- Utility Consumption
- Utility Outages

---

## Inspection Management

- Planned Inspection
- Surprise Inspection
- Maintenance Inspection
- Safety Inspection
- Verification Inspection

---

## SLA Management

- SLA Monitoring
- SLA Escalation
- SLA Breach Handling

---

# 13. Business Invariants

The following invariants shall always remain true.

- Every Work Order references valid Infrastructure.
- Infrastructure ownership remains external to Facilities.
- Every Complaint belongs to at least one Infrastructure location.
- Every Work Order possesses exactly one current lifecycle state.
- Every lifecycle transition shall be audited.
- Completed Work Orders require completion evidence.
- Verification occurs before operational closure.
- Archived operational records remain immutable.
- SLA measurements remain historically accurate.
- Maintenance history shall never be deleted.
- Vendor assignments shall remain historically traceable.
- Utility readings shall remain immutable after verification.

Violation of invariants shall reject the transaction.

---

# 14. Architectural Constraints

Facilities Management shall not:

- Create Infrastructure
- Modify Infrastructure ownership
- Manage Employees
- Execute Procurement
- Maintain Financial Ledgers
- Own Enterprise Inventory
- Replace Platform Workflow Engine
- Replace Notification Services
- Replace Authorization Services

Facilities operates exclusively through published business contracts and platform capabilities.

---

# 15. Repository Principles

Facilities Management conforms to the following enterprise principles.

- Domain Driven Design
- Event Driven Architecture
- CQRS
- Configuration over Customisation
- Mobile First Operations
- Multi Campus Support
- Multi Institution Support
- Multi Country Support
- Immutable Audit History
- Effective Dated Configuration
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Operational Traceability
- Single Maintenance Engine Philosophy
```

# 16. Facility Service Request Workflows

## Purpose

Facility Service Requests provide the formal entry point for requesting operational facility services.

A Service Request represents a demand for maintenance, inspection, utility support, facility assistance, or operational intervention.

Every maintenance activity shall originate from a valid operational trigger.

---

## Service Request Sources

Service Requests may originate from:

- Student
- Faculty
- Employee
- Department
- Hostel
- Library
- Laboratory
- Security
- Medical Centre
- Administration
- Scheduled Maintenance
- Inspection Findings
- Utility Monitoring
- Vendor Recommendation
- Manual Administrative Creation

---

## Request Categories

Institutions may configure categories including:

- Electrical
- Civil
- Plumbing
- HVAC
- Carpentry
- Furniture
- Cleaning
- Pest Control
- Painting
- Networking
- Internet
- Water Supply
- Generator
- Lift
- Safety
- General Maintenance

Additional categories shall remain configuration-driven.

---

## Request Priorities

Priority shall be configurable.

Typical priorities include:

- Low
- Medium
- High
- Critical
- Emergency

Priority shall be independent from Severity.

---

## Request Severity

Severity indicates operational impact.

Examples include:

- Minor
- Moderate
- Major
- Critical
- Life Safety

Severity may influence SLA and escalation policies.

---

## Service Request Workflow

```text
Request Submitted
        │
Authorization Validation
        │
Category Validation
        │
Priority Assignment
        │
Infrastructure Validation
        │
Service Request Created
        │
Publish Domain Event
```

---

## Attachments

Requests may include:

- Photographs
- Videos
- Audio Recordings
- Documents
- Supporting Notes

Attachment policies shall remain configurable.

---

## Request Conversion

A Service Request may result in:

- Complaint
- Inspection
- Preventive Maintenance Activity
- Emergency Work Order
- Administrative Closure

Conversion rules shall remain configuration-driven.

---

# 17. Complaint Management Workflows

## Purpose

Complaint Management governs reporting, assessment, tracking, assignment, escalation, and closure of facility-related operational issues.

Complaints represent operational problems requiring investigation or corrective action.

---

## Complaint Sources

Complaints may originate from:

- Service Requests
- Mobile Applications
- Self-Service Portals
- Administrative Users
- Utility Alerts
- Inspection Findings
- Scheduled Maintenance
- Security Incidents

---

## Complaint Classification

Complaints may be classified by:

- Category
- Priority
- Severity
- Campus
- Building
- Floor
- Space
- Department
- Responsible Team

Institutions may configure additional classifications.

---

## Complaint Workflow

```text
Complaint Registered
          │
Validation
          │
Classification
          │
Assignment
          │
Investigation
          │
Work Order Required?
      ┌───┴────┐
      │        │
     No       Yes
      │        │
 Administrative
 Resolution
               │
               ▼
         Work Order Created
```

---

## Duplicate Complaints

The platform shall support:

- Duplicate Detection
- Complaint Linking
- Complaint Merging
- Parent Complaint
- Child Complaint

Historical references shall remain intact.

---

## Complaint Splitting

A complaint may generate multiple Work Orders.

Example:

Electrical Failure

↓

Electrical Team

+

Civil Team

+

Networking Team

Each Work Order shall possess an independent lifecycle.

---

## Complaint Escalation

Complaints may escalate due to:

- SLA Breach
- Operational Priority
- Administrative Override
- Safety Risk
- Critical Infrastructure

Escalation rules shall be configurable.

---

# 18. Work Order Management

## Purpose

The Work Order represents the primary operational execution aggregate within Facilities Management.

Every maintenance activity shall execute through one or more Work Orders.

---

## Work Order Sources

Work Orders may originate from:

- Complaint
- Preventive Maintenance
- Inspection
- Emergency Maintenance
- Administrative Request
- Vendor Recommendation

---

## Work Order Information

Typical information includes:

- Work Order Number
- Request Reference
- Complaint Reference
- Infrastructure Reference
- Assigned Team
- Assigned Technician
- Vendor
- SLA
- Estimated Duration
- Actual Duration
- Status
- Priority
- Completion Evidence

---

## Work Order Workflow

```text
Created
    │
Assignment
    │
Accepted
    │
Planning
    │
Execution
    │
Verification
    │
Completed
    │
Closed
```

---

## Multiple Technician Assignment

A Work Order may be assigned to:

- One Technician
- Multiple Technicians
- Technician Team
- Internal Team
- Vendor Team

Assignment policies shall remain configurable.

---

## Subcontracting

Work Orders may be subcontracted to authorised vendors.

Facilities Management remains responsible for operational governance and audit history.

---

## Work Order Checklists

Work Orders may contain:

- Safety Checklist
- Technical Checklist
- Quality Checklist
- Completion Checklist

Checklist templates shall be configurable.

---

## Standard Operating Procedures

Work Orders may reference one or more SOPs.

Examples:

- Electrical Repair SOP
- Lift Safety SOP
- Generator SOP
- HVAC SOP
- Fire Equipment SOP

SOP revisions shall be version controlled.

---

## Work Order Attachments

Work Orders may contain:

- Before Photographs
- During Repair Photographs
- After Completion Photographs
- Vendor Reports
- Inspection Certificates
- Technical Documents

---

## Work Order Reopening

Authorised supervisors may reopen completed Work Orders.

Reasons may include:

- Incomplete Resolution
- Quality Failure
- Repeat Failure
- Administrative Review

Previous audit history shall remain immutable.

---

# 19. Technician Assignment Workflows

## Purpose

Technician Assignment governs operational allocation of maintenance work.

Personnel ownership remains within Section E.

Facilities owns operational assignments.

---

## Assignment Types

Supported assignment models include:

- Individual Technician
- Technician Team
- Department Team
- Vendor Team
- Hybrid Assignment

---

## Assignment Workflow

```text
Work Order Created
         │
Skill Matching
         │
Availability Validation
         │
Assignment Rules
         │
Technician Assigned
         │
Notification Published
```

---

## Assignment Factors

Assignment decisions may consider:

- Technician Skills
- Certification
- Availability
- Current Workload
- Campus
- Department
- Shift
- Emergency Priority

Assignment algorithms shall remain configurable.

---

## Manual Override

Authorised supervisors may manually override automated assignments.

Override reasons shall be recorded within the audit history.

---

## Assignment Acceptance

Assigned technicians may:

- Accept Assignment
- Request Reassignment
- Reject Assignment (with reason)

Supervisor approval may be required depending upon institutional policy.

---

## Technician Mobility

Technicians may update operational progress using supported mobile applications.

Supported activities include:

- Accept Work
- Update Progress
- Upload Evidence
- Record Consumables
- Complete Checklist
- Request Assistance
- Complete Work Order

Mobile operations shall function within platform security policies.
```

# 20. Preventive Maintenance Workflows

## Purpose

Preventive Maintenance governs planned maintenance activities intended to prevent infrastructure failures before they occur.

Preventive Maintenance is schedule-driven and proactive.

Execution occurs through Work Orders.

---

## Objectives

Preventive Maintenance aims to:

- Improve infrastructure reliability
- Reduce breakdowns
- Extend infrastructure life
- Improve safety
- Ensure regulatory compliance
- Reduce maintenance cost
- Improve operational availability

---

## Maintenance Sources

Preventive Maintenance Plans may be created for:

- Buildings
- Electrical Systems
- Plumbing Systems
- HVAC Systems
- Lift Systems
- Fire Safety Equipment
- DG Generators
- Water Treatment Systems
- Solar Installations
- CCTV Systems
- Access Control Systems
- Campus Utilities

Institutions may configure additional maintenance plans.

---

## Schedule Types

Supported schedules include:

- Daily
- Weekly
- Monthly
- Quarterly
- Half-Yearly
- Annual
- Usage Based
- Seasonal
- Institution Configurable

---

## Preventive Maintenance Workflow

```text
Maintenance Plan
        │
Scheduler Trigger
        │
Work Order Generated
        │
Technician Assignment
        │
Execution
        │
Verification
        │
Completion
        │
Next Schedule Generated
```

---

## Maintenance Checklists

Preventive Maintenance may reference predefined checklists.

Examples:

- Electrical Safety Checklist
- Generator Inspection Checklist
- HVAC Cleaning Checklist
- Fire Extinguisher Inspection Checklist
- Water Tank Cleaning Checklist

Checklist templates shall be version controlled.

---

## Deferred Maintenance

Preventive Maintenance may be deferred under exceptional circumstances.

Reasons may include:

- Infrastructure Closed
- Emergency Activities
- Resource Constraints
- Administrative Approval

Deferred maintenance shall require an authorised reason and complete audit history.

---

## Missed Maintenance

Missed schedules shall:

- Generate Notifications
- Escalate according to SLA
- Remain visible on operational dashboards
- Preserve historical schedules

Missed maintenance shall never be silently discarded.

---

# 21. Corrective Maintenance Workflows

## Purpose

Corrective Maintenance governs maintenance activities initiated after a defect, malfunction, or operational issue has been identified.

---

## Trigger Sources

Corrective Maintenance may originate from:

- Complaints
- Service Requests
- Inspection Findings
- Technician Observations
- Utility Alerts
- Vendor Reports
- Administrative Reviews

---

## Corrective Maintenance Workflow

```text
Issue Identified
        │
Assessment
        │
Work Order
        │
Technician Assignment
        │
Repair
        │
Verification
        │
Closure
```

---

## Root Cause Analysis

Facilities shall support recording Root Cause Analysis for significant or recurring failures.

Examples include:

- Equipment Failure
- Human Error
- Infrastructure Age
- Utility Failure
- Vendor Deficiency
- Environmental Conditions

Root Cause Analysis shall remain part of maintenance history.

---

## Repeat Failure Tracking

Facilities shall maintain repeat failure history.

Examples:

- Same Space
- Same Equipment
- Same Utility
- Same Vendor
- Same Complaint Category

Repeat failures may influence preventive maintenance planning.

---

# 22. Emergency Maintenance Workflows

## Purpose

Emergency Maintenance governs immediate operational response to incidents that threaten safety, infrastructure, or institutional operations.

---

## Emergency Examples

- Electrical Fire
- Water Pipeline Burst
- Lift Failure
- Structural Damage
- Gas Leakage
- Generator Failure
- Network Backbone Failure
- Flooding
- Roof Collapse
- Fire Alarm Failure

---

## Emergency Workflow

```text
Emergency Reported
        │
Immediate Classification
        │
Emergency Work Order
        │
Technician / Vendor Dispatch
        │
Emergency Repair
        │
Safety Verification
        │
Operational Restoration
```

---

## Emergency Priorities

Emergency activities bypass normal scheduling.

Priority shall always remain above routine maintenance.

Approval workflows may be simplified according to institutional policy.

---

## Emergency Audit

Emergency operations shall capture:

- Incident Time
- Reporter
- Initial Assessment
- Response Team
- Response Time
- Resolution Time
- Supporting Evidence
- Administrative Review

---

# 23. Vendor Service Management

## Purpose

Vendor Service Management governs participation of external service providers within Facilities operations.

Vendor ownership remains external.

Facilities governs operational engagement.

---

## Vendor Activities

Vendors may perform:

- Electrical Maintenance
- Lift Maintenance
- HVAC Maintenance
- Fire Safety Maintenance
- Generator Servicing
- Civil Repairs
- Plumbing
- Pest Control
- Housekeeping Services
- Security Equipment Maintenance

Institution-specific vendor services shall be configurable.

---

## Vendor Assignment Workflow

```text
Vendor Required
       │
Vendor Selection
       │
Assignment
       │
Work Execution
       │
Verification
       │
Performance Review
```

---

## Vendor Performance

Facilities shall maintain operational vendor performance information.

Examples include:

- Response Time
- Completion Time
- SLA Compliance
- Quality Rating
- Repeat Visits
- Customer Satisfaction
- Contract Compliance

Vendor ratings remain operational information.

Commercial evaluation remains outside this domain.

---

## Vendor Documentation

Facilities may maintain:

- Certifications
- Insurance Documents
- Safety Approvals
- Compliance Certificates
- Work Completion Reports

Document lifecycle shall follow Platform Document Management.

---

# 24. Annual Maintenance Contract (AMC) Management

## Purpose

AMC Management governs operational execution of Annual Maintenance Contracts.

Commercial contract ownership remains outside Facilities.

Facilities governs operational utilisation.

---

## Typical AMC Coverage

Examples include:

- Lift Systems
- HVAC
- DG Generator
- Fire Safety Systems
- Solar Systems
- Water Treatment Plants
- CCTV
- Access Control
- Laboratory Utilities

---

## AMC Workflow

```text
AMC Registered
       │
Maintenance Schedule
       │
Vendor Notification
       │
Scheduled Service
       │
Inspection
       │
Verification
       │
AMC Updated
```

---

## AMC Monitoring

Facilities shall monitor:

- Contract Validity
- Service Schedule
- Missed Visits
- SLA Compliance
- Renewal Reminder
- Vendor Performance

---

## AMC Renewal

Renewal notifications shall be generated before contract expiry.

Renewal approval remains institution configurable.

---

# 25. Utility Operations Workflows

## Purpose

Utility Operations govern operational monitoring of institutional utility services.

Utility ownership remains associated with Infrastructure.

Operational monitoring belongs to Facilities.

---

## Supported Utilities

Examples include:

- Electricity
- Water
- Internet
- Gas
- HVAC
- Solar
- DG Generator
- Sewage Systems
- Water Treatment
- Campus Lighting

Institutions may configure additional utilities.

---

## Utility Monitoring Workflow

```text
Utility Reading
        │
Validation
        │
Consumption Recorded
        │
Threshold Evaluation
        │
Alert Generated
        │
Operational Review
```

---

## Meter Management

Facilities shall maintain:

- Meter Identity
- Utility Type
- Infrastructure Location
- Calibration Date
- Operational Status

Meter ownership remains operational.

---

## Utility Alerts

Alerts may be generated for:

- High Consumption
- Low Consumption
- Utility Failure
- Meter Failure
- Communication Failure
- Outage
- Threshold Breach

Alert rules shall remain configurable.

---

## Utility Outages

Utility outages shall support:

- Incident Registration
- Affected Infrastructure
- Estimated Restoration Time
- Resolution Status
- Operational Notifications

Outage history shall remain permanently available.

---

## Consumption History

Facilities shall preserve historical consumption for operational analysis.

Analytical forecasting remains outside this domain.

---

# 26. Inspection Management Workflows

## Purpose

Inspection Management governs operational inspections performed before, during, or after maintenance activities.

Inspection supports operational quality and safety.

---

## Inspection Types

Supported inspection types include:

- Preventive Inspection
- Corrective Inspection
- Emergency Inspection
- Safety Inspection
- Vendor Inspection
- Completion Inspection
- Follow-up Inspection
- Surprise Inspection

Additional inspection types shall remain configurable.

---

## Inspection Workflow

```text
Inspection Planned
         │
Inspector Assigned
         │
Inspection Conducted
         │
Observations Recorded
         │
Corrective Action Required?
      ┌──────┴──────┐
      │             │
     No            Yes
      │             │
Inspection     Work Order
Completed      Generated
```

---

## Inspection Findings

Findings may be classified as:

- Observation
- Minor Defect
- Major Defect
- Safety Risk
- Compliance Issue
- Immediate Action Required

Classification policies shall remain configurable.

---

## Inspection Evidence

Inspection records may include:

- Photographs
- Videos
- Documents
- Measurement Values
- Technical Notes
- Digital Signatures

Evidence shall become part of permanent operational history.

# 27. Service Level Agreement (SLA) Management

## Purpose

Service Level Agreement (SLA) Management governs operational commitments related to response, execution, verification, and closure of Facilities activities.

SLAs establish measurable service expectations for maintenance operations.

---

## SLA Principles

SLA policies shall be:

- Configuration Driven
- Category Specific
- Priority Based
- Severity Aware
- Campus Aware
- Institution Configurable
- Fully Auditable

---

## SLA Components

An SLA may define:

- Response Time
- Assignment Time
- Technician Acceptance Time
- Resolution Time
- Verification Time
- Closure Time
- Escalation Thresholds

---

## SLA Workflow

```text
Work Created
      │
Applicable SLA Identified
      │
SLA Activated
      │
Operational Monitoring
      │
SLA Met?
   ┌──────┴──────┐
   │             │
 Yes            No
   │             │
Complete     Escalation
```

---

## SLA Breach

When an SLA is breached, the platform may:

- Notify Assigned Technician
- Notify Supervisor
- Notify Department Head
- Notify Facilities Manager
- Trigger Escalation Workflow
- Record SLA Violation

Escalation behaviour shall remain configurable.

---

## SLA History

Facilities shall preserve:

- SLA Definition
- Applicable Priority
- Actual Response Time
- Actual Resolution Time
- Breach Status
- Escalation History

Historical SLA records shall never be modified.

---

# 28. Escalation Management

## Purpose

Escalation Management governs operational actions performed when maintenance activities exceed configured operational expectations.

---

## Escalation Triggers

Escalation may occur due to:

- SLA Breach
- Critical Priority
- Safety Risk
- Vendor Delay
- Technician Unavailability
- Repeat Failure
- Management Override

---

## Escalation Levels

Typical escalation hierarchy:

```text
Technician
     │
Supervisor
     │
Facilities Manager
     │
Campus Administrator
     │
Institution Administration
```

Institutions may configure alternative escalation hierarchies.

---

## Escalation Workflow

```text
SLA Violation
      │
Escalation Rule Evaluation
      │
Escalation Level Determined
      │
Notifications Published
      │
Management Review
      │
Operational Resolution
```

---

## Manual Escalation

Authorised personnel may manually escalate any operational activity.

Manual escalation shall require:

- Business Reason
- Authorised User
- Timestamp
- Audit Record

---

## Escalation Audit

Every escalation shall preserve:

- Trigger
- Escalation Level
- Responsible User
- Action Taken
- Resolution
- Timestamp

---

# 29. Verification & Closure Workflows

## Purpose

Verification confirms that maintenance activities have been completed satisfactorily before operational closure.

Verification ensures quality assurance.

---

## Verification Types

Facilities shall support:

- Technician Self Verification
- Supervisor Verification
- Vendor Verification
- Department Verification
- Administrative Verification

Applicable verification rules shall remain configurable.

---

## Verification Workflow

```text
Work Completed
      │
Verification Required
      │
Evidence Review
      │
Quality Validation
      │
Verification Approved
      │
Work Order Closed
```

---

## Completion Evidence

Evidence may include:

- Before Photographs
- After Photographs
- Video Evidence
- Digital Signature
- Checklist Completion
- Inspection Report
- Vendor Certificate

---

## Closure Validation

Before closure the platform shall validate:

- Work Completed
- Checklist Completed
- Mandatory Evidence Available
- Verification Successful
- Audit Recorded

Closure shall be rejected if mandatory validations fail.

---

# 30. Facility Asset Consumption

## Purpose

Facilities records operational consumption of materials used during maintenance execution.

Inventory ownership remains outside this domain.

Facilities records usage references only.

---

## Consumable Examples

Examples include:

- Electrical Cable
- Switches
- LED Fixtures
- Pipes
- Plumbing Fittings
- Paint
- Cleaning Materials
- Fasteners
- Lubricants
- Filters

Institutions may configure additional consumables.

---

## Consumption Workflow

```text
Work Order
      │
Consumable Recorded
      │
Quantity Validated
      │
Usage History Updated
      │
Work Order Completed
```

---

## Consumption History

Facilities shall preserve:

- Material
- Quantity
- Unit
- Work Order
- Technician
- Date
- Business Reason

Inventory valuation remains outside Facilities.

---

# 31. Domain Events

Facilities publishes immutable operational events.

Every significant business transition shall publish a domain event.

---

## Service Request Events

| Event | Description |
|--------|-------------|
| ServiceRequestCreated | New request registered |
| ServiceRequestUpdated | Request modified |
| ServiceRequestCancelled | Request cancelled |
| ServiceRequestConverted | Converted into operational workflow |

---

## Complaint Events

| Event | Description |
|--------|-------------|
| ComplaintRegistered | Complaint created |
| ComplaintAssigned | Complaint assigned |
| ComplaintMerged | Complaint merged |
| ComplaintSplit | Multiple work orders created |
| ComplaintEscalated | Complaint escalated |
| ComplaintClosed | Complaint completed |

---

## Work Order Events

| Event | Description |
|--------|-------------|
| WorkOrderCreated | Work order generated |
| WorkOrderAssigned | Technician assigned |
| WorkOrderAccepted | Technician accepted |
| WorkOrderStarted | Execution started |
| WorkOrderPaused | Execution paused |
| WorkOrderCompleted | Execution completed |
| WorkOrderVerified | Verification completed |
| WorkOrderClosed | Work order closed |
| WorkOrderReopened | Work order reopened |

---

## Maintenance Events

| Event | Description |
|--------|-------------|
| PreventiveMaintenanceScheduled | Schedule generated |
| PreventiveMaintenanceCompleted | Activity completed |
| CorrectiveMaintenanceInitiated | Corrective work started |
| EmergencyMaintenanceDeclared | Emergency response initiated |

---

## Vendor Events

| Event | Description |
|--------|-------------|
| VendorAssigned | Vendor allocated |
| VendorAccepted | Vendor accepted assignment |
| VendorCompleted | Vendor finished work |
| VendorRated | Performance recorded |

---

## Utility Events

| Event | Description |
|--------|-------------|
| UtilityReadingRecorded | Meter reading captured |
| UtilityAlertGenerated | Operational alert created |
| UtilityOutageReported | Utility failure recorded |
| UtilityRestored | Utility restored |

---

## Inspection Events

| Event | Description |
|--------|-------------|
| InspectionScheduled | Inspection planned |
| InspectionStarted | Inspection commenced |
| InspectionCompleted | Inspection finished |
| InspectionApproved | Inspection accepted |
| CorrectiveActionRaised | Follow-up required |

---

## SLA Events

| Event | Description |
|--------|-------------|
| SLAStarted | SLA monitoring initiated |
| SLABreached | SLA exceeded |
| EscalationTriggered | Escalation initiated |
| SLAResolved | SLA successfully completed |

---

# 32. Service Contracts

Facilities exposes business services for enterprise consumption.

---

## Request Services

- Register Service Request
- Update Service Request
- Cancel Service Request
- Query Service Request

---

## Complaint Services

- Register Complaint
- Assign Complaint
- Escalate Complaint
- Close Complaint

---

## Work Order Services

- Create Work Order
- Assign Technician
- Assign Vendor
- Complete Work Order
- Verify Work Order
- Reopen Work Order

---

## Maintenance Services

- Create Preventive Plan
- Generate Maintenance Schedule
- Execute Maintenance
- Record Maintenance History

---

## Utility Services

- Register Utility Meter
- Record Utility Reading
- Report Utility Outage
- Query Utility Status

---

## Inspection Services

- Schedule Inspection
- Record Findings
- Approve Inspection
- Raise Corrective Action

All interactions shall occur through published service contracts.

---

# 33. State Transition Matrix

| Aggregate | Primary Lifecycle |
|-----------|-------------------|
| Service Request | Draft → Submitted → Accepted → Converted → Closed |
| Complaint | Open → Assigned → In Progress → Resolved → Verified → Closed |
| Work Order | Created → Assigned → Accepted → In Progress → Completed → Verified → Closed |
| Preventive Maintenance | Planned → Scheduled → Generated → Executed → Verified → Completed |
| Inspection | Planned → Assigned → Conducted → Reviewed → Approved → Closed |
| Vendor Assignment | Assigned → Accepted → Working → Completed |
| Utility Incident | Reported → Investigating → Resolved → Closed |

Lifecycle extensions shall remain configuration driven.

# 34. Authorization Model

## Purpose

Facilities Management shall enforce enterprise authorization policies through the Platform Authorization Framework.

Authorization determines who may perform operational activities within the Facilities domain.

Facilities does not implement authentication.

Authentication remains a Platform responsibility.

---

## Authorization Principles

Authorization shall support:

- Role Based Access Control (RBAC)
- Hierarchical Authorization
- Campus Scoped Authorization
- Department Scoped Authorization
- Delegated Responsibility
- Temporary Delegation
- Effective-Dated Permissions
- Multi-Institution Isolation

---

## Typical Operational Roles

Examples include:

- Facilities Administrator
- Facilities Manager
- Campus Facilities Manager
- Maintenance Supervisor
- Technician
- Vendor Technician
- Utility Operator
- Inspection Officer
- Compliance Officer
- Department Representative
- Service Requester
- Read Only Auditor

Institutions may define additional operational roles.

---

## Authorization Matrix

| Activity | Typical Authorization |
|----------|-----------------------|
| Raise Service Request | All Authorised Users |
| Register Complaint | All Authorised Users |
| Create Work Order | Facilities Supervisor |
| Assign Technician | Facilities Supervisor |
| Assign Vendor | Facilities Manager |
| Approve Maintenance | Facilities Manager |
| Verify Completion | Supervisor / Configurable |
| Close Work Order | Authorised Verifier |
| Configure SLA | Facilities Administrator |
| Configure AMC | Facilities Administrator |
| Record Utility Reading | Utility Operator |
| Conduct Inspection | Inspection Officer |

Authorization policies shall remain configurable.

---

## Delegation

Operational responsibilities may be delegated.

Delegation shall support:

- Delegate User
- Effective Start Date
- Effective End Date
- Delegation Reason
- Automatic Expiry

Delegation history shall remain immutable.

---

# 35. Reporting Responsibilities

## Purpose

Facilities Management provides operational reporting for maintenance activities.

Analytical processing remains outside this domain.

---

## Operational Reports

Facilities shall support reporting for:

- Open Service Requests
- Active Complaints
- Work Order Status
- Technician Workload
- Vendor Activities
- Inspection Activities
- Utility Incidents
- SLA Status
- Escalation Status
- Maintenance Completion

---

## Management Reports

Examples include:

- Campus Maintenance Summary
- Building Maintenance Summary
- Department Maintenance Summary
- Vendor Performance Summary
- Technician Productivity
- Preventive Maintenance Compliance
- Corrective Maintenance Trends
- Emergency Maintenance Summary

---

## Compliance Reports

Facilities shall support:

- Inspection Compliance
- Preventive Maintenance Compliance
- AMC Compliance
- Utility Compliance
- SLA Compliance

---

## Executive Dashboards

Executive reporting may include:

- Total Open Requests
- Average Resolution Time
- SLA Compliance Percentage
- Emergency Incidents
- Maintenance Cost Summary
- Vendor Performance Index
- Utility Health Overview

Dashboard composition shall remain configurable.

---

# 36. Configuration Management

## Purpose

Facilities Management shall support enterprise configuration without source code modification.

---

## Configurable Elements

Institutions may configure:

### Service Requests

- Categories
- Priorities
- Severity Levels
- Request Types
- Mandatory Fields
- Attachment Policies

---

### Complaints

- Complaint Categories
- Complaint Statuses
- Merge Policies
- Split Policies
- Escalation Policies

---

### Work Orders

- Status Values
- Assignment Rules
- Checklist Templates
- SOP Templates
- Verification Rules
- Closure Policies

---

### Maintenance

- Maintenance Types
- Maintenance Frequencies
- Maintenance Windows
- Schedule Rules
- Deferral Policies

---

### Vendor Management

- Vendor Categories
- Vendor Qualification Rules
- Vendor SLA
- Vendor Performance Parameters

---

### Utilities

- Utility Types
- Threshold Values
- Alert Rules
- Meter Categories

---

### SLA

- Response Times
- Resolution Times
- Escalation Levels
- Priority Mapping

---

## Configuration Principles

Configuration shall support:

- Version Control
- Effective Dating
- Institution Isolation
- Rollback
- Complete Audit History

---

# 37. Audit Responsibilities

## Purpose

Facilities Management shall maintain immutable operational audit history.

Every significant business activity shall be auditable.

---

## Audit Coverage

Audit records shall include:

- User
- Timestamp
- Business Action
- Previous State
- New State
- Infrastructure Reference
- Work Order Reference
- Complaint Reference
- Business Reason
- Source System

---

## Audited Activities

Examples include:

- Service Request Creation
- Complaint Registration
- Complaint Assignment
- Work Order Creation
- Technician Assignment
- Vendor Assignment
- Inspection Results
- Utility Readings
- Verification
- Closure
- Configuration Changes

---

## Audit Principles

Audit history shall:

- Never be deleted
- Never be modified
- Support regulatory review
- Support historical reconstruction
- Support operational investigations

---

# 38. Exception Handling

## Purpose

Facilities Management shall handle operational exceptions while preserving business consistency.

---

## Typical Exceptions

Examples include:

- Invalid Infrastructure Reference
- Duplicate Complaint
- Technician Unavailable
- Vendor Unavailable
- SLA Breach
- Verification Failure
- Utility Failure
- Work Order Conflict
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
Notification Published
        │
Operational Continuation
```

---

## Recovery Principles

Recovery actions may include:

- Retry
- Reassignment
- Escalation
- Manual Intervention
- Administrative Override

Recovery behaviour shall remain configurable.

---

# 39. Performance & Scalability

Facilities Management shall support enterprise-scale deployments.

The architecture shall support:

- Multi Institution
- Multi Campus
- Thousands of Buildings
- Millions of Historical Work Orders
- High Concurrent Service Requests
- Mobile Workforce
- Vendor Operations
- Utility Monitoring
- Large Operational Audit History

The architecture shall remain horizontally scalable.

---

# 40. Glossary

| Term | Definition |
|------|------------|
| Service Request | Formal request for operational facility support. |
| Complaint | Operational issue requiring investigation or corrective action. |
| Work Order | Operational execution unit for maintenance activities. |
| Preventive Maintenance | Planned maintenance performed before failure occurs. |
| Corrective Maintenance | Maintenance performed after failure identification. |
| Emergency Maintenance | Immediate maintenance required to remove operational or safety risks. |
| Technician | Workforce member assigned to maintenance execution. |
| Vendor | External organisation providing operational maintenance services. |
| AMC | Annual Maintenance Contract governing scheduled maintenance services. |
| SLA | Service Level Agreement defining operational commitments. |
| Verification | Confirmation that maintenance has been completed satisfactorily. |
| Escalation | Operational process for unresolved or delayed activities. |
| Utility | Operational infrastructure service such as electricity, water, HVAC, internet, or gas. |
| Consumable | Material used during maintenance execution. |
| Inspection | Operational assessment of facility condition or maintenance quality. |
| Root Cause Analysis | Structured investigation identifying the underlying cause of operational failures. |

---

# Appendix A — Canonical Operational Flow

```text
Service Request
        │
Complaint
        │
Assessment
        │
Work Order
        │
Technician / Vendor Assignment
        │
Execution
        │
Inspection
        │
Verification
        │
Closure
```

---

# Appendix B — Canonical Ownership Matrix

| Business Concept | Canonical Owner |
|------------------|-----------------|
| Campus | G10 Infrastructure |
| Building | G10 Infrastructure |
| Floor | G10 Infrastructure |
| Space | G10 Infrastructure |
| Service Request | G01 Facilities |
| Complaint | G01 Facilities |
| Work Order | G01 Facilities |
| Preventive Maintenance | G01 Facilities |
| Corrective Maintenance | G01 Facilities |
| Emergency Maintenance | G01 Facilities |
| Inspection | G01 Facilities |
| Utility Operations | G01 Facilities |
| Vendor Assignment | G01 Facilities |
| AMC Operations | G01 Facilities |
| Employee | Section E |
| Authentication | Section A |
| Workflow Engine | Section A |
| Notifications | Section H |

---

# Appendix C — Enterprise Design Principles

Facilities Management follows the enterprise architectural principles established by SutraERP.

- Domain-Driven Design (DDD)
- Event-Driven Architecture
- CQRS
- Canonical Ownership
- Configuration over Customisation
- Effective-Dated Configuration
- Immutable Audit History
- Mobile First Operations
- Single Maintenance Engine
- Multi-Campus Architecture
- Multi-Institution Architecture
- Enterprise Scalability
- High Cohesion
- Loose Coupling
- Platform-Oriented Integration

---

**End of Document**

**L2-G01 — Facilities Management Workflows**

**Version:** 1.0  
**Status:** Draft

