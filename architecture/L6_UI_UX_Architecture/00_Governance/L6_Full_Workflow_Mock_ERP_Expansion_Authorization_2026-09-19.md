# SutraERP L6 Full Workflow Mock-ERP Expansion Authorization — 2026-09-19

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Governance  
**Decision Date:** 2026-09-19  
**Supersedes For Current L6 Programme State:** the closed-baseline/proceed interpretation of `L6_UX_Baseline_Lock_and_Backend_Proceed_Decision_2026-09-19.md`  
**Preserves As Historical Baseline:** the locked 99-workspace + 6-shared-surface / 12-archetype / Days 1–13 baseline and its validation evidence

---

## 1. Decision

The Lead Architect authorizes a controlled reopening of L6 for **full workflow UI realization and mock-operational ERP review**.

The prior L6 UX baseline lock is not deleted or rewritten. It remains the historical baseline from which this expansion starts. This decision supersedes only the interpretation that L6 is closed to material UX expansion.

The new programme objective is:

> Every one of the 1,084 canonical L3 scenarios shall receive an explicit UI-realization disposition, and every user-addressable workflow shall be discoverable and reviewable through the SutraERP frontend using coherent deterministic mock data, without requiring a database or backend.

This is an L6/frontend expansion. It does not reopen L0–L5 unless a genuine upstream conflict is discovered.

---

## 2. Why L6 Is Reopened

The Days 1–13 programme successfully established:

- 99 / 99 business workspace-family coverage;
- 6 shared surfaces;
- 12 reusable UX archetypes;
- a coherent shell and visual system;
- 81 shell-level workspace implementations;
- 18 interactive fixture-backed implementations;
- backend-readiness interaction evidence for selected deep journeys;
- static validation, typecheck and build evidence.

However, that programme intentionally optimized for **architecture breadth + selected deep journeys**, not for making every user-facing L3 workflow interactively explorable.

The resulting frontend is architecturally broad but too shallow for the next desired review mode:

> an end user should be able to open SutraERP, browse a realistic fictional institution, discover the full ERP breadth, exercise representative workflows using mock updates, and identify product/UX flaws before backend implementation.

That product-review objective requires controlled expansion.

---

## 3. Invariants That Remain Preserved

This reopening does **not** discard the previous L6 work.

The following remain the default baseline unless a controlled amendment explicitly changes them:

- 99 business workspace families;
- 6 shared L6 surfaces;
- 12 UX archetypes;
- the current design system and visual direction;
- application-shell structure;
- persona-sensitive composition;
- reviewer/end-user metadata separation;
- owner-preserving handoffs;
- shared outcome-state semantics;
- the Global Experience Lab concept;
- canonical upstream ownership and authority boundaries;
- the rule that one L3 scenario does not imply one screen, route, component or button.

The expansion shall extend existing architecture rather than create a parallel UI model.

---

## 4. New Completion Objective

The new L6/frontend completion gate is not “99 workspaces exist.”

It is:

1. **1,084 / 1,084 canonical L3 scenarios have an explicit UI-realization disposition.**
2. **0 unexplained scenarios remain.**
3. **99 / 99 business workspace families are meaningfully browseable as end-user ERP workspaces.**
4. Every user-addressable workflow is discoverable through workspace composition, search, task/work entry, workflow explorer, or governed handoff.
5. Every system/background scenario that should be visible to a user has an observable status, result, history, notification, evidence or exception presentation where appropriate.
6. Important interactive workflows can be exercised using coherent mock state without a backend or database.
7. Mock state changes remain non-authoritative demonstration behavior and never become a source of business semantics.
8. Ordinary UI remains free of L3/L4/L5/reviewer identifiers.
9. Reviewer tooling preserves exact traceability back to canonical architecture.
10. No UI action is invented merely to make a scenario “clickable.”

---

## 5. UI-Realization Disposition Categories

Every canonical L3 scenario shall be classified into exactly one primary UI-realization disposition, with supporting secondary surfaces where justified:

1. **Interactive workflow** — a user legitimately initiates or advances a supported business interaction.
2. **Review / approval workflow** — an authorized user reviews, decides, verifies or handles governed work.
3. **Tracking workflow** — the primary user need is to inspect progress, status, evidence or outstanding work.
4. **Exception / recovery workflow** — the scenario is surfaced through exception handling, correction, retry/reconciliation guidance or escalation.
5. **Cross-workspace handoff** — the current workspace exposes context but detailed work continues with the canonical owner.
6. **System/background observable outcome** — no user action is invented; the UI exposes the resulting status/history/evidence/notification where appropriate.
7. **No direct end-user interaction required** — the scenario has no legitimate direct UI control; any user-visible consequence is represented through its actual owning surface.

