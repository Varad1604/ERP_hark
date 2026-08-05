# L2-G04 — Transport Workflows

**Repository:** SutraERP  
**Section:** G — Campus Operations  
**Domain:** G04 — Transport Management  
**Document ID:** L2-G04  
**Version:** 1.0  
**Status:** Draft  
**Classification:** Level 2 Domain Architecture  
**Owner:** Campus Operations Domain  
**Document Owner:** Campus Operations — Transport

**Upstream Dependencies**

- Section A — Platform Foundation
- Section B — Institution & Structure
- Section C — Student Lifecycle
- Section D — Academic Delivery
- Section E — People & Workforce
- G01 — Facilities Management
- G10 — Infrastructure Management
- Finance Domain

**Downstream Consumers**

- Student Operations
- Parent Communication
- Finance
- Security
- Institutional Reporting
- Notification Services

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §3 Scope; §4 Out of Scope |
| Actors | Typical Operational Roles |
| Entities | §8 Aggregate Design; Aggregate Roots; Aggregate Responsibilities |
| Business Rules | §13 Business Invariants |
| Workflow | §12 Workflow Catalogue; §16 Route Management Workflows; §17 Stop Management Workflows |
| State Machine | §29 State Transition Matrix |
| Contracts | §28 Service Contracts |
| Events | §27 Domain Events; Event Driven Architecture; Route Events |
| Dependencies | §9 Dependencies & Integration (Bounded Context Relationships) |
| Locked Decisions | — not present — |

> **Gap:** 1 required section(s) have no counterpart in this document: Locked Decisions. Recorded, not authored.

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
16. Route Management
17. Stop Management
18. Transport Allocation
19. Driver Assignment
20. Trip Planning
21. Trip Operations
22. Vehicle Operational Management
23. Transport Pass Management
24. Route Change Management
25. Transport Complaints
26. Operational Notifications
27. Domain Events
28. Service Contracts
29. State Transition Matrix
30. Authorization Model
31. Reporting
32. Configuration
33. Audit
34. Exception Handling
35. Performance & Scalability
36. Glossary

---

# 1. Purpose

The Transport Management Domain governs operational transportation services provided by the institution.

The domain coordinates transport routes, transport stops, transport allocation, driver assignment, trip planning, daily trip execution, transport passes, operational transport services, and transport-related reporting.

Transport Management operates using institutional transport assets while remaining independent from fleet ownership, maintenance management, and financial accounting.

Maintenance execution remains within G01.

Personnel ownership remains within Section E.

Student ownership remains within Section C.

Financial accounting remains outside this domain.

This document defines:

- Route Management
- Stop Management
- Student Transport Allocation
- Staff Transport Allocation
- Guest Transport Allocation
- Driver Assignment
- Trip Planning
- Trip Execution
- Transport Pass Management
- Route Changes
- Operational Notifications
- Business Rules
- Domain Events
- Integration Contracts

Implementation technologies, APIs, databases, deployment architecture, and user interface behaviour remain outside the scope of this document.

---

# 2. Architectural Position

Transport Management represents the operational transportation layer within SutraERP.

Vehicle ownership remains outside this domain.

Personnel ownership remains within Section E.

Infrastructure ownership remains within G10.

Repository Position

```text
Platform Foundation
        │
        ▼
Student Lifecycle
        │
        ▼
Transport Management
        │
 ┌──────┼────────────┬─────────────┐
 ▼      ▼            ▼
Finance Security Notifications
```

Transport Management governs transport operations.

It does not govern vehicle ownership or maintenance.

---

# 3. Scope

Transport Management owns:

- Route Management
- Stop Management
- Route Sequencing
- Student Transport Allocation
- Staff Transport Allocation
- Guest Transport Allocation
- Driver Assignment
- Trip Planning
- Trip Execution
- Transport Passes
- Operational Vehicle Assignment
- Route Diversions
- Route Suspension
- Transport Complaints
- Operational Reporting
- Operational Audit

The domain serves as the canonical operational transportation engine for the institution.

---

# 4. Out of Scope

The following responsibilities remain outside this domain.

---

## Fleet Ownership

Vehicle ownership remains outside Transport Management.

Examples:

- Vehicle Procurement
- Vehicle Registration
- Vehicle Asset Lifecycle
- Vehicle Disposal
- Asset Depreciation

Transport Management consumes operational vehicle information.

---

## Maintenance

Owned by G01.

Examples:

- Vehicle Repairs
- Preventive Maintenance
- Breakdown Maintenance
- Work Orders
- Vendor Repairs

