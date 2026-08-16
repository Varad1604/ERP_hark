# SutraERP Team Assignments

## Purpose

This document defines architecture ownership and review responsibility for the SutraERP repository.

---

## Canonical Architecture Hierarchy

```text
L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6
```

| Level | Name | Definition |
|-------|------|------------|
| L0 | Business Vision | Defines why SutraERP exists, strategic intent, architectural context, and constraints. |
| L1 | Business Domain Architecture | Defines the canonical business domains, domain boundaries, and ownership. |
| L2 | Domain Workflow Architecture | Defines workflows, lifecycles, rules, and dependencies within an individual business domain. |
| L3 | Business Scenario Architecture | Defines independent end-to-end business scenarios that may derive information from one or many L2 workflow documents; L3 numbering is not dependent on L2 numbering. |
| L4 | System Behavior Specification | Defines expected system behavior for approved business scenarios without implementation design. |
| L5 | Solution Architecture | Defines engineering solution architecture needed to implement L4 behavior without changing business rules. |
| L6 | UI / UX Architecture | Defines user interaction, navigation, screens, and user journeys derived from L5 solution architecture without owning business logic. |

---

# Dhruv

## Role

**Lead Architect**

## Primary Sections

- Section A - Platform Foundation
- Section H - Institutional Services & Intelligence

## Repository-Wide Responsibilities

- Maintain repository-wide architectural consistency.
- Review architecture across all sections and all levels.
- Approve or request architectural changes.
- Resolve multi-domain architectural conflicts.
- Approve lock changes.
- Maintain architecture standards and naming consistency.

Dhruv remains Lead Architect with repository-wide architectural review responsibilities.

---

# Pallav

## Role

**Architect**

## Primary Sections

- Section B - Institution & Structure
- Section D - Academic Delivery, including D07 Internal Assessment

## Responsibilities

- Design and maintain assigned architecture.
- Ensure consistency within Sections B and D, including D07 Internal Assessment.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

---

# Varad

## Role

**Architect**

## Primary Sections

- Section C - Student Lifecycle
- Section E - People & Workforce

## Responsibilities

- Design and maintain assigned architecture.
- Ensure consistency within Sections C and E.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

---

# Chetan

## Role

**Architect**

## Primary Sections

- Section F - Finance
- Section J - Enterprise Resource Management

## Responsibilities

- Design and maintain Finance architecture.
- Design and maintain Enterprise Resource Management architecture, including vendor management, supplier management, procurement, inventory management, stock management, warehouse management, asset lifecycle management, and resource management.
- Ensure consistency within Sections F and J.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

Chetan remains the implementation and domain owner of Finance.

---

# Devang

## Role

**Architect**

## Primary Sections

- Section G - Campus Operations
- Section I - Examination

## Responsibilities

- Design and maintain assigned architecture.
- Ensure consistency within Sections G and I.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

---

# Finance Review Team

## Members

- Varad
- Devang

## Responsibility

The Finance Review Team provides review responsibility only. It does not transfer implementation ownership or domain ownership away from Chetan.

---

# Canonical Capability Owners (L2)

> Established 2026-08-05 under integrity finding INT-003.

Every L2 document declares exactly one owner, and that owner is a **capability area**,
not a person. Individual architects appear only in the `Accountable Architect` column
below — never in an L2 document header.

The `Accountable Architect` is the individual answerable for the capability area: they
design and maintain it, coordinate cross-section changes, and submit it for review.
Ownership of the *architecture* belongs to the capability area so that ownership
survives reassignment of people.

