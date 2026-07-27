# L2-G03 — Hostel Management Workflows

**Repository:** SutraERP  
**Section:** G — Campus Operations  
**Domain:** G03 — Hostel Management  
**Document ID:** L2-G03  
**Version:** 1.0  
**Status:** Architecture Draft  
**Classification:** Level 2 Domain Architecture  
**Owner:** Campus Operations Domain

**Upstream Dependencies**

- Section A — Platform Foundation
- Section B — Institution & Infrastructure
- Section C — Student Lifecycle
- Section E — People & Workforce
- G10 — Infrastructure Management
- G01 — Facilities Management

**Downstream Consumers**

- Student Operations
- Finance
- Security
- Institutional Reporting
- Parent Communication
- Notifications

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
16. Hostel Admission Workflows
17. Room Allocation Workflows
18. Occupancy Management
19. Check-In Workflows
20. Leave & Movement Management
21. Visitor Management
22. Hostel Discipline
23. Room Transfer
24. Check-Out & Exit Clearance
25. Mess Allocation
26. Hostel Complaints
27. Notifications
28. Domain Events
29. Service Contracts
30. State Transition Matrix
31. Authorization Model
32. Reporting
33. Configuration
34. Audit
35. Exception Handling
36. Performance & Scalability
37. Glossary

---

# 1. Purpose

The Hostel Management Domain governs operational management of institutional hostels throughout the complete student residency lifecycle.

The domain manages hostel admission, room allocation, occupancy, residency operations, leave management, visitor management, discipline, room transfers, hostel complaints, mess allocation, check-out, and operational reporting.

Hostel Management operates upon hostel infrastructure owned by G10.

Student identity and academic lifecycle remain within Section C.

Maintenance execution remains within G01.

Financial accounting remains outside this domain.

This document defines:

- Hostel Admission
- Room Allocation
- Occupancy Management
- Check-In
- Leave Management
- Visitor Management
- Hostel Discipline
- Room Transfers
- Mess Allocation
- Hostel Complaints
- Exit Clearance
- Operational Reporting
- Domain Events
- Business Rules
- Integration Contracts

Implementation technologies, APIs, databases, UI behaviour, and deployment architecture remain outside the scope of this document.

---

# 2. Architectural Position

Hostel Management represents the operational residency layer of SutraERP.

Infrastructure ownership remains within G10.

Student ownership remains within Section C.

Maintenance remains within G01.

Repository Position

```text
Platform Foundation
        │
        ▼
Student Lifecycle
        │
        ▼
G10 Infrastructure
        │
        ▼
G03 Hostel Management
        │
 ┌──────┼──────────┬──────────┐
 ▼      ▼          ▼
Finance Security Notifications
```

Hostel Management governs who occupies institutional hostel facilities.

---

# 3. Scope

Hostel Management owns:

- Hostel Admission
- Room Allocation
- Hostel Occupancy
- Student Residency
- Hostel Check-In
- Hostel Check-Out
- Leave Management
- Curfew Management
- Visitor Management
- Hostel Discipline
- Room Transfers
- Mess Allocation
- Hostel Complaints
- Exit Clearance
- Operational Notifications
- Operational Reporting
- Audit History

The domain serves as the canonical operational residency engine for institutional hostels.

---

# 4. Out of Scope

The following responsibilities remain outside this domain.

---

## Infrastructure

Owned by G10.

Examples:

- Hostel Buildings
- Floors
- Rooms
- Capacity
- Infrastructure Lifecycle

Hostel Management references hostel infrastructure.

---

## Maintenance

Owned by G01.

Examples:

- Room Repairs
- Electrical Repairs
- Plumbing
- Utility Maintenance
- Work Orders

Hostel complaints may recommend maintenance.

Maintenance execution remains external.

---

## Student Lifecycle

Owned by Section C.

Examples:

- Student Identity
- Student Registration
- Student Progression
- Academic Status

---

## Employee Management

Owned by Section E.

Examples:

- Hostel Wardens
- Hostel Staff
- Administrative Staff

Hostel Management assigns operational responsibilities only.

---

## Financial Accounting

Owned by Finance.

Examples:

- Hostel Billing
- Security Deposit
- Ledger
- Accounting
- Fee Collection

Hostel Management provides occupancy references.

Finance owns accounting.

---

## Authentication & Workflow

Authentication, authorization, notifications, workflow execution, document management, and audit infrastructure remain Platform responsibilities.

---

# 5. Domain Responsibilities

Hostel Management owns:

- Residency Operations
- Room Allocation
- Occupancy Management
- Student Movement
- Leave Management
- Curfew Operations
- Visitor Operations
- Hostel Discipline
- Room Transfer
- Mess Allocation
- Complaint Registration
- Exit Clearance
- Residency History
- Hostel Dashboards
- Operational Audit

