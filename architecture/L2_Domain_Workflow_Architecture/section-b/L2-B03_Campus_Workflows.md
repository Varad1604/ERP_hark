# L2-B03 Campus Workflows

---

# Document Control

| Attribute | Value |
|------------|-------|
| **Document ID** | L2-B03 |
| **Document Name** | Campus Workflows |
| **Architecture Level** | Level 2 – Domain Architecture |
| **Domain Code** | B03 |
| **Domain Category** | Foundation Domain |
| **Status** | Architecture Locked |
| **Repository** | SutraERP Enterprise Architecture |
| **Depends On** | L1 Platform Foundation, L2-B01 Institution, L2-B02 Organisation |
| **Consumed By** | All infrastructure-dependent domains |
| **Architecture Style** | Domain-Driven Design (DDD), Event-Driven Architecture, Modular Monolith Ready, Microservice Compatible |

---

# 1. Purpose

The **Campus & Infrastructure** domain defines the **canonical representation of all physical infrastructure** managed within SutraERP.

It establishes the enterprise model for campuses, buildings, floors, spaces, facilities, utilities, infrastructure groupings, operational characteristics, compliance metadata, lifecycle states, and physical relationships.

This domain serves as the **authoritative source of truth** for all physical infrastructure across the platform.

It provides a stable, extensible, and future-proof foundation upon which every infrastructure-dependent business domain operates.

---

# 2. Domain Mission

The mission of this domain is to provide a single, authoritative, and technology-independent representation of institutional infrastructure that enables every other domain to understand, reference, and utilize physical facilities consistently without duplicating infrastructure knowledge.

This domain describes:

- What infrastructure exists
- How infrastructure is organized
- What characteristics infrastructure possesses
- How infrastructure changes throughout its lifecycle

This domain **does not** manage operational activities performed within that infrastructure.

---

# 3. Scope

The Campus & Infrastructure domain governs the complete lifecycle and structure of physical infrastructure, including:

- Campuses
- Campus Sites
- Buildings
- Building Wings
- Floors
- Rooms
- Laboratories
- Auditoriums
- Libraries
- Hostels *(physical facilities only)*
- Sports Infrastructure
- Utility Infrastructure
- Roads
- Parking Areas
- Open Spaces
- Service Facilities
- Physical Zones
- Infrastructure Collections
- Infrastructure Portfolios
- Physical Relationships
- Infrastructure Classification
- Capacity Information
- Accessibility Characteristics
- Sustainability Capabilities
- Compliance Characteristics
- Infrastructure Health
- Infrastructure Criticality
- Infrastructure Lifecycle
- Infrastructure Availability
- Canonical Infrastructure Events

---

# 4. Out of Scope

The following responsibilities explicitly belong to other domains.

## 4.1 Academic Operations

This domain does **not** manage:

- Timetable allocation
- Classroom scheduling
- Examination hall allocation
- Teaching activities

---

## 4.2 Admissions

This domain does **not** manage:

- Seat allocation
- Admission capacity
- Intake planning

Infrastructure only publishes physical capacities.

---

## 4.3 Asset Management

Infrastructure is **not** an asset register.

Examples excluded:

- Furniture
- Computers
- Laboratory equipment
- Projectors
- Vehicles
- Network devices

Those belong to the Asset Management domain.

Infrastructure only defines **where assets exist**.

---

## 4.4 Maintenance

Infrastructure does **not** manage:

- Work orders
- Preventive maintenance
- Corrective maintenance
- Repair activities
- Technician assignments

It only exposes infrastructure health and lifecycle information.

---

## 4.5 Security

Infrastructure does **not** manage:

- Guards
- CCTV operations
- Incident response
- Visitor management
- Physical security operations

Infrastructure merely identifies physical locations and security-related characteristics.

---

## 4.6 Disaster Management

Infrastructure does **not** execute:

- Emergency response
- Crisis management
- Disaster recovery operations
- Incident coordination

It only defines:

- Criticality
- Recovery characteristics
- Alternate facilities
- Resilience metadata

---

## 4.7 Analytics

Infrastructure does **not** calculate:

- KPIs
- Trends
- Benchmarks
- Executive dashboards
- Forecasts

It publishes canonical operational facts.

---

## 4.8 Artificial Intelligence

Infrastructure does **not** maintain:

- AI semantic models
- Knowledge graphs
- AI interpretations
- Recommendations
- Predictive intelligence

Future AI platforms derive meaning from canonical infrastructure facts.

---

# 5. Domain Vision

The Campus & Infrastructure domain enables SutraERP to maintain a single enterprise-wide understanding of every physical facility regardless of institution size, educational model, or deployment architecture.

It supports:

- Single-campus institutions
- Multi-campus universities
- Multi-institute educational trusts
- Autonomous colleges
- Research institutions
- Skill development centres
- School networks
- Technical universities
- Medical universities
- Large educational ecosystems

without changing the core domain model.

---

# 6. Guiding Principles

The domain is governed by the following architectural principles.

---

## GP-01 — Single Source of Truth

Every physical infrastructure entity shall exist only once within the enterprise model.

Duplicate infrastructure definitions are prohibited.

---

## GP-02 — Facts Over Interpretation

Infrastructure owns business facts.

It does **not** own:

- KPIs
- Executive metrics
- AI interpretations
- Predictive analytics

---

## GP-03 — Physical World Representation

Infrastructure represents the physical world.

It does **not** represent business processes executed inside the physical world.

---

## GP-04 — Long-Lived Records

Infrastructure records are persistent enterprise objects.

Deletion is exceptional.

Lifecycle transitions are preferred.

---

## GP-05 — Technology Independence

The domain model shall remain independent of:

- Database technology
- User interface implementation
- API technology
- Messaging platform
- AI platform

---

## GP-06 — Event-Driven Integration

Infrastructure publishes canonical business events.

Consumer domains decide how to respond.

Infrastructure never orchestrates downstream workflows.

---

## GP-07 — Stable Public Contracts

Infrastructure owns its internal model.

External domains consume only published contracts and canonical events.

Internal implementation details remain private.

---

## GP-08 — Future Extensibility

The domain shall accommodate future capabilities without architectural redesign, including:

- Smart Campus
- IoT Integration
- Digital Twins
- GIS Integration
- Facilities Management
- Building Automation
- Sustainability Platforms
- Disaster Management
- Enterprise Analytics

---

# 7. Architecture Position

Within the SutraERP architecture, Campus & Infrastructure is a **Foundation Domain**.

It provides enterprise-wide physical context consumed by numerous operational domains.

```text
                    SutraERP Platform

                           │
                 Institution (L2-B01)
                           │
                    Organisation (L2-B02)
                           │
          ┌─────────────────────────────────┐
          │                                 │
          ▼                                 ▼
Campus & Infrastructure (L2-B03)      Other Foundation Domains
          │
          ├───────────────┬───────────────┬───────────────┐
          ▼               ▼               ▼               ▼
     Admissions      Timetable      Examination      Hostel
          │
          ▼
 HR, Finance, Assets, Transport,
 Library, Maintenance, Security,
 Analytics, AI, Disaster Management
```

The domain supplies the canonical physical infrastructure model while remaining independent of business workflows executed by consuming domains.

---

# 8. Architecture Characteristics

| Characteristic | Decision |
|----------------|----------|
| Domain Type | Foundation Domain |
| Authoritative Source | Yes |
| Event Publisher | Yes |
| Event Consumer | Yes |
| Lifecycle Managed | Yes |
| Configurable | Yes |
| Multi-Campus | Supported |
| Multi-Institution | Supported |
| Multi-Tenant | Supported through L1 |
| DDD Aggregate Based | Yes |
| Event-Driven | Yes |
| CQRS Compatible | Yes |
| AI Ready | Facts Only |
| Analytics Ready | Facts Only |
| Microservice Ready | Yes |
| Modular Monolith Ready | Yes |

---

# Part 1 Completion

This part establishes the architectural foundation of the **Campus & Infrastructure** domain by defining:

- Purpose
- Domain Mission
- Scope
- Out of Scope
- Domain Vision
- Guiding Principles
- Architecture Position
- Architecture Characteristics

Subsequent parts build upon these principles to define the complete canonical infrastructure model, workflows, aggregates, business rules, events, public contracts, and cross-domain interactions.

---

# Part 2 — Domain Vision, Bounded Context & Ownership Model

---

# 9. Domain Vision

The **Campus & Infrastructure** domain establishes the canonical enterprise representation of every physical location within the SutraERP ecosystem.

Its objective is to ensure that every domain refers to the same physical infrastructure using a consistent business language, regardless of deployment model, institution type, or organizational complexity.

The domain serves as the enterprise-wide foundation upon which all infrastructure-dependent capabilities operate.

---

# 10. Strategic Objectives

The Campus & Infrastructure domain exists to achieve the following strategic objectives.

## SO-01 — Establish a Canonical Physical Model

Provide a single enterprise-wide model representing:

- Campuses
- Buildings
- Physical Spaces
- Infrastructure Hierarchies
- Infrastructure Relationships

No other domain shall redefine these concepts.

---

## SO-02 — Eliminate Infrastructure Duplication

Prevent multiple domains from maintaining independent definitions of:

- Buildings
- Rooms
- Laboratories
- Auditoriums
- Facilities

Infrastructure information shall originate only from this domain.

---

## SO-03 — Enable Enterprise Consistency

Every business workflow referencing physical infrastructure shall reference the same canonical infrastructure entity.

Examples include:

- Admissions
- Timetable
- Examination
- Hostel
- Transport
- Library
- Maintenance
- Asset Management
- Security

---

## SO-04 — Support Long-Term Evolution

The domain shall remain stable while supporting future capabilities including:

- Smart Campus
- Digital Twin
- GIS Integration
- IoT
- AI
- Sustainability Platforms
- Enterprise Facilities Management

without requiring redesign of the core infrastructure model.

---

## SO-05 — Enable Loose Coupling

Infrastructure publishes business facts.

Consumer domains independently implement business workflows.

No downstream domain shall become tightly coupled to Infrastructure's internal implementation.

---

# 11. Bounded Context

Within the SutraERP Domain-Driven Design architecture, Campus & Infrastructure represents an independent **Bounded Context**.

It owns every concept describing the physical infrastructure of an institution.

Other domains may reference infrastructure but shall never redefine or own infrastructure concepts.

---

## Bounded Context Responsibilities

This bounded context owns:

- Physical locations
- Infrastructure hierarchy
- Infrastructure classifications
- Capacity metadata
- Accessibility metadata
- Sustainability capabilities
- Infrastructure lifecycle
- Infrastructure health
- Infrastructure criticality
- Infrastructure relationships
- Infrastructure availability
- Canonical infrastructure events

---

## Bounded Context Does Not Own

This bounded context explicitly excludes:

- Academic scheduling
- Student allocation
- Examination planning
- Asset inventories
- Maintenance execution
- Financial costing
- HR assignments
- Security operations
- Analytics
- AI reasoning

---

# 12. Ubiquitous Language

The following terminology is canonical throughout SutraERP.

