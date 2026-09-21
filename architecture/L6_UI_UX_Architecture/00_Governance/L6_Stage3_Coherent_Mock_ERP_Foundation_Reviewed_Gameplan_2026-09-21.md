# SutraERP L6 — Stage 3 Coherent Mock ERP Foundation — Reviewed Gameplan — 2026-09-21

**Version:** 1.0.0  
**Status:** Architecture Approved — Planning Baseline  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Stage:** Stage 3 — Coherent Mock ERP Foundation  
**Execution State:** PLAN READY; IMPLEMENTATION NOT STARTED  
**Depends On:** `L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md`; `../L6_Full_Workflow_Realization_and_Mock_ERP_Execution_Plan_DRAFT.md`; `../L6_Workflow_Realization_Register_DRAFT.md`; Stage-2 Group-15 master reconciliation; current frontend Days 12–13 hardening baseline.

---

## 1. Decision

Stage 3 shall establish one deterministic, coherent, frontend-only fictional institutional world that later workflow-discovery and domain mock-operation stages can safely consume.

Stage 3 is a foundation stage. It shall not implement the complete 595-workflow population and shall not redefine L0–L5 semantics.

The controlling implementation principle is:

```text
one deterministic institutional seed
        ↓
one frontend demonstration state
        ↓
domain-owned projections/selectors
        ↓
existing 99 workspace families / 12 UX archetypes
```

The frontend shall not become a second business-architecture authority.

---

## 2. Verified Starting Baseline

Architecture `main` was verified at:

```text
35d5134b21ff64204663cf369127d619605c15b9
```

That commit contains the Stage-2 final reconciliation and PASS.

Frontend `main` was verified at:

```text
e2560877eac4446b925e45f04b7dfa4c24d3625c
```

That commit is the Days 12–13 global UX-hardening baseline.

Stage-2 canonical realization state:

```text
Canonical scenarios                    1,084 / 1,084
Unique primary assignments             1,084
Missing scenario IDs                       0
Duplicate primary assignments              0
Final L6 realization workflows           595
Material unresolved blockers                0
```

Workflow-level population:

```text
REVIEW-APPROVAL-WORKFLOW               271
INTERACTIVE-WORKFLOW                   154
EXCEPTION-RECOVERY-WORKFLOW            103
CROSS-WORKSPACE-HANDOFF                 28
SYSTEM-BACKGROUND-OBSERVABLE            32
TRACKING-WORKFLOW                        6
NO-DIRECT-END-USER-INTERACTION           1
TOTAL                                  595
```

Workflow-level mock realization:

```text
REQUIRED                               551
NOT-REQUIRED                            11
PROHIBITED                              33
TOTAL                                  595
```

Scenario-level mock realization:

```text
REQUIRED                             1,014
NOT-REQUIRED                            19
PROHIBITED                              51
TOTAL                                1,084
```

---

## 3. Current Frontend Findings

The frontend already provides useful review infrastructure and shall be extended rather than replaced.

Verified reusable foundations include:

- React 19.3 + TypeScript 5.9 + React Router 8.4 Framework Mode;
- npm / `package-lock.json`;
- SPA-first web client;
- 99 workspace families;
- 12 reusable UX archetypes;
- 81 shell-level workspace implementations;
- 18 interactive fixture-backed workspaces;
- deterministic fixture metadata and curated review cases;
- Global Experience Lab;
- existing presentation-only local interactions;
- reusable outcome-state semantics;
- reviewer/end-user metadata separation;
- no established Redux/Zustand-style application store that Stage 3 must preserve;
- no established browser-persistence architecture that Stage 3 must preserve.

Existing fixtures already include recurring fictional subjects such as:

- Sutra Institute of Technology;
- Aarav Desai;
- Dr. Meera Shah;
- current academic-period and timetable examples.

These should be reused where architecture-compatible instead of replacing them with a separate fictional world.

---

