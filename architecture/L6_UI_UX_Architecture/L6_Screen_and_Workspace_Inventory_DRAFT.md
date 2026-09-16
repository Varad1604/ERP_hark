# SutraERP L6 — Screen and Workspace Inventory — Draft

**Version:** 0.1.1  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** `00_Governance/L6_Foundation_Controlled_Authorization_2026-09-16.md`; existing L6 foundation drafts; Locked L3 canonical corpus (1,084 scenarios); L4 full-corpus parent/readiness baseline; approved L5 client architecture including `L5-G-01-001`, `L5-G-01-002`, `L5-G-01-003`; Architecture Index; Dependency Map.  
**Downstream:** `L6_Persona_Journey_and_Navigation_Map_DRAFT.md`; `L6_Screen_Journey_Traceability_Register_DRAFT.md`; production L6 slice specifications; SutraERP frontend implementation slices.  

> This is a controlled inventory draft. It does **not** allocate permanent L6 specification IDs, lock final navigation, create business ownership, define authorization, or assert that every row is a top-level page.

---

## 1. Purpose

Define the repository-wide L6 inventory of user-facing workspace families and shared shell/task surfaces required to realize approved SutraERP architecture without allowing frontend implementation to grow through ad hoc page creation.

The inventory is **workspace-family complete before it is final-screen complete**.

---

## 2. Derivation Rules

1. L6 derives interaction/presentation from approved L5 and user-visible L4/L3 semantics.
2. Canonical business ownership remains upstream.
3. One L3 scenario does not automatically equal one screen.
4. One L2 workflow document does not automatically equal one navigation item.
5. Multiple scenarios may compose into one sustained workspace where that matches the user's job.
6. A complex scenario may later require several bounded task/detail surfaces.
7. Routes and visible/hidden/disabled controls never establish authorization.
8. Exact lifecycle states, decisions, approvals and terminology remain upstream-owned.
9. The frontend is implementation evidence, not the source of truth for scope.
10. Draft keys (`S-*`, `W-*`) are non-canonical row keys only.

---

## 3. Coverage Basis

The first inventory pass used the active non-review L2 workflow/foundation areas and produced 98 business workspace families. Exact final-L3 reconciliation then identified a separate canonical Campus Operations Governance scenario family (`L3-G-001…015`) that is not represented by `L2-G01…G10` and must not be conflated with the review-only `L2-G00` document.

The corrected controlled baseline is:

| Source | Coverage used by this draft |
|---|---:|
| Locked canonical L3 scenarios | 1,084 |
| Corresponding L4 parent/specification identities | 1,084 |
| L2-anchored business workspace families | 98 |
| Additional L3-derived Campus Operations Governance workspace family | 1 |
| **Business workspace families** | **99** |
| Shared L6 shell/task surfaces | 6 |
| **Draft inventory units** | **105** |

The 98 L2-anchored families cover:

`A01–A10`, `B01–B05`, `C01–C08`, `D01–D07`, `E01–E07`, `F00–F14`, `G01–G10`, `H01–H10`, `I01–I16`, `J01–J10`.

The additional L3-derived workspace is:

`W-G00 — Campus Operations Governance`, sourced from final `L3-G-001…015`.

`W-G00` does **not** redefine `L2-G00 Campus Operations Review`, create a new L2 owner, or alter locked Section G ownership. It is an L6 presentation/workspace family required by canonical L3 outcomes.

L2 review documents and architecture-review artifacts do not become product workspaces merely because they exist in the repository.

---

## 4. Shared L6 Surfaces

| Key | Working surface | Primary upstream basis | L6 responsibility |
|---|---|---|---|
| `S-01` | Persona Home | L6 Foundation Authorization; applicable L3/L4 projections | Administrator: Institution Pulse + intervention work; Faculty: Teaching Pulse + next-class/day; Student: My Semester + up-next/due-soon; other profiles use responsibility-relevant composition. |
| `S-02` | Global Search / Command Discovery | `L3-H-017…021`; H03 | Discover permitted entities/workspaces without treating visibility as authority. |
| `S-03` | Notifications | `L3-H-010…016`; H02 | Present governed notification information; notification and task semantics remain distinct. |
| `S-04` | Institution / Academic Context | A01/B01/B05 semantics; `L5-G-01-003`; A04 isolation | Present/select permitted context while preserving server-authoritative validation and state invalidation. |
| `S-05` | Account / Authentication Entry | `L3-A-007…009`, `L3-A-055`; applicable D01 architecture | Sign-in/recovery/MFA/provider experience without moving authentication authority to the client. |
| `S-06` | Task / Work Entry | `L3-A-058` plus task-owning domain context | Provide true human-task entry where upstream workflows create tasks; not the default mental model for Faculty or Student home. |

