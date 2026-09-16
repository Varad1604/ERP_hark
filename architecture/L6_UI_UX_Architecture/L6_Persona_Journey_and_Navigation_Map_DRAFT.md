# SutraERP L6 — Persona Journey and Navigation Map — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** `L6_Screen_and_Workspace_Inventory_DRAFT.md`; `00_Governance/L6_Foundation_Controlled_Authorization_2026-09-16.md`; existing L6 foundation drafts; Locked L1/L2 business architecture; Locked L3 canonical corpus; L4 full-corpus production behavior; approved L5 client/security/authorization architecture.  
**Downstream:** `L6_Screen_Journey_Traceability_Register_DRAFT.md`; production L6 slice specifications; SutraERP frontend navigation and journey implementation.  

> This document defines UX navigation profiles and journey composition. It does **not** create authorization roles, assign permissions, alter actor authority, or transfer canonical business ownership.

---

## 1. Purpose

Define how SutraERP presents its 104 current L6 inventory units as coherent user journeys and navigation rather than exposing the repository's architecture structure directly.

The core problem is intentionally asymmetric:

- a Student should see an academic self-service experience;
- a Faculty member should see a teaching-day experience;
- a Guardian should see relationship- and responsibility-scoped services;
- an institutional specialist should see the work relevant to that specialist role;
- a broad institutional administrator may span several work families;
- a platform administrator should not make platform-governance surfaces dominate ordinary institutional users.

This map resolves **experience organization**, not business authority.

---

## 2. Authority and Safety Rules

### 2.1 UX Profile Is Not an Authorization Role

A navigation profile is a presentation composition chosen from upstream-resolved responsibility/capability context.

```text
Upstream identity + responsibility + authorized scope
                  ↓
        L6 navigation composition
                  ↓
          visible destinations
```

The inverse is prohibited:

```text
visible destination
        ≠
permission / authority / business eligibility
```

Server-side upstream authorization remains authoritative for protected operations.

### 2.2 Multi-Role Users

A Person may hold multiple institutional responsibilities. L6 shall therefore support a composed experience rather than assume one permanent persona label.

Examples supported by repository actor patterns include faculty members who also act as Teacher Guardians, researchers, coordinators or institutional decision participants.

L6 may adapt navigation based on resolved responsibilities but shall not create new roles.

### 2.3 Domain Ownership Is Not Navigation Ownership

Navigation may compose several canonical owners into one user journey. The underlying owner remains unchanged.

Example:

```text
Student view of "Fees"
    ↓
Finance-owned charge/payment projections
    + Student-owned context
    + optional Guardian responsibility context
```

L6 owns the presentation only.

### 2.4 Primary Navigation Is for Frequent Work

Primary navigation shall optimize recognition and frequency.

Lower-frequency governance, history, configuration and specialist work shall remain available through:

- contextual workspace links;
- global search;
- secondary navigation;
- administration/settings areas;
- direct task entry;
- deep links where authorized.

### 2.5 Architecture Sections A–J Are Never User Navigation

The labels `Section A` through `Section J`, L2 IDs, L3 IDs, L5 families and React route-module names shall not be exposed as primary UX taxonomy.

---

## 3. Navigation Model

SutraERP uses five navigational layers.

| Layer | Purpose | Examples |
|---|---|---|
| **Home** | Immediate orientation and next work | Institution Pulse; Teaching Pulse; My Semester |
| **Primary navigation** | Stable, high-frequency user work families | Students; Teaching; Examinations; Finance |
| **Workspace local navigation** | Sections supporting one coherent subject/task | Student Overview; Registration; Finance; Examination |
| **Contextual/task navigation** | Bounded actions reached from the current context | Review admission documents; Take attendance; Revaluation request |
| **Global discovery** | Search/command navigation across authorized discoverable destinations | Student, course, invoice, report, workspace search |

Primary navigation should normally remain short enough to scan without scrolling on standard desktop layouts. The 104 inventory units are therefore deliberately consolidated below.

---

## 4. UX Navigation Profiles

