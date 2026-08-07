# L2-B03 — Campus Workflows

---






## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §3 Scope; §4 Out of Scope |
| Actors | Actors |
| Entities | Part 4 — Canonical Aggregates, Entities & Value Objects |
| Business Rules | §114 Business Rules; Part 5 — Infrastructure Lifecycle & Business Rules (defined by L2-G10) |
| Workflow | §102 Campus Registration Workflow; §103 Campus Activation Workflow; §104 Campus Modification Workflow |
| State Machine | §101 Campus Lifecycle; Part 5 — Infrastructure Lifecycle & Business Rules (defined by L2-G10); Lifecycle Management |
| Contracts | §178 InstitutionalCampus Summary Contract; Part 8 — InstitutionalCampus Events & Contracts; GP-07 — Stable Public Contracts |
| Events | §115 Campus Events Summary; §177 Published Events; Part 8 — InstitutionalCampus Events & Contracts |
| Dependencies | §243 Upstream Dependencies; §245 Dependency Philosophy; §263 Dependency Direction |
| Locked Decisions | — not present — |

> **Gap:** 1 required section(s) have no counterpart in this document: Locked Decisions. Recorded, not authored.

---

# Document Control

| Attribute | Value |
|------------|-------|
| **Document ID** | L2-B03 |
| **Document Name** | Campus Workflows |
| **Architecture Level** | Level 2 – Domain Architecture |
| **Domain Code** | B03 |
| **Domain Category** | Foundation Domain |
| **Status** | Locked |
| **Document Owner** | Institution & Structure — Institutional Campus |
| **Repository** | SutraERP Enterprise Architecture |
| **Depends On** | L1 Platform Foundation, L2-B01 Institution, L2-B02 Organisation |
| **Consumed By** | All infrastructure-dependent domains |
| **Architecture Style** | Domain-Driven Design (DDD), Event-Driven Architecture, Modular Monolith Ready, Microservice Compatible |

---

# 1. Purpose

The **Institutional Campus** domain defines the **canonical representation of the Campus as an institutional entity** within SutraERP.

It establishes how an Institution registers, governs and retires its campuses: campus identity, institutional status, Institution-to-Campus association, governance metadata and institutional addressing.

This domain serves as the **authoritative source of truth for institutional campus governance**.

It is **not** the owner of the physical model. Buildings, floors, spaces, facilities, utilities, physical hierarchy, capacity, availability, compliance characteristics and infrastructure lifecycle are owned by `L2-G10 --- Infrastructure Management Workflows`, consistent with L0 §20, which assigns canonical physical structure and lifecycle to Infrastructure.

> **Ownership split (INT-001, 2026-08-03).** B03 owns the *institutional* Campus — the fact that an Institution has a campus, its identity, and its institutional status. G10 owns the *physical* Campus — the site boundary containing buildings and spaces, and everything beneath it. Where the two refer to the same real-world site, G10's PhysicalInfrastructureCampus references the InstitutionalCampus identity issued here.

It provides a stable, extensible, and future-proof foundation upon which every infrastructure-dependent business domain operates.

---

# 2. Domain Mission

The mission of this domain is to provide a single, authoritative, and technology-independent representation of the **institutional campus**, so that every other domain can resolve which campuses an Institution operates and what their institutional standing is, without duplicating campus identity.

Physical facility knowledge is resolved from `L2-G10`, not from this domain.

This domain describes:

- What infrastructure exists
- How infrastructure is organized
- What characteristics infrastructure possesses
- How infrastructure changes throughout its lifecycle

This domain **does not** manage operational activities performed within that infrastructure.

---

# 3. Scope

The Institutional Campus domain governs the institutional lifecycle of a Campus, including:

- Campus institutional identity and code
- Campus registration against an Institution
- Institution-to-Campus association and cardinality
- Campus institutional status (planned, operational, suspended, retired)
- Campus governance metadata
- Campus institutional addressing and geographic reference
- Campus regulatory and accreditation reference held at institutional level
- Canonical institutional campus events

### Explicitly delegated to L2-G10

The following are **owned by `L2-G10 --- Infrastructure Management Workflows`** and are not in
this domain's scope:

Campus Sites, Buildings, Building Wings, Floors, Rooms, Laboratories, Auditoriums, Libraries,
Hostels (physical facilities), Sports Infrastructure, Utility Infrastructure, Roads, Parking Areas,
Open Spaces, Service Facilities, Physical Zones, Infrastructure Collections, Infrastructure
Portfolios, Physical Relationships, Infrastructure Classification, Capacity Information,
Accessibility Characteristics, Sustainability Capabilities, Compliance Characteristics,
Infrastructure Health, Infrastructure Criticality, Infrastructure Lifecycle, Infrastructure
Availability, and all canonical infrastructure events.