The domain maintains complete residency history throughout the student hostel lifecycle.

---

# 6. Design Principles

## Operational Ownership

Hostel Management owns residency operations.

Infrastructure ownership remains within G10.

---

## Configuration over Customisation

Room allocation policies, leave rules, visitor rules, discipline policies, hostel categories, mess allocation rules, approval workflows, notifications, and operational constraints shall remain configuration driven.

Institution-specific behaviour shall never require application customization.

---

## Manual First Operations

Many institutions continue to operate hostel processes manually.

The architecture shall support manual operations with optional automation.

Automation shall simplify operations without forcing institutional process changes.

---

## Event Driven Architecture

Every significant operational transition shall publish immutable domain events.

Downstream systems consume events.

They shall never manipulate Hostel Management state directly.

---

## Audit First

Every residency activity shall produce immutable audit history.

Historical occupancy records shall never be deleted.

---

## Enterprise Scalability

The architecture shall support:

- Multi Campus
- Multi Institution
- Boys Hostels
- Girls Hostels
- Staff Hostels
- International Hostels
- Guest Hostels
- Future Smart Hostel Integration

without architectural redesign.

# 7. Domain Terminology

| Term | Definition |
|------|------------|
| Hostel | Institutional residential facility providing accommodation for authorised occupants. |
| Hostel Admission | Approval process allowing a student to become eligible for hostel residency. |
| Hostel Occupancy | Operational record representing a student's active residency within a hostel. |
| Room Allocation | Operational assignment of a hostel room to an authorised student. |
| Check-In | Formal commencement of hostel residency. |
| Check-Out | Formal completion of hostel residency. |
| Room Transfer | Operational reassignment of a resident from one room to another. |
| Leave Request | Temporary permission allowing a resident to remain away from the hostel. |
| Curfew | Institution-defined operational return time for hostel residents. |
| Visitor | External or authorised person visiting a hostel resident. |
| Hostel Warden | Operational authority responsible for hostel governance. |
| Hostel Complaint | Operational issue reported within hostel premises. |
| Hostel Discipline | Operational management of hostel conduct, warnings, and disciplinary actions. |
| Mess Allocation | Assignment of a resident to a hostel mess or dining facility. |
| Exit Clearance | Operational verification performed before final hostel departure. |

---

# 8. Aggregate Design

## Aggregate Roots

Hostel Management consists of the following aggregate roots:

- Hostel Admission
- Hostel Occupancy
- Room Allocation
- Check-In
- Leave Request
- Visitor Entry
- Hostel Complaint
- Room Transfer
- Hostel Discipline
- Exit Clearance
- Mess Allocation

Each aggregate owns its operational lifecycle.

---

## Aggregate Responsibilities

### Hostel Admission

Owns:

- Admission Request
- Eligibility Validation
- Approval
- Admission Status
- Residency Eligibility

---

### Hostel Occupancy

Owns:

- Residency Status
- Occupancy History
- Current Room
- Residency Timeline
- Occupancy Lifecycle

---

### Room Allocation

Owns:

- Room Assignment
- Allocation Date
- Allocation Status
- Transfer Reference
- Allocation History

Room ownership remains within G10.

---

### Check-In

Owns:

- Arrival
- Initial Verification
- Residency Activation
- Occupancy Commencement

---

### Leave Request

Owns:

- Leave Application
- Approval
- Leave Duration
- Return Confirmation

---

### Visitor Entry

Owns:

- Visitor Registration
- Approval
- Entry
- Exit
- Visit History

---

### Hostel Complaint

Owns:

- Complaint Registration
- Complaint Classification
- Operational Resolution
- Maintenance Recommendation

---

### Room Transfer

Owns:

- Transfer Request
- Approval
- New Allocation
- Transfer Completion

---

### Hostel Discipline

Owns:

- Incident
- Warning
- Action
- Disciplinary History

---

### Exit Clearance

Owns:

- Checkout Verification
- Clearance Items
- Damage Inspection
- Final Residency Closure

---

### Mess Allocation

Owns:

- Mess Assignment
- Effective Period
- Allocation Status

Mess operations remain intentionally lightweight within this domain.

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
- Document Management
- Configuration Management
- Event Bus

---

### Section C — Student Lifecycle

Provides:

- Student Identity
- Student Status
- Student Programme
- Student Registration

Hostel Management consumes student information.

Student ownership remains external.

---

### G10 — Infrastructure Management

Provides:

- Hostel Building
- Hostel Floor
- Hostel Room
- Capacity
- Infrastructure Availability

Hostel Management governs occupancy.

Infrastructure ownership remains within G10.

---

### G01 — Facilities Management

