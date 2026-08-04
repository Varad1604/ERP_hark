# SutraERP Architecture Index

> **Purpose**
>
> This document serves as the master index of all architectural documents within the SutraERP repository. It provides a single point of reference for document ownership, implementation status, and repository organization.
>
> **This document does NOT contain:**
>
> - Architecture details
> - Business workflows
> - Dependency mappings
> - Development tasks
> - Governance rules
> - Review history
>
> Those are maintained in their respective documents.

---

# Core Architecture Documents

| Document | Owner | Status |
|----------|-------|--------|
| SutraERP L0 Business Vision | Dhruv | Locked |
| SutraERP L1 Business Domain Architecture | Dhruv | Locked |
| DEPENDENCY_MAP | Dhruv | Locked |
| Architecture Level Validation Standards | Lead Architect | Active |
| L2 Domain Workflow Matrix | Dhruv | Active |
| Architecture Index | Dhruv | Active |

---

# Canonical Architecture Definitions

| Level | Name | Definition |
|-------|------|------------|
| L0 | Business Vision | Defines why SutraERP exists, strategic intent, architectural context, and constraints. |
| L1 | Business Domain Architecture | Defines the canonical business domains, domain boundaries, and ownership. |
| L2 | Domain Workflow Architecture | Defines workflows, lifecycles, rules, and dependencies within an individual business domain. |
| L3 | Business Scenario Architecture | Defines independent end-to-end business scenarios that may derive information from one or many L2 workflow documents; L3 numbering is not dependent on L2 numbering. |
| L4 | System Behavior Specification | Defines expected system behavior for approved business scenarios without implementation design. |
| L5 | Solution Architecture | Defines engineering solution architecture needed to implement L4 behavior without changing business rules. |
| L6 | UI / UX Architecture | Defines user interaction, navigation, screens, and user journeys derived from L5 solution architecture without owning business logic. |

```text
L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6
```

---

# L1 Business Domain Ownership

| Section | Domain | Owner |
|----------|--------|-------|
| Section A | Platform Foundation | Dhruv |
| Section B | Institution & Structure | Pallav |
| Section C | Student Lifecycle | Varad |
| Section D | Academic Delivery | Pallav |
| Section E | People & Workforce | Varad |
| Section F | Finance | Chetan |
| Section G | Campus Operations | Devang |
| Section H | Institutional Services & Intelligence | Dhruv |
| Section I | Examination | Devang |
| Section J | Enterprise Resource Management | Chetan |

---

# L2 Domain Workflow Documents

---

# Section A — Platform Foundation

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-A01 | Platform & Tenant Workflows | Dhruv | Draft |
| L2-A02 | Person & Identity Workflows | Dhruv | Draft |
| L2-A03 | Authentication Workflows | Dhruv | Draft |
| L2-A04 | Authorization & Policy Workflows | Dhruv | Draft |
| L2-A05 | Workflow Engine Workflows | Dhruv | Draft |
| L2-A06 | Event Infrastructure Workflows | Dhruv | Draft |
| L2-A07 | Audit Infrastructure Workflows | Dhruv | Draft |
| L2-A08 | Module Registry & Configuration Workflows | Dhruv | Draft |
| L2-A09 | Master Data Workflows | Dhruv | Draft |
| L2-A10 | Operational Readiness Workflows | Dhruv | Draft |
| L2-A00 | Platform Foundation Review | Dhruv | Approved Baseline |

**Section Total:** 11 Documents

---

# Section B — Institution & Structure

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-B01 | Institution Workflows | Pallav | Locked |
| L2-B02 | Organisation Workflows | Pallav | Architecture Locked |
| L2-B03 | Campus Workflows | Pallav | Architecture Locked |
| L2-B04 | Curriculum Framework Workflows | Pallav | Architecture Locked |
| L2-B05 | Academic Calendar Workflows | Pallav | Architecture Locked |
| L2-B00 | Institution & Structure Review | Pallav | Locked |

**Section Total:** 6 Documents