---

# 4. Out of Scope

The following responsibilities explicitly belong to other domains.

## 4.0 Physical Infrastructure

This domain does **not** own:

- Physical campus structure, buildings, wings, floors, rooms or spaces
- Physical hierarchy and physical relationships
- Infrastructure capacity, availability or utilisation
- Infrastructure compliance, health, criticality or classification
- Infrastructure lifecycle and infrastructure domain events
- Maintenance, facilities operations or space allocation

All of the above are owned by `L2-G10 --- Infrastructure Management Workflows`, per L0 §20.
This domain publishes institutional campus identity, which G10 references.

Added 2026-08-03 to close integrity finding INT-001.

---

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

The Institutional Campus domain does not publish physical capacities; `L2-G10` does.

---

## 4.3 Asset Management

The Institutional Campus domain is **not** an asset register.

Examples excluded:

- Furniture
- Computers
- Laboratory equipment
- Projectors
- Vehicles
- Network devices

Those belong to the Asset Management domain.

`L2-G10` defines **where assets exist**; this domain defines only which campuses an Institution operates.

---

## 4.4 Maintenance

The Institutional Campus domain does **not** manage:

- Work orders
- Preventive maintenance
- Corrective maintenance
- Repair activities
- Technician assignments

It only exposes infrastructure health and lifecycle information.

---

## 4.5 Security

The Institutional Campus domain does **not** manage:

- Guards
- CCTV operations
- Incident response
- Visitor management
- Physical security operations

The Institutional Campus domain identifies only the institutional campus record; physical locations and security-related characteristics belong to `L2-G10`.

---

## 4.6 Disaster Management

The Institutional Campus domain does **not** execute:

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

The Institutional Campus domain does **not** calculate:

- KPIs
- Trends
- Benchmarks
- Executive dashboards
- Forecasts

It publishes canonical operational facts.

---

## 4.8 Artificial Intelligence

The Institutional Campus domain does **not** maintain:

- AI semantic models
- Knowledge graphs
- AI interpretations
- Recommendations
- Predictive intelligence

Future AI platforms derive meaning from canonical institutional campus facts.

---

# 5. Domain Vision

The Institutional Campus domain enables SutraERP to maintain a single enterprise-wide understanding of which campuses an Institution operates, regardless of institution size, educational model, or deployment architecture. The corresponding physical facilities are modelled by `L2-G10`.

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

Every InstitutionalCampus identity shall exist only once within the enterprise model.

Duplicate institutional campus definitions are prohibited. Uniqueness of the corresponding *physical* site is governed by `L2-G10`.

---

## GP-02 — Facts Over Interpretation

The Institutional Campus domain owns institutional facts.

It does **not** own:

- KPIs
- Executive metrics
- AI interpretations
- Predictive analytics

---

## GP-03 — Institutional Representation

The Institutional Campus domain represents the institution's declared campus structure — which campuses an Institution operates and under what institutional status.

It does **not** represent the physical world, which is modelled by `L2-G10`, nor the business processes executed inside it.

---

## GP-04 — Long-Lived Records

InstitutionalCampus records are persistent enterprise objects.

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

The Institutional Campus domain publishes canonical institutional campus events.

Consumer domains decide how to respond.

This domain never orchestrates downstream workflows.

---

## GP-07 — Stable Public Contracts

The Institutional Campus domain owns its internal model.

External domains consume only published contracts and canonical events.

Internal implementation details remain private.

---

## GP-08 — Future Extensibility

The domain shall accommodate future institutional capabilities without architectural redesign, including:

- Multi-campus and multi-institute expansion
- Campus reorganisation and institutional transfer
- Regulatory campus accreditation contexts
- Enterprise Analytics consumption of institutional campus facts

Physical-world capabilities — Smart Campus, IoT Integration, Digital Twins, GIS Integration, Facilities Management, Building Automation, Sustainability Platforms and Disaster Management — are extensibility concerns of `L2-G10`, not of this domain.

---

# 7. Architecture Position

Within the SutraERP architecture, Institutional Campus is a **Foundation Domain**.

It provides enterprise-wide institutional campus context consumed by numerous operational domains. Physical context for the same sites is provided by `L2-G10`.

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
Institutional Campus (L2-B03)      Other Foundation Domains
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

The domain supplies the canonical institutional campus model while remaining independent of both the physical model owned by `L2-G10` and the business workflows executed by consuming domains.

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

This part establishes the architectural foundation of the **Institutional Campus** domain by defining:

- Purpose
- Domain Mission
- Scope
- Out of Scope
- Domain Vision
- Guiding Principles
- Architecture Position
- Architecture Characteristics

Subsequent parts build upon these principles to define the complete canonical institutional campus model, workflows, aggregates, business rules, events, public contracts, and multi-domain interactions.

---

# Part 2 — Domain Vision, Bounded Context & Ownership Model

---

# 9. Domain Vision

The **Institutional Campus** domain establishes the canonical enterprise representation of the Institution-to-Campus relationship within the SutraERP ecosystem. Physical locations are represented by `L2-G10 --- Infrastructure Management Workflows`.

Its objective is to ensure that every domain refers to the same physical infrastructure using a consistent business language, regardless of deployment model, institution type, or organizational complexity.

The domain serves as the enterprise-wide foundation upon which all infrastructure-dependent capabilities operate.

---

# 10. Strategic Objectives

The Institutional Campus domain exists to achieve the following strategic objectives.

## SO-01 — Establish a Canonical Institutional Campus Model

Provide a single enterprise-wide model representing:

- The Institution-to-Campus association
- Institutional campus identity and code
- Institutional campus status and its lifecycle
- Institutional campus governance metadata

No other domain shall redefine these concepts.

The canonical *physical* model — buildings, floors, spaces, facilities, physical hierarchy and physical relationships — is established by `L2-G10 --- Infrastructure Management Workflows`.

---

## SO-02 — Eliminate Institutional Campus Duplication

Prevent multiple domains from maintaining independent definitions of which campuses an Institution operates.

Institutional campus identity shall originate only from this domain. Physical site definitions originate only from `L2-G10`.

---

## SO-03 — Enable Enterprise Consistency

Every business workflow referencing an institutional campus shall reference the same canonical `InstitutionalCampus` identity.

Examples include:

- Admissions
- Timetable
- Examination
- Hostel
- Transport
- Library
- Asset Management

Workflows referencing a *physical* location shall reference the corresponding `L2-G10` aggregate, which in turn references the `InstitutionalCampus` identity issued here.

---

## SO-04 — Support Long-Term Evolution

The domain shall remain stable while supporting future institutional capabilities including:

- Multi-campus and multi-institute expansion
- Campus reorganisation and institutional transfer
- Regulatory and accreditation contexts attached to a campus

without requiring redesign of the core institutional model.

Physical-world evolution — Smart Campus, Digital Twin, GIS, IoT, Sustainability Platforms and Enterprise Facilities Management — is a `L2-G10` concern.

---

## SO-05 — Enable Loose Coupling

The Institutional Campus domain publishes institutional business facts.

Consumer domains independently implement business workflows.

No downstream domain shall become tightly coupled to this domain's internal implementation.

---

# 11. Bounded Context

Within the SutraERP Domain-Driven Design architecture, Institutional Campus represents an independent **Bounded Context**.

It owns every concept describing the *institutional* existence of a campus — that an Institution operates it, its identity, and its institutional status.

It does **not** own the physical model. Other domains may reference institutional campus identity but shall never redefine or own it.

---

## Bounded Context Responsibilities

This bounded context owns:

- InstitutionalCampus identity and code
- Institution-to-Campus association
- Institutional campus status and lifecycle
- Institutional campus governance metadata
- Canonical institutional campus events

---

## Bounded Context Does Not Own

This bounded context explicitly excludes:

- Physical locations, buildings, floors, spaces and facilities (`L2-G10`)
- Physical hierarchy, capacity, accessibility, sustainability and criticality metadata (`L2-G10`)
- Infrastructure lifecycle, health and availability (`L2-G10`)
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

The following terminology is canonical throughout SutraERP for the *institutional* campus concept.

| Term | Meaning |
|------|----------|
| Institutional Campus | The institutional fact that an Institution operates a campus, together with its identity, code and institutional status. |
| Campus Code | The institution-scoped unique identifier issued for an InstitutionalCampus. |
| Institutional Status | The lifecycle state of the institutional campus record. |
| Institution-to-Campus Association | The governed relationship linking an InstitutionalCampus to its owning Institution. |

These definitions are authoritative across all SutraERP domains.

Physical terminology — Building, Wing, Floor, Space, Room, Facility, Infrastructure, Zone, Collection, Portfolio, Capacity, Accessibility, Criticality, Lifecycle and Availability — is defined canonically in `L2-G10 --- Infrastructure Management Workflows` section 7 (Domain Terminology). This document shall not restate those definitions.

---

# 13. Domain Responsibilities

Institutional Campus owns the complete lifecycle of the institutional campus record.

Responsibilities include:

## Institutional Campus Registration