| Document | Capability Owner | Accountable Architect |
|---|---|---|
| L2-A00 | Platform Foundation | Dhruv |
| L2-A01 | Platform Foundation — Tenancy | Dhruv |
| L2-A02 | Platform Foundation — Identity | Dhruv |
| L2-A03 | Platform Foundation — Authentication | Dhruv |
| L2-A04 | Platform Foundation — Authorization | Dhruv |
| L2-A05 | Platform Foundation — Workflow Engine | Dhruv |
| L2-A06 | Platform Foundation — Event Infrastructure | Dhruv |
| L2-A07 | Platform Foundation — Audit | Dhruv |
| L2-A08 | Platform Foundation — Module Registry | Dhruv |
| L2-A09 | Platform Foundation — Master Data | Dhruv |
| L2-A10 | Platform Foundation — Operational Readiness Governance & Certification | Dhruv |
| L2-B00 | Institution & Structure | Pallav |
| L2-B01 | Institution & Structure — Institution | Pallav |
| L2-B02 | Institution & Structure — Organisation | Pallav |
| L2-B03 | Institution & Structure — Institutional Campus | Pallav |
| L2-B04 | Institution & Structure — Curriculum Framework | Pallav |
| L2-B05 | Institution & Structure — Academic Calendar | Pallav |
| L2-C00 | Student Lifecycle | Varad |
| L2-C01 | Student Lifecycle — Admissions | Varad |
| L2-C02 | Student Lifecycle — Activation | Varad |
| L2-C03 | Student Lifecycle — Academic Registration | Varad |
| L2-C04 | Student Lifecycle — Academic Progression | Varad |
| L2-C05 | Student Lifecycle — Qualification Completion & Exit | Varad |
| L2-C06 | Student Lifecycle — Guardian | Varad |
| L2-C07 | Student Lifecycle — Scholarship Eligibility | Varad |
| L2-C08 | Student Lifecycle — Alumni Transition | Varad |
| L2-D00 | Academic Delivery | Pallav |
| L2-D01 | Academic Delivery — Course & Offering | Pallav |
| L2-D02 | Academic Delivery — Faculty Assignment | Pallav |
| L2-D03 | Academic Delivery — Timetable | Pallav |
| L2-D04 | Academic Delivery — Teaching Session & Attendance | Pallav |
| L2-D05 | Academic Delivery — Academic Monitoring | Pallav |
| L2-D06 | Academic Delivery — Electives & MOOCs | Pallav |
| L2-D07 | Academic Delivery — Internal Assessment | Pallav |
| L2-E00 | People & Workforce | Varad |
| L2-E01 | People & Workforce — Workforce & Engagement | Varad |
| L2-E02 | People & Workforce — Position & Assignment | Varad |
| L2-E03 | People & Workforce — Faculty Workforce | Varad |
| L2-E04 | People & Workforce — HR | Varad |
| L2-E05 | People & Workforce — Leave Management | Varad |
| L2-E06 | People & Workforce — Payroll | Varad |
| L2-E07 | People & Workforce — Employee Profile | Varad |
| L2-F00 | Finance | Chetan |
| L2-F01 | Finance — Student Charges & Fees | Chetan |
| L2-F02 | Finance — Payments | Chetan |
| L2-F03 | Finance — Refunds | Chetan |
| L2-F04 | Finance — Financial Adjustments | Chetan |
| L2-F05 | Finance — Accounting & General Ledger | Chetan |
| L2-F06 | Finance — Budgeting & Cost Centers | Chetan |
| L2-F07 | Finance — Accounts Receivable | Chetan |
| L2-F08 | Finance — Accounts Payable | Chetan |
| L2-F09 | Finance — Period & Closing | Chetan |
| L2-F10 | Finance — Treasury & Banking | Chetan |
| L2-F11 | Finance — Financial Aid | Chetan |
| L2-F12 | Finance — Asset Accounting | Chetan |
| L2-F13 | Finance — Reporting & Analytics | Chetan |
| L2-F14 | Finance — Procurement Consumption | Chetan |
| L2-G00 | Campus Operations | Devang |
| L2-G01 | Campus Operations — Facilities | Devang |
| L2-G02 | Campus Operations — Laboratory | Devang |
| L2-G03 | Campus Operations — Hostel | Devang |
| L2-G04 | Campus Operations — Transport | Devang |
| L2-G05 | Campus Operations — Library | Devang |
| L2-G06 | Campus Operations — Health Services | Devang |
| L2-G07 | Campus Operations — Security | Devang |
| L2-G08 | Campus Operations — Housekeeping | Devang |
| L2-G09 | Campus Operations — Teacher Guardian Scheme | Devang |
| L2-G10 | Campus Operations — Infrastructure Management | Devang |
| L2-H00 | Institutional Services & Intelligence | Dhruv |
| L2-H01 | Institutional Services — APAAR & DigiLocker | Dhruv |
| L2-H02 | Institutional Services — Notifications | Dhruv |
| L2-H03 | Institutional Services — Directory & Search | Dhruv |
| L2-H04 | Institutional Services — Placement | Dhruv |
| L2-H05 | Institutional Services — Research | Dhruv |
| L2-H06 | Institutional Services — Alumni Engagement | Dhruv |
| L2-H07 | Institutional Intelligence — Enterprise Dashboard | Dhruv |
| L2-H08 | Institutional Intelligence — Statutory Reporting | Dhruv |
| L2-H09 | Institutional Intelligence — Accreditation & Compliance | Dhruv |
| L2-H10 | Institutional Services — External Education Integration | Dhruv |
| L2-I00 | Examinations | Devang |
| L2-I01 | Examinations — Policy & Configuration | Devang |
| L2-I02 | Examinations — Session Management | Devang |
| L2-I03 | Examinations — Planning & Scheduling | Devang |
| L2-I04 | Examinations — Registration | Devang |
| L2-I05 | Examinations — Hall Ticket | Devang |
| L2-I06 | Examinations — Practical & Viva Conduct | Devang |
| L2-I07 | Examinations — Theory Conduct | Devang |
| L2-I08 | Examinations — Marks Collection & Evaluation | Devang |
| L2-I09 | Examinations — Result Processing | Devang |
| L2-I10 | Examinations — Result Publication | Devang |
| L2-I11 | Examinations — Revaluation & Rechecking | Devang |
| L2-I12 | Examinations — Supplementary & Backlog | Devang |
| L2-I13 | Examinations — Academic Completion & CGPA | Devang |
| L2-I14 | Examinations — Malpractice & Committee | Devang |
| L2-I15 | Examinations — Exception & Contingency | Devang |
| L2-I16 | Examinations — Session Closure & Archival | Devang |
| L2-J00 | Enterprise Resource Management | Chetan |
| L2-J01 | Enterprise Resource Management — Procurement Planning | Chetan |
| L2-J02 | Enterprise Resource Management — Vendor & Supplier | Chetan |
| L2-J03 | Enterprise Resource Management — Requisition & Approval | Chetan |
| L2-J04 | Enterprise Resource Management — Purchase Orders | Chetan |
| L2-J05 | Enterprise Resource Management — Goods Receipt | Chetan |
| L2-J06 | Enterprise Resource Management — Inventory & Warehouse | Chetan |
| L2-J07 | Enterprise Resource Management — Stock Operations | Chetan |
| L2-J08 | Enterprise Resource Management — Asset Lifecycle | Chetan |
| L2-J09 | Enterprise Resource Management — Physical Verification | Chetan |
| L2-J10 | Enterprise Resource Management — Disposal & Write-Off | Chetan |

---

# Ownership Rules

- Every Level 2 document has exactly one primary owner, and that owner is a **capability area**
  from the Canonical Capability Owners table above. Individual names shall not appear in an L2
  document's owner fields.
- Review boards and approval authorities are recorded in separate approval fields, never in the
  owner field.
- Only the assigned owner may modify their architecture documents unless a governance migration explicitly authorizes repository-wide synchronization.
- Multi-section architectural changes require discussion with the respective owners.
- Every completed document must be reviewed and approved by the Lead Architect before merging.
- Locked documents may only be modified after approval from the Lead Architect.

---

# Team Summary

| Architect | Primary Sections | Review Responsibilities |
|-----------|------------------|-------------------------|
| Dhruv | A, H | Repository-wide architectural review |
| Pallav | B, D | Assigned section review |
| Varad | C, E | Assigned section review; Finance Review Team |
| Chetan | F, J | Assigned section review; Finance implementation and domain owner |
| Devang | G, I | Assigned section review; Finance Review Team |

---

**Project:** SutraERP

**Document Status:** Active

**Maintained By:** Lead Architect