---

# 5. Business Workspace Inventory

## 5.1 Section A — Platform Foundation

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-A01` | Tenant Administration | `L2-A01` | Tenant establishment, availability, restriction, archival and governed configuration. |
| `W-A02` | Person & Identity | `L2-A02` | Person establishment, reconciliation, correction, onboarding and identity closure. |
| `W-A03` | Authentication & Sign-in | `L2-A03` | Sign-in, credential recovery, MFA and provider interaction. |
| `W-A04` | Access & Responsibility | `L2-A04` | Responsibility/access assignment, delegation, temporary/emergency access, SoD and policy presentation. |
| `W-A05` | Workflow & Human Tasks | `L2-A05` | Workflow definition/execution visibility, recovery and human-task resolution. |
| `W-A06` | Event Governance | `L2-A06` | Event contracts, subscriptions, delivery, replay and compatibility governance. |
| `W-A07` | Audit & Evidence | `L2-A07` | Evidence discovery, investigation, integrity, retention, archive and restored access. |
| `W-A08` | Capability / Module Registry | `L2-A08` | Capability/module registration, change, suspension, retirement and dependency governance. |
| `W-A09` | Master Data | `L2-A09` | Dataset registration, publication, versioning, import/export, localization, synchronization and retirement. |
| `W-A10` | Operational Readiness | `L2-A10` | Assessment, evidence review, improvement, certification, suspension, supersession and closure. |

## 5.2 Section B — Institution & Structure

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-B01` | Institution Administration | `L2-B01` | Institution establishment, activation, profile/policy change, suspension/restoration and archive. |
| `W-B02` | Organisation Structure & Positions | `L2-B02` | Organisation units, structure/governance and position-definition lifecycle. |
| `W-B03` | Institutional Campus Registry | `L2-B03` | Campus registration, activation, modification, suspension/restoration, decommissioning and history. |
| `W-B04` | Curriculum Framework | `L2-B04` | Programme/curriculum and school academic-model establishment, revision and retirement. |
| `W-B05` | Academic Calendar | `L2-B05` | Calendar creation, assignment, revision, exceptions, impact review, year closure and archive. |

## 5.3 Section C — Student Lifecycle

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-C01` | Admissions | `L2-C01` | Candidate/admission activation, cancellation, withdrawal, revocation, readmission and document decisions. |
| `W-C02` | Student Workspace | `L2-C02` | Student activation, enrollment/profile/context changes, suspension/restoration, transfer and lifecycle closure. |
| `W-C03` | Academic Registration | `L2-C03` | Registration windows, registration/add-drop/cancellation, backlog registration, holds and finalization. |
| `W-C04` | Academic Progression & Credits | `L2-C04` | Progression, credit ledger, probation/detention, transfer, migration, break/rejoin and eligibility. |
| `W-C05` | Qualification & Exit | `L2-C05` | Completion, conferment, exit variants, clearance, deferral, correction/revocation and record closure. |
| `W-C06` | Guardian Relationships | `L2-C06` | Relationship establishment, custody, consent, communication, financial responsibility and lifecycle. |
| `W-C07` | Scholarship Eligibility | `L2-C07` | Scholarship availability/policy presentation, eligibility, reassessment, renewal and aid handoff. |
| `W-C08` | Alumni Transition | `L2-C08` | Exit-to-alumni transition, correction and archive. |

## 5.4 Section D — Academic Delivery

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-D01` | Course Offerings | `L2-D01` | Offering establishment, capacity/eligibility/quota/window governance, waitlists, revision and archive. |
| `W-D02` | Faculty Academic Assignment | `L2-D02` | Faculty academic-assignment establishment and governed operational change. |
| `W-D03` | Timetable | `L2-D03` | Operational timetable creation/revision/history and user schedule presentation. |
| `W-D04` | Teaching Sessions & Attendance | `L2-D04` | Teaching-session outcome, attendance capture/finalization/correction and syllabus coverage. |
| `W-D05` | Academic Monitoring | `L2-D05` | Monitoring outcomes, risk, exceptions, intervention, governance, closure and reopening. |
| `W-D06` | Flexible Learning / Electives / MOOCs | `L2-D06` | Opportunities, enrollment/seat allocation, external learning, completion, verification and credit recognition. |
| `W-D07` | Internal Assessment | `L2-D07` | Blueprints, windows, submission, moderation, finalization, correction and archive. |

