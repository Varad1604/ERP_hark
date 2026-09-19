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
| DEPENDENCY_MAP | Dhruv | Active |
| Architecture Level Validation Standards | Lead Architect | Locked |
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
| L6 | UI / UX Architecture | Defines user interaction, navigation, screens and user journeys derived from L5 solution architecture without owning business logic. |

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
| L2-A01 | Platform & Tenant Workflows | Platform Foundation — Tenancy | Locked |
| L2-A02 | Person & Identity Workflows | Platform Foundation — Identity | Locked |
| L2-A03 | Authentication Workflows | Platform Foundation — Authentication | Locked |
| L2-A04 | Authorization & Policy Workflows | Platform Foundation — Authorization | Locked |
| L2-A05 | Workflow Engine Workflows | Platform Foundation — Workflow Engine | Locked |
| L2-A06 | Event Infrastructure Workflows | Platform Foundation — Event Infrastructure | Locked |
| L2-A07 | Audit Infrastructure Workflows | Platform Foundation — Audit | Locked |
| L2-A08 | Module Registry & Configuration Workflows | Platform Foundation — Module Registry | Locked |
| L2-A09 | Master Data Workflows | Platform Foundation — Master Data | Locked |
| L2-A10 | Operational Readiness Governance & Certification Workflows | Platform Foundation — Operational Readiness Governance & Certification | Locked |
| L2-A00 | Platform Foundation Review | Platform Foundation | Locked |

**Section Total:** 11 Documents

---

# Section B — Institution & Structure

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-B01 | Institution Workflows | Institution & Structure — Institution | Locked |
| L2-B02 | Organisation Workflows | Institution & Structure — Organisation | Locked |
| L2-B03 | Campus Workflows | Institution & Structure — Institutional Campus | Locked |
| L2-B04 | Curriculum Framework Workflows | Institution & Structure — Curriculum Framework | Locked |
| L2-B05 | Academic Calendar Workflows | Institution & Structure — Academic Calendar | Locked |
| L2-B00 | Institution & Structure Review | Institution & Structure | Locked |

**Section Total:** 6 Documents

---

# Section C — Student Lifecycle

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-C01 | Admissions Workflows | Student Lifecycle — Admissions | Locked |
| L2-C02 | Student Lifecycle & Activation Workflows | Student Lifecycle — Activation | Locked |
| L2-C03 | Academic Registration Workflows | Student Lifecycle — Academic Registration | Locked |
| L2-C04 | Student Academic Progression Workflows | Student Lifecycle — Academic Progression | Locked |
| L2-C05 | Qualification Completion & Student Exit Workflows | Student Lifecycle — Qualification Completion & Exit | Locked |
| L2-C06 | Guardian Workflows | Student Lifecycle — Guardian | Locked |
| L2-C07 | Scholarship Eligibility Workflows | Student Lifecycle — Scholarship Eligibility | Locked |
| L2-C08 | Alumni Transition Workflows | Student Lifecycle — Alumni Transition | Locked |
| L2-C00 | Student Lifecycle Review | Student Lifecycle | Locked |

**Section Total:** 9 Documents

---
# Section D — Academic Delivery

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-D01 | Course & Course Offering Workflows | Academic Delivery — Course & Offering | Locked |
| L2-D02 | Faculty Academic Assignment Workflows | Academic Delivery — Faculty Assignment | Locked |
| L2-D03 | Timetable Workflows | Academic Delivery — Timetable | Locked |
| L2-D04 | Teaching Session & Attendance Workflows | Academic Delivery — Teaching Session & Attendance | Locked |
| L2-D05 | Academic Monitoring Workflows | Academic Delivery — Academic Monitoring | Locked |
| L2-D06 | Open Electives & MOOCs Workflows | Academic Delivery — Electives & MOOCs | Locked |
| L2-D07 | Internal Assessment Workflows | Academic Delivery — Internal Assessment | Locked |
| L2-D00 | Academic Delivery Review | Academic Delivery | Locked |

**Section Total:** 8 Documents

---

# Section E — People & Workforce

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-E01 | Workforce & Engagement Workflows | People & Workforce — Workforce & Engagement | Locked |
| L2-E02 | Position & Assignment Workflows | People & Workforce — Position & Assignment | Locked |
| L2-E03 | Faculty Workforce Workflows | People & Workforce — Faculty Workforce | Locked |
| L2-E04 | HR Workflows | People & Workforce — HR | Locked |
| L2-E05 | Leave Management Workflows | People & Workforce — Leave Management | Locked |
| L2-E06 | Payroll Workflows | People & Workforce — Payroll | Locked |
| L2-E07 | Employee Profile Workflows | People & Workforce — Employee Profile | Locked |
| L2-E00 | People & Workforce Review | People & Workforce | Locked |

