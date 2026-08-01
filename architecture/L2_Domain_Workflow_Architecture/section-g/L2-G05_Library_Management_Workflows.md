# L2-G05 — Library Workflows

**Repository:** SutraERP  
**Section:** G — Campus Operations  
**Domain:** G05 — Library Management  
**Document ID:** L2-G05  
**Version:** 1.0  
**Status:** Draft  
**Classification:** Level 2 Domain Architecture  
**Owner:** Campus Operations Domain  
**Document Owner:** Devang

**Upstream Dependencies**

- Section A — Platform Foundation
- Section B — Institution & Structure
- Section C — Student Lifecycle
- Section D — Academic Delivery
- Section E — People & Workforce
- Finance Domain

**Downstream Consumers**

- Student Portal
- Faculty Portal
- Parent Portal
- Finance
- Institutional Reporting
- Accreditation
- Notification Services

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
16. Library Membership
17. Catalogue Management
18. Library Copy Management
19. Circulation Management
20. Reservation Management
21. Fine & Penalty Management
22. Acquisition & Cataloguing
23. Reading Room Management
24. Stock Verification
25. Library Clearance
26. Notifications
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

The Library Management Domain governs operational management of institutional libraries throughout the complete library lifecycle.

The domain manages library membership, catalogue management, physical library collections, circulation, reservations, renewals, fines, acquisitions, cataloguing, stock verification, reading room operations, library clearance, operational reporting, and institutional library governance.

Library Management operates independently of financial accounting, digital learning repositories, and document management platforms.

Finance owns accounting and payment processing.

Student ownership remains within Section C.

Employee ownership remains within Section E.

This document defines:

- Library Membership
- Catalogue Management
- Library Collections
- Circulation
- Reservations
- Renewals
- Fines
- Acquisitions
- Cataloguing
- Reading Rooms
- Stock Verification
- Library Clearance
- Operational Reporting
- Domain Events
- Integration Contracts

Implementation technologies, APIs, databases, deployment architecture, and UI behaviour remain outside the scope of this document.

---

# 2. Architectural Position

Library Management represents the operational library services layer within SutraERP.

Repository Position

```text
Platform Foundation
        │
        ▼
Student Lifecycle
        │
        ▼
Library Management
        │
 ┌──────────┼─────────────┬─────────────┐
 ▼          ▼             ▼
Finance Notifications Institutional Reporting
```

Library Management governs operational library services.

It does not govern:

- Digital Learning Content
- Learning Management Systems
- Research Repository
- Document Repository
- Financial Accounting

---

# 3. Scope

Library Management owns:

- Library Membership
- Library Catalogue
- Physical Library Copies
- Circulation
- Book Issue
- Book Return
- Renewals
- Reservations
- Waiting Lists
- Fine Calculation
- Acquisition Workflow
- Cataloguing
- Library Sections
- Reading Rooms
- Stock Verification
- Library Clearance
- Library Notices
- Operational Dashboards
- Operational Audit

The domain serves as the canonical operational library engine for the institution.

---

# 4. Out of Scope

The following responsibilities remain outside this domain.

---

## Financial Accounting

Owned by Finance.

Examples:

- Payment Collection
- Ledger
- Accounting
- Receipts
- Refunds

Library Management calculates operational fines.

Finance owns accounting.

---

## Student Lifecycle

Owned by Section C.

Examples:

- Student Identity
- Student Registration
- Student Programme
- Student Status

Library Management consumes student information.

---

## Employee Management

Owned by Section E.

Examples:

- Librarians
- Library Assistants
- Administrative Staff

Library Management assigns operational responsibilities only.

---

## Learning Content

The following remain outside this domain.

Examples:

- LMS
- E-Learning Content
- Research Repository
- Digital Repository
- Document Management

Library Management may reference external resources.

---

## Authentication & Workflow

Authentication, authorization, workflow execution, notifications, audit infrastructure, and document management remain Platform responsibilities.

---

# 5. Domain Responsibilities

Library Management owns:

- Library Membership
- Catalogue Operations
- Copy Management
- Book Circulation
- Reservations
- Waiting Lists
- Renewals
- Fine Calculation
- Acquisition Workflow
- Cataloguing
- Reading Room Operations
- Stock Verification
- Library Clearance
- Library Notices
- Operational Dashboards
- Operational Audit

