# L6 UI / UX Architecture

**Architecture Level:** L6 — UI / UX Architecture  
**Current State:** Controlled L6 Reopening Authorized for Full Workflow UI Realization + Mock-Operational ERP; Days 1–13 Baseline Preserved  
**Broad Production Authoring:** Full 1,084-scenario UI-realization audit and mock-operational frontend expansion authorized under controlled amendment  
**Downstream:** SutraERP frontend implementation repository after applicable L6 slice approval/authorization  

L6 defines user interaction, navigation, screens, workspaces and user journeys derived from approved upstream architecture without owning business logic.

```text
L5 Solution Architecture
        ↓
L6 UI / UX Architecture
        ↓
Frontend Implementation
```


## Current Controlled Expansion

`00_Governance/L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md` is the current L6 programme authority.

It preserves the completed Days 1–13 baseline but reopens L6 in a controlled manner for a stronger product-review objective:

- classify **1,084 / 1,084** canonical L3 scenarios into explicit UI-realization dispositions;
- make every legitimate user-addressable workflow discoverable in the ERP;
- make appropriate system/background workflows observable without inventing user actions;
- turn all 99 workspace families into meaningfully browseable end-user ERP workspaces;
- establish one coherent fictional institution dataset across domains;
- permit deterministic frontend-only mock updates and reset behavior;
- perform a full end-user mock-ERP review before significant backend integration.

The prior `L6_UX_Baseline_Lock_and_Backend_Proceed_Decision_2026-09-19.md` remains preserved historical baseline evidence. It is not deleted. Its closed-baseline/backend-next current-state interpretation is superseded by the controlled reopening authorization.

`L6_Full_Workflow_Realization_and_Mock_ERP_Execution_Plan_DRAFT.md` governs execution.

`L6_Workflow_Realization_Register_DRAFT.md` defines the controlled 1,084-scenario realization schema. It currently establishes the schema only; its 1,084 rows must be populated during the full-corpus audit before any completion claim.

## Current Foundation Package

```text
00_Governance/
├── L6_Governance_and_Architecture_Standard_DRAFT.md
├── L6_UI_UX_Architecture_Template_v0.1_DRAFT.md
├── L6_Validation_Standard_DRAFT.md
├── L6_Product_UX_Decision_Field_Map_DRAFT.md
├── L6_Foundation_Controlled_Authorization_2026-09-16.md
├── L6_Authenticated_Shell_Slice_Architecture_Approval_Decision_2026-09-16.md
├── L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md
├── L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md
├── L6_UX_Baseline_Lock_and_Backend_Proceed_Decision_2026-09-19.md
├── L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md
└── L6_Frontend_Backend_Integration_Guardrails.md

01_Foundation/
├── L6_Design_System_Foundation_DRAFT.md
├── L6_Application_Shell_and_Information_Architecture_DRAFT.md
└── L6_Interaction_State_and_Feedback_Standard_DRAFT.md

02_Implementation_Review/
├── README.md
└── W-C02_Student_Workspace_Review_Pack.md
```

The six foundation artifacts remain `Draft`; they have not been silently renamed, locked, or assigned permanent L6 specification IDs. `L6_Foundation_Controlled_Authorization_2026-09-16.md` remains the controlling foundation authorization. The current programme-level authority is `L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md`.

`L6_Product_UX_Decision_Field_Map_DRAFT.md` is the controlled working taxonomy for product/UX decisions during L6 authoring. It defines 64 core decision fields across eight families, distinguishes upstream-fixed, product-direction, architecture-derived, slice-specific and implementation-local matters, and requires reuse of previously resolved fields before new product questions are opened.

`L6_Authenticated_Shell_Slice_Architecture_Approval_Decision_2026-09-16.md` authorizes controlled frontend implementation of the shared authenticated shell, Institution/academic context presentation and switching, global search/discovery, notification presentation framework, account/session projection affordance, responsive shell behavior and accessibility requirements within its recorded boundaries.

`L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md` records the selected W-C02 product/UX direction.

`L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md` is the current W-C02 authority. The former HF-L6-12 traceability blocker has been closed; W-C02 is **Architecture Approved — Controlled Frontend Implementation Authorized**.

`L6_Frontend_Backend_Integration_Guardrails.md` is derived L6 governance for AI-assisted frontend implementation. It does not define backend/API architecture; it constrains the frontend to consume released authoritative contracts without inventing business rules, authorization, lifecycle outcomes, endpoint semantics or canonical ownership.

