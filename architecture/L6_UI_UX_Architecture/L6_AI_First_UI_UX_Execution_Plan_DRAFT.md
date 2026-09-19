# SutraERP L6 — AI-First UI / UX Execution Plan — Draft

**Architecture Level:** L6 — UI / UX Architecture Support  
**Status:** Draft Active Execution Plan  
**Purpose:** Accelerate SutraERP frontend delivery by using AI-first, pattern-led, reviewable implementation while preserving the existing architecture coverage baseline and canonical ownership.

---

## 1. Objective

The objective of this execution plan is not to claim that all 99 business workspace families are production-complete.

The objective is to establish, within one concentrated delivery cycle:

- 99 / 99 business workspace families UX-mapped;
- 99 / 99 business workspace families discoverable through meaningful navigation/search at an appropriate shell level;
- a reusable SutraERP UX archetype system;
- a global non-production Experience Lab / Reviewer Mode;
- approximately 12–18 strategically selected workspace families deeply interactive enough to expose backend data, authority and integration needs;
- Student, Academic, Attendance, Examination and Finance journeys sufficiently reviewable to support backend implementation planning;
- frontend architecture, states, handoffs and integration boundaries documented without inventing backend contracts.

This plan does not change the existing L6 architecture coverage baseline:

```text
99 business workspace families
+
6 shared L6 surfaces
=
105 L6 inventory units
```

---

## 2. Delivery Strategy

Do not use the 99 workspace families as 99 independent handcrafted frontend projects.

Use:

```text
architecture coverage
→ UX archetype mapping
→ global workspace registry
→ navigable workspace composition
→ selected deep experience slices
→ Experience Lab review
→ finding classification
→ UX correction / controlled amendment
→ backend-ready interaction evidence
```

The default frontend production method shifts from **workspace-by-workspace bespoke implementation** to **pattern-led assembly with deep review of backend-defining journeys**.

---

## 3. Core UX Archetypes

The implementation should converge on a reusable set of approximately 10–12 archetypes, including:

1. Entity 360 / Record Workspace
2. Search + Worklist
3. Master / Detail Operations Queue
4. Multi-step Transaction / Guided Flow
5. Lifecycle / High-consequence Action
6. Roster / Bulk Action
7. Schedule / Calendar
8. Financial Ledger / Transaction
9. Request / Document Tracking
10. Configuration / Master Data
11. Analytics / Monitoring
12. Persona Home / Dashboard

Workspace families may use more than one archetype. Archetypes are implementation/UX composition patterns, not new business domains or architecture ownership units.

---

## 4. AI Work Lanes

The work should run through four coordinated AI-first lanes.

### Lane A — Architecture and UX Mapping

Owns:

- workspace → persona/presentation profile mapping;
- workspace → user-goal mapping;
- workspace → UX archetype mapping;
- journey / handoff identification;
- upstream traceability and ownership checks;
- detection of decisions requiring L6 amendment.

Lane A does not write production frontend code by default.

### Lane B — Frontend Foundation

Owns:

- shared shell;
- workspace registry;
- UX archetype components;
- forms/tables/detail patterns;
- state surfaces;
- responsive patterns;
- accessibility foundations;
- global Experience Lab / Reviewer Mode;
- shared integration maturity and handoff components.

### Lane C — Domain Implementation

Owns:

- approved workspace compositions;
- selected experience slices;
- fixtures;
- scripted review cases;
- frontend interaction implementation;
- reuse of approved UX archetypes.

### Lane D — Independent Review

Owns:

- architecture conformity;
- broken or dead interaction detection;
- terminology consistency;
- state coverage;
- responsive/accessibility review;
- duplicate-pattern detection;
- finding classification.

Human review remains necessary for comprehension, usability and product judgement.

---

## 5. Day-by-Day Execution Plan

### Days 1–2 — Build the Global UX Production Foundation

Complete:

- global Workspace Registry;
- UX Archetype Registry;
- global Experience Lab / Reviewer Mode;
- standard feature-module structure;
- reusable loading / empty / denied / degraded / pending / failure / indeterminate state surfaces;
- reusable governed-action pattern;
- reusable owner-preserving handoff pattern;
- fixture provenance/fidelity convention;
- shared integration-maturity convention;
- initial 99-workspace → archetype mapping.

Do not start broad bespoke workspace implementation during this phase.

**Exit condition:** AI can take an approved workspace mapping and assemble it using shared patterns without inventing business authority.

### Days 1–2 implementation status

The foundation phase is implemented through the controlled Day-1/Day-2 global foundation baseline:

- 99 / 99 workspace families UX-mapped;
- 12 UX archetypes registered;
- reusable archetype implementation bindings established;
- typed feature-module registration established;
- Global Experience Lab established with W-C02 as the initial curated case population;
- shared state, governed-action and owner-handoff primitives established;
- fixture fidelity and integration-maturity conventions established;
- global readability, focus, reduced-motion, responsive and coarse-pointer safeguards established;
- static CI protection extended across the above rules.

Broad workspace implementation remains intentionally deferred to later phases. Browser/assistive-technology acceptance remains separate evidence and is not implied by foundation completion.

**Phase proceed rule:** move to Days 3–5 only after the Day-2 frontend branch passes review validation, typecheck and production build with no merge-blocking foundation defect.

---

### Days 3–5 — Establish the Reference UX System

Use representative SutraERP journeys to prove the reusable archetypes.

Priority references:

- Student Workspace / Entity 360;
- Academic Registration / guided transaction;
- Teaching + Attendance / roster and bulk action;
- Examination / governed operational workflow;
- Finance / ledger and transaction;
- Timetable / schedule;
- Documents / request tracking;
- Institution Administration / configuration.

During these days:

- refine shared components;
- eliminate duplicated interaction patterns;
- stabilize responsive behavior;
- stabilize information-density rules;
- establish standard review-case structures.

**Exit condition:** most future workspaces can be composed from established UX patterns instead of being independently invented.

---

### Days 6–8 — Achieve Full ERP UX Breadth

Map and represent all 99 business workspace families at meaningful shell level.

Each represented workspace must have, at minimum:

- canonical workspace identity;
- primary presentation profiles;
- primary user goal;
- assigned UX archetype(s);
- meaningful information architecture;
- meaningful entry path through navigation/search;
- major action categories;
- cross-domain handoffs;
- fixture-backed representative content where required for review;
- loading / empty / degraded behavior where applicable;
- integration maturity;
- explicit unavailable/preview boundaries for unimplemented owning destinations.

Do not use meaningless “Coming Soon” pages as workspace completion.

**Exit condition:** the ERP is broadly navigable and architecture coverage can be inspected through the frontend.

---

### Days 9–11 — Deepen Backend-Defining Journeys

Concentrate implementation depth on the journeys most likely to expose backend requirements.

Priority domain groups:

#### Student
- Student Workspace;
- Admissions;
- Academic Registration;
- Progression.

#### Academic Delivery
- Teaching Sessions;
- Attendance;
- Timetable;
- Internal Assessment.

#### Examination
- examination planning / registration;
- marks / results / governed outcome flows.

#### Finance
- fees / charges;
- payments / receipts;
- refunds / adjustments.

#### People
- representative employee workspace;
- representative leave / payroll interaction where approved.

For selected deep journeys, reviewers should be able to exercise:

```text
discover / enter
→ inspect context
→ initiate supported action
→ review consequence
→ submit fixture intent
→ pending / success / rejection / failure / indeterminate presentation
→ owner-preserving handoff where required
→ meaningful return
```

**Exit condition:** major backend-facing interaction, data and authority requirements are visible enough to begin backend implementation planning safely.

---

### Days 12–13 — Freeze Breadth and Perform ERP-Wide UX Review