The domain maintains complete historical records throughout the library lifecycle.

---

# 6. Design Principles

## Operational Ownership

Library Management owns library operations.

Financial ownership remains outside this domain.

---

## Configuration over Customisation

Membership policies, circulation rules, loan periods, fine rules, reservation policies, catalogue categories, notification templates, approval workflows, and operational constraints shall remain configuration driven.

Institution-specific behaviour shall never require application customization.

---

## Manual First Operations

Many institutional library activities continue to be managed manually.

The architecture shall support manual operations while enabling configurable automation where appropriate.

---

## Event Driven Architecture

Every significant library activity shall publish immutable domain events.

Downstream systems consume published events.

They shall never manipulate Library Management state directly.

---

## Audit First

Every library operation shall produce immutable audit history.

Historical circulation records shall never be deleted.

---

## Enterprise Scalability

The architecture shall support:

- Multi Institution
- Multi Campus
- Multiple Library Branches
- Department Libraries
- Hostel Libraries
- Digital Resource References
- Future RFID
- Future QR Support
- Future AI Recommendations
- Future Self-Service Kiosks

without architectural redesign.

# 7. Domain Terminology

| Term | Definition |
|------|------------|
| Library | Organised institutional facility providing library services and learning resources. |
| Library Branch | Independent operational library within an institution or campus. |
| Catalogue | Canonical bibliographic record describing a library resource. |
| Library Copy | Individual physical copy of a catalogue resource available for circulation. |
| Accession Number | Institution-defined unique identifier assigned to a library copy. |
| Circulation | Operational process governing issue, renewal, return, and reservation of library resources. |
| Reservation | Request to borrow a currently unavailable library resource. |
| Waiting List | Ordered list of members awaiting resource availability. |
| Renewal | Extension of an active borrowing period. |
| Library Membership | Operational authorisation permitting a user to utilise library services. |
| Reading Room | Dedicated library area supporting on-premises study and reference usage. |
| Library Clearance | Operational verification ensuring all library obligations are fulfilled before institutional exit. |
| Stock Verification | Operational audit confirming physical library inventory. |
| Fine | Operational calculation for overdue or policy violations. |
| Library Notice | Operational communication published by library administration. |

---

# 8. Aggregate Design

## Aggregate Roots

Library Management consists of the following aggregate roots:

- Library Membership
- Catalogue
- Library Copy
- Circulation
- Reservation
- Fine
- Acquisition
- Cataloguing
- Reading Room
- Stock Verification
- Library Clearance

Each aggregate owns its operational lifecycle.

---

## Aggregate Responsibilities

### Library Membership

Owns:

- Membership Registration
- Membership Status
- Borrowing Eligibility
- Membership History

---

### Catalogue

Owns:

- Bibliographic Information
- Classification
- Categories
- Search Information
- Resource Metadata

---

### Library Copy

Owns:

- Physical Copy
- Accession Number
- Barcode
- Operational Availability
- Physical Condition

---

### Circulation

Owns:

- Book Issue
- Renewal
- Return
- Due Date
- Borrowing History

---

### Reservation

Owns:

- Reservation Request
- Waiting List
- Reservation Status
- Reservation Expiry

---

### Fine

Owns:

- Fine Calculation
- Penalty Rules
- Waiver Recommendation
- Outstanding Operational Dues

Financial accounting remains outside this domain.

---

### Acquisition

Owns:

- Acquisition Request
- Vendor Reference
- Approval Workflow
- Acquisition Status

Procurement remains external.

---

### Cataloguing

Owns:

- Resource Classification
- Catalogue Publication
- Metadata Verification
- Library Classification

---

### Reading Room

Owns:

- Reading Room Operations
- Operational Availability
- Reading Room Rules

Seat reservation remains intentionally outside scope.

---

### Stock Verification

Owns:

- Inventory Verification
- Missing Resource Identification
- Damaged Resource Recording
- Verification History

---

### Library Clearance

Owns:

- Outstanding Borrowings
- Outstanding Fines
- Pending Replacements
- Clearance Approval

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

### Section C — Student Lifecycle

Provides:

- Student Identity
- Student Status
- Student Programme
- Student Registration

Library Management consumes student information.

Student ownership remains external.

---

### Section E — People & Workforce

Provides:

- Librarians
- Library Assistants
- Administrative Staff