- Register an institutional campus
- Update institutional campus attributes
- Retire an institutional campus
- Archive an institutional campus

---

## Institutional Association Management

Maintain the governed Institution-to-Campus association, including transfer of a campus between institutions where institutional policy permits it.

---

## Institutional Status Management

Manage institutional status transitions.

Examples:

- Registered
- Active
- Suspended
- Decommissioned
- Archived

---

## Governance Metadata

Maintain institutional governance metadata attached to the campus record, such as regulatory registration context and institutional ownership designation.

---

## Event Publication

Publish canonical institutional campus events for downstream consumers.

---

## Public Data Contracts

Provide stable integration contracts without exposing internal implementation.

---

# 14. Ownership Boundaries

## Institutional Campus Owns

- Campus institutional identity and code
- Institution-to-Campus association
- Campus institutional status and its lifecycle
- Campus governance metadata
- Institutional campus events

---

## Institutional Campus References

The domain references but does not own:

- Institution
- Organisation
- Geographic Location
- Regulatory Authorities
- **Physical infrastructure (Building, Floor, Space, capacity, availability) --- owned by `L2-G10`**

---

## Institutional Campus Is Referenced By

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
| Institutional Campus (identity, code, institutional status) | L2-B03 |
| Institution-to-Campus association | L2-B03 |
| Campus governance metadata | L2-B03 |
| Physical Campus (site boundary) | L2-G10 |
| Building | L2-G10 |
| Floor | L2-G10 |
| Space / Room | L2-G10 |
| Infrastructure Hierarchy | L2-G10 |
| Capacity Facts | L2-G10 |
| Accessibility Facts | L2-G10 |
| Sustainability Capabilities | L2-G10 |
| Infrastructure Health | L2-G10 |
| Infrastructure Lifecycle | L2-G10 |
| Infrastructure Criticality | L2-G10 |
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

The Institutional Campus domain owns institutional facts.

Consumers own business decisions.

---

## DIP-02

The Institutional Campus domain publishes events.

Consumers subscribe independently.

---

## DIP-03

The Institutional Campus domain never invokes downstream business workflows.

---

## DIP-04

The Institutional Campus domain exposes contracts.

Consumers never access internal persistence directly.

---

## DIP-05

The Institutional Campus domain evolves independently without breaking consuming domains.

---

# 17. Strategic Position Within SutraERP

Institutional Campus is one of the foundational enterprise domains.

It establishes the physical context required by nearly every operational capability.

