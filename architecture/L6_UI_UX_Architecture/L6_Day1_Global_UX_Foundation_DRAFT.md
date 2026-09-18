# SutraERP L6 — Day 1 Global UX Foundation

**Version:** 0.2.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Repository State:** Day-1 baseline merged to `main`; Day-2 hardening continues under controlled branch governance.  
**Depends On:** existing L6 controlled foundation, workspace inventory, persona/journey map, experience-slice register, implementation-review framework, approved L5 client architecture, locked upstream architecture.  
**Purpose:** Govern the reusable frontend/L6 foundation for workspace mapping, UX archetypes, review evidence, shared interaction patterns and maturity reporting without redefining business behavior.

## 1. Scope and Architecture Boundary

The controlling coverage baseline remains **99 business workspace families + 6 shared L6 surfaces = 105 inventory units**. This document does not create screens, business workflows, permissions, lifecycle logic, backend contracts, data models, canonical L3/L4 traceability, or implementation authority.

The delivery model remains:

`Workspace Family → UX Journey → Experience Slice → Flow / State / Fixture / Evidence`.

## 2. UX Archetype Registry

The following twelve reusable composition patterns are adopted for Day 1. They are UX patterns only and do not alter canonical ownership or business behavior.

| Key | Pattern |
|---|---|
| `entity-360` | Entity 360 / Record Workspace |
| `search-worklist` | Search + Worklist |
| `master-detail-queue` | Master / Detail Operations Queue |
| `guided-flow` | Multi-step Transaction / Guided Flow |
| `governed-lifecycle-action` | Lifecycle / High-consequence Action |
| `roster-bulk-action` | Roster / Bulk Action |
| `schedule-calendar` | Schedule / Calendar |
| `financial-ledger` | Financial Ledger / Transaction |
| `request-document-tracking` | Request / Document Tracking |
| `configuration-master-data` | Configuration / Master Data |
| `analytics-monitoring` | Analytics / Monitoring |
| `persona-home-dashboard` | Persona Home / Dashboard |

Workspace counts are non-exclusive because a workspace may legitimately compose multiple archetypes:

| Archetype | Workspace count |
|---|---:|
| `configuration-master-data` | 22 |
| `governed-lifecycle-action` | 47 |
| `entity-360` | 36 |
| `search-worklist` | 13 |
| `guided-flow` | 21 |
| `master-detail-queue` | 48 |
| `analytics-monitoring` | 24 |
| `request-document-tracking` | 15 |
| `schedule-calendar` | 12 |
| `roster-bulk-action` | 19 |
| `financial-ledger` | 15 |
| `persona-home-dashboard` | 1 |

## 3. Complete 99-Workspace Archetype Mapping