Provides:

- Maintenance
- Work Orders
- Facility Services
- Utility Operations

Hostel complaints may recommend maintenance.

Maintenance execution remains within G01.

---

### Section E — People & Workforce

Provides:

- Hostel Wardens
- Hostel Staff
- Administrative Personnel

Hostel Management assigns operational responsibility.

Personnel ownership remains external.

---

### Finance

Consumes:

- Hostel Occupancy
- Mess Allocation
- Residency Period
- Exit Clearance

Finance owns billing and accounting.

---

## Downstream Consumers

Hostel Management publishes information to:

### Security

Consumes:

- Visitor Information
- Leave Information
- Curfew Violations
- Occupancy Status

---

### Parent Communication

Consumes:

- Leave Approval
- Emergency Notifications
- Residency Status

---

### Institutional Reporting

Consumes:

- Occupancy Statistics
- Hostel Utilisation
- Leave Reports
- Discipline Reports

---

# 10. Domain Model

Hostel Management governs operational residency throughout the hostel lifecycle.

```text
Student
    │
Hostel Admission
    │
Room Allocation
    │
Check-In
    │
Occupancy
    │
 ┌────┼───────────┬─────────────┐
 ▼    ▼           ▼             ▼
Leave Visitors Complaints Discipline
    │
    ▼
Room Transfer
    │
    ▼
Exit Clearance
    │
    ▼
Check-Out
```

Supporting operational model:

```text
Occupancy
      │
      ├── Leave
      ├── Visitors
      ├── Mess Allocation
      ├── Complaints
      ├── Discipline
      ├── Audit
      └── Notifications
```

---

# 11. Lifecycle Overview

## Hostel Admission Lifecycle

```text
Applied
    │
Verification
    │
Approved
    │
Room Allocation
```

---

## Occupancy Lifecycle

```text
Allocated
     │
Checked-In
     │
Resident
     │
Leaving
     │
Checked-Out
```

---

## Leave Lifecycle

```text
Applied
    │
Under Review
    │
Approved
    │
Resident Left
    │
Resident Returned
    │
Closed
```

---

## Visitor Lifecycle

```text
Registered
      │
Approved
      │
Checked-In
      │
Checked-Out
      │
Closed
```

---

## Complaint Lifecycle

```text
Registered
      │
Reviewed
      │
Recommendation
      │
Resolved
      │
Closed
```

---

## Exit Clearance Lifecycle

```text
Requested
      │
Verification
      │
Inspection
      │
Approved
      │
Completed
```

---

# 12. Workflow Catalogue

Hostel Management governs the following workflow families.

## Residency Operations

- Hostel Admission
- Room Allocation
- Occupancy Activation
- Occupancy Closure

---

## Resident Operations

- Check-In
- Check-Out
- Leave Management
- Return Confirmation

---

## Accommodation Operations

- Room Transfer
- Occupancy Review
- Room Availability
- Allocation Review

---

## Visitor Operations

- Visitor Registration
- Visitor Approval
- Visitor Entry
- Visitor Exit

---

## Discipline Operations

- Incident Recording
- Warning Issuance
- Disciplinary Review
- Resolution

---

## Complaint Operations

- Complaint Registration
- Complaint Classification
- Maintenance Recommendation
- Complaint Closure

---

## Mess Operations

- Mess Allocation
- Allocation Change
- Allocation History

---

## Exit Operations

- Exit Clearance
- Damage Inspection
- Administrative Approval
- Residency Closure

---

# 13. Business Invariants

The following invariants shall always remain true.

- Every resident shall reference a valid student.
- Every room allocation shall reference a valid hostel room.
- Hostel rooms remain owned by G10.
- Only approved admissions may create occupancy.
- Every occupancy shall reference exactly one active room allocation.
- Every visitor shall reference an active resident.
- Every leave request shall belong to an active resident.
- Hostel complaints shall never directly create maintenance work orders.
- Every residency transition shall be audited.
- Historical residency information shall never be deleted.
- Finance consumes occupancy information but does not modify residency state.

Violation of business invariants shall reject the transaction.

---

# 14. Architectural Constraints

Hostel Management shall not:

- Create Hostel Buildings
- Modify Infrastructure
- Execute Maintenance
- Manage Student Lifecycle
- Manage Employees
- Execute Financial Accounting
- Replace Workflow Engine
- Replace Notification Services
- Replace Authentication Services

Hostel Management operates exclusively through published business contracts.

---

# 15. Repository Principles

Hostel Management conforms to the following enterprise principles.

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
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Operational Traceability
- Audit First

# 16. Hostel Admission Workflows

## Purpose

Hostel Admission governs the operational process through which eligible students are admitted into institutional hostel facilities.

Academic admission remains outside this domain.

