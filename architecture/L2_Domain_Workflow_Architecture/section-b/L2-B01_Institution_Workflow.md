# L2-B01 — Institution Workflows
**Section:** B — Institution & Structure  
**Architecture Level:** Level 2 (Domain Architecture)  
**Status:** Draft  
**Document Owner:** Institution & Structure — Institution  
**Owner:** Section B Architect  
**Depends On:** L2-A01 — Platform & Tenant Foundation  
**Used By:** L2-B02, L2-B03, L2-B04, L2-B05, L2-A10 (Operational Readiness, Section A) and all downstream domains

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §2 Scope |
| Actors | — not present — |
| Entities | — not present — |
| Business Rules | §13 Business Rules |
| Workflow | §8 Institution Lifecycle Workflow; L2-B01 — Institution Workflows |
| State Machine | §6 Institution Lifecycle States |
| Contracts | — not present — |
| Events | §10 Domain Events; Business Events (Internal); Integration Events (Public) |
| Dependencies | §14 Cross-Domain Dependencies |
| Locked Decisions | — not present — |

> **Gap:** 4 required section(s) have no counterpart in this document: Actors, Entities, Contracts, Locked Decisions. Recorded, not authored.

---

# 1. Purpose

## Objective

L2-B01 defines the **canonical Institution Aggregate** for SutraERP.

It establishes how an Institution is created, configured, activated, managed, suspended, and archived throughout its lifecycle.

This document serves as the **single source of truth** for every Institution managed within SutraERP.

---

# 2. Scope

## This document OWNS

- Institution
- Institution Identity
- Institution Lifecycle
- Institution Status
- Institution Configuration
- Institution Activation
- Institution Suspension
- Institution Reactivation
- Institution Archival
- Institution Domain Events

---

## This document DOES NOT OWN

- Organization Structure
- Departments
- Campuses
- Academic Structure
- Academic Calendar
- Operational Readiness
- Students
- Faculty
- Courses
- Finance
- Examination
- Library
- Hostel
- Transport

Those domains are defined separately.

---

# 3. Domain Definition

## What is an Institution?

An Institution is a legally recognized educational organization operating under a Tenant that delivers educational services through one or more campuses.

Examples include:

- Engineering College
- Polytechnic Institute
- Pharmacy College
- Management Institute
- School
- Junior College
- Skill Development Institute

The Institution is independent of the academic programs it offers.

Programs such as B.Tech, MBA, MCA, Diploma, etc. belong to the Academic Structure domain (L2-B04).

---

# 4. Architectural Position

```
Tenant
│
├── Institution A
├── Institution B
├── Institution C
└── Institution D
```

One Tenant may own multiple Institutions.

Every Institution belongs to exactly one Tenant.

### Institution ↔ Organization Domain Relationship
- **Context of Operation:** The Organization Domain (L2-B02) operates within the context of an Institution.
- **Upstream Dependency:** An Institution must exist and be in the `Active` lifecycle state before its Organization Domain (including hierarchical Organization Units) can be configured.
- **Separation of Concerns:** L2-B01 owns the Institution lifecycle and canonical **Institution Aggregate**, whereas L2-B02 owns the internal organizational model (Organization, Organization Units, Organization Types, and structure) of that Institution.

---

# 5. Business Ownership

Institution owns:

- Institutional Identity
- Institutional Configuration
- Institutional Lifecycle
- Institutional Status
- Institutional Administration

Institution does NOT own:

- Departments
- Faculty
- Students
- Courses
- Fees
- Timetable
- Examination

---

# 6. Institution Lifecycle States

```
Draft
   │
   ▼
Configured ◄──────────────────────┐
   │                             │
   ▼                             │
Pending Approval ────────────────┼────┘ (Returned)
   ┌────┴────────────────────────┐ │
   ▼                             ▼ │
Active                       Rejected
┌─┴──────────────┐
▼                ▼
Suspended       Archived (Terminal)
```

---

## Draft

Institution record has been created.

Characteristics

- Institution Business Identity initialized
- Basic information entered
- Institution Configuration incomplete

Institution is not operational.

---

## Configured

Mandatory Institution Configuration has been completed.

Characteristics

