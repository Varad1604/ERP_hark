# L6 UI / UX Architecture

**Architecture Level:** L6 — UI / UX Architecture  
**Current State:** Controlled Foundation Authorized  
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
└── L6_Foundation_Controlled_Authorization_2026-09-16.md

01_Foundation/
├── L6_Design_System_Foundation_DRAFT.md
├── L6_Application_Shell_and_Information_Architecture_DRAFT.md
└── L6_Interaction_State_and_Feedback_Standard_DRAFT.md
```

The six foundation artifacts remain `Draft`; they have not been silently renamed, locked, or assigned permanent L6 specification IDs. `L6_Foundation_Controlled_Authorization_2026-09-16.md` is the active Architecture Approved decision authorizing controlled L6 production authoring and implementation slices while those drafts mature through normal governance.

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
- an explicit per-slice gate requiring material L4 BH→L5 specification/evidence binding before a production slice can become `Slice Ready`.

The three planning/traceability artifacts remain `Draft`. They do not constitute a final screen corpus, final navigation lock or Final L6 Corpus Lock.

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
5. Exact production-slice L4 BH→L5 specification/evidence binding — **required next**.
6. Production L6 slice authoring using the controlled template.
7. Independent L6 validation / proceed gate.
8. Frontend implementation only after the corresponding L6 boundary is sufficiently approved/authorized.
9. Material usability or feasibility findings return through controlled L6 amendment rather than silently changing architecture in code.

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