Library Management assigns operational responsibilities.

Personnel ownership remains external.

---

### Finance

Consumes:

- Fine Information
- Replacement Cost References
- Outstanding Library Dues
- Payment Status

Finance owns accounting and payment processing.

---

## Downstream Consumers

Library Management publishes information to:

### Student Portal

Consumes:

- Borrowed Books
- Due Dates
- Reservations
- Fine Status
- Library Dashboard

---

### Parent Portal

Consumes:

- Library Notifications
- Outstanding Library Dues
- Important Notices

---

### Institutional Reporting

Consumes:

- Library Statistics
- Circulation Reports
- Resource Utilisation
- Accreditation Reports

---

# 10. Domain Model

Library Management governs the complete operational lifecycle of institutional library resources.

```text
Catalogue
     │
Library Copy
     │
Membership
     │
Circulation
     │
 ┌────┼─────────┬──────────┐
 ▼    ▼         ▼          ▼
Renew Reservation Fine Stock Verification
     │
     ▼
Library Clearance
```

Supporting operational model:

```text
Circulation
      │
      ├── Membership
      ├── Library Copy
      ├── Reservation
      ├── Fine
      ├── Notifications
      └── Audit
```

---

# 11. Lifecycle Overview

## Membership Lifecycle

```text id="g05life1"
Applied
    │
Approved
    │
Active
    │
Suspended
    │
Closed
```

---

## Circulation Lifecycle

```text id="g05life2"
Issued
    │
Active
    │
Renewed
    │
Returned
    │
Closed
```

---

## Reservation Lifecycle

```text id="g05life3"
Requested
      │
Waiting
      │
Available
      │
Collected
      │
Closed
```

---

## Fine Lifecycle

```text id="g05life4"
Calculated
      │
Pending
      │
Paid / Waived
      │
Closed
```

---

## Library Clearance Lifecycle

```text id="g05life5"
Requested
      │
Verification
      │
Outstanding Review
      │
Approved
      │
Completed
```

---

# 12. Workflow Catalogue

Library Management governs the following workflow families.

## Membership Operations

- Membership Registration
- Membership Approval
- Membership Suspension
- Membership Closure

---

## Catalogue Operations

- Catalogue Creation
- Catalogue Update
- Classification
- Catalogue Publication

---

## Circulation Operations

- Book Issue
- Book Return
- Renewal
- Borrowing History

---

## Reservation Operations

- Reservation Request
- Waiting List
- Reservation Allocation
- Reservation Closure

---

## Fine Operations

- Fine Calculation
- Fine Waiver Recommendation
- Outstanding Dues
- Fine Closure

---

## Acquisition Operations

- Acquisition Request
- Vendor Recommendation
- Acquisition Approval
- Cataloguing

---

## Library Operations

- Reading Room
- Stock Verification
- Library Notices
- Library Clearance

---

# 13. Business Invariants

The following invariants shall always remain true.

- Every Library Copy shall reference exactly one Catalogue record.
- Every Circulation transaction shall reference one active Library Copy.
- Every active borrowing shall reference one valid Library Member.
- Borrowing history shall never be deleted.
- Reservation shall reference a valid Catalogue resource.
- Fine calculation shall remain within Library Management.
- Finance owns accounting and payment processing.
- Every lifecycle transition shall be audited.
- Historical catalogue information shall never be deleted.
- Library Clearance shall verify all outstanding obligations before approval.

Violation of business invariants shall reject the transaction.

---

# 14. Architectural Constraints

Library Management shall not:

- Execute Financial Accounting
- Manage Student Lifecycle
- Manage Employees
- Replace Digital Repository
- Replace Learning Management System
- Replace Platform Workflow Engine
- Replace Notification Services
- Replace Authentication Services

Library Management operates exclusively through published business contracts.

---

# 15. Repository Principles

Library Management conforms to the following enterprise principles.

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
- Multiple Library Branch Support
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Future RFID Ready
- Future AI Ready

# 16. Library Membership Workflows

## Purpose

Library Membership governs eligibility and operational access to institutional library services.

Membership authorises borrowing, reservations, renewals, reading room access, and library operations according to institutional policies.

---

## Membership Categories

Supported membership categories may include:

- Student
- Faculty
- Staff
- Research Scholar
- Alumni
- Guest Member
- External Member

Membership categories shall remain configurable.

---