| Term | Meaning |
|------|----------|
| Campus | A geographically identifiable institutional site. |
| Building | A physical structure within a campus. |
| Wing | An optional child entity owned by the Building Aggregate representing a building subdivision. |
| Floor | A physical floor within a building or wing. |
| Space | Any usable physical location. |
| Room | An enclosed physical space. |
| Facility | A specialized infrastructure resource serving a business purpose. |
| Infrastructure | Any managed physical entity within institutional premises. |
| Zone | Logical grouping of nearby infrastructure for operational purposes. |
| Collection | Logical grouping based upon business characteristics. |
| Portfolio | Strategic grouping used for governance and investment planning. |
| Capacity | Maximum supported physical occupancy or utilization limit. |
| Accessibility | Characteristics enabling inclusive access. |
| Criticality | Importance of infrastructure for business continuity. |
| Lifecycle | Represents the long-term business existence of infrastructure rather than its current operational condition. |
| Availability | Whether infrastructure is currently usable. |

These definitions are authoritative across all SutraERP domains.

---

# 13. Domain Responsibilities

Campus & Infrastructure owns the complete lifecycle of physical infrastructure.

Responsibilities include:

## Infrastructure Registration

- Create infrastructure
- Update infrastructure
- Retire infrastructure
- Archive infrastructure

---

## Hierarchy Management

Manage enterprise physical hierarchy including:

- Campus
- Building
- Wing
- Floor
- Space

---

## Classification

Assign and manage:

- Infrastructure Type
- Infrastructure Category
- Functional Classification
- Operational Classification

---

## Capacity Management

Maintain canonical capacity information.

Examples:

- Seating Capacity
- Standing Capacity
- Operational Capacity
- Occupancy Limit

---

## Accessibility

Maintain physical accessibility characteristics including:

- Wheelchair Accessibility
- Elevator Availability
- Accessible Restrooms
- Accessible Parking

---

## Sustainability

Maintain infrastructure sustainability capabilities.

Examples:

- Solar Ready
- Rainwater Harvesting
- EV Charging
- Green Building Certification

Performance metrics remain outside this domain.

---

## Compliance Metadata

Maintain infrastructure compliance classifications.

Examples:

- Fire Compliance
- Structural Certification
- Accessibility Certification

Operational audits belong elsewhere.

---

## Health Classification

Maintain canonical infrastructure health.

Examples:

- Excellent
- Good
- Fair
- Poor

Maintenance determines health changes.

Infrastructure stores current status.

---

## Criticality

Maintain business continuity classifications.

Examples:

- Mission Critical
- High Priority
- Standard
- Low Priority

Emergency response remains outside this domain.

---

## Lifecycle Management

Manage lifecycle transitions.

Examples:

- Planned
- Operational
- Renovation
- Decommissioned
- Archived

---

## Event Publication

Publish canonical business events for downstream consumers.

---

## Public Data Contracts

Provide stable integration contracts without exposing internal implementation.

---

# 14. Ownership Boundaries

## Campus & Infrastructure Owns

- Physical entities
- Infrastructure hierarchy
- Physical relationships
- Infrastructure metadata
- Infrastructure lifecycle
- Capacity facts
- Availability facts
- Infrastructure events

---

## Campus & Infrastructure References

The domain references but does not own:

- Institution
- Organisation
- Geographic Location
- Regulatory Authorities

---

## Campus & Infrastructure Is Referenced By

This domain is consumed by:

- Admissions
- Student Management
- Timetable
- Examination
- Hostel
- Library
- Transport
- Maintenance
- Security
- Asset Management
- Procurement
- Finance
- HR
- Analytics
- AI
- Disaster Management

---

# 15. Ownership Matrix

| Business Capability | Owner Domain |
|---------------------|--------------|
| Campus | L2-B03 |
| Building | L2-B03 |
| Room | L2-B03 |
| Infrastructure Hierarchy | L2-B03 |
| Capacity Facts | L2-B03 |
| Accessibility Facts | L2-B03 |
| Sustainability Capabilities | L2-B03 |
| Infrastructure Health | L2-B03 |
| Infrastructure Lifecycle | L2-B03 |
| Infrastructure Criticality | L2-B03 |
| Timetable Allocation | Timetable |
| Student Allocation | Admissions |
| Examination Allocation | Examination |
| Maintenance Work Orders | Maintenance |
| Asset Inventory | Asset Management |
| Security Incidents | Security |
| KPIs | Analytics |
| AI Interpretation | AI Platform |
| Disaster Response | Disaster Management |

---

# 16. Domain Interaction Principles

The following architectural principles govern interactions.

---

## DIP-01

Infrastructure owns facts.

Consumers own business decisions.

---

## DIP-02

Infrastructure publishes events.

Consumers subscribe independently.

---

## DIP-03

Infrastructure never invokes downstream business workflows.

---

## DIP-04

Infrastructure exposes contracts.

Consumers never access internal persistence directly.

---

## DIP-05

Infrastructure evolves independently without breaking consuming domains.

---

# 17. Strategic Position Within SutraERP

Campus & Infrastructure is one of the foundational enterprise domains.

It establishes the physical context required by nearly every operational capability.

```text
                   Enterprise Foundation

         Institution
               │
        Organisation
               │
    Campus & Infrastructure
               │
─────────────────────────────────────────────
 Academic      Administrative      Enterprise
   Domains         Domains           Services
─────────────────────────────────────────────
 Admissions      HR              Analytics
 Timetable       Finance         AI
 Examination     Assets          Security
 Library         Procurement    Disaster Mgmt
 Hostel          Maintenance    Notifications
 Transport
```

No operational domain may redefine infrastructure concepts.

---

# 18. Architecture Principles Summary

The Campus & Infrastructure domain is governed by the following constitutional principles:

1. Single Source of Truth
2. Canonical Physical Representation
3. Facts Over Interpretation
4. Stable Public Contracts
5. Event-Driven Integration
6. Long-Lived Infrastructure Records
7. Technology Independence
8. Domain Isolation
9. Enterprise Scalability
10. Future Extensibility

These principles govern every architectural decision contained within the remainder of this document.

---

# Part 2 Completion

This section establishes:

- Strategic Domain Vision
- Bounded Context
- Ubiquitous Language
- Domain Responsibilities
- Ownership Boundaries
- Cross-Domain Responsibilities
- Interaction Principles
- Strategic Position within SutraERP

These definitions serve as the constitutional foundation for all subsequent domain models, workflows, aggregates, events, and integration contracts described in the remaining sections of this architecture.

---

# Part 3 — Canonical Domain Model

---

# 19. Domain Model Overview

The Campus & Infrastructure domain models the complete physical infrastructure of an institution using a hierarchical, aggregate-oriented domain model.

The model represents physical reality while remaining independent of operational workflows performed within those physical entities.

Every infrastructure entity belongs to a single enterprise hierarchy and may participate in multiple logical groupings without violating ownership boundaries.

---

# 20. Canonical Domain Model

```text
Institution (L2-B01)
        │
        ▼
Campus
        │
        ▼
Building
        │
        ▼
Wing (Optional)
        │
        ▼
Floor
        │
        ▼
Space
        │
        ▼
Infrastructure Unit (Conceptual)
```

Every Infrastructure Unit may additionally belong to:

- Zones
- Collections
- Portfolios

without affecting its physical hierarchy.

---

# 21. Core Aggregates

The domain consists of the following canonical aggregates.

| Aggregate | Purpose |
|------------|---------|
| Campus | Represents a physical institutional site |
| Building | Represents a physical structure |
| Floor | Represents a building floor |
| Space | Represents any usable physical location |
| Zone | Operational grouping of infrastructure |
| Collection | Logical grouping of infrastructure |
| Portfolio | Strategic governance grouping |

Infrastructure Unit is a conceptual business abstraction representing managed physical infrastructure and is NOT an Aggregate Root. The canonical persisted Aggregate Roots remain: Campus, Building, Floor, Space, Zone, Collection, and Portfolio.

---

# 22. Aggregate Hierarchy

```text
Campus
 │
 ├── Building
 │      │
 │      ├── Wing (Optional)
 │      │      │
 │      │      └── Floor
 │      │
 │      └── Floor
 │
 └── External Infrastructure
```

A building may or may not contain wings.

The hierarchy remains flexible without enforcing unnecessary structural levels.

---

# 23. Campus Aggregate

## Purpose

Represents a geographically identifiable institutional site.

Examples:

- Main Campus
- City Campus
- Medical Campus
- Engineering Campus
- Research Campus

---

### Campus Owns

- Campus Identity
- Campus Metadata
- Campus Status
- Campus Address
- Campus Boundary
- Campus Relationships

---

### Campus Does Not Own

- Buildings
- Employees
- Students
- Departments
- Timetables

Those belong to independent aggregates.

---

### Campus Attributes

Examples include:

- Campus ID
- Campus Code
- Campus Name
- Campus Type
- Operational Status
- Address
- Geolocation
- Time Zone
- Establishment Date

---

# 24. Building Aggregate

## Purpose

Represents a permanent physical structure.

Examples:

- Academic Block
- Administrative Block
- Library
- Auditorium
- Hostel
- Sports Complex

---

### Building Owns

- Building Identity
- Classification
- Physical Characteristics
- Operational Characteristics
- Floors
- Infrastructure Relationships

---

### Building Attributes

Examples include:

- Building Code
- Building Name
- Building Type
- Number of Floors
- Gross Area
- Construction Year
- Operational Status

---

# 25. Floor Aggregate

## Purpose

Represents a physical floor inside a building.

A floor serves as an organizational level for spaces.

---

### Floor Attributes

Examples:

- Floor Number
- Floor Name
- Operational Status
- Accessibility Features

---

### Floor Contains

- Rooms
- Laboratories
- Offices
- Utility Spaces
- Corridors
- Service Areas

---

# 26. Space Aggregate

## Purpose

Represents a usable physical space.

Examples:

- Classroom
- Laboratory
- Office
- Auditorium
- Library Hall
- Hostel Room
- Seminar Hall

Space is the canonical abstraction used throughout SutraERP.

---

### Space Owns

- Identity
- Capacity
- Physical Characteristics
- Accessibility
- Availability
- Lifecycle
- Relationships

---

### Space Attributes

Examples include:

- Space Code
- Space Name
- Space Type
- Capacity
- Area
- Operational Status
- Accessibility Status

---

# 27. Infrastructure Unit

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

# 28. Physical Hierarchy Rules

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

# 29. Logical Groupings

Logical groupings do not alter the physical hierarchy.

They provide alternate organizational views.

Three canonical grouping mechanisms exist.

---

## Zones

Purpose:

Operational management.

Examples:

- Examination Zone
- North Zone
- Sports Zone
- Residential Zone

A Space may belong to multiple Zones.

---

## Collections

Purpose:

Business categorization.

Examples:

- Smart Classrooms
- AI Laboratories
- Accessible Facilities
- Heritage Buildings

Collections classify infrastructure across physical boundaries.

---

## Portfolios

Purpose:

Strategic governance and investment planning.

Examples:

- Green Infrastructure Portfolio
- Research Portfolio
- Capital Investment Portfolio
- Heritage Portfolio

Portfolios are used by executive leadership.

---

# 30. Relationship Model

Infrastructure relationships are classified into three categories.

---

## Physical Relationships

Represent structural containment.

Examples:

- Campus contains Building
- Building contains Floor
- Floor contains Space

---

## Logical Relationships

Represent organizational associations.

Examples:

- Member of Zone
- Member of Collection
- Member of Portfolio

---

## Reference Relationships

Represent external references.

Examples:

- Institution
- Organisation
- Geographic Region
- Municipality

---

# 31. Aggregate Ownership Rules

Each aggregate owns its own lifecycle.

Examples:

| Aggregate | Owns Lifecycle |
|------------|---------------|
| Campus | Yes |
| Building | Yes |
| Floor | Yes |
| Space | Yes |
| Zone | Yes |
| Collection | Yes |
| Portfolio | Yes |

Lifecycle ownership never crosses aggregate boundaries.

---

# 32. Aggregate Consistency Rules

The following consistency rules apply.

---

## ACR-01

A Building cannot exist without a Campus.

---

## ACR-02

A Floor cannot exist without a Building.

---

## ACR-03

A Space cannot exist without a Floor.

---

## ACR-04

Deleting parent infrastructure is prohibited while child infrastructure exists.

Lifecycle transition must occur instead.

---

## ACR-05

Infrastructure identifiers are immutable.

Business names may change.

Identifiers shall not.

---

## ACR-06

Logical group membership shall never modify physical hierarchy.

---

## ACR-07

Physical hierarchy shall remain acyclic.

Recursive containment is prohibited.

---

# 33. Domain Model Principles

The canonical model follows these principles.

## DM-01

Model physical reality.

---

## DM-02

Separate physical hierarchy from logical organization.

---

## DM-03

Infrastructure is long-lived.

---

## DM-04

Ownership follows aggregate boundaries.

---

## DM-05

Relationships are explicit.

---

## DM-06

Infrastructure facts are authoritative.

---

## DM-07

Consumer domains extend infrastructure through references rather than modification.

---

# Part 3 Completion

This section establishes the canonical domain model for Campus & Infrastructure by defining:

- Core Aggregates
- Physical Hierarchy
- Logical Groupings
- Aggregate Relationships
- Ownership Rules
- Consistency Constraints
- Canonical Domain Principles

These concepts form the structural foundation upon which lifecycle management, business workflows, validation rules, events, and integration contracts are built.

---

# Part 4 — Canonical Aggregates, Entities & Value Objects

---

# 34. Aggregate Design Philosophy

The Campus & Infrastructure domain follows **Domain-Driven Design (DDD)**.

Each Aggregate represents a consistency boundary responsible for maintaining business invariants within its scope.

Aggregates own:

- Business Rules
- Lifecycle
- Validation
- Child Entities
- Domain Events

Aggregates do **not** expose internal entities directly to external domains.

External consumers interact through published contracts and canonical events.

---

# 35. Aggregate Overview

| Aggregate | Aggregate Root | Primary Responsibility |
|------------|----------------|------------------------|
| Campus | Campus | Physical institutional site |
| Building | Building | Physical building |
| Floor | Floor | Physical floor within a building |
| Space | Space | Usable physical space |
| Zone | Zone | Operational grouping |
| Collection | Collection | Business grouping |
| Portfolio | Portfolio | Strategic governance grouping |

---

# 36. Campus Aggregate

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

# 37. Building Aggregate

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

# 38. Floor Aggregate

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

# 39. Space Aggregate

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

# 40. Zone Aggregate

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

# 41. Collection Aggregate

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

# 42. Portfolio Aggregate

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

# 43. Shared Value Objects

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

# 44. Entity Relationships

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

# 45. Aggregate Communication

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

# 46. Aggregate Transaction Rules

Each aggregate represents an independent transactional consistency boundary.

Rules:

- Transactions never span multiple aggregates.
- Cross-aggregate updates occur using events.
- Eventual consistency is acceptable across aggregates.
- Business invariants are enforced only within the owning aggregate.

---

# 47. Aggregate Size Guidelines

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

# 48. Aggregate Evolution Principles

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

# Part 4 Completion

This section defines the canonical aggregate model for the Campus & Infrastructure domain, including:

- Aggregate Roots
- Child Entities
- Value Objects
- Aggregate Responsibilities
- Business Invariants
- Aggregate Relationships
- Transaction Boundaries
- Evolution Principles

These aggregate definitions form the foundation for lifecycle management, business workflows, validation rules, event publishing, and integration contracts described in subsequent sections.

---

# Part 5 — Infrastructure Lifecycle & State Management

---

# 49. Lifecycle Philosophy

Infrastructure entities are **long-lived enterprise objects**.

Unlike transactional records, physical infrastructure represents real-world assets that evolve over many years or decades.

Infrastructure shall therefore follow a **controlled business lifecycle** rather than relying on record creation and deletion.

The lifecycle represents the operational existence of infrastructure rather than its persistence within the database.

---

# 50. Lifecycle Objectives

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

# 51. Canonical Lifecycle States

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

# 52. Lifecycle State Definitions

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

# 53. Lifecycle Transition Rules

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

# 54. Invalid Lifecycle Transitions

The following transitions are prohibited.

| Invalid Transition | Reason |
|-------------------|--------|
| Archived → Operational | Archived infrastructure is permanently retired. |
| Proposed → Operational | Planning stages cannot be skipped. |
| Decommissioned → Construction | Requires creation of new infrastructure. |
| Archived → Planned | Historical records cannot be reactivated. |

---

# 55. Aggregate Lifecycle Ownership

Each aggregate independently owns its lifecycle.

| Aggregate | Lifecycle Managed |
|------------|-------------------|
| Campus | Yes |
| Building | Yes |
| Floor | Yes |
| Space | Yes |
| Zone | Yes |
| Collection | Yes |
| Portfolio | Yes |

Child aggregates may never exceed the lifecycle maturity of their parent.

---

# 56. Lifecycle Dependency Rules

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

# 57. Operational Availability

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

# 58. Lifecycle Events

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

# 59. Consumer Behaviour

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

# 60. Lifecycle Audit Requirements

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

# 61. Lifecycle Design Principles

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

# Part 5 Completion

This section defines the canonical lifecycle architecture for Campus & Infrastructure, including:

- Lifecycle philosophy
- Canonical lifecycle states
- Transition model
- Aggregate lifecycle ownership
- Parent-child consistency rules
- Operational availability
- Lifecycle events
- Audit requirements
- Lifecycle design principles

This lifecycle model provides the foundation for all infrastructure state transitions, operational validation, business workflows, and event-driven integrations described in subsequent sections.

---

# Part 6 — Business Rules, Validation Rules & Domain Constraints

---

# 62. Business Rule Philosophy

The Campus & Infrastructure domain protects the integrity of the enterprise physical model by enforcing canonical business rules.

Business rules ensure:

- Physical consistency
- Data integrity
- Operational correctness
- Cross-domain reliability
- Regulatory readiness
- Long-term maintainability

Every Infrastructure Aggregate is responsible for enforcing its own invariants.

No external domain may bypass these rules.

---

# 63. Business Rule Categories

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

# 64. Identity Rules

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

# 65. Structural Rules

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

# 66. Operational Rules

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

# 67. Capacity Rules

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

# 68. Area Rules

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

# 69. Lifecycle Rules

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

# 70. Relationship Rules

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

# 71. Accessibility Rules

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

# 72. Sustainability Rules

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

# 73. Health Rules

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

# 74. Criticality Rules

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

# 75. Availability Rules

Infrastructure Availability is derived from multiple business facts.

Availability depends upon:

- Lifecycle
- Operational Status
- Closure Status
- Maintenance State

Availability shall never be manually overridden without authorization.

---

# 76. Event Rules

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

# 77. Public Contract Rules

Infrastructure exposes stable public contracts.

External domains:

- Shall not access internal entities.
- Shall not depend upon persistence models.
- Shall consume canonical contracts only.

---

# 78. Validation Rules

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

# 79. Cross-Aggregate Validation

Cross-aggregate validation shall be minimized.

Permitted examples include:

- Parent lifecycle verification.
- Parent existence verification.
- Parent operational status verification.

Cross-aggregate business logic is prohibited.

---

# 80. Data Integrity Principles

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

# 81. Rule Enforcement Responsibility

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

# Part 6 Completion

This section defines the constitutional business rules governing the Campus & Infrastructure domain.

It establishes:

- Identity Rules
- Structural Rules
- Operational Rules
- Capacity Rules
- Lifecycle Rules
- Relationship Rules
- Validation Rules
- Event Rules
- Public Contract Rules
- Data Integrity Principles

These rules ensure that every infrastructure entity remains consistent, authoritative, and enterprise-compliant while supporting long-term scalability and interoperability across the SutraERP platform.

---

# Part 7 — Campus & Infrastructure Business Workflows

---

# 82. Workflow Architecture

The Campus & Infrastructure domain manages the complete business lifecycle of institutional physical infrastructure.

Every workflow shall:

- Preserve domain integrity
- Enforce aggregate boundaries
- Publish canonical business events
- Maintain audit history
- Support future extensibility

The workflows defined in this section represent **canonical business workflows**, independent of implementation technology.

---

# 83. Workflow Classification

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

# 84. WF-INF-001 — Create Campus

## Purpose

Register a new institutional campus.

---

## Preconditions

- Institution exists.
- User possesses Infrastructure Administration permission.
- Campus Code is unique.

---

## Primary Flow

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

## Business Rules

- Campus Code must be unique.
- Campus belongs to exactly one Institution.
- Initial Lifecycle = Proposed or Planned.
- Campus is unavailable until Operational.

---

## Published Events

- CampusCreated

---

# 85. WF-INF-002 — Update Campus

## Purpose

Modify campus information.

---

## Editable Information

- Name
- Address
- Contact Information
- Geographical Metadata
- Operational Metadata

Infrastructure Identity remains immutable.

---

## Workflow

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

## Published Events

- CampusUpdated

---

# 86. WF-INF-003 — Register Building

## Purpose

Register a building within an operational campus.

---

## Preconditions

- Campus exists.
- Campus is Operational.
- Building Code is unique.

---

## Workflow

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

## Business Rules

- Building belongs to one Campus.
- Building cannot exist independently.
- Initial Lifecycle follows configured defaults.

---

## Published Events

- BuildingCreated

---

# 87. WF-INF-004 — Register Floor

## Purpose

Create a floor within an existing building.

---

## Workflow

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

## Business Rules

- Floor Number unique within Building.
- Parent Building must exist.
- Parent Building lifecycle validated.

---

## Published Events

- FloorCreated

---

# 88. WF-INF-005 — Register Space

## Purpose

Register a usable physical space.

Examples:

- Classroom
- Laboratory
- Office
- Auditorium
- Library Hall
- Hostel Room

---

## Workflow

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

## Business Rules

- Space belongs to exactly one Floor.
- Capacity must be valid.
- Area must be positive.

---

## Published Events

- SpaceCreated

---

# 89. WF-INF-006 — Update Infrastructure

## Purpose

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

## Workflow

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

## Published Events

- InfrastructureUpdated

---

# 90. WF-INF-007 — Change Infrastructure Classification

## Purpose

Modify the business classification of infrastructure.

Examples:

- Classroom → Smart Classroom
- Laboratory → AI Laboratory
- Office → Administrative Office

---

## Workflow

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

## Published Events

- InfrastructureClassificationChanged

---

# 91. WF-INF-008 — Update Capacity

## Purpose

Maintain canonical infrastructure capacity.

---

## Workflow

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

## Business Rules

- Capacity ≥ 0
- Capacity complies with institutional configuration.

---

## Published Events

- RoomCapacityChanged

---

# 92. WF-INF-009 — Update Accessibility Profile

## Purpose

Maintain accessibility characteristics.

---

## Workflow

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

## Published Events

- SpaceAccessibilityUpdated

---

# 93. WF-INF-010 — Update Sustainability Profile

## Purpose

Maintain sustainability capabilities.

---

## Workflow

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

