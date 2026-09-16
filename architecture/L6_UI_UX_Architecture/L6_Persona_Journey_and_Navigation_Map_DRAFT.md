# SutraERP L6 — Persona Journey and Navigation Map — Draft

**Version:** 0.1.1  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** `L6_Screen_and_Workspace_Inventory_DRAFT.md`; `L6_Screen_Journey_Traceability_Register_DRAFT.md`; `00_Governance/L6_Foundation_Controlled_Authorization_2026-09-16.md`; existing L6 foundation drafts; Locked L1/L2 architecture; Locked L3 canonical corpus; L4 production behavior; approved L5 client/security/authorization architecture.  
**Downstream:** production L6 slice specifications; SutraERP frontend navigation and journey implementation.  

> UX navigation profiles are presentation compositions. They are not security roles, do not assign permissions, do not alter actor authority, and do not transfer canonical business ownership.

---

## 1. Purpose

Define how SutraERP presents its **105 current L6 inventory units** as coherent user journeys and navigation rather than exposing repository architecture directly.

The corrected 105-unit baseline consists of:

- 99 business workspace families, including the L3-derived `W-G00 Campus Operations Governance` family;
- 6 shared L6 surfaces.

The experience is intentionally asymmetric:

- Student → academic self-service and progress;
- Faculty → teaching-day and assigned academic responsibility;
- Guardian → relationship- and responsibility-scoped services;
- institutional specialists → focused operational work;
- broad administrators → cross-institution intervention and governance;
- platform administrators → specialist platform/tenant governance.

---

## 2. Authority and Composition Rules

### 2.1 Navigation Profile Is Not Authorization

```text
Upstream identity + responsibility + authorized scope
                  ↓
        L6 navigation composition
                  ↓
          visible destinations
```

But:

```text
visible destination ≠ permission / authority / eligibility
```

Protected operations remain server-authoritative.

### 2.2 Multi-Responsibility Users

One Person may hold several responsibilities. L6 shall compose navigation rather than force one permanent persona identity.

Examples supported by upstream actor patterns include:

- Faculty + Teacher Guardian;
- Faculty + Research responsibility;
- Academic Administrator + timetable coordination;
- Registrar + institutional governance;
- Finance Manager + reporting responsibility.

Duplicate destinations are merged; authority is never inferred from the merged presentation.

### 2.3 Navigation Does Not Change Ownership

A user-facing workspace may compose projections from several canonical owners. Each owner remains unchanged.

### 2.4 Progressive Disclosure

Primary navigation is for high-frequency recognizable work. Lower-frequency governance, history and configuration remain reachable through contextual navigation, global search, secondary administration areas, task entry and deep links.

### 2.5 Architecture IDs Are Not UX Labels

Sections A–J, L2/L3/L4 IDs, L5 families and React route names shall not become primary navigation taxonomy.

---

## 3. Navigation Layers

| Layer | Purpose | Examples |
|---|---|---|
| Home | Immediate orientation and next work | Institution Pulse; Teaching Pulse; My Semester |
| Primary navigation | Stable user work families | Students; Teaching; Examinations; Finance |
| Workspace local navigation | Sections within one coherent subject/task | Overview; Registration; Finance; History |
| Contextual/task navigation | Bounded actions reached from context | Review documents; take attendance; revaluation request |
| Global discovery | Search/command access to permitted destinations | Student, course, invoice, asset, report, workspace |

The 105 inventory units must remain consolidated; primary navigation shall not become a module dump.

---

## 4. UX Navigation Profiles

