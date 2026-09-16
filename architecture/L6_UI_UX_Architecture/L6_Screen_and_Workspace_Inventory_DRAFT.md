# SutraERP L6 — Screen and Workspace Inventory — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** `00_Governance/L6_Foundation_Controlled_Authorization_2026-09-16.md`; existing L6 foundation drafts; Locked L3 canonical corpus (1,084 scenarios); L4 full-corpus parent/readiness baseline; approved L5 client architecture including `L5-G-01-001`, `L5-G-01-002`, `L5-G-01-003`; Architecture Index; Dependency Map.  
**Downstream:** Persona journey/navigation map, screen-journey traceability register, and SutraERP frontend implementation slices.  

> This is a controlled inventory draft. It does **not** allocate permanent L6 specification IDs, lock final navigation, create business ownership, define authorization, or assert that every row is a top-level page.

---

## 1. Purpose

Define the first repository-wide L6 inventory of user-facing workspace families and shared shell/task surfaces needed to realize approved SutraERP architecture.

This inventory prevents the frontend from growing through ad hoc page creation.

It is intentionally **workspace-family complete before it is final-screen complete**.

---

## 2. Derivation Rules

1. L6 derives interaction/presentation from approved L5 and the user-visible semantics of L4/L3.
2. Canonical business ownership remains upstream.
3. One L3 scenario does **not** automatically equal one screen.
4. One L2 workflow document does **not** automatically equal one navigation item.
5. Multiple scenarios may compose into one sustained workspace where that matches the user's job.
6. A complex scenario may later require several bounded task/detail surfaces.
7. A route, visible control, hidden control or disabled control never establishes authorization.
8. Exact lifecycle states, decisions, approvals and terminology must be inherited from upstream sources in individual L6 specifications.
9. The current frontend is implementation evidence for the approved visual language, not the source of truth for scope.
10. Draft keys in this file (`S-*`, `W-*`) are non-canonical row keys only. They are not permanent L6 IDs.

---

## 3. Coverage Basis

| Source | Coverage used by this draft |
|---|---:|
| Locked canonical L3 scenarios | 1,084 |
| Authorized L4 parent identities | 1,084 |
| Active L2 workflow/foundation areas represented as business workspace families | 98 |
| Shared L6 shell/task surfaces | 6 |
| **Draft inventory units** | **104** |

The 98 business workspace families cover the active non-review workflow/foundation areas:

`A01–A10`, `B01–B05`, `C01–C08`, `D01–D07`, `E01–E07`, `F00–F14`, `G01–G10`, `H01–H10`, `I01–I16`, `J01–J10`.

L2 review documents and architecture-review artifacts do not become product workspaces merely because they exist in the repository.

Exact final L3 IDs, L4 specification references and specific L5 contract/solution dependencies will be resolved row-by-row in the dedicated traceability register. A row is therefore **not implementation-authorized solely because it appears here**.

---

## 4. Shared L6 Surfaces

| Key | Working surface | Primary upstream basis | L6 responsibility |
|---|---|---|---|
| `S-01` | Persona Home | L6 Foundation Authorization; L6 Application Shell draft | Administrator: Institution Pulse + intervention work; Faculty: Teaching Pulse + next-class/day; Student: My Semester + up-next/due-soon. |
| `S-02` | Global Search / Command Discovery | `L2-H03`; L3-H Enterprise Directory & Search family; L6 shell draft | Discover authorized entities/workspaces without treating search visibility as mutation authority. |
| `S-03` | Notifications | `L2-H02`; L3-H Notification family; L6 shell draft | Present governed notification information; task/work-item semantics remain separate. |
| `S-04` | Institution / Academic Context | `L5-G-01-003`; `L2-B01`; `L2-B05`; L6 shell draft | Present/select permitted context while preserving server-authoritative validation and stale-state invalidation. |
| `S-05` | Account / Authentication Entry | `L2-A03`; L3-A Authentication family; applicable L5 identity/security architecture | Sign-in, recovery, MFA and provider experiences without moving authentication authority into the client. |
| `S-06` | Task / Work Entry | `L2-A05`; L3-A human-decision-task scenario; L6 Foundation Authorization | Provide task/work entry where upstream workflows create human decisions. Not the default mental model for Faculty or Student home. |

