# SutraERP L6 — Days 3–5 Reference UX System

**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Repository Base:** architecture `main` at `e0cb58fe0e22dc32b53dca2438ea6f17ecf5e38c`  
**Frontend Base:** frontend `main` at `f14a54d095280ca82214db7fa7f7d9fe3c924cad`  
**Purpose:** Prove the reusable Days 1–2 UX foundation across materially different business domains without redefining L0–L5 architecture.

## 1. Controlled Reference Set

The Days 3–5 reference system uses exactly these business workspace families:

| Workspace | Reference role | Primary architecture support | Primary UX archetypes |
|---|---|---|---|
| `W-C02` Student Workspace | Existing Entity 360 and request/document reference; regression baseline | L2-C02; L3/L4 C:001,008–019; approved W-C02 L6 gate | entity-360; governed-lifecycle-action; request-document-tracking |
| `W-C03` Academic Registration | Guided transaction and review-before-submit | L2-C03; L3/L4 C:020–026 | guided-flow; roster-bulk-action; governed-lifecycle-action |
| `W-D04` Teaching Sessions & Attendance | Faculty operational roster, exceptions and bulk-action presentation | L2-D04; L3/L4 D:023–026 | schedule-calendar; roster-bulk-action; master-detail-queue |
| `W-D03` Timetable | Multi-profile schedule/calendar reference | L2-D03; L3/L4 D:020–022 | schedule-calendar; configuration-master-data |
| `W-I08` Marks & Evaluation | Examination governed operational reference | L2-I08; L3/L4 I:071–082 | roster-bulk-action; master-detail-queue; governed-lifecycle-action |
| `W-F02` Payments & Receipts | Finance-owned transaction/ledger reference | L2-F02; L3/L4 F:007–013 | financial-ledger; guided-flow; master-detail-queue |
| `W-B01` Institution Administration | Institution configuration/master-data reference | L2-B01; L3/L4 B:001–010 | entity-360; configuration-master-data; governed-lifecycle-action |

No new Documents workspace is created. Request/document tracking remains proven through W-C02 and its existing C-018 document/certificate request boundary.

### Examination selection rationale

`W-I08 — Marks & Evaluation` is selected instead of inventing or choosing a result workspace by label alone because it:

- is already an approved workspace family;
- has resolved L2/L3/L4 support;
- is operational rather than dashboard-only;
- exercises roster/bulk, master/detail and governed-action composition;
- exposes high-consequence outcome semantics without requiring the client to own result computation or publication;
- remains materially distinct from Registration, Attendance and Finance.

## 2. Architecture-Grounded Primary User Goals

These goals are presentation outcomes only. They do not create business rules or authority.

| Workspace | Primary user goal |
|---|---|
| W-C02 | Understand the current Student Context, source-owned related status, supported lifecycle/request options and preserved history without losing domain ownership boundaries. |
| W-C03 | Register or amend a Student's academic participation within the authoritative registration context, review the proposed selection, and follow the source-owned registration outcome without client-side eligibility decisions. |
| W-D04 | Enter the scheduled teaching context, work through the applicable roster, capture attendance intent and exceptions, and observe the authoritative attendance outcome without client-owned attendance policy. |
| W-D03 | Understand the current timetable for the selected Student, Faculty or Academic Operations context and continue to the owning academic workspace when further action is required. |
| W-I08 | Work through assigned marks/evaluation activity, maintain clear assessment/candidate context, and submit or inspect governed evaluation outcomes while Examination retains authority. |
| W-F02 | Understand payer/payment context, initiate a supported payment intent, and follow Finance-owned allocation, receipt, failure or reconciliation outcomes without assuming monetary success. |
| W-B01 | Inspect and maintain Institution profile and policy/configuration context through governed change presentation while Institution remains authoritative. |

Quality convention for future Days 6–8 work: a `primaryUserGoal` must name the user's intended business outcome, the authoritative context that constrains it where material, and any ownership/authority boundary that prevents the client from becoming the business decision maker. Generic “work with X” placeholder goals are invalid for implemented workspaces.

## 3. Reference Workspace Standard

Each implemented reference workspace shall declare:

- canonical workspace identity and source;
- supported presentation profiles;
- architecture-grounded primary user goal;
- reused archetypes;
- major user tasks and entry paths;
- material handoffs;
- action category;
- applicable shared state classes;
- fixture provenance/fidelity/reset behavior;
- implementation, review, integration and production-validation evidence separately;
- explicit unavailable/integration-pending boundaries.