Transport Management may recommend maintenance.

Maintenance execution remains external.

---

## Student Lifecycle

Owned by Section C.

Examples:

- Student Identity
- Student Registration
- Student Programme
- Student Status

Transport Management allocates transport only.

---

## Employee Management

Owned by Section E.

Examples:

- Drivers
- Conductors
- Transport Supervisors

Transport Management assigns operational duties only.

---

## Financial Accounting

Owned by Finance.

Examples:

- Transport Billing
- Fee Collection
- Ledger
- Refunds
- Accounting

Transport Management provides operational allocation information.

---

## Authentication & Workflow

Authentication, authorization, notification delivery, workflow execution, audit infrastructure, and document management remain Platform responsibilities.

---

# 5. Domain Responsibilities

Transport Management owns:

- Route Operations
- Stop Operations
- Transport Allocation
- Driver Assignment
- Trip Planning
- Trip Execution
- Transport Pass Management
- Route Changes
- Operational Vehicle Assignment
- Transport Complaints
- Parent Notifications
- Operational Dashboards
- Operational Audit

The domain maintains complete operational transport history throughout the transport lifecycle.

---

# 6. Design Principles

## Operational Ownership

Transport Management owns transport operations.

Vehicle ownership remains outside this domain.

---

## Configuration over Customisation

Transport routes, stop policies, allocation policies, transport categories, approval workflows, pass formats, notification rules, operational schedules, and transport policies shall remain configuration driven.

Institution-specific behaviour shall never require application customization.

---

## Manual First Operations

Institutional transport operations frequently rely upon manual coordination.

The architecture shall support manual operational decisions while enabling optional automation.

Driver replacement, vehicle replacement, route diversion, and trip adjustments shall remain manually overridable.

---

## Event Driven Architecture

Every significant transport activity shall publish immutable domain events.

Downstream systems consume transport events.

They shall never manipulate transport state directly.

---

## Audit First

Every transport activity shall produce immutable operational audit history.

Historical transport operations shall never be deleted.

---

## Enterprise Scalability

The architecture shall support:

- Multi Campus
- Multi Institution
- Student Transport
- Staff Transport
- Guest Transport
- Shared Transport Services
- Future GPS Integration
- Future Driver Mobile Applications
- Future AI Route Optimisation

without architectural redesign.

# 7. Domain Terminology

| Term | Definition |
|------|------------|
| Transport Route | Operational path followed by institutional transport services. |
| Transport Stop | Designated pickup or drop location along a transport route. |
| Trip | Scheduled operational journey executed on a transport route. |
| Route Sequence | Configured order in which transport stops are serviced. |
| Transport Allocation | Assignment of transport services to students, staff, or guests. |
| Driver Assignment | Operational assignment of a driver to a transport trip. |
| Vehicle Assignment | Operational assignment of an available vehicle to a transport trip. |
| Transport Pass | Operational credential authorising transport usage. |
| Route Diversion | Temporary operational modification of a transport route. |
| Transport Complaint | Operational issue reported regarding transport services. |
| Trip Schedule | Planned operational timing of transport services. |
| Pickup Point | Operational boarding location for transport users. |
| Drop Point | Operational alighting location for transport users. |
| Transport Supervisor | Operational authority responsible for transport coordination. |
| Guest Transport | Temporary transport allocation provided to authorised visitors or guests. |

---

# 8. Aggregate Design

## Aggregate Roots

Transport Management consists of the following aggregate roots:

- Transport Route
- Transport Stop
- Trip
- Transport Allocation
- Driver Assignment
- Vehicle Assignment
- Transport Pass
- Route Diversion
- Transport Complaint

Each aggregate owns its operational lifecycle.

---

## Aggregate Responsibilities

### Transport Route

Owns:

- Route Definition
- Route Sequencing
- Operational Status
- Assigned Stops
- Route Availability

---

### Transport Stop

Owns:

- Stop Information
- Route Association
- Pickup Rules
- Drop Rules
- Operational Status

---

### Trip

Owns:

- Trip Schedule
- Vehicle Assignment
- Driver Assignment
- Trip Execution
- Trip Completion

---

### Transport Allocation

Owns:

- Student Allocation
- Staff Allocation
- Guest Allocation
- Allocation Status
- Allocation History

---

### Driver Assignment

Owns:

- Driver Assignment
- Operational Responsibility
- Assignment Status
- Substitution History

Driver ownership remains within Section E.

---

### Vehicle Assignment

Owns:

- Operational Vehicle Assignment
- Assignment Duration
- Operational Availability
- Replacement History