---

# 5. Business Workspace Inventory

## 5.1 Section A — Platform Foundation

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-A01` | Tenant Administration | `L2-A01` — Platform & Tenant Workflows | Tenant establishment, availability, restriction, archival and governed configuration. |
| `W-A02` | Person & Identity | `L2-A02` — Person & Identity Workflows | Person establishment, duplicate resolution, correction, onboarding and identity closure. |
| `W-A03` | Authentication & Sign-in | `L2-A03` — Authentication Workflows | Sign-in, credential recovery, MFA and authentication-provider interaction. |
| `W-A04` | Access & Responsibility | `L2-A04` — Authorization & Policy Workflows | Responsibility/access assignment, delegation, temporary/emergency access, SoD and policy presentation. |
| `W-A05` | Workflow & Human Tasks | `L2-A05` — Workflow Engine Workflows | Workflow definition/execution visibility, recovery and human task resolution. |
| `W-A06` | Event Governance | `L2-A06` — Event Infrastructure Workflows | Event contracts, subscriptions, delivery, replay and compatible-version governance. |
| `W-A07` | Audit & Evidence | `L2-A07` — Audit Infrastructure Workflows | Evidence discovery, investigation, integrity verification, retention, archive and restored access. |
| `W-A08` | Capability / Module Registry | `L2-A08` — Module Registry & Configuration Workflows | Capability/module registration, change, suspension, retirement and dependency governance. |
| `W-A09` | Master Data | `L2-A09` — Master Data Workflows | Dataset registration, publication, versioning, import/export, localization, synchronization and retirement. |
| `W-A10` | Operational Readiness | `L2-A10` — Operational Readiness Governance & Certification Workflows | Assessment, evidence review, improvement, certification, suspension, supersession and closure. |

## 5.2 Section B — Institution & Structure

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-B01` | Institution Administration | `L2-B01` — Institution Workflows | Institution establishment, activation, profile/policy change, suspension/restoration and archive. |
| `W-B02` | Organisation Structure & Positions | `L2-B02` — Organisation Workflows | Organisation units, structure/governance and position-definition lifecycle. |
| `W-B03` | Institutional Campus Registry | `L2-B03` — Campus Workflows | Campus registration, activation, modification, suspension/restoration, decommissioning and history. |
| `W-B04` | Curriculum Framework | `L2-B04` — Curriculum Framework Workflows | Programme/curriculum and school academic-model establishment, revision and retirement. |
| `W-B05` | Academic Calendar | `L2-B05` — Academic Calendar Workflows | Calendar creation, assignment, revision, exceptions, impact review, year closure and archive. |

## 5.3 Section C — Student Lifecycle

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-C01` | Admissions | `L2-C01` — Admissions Workflows | Candidate/admission activation, cancellation, withdrawal, revocation, readmission and document decisions. |
| `W-C02` | Student Workspace | `L2-C02` — Student Lifecycle & Activation Workflows | Student activation, enrollment/profile/context changes, suspension/restoration, transfer and lifecycle closure. |
| `W-C03` | Academic Registration | `L2-C03` — Academic Registration Workflows | Registration windows, registration/add-drop/cancellation, backlog registration, holds and finalization. |
| `W-C04` | Academic Progression & Credits | `L2-C04` — Student Academic Progression Workflows | Progression, credit ledger, probation/detention, transfer, programme migration, break/rejoin and eligibility. |
| `W-C05` | Qualification & Exit | `L2-C05` — Qualification Completion & Student Exit Workflows | Completion, conferment, exit variants, clearance, deferral, correction/revocation and final record closure. |
| `W-C06` | Guardian Relationships | `L2-C06` — Guardian Workflows | Guardian establishment, custody/consent/communication/financial responsibility and relationship lifecycle. |
| `W-C07` | Scholarship Eligibility | `L2-C07` — Scholarship Eligibility Workflows | Scholarship definition/policy presentation, eligibility, reassessment, renewal and financial-aid handoff. |
| `W-C08` | Alumni Transition | `L2-C08` — Alumni Transition Workflows | Exit-to-alumni transition, correction and archive. |

## 5.4 Section D — Academic Delivery

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-D01` | Course Offerings | `L2-D01` — Course & Course Offering Workflows | Offering establishment, capacity/eligibility/quota/window governance, waitlists, revision and archive. |
| `W-D02` | Faculty Academic Assignment | `L2-D02` — Faculty Academic Assignment Workflows | Faculty academic-assignment establishment and governed operational change. |
| `W-D03` | Timetable | `L2-D03` — Timetable Workflows | Operational timetable creation, revision, historical retirement and user schedule presentation. |
| `W-D04` | Teaching Sessions & Attendance | `L2-D04` — Teaching Session & Attendance Workflows | Teaching-session outcome, attendance capture/finalization/correction and syllabus coverage. |
| `W-D05` | Academic Monitoring | `L2-D05` — Academic Monitoring Workflows | Monitoring outcomes, risk assessment, exceptions, intervention, governance, closure and reopening. |
| `W-D06` | Flexible Learning / Electives / MOOCs | `L2-D06` — Open Electives & MOOCs Workflows | Opportunities, enrollment/seat allocation, external learning, completion, verification and credit recognition. |
| `W-D07` | Internal Assessment | `L2-D07` — Internal Assessment Workflows | Blueprints, assessment windows, submission, moderation, finalization, correction and archive. |