## Membership Workflow

```text
Membership Requested
         │
Eligibility Validation
         │
Approval
         │
Membership Activated
         │
Library Services Available
```

---

## Membership Eligibility

Eligibility validation may consider:

- User Status
- Institution
- Campus
- Programme
- Outstanding Obligations
- Administrative Approval

Eligibility policies shall remain configurable.

---

## Membership Status

Supported statuses include:

- Pending
- Active
- Suspended
- Expired
- Cancelled

Status values shall remain configurable.

---

## Member Dashboard

Every authorised member shall have access to a Library Dashboard.

Typical information includes:

- Active Borrowings
- Borrowing History
- Reserved Resources
- Due Dates
- Renewal Eligibility
- Outstanding Fines
- Payment Status (Reference)
- Library Notices
- Recommendations (Future)

Finance owns payment processing.

Library Management displays operational status only.

---

# 17. Catalogue Management

## Purpose

Catalogue Management governs bibliographic information describing institutional library resources.

Catalogue records represent intellectual works.

Physical copies are managed separately.

---

## Catalogue Workflow

```text
Resource Identified
        │
Metadata Prepared
        │
Classification
        │
Approval
        │
Catalogue Published
```

---

## Catalogue Information

Typical catalogue information includes:

- Title
- Author
- Publisher
- Edition
- ISBN
- Subject
- Language
- Keywords
- Classification
- Resource Type

Institutions may configure additional metadata.

---

## Resource Types

Examples include:

- Books
- Journals
- Magazines
- Newspapers
- Thesis
- Dissertation
- Project Reports
- Maps
- Audio Visual Media
- Question Papers

Additional resource types shall remain configurable.

---

## Classification

Supported classification systems may include:

- Dewey Decimal Classification
- Library of Congress Classification
- Institution-specific Classification

Classification systems shall remain configurable.

---

## Multilingual Support

Catalogue information may support multiple languages.

Language configuration shall remain institution configurable.

---

# 18. Library Copy Management

## Purpose

Library Copy Management governs operational handling of physical library resources.

Each physical copy references one catalogue record.

---

## Copy Workflow

```text
Catalogue Available
        │
Copy Added
        │
Accession Number Assigned
        │
Barcode Assigned
        │
Available For Circulation
```

---

## Copy Information

Typical information includes:

- Accession Number
- Barcode
- Library Branch
- Library Section
- Rack
- Shelf
- Physical Condition
- Operational Status

Detailed shelf positioning shall remain intentionally simple.

---

## Operational Status

Supported statuses include:

- Available
- Issued
- Reserved
- Reference Only
- Under Verification
- Lost
- Damaged
- Withdrawn

Status values shall remain configurable.

---

## Physical Location

Library Management may record:

- Library Branch
- Section
- Rack
- Shelf

The architecture intentionally avoids unnecessary micro-positioning.

---

# 19. Circulation Management

## Purpose

Circulation Management governs issue, renewal, return, and operational borrowing of library resources.

It forms the core operational workflow of Library Management.

---

## Circulation Workflow

```text
Member Verified
      │
Copy Available
      │
Issue Approved
      │
Borrowing Active
      │
Renewal / Return
      │
Circulation Closed
```

---

## Borrowing Validation

Validation may consider:

- Active Membership
- Borrowing Limit
- Outstanding Dues
- Resource Availability
- Reservation Priority
- Administrative Restrictions

Validation rules shall remain configurable.

---

## Borrowing Policies

Policies may define:

- Maximum Borrowings
- Borrowing Duration
- Renewal Eligibility
- Reservation Priority
- Restricted Resources

Policy definitions shall remain configurable.

---

## Renewal

Renewals may be permitted when:

- No Reservation Exists
- Renewal Limit Not Exceeded
- Membership Active
- Resource Eligible

Renewal rules shall remain configurable.

---

## Return Processing

Return processing may record:

- Return Date
- Return Time
- Physical Condition
- Damage Observations
- Lost Resource Information
- Outstanding Fine

Historical circulation records shall remain immutable.

---

# 20. Reservation Management

## Purpose

Reservation Management governs requests for unavailable library resources.

Reservations ensure equitable access to limited resources.

---

## Reservation Workflow

```text
Reservation Requested
         │
Waiting List
         │
Copy Available
         │
Member Notified
         │
Collection
         │
Reservation Closed
```