- Master data completed
- Validation successful
- Ready for activation

Institution is still not operational.

---

## Pending Approval

Institution activation request has been submitted and is awaiting authorization.

Characteristics

- Configuration locked during review
- Validation passed
- Awaiting admin/board decision

Institution is not operational.

---

## Returned

Activation request has been returned to the configuration stage for corrections.

Characteristics

- Unlock configuration for edit
- Corrections required by approval authority
- Transitions back to Configured once resaved

Institution is not operational.

---

## Rejected

Activation request was explicitly denied. This represents the end of the current approval request cycle.

Characteristics

- Locked state
- Activation denied
- Direct transition back to Active is prohibited
- Re-submission requires resetting back to Configured or launching a new configuration/approval request case

Institution is not operational.

---

## Active

Institution becomes an active business entity inside SutraERP.

Characteristics

- Can participate in downstream domain configuration
- Enables creation of organization structure and units (L2-B02)
- Can configure campuses (L2-B03)
- Can configure academic structure (L2-B04)
- Can configure academic calendar (L2-B05)

Operational readiness is still pending (L2-A10, Section A).

---

## Suspended

Institution is temporarily disabled.

Characteristics

- Historical data preserved
- No new operational activities
- Reports remain accessible
- Can later be reactivated

---

## Archived

Institution has permanently retired.

Characteristics

- Read-only
- No operational activities
- Historical reporting available
- Cannot be modified

---

# 7. Mandatory Master Information

The following categories must be configured.

## Identity

- Institution Name
- Institution Code
- Short Name
- Institution Type

---

## Regulatory

- Affiliated University (if applicable)
- Regulatory Bodies
- Registration Details

---

## Communication

- Address
- Contact Details
- Official Email
- Website

---

## Operational

- Academic Pattern
- Time Zone
- Working Days
- Default Language

---

## Administration

- Institution Administrator
- Default Policies

---

# 8. Institution Lifecycle Workflow

```
                                TENANT EXISTS
                                      │
                                      ▼
                           Create New Institution
                                      │
                                      ▼
                         Generate Institution Identity
                         (UUID, Institution Code)
                                      │
                                      ▼
                     Enter Mandatory Institution Details
                                      │
                                      ▼
                     Save Institution as "Draft"
                                      │
                                      ▼
                      Configure Institution Settings
        ┌────────────────────────────────────────────────────┐
        │                                                    │
        │ Identity Information                               │
        │ Legal / Regulatory Information                     │
        │ Communication Information                          │
        │ Operational Preferences                            │
        │ Administrative Information                         │
        └────────────────────────────────────────────────────┘
                                      │
                                      ▼
                         System Validation Begins
                                      │
                ┌─────────────────────┴─────────────────────┐
                │                                           │
          Validation Failed                         Validation Passed
                │                                           │
                ▼                                           ▼
     Return Validation Errors                  Mark as "Configured"
                │                                           │
                └─────────────────────┬─────────────────────┘
                                      │
                                      ▼
                     Institution Activation Request
                                      │
                        Final Eligibility Validation
                                      │
                ┌─────────────────────┴─────────────────────┐
                │                                           │
          Activation Rejected                    Activation Approved
                │                                           │
                ▼                                           ▼
        Return Error Messages                   Institution Status = ACTIVE
                                                            │
                                                            ▼
                                             Publish InstitutionActivated Event
                                                            │
                                                            ▼
                                      Institution Available for
                                    L2-B02 → L2-A10 Readiness
                                                            │
                                                            ▼
                                   Operational Readiness (L2-A10)
                                                            │
                                                            ▼
                              Ready for Admissions & Academic Operations
                                                            │
                                                            ▼
                                         Operational Lifecycle Management
                                                            │
                 ┌──────────────────────────────┬───────────────────────────┐
                 │                              │                           │
                 ▼                              ▼                           ▼
          Update Information             Suspend Institution         Archive Institution
                 │                              │                           │
                 │                              ▼                           ▼
                 │                    Status = Suspended          Status = Archived
                 │                              │                           │
                 │                              ▼                           ▼
                 │                     Publish Event             Publish Event
                 │                              │
                 │                              ▼
                 │                     Reactivation Request
                 │                              │
                 │                    Validation Successful?
                 │                              │
                 └──────────────────────────────┼───────────────────────┐
                                                │                       │
                                               No                      Yes
                                                │                       │
                                                ▼                       ▼
                                         Remain Suspended        Status = Active
```