---

# Section C — Student Lifecycle

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-C01 | Admissions Workflows | Varad | Draft |
| L2-C02 | Student Lifecycle & Activation Workflows | Varad | Locked |
| L2-C03 | Academic Registration Workflows | Varad | Draft |
| L2-C04 | Student Academic Progression Workflows | Varad | Draft |
| L2-C05 | Qualification Completion & Student Exit Workflows | Varad | Draft |
| L2-C06 | Guardian Workflows | Varad | Draft |
| L2-C07 | Scholarship Eligibility Workflows | Varad | Draft |
| L2-C08 | Alumni Transition Workflows | Varad | Draft |
| L2-C00 | Student Lifecycle Review | Varad | Draft |

**Section Total:** 9 Documents

---
# Section D — Academic Delivery

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-D01 | Course & Course Offering Workflows | Pallav | Architecture Locked |
| L2-D02 | Faculty Academic Assignment Workflows | Pallav | Approved Baseline |
| L2-D03 | Timetable Workflows | Pallav | Architecture Locked |
| L2-D04 | Teaching Session & Attendance Workflows | Pallav | Architecture Locked |
| L2-D05 | Academic Monitoring Workflows | Pallav | Architecture Approved |
| L2-D06 | Open Electives & MOOCs Workflows | Pallav | Architecture Locked |
| L2-D07 | Internal Assessment Workflows | Pallav | Architecture Locked |
| L2-D00 | Academic Delivery Review | Pallav | Certified & Locked |

**Section Total:** 8 Documents

---

# Section E — People & Workforce

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-E01 | Workforce & Engagement Workflows | Varad | Draft |
| L2-E02 | Position & Assignment Workflows | Varad | Draft |
| L2-E03 | Faculty Workforce Workflows | Varad | Architecture Baseline |
| L2-E04 | HR Workflows | Varad | Draft |
| L2-E05 | Leave Management Workflows | Varad | Draft |
| L2-E06 | Payroll Workflows | Varad | Architecture Locked |
| L2-E07 | Employee Profile Workflows | Varad | Architecture Baseline |
| L2-E00 | People & Workforce Review | Varad | Locked |

**Section Total:** 8 Documents

---

# Section F — Finance

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-F00 | Finance Domain Model & Foundation | Chetan | Draft |
| L2-F01 | Student Charge & Fee Management | Chetan | Draft |
| L2-F02 | Payment Management Architecture | Chetan | Draft |
| L2-F03 | Refund Management | Chetan | Draft |
| L2-F04 | Financial Adjustment Management | Chetan | Draft |
| L2-F05 | Accounting & General Ledger Management | Chetan | Draft |
| L2-F06 | Budget & Cost Center Management | Chetan | Draft |
| L2-F07 | Accounts Receivable Management | Chetan | Draft |
| L2-F08 | Accounts Payable Management | Chetan | Draft |
| L2-F09 | Financial Period & Closing | Chetan | Draft |
| L2-F10 | Treasury & Banking | Chetan | Draft |
| L2-F11 | Scholarship & Financial Aid | Chetan | Draft |
| L2-F12 | Asset Financial Accounting Workflows | Chetan | Draft |
| L2-F13 | Financial Reporting & Analytics Workflows | Chetan | Draft |
| L2-F14 | Procurement Finance Consumption Workflows | Chetan | Draft |

**Section Total:** 15 Documents

---

## Running Totals

| Section | Documents |
|----------|----------:|
| Section A | 11 |
| Section B | 6 |
| Section C | 9 |
| Section D | 8 |
| Section E | 8 |
| Section F | 15 |
| **Total So Far** | **57 Documents** |