---

## Reservation Validation

Validation may consider:

- Active Membership
- Existing Borrowings
- Reservation Limits
- Resource Availability
- Duplicate Reservations

Validation policies shall remain configurable.

---

## Waiting List

Waiting lists shall support:

- Queue Position
- Reservation Date
- Priority
- Notification Status
- Reservation Expiry

Queue processing policies shall remain configurable.

---

## Reservation Status

Supported statuses include:

- Requested
- Waiting
- Ready for Collection
- Collected
- Expired
- Cancelled

Status values shall remain configurable.

---

# 21. Fine & Penalty Management

## Purpose

Fine Management governs operational calculation of library penalties.

Accounting and payment processing remain within Finance.

---

## Fine Workflow

```text
Due Date Exceeded
        │
Fine Calculated
        │
Member Notified
        │
Finance Reference
        │
Payment Status Updated
        │
Fine Closed
```

---

## Fine Categories

Examples include:

- Overdue Fine
- Lost Resource
- Damaged Resource
- Replacement Charge
- Administrative Penalty

Categories shall remain configurable.

---

## Fine Calculation

Fine calculation may consider:

- Resource Category
- Membership Category
- Grace Period
- Number of Days
- Institutional Policies

Calculation rules shall remain configurable.

---

## Fine Waiver

Waivers may require:

- Administrative Review
- Approval Workflow
- Reason Recording
- Audit History

Financial approval remains within institutional policy.

---

## Payment Status

Library Management may display:

- Outstanding
- Partially Paid
- Fully Paid
- Waived

Payment execution remains within Finance.

---

# 22. Acquisition & Cataloguing

## Purpose

Acquisition and Cataloguing govern introduction of new resources into the institutional library.

Procurement remains outside this domain.

---

## Acquisition Workflow

```text
Acquisition Requested
         │
Vendor Reference
         │
Approval
         │
Resource Received
         │
Cataloguing
         │
Library Copy Created
```

---

## Acquisition Sources

Resources may originate from:

- Purchase
- Donation
- Exchange
- Institutional Publication
- Government Distribution

Acquisition sources shall remain configurable.

---

## Vendor References

Library Management may maintain operational references for:

- Publisher
- Book Supplier
- Distributor
- Donor

Vendor ownership remains external.

---

## Cataloguing Workflow

```text
Resource Received
       │
Metadata Entry
       │
Classification
       │
Catalogue Verification
       │
Catalogue Published
```

---

## Accession

Every physical copy may receive:

- Accession Number
- Barcode
- Branch Assignment
- Shelf Assignment
- Operational Status

Accession numbering shall remain configurable.

# 23. Reading Room Management

## Purpose

Reading Room Management governs operational management of institutional reading areas supporting on-premises study.

Reading Rooms support reference materials and quiet study.

Detailed seat reservation remains outside the current architectural scope.

---

## Reading Room Workflow

```text
Reading Room Available
          │
Operational Monitoring
          │
Usage Supervision
          │
Reading Session
          │
Operational Closure
```

---

## Reading Room Categories

Examples include:

- General Reading Room
- Silent Reading Room
- Reference Reading Room
- Research Reading Room
- Digital Reading Room

Institutions may configure additional categories.

---

## Reading Room Rules

Operational rules may include:

- Operating Hours
- Silence Policy
- Reference Material Usage
- Food Restrictions
- Device Usage
- Entry Restrictions

Rules shall remain configuration driven.

---

## Reading Room Availability

Reading Rooms may become unavailable due to:

- Maintenance
- Administrative Closure
- Examination Requirements
- Renovation
- Emergency Situations

Operational history shall remain preserved.

---

# 24. Stock Verification

## Purpose

Stock Verification governs operational verification of institutional library collections.

Verification ensures physical resources remain aligned with catalogue records.

---

## Verification Workflow

```text
Verification Scheduled
          │
Physical Verification
          │
Difference Analysis
          │
Missing Resources
          │
Verification Report
          │
Administrative Closure
```

---

## Verification Types

Supported verification types include:

- Complete Verification
- Section Verification
- Branch Verification
- Surprise Verification
- Annual Verification

Verification policies shall remain configurable.

---

## Verification Results

Verification may identify:

- Available Resources
- Missing Resources
- Damaged Resources
- Misplaced Resources
- Duplicate Records