`02_Implementation_Review/` defines the incremental UI review model. It exists so implementation is evaluated through reproducible journeys, fixtures, alternate states, failure states, responsive behavior and accessibility rather than only polished default screens.

## Current Planning and Traceability Package

```text
L6_Screen_and_Workspace_Inventory_DRAFT.md
L6_Persona_Journey_and_Navigation_Map_DRAFT.md
L6_Screen_Journey_Traceability_Register_DRAFT.md
L6_Workflow_Realization_Register_DRAFT.md
L6_Experience_Slice_Register_DRAFT.md
L6_AI_First_UI_UX_Execution_Plan_DRAFT.md                     # historical Days 1–13 plan
L6_Full_Workflow_Realization_and_Mock_ERP_Execution_Plan_DRAFT.md  # active expansion plan
```

### Screen / Workspace Inventory

The corrected inventory establishes:

- 98 L2-anchored business workspace families;
- 1 additional L3-derived `W-G00 Campus Operations Governance` workspace family for final `L3-G-001…015`;
- **99 business workspace families** in total;
- 6 shared L6 shell/task surfaces;
- **105 total draft inventory units**;
- the approved asymmetric Administrator / Faculty / Student Home composition;
- the rule that inventory rows are not automatically navigation items or final screens.

The `W-G00` correction does not redefine `L2-G00 Campus Operations Review`; it closes an L6 presentation-coverage gap discovered during exact final-L3 reconciliation.

### Persona Journey / Navigation Map

The journey map establishes:

- UX navigation profiles for Student, Faculty, Guardian, Institutional Administrator and specialist institutional work;
- presentation profiles as non-authoritative UX compositions rather than security roles;
- multi-responsibility navigation composition;
- primary navigation consolidation of the 105 inventory units into task-recognizable user-facing families;
- shared shell, contextual navigation, task entry, search/discovery and deep-link behavior;
- representative Student, Faculty, Guardian, Administrator and specialist journeys;
- Campus Operations navigation including responsibility-scoped `W-G00` Governance & Readiness.

### Screen / Journey Traceability Register

The traceability register establishes:

- exact final-L3 scenario spans for every business workspace family;
- deterministic L3→L4 identity linkage under the canonical L4 parent-ID convention;
- coverage reconciliation across the full **1,084 final L3 scenarios**;
- the mandatory shared Web Client Baseline inherited by L6 web surfaces;
- actor/profile presentation scope without converting UX profiles into authorization roles;
- an explicit per-slice gate requiring material L4 BH→L5 specification/evidence binding before a production slice can become implementation-authorized.

The planning/traceability/realization artifacts remain controlled Draft working records. The workflow-realization register is the new full-corpus realization view and must reach 1,084/1,084 before the expansion may claim complete UI realization.

### Architecture Coverage vs Delivery Planning

The 99 business workspace families + 6 shared surfaces remain the **architecture coverage baseline** for the 1,084 canonical L3 scenarios. They are not a direct frontend backlog and do not imply one workspace = one implementation task.

For controlled delivery, L6 distinguishes:

- **workspace family** — sustained architecture coverage/context;
- **UX journey** — meaningful user goal, which may cross workspace families;
- **delivery experience slice** — bounded implementation/review increment inside an approved L6 package;
- **flows, states, fixtures and acceptance evidence** — supporting review material, not new architecture levels.

`L6_Experience_Slice_Register_DRAFT.md` is the thin locator for selected delivery increments. It does not duplicate canonical BH→L5 relations, allocate permanent L6 IDs, or create a new approval authority.

Architecture authorization, frontend implementation, fixture-based UX acceptance, released-contract integration and integrated production validation remain separate status dimensions.

### AI-First UI / UX Execution Plan

`L6_AI_First_UI_UX_Execution_Plan_DRAFT.md` is the historical execution plan for the completed Days 1–13 baseline. The active expansion plan is `L6_Full_Workflow_Realization_and_Mock_ERP_Execution_Plan_DRAFT.md`.

It preserves the 105-unit architecture coverage baseline while changing the implementation method from bespoke workspace-by-workspace delivery to:

```text
architecture coverage
→ UX archetype mapping
→ workspace registry
→ reusable UX patterns
→ navigable ERP breadth
→ selected deep experience slices
→ global Experience Lab review
→ controlled findings / amendment
→ backend-ready interaction evidence
```