These profiles are L6 presentation cohorts, not canonical business roles.

| Profile key | UX profile | Repository-supported actor basis | Core mental model |
|---|---|---|---|
| `P-STUDENT` | Student | Student / Candidate actors across Student Lifecycle, Academic Delivery and Examination | "What is happening, what is due, and am I on track?" |
| `P-FACULTY` | Faculty / Teacher | Faculty Member, Teacher Guardian and faculty workforce/teaching actors | "What am I teaching or responsible for today?" |
| `P-GUARDIAN` | Parent / Guardian / Authorized Representative | Guardian relationship actors, custody/consent/communication/financial responsibility architecture | "What am I authorized to know or do for the student relationships I hold?" |
| `P-INST-ADMIN` | Institutional Administrator | Institute Administrator, Registrar, Academic Administrator and other broad institutional governance actors where assigned | "How is the institution operating and what requires intervention?" |
| `P-STUDENT-OPS` | Admissions & Student Operations | Admission, student-services, registration, progression and qualification/exit operational actors | "Which student lifecycle work requires processing or resolution?" |
| `P-ACADEMIC-OPS` | Academic Operations | Academic Scheduler, Department Timetable Coordinator, Programme Coordinator, Academic Administrator and related academic-operational actors | "What academic delivery must be planned, coordinated or corrected?" |
| `P-EXAM-OPS` | Examination Operations | Controller of Examination and other examination actors defined by I01–I16 | "What examination cycle work must be prepared, conducted, evaluated or published?" |
| `P-FINANCE` | Finance Operations | Accountant, Senior Accountant, Finance Manager, CFO/finance authorities and other F00–F14 actors | "What financial work must be processed, reconciled, controlled or closed?" |
| `P-PEOPLE` | People / HR Operations | HR Administrator, HR Executive and workforce/assignment/payroll actors | "What workforce and people-service work requires action?" |
| `P-CAMPUS` | Campus Operations | Facility/lab/hostel/transport/library/medical/security/housekeeping/TGS/infrastructure actors | "What campus service or operational work requires attention?" |
| `P-RESOURCE` | Procurement / Inventory / Asset Operations | Procurement, supplier, warehouse, stock, asset and verification actors | "What resource lifecycle work requires processing or control?" |
| `P-SERVICES` | Institutional Services / Intelligence | Placement, Research, Alumni, statutory reporting, accreditation/compliance, dashboard and external-integration actors | "What institutional service, intelligence or compliance outcome am I responsible for?" |
| `P-PLATFORM` | Platform / Tenant Administration | Platform Administrator and specialist Platform Foundation actors | "What platform, identity, access, workflow, audit, data or readiness governance requires action?" |

### 4.1 Profile Composition

A user may receive a composed navigation model from more than one profile.

Examples:

- Faculty + Research responsibility;
- Faculty + Teacher Guardian responsibility;
- Academic Administrator + Timetable coordination;
- Registrar + institutional administration;
- Finance Manager + institutional reporting responsibility.

Composition shall merge duplicate destinations rather than create duplicated menu trees.

---

# 5. Shared Shell for All Authenticated Profiles

The following remain common shell capabilities where upstream-supported:

- Home;
- current Institution / academic context presentation;
- global search / command discovery;
- notifications;
- task/work entry when the user actually has workflow tasks;
- help/support;
- user/account/session presentation.

Context presentation must remain visually quiet but unmistakable before protected actions.

A context switch must follow L5-G-01-003 state invalidation and revalidation semantics before prior protected projections are reused.

---

# 6. Student Navigation and Journeys

## 6.1 Student Primary Navigation

Recommended working labels:

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

`More` is progressive disclosure, not a domain dump. It may expose only upstream-supported destinations relevant to the current student, such as flexible learning, placements, external education or alumni transition when applicable.

### 6.2 Student Workspace Mapping