## 5.5 Section E — People & Workforce

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-E01` | Workforce Engagement | `L2-E01` | Engagement onboarding/change/transfer, suspension, deputation/long leave, closure and rejoining. |
| `W-E02` | Position & Assignment | `L2-E02` | Occupancy, assignment/reassignment, delegated charge, reporting, conflicts, expiry and bulk work. |
| `W-E03` | Faculty Workforce | `L2-E03` | Faculty establishment, capability/qualification/eligibility, availability/workload and closure. |
| `W-E04` | HR Services & Cases | `L2-E04` | Onboarding, HR requests, grievance, promotion/transfer, certificates, renewal, separation and probation. |
| `W-E05` | Leave Management | `L2-E05` | Leave requests/decisions, accrual, consumption, cancellation, adjustment, carry-forward and exceptions. |
| `W-E06` | Payroll | `L2-E06` | Payroll processing, adjustments, retro pay, final settlement, off-cycle payroll and governance. |
| `W-E07` | Employee Profile | `L2-E07` | Profile, qualifications, certifications, skills, documents, corrections, archive/restore and duplicate resolution. |

## 5.6 Section F — Finance

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-F00` | Finance Control & Configuration | `L2-F00` | Finance configuration/policy context and cross-finance orientation without redefining rules. |
| `W-F01` | Student Fees & Charges | `L2-F01` | Charges, revisions, disputes, installments and closure. |
| `W-F02` | Payments & Receipts | `L2-F02` | Payments, allocations, unidentified receipts, failure recovery, reversals and bulk collection. |
| `W-F03` | Refunds | `L2-F03` | Refund request/decision, completion, withdrawal, failure resolution, reversal and history. |
| `W-F04` | Financial Adjustments | `L2-F04` | Credit/debit notes, write-offs, reversals and corrected successor adjustments. |
| `W-F05` | General Ledger | `L2-F05` | Ledger posting, journals, batches, reversals, reconciliation, trial balance, close and history. |
| `W-F06` | Budgets & Cost Centres | `L2-F06` | Budget proposals/revisions/allocations, control, cost centres and period closure. |
| `W-F07` | Accounts Receivable | `L2-F07` | Receivable recognition, dunning, settlement, write-off and reopening. |
| `W-F08` | Accounts Payable | `L2-F08` | Vendor liabilities, settlement, disputes/holds, adjustment/reversal and reconciliation. |
| `W-F09` | Financial Periods & Closing | `L2-F09` | Year/period establishment, locking, year-end close, reopening and archive. |
| `W-F10` | Treasury & Banking | `L2-F10` | Bank accounts, payment execution, reconciliation, liquidity, transfers and uncertain transactions. |
| `W-F11` | Financial Aid | `L2-F11` | Aid allocation, disbursement/charge adjustment, renewal, revocation/recovery and reconciliation. |
| `W-F12` | Asset Accounting | `L2-F12` | Capitalization, depreciation, revaluation, impairment, disposal accounting and write-off. |
| `W-F13` | Financial Reports & Analytics | `L2-F13` | Statements, statutory/management reports, KPIs, correction/supersession and reconstruction. |
| `W-F14` | Procurement Finance | `L2-F14` | Budget commitments, matching, supplier-liability recognition and procurement financial closure. |