| Profile key | UX profile | Repository-supported basis | Core mental model |
|---|---|---|---|
| `P-STUDENT` | Student | Student/Candidate actors across C/D/I and applicable services | What is happening, what is due, and am I on track? |
| `P-FACULTY` | Faculty / Teacher | Faculty Member, Teacher Guardian and academic/faculty actors | What am I teaching or responsible for today? |
| `P-GUARDIAN` | Parent / Guardian / Authorized Representative | C06 guardian relationship, custody, consent, communication and responsibility architecture | What am I authorized to know or do for the student relationships I hold? |
| `P-INST-ADMIN` | Institutional Administrator | Institute Administrator, Registrar, Academic Administrator and other broad governance actors where assigned | How is the institution operating and what requires intervention? |
| `P-STUDENT-OPS` | Admissions & Student Operations | Admissions, registration, progression, qualification/exit operational actors | Which student-lifecycle work requires processing or resolution? |
| `P-ACADEMIC-OPS` | Academic Operations | Scheduler, timetable/programme coordinators, Academic Administrator and related actors | What academic delivery must be planned, coordinated or corrected? |
| `P-EXAM-OPS` | Examination Operations | Controller of Examination and I01–I16 actors | What examination-cycle work must be prepared, conducted, evaluated or published? |
| `P-FINANCE` | Finance Operations | Accountant, Senior Accountant, Finance Manager and other F actors | What financial work must be processed, reconciled, controlled or closed? |
| `P-PEOPLE` | People / HR Operations | HR Administrator, HR Executive and workforce/payroll actors | What workforce and people-service work requires action? |
| `P-CAMPUS` | Campus Operations | Campus governance plus facilities/lab/hostel/transport/library/health/security/housekeeping/TGS/infrastructure actors | What campus service, operational or governance work requires attention? |
| `P-RESOURCE` | Procurement / Inventory / Asset Operations | Procurement, supplier, warehouse, stock, asset and verification actors | What resource-lifecycle work requires processing or control? |
| `P-SERVICES` | Institutional Services / Intelligence | Placement, Research, Alumni, reporting, accreditation/compliance, dashboard and external-integration actors | What institutional service, intelligence or compliance outcome am I responsible for? |
| `P-PLATFORM` | Platform / Tenant Administration | Platform Administrator and specialist Platform Foundation actors | What platform, identity, access, workflow, audit, data or readiness governance requires action? |

---

## 5. Shared Authenticated Shell

Where upstream-supported, all authenticated profiles may receive:

- Home;
- Institution/academic context presentation;
- global search/command discovery;
- notifications;
- task/work entry only when an actual workflow task exists;
- help/support;
- user/account/session presentation.

Context change follows L5-G-01-003/A04 invalidation and revalidation before previous protected projections are reused.

---

# 6. Student Experience

## 6.1 Primary Navigation

```text
Home
My Academics
Timetable
Assessments & Exams
Fees
Documents & Services
Campus
More
```

`More` is progressive disclosure, not a domain dump.

### 6.2 Mapping

| Student label | Inventory sources | Boundary |
|---|---|---|
| Home | `S-01`, applicable D/I/F projections | My Semester, Up Next, Due Soon and day context. |
| My Academics | `W-C03`, `W-C04`, `W-D01`, `W-D04`, `W-D06` | Registration, credits/progress, course context, attendance and eligible learning. |
| Timetable | `W-D03` | Student schedule projection only. |
| Assessments & Exams | `W-D07`, `W-I04`, `W-I05`, `W-I10`, `W-I11`, `W-I12`, `W-I13` | Candidate-facing journeys only where applicable. |
| Fees | `W-F01`, `W-F02`, `W-F03`, `W-F11` | Student-facing participation/projection only where authorized. |
| Documents & Services | `W-C02`, `W-C05`, `W-H01`, applicable H02/H03 | Documents, credentials and relevant services. |
| Campus | applicable `W-G03–G06`, `W-G09` and other G services only where a student interaction exists | Never expose campus operational/governance administration merely because it exists. |

### 6.3 Home Journey

```text
Home
 ↓
My Semester
 ↓
Up Next → class/timetable context

or

Due Soon → assessment/exam/academic obligation → detail/action
```

No generic enterprise work queue.

### 6.4 Academic Registration Journey

```text
My Academics
 ↓
Registration
 ↓
current registration context
 ↓
upstream-supported action
 ↓
authoritative success / rejection / pending / indeterminate state
 ↓
updated projection
```