The client shall not add locally invented eligibility, lifecycle, approval, calculation, payment, attendance, examination or policy semantics.

## 4. Maturity Evidence

The Days 3–5 implementation follows the controlled refinement recorded in `L6_Day1_Global_UX_Foundation_DRAFT.md`:

| Evidence dimension | Controlled vocabulary |
|---|---|
| Implementation | UX-MAPPED; SHELL-IMPLEMENTED; INTERACTIVE-FIXTURE-IMPLEMENTED |
| UX review | NOT-REVIEWED; STATIC-REVIEWED; UX-REVIEWED |
| Integration | presentation-only; contract-backed; integrated |
| Production validation | not-validated; production-validated |

Static code review or CI does not justify `UX-REVIEWED`. Fixture interaction does not justify `integrated`. No Days 3–5 reference may report `production-validated`.

## 5. Experience Lab Architecture

The Global Experience Lab remains:

- non-production;
- build/config gated;
- deterministic;
- fixture-provenance aware;
- unable to grant authorization;
- unable to perform real institutional mutation;
- restricted to curated cases;
- based on production-intended components.

Days 3–5 shall add curated cases for W-C03, W-D03, W-D04, W-I08, W-F02 and W-B01 while preserving all W-C02 cases. Normal user UX shall not expose internal workspace IDs.

## 6. Reuse and Composition

No new UX archetype is authorized by this reference phase. The 12-archetype baseline remains unchanged.

Reference implementations may strengthen reusable composition primitives, but shared components shall remain semantically neutral. Business-specific meaning remains in the owning feature configuration/module.

## 7. State and Outcome Rules

Reference workspaces reuse the shared loading, empty, denied, degraded, pending, business-rejection, technical-failure, indeterminate and stale-context surfaces only where the upstream journey makes the state meaningful.

A presentation fixture may demonstrate a state but shall be visibly non-authoritative. In ordinary end-user mode, fixture submission shall never be presented as authoritative completion.

## 8. Navigation and Presentation

Reference workspaces must be reachable through task/domain-oriented shell navigation or search/discovery without listing all 99 workspaces.

Presentation profile is not authorization.

- W-D03 proves Student, Faculty and Academic Operations-oriented schedule presentation.
- W-D04 is Faculty-first operational presentation.
- W-C03 supports Student and operations presentation.
- W-F02 supports Finance Operations plus Student/payer participation where upstream architecture supports it.
- W-I08 supports Examination Operations and authorized evaluator presentation.
- W-B01 remains administrative.
- W-C02 retains its existing staff and Student profiles.

## 9. Responsive and Accessibility Completion Conditions

Static implementation must preserve:

- semantic headings and regions;
- visible focus;
- keyboard-operable controls;
- associated labels;
- status announcements for asynchronous/outcome states;
- no color-only meaning;
- coarse-pointer touch targets;
- reduced-motion handling;
- readable type and reflow.

Reference layouts shall avoid compressed desktop tables on narrow screens. Roster/master-detail/ledger structures must reduce to readable stacked/context-preserving presentations.

Actual browser and assistive-technology review is separate evidence. Until performed, reference workspaces remain below `UX-REVIEWED`.

## 10. Days 3–5 Validation Gate

The reference phase may pass only if:

- workspace registry remains 99 unique families;
- archetype registry remains 12;
- all implemented references use typed module registration;
- W-C02 remains intact;
- Experience Lab includes all implemented reference workspaces and complete fixture metadata;
- implemented-reference placeholder goals are eliminated;
- normal mode cannot fabricate authoritative success;
- review tooling remains build/config gated and not URL-enabled;
- maturity evidence remains dimensionally honest;
- CI runs `npm ci`, `npm run review:validate`, `npm run typecheck`, and `npm run build` successfully.

## 11. Current Integration Boundary

All newly added Days 3–5 reference implementations are presentation-only unless a released contract is explicitly present and verified. This document does not define APIs, payloads, endpoints, retry semantics, authorization rules or backend state transitions.

## 12. Completion Record

Final counts, workflow-run evidence, commit SHA, browser evidence and unresolved findings shall be recorded after frontend validation. Until then this artifact remains a controlled Draft and does not authorize merge or Final L6 Corpus Lock.