## Published Events

- SustainabilityProfileUpdated

---

# 94. WF-INF-011 — Change Operational Status

## Purpose

Enable or disable operational use.

Examples:

- Open Building
- Close Building
- Reopen Campus

---

## Workflow

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

## Published Events

- InfrastructureActivated
- InfrastructureDeactivated

---

# 95. WF-INF-012 — Lifecycle Transition

## Purpose

Transition infrastructure through approved lifecycle states.

---

## Workflow

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

## Business Rules

- Transition must be valid.
- Transition requires authorization.
- Audit record mandatory.

---

## Published Events

Examples:

- ConstructionStarted
- InfrastructureOperational
- RenovationStarted
- InfrastructureDecommissioned
- InfrastructureArchived

---

# 96. WF-INF-013 — Decommission Infrastructure

## Purpose

Retire infrastructure permanently.

---

## Preconditions

- No active operational dependencies.
- Approval completed.

---

## Workflow

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

## Business Rules

Infrastructure remains historically available.

Deletion does not occur.

---

# 97. WF-INF-014 — Archive Infrastructure

## Purpose

Move permanently retired infrastructure into historical state.

---

## Workflow

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

# 98. Workflow Events Summary

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

# 99. Workflow Design Principles

## WDP-01

Every workflow modifies exactly one Aggregate Root.

---

## WDP-02

Every successful workflow publishes at least one canonical domain event.

---

## WDP-03

Every workflow is fully auditable.

---

## WDP-04

Business validation precedes persistence.

---

## WDP-05

Cross-domain orchestration is prohibited.

Infrastructure publishes events.

Consumer domains determine their own reactions.

---

## WDP-06

Infrastructure workflows manage physical entities only.

Operational activities (Admissions, Timetable, Examination, Maintenance, Security, etc.) belong to their respective domains.

---

# Part 7 Completion

This section defines the canonical operational workflows of the Campus & Infrastructure domain, including:

- Campus Management
- Building Management
- Floor Management
- Space Management
- Infrastructure Updates
- Capacity Management
- Accessibility Management
- Sustainability Management
- Lifecycle Management
- Decommissioning
- Archival

These workflows represent the authoritative business processes for managing institutional physical infrastructure while maintaining clear aggregate ownership and event-driven integration with the rest of the SutraERP platform.

---

# Part 8 — Campus Management Workflows

---

# 100. Campus Domain Overview

A Campus represents the highest-level physical location within an Institution.

A Campus provides the geographical boundary within which all physical infrastructure exists.

Examples include:

- Main Campus
- Medical Campus
- Engineering Campus
- City Campus
- Research Campus
- Satellite Campus

A Campus does **not** own academic, administrative, or financial operations.

It owns only the physical representation of the campus.

---

# 101. Campus Lifecycle

Every Campus progresses through the canonical lifecycle.

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

# 102. Campus Registration Workflow

## Purpose

Register a new institutional campus.

---

## Actors

- Infrastructure Administrator
- Institution Administrator

---

## Preconditions

- Institution exists.
- User has Infrastructure Administration permissions.
- Campus Code is unique.

---

## Workflow

```text
Administrator
      │
      ▼
Create Campus
      │
      ▼
Enter Campus Information
      │
      ▼
Validate Business Rules
      │
      ▼
Generate Campus Identifier
      │
      ▼
Persist Campus Aggregate
      │
      ▼
Publish CampusCreated
```

---

## Business Rules

- Campus belongs to exactly one Institution.
- Campus Code is immutable.
- Campus Name is configurable.
- Campus initially remains non-operational.

---

## Published Events

- CampusCreated

---

# 103. Campus Activation Workflow

## Purpose

Make a campus available for operational use.

---

## Preconditions

- Campus exists.
- Lifecycle permits activation.
- Mandatory metadata completed.

---

## Workflow

```text
Campus
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
Publish CampusActivated
```

---

## Published Events

- CampusActivated

---

# 104. Campus Modification Workflow

Campus metadata may change during its lifetime.

Editable information includes:

- Campus Name
- Address
- Contact Details
- Geo Coordinates
- Campus Description
- Working Hours
- Time Zone

The following are immutable:

- Campus Identifier
- Institution Association

---

## Workflow

```text
Administrator
      │
      ▼
Modify Campus
      │
      ▼
Validate Changes
      │
      ▼
Persist
      │
      ▼
CampusUpdated
```

---

## Published Events

- CampusUpdated

---

# 105. Campus Boundary Management

Campus boundaries define the physical extent of institutional property.

Examples include:

- Land Parcel
- Survey Boundary
- GIS Polygon
- Administrative Boundary

---

## Workflow

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

## Published Events

- CampusBoundaryUpdated

---

# 106. Campus Operational Status Workflow

Operational Status represents whether a campus is available for institutional operations.

Possible statuses include:

- Operational
- Temporarily Closed
- Restricted Access
- Permanently Closed

---

## Workflow

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

## Published Events

- CampusOperationalStatusChanged

---

# 107. Campus Capacity Profile Workflow

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

## Workflow

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

## Published Events

- CampusCapacityUpdated

---

# 108. Campus Accessibility Workflow

Maintain accessibility capabilities at campus level.

Examples:

- Accessible Entrances
- Accessible Parking
- Campus Shuttle
- Accessible Navigation
- Emergency Evacuation Routes

---

## Workflow

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

## Published Events

- CampusAccessibilityUpdated

---

# 109. Campus Sustainability Workflow

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

## Workflow

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

## Published Events

- CampusSustainabilityUpdated

---

# 110. Campus Criticality Workflow

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

## Workflow

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

## Published Events

- CampusCriticalityChanged

---

# 111. Campus Decommission Workflow

Retire a Campus permanently.

---

## Preconditions

- No operational Buildings remain.
- Dependent infrastructure has been transitioned.
- Required approvals completed.

---

## Workflow

```text
Campus
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
CampusDecommissioned
```

---

## Business Rules

- Campus remains historically available.
- Historical references remain valid.
- Physical deletion is prohibited.

---

## Published Events

- CampusDecommissioned

---

# 112. Campus Archive Workflow

Archive a permanently retired Campus.

---