**Section Total:** 8 Documents

---

# Section F — Finance

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-F00 | Finance Domain Model & Foundation | Finance | Locked |
| L2-F01 | Student Charge & Fee Management | Finance — Student Charges & Fees | Locked |
| L2-F02 | Payment Management Architecture | Finance — Payments | Locked |
| L2-F03 | Refund Management | Finance — Refunds | Locked |
| L2-F04 | Financial Adjustment Management | Finance — Financial Adjustments | Locked |
| L2-F05 | Accounting & General Ledger Management | Finance — Accounting & General Ledger | Locked |
| L2-F06 | Budget & Cost Center Management | Finance — Budgeting & Cost Centers | Locked |
| L2-F07 | Accounts Receivable Management | Finance — Accounts Receivable | Locked |
| L2-F08 | Accounts Payable Management | Finance — Accounts Payable | Locked |
| L2-F09 | Financial Period & Closing | Finance — Period & Closing | Locked |
| L2-F10 | Treasury & Banking | Finance — Treasury & Banking | Locked |
| L2-F11 | Scholarship & Financial Aid | Finance — Financial Aid | Locked |
| L2-F12 | Asset Financial Accounting Workflows | Finance — Asset Accounting | Locked |
| L2-F13 | Financial Reporting & Analytics Workflows | Finance — Reporting & Analytics | Locked |
| L2-F14 | Procurement Finance Consumption Workflows | Finance — Procurement Consumption | Locked |

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
| L2-G01 | Facilities Management Workflows | Campus Operations — Facilities | Locked |
| L2-G02 | Laboratory Management Workflows | Campus Operations — Laboratory | Locked |
| L2-G03 | Hostel Workflows | Campus Operations — Hostel | Locked |
| L2-G04 | Transport Workflows | Campus Operations — Transport | Locked |
| L2-G05 | Library Workflows | Campus Operations — Library | Locked |
| L2-G06 | Medical Workflows | Campus Operations — Health Services | Locked |
| L2-G07 | Security Operations Workflows | Campus Operations — Security | Locked |
| L2-G08 | Housekeeping Operations & Service Governance Workflows | Campus Operations — Housekeeping | Locked |
| L2-G09 | Teacher Guardian Scheme (TGS) Workflows | Campus Operations — Teacher Guardian Scheme | Locked |
| L2-G10 | Infrastructure Management Workflows | Campus Operations — Infrastructure Management | Locked |
| L2-G00 | Campus Operations Review | Campus Operations | Locked |

**Section Total:** 11 Documents

---

# Section H — Institutional Services & Intelligence

## Institutional Services

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-H01 | APAAR & DigiLocker Workflows | Institutional Services — APAAR & DigiLocker | Locked |
| L2-H02 | Notification Workflows | Institutional Services — Notifications | Locked |
| L2-H03 | Enterprise Directory & Search Workflows | Institutional Services — Directory & Search | Locked |
| L2-H04 | Placement Workflows | Institutional Services — Placement | Locked |
| L2-H05 | Research Workflows | Institutional Services — Research | Locked |
| L2-H06 | Alumni Workflows | Institutional Services — Alumni Engagement | Locked |

## Institutional Intelligence

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-H07 | Enterprise Dashboard Workflows | Institutional Intelligence — Enterprise Dashboard | Locked |
| L2-H08 | NEP, NAAC & AISHE Reporting Workflows | Institutional Intelligence — Statutory Reporting | Locked |
| L2-H09 | Accreditation & Compliance Workflows | Institutional Intelligence — Accreditation & Compliance | Locked |
| L2-H10 | External Education Integration Workflows | Institutional Services — External Education Integration | Locked |
| L2-H00 | Services & Intelligence Review | Institutional Services & Intelligence | Locked |

**Section Total:** 11 Documents

---

