# SutraERP L6 — Incremental UI Implementation Review

**Architecture Level:** L6 — UI / UX Architecture Support  
**Status:** Active Review Framework  
**Purpose:** Make frontend implementation reviewable as realistic, interactive workflow slices before backend integration is complete.  

---

## 1. Why This Area Exists

Static screen review is not enough for SutraERP.

A production-grade ERP UI must be reviewable through realistic user journeys, alternate paths, failures, empty states, authorization outcomes, stale context and high-consequence interactions. Reviewers should be able to use the frontend like a real institutional product and discover UX defects before backend implementation is complete.

This area governs **implementation review evidence**. It does not create business rules, backend contracts or new L6 ownership.

---

## 2. New Delivery Model

Every implementation-authorized L6 workspace shall be developed as a **reviewable vertical UI slice**.

Do not use this sequence:

```text
make static screen
→ approve appearance
→ later add workflow
→ later discover navigation/state problems
```

Use this sequence:

```text
approved L6 workspace
→ reviewer scenario pack
→ interactive fixture/data states
→ complete primary journey
→ alternate/error/edge states
→ responsive + accessibility review
→ UX findings
→ controlled adjustment
→ next increment
```

The goal is not to fake the backend. The goal is to expose enough controlled state and interaction for meaningful UX testing without inventing business semantics.

---

## 3. Required Reviewer Mode

The frontend should provide a non-production **UI Review Mode** for controlled visual/workflow evaluation.

Reviewer Mode may allow a reviewer to select approved fixture conditions such as:

- persona/presentation profile;
- Institution / academic context;
- representative record;
- scenario/journey;
- current authoritative-looking fixture state;
- pending mutation state;
- authoritative success fixture;
- authoritative rejection fixture;
- technical failure fixture;
- indeterminate fixture;
- stale/invalidated context fixture;
- authorization-denied fixture;
- legitimate empty/no-data fixture.

Reviewer Mode is not authorization, not a backend simulator and not canonical business state.

It must be visibly marked as review/test functionality and must not be confused with the production experience.

---

## 4. Workspace Review Pack

Each implementation-authorized workspace should have a review pack in this folder, for example:

```text
02_Implementation_Review/
├── README.md
├── W-C02_Student_Workspace_Review_Pack.md
├── W-C03_Academic_Registration_Review_Pack.md
└── ...
```

A review pack should contain:

1. workspace and approved L6 source;
2. personas/presentation profiles to test;
3. primary user journeys;
4. alternate journeys;
5. high-consequence actions;
6. required fixture records;
7. required UI states;
8. cross-domain projections/handoffs;
9. responsive checkpoints;
10. accessibility checkpoints;
11. issues discovered during visual/workflow review;
12. disposition: implementation-local fix vs L6 amendment vs upstream issue.

---

## 5. Minimum State Matrix

For each meaningful data surface or mutation-capable journey, test applicable states from this matrix:

| State | Review question |
|---|---|
| Normal populated | Can the user understand the current context and next action? |
| Loading | Is progress clear without layout instability or false data? |
| Legitimate empty | Does the UI explain that no data is valid, rather than appearing broken? |
| Pending | Does the UI avoid claiming completion too early? |
| Authoritative success | Is the completed outcome clear and does the view re-resolve correctly? |
| Authoritative rejection | Can the user understand what happened without confusing it with a technical error? |
| Technical failure | Can the user safely recover without inferring business failure/success incorrectly? |
| Indeterminate | Does the UI prevent unsafe repeat action and explain that outcome must be reconciled? |
| Stale / invalidated | Is old protected data quarantined and re-resolution clear? |
| Authorization denied | Is access refusal clear without treating hidden UI as the security mechanism? |
| Partial/degraded projection | Can the user tell which source is unavailable while other data remains usable? |

---

## 6. Scenario Fixture Rules

Fixtures exist only to make approved UI behavior testable.

Fixtures may represent:

- valid upstream state combinations already supported by architecture;
- sample records and histories;
- alternate outcome states;
- source-owned cross-domain projections;
- simulated latency/error/rejection/indeterminate technical outcomes.

Fixtures shall not invent:

- lifecycle states;
- business events;
- approval authorities;
- eligibility rules;
- backend endpoints;
- payloads;
- business calculations;
- unsupported persona authority;
- canonical ownership.

Every fixture source must be obviously non-authoritative in code/review mode.

---

## 7. Incremental Review Checkpoints

A workspace should pass through these frontend review checkpoints:

### R1 — Orientation

Reviewer can identify:

- where they are;
- whose/which record is active;
- Institution/academic context;
- workspace purpose;
- primary next actions.

### R2 — Primary Journey

Reviewer can complete the main approved user journey using fixtures without dead ends.

### R3 — Alternate and Exception Paths

Reviewer can test alternate outcomes, rejection, failure, stale context and authorization denial.

### R4 — Cross-Workspace Continuity

Read-only projections and handoffs clearly preserve the canonical owning workspace.

### R5 — Responsive and Accessibility

The same critical journey is usable by keyboard and across approved viewport classes.

### R6 — Visual / Cognitive Review

A reviewer unfamiliar with implementation can identify confusing labels, hierarchy problems, unnecessary elements, missing context and unclear consequences.

### R7 — Conformance

Implementation is checked against the approved L6 slice and `00_Governance/L6_Frontend_Backend_Integration_Guardrails.md`.

---

## 8. Issue Classification

Every review finding must be classified before changing architecture:

| Classification | Meaning | Action |
|---|---|---|
| Implementation defect | Code does not realize already-approved L6 behavior | Fix frontend |
| L6 UX architecture defect | Approved presentation/journey is materially insufficient or contradictory | Controlled L6 amendment |
| L5 feasibility/integration issue | Approved UI cannot be realized within technical architecture/contract boundary | Escalate to L5 |
| L4/L3/L2 business architecture issue | Problem is actually business behavior/ownership/state/rule ambiguity | Escalate upstream; do not patch in UI |
| Review preference | Subjective/local preference that does not improve approved user outcome | Do not automatically change architecture |

---

## 9. Frontend File Strategy

Frontend implementation should favor reusable workspace architecture rather than one oversized file per domain.

Recommended conceptual shape:

```text
app/
├── features/
│   └── student-workspace/
│       ├── student-workspace.tsx
│       ├── sections/
│       ├── components/
│       ├── review-fixtures.ts
│       ├── review-scenarios.ts
│       └── types.ts
├── review/
│   ├── review-mode.tsx
│   └── review-state.ts
└── components/
    └── shared design-system/application components
```

Exact filenames remain implementation-local. The architectural requirement is separation of:

- production UI composition;
- shared reusable components;
- explicit review fixtures/scenarios;
- backend integration seams;
- local presentation state.

Do not embed large fixture datasets and all workflow variants directly inside a single production component.

---

## 10. Review Acceptance Principle

A frontend slice is not considered sufficiently reviewable merely because the normal/default screen looks polished.

Before moving on, reviewers should be able to answer:

- Can I understand the workflow without architecture knowledge?
- Can I find the action I expect?
- Can I tell what happened after I act?
- Can I distinguish pending, rejected, failed and unknown outcomes?
- Can I recover from errors?
- Can I understand ownership when information comes from another domain?
- Can I identify stale or wrong context before acting?
- Can I use the critical path on keyboard/mobile where required?
- Can another reviewer reproduce the same scenario and report a UI problem precisely?

That is the standard this folder exists to enforce.
