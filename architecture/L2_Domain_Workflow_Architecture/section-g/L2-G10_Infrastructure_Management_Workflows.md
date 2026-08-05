# L2-G10 — Infrastructure Management Workflows

**Repository:** SutraERP  
**Section:** G — Campus Operations  
**Domain:** G10 — Infrastructure Management  
**Document ID:** L2-G10  
**Version:** 1.0  
**Status:** Draft  
**Classification:** Level 2 Domain Architecture  
**Owner:** Campus Operations Domain  
**Document Owner:** Devang  
**Dependencies:** Section A – Platform Foundation, Section B – Institution & Structure (Institutional Campus Governance Reference), Section E – People & Workforce

---





## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §3 Scope; §4 Out of Scope |
| Actors | 48A.204 Infrastructure Roles |
| Entities | Child Entities; 8B.44 Entity Relationships |
| Business Rules | §47 Business Rules; 13A.62 Business Rule Philosophy; 13A.63 Business Rule Categories |
| Workflow | §12 Workflow Catalogue; §16 Infrastructure Registration Workflows; §17 Campus Management Workflows |
| State Machine | 11A.51 Canonical Lifecycle States; 11A.52 Lifecycle State Definitions |
| Contracts | §45 Service Contracts; 44A. Infrastructure Events, Integration Contracts & External Interfaces; 13A.77 Public Contract Rules |
| Events | §44 Domain Events; 44A. Infrastructure Events, Integration Contracts & External Interfaces; Event-Driven Architecture |
| Dependencies | §9 Dependencies & Integration (Bounded Context Relationships); 11A.56 Lifecycle Dependency Rules |
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
16. (Continued in Part 2)

---

# 1. Purpose

The Infrastructure Management Domain is the canonical business owner of physical institutional infrastructure within SutraERP.

It provides the authoritative representation of physical infrastructure including:

- Physical Campus Structures
- Buildings
- Blocks
- Wings
- Floors
- Operational Spaces
- Infrastructure Capacity
- Infrastructure Availability
- Infrastructure Lifecycle
- Compliance Status
- Infrastructure Allocation

Institutional campus governance, institution-level campus definition, and Institution-to-Campus association remain owned by Section B — Institution & Structure.

Infrastructure Management establishes the enterprise-wide physical model used by operational business domains.

This document defines:

- Physical infrastructure ownership
- Physical hierarchy
- Infrastructure lifecycle
- Space lifecycle
- Operational allocation
- Capacity governance
- Availability governance
- Compliance governance
- Maintenance governance
- Reservation governance
- Infrastructure configuration
- Domain events
- Architectural boundaries
- Business invariants
- Enterprise responsibilities

This document intentionally excludes implementation design, database structure, APIs, UI behaviour, and technology-specific concerns.

---

# 2. Architectural Position

Infrastructure Management is the canonical physical infrastructure domain of SutraERP.

Every physical location referenced anywhere within the ERP shall originate from this domain.

The Infrastructure Management Domain acts as the enterprise Digital Physical Twin of the institution at the business architecture level.

The domain owns physical infrastructure representation and lifecycle.

The domain does not own institutional campus governance.

Section B — Institution & Structure remains responsible for:

- Institution-level campus definition.
- Institutional campus governance.
- Institution-to-campus association.
- Institutional governance metadata.

Within G10, the term Physical Campus represents the physical infrastructure boundary containing buildings, spaces, and operational facilities.

It does not represent institutional governance ownership.

Repository Position

```text
Platform Foundation (Section A)
          │
          ▼
Institution & Structure (Section B)
          │
          ▼
Infrastructure Management (G10)
          │
 ┌────────┼────────┬────────┬────────┬────────┬────────┐
 ▼        ▼        ▼        ▼        ▼        ▼
Facilities Hostel Library Laboratory Security Examination
```

Infrastructure serves as the canonical provider of physical context for downstream business domains.

No downstream domain shall redefine physical infrastructure already owned by this domain.

---

# 3. Scope

Infrastructure Management owns the complete business lifecycle of institutional physical infrastructure.

This includes the physical representation of campus structures but excludes institutional campus governance.

Responsibilities include:

- Physical Campus hierarchy management
- Infrastructure registration
- Building management
- Zone management
- Wing and Block management
- Floor management
- Space management
- Infrastructure identity
- Physical classification
- Operational allocation
- Department allocation
- Shared infrastructure governance
- Capacity management
- Infrastructure availability
- Infrastructure reservation
- Compliance management
- Inspection governance
- Infrastructure lifecycle
- Decommissioning
- Archival
- Infrastructure audit
- Infrastructure configuration
- Infrastructure event publication

Infrastructure Management is the authoritative source of truth for physical institutional infrastructure.

---

# 4. Out of Scope

The following responsibilities are intentionally excluded.

## Academic Operations

Owned by Section D.

Examples:

- Teaching Sessions
- Timetable
- Course Offerings
- Faculty Assignment

Infrastructure provides physical locations only.

---

## Student Lifecycle

Owned by Section C.

Examples:

- Student records
- Student registration
- Student progression
- Student attendance

Infrastructure supplies physical context only.

---

## Facilities Maintenance Execution

Owned by G01.

Examples:

- Complaint handling
- Work Orders
- Repair execution
- Vendor assignment
- AMC execution

Infrastructure owns infrastructure identity.

Facilities owns operational maintenance activities.

---

## Hostel Operations

Owned by G03.

Examples:

- Hostel allocation
- Room occupancy
- Hostel transfers

Infrastructure owns hostel buildings and spaces.

Hostel owns accommodation operations.

---

## Laboratory Operations

Owned by G02.

Examples:

- Laboratory scheduling
- Practical management
- Equipment usage

Infrastructure owns laboratory spaces.

Laboratory Management owns laboratory operations.

---

## Library Operations

Owned by G05.

Infrastructure owns library spaces.

Library owns circulation, cataloguing, and member services.

---

## Security Operations

Owned by G07.

Infrastructure owns security locations.

Security owns operational access control, visitor management, patrols, and incident response.

---

# 5. Domain Responsibilities

Infrastructure Management is responsible for maintaining the enterprise representation of institutional physical assets and spaces.

Primary responsibilities include:

- Infrastructure registration
- Infrastructure lifecycle
- Space governance
- Infrastructure availability
- Capacity governance
- Department allocation
- Shared infrastructure governance
- Reservation governance
- Compliance governance
- Inspection governance
- Infrastructure classification
- Infrastructure auditability
- Infrastructure event publication
- Configuration governance
- Enterprise physical consistency

Infrastructure shall maintain complete historical integrity throughout its lifecycle.

---

# 6. Design Principles

The Infrastructure Management Domain follows the enterprise architectural principles established by Platform Foundation.

## Canonical Ownership

Infrastructure shall be the single canonical owner of physical institutional infrastructure.

No downstream domain shall create independent physical location models.

---

## Single Source of Truth

Every physical location referenced anywhere within SutraERP shall originate from Infrastructure Management.

---

## Digital Physical Twin

Infrastructure represents the enterprise Digital Physical Twin of institutional infrastructure at the business architecture level.

Every operational module references Infrastructure rather than duplicating physical models.

---

## Event-Driven Architecture

Every significant business state transition shall publish immutable domain events.

Downstream systems consume events.

They shall never directly manipulate Infrastructure state.

---

## Configuration over Customisation

Institution-specific behaviour shall be controlled through configuration.

Business logic shall never be hardcoded.

---

## Effective Dating

Operational allocations, responsibility assignments, and configurable policies shall support effective dating where business scenarios require historical traceability.

---

## Complete Auditability

Every lifecycle transition shall produce immutable audit history.

Infrastructure history shall never be deleted.

Historical reconstruction shall always remain possible.

---

## Enterprise Scalability

The domain shall support:

- Multi-campus institutions
- Multi-institution deployments
- Shared infrastructure
- Multi-country deployments
- Institution-specific configuration
- Future enterprise expansion

The architecture shall remain independent of deployment size.

---
# 7. Domain Terminology

| Term | Definition |
|------|------------|
| Infrastructure | The complete physical environment represented by Infrastructure Management including physical campus structures, buildings, spaces, supporting facilities, and operational physical resources. |
| Physical Campus | A physical infrastructure boundary managed by Infrastructure Management containing buildings, spaces, supporting facilities, and operational resources. Institutional campus governance remains owned by Section B. |
| Zone | An optional logical grouping of buildings or spaces within a campus used for operational, security, or administrative purposes. |
| Building | A permanent physical structure owned or managed by the Institution. |
| Wing | A logical subdivision of a Building used for operational organisation. |
| Block | A physical subdivision of a Building. Institutions may configure Wings or Blocks according to local conventions. |
| Floor | A physical level within a Building. |
| Space | The smallest canonical physical unit managed by Infrastructure Management. Every operational location within SutraERP shall reference a Space. |
| Space Type | A configurable classification describing the intended purpose of a Space. |
| Operational Capability | Activities that a Space is permitted to support such as Teaching, Examination, Workshop, Conference, or Events. |
| Infrastructure Allocation | Assignment of physical infrastructure to an organisational unit for operational responsibility. |
| Reservation | Temporary booking of a Space for a defined operational purpose. |
| Capacity | Maximum supported occupancy or utilisation of Infrastructure under configured business rules. |
| Infrastructure Availability | Operational state indicating whether Infrastructure may be consumed by downstream domains. |
| Custodian | Person responsible for administrative oversight of Infrastructure. |
| Operational Manager | Person responsible for day-to-day operational management of Infrastructure. |
| Compliance | Regulatory, statutory, institutional, or safety requirements applicable to Infrastructure. |
| Inspection | Planned or unplanned verification of Infrastructure condition or compliance. |
| Infrastructure Lifecycle | Controlled progression of Infrastructure through defined business states. |
| Shared Infrastructure | Infrastructure consumed by multiple organisational units simultaneously. |
| Canonical Ownership | Exclusive business ownership of Infrastructure maintained by this domain. |
| Digital Physical Twin | Business representation of the institution's physical environment used consistently across all domains. |

---

# 8. Aggregate Design

## Aggregate Root

```
Infrastructure
```

The Infrastructure aggregate represents the canonical owner of every physical location within the enterprise.

No downstream domain shall create independent physical location aggregates.

---

## Primary Aggregates

The Infrastructure domain owns the following physical infrastructure aggregates:

- Physical Campus
- Zone
- Building
- Wing / Block
- Floor
- Space
- Infrastructure Allocation
- Capacity Profile
- Availability Profile
- Reservation
- Compliance Record
- Inspection Record

Each aggregate owns its own lifecycle while remaining part of the Infrastructure bounded context.

The Physical Campus aggregate represents the physical infrastructure structure.

It must not be confused with the institutional Campus governance concept maintained by Section B.

Section B owns:

Institution → Campus Association

G10 owns:

Physical Campus → Building → Floor → Space hierarchy

---

## Aggregate Responsibilities

Infrastructure owns:

- Physical hierarchy
- Infrastructure identity
- Physical metadata
- Space classification
- Operational capabilities
- Capacity
- Availability
- Department allocation
- Shared infrastructure
- Compliance
- Inspection history
- Reservation governance
- Lifecycle
- Audit references

Infrastructure does **not** own:

- Students
- Faculty
- Timetable
- Course Delivery
- Hostel Occupancy
- Library Circulation
- Laboratory Operations
- Examination Scheduling
- Maintenance Execution
- Visitor Management

Those domains reference Infrastructure through published contracts.

---

## Aggregate Identity

Every Infrastructure aggregate shall possess:

- Immutable System Identifier
- Configurable Business Code
- Human Readable Name

Business Codes and Names may change according to institutional policy.

Aggregate Identity shall never change.

---

## 8A. Canonical Aggregate Definitions

> Migrated from `L2-B03_Campus_Workflows.md` on 2026-08-05 under integrity finding
> INT-008. B03 defined these physical aggregates in full while G10, their canonical
> owner, listed only their names. The definitions now live with the owner. B03 retains
> the InstitutionalCampus aggregate and references these by identity.

The aggregates named in Section 8 are defined below. Section 36 of the source document
described the *physical* campus site; it is titled Physical Campus Aggregate here to
distinguish it from the InstitutionalCampus governed by `L2-B03`.

---

## 8.1 Physical Campus Aggregate

## Purpose

Represents an independent physical campus.

---

## Aggregate Root

```text
Campus
```

---

## Child Entities

- Campus Address
- Campus Contact
- Campus Boundary
- Campus Facilities
- Campus Metadata

---

## Value Objects

- Campus Name
- Campus Code
- Geo Location
- Postal Address
- Time Zone

---

## Responsibilities

The Campus aggregate is responsible for:

- Campus Identity
- Campus Lifecycle
- Campus Availability
- Campus Status
- Campus Metadata

---

## Business Invariants

- Campus Code must be unique.
- Campus belongs to exactly one Institution.
- Campus lifecycle governs operational availability.
- Campus cannot be deleted once operational.

---

## 8.2 Building Aggregate

## Purpose

Represents a permanent physical structure.

---

## Aggregate Root

```text
Building
```

---

## Child Entities

- Building Wing (Optional child entity; not an Aggregate Root)
- Building Entrance
- Building Utilities
- Building Contacts

---

## Value Objects

- Building Name
- Building Code
- Building Classification
- Area Measurement

---

## Responsibilities

The Building aggregate owns:

- Building Identity
- Building Lifecycle
- Physical Characteristics
- Operational Characteristics
- Structural Metadata

---

## Business Invariants

- Building belongs to exactly one Campus.
- Building Code must be unique within Campus.
- Building cannot exist without Campus.
- Gross Area must be greater than zero.
- Construction Year cannot be in the future.

---

## 8.3 Floor Aggregate

## Purpose

Represents a floor within a building.

---

## Aggregate Root

```text
Floor
```

---

## Child Entities

- Floor Facilities
- Floor Metadata

---

## Value Objects

- Floor Number
- Floor Name
- Accessibility Profile

---

## Responsibilities

The Floor aggregate owns:

- Floor Identity
- Floor Ordering
- Accessibility Characteristics
- Operational Status

---

## Business Invariants

- Floor belongs to exactly one Building.
- Floor Number must be unique within Building.
- Floor lifecycle cannot exceed parent Building lifecycle.

---

## 8.4 Space Aggregate

## Purpose

Represents a usable physical location.

---

## Aggregate Root

```text
Space
```

---

## Child Entities

- Space Access Point
- Space Facility
- Space Feature
- Space Compliance Record

---

## Value Objects

- Capacity
- Area
- Accessibility Profile
- Availability Profile
- Criticality Profile
- Health Profile

---

## Responsibilities

The Space aggregate owns:

- Identity
- Classification
- Capacity
- Physical Characteristics
- Accessibility
- Availability
- Lifecycle
- Compliance
- Criticality
- Sustainability Capabilities

---

## Business Invariants

- Space belongs to exactly one Floor.
- Space Code must be unique within Building.
- Capacity cannot be negative.
- Area cannot be negative.
- Lifecycle transitions follow approved workflow.

---

## 8.5 Zone Aggregate

## Purpose

Represents an operational grouping of infrastructure.

Examples:

- North Zone
- Examination Zone
- Residential Zone
- Academic Zone

---

## Aggregate Root

```text
Zone
```

---

## Child Entities

- Zone Membership

---

## Value Objects

- Zone Name
- Zone Type

---

## Responsibilities

- Zone Membership
- Zone Metadata
- Zone Lifecycle

---

## Business Invariants

- Infrastructure may belong to multiple Zones.
- Zone membership never changes physical hierarchy.

---

## 8.6 Collection Aggregate

## Purpose

Represents logical business categorization.

Examples:

- Smart Classrooms
- Accessible Facilities
- Research Laboratories
- Heritage Buildings

---

## Aggregate Root

```text
Collection
```

---

## Child Entities

- Collection Membership

---

## Value Objects

- Collection Name
- Collection Category

---

## Responsibilities

- Business Classification
- Collection Membership
- Collection Metadata

---

## Business Invariants

Collections never own infrastructure.

Collections reference infrastructure.

---

## 8.7 Portfolio Aggregate

## Purpose

Represents executive governance grouping.

Examples:

- Capital Investment Portfolio
- Green Campus Portfolio
- Digital Infrastructure Portfolio

---

## Aggregate Root

```text
Portfolio
```

---

## Child Entities

- Portfolio Membership

---

## Value Objects

- Portfolio Name
- Investment Category
- Governance Category

---

## Responsibilities

- Strategic Grouping
- Investment Classification
- Executive Governance

---

## Business Invariants

Portfolio membership has no effect on physical hierarchy.

---

## 8.8 Shared Value Objects

The following Value Objects are reusable across multiple aggregates.

---

## Identity

Contains:

- Identifier
- Business Code
- Display Name

---

## Lifecycle

Contains:

- Current State
- Effective Date
- Activation Date
- Deactivation Date

---

## Capacity

Contains:

- Seating Capacity
- Standing Capacity
- Maximum Occupancy

---

## Area

Contains:

- Built-up Area
- Carpet Area
- Measurement Unit

---

## Accessibility

Contains:

- Wheelchair Access
- Lift Access
- Accessible Restroom
- Accessible Parking

---

## Criticality

Contains:

- Criticality Level
- Recovery Priority
- Maximum Downtime

---

## Sustainability

Contains:

- Solar Infrastructure
- Rainwater Harvesting
- Grey Water Recycling
- EV Charging
- Green Certification