The earlier AI-first plan established the Days 1–13 baseline. Its original 12–18-deep-workspace target is now historical for that completed phase. The active expansion target is governed by `L6_Full_Workflow_Realization_and_Mock_ERP_Execution_Plan_DRAFT.md`: 1,084/1,084 scenario realization dispositions, 99/99 browseable workspaces, full workflow discoverability, coherent mock data, and deterministic mock interactions where legitimate.

## Workflow Realization & Mock ERP Expansion

The expansion distinguishes:

- **scenario** — canonical L3 institutional outcome;
- **workspace family** — sustained L6 UI context;
- **workflow realization** — how a scenario is interacted with or observed in the UI;
- **UX journey** — user-goal path that may cross workspaces;
- **delivery experience slice** — bounded implementation/review increment;
- **fixture/mock state** — non-authoritative review evidence.

Primary realization categories are:

1. interactive workflow;
2. review/approval workflow;
3. tracking workflow;
4. exception/recovery workflow;
5. cross-workspace handoff;
6. system/background observable outcome;
7. no direct end-user interaction required.

The permanent sidebar shall remain curated. Full workflow discoverability should come from workflow-aware search, workspace-local workflow/task/exception surfaces, contextual handoffs, task/work entry and an optional Browse Workflows / Workflow Explorer surface.

Mock ERP behavior requires **no database and no backend**. Mock updates are local, deterministic, resettable, fictional and explicitly non-authoritative.

## Product / UX Decision Field Governance

`00_Governance/L6_Product_UX_Decision_Field_Map_DRAFT.md` defines the controlled decision field catalogue used by L6 authoring.

The map contains **64 core fields across eight families**:

1. Product Experience Foundation;
2. Global Shell and Information Architecture;
3. Persona, Responsibility and Home Experience;
4. Workspace Composition and Local Information Architecture;
5. Actions, Forms and Workflow Interaction;
6. Data Presentation, Exploration and Insight;
7. State, Feedback, Attention and Communication;
8. Responsive, Accessibility and Input Model.

It also defines conditional decision fields for capabilities such as onboarding, localization, files/evidence, printing, session recovery, degraded connectivity, live updates, high-risk confirmation, external-system handoffs, analytics and bulk operations.

The decision map shall be used to avoid re-asking choices already fixed upstream or already resolved by L6. A production slice should open new product-direction decisions only where a genuine unresolved UX choice materially affects the slice.

Student Workspace product/UX selections are recorded in `00_Governance/L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md`, incorporated into `L6_Student_Workspace_DRAFT.md`, and approved for controlled frontend implementation by the W-C02 proceed-gate decision.

The selected direction includes:

- one canonical Student context with responsibility-sensitive Student self-view and authorized staff presentation;
- core Student-owned record plus governed read-only cross-domain projections, with detailed work returning to canonical owning workspaces;
- balanced staff overview and conceptual local navigation `Overview / Student Record / Lifecycle / Related / Documents & History`;
- simplified Student self-service composition;
- risk-sensitive lifecycle action presentation;
- human-readable lifecycle history separated from governed audit evidence;
- integrated supported document/certificate request presentation;
- source-owned Student Lifecycle attention and ownership-preserving cross-workspace handoff;
- progressive detail, authoritative mutation-state feedback and task-sensitive responsive/mobile behavior.

No additional W-C02 product/UX decision is currently open.

## First Production Slice

```text
L6_Authenticated_Application_Shell_Context_Search_and_Notifications_DRAFT.md
```

This slice is **Architecture Approved for controlled frontend implementation** by `00_Governance/L6_Authenticated_Shell_Slice_Architecture_Approval_Decision_2026-09-16.md`.

The approved implementation boundary includes:

- authenticated shell framing and orientation;
- responsibility-sensitive navigation composition;
- Institution and academic-context presentation;
- context-switch pending/revalidation/stale-state behavior;
- global enterprise search/discovery presentation against approved/released search contracts;
- search empty/degraded/stale/unavailable states;
- notification entry/list/detail presentation framework;
- explicit separation of Notifications from workflow Tasks;
- account/session projection affordance without client ownership of credentials/session authority;
- responsive and accessibility behavior.

Live notification integration remains constrained to released upstream contracts because applicable live-integration architecture remains outside L6 ownership. The approval does not authorize provider/retry/channel/template-administration behavior in L6.