| Workspace | Working name | Architecture source | UX archetype(s) |
|---|---|---|---|
| `W-A01` | Tenant Administration | `L2-A01` | `configuration-master-data`, `governed-lifecycle-action` |
| `W-A02` | Person & Identity | `L2-A02` | `entity-360`, `search-worklist`, `governed-lifecycle-action` |
| `W-A03` | Authentication & Sign-in | `L2-A03` | `guided-flow`, `governed-lifecycle-action` |
| `W-A04` | Access & Responsibility | `L2-A04` | `entity-360`, `configuration-master-data`, `governed-lifecycle-action` |
| `W-A05` | Workflow & Human Tasks | `L2-A05` | `master-detail-queue`, `search-worklist`, `configuration-master-data` |
| `W-A06` | Event Governance | `L2-A06` | `configuration-master-data`, `analytics-monitoring` |
| `W-A07` | Audit & Evidence | `L2-A07` | `search-worklist`, `analytics-monitoring`, `request-document-tracking` |
| `W-A08` | Capability / Module Registry | `L2-A08` | `configuration-master-data`, `governed-lifecycle-action` |
| `W-A09` | Master Data | `L2-A09` | `configuration-master-data`, `search-worklist` |
| `W-A10` | Operational Readiness | `L2-A10` | `analytics-monitoring`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-B01` | Institution Administration | `L2-B01` | `entity-360`, `configuration-master-data`, `governed-lifecycle-action` |
| `W-B02` | Organisation Structure & Positions | `L2-B02` | `entity-360`, `configuration-master-data` |
| `W-B03` | Institutional Campus Registry | `L2-B03` | `entity-360`, `configuration-master-data`, `governed-lifecycle-action` |
| `W-B04` | Curriculum Framework | `L2-B04` | `configuration-master-data`, `entity-360` |
| `W-B05` | Academic Calendar | `L2-B05` | `schedule-calendar`, `configuration-master-data`, `governed-lifecycle-action` |
| `W-C01` | Admissions | `L2-C01` | `master-detail-queue`, `guided-flow`, `governed-lifecycle-action` |
| `W-C02` | Student Workspace | `L2-C02` | `entity-360`, `governed-lifecycle-action`, `request-document-tracking` |
| `W-C03` | Academic Registration | `L2-C03` | `guided-flow`, `roster-bulk-action`, `governed-lifecycle-action` |
| `W-C04` | Academic Progression & Credits | `L2-C04` | `entity-360`, `roster-bulk-action`, `governed-lifecycle-action` |
| `W-C05` | Qualification & Exit | `L2-C05` | `guided-flow`, `governed-lifecycle-action`, `request-document-tracking` |
| `W-C06` | Guardian Relationships | `L2-C06` | `entity-360`, `guided-flow`, `governed-lifecycle-action` |
| `W-C07` | Scholarship Eligibility | `L2-C07` | `master-detail-queue`, `guided-flow`, `governed-lifecycle-action` |
| `W-C08` | Alumni Transition | `L2-C08` | `guided-flow`, `governed-lifecycle-action` |
| `W-D01` | Course Offerings | `L2-D01` | `entity-360`, `configuration-master-data`, `roster-bulk-action` |
| `W-D02` | Faculty Academic Assignment | `L2-D02` | `roster-bulk-action`, `master-detail-queue`, `schedule-calendar` |
| `W-D03` | Timetable | `L2-D03` | `schedule-calendar`, `configuration-master-data` |
| `W-D04` | Teaching Sessions & Attendance | `L2-D04` | `schedule-calendar`, `roster-bulk-action`, `master-detail-queue` |
| `W-D05` | Academic Monitoring | `L2-D05` | `analytics-monitoring`, `master-detail-queue`, `entity-360` |
| `W-D06` | Flexible Learning / Electives / MOOCs | `L2-D06` | `search-worklist`, `guided-flow`, `entity-360` |
| `W-D07` | Internal Assessment | `L2-D07` | `roster-bulk-action`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-E01` | Workforce Engagement | `L2-E01` | `entity-360`, `guided-flow`, `governed-lifecycle-action` |
| `W-E02` | Position & Assignment | `L2-E02` | `roster-bulk-action`, `entity-360`, `governed-lifecycle-action` |
| `W-E03` | Faculty Workforce | `L2-E03` | `entity-360`, `search-worklist`, `analytics-monitoring` |
| `W-E04` | HR Services & Cases | `L2-E04` | `master-detail-queue`, `request-document-tracking`, `governed-lifecycle-action` |
| `W-E05` | Leave Management | `L2-E05` | `guided-flow`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-E06` | Payroll | `L2-E06` | `financial-ledger`, `roster-bulk-action`, `governed-lifecycle-action` |
| `W-E07` | Employee Profile | `L2-E07` | `entity-360`, `request-document-tracking` |
| `W-F00` | Finance Control & Configuration | `L2-F00` | `configuration-master-data`, `analytics-monitoring` |
| `W-F01` | Student Fees & Charges | `L2-F01` | `financial-ledger`, `entity-360`, `governed-lifecycle-action` |
| `W-F02` | Payments & Receipts | `L2-F02` | `financial-ledger`, `guided-flow`, `master-detail-queue` |
| `W-F03` | Refunds | `L2-F03` | `financial-ledger`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-F04` | Financial Adjustments | `L2-F04` | `financial-ledger`, `governed-lifecycle-action` |
| `W-F05` | General Ledger | `L2-F05` | `financial-ledger`, `search-worklist`, `analytics-monitoring` |
| `W-F06` | Budgets & Cost Centres | `L2-F06` | `financial-ledger`, `configuration-master-data`, `analytics-monitoring` |
| `W-F07` | Accounts Receivable | `L2-F07` | `financial-ledger`, `master-detail-queue`, `analytics-monitoring` |
| `W-F08` | Accounts Payable | `L2-F08` | `financial-ledger`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-F09` | Financial Periods & Closing | `L2-F09` | `financial-ledger`, `governed-lifecycle-action`, `configuration-master-data` |
| `W-F10` | Treasury & Banking | `L2-F10` | `financial-ledger`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-F11` | Financial Aid | `L2-F11` | `financial-ledger`, `master-detail-queue`, `entity-360` |
| `W-F12` | Asset Accounting | `L2-F12` | `financial-ledger`, `entity-360`, `governed-lifecycle-action` |
| `W-F13` | Financial Reports & Analytics | `L2-F13` | `analytics-monitoring`, `financial-ledger` |
| `W-F14` | Procurement Finance | `L2-F14` | `financial-ledger`, `master-detail-queue` |
| `W-G00` | Campus Operations Governance | `L3-G-001…015` | `analytics-monitoring`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-G01` | Facilities | `L2-G01` | `master-detail-queue`, `request-document-tracking` |
| `W-G02` | Laboratories | `L2-G02` | `entity-360`, `schedule-calendar`, `master-detail-queue` |
| `W-G03` | Hostel | `L2-G03` | `entity-360`, `master-detail-queue`, `guided-flow` |
| `W-G04` | Transport | `L2-G04` | `schedule-calendar`, `entity-360`, `master-detail-queue` |
| `W-G05` | Library | `L2-G05` | `search-worklist`, `entity-360`, `master-detail-queue` |
| `W-G06` | Health / Medical Services | `L2-G06` | `master-detail-queue`, `entity-360`, `request-document-tracking` |
| `W-G07` | Security Operations | `L2-G07` | `master-detail-queue`, `analytics-monitoring` |
| `W-G08` | Housekeeping & Service Governance | `L2-G08` | `master-detail-queue`, `analytics-monitoring` |
| `W-G09` | Teacher Guardian Scheme | `L2-G09` | `entity-360`, `master-detail-queue`, `schedule-calendar` |
| `W-G10` | Infrastructure Management | `L2-G10` | `entity-360`, `configuration-master-data`, `master-detail-queue` |
| `W-H01` | APAAR & DigiLocker | `L2-H01` | `entity-360`, `request-document-tracking`, `governed-lifecycle-action` |
| `W-H02` | Notifications | `L2-H02` | `configuration-master-data`, `analytics-monitoring` |
| `W-H03` | Enterprise Directory & Search | `L2-H03` | `search-worklist`, `entity-360` |
| `W-H04` | Placement | `L2-H04` | `search-worklist`, `master-detail-queue`, `guided-flow` |
| `W-H05` | Research | `L2-H05` | `entity-360`, `master-detail-queue`, `request-document-tracking` |
| `W-H06` | Alumni Engagement | `L2-H06` | `entity-360`, `search-worklist`, `guided-flow` |
| `W-H07` | Enterprise Dashboards | `L2-H07` | `persona-home-dashboard`, `analytics-monitoring`, `configuration-master-data` |
| `W-H08` | Statutory / Regulatory Reporting | `L2-H08` | `analytics-monitoring`, `request-document-tracking`, `master-detail-queue` |
| `W-H09` | Accreditation & Compliance | `L2-H09` | `analytics-monitoring`, `request-document-tracking`, `master-detail-queue` |
| `W-H10` | External Education Integrations | `L2-H10` | `master-detail-queue`, `analytics-monitoring`, `configuration-master-data` |
| `W-I01` | Examination Policy & Configuration | `L2-I01` | `configuration-master-data`, `governed-lifecycle-action` |
| `W-I02` | Examination Sessions | `L2-I02` | `entity-360`, `schedule-calendar`, `governed-lifecycle-action` |
| `W-I03` | Examination Planning & Scheduling | `L2-I03` | `schedule-calendar`, `roster-bulk-action`, `configuration-master-data` |
| `W-I04` | Examination Registration | `L2-I04` | `roster-bulk-action`, `guided-flow`, `master-detail-queue` |
| `W-I05` | Hall Tickets | `L2-I05` | `request-document-tracking`, `roster-bulk-action` |
| `W-I06` | Practical & Viva Conduct | `L2-I06` | `roster-bulk-action`, `schedule-calendar`, `master-detail-queue` |
| `W-I07` | Theory Examination Conduct | `L2-I07` | `roster-bulk-action`, `schedule-calendar`, `master-detail-queue` |
| `W-I08` | Marks & Evaluation | `L2-I08` | `roster-bulk-action`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-I09` | Result Processing | `L2-I09` | `roster-bulk-action`, `analytics-monitoring`, `governed-lifecycle-action` |
| `W-I10` | Result Publication | `L2-I10` | `governed-lifecycle-action`, `analytics-monitoring` |
| `W-I11` | Revaluation & Rechecking | `L2-I11` | `master-detail-queue`, `request-document-tracking`, `governed-lifecycle-action` |
| `W-I12` | Supplementary / Backlog / Improvement Exams | `L2-I12` | `roster-bulk-action`, `schedule-calendar`, `master-detail-queue` |
| `W-I13` | Academic Completion & CGPA Evidence | `L2-I13` | `request-document-tracking`, `analytics-monitoring`, `entity-360` |
| `W-I14` | Malpractice & Examination Committee | `L2-I14` | `master-detail-queue`, `request-document-tracking`, `governed-lifecycle-action` |
| `W-I15` | Examination Exceptions & Contingency | `L2-I15` | `master-detail-queue`, `governed-lifecycle-action` |
| `W-I16` | Examination Closure & Archive | `L2-I16` | `governed-lifecycle-action`, `analytics-monitoring` |
| `W-J01` | Procurement Planning & Sourcing | `L2-J01` | `guided-flow`, `master-detail-queue`, `analytics-monitoring` |
| `W-J02` | Vendors & Suppliers | `L2-J02` | `entity-360`, `search-worklist`, `governed-lifecycle-action` |
| `W-J03` | Purchase Requisitions & Approvals | `L2-J03` | `guided-flow`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-J04` | Purchase Orders | `L2-J04` | `entity-360`, `guided-flow`, `governed-lifecycle-action` |
| `W-J05` | Goods Receipt & Inspection | `L2-J05` | `guided-flow`, `master-detail-queue`, `governed-lifecycle-action` |
| `W-J06` | Inventory & Warehouse | `L2-J06` | `entity-360`, `search-worklist`, `roster-bulk-action` |
| `W-J07` | Stock Operations & Distribution | `L2-J07` | `roster-bulk-action`, `guided-flow`, `master-detail-queue` |
| `W-J08` | Asset Lifecycle | `L2-J08` | `entity-360`, `governed-lifecycle-action`, `master-detail-queue` |
| `W-J09` | Physical Verification & Reconciliation | `L2-J09` | `roster-bulk-action`, `analytics-monitoring`, `master-detail-queue` |
| `W-J10` | Disposal / Scrap / Write-Off | `L2-J10` | `guided-flow`, `governed-lifecycle-action`, `master-detail-queue` |

This mapping is complete for the current 99-family baseline. It is not a route count, screen count, permission matrix, or replacement traceability store.

## 4. Maturity Dimensions

Frontend reporting must keep maturity dimensions separate. Allowed UX/review states are:

- `UX-MAPPED`
- `SHELL-IMPLEMENTED`
- `INTERACTIVE-FIXTURE-IMPLEMENTED`
- `UX-REVIEWED`
- `INTEGRATION-PENDING`
- `INTEGRATED`
- `PRODUCTION-VALIDATED`

Integration maturity is separate:

- `presentation-only`
- `contract-backed`
- `integrated`

Day 1 rule: all workspace families remain `UX-MAPPED` except where live implementation evidence supports a stronger state. W-C02 remains an interactive fixture-backed reference implementation and is **not** represented as production-integrated.

## 5. Global Experience Lab

The existing W-C02 UI Review Mode is generalized into the **Global Experience Lab**.

Required selector order:

`Workspace → Journey / Experience Slice → Presentation Profile → Fixture → Review Case`

Only curated valid combinations may be exposed. Review tooling shall remain visibly non-production, deterministic, build/config controlled, impossible to enable by a normal URL query parameter alone, based on production-intended components, unable to grant authorization, and unable to perform real institutional mutations from fixture controls.

Each case records stable case identity, workspace, journey/slice, presentation profile, fixture identity/fidelity, integration maturity, architecture revision, frontend revision, case revision and deterministic reset behavior.

Internal L3/L4/L6 identifiers may appear inside Experience Lab tooling and evidence but shall not be exposed as normal end-user UX.

## 6. Fixture Fidelity and Provenance

Allowed fidelity classes remain:

1. **presentation fixture**
2. **contract-backed integration fixture**
3. **integrated environment data**

Every fixture records identity, provenance, fidelity, supported presentation profile, workspace/slice association and deterministic reset behavior.

Presentation fixtures must not invent lifecycle states, business events, approval actors, permissions, eligibility rules, calculations, backend endpoints/payloads, or canonical ownership.

## 7. Shared Interaction Patterns

### State surfaces
Reusable presentation primitives may cover loading, legitimate empty, authorization denied, degraded source, stale/late-response containment, pending, business rejection, technical failure and indeterminate outcome. Applicability is case-specific; no combinatorial state generator is authorized.

### Governed Action
The shared primitive may present action identity, subject/current context, source-provided consequence, integration note, pending, authoritative rejection, technical failure, indeterminate outcome and safe repeat suppression. It shall not own eligibility, permission, approval, business validation, lifecycle transition, success inference, API schema or retry/idempotency rules.

### Owner-Preserving Handoff
The shared primitive must state that the source workspace is showing a projection, identify the source owner, preserve subject/context, state whether the destination is implemented, and provide a meaningful return. Navigation never transfers ownership and unavailable destinations are presented honestly.

## 8. W-C02 Preservation

W-C02 remains the reference proof, not a special permanent framework. Generalization is limited to reusable interaction mechanics.

The following remain Student-specific/upstream-owned and must not be generalized into shared business semantics:

- Student Lifecycle rules;
- staff versus Student composition;
- C-012 versus C-013 meaning;
- Person & Identity ownership boundary;
- document rendering versus authoritative issuance;
- Student-specific lifecycle actions;
- authoritative lifecycle transitions and eligibility.

## 9. Frontend Structure and Validation

Preferred additive structure:

```text
app/registry/
app/review/
app/components/
app/features/<workspace>/
```

Do not reorganize the whole frontend merely to fit this convention.

Static validation must protect:

- exactly 99 business workspace entries;
- unique workspace keys;
- at least one known archetype per workspace;
- allowed maturity values;
- build/config-gated Experience Lab;
- no simple URL review-mode enablement;
- no fabricated authoritative production state;
- existing W-C02 review-case continuity;
- fixture provenance/reset metadata;
- current readability-floor guard.

## 10. Accessibility / Responsive Foundation

Shared primitives shall use semantic controls, keyboard-operable interactions, visible focus, dialog focus restoration, status/error announcements, non-color-only meaning, responsive composition, readable supporting text, reduced-motion compatibility where animation exists, and usable touch targets.

Static inspection is evidence only. **WCAG conformance is not claimed without browser/assistive-technology validation.**

## 11. Day-1 Status

- Workspace registry definition: established.
- UX archetype registry: established.
- 99-workspace mapping: complete.
- Experience Lab convention: established; initial operational case population remains W-C02 only.
- Shared state/governed-action/handoff conventions: established.
- Fixture/integration maturity conventions: established.
- Frontend feature-module direction: established additively.
- Static validation rules: established.
- Browser accessibility/responsive acceptance: pending evidence.
- Released-contract integration beyond existing repository evidence: not claimed.


## 12. Day-2 Foundation Hardening

Day 2 completes the second half of the global foundation phase without beginning broad bespoke workspace implementation.

### Reusable archetype implementation layer

The frontend now binds every approved UX archetype to a reusable presentation/composition primitive. These primitives define layout and interaction composition only. They do not define business rules, eligibility, authority, lifecycle transitions, events, APIs, payloads, or canonical ownership.

The shared composition set includes:

- Workspace Frame;
- Entity 360 layout;
- Search / Worklist layout;
- Master / Detail layout;
- Guided Flow layout;
- Governed Action primitive;
- Roster layout;
- Schedule layout;
- Ledger layout;
- Request / Tracking layout;
- Configuration layout;
- Analytics layout;
- Persona Home layout.

A workspace may compose several of these patterns where the Day-1 mapping requires it.

### Standard feature-module contract

Implemented workspaces use a typed frontend module registration seam keyed only by the existing global workspace registry.

The module contract does not create routes, permissions, lifecycle rules, integration contracts, or architecture traceability. It exists only to make frontend composition predictable and prevent ad hoc workspace registration.

W-C02 is the first existing deep implementation adapted to this module contract. This adaptation does not change W-C02 business semantics.

### Accessibility and responsive hardening

The shared frontend foundation now protects:

- visible keyboard focus;
- global reduced-motion behavior;
- a 12px minimum supporting-text floor across loaded application styles;
- responsive archetype collapse behavior;
- 40px general interactive-control minimums for governed shared patterns;
- 44px minimum targets for coarse-pointer/touch contexts;
- existing dialog focus restoration and semantic native controls.

These are implementation safeguards. They are not a claim of WCAG conformance. Browser, assistive-technology and device evidence remain separately required.

### Experience Lab synchronization

The canonical non-production tooling name is **Global Experience Lab**.

The build/config flag is:

`VITE_ENABLE_EXPERIENCE_LAB`

The older W-C02 `ReviewModePanel` naming remains only as a compatibility bridge where needed by the current route composition. It does not create a second review framework.

### Day-2 static protection

Frontend validation now additionally requires:

- exactly 12 archetypes with reusable implementation bindings;
- exactly 99 unique workspace registrations;
- presentation profile, user-goal, navigation and archetype metadata for every workspace;
- typed workspace keys;
- typed feature-module registration;
- W-C02 registration through the common module seam;
- the Global Experience Lab build/config boundary;
- no simple URL-based review enablement;
- no deprecated review-mode environment flag;
- loaded global foundation CSS;
- no 8–11px supporting-text regression in loaded application styles;
- global visible-focus and reduced-motion rules;
- coarse-pointer touch-target hardening;
- W-C02 case and production-truth safeguards.

### Phase exit interpretation

The Days 1–2 foundation phase is complete when repository CI confirms the shared registry, archetype layer, feature-module contract, Experience Lab, state/action/handoff primitives, responsive/accessibility safeguards and W-C02 regression controls all typecheck and build successfully.

This phase exit does **not** mean:

- all 99 workspaces are implemented;
- the 99 workspace families are shell-complete;
- browser accessibility acceptance is complete;
- backend integration exists;
- fixture outcomes are production truth.

The next execution phase remains the planned **Reference UX System** work using representative Student, Academic Registration, Teaching/Attendance, Examination, Finance, Timetable, Documents and Institution Administration journeys.
