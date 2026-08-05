# L5-01 — Infrastructure Persistence & Repository Baseline

**Status:** Draft
**Architecture Level:** L5 — Solution Architecture
**Document Owner:** Devang
**Created:** 2026-08-05
**Source:** Extracted verbatim from
`architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md`
Part 14 under integrity finding INT-008.

---

# 1. Purpose

This document records the persistence, repository and data-model strategy for physical
infrastructure aggregates owned by `L2-G10 --- Infrastructure Management Workflows`.

It exists because persistence design is L5 Solution Architecture, not L2 Domain Workflow
Architecture. `governance/constitution/Volume_3_Architecture_Framework.md` scopes L2 to
workflows, lifecycles, rules and dependencies; storage mechanisms, keys, indexing and
transaction strategy are implementation concerns. Holding them in an L2 document was the
same level violation recorded as INT-002 for L0.

---

# 2. Status of This Document

**This content has not been re-reviewed as L5 architecture.** It was moved here to remove
a level violation, not because it was validated against an approved L4 System Behavior
Specification. No L4 documents exist yet.

Treat it as a recorded design intent carried over from L2, not as approved solution
architecture. It must be re-authored against `templates/L5_SolutionArchitecture_Template.md`
before it can be cited as canonical.

---

# 3. Recorded Persistence Baseline


---

## 220. Persistence Philosophy

The Campus & Infrastructure domain owns the **canonical business model**, not the physical database implementation.

Persistence exists solely to preserve authoritative business facts.

The persistence layer shall never dictate:

- Business Rules
- Aggregate Design
- Workflow Behaviour
- Domain Relationships

The Domain Model remains the source of truth.

---

## 221. Persistence Architecture Principles

The Infrastructure domain follows the following principles.

### PAP-01

Domain Model is technology independent.

---

### PAP-02

Persistence models remain internal.

---

### PAP-03

Consumers never access Infrastructure database tables.

---

### PAP-04

Repositories persist Aggregate Roots only.

---

### PAP-05

Persistence must preserve Aggregate consistency.

---

### PAP-06

Read Contracts are independent of storage implementation.

---

## 222. Aggregate Persistence Strategy

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

## 223. Repository Responsibilities

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

## 224. Canonical Persistence Model

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

## 225. Identity Model

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

## 226. Infrastructure Keys

Recommended key hierarchy:

| Key | Purpose |
|------|---------|
| UUID | Internal identity |
| Business Code | Institutional reference |
| External Reference | Third-party integration (optional) |

Only UUID is guaranteed globally unique.

---

## 227. Versioning Strategy

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

## 228. Soft Delete Strategy

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

## 229. Physical Deletion Policy

Physical deletion is exceptional.

Permitted only when:

- Record created in error.
- No downstream references exist.
- Governance approval granted.

Deletion shall generate an audit record.

Operational infrastructure shall never be physically deleted.

---

## 230. Relationship Persistence

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

## 231. Value Object Persistence

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

## 232. Repository Interface Principles

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

## 233. Query Model

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

## 234. Search Strategy

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

## 235. Indexing Principles

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

## 236. Transaction Strategy

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

## 237. Event Publication Strategy

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

## 238. Audit Persistence

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

## 239. Historical Data Strategy

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

## 240. Repository Design Principles

### RDP-01

Repositories manage Aggregate persistence only.

---

### RDP-02

Aggregate boundaries define transaction boundaries.

---

### RDP-03

Persistence technology remains replaceable.

---

### RDP-04

Read models remain independent of write models.

---

### RDP-05

Infrastructure remains the authoritative source of physical master data.

---

### RDP-06

Logical relationships never compromise physical hierarchy.

---

### RDP-07

Lifecycle replaces deletion as the primary state management mechanism.

---

### RDP-08

Events are published only after successful persistence.

---

## 241. Responsibility Matrix

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


---

# 4. Traceability

| Item | Value |
|---|---|
| Extracted from | L2-B03 Part 14 |
| Extraction date | 2026-08-05 |
| Integrity finding | INT-008 |
| Domain owner of the aggregates described | L2-G10 |
| Template to adopt | `templates/L5_SolutionArchitecture_Template.md` |