Operational handling remains within Library Management.

---

## Missing Resource Workflow

```text
Resource Missing
       │
Investigation
       │
Administrative Review
       │
Recovered?
   ┌────┴─────┐
   │          │
  Yes        No
   │          │
Inventory   Write-Off Review
Updated
```

---

## Write-Off

Write-off recommendations may occur due to:

- Irrecoverable Loss
- Severe Damage
- Administrative Decision
- Obsolete Material

Final approval shall follow institutional policy.

---

# 25. Library Clearance

## Purpose

Library Clearance governs verification of library obligations before institutional exit.

Library Clearance supports:

- Student Graduation
- Student Transfer
- Student Exit
- Employee Separation

---

## Clearance Workflow

```text
Clearance Requested
         │
Borrowing Verification
         │
Outstanding Fine Verification
         │
Replacement Verification
         │
Administrative Review
         │
Library Clearance Approved
```

---

## Clearance Validation

Validation may consider:

- Outstanding Borrowings
- Active Reservations
- Outstanding Fines
- Replacement Obligations
- Pending Administrative Actions

Validation rules shall remain configurable.

---

## Clearance Status

Supported statuses include:

- Requested
- Under Review
- Pending
- Approved
- Rejected
- Completed

Status values shall remain configurable.

---

## Clearance Integration

Library Clearance may publish completion status to:

- Student Exit
- Employee Exit
- Graduation
- Transfer Workflow

Library Management remains the canonical owner of clearance status.

---

# 26. Operational Notifications

## Purpose

Operational Notifications inform library stakeholders regarding significant operational activities.

Notification delivery remains within Section H.

Library Management publishes business events only.

---

## Notification Sources

Notifications may originate from:

- Membership Approval
- Book Issue
- Due Date Reminder
- Overdue Reminder
- Reservation Available
- Fine Generated
- Fine Paid
- Library Notice
- Clearance Request
- Clearance Approved

---

## Typical Notifications

Examples include:

- Membership Activated
- Book Due Tomorrow
- Book Overdue
- Reserved Book Available
- Fine Generated
- Fine Payment Confirmed
- Library Closed
- New Resource Available
- Clearance Pending
- Clearance Completed

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

Library Management shall not implement notification delivery.

---

# 27. Domain Events

Library Management publishes immutable operational events.

Every significant business transition shall publish a domain event.

---

## Membership Events

| Event | Description |
|--------|-------------|
| MembershipRequested | Membership application submitted |
| MembershipApproved | Membership activated |
| MembershipSuspended | Membership suspended |
| MembershipClosed | Membership closed |

---

## Catalogue Events

| Event | Description |
|--------|-------------|
| CatalogueCreated | Catalogue published |
| CatalogueUpdated | Catalogue modified |
| CatalogueArchived | Catalogue archived |

---

## Circulation Events

| Event | Description |
|--------|-------------|
| ResourceIssued | Library resource issued |
| ResourceReturned | Resource returned |
| ResourceRenewed | Borrowing renewed |
| ResourceOverdue | Due date exceeded |

---

## Reservation Events

| Event | Description |
|--------|-------------|
| ReservationCreated | Reservation registered |
| ReservationAvailable | Resource available |
| ReservationCollected | Reservation fulfilled |
| ReservationExpired | Reservation expired |

---

## Fine Events

| Event | Description |
|--------|-------------|
| FineCalculated | Fine generated |
| FineWaiverRequested | Waiver requested |
| FineClosed | Fine resolved |

---

## Acquisition Events

| Event | Description |
|--------|-------------|
| AcquisitionRequested | Resource requested |
| ResourceCatalogued | Resource catalogued |
| LibraryCopyCreated | Physical copy created |

---

## Verification Events

| Event | Description |
|--------|-------------|
| VerificationStarted | Stock verification initiated |
| VerificationCompleted | Verification completed |
| MissingResourceIdentified | Missing resource detected |

---

## Clearance Events

| Event | Description |
|--------|-------------|
| ClearanceRequested | Clearance initiated |
| ClearanceApproved | Clearance approved |
| ClearanceCompleted | Clearance completed |

---

# 28. Service Contracts

Library Management exposes operational business services.

---

## Membership Services

- Register Membership
- Approve Membership
- Suspend Membership
- Query Membership

---

## Catalogue Services