---
# Section G — Campus Operations

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-G01 | Facilities Management Workflows | Devang | Draft |
| L2-G02 | Laboratory Management Workflows | Devang | Draft |
| L2-G03 | Hostel Workflows | Devang | Draft |
| L2-G04 | Transport Workflows | Devang | Draft |
| L2-G05 | Library Workflows | Devang | Draft |
| L2-G06 | Medical Workflows | Devang | Draft |
| L2-G07 | Security Operations Workflows | Devang | Draft |
| L2-G08 | Housekeeping Operations & Service Governance Workflows | Devang | Draft |
| L2-G09 | Teacher Guardian Scheme (TGS) Workflows | Devang | Architecture Locked |
| L2-G10 | Infrastructure Management Workflows | Devang | Draft |
| L2-G00 | Campus Operations Review | Devang | Draft |

**Section Total:** 11 Documents

---

# Section H — Institutional Services & Intelligence

## Institutional Services

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-H01 | APAAR & DigiLocker Workflows | Dhruv | Locked |
| L2-H02 | Notification Workflows | Dhruv | Architecture Baseline |
| L2-H03 | Enterprise Directory & Search Workflows | Dhruv | Draft |
| L2-H04 | Placement Workflows | Dhruv | Architecture Baseline |
| L2-H05 | Research Workflows | Dhruv | Architecture Baseline |
| L2-H06 | Alumni Workflows | Dhruv | Draft |

## Institutional Intelligence

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-H07 | Enterprise Dashboard Workflows | Dhruv | Architecture Locked |
| L2-H08 | NEP, NAAC & AISHE Reporting Workflows | Dhruv | Architecture Locked |
| L2-H09 | Accreditation & Compliance Workflows | Dhruv | Draft |
| L2-H10 | External Education Integration Workflows | Dhruv | Architecture Locked |
| L2-H00 | Services & Intelligence Review | Dhruv | Locked |

**Section Total:** 11 Documents

---

# Section I — Examination

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-I00 | Examination Review | Devang | Draft |
| L2-I01 | Examination Policy & Configuration Workflows | Devang | Draft |
| L2-I02 | Examination Session Management Workflows | Devang | Draft |
| L2-I03 | Examination Planning & Scheduling Workflows | Devang | Draft |
| L2-I04 | Examination Registration Workflows | Devang | Draft |
| L2-I05 | Hall Ticket Management Workflows | Devang | Draft |
| L2-I06 | Practical & Viva Examination Conduct Workflows | Devang | Draft |
| L2-I07 | Theory Examination Conduct Workflows | Devang | Draft |
| L2-I08 | Marks Collection & Evaluation Workflows | Devang | Draft |
| L2-I09 | Result Processing & Computation Workflows | Devang | Draft |
| L2-I10 | Result Publication Workflows | Devang | Draft |
| L2-I11 | Revaluation & Rechecking Workflows | Devang | Draft |
| L2-I12 | Supplementary / Backlog / Improvement Examination Execution Workflows | Devang | Draft |
| L2-I13 | Academic Completion, CGPA & Completion Evidence Workflows | Devang | Draft |
| L2-I14 | Malpractice & Examination Committee Workflows | Devang | Draft |
| L2-I15 | Examination Exception & Contingency Workflows | Devang | Draft |
| L2-I16 | Examination Session Closure & Archival Workflows | Devang | Draft |

**Section Total:** 17 Documents

---

# Section J — Enterprise Resource Management

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-J00 | Enterprise Resource Management Architecture & Governance | Chetan | Trust Certification Ready |
| L2-J01 | Procurement Planning & Sourcing Workflows | Chetan | Trust Certification Ready |
| L2-J02 | Vendor & Supplier Management Workflows | Chetan | Trust Certification Ready |
| L2-J03 | Purchase Requisition & Approval Workflows | Chetan | Trust Certification Ready |
| L2-J04 | Purchase Order Management | Chetan | Trust Certification Ready |
| L2-J05 | Goods Receipt, Inspection & Acceptance | Chetan | Trust Certification Ready |
| L2-J06 | Inventory & Warehouse Management | Chetan | Trust Certification Ready |
| L2-J07 | Stock Operations & Distribution | Chetan | Trust Certification Ready |
| L2-J08 | Asset Lifecycle Management | Chetan | Trust Certification Ready |
| L2-J09 | Physical Verification, Audit & Reconciliation | Chetan | Trust Certification Ready |
| L2-J10 | Disposal, Scrap & Write-Off | Chetan | Trust Certification Ready |