## 5.5 Section E — People & Workforce

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-E01` | Workforce Engagement | `L2-E01` — Workforce & Engagement Workflows | Engagement onboarding/change/transfer, suspension, deputation/long leave, closure and rejoining. |
| `W-E02` | Position & Assignment | `L2-E02` — Position & Assignment Workflows | Position occupancy, assignment/reassignment, delegated charge, reporting, conflicts, expiry and bulk work. |
| `W-E03` | Faculty Workforce | `L2-E03` — Faculty Workforce Workflows | Faculty establishment, capability/qualification/eligibility, availability/workload and closure. |
| `W-E04` | HR Services & Cases | `L2-E04` — HR Workflows | Onboarding, HR requests, grievance, promotion/transfer, certificates, renewal, separation and probation. |
| `W-E05` | Leave Management | `L2-E05` — Leave Management Workflows | Leave requests/decisions, accrual, consumption, cancellation, adjustment, carry-forward and exceptions. |
| `W-E06` | Payroll | `L2-E06` — Payroll Workflows | Payroll processing, adjustments, retro pay, final settlement, off-cycle payroll and governance. |
| `W-E07` | Employee Profile | `L2-E07` — Employee Profile Workflows | Profile, qualifications, certifications, skills, documents, corrections, archive/restore and duplicate resolution. |

## 5.6 Section F — Finance

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-F00` | Finance Control & Configuration | `L2-F00` — Finance Domain Model & Foundation | Finance configuration/policy context and cross-finance orientation without redefining rules. |
| `W-F01` | Student Fees & Charges | `L2-F01` — Student Charge & Fee Management | Charges, revisions/adjustments, suspension/resumption, disputes, installments and closure. |
| `W-F02` | Payments & Receipts | `L2-F02` — Payment Management Architecture | Payments, allocations, unidentified receipts, failure recovery, reversals and bulk collection. |
| `W-F03` | Refunds | `L2-F03` — Refund Management | Refund request/decision, completion, withdrawal, failure resolution, reversal and history. |
| `W-F04` | Financial Adjustments | `L2-F04` — Financial Adjustment Management | Credit/debit notes, write-offs, reversals and corrected successor adjustments. |
| `W-F05` | General Ledger | `L2-F05` — Accounting & General Ledger Management | Ledger posting, journals, batches, reversals, reconciliation, trial balance, close and audit history. |
| `W-F06` | Budgets & Cost Centres | `L2-F06` — Budget & Cost Center Management | Budget proposals/revisions/allocations, budget control, cost centres and period closure. |
| `W-F07` | Accounts Receivable | `L2-F07` — Accounts Receivable Management | Receivable recognition, dunning, settlement, write-off and reopening. |
| `W-F08` | Accounts Payable | `L2-F08` — Accounts Payable Management | Vendor liabilities, settlement, disputes/holds, adjustment/reversal and reconciliation. |
| `W-F09` | Financial Periods & Closing | `L2-F09` — Financial Period & Closing | Year/period establishment, locking, year-end close, controlled reopening and archive. |
| `W-F10` | Treasury & Banking | `L2-F10` — Treasury & Banking | Bank accounts, payment execution, reconciliation, liquidity, transfers and uncertain transaction resolution. |
| `W-F11` | Financial Aid | `L2-F11` — Scholarship & Financial Aid | Aid allocation, disbursement/charge adjustment, renewal, revocation/recovery and reconciliation. |
| `W-F12` | Asset Accounting | `L2-F12` — Asset Financial Accounting Workflows | Capitalization, depreciation, revaluation, impairment, disposal accounting and write-off. |
| `W-F13` | Financial Reports & Analytics | `L2-F13` — Financial Reporting & Analytics Workflows | Statements, statutory/management reports, KPIs, correction/supersession and evidence reconstruction. |
| `W-F14` | Procurement Finance | `L2-F14` — Procurement Finance Consumption Workflows | Budget commitments, matching, supplier-liability recognition and procurement financial closure. |