| Student label | Primary inventory sources | Notes |
|---|---|---|
| Home | `S-01`, `W-D03`, `W-D04`, `W-D07`, `W-I*`, `W-F01` | My Semester, Up Next, Due Soon and chronological day. |
| My Academics | `W-C03`, `W-C04`, `W-D01`, `W-D04`, `W-D06` | Registration, progress/credits, course context, attendance and eligible learning opportunities. |
| Timetable | `W-D03` | Student schedule projection; no timetable governance controls. |
| Assessments & Exams | `W-D07`, `W-I04`, `W-I05`, `W-I10`, `W-I11`, `W-I12`, `W-I13` | Candidate-facing assessment/exam journeys only where supported. |
| Fees | `W-F01`, `W-F02`, `W-F03`, `W-F11` | Student-facing financial projection/action surfaces only where the user is an authorized participant. |
| Documents & Services | `W-C02`, `W-C05`, `W-H01`, applicable `W-H02/H03` | Certificates/documents, government credential interactions and relevant services. |
| Campus | applicable `W-G03–G06`, `W-G09`, other G workspaces only where a student-facing interaction exists | Never expose campus operational administration merely because the service exists. |

### 6.3 Student Home Journey

```text
Home
 ↓
My Semester orientation
 ↓
Up Next
 ↓
class / timetable context

or

Due Soon
 ↓
assessment / exam / academic obligation
 ↓
relevant detail or bounded action
```

The Student home shall not use an enterprise work-queue/table mental model.

### 6.4 Student Academic Registration Journey

```text
My Academics
 ↓
Registration
 ↓
current registration window/context
 ↓
eligible academic-registration action
 ↓
authoritative outcome / rejection / pending / indeterminate state
 ↓
updated registration projection
```

L6 shall not calculate eligibility or interpret successful local submission as canonical registration success.

### 6.5 Student Examination Journey

Candidate journey family, subject to exact I-domain traceability:

```text
Assessments & Exams
 ↓
exam context
 ├─ Registration
 ├─ Hall Ticket
 ├─ Published Result
 ├─ Revaluation / Rechecking
 └─ Supplementary / Backlog / Improvement
```

No examination function appears merely because the user is a Student; applicability is determined upstream.

---

# 7. Faculty Navigation and Journeys

## 7.1 Faculty Primary Navigation

Recommended working labels:

```text
Home
Teaching
Timetable
Students
Assessments
Academic Work
More
```

Optional responsibility-driven additions such as `Research` or `Mentoring` may appear when the faculty member has corresponding upstream responsibility.

### 7.2 Faculty Workspace Mapping

| Faculty label | Primary inventory sources | Notes |
|---|---|---|
| Home | `S-01`, `W-D02`, `W-D03`, `W-D04`, `W-D07` | Teaching Pulse, Next Class, Today's Teaching, day schedule. |
| Teaching | `W-D01`, `W-D02`, `W-D04` | Assigned course/class context, teaching sessions, attendance and syllabus progress. |
| Timetable | `W-D03` | Faculty schedule projection; planning controls only through Academic Operations responsibility. |
| Students | contextual projections from `W-C02`, `W-C04`, `W-D04`, `W-D05` | Only students/contexts supported by the faculty member's resolved responsibility. |
| Assessments | `W-D07`; applicable `W-I06–I08` only where examination responsibility exists | Internal-assessment teaching/review work by default; exam roles are separate responsibility. |
| Academic Work | `W-D05`, `W-D06`; optional `W-G09`, `W-H05` | Monitoring/flexible learning, mentoring, research or other assigned academic work. |

### 7.3 Faculty Teaching-Day Journey

```text
Home
 ↓
Teaching Pulse
 ↓
Next Class
 ↓
Class workspace
 ├─ session context
 ├─ attendance action
 ├─ teaching progress
 └─ relevant student/class context
 ↓
authoritative completion / pending / rejection state
```

The Faculty experience shall not present this as an administrator ticket queue.

### 7.4 Faculty Assessment Journey

```text
Assessments
 ↓
assigned assessment / submissions
 ↓
review / evaluation action supported by upstream behavior
 ↓
moderation/finalization boundary where applicable
 ↓
authoritative result state
```

### 7.5 Faculty Multi-Responsibility Journey

