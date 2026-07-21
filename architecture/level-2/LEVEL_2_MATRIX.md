# SutraERP --- Level 2 Domain Workflows

> Purpose: Define domain-level workflows, ownership, lifecycle,
> boundaries, and interactions.
>
> This level does NOT include: - UI/Screens - APIs - Database Schema -
> Rust Implementation - DTOs - Event Payloads - Module Integration Flows
> (Level 3)

------------------------------------------------------------------------

# L2-00 --- Architecture Control (Current Chat)

Purpose: - Level 2 planning - Track progress - Resolve architecture
discrepancies - Decide next domain - Track dependencies - Final Level 2
review planning

Status: - \[ \] In Progress

------------------------------------------------------------------------

# SECTION A --- Platform Foundation

Purpose: Core platform infrastructure that every business domain depends
upon.

## L2-A01 --- Platform & Tenant Workflows

Status: \[ \]

## L2-A02 --- Person & Identity Workflows

Status: \[ \]

## L2-A03 --- Authentication Workflows

Status: \[ \]

## L2-A04 --- Authorization & Policy Workflows

Status: \[ \]

## L2-A05 --- Workflow Engine Workflows

Status: \[ \]

## L2-A06 --- Event Infrastructure Workflows

Status: \[ \]

## L2-A07 --- Audit Infrastructure Workflows

Status: \[ \]

## L2-A08 --- Module Registry & Configuration Workflows

Status: \[ \]

## L2-A09 --- Master Data Workflows

Status: \[ \]

### L2-A00 --- Platform Foundation Review

Status: \[ \]

------------------------------------------------------------------------

# SECTION B --- Institution & Structure

Purpose: Institutional hierarchy and academic foundation.

## L2-B01 --- Institution Workflows

Status: \[ \]

## L2-B02 --- Organisation Workflows

Status: \[ \]

## L2-B03 --- Campus & Infrastructure Workflows

Status: \[ \]

## L2-B04 --- Academic Structure Workflows

Status: \[ \]

## L2-B05 --- Academic Year & Academic Period Workflows

Status: \[ \]

## L2-B06 --- Operational Readiness Workflows

Status: \[ \]

### L2-B00 --- Institution & Structure Review

Status: \[ \]

------------------------------------------------------------------------

# SECTION C --- Student Lifecycle

Purpose: Complete student lifecycle from admission to graduation.

## L2-C01 --- Admissions Workflows

Status: \[ \]

## L2-C02 --- Student Lifecycle & Activation Workflows

Status: \[ \]

## L2-C03 --- Student Academic Registration Workflows

Status: \[ \]

## L2-C04 --- Student Academic Progression Workflows

Status: \[ \]

## L2-C05 --- Graduation & Student Exit Workflows

Status: \[ \]

## L2-C06 --- Guardian Workflows

Status: \[ \]

## L2-C07 --- Scholarship Workflows

Status: \[ \]

### L2-C00 --- Student Lifecycle Review

Status: \[ \]

------------------------------------------------------------------------

# SECTION D --- Academic Delivery

Purpose: Teaching operations and academic execution.

## L2-D01 --- Course & Course Offering Workflows

Status: \[ \]

## L2-D02 --- Faculty Academic Assignment Workflows

Status: \[ \]

## L2-D03 --- Timetable Workflows

Status: \[ \]

## L2-D04 --- Teaching Session & Attendance Workflows

Status: \[ \]

## L2-D05 --- Academic Monitoring Workflows

Status: \[ \]

### L2-D06 --- Open Electives & MOOCs Workflows

Status: \[ \]

### L2-D00 --- Academic Delivery Review

Status: \[ \]




------------------------------------------------------------------------

# SECTION E --- People & Workforce

Purpose: Workforce management and employee lifecycle.

## L2-E01 --- Workforce & Engagement Workflows

Status: \[ \]

## L2-E02 --- Position & Assignment Workflows

Status: \[ \]