## 5.7 Section G — Campus Operations

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-G00` | Campus Operations Governance | `L3-G-001…015` | Campus readiness, operational review, governance findings/risks/actions, SLA/KPI/maturity review and governance closure. This L3-derived workspace does not redefine `L2-G00`. |
| `W-G01` | Facilities | `L2-G01` | Facility services, maintenance and operational facility workflows. |
| `W-G02` | Laboratories | `L2-G02` | Laboratory readiness, operations, equipment, safety and governed laboratory context. |
| `W-G03` | Hostel | `L2-G03` | Accommodation, residency and governed hostel services. |
| `W-G04` | Transport | `L2-G04` | Institutional transport operations and user-relevant transport context. |
| `W-G05` | Library | `L2-G05` | Membership, catalogue/circulation/resources and library operations. |
| `W-G06` | Health / Medical Services | `L2-G06` | Institutional medical-service workflows and supported service interactions. |
| `W-G07` | Security Operations | `L2-G07` | Campus-security operations and supported incident/access context. |
| `W-G08` | Housekeeping & Service Governance | `L2-G08` | Service requests, cleaning/service operations, governance and completion context. |
| `W-G09` | Teacher Guardian Scheme | `L2-G09` | Teacher-guardian interactions and governed student-support context. |
| `W-G10` | Infrastructure Management | `L2-G10` | Physical infrastructure operations and governed infrastructure records. |

## 5.8 Section H — Institutional Services & Intelligence

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-H01` | APAAR & DigiLocker | `L2-H01` | Learner linking, credential publication/consent/revocation/republication and reconciliation. |
| `W-H02` | Notifications | `L2-H02` | Notification outcomes, schedules, templates, preferences, emergency communication and evidence. |
| `W-H03` | Enterprise Directory & Search | `L2-H03` | Discovery/search projections, synchronization, recovery and configuration. |
| `W-H04` | Placement | `L2-H04` | Employers, opportunities, applications, selection, offers/joining, internships and reporting. |
| `W-H05` | Research | `L2-H05` | Proposals, projects, grants, ethics, outputs, IP, collaborations, consultancy and portfolio governance. |
| `W-H06` | Alumni Engagement | `L2-H06` | Membership/profile, chapters, networking, mentoring, contribution, recognition and donation intent. |
| `W-H07` | Enterprise Dashboards | `L2-H07` | Dashboard requirements/templates, availability, sharing, versioning, retirement and widgets. |
| `W-H08` | Statutory / Regulatory Reporting | `L2-H08` | Frameworks, collection plans, snapshots, packages, submissions and corrections. |
| `W-H09` | Accreditation & Compliance | `L2-H09` | Cycles, obligations, evidence, audits/assessments, findings/CAPA and readiness. |
| `W-H10` | External Education Integrations | `L2-H10` | Providers/connectors/trust, synchronization, enrollment/progress/credit/credential exchange and reconciliation. |

## 5.9 Section I — Examination

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-I01` | Examination Policy & Configuration | `L2-I01` | Exam policy/configuration experiences and governed setup context. |
| `W-I02` | Examination Sessions | `L2-I02` | Examination-session lifecycle and operational context. |
| `W-I03` | Examination Planning & Scheduling | `L2-I03` | Planning/scheduling, venue/invigilation context and governed revision. |
| `W-I04` | Examination Registration | `L2-I04` | Examination registration and governed registration outcomes. |
| `W-I05` | Hall Tickets | `L2-I05` | Hall-ticket generation/availability/correction and candidate access. |
| `W-I06` | Practical & Viva Conduct | `L2-I06` | Practical/viva conduct and supported operational actions. |
| `W-I07` | Theory Examination Conduct | `L2-I07` | Theory examination conduct and supported operational actions. |
| `W-I08` | Marks & Evaluation | `L2-I08` | Marks collection, evaluator activity, moderation/verification and governed outcomes. |
| `W-I09` | Result Processing | `L2-I09` | Result computation, validation/reconciliation and processing status. |
| `W-I10` | Result Publication | `L2-I10` | Result publication, candidate visibility and governed publication outcomes. |
| `W-I11` | Revaluation & Rechecking | `L2-I11` | Revaluation/rechecking requests and review outcomes. |
| `W-I12` | Supplementary / Backlog / Improvement Exams | `L2-I12` | Subsequent examination execution and outcomes. |
| `W-I13` | Academic Completion & CGPA Evidence | `L2-I13` | Completion/CGPA evidence and examination-owned completion projections. |
| `W-I14` | Malpractice & Examination Committee | `L2-I14` | Malpractice cases, committee review/decisions, evidence and history. |
| `W-I15` | Examination Exceptions & Contingency | `L2-I15` | Contingencies, exceptions, recovery and resolution. |
| `W-I16` | Examination Closure & Archive | `L2-I16` | Session closure, archive and historical examination evidence. |

## 5.10 Section J — Enterprise Resource Management

`L2-J00` remains architecture/governance and is not converted into a product workspace.

| Key | Working workspace family | Canonical source | L6 intent |
|---|---|---|---|
| `W-J01` | Procurement Planning & Sourcing | `L2-J01` | Procurement planning, sourcing and governed sourcing outcomes. |
| `W-J02` | Vendors & Suppliers | `L2-J02` | Supplier onboarding, records, status and lifecycle. |
| `W-J03` | Purchase Requisitions & Approvals | `L2-J03` | Requisitions, review/approval routing and outcomes. |
| `W-J04` | Purchase Orders | `L2-J04` | Purchase-order creation/change/status and lifecycle. |
| `W-J05` | Goods Receipt & Inspection | `L2-J05` | Receipt, inspection, acceptance/rejection and evidence. |
| `W-J06` | Inventory & Warehouse | `L2-J06` | Warehouse/inventory records, stock position and operations. |
| `W-J07` | Stock Operations & Distribution | `L2-J07` | Stock issue/transfer/distribution and governed movement. |
| `W-J08` | Asset Lifecycle | `L2-J08` | Operational asset establishment, assignment/use and lifecycle outcomes. |
| `W-J09` | Physical Verification & Reconciliation | `L2-J09` | Verification, discrepancy, audit/reconciliation and closure. |
| `W-J10` | Disposal / Scrap / Write-Off | `L2-J10` | Disposal, scrap/write-off decisions and completion. |

---

## 6. Approved Persona Home Composition

### Administrator

`Context → Institution Pulse → Attention / exceptions → Priority Work / true work queue → Recent work → Day / lower-priority institutional updates`

### Faculty

`Teaching context → Teaching Pulse → Next class → Today's Teaching / Needs Attention → Recent teaching work → Day schedule / lower-priority updates`