Vehicle ownership remains outside this domain.

---

### Transport Pass

Owns:

- Pass Information
- Validity
- Assigned Transport Service
- Operational Status

---

### Route Diversion

Owns:

- Diversion Reason
- Temporary Route
- Effective Period
- Operational Status

---

### Transport Complaint

Owns:

- Complaint Registration
- Classification
- Operational Resolution
- Maintenance Recommendation

---

## Aggregate Identity

Every aggregate shall possess:

- Immutable System Identifier
- Configurable Business Code
- Human Readable Reference

Aggregate identity shall remain immutable throughout its lifecycle.

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
- Document Management
- Configuration Framework
- Event Bus

---

### Section C — Student Lifecycle

Provides:

- Student Identity
- Student Status
- Student Programme
- Student Registration

Transport Management consumes student information.

Student ownership remains external.

---

### Section E — People & Workforce

Provides:

- Drivers
- Conductors
- Transport Staff
- Supervisors

Transport Management performs operational assignments only.

---

### G01 — Facilities Management

Provides:

- Maintenance
- Work Orders
- Vendor Services
- Breakdown Handling

Transport Management recommends maintenance where required.

Maintenance execution remains within G01.

---

### Finance

Consumes:

- Transport Allocation
- Transport Pass
- Active Service
- Transport Category

Finance owns transport billing.

---

## Downstream Consumers

Transport Management publishes operational information to:

### Parent Communication

Consumes:

- Route Assignment
- Transport Allocation
- Emergency Notifications

---

### Security

Consumes:

- Vehicle Movements
- Emergency Transport
- Operational Alerts

---

### Institutional Reporting

Consumes:

- Route Utilisation
- Transport Statistics
- Operational Reports

---

# 10. Domain Model

Transport Management governs operational transportation services.

```text
Transport Route
        │
        ▼
Transport Stops
        │
        ▼
Trip Planning
        │
 ┌──────┼────────────┬──────────┐
 ▼      ▼            ▼
Vehicle Driver Transport Allocation
        │
        ▼
Trip Execution
        │
        ▼
Transport Complaint (If Required)
        │
        ▼
Trip Completion
```

Supporting operational model:

```text
Trip
   │
   ├── Driver Assignment
   ├── Vehicle Assignment
   ├── Route
   ├── Stops
   ├── Pass Validation
   ├── Complaints
   └── Audit
```

---

# 11. Lifecycle Overview

## Route Lifecycle

```text
Draft
   │
Configured
   │
Active
   │
Suspended
   │
Retired
```

---

## Trip Lifecycle

```text
Planned
    │
Scheduled
    │
Ready
    │
In Progress
    │
Completed
```

---

## Allocation Lifecycle

```text
Applied
    │
Approved
    │
Allocated
    │
Active
    │
Withdrawn
```

---

## Complaint Lifecycle

```text
Registered
      │
Reviewed
      │
Resolved
      │
Closed
```

---

## Transport Pass Lifecycle

```text
Issued
    │
Active
    │
Suspended
    │
Expired
```

---

# 12. Workflow Catalogue

Transport Management governs the following workflow families.

## Route Operations

- Route Creation
- Route Modification
- Route Activation
- Route Suspension

---

## Stop Operations

- Stop Registration
- Stop Assignment
- Route Sequencing
- Stop Suspension

---

## Allocation Operations

- Student Allocation
- Staff Allocation
- Guest Allocation
- Allocation Withdrawal

---

## Driver Operations

- Driver Assignment
- Driver Replacement
- Operational Reassignment

---

## Trip Operations

- Trip Planning
- Trip Scheduling
- Trip Execution
- Trip Completion

---

## Transport Pass Operations

- Pass Issuance
- Pass Renewal
- Pass Suspension
- Pass Replacement

---

## Complaint Operations

- Complaint Registration
- Complaint Classification
- Maintenance Recommendation
- Complaint Resolution

---

## Route Change Operations

- Route Diversion
- Temporary Stops
- Emergency Route Changes

---

# 13. Business Invariants

The following invariants shall always remain true.

- Every Trip shall reference exactly one Route.
- Every Route shall contain at least one Stop.
- Every Allocation shall reference an active Route.
- Driver ownership shall remain within Section E.
- Vehicle ownership shall remain outside this domain.
- Maintenance shall never be executed by Transport Management.
- Every Complaint shall reference a transport operation.
- Every lifecycle transition shall be audited.
- Historical trip information shall never be deleted.
- Finance consumes transport allocation but does not modify transport operations.

Violation of business invariants shall reject the transaction.