### 6.5 Examination Journey

```text
Assessments & Exams
 ├─ Registration
 ├─ Hall Ticket
 ├─ Published Result
 ├─ Revaluation / Rechecking
 └─ Supplementary / Backlog / Improvement
```

Applicability remains upstream-determined.

---

# 7. Faculty Experience

## 7.1 Primary Navigation

```text
Home
Teaching
Timetable
Students
Assessments
Academic Work
More
```

Responsibility-driven additions such as Research or Mentoring may appear when upstream responsibility exists.

### 7.2 Mapping

| Faculty label | Inventory sources | Boundary |
|---|---|---|
| Home | `S-01`, `W-D02`, `W-D03`, `W-D04`, `W-D07` | Teaching Pulse, Next Class, Today's Teaching, day schedule. |
| Teaching | `W-D01`, `W-D02`, `W-D04` | Assigned course/class, teaching sessions, attendance and progress. |
| Timetable | `W-D03` | Faculty schedule; planning controls require Academic Operations responsibility. |
| Students | contextual `W-C02`, `W-C04`, `W-D04`, `W-D05` | Only supported student/class responsibility scope. |
| Assessments | `W-D07`; applicable `W-I06–I08` only where examination responsibility exists | Internal assessment by default; exam authority is separate. |
| Academic Work | `W-D05`, `W-D06`; optional `W-G09`, `W-H05` | Monitoring, flexible learning, mentoring, research or other assigned work. |

### 7.3 Teaching-Day Journey

```text
Home → Teaching Pulse → Next Class → Class workspace
      ├─ session context
      ├─ attendance
      ├─ teaching progress
      └─ relevant student/class context
      ↓
authoritative completion / pending / rejection
```

No administrator ticket-queue metaphor.

### 7.4 Multi-Responsibility Addition

```text
Faculty base navigation
      + Mentoring → W-G09
      + Research → W-H05
```

Only where upstream responsibility supports the addition.

---

# 8. Guardian Experience

Guardian navigation is relationship- and authorization-scoped. Guardian existence does not imply unrestricted student access.

## 8.1 Primary Navigation

```text
Home
My Students
Relationships & Consents
Communications
Financial Responsibility
Services
```

### 8.2 Mapping

| Label | Inventory sources | Boundary |
|---|---|---|
| Home | `S-01`, `W-C06`, applicable `W-H02` | Relationship status, notices and bounded actions. |
| My Students | `W-C06` + permitted student projection | Active/authorized relationships only. |
| Relationships & Consents | `W-C06` | Custody, consent, communication, emergency contact, pickup and lifecycle. |
| Communications | `W-H02` + C06 authorization context | Notification delivery/preferences remain upstream-owned. |
| Financial Responsibility | `W-C06`, `W-F01`, `W-F02` | Only where canonical financial responsibility exists. |
| Services | applicable campus/student services | No generic access to all Student/Campus workspaces. |

### 8.3 Relationship Journey

```text
Home / My Students
 ↓
authorized relationship context
 ↓
relationship overview
 ↓
allowed consent / communication / financial-responsibility action
 ↓
authoritative outcome
```

---

# 9. Institutional Administrator Experience

## 9.1 Primary Navigation

```text
Home
Students
Academics
Examinations
Finance
People
Campus
Resources
Insights & Reports
Administration
```

Institutional Services may later become a separate top-level family after validation; no ownership changes are implied either way.

### 9.2 Home

```text
Context / greeting
 ↓
Institution Pulse
 ↓
Critical attention / exceptions
 ↓
Priority Work / true work queue
 ↓
Recent work
 ↓
Today / lower-priority updates
```

### 9.3 Broad Mapping