# Section I — Examination

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-I00 | Examination Review | Examinations | Locked |
| L2-I01 | Examination Policy & Configuration Workflows | Examinations — Policy & Configuration | Locked |
| L2-I02 | Examination Session Management Workflows | Examinations — Session Management | Locked |
| L2-I03 | Examination Planning & Scheduling Workflows | Examinations — Planning & Scheduling | Locked |
| L2-I04 | Examination Registration Workflows | Examinations — Registration | Locked |
| L2-I05 | Hall Ticket Management Workflows | Examinations — Hall Ticket | Locked |
| L2-I06 | Practical & Viva Examination Conduct Workflows | Examinations — Practical & Viva Conduct | Locked |
| L2-I07 | Theory Examination Conduct Workflows | Examinations — Theory Conduct | Locked |
| L2-I08 | Marks Collection & Evaluation Workflows | Examinations — Marks Collection & Evaluation | Locked |
| L2-I09 | Result Processing & Computation Workflows | Examinations — Result Processing | Locked |
| L2-I10 | Result Publication Workflows | Examinations — Result Publication | Locked |
| L2-I11 | Revaluation & Rechecking Workflows | Examinations — Revaluation & Rechecking | Locked |
| L2-I12 | Supplementary / Backlog / Improvement Examination Execution Workflows | Examinations — Supplementary & Backlog | Locked |
| L2-I13 | Academic Completion, CGPA & Completion Evidence Workflows | Examinations — Academic Completion & CGPA | Locked |
| L2-I14 | Malpractice & Examination Committee Workflows | Examinations — Malpractice & Committee | Locked |
| L2-I15 | Examination Exception & Contingency Workflows | Examinations — Exception & Contingency | Locked |
| L2-I16 | Examination Session Closure & Archival Workflows | Examinations — Session Closure & Archival | Locked |

**Section Total:** 17 Documents

---

# Section J — Enterprise Resource Management

| ID | Document | Owner | Status |
|----|----------|-------|--------|
| L2-J00 | Enterprise Resource Management Architecture & Governance | Enterprise Resource Management | Locked |
| L2-J01 | Procurement Planning & Sourcing Workflows | Enterprise Resource Management — Procurement Planning | Locked |
| L2-J02 | Vendor & Supplier Management Workflows | Enterprise Resource Management — Vendor & Supplier | Locked |
| L2-J03 | Purchase Requisition & Approval Workflows | Enterprise Resource Management — Requisition & Approval | Locked |
| L2-J04 | Purchase Order Management | Enterprise Resource Management — Purchase Orders | Locked |
| L2-J05 | Goods Receipt, Inspection & Acceptance | Enterprise Resource Management — Goods Receipt | Locked |
| L2-J06 | Inventory & Warehouse Management | Enterprise Resource Management — Inventory & Warehouse | Locked |
| L2-J07 | Stock Operations & Distribution | Enterprise Resource Management — Stock Operations | Locked |
| L2-J08 | Asset Lifecycle Management | Enterprise Resource Management — Asset Lifecycle | Locked |
| L2-J09 | Physical Verification, Audit & Reconciliation | Enterprise Resource Management — Physical Verification | Locked |
| L2-J10 | Disposal, Scrap & Write-Off | Enterprise Resource Management — Disposal & Write-Off | Locked |

**Section Total:** 11 Documents

---

# L3 — Business Scenario Architecture

The canonical L3 scenario population is **1,084** across Sections A–J. L3 uses the governed scenario-document structure under:

```text
architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/
```

The complete canonical L3 corpus is globally **Locked — 1,084 / 1,084 scenarios** under the final corpus lock decision dated 2026-08-29.

The earlier 48-parent certification wave remains historical readiness evidence only. Effective 2026-08-29, all 1,084 canonical L3 scenarios are Locked and form the active parent population for full-corpus L4 mass authoring.

**Canonical L3 Population:** 1,084 scenarios  
**Current L4 workload population:** 1,084 Locked and ready parent scenarios

---

# L4 — System Behavior Specification

## Canonical Governance