```text
                   Enterprise Foundation

         Institution
               │
        Organisation
               │
    Institutional Campus
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

The Institutional Campus domain is governed by the following constitutional principles:

1. Single Source of Truth
2. Canonical Institutional Representation
3. Facts Over Interpretation
4. Stable Public Contracts
5. Event-Driven Integration
6. Long-Lived Institutional Campus Records
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
- Multi-Domain Responsibilities
- Interaction Principles
- Strategic Position within SutraERP

These definitions serve as the constitutional foundation for all subsequent domain models, workflows, aggregates, events, and integration contracts described in the remaining sections of this architecture.

---

# Part 3 — Canonical Domain Model

---

# 19. Domain Model Overview

The Institutional Campus domain models the institutional standing of each campus using a single aggregate. The physical infrastructure beneath a campus - buildings, floors, spaces and their hierarchy - is modelled by `L2-G10`.

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

| Aggregate | Purpose | Owner |
|------------|---------|-------|
| InstitutionalCampus | Represents a campus as an institutional entity registered against an Institution | **L2-B03** |

This domain has exactly **one** aggregate root: `InstitutionalCampus`.

The following are **not aggregates of this domain**. They are owned by `L2-G10 --- Infrastructure
Management Workflows` and are listed here only so cross-references resolve:

| Aggregate | Purpose | Owner |
|------------|---------|-------|
| PhysicalInfrastructureCampus | Physical site boundary containing buildings and spaces | L2-G10 |
| Building | Represents a physical structure | L2-G10 |
| Floor | Represents a building floor | L2-G10 |
| Space | Represents any usable physical location | L2-G10 |
| Zone | Operational grouping of infrastructure | L2-G10 |
| Collection | Logical grouping of infrastructure | L2-G10 |
| Portfolio | Strategic governance grouping | L2-G10 |

Infrastructure Unit is a conceptual business abstraction owned by L2-G10 and is not an Aggregate Root.

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

# 23. InstitutionalCampus Aggregate

## Purpose

Represents a campus as an **institutional entity** — the fact that an Institution operates a
geographically identifiable site, and that site's institutional standing.

The corresponding *physical* site — its buildings, floors and spaces — is modelled separately by
`L2-G10` as PhysicalInfrastructureCampus, which references the identity issued here.

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

# 24. Physical Aggregates (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-008. B03 previously defined the
> physical aggregates twice - once here and again in Part 4 - while `L2-G10` owned
> them. Both copies were removed and the definitions moved to
> `L2-G10_Infrastructure_Management_Workflows.md` Section 8A.

The Building, Floor, Space, Zone, Collection and Portfolio aggregates, the Physical
Campus aggregate and their shared value objects are owned and defined by
`L2-G10 --- Infrastructure Management Workflows`, Section 8A.

B03 owns the InstitutionalCampus aggregate (Section 23) and nothing beneath it. Where
this domain needs to refer to a physical structure it does so by the identity G10
issues; it never restates the aggregate's attributes, invariants or lifecycle.

| Aggregate | Defined in |
|---|---|
| InstitutionalCampus | This document, Section 23 |
| Physical Campus | L2-G10 Section 8A.1 |
| Building | L2-G10 Section 8A.2 |
| Floor | L2-G10 Section 8A.3 |
| Space | L2-G10 Section 8A.4 |
| Zone | L2-G10 Section 8A.5 |
| Collection | L2-G10 Section 8A.6 |
| Portfolio | L2-G10 Section 8A.7 |
| Shared value objects | L2-G10 Section 8A.8 |

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

InstitutionalCampus relationships are classified into two categories.

---

## Reference Relationships

Represent external references held by the institutional campus record.

Examples:

- Institution
- Organisation
- Geographic Region
- Municipality

---

## Physical and Logical Relationships (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-004. Structural containment
> (Campus contains Building, Building contains Floor, Floor contains Space) and
> logical grouping membership (Zone, Collection, Portfolio) describe the physical
> model and are owned and defined by `L2-G10 --- Infrastructure Management Workflows`.

`L2-G10`'s PhysicalInfrastructureCampus references the InstitutionalCampus identity
issued by this domain. This document does not restate the physical relationship model.

---

# 31. Aggregate Ownership Rules

Each aggregate owns its own lifecycle, and that lifecycle is governed by the
domain that owns the aggregate. Per Section 15, B03 owns only the Institutional
Campus; every physical aggregate belongs to `L2-G10`.

| Aggregate | Owns Its Lifecycle | Owning Domain |
|------------|-------------------|---------------|
| Institutional Campus | Yes | L2-B03 |
| Physical Campus (site boundary) | Yes | L2-G10 |
| Building | Yes | L2-G10 |
| Floor | Yes | L2-G10 |
| Space | Yes | L2-G10 |
| Zone | Yes | L2-G10 |
| Collection | Yes | L2-G10 |
| Portfolio | Yes | L2-G10 |

Lifecycle ownership never crosses aggregate boundaries, and never crosses domain
boundaries. B03 shall not transition the lifecycle of any `L2-G10` aggregate; it
consumes their published state.

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

InstitutionalCampus identifiers are immutable.

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

The InstitutionalCampus record is long-lived.

---

## DM-04

Ownership follows aggregate boundaries.

---

## DM-05

Relationships are explicit.

---

## DM-06

Institutional campus facts are authoritative.

---

## DM-07

Consumer domains extend infrastructure through references rather than modification.

---

# Part 3 Completion

This section establishes the canonical domain model for Institutional Campus by defining:

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

The Institutional Campus domain follows **Domain-Driven Design (DDD)**.

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

# 44. Aggregate Relationships (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-008 to `L2-G10` Section 8B.

Entity relationships, aggregate communication, transaction rules, size guidelines and
evolution principles for physical aggregates are defined by
`L2-G10 --- Infrastructure Management Workflows`, Section 8B.

The InstitutionalCampus aggregate (Section 23) communicates with G10 aggregates only by
publishing its own events and consuming theirs. It holds no reference to a G10 aggregate
beyond the Campus Identifier it issues.

---

# Part 4 Completion

This section defines the canonical aggregate model for the Institutional Campus domain, including:

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

# Part 5 — Infrastructure Lifecycle & Business Rules (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-008. Parts 5 and 6 held the detailed
> lifecycle model and rule set for aggregates owned by `L2-G10`. Both moved to
> `L2-G10_Infrastructure_Management_Workflows.md`, Sections 11A and 13A.

The lifecycle states, transition rules, dependency rules, availability semantics and the
identity, structural, operational, capacity, area, accessibility, sustainability, health,
criticality, availability, event, contract and validation rule sets for physical
infrastructure are owned and defined by `L2-G10 --- Infrastructure Management Workflows`.

B03 defines the lifecycle of the InstitutionalCampus aggregate only (Section 23). It
consumes G10's published physical states and never transitions them.

| Content | Defined in |
|---|---|
| Infrastructure lifecycle states and transitions | L2-G10 Section 11A |
| Lifecycle dependency and availability rules | L2-G10 Section 11A |
| Physical infrastructure business and validation rules | L2-G10 Section 13A |
| InstitutionalCampus lifecycle | This document, Section 23 |

---


This section defines the canonical lifecycle architecture for Institutional Campus, including:

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


This section defines the constitutional business rules governing the Institutional Campus domain.

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


This section defines the canonical operational workflows of the Institutional Campus domain, including:

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

# Part 6 — Physical Workflow Specifications (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-008. Parts 7, 9, 10 and 11 held detailed
> workflow specifications for aggregates owned by `L2-G10`. They moved to
> `L2-G10_Infrastructure_Management_Workflows.md` Section 22A.

| Workflow area | Defined in |
|---|---|
| Infrastructure business workflows (WF-INF-001 to WF-INF-014) | L2-G10 Section 22A |
| Building management workflows | L2-G10 Section 22A |
| Floor and space management workflows | L2-G10 Section 22A |
| Zone, collection and portfolio workflows | L2-G10 Section 22A |
| Institutional campus workflows | This document, Part 8 |

B03 executes no workflow that creates, alters or retires a physical structure. Where an
institutional campus workflow depends on a physical outcome, it consumes the event G10
publishes.

---

# Part 7 — Campus Management Workflows

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

It owns only the institutional representation of the campus. The physical representation is owned by `L2-G10`.

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

# 105. Physical Campus Attribute Workflows (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-008 to
> `L2-G10_Infrastructure_Management_Workflows.md` Section 17A.

Boundary management, operational status, capacity profile, accessibility, sustainability
and criticality describe the **physical site**, which `L2-G10` owns. B03 retains the
**institutional** campus workflows: registration, activation, modification, decommission
and archive.

| Event | Published by |
|---|---|
| CampusCreated, CampusUpdated, CampusActivated, CampusDecommissioned, CampusArchived | L2-B03 (institutional) |
| CampusBoundaryUpdated, CampusCapacityUpdated, CampusAccessibilityUpdated, CampusSustainabilityUpdated, CampusCriticalityChanged, CampusOperationalStatusChanged | L2-G10 (physical) |

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

# 113. Multi-Domain Consumers

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

Events published by **this domain** for the InstitutionalCampus aggregate:

| Workflow | Event |
|----------|-------|
| Register Campus | CampusCreated |
| Update Campus | CampusUpdated |
| Activate Campus | CampusActivated |
| Decommission | CampusDecommissioned |
| Archive | CampusArchived |

Events describing the **physical site**, published by `L2-G10` (Section 17A) and
consumed here where institutional status depends on them:

| Workflow | Event | Publisher |
|----------|-------|-----------|
| Change Status | CampusOperationalStatusChanged | L2-G10 |
| Update Boundary | CampusBoundaryUpdated | L2-G10 |
| Update Capacity | CampusCapacityUpdated | L2-G10 |
| Update Accessibility | CampusAccessibilityUpdated | L2-G10 |
| Update Sustainability | CampusSustainabilityUpdated | L2-G10 |
| Update Criticality | CampusCriticalityChanged | L2-G10 |

---

# 116. Workflow Design Principles

## CWP-01

Campus owns only institutional campus information.

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

# Part 7 Completion

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

These workflows establish the authoritative business processes governing the lifecycle and management of Buildings within the Institutional Campus domain.

---


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

These workflows establish the authoritative operational model for the lowest levels of the Institutional Campus hierarchy and provide the canonical institutional campus identities referenced by scheduling, examination, admissions, maintenance, security, and other enterprise domains.

---


This section defines the canonical workflows for **Zones**, **Collections**, and **Portfolios**, including:

- Zone Management
- Zone Membership
- Collection Management
- Collection Membership
- Portfolio Management
- Portfolio Membership
- Logical Relationship Model
- Multi-Domain Consumers
- Business Rules
- Domain Events
- Workflow Design Principles

These workflows establish the enterprise-standard logical organization model for Institutional Campus, enabling operational grouping, business categorization, and strategic governance while preserving the integrity of the canonical institutional campus record.

---

# Part 8 — InstitutionalCampus Events & Contracts

> Rewritten 2026-08-05 under integrity finding INT-008. The previous Part 12 described the
> Infrastructure aggregate's event architecture and read contracts throughout, which belong
> to `L2-G10`. It moved to `L2-G10_Infrastructure_Management_Workflows.md` Section 44A.
> What remains below is what this domain actually publishes and exposes.

## 177. Published Events

B03 publishes exactly these events, all describing the InstitutionalCampus aggregate:

| Event | Meaning |
|---|---|
| CampusCreated | An Institution registered a campus as an institutional entity. |
| CampusUpdated | Institutional campus metadata changed. |
| CampusActivated | The campus became institutionally active. |
| CampusDecommissioned | The Institution retired the campus. |
| CampusArchived | The institutional campus record was archived for history. |

Events describing the physical site are published by `L2-G10` and listed in Section 115.
B03 publishes no event about a building, floor, space, capacity, accessibility,
sustainability, criticality or physical operational status.

## 178. InstitutionalCampus Summary Contract

The single read contract this domain exposes.

| Field | Note |
|---|---|
| Campus Identifier | Canonical identity issued here; G10 references it. |
| Campus Code | Institutional code, unique within the Institution. |
| Campus Name | Institutional name. |
| Institution Identifier | The owning Institution (`L2-B01`). |
| Institutional Status | Institutional lifecycle state, not physical readiness. |
| Institutional Address | Registered address for institutional purposes. |

Geo coordinates, physical operational status, capacity, accessibility and sustainability
are **not** in this contract. They belong to G10's Physical Campus Summary Contract
(Section 44A.190). Consumers needing both join on Campus Identifier.

## 179. Consumption

B03 consumes G10's physical campus events where institutional status depends on physical
readiness. It never mutates a G10 aggregate and never republishes a G10 fact as its own.

---


This section defines the enterprise integration architecture for the Institutional Campus domain, including:

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

These integration mechanisms establish the Institutional Campus domain as the authoritative source of physical infrastructure master data while enabling scalable, event-driven interoperability across all SutraERP domains.

---

# Part 9 — Infrastructure Security & Governance (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-008. Part 13 held the security,
> authorization, audit and governance model for physical infrastructure, owned by
> `L2-G10`. It moved to `L2-G10_Infrastructure_Management_Workflows.md` Section 48A.

Infrastructure roles, permission categories, resource scoping, the authorization
workflow, auditable operations, governance overrides, retention, privacy, compliance,
multi-tenant governance and the responsibility matrix for physical infrastructure are
owned and defined by `L2-G10 --- Infrastructure Management Workflows`, Section 48A.

Authorization for InstitutionalCampus operations is governed by Section A
(`L2-A04 --- Authorization & Policy Workflows`); B03 defines no authorization model of
its own.

---


This section defines the enterprise security and governance architecture for the Institutional Campus domain, including:

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

These principles ensure that Institutional Campus master data remains secure, auditable, governable, and compliant while delegating authentication, authorization, workflow approval, and platform-level enforcement to their respective enterprise domains.

---

# Part 10 — Persistence & Repository Architecture (extracted to L5)

> Extracted 2026-08-05 under integrity finding INT-008.

Persistence philosophy, aggregate persistence strategy, repository responsibilities, the
canonical persistence model, identity and key model, versioning, soft and physical delete
policy, relationship and value-object persistence, query and search strategy, indexing,
transaction strategy, event publication strategy, audit and historical data strategy moved
to `architecture/L5_Solution_Architecture/L5-01_Infrastructure_Persistence_Baseline.md`.

**This content belonged in neither L2 document.** Volume 3 scopes L2 to workflows,
lifecycles, rules and dependencies; storage mechanisms, keys and indexing are L5 Solution
Architecture. The aggregates it describes are owned by `L2-G10`, so the extracted document
records G10's persistence baseline, not B03's.

---


This section defines the canonical persistence architecture for the Institutional Campus domain, including:

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

# Part 11 — Multi-Domain Dependencies, Consumption Matrix & Architecture Boundaries

---

# 242. Domain Position in SutraERP

The Institutional Campus domain is a **foundational master data domain**.

It provides the authoritative representation of the institutional campus record that is consumed by multiple operational and analytical domains.

The domain owns:

- InstitutionalCampus identity and code
- Institution-to-Campus association
- Institutional campus status and lifecycle
- Institutional campus governance metadata

The domain does **not** own the physical infrastructure hierarchy, infrastructure lifecycle, infrastructure metadata, operational availability or logical infrastructure organization — these are owned by `L2-G10 --- Infrastructure Management Workflows`.

The domain does **not** own business activities performed within infrastructure.

---

# 243. Upstream Dependencies

The Institutional Campus domain depends upon the following enterprise domains.

| Domain | Dependency | Purpose |
|---------|------------|---------|
| Platform Foundation | Mandatory | Multi-tenancy, configuration, auditing, events |
| Person & Identity | Mandatory | User identity and authentication context |
| Institution Management | Mandatory | Parent Institution ownership |
| Organization Management | Optional | Organizational associations for administration |

The Institutional Campus domain cannot function independently of these foundational domains.

---

# 244. Downstream Consumers

The following domains consume Institutional Campus information.

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

The Institutional Campus domain publishes canonical facts to all consumers.

---

# 245. Dependency Philosophy

The Institutional Campus domain follows a publish-and-consume model.

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

The Institutional Campus domain never performs downstream orchestration.

---

# 246. Consuming-Domain Integration (defined by L2-G10)

> Migrated 2026-08-05 under integrity finding INT-008 to `L2-G10` Section 9A.

Integration with Admissions, Student Lifecycle, Timetable, Examination, Attendance, Hostel,
Library, Asset Management, Maintenance, Security, HR, Analytics, AI, Disaster Management and
Notification describes what **physical infrastructure** provides to those domains. It is
owned by `L2-G10 --- Infrastructure Management Workflows`, Section 9A.

B03's own consumption surface is narrow: domains that need to know **which campuses an
Institution operates** consume the InstitutionalCampus Summary Contract (Part 12). Domains
needing rooms, capacity, accessibility or physical availability consume G10 directly.

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

The Institutional Campus domain publishes facts.

Consumers interpret them.

---

## IDR-02

The Institutional Campus domain never executes consumer workflows.

---

## IDR-03

Consumer domains shall not modify InstitutionalCampus aggregates.

---

## IDR-04

Multi-domain communication shall occur through published contracts and domain events.

---

## IDR-05

The Institutional Campus domain remains implementation independent.

---

## IDR-06

Business ownership shall never overlap amulti-domains.

---

# 263. Dependency Direction

```text
Platform Foundation
         │
         ▼