## 5.7 Section G — Campus Operations

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-G01` | Facilities | `L2-G01` — Facilities Management Workflows | Facility records and operational facility workflows. |
| `W-G02` | Laboratories | `L2-G02` — Laboratory Management Workflows | Laboratory operations and governed laboratory context. |
| `W-G03` | Hostel | `L2-G03` — Hostel Workflows | Hostel accommodation and governed hostel operations. |
| `W-G04` | Transport | `L2-G04` — Transport Workflows | Institutional transport operations and user-relevant transport context. |
| `W-G05` | Library | `L2-G05` — Library Workflows | Library membership, circulation/resources and governed library operations. |
| `W-G06` | Health / Medical Services | `L2-G06` — Medical Workflows | Institutional medical-service workflows and supported service interactions. |
| `W-G07` | Security Operations | `L2-G07` — Security Operations Workflows | Campus-security operations and supported incident/access context. |
| `W-G08` | Housekeeping & Service Governance | `L2-G08` — Housekeeping Operations & Service Governance Workflows | Service requests, operational governance and completion context. |
| `W-G09` | Teacher Guardian Scheme | `L2-G09` — Teacher Guardian Scheme (TGS) Workflows | Teacher-guardian interactions and governed student-support context. |
| `W-G10` | Infrastructure Management | `L2-G10` — Infrastructure Management Workflows | Institutional infrastructure operations and governed infrastructure records. |

## 5.8 Section H — Institutional Services & Intelligence

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-H01` | APAAR & DigiLocker | `L2-H01` — APAAR & DigiLocker Workflows | Learner linking, credential publication/consent/revocation/republication and reconciliation. |
| `W-H02` | Notifications | `L2-H02` — Notification Workflows | Notification outcomes, schedules, templates, preferences, emergency communication and evidence. |
| `W-H03` | Enterprise Directory & Search | `L2-H03` — Enterprise Directory & Search Workflows | Enterprise discovery/search projections, synchronization, recovery and configuration. |
| `W-H04` | Placement | `L2-H04` — Placement Workflows | Employers, opportunities, applications, selection, offers/joining, internships and outcome reporting. |
| `W-H05` | Research | `L2-H05` — Research Workflows | Proposals, projects, grants, ethics, outputs, IP, collaborations, consultancy and portfolio governance. |
| `W-H06` | Alumni Engagement | `L2-H06` — Alumni Workflows | Membership/profile, chapters, networking, mentoring, contribution, recognition and donation intent. |
| `W-H07` | Enterprise Dashboards | `L2-H07` — Enterprise Dashboard Workflows | Dashboard requirements/templates, availability, sharing, versioning, retirement and widget governance. |
| `W-H08` | Statutory / Regulatory Reporting | `L2-H08` — NEP, NAAC & AISHE Reporting Workflows | Frameworks, collection plans, regulatory snapshots, packages, submission acknowledgement and corrections. |
| `W-H09` | Accreditation & Compliance | `L2-H09` — Accreditation & Compliance Workflows | Accreditation cycles, obligations, evidence, audits/assessments, findings/CAPA and readiness. |
| `W-H10` | External Education Integrations | `L2-H10` — External Education Integration Workflows | Providers/connectors/trust, synchronization, external enrollment/progress/credit/credential exchange and reconciliation. |