| Document | Owner | Status | Execution Relevance |
|---|---|---|---|
| L4 Governance Baseline Lock Decision | Lead Architect | Locked | Canonical governance |
| L4 Governance and Architecture Standard | Lead Architect | Locked | Active |
| L4 System Behavior Specification Template v1.0 | Lead Architect | Locked | Active |
| L4 Validation Standard | Lead Architect | Locked | Active |
| L4 Initial Certified Parent Eligibility Baseline | Lead Architect | Architecture Baseline | Historical readiness evidence |
| L4 Parent-Readiness Validation — 45 Newly Approved Scenarios | Lead Architect | Architecture Approved | Historical readiness evidence |
| L4 Production Parent Baseline — 48 Certified Parents | Lead Architect | Architecture Baseline | Historical first-wave evidence |
| L4 Parallel Authoring Execution Plan — 48 Parents | Lead Architect | Architecture Approved | Superseded for active execution |
| L4 Parallel Authoring Authorization — 2026-08-27 | Lead Architect | Locked | Superseded for active execution |
| L4 Full-Corpus Parent Readiness Certification Workplan | Lead Architect | Architecture Approved | Completed readiness program / historical method |
| L3 Final Corpus Lock Decision — 1,084 Scenarios | Lead Architect | Locked | Active upstream parent baseline |
| L4 Full-Corpus Parent Readiness Baseline — 1,084 Parents | Lead Architect | Locked | Active readiness baseline |
| L4 Full-Corpus Parallel Authoring Execution Plan — 1,084 Scenarios | Lead Architect | Architecture Approved | Active execution plan |
| L4 Full-Corpus Parallel Authoring Authorization — 2026-08-29 | Lead Architect | Locked | Active authorization |

Effective 2026-08-29, the active L4 workload covers the complete **1,084-scenario canonical L3 population** using the exact locked L3 Phase 5 human allocation. The former 48-parent wave remains historical decision evidence and no longer constrains active execution.

## Canonical Production Structure

```text
architecture/L4_System_Behavior_Specification/
├── 00_Governance/
├── 00_Pilot/          # historical methodology evidence
└── 01_Production/     # canonical production specifications
    ├── section-a/
    ├── section-b/
    ├── section-c/
    ├── section-d/
    ├── section-e/
    ├── section-f/
    ├── section-g/
    ├── section-h/
    ├── section-i/
    └── section-j/
```

All A–J production folders are pre-materialized. Folder existence carries no readiness or authoring-completion semantics.

`L4_Full_Corpus_Parallel_Authoring_Authorization_2026-08-29.md` controls active full-corpus authoring. All 1,084 parents are currently Locked and ready; authors perform the normal no-drift check before file creation.

**L4 workload population:** 1,084 assigned scenarios  
**Static 48-parent execution limit:** Superseded 2026-08-29  
**Pilot specifications retained as historical evidence:** 3

---

# L5 — Solution Architecture

The active L5 program is governed under:

```text
architecture/L5_Solution_Architecture/governance/
```

The representative nine-specification pilot is complete, the L5 authoring methodology is locked, the **105-specification authoring population is frozen**, workload and execution sequencing are locked, and **broad L5 authoring is authorized**. The Final L5 Corpus Lock has not been issued.

## Canonical L5 Repository Structure

```text
architecture/L5_Solution_Architecture/
├── README.md
├── governance/          # standards, templates, registers, methodology and decisions
├── specifications/      # canonical production authoring location
├── historical/          # non-canonical preserved extracted technical material
├── adr/                 # materialize when first canonical ADR exists
├── L5-00_Technology_Stack_Baseline.md  # temporary compatibility pointer
└── L5-01_Infrastructure_Persistence_Baseline.md  # temporary compatibility pointer
```

No empty `adr/` or family folders are required. Family directories under `specifications/` are materialized when their first real specification is committed.

## Canonical L5 Governance and Planning Records