---

## Health

Contains:

- Current Health
- Assessment Date
- Assessment Authority

---

## Compliance

Contains:

- Fire Compliance
- Structural Compliance
- Accessibility Compliance
- Regulatory Certifications

---

---


> Sections 8.9 and 8.10 were recovered on 2026-08-05: they existed only in the
> B03 Part 3 block removed during migration and had no Part 4 counterpart.

## 8.9 Infrastructure Unit

Infrastructure Unit is a conceptual business abstraction representing managed physical infrastructure and is NOT an Aggregate Root.

It represents any managed physical entity within institutional premises.

Examples include:

- Generator Room
- Parking Area
- Cafeteria
- Water Tank
- Sports Ground
- Amphitheatre
- Server Room
- Security Cabin

Every Infrastructure Unit belongs to exactly one physical parent.

---

## 8.10 Physical Hierarchy Rules

The following hierarchy is canonical.

```text
Campus
    │
Building
    │
Wing (Optional)
    │
Floor
    │
Space
```

Rules:

- A Campus contains multiple Buildings.
- A Building belongs to exactly one Campus.
- A Floor belongs to exactly one Building (or optionally to a Wing within that Building).
- A Wing is an optional child entity owned by the Building Aggregate and is not an independent Aggregate Root. It exists only where required by institutional building layouts; buildings without wings remain completely valid.
- A Space belongs to exactly one Floor.
- A Space cannot belong to multiple Floors.
- Circular relationships are prohibited.

---

# 8B. Aggregate Relationships, Communication & Evolution

> Migrated from `L2-B03_Campus_Workflows.md` Part 4 on 2026-08-05 under integrity finding
> INT-008, completing step 1. These sections state generic aggregate patterns but
> illustrate every one of them with Building, Floor and Space - G10's aggregates.

---

## 8B.44 Entity Relationships

```text
Campus
 │
 ├────────── Building
 │               │
 │               ├──────── Floor
 │               │              │
 │               │              └──────── Space
 │               │
 │               └──────── Utilities
 │
 ├──────── Zones
 │
 ├──────── Collections
 │
 └──────── Portfolios
```

---

## 8B.45 Aggregate Communication

Aggregates communicate using domain events.

Example:

```text
Building Activated

↓

BuildingActivated Event

↓

Floor Aggregate

↓

Operational Validation
```

No aggregate directly modifies another aggregate.

---

## 8B.46 Aggregate Transaction Rules

Each aggregate represents an independent transactional consistency boundary.

Rules:

- Transactions never span multiple aggregates.
- Cross-aggregate updates occur using events.
- Eventual consistency is acceptable across aggregates.
- Business invariants are enforced only within the owning aggregate.

---

## 8B.47 Aggregate Size Guidelines

Aggregates should remain cohesive and manageable.

Recommended approach:

- Campus: Coarse-grained
- Building: Medium-grained
- Floor: Lightweight
- Space: Rich aggregate
- Zone: Lightweight
- Collection: Lightweight
- Portfolio: Lightweight

Avoid large aggregates that own excessive unrelated responsibilities.

---

## 8B.48 Aggregate Evolution Principles

Future capabilities such as:

- Digital Twin
- IoT Devices
- Smart Sensors
- Building Automation
- GIS Mapping
- Predictive Maintenance

shall extend existing aggregates through composition or related domains.

Core aggregates defined in this document should remain stable over time.

---

---

# 9. Dependencies & Integration (Bounded Context Relationships)

## Upstream Domains

### Section A — Platform Foundation

Provides:

- Identity
- Authorization
- Workflow Engine
- Notification Framework
- Event Bus
- Audit Framework
- Document Management
- Configuration Framework

---

### Section B — Institution & Structure

Provides:

- Institution Identity
- Organisation Structure
- Institutional Campus Governance Reference
- Department Structure
- Academic Organisation Context

Infrastructure consumes institutional context but owns physical implementation.

Section B does not own:

- Buildings
- Floors
- Rooms
- Spaces
- Infrastructure Lifecycle
- Infrastructure Capacity
- Infrastructure Availability

---

### Section E — People & Workforce

Provides:

- Employee
- Position
- Workforce Engagement
- Vendor Workforce
- Responsibility Assignment

Infrastructure references workforce participants for custodianship and operational responsibility.

---

## Downstream Domains

Infrastructure publishes physical context to:

### G01 — Facilities

Consumes:

- Buildings
- Spaces
- Infrastructure Identity
- Compliance Context

Produces:

- Maintenance Status
- Work Order References

---

### G02 — Laboratory Management

Consumes:

- Laboratory Spaces
- Capacity
- Availability

Produces:

- Operational Utilisation

---

### G03 — Hostel Management

Consumes:

- Hostel Buildings
- Hostel Rooms
- Capacity

Produces:

- Occupancy Status

---

### G04 — Transport

Consumes:

- Parking Areas
- Vehicle Depots
- Transport Infrastructure

---

### G05 — Library

Consumes:

- Library Spaces
- Reading Areas
- Storage Areas

---

### G06 — Medical

Consumes:

- Medical Spaces
- Clinic Areas
- Emergency Rooms

---

### G07 — Security

Consumes:

- Buildings
- Security Zones
- Access Locations

Produces:

- Security Classification
- Incident References

---

### Section D — Academic Operations

Consumes:

- Teaching Spaces
- Academic Infrastructure
- Reservation Services

---

### Section I — Examination

Consumes:

- Examination Venues
- Capacity
- Availability
- Reservation Services

---

### Future Domains

Future enterprise domains shall reference Infrastructure rather than defining independent physical location models.

---

# 10. Domain Model

The Infrastructure domain models the complete physical enterprise hierarchy.

```
Institution
        │
        ▼
Institutional Campus Reference
        │
        ▼
Physical Campus
        │
        ▼
Zone (Optional)
        │
        ▼
Building
        │
        ▼
Wing / Block (Optional)
        │
        ▼
Floor
        │
        ▼
Space
```

Institutional Campus Reference is provided by Section B.

Physical Campus and all downstream physical infrastructure entities are owned by G10.

Every operational location within SutraERP shall resolve to a canonical **Infrastructure Space**.

Each Space may contain:

- One Space Type
- Multiple Operational Capabilities
- Multiple Capacity Profiles
- Multiple Compliance Records
- Multiple Inspection Records
- One Current Availability State
- Multiple Reservations
- Effective-Dated Department Allocations
- Assigned Custodian
- Assigned Operational Manager

The Space aggregate represents the primary unit of operational consumption across the enterprise.

---

# 11. Lifecycle Overview

Every Infrastructure aggregate follows a controlled lifecycle governed by institutional policy.

Illustrative lifecycle:

```text
Draft
   │
   ▼
Submitted
   │
   ▼
Approved
   │
   ▼
Commissioned
   │
   ▼
Operational
   │
   ├──────────────┐
   ▼              ▼
Temporarily   Under
Closed        Maintenance
   │              │
   └──────┬───────┘
          ▼
Operational
          │
          ▼
Under Renovation
          │
          ▼
Decommissioned
          │
          ▼
Archived
```

Lifecycle transitions shall:

- Respect institutional policies
- Be fully auditable
- Publish immutable domain events
- Preserve historical integrity

---

## 11A. Canonical Infrastructure Lifecycle

> Migrated from `L2-B03_Campus_Workflows.md` Part 5 on 2026-08-05 under integrity
> finding INT-008. B03 held the detailed lifecycle model for aggregates G10 owns.
> Section 11 above remains the summary; the canonical detail is below.

---


---

## 11A.49 Lifecycle Philosophy

Infrastructure entities are **long-lived enterprise objects**.

Unlike transactional records, physical infrastructure represents real-world assets that evolve over many years or decades.

Infrastructure shall therefore follow a **controlled business lifecycle** rather than relying on record creation and deletion.

The lifecycle represents the operational existence of infrastructure rather than its persistence within the database.

---

## 11A.50 Lifecycle Objectives

The infrastructure lifecycle exists to achieve the following objectives.

## LO-01 — Preserve Historical Integrity

Infrastructure shall remain historically traceable even after it is no longer operational.

---

## LO-02 — Maintain Referential Integrity

Historical references from:

- Timetable
- Examination
- Admissions
- Asset Management
- Maintenance
- Analytics

shall remain valid throughout the infrastructure lifecycle.

---

## LO-03 — Prevent Unsafe Deletion

Operational infrastructure shall never be physically deleted.

Lifecycle transitions shall replace deletion.

---

## LO-04 — Enable Operational Planning

Infrastructure shall support future operational states such as:

- Planned Expansion
- Construction
- Renovation
- Temporary Closure
- Retirement

---

## LO-05 — Support Regulatory Compliance

Lifecycle history shall remain auditable for regulatory, accreditation, and institutional governance purposes.

---

## 11A.51 Canonical Lifecycle States

Every Infrastructure Aggregate shall exist in exactly one lifecycle state.

```text
Proposed
     │
     ▼
Planned
     │
     ▼
Approved
     │
     ▼
Under Construction
     │
     ▼
Operational
     │
     ├─────────────► Under Maintenance
     │                    │
     │                    ▼
     │               Operational
     │
     ├─────────────► Under Renovation
     │                    │
     │                    ▼
     │               Operational
     │
     ├─────────────► Temporarily Closed
     │                    │
     │                    ▼
     │               Operational
     │
     ▼
Decommissioned
     │
     ▼
Archived
```

---

## 11A.52 Lifecycle State Definitions

## Proposed

Infrastructure has been identified but has not yet received formal approval.

Examples:

- New Building Proposal
- New Campus Proposal
- Future Hostel

---

## Planned

Infrastructure has entered planning activities.

Examples:

- Architectural Planning
- Budget Allocation
- Site Survey

---

## Approved

Infrastructure has received institutional approval for implementation.

Construction may begin.

---

## Under Construction

Physical construction is in progress.

Infrastructure cannot be allocated to business operations.

---

## Operational

Infrastructure is available for normal institutional use.

Only operational infrastructure may be allocated by other domains.

---

## Under Maintenance

Infrastructure is temporarily unavailable due to maintenance activities.

Maintenance workflows belong to the Maintenance domain.

Infrastructure stores only its lifecycle state.

---

## Under Renovation

Infrastructure is undergoing renovation or major modification.

Allocation is prohibited during this state.

---

## Temporarily Closed

Infrastructure has been intentionally closed without permanent retirement.

Examples:

- Summer Closure
- Safety Inspection
- Government Restrictions

---

## Decommissioned

Infrastructure has permanently ceased operational use.

Historical records remain available.

No future allocations are permitted.

---

## Archived

Infrastructure remains part of institutional history.

It is excluded from operational workflows.

---

## 11A.53 Lifecycle Transition Rules

The following transitions are permitted.

| From | To |
|------|----|
| Proposed | Planned |
| Planned | Approved |
| Approved | Under Construction |
| Under Construction | Operational |
| Operational | Under Maintenance |
| Operational | Under Renovation |
| Operational | Temporarily Closed |
| Operational | Decommissioned |
| Under Maintenance | Operational |
| Under Renovation | Operational |
| Temporarily Closed | Operational |
| Decommissioned | Archived |

All other transitions require explicit administrative override and audit approval.

---

## 11A.54 Invalid Lifecycle Transitions

The following transitions are prohibited.

| Invalid Transition | Reason |
|-------------------|--------|
| Archived → Operational | Archived infrastructure is permanently retired. |
| Proposed → Operational | Planning stages cannot be skipped. |
| Decommissioned → Construction | Requires creation of new infrastructure. |
| Archived → Planned | Historical records cannot be reactivated. |

---

## 11A.55 Aggregate Lifecycle Ownership


| Aggregate | Lifecycle Managed |
|------------|---------------------|
| Physical Campus (site boundary) | Yes |
| Building | Yes |
| Floor | Yes |
| Space | Yes |
| Zone | Yes |
| Collection | Yes |
| Portfolio | Yes |

Child aggregates may never exceed the lifecycle maturity of their parent. The

---

## 11A.56 Lifecycle Dependency Rules


## LR-01

A Campus cannot become Operational unless required institutional prerequisites are satisfied.

---

## LR-02

A Building cannot become Operational unless its parent Campus is Operational.

---

## LR-03

A Floor cannot become Operational unless its parent Building is Operational.

---

## LR-04

A Space cannot become Operational unless its parent Floor is Operational.

---

## LR-05

A parent infrastructure entity cannot transition to Archived while active child entities remain Operational.

---

## LR-06

Decommissioning a parent infrastructure entity requires evaluation of all dependent child entities.

---

## 11A.57 Operational Availability

To resolve semantic ambiguity, the system explicitly distinguishes these concepts:
- **Lifecycle** represents long-term business existence.
- **Operational Status** represents the current operational condition.
- **Availability** is a derived business fact determined from Lifecycle together with Operational Status (and any other applicable operational constraints).

Lifecycle state determines operational availability.

| Lifecycle State | Operationally Available |
|-----------------|-------------------------|
| Proposed | No |
| Planned | No |
| Approved | No |
| Under Construction | No |
| Operational | Yes |
| Under Maintenance | No |
| Under Renovation | No |
| Temporarily Closed | No |
| Decommissioned | No |
| Archived | No |

Only infrastructure in the **Operational** state may be used by operational domains.

---

## 11A.58 Lifecycle Events

Every lifecycle transition publishes a canonical business event.

Examples include:

- InfrastructurePlanned
- InfrastructureApproved
- ConstructionStarted
- InfrastructureOperational
- MaintenanceStarted
- MaintenanceCompleted
- RenovationStarted
- RenovationCompleted
- InfrastructureTemporarilyClosed
- InfrastructureReopened
- InfrastructureDecommissioned
- InfrastructureArchived

Events represent completed business facts and are immutable.

---

## 11A.59 Consumer Behaviour

Consumer domains shall interpret lifecycle states according to their own business rules.

Examples:

### Timetable

- Ignore non-operational spaces.
- Stop allocating decommissioned classrooms.

---

### Examination

- Exclude archived examination halls.
- Prevent scheduling in renovated buildings.

---

### Admissions

- Consider only operational intake facilities.

---

### Analytics

- Preserve historical utilization.
- Report lifecycle trends.

---

### Maintenance

- Initiate work orders based on lifecycle transitions.

---

## 11A.60 Lifecycle Audit Requirements

Every lifecycle transition shall be auditable.

Audit information includes:

- Previous State
- New State
- Transition Timestamp
- Effective Date
- Initiated By
- Approval Authority
- Transition Reason
- Supporting Documentation Reference

Lifecycle history is immutable and retained according to institutional retention policies.

---

## 11A.61 Lifecycle Design Principles

The lifecycle model follows these principles.

## LP-01

Infrastructure has a business lifecycle, not merely a database lifecycle.

---

## LP-02

Lifecycle transitions represent business decisions.

---

## LP-03

Historical infrastructure remains visible for audit and reporting.

---

## LP-04

Deletion is exceptional and governed by platform-level data governance.

---

## LP-05

Operational availability is determined by lifecycle state.

---

## LP-06

Lifecycle events are authoritative business events published to the enterprise.

---

## LP-07

Parent-child lifecycle consistency shall always be maintained.

---

---

# 12. Workflow Catalogue

The Infrastructure Management domain governs the following workflow families:

### Registration Workflows

- Campus Registration
- Zone Registration
- Building Registration
- Wing / Block Registration
- Floor Registration
- Space Registration

---

### Structure Management Workflows

- Campus Hierarchy Management
- Building Management
- Floor Management
- Space Management
- Department Allocation
- Shared Infrastructure Management

---

### Operational Workflows

- Capacity Management
- Availability Management
- Reservation Management
- Infrastructure Allocation
- Operational Responsibility Assignment

---

### Governance Workflows

- Compliance Management
- Inspection Management
- Certification Management
- Infrastructure Audit

---

### Lifecycle Workflows

- Commissioning
- Temporary Closure
- Maintenance State
- Renovation
- Decommissioning
- Archival

---

# 13. Business Invariants

The following invariants shall always hold.

- Every Space belongs to exactly one Floor.
- Every Floor belongs to exactly one Building.
- Every Building belongs to exactly one Physical Campus.
- Aggregate Identity is immutable.
- Business Codes may change without affecting Identity.
- A Reservation shall reference an existing Space.
- Capacity shall never be negative.
- Archived Infrastructure is immutable.
- Every lifecycle transition shall be auditable.
- Compliance history shall never be deleted.
- Inspection history shall remain immutable.
- Infrastructure ownership shall remain exclusively within this domain.
- Downstream domains shall reference Infrastructure rather than duplicate physical models.
- Infrastructure shall support effective-dated operational allocations where required by institutional policy.
- Institutional Campus governance remains exclusively within Section B.
- Physical Campus ownership remains exclusively within Infrastructure Management.
- Downstream domains shall consume physical infrastructure information from G10.
- No downstream domain shall create independent physical infrastructure models.

Violation of these invariants shall reject the transaction.

---

## 13A. Canonical Business, Validation & Constraint Rules

> Migrated from `L2-B03_Campus_Workflows.md` Part 6 on 2026-08-05 under integrity
> finding INT-008. Section 13 above remains the invariant summary; the canonical
> rule set is below.

---


---

## 13A.62 Business Rule Philosophy

The Campus & Infrastructure domain protects the integrity of the enterprise physical model by enforcing canonical business rules.

Business rules ensure:

- Physical consistency
- Data integrity
- Operational correctness
- Multi-domain reliability
- Regulatory readiness
- Long-term maintainability

Every Infrastructure Aggregate is responsible for enforcing its own invariants.

No external domain may bypass these rules.

---

## 13A.63 Business Rule Categories

Business rules are classified into six categories.

| Category | Purpose |
|----------|---------|
| Identity Rules | Ensure unique identification |
| Structural Rules | Protect physical hierarchy |
| Operational Rules | Govern operational use |
| Lifecycle Rules | Govern lifecycle transitions |
| Relationship Rules | Maintain infrastructure associations |
| Integrity Rules | Preserve enterprise consistency |

---

## 13A.64 Identity Rules

## BR-ID-01

Every Infrastructure entity shall possess a globally unique identifier.

Identifiers are immutable.

---

## BR-ID-02

Every Infrastructure entity shall possess a unique business code.

Business Codes shall be unique within the owning Institution.

Examples:

- CAMP-MAIN
- BLDG-A
- ROOM-101

---

## BR-ID-03

Infrastructure names may change.

Identifiers shall never change.

---

## BR-ID-04

Duplicate infrastructure records are prohibited.

---

## 13A.65 Structural Rules

## BR-ST-01

Every Campus belongs to exactly one Institution.

---

## BR-ST-02

Every Building belongs to exactly one Campus.

---

## BR-ST-03

Every Floor belongs to exactly one Building.

---

## BR-ST-04

Every Space belongs to exactly one Floor.

---

## BR-ST-05

Every Infrastructure Unit belongs to exactly one parent in the physical hierarchy.

---

## BR-ST-06

Circular hierarchy relationships are prohibited.

Example:

```text
Building A

↓

Floor 1

↓

Room 101

↓

Building A ❌
```

---

## BR-ST-07

Physical hierarchy depth shall remain canonical.

Example:

```text
Campus
   ↓
Building
   ↓
Floor
   ↓
Space
```

Additional arbitrary hierarchy levels are prohibited unless introduced through future architectural revisions.

---

## 13A.66 Operational Rules

## BR-OP-01

Only Operational infrastructure may participate in institutional operations.

---

## BR-OP-02

Infrastructure Under Construction shall never be allocated.

---

## BR-OP-03

Infrastructure Under Renovation shall never be allocated.

---

## BR-OP-04

Temporarily Closed infrastructure shall not be available for scheduling.

---

## BR-OP-05

Decommissioned infrastructure shall remain visible for historical purposes but unavailable for future operational use.

---

## BR-OP-06

Archived infrastructure shall not appear in operational searches by default.

---

## 13A.67 Capacity Rules

## BR-CP-01

Capacity shall never be negative.

---

## BR-CP-02

Standing Capacity shall not exceed Maximum Occupancy.

---

## BR-CP-03

Seating Capacity shall not exceed Maximum Occupancy.

---

## BR-CP-04

Operational Capacity shall be less than or equal to Maximum Occupancy.

---

## BR-CP-05

Capacity values shall use approved measurement units.

---

## 13A.68 Area Rules

## BR-AR-01

Physical Area shall be greater than zero.

---

## BR-AR-02

Measurement units shall follow enterprise configuration.

Examples:

- Square Feet
- Square Metres

---

## BR-AR-03

Gross Area shall be greater than or equal to Carpet Area.

---

## 13A.69 Lifecycle Rules

## BR-LC-01

Lifecycle transitions shall follow the approved lifecycle model.

---

## BR-LC-02

Skipped lifecycle transitions are prohibited.

Example:

```text
Planned

↓

Operational ❌
```

---

## BR-LC-03

Only authorized users may perform lifecycle transitions.

---

## BR-LC-04

Every lifecycle transition shall generate an audit record.

---

## BR-LC-05

Every lifecycle transition shall publish a canonical business event.

---

## 13A.70 Relationship Rules

## BR-RL-01

Zone membership shall never modify physical hierarchy.

---

## BR-RL-02

Collection membership shall never imply ownership.

---

## BR-RL-03

Portfolio membership shall never alter infrastructure lifecycle.

---

## BR-RL-04

Logical relationships shall remain independent of physical relationships.

---

## 13A.71 Accessibility Rules

## BR-AC-01

Accessibility characteristics describe infrastructure capabilities.

They do not certify regulatory compliance.

---

## BR-AC-02

Accessibility metadata shall remain configurable.

---

## BR-AC-03

Infrastructure shall support multiple accessibility characteristics simultaneously.

Examples:

- Wheelchair Accessible
- Braille Signage
- Lift Access
- Hearing Assistance

---

## 13A.72 Sustainability Rules

## BR-SU-01

Infrastructure stores sustainability capabilities.

---

## BR-SU-02

Infrastructure does not calculate sustainability performance.

---

## BR-SU-03

Capability metadata shall remain configurable.

Examples:

- Solar Installed
- Solar Ready
- Rainwater Harvesting
- EV Charging

---

## 13A.73 Health Rules

## BR-HL-01

Infrastructure Health represents current business status.

---

## BR-HL-02

Health assessments shall include assessment date.

---

## BR-HL-03

Historical health assessments shall remain auditable.

---

## BR-HL-04

Health changes shall publish canonical business events.

---

## 13A.74 Criticality Rules

## BR-CR-01

Every Infrastructure entity may possess one Business Continuity Classification.

---

## BR-CR-02

Criticality classifications are configurable.

Examples:

- Mission Critical
- Business Critical
- Standard
- Low Priority

---

## BR-CR-03

Criticality classification does not perform disaster recovery.

It only provides recovery metadata.

---

## 13A.75 Availability Rules

Infrastructure Availability is derived from multiple business facts.

Availability depends upon:

- Lifecycle
- Operational Status
- Closure Status
- Maintenance State

Availability shall never be manually overridden without authorization.

---

## 13A.76 Event Rules

Infrastructure shall publish business events whenever significant business facts change.

Examples include:

- SpaceCreated
- InfrastructureActivated
- RoomCapacityChanged
- SpaceAccessibilityUpdated
- InfrastructureLifecycleChanged
- SpaceCriticalityChanged

Events represent completed business facts.

Events shall be immutable.

---

## 13A.77 Public Contract Rules

Infrastructure exposes stable public contracts.

External domains:

- Shall not access internal entities.
- Shall not depend upon persistence models.
- Shall consume canonical contracts only.

---

## 13A.78 Validation Rules

The following validations are mandatory before an Infrastructure entity becomes Operational.

| Validation | Required |
|------------|----------|
| Identifier Present | Yes |
| Business Code Present | Yes |
| Name Present | Yes |
| Parent Hierarchy Valid | Yes |
| Lifecycle Valid | Yes |
| Capacity Valid | Yes |
| Area Valid | Yes |
| Operational Status Defined | Yes |

If any mandatory validation fails:

- Activation shall be rejected.
- Validation errors shall be returned.
- No business events shall be published.

---

## 13A.79 Cross-Aggregate Validation

Cross-aggregate validation shall be minimized.

Permitted examples include:

- Parent lifecycle verification.
- Parent existence verification.
- Parent operational status verification.

Cross-aggregate business logic is prohibited.

---

## 13A.80 Data Integrity Principles

The Campus & Infrastructure domain follows these integrity principles.

## DIP-01

Every Infrastructure record shall be uniquely identifiable.

---

## DIP-02

Every Infrastructure record shall have exactly one owner.

---

## DIP-03

Business rules are enforced by the owning Aggregate.

---

## DIP-04

Business rules shall remain technology independent.

---

## DIP-05

Validation shall occur before state mutation.

---

## DIP-06

Infrastructure facts are authoritative.

Derived interpretations belong to downstream domains.

---

## DIP-07

Infrastructure integrity takes precedence over operational convenience.

---

## 13A.81 Rule Enforcement Responsibility

| Rule Type | Enforced By |
|-----------|-------------|
| Identity Rules | Aggregate |
| Structural Rules | Aggregate |
| Lifecycle Rules | Aggregate |
| Capacity Rules | Aggregate |
| Relationship Rules | Aggregate |
| Validation Rules | Application Layer |
| Authorization Rules | Security Domain |
| Audit Rules | Platform Foundation |

---

---

# 14. Architectural Constraints

The Infrastructure Management Domain shall not:

- Own Student data
- Own Faculty data
- Own Timetable scheduling
- Own Examination scheduling
- Own Hostel occupancy
- Own Library circulation
- Own Laboratory operations
- Execute maintenance work
- Manage visitors
- Execute analytics
- Maintain dashboards
- Compute KPIs

Cross-domain interaction shall occur through:

- Domain Events
- Published Service Contracts
- Platform Services
- Reservation Interfaces
- Configuration Framework

The Infrastructure Management Domain shall not:

- Own institutional campus governance.
- Define Institution-to-Campus governance relationships.
- Replace Section B organisational ownership.
- Manage institutional approval of campus existence.

---

# 15. Repository Principles

The Infrastructure Management Domain shall conform to the following repository principles.

- Domain-Driven Design
- Event-Driven Architecture
- CQRS
- Canonical Infrastructure Ownership
- Digital Physical Twin
- Immutable Audit History
- Effective-Dated Configuration
- Multi-Campus Support
- Multi-Institution Support
- Multi-Country Support
- High Cohesion
- Loose Coupling
- Configuration over Customisation
- Single Aggregate Ownership
- Enterprise Scalability

---

**End of Part 1**

**Next Part:** **Infrastructure Registration, Campus Hierarchy, Building Management, Floor Management, Space Management, Department Allocation, and Validation Workflows.**

# 16. Infrastructure Registration Workflows

> Detailed workflow specification for this area is in Section 22A (migrated from L2-B03 under INT-008).

## Purpose

Infrastructure Registration establishes the canonical physical representation of institutional infrastructure within SutraERP.

Registration creates enterprise-recognized infrastructure entities that may subsequently participate in downstream operational workflows.

Infrastructure Registration does not imply operational availability, departmental allocation, reservation eligibility, or infrastructure commissioning.

Every newly registered infrastructure entity shall initially exist in the **Draft** lifecycle state unless institutional policy specifies otherwise.

---

## Registration Principles

Infrastructure Registration shall satisfy the following principles.

- Canonical ownership
- Globally unique identity
- Configurable business codes
- Enterprise-wide discoverability
- Complete auditability
- Lifecycle governance
- Configuration-driven validation
- Event publication

Registration shall never bypass lifecycle governance.

---

## Supported Registration Types

Infrastructure Registration supports the following entity types.

- Campus
- Zone
- Building
- Wing
- Block
- Floor
- Space

Institution-specific hierarchy extensions may be configured where permitted by institutional policy.

---

## Registration Preconditions

Before registration may occur, the following conditions shall be satisfied.

### Organisational Validation

- Institution exists.
- Campus ownership is valid.
- Organisational hierarchy is active.
- Parent infrastructure exists where applicable.

---

### Configuration Validation

The platform shall validate:

- Infrastructure policies
- Naming conventions
- Business code configuration
- Hierarchy rules
- Space classification rules
- Capacity configuration
- Compliance configuration

---

### Authorization Validation

The requesting user shall possess Infrastructure Registration permission.

Authorization shall be evaluated through the Platform Authorization Framework.

---

## Registration Inputs

Typical inputs include:

- Institution
- Campus
- Parent Infrastructure
- Infrastructure Type
- Business Code
- Name
- Description
- Classification
- Operational Capabilities
- Initial Capacity
- Responsible Department
- Custodian
- Operational Manager

Additional institution-specific metadata may be configured.

---

## Registration Workflow

```text
Registration Request
          │
          ▼
Authorization Validation
          │
          ▼
Hierarchy Validation
          │
          ▼
Configuration Validation
          │
          ▼
Business Rule Validation
          │
          ▼
Create Infrastructure Aggregate
          │
          ▼
Assign Immutable Identity
          │
          ▼
Lifecycle = Draft
          │
          ▼
Publish Domain Event
```

---

## Registration Post Conditions

Upon successful registration:

- Infrastructure Aggregate exists.
- Immutable Identity assigned.
- Lifecycle = Draft.
- Audit record created.
- Registration event published.

The infrastructure entity shall not yet be operational.

---

# 17. Campus Management Workflows

> Detailed workflow specification for this area is in Section 22A (migrated from L2-B03 under INT-008).

## Purpose

Campus Management governs the lifecycle of institutional campuses.

A Campus represents the highest physical operational boundary within the Infrastructure domain.

Every downstream infrastructure entity shall belong to exactly one Campus.

---

## Campus Creation

Campus creation establishes a new operational campus.

Creation shall not automatically activate the Campus.

---

### Creation Workflow

```text
Campus Registration
         │
Validation
         │
Approval (If Configured)
         │
         ▼
Campus Created
         │
Lifecycle = Draft
```

---

## Campus Activation

Before activation:

- Mandatory configuration completed.
- Required infrastructure exists.
- Compliance validation successful.
- Lifecycle transition permitted.

---

### Activation Workflow

```text
Draft
   │
Activation Request
   │
Infrastructure Validation
   │
Compliance Validation
   │
Approval (Optional)
   │
   ▼
Operational
```

---

## Campus Modification

Authorised administrators may modify:

- Campus Name
- Business Code
- Description
- Operational Status
- Responsible Organisation
- Configuration References

Aggregate Identity shall remain immutable.

---

## Campus Deactivation

Campuses may be temporarily deactivated.

Examples:

- Campus renovation
- Administrative closure
- Seasonal shutdown
- Disaster recovery

Deactivation shall not delete infrastructure.

Historical integrity shall remain preserved.

---

# 17A. Physical Campus Attribute Workflows

> Migrated from `L2-B03_Campus_Workflows.md` Part 8 on 2026-08-05 under integrity finding
> INT-008. Campus workflows split by the boundary in B03 Section 15: institutional campus
> identity, registration, activation, decommission and archive remain with `L2-B03`; the
> physical-site attribute workflows below belong here, because boundary, capacity,
> accessibility, sustainability, criticality and operational status describe the physical
> site rather than the institution's relationship to it.

Corresponding events - `CampusBoundaryUpdated`, `CampusCapacityUpdated`,
`CampusAccessibilityUpdated`, `CampusSustainabilityUpdated`, `CampusCriticalityChanged`
and `CampusOperationalStatusChanged` - are published by this domain, not by B03.

---

## 17A.105 Campus Boundary Management

Campus boundaries define the physical extent of institutional property.

Examples include:

- Land Parcel
- Survey Boundary
- GIS Polygon
- Administrative Boundary

---

#### Workflow

```text
Campus
      │
      ▼
Update Boundary
      │
      ▼
Validate Geometry
      │
      ▼
Persist
      │
      ▼
Publish CampusBoundaryUpdated
```

---

#### Published Events

- CampusBoundaryUpdated

---

## 17A.106 Campus Operational Status Workflow

Operational Status represents whether a campus is available for institutional operations.

Possible statuses include:

- Operational
- Temporarily Closed
- Restricted Access
- Permanently Closed

---

#### Workflow

```text
Campus
     │
     ▼
Status Change
     │
     ▼
Business Validation
     │
     ▼
Persist
     │
     ▼
CampusOperationalStatusChanged
```

---

#### Published Events

- CampusOperationalStatusChanged

---

## 17A.107 Campus Capacity Profile Workflow

A Campus may maintain aggregate capacity information.

Examples include:

- Maximum Student Capacity
- Maximum Staff Capacity
- Parking Capacity
- Building Count
- Total Area

These are canonical facts.

Derived utilization metrics belong to Analytics.

---

#### Workflow

```text
Campus
      │
      ▼
Update Capacity Profile
      │
      ▼
Validate
      │
      ▼
Persist
      │
      ▼
CampusCapacityUpdated
```

---

#### Published Events

- CampusCapacityUpdated

---

## 17A.108 Campus Accessibility Workflow

Maintain accessibility capabilities at campus level.

Examples:

- Accessible Entrances
- Accessible Parking
- Campus Shuttle
- Accessible Navigation
- Emergency Evacuation Routes

---

#### Workflow

```text
Campus
      │
      ▼
Update Accessibility
      │
      ▼
Validate
      │
      ▼
Persist
      │
      ▼
CampusAccessibilityUpdated
```

---

#### Published Events

- CampusAccessibilityUpdated

---

## 17A.109 Campus Sustainability Workflow

Maintain sustainability capabilities.

Examples:

- Solar Infrastructure
- Rainwater Harvesting
- Waste Management
- Green Certification
- EV Charging

The Campus domain stores capabilities only.

Performance metrics belong to Analytics or ESG domains.

---

#### Workflow

```text
Campus
      │
      ▼
Update Sustainability
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
CampusSustainabilityUpdated
```

---

#### Published Events

- CampusSustainabilityUpdated

---

## 17A.110 Campus Criticality Workflow

Maintain business continuity classification.

Examples:

- Mission Critical
- High Priority
- Standard

Maintain associated recovery metadata including:

- Recovery Priority
- Alternate Campus
- Backup Utilities

---

#### Workflow

```text
Campus
      │
      ▼
Update Criticality
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
CampusCriticalityChanged
```

---

#### Published Events

- CampusCriticalityChanged

---

---

# 18. Infrastructure Hierarchy Management

## Purpose

Infrastructure Hierarchy Management governs the structural organisation of institutional infrastructure.

The hierarchy defines parent-child relationships across all physical entities.

---

## Canonical Hierarchy