## 5.9 Section I — Examination

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-I01` | Examination Policy & Configuration | `L2-I01` — Examination Policy & Configuration Workflows | Exam policy/configuration experiences and governed setup context. |
| `W-I02` | Examination Sessions | `L2-I02` — Examination Session Management Workflows | Examination-session lifecycle and operational session context. |
| `W-I03` | Examination Planning & Scheduling | `L2-I03` — Examination Planning & Scheduling Workflows | Exam planning/scheduling and governed revision context. |
| `W-I04` | Examination Registration | `L2-I04` — Examination Registration Workflows | Examination registration and governed registration outcomes. |
| `W-I05` | Hall Tickets | `L2-I05` — Hall Ticket Management Workflows | Hall-ticket generation/availability/correction and supported candidate access. |
| `W-I06` | Practical & Viva Conduct | `L2-I06` — Practical & Viva Examination Conduct Workflows | Practical/viva conduct, attendance/evaluation context and operational actions. |
| `W-I07` | Theory Examination Conduct | `L2-I07` — Theory Examination Conduct Workflows | Theory examination conduct and supported operational actions. |
| `W-I08` | Marks & Evaluation | `L2-I08` — Marks Collection & Evaluation Workflows | Marks collection, evaluator activity, moderation/verification and governed outcomes. |
| `W-I09` | Result Processing | `L2-I09` — Result Processing & Computation Workflows | Result computation, validation/reconciliation and governed processing status. |
| `W-I10` | Result Publication | `L2-I10` — Result Publication Workflows | Result publication, candidate visibility and governed publication outcomes. |
| `W-I11` | Revaluation & Rechecking | `L2-I11` — Revaluation & Rechecking Workflows | Revaluation/rechecking requests and governed review outcomes. |
| `W-I12` | Supplementary / Backlog / Improvement Exams | `L2-I12` — Supplementary / Backlog / Improvement Examination Execution Workflows | Supplementary/backlog/improvement examination execution and outcomes. |
| `W-I13` | Academic Completion & CGPA Evidence | `L2-I13` — Academic Completion, CGPA & Completion Evidence Workflows | CGPA/completion evidence and examination-owned completion projections. |
| `W-I14` | Malpractice & Examination Committee | `L2-I14` — Malpractice & Examination Committee Workflows | Malpractice cases, committee review/decisions, evidence and history. |
| `W-I15` | Examination Exceptions & Contingency | `L2-I15` — Examination Exception & Contingency Workflows | Examination contingencies, exceptions, recovery and governed resolution. |
| `W-I16` | Examination Closure & Archive | `L2-I16` — Examination Session Closure & Archival Workflows | Session closure, archive and historical examination evidence. |

## 5.10 Section J — Enterprise Resource Management

`L2-J00` is architecture/governance and is not converted into a product workspace.

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-J01` | Procurement Planning & Sourcing | `L2-J01` — Procurement Planning & Sourcing Workflows | Procurement demand/planning, sourcing and governed sourcing outcomes. |
| `W-J02` | Vendors & Suppliers | `L2-J02` — Vendor & Supplier Management Workflows | Supplier onboarding, records, status and governed lifecycle. |
| `W-J03` | Purchase Requisitions & Approvals | `L2-J03` — Purchase Requisition & Approval Workflows | Requisitions, review/approval routing and governed outcomes. |
| `W-J04` | Purchase Orders | `L2-J04` — Purchase Order Management | Purchase-order creation/change/status and governed lifecycle. |
| `W-J05` | Goods Receipt & Inspection | `L2-J05` — Goods Receipt, Inspection & Acceptance | Receipt, inspection, acceptance/rejection and evidence. |
| `W-J06` | Inventory & Warehouse | `L2-J06` — Inventory & Warehouse Management | Warehouse/inventory records, stock position and governed operations. |
| `W-J07` | Stock Operations & Distribution | `L2-J07` — Stock Operations & Distribution | Stock issue/transfer/distribution and governed stock movement. |
| `W-J08` | Asset Lifecycle | `L2-J08` — Asset Lifecycle Management | Operational asset establishment, assignment/use and lifecycle outcomes. |
| `W-J09` | Physical Verification & Reconciliation | `L2-J09` — Physical Verification, Audit & Reconciliation | Physical verification, discrepancy, audit/reconciliation and governed closure. |
| `W-J10` | Disposal / Scrap / Write-Off | `L2-J10` — Disposal, Scrap & Write-Off | Asset/stock disposal, scrap/write-off decisions and governed completion. |

