# SutraERP Architecture Dependency Map

## 1. Purpose

The Dependency Map defines the architectural dependency relationships across every level of the SutraERP architecture.

Its purpose is to ensure that:

- Architectural decisions flow in a single direction.
- Upstream documents define standards and constraints.
- Downstream documents consume and extend those standards.
- Architectural ownership remains clear.
- Business logic is never duplicated across domains.

This document defines **architectural dependencies only**.

Repository governance, Git workflows, collaboration rules, and development practices are maintained in their respective documents.

---

# 2. Architecture Dependency Hierarchy

```text
Baseline
    │
    ▼
Level 0
    │
    ▼
Level 1
    │
    ▼
Level 2
    │
    ▼
Level 3
    │
    ▼
Level 4
    │
    ▼
Level 5
    │
    ▼
Level 6
```

---

# 3. Level Dependency Matrix

| Architecture Level | Depends On | Used By | Purpose |
|--------------------|------------|----------|---------|
| Baseline | None | Level 0, Level 1 | Core vision, constraints and goals |
| Level 0 | Baseline | Level 1 | External system boundaries |
| Level 1 | Baseline, Level 0 | Level 2 | Platform-wide architecture |
| Level 2 | Level 1 | Level 3 | Business domains |
| Level 3 | Level 2 | Level 4 | Cross-domain orchestration |
| Level 4 | Level 3 | Level 5 | Module & technical design |
| Level 5 | Level 2, Level 4 | Level 6 | Roles & permissions |
| Level 6 | Level 5 | Development | User workflows |

---

# 4. Level 2 Section Dependency Graph

```text
                         Section A
                 Platform Foundation
                        │
                        ▼
              Section B
      Institution & Structure
                        │
        ┌───────────────┼────────────────┐
        ▼               ▼                ▼
 Section C         Section E       Section G
 Student           People &        Campus
 Lifecycle         Workforce       Operations
        │
        ▼
 Section D
 Academic Delivery
        │
        ▼
 Section F
 Finance
        │
        ▼
 Section I
 Examination

────────────────────────────────────────────

Section H
Institutional Services & Intelligence

Consumes information from all sections and
provides shared institutional services.
```

---

# 5. Level 2 Section Dependency Matrix

| Section | Depends On |
|----------|------------|
| Section A – Platform Foundation | Level 1 |
| Section B – Institution & Structure | Section A |
| Section C – Student Lifecycle | Sections A, B |
| Section D – Academic Delivery | Sections A, B, C, E |
| Section E – People & Workforce | Sections A, B |
| Section F – Finance | Sections A, B, C |
| Section G – Campus Operations | Sections A, B, E |
| Section H – Institutional Services & Intelligence | All Sections |
| Section I – Examination | Sections C, D, F |

---

# 6. Cross-Section Dependency Principles

## Section A – Platform Foundation

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

Every section depends on Section A.

---

## Section B – Institution & Structure

Provides organisational and academic structure.

Owns:

- Institutions
- Organisation hierarchy
- Campuses
- Academic structure
- Curriculum framework
- Academic calendar

Business domains consume these definitions.

---

## Section C – Student Lifecycle

Owns every stage of a student's lifecycle.

Examples:

- Admissions
- Activation
- Registration
- Progression
- Graduation
- Guardians
- Financial Aid Eligibility

---

## Section D – Academic Delivery

Consumes:

- Students
- Faculty
- Curriculum

Owns:

- Courses
- Timetables
- Teaching Sessions
- Attendance
- Academic Monitoring

---

## Section E – People & Workforce

Owns workforce information.

Examples:

- Employees
- Faculty
- HR
- Leave
- Payroll
- Position Assignment

---

## Section F – Finance

Owns all monetary operations.

Examples:

- Fees
- Payments
- Refunds
- Accounting
- Scholarships
- Financial Operations

No other section may own financial transactions.

---

## Section G – Campus Operations

Consumes institutional and workforce information.

Owns:

- Facilities
- Laboratories
- Hostel
- Library
- Transport
- Medical
- Security
- Housekeeping
- Teacher Guardian Scheme (TGS)

---

## Section H – Institutional Services & Intelligence

Provides shared institutional capabilities.

Examples:

- APAAR & DigiLocker
- Notifications
- Directory
- Placement
- Research
- Alumni
- Dashboards
- Reporting
- Compliance
- External Integrations

Section H provides services.

It does **not** own canonical business entities.

---

## Section I – Examination

Consumes:

- Student data
- Academic Delivery
- Finance

Owns:

- Assessment
- Examination
- Evaluation
- Grade Processing
- Result Processing
- Academic Outcomes

No other section owns examination results.

---

# 7. Dependency Rules

1. Dependencies are strictly one-directional.

2. Upstream documents define architecture.

3. Downstream documents consume architecture.

4. Downstream documents must never redefine upstream decisions.

5. Every business concept has exactly one canonical owner.

6. Cross-domain workflows must reference the owning domain instead of duplicating logic.

7. Any dependency change must be reviewed before implementation.

---

# 8. Future Expansion Rules

- Every new Level 2 document must declare its upstream dependencies.
- Cross-domain orchestration belongs in Level 3.
- Level 4 implements architecture without redefining business logic.
- Level 5 derives permissions from Levels 2 and 4.
- Level 6 derives user workflows from Levels 2–5.
- New sections or domains must update this document before architecture approval.

---

# 9. Dependency Change Policy

Whenever a new architectural dependency is introduced:

1. Update the affected Level 2 document.
2. Update this Dependency Map.
3. Validate upstream and downstream impacts.
4. Review architectural consistency.
5. Obtain Lead Architect approval before locking the document.

---

**Project:** SutraERP

**Document:** Dependency Map

**Status:** Active

**Maintained By:** Lead Architect