```
Institution
      │
      ▼
Campus
      │
      ▼
Zone (Optional)
      │
      ▼
Building
      │
      ▼
Wing / Block (Optional)
      │
      ▼
Floor
      │
      ▼
Space
```

The hierarchy shall remain acyclic.

Circular references shall be rejected.

---

## Hierarchy Validation

The platform shall validate:

- Parent exists.
- Parent lifecycle permits child.
- Duplicate hierarchy prevented.
- Organisational ownership valid.
- Infrastructure policies satisfied.

---

## Hierarchy Modification

Structural changes shall:

- Preserve historical integrity.
- Produce complete audit history.
- Publish domain events.
- Maintain aggregate consistency.

---

# 19. Building Management Workflows

> Detailed workflow specification for this area is in Section 22A (migrated from L2-B03 under INT-008).

## Purpose

Building Management governs the complete lifecycle of institutional buildings.

Buildings represent primary operational structures within a Campus.

---

## Building Registration

Building Registration follows Infrastructure Registration principles.

Additional metadata may include:

- Building Category
- Construction Year
- Total Floors
- Accessibility Features
- Emergency Assembly Area
- Building Status

Institution-specific metadata may be configured.

---

## Building Lifecycle

Illustrative lifecycle:

```text
Draft
   │
Approved
   │
Commissioned
   │
Operational
   │
Under Maintenance
   │
Under Renovation
   │
Operational
   │
Decommissioned
   │
Archived
```

---

## Building Modification

Authorised users may modify:

- Building Name
- Business Code
- Accessibility Metadata
- Operational Configuration
- Capacity Profiles
- Responsible Personnel

Identity shall remain immutable.

---

## Building Closure

Buildings may be:

- Temporarily Closed
- Partially Closed
- Emergency Closed
- Permanently Decommissioned

Closure policies shall be institution configurable.

---

# 20. Floor Management Workflows

> Detailed workflow specification for this area is in Section 22A (migrated from L2-B03 under INT-008).

## Purpose

Floor Management governs operational organisation within Buildings.

Every Floor belongs to exactly one Building.

---

## Supported Operations

- Floor Registration
- Floor Modification
- Floor Activation
- Floor Closure
- Floor Reopening
- Floor Decommissioning

---

## Validation

The platform shall validate:

- Parent Building exists.
- Parent Building operational.
- Duplicate Floor prohibited.
- Business Code valid.
- Lifecycle transition permitted.

---

## Operational Effects

Floor availability may affect downstream availability calculations.

Closed Floors shall prevent operational use of dependent Spaces unless explicitly overridden by authorised administrators.

---

# 21. Space Management Workflows

> Detailed workflow specification for this area is in Section 22A (migrated from L2-B03 under INT-008).

## Purpose

Space Management governs the enterprise lifecycle of physical operational spaces.

Every operational activity occurring within the institution shall reference an Infrastructure Space.

Spaces represent the smallest canonical physical unit within SutraERP.

---

## Supported Space Types

Examples include:

- Classroom
- Smart Classroom
- Laboratory
- Computer Laboratory
- Office
- Faculty Cabin
- Seminar Hall
- Auditorium
- Library Hall
- Reading Hall
- Hostel Room
- Medical Room
- Pharmacy
- Cafeteria
- Conference Room
- Sports Hall
- Storage Room
- Server Room
- Security Cabin
- Utility Area
- Parking Area

Institutions may configure additional Space Types.

---

## Space Registration Workflow

```text
Space Registration Request
           │
Authorization Validation
           │
Hierarchy Validation
           │
Configuration Validation
           │
Classification Validation
           │
Create Space
           │
Assign Identity
           │
Lifecycle = Draft
           │
Publish Event
```

---

## Space Modification

Authorised administrators may update:

- Name
- Business Code
- Space Classification
- Operational Capabilities
- Capacity Profile
- Responsible Department
- Custodian
- Operational Manager

Identity shall remain immutable.

---

## Space Retirement

Spaces may be:

- Temporarily Closed
- Permanently Closed
- Decommissioned
- Archived

Historical references shall always remain valid.

---

# 22. Space Classification Workflows

Every Space shall possess exactly one primary Space Type.

Examples include:

- Academic
- Administrative
- Residential
- Healthcare
- Security
- Utility
- Recreational
- Shared

Institutions may define custom classifications.

Classification changes shall be auditable and shall never affect Aggregate Identity.

---

# 22A. Detailed Physical Workflow Specifications

> Migrated from `L2-B03_Campus_Workflows.md` Parts 7, 9, 10 and 11 on 2026-08-05 under
> integrity finding INT-008. B03 held detailed workflow specifications for aggregates this
> document owns, while Sections 16-22 here carried only catalogue summaries of the same
> ground. The specifications now live with the owner. Sections 16-22 remain the catalogue;
> this section is the detail they refer to.

---

### Infrastructure Business Workflows (WF-INF series)


---

#### 82. Workflow Architecture

The Campus & Infrastructure domain manages the complete business lifecycle of institutional physical infrastructure.

Every workflow shall:

- Preserve domain integrity
- Enforce aggregate boundaries
- Publish canonical business events
- Maintain audit history
- Support future extensibility

The workflows defined in this section represent **canonical business workflows**, independent of implementation technology.

---

#### 83. Workflow Classification

Infrastructure workflows are grouped into the following categories.

| Workflow Category | Purpose |
|-------------------|---------|
| Campus Workflows | Campus lifecycle management |
| Building Workflows | Building lifecycle management |
| Floor Workflows | Floor management |
| Space Workflows | Room and facility management |
| Classification Workflows | Infrastructure categorization |
| Relationship Workflows | Zones, Collections & Portfolios |
| Operational Workflows | Activation, Closure & Availability |
| Lifecycle Workflows | Infrastructure lifecycle transitions |

---

#### 84. WF-INF-001 — Create Campus

##### Purpose

Register a new institutional campus.

---

##### Preconditions

- Institution exists.
- User possesses Infrastructure Administration permission.
- Campus Code is unique.

---

##### Primary Flow

```text
Administrator
        │
        ▼
Create Campus
        │
        ▼
Validate Campus Information
        │
        ▼
Generate Campus Identifier
        │
        ▼
Persist Campus Aggregate
        │
        ▼
Publish CampusCreated Event
        │
        ▼
Campus Available
```

---

##### Business Rules

- Campus Code must be unique.
- Campus belongs to exactly one Institution.
- Initial Lifecycle = Proposed or Planned.
- Campus is unavailable until Operational.

---

##### Published Events

- CampusCreated

---

#### 85. WF-INF-002 — Update Campus

##### Purpose

Modify campus information.

---

##### Editable Information

- Name
- Address
- Contact Information
- Geographical Metadata
- Operational Metadata

Infrastructure Identity remains immutable.

---

##### Workflow

```text
Administrator
      │
      ▼
Update Campus
      │
      ▼
Business Validation
      │
      ▼
Persist Changes
      │
      ▼
Publish CampusUpdated
```

---

##### Published Events

- CampusUpdated

---

#### 86. WF-INF-003 — Register Building (Executed in L2-G10)

##### Purpose

Register a building within an operational campus.

---

##### Preconditions

- Campus exists.
- Campus is Operational.
- Building Code is unique.

---

##### Workflow

```text
Administrator
      │
      ▼
Create Building
      │
      ▼
Validate Parent Campus
      │
      ▼
Validate Building Metadata
      │
      ▼
Persist Building
      │
      ▼
Publish BuildingCreated
```

---

##### Business Rules

- Building belongs to one Campus.
- Building cannot exist independently.
- Initial Lifecycle follows configured defaults.

---

##### Published Events

- BuildingCreated

---

#### 87. WF-INF-004 — Register Floor (Executed in L2-G10)

##### Purpose

Create a floor within an existing building.

---

##### Workflow

```text
Building
      │
      ▼
Create Floor
      │
      ▼
Validate Building
      │
      ▼
Validate Floor Number
      │
      ▼
Persist Floor
      │
      ▼
Publish FloorCreated
```

---

##### Business Rules

- Floor Number unique within Building.
- Parent Building must exist.
- Parent Building lifecycle validated.

---

##### Published Events

- FloorCreated

---

#### 88. WF-INF-005 — Register Space (Executed in L2-G10)

##### Purpose

Register a usable physical space.

Examples:

- Classroom
- Laboratory
- Office
- Auditorium
- Library Hall
- Hostel Room

---

##### Workflow

```text
Floor
     │
     ▼
Create Space
     │
     ▼
Validate Parent Floor
     │
     ▼
Validate Capacity
     │
     ▼
Validate Area
     │
     ▼
Persist Space
     │
     ▼
Publish SpaceCreated
```

---

##### Business Rules

- Space belongs to exactly one Floor.
- Capacity must be valid.
- Area must be positive.

---

##### Published Events

- SpaceCreated

---

#### 89. WF-INF-006 — Update Infrastructure

##### Purpose

Modify infrastructure metadata.

Examples:

- Name
- Description
- Capacity
- Area
- Classification
- Accessibility
- Sustainability Capabilities

---

##### Workflow

```text
Administrator
      │
      ▼
Select Infrastructure
      │
      ▼
Modify Metadata
      │
      ▼
Business Validation
      │
      ▼
Persist Changes
      │
      ▼
Publish InfrastructureUpdated
```

---

##### Published Events

- InfrastructureUpdated

---

#### 90. WF-INF-007 — Change Infrastructure Classification

##### Purpose

Modify the business classification of infrastructure.

Examples:

- Classroom → Smart Classroom
- Laboratory → AI Laboratory
- Office → Administrative Office

---

##### Workflow

```text
Administrator
      │
      ▼
Select Infrastructure
      │
      ▼
Choose Classification
      │
      ▼
Validate Configuration
      │
      ▼
Persist
      │
      ▼
Publish ClassificationChanged
```

---

##### Published Events

- InfrastructureClassificationChanged

---

#### 91. WF-INF-008 — Update Capacity

##### Purpose

Maintain canonical infrastructure capacity.

---

##### Workflow

```text
Administrator
      │
      ▼
Select Space
      │
      ▼
Update Capacity
      │
      ▼
Capacity Validation
      │
      ▼
Persist
      │
      ▼
Publish CapacityChanged
```

---

##### Business Rules

- Capacity ≥ 0
- Capacity complies with institutional configuration.

---

##### Published Events

- RoomCapacityChanged

---

#### 92. WF-INF-009 — Update Accessibility Profile

##### Purpose

Maintain accessibility characteristics.

---

##### Workflow

```text
Administrator
      │
      ▼
Select Infrastructure
      │
      ▼
Modify Accessibility
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
Publish SpaceAccessibilityUpdated
```

---

##### Published Events

- SpaceAccessibilityUpdated

---

#### 93. WF-INF-010 — Update Sustainability Profile

##### Purpose

Maintain sustainability capabilities.

---

##### Workflow

```text
Administrator
      │
      ▼
Update Sustainability
      │
      ▼
Validate
      │
      ▼
Persist
      │
      ▼
Publish SustainabilityUpdated
```

---

##### Published Events

- SustainabilityProfileUpdated

---

#### 94. WF-INF-011 — Change Operational Status

##### Purpose

Enable or disable operational use.

Examples:

- Open Building
- Close Building
- Reopen Campus

---

##### Workflow

```text
Administrator
      │
      ▼
Select Infrastructure
      │
      ▼
Change Operational Status
      │
      ▼
Business Validation
      │
      ▼
Persist
      │
      ▼
Publish Campus/Building/SpaceOperationalStatusChanged
```

---

##### Published Events

- InfrastructureActivated
- InfrastructureDeactivated

---

#### 95. WF-INF-012 — Lifecycle Transition

##### Purpose

Transition infrastructure through approved lifecycle states.

---

##### Workflow

```text
Current State
       │
       ▼
Requested Transition
       │
       ▼
Lifecycle Validation
       │
       ▼
Authorization
       │
       ▼
Persist New State
       │
       ▼
Publish Lifecycle Event
```

---

##### Business Rules

- Transition must be valid.
- Transition requires authorization.
- Audit record mandatory.

---

##### Published Events

Examples:

- ConstructionStarted
- InfrastructureOperational
- RenovationStarted
- InfrastructureDecommissioned
- InfrastructureArchived

---

#### 96. WF-INF-013 — Decommission Infrastructure

##### Purpose

Retire infrastructure permanently.

---

##### Preconditions

- No active operational dependencies.
- Approval completed.

---

##### Workflow

```text
Administrator
      │
      ▼
Request Decommission
      │
      ▼
Dependency Validation
      │
      ▼
Approval
      │
      ▼
Update Lifecycle
      │
      ▼
Publish InfrastructureDecommissioned
```

---

##### Business Rules

Infrastructure remains historically available.

Deletion does not occur.

---

#### 97. WF-INF-014 — Archive Infrastructure

##### Purpose

Move permanently retired infrastructure into historical state.

---

##### Workflow

```text
Decommissioned
        │
        ▼
Archive Request
        │
        ▼
Validation
        │
        ▼
Archive
        │
        ▼
Publish InfrastructureArchived
```

---

#### 98. Workflow Events Summary

| Workflow | Published Event |
|-----------|-----------------|
| Create Campus | CampusCreated |
| Update Campus | CampusUpdated |
| Create Building | BuildingCreated |
| Create Floor | FloorCreated |
| Create Space | SpaceCreated |
| Update Infrastructure | InfrastructureUpdated |
| Change Classification | InfrastructureClassificationChanged |
| Update Capacity | RoomCapacityChanged |
| Update Accessibility | SpaceAccessibilityUpdated |
| Update Sustainability | SustainabilityProfileUpdated |
| Activate Infrastructure | InfrastructureActivated |
| Deactivate Infrastructure | InfrastructureDeactivated |
| Lifecycle Transition | InfrastructureLifecycleChanged |
| Decommission | InfrastructureDecommissioned |
| Archive | InfrastructureArchived |

---

#### 99. Workflow Design Principles

##### WDP-01

Every workflow modifies exactly one Aggregate Root.

---

##### WDP-02

Every successful workflow publishes at least one canonical domain event.

---

##### WDP-03

Every workflow is fully auditable.

---

##### WDP-04

Business validation precedes persistence.

---

##### WDP-05

Multi-domain orchestration is prohibited.

Infrastructure publishes events.

Consumer domains determine their own reactions.

---

##### WDP-06

Infrastructure workflows manage physical entities only.

Operational activities (Admissions, Timetable, Examination, Maintenance, Security, etc.) belong to their respective domains.

---

---

### Building Management Workflows


---

#### 117. Building Domain Overview

A **Building** represents a permanent physical structure located within a Campus.

Buildings provide the physical environment in which institutional activities occur.

Examples include:

- Academic Building
- Administrative Building
- Library
- Auditorium
- Hostel
- Sports Complex
- Research Centre
- Medical Block
- Cafeteria
- Utility Building

A Building is a physical entity.

It does **not** represent departments, academic programs, organizational units, or assets.

---

#### 118. Building Lifecycle

Every Building follows the canonical Infrastructure Lifecycle.

```text
Proposed
     │
     ▼
Planned
     │
     ▼
Approved
     │
     ▼
Under Construction
     │
     ▼
Operational
     │
     ├────────► Under Maintenance
     │                 │
     │                 ▼
     │            Operational
     │
     ├────────► Under Renovation
     │                 │
     │                 ▼
     │            Operational
     │
     ├────────► Temporarily Closed
     │                 │
     │                 ▼
     │            Operational
     │
     ▼
Decommissioned
     │
     ▼
Archived
```

---

#### 119. Building Registration Workflow

##### Purpose

Register a new Building within an Operational Campus.

---

##### Actors

- Infrastructure Administrator
- Campus Administrator

---

##### Preconditions

- Parent Campus exists.
- Parent Campus is Operational.
- Building Code is unique within Campus.

---

##### Workflow

```text
Administrator
      │
      ▼
Create Building
      │
      ▼
Select Campus
      │
      ▼
Enter Building Details
      │
      ▼
Validate Business Rules
      │
      ▼
Persist Building Aggregate
      │
      ▼
Publish BuildingCreated
```

---

##### Business Rules

- Building belongs to one Campus.
- Building Code is immutable.
- Initial Lifecycle follows configuration.
- Parent Campus must exist.

---

##### Published Events

- BuildingCreated

---

#### 120. Building Activation Workflow

##### Purpose

Make a Building operational.

---

##### Preconditions

- Parent Campus is Operational.
- Building lifecycle permits activation.
- Mandatory metadata completed.

---

##### Workflow

```text
Building
      │
      ▼
Activation Request
      │
      ▼
Business Validation
      │
      ▼
Lifecycle Validation
      │
      ▼
Operational
      │
      ▼
Publish BuildingActivated
```

---

##### Published Events

- BuildingActivated

---

#### 121. Building Metadata Workflow

Maintain Building information.

Editable attributes include:

- Building Name
- Description
- Building Type
- Number of Floors
- Construction Year
- Gross Area
- Operational Notes

Immutable:

- Building Identifier
- Building Code
- Parent Campus

---

##### Workflow

```text
Administrator
      │
      ▼
Update Building
      │
      ▼
Validate Changes
      │
      ▼
Persist
      │
      ▼
BuildingUpdated
```

---

##### Published Events

- BuildingUpdated

---

#### 122. Building Structure Workflow

Manage structural characteristics.

Examples include:

- Number of Floors
- Wings
- Entrances
- Emergency Exits
- Lift Availability
- Staircases