| Document | Owner | Status | Execution Relevance |
|---|---|---|---|
| L5 Major Architecture Areas Baseline | Lead Architect | Architecture Approved | Active 30-area baseline |
| L5 Architecture Family Register | Lead Architect | Architecture Approved | Active seven-family model |
| L5 Naming & Document Granularity Specialization | Lead Architect | Architecture Approved | Active naming/granularity model |
| L5 Naming Specialization Approval Decision — 2026-09-09 | Lead Architect | Locked | Active naming approval |
| L5 L4-to-L5 Coverage Audit Decision — 105 | Lead Architect | Architecture Approved | Active coverage decision |
| L5 Candidate Document Corpus Register — 105 | Lead Architect | Architecture Approved | Active specification register |
| L5 Authoring Population Freeze Decision — 105 | Lead Architect | Locked | Freezes 105 IDs/responsibility boundaries |
| L5 Governance and Architecture Standard | Lead Architect | Locked | Active governance baseline |
| L5 Solution Architecture Specification Template v1.0 | Lead Architect | Locked | Active authoring template |
| L5 Validation Standard | Lead Architect | Locked | Active validation baseline |
| L5 Technology Selection and ADR Methodology | Lead Architect | Locked | Active technology/ADR governance |
| L5 L4-to-L5 Traceability Methodology | Lead Architect | Locked | Active traceability governance |
| L5 ADR Register | Lead Architect | Locked | Canonical ADR identity register |
| L5 Pilot ADR Candidate Disposition — 2026-09-09 | Lead Architect | Locked | Six material ADR candidates retained; no ADR IDs allocated by this record |
| L5 Workload Complexity and Five-Person Allocation Decision — 2026-09-09 | Lead Architect | Locked | Active authoring allocation |
| L5 Dependency-Gated Parallel Authoring Execution Timeline — 2026-09-09 | Lead Architect | Locked | Active six-stage execution model |
| L5 Broad Authoring Authorization — 105 — 2026-09-09 | Lead Architect | Locked | Broad authoring authorized |
| L5 Repository Structure and Content Placement Decision — 2026-09-09 | Lead Architect | Locked | Canonical L5 folder/content placement |
| L5 Candidate Document Corpus Register — 99 | Lead Architect | Proposed | Historical; superseded for active corpus planning |

## Frozen Authoring Population

| Family | Specifications |
|---|---:|
| L5-A — Core Platform & Application | 20 |
| L5-B — Data & Information | 21 |
| L5-C — Contracts, Events & Integration | 18 |
| L5-D — Identity, Security & Trust | 19 |
| L5-E — Reliability, Deployment & Operations | 16 |
| L5-F — Engineering Delivery & Quality | 7 |
| L5-G — Client Platform | 4 |
| **Total** | **105** |

The 105 identifiers and responsibility boundaries are frozen for controlled broad authoring. This is not the Final L5 Corpus Lock.

## Locked Authoring Allocation

| Author | Specifications |
|---|---:|
| Varad | 31 |
| Pallav | 30 |
| Dhruv | 20 |
| Devang | 14 |
| Chetan | 10 |
| **Total** | **105** |

This table records authoring allocation only. It does not transfer canonical technical ownership or upstream business ownership.

## Active Execution Model

L5 uses **dependency-gated parallel authoring** across six execution stages. The stages are not strict serial waves. A specification may start once its required upstream L5 dependency is architecturally stable enough that the downstream author does not need to invent that responsibility.

Broad L5 authoring is **authorized** by `L5_Broad_Authoring_Authorization_105_2026-09-09.md`.

Canonical production specifications shall be authored under:

```text
architecture/L5_Solution_Architecture/specifications/
```

## Historical Extracted L5 Documents

| ID | Document | Historical Owner | Status | Canonical Location |
|----|----------|------------------|--------|-------------------|
| L5-00 | Technology Stack Baseline | Dhruv | Trust Certification Ready | `historical/L5-00_Technology_Stack_Baseline.md` |
| L5-01 | Infrastructure Persistence & Repository Baseline | Devang | Trust Certification Ready | `historical/L5-01_Infrastructure_Persistence_Baseline.md` |

These documents are historical technical inputs only. They are not part of the frozen 105-specification production corpus and are not automatic authority for greenfield technology or persistence architecture. The former root paths are temporary compatibility pointers retained for reference integrity.

---

# L6 — UI / UX Architecture

The active L6 area is located under:

```text
architecture/L6_UI_UX_Architecture/
├── README.md
├── 00_Governance/
├── 01_Foundation/
├── 02_Implementation_Review/
├── L6_Screen_and_Workspace_Inventory_DRAFT.md
├── L6_Persona_Journey_and_Navigation_Map_DRAFT.md
├── L6_Screen_Journey_Traceability_Register_DRAFT.md
├── L6_Experience_Slice_Register_DRAFT.md
├── L6_AI_First_UI_UX_Execution_Plan_DRAFT.md
└── L6_Student_Workspace_DRAFT.md
```

The active L6 package contains the controlled foundation, approved slice decisions, Draft planning/traceability artifacts, the approved W-C02 controlled draft package, and implementation-review support artifacts. The 105-unit inventory remains the architecture coverage baseline; delivery experience slices are a separate implementation/review planning view.