## Second Production Slice — Student Workspace

```text
L6_Student_Workspace_DRAFT.md
```

The Student Workspace was authored against exact workspace scope:

```text
L3/L4-C-001
L3/L4-C-008 … L3/L4-C-019
```

The slice preserves Student Lifecycle ownership, Person & Identity boundaries, read-only cross-domain projections, non-authoritative client state, server-authoritative authorization and protected-scope behavior.

Current W-C02 status:

```text
Product / UX decision phase        = COMPLETE
L6 draft authoring                 = COMPLETE
Exact L3/L4 scenario boundary      = RESOLVED
Mandatory Web Client Baseline      = RESOLVED
L4 parent readiness                = 13 / 13 ARCHITECTURE APPROVED
Domain BH→L5 technical coverage    = 89 / 89 VALIDATED COVERAGE
L6 hard fails 01–12                = PASS
Architecture Approval              = AUTHORIZED
Frontend implementation gate       = AUTHORIZED WITHIN CONTROLLED BOUNDARY
```

The current governing decision is `00_Governance/L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md`.

Controlled frontend implementation is authorized for the approved W-C02 composition, read-only projections, supported lifecycle action entry where released contracts exist, lifecycle/history presentation, document-request/status/result presentation, required outcome states, responsive behavior and accessibility within the recorded boundary.

The approval does **not** authorize backend/API/database/service redesign, new Student Lifecycle rules/states/events, new Person/Identity merge semantics, cross-domain edit ownership, client-side authorization authority or invented contract semantics.

## Incremental UI Implementation Review

SutraERP frontend work shall not rely on static happy-path screen review alone.

For every implementation-authorized workspace, the preferred flow is:

```text
approved L6 slice
→ reviewer scenario pack
→ explicit non-authoritative fixtures
→ primary interactive journey
→ alternate / failure / stale / denied states
→ responsive + accessibility review
→ classify findings
→ implementation fix or controlled architecture amendment
→ next increment
```

`02_Implementation_Review/README.md` defines the review framework. `02_Implementation_Review/W-C02_Student_Workspace_Review_Pack.md` is the first concrete review pack.

The frontend should provide an explicit non-production **UI Review Mode** capable of selecting representative personas, records, scenario states and outcome conditions. Review Mode is not authorization and is not a backend simulator; it exists to make approved UI behavior reproducible and critiqueable before backend integration is complete.

## Controlled Foundation Authorization

Effective 2026-09-16, repository governance explicitly authorized the initial L6 foundation for controlled production use.

The authorization establishes, for current L6 work:

- one shared SutraERP design system with persona-sensitive composition;
- calm, modern, premium, education-appropriate light-theme direction;
- responsive desktop/tablet/mobile-supported experience;
- user-goal-oriented navigation rather than an architecture/module dump;
- Administrator home centered on Institution Pulse, exceptions and true priority work;
- Faculty home centered on Teaching Pulse, next class and teaching actions rather than an enterprise ticket queue;
- Student home centered on My Semester, Up Next and Due Soon rather than an enterprise work queue;
- global shell context, search, notifications and responsive navigation as presentation concerns only, never security authority;
- dark mode, permanent numbering, exact token values and several implementation-library choices remain deferred.

The reviewed frontend implementation is evidence, not architecture authority.

## Current Governed Sequence

1. L6 foundation — **controlled authorization passed**.
2. Repository-wide screen/workspace inventory — **Draft established and corrected to 105 units**.
3. Persona/task journey and navigation mapping — **Draft established**.
4. Screen/journey full-corpus traceability register — **Draft established; 1,084 final L3 scenarios reconciled at workspace-family level**.
5. Product/UX decision field map — **Draft established; reusable field taxonomy active for controlled authoring**.
6. Authenticated shell L4/L5 binding and L6 approval — **completed**.
7. Authenticated shell frontend implementation — **authorized within its approval boundary**.
8. W-C02 Student Workspace product/UX decisions — **complete**.
9. W-C02 Student Workspace production-slice draft — **complete**.
10. W-C02 parent/L5 traceability remediation — **complete; 13/13 parents and 89/89 BHs validated**.
11. W-C02 validation/proceed gate — **passed; controlled frontend implementation authorized**.
12. Frontend/backend integration guardrails — **active derived L6 guidance**.
13. Incremental UI implementation review framework — **active; W-C02 review pack established**.
14. Material usability or feasibility findings return through controlled classification/amendment rather than silently changing architecture in code.