Stop adding broad new workspace coverage unless a blocking gap is discovered.

Focus on:

- navigation consistency;
- terminology consistency;
- workspace discoverability;
- cross-domain handoffs;
- desktop / tablet / mobile behavior;
- keyboard and focus behavior;
- accessibility;
- information density;
- dead or misleading controls;
- duplicate components/patterns;
- Experience Lab case walkthroughs;
- fixture fidelity;
- integration-boundary clarity;
- backend data/integration requirement extraction.

Classify findings as:

- Implementation defect
- L6 UX architecture defect
- L5 feasibility / integration issue
- L4/L3/L2 business architecture issue
- Review-process issue
- Implementation-local improvement

**Exit condition:** frontend breadth is frozen into a coherent reviewable baseline and the project can begin backend implementation without pretending the entire frontend is production-final.

---

## 6. Completion Model

Do not use one generic `Done` state.

Track each workspace through separate dimensions:

1. **UX-MAPPED** — personas, goals, archetypes and boundaries are known.
2. **SHELL-IMPLEMENTED** — workspace is meaningfully discoverable/navigable.
3. **INTERACTIVE-FIXTURE-IMPLEMENTED** — material journeys can be exercised using controlled fixtures.
4. **UX-REVIEWED** — human/browser review completed for applicable journeys.
5. **INTEGRATION-PENDING / INTEGRATED** — released backend contracts are absent/present and connected.
6. **PRODUCTION-VALIDATED** — integrated production-like validation completed.

`SHELL-IMPLEMENTED` is not `UX-REVIEWED`.  
`INTERACTIVE-FIXTURE-IMPLEMENTED` is not `INTEGRATED`.  
`UX-REVIEWED` is not `PRODUCTION-VALIDATED`.

---

## 7. Target State at the End of the Cycle

The intended target is:

- **99 / 99 workspace families UX-mapped**;
- **99 / 99 workspace families meaningfully navigable at shell/composition level**;
- **10–12 reusable UX archetypes established**;
- **global Experience Lab operational**;
- **approximately 12–18 high-value workspace families deeply interactive**;
- **major Student / Academic / Attendance / Examination / Finance journeys reviewable**;
- **frontend CI/type/build validation active**;
- **backend integration needs visible and traceable**.

This is a frontend/UX baseline for beginning backend implementation. It is not a claim that 99 workspace families are fully integrated, production-validated or visually final.

---

## 8. Governance Constraints

This accelerated plan shall not:

- change L0–L5 ownership for UI convenience;
- invent business rules, lifecycle states, events or permissions;
- invent backend APIs, payloads, schemas or services;
- treat persona/profile selection as authorization;
- treat fixture outcomes as production truth;
- use AI-generated breadth as evidence of UX acceptance;
- create one route/component/folder per experience-slice row automatically;
- reopen resolved L6 product decisions without new material evidence.

When accelerated implementation discovers a material conflict:

```text
implementation defect → fix locally
review-process gap → improve review evidence
L6 semantic problem → controlled L6 amendment
L5 integration problem → escalate to L5
business ownership/rule problem → escalate upstream
```

Repository correctness remains higher priority than speed.

---

## 9. Operating Principle

The purpose of AI acceleration is not to generate more screens.

The purpose is to make the entire ERP **navigable, reviewable and architecturally coherent faster**, while spending deep design effort on the journeys that expose the information, authority, state and interaction requirements the backend must support.



---

## Day 1 Global UX Foundation

The reusable production foundation for the accelerated programme is governed by:

`L6_Day1_Global_UX_Foundation_DRAFT.md`

That artifact preserves this plan's 99-business-workspace + 6-shared-surface baseline and adds the controlled UX archetype mapping, Global Experience Lab convention, fixture/integration maturity conventions, shared interaction patterns and static validation expectations. It does not authorize mass workspace implementation or change upstream business ownership.