---

# 14. Architectural Constraints

Transport Management shall not:

- Own Vehicle Assets
- Execute Vehicle Maintenance
- Manage Student Lifecycle
- Manage Employees
- Execute Financial Accounting
- Replace Platform Workflow Engine
- Replace Notification Services
- Replace Authentication Services

Transport Management operates exclusively through published business contracts.

---

# 15. Repository Principles

Transport Management conforms to the following enterprise principles.

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
- Future GPS Ready
```

# 16. Route Management Workflows

## Purpose

Route Management governs creation, maintenance, activation, and operational management of institutional transport routes.

Routes represent the canonical transport paths followed during daily transport operations.

---

## Route Components

A transport route may include:

- Route Code
- Route Name
- Route Category
- Campus
- Direction
- Stops
- Operational Distance
- Estimated Duration
- Operational Status

Additional attributes shall remain configurable.

---

## Route Workflow

```text
Route Created
      │
Stop Assignment
      │
Sequence Validation
      │
Operational Review
      │
Route Activated
```

---

## Route Status

Supported operational states include:

- Draft
- Active
- Suspended
- Temporarily Diverted
- Retired

Status values shall remain configurable.

---

## Route Sequencing

Every route shall maintain an operational stop sequence.

Sequence modifications may occur due to:

- Traffic Conditions
- Road Closures
- Administrative Decisions
- Temporary Diversions
- Emergency Situations

Historical route versions shall remain preserved.

---

## Route Suspension

Routes may be temporarily suspended due to:

- Holidays
- Low Demand
- Vehicle Unavailability
- Administrative Decision
- Emergency Situation

Suspension shall not delete historical trip information.

---

# 17. Stop Management Workflows

## Purpose

Stop Management governs operational management of transport pickup and drop locations.

Stops remain reusable across multiple transport routes.

---

## Stop Workflow

```text
Stop Created
      │
Location Verified
      │
Route Assignment
      │
Operational Validation
      │
Stop Activated
```

---

## Stop Information

Typical stop information includes:

- Stop Code
- Stop Name
- GPS Reference (Future Ready)
- Pickup Availability
- Drop Availability
- Campus
- Operational Status

Additional information shall remain configurable.

---

## Stop Categories

Examples include:

- Campus Stop
- Residential Stop
- Public Pickup Point
- Hostel Stop
- Staff Stop
- Temporary Stop

Institutions may configure additional categories.

---

## Stop Availability

Stops may become temporarily unavailable due to:

- Road Closure
- Construction
- Administrative Restriction
- Emergency Diversion

Temporary restrictions shall preserve historical stop information.

---

# 18. Transport Allocation Workflows

## Purpose

Transport Allocation governs assignment of transport services to authorised users.

Allocation supports students, staff, and authorised guests.

---

## Allocation Types

Supported allocation types include:

- Student Transport
- Staff Transport
- Guest Transport
- Temporary Transport
- Emergency Transport

Additional allocation types shall remain configurable.

---

## Allocation Workflow

```text
Application Submitted
        │
Eligibility Validation
        │
Route Selection
        │
Approval
        │
Transport Activated
```

---

## Allocation Validation

Validation may consider:

- User Status
- Campus
- Route Availability
- Stop Availability
- Transport Category
- Administrative Approval

Validation rules shall remain configuration driven.

---

## Allocation Changes

Authorised users may request:

- Route Change
- Stop Change
- Temporary Suspension
- Transport Withdrawal

Approval policies shall remain configurable.

---

## Allocation History

The platform shall permanently preserve:

- Route
- Stop
- Allocation Date
- Withdrawal Date
- Change History
- Administrative Remarks

Historical allocation information shall remain immutable.

---

# 19. Driver Assignment Workflows

## Purpose

Driver Assignment governs operational assignment of drivers to transport trips.

Personnel ownership remains within Section E.

Transport Management governs operational responsibility.

---

## Driver Assignment Workflow

```text
Trip Planned
      │
Driver Availability
      │
Operational Assignment
      │
Trip Ready
```

---

## Assignment Principles

Assignments may consider:

- Driver Availability
- Campus
- Vehicle Category
- Operational Shift
- Route Familiarity
- Administrative Assignment

Assignment policies shall remain configurable.

---

## Driver Replacement

Driver replacement may occur due to:

- Leave
- Illness
- Emergency
- Administrative Decision
- Operational Requirement

Replacement shall normally occur through manual reassignment.

---

## Assignment History

Historical assignment information shall preserve:

- Driver
- Trip
- Assignment Time
- Replacement History
- Administrative Remarks

Assignment history shall never be deleted.

---

# 20. Trip Planning Workflows

## Purpose

Trip Planning governs operational preparation of daily transport services.

Trip Planning converts operational routes into executable transport activities.

---

## Trip Workflow

```text
Route Selected
      │