- Create Catalogue
- Update Catalogue
- Publish Catalogue
- Query Catalogue

---

## Circulation Services

- Issue Resource
- Return Resource
- Renew Borrowing
- Query Borrowing

---

## Reservation Services

- Reserve Resource
- Cancel Reservation
- Allocate Reserved Resource
- Query Reservation

---

## Fine Services

- Calculate Fine
- Recommend Waiver
- Update Payment Status
- Query Outstanding Fine

---

## Acquisition Services

- Register Acquisition
- Complete Cataloguing
- Create Library Copy
- Query Acquisition

---

## Clearance Services

- Initiate Clearance
- Verify Obligations
- Approve Clearance
- Complete Clearance

All interactions shall occur through published service contracts.

---

# 29. State Transition Matrix

| Aggregate | Primary Lifecycle |
|-----------|-------------------|
| Library Membership | Applied → Approved → Active → Suspended → Closed |
| Catalogue | Draft → Verified → Published → Archived |
| Library Copy | Added → Available → Issued → Returned → Withdrawn |
| Circulation | Issued → Active → Renewed → Returned → Closed |
| Reservation | Requested → Waiting → Available → Collected → Closed |
| Fine | Calculated → Pending → Paid / Waived → Closed |
| Acquisition | Requested → Approved → Received → Catalogued → Completed |
| Stock Verification | Planned → In Progress → Verified → Closed |
| Library Clearance | Requested → Review → Approved → Completed |

Lifecycle extensions shall remain configuration driven.

# 30. Authorization Model

## Purpose

Library Management shall enforce enterprise authorization through the Platform Authorization Framework.

Authentication remains the responsibility of the Platform.

Library Management governs operational permissions only.

---

## Authorization Principles

Authorization shall support:

- Role Based Access Control (RBAC)
- Campus Scoped Authorization
- Library Branch Scoped Authorization
- Delegated Responsibility
- Temporary Delegation
- Effective-Dated Permissions
- Multi-Institution Isolation

---

## Typical Operational Roles

Examples include:

- Library Administrator
- Chief Librarian
- Librarian
- Library Assistant
- Circulation Desk Operator
- Student
- Faculty
- Staff
- Guest Member
- Read Only Auditor

Institutions may define additional operational roles.

---

## Authorization Matrix

| Activity | Typical Authorization |
|----------|-----------------------|
| Configure Library | Library Administrator |
| Create Catalogue | Librarian |
| Add Library Copy | Librarian |
| Issue Resource | Librarian / Circulation Desk |
| Return Resource | Librarian / Circulation Desk |
| Approve Membership | Librarian |
| Approve Fine Waiver | Library Administrator |
| Perform Stock Verification | Librarian |
| Approve Library Clearance | Library Administrator |
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

# 31. Reporting Responsibilities

## Purpose

Library Management provides operational reporting supporting institutional library governance.

Analytical processing remains outside this domain.

---

## Operational Reports

The domain shall support reporting for:

- Active Memberships
- Library Catalogue
- Available Resources
- Issued Resources
- Reserved Resources
- Overdue Resources
- Outstanding Fines
- Stock Verification
- Library Clearance
- Acquisition Status

---

## Management Reports

Examples include:

- Library Utilisation
- Branch-wise Statistics
- Resource Category Summary
- Circulation Summary
- Fine Summary
- Membership Summary
- Reading Room Usage
- Stock Verification Summary
- Clearance Summary

---

## Accreditation Reports

Examples include:

- Library Collection Statistics
- Resource Growth
- Circulation Statistics
- Library Infrastructure Usage
- Annual Stock Verification
- Resource Availability

---

## Executive Dashboards

Executive dashboards may include:

- Total Library Members
- Total Catalogue Records
- Total Physical Copies
- Resources Issued
- Overdue Resources
- Reservations Pending
- Outstanding Fines
- Library Clearance Pending

Dashboard composition shall remain configurable.

---

# 32. Configuration Management

## Purpose

Library Management shall support enterprise configuration without source code modification.

---

## Configurable Elements

Institutions may configure:

### Membership

- Membership Categories
- Borrowing Eligibility
- Membership Validity
- Approval Workflow

---

### Circulation

- Borrowing Limits
- Loan Duration
- Renewal Limits
- Grace Period
- Reservation Rules

---

### Catalogue