## 4. Stage-3 Target Responsibilities

Stage 3 shall establish the following conceptual responsibilities:

1. deterministic fictional institution seed;
2. stable fictional identities and stable mock references;
3. domain-owned mock facts aligned to canonical ownership;
4. cross-domain reference relationships;
5. reusable read/projection/selectors for workspace consumption;
6. controlled frontend-only mutable demonstration state;
7. deterministic reset;
8. one deterministic mock clock;
9. seeded tasks, notifications, exceptions and history/activity;
10. reviewer/demo controls through the existing Experience Lab;
11. deterministic integrity validation.

Physical folder names are implementation choices and shall not be treated as business architecture.

A compact structure such as the following is preferred over independent per-screen fixture islands:

```text
app/
  mock/
    seed/
    selectors/
    state/
    presets/
    validation/
```

Equivalent organization is acceptable if these responsibilities remain explicit.

---

## 5. Fictional Institution Model

The initial institutional seed shall be deliberately small but relationally complete.

It should contain enough supported records to demonstrate the same subjects across multiple domains:

- Tenant;
- Institution;
- College / School where supported;
- institutional Campus;
- organisational units;
- Academic Year;
- Academic Period;
- Programme;
- Programme Version;
- selected Courses;
- selected Course Offerings;
- canonical Person identities;
- Student context;
- Faculty / workforce context;
- academic assignment;
- timetable / teaching-session context;
- attendance context;
- assessment / examination context;
- finance context;
- document/history context;
- seeded tasks;
- seeded notifications;
- seeded exceptions.

The seed shall use canonical L1–L4 ownership to determine which domain owns each fact.

Stage 3 shall not create new canonical business entities.

---

## 6. Identity and Referential-Coherence Rules

Cross-domain coherence is a hard Stage-3 requirement.

The same fictional human shall not be independently re-created in Student, Finance, Attendance, Examination or People fixtures.

Stable mock identifiers may be used for frontend demonstration, but they are not proposed backend identifier formats.

Conceptually:

```text
canonical Person
   ↓
Student Context
   ↓
Attendance / Examination / Finance / Documents projections
```

and:

```text
canonical Person
   ↓
Workforce / Faculty Context
   ↓
Academic Assignment / Timetable / Teaching projections
```

References shall be ID-based or equivalently deterministic rather than maintained through duplicated human-readable text.

A smaller coherent graph is preferred over a large contradictory fixture corpus.

---

## 7. Mock State Strategy

The recommended Stage-3 state model is:

```text
immutable deterministic seed
        ↓ fresh copy
active demo state
        ↓ controlled local mutation
updated demo state + history
        ↓ deterministic reset
fresh copy of canonical seed
```

Requirements:

- seed data itself is never mutated;
- mock mutation is frontend-local and explicitly non-authoritative;
- local actions may represent only source-supported states and outcomes;
- pending, rejection, technical failure, indeterminate and confirmed outcomes remain distinguishable;
- no user action may be invented for Stage-2 mock-`PROHIBITED` scenarios/workflows;
- reset must restore exact seed state.

### Persistence decision

Default browser persistence is **deferred**.

Stage 3 should not initially use `localStorage` or `sessionStorage` for demo state because persistent stale state would weaken deterministic review and reset guarantees.

Default review behavior should remain:

```text
fresh load → canonical seed
reset      → canonical seed
```

Persistence may be reconsidered later only through a separate justified decision.

---

## 8. Mock Time Strategy

Stage 3 shall use one deterministic mock clock.

Existing fixed-date review practice should be preserved and generalized rather than introducing uncontrolled real-time dates.

Seeded timestamps for:

- tasks;
- notifications;
- exceptions;
- history;
- academic activity;
- exam/payment examples where needed

shall remain internally coherent with the same mock timeline.

Uncontrolled use of `Date.now()` / real current time shall not determine canonical fixture state.