A Faculty member who is also a Teacher Guardian or researcher should not switch to an unrelated application shell.

Instead:

```text
Faculty base navigation
      +
responsibility-specific destination
```

Examples:

- `Mentoring` → `W-G09`;
- `Research` → `W-H05`.

These additions remain upstream responsibility-driven.

---

# 8. Guardian Navigation and Journeys

Guardian access must remain relationship- and authorization-scoped. Guardian existence does **not** imply unrestricted student access.

## 8.1 Guardian Primary Navigation

Recommended working labels:

```text
Home
My Students
Relationships & Consents
Communications
Financial Responsibility
Services
```

Additional student academic, attendance, fee or campus projections may appear only where a separate upstream authorization/relationship rule permits them.

### 8.2 Guardian Mapping

| Guardian label | Primary inventory sources | Notes |
|---|---|---|
| Home | `S-01`, `W-C06`, applicable `W-H02` | Relationship-relevant status, notices and bounded actions. |
| My Students | `W-C06` + permitted student projection | Lists only active/authorized relationships. |
| Relationships & Consents | `W-C06` | Custody, consent, communication authorization, emergency contact, pickup and relationship lifecycle as applicable. |
| Communications | `W-H02` + `W-C06` communication-authorization context | Delivery/preferences remain H02/A-owned as applicable. |
| Financial Responsibility | `W-C06`, `W-F01`, `W-F02` | Only where canonical guardian financial responsibility has been established. |
| Services | applicable campus/student services | No generic visibility to all Student or Campus workspaces. |

### 8.3 Guardian Relationship Journey

```text
Home / My Students
 ↓
select authorized relationship context
 ↓
relationship overview
 ↓
allowed consent / communication / financial-responsibility action
 ↓
authoritative outcome
```

Where legal/custody status changes, L6 must present the resulting access implications clearly but shall not calculate them itself.

---

# 9. Institutional Administrator Navigation

The Administrator experience is broad but must still use progressive disclosure.

## 9.1 Primary Navigation

The current approved navigation family is retained as the institutional broad-work model:

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

`Institutional Services` may be exposed as its own family or through contextual/secondary navigation after later validation; it shall not be silently merged into another canonical owner.

### 9.2 Administrator Home

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
Today / lower-priority institutional updates
```

### 9.3 Broad Navigation Mapping

| Navigation family | Inventory workspace families |
|---|---|
| Students | `W-C01–C08` plus permitted cross-domain student projections |
| Academics | `W-B04–B05`, `W-D01–D07` |
| Examinations | `W-I01–I16` |
| Finance | `W-F00–F14` |
| People | `W-E01–E07` |
| Campus | `W-G01–G10` |
| Resources | `W-J01–J10`, with F14 cross-domain finance consumption shown contextually |
| Insights & Reports | `W-H07–H09`, `W-F13`, other approved reporting projections |
| Administration | `W-B01–B03`, applicable `W-A*`, configuration surfaces appropriate to the user's responsibilities |

This broad mapping is a composition layer only. It does not mean every institutional administrator sees every family.

### 9.4 Administrator Exception-to-Resolution Journey

```text
Home
 ↓
attention / priority work item
 ↓
canonical owning workspace
 ↓
context + evidence
 ↓
approved decision/action UI
 ↓
authoritative outcome
 ↓
return to priority work / owning workspace
```

A task is never resolved merely because the local UI closes or disappears.

---

# 10. Specialist Institutional Navigation Profiles

Specialists should see a focused subset rather than the full Administrator navigation.

## 10.1 Admissions & Student Operations (`P-STUDENT-OPS`)

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

Primary inventory: `W-C01–C08`, plus contextual `W-F01/F11`, `W-D01`, `W-I13`, `W-H01/H02/H03` only when needed by the supported journey.

Typical journey:

```text
Admissions attention
 ↓
candidate/admission workspace
 ↓
document/evidence review
 ↓
upstream-supported decision
 ↓