## L2-E03 --- Faculty Workforce Workflows

Status: \[ \]

## L2-E04 --- HR Workflows

Status: \[ \]

## L2-E05 --- Leave Workflows

Status: \[ \]

## L2-E06 --- Payroll Workflows

Status: \[ \]

### L2-E00 --- People & Workforce Review

Status: \[ \]

------------------------------------------------------------------------

# SECTION F --- Finance

Purpose: Financial business domains.

## L2-F01 --- Finance Domain Workflows

Status: \[ \]

## L2-F02 --- Financial Aid, Fees & Scholarship Finance Workflows

Status: \[ \]

### L2-F00 --- Finance Review

Status: \[ \]

------------------------------------------------------------------------

# SECTION G --- Campus Operations

Purpose: Operational institutional services.

## L2-G01 --- Facilities Workflows

Status: \[ \]

## L2-G02 --- Lab Management Workflows

Status: \[ \]

## L2-G03 --- Hostel Workflows

Status: \[ \]

## L2-G04 --- Transport Workflows

Status: \[ \]

## L2-G05 --- Library Workflows

Status: \[ \]

## L2-G06 --- Medical Workflows

Status: \[ \]

## L2-G07 --- Security Operations Workflows

Status: \[ \]

## L2-G08 --- Housekeeping Operations Workflows

Status: \[ \]

## L2-G09 --- Teacher Guardian Scheme 

Status: \[ \]

### L2-G00 --- Campus Operations Review

Status: \[ \]

------------------------------------------------------------------------

# SECTION H --- Institutional Services & Intelligence

Purpose: Supporting institutional services and read models.

## L2-H01 --- Document Domain Workflows

Status: \[ \]

## L2-H02 --- Notification Workflows

Status: \[ \]

## L2-H03 --- Directory Workflows

Status: \[ \]

## L2-H04 --- Placement Workflows

Status: \[ \]

## L2-H05 --- Research Workflows

Status: \[ \]

## L2-H06 --- Alumni Workflows

Status: \[ \]

## L2-H07 --- Dashboard Workflows

Status: \[ \]

## L2-H08 --- Reporting Workflows

Status: \[ \]

### L2-H00 --- Services & Intelligence Review

Status: \[ \]

------------------------------------------------------------------------


# SECTION I --- Examination

Purpose: Assessment, examination and academic evaluation.

## L2-I01 --- Examination Domain Workflows

Status: \[ \]

## L2-I02 --- Assessment, Results & Grade Processing Workflows

Status: \[ \]

### L2-I00 --- Examination Review

Status: \[ \] Status: \[ \]



# Final Review

## L2-99 --- Complete Level 2 Architecture Review

Purpose: - Cross-domain consistency - Ownership validation - Dependency
validation - Remove contradictions - Lock Level 2 Architecture

Status: \[ \]

------------------------------------------------------------------------

# Suggested Chat Order

``` mermaid
flowchart TD
    A[L2-00 Architecture Control] --> B[Section A: Platform Foundation]
    B --> C[Section A Review: Platform Foundation Review]
    C --> D[Section B: Institution & Structure]
    D --> E[Section B Review: Institution & Structure Review]
    E --> F[Section C: Student Lifecycle]
    F --> G[Section D: Academic Delivery]
    G --> H[Section D Review: Academic Delivery Review]
    H --> I[Section C Review: Student Lifecycle Review]
    I --> J[Section E: People & Workforce]
    J --> K[Section E Review: People & Workforce Review]
    K --> L[Section F: Finance & Examination]
    L --> M[Section F Review: Finance & Examination Review]
    M --> N[Section I: Examination]
    N --> O[Section G: Campus Operations]
    O --> O_REV[Section G Review: Campus Operations Review]
    O_REV --> P[Section H: Institutional Services & Intelligence]
    P --> R[Section H Review: Services & Intelligence Review]
    R --> S[Section J: Team Planning]
    S --> T[L2-99 Final Architecture Review]
```