## Workflow

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
CampusArchived
```

---

## Published Events

- CampusArchived

---

# 113. Cross-Domain Consumers

| Consumer Domain | Campus Information Used |
|-----------------|-------------------------|
| Admissions | Campus availability |
| Timetable | Operational campuses |
| Examination | Examination centres |
| HR | Employee workplace |
| Hostel | Campus association |
| Library | Branch location |
| Transport | Pickup and drop points |
| Asset Management | Asset location |
| Security | Campus security zones |
| Analytics | Campus master data |
| AI | Infrastructure facts |
| Disaster Management | Recovery planning |

---

# 114. Business Rules

## CM-01

A Campus belongs to exactly one Institution.

---

## CM-02

Campus Code is immutable.

---

## CM-03

Only Operational campuses may host operational infrastructure.

---

## CM-04

Campus deletion is prohibited after operational activation.

---

## CM-05

Campus lifecycle governs the maximum lifecycle of all child infrastructure.

---

## CM-06

Campus events shall be published for all significant business changes.

---

# 115. Campus Events Summary

| Workflow | Event |
|----------|-------|
| Register Campus | CampusCreated |
| Update Campus | CampusUpdated |
| Activate Campus | CampusActivated |
| Change Status | CampusOperationalStatusChanged |
| Update Boundary | CampusBoundaryUpdated |
| Update Capacity | CampusCapacityUpdated |
| Update Accessibility | CampusAccessibilityUpdated |
| Update Sustainability | CampusSustainabilityUpdated |
| Update Criticality | CampusCriticalityChanged |
| Decommission | CampusDecommissioned |
| Archive | CampusArchived |

---

# 116. Workflow Design Principles

## CWP-01

Campus owns only physical infrastructure information.

---

## CWP-02

Campus never owns operational business processes.

---

## CWP-03

Campus publishes facts.

Consumers implement business workflows.

---

## CWP-04

Every workflow shall be auditable.

---

## CWP-05

Every successful workflow shall publish canonical business events.

---

# Part 8 Completion

This section defines the complete canonical Campus Management workflows for the Campus aggregate, including:

- Campus Registration
- Campus Activation
- Campus Updates
- Boundary Management
- Operational Status
- Capacity Management
- Accessibility Management
- Sustainability Management
- Criticality Management
- Decommissioning
- Archival

These workflows establish the authoritative business processes governing the lifecycle and management of institutional campuses within the SutraERP platform.

---

# Part 9 — Building Management Workflows

---

# 117. Building Domain Overview

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

# 118. Building Lifecycle

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

# 119. Building Registration Workflow

## Purpose

Register a new Building within an Operational Campus.

---

## Actors

- Infrastructure Administrator
- Campus Administrator

---

## Preconditions

- Parent Campus exists.
- Parent Campus is Operational.
- Building Code is unique within Campus.

---

## Workflow

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

## Business Rules

- Building belongs to one Campus.
- Building Code is immutable.
- Initial Lifecycle follows configuration.
- Parent Campus must exist.

---

## Published Events

- BuildingCreated

---

# 120. Building Activation Workflow

## Purpose

Make a Building operational.

---

## Preconditions

- Parent Campus is Operational.
- Building lifecycle permits activation.
- Mandatory metadata completed.

---

## Workflow

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

## Published Events

- BuildingActivated

---

# 121. Building Metadata Workflow

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

## Workflow

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

## Published Events

- BuildingUpdated

---

# 122. Building Structure Workflow

Manage structural characteristics.

Examples include:

- Number of Floors
- Wings
- Entrances
- Emergency Exits
- Lift Availability
- Staircases

---

## Workflow

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

## Published Events

- BuildingStructureUpdated

---

# 123. Building Classification Workflow

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

## Workflow

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

## Published Events

- BuildingClassificationChanged

---

# 124. Building Capacity Workflow

Maintain canonical building capacity.

Examples:

- Maximum Occupancy
- Total Seating Capacity
- Parking Capacity
- Emergency Capacity

Infrastructure stores facts only.

Derived utilization belongs to Analytics.

---

## Workflow

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

## Published Events

- BuildingCapacityUpdated

---

# 125. Building Accessibility Workflow

Maintain accessibility capabilities.

Examples:

- Wheelchair Accessible
- Lift Available
- Accessible Toilets
- Braille Signage
- Accessible Entrance
- Accessible Parking

---

## Workflow

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

## Published Events

- BuildingAccessibilityUpdated

---

# 126. Building Sustainability Workflow

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

## Workflow

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

## Published Events

- BuildingSustainabilityUpdated

---

# 127. Building Health Workflow

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

## Workflow

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

## Published Events

- BuildingHealthUpdated

---

# 128. Building Criticality Workflow

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

## Workflow

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

## Published Events

- BuildingCriticalityChanged

---

# 129. Building Operational Status Workflow

Manage operational availability.

Possible statuses:

- Operational
- Restricted
- Temporarily Closed
- Emergency Closure

Operational Status works together with Lifecycle to determine availability.

---

## Workflow

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

## Published Events

- BuildingOperationalStatusChanged

---

# 130. Building Decommission Workflow

Retire a Building permanently.

---

## Preconditions

- Child Floors transitioned.
- Child Spaces transitioned.
- Required approvals completed.

---

## Workflow

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

## Business Rules

- Historical references remain valid.
- Physical deletion prohibited.
- Building becomes unavailable.

---

## Published Events

- BuildingDecommissioned

---

# 131. Building Archive Workflow

Archive permanently retired Buildings.

---

## Workflow

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

## Published Events

- BuildingArchived

---

# 132. Cross-Domain Consumers

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

# 133. Business Rules

## BM-01

A Building belongs to exactly one Campus.

---

## BM-02

Building Code is unique within Campus.

---

## BM-03

Building Identifier is immutable.

---

## BM-04

Only Operational Buildings may contain Operational Floors.

---

## BM-05

Building deletion is prohibited after operational activation.

---

## BM-06

Parent Campus lifecycle limits Building lifecycle.

---

## BM-07

Every significant Building change publishes a canonical business event.

---

# 134. Building Events Summary

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

# 135. Workflow Design Principles

## BWP-01

Building owns only Building-related business rules.

---

## BWP-02

Building does not manage activities performed inside the Building.

---

## BWP-03

Building publishes canonical facts.

Consumers determine business behaviour.

---

## BWP-04

Every Building workflow shall be fully auditable.

---

## BWP-05

Every successful Building workflow shall publish canonical business events.

---

## BWP-06

Building remains independent of Timetable, Admissions, Examination, Maintenance, Asset Management, and Analytics implementations.

---

# Part 9 Completion

This section defines the complete canonical Building Management workflows for the Building aggregate, including:

- Building Registration
- Building Activation
- Metadata Management
- Structural Management
- Classification Management
- Capacity Management
- Accessibility Management
- Sustainability Management
- Health Management
- Criticality Management
- Operational Status Management
- Decommissioning
- Archival

These workflows establish the authoritative business processes governing the lifecycle and management of Buildings within the Campus & Infrastructure domain.

---

# Part 10 — Floor & Space Management Workflows

---

# 136. Domain Overview

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

# 137. Aggregate Responsibilities

## Floor Aggregate

Owns:

- Floor Identity
- Floor Ordering
- Floor Metadata
- Floor Accessibility
- Floor Operational Status
- Parent Building Relationship

---

## Space Aggregate

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

# 138. Floor Registration Workflow

## Purpose

Register a Floor within a Building.

---

## Actors

- Infrastructure Administrator

---

## Preconditions

- Parent Building exists.
- Parent Building is Operational.
- Floor Number is unique.

---

## Workflow

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

## Business Rules

- Floor belongs to exactly one Building.
- Floor Number is immutable after operational use.
- Parent lifecycle validated.

---

## Published Events

- FloorCreated

---

# 139. Floor Update Workflow

Editable information includes:

- Floor Name
- Description
- Accessibility Metadata
- Operational Metadata

Immutable:

- Identifier
- Parent Building

---

## Workflow

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

## Published Events

- FloorUpdated

---

# 140. Space Registration Workflow

## Purpose

Register a usable Space.

---

## Preconditions

- Parent Floor exists.
- Parent Floor is Operational.
- Space Code is unique within Building.

---

## Workflow

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

## Mandatory Information

- Space Name
- Space Code
- Space Type
- Capacity
- Area
- Parent Floor

---

## Published Events

- SpaceCreated

---

# 141. Space Classification Workflow

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

## Workflow

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

## Published Events

- SpaceClassificationChanged

---

# 142. Space Capacity Workflow

Maintain canonical capacity information.

Capacity includes:

- Seating Capacity
- Standing Capacity
- Maximum Occupancy
- Operational Capacity

Infrastructure stores facts only.

Utilization belongs to Analytics.

---

## Workflow

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

## Business Rules

- Capacity cannot be negative.
- Operational Capacity ≤ Maximum Occupancy.

---

## Published Events

- RoomCapacityChanged

---

# 143. Space Area Workflow

Maintain physical measurements.

Examples:

- Carpet Area
- Built-up Area
- Volume (optional future extension)

Measurement units follow enterprise configuration.

---

## Workflow

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

## Published Events

- SpaceAreaUpdated

---

# 144. Space Accessibility Workflow

Maintain accessibility capabilities.

Examples:

- Wheelchair Accessible
- Braille Signage
- Hearing Assistance
- Lift Accessible
- Adjustable Furniture
- Accessible Restroom Nearby

---

## Workflow

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

## Published Events

- SpaceAccessibilityUpdated

---

# 145. Space Facility Workflow

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

## Workflow

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

## Published Events

- SpaceFacilitiesUpdated

---

# 146. Space Sustainability Workflow

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

## Workflow

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

## Published Events

- SustainabilityProfileUpdated

---

# 147. Space Health Workflow

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

## Workflow

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

## Published Events

- SpaceHealthUpdated

---

# 148. Space Criticality Workflow

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

## Workflow

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

## Published Events

- SpaceCriticalityChanged

---

# 149. Space Operational Status Workflow

Operational Status determines whether a Space is currently available.

Possible statuses include:

- Available
- Restricted
- Reserved
- Closed
- Emergency Closure

Operational Status complements—but does not replace—the canonical Lifecycle.

---

## Workflow

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

## Published Events

- SpaceOperationalStatusChanged

---

# 150. Space Lifecycle Workflow

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

# 151. Space Decommission Workflow

Retire a Space permanently.

---

## Preconditions

- No active operational allocations.
- Required approvals completed.

---

## Workflow

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

## Published Events

- SpaceDecommissioned

---

# 152. Space Archive Workflow

Archive permanently retired Spaces.

---

## Workflow

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

## Published Events

- SpaceArchived

---

# 153. Cross-Domain Consumers

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

# 154. Business Rules

## FSM-01

Every Floor belongs to exactly one Building.

---

## FSM-02

Every Space belongs to exactly one Floor.

---

## FSM-03

Space Code is unique within the parent Building.

---

## FSM-04

Only Operational Spaces may be allocated by consumer domains.

---

## FSM-05

Facilities describe capabilities, not owned assets.

---

## FSM-06

Operational Status and Lifecycle together determine availability.

---

## FSM-07

Every significant change publishes a canonical domain event.

---

# 155. Floor & Space Events Summary

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

# 156. Workflow Design Principles

## FSWP-01

Floor manages only floor-level metadata.

---

## FSWP-02

Space is the primary operational aggregate consumed by enterprise workflows.

---

## FSWP-03

Infrastructure publishes canonical physical facts.

Operational domains determine business usage.

---

## FSWP-04

Facilities represent infrastructure capabilities.

Physical assets remain under Asset Management ownership.

---

## FSWP-05

Every successful workflow publishes immutable domain events.

---

## FSWP-06

Floor and Space remain independent of scheduling, examinations, admissions, maintenance execution, and analytics logic.

---

# Part 10 Completion

This section defines the canonical workflows for the **Floor** and **Space** aggregates, including:

- Floor Registration and Management
- Space Registration
- Classification Management
- Capacity Management
- Area Management
- Accessibility Management
- Facility Management
- Sustainability Management
- Health Management
- Criticality Management
- Operational Status Management
- Lifecycle Management
- Decommissioning and Archival

These workflows establish the authoritative operational model for the lowest levels of the Campus & Infrastructure hierarchy and provide the canonical infrastructure entities consumed by scheduling, examination, admissions, maintenance, security, and other enterprise domains.

---

# Part 11 — Zones, Collections & Portfolio Management Workflows

---

# 157. Domain Overview

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

# 158. Logical Organization Principles

Logical organization exists to support:

- Operational management
- Administrative grouping
- Executive governance
- Strategic investment
- Reporting
- Search and discovery

Logical organization shall never redefine ownership.

---

# 159. Zone Overview

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

# 160. Zone Lifecycle

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

# 161. Zone Registration Workflow

## Purpose

Create a new operational Zone.

---

## Actors

- Infrastructure Administrator

---

## Preconditions

- Zone Name unique.
- Zone Type configured.

---

## Workflow

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

## Published Events

- ZoneCreated

---

# 162. Zone Membership Workflow

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

## Workflow

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

## Business Rules

- Multiple memberships permitted.
- Membership does not alter hierarchy.
- Membership is independently auditable.

---

## Published Events

- AssignedToZone

---

# 163. Zone Removal Workflow

Remove infrastructure from a Zone.

---

## Workflow

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

## Published Events

- RemovedFromZone

---

# 164. Collection Overview

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

# 165. Collection Registration Workflow

## Purpose

Create a Collection.

---

## Workflow

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

## Published Events

- CollectionCreated

---

# 166. Collection Membership Workflow

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

## Workflow

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

## Business Rules

- Infrastructure may belong to multiple Collections.
- Collections never own infrastructure.
- Membership remains independent.

---

## Published Events

- AssignedToCollection

---

# 167. Collection Removal Workflow

Remove Collection membership.

---

## Workflow

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

## Published Events

- RemovedFromCollection

---

# 168. Portfolio Overview

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

# 169. Portfolio Registration Workflow

## Purpose

Create a strategic Portfolio.

---

## Workflow

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

## Published Events

- PortfolioCreated

---

# 170. Portfolio Membership Workflow

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

## Workflow

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

## Business Rules

- Multiple memberships permitted.
- Membership never changes lifecycle.
- Membership never changes ownership.
- Membership supports governance only.

---

## Published Events

- AssignedToPortfolio

---

# 171. Portfolio Removal Workflow

Remove Portfolio membership.

---

## Workflow

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

## Published Events

- RemovedFromPortfolio

---

# 172. Logical Relationship Model

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

# 173. Cross-Domain Consumers

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

# 174. Business Rules

## ZCP-01

Logical structures shall never alter physical hierarchy.

---

## ZCP-02

Infrastructure may belong to multiple Zones.

---

## ZCP-03

Infrastructure may belong to multiple Collections.

---

## ZCP-04

Infrastructure may belong to multiple Portfolios.

---

## ZCP-05

Membership is many-to-many.

---

## ZCP-06

Logical structures never own infrastructure.

---

## ZCP-07

Removing membership never deletes infrastructure.

---

## ZCP-08

Logical structures remain independent from lifecycle.

---

# 175. Domain Events Summary

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

# 176. Workflow Design Principles

## ZCPWP-01

Zones support operational organization.

---

## ZCPWP-02

Collections support logical categorization.

---

## ZCPWP-03

Portfolios support executive governance and investment planning.

---

## ZCPWP-04

Logical structures never replace the physical hierarchy.

---

## ZCPWP-05

Infrastructure ownership always remains with the physical aggregates.

---

## ZCPWP-06

Every membership change publishes a canonical domain event.

---

## ZCPWP-07

Logical organization remains fully configurable to support diverse institutional structures without requiring architectural changes.

---

# Part 11 Completion

This section defines the canonical workflows for **Zones**, **Collections**, and **Portfolios**, including:

- Zone Management
- Zone Membership
- Collection Management
- Collection Membership
- Portfolio Management
- Portfolio Membership
- Logical Relationship Model
- Cross-Domain Consumers
- Business Rules
- Domain Events
- Workflow Design Principles

These workflows establish the enterprise-standard logical organization model for Campus & Infrastructure, enabling operational grouping, business categorization, and strategic governance while preserving the integrity of the canonical physical hierarchy.

---

# Part 12 — Domain Events, Integration Contracts & External Interfaces

---

# 177. Integration Philosophy

The Campus & Infrastructure domain is an **authoritative master data domain**.

It owns canonical facts regarding institutional physical infrastructure.

The domain shall integrate with the remainder of SutraERP using:

- Canonical Domain Events
- Public Read Contracts
- Query APIs
- Integration Services

External systems shall **never depend upon internal persistence models**.

---

# 178. Integration Principles

The domain follows these integration principles.

## IP-01

Infrastructure publishes facts.

Consumers determine business behaviour.

---

## IP-02

Infrastructure never invokes downstream business processes.

---

## IP-03

Integration shall be event-driven wherever possible.

---

## IP-04

Public contracts remain stable across implementation changes.

---

## IP-05

Internal aggregate models remain private.

---

## IP-06

Infrastructure is the single source of truth for physical infrastructure.

---

# 179. Event Architecture

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

# 180. Event Categories

Infrastructure publishes four categories of events.

| Category | Purpose |
|-----------|---------|
| Lifecycle Events | State transitions |
| Structural Events | Physical hierarchy changes |
| Classification Events | Metadata updates |
| Relationship Events | Logical organization changes |

---

# 181. Lifecycle Events

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

# 182. Structural Events

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

# 183. Classification Events

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

# 184. Relationship Events

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

# 185. Event Structure

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

# 186. Event Payload Principles

Event payloads shall contain only canonical business facts.

Payloads shall not contain:

- UI information
- Calculated KPIs
- Analytics
- Temporary processing data
- Consumer-specific projections

Events remain implementation-independent.

---

# 187. Event Versioning

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

# 188. Event Consumers

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

# 189. Public Read Contracts

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

# 190. Campus Summary Contract

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

# 191. Building Summary Contract

Fields include:

- Building Identifier
- Building Code
- Building Name
- Campus Identifier
- Building Type
- Lifecycle
- Operational Status

---

# 192. Floor Summary Contract

Fields include:

- Floor Identifier
- Floor Number
- Floor Name
- Parent Building
- Operational Status

---

# 193. Space Summary Contract

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

# 194. Capacity Profile Contract

Contains canonical capacity facts.

Examples:

- Seating Capacity
- Standing Capacity
- Maximum Occupancy
- Operational Capacity

Analytics derives utilization independently.

---

# 195. Accessibility Profile Contract

Contains accessibility capabilities.

Examples:

- Wheelchair Access
- Lift Access
- Braille Signage
- Hearing Assistance

Infrastructure exposes facts only.

Compliance interpretation belongs elsewhere.

---

# 196. Sustainability Profile Contract

Contains sustainability capabilities.

Examples:

- Solar Installed
- Rainwater Harvesting
- LED Lighting
- EV Charging

Infrastructure does not expose sustainability KPIs.

---

# 197. Query Services

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

# 198. Consumer Responsibilities

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

# 199. Integration Constraints

External consumers shall **not**:

- Modify Infrastructure aggregates directly.
- Depend on database schemas.
- Assume event ordering across aggregates.
- Infer undocumented business meaning.

Consumers interact only through supported contracts and events.

---

# 200. Integration Design Principles

## ICP-01

Infrastructure publishes immutable business facts.

---

## ICP-02

Events describe completed business actions.

---

## ICP-03

Public contracts shield consumers from implementation changes.

---

## ICP-04

Consumers own business reactions.

---

## ICP-05

Infrastructure never embeds consumer-specific behaviour.

---

## ICP-06

Integration remains asynchronous wherever practical.

---

## ICP-07

Infrastructure remains the canonical master for physical infrastructure data.

---

# Part 12 Completion

This section defines the enterprise integration architecture for the Campus & Infrastructure domain, including:

- Integration Philosophy
- Event Architecture
- Canonical Domain Events
- Event Versioning
- Consumer Domains
- Public Read Contracts
- Query Services
- Consumer Responsibilities
- Integration Constraints
- Integration Design Principles

These integration mechanisms establish the Campus & Infrastructure domain as the authoritative source of physical infrastructure master data while enabling scalable, event-driven interoperability across all SutraERP domains.

---

# Part 13 — Security, Authorization, Audit & Governance

---

# 201. Security Philosophy

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

# 202. Security Architecture Principles

The domain follows the enterprise security architecture established by the Platform Foundation.

Infrastructure does **not** implement authentication.

Authentication is delegated to:

- Identity Domain
- Authorization Domain
- Platform Foundation

The Infrastructure domain consumes authenticated user context.

---

# 203. Authorization Model

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

# 204. Infrastructure Roles

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

# 205. Permission Categories

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

# 206. Resource Scoping

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

# 207. Authorization Workflow

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

# 208. Audit Philosophy

Every significant business action shall be auditable.

Audit records provide:

- Accountability
- Regulatory compliance
- Operational traceability
- Historical reconstruction
- Governance evidence

Audit history is immutable.

---

# 209. Auditable Operations

The following operations shall generate audit records.

## Infrastructure Creation

Examples:

- Campus Created
- Building Created
- Floor Created
- Space Created

---

## Infrastructure Modification

Examples:

- Name Changed
- Capacity Updated
- Accessibility Updated
- Sustainability Updated
- Classification Updated

---

## Lifecycle Operations

Examples:

- Activated
- Decommissioned
- Archived
- Reopened
- Temporarily Closed

---

## Relationship Operations

Examples:

- Assigned to Zone
- Removed from Zone
- Assigned to Collection
- Assigned to Portfolio

---

## Administrative Operations

Examples:

- Configuration Changes
- Governance Overrides
- Bulk Updates

---

# 210. Audit Record Structure

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

# 211. Governance Overrides

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

# 212. Data Retention

Infrastructure master data is retained according to institutional retention policies.

General principles include:

- Operational infrastructure is never physically deleted.
- Archived infrastructure remains queryable for authorized users.
- Audit history is retained independently of operational status.
- Historical references remain valid across the platform.

Retention periods are configured by institutional governance policies.

---

# 213. Privacy & Sensitive Data

The Campus & Infrastructure domain stores infrastructure metadata only.

It does **not** own or manage:

- Student personal information
- Employee personal information
- Academic records
- Financial records
- Medical records

Infrastructure references individuals only where operationally required (e.g., "Building Administrator"), with identity details resolved through the Identity domain.

---

# 214. Compliance Considerations

The architecture supports institutional compliance obligations, including:

- Accreditation audits
- Government inspections
- Fire and safety audits
- Accessibility reviews
- Infrastructure governance reviews
- Internal audit requirements

The domain provides authoritative evidence but does not interpret regulatory requirements.

---

# 215. Data Integrity Controls

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

# 216. Multi-Tenant Governance

Infrastructure data is isolated by Tenant.

Governance principles include:

- Tenant data isolation
- Institution-level ownership
- No cross-tenant visibility
- Tenant-specific configuration
- Independent audit history

Platform Foundation enforces isolation boundaries.

---

# 217. Error Handling Principles

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

# 218. Governance Principles

## GP-01

Authentication is external to the Infrastructure domain.

---

## GP-02

Authorization precedes business validation.

---

## GP-03

Every successful state change is auditable.

---

## GP-04

Operational infrastructure is preserved for historical integrity.

---

## GP-05

Security policies remain independent of business rules.

---

## GP-06

Governance approvals supplement business validation; they do not replace it.

---

## GP-07

Audit records are immutable and independently retained.

---

## GP-08

Infrastructure remains the authoritative source for physical infrastructure master data.

---

# 219. Responsibility Matrix

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

# Part 13 Completion

This section defines the enterprise security and governance architecture for the Campus & Infrastructure domain, including:

- Security Philosophy
- Authorization Model
- Role & Permission Concepts
- Resource Scoping
- Audit Architecture
- Governance Controls
- Data Retention
- Privacy Boundaries
- Compliance Support
- Multi-Tenant Governance
- Error Handling
- Responsibility Matrix

These principles ensure that Campus & Infrastructure master data remains secure, auditable, governable, and compliant while delegating authentication, authorization, workflow approval, and platform-level enforcement to their respective enterprise domains.

---

# Part 14 — Data Model, Persistence Strategy & Repository Architecture

---

# 220. Persistence Philosophy

The Campus & Infrastructure domain owns the **canonical business model**, not the physical database implementation.

Persistence exists solely to preserve authoritative business facts.

The persistence layer shall never dictate:

- Business Rules
- Aggregate Design
- Workflow Behaviour
- Domain Relationships

The Domain Model remains the source of truth.

---

# 221. Persistence Architecture Principles

The Infrastructure domain follows the following principles.

## PAP-01

Domain Model is technology independent.

---

## PAP-02

Persistence models remain internal.

---

## PAP-03

Consumers never access Infrastructure database tables.

---

## PAP-04

Repositories persist Aggregate Roots only.

---

## PAP-05

Persistence must preserve Aggregate consistency.

---

## PAP-06

Read Contracts are independent of storage implementation.

---

# 222. Aggregate Persistence Strategy

Each Aggregate Root owns its persistence boundary.

| Aggregate | Repository |
|------------|------------|
| Campus | Campus Repository |
| Building | Building Repository |
| Floor | Floor Repository |
| Space | Space Repository |
| Zone | Zone Repository |
| Collection | Collection Repository |
| Portfolio | Portfolio Repository |

Repositories never expose internal child entities directly.

---

# 223. Repository Responsibilities

Repositories are responsible for:

- Aggregate Persistence
- Aggregate Retrieval
- Identity Resolution
- Optimistic Concurrency
- Transaction Boundary Management

Repositories are **not responsible** for:

- Business Rules
- Authorization
- Workflow Orchestration
- Validation Logic
- Reporting

---

# 224. Canonical Persistence Model

```text
Campus
   │
   ├──────── Building
   │             │
   │             ├──────── Floor
   │             │              │
   │             │              └──────── Space
   │
   ├──────── Zone Membership
   │
   ├──────── Collection Membership
   │
   └──────── Portfolio Membership