Hostel Admission governs residency eligibility only.

---

## Admission Sources

Hostel admission may originate from:

- Student Application
- Administrative Allocation
- Management Quota
- International Student Allocation
- Special Accommodation Request
- Hostel Renewal

Admission sources shall remain configurable.

---

## Admission Workflow

```text
Application Submitted
        │
Eligibility Verification
        │
Approval Workflow
        │
Hostel Admission Approved
        │
Room Allocation Initiated
```

---

## Eligibility Validation

Admission validation may consider:

- Student Status
- Academic Programme
- Campus
- Hostel Category
- Gender Policy
- Institution Rules
- Hostel Availability
- Administrative Approval

Validation rules shall remain configurable.

---

## Admission Status

Supported admission states include:

- Applied
- Under Review
- Approved
- Waitlisted
- Rejected
- Withdrawn

Status values shall remain configurable.

---

## Waiting List

Where hostel capacity is unavailable, applications may be placed on a waiting list.

Waiting lists shall support:

- Priority
- Application Date
- Category
- Administrative Override

Allocation policies shall remain configurable.

---

# 17. Room Allocation Workflows

## Purpose

Room Allocation governs assignment of hostel rooms to approved residents.

Infrastructure ownership remains within G10.

Hostel Management owns operational occupancy.

---

## Allocation Workflow

```text
Admission Approved
        │
Room Search
        │
Eligibility Validation
        │
Manual / Configurable Allocation
        │
Occupancy Created
```

---

## Allocation Principles

Room allocation may consider:

- Hostel Category
- Gender
- Department
- Programme
- Academic Year
- Medical Requirements
- Accessibility
- Administrative Preferences

Allocation rules shall remain configuration driven.

---

## Allocation Methods

Institutions may support:

- Manual Allocation
- Assisted Allocation
- Rule-Based Allocation

Fully automatic allocation shall remain optional.

---

## Allocation History

The platform shall preserve:

- Previous Room
- New Room
- Allocation Date
- Allocation Reason
- Authorised User

Historical allocation records shall remain immutable.

---

## Room Availability

Hostel Management consumes room availability from G10.

Operational occupancy determines assignable rooms.

Temporary operational restrictions may prevent allocation without modifying infrastructure ownership.

---

# 18. Occupancy Management

## Purpose

Occupancy Management governs the active residency lifecycle of hostel residents.

Occupancy begins after successful check-in and ends after authorised checkout.

---

## Occupancy Workflow

```text
Room Allocated
      │
Check-In
      │
Resident Active
      │
Transfer / Leave
      │
Exit Clearance
      │
Check-Out
```

---

## Occupancy Status

Supported occupancy states include:

- Reserved
- Active
- On Leave
- Temporarily Suspended
- Transfer Pending
- Checkout Pending
- Closed

Status values shall remain configurable.

---

## Residency History

Hostel Management shall permanently preserve:

- Hostel
- Room
- Residency Period
- Transfers
- Leave History
- Complaints
- Discipline
- Exit Information

Residency history shall never be deleted.

---

# 19. Check-In Workflows

## Purpose

Check-In formally activates hostel residency.

Check-In confirms operational occupancy.

---

## Check-In Workflow

```text
Room Allocated
      │
Arrival Verification
      │
Document Verification
      │
Hostel Acceptance
      │
Occupancy Activated
```

---

## Check-In Validation

Operational validation may include:

- Approved Admission
- Valid Student Status
- Room Allocation
- Identity Verification
- Required Documents
- Administrative Approval

Validation policies shall remain configurable.

---

## Check-In Information

Typical information includes:

- Check-In Date
- Check-In Time
- Assigned Room
- Occupancy Status
- Remarks
- Supporting Documents

---

## Initial Issuance

Where applicable, institutions may operationally record issuance of:

- Room Key
- Cupboard Key
- Access Card
- Mattress
- Furniture References

These are operational records only.

Asset ownership remains outside this domain.

---

# 20. Leave & Movement Management

## Purpose

Leave Management governs temporary absence of hostel residents.

Movement records improve operational awareness without affecting academic status.

---

## Leave Types

Examples include:

- Weekend Leave
- Vacation Leave
- Medical Leave
- Emergency Leave
- Local Permission
- Special Permission

Leave categories shall remain configurable.

---

## Leave Workflow

```text
Leave Requested
       │
Approval Review
       │
Approved
       │
Resident Exit
       │
Return Verification
       │
Leave Closed
```

---

## Curfew Management

Institutions may define configurable curfew policies.

Curfew management may include:

- Curfew Time
- Late Entry
- Exception Approval
- Administrative Remarks

Policy enforcement remains configurable.

---

## Late Entry

Late entry records may include:

- Entry Time
- Reason
- Approving Authority
- Supporting Remarks

Repeated violations may be linked to disciplinary workflows.

---

## Return Verification

Residents returning from approved leave may require operational verification.

Verification policies remain institution configurable.

---

# 21. Visitor Management

## Purpose

Visitor Management governs authorised visitor access to hostel residents.

Visitor records support security and operational governance.

---

## Visitor Workflow

```text
Visitor Registered
       │
Resident Verification
       │
Approval
       │
Visitor Entry
       │
Visitor Exit
       │
Visit Closed
```

---

## Visitor Information

Typical visitor information includes:

- Visitor Name
- Identification Reference
- Resident
- Relationship
- Entry Time
- Exit Time
- Contact Information
- Purpose of Visit

Visitor data policies shall follow institutional privacy requirements.

---

## Visitor Approval

Approval may require:

- Resident Confirmation
- Warden Approval
- Security Approval

Approval workflows shall remain configurable.

---

## Visitor History

Visitor history shall permanently preserve:

- Entry
- Exit
- Duration
- Approval
- Associated Resident
- Audit History

---

# 22. Hostel Discipline

## Purpose

Hostel Discipline governs operational recording of disciplinary matters occurring within hostel premises.

Academic disciplinary proceedings remain outside this domain.

---

## Discipline Workflow

```text
Incident Reported
        │
Review
        │
Investigation
        │
Decision
        │
Action Recorded
        │
Closed
```

---

## Discipline Categories

Examples include:

- Curfew Violation
- Hostel Rule Violation
- Property Damage
- Misconduct
- Safety Violation
- Visitor Policy Violation

Additional categories shall remain configurable.

---

## Disciplinary Actions

Examples include:

- Warning
- Written Warning
- Parent Notification
- Hostel Restriction
- Administrative Review

Institution policies determine applicable actions.

---

## Discipline History

Every disciplinary record shall preserve:

- Incident
- Investigation
- Decision
- Action
- Responsible Authority
- Audit History

Historical disciplinary records shall remain immutable.

# 23. Room Transfer Workflows

## Purpose

Room Transfer governs operational relocation of residents from one hostel room to another.

Infrastructure ownership remains within G10.

Hostel Management governs operational occupancy changes.

---

## Transfer Triggers

Room transfers may originate from:

- Student Request
- Medical Requirement
- Administrative Decision
- Hostel Maintenance Recommendation
- Disciplinary Action
- Capacity Optimisation
- Special Accommodation Requirement

Transfer reasons shall remain configurable.

---

## Transfer Workflow

```text
Transfer Requested
        │
Eligibility Review
        │
Target Room Validation
        │
Approval
        │
Current Allocation Closed
        │
New Allocation Created
        │
Occupancy Updated
```

---

## Transfer Validation

Validation may consider:

- Active Occupancy
- Room Availability
- Hostel Category
- Gender Policy
- Medical Requirements
- Administrative Approval
- Existing Restrictions

Validation rules shall remain configuration driven.

---

## Transfer History

Hostel Management shall permanently preserve:

- Previous Room
- New Room
- Transfer Date
- Transfer Reason
- Approving Authority
- Supporting Remarks

Historical transfer records shall never be modified.

---

# 24. Check-Out & Exit Clearance

## Purpose

Exit Clearance governs completion of the hostel residency lifecycle.

Checkout confirms that the resident has fulfilled all operational responsibilities before vacating the hostel.

Financial settlement remains outside this domain.

---

## Exit Clearance Workflow

```text
Checkout Requested
        │
Room Inspection
        │
Operational Verification
        │
Clearance Review
        │
Exit Approved
        │
Check-Out Completed
```

---

## Operational Verification

Exit verification may include:

- Room Inspection
- Hostel Property Verification
- Outstanding Complaints
- Active Leave Validation
- Mess Allocation Closure
- Administrative Approval

Institutions may configure additional verification rules.

---

## Damage Inspection

Operational inspection may record:

- Room Condition
- Furniture Condition
- Fixture Damage
- Missing Issued Items
- Supporting Photographs
- Inspection Remarks

Damage information may be shared with Finance.

Financial recovery remains outside Hostel Management.

---

## Check-Out Information

Typical information includes:

- Checkout Date
- Checkout Time
- Final Room
- Clearance Status
- Inspection Reference
- Remarks

Historical residency records shall remain immutable.

---

# 25. Mess Allocation

## Purpose

Mess Allocation governs assignment of hostel residents to institutional dining facilities.

This domain manages operational allocation only.

Meal planning, billing, and catering operations remain intentionally lightweight.

---

## Mess Allocation Workflow

```text
Resident Active
      │
Mess Allocation
      │
Effective Period
      │
Allocation Updated
      │
Allocation Closed
```