student activation / rejection / pending outcome
```

## 10.2 Academic Operations (`P-ACADEMIC-OPS`)

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

Primary inventory: `W-B04–B05`, `W-D01–D07`, contextual `W-E03`, `W-C03/C04`.

Typical journey:

```text
Academic requirement
 ↓
Course Offering / Timetable workspace
 ↓
capacity / assignment / schedule context
 ↓
validation / review
 ↓
activation or governed revision outcome
```

## 10.3 Examination Operations (`P-EXAM-OPS`)

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

Primary inventory: `W-I01–I16`.

The information architecture intentionally keeps planning, conduct, evaluation, publication, review and closure distinguishable because upstream examination architecture separates them.

## 10.4 Finance Operations (`P-FINANCE`)

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

Primary inventory: `W-F00–F14`.

Finance roles may receive different actions within the same workspace due to maker-checker and approval separation; L6 navigation must not collapse that separation into a single unrestricted "Finance Admin" authority.

## 10.5 People / HR Operations (`P-PEOPLE`)

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

Primary inventory: `W-E01–E07`, contextual `W-B02`, `W-D02`.

## 10.6 Campus Operations (`P-CAMPUS`)

```text
Home
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

Primary inventory: `W-G01–G10`.

A specific campus operator should receive only the applicable subset.

## 10.7 Procurement / Resources (`P-RESOURCE`)

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

Primary inventory: `W-J01–J10`; finance handoffs from `W-F14/F08/F12` are contextual rather than ownership transfer.

## 10.8 Institutional Services / Intelligence (`P-SERVICES`)

Because H contains materially different institutional jobs, this profile may decompose into subprofiles during traceability:

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

Notification and Enterprise Search remain shared shell/services rather than necessarily permanent primary-nav items.

## 10.9 Platform / Tenant Administration (`P-PLATFORM`)

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

Primary inventory: `W-A01–A10`.

This profile is specialist administration and shall not appear in ordinary Student, Faculty or Guardian navigation.

---

# 11. Cross-Workspace Journey Rules

## 11.1 Preserve Context Through Handoffs

When a user follows a cross-domain journey, SutraERP should carry presentation context such as:

- current Person/Student/Employee/Offering/Exam/financial subject identity;
- current Institution/academic context;
- source workspace;
- reason for handoff.

It must not carry prior-domain authority as if it were permission in the destination.

## 11.2 Contextual Links Beat Duplicate Data Entry

When the authoritative record belongs elsewhere, L6 should deep-link or compose a projection rather than duplicate an editable copy.

## 11.3 Tasks Link to Owners

A work item should open the owning workspace/task context rather than becoming an independent shadow record in the Home queue.

## 11.4 Return Path

After bounded work completes, users should have a predictable return path:

- owning workspace;
- originating task queue;
- prior list/search context;
- Home only when no more specific return context exists.

---

# 12. Search and Command Navigation

Global discovery should help users bypass deep menu traversal.

Search/command results may include, where authorized and supported:

- people/students/employees;
- courses/offerings;
- examination contexts;
- financial documents;
- assets/resources;
- reports/dashboards;
- workspaces;
- recent destinations.

Rules:

1. search visibility is not mutation permission;
2. results preserve Institution/scope identity;
3. result types are visually distinguishable;
4. deep links resolve through normal context/auth validation;
5. sensitive result categories follow upstream privacy/authorization constraints;
6. command shortcuts cannot bypass protected business preconditions.

---

# 13. Notifications vs Tasks

The UI shall preserve the architecture distinction:

- **Notification** = information delivered to a user;
- **Task** = an upstream workflow requires a human action/decision.

A notification may deep-link to a workspace without becoming a task.

A task may appear in Home / Task Entry while its canonical context remains the owning workflow/domain.

Faculty and Student Home may show action-oriented items, but they shall not automatically be represented as generic workflow tasks unless upstream workflow semantics actually create one.

---

# 14. Workspace Local Navigation Pattern

Each sustained workspace should choose only the local sections needed for its user goal.

Example — Student Workspace candidate composition:

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

This composition is still candidate-level. Exact sections require the traceability register and relevant L6 slice specification.