```

Logical relationships are stored independently of the physical hierarchy.

---

# 225. Identity Model

Every Infrastructure Aggregate contains:

```text
Identifier (UUID)

Business Code

Display Name
```

Characteristics:

- Identifier is immutable.
- Business Code is immutable after operational activation.
- Display Name is editable.

Identifiers are used for internal references.

Business Codes support institutional operations.

---

# 226. Infrastructure Keys

Recommended key hierarchy:

| Key | Purpose |
|------|---------|
| UUID | Internal identity |
| Business Code | Institutional reference |
| External Reference | Third-party integration (optional) |

Only UUID is guaranteed globally unique.

---

# 227. Versioning Strategy

Every Aggregate shall support optimistic concurrency.

Each Aggregate contains:

```text
Version Number
```

Example:

```text
Building

Version = 15
```

Updates require matching version values.

Version mismatch results in concurrency conflict.

---

# 228. Soft Delete Strategy

Infrastructure uses lifecycle management rather than soft deletion.

Instead of:

```text
Deleted = True
```

Infrastructure transitions through:

```text
Operational

↓

Decommissioned

↓

Archived
```

Soft-delete flags are discouraged for Aggregate state management.

---

# 229. Physical Deletion Policy

Physical deletion is exceptional.

Permitted only when:

- Record created in error.
- No downstream references exist.
- Governance approval granted.

Deletion shall generate an audit record.

Operational infrastructure shall never be physically deleted.

---

# 230. Relationship Persistence

Physical hierarchy relationships are mandatory.

```text
Campus

↓

Building

↓

Floor

↓

Space
```

Logical relationships are independent.

Examples:

```text
Space

↓

Zone Membership

↓

Collection Membership

↓

Portfolio Membership
```

Logical relationships use reference mappings.

---

# 231. Value Object Persistence

Value Objects persist with the owning Aggregate.

Examples include:

Capacity

```text
Seating