---

##### Workflow

```text
Administrator
      │
      ▼
Modify Building Structure
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
BuildingStructureUpdated
```

---

##### Published Events

- BuildingStructureUpdated

---

#### 123. Building Classification Workflow

Buildings may be classified according to institutional configuration.

Examples:

Functional Classification

- Academic
- Administrative
- Residential
- Research
- Sports
- Utility

Construction Classification

- Permanent
- Temporary
- Heritage

Operational Classification

- Public
- Restricted
- High Security

---

##### Workflow

```text
Building
      │
      ▼
Update Classification
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
BuildingClassificationChanged
```

---

##### Published Events

- BuildingClassificationChanged

---

#### 124. Building Capacity Workflow

Maintain canonical building capacity.

Examples:

- Maximum Occupancy
- Total Seating Capacity
- Parking Capacity
- Emergency Capacity

Infrastructure stores facts only.

Derived utilization belongs to Analytics.

---

##### Workflow

```text
Building
      │
      ▼
Update Capacity
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
BuildingCapacityUpdated
```

---

##### Published Events

- BuildingCapacityUpdated

---

#### 125. Building Accessibility Workflow

Maintain accessibility capabilities.

Examples:

- Wheelchair Accessible
- Lift Available
- Accessible Toilets
- Braille Signage
- Accessible Entrance
- Accessible Parking

---

##### Workflow

```text
Building
      │
      ▼
Update Accessibility
      │
      ▼
Validate
      │
      ▼
Persist
      │
      ▼
BuildingAccessibilityUpdated
```

---

##### Published Events

- BuildingAccessibilityUpdated

---

#### 126. Building Sustainability Workflow

Maintain sustainability capabilities.

Examples:

- Solar Installed
- Solar Ready
- Rainwater Harvesting
- Grey Water Recycling
- Waste Segregation
- EV Charging
- Green Building Certification

The Building aggregate stores capabilities only.

Performance metrics belong to Analytics or ESG domains.

---

##### Workflow

```text
Building
      │
      ▼
Update Sustainability
      │
      ▼
Validate
      │
      ▼
Persist
      │
      ▼
BuildingSustainabilityUpdated
```

---

##### Published Events

- BuildingSustainabilityUpdated

---

#### 127. Building Health Workflow

Maintain the current health assessment of the Building.

Examples:

- Excellent
- Good
- Fair
- Poor
- Critical

Maintenance determines health.

Infrastructure stores the current health classification.

---

##### Workflow

```text
Building
      │
      ▼
Update Health
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
BuildingHealthUpdated
```

---

##### Published Events

- BuildingHealthUpdated

---

#### 128. Building Criticality Workflow

Maintain Business Continuity classification.

Examples:

- Mission Critical
- Business Critical
- Standard
- Low Priority

Associated metadata includes:

- Recovery Priority
- Maximum Downtime
- Backup Utility Availability
- Alternate Facility

---

##### Workflow

```text
Building
      │
      ▼
Update Criticality
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
BuildingCriticalityChanged
```

---

##### Published Events

- BuildingCriticalityChanged

---

#### 129. Building Operational Status Workflow

Manage operational availability.

Possible statuses:

- Operational
- Restricted
- Temporarily Closed
- Emergency Closure

Operational Status works together with Lifecycle to determine availability.

---

##### Workflow

```text
Building
      │
      ▼
Change Status
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
BuildingOperationalStatusChanged
```

---

##### Published Events

- BuildingOperationalStatusChanged

---

#### 130. Building Decommission Workflow

Retire a Building permanently.

---

##### Preconditions

- Child Floors transitioned.
- Child Spaces transitioned.
- Required approvals completed.

---

##### Workflow

```text
Building
      │
      ▼
Request Decommission
      │
      ▼
Dependency Validation
      │
      ▼
Approval
      │
      ▼
Lifecycle Transition
      │
      ▼
BuildingDecommissioned
```

---

##### Business Rules

- Historical references remain valid.
- Physical deletion prohibited.
- Building becomes unavailable.

---

##### Published Events

- BuildingDecommissioned

---

#### 131. Building Archive Workflow

Archive permanently retired Buildings.

---

##### Workflow

```text
Decommissioned
      │
      ▼
Archive Request
      │
      ▼
Validation
      │
      ▼
Archive
      │
      ▼
BuildingArchived
```

---

##### Published Events

- BuildingArchived

---

#### 132. Multi-Domain Consumers

| Consumer Domain | Building Information Used |
|-----------------|---------------------------|
| Admissions | Academic infrastructure availability |
| Timetable | Building operational status |
| Examination | Examination centres |
| HR | Workplace assignment |
| Hostel | Hostel buildings |
| Asset Management | Building location hierarchy |
| Maintenance | Physical infrastructure |
| Security | Building security planning |
| Transport | Building access planning |
| Analytics | Building facts |
| AI | Infrastructure facts |
| Disaster Management | Recovery planning |

---

#### 133. Business Rules

##### BM-01

A Building belongs to exactly one Campus.

---

##### BM-02

Building Code is unique within Campus.

---

##### BM-03

Building Identifier is immutable.

---

##### BM-04

Only Operational Buildings may contain Operational Floors.

---

##### BM-05

Building deletion is prohibited after operational activation.

---

##### BM-06

Parent Campus lifecycle limits Building lifecycle.

---

##### BM-07

Every significant Building change publishes a canonical business event.

---

#### 134. Building Events Summary

| Workflow | Event |
|----------|-------|
| Register Building | BuildingCreated |
| Update Building | BuildingUpdated |
| Activate Building | BuildingActivated |
| Structure Update | BuildingStructureUpdated |
| Classification Update | BuildingClassificationChanged |
| Capacity Update | BuildingCapacityUpdated |
| Accessibility Update | BuildingAccessibilityUpdated |
| Sustainability Update | BuildingSustainabilityUpdated |
| Health Update | BuildingHealthUpdated |
| Criticality Update | BuildingCriticalityChanged |
| Operational Status | BuildingOperationalStatusChanged |
| Decommission | BuildingDecommissioned |
| Archive | BuildingArchived |

---

#### 135. Workflow Design Principles

##### BWP-01

Building owns only Building-related business rules.

---

##### BWP-02

Building does not manage activities performed inside the Building.

---

##### BWP-03

Building publishes canonical facts.

Consumers determine business behaviour.

---

##### BWP-04

Every Building workflow shall be fully auditable.

---

##### BWP-05

Every successful Building workflow shall publish canonical business events.

---

##### BWP-06

Building remains independent of Timetable, Admissions, Examination, Maintenance, Asset Management, and Analytics implementations.

---

---

### Floor & Space Management Workflows


---

#### 136. Domain Overview

Floors and Spaces represent the operational infrastructure used by institutional business processes.

They provide the physical locations where teaching, administration, research, examinations, accommodation, healthcare, and student services occur.

Unlike Campus and Building, these aggregates are directly consumed by many operational domains.

Examples of Spaces include:

- Classroom
- Laboratory
- Seminar Hall
- Examination Hall
- Faculty Office
- Principal Office
- Library Reading Hall
- Hostel Room
- Medical Room
- Server Room
- Auditorium
- Conference Room
- Cafeteria
- Sports Hall

---

#### 137. Aggregate Responsibilities

##### Floor Aggregate

Owns:

- Floor Identity
- Floor Ordering
- Floor Metadata
- Floor Accessibility
- Floor Operational Status
- Parent Building Relationship

---

##### Space Aggregate

Owns:

- Space Identity
- Space Classification
- Capacity
- Physical Characteristics
- Accessibility
- Sustainability Capabilities
- Health Status
- Criticality
- Operational Availability
- Lifecycle

---

#### 138. Floor Registration Workflow

##### Purpose

Register a Floor within a Building.

---

##### Actors

- Infrastructure Administrator

---

##### Preconditions

- Parent Building exists.
- Parent Building is Operational.
- Floor Number is unique.

---

##### Workflow

```text
Administrator
      │
      ▼
Create Floor
      │
      ▼
Select Building
      │
      ▼
Enter Floor Details
      │
      ▼
Business Validation
      │
      ▼
Persist Floor
      │
      ▼
Publish FloorCreated
```

---

##### Business Rules

- Floor belongs to exactly one Building.
- Floor Number is immutable after operational use.
- Parent lifecycle validated.

---

##### Published Events

- FloorCreated

---

#### 139. Floor Update Workflow

Editable information includes:

- Floor Name
- Description
- Accessibility Metadata
- Operational Metadata

Immutable:

- Identifier
- Parent Building

---

##### Workflow

```text
Administrator
      │
      ▼
Modify Floor
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
FloorUpdated
```

---

##### Published Events

- FloorUpdated

---

#### 140. Space Registration Workflow

##### Purpose

Register a usable Space.

---

##### Preconditions

- Parent Floor exists.
- Parent Floor is Operational.
- Space Code is unique within Building.

---

##### Workflow

```text
Administrator
      │
      ▼
Create Space
      │
      ▼
Select Floor
      │
      ▼
Enter Space Details
      │
      ▼
Business Validation
      │
      ▼
Persist Space
      │
      ▼
Publish SpaceCreated
```

---

##### Mandatory Information

- Space Name
- Space Code
- Space Type
- Capacity
- Area
- Parent Floor

---

##### Published Events

- SpaceCreated

---

#### 141. Space Classification Workflow

Infrastructure classifications are configurable.

Examples:

Academic

- Classroom
- Laboratory
- Workshop
- Seminar Hall

Administrative

- Office
- Meeting Room
- Record Room

Residential

- Hostel Room
- Guest Room

Utility

- Washroom
- Pantry
- Storage
- Server Room

Healthcare

- Clinic
- Medical Room
- Isolation Room

Sports

- Indoor Court
- Gymnasium

---

##### Workflow

```text
Space
     │
     ▼
Update Classification
     │
     ▼
Validation
     │
     ▼
Persist
     │
     ▼
SpaceClassificationChanged
```

---

##### Published Events

- SpaceClassificationChanged

---

#### 142. Space Capacity Workflow

Maintain canonical capacity information.

Capacity includes:

- Seating Capacity
- Standing Capacity
- Maximum Occupancy
- Operational Capacity

Infrastructure stores facts only.

Utilization belongs to Analytics.

---

##### Workflow

```text
Space
     │
     ▼
Update Capacity
     │
     ▼
Validate
     │
     ▼
Persist
     │
     ▼
RoomCapacityChanged
```

---

##### Business Rules

- Capacity cannot be negative.
- Operational Capacity ≤ Maximum Occupancy.

---

##### Published Events

- RoomCapacityChanged

---

#### 143. Space Area Workflow

Maintain physical measurements.

Examples:

- Carpet Area
- Built-up Area
- Volume (optional future extension)

Measurement units follow enterprise configuration.

---

##### Workflow

```text
Space
     │
     ▼
Update Area
     │
     ▼
Validation
     │
     ▼
Persist
     │
     ▼
SpaceAreaUpdated
```

---

##### Published Events

- SpaceAreaUpdated

---

#### 144. Space Accessibility Workflow

Maintain accessibility capabilities.

Examples:

- Wheelchair Accessible
- Braille Signage
- Hearing Assistance
- Lift Accessible
- Adjustable Furniture
- Accessible Restroom Nearby

---

##### Workflow

```text
Space
     │
     ▼
Update Accessibility
     │
     ▼
Validation
     │
     ▼
Persist
     │
     ▼
SpaceAccessibilityUpdated
```

---

##### Published Events

- SpaceAccessibilityUpdated

---

#### 145. Space Facility Workflow

Maintain facilities available within the Space.

Examples:

- Projection Capability
- Presentation Capability
- Audio Capability
- Climate Control
- Network Connectivity
- Power Availability
- Environmental Controls

Facilities describe available infrastructure.

Assets remain owned by Asset Management.

---

##### Workflow

```text
Space
     │
     ▼
Update Facilities
     │
     ▼
Validation
     │
     ▼
Persist
     │
     ▼
SpaceFacilitiesUpdated
```

---

##### Published Events

- SpaceFacilitiesUpdated

---

#### 146. Space Sustainability Workflow

Maintain sustainability capabilities.

Examples:

- Solar Powered
- Natural Ventilation
- LED Lighting
- Occupancy Sensors
- Rainwater Usage
- Smart Energy Controls

Infrastructure stores capability metadata only.

---

##### Workflow

```text
Space
     │
     ▼
Update Sustainability
     │
     ▼
Validation
     │
     ▼
Persist
     │
     ▼
SustainabilityProfileUpdated
```

---

##### Published Events

- SustainabilityProfileUpdated

---

#### 147. Space Health Workflow

Maintain current health classification.

Examples:

- Excellent
- Good
- Fair
- Poor
- Critical

Maintenance determines condition.

Infrastructure stores the latest assessment.

---

##### Workflow

```text
Space
     │
     ▼
Update Health
     │
     ▼
Validation
     │
     ▼
Persist
     │
     ▼
SpaceHealthUpdated
```

---

##### Published Events

- SpaceHealthUpdated

---

#### 148. Space Criticality Workflow

Maintain Business Continuity classification.

Examples:

- Mission Critical
- High Priority
- Standard
- Low Priority

Associated metadata:

- Recovery Priority
- Maximum Downtime
- Alternate Facility

---

##### Workflow

```text
Space
     │
     ▼
Update Criticality
     │
     ▼
Validation
     │
     ▼
Persist
     │
     ▼
SpaceCriticalityChanged
```

---

##### Published Events

- SpaceCriticalityChanged

---

#### 149. Space Operational Status Workflow

Operational Status determines whether a Space is currently available.

Possible statuses include:

- Available
- Restricted
- Reserved
- Closed
- Emergency Closure

Operational Status complements—but does not replace—the canonical Lifecycle.

---

##### Workflow

```text
Space
     │
     ▼
Status Change
     │
     ▼
Business Validation
     │
     ▼
Persist
     │
     ▼
SpaceOperationalStatusChanged
```

---

##### Published Events

- SpaceOperationalStatusChanged

---

#### 150. Space Lifecycle Workflow

Space follows the canonical infrastructure lifecycle.

```text
Proposed
      │
      ▼
Planned
      │
      ▼
Approved
      │
      ▼
Operational
      │
      ├────────► Under Maintenance
      │
      ├────────► Under Renovation
      │
      ├────────► Temporarily Closed
      │
      ▼
Decommissioned
      │
      ▼
Archived
```

All lifecycle transitions follow the enterprise lifecycle rules defined earlier.

---

#### 151. Space Decommission Workflow

Retire a Space permanently.

---

##### Preconditions

- No active operational allocations.
- Required approvals completed.

---

##### Workflow

```text
Space
     │
     ▼
Request Decommission
     │
     ▼
Dependency Validation
     │
     ▼
Approval
     │
     ▼
Lifecycle Transition
     │
     ▼
SpaceDecommissioned
```

---

##### Published Events

- SpaceDecommissioned

---

#### 152. Space Archive Workflow

Archive permanently retired Spaces.

---

##### Workflow

```text
Decommissioned
      │
      ▼
Archive
      │
      ▼
Validation
      │
      ▼
SpaceArchived
```

---

##### Published Events

- SpaceArchived

---

#### 153. Multi-Domain Consumers

| Consumer Domain | Space Information Consumed |
|-----------------|----------------------------|
| Timetable | Classroom allocation |
| Examination | Examination hall allocation |
| Admissions | Interview rooms and counselling spaces |
| Library | Reading rooms and study spaces |
| Hostel | Hostel room master data |
| HR | Office assignment |
| Asset Management | Physical asset location |
| Maintenance | Work order location |
| Security | Access control and patrol routes |
| Attendance | Classroom location |
| Transport | Pickup/drop infrastructure |
| Analytics | Capacity and utilization facts |
| AI | Canonical infrastructure facts |

---

#### 154. Business Rules

##### FSM-01

Every Floor belongs to exactly one Building.

---

##### FSM-02

Every Space belongs to exactly one Floor.

---

##### FSM-03

Space Code is unique within the parent Building.

---

##### FSM-04

Only Operational Spaces may be allocated by consumer domains.

---

##### FSM-05

Facilities describe capabilities, not owned assets.

---

##### FSM-06

Operational Status and Lifecycle together determine availability.

---

##### FSM-07

Every significant change publishes a canonical domain event.

---

#### 155. Floor & Space Events Summary

| Workflow | Published Event |
|----------|-----------------|
| Register Floor | FloorCreated |
| Update Floor | FloorUpdated |
| Register Space | SpaceCreated |
| Update Space | SpaceUpdated |
| Classification | SpaceClassificationChanged |
| Capacity | RoomCapacityChanged |
| Area | SpaceAreaUpdated |
| Accessibility | SpaceAccessibilityUpdated |
| Facilities | SpaceFacilitiesUpdated |
| Sustainability | SustainabilityProfileUpdated |
| Health | SpaceHealthUpdated |
| Criticality | SpaceCriticalityChanged |
| Operational Status | SpaceOperationalStatusChanged |
| Decommission | SpaceDecommissioned |
| Archive | SpaceArchived |

---

#### 156. Workflow Design Principles

##### FSWP-01

Floor manages only floor-level metadata.

---

##### FSWP-02

Space is the primary operational aggregate consumed by enterprise workflows.

---

##### FSWP-03

Infrastructure publishes canonical physical facts.

Operational domains determine business usage.

---

##### FSWP-04