Vehicle Assigned
      │
Driver Assigned
      │
Trip Generated
      │
Operational Review
      │
Trip Ready
```

---

## Trip Planning Sources

Trips may originate from:

- Daily Schedule
- Academic Calendar
- Examination Schedule
- Special Event
- Administrative Request
- Emergency Requirement

Scheduling sources shall remain configurable.

---

## Trip Information

Typical trip information includes:

- Trip Number
- Route
- Vehicle
- Driver
- Planned Departure
- Planned Arrival
- Operational Status

---

## Trip Scheduling

Transport Management shall support:

- Morning Trips
- Afternoon Trips
- Evening Trips
- Night Trips
- Special Trips

Scheduling policies shall remain configurable.

---

## Trip Cancellation

Trips may be cancelled due to:

- Administrative Decision
- Holiday
- Vehicle Unavailability
- Low Operational Demand
- Emergency Situation

Cancelled trips shall remain part of operational history.

---

# 21. Trip Operations

## Purpose

Trip Operations govern execution and operational monitoring of transport services.

Trip execution remains intentionally lightweight.

The platform records operational activities without imposing unnecessary digital processes.

---

## Trip Execution Workflow

```text
Trip Ready
     │
Departure
     │
Route Execution
     │
Stop Operations
     │
Arrival
     │
Trip Completed
```

---

## Pre-Trip Checklist

Transport Management may provide configurable pre-trip guidance.

Examples include:

- Fuel Level
- Tyre Condition
- Brake Check
- Lights
- First Aid Kit
- Fire Extinguisher
- Documents Available
- Vehicle Cleanliness

Checklist completion may remain optional according to institutional policy.

---

## Post-Trip Checklist

Operational guidance may include:

- Vehicle Inspection
- Damage Observation
- Fuel Recording
- Lost & Found Review
- Cleaning Requirement
- Operational Remarks

Checklist policies shall remain configurable.

---

## Trip Completion

Operational completion may include:

- Departure Time
- Arrival Time
- Operational Remarks
- Route Deviations
- Replacement Information
- Complaint References

Historical trip execution shall remain permanently available.

---

# 22. Vehicle Operational Management

## Purpose

Vehicle Operational Management governs operational assignment and availability of vehicles.

Vehicle ownership remains outside this domain.

Maintenance execution remains within G01.

---

## Operational Status

Supported operational states include:

- Available
- Assigned
- In Trip
- Restricted
- Replacement Assigned
- Unavailable

Status values shall remain configurable.

---

## Vehicle Assignment Workflow

```text
Vehicle Available
        │
Trip Assignment
        │
Operational Validation
        │
Vehicle Assigned
```

---

## Vehicle Replacement

Replacement vehicles may be assigned due to:

- Breakdown
- Maintenance Recommendation
- Administrative Decision
- Emergency Requirement

Replacement history shall remain permanently available.

---

## Maintenance Recommendation

Where operational issues are identified:

```text
Vehicle Issue
      │
Operational Assessment
      │
Maintenance Recommendation
      │
Administrative Review
      │
Manual G01 Service Request
```

Transport Management recommends maintenance.

Facilities Management executes maintenance.

---

## Operational References

Transport Management may reference:

- Registration Number
- Vehicle Category
- Seating Capacity
- Insurance Status
- Permit Status
- Fitness Certificate

Operational references shall not replace vehicle asset ownership.

# 23. Transport Pass Management

## Purpose

Transport Pass Management governs operational issuance, renewal, suspension, replacement, and lifecycle management of institutional transport passes.

Financial ownership remains outside this domain.

The transport pass represents operational authorization to use institutional transport services.

---

## Pass Workflow

```text
Transport Allocation Approved
          │
Pass Generated
          │
Pass Issued
          │
Active Usage
          │
Renewal / Replacement
          │