| Document | Owner | Status | Execution Relevance |
|---|---|---|---|
| L6 Governance and Architecture Standard — Draft | Lead Architect | Draft | Controlled working governance |
| L6 UI / UX Architecture Template v0.1 — Draft | Lead Architect | Draft | Controlled working template |
| L6 Validation Standard — Draft | Lead Architect | Draft | Controlled working validation |
| L6 Design System Foundation — Draft | Lead Architect | Draft | Controlled working foundation |
| L6 Application Shell and Information Architecture — Draft | Lead Architect | Draft | Controlled working foundation |
| L6 Interaction, State and Feedback Standard — Draft | Lead Architect | Draft | Controlled working foundation |
| L6 Foundation Controlled Authorization — 2026-09-16 | Lead Architect | Architecture Approved | Active controlled-authoring authorization |
| L6 Screen and Workspace Inventory — Draft | Lead Architect | Draft | 105-unit repository-wide planning baseline |
| L6 Persona Journey and Navigation Map — Draft | Lead Architect | Draft | Persona/task navigation composition |
| L6 Screen / Journey Traceability Register — Draft | Lead Architect | Draft | Full-corpus L3/L4 workspace-family traceability and per-slice L5 gate |
| L6 Experience Slice Register — Draft | Lead Architect | Draft | Thin locator for selected delivery experience slices; no new approval authority |
| L6 AI-First UI / UX Execution Plan — Draft | Lead Architect | Draft Active Execution Plan | Day-based accelerated UX archetype, workspace breadth and deep-journey delivery plan |
| L6 Days 3–5 Reference UX System — Draft | Lead Architect | Draft | Multi-domain reference-workspace proof using the established 99-workspace / 12-archetype foundation; no L0–L5 semantics changed |
| L6 Days 3–5 Reference UX Validation & Readiness Record | Lead Architect | Draft | Records reference-set counts, CI evidence, W-C02 regression result, evidence gaps and proceed verdict |
| L6 Days 6–8 Full ERP UX Breadth — Draft | Lead Architect | Draft | Governs meaningful shell-level UX representation and search discovery across all 99 canonical workspace families using the established 12-archetype/locked-visual baseline |
| L6 Days 6–8 Full ERP UX Breadth Validation & Readiness Record | Lead Architect | Draft | Records 99/99 breadth counts, CI evidence, maturity distribution, regression checks and phase verdict |
| W-C02 Student Workspace — Controlled Draft | Lead Architect | Architecture Approved by controlling proceed-gate decision; underlying document remains Draft-form | Approved W-C02 interaction package for controlled frontend implementation |
| L6 Implementation Review Framework | Lead Architect | Active supporting guidance | Reproducible experience-slice review cases and acceptance evidence |
| L6 UX Baseline Lock and Backend Proceed Decision — 2026-09-19 | Lead Architect | Locked / Historical Baseline | Preserved Days 1–13 baseline; closed-baseline current-state interpretation superseded by controlled workflow-realization expansion |
| L6 Full Workflow Mock-ERP Expansion Authorization — 2026-09-19 | Lead Architect | Architecture Approved | Current L6 programme authority; reopens L6 for 1,084-scenario UI realization and mock-operational ERP review |
| L6 Full Workflow Realization & Mock-ERP Execution Plan — Draft | Lead Architect | Draft Active Execution Plan | Governs full-corpus workflow realization, coherent mock data/state, discoverability and human review |
| L6 Workflow Realization Register — Draft | Lead Architect | Draft Expansion Register | Controlled 1,084-scenario UI-realization schema; population pending full-corpus audit |

### Active L6 Planning Baseline

The corrected L6 inventory contains:

- 98 L2-anchored business workspace families;
- 1 L3-derived `W-G00 Campus Operations Governance` workspace family covering final `L3-G-001…015` without redefining the review-only `L2-G00` artifact;
- **99 business workspace families** total;
- 6 shared L6 shell/task surfaces;
- **105 total draft inventory units**.

The traceability register reconciles the full **1,084 final L3 scenarios** to L6 workspace-family/shared presentation coverage and uses the canonical L3→L4 parent identity convention. Exact material L4 BH→L5 specification/evidence binding remains a required per-production-slice gate under the Locked L5 traceability methodology.

The W-C02 proceed-gate decision dated 2026-09-17 records `ARCHITECTURE APPROVED / SLICE READY / CONTROLLED FRONTEND IMPLEMENTATION AUTHORIZED` for its stated boundary. The delivery experience-slice register does not expand that approval and does not duplicate canonical BH→L5 evidence.