### Student

`Academic context → My Semester → Up Next → Due Soon → Academic progress / recent destinations → Day schedule / lower-priority campus updates`

A true work queue is valid for administrator/operational contexts where upstream workflows create human tasks. It shall not be imposed as the default interaction metaphor for Faculty or Student.

---

## 7. Navigation Implications

The 105 inventory units are intentionally larger than any sensible primary navigation.

Therefore:

- inventory row ≠ navigation item;
- several workspace families may be grouped behind one user-facing navigation family;
- persona/task navigation exposes different subsets;
- Home, Search, recent work and contextual links may bypass primary hierarchy;
- specialist Platform/Foundation configuration must not dominate ordinary Student or Faculty navigation;
- navigation grouping never changes canonical business ownership.

`L6_Persona_Journey_and_Navigation_Map_DRAFT.md` defines the controlled navigation composition.

---

## 8. Required Scenario-Level Decomposition

Before any workspace family becomes production-ready, its L6 specification must resolve:

1. exact final L3 scenario IDs;
2. applicable L4 behavior specifications and material BHs;
3. applicable L5 solution/contract/security/client dependencies and evidence;
4. supported actors/responsibility context;
5. primary user outcome;
6. required overview/list/detail/task/history surfaces;
7. authoritative states and decisions that must be presented;
8. loading/empty/pending/rejection/indeterminate/unavailable behavior;
9. scope/context behavior;
10. accessibility and responsive requirements;
11. navigation destination vs contextual/task-only placement.

No production frontend slice may invent unresolved details.

---

## 9. High-Risk Decomposition Areas

- **Campus Operations:** 320 final L3 scenarios plus the explicit Campus Operations Governance family; scenario volume must not become hundreds of unrelated pages.
- **Examination:** high-consequence policy, registration, conduct, evaluation, processing, publication, review, exception and closure semantics must remain distinct where upstream does.
- **Finance + ERM:** cross-domain handoffs may be composed in user journeys but must not merge canonical ownership.
- **Student + Guardian:** self-service exposure must follow actor/relationship/authority evidence.
- **Platform Foundation:** most workspaces are specialist governance surfaces, not ordinary Faculty or Student navigation.

---

## 10. Inventory Gate Result

The corrected draft passes the repository-wide workspace-family inventory gate because:

- 98 active L2 workflow/foundation areas are represented once at L2-anchored workspace-family level;
- the additional canonical `L3-G-001…015` Campus Operations Governance outcomes are represented through `W-G00` without converting `L2-G00` into a business owner;
- all ten L1 sections are covered;
- the traceability register reconciles all 1,084 final L3 scenarios to L6 workspace/shared presentation coverage;
- shared L6 surfaces remain separate from canonical business ownership;
- persona Home asymmetry matches the controlled foundation authorization;
- no row claims authorization, business-rule or lifecycle ownership.

**Result:** PASS at corrected workspace-family coverage level — **99 business workspace families + 6 shared surfaces = 105 inventory units**.

This is still not a final screen corpus and does not authorize uncontrolled implementation.

---

## 11. Current Controlled Sequence

1. Screen/workspace inventory — **Draft established and corrected to 105 units**.
2. Persona/task journey and navigation map — **Draft established**.
3. Screen/journey traceability register — **Draft established; all 1,084 final L3 scenarios reconciled at workspace-family level**.
4. Exact per-slice L4 BH → L5 evidence binding — required before production-slice approval.
5. Production L6 slice specification and validation.
6. Frontend implementation after the corresponding L6 slice passes the proceed gate.

---

**Draft verdict:** repository-wide workspace-family inventory established and traceability-corrected; not a final screen corpus and not a Final L6 Corpus Lock.