Standing

Maximum Occupancy
```

Accessibility

```text
Wheelchair

Lift

Braille

Hearing Assistance
```

Criticality

```text
Priority

Downtime

Recovery Level
```

Value Objects have no independent lifecycle.

---

# 232. Repository Interface Principles

Repositories expose aggregate-oriented operations.

Examples:

```text
Save Aggregate

Load Aggregate

Find by Identifier

Find by Business Code

Exists
```

Repositories shall not expose generic CRUD interfaces.

---

# 233. Query Model

Operational queries use dedicated read models.

Examples:

- Campus Summary
- Building Summary
- Space Summary
- Capacity View
- Accessibility View

Queries may use optimized read storage.

Read models are disposable projections. They may be regenerated from authoritative Aggregate data and Domain Events, and they are never the system of record.

---

# 234. Search Strategy

Infrastructure supports enterprise search.

Search criteria include:

- Campus
- Building
- Floor
- Space
- Classification
- Capacity
- Accessibility
- Lifecycle
- Zone
- Collection
- Portfolio

Search operates against published query models.

---

# 235. Indexing Principles

Typical indexes include:

Identity

- Identifier
- Business Code

Hierarchy

- Campus
- Building
- Floor

Operational

- Lifecycle
- Operational Status

Classification

- Infrastructure Type
- Accessibility
- Criticality

Indexes are implementation decisions and may vary by persistence technology.

---

# 236. Transaction Strategy

Each Aggregate modification executes within a single transaction.

```text
Load Aggregate

↓

Validate

↓

Modify

↓

Persist

↓

Commit

↓

Publish Event
```

Transactions never span multiple Aggregates.

Cross-aggregate consistency is achieved through domain events.

---

# 237. Event Publication Strategy

The recommended persistence pattern is:

```text
Aggregate Updated

↓

Database Commit

↓

Transactional Outbox

↓

Message Broker

↓

Consumers
```

This ensures:

- Reliable event delivery
- No lost events
- Consistent state
- Eventual consistency

Infrastructure remains independent of the messaging technology.

---

# 238. Audit Persistence

Audit records are persisted separately from Aggregate data.

Audit records contain:

- Who
- What
- When
- Previous State
- New State
- Correlation Identifier

Audit history shall not modify Aggregate state.

---

# 239. Historical Data Strategy

Historical Infrastructure remains queryable.

Historical information includes:

- Lifecycle History
- Capacity Changes
- Classification Changes
- Relationship Changes
- Health Assessments

Historical information supports:

- Analytics
- Governance
- Compliance
- Audit

---

# 240. Repository Design Principles

## RDP-01

Repositories manage Aggregate persistence only.

---

## RDP-02

Aggregate boundaries define transaction boundaries.

---

## RDP-03

Persistence technology remains replaceable.

---

## RDP-04

Read models remain independent of write models.

---

## RDP-05

Infrastructure remains the authoritative source of physical master data.

---

## RDP-06

Logical relationships never compromise physical hierarchy.

---

## RDP-07

Lifecycle replaces deletion as the primary state management mechanism.

---

## RDP-08

Events are published only after successful persistence.

---

# 241. Responsibility Matrix

| Responsibility | Owner |
|---------------|-------|
| Aggregate Persistence | Repository |
| Business Rules | Aggregate |
| Validation | Application Layer + Aggregate |
| Authorization | Identity & Access Management |
| Audit Logging | Platform Foundation |
| Read Models | Query Layer |
| Analytics | Analytics Domain |
| Event Delivery | Integration Platform |

---

# Part 14 Completion

This section defines the canonical persistence architecture for the Campus & Infrastructure domain, including:

- Persistence Philosophy
- Repository Architecture
- Aggregate Persistence
- Identity & Key Strategy
- Versioning
- Lifecycle-Based Retention
- Relationship Persistence
- Value Object Persistence
- Query Models
- Transaction Strategy
- Event Publication
- Audit Persistence
- Repository Design Principles

These principles ensure that the persistence implementation faithfully supports the domain model while remaining technology-independent, scalable, auditable, and aligned with Domain-Driven Design and the overall SutraERP enterprise architecture.

---

# Part 15 — Cross-Domain Dependencies, Consumption Matrix & Architecture Boundaries

---

# 242. Domain Position in SutraERP

The Campus & Infrastructure domain is a **foundational master data domain**.

It provides the authoritative representation of institutional physical infrastructure that is consumed by multiple operational and analytical domains.

The domain owns:

- Physical infrastructure hierarchy
- Infrastructure lifecycle
- Infrastructure metadata
- Operational availability
- Logical infrastructure organization

The domain does **not** own business activities performed within infrastructure.

---

# 243. Upstream Dependencies

The Campus & Infrastructure domain depends upon the following enterprise domains.

| Domain | Dependency | Purpose |
|---------|------------|---------|
| Platform Foundation | Mandatory | Multi-tenancy, configuration, auditing, events |
| Person & Identity | Mandatory | User identity and authentication context |
| Institution Management | Mandatory | Parent Institution ownership |
| Organization Management | Optional | Organizational associations for administration |

Infrastructure cannot function independently of these foundational domains.

---

# 244. Downstream Consumers

The following domains consume Infrastructure information.

| Domain | Dependency Type |
|----------|----------------|
| Admissions | Mandatory |
| Student Lifecycle | Mandatory |
| Timetable | Mandatory |
| Examination | Mandatory |
| Attendance | Mandatory |
| Hostel | Mandatory |
| Library | Mandatory |
| Transport | Optional |
| Asset Management | Mandatory |
| Maintenance | Mandatory |
| Security | Mandatory |
| HR | Optional |
| Analytics | Mandatory |
| AI Services | Mandatory |
| Disaster Management | Optional |
| Notifications | Optional |

Infrastructure publishes canonical facts to all consumers.

---

# 245. Dependency Philosophy

Infrastructure follows a publish-and-consume model.

```text
Infrastructure

↓

Business Facts

↓

Enterprise Events

↓

Consumer Domains

↓

Business Decisions
```

Infrastructure never performs downstream orchestration.

---

# 246. Admissions Integration

Admissions consumes Infrastructure for:

- Campus selection
- Admission centres
- Interview rooms
- Counselling locations
- Document verification facilities

Admissions owns:

- Seat allocation
- Student admission workflow
- Admission scheduling

Infrastructure provides only physical location information.

---

# 247. Student Lifecycle Integration

Student Lifecycle consumes:

- Campus assignment
- Hostel locations
- Learning facilities
- Student service centres

Infrastructure does not manage student movement.

---

# 248. Timetable Integration

Timetable is one of the largest Infrastructure consumers.

Infrastructure provides:

- Classrooms
- Laboratories
- Seminar Rooms
- Capacity
- Accessibility
- Operational Status

Timetable determines:

- Allocation
- Scheduling
- Conflict detection
- Resource optimization

Infrastructure never performs scheduling.

---

# 249. Examination Integration

Examination consumes:

- Examination Halls
- Capacity
- Accessibility
- Operational Availability
- Campus Information

Examination owns:

- Seating plans
- Invigilation
- Scheduling
- Candidate allocation

Infrastructure owns only examination facilities.

---

# 250. Attendance Integration

Attendance consumes:

- Classroom master data
- Lecture halls
- Practical laboratories
- Campus information

Attendance owns:

- Attendance recording
- Attendance policies
- Attendance calculation

Infrastructure provides locations only.

---

# 251. Hostel Integration

Hostel consumes:

- Hostel Buildings
- Hostel Floors
- Hostel Rooms

Hostel owns:

- Room allocation
- Student occupancy
- Hostel operations

Infrastructure owns the physical hostel.

---

# 252. Library Integration

Library consumes:

- Library Buildings
- Reading Rooms
- Discussion Rooms
- Digital Learning Spaces

Infrastructure provides locations.

Library owns library operations.

---

# 253. Asset Management Integration

Infrastructure and Asset Management are complementary.

Infrastructure owns:

- Buildings
- Floors
- Rooms
- Physical hierarchy

Asset Management owns:

- Furniture
- Equipment
- Computers
- Laboratory Instruments
- HVAC Assets
- Electrical Equipment

Assets are located within Infrastructure.

Infrastructure does not own movable assets.

---

# 254. Maintenance Integration

Infrastructure provides:

- Physical infrastructure
- Health Status
- Criticality
- Operational Availability

Maintenance owns:

- Work Orders
- Preventive Maintenance
- Repairs
- Inspection Scheduling
- Maintenance History

Infrastructure stores only current business facts.

---

# 255. Security Integration

Security consumes:

- Buildings
- Restricted Areas
- Security Zones
- Access Points
- Criticality

Security owns:

- Guards
- Patrols
- Access Control
- CCTV Operations
- Incident Response

Infrastructure provides physical topology.

---

# 256. HR Integration

HR consumes:

- Office Locations
- Department Buildings
- Staff Facilities

HR owns:

- Employee assignments
- Department administration

Infrastructure owns physical offices only.

---

# 257. Analytics Integration

Infrastructure publishes canonical facts.

Analytics derives:

- Utilization
- Occupancy Trends
- Capacity Forecasting
- Infrastructure KPIs
- Dashboards
- Executive Reports

Infrastructure never computes KPIs.

---

# 258. AI Integration

Infrastructure provides canonical enterprise facts.

Examples:

- Building hierarchy
- Space relationships
- Capacity
- Accessibility
- Sustainability capabilities
- Criticality

AI derives:

- Semantic understanding
- Recommendations
- Optimization
- Predictions
- Natural language interpretation

Infrastructure intentionally avoids semantic interpretation.

---

# 259. Disaster Management Integration

Infrastructure provides:

- Criticality
- Recovery Metadata
- Alternate Facilities
- Backup Utilities

Disaster Management owns:

- Emergency Plans
- Recovery Procedures
- Incident Management
- Crisis Response

Infrastructure stores resilience facts only.

---

# 260. Notification Integration

Infrastructure publishes business events.

Notification consumes:

- InfrastructureActivated
- BuildingClosed
- MaintenanceStarted
- SpaceUnavailable

Notification determines:

- Delivery Channels
- Templates
- Recipients
- Escalation

Infrastructure never sends notifications directly.

---

# 261. Architecture Boundary Matrix

| Capability | Infrastructure | Consumer Domain |
|------------|---------------|-----------------|
| Campus | ✓ | |
| Building | ✓ | |
| Floor | ✓ | |
| Space | ✓ | |
| Capacity Facts | ✓ | |
| Accessibility Facts | ✓ | |
| Sustainability Capabilities | ✓ | |
| Criticality Metadata | ✓ | |
| Scheduling | | Timetable |
| Student Allocation | | Admissions |
| Room Allocation | | Timetable / Hostel |
| Examination Planning | | Examination |
| Asset Ownership | | Asset Management |
| Maintenance Execution | | Maintenance |
| Security Operations | | Security |
| KPIs | | Analytics |
| AI Interpretation | | AI Platform |

---

# 262. Integration Rules

## IDR-01

Infrastructure publishes facts.

Consumers interpret them.

---

## IDR-02

Infrastructure never executes consumer workflows.

---

## IDR-03

Consumer domains shall not modify Infrastructure aggregates.

---

## IDR-04

Cross-domain communication shall occur through published contracts and domain events.

---

## IDR-05

Infrastructure remains implementation independent.

---

## IDR-06

Business ownership shall never overlap across domains.

---

# 263. Dependency Direction

```text
Platform Foundation
         │
         ▼
