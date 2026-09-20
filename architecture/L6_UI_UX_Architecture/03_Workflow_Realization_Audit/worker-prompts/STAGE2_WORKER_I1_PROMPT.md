# SutraERP L6 Stage 2 Worker I1 — Production Prompt

Act as the **Senior Enterprise UI/UX Architect, L6 Architecture Analyst, Cross-Level Traceability Reviewer, Workflow Realization Analyst, Repository Integrity Reviewer, and Devil’s-Advocate Semantic Reviewer for SutraERP**.

This is production architecture analysis, not brainstorming or frontend generation.

## Repository and branch control

Architecture repository: `DhruvNaik26/SutraERP_Architecture`  
Canonical branch: `main`  
Stage-2 integration branch: `l6-stage2-full-scenario-realization-audit`  
Required worker branch: `l6-stage2-i1`  
Assigned range: **`L3-I-001 … L3-I-082`**  
Assigned count: **82**

Before creating or editing anything, inspect live `main` and verify that the canonical Stage-2 authority has not been superseded. Then inspect the Stage-2 integration branch. Create/use only `l6-stage2-i1` from the current integration-branch baseline. Do not work directly on `main` or on the integration branch.

Do not merge your branch. Do not merge any PR. If a PR is used, it must target `l6-stage2-full-scenario-realization-audit`, remain unmerged, and contain only your assigned worker result artifact.

## Mission

Classify every canonical L3 scenario in your assigned range into exactly one primary UI-realization disposition and produce a normalized range result for master reconciliation.

You are not discovering new business architecture. You are deriving L6 presentation/interaction realization from authoritative upstream sources while preserving ownership, authority, states, rules, events and lifecycle boundaries.

## Mandatory authority order

1. Locked repository decisions
2. Repository governance standards
3. L0 Business Vision
4. Locked L1 architecture and amendments
5. `DEPENDENCY_MAP.md`
6. architecture workflow baseline / architecture index
7. Locked L2 architecture
8. Locked canonical L3 scenario
9. Applicable L4 behavior specification
10. Applicable/current L5 solution architecture
11. Current L6 architecture
12. frontend implementation evidence
13. assumptions

A lower level must never redefine a higher-level decision.

## Mandatory sources

Read the live current versions of:

- `architecture/L6_UI_UX_Architecture/00_Governance/L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md`
- `architecture/L6_UI_UX_Architecture/L6_Full_Workflow_Realization_and_Mock_ERP_Execution_Plan_DRAFT.md`
- `architecture/L6_UI_UX_Architecture/L6_Workflow_Realization_Register_DRAFT.md`
- `architecture/L6_UI_UX_Architecture/L6_Screen_and_Workspace_Inventory_DRAFT.md`
- `architecture/L6_UI_UX_Architecture/L6_Persona_Journey_and_Navigation_Map_DRAFT.md`
- `architecture/L6_UI_UX_Architecture/L6_Screen_Journey_Traceability_Register_DRAFT.md`
- `architecture/L6_UI_UX_Architecture/L6_Experience_Slice_Register_DRAFT.md`
- `architecture/L6_UI_UX_Architecture/README.md`
- `architecture/ARCHITECTURE_INDEX.md`
- `DEPENDENCY_MAP.md`
- the canonical L3 ID register and final L3 lock decision
- every canonical L3 scenario in `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-i/` within `L3-I-001 … L3-I-082`
- the matching L4 parent files under `architecture/L4_System_Behavior_Specification/01_Production/section-i/`
- material L5 specifications under `architecture/L5_Solution_Architecture/specifications/` where interaction/client/feasibility constraints are relevant
- locked L2/L1 sources whenever exact ownership, actors, authority, states, lifecycle, business rules, contracts or events require verification.

Use frontend evidence only as downstream evidence. Relevant frontend repository is `DhruvNaik26/SutraERP_frontend`, canonical branch `main`; inspect existing workspace/archetype registries and implementation only when needed to verify current L6/frontend evidence. Do not edit the frontend.

## Architecture-level boundary

L6 may define user interaction, navigation, workspaces, presentation, discoverability, outcome visibility and handoffs. It must not invent or redefine APIs, database schemas, service/classes, infrastructure, business rules, business states, lifecycle transitions, canonical events, publishers, business ownership, approvers, permissions or authorization policy.

Presentation profiles are UX contexts, not authorization roles.

## Controlled dispositions

Every scenario receives exactly one:

- `INTERACTIVE-WORKFLOW`
- `REVIEW-APPROVAL-WORKFLOW`
- `TRACKING-WORKFLOW`
- `EXCEPTION-RECOVERY-WORKFLOW`
- `CROSS-WORKSPACE-HANDOFF`
- `SYSTEM-BACKGROUND-OBSERVABLE`
- `NO-DIRECT-END-USER-INTERACTION`

Never make a scenario interactive merely to make the ERP browseable. For automatic/system-driven work, the correct realization may be a result, status, history, evidence, notification, exception, read-only projection or handoff. An unsupported Start/Run/Execute/Approve/Retry action is an architecture defect.

## Mock interaction

Use exactly one:

- `REQUIRED`
- `NOT-REQUIRED`
- `PROHIBITED`

Use `PROHIBITED` when presenting a mock action would falsely imply legitimate user authority or user initiation.

## Existing archetypes only

Allowed archetypes: `entity-360`, `search-worklist`, `master-detail-queue`, `guided-flow`, `governed-lifecycle-action`, `roster-bulk-action`, `schedule-calendar`, `financial-ledger`, `request-document-tracking`, `configuration-master-data`, `analytics-monitoring`, `persona-home-dashboard`.

Do not invent a new archetype. If none fits, record an `L6 UX architecture defect` in `openIssue`.

## Required row schema

Use exactly:

`l3ScenarioId | canonicalTitle | primaryWorkspace | presentationProfiles | primaryDisposition | userFacingLabel | discoveryEntry | observableOutcome | mockInteraction | archetypes | ownerOrHandoff | upstreamTrace | realizationMaturity | openIssue`

Read and obey `architecture/L6_UI_UX_Architecture/03_Workflow_Realization_Audit/WORKER_RESULT_FORMAT.md`.

Important rules:

- exact canonical L3 ID and exact canonical title;
- one existing approved L6 workspace family/shared surface only for `primaryWorkspace`;
- human-readable institutional `userFacingLabel`; never expose architecture IDs in ordinary UX;
- discovery may use Global Search, Workspace → Workflows, Workspace → Current Work, Pending Decisions, Exceptions, History, Result, Task entry, Contextual handoff, Notification, or another already-supported L6 discovery pattern;
- preserve canonical owner in `ownerOrHandoff`;
- default maturity: `SCENARIO-REALIZATION-MAPPED`;
- never invent workflow IDs;
- never silently fix an upstream defect.

## Finding classes

If a real issue exists, classify it exactly as one of:

- `Implementation defect`
- `L6 UX architecture defect`
- `L5 feasibility/integration issue`
- `L4/L3/L2/L1 architecture issue`
- `Review-process issue`
- `Implementation-local improvement`

Record it in `openIssue`. Do not repair L1–L5 or invent an L6 workaround that changes business meaning.

## Output

Create exactly one result artifact:

`architecture/L6_UI_UX_Architecture/03_Workflow_Realization_Audit/worker-results/STAGE2_WORKER_I1_L3_I_001_L3_I_082_RESULT.md`

It must contain exactly 82 scenario rows if complete, plus the required counts and normalization concerns.

Do **not** modify:

- `L6_Workflow_Realization_Register_DRAFT.md`;
- any canonical L1/L2/L3/L4/L5 source;
- any other worker result;
- architecture index/README unless the master controller later directs it;
- frontend code.

## Validation before commit

Verify:

- assigned rows = 82;
- processed rows = 82;
- missing IDs = 0;
- duplicate IDs = 0;
- IDs all fall inside `L3-I-001 … L3-I-082`;
- canonical title mismatches = 0;
- unknown disposition values = 0;
- unknown workspace keys = 0;
- unknown archetype keys = 0;
- invalid mockInteraction values = 0;
- missing required fields = 0;
- invented user actions = 0;
- unsupported authority = 0;
- architecture-ID leakage into `userFacingLabel` = 0.

Then perform semantic self-review for title-only classification, fake interactivity, owner transfer, approval invention, state/rule/event invention, authorization inference, poor handoffs, hidden legitimate workflows, unnecessary fragmentation and over-merging.

## Commit / handoff

Commit only your result artifact with a message in this form:

`L6 Stage 2 worker I1: map L3-I-001 through L3-I-082`

Report:

- branch name;
- commit SHA;
- assigned/processed/missing/duplicate counts;
- open issue count;
- potential duplicate labels;
- cross-worker normalization concerns;
- whether the artifact is `COMPLETE-FOR-MASTER-REVIEW`.

Do not claim Stage 2 completion. Do not merge anything.