Expired / Withdrawn
```

---

## Pass Information

Typical transport pass information includes:

- Pass Number
- Pass Holder
- Transport Category
- Assigned Route
- Assigned Stop
- Effective Date
- Expiry Date
- Operational Status

Additional attributes shall remain configurable.

---

## Pass Status

Supported pass states include:

- Draft
- Issued
- Active
- Suspended
- Expired
- Cancelled
- Replaced

Status values shall remain configurable.

---

## Pass Replacement

Replacement may occur due to:

- Lost Pass
- Damaged Pass
- Route Change
- Student Status Change
- Administrative Decision

Replacement history shall remain permanently available.

---

# 24. Route Change Management

## Purpose

Route Change Management governs temporary and permanent operational changes to transport routes and transport allocations.

Academic schedules remain unaffected.

---

## Route Change Types

Supported operational changes include:

- Route Diversion
- Stop Addition
- Stop Removal
- Temporary Stop Skip
- Emergency Route
- Permanent Route Revision

Additional change types shall remain configurable.

---

## Route Change Workflow

```text
Change Requested
        │
Operational Review
        │
Impact Assessment
        │
Approval
        │
Route Updated
        │
Stakeholders Notified
```

---

## Change Validation

Validation may consider:

- Active Trips
- Route Capacity
- Vehicle Availability
- Driver Assignment
- Student Allocation
- Administrative Approval

Validation rules shall remain configuration driven.

---

## Emergency Diversion

Emergency diversions may occur due to:

- Road Closure
- Traffic Incident
- Weather Conditions
- Public Events
- Administrative Direction

Historical route information shall remain preserved.

---

# 25. Transport Complaints

## Purpose

Transport Complaints govern operational issues reported regarding institutional transport services.

Maintenance execution remains within G01.

Transport Management coordinates complaint resolution.

---

## Complaint Sources

Complaints may originate from:

- Students
- Parents
- Staff
- Drivers
- Conductors
- Transport Supervisors

---

## Complaint Categories

Examples include:

- Vehicle Condition
- Driver Behaviour
- Delay
- Missed Stop
- Route Issue
- Safety Concern
- Cleanliness
- Overcrowding
- Operational Service

Institutions may configure additional categories.

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
- Driver Counselling
- Route Modification
- Passenger Communication
- Maintenance Recommendation
- Complaint Closure

Maintenance work remains external.

---

## Complaint History

Every complaint shall preserve:

- Reporter
- Route
- Trip
- Category
- Severity
- Resolution
- Supporting Evidence
- Audit History

Historical complaint records shall remain immutable.

---

# 26. Operational Notifications

## Purpose

Operational Notifications inform transport stakeholders regarding transport activities requiring awareness or action.

Notification delivery remains within Section H.

Transport Management publishes business events only.

---

## Notification Sources

Notifications may originate from:

- Transport Allocation
- Route Changes
- Stop Changes
- Trip Cancellation
- Trip Completion
- Driver Assignment
- Vehicle Replacement
- Complaint Updates
- Emergency Transport
- Pass Expiry

---

## Typical Notifications

Examples include:

- Transport Allocation Approved
- Route Changed
- Stop Updated
- Trip Cancelled
- Emergency Route Activated
- Driver Reassigned
- Vehicle Replaced
- Complaint Updated
- Pass Expiry Reminder
- Transport Withdrawal Approved

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

Transport Management shall not implement notification delivery.

---

# 27. Domain Events

Transport Management publishes immutable operational events.

Every significant operational transition shall publish a domain event.

---

## Route Events

| Event | Description |
|--------|-------------|
| RouteCreated | Route created |
| RouteActivated | Route activated |
| RouteSuspended | Route suspended |
| RouteModified | Route updated |

---

## Stop Events

| Event | Description |
|--------|-------------|
| StopCreated | Stop registered |
| StopAssigned | Stop assigned to route |
| StopSuspended | Stop suspended |

---

## Allocation Events

| Event | Description |
|--------|-------------|
| TransportAllocated | Transport allocation completed |
| TransportAllocationUpdated | Allocation modified |
| AllocationWithdrawn | Allocation withdrawn |

---

## Driver Events

| Event | Description |
|--------|-------------|
| DriverAssigned | Driver assigned |
| DriverReassigned | Driver replaced |
| DriverAssignmentCompleted | Assignment completed |

---

## Trip Events

| Event | Description |
|--------|-------------|
| TripPlanned | Trip generated |
| TripStarted | Trip commenced |
| TripCompleted | Trip completed |
| TripCancelled | Trip cancelled |

---

## Vehicle Events

| Event | Description |
|--------|-------------|
| VehicleAssigned | Vehicle assigned |
| VehicleReplaced | Replacement vehicle assigned |
| MaintenanceRecommended | Maintenance recommendation issued |

---

## Pass Events

| Event | Description |
|--------|-------------|
| PassIssued | Pass generated |
| PassRenewed | Pass renewed |
| PassSuspended | Pass suspended |
| PassReplaced | Replacement issued |

---

## Complaint Events

| Event | Description |
|--------|-------------|
| TransportComplaintRegistered | Complaint recorded |
| ComplaintResolved | Complaint resolved |
| OperationalIssueEscalated | Issue escalated |

---

# 28. Service Contracts

Transport Management exposes operational business services.

---

## Route Services

- Create Route
- Update Route
- Suspend Route
- Query Route

---

## Stop Services

- Register Stop
- Update Stop
- Assign Stop
- Query Stop

---

## Allocation Services

- Allocate Transport
- Update Allocation
- Withdraw Allocation
- Query Allocation

---

## Driver Services

- Assign Driver
- Replace Driver
- Query Driver Assignment

---

## Trip Services

- Plan Trip
- Update Trip
- Complete Trip
- Cancel Trip

---

## Pass Services

- Issue Pass
- Renew Pass
- Replace Pass
- Suspend Pass

---

## Complaint Services

- Register Complaint
- Update Complaint
- Recommend Maintenance
- Close Complaint

All interactions shall occur through published service contracts.

---

# 29. State Transition Matrix

| Aggregate | Primary Lifecycle |
|-----------|-------------------|
| Transport Route | Draft → Configured → Active → Suspended → Retired |
| Transport Stop | Created → Assigned → Active → Suspended |
| Transport Allocation | Applied → Approved → Allocated → Active → Withdrawn |
| Driver Assignment | Planned → Assigned → Active → Completed |
| Trip | Planned → Scheduled → Ready → In Progress → Completed |
| Vehicle Assignment | Available → Assigned → Active → Released |
| Transport Pass | Draft → Issued → Active → Suspended / Expired |
| Transport Complaint | Registered → Reviewed → Resolved → Closed |

Lifecycle extensions shall remain configuration driven.

# 30. Authorization Model

## Purpose

Transport Management shall enforce enterprise authorization through the Platform Authorization Framework.

Authentication remains the responsibility of the Platform.

Transport Management governs operational permissions only.

---

## Authorization Principles

Authorization shall support:

- Role Based Access Control (RBAC)
- Campus Scoped Authorization
- Route Scoped Authorization
- Delegated Responsibility
- Temporary Delegation
- Effective-Dated Permissions
- Multi-Institution Isolation

---

## Typical Operational Roles

Examples include:

- Transport Administrator
- Transport Manager
- Transport Supervisor
- Driver
- Conductor
- Transport Coordinator
- Student
- Parent (Limited Access)
- Read Only Auditor

Institutions may define additional operational roles.

---

## Authorization Matrix

| Activity | Typical Authorization |
|----------|-----------------------|
| Configure Routes | Transport Administrator |
| Configure Stops | Transport Administrator |
| Allocate Transport | Transport Administrator |
| Assign Driver | Transport Manager |
| Assign Vehicle | Transport Manager |
| Approve Route Changes | Transport Manager |
| Register Complaint | Student / Staff / Parent |
| Review Complaint | Transport Supervisor |
| Issue Transport Pass | Transport Coordinator |
| View Operational Reports | Management |

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

# 31. Reporting Responsibilities

## Purpose

Transport Management provides operational reporting supporting institutional transportation services.

Analytical processing remains outside this domain.

---

## Operational Reports

The domain shall support reporting for:

- Active Routes
- Active Stops
- Transport Allocations
- Daily Trip Schedule
- Trip Completion
- Driver Assignments
- Vehicle Assignments
- Transport Passes
- Route Diversions
- Transport Complaints

---

## Management Reports

Examples include:

- Route Utilisation
- Vehicle Utilisation
- Driver Assignment Summary
- Student Transport Summary
- Staff Transport Summary
- Guest Transport Summary
- Route Performance
- Complaint Summary
- Trip Statistics

---

## Executive Dashboards

Executive dashboards may include:

- Total Routes
- Total Stops
- Active Trips
- Transport Users
- Active Drivers
- Vehicle Availability
- Open Complaints
- Route Diversions

Dashboard composition shall remain configurable.

---

## Parent & Student Views

Operational views may provide:

- Assigned Route
- Pickup Stop
- Drop Stop
- Assigned Vehicle
- Assigned Driver
- Pass Status
- Transport Notifications

Visibility shall remain role based.

---

# 32. Configuration Management

## Purpose

Transport Management shall support enterprise configuration without source code modification.

---

## Configurable Elements

Institutions may configure:

### Route Management

- Route Categories
- Route Policies
- Operational Status
- Diversion Policies

---

### Stop Management

- Stop Categories
- Pickup Rules
- Drop Rules
- Stop Availability

---

### Transport Allocation

- Allocation Categories
- Approval Workflow
- Eligibility Rules
- Suspension Policies

---

### Trip Operations

- Trip Categories
- Operational Shifts
- Holiday Behaviour
- Special Trip Policies

---

### Driver Assignment

- Assignment Rules
- Replacement Policies
- Operational Roles

---

### Transport Pass

- Pass Format
- Pass Categories
- Renewal Policies
- Replacement Policies

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

# 33. Audit Responsibilities

## Purpose

Transport Management shall maintain immutable operational audit history.

Every significant transport activity shall be auditable.

---

## Audit Coverage

Audit records shall include:

- User
- Timestamp
- Business Action
- Previous State
- New State
- Route Reference
- Trip Reference
- Allocation Reference
- Business Reason
- Source System

---

## Audited Activities

Examples include:

- Route Creation
- Stop Assignment
- Transport Allocation
- Driver Assignment
- Vehicle Assignment
- Trip Planning
- Pass Issuance
- Route Diversion
- Complaint Registration
- Configuration Changes

---

## Audit Principles

Audit history shall:

- Never be deleted
- Never be modified
- Support operational investigations
- Support compliance reviews
- Support historical reconstruction

---

# 34. Exception Handling

## Purpose

Transport Management shall manage operational exceptions while preserving business consistency.

---

## Typical Exceptions

Examples include:

- Route Unavailable
- Stop Unavailable
- Vehicle Unavailable
- Driver Unavailable
- Allocation Conflict
- Route Diversion
- Trip Cancellation
- Authorization Failure
- Configuration Error
- Invalid Operational State

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
- Manual Driver Replacement
- Manual Vehicle Replacement
- Temporary Route Diversion
- Alternative Stop Assignment
- Maintenance Recommendation
- Passenger Notification

Recovery behaviour shall remain configuration driven.

---

# 35. Performance & Scalability

Transport Management shall support enterprise-scale institutional deployments.

The architecture shall support:

- Multi Institution
- Multi Campus
- Hundreds of Routes
- Thousands of Transport Users
- Multiple Daily Trips
- Student Transport
- Staff Transport
- Guest Transport
- Future GPS Integration
- Future Mobile Applications
- Future AI Route Optimisation

The architecture shall remain horizontally scalable without domain redesign.

---

# 36. Glossary

| Term | Definition |
|------|------------|
| Transport Route | Operational transport path followed by institutional vehicles. |
| Transport Stop | Approved pickup or drop location on a transport route. |
| Trip | Scheduled operational journey executed on a route. |
| Transport Allocation | Assignment of transport services to an authorised user. |
| Driver Assignment | Operational assignment of a driver to a transport trip. |
| Vehicle Assignment | Operational assignment of a vehicle to a transport trip. |
| Transport Pass | Operational authorisation allowing use of transport services. |
| Route Diversion | Temporary operational modification of a transport route. |
| Transport Complaint | Operational issue reported regarding transport services. |
| Trip Schedule | Planned operational execution of transport services. |

---

# Appendix A — Canonical Operational Flow

```text
Transport Allocation
        │