| Family | Inventory workspace families |
|---|---|
| Students | `W-C01–C08` plus permitted cross-domain projections |
| Academics | `W-B04–B05`, `W-D01–D07` |
| Examinations | `W-I01–I16` |
| Finance | `W-F00–F14` |
| People | `W-E01–E07` |
| Campus | `W-G00–G10` |
| Resources | `W-J01–J10`, with F14 finance consumption contextual |
| Insights & Reports | `W-H07–H09`, `W-F13`, `W-G00` governance projections where authorized, other approved reporting projections |
| Administration | `W-B01–B03`, applicable `W-A*`, responsibility-appropriate configuration surfaces |

No broad institutional administrator automatically sees every destination or action.

### 9.4 Exception-to-Resolution Journey

```text
Home → priority/attention item → canonical owning workspace
     → context/evidence → supported decision/action
     → authoritative outcome → return to queue/workspace
```

---

# 10. Specialist Navigation Profiles

## 10.1 Admissions & Student Operations

```text
Home
Admissions
Students
Registration
Progression
Qualification & Exit
Documents
More
```

Primary: `W-C01–C08`; contextual D/F/H/I work only where required by the journey.

## 10.2 Academic Operations

```text
Home
Curriculum
Course Offerings
Timetable
Faculty Assignments
Teaching & Attendance
Monitoring
Assessments
```

Primary: `W-B04–B05`, `W-D01–D07`; contextual `W-E03`, `W-C03/C04`.

## 10.3 Examination Operations

```text
Home
Policy & Setup
Sessions
Planning & Scheduling
Registration
Conduct
Marks & Evaluation
Results
Reviews & Exceptions
Closure
```

Primary: `W-I01–I16`.

## 10.4 Finance Operations

```text
Home
Student Finance
Payments & Refunds
Ledger
Receivables & Payables
Budgets
Treasury
Periods & Closing
Assets
Reports
```

Primary: `W-F00–F14`. Maker-checker and approval separation remains upstream-authoritative.

## 10.5 People / HR Operations

```text
Home
Workforce
Positions & Assignments
Faculty Workforce
HR Services
Leave
Payroll
Employee Profiles
```

Primary: `W-E01–E07`; contextual B02/D02.

## 10.6 Campus Operations

```text
Home
Governance & Readiness
Facilities
Laboratories
Hostel
Transport
Library
Health
Security
Services
Mentoring
Infrastructure
```

Primary: `W-G00–G10`. `Governance & Readiness` maps to `W-G00` and is intended only for actors with the corresponding campus-governance responsibility. A specific operator receives only the applicable subset.

## 10.7 Procurement / Resources

```text
Home
Procurement
Suppliers
Requisitions
Purchase Orders
Goods Receipt
Inventory
Stock Operations
Assets
Verification
Disposal
```

Primary: `W-J01–J10`; F14/F08/F12 handoffs stay contextual.

## 10.8 Institutional Services / Intelligence

Potential subprofiles:

```text
Placement
Research
Alumni
Dashboards
Statutory Reporting
Accreditation & Compliance
External Education
Digital Credentials
```

Notifications and Search remain shared services rather than necessarily permanent primary-nav items.

## 10.9 Platform / Tenant Administration

```text
Home
Tenants
Identity
Access
Workflow
Events
Audit
Capabilities
Master Data
Readiness
```

Primary: `W-A01–A10`; specialist only.

---

# 11. Cross-Workspace Journey Rules

1. Preserve subject and Institution/academic context through handoffs, but never carry prior-domain authority as permission in the destination.
2. Prefer contextual links or source-owned projections over duplicate editable copies.
3. A Home/task item opens the canonical owning workspace/task context rather than becoming a shadow business record.
4. After bounded work, return to the owning workspace, originating queue/list/search context, or Home only when no more specific context exists.
5. Cross-domain composition never changes canonical ownership.

---

# 12. Search / Command Navigation

Global discovery may expose authorized/discoverable:

- people/students/employees;
- courses/offerings;
- examination contexts;
- financial documents;
- assets/resources;
- reports/dashboards;
- workspaces;
- recent destinations.

Rules:

- search visibility is not mutation authority;
- scope identity is preserved;
- result types are distinguishable;
- deep links revalidate normal context/auth boundaries;
- shortcuts cannot bypass business preconditions.

---

# 13. Notifications vs Tasks

- **Notification** = information delivered to a user.
- **Task** = an upstream workflow requires a human action/decision.

A notification may link to a workspace without becoming a task. Faculty/Student Home may show action-oriented items without falsely converting them into generic workflow tasks.

---

# 14. Workspace Local Navigation

A sustained workspace should expose only local sections required by its user goal.

Candidate Student Workspace composition:

```text
Overview
Identity / Context
Academics
Registration
Attendance
Finance
Examinations
Documents
Guardian Relationships
Progression / Qualification
History
```

Exact sections require the traceability register and slice specification. The same canonical fact shall not become editable in multiple places merely for convenience.

---

# 15. Responsive Navigation

### Desktop

Persistent primary navigation for broad institutional/specialist work where width permits; current scope and workspace remain visible.

### Tablet

Collapsible navigation while retaining orientation and context.

### Mobile

Task-oriented rather than compressed desktop ERP.

Prioritize:

- Student: timetable, due work, exams/results, fees/documents, relevant services;
- Faculty: next class, timetable, attendance, bounded review work;
- Guardian: relationships, consents/communications and responsibility-scoped services;
- specialists: approvals, alerts, quick lookup and bounded tasks where safe.

Complex finance close, timetable planning, bulk administration, platform governance and multi-panel campus governance may remain desktop/tablet-primary unless later L6 evidence justifies full mobile optimization.

---

# 16. Language

Prefer user/institutional language such as `Students`, `Teaching`, `Fees`, `Assessments & Exams`, `People`, `Campus`, `Resources` rather than architecture IDs or bounded-context names.

Friendly labels must not rename canonical business states, decisions, evidence or legal/governance concepts.

---

# 17. Home Rules by Profile

| Profile | Home emphasis | Queue model |
|---|---|---|
| Student | My Semester, Up Next, Due Soon | No generic enterprise queue |
| Faculty | Teaching Pulse, Next Class, Today's Teaching | No generic enterprise queue |
| Guardian | Relationships, notices, authorized responsibilities | No generic enterprise queue by default |
| Institutional Administrator | Institution Pulse, exceptions, Priority Work | True queue appropriate |
| Institutional specialists | Responsibility-relevant pulse/attention/tasks | Queue only where upstream human-task semantics justify it |
| Campus governance actors | Campus readiness/operational attention from `W-G00` where authorized | Queue only where upstream task semantics exist |
| Platform Administrator | platform governance attention/tasks | Queue appropriate where workflows create tasks |

---

# 18. Open Decisions

Still not locked:

- exact final ordering/wording of all primary navigation;
- exact multi-role merge behavior;
- whether Institutional Services becomes separate broad-admin navigation;
- final mobile specialist task set;
- command-palette shortcuts;
- exact Guardian self-service exposure beyond canonical relationship/responsibility functions;
- exact Student/Faculty access to Campus/Research/Placement/External Education;
- final local tabs inside the 99 business workspace families;
- permanent L6 IDs/folder taxonomy.

---

# 19. Traceability Dependency

`L6_Screen_Journey_Traceability_Register_DRAFT.md` is the controlled provenance companion to this map. It resolves the final L3/L4 scenario population for every workspace family and records the mandatory L5 web-client baseline plus per-slice L5 binding gate.

The traceability audit corrected the inventory by adding `W-G00`; this document now consumes that correction.

---

# 20. Proceed Gate

**Result:** PASS — suitable as the controlled working journey/navigation map for production-slice trace authoring.

Next controlled step:

- select a production L6 slice;
- bind its exact L4 BHs to applicable L5 specifications/evidence;
- author the full L6 screen/workspace specification using the controlled template;
- validate it before frontend production implementation.

---

**Draft verdict:** repository-derived persona/task navigation architecture synchronized with the corrected 105-unit inventory and full-corpus traceability baseline.