Institution Management
         │
         ▼
Campus & Infrastructure
         │
         ├──────── Admissions
         ├──────── Timetable
         ├──────── Examination
         ├──────── Attendance
         ├──────── Hostel
         ├──────── Library
         ├──────── Asset Management
         ├──────── Maintenance
         ├──────── Security
         ├──────── HR
         ├──────── Analytics
         ├──────── AI
         └──────── Disaster Management
```

Dependency flow is unidirectional.

Consumer domains depend on Infrastructure.

Infrastructure does not depend on operational domains.

---

# 264. Anti-Corruption Principles

To preserve architectural integrity:

- Consumer domains shall use published contracts.
- Internal Infrastructure entities remain private.
- No consumer may access Infrastructure persistence directly.
- Integration shall remain loosely coupled.
- Domain events shall remain immutable.

Anti-Corruption Layers (ACLs) should be introduced when integrating with external or legacy systems to translate external models into canonical Infrastructure concepts.

---

# 265. Future Extensibility

The domain is designed to support future capabilities without altering core aggregate boundaries.

Potential future integrations include:

- Digital Twin platforms
- BIM (Building Information Modeling)
- GIS and Mapping Services
- IoT Sensors
- Smart Building Automation
- Indoor Navigation
- Energy Management Systems
- Visitor Management
- Parking Management
- Environmental Monitoring

These capabilities should extend the domain through composition, integration contracts, or adjacent domains rather than modifying the canonical Infrastructure model.

---

# 266. Architectural Principles

## ADP-01

Infrastructure owns physical infrastructure only.

---

## ADP-02

Operational domains own operational behaviour.

---

## ADP-03

Analytics owns interpretation.

---

## ADP-04

AI owns semantic understanding.

---

## ADP-05

Infrastructure publishes immutable business facts.

---

## ADP-06

Consumers remain independently deployable.

---

## ADP-07

Domain ownership shall remain explicit and non-overlapping.

---

# Part 15 Completion

This section defines the enterprise architecture boundaries and cross-domain interactions for the Campus & Infrastructure domain, including:

- Upstream Dependencies
- Downstream Consumers
- Domain Responsibility Matrix
- Integration Responsibilities
- Architecture Boundary Matrix
- Dependency Direction
- Anti-Corruption Principles
- Future Extensibility
- Cross-Domain Design Principles

These boundaries establish the Campus & Infrastructure domain as the authoritative master data provider for institutional physical infrastructure while ensuring clear ownership, loose coupling, and scalable integration across the SutraERP enterprise architecture.

---

# Part 16 — Canonical Architecture Principles, Quality Attributes & Future Evolution

---

# 267. Architecture Vision

The Campus & Infrastructure domain establishes the enterprise-wide canonical representation of physical infrastructure.

Its architecture is designed to be:

- Stable
- Technology Independent
- Domain Driven
- Event Driven
- Extensible
- Multi-Tenant
- Cloud Ready
- Deployment Agnostic

The domain is expected to remain stable for decades while supporting future institutional growth.

---

# 268. Architectural Goals

The architecture shall achieve the following goals.

| Goal | Description |
|-------|-------------|
| AG-01 | Single Source of Truth for physical infrastructure |
| AG-02 | Clear Domain Ownership |
| AG-03 | Stable Public Contracts |
| AG-04 | Event-Driven Integration |
| AG-05 | Long-Term Maintainability |
| AG-06 | Technology Independence |
| AG-07 | Enterprise Scalability |
| AG-08 | Regulatory Readiness |

---

# 269. Domain Quality Attributes

The Campus & Infrastructure domain is designed around the following quality attributes.

| Quality Attribute | Priority |
|-------------------|----------|
| Correctness | Critical |
| Consistency | Critical |
| Availability | High |
| Reliability | High |
| Maintainability | High |
| Extensibility | High |
| Security | High |
| Auditability | High |
| Performance | Medium |
| Portability | High |

Correctness and consistency always take precedence over optimization.

---

# 270. Scalability Principles

The architecture supports institutional growth without changing the domain model.

Scalability dimensions include:

- Multiple Tenants
- Multiple Institutions
- Multiple Campuses
- Thousands of Buildings
- Tens of Thousands of Spaces
- Millions of historical events
- Long-term audit retention

Scaling concerns shall be addressed by infrastructure and deployment architecture, not by changing business semantics.

---

# 271. Deployment Independence

The Campus & Infrastructure domain is deployment agnostic.

It supports:

- Single-server deployments
- On-premises deployments
- Private cloud deployments
- Public cloud deployments
- Hybrid deployments
- Containerized environments
- Distributed enterprise deployments

Business architecture remains unchanged regardless of deployment topology.

---

# 272. Technology Independence

The canonical architecture intentionally avoids dependence upon:

- Database vendors
- Messaging platforms
- Programming languages
- Frameworks
- UI technologies
- Cloud providers

Implementation choices may evolve without requiring changes to the business architecture.

---

# 273. Extensibility Strategy

The domain is designed for controlled evolution.

Future capabilities shall be introduced through:

- New Value Objects
- New Events
- New Read Contracts
- New Integration Services
- Adjacent Domains
- Extension Aggregates (where justified)

Existing Aggregate boundaries should remain stable.

Breaking changes are strongly discouraged.

---

# 274. Backward Compatibility

Public integration contracts shall evolve compatibly.

Guidelines include:

- Additive changes preferred.
- Existing fields remain supported.
- Event versioning used for incompatible changes.
- Deprecated contracts remain available according to enterprise policy.

Consumer domains should not require immediate upgrades.

---

# 275. Configuration Principles

Institution-specific variation shall be handled through configuration rather than architecture changes.

Examples include:

- Building Classifications
- Space Types
- Accessibility Characteristics
- Sustainability Capabilities
- Criticality Levels
- Zone Types
- Collection Categories
- Portfolio Categories

Configuration shall never alter core business semantics.

---

# 276. Localization Principles

The architecture supports localization through configuration.

Examples include:

- Measurement Units
- Address Formats
- Date Formats
- Time Zones
- Languages
- Naming Conventions

Localization affects presentation, not domain behaviour.

---

# 277. Performance Principles

Performance optimization shall never compromise correctness.

Preferred optimization strategies include:

- Read Models
- Caching of immutable reference data
- Indexed queries
- Asynchronous event processing
- Pagination for search results

Business validation remains authoritative even when optimization is applied.

---

# 278. Reliability Principles

The domain shall provide reliable business processing.

Key principles include:

- Atomic aggregate transactions
- Optimistic concurrency control
- Reliable event publication
- Retryable integration
- Immutable audit history

Partial updates are prohibited.

---

# 279. Availability Principles

Infrastructure master data should remain highly available.

Recommended practices include:

- Redundant deployment
- Database replication
- Automated backups
- Disaster recovery procedures
- Health monitoring

Availability requirements are achieved through platform architecture rather than business logic.

---

# 280. Maintainability Principles

Maintainability is achieved through:

- Clear aggregate boundaries
- Explicit ownership
- Stable interfaces
- Comprehensive documentation
- Consistent naming
- Domain-driven modelling

Complexity should be localized within the owning aggregate.

---

# 281. Testability Principles

The architecture supports automated testing.

Testing levels include:

| Level | Purpose |
|--------|---------|
| Unit Tests | Aggregate business rules |
| Integration Tests | Repository and event publication |
| Contract Tests | Public read contracts |
| Workflow Tests | Business workflows |
| End-to-End Tests | Cross-domain interactions |

Business rules should be testable independently of persistence and UI.

---

# 282. Observability Principles

Operational observability should include:

- Structured logging
- Correlation identifiers
- Audit records
- Domain event tracing
- Performance metrics
- Health checks

Observability assists operational support but does not replace business audit.

---

# 283. Evolution Roadmap

The architecture anticipates future enhancements such as:

### Infrastructure Intelligence

- Digital Twin integration
- Indoor navigation
- Space utilization optimization

### Smart Campus

- IoT sensors
- Occupancy monitoring
- Environmental sensing

### Sustainability

- Energy monitoring
- Water monitoring
- Carbon accounting
- ESG reporting

### Facilities

- Predictive maintenance
- Automated inspections
- Asset synchronization

### Planning

- Campus expansion modelling
- Capital planning
- Infrastructure forecasting

These capabilities should consume or extend canonical Infrastructure facts rather than redefining them.

---

# 284. Architectural Risks

Potential risks include:

| Risk | Mitigation |
|------|------------|
| Domain Scope Creep | Enforce ownership boundaries |
| Tight Consumer Coupling | Stable public contracts |
| Direct Database Access | Repository and API enforcement |
| Duplicate Infrastructure Data | Canonical master ownership |
| Breaking Integrations | Versioned events and contracts |
| Uncontrolled Customization | Configuration-first approach |

---

# 285. Architecture Principles Summary

## AP-01

Infrastructure is the authoritative master for physical infrastructure.

---

## AP-02

Physical hierarchy is canonical.

---

## AP-03

Logical organization never replaces physical ownership.

---

## AP-04

Infrastructure publishes immutable business facts.

---

## AP-05

Consumers own business interpretation.

---

## AP-06

Public contracts shield implementation details.

---

## AP-07

Lifecycle replaces deletion.

---

## AP-08

Configuration replaces customization wherever possible.

---

## AP-09

Technology choices shall never dictate business architecture.

---

## AP-10

Future capabilities shall extend—not redefine—the canonical domain model.

---

# 286. Repository Compliance Checklist

This document is repository-complete when it satisfies the following.

| Requirement | Status |
|-------------|--------|
| Domain Purpose Defined | ✅ |
| Bounded Context Defined | ✅ |
| Aggregate Model Defined | ✅ |
| Lifecycle Defined | ✅ |
| Business Rules Defined | ✅ |
| Workflows Defined | ✅ |
| Domain Events Defined | ✅ |
| Public Contracts Defined | ✅ |
| Security Model Defined | ✅ |
| Persistence Strategy Defined | ✅ |
| Cross-Domain Boundaries Defined | ✅ |
| Quality Attributes Defined | ✅ |
| Future Evolution Defined | ✅ |

---

# 287. Final Architectural Statement

The Campus & Infrastructure domain provides the canonical enterprise model for institutional physical infrastructure within SutraERP.

It establishes:

- A stable physical hierarchy
- Explicit aggregate ownership
- Controlled lifecycle management
- Immutable business events
- Stable public contracts
- Clear architectural boundaries
- Technology-independent business semantics

By separating physical infrastructure facts from operational behaviour, analytical interpretation, and AI semantics, the domain remains cohesive, extensible, and resilient to future organizational and technological change.

This architecture serves as the authoritative foundation upon which all infrastructure-dependent domains within SutraERP can reliably build.

---

# Part 16 Completion

This final section concludes the Campus & Infrastructure domain architecture by defining:

- Enterprise Architecture Vision
- Quality Attributes
- Scalability & Deployment Principles
- Technology Independence
- Extensibility Strategy
- Localization & Configuration Principles
- Reliability, Performance & Maintainability Guidelines
- Testing & Observability Expectations
- Future Evolution Roadmap
- Architectural Risks
- Canonical Architecture Principles
- Repository Compliance Checklist
- Final Architectural Statement

With this section, the **L2-B03 — Campus Workflows** document is architecturally complete and ready for independent validation, repository compliance review, and merge approval.