**Section Total:** 11 Documents

---

# Future L3-L6 Architecture Placeholders

> These are level placeholders only. Placeholder README files are present to preserve folder structure. No L3, L4, L5, or L6 source architecture documents are present in this workspace yet.

| Level | Future Artifact Type | Indexing Rule |
|-------|----------------------|---------------|
| L3 | Business Scenario Architecture | Add only after an approved L3 source document exists. L3 numbering is independent of L2 numbering. |
| L4 | System Behavior Specification | Add only after an approved L4 source document exists and traces to L3. |
| L5 | Solution Architecture | Add only after an approved L5 source document exists and traces to L4. |
| L6 | UI / UX Architecture | Add only after an approved L6 source document exists and traces to L5. |

---

# Architecture Summary

| Section | Documents | Owner |
|----------|----------:|-------|
| Section A — Platform Foundation | 11 | Dhruv |
| Section B — Institution & Structure | 6 | Pallav |
| Section C – Student Lifecycle | 9 | Varad |
| Section D — Academic Delivery | 8 | Pallav |
| Section E — People & Workforce | 8 | Varad |
| Section F — Finance | 15 | Chetan |
| Section G — Campus Operations | 11 | Devang |
| Section H — Institutional Services & Intelligence | 11 | Dhruv |
| Section I — Examination | 17 | Devang |
| Section J — Enterprise Resource Management | 11 | Chetan |

---

# Repository Statistics

| Item | Count |
|------|------:|
| Core Architecture Documents | 6 |
| L1 Business Domains | 10 |
| L2 Workflow Documents | 107 |
| L3 Scenario Documents Present  | 0 |
| L4 System Behavior Documents Present | 0 |
| L5 Solution Architecture Documents Present | 0 |
| L6 UI/UX Documents Present | 0 |
| Section Review Documents | 10 |
| Total Indexed Architecture Documents Present | 113 |
| Active Architects | 5 |

---

**Document Status Legend**

| Status | Meaning |
|--------|---------|
| Draft | Under development |
| Review | Under architecture review |
| Trust Certification Ready | Complete and certification-ready, pending final lock decision |
| Architecture Approved | Approved by architecture review, not yet locked |
| Approved Baseline | Approved baseline state pending broader freeze controls |
| Architecture Baseline | Baseline architecture state used for controlled reference |
| Architecture Locked | Approved and frozen for architectural use |
| Certified & Locked | Certified final state and frozen |
| Locked | Approved and frozen |

> Status values in this repository are intentional and exact. Do not collapse them into a single generic state when updating index rows or review controls.
>
> This legend mirrors the canonical status vocabulary defined in `governance/constitution/Volume_2_Repository_Standards.md` §7, which is the single canonical source and governs if the two ever diverge. Volume 2 defines **13** status values; the nine above are those in active use. Four constitutional values — Proposed, Approved, Deprecated, Archived — are valid but not yet in active use by any indexed document, so they are omitted from the table above.

---

# Recent Changes

**2026-07-31 — Status reconciliation (INT-011/012/013/023):** Resolved 47 file-vs-Index/Matrix document status conflicts (each document's own declared status was taken as authoritative and normalized to the canonical legend wording, then mirrored into this Index and the Domain Workflow Matrix), added or corrected the `Document Owner` metadata field on 92 L2 files to match the architect named in `TEAM_ASSIGNMENTS.md`, corrected 19 H1 headings to match their Index title, and amended `governance/constitution/Volume_2_Repository_Standards.md` (v1.0 → v1.1) to formally adopt the 9-value status vocabulary already in use here. See `generated/reviews/integrity-2026-07-31-complete.md` for the source findings.

---

**Last Updated:** July 2026

**Repository:** SutraERP

**Maintained By:** Dhruv
