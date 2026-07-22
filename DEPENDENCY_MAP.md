# SutraERP Architecture Dependency Map

## 1. Purpose

The Dependency Map defines architectural dependency direction across the SutraERP repository.

It ensures that:

- higher-level documents constrain lower-level documents
- domain ownership remains unique
- workflow collaboration does not get mistaken for shared ownership
- dependency direction remains acyclic at the architectural level

This document defines **architectural dependency direction**, not implementation topology.

---

## 2. Architecture Dependency Hierarchy

```text
L0 - Business Vision
    |
    v
L1 - Business Domain Architecture
    |
    v
L2 - Domain Workflow Architecture
    |
    v
L3 - Business Scenario Architecture
    |
    v
L4 - System Behavior Specification
    |
    v
L5 - Solution Architecture
    |
    v
L6 - UI / UX Architecture
```

---

## 3. Level Dependency Matrix

| Architecture Level | Depends On | Used By | Purpose |
|--------------------|------------|---------|---------|
| L0 - Business Vision | None | L1 | Context, constraints, and institutional operating model |
| L1 - Business Domain Architecture | L0 | L2, L3 | Canonical domains and ownership |
| L2 - Domain Workflow Architecture | L1 | L3, L4 | Domain workflows, lifecycles, and boundaries |
| L3 - Business Scenario Architecture | L1, L2 | L4, L5 | End-to-end multi-domain scenarios |
| L4 - System Behavior Specification | L3 | L5 | Expected system behavior without solution design |
| L5 - Solution Architecture | L4 | L6 | Engineering implementation design |
| L6 - UI / UX Architecture | L5 | Development | Interaction and experience design |

---

## 4. L1 Business Domain Dependency Graph

```text
Section A  Platform Foundation
    |
    v
Section B  Institution & Structure
    |
    +--> Section C  Student Lifecycle
    |
    +--> Section E  People & Workforce
    |
    +--> Section G  Campus Operations
               |
Section C  ----+----> Section D  Academic Delivery
Section E  ---------------^
Section D  ------------------> Section I  Examination
Section C  ------------------> Section F  Finance

Section H consumes approved read contracts and published events from
all sections but does not own canonical business entities.
```

---

## 5. L1 Business Domain Dependency Matrix

The matrix below records **foundational architectural dependencies** only.
Runtime event feedback does not reverse ownership or create a new upstream dependency.

| Section | Foundational Depends On |
|----------|-------------------------|
| Section A - Platform Foundation | L0 |
| Section B - Institution & Structure | Section A |
| Section C - Student Lifecycle | Sections A, B |
| Section D - Academic Delivery | Sections A, B, C, E, G |
| Section E - People & Workforce | Sections A, B |
| Section F - Finance | Sections A, B, C |
| Section G - Campus Operations | Sections A, B, E |
| Section H - Institutional Services & Intelligence | All Sections |
| Section I - Examination | Sections C, D, F |

Special rule:

- Student Lifecycle may consume **published official results** from Examination for progression and qualification completion.
- That runtime consumption does not make Examination an upstream architectural dependency of Section C.

---

## 6. Cross-Section Dependency Principles

## Section A - Platform Foundation

Provides shared infrastructure used by every business domain.

Owns:

- Identity
- Authentication
- Authorization
- Workflow Engine
- Events
- Audit
- Module Registry
- Master Data

## Section B - Institution & Structure

Provides institutional structure and canonical academic structure.

Owns:

- Institution
- Organisation hierarchy
- Campus
- Academic Structure
- Curriculum framework
- Academic Calendar

Does not own:

- Infrastructure Management
- Course Offering
- Student lifecycle state

## Section C - Student Lifecycle

Owns the student lifecycle and student academic state.

Examples:

- Admissions
- Activation
- Academic Registration
- Academic Progression
- Qualification Completion and Student Exit
- Guardian relationships
- Scholarship Eligibility
- Alumni transition

## Section D - Academic Delivery

Consumes:

- Students
- Faculty
- Curriculum
- Campus and infrastructure context

Owns:

- Course Offering
- Faculty Academic Assignment
- Timetable
- Teaching Sessions
- Attendance
- Academic Monitoring

Does not own:

- Course
- Student promotion
- Official results

## Section E - People & Workforce

Owns workforce information.

Examples:

- Employees
- Faculty workforce profile
- HR
- Leave
- Payroll
- Position Assignment

## Section F - Finance

Owns all monetary operations.

Examples:

- Fees
- Payments
- Refunds
- Accounting
- Scholarship Financial Processing
- Financial Operations

No other section owns financial transactions.

## Section G - Campus Operations

Consumes institutional and workforce information.

Owns:

- Facilities
- Lab Management
- Hostel
- Library
- Transport
- Medical
- Security
- Housekeeping
- Teacher Guardian Scheme (TGS)
- Infrastructure Management

## Section H - Institutional Services & Intelligence

Provides supporting services and read-model surfaces.

Examples:

- APAAR & DigiLocker
- Notifications
- Directory
- Placement
- Research
- Dashboards
- Reporting
- Compliance
- External Integrations

Section H provides services and projections.

It does **not** own canonical business entities such as Student Context, Alumni, Course, Finance Ledger, or Examination Result.

## Section I - Examination

Consumes:

- Student Lifecycle context
- Academic Delivery inputs
- Finance status where policy requires it

Owns:

- Assessment
- Examination
- Evaluation
- Grade Processing
- Result Processing
- Academic outcomes

No other section owns official examination results.

---

## 7. Dependency Rules

1. Dependencies are one-directional at the architectural level.
2. Upstream documents define architecture.
3. Downstream documents consume architecture.
4. Downstream documents must never redefine upstream decisions.
5. Every business concept has exactly one canonical owner.
6. Runtime events do not transfer ownership.
7. Cross-domain workflows must reference the owning domain instead of duplicating logic.
8. Any dependency change must be reviewed before implementation.

---

## 8. Future Expansion Rules

- Every new L1 or L2 document must declare upstream dependencies.
- Cross-domain orchestration belongs in L3 Business Scenarios.
- L4 defines system behavior without redefining business logic.
- L5 derives technical constraints from L4.
- L6 derives user interaction specifications from L5.
- New sections or domains must update this document before architecture approval.

---

## 9. Dependency Change Policy

Whenever a new architectural dependency is introduced:

1. Update the affected L1/L2 document.
2. Update this Dependency Map.
3. Validate upstream and downstream impacts.
4. Review architectural consistency.
5. Obtain Lead Architect approval before locking the document.

---

**Project:** SutraERP

**Document:** Dependency Map

**Status:** Active

**Maintained By:** Lead Architect
