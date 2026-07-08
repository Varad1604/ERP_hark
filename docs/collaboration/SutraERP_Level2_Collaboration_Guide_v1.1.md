# SutraERP Level 2 Collaboration Guide (Updated)

**Version:** 1.1\
**Timeline:** Wednesday → Sunday (Architecture Freeze)\
**Coding Starts:** Monday

## Objective

Complete and lock the entire Level 2 architecture by Sunday. `main`
always represents the approved architecture.

------------------------------------------------------------------------

# Team Responsibilities

## Dhruv --- Lead Architect

Owns:

-   Section A --- Platform Foundation
-   Section B --- Institution & Structure
-   Section E --- People & Workforce
-   Architecture governance
-   Cross-domain ownership
-   Section reviews
-   Final architecture approval
-   GitHub repository maintenance
-   Pull Request approval and merge

## Pallav --- Domain Architect

Owns:

-   Section C --- Student Lifecycle
-   Section D --- Academic Delivery
-   Section G --- Campus Operations
-   Section H --- Institutional Services
-   Section F03 --- Team Contract Boundary Draft

------------------------------------------------------------------------

# GitHub Workflow

Branches

    main
    ├── dhruv
    └── pallav

Rules

1.  Work only on your branch.
2.  One completed MD file = one commit.
3.  Open Pull Request.
4.  Dhruv reviews.
5.  Merge to `main` only after approval.
6.  Locked files require a new PR and version increment.

------------------------------------------------------------------------

# Chat Structure

-   L2-00 → Architecture Control
-   One Chat = One Domain
-   One Section Review Chat after every section
-   L2-99 → Final Level 2 Review

------------------------------------------------------------------------

# Timeline

## Wednesday

### Dhruv

-   A01 Platform & Tenant
-   A02 Person & Identity
-   A03 Authentication

### Pallav

-   Read baseline
-   Prepare templates
-   Create project chats

------------------------------------------------------------------------

## Thursday

### Dhruv

-   A04 Authorization
-   A05 Workflow Engine
-   A06 Events

### Pallav

-   C01 Admissions
-   C02 Student Lifecycle
-   C03 Student Registration

------------------------------------------------------------------------

## Friday

### Dhruv

-   A07 Audit
-   A08 Module Registry
-   A09 Master Data
-   A00 Review

### Pallav

-   D01 Course & Offering
-   D02 Faculty Assignment
-   D03 Timetable

------------------------------------------------------------------------

## Saturday

### Dhruv

-   B01 Institution
-   B02 Organisation
-   B03 Campus
-   E01 Workforce
-   E02 Position & Assignment

### Pallav

-   D04 Attendance
-   D05 Academic Monitoring
-   D00 Review
-   C04 Progression
-   C05 Graduation

------------------------------------------------------------------------

## Sunday

### Dhruv

-   B04 Academic Structure
-   B05 Academic Year & Period
-   B06 Operational Readiness
-   B00 Review
-   E03 Faculty Workforce
-   E04 HR
-   E05 Leave
-   E06 Payroll
-   E00 Review
-   Review all PRs
-   Merge approved work
-   Freeze Level 2

### Pallav

-   C06 Guardians
-   C07 Scholarships
-   G01--G08 Campus Operations (Level 2)
-   H01--H08 Institutional Services (Level 2)
-   F03 Team Contract Boundary Draft

------------------------------------------------------------------------

# Standard MD File Structure

Every Level 2 domain document must follow this structure.

``` text
Title
Version
Status (Draft / Review / Locked)

1. Purpose
2. Scope
3. Domain Ownership
4. Actors
5. Core Concepts / Entities
6. Lifecycle States
7. Domain Workflows
8. Preconditions
9. Business Rules
10. Domain Boundaries
11. Dependencies
12. Synchronous Contracts Required
13. Domain Events (High Level)
14. Failure Scenarios
15. Audit Requirements
16. Optional Module Behaviour
17. Open Questions
18. Risks
19. Decisions Locked
20. Version History
```

------------------------------------------------------------------------

# Review Workflow

``` text
Draft
↓
Self Review
↓
Commit
↓
Pull Request
↓
Architecture Review
↓
Corrections
↓
Merge to main
↓
LOCKED
```

------------------------------------------------------------------------

# Deliverables

By Sunday:

-   Section A Locked
-   Section B Locked
-   Section C Drafted
-   Section D Drafted
-   Section E Locked
-   Section F Boundaries Drafted
-   Section G Drafted
-   Section H Drafted
-   Final Review
-   Architecture Freeze

Monday:

Development starts from the approved `main` branch only.