Facilities represent infrastructure capabilities.

Physical assets remain under Asset Management ownership.

---

##### FSWP-05

Every successful workflow publishes immutable domain events.

---

##### FSWP-06

Floor and Space remain independent of scheduling, examinations, admissions, maintenance execution, and analytics logic.

---

---

### Zone, Collection & Portfolio Management Workflows


---

#### 157. Domain Overview

The Campus & Infrastructure domain supports three logical organizational mechanisms in addition to the physical hierarchy:

- **Zones**
- **Collections**
- **Portfolios**

These constructs **never modify the physical hierarchy**.

Instead, they provide independent business views of the same infrastructure for operational management, strategic planning, and governance.

```text
Physical Hierarchy

Campus
   │
Building
   │
 Floor
   │
 Space

↓

Logical Views

Zones
Collections
Portfolios
```

---

#### 158. Logical Organization Principles

Logical organization exists to support:

- Operational management
- Administrative grouping
- Executive governance
- Strategic investment
- Reporting
- Search and discovery

Logical organization shall never redefine ownership.

---

#### 159. Zone Overview

A **Zone** represents an operational grouping of infrastructure.

Examples include:

- Academic Zone
- Residential Zone
- Examination Zone
- Administrative Zone
- North Campus Zone
- South Campus Zone
- High Security Zone
- Sports Zone

Zones simplify operational administration.

---

#### 160. Zone Lifecycle

Zones follow the canonical lifecycle.

```text
Proposed
      │
      ▼
Operational
      │
      ▼
Archived
```

Most institutions will primarily use Operational and Archived states.

---

#### 161. Zone Registration Workflow

##### Purpose

Create a new operational Zone.

---

##### Actors

- Infrastructure Administrator

---

##### Preconditions

- Zone Name unique.
- Zone Type configured.

---

##### Workflow

```text
Administrator
      │
      ▼
Create Zone
      │
      ▼
Enter Metadata
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
ZoneCreated
```

---

##### Published Events

- ZoneCreated

---

#### 162. Zone Membership Workflow

Assign infrastructure into operational Zones.

Infrastructure may belong to multiple Zones.

Examples:

Room 101

↓

Academic Zone

↓

Smart Classroom Zone

↓

Examination Zone

---

##### Workflow

```text
Administrator
      │
      ▼
Select Zone
      │
      ▼
Select Infrastructure
      │
      ▼
Validate
      │
      ▼
Assign
      │
      ▼
AssignedToZone
```

---

##### Business Rules

- Multiple memberships permitted.
- Membership does not alter hierarchy.
- Membership is independently auditable.

---

##### Published Events

- AssignedToZone

---

#### 163. Zone Removal Workflow

Remove infrastructure from a Zone.

---

##### Workflow

```text
Infrastructure
      │
      ▼
Remove Membership
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
RemovedFromZone
```

---

##### Published Events

- RemovedFromZone

---

#### 164. Collection Overview

Collections represent logical business categorization.

Collections support discovery, reporting, and administration.

Examples:

- Smart Classrooms
- Heritage Buildings
- Research Laboratories
- Accessible Facilities
- Digital Learning Spaces
- Innovation Centres

Collections are descriptive.

They never imply ownership.

---

#### 165. Collection Registration Workflow

##### Purpose

Create a Collection.

---

##### Workflow

```text
Administrator
      │
      ▼
Create Collection
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
CollectionCreated
```

---

##### Published Events

- CollectionCreated

---

#### 166. Collection Membership Workflow

Assign infrastructure into Collections.

Examples:

Research Lab

↓

Research Infrastructure

↓

AI Research Collection

↓

Innovation Collection

---

##### Workflow

```text
Infrastructure
      │
      ▼
Assign Collection
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
AssignedToCollection
```

---

##### Business Rules

- Infrastructure may belong to multiple Collections.
- Collections never own infrastructure.
- Membership remains independent.

---

##### Published Events

- AssignedToCollection

---

#### 167. Collection Removal Workflow

Remove Collection membership.

---

##### Workflow

```text
Infrastructure
      │
      ▼
Remove Collection
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
RemovedFromCollection
```

---

##### Published Events

- RemovedFromCollection

---

#### 168. Portfolio Overview

A Portfolio represents a strategic governance grouping.

Unlike Zones and Collections, Portfolios support executive planning and investment decisions.

Examples:

- Capital Investment Portfolio
- Green Campus Portfolio
- Digital Infrastructure Portfolio
- Research Infrastructure Portfolio
- Heritage Preservation Portfolio

Portfolios enable institutional governance without affecting operational ownership.

---

#### 169. Portfolio Registration Workflow

##### Purpose

Create a strategic Portfolio.

---

##### Workflow

```text
Administrator
      │
      ▼
Create Portfolio
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
PortfolioCreated
```

---

##### Published Events

- PortfolioCreated

---

#### 170. Portfolio Membership Workflow

Assign infrastructure to Portfolios.

Examples:

Engineering Block

↓

Research Portfolio

↓

Capital Investment Portfolio

↓

Green Campus Portfolio

---

##### Workflow

```text
Infrastructure
      │
      ▼
Assign Portfolio
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
AssignedToPortfolio
```

---

##### Business Rules

- Multiple memberships permitted.
- Membership never changes lifecycle.
- Membership never changes ownership.
- Membership supports governance only.

---

##### Published Events

- AssignedToPortfolio

---

#### 171. Portfolio Removal Workflow

Remove Portfolio membership.

---

##### Workflow

```text
Infrastructure
      │
      ▼
Remove Portfolio
      │
      ▼
Validation
      │
      ▼
Persist
      │
      ▼
RemovedFromPortfolio
```

---

##### Published Events

- RemovedFromPortfolio

---

#### 172. Logical Relationship Model

```text
Campus
 │
 ├──────── Building
 │             │
 │             ├──────── Floor
 │             │              │
 │             │              └──────── Space
 │
 │
 ├──────────── Zones
 │
 ├──────────── Collections
 │
 └──────────── Portfolios
```

Logical structures reference Infrastructure.

Infrastructure does not belong physically beneath them.

---

#### 173. Multi-Domain Consumers

| Consumer Domain | Usage |
|-----------------|-------|
| Timetable | Academic Zones |
| Examination | Examination Zones |
| Admissions | Campus and Facility Collections |
| Security | Restricted Zones |
| Maintenance | Maintenance Zones |
| Asset Management | Infrastructure Collections |
| Analytics | Portfolio reporting |
| Executive Dashboard | Strategic Portfolios |
| AI | Infrastructure relationships |
| Disaster Management | Criticality Zones |

---

#### 174. Business Rules

##### ZCP-01

Logical structures shall never alter physical hierarchy.

---

##### ZCP-02

Infrastructure may belong to multiple Zones.

---

##### ZCP-03

Infrastructure may belong to multiple Collections.

---

##### ZCP-04

Infrastructure may belong to multiple Portfolios.

---

##### ZCP-05

Membership is many-to-many.

---

##### ZCP-06

Logical structures never own infrastructure.

---

##### ZCP-07

Removing membership never deletes infrastructure.

---

##### ZCP-08

Logical structures remain independent from lifecycle.

---

#### 175. Domain Events Summary

| Workflow | Published Event |
|----------|-----------------|
| Create Zone | ZoneCreated |
| Assign Zone | AssignedToZone |
| Remove Zone | RemovedFromZone |
| Create Collection | CollectionCreated |
| Assign Collection | AssignedToCollection |
| Remove Collection | RemovedFromCollection |
| Create Portfolio | PortfolioCreated |
| Assign Portfolio | AssignedToPortfolio |
| Remove Portfolio | RemovedFromPortfolio |

---

#### 176. Workflow Design Principles

##### ZCPWP-01

Zones support operational organization.

---

##### ZCPWP-02

Collections support logical categorization.

---

##### ZCPWP-03

Portfolios support executive governance and investment planning.

---

##### ZCPWP-04

Logical structures never replace the physical hierarchy.

---

##### ZCPWP-05

Infrastructure ownership always remains with the physical aggregates.

---

##### ZCPWP-06

Every membership change publishes a canonical domain event.

---

##### ZCPWP-07

Logical organization remains fully configurable to support diverse institutional structures without requiring architectural changes.

---

---

# 23. Space Capability Management

A Space may support one or more operational capabilities.

Examples:

- Teaching
- Examination
- Placement Activities
- Conferences
- Meetings
- Workshops
- Cultural Events
- Research

Capabilities determine operational eligibility for downstream domains.

Capability assignment shall be configuration-driven and fully auditable.

---

# 24. Capacity Management Workflows

## Purpose

Capacity Management governs the operational limits of Infrastructure entities.

Capacity defines the maximum supported utilisation of Infrastructure under configured institutional policies.

Infrastructure owns capacity.

Downstream domains consume capacity information.

---

## Capacity Principles

Infrastructure shall support:

- Configurable Capacity Profiles
- Multiple Capacity Types
- Effective-Dated Capacity Changes
- Temporary Capacity Overrides
- Historical Capacity Tracking
- Capacity Auditability

Capacity shall never become negative.

---

## Supported Capacity Types

Examples include:

- Standard Occupancy Capacity
- Teaching Capacity
- Examination Capacity
- Conference Capacity
- Emergency Evacuation Capacity
- Accessibility Capacity
- Standing Capacity

Institutions may configure additional capacity categories.

---

## Capacity Configuration Workflow

```text
Capacity Configuration Request
              │
Authorization Validation
              │
Infrastructure Validation
              │
Policy Validation
              │
Business Validation
              │
Capacity Updated
              │
Audit Recorded
              │
Publish Domain Event
```

---

## Capacity Validation

The platform shall validate:

- Infrastructure exists
- Infrastructure operational
- Capacity value valid
- Capacity type supported
- Policy compliance
- Lifecycle permits modification

Invalid requests shall be rejected.

---

## Temporary Capacity Override

Institutions may temporarily override configured capacity.

Examples include:

- Institutional Event
- Convocation
- Emergency Shelter
- Examination Arrangement
- Special Approval

Temporary overrides shall include:

- Effective Date
- Expiry Date
- Approval Reference (if required)
- Audit History

After expiry, configured capacity shall automatically become effective.

---

## Capacity History

Every capacity modification shall preserve:

- Previous Capacity
- New Capacity
- Effective Date
- Modified By
- Business Reason
- Audit Reference

Historical capacity shall remain immutable.

---

# 25. Infrastructure Availability Management

## Purpose

Infrastructure Availability determines whether physical infrastructure is available for operational consumption.

Availability shall be maintained independently from lifecycle.

---

## Availability States

Typical availability states include:

- Available
- Reserved
- Under Maintenance
- Under Renovation
- Temporarily Closed
- Emergency Closed
- Operationally Restricted

Institutions may configure additional operational availability states.

---

## Availability Workflow

```text
Availability Change Request
             │
Authorization Validation
             │
Lifecycle Validation
             │
Business Validation
             │
Availability Updated
             │
Publish Domain Event
```

---

## Availability Sources

Availability may change due to:

- Planned Maintenance
- Infrastructure Failure
- Administrative Closure
- Emergency
- Reservation
- Compliance Restriction
- Disaster Recovery
- Manual Administrative Action

---

## Scheduled Closures

Infrastructure may be scheduled for temporary closure.

Examples include:

- Summer Vacation
- Annual Maintenance
- Renovation
- Institutional Holidays
- Utility Shutdown

Scheduled closures shall support:

- Start Date
- End Date
- Closure Reason
- Responsible Authority

---

## Partial Availability

Operational availability may exist at multiple hierarchy levels.

Examples:

- Entire Campus Closed
- One Building Closed
- One Floor Closed
- Selected Spaces Closed

Closure of one hierarchy level shall not necessarily affect unrelated infrastructure.

---

# 26. Infrastructure Reservation Workflows

## Purpose

Reservation Management governs temporary allocation of Infrastructure for operational activities.

Infrastructure owns reservation governance.

Operational domains consume reservation services.

---

## Reservation Sources

Reservations may originate from:

- Academic Timetable
- Examination
- Placement Activities
- Institutional Events
- Meetings
- Workshops
- Administrative Activities
- External Bookings (where permitted)

---

## Reservation Workflow

```text
Reservation Request
         │
Authorization Validation
         │
Availability Validation
         │
Conflict Detection
         │
Policy Validation
         │
Reservation Created
         │
Audit Recorded
         │
Publish Domain Event
```

---

## Reservation Validation

The platform shall validate:

- Space operational
- Space available
- Capacity sufficient
- Reservation period valid
- No conflicting reservation
- Reservation policy satisfied

Conflicting reservations shall be rejected unless overridden by authorised administrators.

---

## Reservation Modification

Authorised users may:

- Change Date
- Change Time
- Change Purpose
- Change Duration
- Change Participants
- Cancel Reservation

Every modification shall remain auditable.

---

## Reservation Cancellation

Cancellation shall:

- Release Infrastructure
- Restore Availability
- Publish Domain Event
- Preserve Audit History

Reservation deletion shall never occur.

---

## Recurring Reservations

Institutions may configure recurring reservations.

Examples:

- Weekly Classes
- Monthly Meetings
- Annual Functions
- Training Sessions

Recurring reservations shall support:

- Frequency
- Effective Dates
- Exception Dates
- Administrative Override

---

# 27. Operational Allocation Workflows

## Purpose

Operational Allocation governs long-term assignment of Infrastructure to organisational units.

Allocation does not transfer canonical ownership.

Infrastructure remains the authoritative owner.

---

## Allocation Principles

Infrastructure owns:

- Physical Infrastructure
- Identity
- Lifecycle
- Capacity

Departments receive:

- Operational Responsibility
- Administrative Usage
- Long-Term Allocation

---

## Allocation Workflow

```text
Allocation Request
        │
Authorization Validation
        │
Organisation Validation
        │
Policy Validation
        │
Effective Date Assignment
        │
Allocation Created
        │
Audit Recorded
```

---

## Effective-Dated Allocation

Operational allocations may specify:

- Start Date
- End Date
- Responsible Department
- Responsible Person
- Allocation Status

Historical allocations shall remain preserved.

---

## Shared Infrastructure

Infrastructure may be allocated to multiple organisational units.

Examples:

- Auditorium
- Central Library
- Sports Complex
- Convention Centre
- Seminar Hall

Shared allocation shall remain configuration-driven.

---

# 28. Operational Responsibility Management

Infrastructure may assign multiple operational roles.

Supported responsibilities include:

- Infrastructure Custodian
- Operational Manager
- Administrative Owner
- Maintenance Coordinator
- Compliance Officer
- Safety Officer

Responsibility assignments may be delegated temporarily.

Delegation shall support:

- Effective Dates
- Delegation Reason
- Automatic Restoration
- Complete Audit History

---

# 29. Infrastructure Validation Framework

Every Infrastructure transaction shall execute enterprise validation before state mutation.

Validation categories include:

---

## Structural Validation

Examples:

- Parent Exists
- Hierarchy Valid
- Duplicate Prevention
- Infrastructure Active

---

## Lifecycle Validation

Examples:

- Transition Allowed
- Entity Operational
- Archive Restrictions
- Decommission Rules

---

## Capacity Validation

Examples:

- Capacity Configured
- Capacity Available
- Capacity Override Valid

---

## Availability Validation

Examples:

- Infrastructure Open
- No Operational Restriction
- Reservation Conflict Free

---

## Policy Validation

Examples:

- Institutional Policies
- Reservation Policies
- Compliance Policies
- Allocation Policies

---

## Authorization Validation

Platform Authorization validates:

- User Role
- Permission
- Institution Access
- Campus Access
- Infrastructure Access

Transactions failing validation shall be rejected before any Infrastructure state changes occur.

---

# 30. Administrative Infrastructure Operations

Authorised administrators may perform controlled operational activities.

Supported operations include:

- Rename Infrastructure
- Update Business Code
- Correct Metadata
- Modify Operational Configuration
- Reassign Responsibilities
- Modify Capacity
- Configure Availability
- Configure Operational Policies
- Attach Documentation
- Correct Administrative Errors

Every administrative operation shall:

- Execute Business Validation
- Produce Immutable Audit History
- Publish Domain Events
- Respect Lifecycle Constraints
- Preserve Aggregate Identity

Administrative operations shall never compromise historical integrity.

---
```
# 31. Compliance Management Workflows

## Purpose

Compliance Management governs statutory, regulatory, institutional, operational, environmental, and safety compliance applicable to Infrastructure.

The Infrastructure domain owns compliance records associated with physical infrastructure.

Operational enforcement within downstream domains remains outside the scope of this domain.

---

## Compliance Principles

Infrastructure Compliance shall support:

- Institution-specific compliance frameworks
- Multi-country regulatory compliance
- Version-controlled compliance
- Effective dating
- Expiry management
- Auditability
- Workflow-driven approvals
- Configurable compliance policies

---

## Compliance Categories

Examples include:

### Statutory Compliance

- Fire NOC
- Occupancy Certificate
- Building Completion Certificate
- Structural Stability Certificate

---

### Safety Compliance

- Fire Extinguisher Inspection
- Emergency Exit Compliance
- Lift Safety Inspection
- Electrical Safety Audit

---

### Utility Compliance

- Water Quality Certification
- Generator Inspection
- DG Pollution Compliance
- Sewage Treatment Compliance

---

### Environmental Compliance

- Pollution Control
- Rainwater Harvesting
- Solar Installation Compliance
- Green Building Certification

---

### Institutional Compliance

- Internal Infrastructure Audit
- Accessibility Compliance
- Academic Infrastructure Standards
- Campus Safety Standards

Institutions may configure additional compliance categories.

---

## Compliance Registration Workflow

```text
Compliance Registration
          │