---

## Allocation Information

Typical allocation includes:

- Mess
- Meal Plan
- Effective Date
- Allocation Status
- Operational Remarks

Institutions may configure additional allocation attributes.

---

## Allocation Changes

Residents may request allocation changes.

Approval policies shall remain configurable.

Allocation history shall remain permanently available.

---

# 26. Hostel Complaints

## Purpose

Hostel Complaints govern operational issues reported within hostel premises.

Facilities Management executes maintenance.

Hostel Management records and coordinates complaints.

---

## Complaint Sources

Complaints may originate from:

- Residents
- Wardens
- Hostel Staff
- Security
- Administrative Users

---

## Complaint Categories

Examples include:

- Room Issue
- Cleanliness
- Furniture
- Water Supply
- Electricity
- Internet
- Safety
- Behaviour
- Noise
- General Complaint

Institutions may configure additional complaint categories.

---

## Complaint Workflow

```text
Complaint Registered
         │
Classification
         │
Operational Review
         │
Maintenance Required?
     ┌──────┴──────┐
     │             │
    No            Yes
     │             │
Operational   Maintenance
Resolution    Recommendation
                  │
                  ▼
         Manual G01 Service Request
```

---

## Complaint Resolution

Operational resolution may include:

- Administrative Action
- Resident Communication
- Hostel Policy Action
- Maintenance Recommendation
- Complaint Closure

Maintenance work remains external.

---

## Complaint History

Every complaint shall preserve:

- Reporter
- Room
- Category
- Severity
- Resolution
- Supporting Evidence
- Audit History

---

# 27. Operational Notifications

## Purpose

Operational Notifications inform residents, wardens, administrators, and other stakeholders regarding significant hostel activities.

Notification delivery remains within Section H.

Hostel Management publishes business events only.

---

## Notification Sources

Notifications may originate from:

- Hostel Admission
- Room Allocation
- Check-In
- Leave Approval
- Return Reminder
- Curfew Alert
- Visitor Approval
- Complaint Updates
- Room Transfer
- Exit Clearance
- Administrative Notices

---

## Typical Notifications

Examples include:

- Hostel Admission Approved
- Room Allocated
- Check-In Reminder
- Leave Approved
- Return Due Reminder
- Visitor Approved
- Curfew Reminder
- Complaint Updated
- Room Transfer Approved
- Exit Clearance Approved

Templates shall remain configurable.

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

Hostel Management shall not implement delivery channels.

---

# 28. Domain Events

Hostel Management publishes immutable operational events.

Every significant business transition shall publish a domain event.

---

## Admission Events

| Event | Description |
|--------|-------------|
| HostelAdmissionApplied | Hostel application submitted |
| HostelAdmissionApproved | Admission approved |
| HostelAdmissionRejected | Admission rejected |
| HostelAdmissionWaitlisted | Waiting list updated |

---

## Allocation Events

| Event | Description |
|--------|-------------|
| RoomAllocated | Room allocated |
| RoomReallocated | Room changed |
| OccupancyActivated | Residency activated |
| OccupancyClosed | Residency closed |

---

## Leave Events

| Event | Description |
|--------|-------------|
| LeaveRequested | Leave submitted |
| LeaveApproved | Leave approved |
| ResidentDeparted | Resident left hostel |
| ResidentReturned | Resident returned |

---

## Visitor Events

| Event | Description |
|--------|-------------|
| VisitorRegistered | Visitor registered |
| VisitorApproved | Visitor approved |
| VisitorCheckedIn | Visitor entered |
| VisitorCheckedOut | Visitor exited |

---

## Complaint Events

| Event | Description |
|--------|-------------|
| HostelComplaintRegistered | Complaint recorded |
| HostelComplaintResolved | Complaint resolved |
| MaintenanceRecommended | Recommendation issued |

---

## Discipline Events

| Event | Description |
|--------|-------------|
| DisciplineIncidentReported | Incident recorded |
| WarningIssued | Warning issued |
| DisciplineClosed | Case closed |

---

## Exit Events

| Event | Description |
|--------|-------------|
| ExitClearanceRequested | Clearance initiated |
| ExitApproved | Clearance approved |
| ResidentCheckedOut | Checkout completed |

---

# 29. Service Contracts

Hostel Management exposes operational business services.

---

## Admission Services

- Apply for Hostel Admission
- Review Admission
- Approve Admission
- Reject Admission

---

## Residency Services

- Allocate Room
- Transfer Room
- Activate Occupancy
- Close Occupancy

---

## Leave Services

- Submit Leave
- Approve Leave
- Record Return
- Query Leave Status

---

## Visitor Services

- Register Visitor
- Approve Visitor
- Record Entry
- Record Exit

---