The Days 1–13 L6 UX baseline remains preserved by `00_Governance/L6_UX_Baseline_Lock_and_Backend_Proceed_Decision_2026-09-19.md`, but the active programme is now governed by `00_Governance/L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md`. L6 is reopened in a controlled manner for 1,084-scenario UI-realization mapping, 99-workspace mock-operational breadth, workflow-aware discovery and end-user review. The previous baseline is extended, not discarded.

---

# Architecture Summary

| Section | L2 Documents | Owner |
|----------|-------------:|-------|
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

| Item | Count / State |
|------|---------------|
| Core Architecture Documents | 6 |
| L1 Business Domains | 10 |
| L2 Workflow Documents | 107 |
| L3 Canonical Scenario Population | 1,084 |
| L3 Scenarios Locked / Active L4 Parents | 1,084 / 1,084 |
| Historical First-Wave Certified Parents | 48 |
| L4 Active Execution Scope | Full corpus — 1,084 Locked L3 parents |
| L4 Pilot Specifications Present | 3 |
| L5 Frozen Authoring Population | 105 specifications |
| L5 Broad Authoring | Authorized |
| L5 Historical Extracted Documents | 2 substantive artifacts + 2 temporary root compatibility pointers |
| L6 UI/UX Documents Present | 10 active governance/foundation/planning artifacts |
| L6 Business Workspace Families | 99 — Locked baseline |
| L6 Shared Surfaces | 6 — Locked baseline |
| L6 Architecture Coverage Units | 105 — Locked baseline |
| L6 Full-Corpus L3 Workspace Traceability | 1,084 / 1,084 reconciled at workspace-family/shared-presentation level |
| L6 Days 1–13 UX Baseline | Preserved historical locked baseline |
| L6 Active Programme | Controlled workflow-realization + mock-ERP expansion authorized 2026-09-19 |
| L6 Workflow Realization Register | Schema established; 0 / 1,084 rows populated pending Stage-2 audit |
| Active Architects | 5 |

---

**Document Status Legend**

| Status | Meaning |
|--------|---------|
| Draft | Under development |
| Proposed | Not yet in active use; reserved for documents submitted for review before drafting begins |
| Review | Under architecture review |
| Trust Certification Ready | Complete and certification-ready, pending final lock decision |
| Architecture Approved | Approved by architecture review, not yet locked |
| Approved | Not yet in active use; reserved for a general post-review approval state distinct from the Architecture-prefixed states below |
| Approved Baseline | Approved baseline state pending broader freeze controls |
| Architecture Baseline | Baseline architecture state used for controlled reference |
| Architecture Locked | Approved and frozen for architectural use |
| Certified & Locked | Certified final state and frozen |
| Locked | Approved and frozen |
| Deprecated | Not yet in active use; reserved for superseded documents kept for historical reference |
| Archived | Not yet in active use; reserved for retired documents removed from active governance |

> Status values in this repository are intentional and exact. Do not collapse them into a single generic state when updating index rows or review controls.
>
> This legend mirrors the canonical status vocabulary defined in `governance/constitution/Volume_2_Repository_Standards.md` §7, which is the single canonical source and governs if the two ever diverge. All **13** values are reproduced above, verbatim, in Volume 2's order.
>
> **As of 2026-08-07 L0, L1 and every L2 document are `Locked`.** L3 and L4 use their own governed scenario/specification lifecycle and are not globally implied Locked by the L0/L1/L2 decision.
>
> L5 is **not** covered by the L0/L1/L2 lock. Current L5 governance freezes a 105-specification authoring population and authorizes broad authoring under dependency-gated execution, but the Final L5 Corpus Lock has not been issued. Historical `L5-00` and `L5-01` remain non-canonical technical inputs requiring current-governance revalidation before adoption.
>
> The Days 1–13 L6 UX baseline remains preserved as a Locked historical baseline. Effective later on 2026-09-19, `L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md` became the current programme authority and explicitly reopened L6 under controlled amendment for full 1,084-scenario UI realization and mock-operational ERP review. No L0–L5 decision is reopened automatically.

---

# Recent Changes

**2026-09-19 — L6 controlled reopening for full workflow realization:** Preserved the completed Days 1–13 baseline but superseded its closed-programme interpretation. Authorized a full 1,084-scenario UI-realization audit, coherent fictional-institution mock dataset, deterministic frontend-only mock updates/reset, workflow-aware discovery, 99/99 browseable ERP workspaces and full end-user mock-ERP review before significant backend integration. Added the Full Workflow Realization & Mock-ERP Execution Plan and Workflow Realization Register schema.