Institution Management
         │
         ▼
Institutional Campus
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

The Institutional Campus domain does not depend on operational domains.

---

# 264. Anti-Corruption Principles

To preserve architectural integrity:

- Consumer domains shall use published contracts.
- Internal Infrastructure entities remain private.
- No consumer may access Infrastructure persistence directly.
- Integration shall remain loosely coupled.
- Domain events shall remain immutable.

Anti-Corruption Layers (ACLs) should be introduced when integrating with external or legacy systems to translate external models into canonical Institutional Campus concepts.

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

These capabilities should extend the domain through composition, integration contracts, or adjacent domains rather than modifying the canonical Institutional Campus model.

---

# 266. Architectural Principles

## ADP-01

The Institutional Campus domain owns institutional campus facts only. Physical infrastructure is owned by `L2-G10`.

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

The Institutional Campus domain publishes immutable business facts.

---

## ADP-06

Consumers remain independently deployable.

---

## ADP-07

Domain ownership shall remain explicit and non-overlapping.

---

# Part 11 Completion

This section defines the enterprise architecture boundaries and multi-domain interactions for the Institutional Campus domain, including:

- Upstream Dependencies
- Downstream Consumers
- Domain Responsibility Matrix
- Integration Responsibilities
- Architecture Boundary Matrix
- Dependency Direction
- Anti-Corruption Principles
- Future Extensibility
- Multi-Domain Design Principles