Route Assignment
        │
Stop Assignment
        │
Driver Assignment
        │
Vehicle Assignment
        │
Trip Planning
        │
Trip Execution
        │
Complaint (If Required)
        │
Trip Completion
        │
Audit History
```

---

# Appendix B — Canonical Ownership Matrix

| Business Concept | Canonical Owner |
|------------------|-----------------|
| Vehicle Asset | External Fleet / Asset Domain |
| Route | G04 Transport Management |
| Stop | G04 Transport Management |
| Trip | G04 Transport Management |
| Transport Allocation | G04 Transport Management |
| Driver Assignment | G04 Transport Management |
| Vehicle Assignment | G04 Transport Management |
| Transport Pass | G04 Transport Management |
| Route Diversion | G04 Transport Management |
| Transport Complaints | G04 Transport Management |
| Student Identity | Section C |
| Driver & Staff | Section E |
| Vehicle Maintenance | G01 Facilities |
| Transport Billing | Finance Domain |
| Authentication | Section A |
| Notifications | Section H |

---

# Appendix C — Enterprise Design Principles

Transport Management follows the enterprise architectural principles established by SutraERP.

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
- Future GPS Ready
- Future Mobile Application Support
- Future AI Route Optimisation

---

**End of Document**

**L2-G04 — Transport Management Workflows**

**Version:** 1.0  
**Status:** Draft