Separate independent clocks per domain are not recommended.

---

## 9. Scenario Presets

Presets are optional.

For the Stage-3 foundation, only two are initially recommended:

1. baseline institution;
2. exception-rich review state.

Admission-peak, examination-period, fee-due, campus-incident and other thematic presets should be added later only where they materially improve Stage-5 review.

Stage 3 shall not become a premature workflow-simulation stage.

---

## 10. Stage-2 Workflow Compatibility

Stage 3 consumes the final Stage-2 realization register as authority.

It shall support later realization of:

```text
551 mock-required workflows
11 mock-not-required workflows
33 mock-prohibited workflows
```

Stage 3 does not need to implement those 595 workflows.

Its responsibility is to provide a stable institutional world that later stages can use safely.

`L6-RWF-####` identities remain reviewer/traceability metadata and shall not become ordinary end-user labels.

---

## 11. Reviewability

The existing Global Experience Lab remains the preferred reviewer surface.

Stage 3 should extend it rather than create a parallel mock-admin application.

Review/demo mode should make available, where appropriate:

- current demo institution/preset;
- current deterministic mock date;
- Reset Demo Data;
- seeded task inspection;
- seeded notification inspection;
- seeded exception inspection;
- history/activity inspection;
- reviewer trace metadata.

Ordinary end-user surfaces shall remain free of architecture/reviewer identifiers.

Demo controls shall never imply production authorization or authoritative backend execution.

---

## 12. Deterministic Validation

Stage-3 implementation shall add a purpose-built mock integrity validator.

Hard failures should include:

- duplicate mock IDs;
- missing identifiers;
- dangling references;
- references to the wrong record class;
- orphan domain records;
- duplicate canonical Person identity representation;
- broken Student→Person references;
- invalid academic references;
- invalid cross-domain references;
- nondeterministic seed generation;
- reset not restoring exact canonical seed;
- uncontrolled runtime time dependence in canonical fixture state;
- fake mutation attached to a Stage-2 mock-`PROHIBITED` workflow;
- architecture/reviewer IDs leaked into ordinary UX;
- maturity falsely upgraded from presentation-only/integration-pending.

Minimum validation evidence:

```text
npm run review:validate
new deterministic mock integrity validation
npm run typecheck
npm run build
```

The current frontend has no established lint command or conventional unit-test runner. Stage 3 shall not install a large toolchain merely to satisfy label-only gates. If lint/test infrastructure exists by the time Stage 3 closes, those configured gates must also pass.

---

## 13. Controlled Implementation Batches

### Batch 0 — Authority and baseline freeze

**Goal:** re-check current repository heads immediately before implementation.

**Repositories:** Architecture + Frontend.

**Exit:** exact execution baseline recorded; no unreviewed drift.

### Batch 1 — Canonical fictional seed

**Goal:** establish the smallest coherent institutional world.

**Frontend outputs:** immutable deterministic seed and seed-integrity validator.

**Exit:** unique IDs, valid references, canonical Person coherence and deterministic serialization.

### Batch 2 — Read/projection layer

**Goal:** prevent workspace-owned copies of repeated institutional subjects.

**Initial cross-domain proof:** Student + Attendance + Examination + Finance for the same Student context.

**Exit:** the same Person/Student Context is resolved across those domains.

### Batch 3 — Demo state and deterministic reset

**Goal:** establish controlled frontend-only mutation.

**Exit:** mutation changes active state; seed is unchanged; reset returns deep-equal seed.

### Batch 4 — Mock clock and seeded operational context

**Goal:** establish one reproducible timeline with tasks, notifications, exceptions and history.

**Exit:** seeded temporal context is deterministic and internally coherent.

### Batch 5 — Experience Lab integration

**Goal:** make the Stage-3 foundation inspectable and resettable through existing review tooling.

**Exit:** reviewer can identify demo context and reset/reproduce it without architecture metadata leaking to normal UX.