---

## 6. Approved Persona Home Composition

### Administrator

`Context → Institution Pulse → Attention / exceptions → Priority Work / true work queue → Recent work → Day / lower-priority institutional updates`

### Faculty

`Teaching context → Teaching Pulse → Next class → Today's Teaching / Needs Attention → Recent teaching work → Day schedule / lower-priority updates`

### Student

`Academic context → My Semester → Up Next → Due Soon → Academic progress / recent destinations → Day schedule / lower-priority campus updates`

A work queue is valid for administrator/operational task contexts where upstream workflows create human decisions. It shall not be imposed as the default interaction metaphor for Faculty or Student.

---

## 7. Navigation Implications — Not Yet a Navigation Decision

This inventory is intentionally larger than any sensible primary navigation.

Therefore:

- inventory row ≠ navigation item;
- several workspace families may be grouped behind one user-facing navigation family;
- persona/task-specific navigation may expose different subsets;
- Home, Search, recent work and contextual links may bypass the primary hierarchy;
- specialist Platform/Foundation configuration must not dominate ordinary Student or Faculty navigation;
- navigation grouping never changes canonical business ownership.

The next L6 artifact shall derive the persona journey and navigation model from this inventory.

---

## 8. Required Scenario-Level Decomposition

Before any workspace family becomes production-ready, its L6 specification or traceability row must resolve:

1. exact final L3 scenario IDs;
2. applicable L4 behavior specifications;
3. applicable L5 solution/contract/security/client dependencies;
4. supported actors/personas;
5. primary user outcome;
6. required overview/list/detail/task/history surfaces;
7. authoritative states and decisions that must be presented;
8. loading/empty/pending/rejection/indeterminate/unavailable behavior;
9. scope/context behavior;
10. accessibility and responsive requirements;
11. whether the surface is a navigation destination, contextual destination or task-only surface.

No frontend production slice may invent these details where they remain unresolved.

---

## 9. High-Risk Decomposition Areas

- **Section G — Campus Operations:** the largest L3 scenario population; scenario volume must not become hundreds of unrelated pages.
- **Section I — Examination:** high-consequence lifecycle/evidence flows require policy, registration, conduct, evaluation, processing, publication, review, exception and closure semantics to remain distinct where upstream does.
- **Finance + ERM:** cross-domain handoffs may be composed in user journeys but must not merge canonical ownership.
- **Student + Guardian:** self-service exposure must follow actor/authority evidence; not every Student Lifecycle workspace is automatically student-visible.
- **Platform Foundation:** most workspaces are specialist administrative/governance surfaces, not ordinary Faculty or Student navigation.

---

## 10. Inventory Gate Result

This draft passes the first repository-wide inventory gate because:

- all 98 active workflow/foundation areas are represented once at workspace-family level;
- L2 review/governance-only artifacts are not converted into product pages;
- all ten L1 sections are covered;
- shared L6 shell surfaces remain separate from canonical business ownership;
- persona Home asymmetry matches the controlled L6 foundation authorization;
- no row claims authorization, business-rule or lifecycle ownership;
- exact scenario-level trace gaps are recorded rather than guessed.

**Result:** PASS at workspace-family coverage level. Scenario-to-screen decomposition and exact traceability remain required before production slice authorization.

---

## 11. Next Controlled Artifacts

1. `L6_Persona_Journey_and_Navigation_Map_DRAFT.md`
2. `L6_Screen_Journey_Traceability_Register_DRAFT.md`

The journey/navigation map shall use this inventory as input. The traceability register shall resolve exact L3/L4/L5 provenance before production implementation slices are authorized.

---

**Draft verdict:** repository-wide workspace-family inventory established; not a final screen corpus and not a Final L6 Corpus Lock.