Authorization Validation
          │
Infrastructure Validation
          │
Policy Validation
          │
Compliance Created
          │
Publish Domain Event
```

---

## Compliance Version Management

Compliance records shall support:

- Multiple Versions
- Effective Dates
- Expiry Dates
- Superseded Versions
- Historical Versions

Historical versions shall remain immutable.

---

## Compliance Renewal Workflow

```text
Compliance Near Expiry
          │
Notification
          │
Renewal Initiated
          │
Inspection
          │
Approval
          │
Updated Compliance Version
```

---

## Expired Compliance

Expired compliance shall not automatically disable infrastructure.

Instead the platform shall:

- Notify Responsible Personnel
- Notify Compliance Officers
- Publish Compliance Expired Event
- Mark Infrastructure as "Compliance Attention Required"

Institutions may configure stricter enforcement policies.

---

# 32. Inspection Management Workflows

## Purpose

Inspection Management governs verification of Infrastructure condition, safety, operational readiness, and compliance.

Inspection records remain permanently associated with Infrastructure.

---

## Inspection Types

Supported examples include:

- Scheduled Inspection
- Surprise Inspection
- Safety Inspection
- Structural Inspection
- Compliance Inspection
- Accessibility Inspection
- Maintenance Inspection
- Administrative Inspection

Institutions may configure additional inspection types.

---

## Inspection Workflow

```text
Inspection Scheduled
         │
Inspector Assigned
         │
Inspection Conducted
         │
Findings Recorded
         │
Inspection Approved
         │
Inspection Closed
```

---

## Inspection Findings

Inspection findings may be classified as:

- Observation
- Minor Issue
- Major Issue
- Critical Risk
- Compliance Violation
- Recommendation

Institutions may configure additional classifications.

---

## Inspection Resolution

Inspection findings may initiate:

- Administrative Review
- Maintenance Request
- Compliance Review
- Re-inspection

Creation of Maintenance Requests shall remain a manual business decision.

Automatic work order generation shall not occur.

---

## Inspection Audit

Every inspection shall preserve:

- Inspector
- Inspection Date
- Findings
- Recommendations
- Supporting Documents
- Approval History

Inspection history shall never be deleted.

---

# 33. Preventive Maintenance Governance

## Purpose

Infrastructure governs preventive maintenance planning.

Execution of maintenance activities belongs to the Facilities domain.

---

## Preventive Maintenance Coverage

Examples include:

- Air Conditioning
- Lift Servicing
- Generator Servicing
- Fire Equipment
- Electrical Systems
- Plumbing Systems
- Water Tanks
- Roofing
- Building Structure

Institutions may configure additional maintenance programmes.

---

## Preventive Maintenance Workflow

```text
Maintenance Schedule
          │
Reminder Generated
          │
Maintenance Request Created
          │
Facilities Domain
          │
Maintenance Completed
          │
Infrastructure Updated
```

Infrastructure does not execute maintenance.

Infrastructure governs maintenance schedules and records.

---

## Preventive Maintenance Schedule

Maintenance schedules shall support:

- Daily
- Weekly
- Monthly
- Quarterly
- Half-Yearly
- Annual
- Usage-Based
- Institution Configurable

---

# 34. Corrective Maintenance Governance

Corrective Maintenance addresses defects identified after Infrastructure becomes operational.

Typical sources include:

- Inspection Findings
- Maintenance Requests
- Infrastructure Failure
- Administrative Review

Corrective Maintenance planning belongs to Infrastructure.

Execution belongs to Facilities.

---

## Corrective Maintenance Workflow

```text
Issue Identified
        │
Infrastructure Review
        │
Maintenance Required
        │
Facilities Execution
        │
Completion Confirmation
        │
Infrastructure Updated
```

---

# 35. Emergency Maintenance Governance

Emergency Maintenance addresses situations requiring immediate intervention.

Examples include:

- Electrical Failure
- Water Leakage
- Fire Damage
- Structural Damage
- Safety Hazard
- Natural Disaster

---

## Emergency Workflow

```text
Emergency Reported
         │
Infrastructure Assessment
         │
Emergency Classification
         │
Facilities Execution
         │
Infrastructure Revalidated
```

Emergency workflows may bypass normal approval chains where permitted by institutional policy.

Complete audit history shall still be maintained.

---

# 36. Capital Renovation Governance

Capital Renovation governs major modifications to Infrastructure.

Examples include:

- Building Renovation
- Floor Renovation
- Space Conversion
- Infrastructure Expansion
- Structural Strengthening

---

## Renovation Workflow

```text
Renovation Proposal
        │
Approval
        │
Infrastructure Closed
        │
Renovation Execution
        │
Inspection
        │
Compliance Validation
        │
Infrastructure Operational
```

Renovation shall preserve historical infrastructure identity wherever physical identity remains unchanged.

---

# 37. Sustainability Management

Infrastructure shall support recording sustainability information.

Examples include:

- Electricity Consumption
- Water Consumption
- Solar Generation
- Rainwater Harvesting
- Waste Management
- Carbon Reduction Initiatives

The Infrastructure domain owns operational sustainability records.

Advanced analytics remain outside this domain.

---

## Sustainability Recording Workflow

```text
Usage Data Recorded
         │
Validation
         │
Infrastructure Updated
         │
Publish Sustainability Event
```

---

# 38. Security Classification Management

Infrastructure may classify physical areas according to security requirements.

Examples include:

- Public
- Controlled
- Restricted
- Confidential
- Critical Infrastructure

Institutions may configure additional classifications.

---

## Security Classification Workflow

```text
Classification Request
         │
Authorization Validation
         │
Policy Validation
         │
Security Classification Updated
```

Security Classification shall not replace operational access control owned by the Security domain.

---

# 39. Disaster Recovery Governance

Infrastructure supports disaster recovery planning for physical facilities.

Examples include:

- Fire
- Flood
- Earthquake
- Structural Failure
- Utility Failure
- Civil Emergency

---

## Disaster Recovery Workflow

```text
Disaster Declared
        │
Infrastructure Assessment
        │
Affected Areas Identified
        │
Temporary Closure
        │
Recovery Planning
        │
Infrastructure Restored
```

---

## Evacuation Planning

Infrastructure may maintain:

- Assembly Points
- Emergency Routes
- Evacuation Maps
- Safe Zones

Operational evacuation activities remain within the Security domain.

---

# 40. Infrastructure Lifecycle Management

Infrastructure entities shall progress through controlled lifecycle states.

Illustrative lifecycle:

```text
Draft
   │
Submitted
   │
Approved
   │
Commissioned
   │
Operational
   │
Under Maintenance
   │
Operational
   │
Under Renovation
   │
Operational
   │
Temporarily Closed
   │
Operational
   │
Decommissioned
   │
Archived
```

Lifecycle transitions shall:

- Validate policy
- Validate authorization
- Produce audit history
- Publish immutable domain events
- Preserve historical integrity

Archived Infrastructure shall become read-only.

Deletion of Infrastructure aggregates shall never occur.

```

# 41. Infrastructure Decommissioning Workflows

## Purpose

Infrastructure Decommissioning governs the controlled retirement of Infrastructure from operational use.

Decommissioning permanently removes Infrastructure from operational availability while preserving complete historical integrity.

Infrastructure shall never be physically deleted from the enterprise architecture.

---

## Decommissioning Principles

Infrastructure Decommissioning shall ensure:

- Business authorization
- Policy compliance
- Historical preservation
- Audit completeness
- Dependency validation
- Event publication

---

## Decommissioning Preconditions

Before Infrastructure may be decommissioned, the platform shall validate:

### Operational Validation

- No active reservations.
- No pending maintenance activities.
- No active compliance actions.
- No active inspections.
- No operational dependency preventing retirement.

---

### Administrative Validation

- Administrative approval completed.
- Required documentation available.
- Decommission reason recorded.
- Lifecycle transition permitted.

---

## Decommission Workflow

```text
Operational Infrastructure
          │
Decommission Request
          │
Authorization Validation
          │
Dependency Validation
          │
Policy Validation
          │
Approval
          │
Infrastructure Decommissioned
          │
Publish Domain Event
```

---

## Decommission Effects

Upon successful decommissioning:

- Infrastructure becomes unavailable.
- New reservations prohibited.
- New allocations prohibited.
- Historical references preserved.
- Audit history retained.
- Downstream systems notified.

---

# 42. Infrastructure Archive Workflows

## Purpose

Archival preserves historical Infrastructure while preventing future operational modification.

Archived Infrastructure remains available for:

- Audit
- Reporting
- Regulatory Review
- Historical Reconstruction

---

## Archive Workflow

```text
Decommissioned
       │
Archive Request
       │
Validation
       │
Approval
       │
Archived
```

---

## Archive Characteristics

Archived Infrastructure:

- Becomes immutable.
- Remains searchable.
- Remains reportable.
- Remains auditable.
- Cannot be reconfigured.
- Cannot participate in operational workflows.

Infrastructure shall never be deleted through archival.

---

# 43. Configuration Management

## Purpose

Infrastructure behaviour shall remain configuration driven.

Institution-specific behaviour shall be controlled through configurable policies rather than application customization.

---

## Configurable Elements

Examples include:

### Infrastructure Configuration

- Campus Types
- Building Types
- Space Types
- Zone Types
- Capacity Types
- Operational Capabilities
- Security Classifications

---

### Workflow Configuration

Institutions may configure:

- Approval Workflows
- Lifecycle Policies
- Reservation Policies
- Allocation Policies
- Compliance Policies
- Inspection Policies

---

### Business Configuration

Institutions may configure:

- Naming Conventions
- Business Code Formats
- Infrastructure Tags
- Custom Metadata
- Classification Schemes

---

## Configuration Principles

Configuration shall support:

- Effective Dating
- Version Control
- Auditability
- Rollback
- Institution Isolation

---

# 44. Domain Events

Infrastructure Management publishes immutable domain events.

Every significant business transition shall generate an event.

---

## Registration Events

| Event | Description |
|--------|-------------|
| CampusRegistered | Campus successfully registered |
| ZoneRegistered | Zone created |
| BuildingRegistered | Building created |
| FloorRegistered | Floor created |
| SpaceRegistered | Space created |

---

## Lifecycle Events

| Event | Description |
|--------|-------------|
| InfrastructureApproved | Infrastructure approved |
| InfrastructureCommissioned | Infrastructure commissioned |
| InfrastructureOperational | Infrastructure operational |
| InfrastructureClosed | Infrastructure closed |
| InfrastructureReopened | Infrastructure reopened |
| InfrastructureDecommissioned | Infrastructure retired |
| InfrastructureArchived | Infrastructure archived |

---

## Allocation Events

| Event | Description |
|--------|-------------|
| DepartmentAllocated | Department allocation completed |
| InfrastructureAllocationUpdated | Allocation modified |
| AllocationEnded | Allocation closed |

---

## Reservation Events

| Event | Description |
|--------|-------------|
| InfrastructureReservationCreated | Reservation successful |
| ReservationModified | Reservation updated |
| ReservationCancelled | Reservation cancelled |
| InfrastructureReservationExpired | Reservation completed |

---

## Capacity Events

| Event | Description |
|--------|-------------|
| CapacityUpdated | Capacity modified |
| CapacityOverrideApplied | Temporary capacity override |
| CapacityOverrideExpired | Capacity restored |

---

## Compliance Events

| Event | Description |
|--------|-------------|
| ComplianceRegistered | Compliance created |
| ComplianceUpdated | Compliance modified |
| ComplianceExpired | Compliance expired |
| ComplianceRenewed | Compliance renewed |

---

## Inspection Events

| Event | Description |
|--------|-------------|
| InfrastructureInspectionScheduled | Inspection scheduled |
| InfrastructureInspectionCompleted | Inspection completed |
| FindingRecorded | Finding recorded |

---

## Maintenance Events

| Event | Description |
|--------|-------------|
| PreventiveMaintenanceScheduled | Preventive maintenance planned |
| CorrectiveMaintenanceRequested | Corrective maintenance initiated |
| EmergencyMaintenanceDeclared | Emergency maintenance required |

---

## Sustainability Events

| Event | Description |
|--------|-------------|
| UtilityConsumptionRecorded | Utility usage recorded |
| SustainabilityMetricUpdated | Sustainability data updated |

---

# 44A. Infrastructure Events, Integration Contracts & External Interfaces

> Migrated from `L2-B03_Campus_Workflows.md` Part 12 on 2026-08-05 under integrity finding
> INT-008. The part described the event architecture and read contracts of the
> Infrastructure aggregate throughout - it was G10's integration architecture held in B03.
> Sections 44 and 45 above remain the summary; the detail is below.

Section 44A.190 was titled "Campus Summary Contract" in the source and mixed institutional
fields with physical ones. It is retitled Physical Campus Summary Contract here and carries
the physical fields; the institutional counterpart stays in `L2-B03`.

---


---

## 44A.177 Integration Philosophy

The Campus & Infrastructure domain is an **authoritative master data domain**.

It owns canonical facts regarding institutional physical infrastructure.

The domain shall integrate with the remainder of SutraERP using:

- Canonical Domain Events
- Public Read Contracts
- Query APIs
- Integration Services

External systems shall **never depend upon internal persistence models**.

---

## 44A.178 Integration Principles

The domain follows these integration principles.

#### IP-01

Infrastructure publishes facts.

Consumers determine business behaviour.

---

#### IP-02

Infrastructure never invokes downstream business processes.

---

#### IP-03

Integration shall be event-driven wherever possible.

---

#### IP-04

Public contracts remain stable across implementation changes.

---

#### IP-05

Internal aggregate models remain private.

---

#### IP-06

Infrastructure is the single source of truth for physical infrastructure.

---

## 44A.179 Event Architecture

```text
Infrastructure Aggregate
          │
          ▼
Business Validation
          │
          ▼
State Change
          │
          ▼
Persist Aggregate
          │
          ▼
Publish Domain Event
          │
          ▼
Enterprise Event Bus
          │
          ▼
Consumer Domains
```

Events are published **after successful transaction completion**.

Events never represent commands.

---

## 44A.180 Event Categories

Infrastructure publishes four categories of events.

| Category | Purpose |
|-----------|---------|
| Lifecycle Events | State transitions |
| Structural Events | Physical hierarchy changes |
| Classification Events | Metadata updates |
| Relationship Events | Logical organization changes |

---

## 44A.181 Lifecycle Events

Lifecycle Events communicate changes in operational existence.

Examples include:

- CampusActivated
- CampusDecommissioned
- BuildingActivated
- BuildingDecommissioned
- FloorActivated
- FloorArchived
- SpaceOperational
- SpaceArchived

Lifecycle Events are immutable business facts.

---

## 44A.182 Structural Events

Structural Events communicate changes to the physical hierarchy.

Examples include:

- CampusCreated
- BuildingCreated
- FloorCreated
- SpaceCreated
- BuildingUpdated
- FloorUpdated
- SpaceUpdated
- SpaceMoved *(future extension if relocation is supported)*

Structural Events communicate authoritative infrastructure facts.

---

## 44A.183 Classification Events

Classification Events describe changes in infrastructure metadata.

Examples include:

- InfrastructureClassificationChanged
- RoomCapacityChanged
- SpaceAreaUpdated
- SpaceAccessibilityUpdated
- SustainabilityProfileUpdated
- SpaceHealthUpdated
- SpaceCriticalityChanged

These events notify consumers that canonical metadata has changed.

---

## 44A.184 Relationship Events

Relationship Events communicate logical organization changes.

Examples include:

- AssignedToZone
- RemovedFromZone
- AssignedToCollection
- RemovedFromCollection
- AssignedToPortfolio
- RemovedFromPortfolio

Relationship Events never modify the physical hierarchy.

---

## 44A.185 Event Structure

Every Domain Event contains a common envelope.

```text
Event ID

Event Type

Aggregate Type

Aggregate Identifier

Institution Identifier

Tenant Identifier

Occurred Timestamp

Event Version

Correlation Identifier

Initiated By

Payload
```

---

## 44A.186 Event Payload Principles

Event payloads shall contain only canonical business facts.

Payloads shall not contain:

- UI information
- Calculated KPIs
- Analytics
- Temporary processing data
- Consumer-specific projections

Events remain implementation-independent.

---

## 44A.187 Event Versioning

Every event shall be versioned.

Example

```text
SpaceCreated

Version 1
```

Future changes shall create new event versions without breaking existing consumers.

Example

```text
SpaceCreated V1

↓

SpaceCreated V2
```

Older versions remain supported according to enterprise integration policy.

---

## 44A.188 Event Consumers

| Consumer Domain | Event Usage |
|-----------------|-------------|
| Admissions | Campus & room availability |
| Timetable | Classroom allocation |
| Examination | Examination hall availability |
| HR | Workplace locations |
| Hostel | Hostel infrastructure |
| Library | Branch infrastructure |
| Maintenance | Maintenance scheduling |
| Asset Management | Physical hierarchy |
| Security | Access planning |
| Transport | Pickup infrastructure |
| Analytics | Infrastructure facts |
| AI | Semantic interpretation |
| Disaster Management | Recovery planning |
| Notifications | Business notifications |