### Batch 6 — Second deterministic preset

**Goal:** prove a controlled alternate review state.

**Exit:** baseline and exception-rich presets remain deterministic and resettable.

### Batch 7 — Existing fixture convergence

**Goal:** remove important duplicate fixture islands without unnecessary rewriting.

**Exit:** recurring subjects in `demo.ts`, Student fixtures, reference fixtures and deep-workspace fixtures use the shared coherent source where materially applicable.

### Batch 8 — Final Stage-3 validation and evidence

**Goal:** objectively determine Stage-3 PASS/FAIL.

**Exit:** all hard gates below pass and architecture status/evidence is synchronized.

---

## 14. Stage-3 Objective Exit Gate

Stage 3 may pass only when all applicable conditions are satisfied:

```text
[ ] one coherent fictional institutional world exists
[ ] stable mock identities exist
[ ] canonical Person coherence is preserved
[ ] multiple domains use the same subjects
[ ] duplicate mock IDs = 0
[ ] dangling references = 0
[ ] invalid cross-domain references = 0
[ ] deterministic seed validation passes
[ ] controlled mock read path works
[ ] controlled local mutation works
[ ] immutable seed remains unchanged
[ ] reset restores exact deterministic seed
[ ] one deterministic mock clock exists
[ ] seeded tasks exist
[ ] seeded notifications exist
[ ] seeded exceptions exist
[ ] seeded history/activity exists
[ ] Experience Lab makes demo/review context understandable
[ ] reviewer can reset and reproduce the world
[ ] ordinary users do not see architecture/reviewer IDs
[ ] Stage-2 mock-PROHIBITED workflows receive no fake user mutation
[ ] existing 99-workspace model remains intact
[ ] existing 12-archetype model remains intact
[ ] existing visual system remains intact
[ ] no backend is required
[ ] no database is required
[ ] no API/service/database architecture is invented
[ ] no L0-L5 ownership/rule/state/event/permission semantics are invented
[ ] frontend review validation passes
[ ] mock integrity/reset validation passes
[ ] TypeScript typecheck passes
[ ] production build passes
```

A mock-data folder by itself does not satisfy Stage 3.

---

## 15. Self-Review Findings Incorporated

The gameplan was independently challenged against architecture integrity, repository integrity, data coherence, UX reviewability, engineering risk, sequencing and validation.

Corrections incorporated into the approved planning baseline:

1. prefer one coherent institutional source over independent per-domain fixture worlds;
2. reuse existing frontend fixtures/Experience Lab rather than replace them;
3. defer default browser persistence;
4. start with only two deterministic presets;
5. reject a generalized mock workflow engine for Stage 3;
6. make runtime referential-integrity validation a hard gate;
7. retain one deterministic mock clock;
8. migrate existing fixtures incrementally rather than rewrite the entire frontend;
9. explicitly protect mock-`PROHIBITED` workflows from fabricated actions;
10. keep Stage 3 separate from Stage 4 workflow discovery and Stage 5 domain mock operation.

Remaining implementation-local choices, including exact folder names and whether the small state layer uses React Context/reducer or an equivalent lightweight pattern, remain intentionally open.

---

## 16. First Implementation Batch

When explicit execution approval is given, the first implementation work shall be:

```text
Batch 0 — re-check both main heads
        ↓
create controlled Stage-3 branches
        ↓
Batch 1 — canonical immutable institutional seed
        ↓
deterministic referential-integrity validator
```

Do not begin with a generalized store, persistence layer, workflow engine or bulk workspace migration.

The first proof must show that the same existing fictional subjects remain coherent across institutional, academic and cross-domain projections.

---

## 17. Planning Verdict

```text
STAGE 3 PLAN — READY TO EXECUTE
```

This document records planning approval only.

**Stage-3 implementation has not started.**  
Implementation requires a separate explicit instruction such as `Proceed with Stage 3`.