## Complaint Services

- Register Complaint
- Update Complaint
- Recommend Maintenance
- Close Complaint

---

## Exit Services

- Initiate Exit Clearance
- Record Inspection
- Approve Checkout
- Complete Checkout

All interactions shall occur through published service contracts.

---

# 30. State Transition Matrix

| Aggregate | Primary Lifecycle |
|-----------|-------------------|
| Hostel Admission | Applied → Under Review → Approved / Rejected / Waitlisted |
| Hostel Occupancy | Allocated → Checked-In → Active → Leaving → Checked-Out |
| Room Allocation | Created → Allocated → Active → Closed |
| Leave Request | Applied → Review → Approved → Resident Left → Returned → Closed |
| Visitor Entry | Registered → Approved → Checked-In → Checked-Out → Closed |
| Hostel Complaint | Registered → Reviewed → Resolved → Closed |
| Room Transfer | Requested → Approved → Completed |
| Exit Clearance | Requested → Verified → Approved → Completed |
| Mess Allocation | Assigned → Active → Changed → Closed |

Lifecycle extensions shall remain configuration driven.

# 31. Authorization Model

## Purpose

Hostel Management shall enforce enterprise authorization through the Platform Authorization Framework.

Authentication remains the responsibility of the Platform.

Hostel Management governs operational permissions only.

---

## Authorization Principles

Authorization shall support:

- Role Based Access Control (RBAC)
- Campus Scoped Authorization
- Hostel Scoped Authorization
- Department Scoped Authorization
- Delegated Responsibility
- Temporary Delegation
- Effective-Dated Permissions
- Multi-Institution Isolation

---

## Typical Operational Roles

Examples include:

- Hostel Administrator
- Chief Warden
- Hostel Warden
- Assistant Warden
- Hostel Office Staff
- Security Personnel
- Student
- Parent (Limited Access)
- Read Only Auditor

Institutions may define additional operational roles.

---

## Authorization Matrix

| Activity | Typical Authorization |
|----------|-----------------------|
| Apply for Hostel | Student |
| Approve Admission | Hostel Administrator / Warden |
| Allocate Room | Hostel Administrator |
| Approve Leave | Warden |
| Register Visitor | Security / Warden |
| Approve Visitor | Warden |
| Record Discipline | Warden |
| Register Complaint | Resident / Staff |
| Approve Checkout | Hostel Administrator |
| Configure Hostel Rules | Hostel Administrator |

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

# 32. Reporting Responsibilities

## Purpose

Hostel Management provides operational reporting supporting institutional hostel administration.

Analytical processing remains outside this domain.

---

## Operational Reports

The domain shall support reporting for:

- Hostel Occupancy
- Vacant Rooms
- Waiting List
- Resident Directory
- Leave Status
- Curfew Violations
- Visitor Register
- Complaint Register
- Room Transfers
- Exit Clearance Status

---

## Management Reports

Examples include:

- Hostel Utilisation Summary
- Campus Hostel Summary
- Gender-wise Occupancy
- Hostel Category Summary
- Admission Statistics
- Transfer Statistics
- Discipline Summary
- Complaint Summary
- Mess Allocation Summary

---

## Security Reports

Examples include:

- Current Residents
- Residents on Leave
- Late Entry Register
- Visitor History
- Emergency Contact Directory
- Emergency Occupancy List

---

## Executive Dashboards

Executive dashboards may include:

- Total Hostels
- Occupied Rooms
- Available Rooms
- Waiting List Count
- Leave Requests
- Active Visitors
- Open Complaints
- Discipline Cases

Dashboard composition shall remain configurable.

---

# 33. Configuration Management

## Purpose

Hostel Management shall support enterprise configuration without source code modification.

---

## Configurable Elements

Institutions may configure:

### Hostel Operations

- Hostel Categories
- Allocation Policies
- Waiting List Policies
- Transfer Policies
- Occupancy Rules

---

### Leave Management

- Leave Categories
- Approval Workflow
- Maximum Leave Duration
- Return Validation
- Curfew Rules

---

### Visitors

- Visitor Categories
- Approval Policies
- Visiting Hours
- Identity Requirements

---

### Discipline

- Discipline Categories
- Warning Levels
- Escalation Policies
- Approval Rules

---

### Complaints

- Complaint Categories
- Priority Levels
- Escalation Policies
- Resolution Templates

---

### Mess

- Mess Categories
- Meal Plans
- Allocation Rules
- Effective Period Rules

---

### Notifications

- Notification Templates
- Reminder Policies
- Escalation Rules
- Recipient Policies

---

## Configuration Principles

Configuration shall support:

- Version Control
- Effective Dating
- Institution Isolation
- Rollback
- Complete Audit History

---

