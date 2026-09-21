# SutraERP L6 — Full Workflow Realization & Mock-Operational ERP Execution Plan — Draft

**Version:** 0.1.0  
**Status:** Draft Active Execution Plan  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Depends On:** `00_Governance/L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md`; existing 99-workspace + 6-shared-surface baseline; canonical 1,084-scenario L3 corpus; L4/L5 traceability; existing 12-archetype frontend system.  
**Purpose:** Turn the established L6/frontend baseline into a fully browseable, deterministic mock ERP in which every canonical L3 scenario has an explicit UI realization and every user-addressable workflow is discoverable/reviewable without a backend or database.

---

## 1. Product Review Goal

A reviewer should be able to open SutraERP and experience a realistic fictional institution as an end user.

They should be able to:

- switch legitimate presentation contexts;
- browse all major ERP areas;
- inspect coherent records;
- search for work by natural workflow terminology;
- initiate supported mock interactions;
- observe local mock updates;
- follow owner-preserving handoffs;
- see pending/rejection/failure/indeterminate outcomes;
- understand history/status/evidence;
- discover missing, confusing or duplicated UX;
- reset the demo deterministically.

The mock ERP exists to expose product and architecture flaws before backend integration.

---

## 2. Non-Goals

This programme shall not:

- create a real database;
- create backend APIs/services;
- define payloads/endpoints;
- simulate production authorization;
- invent business rules/states/events;
- create one screen/button per L3 scenario;
- replace canonical ownership with frontend convenience;
- convert fixtures into architecture authority;
- claim production readiness.

---

## 3. Core Delivery Model

```text
1,084 L3 scenarios
→ UI-realization audit
→ workflow-realization registry
→ 99 owning/browseable workspaces
→ reusable 12-archetype composition
→ coherent mock institutional dataset
→ local deterministic mock state
→ workflow-aware discovery
→ full mock-operational ERP
→ human end-user review
→ controlled findings/corrections
```

---

## 4. Stage 1 — Governance Reopening and Registry Foundation

Outputs:

- controlled reopening authorization;
- workflow-realization taxonomy;
- workflow-realization register schema;
- updated L6 README/index;
- validation rules for complete realization coverage;
- explicit distinction between scenario, workflow, workspace, journey and delivery slice.

Exit gate:

- repository authority is unambiguous;
- no previous lock history is deleted;
- new expansion scope is explicit.

---

## 5. Stage 2 — Full 1,084-Scenario UI-Realization Audit

Process every canonical L3 scenario.

Required classification fields:

- canonical L3 scenario ID;
- canonical scenario title;
- owning/primary L6 workspace family;
- relevant presentation profiles;
- primary UI-realization disposition;
- human-readable workflow/result label;
- discoverability entry;
- expected observable outcome;
- whether local mock interaction is required;
- relevant UX archetype(s);
- owner/handoff destination where applicable;
- source trace reference;
- realization status;
- open issue / escalation.

Rules:

- no sampling;
- no invented scenario semantics;
- no automatic “interactive” classification;
- system/background scenarios remain non-interactive when appropriate;
- every scenario must end in an explicit disposition.

Exit gate:

**1,084 / 1,084 classified; 0 unexplained gaps.**

The audit shall discover the actual number of unique user-facing workflows; no target workflow count is pre-invented.

---

## 6. Stage 3 — Coherent Mock ERP Foundation

The reviewed Stage-3 planning baseline is:

`00_Governance/L6_Stage3_Coherent_Mock_ERP_Foundation_Reviewed_Gameplan_2026-09-21.md`

That document supersedes the earlier illustrative Stage-3 folder proposal as the active detailed Stage-3 gameplan without changing the programme objective.

Stage 3 shall establish:

- one deterministic institutional seed rather than disconnected per-screen fixture islands;
- stable fictional identities and stable mock references;
- cross-domain referential coherence;
- domain-owned mock facts aligned to canonical upstream ownership;
- reusable read/projection access;
- controlled frontend-only mutable demo state;
- deterministic reset;
- one deterministic mock clock;
- seeded tasks, notifications, exceptions and history/activity;
- review/demo controls through the existing Global Experience Lab;
- deterministic referential-integrity and reset validation.

The existing frontend fixture, workspace, archetype, visual-system and Experience Lab foundations shall be extended rather than replaced.

Default browser persistence is deferred for Stage 3. The default review model is fresh-load/reset to the deterministic canonical seed.

Initial scenario presets are limited to:

1. baseline institution;
2. exception-rich review state.

Stage 3 consumes the final Stage-2 realization authority:

```text
Final L6 realization workflows          595
Mock REQUIRED                           551
Mock NOT-REQUIRED                        11
Mock PROHIBITED                          33
```

Stage 3 does not implement all 595 workflows. In particular, no fake user mutation may be attached to a mock-`PROHIBITED` realization merely to increase interactivity.

Controlled implementation batches are:

```text
0. authority and baseline freeze
1. canonical fictional seed + integrity validator
2. shared read/projection layer
3. demo state + deterministic reset
4. mock clock + seeded tasks/notifications/exceptions/history
5. Experience Lab integration
6. second deterministic preset
7. existing fixture convergence
8. final Stage-3 validation and evidence
```

Objective exit gate:

- one coherent fictional institutional world exists;
- stable identities and valid cross-domain references exist;
- multiple domains resolve the same subjects;
- mock read and controlled mutation paths work;
- reset restores exact deterministic seed;
- one deterministic mock timeline exists;
- seeded tasks/notifications/exceptions/history exist;
- review/demo mode is understandable;
- ordinary UX remains free of reviewer/architecture IDs;
- Stage-2 mock-`PROHIBITED` workflows receive no fake actions;
- no backend/database is required;
- no L0–L5 semantics are invented;
- review validation, mock-integrity validation, typecheck and production build pass.

**Planning state:** `STAGE 3 PLAN — READY TO EXECUTE`.

**Execution state:** `STAGE 3 — PASS WITH NON-BLOCKING DEFERRED FINDINGS` under `00_Governance/L6_Stage3_Final_Validation_and_Closure_Decision_2026-09-21.md`. Stage 3 is complete and may be closed. Stage 4 has not started.

---

## 7. Stage 4 — Workflow Discovery Layer

Upgrade discovery from workspace-only to:

**workspace + workflow + task + result/exception**

Required capabilities:

- workflow-aware Global Search;
- workspace-local Workflows/Tasks/Exceptions areas where appropriate;
- task/work entry;
- contextual cross-links;
- owner-preserving handoffs;
- optional “Browse Workflows” / Workflow Explorer.

Rules:

- sidebar remains curated;
- ordinary UI uses human-readable terms;
- L3/L4/L5 IDs remain reviewer-only;
- discoverability must respect supported presentation profiles without becoming authorization.

Exit gate:

- every user-addressable registered workflow has a discoverable UI entry;
- no broken workflow destination;
- no 1,084-item navigation dump.

---

## 8. Stage 5 — Domain-by-Domain Mock-Operational Implementation

Implement using the existing 12 archetypes.

Recommended domain sequence:

1. Platform / Administration
2. Institution & Structure
3. Student Lifecycle
4. Academic Delivery
5. People & Workforce
6. Finance
7. Campus Operations
8. Institutional Services & Intelligence
9. Examination
10. Enterprise Resource Management

For each workflow where interaction is legitimate, implement enough deterministic behavior to review:

```text
enter/discover
→ inspect context
→ perform mock intent
→ intermediate state if applicable
→ confirmed/rejected/failed/indeterminate/handoff outcome
→ updated related presentation
→ history/activity visibility
```

For background/system workflows, implement the correct observable consequence rather than a fake action.

Exit gate per domain:

- all domain scenarios have realizations;
- all user-addressable workflows are reachable;
- representative records are coherent;
- no architecture hard fail is introduced.

---

## 9. Stage 6 — Full ERP Human Review

Review the mock ERP as a real institution would experience it.

Review dimensions:

- discoverability;
- terminology;
- navigation;
- task continuity;
- workflow grouping;
- information density;
- forms and data-entry burden;
- dead ends;
- duplicate workflows;
- owner handoffs;
- exception/recovery clarity;
- history/evidence clarity;
- responsive behavior;
- keyboard/focus;
- accessibility;
- role/presentation confusion;
- cross-domain subject/context continuity.

Questions reviewers should be able to ask:

- “Where do I go to do X?”
- “What happens after this?”
- “Who owns the next step?”
- “Why can I see this?”
- “Can I recover from this failure?”
- “Did the action actually complete?”
- “Why is this information duplicated?”
- “Does the same student/context remain consistent across areas?”

Every finding is classified before correction.

Exit gate:

- major product/UX blockers resolved or explicitly escalated;
- no unexplained workflow coverage gap remains.

---

## 10. Validation and CI Requirements

Frontend validation should eventually fail when:

- any canonical L3 scenario lacks a realization disposition;
- a realization references a nonexistent workspace;
- a user-addressable workflow has no discoverable destination;
- a workflow destination is dead;
- duplicate/conflicting realization ownership is introduced;
- a system/background scenario is incorrectly represented as a user action;
- ordinary UX exposes reviewer/architecture IDs;
- reviewer tooling loses exact scenario traceability;
- mock fixtures lose deterministic reset/provenance;
- maturity is falsely upgraded to integrated/production-validated.

---

## 11. Maturity Model

Maintain separate dimensions:

1. **SCENARIO-REALIZATION-MAPPED**
2. **WORKFLOW-DISCOVERABLE**
3. **MOCK-INTERACTIVE** or **OBSERVABLE-NONINTERACTIVE**
4. **UX-REVIEWED**
5. **INTEGRATION-PENDING / INTEGRATED**
6. **PRODUCTION-VALIDATED**

Mock interaction does not equal integration.

---

## 12. Programme Completion Gate

The programme is complete only when the repository can evidence:

- 1,084 / 1,084 scenario realization dispositions;
- 0 unexplained scenario gaps;
- 99 / 99 browseable business workspaces;
- all user-addressable workflows discoverable;
- all appropriate non-interactive scenarios observably represented;
- deterministic mock data and reset;
- no backend/database requirement for demo mode;
- no broken workflow destinations;
- no silent ownership/rule/state invention;
- a completed human end-user review pass with findings dispositioned.

The final counts of unique workflows shall be discovered from the audit and recorded as evidence.