A scenario may contribute to multiple screens/states, but one primary disposition must be recorded for coverage accounting.

---

## 6. Mock-Operational ERP Authorization

The frontend is explicitly authorized to implement a coherent **fictional institutional dataset and local mock state layer** for product/UX review.

The mock ERP may include representative:

- institution structure;
- applicants;
- students;
- guardians where supported;
- faculty and employees;
- programmes, courses, classes and academic periods;
- timetable and teaching sessions;
- attendance;
- internal assessment;
- examinations, marks and results;
- fees, invoices, payments, refunds and adjustments;
- documents and requests;
- library/resource activity;
- campus/facility/service activity;
- tasks, approvals, exceptions and notifications;
- reports/configuration/master data where appropriate.

Mock records should be relationally coherent across workspaces. The same fictional subject/context should remain recognizably consistent across Student, Registration, Attendance, Examination, Finance, Documents and other related views.

---

## 7. Mock Update Rules

Local mock updates are permitted only to make the ERP reviewable.

They shall:

- operate entirely in frontend/local demonstration state;
- have deterministic reset behavior;
- use seeded fictional data;
- preserve existing architecture semantics;
- represent only states/outcomes supported by upstream architecture;
- distinguish pending, rejection, failure, indeterminate and confirmed mock outcomes;
- never claim authoritative backend execution;
- never define API/database/service design;
- never create new business rules, permissions, lifecycle states, events or owners.

A “Reset Demo Data” or equivalent deterministic restore mechanism shall be available in review/demo mode.

---

## 8. Discoverability Model

The permanent sidebar shall remain curated. It shall not list 1,084 scenarios.

Full workflow discoverability shall be achieved through a combination of:

- workspace-local **Workflows / Tasks / Actions / Exceptions** surfaces;
- workflow-aware global search;
- task/work entry;
- contextual links and owner-preserving handoffs;
- an optional end-user **Workflow Explorer / Browse Workflows** surface;
- reviewer tooling with exact internal traceability.

Normal users see human-readable workflow names, not architecture IDs.

---

## 9. Repository Outputs Required

This expansion shall maintain:

- the existing workspace inventory;
- the existing screen/journey traceability register;
- a new workflow-realization register covering all 1,084 scenarios;
- the existing experience-slice register for selected deep delivery increments;
- a coherent mock-data convention;
- a mock-state/reset convention;
- frontend validation that fails on unmapped scenarios or broken workflow destinations;
- explicit maturity dimensions for architecture, mock interaction, UX review, integration and production validation.

The workflow-realization register shall link to canonical sources rather than duplicate business architecture.

---

## 10. Backend Sequencing

The earlier backend-proceed authorization is **paused as the primary programme next step**, not revoked as an architectural possibility.

Reason: the project has identified a material product-review objective that is better resolved before significant backend integration effort.

Recommended sequence:

```text
locked Days 1–13 baseline
→ controlled L6 reopening
→ 1,084-scenario UI-realization audit
→ coherent mock ERP foundation
→ full workflow discoverability
→ domain-by-domain mock-operational implementation
→ full ERP human UX review
→ controlled corrections / upstream escalations
→ revised stable L6/frontend baseline
→ backend integration
```

Backend work that is independent of unresolved UI realization may continue only where separately justified; this decision does not redefine L5 ownership or technical sequencing.

---

## 11. Exit Gate

This expansion may close only when the repository can evidence:

```text
Canonical L3 scenarios                  1,084
UI-realization dispositions             1,084 / 1,084
Unexplained scenario gaps               0
Business workspaces browseable          99 / 99
Broken workflow destinations            0
User-facing workflows                   X / X
Interactive mock workflows              X / X
Observable non-interactive workflows    X / X
Backend dependency for demo mode        NONE
Database dependency for demo mode       NONE
```

The values represented by `X` shall be discovered by the full-corpus audit and shall not be guessed in advance.

---

## 12. Change-Control Rule

Findings shall be classified as:

- implementation defect;
- L6 UX architecture defect;
- L5 feasibility/integration issue;
- L4/L3/L2/L1 architecture issue;
- review-process issue;
- implementation-local improvement.

No finding may be silently repaired at the wrong architecture level.

---

## 13. Final Decision

**AUTHORIZED — L6 REOPENED FOR FULL WORKFLOW UI REALIZATION AND MOCK-OPERATIONAL ERP REVIEW.**

The previous locked UX baseline remains the preserved starting baseline and historical decision evidence. Material expansion now proceeds only under this authorization and controlled repository updates.