The same data shall not be made editable in multiple local sections merely for convenience if ownership is elsewhere.

---

# 15. Responsive Navigation Strategy

## Desktop

- persistent side navigation for broad institutional/specialist work where viewport permits;
- Home + profile-relevant work families;
- context/search/user controls remain visible;
- local workspace tabs/sections may remain persistent where density requires.

## Tablet

- collapsible primary navigation;
- retain current workspace and scope orientation;
- reduce simultaneous columns before removing information.

## Mobile

Mobile is task-oriented, not a compressed desktop ERP.

Prioritize:

- Student: timetable, due work, exams/results, fees/documents, relevant services;
- Faculty: next class, timetable, attendance, bounded review work;
- Guardian: student relationship, consents/communications/responsibility-scoped services;
- institutional specialists: approvals, alerts, quick lookup and bounded operational tasks where safe.

Complex finance close, timetable planning, bulk administration, configuration or multi-panel governance may remain desktop/tablet-primary unless later L6 evidence justifies full mobile optimization.

---

# 16. Navigation Labels and Language

Prefer institutional/user language:

- `Students`, not `Student Lifecycle Domain`;
- `Teaching`, not `Academic Delivery D04`;
- `Fees`, not `Student Charge & Fee Management` for Student-facing navigation;
- `Assessments & Exams`, not internal workflow names;
- `People`, not Section E;
- `Resources`, not Section J.

However, friendly labels must not distort business meaning. Exact names for states, decisions, evidence and legal/governance concepts remain upstream-controlled.

---

# 17. Profile-Specific Home Rules

| Profile | Home emphasis | Queue model |
|---|---|---|
| Student | My Semester, Up Next, Due Soon | No generic enterprise queue |
| Faculty | Teaching Pulse, Next Class, Today's Teaching | No generic enterprise queue |
| Guardian | Relationships, notices, authorized responsibilities | No generic enterprise queue by default |
| Institutional Administrator | Institution Pulse, exceptions, Priority Work | True work queue appropriate |
| Student/Academic/Exam/Finance/HR/Campus/Resource/Services specialists | domain-relevant pulse/attention/task summary | Work queue only where upstream human-task semantics justify it |
| Platform Administrator | platform health/governance attention and tasks | Work queue appropriate where governed workflows create tasks |

---

# 18. Open Decisions Deliberately Preserved

The following are not locked by this draft:

- final order and exact wording of every profile's primary navigation;
- exact merged navigation for multi-role users;
- whether Institutional Services is a separate broad-admin primary item;
- exact mobile-supported institutional specialist task set;
- command-palette keyboard shortcuts;
- exact Guardian self-service exposure beyond canonical relationship/responsibility functions;
- exact Student/Faculty exposure to Campus/Research/Placement/External Education based on responsibility and lifecycle;
- final local tabs inside each of the 98 business workspace families;
- permanent L6 IDs and production folder structure.

These require the traceability register and subsequent L6 slice validation.

---

# 19. Validation Against Repository Principles

This draft preserves:

- one canonical business owner per concept/entity/lifecycle;
- server-authoritative permissions and protected scope;
- L6 independence from L5 technical route/module structure;
- persona-sensitive UX without persona-as-security;
- progressive disclosure instead of a 104-item navigation dump;
- cross-domain workspace composition without cross-domain ownership transfer;
- Administrator / Faculty / Student Home asymmetry already approved by the L6 Foundation Authorization;
- explicit unresolved areas instead of invented detail.

---

# 20. Proceed Gate

**Result:** PASS — suitable as the controlled working journey/navigation map for traceability authoring.

The next required artifact is:

`L6_Screen_Journey_Traceability_Register_DRAFT.md`

That register must resolve exact L3 scenario IDs, L4 behavior specifications, L5 dependencies, actors and expected L6 surface types before individual production UI slices are authorized.

---

**Draft verdict:** SutraERP now has a repository-derived persona/task navigation architecture covering self-service, teaching, guardian, broad administration and specialist institutional work without converting UX profiles into authorization roles.