---

## 44A.189 Public Read Contracts

Infrastructure exposes stable read models.

Examples include:

- Campus Summary
- Building Summary
- Floor Summary
- Space Summary
- Infrastructure Capacity Profile
- Accessibility Profile
- Sustainability Profile
- Criticality Profile
- Lifecycle Status
- Operational Availability

Consumers retrieve information through these contracts.

---

## 44A.190 Physical Campus Summary Contract

Provides canonical Campus information.

Fields include:

- Campus Identifier
- Campus Code
- Campus Name
- Institution Identifier
- Lifecycle State
- Operational Status
- Address
- Geo Coordinates

Consumers shall not access Campus aggregates directly.

---

## 44A.191 Building Summary Contract

Fields include:

- Building Identifier
- Building Code
- Building Name
- Campus Identifier
- Building Type
- Lifecycle
- Operational Status

---

## 44A.192 Floor Summary Contract

Fields include:

- Floor Identifier
- Floor Number
- Floor Name
- Parent Building
- Operational Status

---

## 44A.193 Space Summary Contract

Fields include:

- Space Identifier
- Space Code
- Space Name
- Space Type
- Parent Floor
- Capacity
- Area
- Operational Status
- Lifecycle

This is the most frequently consumed Infrastructure contract.

---

## 44A.194 Capacity Profile Contract

Contains canonical capacity facts.

Examples:

- Seating Capacity
- Standing Capacity
- Maximum Occupancy
- Operational Capacity

Analytics derives utilization independently.

---

## 44A.195 Accessibility Profile Contract

Contains accessibility capabilities.

Examples:

- Wheelchair Access
- Lift Access
- Braille Signage
- Hearing Assistance

Infrastructure exposes facts only.

Compliance interpretation belongs elsewhere.

---

## 44A.196 Sustainability Profile Contract

Contains sustainability capabilities.

Examples:

- Solar Installed
- Rainwater Harvesting
- LED Lighting
- EV Charging

Infrastructure does not expose sustainability KPIs.

---

## 44A.197 Query Services

Infrastructure supports enterprise query services.

Typical queries include:

- Find Campus
- Find Building
- Find Floor
- Find Space
- Search by Capacity
- Search by Classification
- Search by Accessibility
- Search by Zone
- Search by Collection
- Search by Portfolio
- Search by Lifecycle

Queries never expose aggregate internals.

---

## 44A.198 Consumer Responsibilities

Consumers are responsible for:

- Business Decisions
- Scheduling
- Allocations
- Analytics
- Reporting
- AI Interpretation
- Notifications

Infrastructure remains responsible only for physical facts.

---

## 44A.199 Integration Constraints

External consumers shall **not**:

- Modify Infrastructure aggregates directly.
- Depend on database schemas.
- Assume event ordering across aggregates.
- Infer undocumented business meaning.

Consumers interact only through supported contracts and events.

---

## 44A.200 Integration Design Principles

#### ICP-01

Infrastructure publishes immutable business facts.

---

#### ICP-02

Events describe completed business actions.

---

#### ICP-03

Public contracts shield consumers from implementation changes.

---

#### ICP-04

Consumers own business reactions.

---

#### ICP-05

Infrastructure never embeds consumer-specific behaviour.

---

#### ICP-06

Integration remains asynchronous wherever practical.

---

#### ICP-07

Infrastructure remains the canonical master for physical infrastructure data.

---

---

# 45. Service Contracts

Infrastructure exposes business services to downstream domains.

Examples include:

### Registration Services

- Register Campus
- Register Building
- Register Floor
- Register Space

---

### Query Services

- Retrieve Infrastructure
- Retrieve Capacity
- Retrieve Availability
- Retrieve Reservations
- Retrieve Compliance Status

---

### Operational Services

- Reserve Space
- Release Reservation
- Allocate Department
- Assign Custodian
- Assign Operational Manager

---

### Governance Services

- Register Compliance
- Schedule Inspection
- Record Inspection
- Update Capacity
- Change Availability

All interactions shall occur through published service contracts.

---

# 46. State Transition Matrix

| Entity | Primary Lifecycle |
|---------|-------------------|
| Campus | Draft → Approved → Commissioned → Operational → Closed → Decommissioned → Archived |
| Building | Draft → Approved → Operational → Maintenance → Operational → Archived |
| Floor | Draft → Approved → Operational → Closed → Archived |
| Space | Draft → Approved → Operational → Reserved → Available → Closed → Archived |
| Compliance | Draft → Active → Expiring → Expired → Renewed |
| Inspection | Scheduled → In Progress → Completed → Approved → Closed |
| Reservation | Requested → Confirmed → Active → Completed → Archived |

Institution-specific lifecycle extensions may be configured.

---

# 47. Business Rules

The following business rules shall always apply.

- Infrastructure Identity is immutable.
- Business Codes may change.
- Every Space belongs to one Floor.
- Every Floor belongs to one Building.
- Every Building belongs to one Campus.
- Capacity shall never become negative.
- Archived Infrastructure is immutable.
- Infrastructure shall never be physically deleted.
- Reservation conflicts shall be prevented unless administratively overridden.
- Compliance history shall remain immutable.
- Inspection history shall remain immutable.
- Every lifecycle transition shall be audited.
- Every significant transition shall publish a domain event.
- Infrastructure ownership shall remain canonical within this domain.
- Downstream domains shall reference Infrastructure rather than duplicate physical locations.

Violation of business rules shall reject the transaction.

---

# 48. Authorization Model

Infrastructure authorization shall be evaluated through the Platform Authorization Framework.

Typical responsibilities include:

- Infrastructure Administrator
- Campus Administrator
- Infrastructure Manager
- Compliance Officer
- Maintenance Coordinator
- Operational Manager
- Read-Only Auditor

Institutions may define additional roles.

Authorization shall support:

- Role Based Access Control
- Hierarchical Authorization
- Delegated Responsibility
- Effective-Dated Delegation

---

# 48A. Infrastructure Security, Authorization, Audit & Governance

> Migrated from `L2-B03_Campus_Workflows.md` Part 13 on 2026-08-05 under integrity
> finding INT-008. Sections 48-51 above remain the summary; the detail is below.

---


---

## 48A.201 Security Philosophy

The Campus & Infrastructure domain protects institutional physical infrastructure master data.

Security objectives include:

- Protecting infrastructure integrity
- Preventing unauthorized modification
- Ensuring traceability
- Supporting institutional governance
- Preserving historical accuracy
- Enforcing least-privilege access

Security policies define **who may perform an action**, while business rules define **whether the action is valid**.

---

## 48A.202 Security Architecture Principles

The domain follows the enterprise security architecture established by the Platform Foundation.

Infrastructure does **not** implement authentication.

Authentication is delegated to:

- Identity Domain
- Authorization Domain
- Platform Foundation

The Infrastructure domain consumes authenticated user context.

---

## 48A.203 Authorization Model

Authorization is permission-based.

Permissions are evaluated before any business operation.

Authorization decisions consider:

- Tenant
- Institution
- User
- Role
- Permission
- Scope

Business rules execute only after authorization succeeds.

---

## 48A.204 Infrastructure Roles

The following logical roles represent common access patterns.

| Role | Typical Responsibilities |
|------|---------------------------|
| Platform Administrator | Global platform administration |
| Tenant Administrator | Tenant-level governance |
| Institution Administrator | Institution-wide infrastructure oversight |
| Infrastructure Administrator | Full infrastructure management |
| Campus Administrator | Campus-level administration |
| Building Administrator | Building-level administration |
| Facilities Manager | Operational facility management |
| Maintenance Coordinator | Infrastructure maintenance coordination |
| Auditor | Read-only audit access |
| Read-Only User | Infrastructure viewing only |

Role definitions remain configurable through the Identity & Access Management domain.

---

## 48A.205 Permission Categories

Infrastructure permissions are grouped into functional categories.

| Category | Examples |
|----------|----------|
| View | View Campus, Building, Space |
| Create | Register Infrastructure |
| Update | Modify Metadata |
| Lifecycle | Activate, Decommission, Archive |
| Classification | Update Classification |
| Relationship | Manage Zones, Collections, Portfolios |
| Administration | Infrastructure Configuration |
| Audit | View History & Audit Trail |

Permissions are fine-grained and independently assignable.

---

## 48A.206 Resource Scoping

Permissions may be scoped to different organizational levels.

Examples include:

- Tenant
- Institution
- Campus
- Building
- Zone

Illustrative examples:

- A Campus Administrator may manage only infrastructure within assigned campuses.
- A Building Administrator may update spaces only within assigned buildings.

Scope evaluation is performed by the Authorization domain.

---

## 48A.207 Authorization Workflow

```text
User Request
      │
      ▼
Authentication
      │
      ▼
Authorization Check
      │
      ▼
Permission Granted
      │
 ┌────┴────┐
 │         │
 ▼         ▼
Yes       No
 │         │
 ▼         ▼
Business   Access
Rules      Denied
 │
 ▼
Persist
 │
 ▼
Audit
 │
 ▼
Publish Event
```

Authorization failures terminate processing before any state mutation.

---

## 48A.208 Audit Philosophy

Every significant business action shall be auditable.

Audit records provide:

- Accountability
- Regulatory compliance
- Operational traceability
- Historical reconstruction
- Governance evidence

Audit history is immutable.

---

## 48A.209 Auditable Operations

The following operations shall generate audit records.

#### Infrastructure Creation

Examples:

- Campus Created
- Building Created
- Floor Created
- Space Created

---

#### Infrastructure Modification

Examples:

- Name Changed
- Capacity Updated
- Accessibility Updated
- Sustainability Updated
- Classification Updated

---

#### Lifecycle Operations

Examples:

- Activated
- Decommissioned
- Archived
- Reopened
- Temporarily Closed

---

#### Relationship Operations

Examples:

- Assigned to Zone
- Removed from Zone
- Assigned to Collection
- Assigned to Portfolio

---

#### Administrative Operations

Examples:

- Configuration Changes
- Governance Overrides
- Bulk Updates

---

## 48A.210 Audit Record Structure

Every audit entry shall include:

- Audit Identifier
- Timestamp
- User Identifier
- Tenant Identifier
- Institution Identifier
- Aggregate Type
- Aggregate Identifier
- Operation Type
- Previous Values (where applicable)
- New Values
- Business Reason (if supplied)
- Correlation Identifier
- Source System

Audit records shall be immutable after creation.

---

## 48A.211 Governance Overrides

Some operations require elevated approval.

Examples include:

- Archive Operational Infrastructure
- Decommission Critical Buildings
- Bulk Lifecycle Changes
- Administrative Data Corrections

Governance approval requirements are configurable.

Infrastructure records the approval outcome but does not implement workflow routing.

Approval orchestration belongs to Workflow or Governance domains.

---

## 48A.212 Data Retention

Infrastructure master data is retained according to institutional retention policies.

General principles include:

- Operational infrastructure is never physically deleted.
- Archived infrastructure remains queryable for authorized users.
- Audit history is retained independently of operational status.
- Historical references remain valid across the platform.

Retention periods are configured by institutional governance policies.

---

## 48A.213 Privacy & Sensitive Data

The Campus & Infrastructure domain stores infrastructure metadata only.

It does **not** own or manage:

- Student personal information
- Employee personal information
- Academic records
- Financial records
- Medical records

Infrastructure references individuals only where operationally required (e.g., "Building Administrator"), with identity details resolved through the Identity domain.

---

## 48A.214 Compliance Considerations

The architecture supports institutional compliance obligations, including:

- Accreditation audits
- Government inspections
- Fire and safety audits
- Accessibility reviews
- Infrastructure governance reviews
- Internal audit requirements

The domain provides authoritative evidence but does not interpret regulatory requirements.

---

## 48A.215 Data Integrity Controls

The following controls protect master data integrity.

| Control | Purpose |
|---------|---------|
| Immutable Identifiers | Stable entity identity |
| Business Rule Validation | Prevent invalid states |
| Lifecycle Enforcement | Controlled state transitions |
| Authorization Checks | Prevent unauthorized actions |
| Audit Logging | Complete traceability |
| Event Publication | Enterprise consistency |

These controls operate together to preserve authoritative infrastructure records.

---

## 48A.216 Multi-Tenant Governance

Infrastructure data is isolated by Tenant.

Governance principles include:

- Tenant data isolation
- Institution-level ownership
- No cross-tenant visibility
- Tenant-specific configuration
- Independent audit history

Platform Foundation enforces isolation boundaries.

---

## 48A.217 Error Handling Principles

Security and governance failures follow consistent handling rules.

Examples:

| Failure | Outcome |
|---------|---------|
| Authentication Failure | Reject request |
| Authorization Failure | Reject request |
| Validation Failure | Reject request with validation details |
| Business Rule Violation | Reject request |
| Audit Failure | Transaction fails |
| Event Publication Failure* | Retry according to enterprise messaging policy |

\*Where the enterprise uses reliable messaging (e.g., transactional outbox), persistence succeeds before asynchronous event delivery.

No partial business updates are permitted.

---

## 48A.218 Governance Principles

#### GP-01

Authentication is external to the Infrastructure domain.

---

#### GP-02

Authorization precedes business validation.

---

#### GP-03

Every successful state change is auditable.

---

#### GP-04

Operational infrastructure is preserved for historical integrity.

---

#### GP-05

Security policies remain independent of business rules.

---

#### GP-06

Governance approvals supplement business validation; they do not replace it.

---

#### GP-07

Audit records are immutable and independently retained.

---

#### GP-08

Infrastructure remains the authoritative source for physical infrastructure master data.

---

## 48A.219 Responsibility Matrix

| Capability | Owning Domain |
|------------|---------------|
| Authentication | Identity & Access Management |
| Authorization | Identity & Access Management |
| Infrastructure Business Rules | Campus & Infrastructure |
| Audit Logging | Platform Foundation |
| Workflow Approval | Workflow / Governance |
| Data Retention Policy | Governance |
| Tenant Isolation | Platform Foundation |
| Event Delivery | Integration Platform |
| Regulatory Reporting | Analytics / Compliance Domains |

---

---

# 49. Audit Responsibilities

Infrastructure shall maintain immutable audit history.

Audit records shall include:

- Actor
- Timestamp
- Previous State
- New State
- Business Reason
- Infrastructure Reference
- Approval Reference
- Source System

Audit history shall never be modified or deleted.

---

# 50. Reporting Responsibilities

Infrastructure provides operational information for reporting.

Examples include:

- Campus Inventory
- Building Inventory
- Space Inventory
- Capacity Reports
- Utilisation Reports
- Reservation Reports
- Compliance Reports
- Inspection Reports
- Sustainability Reports
- Infrastructure Lifecycle Reports

Analytical processing remains outside this domain.

---

# 51. Exception Handling

Infrastructure shall support controlled handling of exceptional situations.

Examples include:

- Invalid Hierarchy
- Reservation Conflict
- Compliance Failure
- Capacity Violation
- Infrastructure Closure
- Disaster Recovery
- Emergency Maintenance
- Authorization Failure

Exceptions shall:

- Preserve consistency.
- Produce audit history.
- Publish events where applicable.
- Prevent invalid state transitions.

---

# 52. Performance & Scalability

The Infrastructure domain shall support enterprise deployments including:

- Multiple Institutions
- Multiple Countries
- Hundreds of Campuses
- Hundreds of Buildings
- Thousands of Floors
- Tens of Thousands of Spaces
- Millions of Historical Audit Records

The architecture shall remain horizontally scalable and configuration driven.

---

# 53. Repository Principles

The Infrastructure Management domain conforms to the following enterprise principles.

- Domain-Driven Design
- Event-Driven Architecture
- CQRS
- Canonical Infrastructure Ownership
- Digital Physical Twin
- Immutable Audit History
- Effective-Dated Configuration
- Multi-Campus Support
- Multi-Institution Support
- Multi-Country Support
- Configuration over Customisation
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- Single Aggregate Ownership

---

# 54. Glossary

| Term | Description |
|------|-------------|
| Canonical Ownership | Exclusive ownership of a business concept by a single domain. |
| Digital Physical Twin | Enterprise business representation of physical infrastructure. |
| Infrastructure Space | Smallest operational physical unit within SutraERP. |
| Capacity Profile | Configurable operational capacity definition for Infrastructure. |
| Reservation | Temporary operational assignment of Infrastructure. |
| Compliance | Regulatory, statutory, or institutional requirement associated with Infrastructure. |
| Custodian | Administrative owner responsible for Infrastructure oversight. |
| Operational Manager | Person responsible for day-to-day infrastructure operations. |
| Decommissioning | Permanent retirement of Infrastructure from operational use. |
| Archival | Historical preservation of Infrastructure after operational retirement. |

---

# 55. Appendices

## Appendix A — Canonical Infrastructure Hierarchy

```text
Institution
    │
Campus
    │
Zone (Optional)
    │
Building
    │
Wing / Block (Optional)
    │
Floor
    │
Space
```

---

## Appendix B — Infrastructure Ownership

Infrastructure Management owns:

- Physical Hierarchy
- Infrastructure Identity
- Capacity
- Availability
- Reservation
- Compliance
- Inspection
- Lifecycle
- Operational Allocation

Downstream domains consume Infrastructure through published service contracts and immutable domain events.

---

**End of Document**

**L2-G10 — Infrastructure Management Workflows**

**Version:** 1.0  
**Status:** Draft