These boundaries establish the Institutional Campus domain as the authoritative master data provider for institutional physical infrastructure while ensuring clear ownership, loose coupling, and scalable integration across the SutraERP enterprise architecture.

---

# Part 12 — Canonical Architecture Principles, Quality Attributes & Future Evolution

---

# 267. Architecture Vision

The Institutional Campus domain establishes the enterprise-wide canonical representation of physical infrastructure.

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

The Institutional Campus domain is designed around the following quality attributes.

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

The Institutional Campus domain is deployment agnostic.

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

Institutional campus master data should remain highly available.

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
| End-to-End Tests | Multi-domain interactions |

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

These capabilities should consume or extend canonical Institutional Campus facts rather than redefining them.

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

The Institutional Campus domain is the authoritative master for institutional campus identity. `L2-G10` is the authoritative master for physical infrastructure.

---

## AP-02

Physical hierarchy is canonical.

---

## AP-03

Logical organization never replaces physical ownership.

---

## AP-04

The Institutional Campus domain publishes immutable business facts.

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
| Multi-Domain Boundaries Defined | ✅ |
| Quality Attributes Defined | ✅ |
| Future Evolution Defined | ✅ |

---

# 287. Final Architectural Statement

The Institutional Campus domain provides the canonical enterprise model for institutional physical infrastructure within SutraERP.

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

# Part 12 Completion

This final section concludes the Institutional Campus domain architecture by defining:

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