---


**2026-09-19 — L6 UX baseline lock and backend proceed:** Locked the established 99-workspace + 6-shared-surface L6 UX baseline, 12 archetypes, shared navigation/state/handoff/reviewer-boundary rules, 81 shell implementations, 18 interactive fixture-backed workspaces and backend-readiness interaction evidence. Backend/integration work is authorized to proceed against this stable UX contract. `UX-REVIEWED`, released-contract integration, accessibility certification and production validation remain separate downstream evidence dimensions.

---


**2026-09-16 — L6 inventory, journey and full-corpus planning traceability:** Added the Screen and Workspace Inventory, Persona Journey and Navigation Map, and Screen / Journey Traceability Register. Exact final-L3 reconciliation identified and corrected a coverage gap for `L3-G-001…015` by introducing provisional L6 workspace key `W-G00 Campus Operations Governance` without redefining `L2-G00`. The corrected planning baseline is 99 business workspace families + 6 shared surfaces = 105 inventory units. All 1,084 final L3 scenarios now reconcile to an L6 workspace/shared presentation family at planning level. Exact material L4 BH→L5 evidence binding remains mandatory before a production L6 slice can be treated as ready.

---

**2026-09-16 — L6 controlled foundation authorization:** Recorded the Architecture Approved L6 foundation authorization after review of the implemented SutraERP visual direction. Controlled L6 production authoring may now proceed for defined slices. The approved direction establishes one shared persona-sensitive design system, Administrator Institution Pulse/priority-work hierarchy, Faculty teaching-day hierarchy without an enterprise work queue, Student semester/deadline hierarchy without an enterprise work queue, and a responsive light-theme-first shell. Permanent L6 numbering, complete screen corpus, exact token values, dark mode and several implementation-library choices remain deferred.

---

**2026-09-09 — L5 broad-authoring and repository-structure synchronization:** Broad authoring of the frozen 105-specification L5 corpus was formally authorized under dependency-gated parallel execution. The canonical production location is now `architecture/L5_Solution_Architecture/specifications/`; governance remains under `governance/`; substantive `L5-00` / `L5-01` historical extractions moved under `historical/`; temporary root compatibility pointers preserve older locked/historical references; and `adr/` will materialize only when the first canonical ADR exists.

---

**2026-09-09 — L5 planning/governance synchronization:** Recorded the representative pilot completion; 105-specification authoring population freeze; six pilot ADR candidate areas; five-person workload allocation at Varad 31 / Pallav 30 / Dhruv 20 / Devang 14 / Chetan 10; and six-stage dependency-gated parallel execution.

---

**2026-08-29 — L4 full-corpus execution authorization:** Superseded the 48-parent production-wave limit as the active execution model. L4 now inherits the complete 1,084-scenario locked L3 Phase 5 human allocation, with a Locked readiness baseline of 1,084 / 1,084 and a normal no-drift check before authoring. All A–J production folders are active repository structure; historical 48-parent records remain decision provenance only.

---

**2026-08-27 — L3/L4 synchronization and L4 production preparation:** Corrected stale placeholder language that previously stated no L3/L4 source architecture existed. Recorded the 1,084-scenario canonical L3 population, the 45 independently Architecture Approved G/H remediation scenarios, the 48 certified L4 parent baseline, Locked L4 governance/template/validation framework, separate `00_Pilot/` and canonical `01_Production/` workspaces, and the controlled 48-parent production-wave execution model. No L1/L2 business architecture was changed by this index synchronization.

---

**2026-08-07 — L0/L1/L2 lock:** Took the final lock decision for the business architecture levels. All 107 L2 documents moved to `Locked`. L0 and L1 were verified as already `Locked`. L3 and L5 were outside that lock decision.

---

**2026-08-05 — Core context alignment:** Confirmed the L0 Business Vision as `Locked`, retained the Dependency Map as `Active`, and aligned its document responsibility metadata to Dhruv as Lead Architect.

---

**2026-07-31 — Status reconciliation (INT-011/012/013/023):** Resolved repository status/index inconsistencies, aligned L2 ownership metadata, and synchronized the canonical status vocabulary with Repository Constitution Volume 2.

---

**Last Updated:** 2026-09-19

**Repository:** SutraERP

**Maintained By:** Dhruv