# L6 UI / UX Architecture

**Architecture Level:** L6 — UI / UX Architecture  
**Current State:** Controlled Foundation Authorized + First Production Slice Architecture Approved + Student Workspace Draft Authored  
**Broad Production Authoring:** Controlled slices authorized; full corpus not yet frozen or locked  
**Downstream:** SutraERP frontend implementation repository after applicable L6 slice approval/authorization  

L6 defines user interaction, navigation, screens, workspaces and user journeys derived from approved upstream architecture without owning business logic.

```text
L5 Solution Architecture
        ↓
L6 UI / UX Architecture
        ↓
Frontend Implementation
```

## Current Foundation Package

```text
00_Governance/
├── L6_Governance_and_Architecture_Standard_DRAFT.md
├── L6_UI_UX_Architecture_Template_v0.1_DRAFT.md
├── L6_Validation_Standard_DRAFT.md
├── L6_Product_UX_Decision_Field_Map_DRAFT.md
├── L6_Foundation_Controlled_Authorization_2026-09-16.md
├── L6_Authenticated_Shell_Slice_Architecture_Approval_Decision_2026-09-16.md
└── L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md

01_Foundation/
├── L6_Design_System_Foundation_DRAFT.md
├── L6_Application_Shell_and_Information_Architecture_DRAFT.md
└── L6_Interaction_State_and_Feedback_Standard_DRAFT.md
```

The six foundation artifacts remain `Draft`; they have not been silently renamed, locked, or assigned permanent L6 specification IDs. `L6_Foundation_Controlled_Authorization_2026-09-16.md` is the active Architecture Approved foundation decision authorizing controlled L6 production authoring while those drafts mature through normal governance.

`L6_Product_UX_Decision_Field_Map_DRAFT.md` is the controlled working taxonomy for product/UX decisions during L6 authoring. It defines 64 core decision fields across eight families, distinguishes upstream-fixed, product-direction, architecture-derived, slice-specific and implementation-local matters, and requires reuse of previously resolved fields before new product questions are opened.

`L6_Authenticated_Shell_Slice_Architecture_Approval_Decision_2026-09-16.md` is the first production-slice approval decision. It authorizes controlled frontend implementation of the shared authenticated shell, Institution/academic context presentation and switching, global search/discovery, notification presentation framework, account/session projection affordance, responsive shell behavior and accessibility requirements within its recorded boundaries.

`L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md` records the selected W-C02 product/UX direction. Its decisions remain pending formal L6 slice approval and do not independently authorize frontend implementation.

## Current Planning and Traceability Package

```text
L6_Screen_and_Workspace_Inventory_DRAFT.md
L6_Persona_Journey_and_Navigation_Map_DRAFT.md
L6_Screen_Journey_Traceability_Register_DRAFT.md
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

The three planning/traceability artifacts remain `Draft`. They do not constitute a final screen corpus, final navigation lock or Final L6 Corpus Lock.

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

Student Workspace product/UX selections have now been recorded in `00_Governance/L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md` and incorporated into `L6_Student_Workspace_DRAFT.md`. They remain **Selected — Pending Formal L6 Slice Approval** rather than Architecture Approved.

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

No additional W-C02 product/UX question is currently open.

## First Production Slice

```text
L6_Authenticated_Application_Shell_Context_Search_and_Notifications_DRAFT.md
```

This first production slice is **Architecture Approved for controlled frontend implementation** by `00_Governance/L6_Authenticated_Shell_Slice_Architecture_Approval_Decision_2026-09-16.md`.

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

Live notification integration remains constrained to released upstream contracts because `L5-C-04-001/002/003` still carry `Review` metadata. The approval does not promote those L5 documents or authorize provider/retry/channel/template-administration behavior in L6.

## Second Production Slice — Student Workspace

```text
L6_Student_Workspace_DRAFT.md
```

The Student Workspace draft has been authored against exact workspace scope:

```text
L3/L4-C-001
L3/L4-C-008 … L3/L4-C-019
```

The draft preserves Student Lifecycle ownership, Person & Identity boundaries, read-only cross-domain projections, non-authoritative client state, server-authoritative authorization and protected-scope behavior.

Current W-C02 status:

```text
Product / UX decision phase        = complete
L6 draft authoring                 = complete
Exact L3/L4 scenario boundary      = resolved
Mandatory Web Client Baseline      = resolved
Domain-specific BH→L5 evidence     = open
Architecture Approval              = not yet authorized
Frontend implementation gate       = not yet authorized
```

The blocking issue is **HF-L6-12 / parent traceability maturity**, not an unresolved UX preference. The locked L5 traceability methodology requires BH-level many-to-many L4→L5 evidence; Draft L4 relations cannot become `Validated`. At least some W-C02 parent L4 artifacts still carry `Draft` metadata, and the live repository does not currently expose a canonical Section-C BH→L5 relation store sufficient to prove validated coverage for the full W-C02 slice.

Therefore W-C02 is a controlled production-slice draft but is **not `Slice Ready` and is not yet authorized for production frontend implementation**.

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

The reviewed frontend implementation recorded in the authorization is evidence, not architecture authority.

## Current Governed Sequence

1. L6 foundation — **controlled authorization passed**.
2. Repository-wide screen/workspace inventory — **Draft established and corrected to 105 units**.
3. Persona/task journey and navigation mapping — **Draft established**.
4. Screen/journey full-corpus traceability register — **Draft established; 1,084 final L3 scenarios reconciled at workspace-family level**.
5. Product/UX decision field map — **Draft established; reusable field taxonomy active for controlled authoring**.
6. First production slice L4/L5 binding — **completed for authenticated shell/context/search/notification presentation**.
7. First production L6 slice authoring — **completed**.
8. Independent L6 validation / proceed gate — **passed with controlled C04 live-integration constraint**.
9. Frontend implementation of the approved shell slice — **authorized to proceed within the approval decision boundary**.
10. W-C02 Student Workspace product/UX decisions — **selected and recorded**.
11. W-C02 Student Workspace production-slice draft — **authored**.
12. W-C02 domain-specific BH→L5 evidence / parent maturity gate — **open; blocks Architecture Approval and frontend implementation authorization**.
13. Material usability or feasibility findings return through controlled L6 amendment rather than silently changing architecture in code.

This authorization is not a Final L6 Corpus Lock and does not freeze the final screen population.

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

## Existing Repository Template

`templates/L6_UIUX_Template.md` is the earlier lightweight repository placeholder. It remains repository history while `00_Governance/L6_UI_UX_Architecture_Template_v0.1_DRAFT.md` is the controlled working template under the current foundation authorization.

Do not silently delete or replace the earlier placeholder. Final template promotion/deprecation remains a separate governance action.

## Naming / Numbering

No final L6 naming and numbering scheme has been locked yet.

Permanent IDs shall be introduced only through an approved naming/numbering decision. Controlled authoring before that decision shall use descriptive governed filenames and explicit upstream traceability rather than inventing permanent identifiers.