---

# 9. Activation Validation Rules

Before an Institution becomes Active, the system must validate:

- Tenant exists
- Institution identity completed
- Mandatory master information completed
- Institution administrator assigned
- Institution code is unique
- Validation completed successfully

If any validation fails, activation must be rejected.

---

# 10. Domain Events

The Institution domain defines internal **Business Events** and public **Integration Events** as follows.

## Business Events (Internal)

Business Events communicate business facts within the Institution domain and remain internal.

| Event | Description |
|---|---|
| Institution Created | Institution record created |
| Institution Configured | Mandatory configuration completed |
| Institution Submitted for Approval | Institution submitted for activation review |
| Institution Returned for Correction | Activation request returned for correction |
| Institution Rejected | Activation request denied |
| Institution Activated | Institution activated |
| Institution Updated | Institution information modified |
| Institution Suspended | Institution temporarily disabled |
| Institution Reactivated | Institution restored |
| Institution Archived | Institution permanently archived |

## Integration Events (Public)

Integration Events communicate business facts to external domains (e.g., L2-B02 Organization Domain). These are public contracts and are published using PascalCase naming.

| Event | Description |
|---|---|
| InstitutionCreated | Institution record created |
| InstitutionConfigured | Mandatory configuration completed |
| InstitutionSubmittedForApproval | Institution submitted for activation review |
| InstitutionReturnedForCorrection | Activation request returned for correction |
| InstitutionRejected | Activation request denied |
| InstitutionActivated | Institution activated |
| InstitutionUpdated | Institution information modified |
| InstitutionSuspended | Institution temporarily disabled |
| InstitutionReactivated | Institution restored |
| InstitutionArchived | Institution permanently archived |

---

# 11. Inputs

- Active Tenant (L2-A01 Platform Foundation)
- Tenant Administrator (L2-A01 Platform Foundation)
- Institution Master Information

---

# 12. Outputs

Produces:

- Canonical Institution Aggregate
- Institution Business Identity (Immutable UUID and code)
- Institution Lifecycle State
- Institution Configuration
- Institution Business Events & Integration Events

Consumed by:

- L2-B02 — Organization Domain
- L2-B03 — Campus & Infrastructure
- L2-B04 — Curriculum Framework
- L2-B05 — Academic Calendar
- L2-A10 — Operational Readiness (Section A)

---

# 13. Business Rules

1. Every Institution belongs to exactly one Tenant.
2. One Tenant may own multiple Institutions.
3. Institution Code must be unique within a Tenant.
4. Institution cannot become Active unless mandatory validation succeeds.
5. Suspended Institutions retain all historical information.
6. Archived Institutions are read-only.
7. Institution in Rejected state cannot become Active directly; it must be returned to Configured/Draft or a new activation request must be submitted.
8. Institution never owns Students, Faculty, Courses, Finance, or Examination.
9. Institution serves as the root aggregate for all Section B domains.

---

# 14. Cross-Domain Dependencies

## Consumes

- Tenant (L2-A01 Platform Foundation)

---

## Produces

- Canonical Institution Aggregate

---

## Enables

- Organization Domain (L2-B02)
- Campus & Infrastructure (L2-B03)
- Curriculum Framework (L2-B04)
- Academic Calendar (L2-B05)
- Operational Readiness (L2-A10, Section A)

---

# 15. Architecture Notes

- Institution is the canonical business entity representing an educational organization.
- Institution is independent of the academic programs it offers.
- Academic programs are defined in L2-B04.
- Operational readiness is not part of Institution activation and is completed separately in L2-A10 (Section A).
- Admissions and academic operations can begin only after Operational Readiness is complete.

---

# 16. Architecture Lock

**Document Status:** Draft

This document establishes the canonical Institution Aggregate for SutraERP.

No downstream document may redefine:

- Institution Identity
- Institution Lifecycle
- Institution Ownership
- Institution Status

All subsequent Section B documents shall extend this aggregate without modifying its foundational architecture.