The 2026-09-19 L6 UX Baseline Lock supersedes this earlier controlled-authoring-only current-state statement for the established baseline. Future material detailed slices remain subject to normal per-slice upstream evidence gates and controlled amendment.

## L5 Client Architecture Inheritance

Current L6 web work inherits approved L5 client architecture, including:

- React + TypeScript;
- React Router Framework Mode;
- SPA-first rendering;
- static browser artifact;
- modular single web application;
- non-authoritative browser/client state;
- governed contract consumption;
- server-authoritative authentication/authorization/scope and business truth;
- scope-isolated protected state;
- no presentation of indeterminate mutation outcome as confirmed success.

L6 may define how these constraints are experienced by users but shall not weaken or redefine them.


## Days 9–11 Deep Backend-Defining Journeys

`L6_Days9_11_Deep_Backend_Defining_Journeys_DRAFT.md` controls the selected 18-family deep-interaction phase. It preserves the 99-workspace / 12-archetype baseline while exposing source-owned information, authoritative-decision dependencies, material outcome classes, reconciliation needs and owner-preserving handoffs required for later backend planning.

`L6_Backend_Readiness_Interaction_Evidence_Register_DRAFT.md` is the implementation-independent evidence register for those deep journeys. It is explicitly not an API, payload, database or transport specification.

Validation and readiness evidence is recorded in `00_Governance/L6_Days9_11_Deep_Backend_Defining_Journeys_Validation_and_Readiness_Record_2026-09-19.md`.

The phase keeps ordinary user UX free of L-level/workspace/reviewer metadata and retains integration maturity independently from fixture interaction depth.

## Days 12–13 Global UX Hardening

`L6_Days12_13_Global_UX_Hardening_and_Readiness_DRAFT.md` records the global hardening pass over the Days 1–11 L6/frontend corpus.

The phase preserves the established 99-workspace / 12-archetype / 81-shell / 18-interactive baseline and corrects implementation defects without redesigning the locked visual system or changing upstream business architecture.

The recorded hardening evidence includes persona-sensitive navigation alignment, Administrator-only Administration navigation, unsupported-profile action blocking, W-C02 persona-boundary correction, review-shell gating behind the non-production Experience Lab, reviewer/integration-language removal from ordinary W-C02 and owner-handoff copy, stronger governed-action/state language, 24-fixture / 121-case Experience Lab validation, unchanged maturity, and final frontend CI success.

No live browser or assistive-technology review was available in the hardening execution environment. `UX-REVIEWED` therefore remains **0**. The subsequent 2026-09-19 L6 UX Baseline Lock freezes the architecture/UX contract without upgrading UX review, integration or production-readiness maturity.

## Existing Repository Template

`templates/L6_UIUX_Template.md` is the earlier lightweight repository placeholder. It remains repository history while `00_Governance/L6_UI_UX_Architecture_Template_v0.1_DRAFT.md` is the controlled working template under the current foundation authorization.

Do not silently delete or replace the earlier placeholder. Final template promotion/deprecation remains a separate governance action.

## Naming / Numbering

No final L6 naming and numbering scheme has been locked yet.

Permanent IDs shall be introduced only through an approved naming/numbering decision. Controlled authoring before that decision shall use descriptive governed filenames and explicit upstream traceability rather than inventing permanent identifiers.


## Days 3–5 Reference UX System

`L6_Days3_5_Reference_UX_System_DRAFT.md` records the controlled multi-domain reference implementation phase. It preserves the 99-workspace and 12-archetype baselines, separates implementation/review/integration/production-validation evidence, and does not authorize Final L6 Corpus Lock.


## Days 6–8 Full ERP UX Breadth

`L6_Days6_8_Full_ERP_UX_Breadth_DRAFT.md` governs the full-corpus breadth phase. It keeps the 99-workspace / 12-archetype architecture baseline, uses the locked frontend visual system as the presentation baseline, requires meaningful shell-level representation and Global Search discovery for all 99 workspaces, and does not authorize deep workflow semantics or Final L6 Corpus Lock.


Validation evidence for this phase is recorded in:

`00_Governance/L6_Days6_8_Full_ERP_UX_Breadth_Validation_and_Readiness_Record_2026-09-19.md`.