# 34. Audit Responsibilities

## Purpose

Hostel Management shall maintain immutable operational audit history.

Every significant residency activity shall be auditable.

---

## Audit Coverage

Audit records shall include:

- User
- Timestamp
- Business Action
- Previous State
- New State
- Hostel Reference
- Room Reference
- Resident Reference
- Business Reason
- Source System

---

## Audited Activities

Examples include:

- Hostel Admission
- Room Allocation
- Check-In
- Leave Approval
- Visitor Approval
- Complaint Registration
- Room Transfer
- Discipline Recording
- Exit Clearance
- Configuration Changes

---

## Audit Principles

Audit history shall:

- Never be deleted
- Never be modified
- Support operational investigations
- Support regulatory review
- Support historical reconstruction

---

# 35. Exception Handling

## Purpose

Hostel Management shall manage operational exceptions while preserving business consistency.

---

## Typical Exceptions

Examples include:

- No Rooms Available
- Waiting List Required
- Invalid Room Allocation
- Leave Approval Failure
- Curfew Violation
- Visitor Restriction
- Active Complaint During Checkout
- Authorization Failure
- Configuration Error
- Invalid Occupancy State

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

- Administrative Override
- Manual Approval
- Alternative Room Allocation
- Waiting List Placement
- Hostel Transfer
- Maintenance Recommendation
- Resident Notification

Recovery behaviour shall remain configuration driven.

---

# 36. Performance & Scalability

Hostel Management shall support enterprise-scale institutional deployments.

The architecture shall support:

- Multi Institution
- Multi Campus
- Hundreds of Hostels
- Thousands of Rooms
- Large Resident Populations
- Shared Hostels
- Multiple Hostel Categories
- Large Visitor Volumes
- Historical Residency Records
- Future Smart Hostel Integration

The architecture shall remain horizontally scalable without domain redesign.

---

# 37. Glossary

| Term | Definition |
|------|------------|
| Hostel Admission | Operational approval granting hostel residency eligibility. |
| Hostel Occupancy | Active operational residency of a student within a hostel. |
| Room Allocation | Operational assignment of a hostel room to a resident. |
| Check-In | Formal commencement of hostel residency. |
| Check-Out | Formal completion of hostel residency. |
| Room Transfer | Operational reassignment of a resident to another room. |
| Leave Request | Temporary permission allowing absence from the hostel. |
| Curfew | Institution-defined latest permitted return time. |
| Visitor | Authorised external person visiting a resident. |
| Hostel Discipline | Operational management of hostel conduct and disciplinary actions. |
| Exit Clearance | Operational verification completed before hostel departure. |
| Mess Allocation | Assignment of a resident to a dining facility or meal plan. |

---

# Appendix A — Canonical Operational Flow

```text
Hostel Admission
        │
Room Allocation
        │
Check-In
        │
Active Occupancy
        │
 ┌──────┼─────────┬─────────┐
 ▼      ▼         ▼         ▼
Leave Visitors Complaints Discipline
        │
        ▼
Room Transfer
        │
        ▼
Exit Clearance
        │
        ▼
Check-Out
```

---

# Appendix B — Canonical Ownership Matrix

| Business Concept | Canonical Owner |
|------------------|-----------------|
| Hostel Building | G10 Infrastructure |
| Hostel Floor | G10 Infrastructure |
| Hostel Room | G10 Infrastructure |
| Infrastructure Capacity | G10 Infrastructure |
| Student Identity | Section C |
| Hostel Admission | G03 Hostel Management |
| Room Allocation | G03 Hostel Management |
| Hostel Occupancy | G03 Hostel Management |
| Leave Management | G03 Hostel Management |
| Visitor Management | G03 Hostel Management |
| Hostel Discipline | G03 Hostel Management |
| Room Transfer | G03 Hostel Management |
| Mess Allocation | G03 Hostel Management |
| Hostel Complaints | G03 Hostel Management |
| Exit Clearance | G03 Hostel Management |
| Maintenance | G01 Facilities |
| Employees | Section E |
| Finance & Billing | Finance Domain |
| Authentication | Section A |
| Notifications | Section H |

---

# Appendix C — Enterprise Design Principles

Hostel Management follows the enterprise architectural principles established by SutraERP.

- Domain-Driven Design (DDD)
- Event-Driven Architecture
- CQRS
- Canonical Domain Ownership
- Configuration over Customisation
- Manual First Operations
- Effective-Dated Configuration
- Immutable Audit History
- Multi-Campus Architecture
- Multi-Institution Architecture
- Audit First
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Future Smart Hostel Integration

---

**End of Document**

**L2-G03 — Hostel Management Workflows**

**Version:** 1.0  
**Status:** Architecture Draft (Ready for Architectural Review)