- Resource Categories
- Classification System
- Library Sections
- Accession Number Format

---

### Fine Management

- Fine Categories
- Calculation Rules
- Grace Period
- Waiver Policies

---

### Reading Room

- Reading Room Categories
- Operating Hours
- Usage Rules
- Silent Zone Policies

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

Library Management shall maintain immutable operational audit history.

Every significant library activity shall be auditable.

---

## Audit Coverage

Audit records shall include:

- User
- Timestamp
- Business Action
- Previous State
- New State
- Membership Reference
- Catalogue Reference
- Circulation Reference
- Business Reason
- Source System

---

## Audited Activities

Examples include:

- Membership Approval
- Catalogue Creation
- Copy Registration
- Resource Issue
- Resource Return
- Renewal
- Reservation
- Fine Calculation
- Stock Verification
- Library Clearance
- Configuration Changes

---

## Audit Principles

Audit history shall:

- Never be deleted
- Never be modified
- Support operational investigations
- Support accreditation
- Support historical reconstruction

---

# 34. Exception Handling

## Purpose

Library Management shall manage operational exceptions while preserving business consistency.

---

## Typical Exceptions

Examples include:

- Membership Inactive
- Borrowing Limit Exceeded
- Resource Unavailable
- Reservation Conflict
- Renewal Not Allowed
- Outstanding Fine
- Lost Resource
- Damaged Resource
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

- Administrative Override
- Alternative Resource Suggestion
- Reservation Placement
- Fine Waiver Recommendation
- Replacement Recommendation
- Manual Approval
- Member Notification

Recovery behaviour shall remain configuration driven.

---

# 35. Performance & Scalability

Library Management shall support enterprise-scale institutional deployments.

The architecture shall support:

- Multi Institution
- Multi Campus
- Multiple Library Branches
- Millions of Catalogue Records
- Large Physical Collections
- Thousands of Concurrent Members
- High Volume Circulation
- Long-Term Historical Records
- Future RFID Integration
- Future QR Integration
- Future AI Recommendations
- Future Self-Service Library Kiosks
- Future Digital Repository Integration

The architecture shall remain horizontally scalable without domain redesign.

---

# 36. Glossary

| Term | Definition |
|------|------------|
| Library Membership | Operational authorisation permitting use of institutional library services. |
| Catalogue | Canonical bibliographic description of a library resource. |
| Library Copy | Individual physical copy available for circulation. |
| Circulation | Operational lifecycle governing issue, renewal, and return of resources. |
| Reservation | Request for a currently unavailable library resource. |
| Fine | Operational penalty calculated according to institutional policy. |
| Acquisition | Operational workflow introducing new resources into the library. |
| Cataloguing | Preparation and publication of bibliographic records. |
| Stock Verification | Operational verification of physical library inventory. |
| Library Clearance | Operational verification ensuring all library obligations are completed before institutional exit. |

---

# Appendix A — Canonical Operational Flow

```text
Library Membership
        │
Catalogue Search
        │
Resource Issue
        │
Borrowing
        │
Renewal / Reservation
        │
Return
        │
Fine (If Required)
        │
Library Clearance
        │
Audit History
```

---

# Appendix B — Canonical Ownership Matrix

| Business Concept | Canonical Owner |
|------------------|-----------------|
| Library Membership | G05 Library Management |
| Catalogue | G05 Library Management |
| Library Copy | G05 Library Management |
| Circulation | G05 Library Management |
| Reservation | G05 Library Management |
| Fine Calculation | G05 Library Management |
| Acquisition Workflow | G05 Library Management |
| Cataloguing | G05 Library Management |
| Reading Room | G05 Library Management |
| Stock Verification | G05 Library Management |
| Library Clearance | G05 Library Management |
| Student Identity | Section C |
| Library Staff | Section E |
| Finance & Payments | Finance Domain |
| Authentication | Section A |
| Notifications | Section H |

---

# Appendix C — Enterprise Design Principles

Library Management follows the enterprise architectural principles established by SutraERP.

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
- Multiple Library Branch Support
- Audit First
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Future RFID Ready
- Future QR Ready
- Future AI Recommendations
- Future Self-Service Library Support
- Future Digital Repository Integration

---

**End of Document**

**L2-G05 — Library Management Workflows**

**Version:** 1.0  
**Status:** Architecture Draft (Ready for Architectural Review)
