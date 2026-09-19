# SutraERP L6 — Days 9–11 Deep Backend-Defining Journeys

**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Architecture Base:** `1f26916be781e9dff1035c30d06a81ba36246a84`  
**Frontend Base:** `2bfd8566e92050e9e5de47a58a00271e2d5842b2`  
**Purpose:** Deepen selected high-value L6 journeys so future backend planning can derive information, authority, outcome, handoff and reconciliation needs without L6 defining APIs, schemas, business rules or source-owned decisions.

## 1. Boundary

This phase remains L6.

It may define:

- information a user must see;
- source-owned context required by an interaction;
- review-before-submit and consequence presentation;
- supported business-interaction classes;
- presentation of source-owned outcome classes;
- reconciliation-oriented UX where the authoritative result is unknown;
- owner-preserving handoffs;
- deterministic review fixtures and evidence.

It shall not define:

- endpoints, HTTP methods, payloads or JSON;
- database fields or schemas;
- backend services/components;
- business eligibility, calculations, approval policy or lifecycle states;
- retry/idempotency mechanisms;
- permission codes or technical RBAC.

## 2. Live Baseline

The requested handoff SHAs were rechecked and remained the live `main` heads before branch creation:

- architecture: `1f26916be781e9dff1035c30d06a81ba36246a84`;
- frontend: `2bfd8566e92050e9e5de47a58a00271e2d5842b2`.

Days 1–8 baseline preserved:

- 99 business workspace families;
- 6 shared L6 surfaces;
- 105 L6 inventory units;
- 12 UX archetypes;
- 1,084 canonical L3 scenarios;
- 99 breadth traces;
- 7 existing interactive fixture implementations;
- 92 shell implementations;
- 99 static-reviewed;
- 0 UX-reviewed;
- 0 integrated;
- 0 production-validated;
- reviewer/internal architecture metadata isolated to reviewer tooling.

## 3. Selected Deep Set

The final Days 9–11 deep set is **18 workspace families total**, including all seven prior deep/reference implementations.

| Workspace | Scenario span | Reason |
|---|---|---|
| W-B01 Institution Administration | B:001–005 | Existing configuration/master-data reference retained as regression baseline. |
| W-C01 Admissions | C:001–007 | Exposes application/evidence/outcome and Student Lifecycle handoff needs. |
| W-C02 Student Workspace | C:001,008–019 | Existing controlled deep Student entity/lifecycle reference retained. |
| W-C03 Academic Registration | C:020–026 | High-value eligibility/choice/submission/outcome boundary. |
| W-C04 Academic Progression & Credits | C:027–038,075 | Exposes official-result inputs, credit context and progression authority. |
| W-D03 Timetable | D:020–022 | Multi-persona schedule/read continuity and source-unavailable behavior. |
| W-D04 Teaching Sessions & Attendance | D:023–026 | High-frequency roster/bulk governed action with consequential outcomes. |
| W-D07 Internal Assessment | D:051–057 | Repeated candidate entry/review without client-owned assessment policy. |
| W-E05 Leave Management | E:044–052 | Representative governed workforce transaction outside student/finance/exam. |
| W-E07 Employee Profile | E:058–070,075 | Employee Entity 360 and Person/Identity ownership preservation. |
| W-F01 Student Fees & Charges | F:001–006 | Finance-owned obligations and payment/adjustment continuation. |
| W-F02 Payments & Receipts | F:007–013 | High-consequence monetary intent, indeterminate outcome and reconciliation. |
| W-F03 Refunds | F:014–019 | Distinct monetary reversal with entitlement and reconciliation boundary. |
| W-F04 Financial Adjustments | F:020–024 | Governed financial change without local formulas/posting logic. |
| W-I03 Examination Planning & Scheduling | I:007–016 | Planning/resource/conflict presentation without scheduling-engine behavior. |
| W-I04 Examination Registration | I:017–036 | Candidate availability/submission/outcome without eligibility invention. |
| W-I08 Marks & Evaluation | I:071–082 | Existing authorized evaluator reference; no Student presentation. |
| W-I09 Result Processing | I:083–100 | Source-owned result processing without browser computation/publication semantics. |

### Selection note

W-E06 Payroll was not added. W-E05 already proves a People/Workforce governed transaction while avoiding unnecessary exposure to payroll-calculation semantics. Payroll remains available for a later slice when stronger integration evidence is required.

## 4. Deep Journey Pattern

The reusable interaction pattern is:

`enter → inspect context → inspect source-owned information → review supported intent/consequence → submit fixture intent → present source-owned outcome → reconcile if indeterminate → owner-preserving continuation`.

Not every journey mechanically uses every state.

Read-oriented journeys such as Timetable, Fees/Charges orientation and Employee Profile use source-unavailable/stale-context behavior rather than fabricated mutation failures.

## 5. Experience Slice Definitions

| Workspace | Journey | Experience slice |
|---|---|---|
| W-C01 | Application progression | context/evidence/next-work/outcome |
| W-C03 | Academic registration | source choices/review/submit/outcome |
| W-C04 | Progression & credits | progression/credit context and authoritative outcome |
| W-D03 | Timetable orientation | day/week/session/continuation |
| W-D04 | Attendance session | roster/exceptions/review/submit/outcome |
| W-D07 | Internal assessment | assessment/roster/entry-review/outcome |
| W-E05 | Leave request | context/availability/request/outcome |
| W-E07 | Employee Entity 360 | employment context/projections/owner handoff |
| W-F01 | Fees & charges | charge/history/payment-or-adjustment continuation |
| W-F02 | Payment | context/intent/outcome/reconciliation/receipt boundary |
| W-F03 | Refund request | refundable-position/request/outcome/reconciliation |
| W-F04 | Financial adjustment | source context/proposed intent/outcome |
| W-I03 | Examination planning | plan/resource/conflict/governed continuation |
| W-I04 | Examination registration | availability/review/submit/outcome |
| W-I08 | Marks & evaluation | assigned evaluation/context/intent/outcome |
| W-I09 | Result processing | processing inputs/exceptions/governed outcome |
| W-B01 | Institution configuration | context/governed-change outcome |
| W-C02 | Student lifecycle/entity 360 | existing controlled W-C02 slice corpus |

## 6. Outcome and Reconciliation Rules

High-consequence intent journeys may present:

- pending;
- business rejection;
- technical failure;
- indeterminate;
- confirmed authoritative source outcome where a fixture explicitly represents source confirmation.

An indeterminate fixture must not expose a normal repeat action. The user is told that the result requires authoritative reconciliation.

No client action, timeout, navigation or local state may manufacture authoritative success.

## 7. Owner-Preserving Handoffs

Material handoffs include:

- Admissions → Student Lifecycle after an approved admission outcome;
- Timetable → Teaching Session/Attendance for session execution;
- Student Fees & Charges → Payments & Receipts or Financial Adjustments;
- Employee Profile → Person & Identity, Leave or Payroll as owned;
- Examination evaluation/result-processing boundaries remain within Examination while publication remains separate;
- Academic Progression consumes official Examination outputs without transferring result ownership.

Navigation never transfers canonical ownership.

## 8. Reuse Strategy

No thirteenth archetype is introduced.

Days 9–11 reuse:

- WorkspaceFrame;
- guided-flow;
- roster-bulk-action;
- schedule-calendar;
- financial-ledger;
- master-detail-queue;
- GovernedAction;
- shared outcome-state surfaces;
- deterministic Experience Lab controls;
- typed workspace-module registration.

The 11 newly deepened families use a shared data-driven deep-workspace module rather than bespoke mini-apps.

## 9. Frontend Maturity Target

After successful implementation and CI:

- `INTERACTIVE-FIXTURE-IMPLEMENTED`: 18;
- `SHELL-IMPLEMENTED`: 81;
- `STATIC-REVIEWED`: 99;
- `UX-REVIEWED`: 0 unless genuine browser/AT evidence is recorded;
- integration remains presentation-only except where released evidence separately proves otherwise;
- production validation remains 0.

## 10. Integration Boundary

W-C02 retains its previously approved L4→L5 traceability evidence.

The other deep families remain suitable for backend-readiness evidence but do not claim released integration contracts. Their current unresolved integration requirement is recorded in the Backend-Readiness Interaction Evidence Register rather than repaired by L6.

## 11. Exit Gate

Days 9–11 pass only if:

- exactly 99 workspace families and 99 breadth traces remain;
- exactly 12 archetypes remain;
- all 18 deep families are typed and fixture-reviewable;
- W-C02 and W-I08 profile boundaries remain intact;
- Finance never infers monetary success;
- indeterminate high-consequence cases constrain unsafe repeat;
- ordinary UX does not expose architecture/reviewer metadata;
- required CI is green on the final frontend branch head;
- no architecture hard fail remains.

This artifact does not authorize Final L6 Corpus Lock.
