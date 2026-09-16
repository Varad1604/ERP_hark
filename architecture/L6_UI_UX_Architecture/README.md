# L6 UI / UX Architecture

**Architecture Level:** L6 — UI / UX Architecture  
**Current State:** Controlled Foundation Authorized  
**Broad Production Authoring:** Controlled slices authorized; full corpus not yet frozen or locked  
**Downstream:** SutraERP frontend implementation repository after applicable L6 slice approval/authorization  

L6 defines user interaction, navigation, screens, workspaces and user journeys derived from approved L5 Solution Architecture without owning business logic.

```text
L5 Solution Architecture
        ↓
L6 UI / UX Architecture
        ↓
Frontend Implementation
```

## Current Foundation Package

The initial L6 foundation package is located under:

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

The six foundation artifacts remain `Draft`; they have not been silently renamed, locked, or assigned permanent L6 specification IDs. `L6_Foundation_Controlled_Authorization_2026-09-16.md` is the active Architecture Approved decision authorizing controlled L6 production authoring and implementation slices while the drafts mature through normal governance.

## Controlled Foundation Authorization

Effective 2026-09-16, repository governance has explicitly authorized the initial L6 foundation for controlled production use.

The authorization records the approved product experience direction and resolves the following matters for current L6 work:

- one shared SutraERP design system with persona-sensitive composition;
- calm, modern, premium, education-appropriate light-theme direction;
- responsive desktop/tablet/mobile-supported experience;
- user-goal-oriented navigation rather than an architecture/module dump;
- Administrator home centered on Institution Pulse, exceptions and true priority work;
- Faculty home centered on Teaching Pulse, next class and teaching actions rather than an enterprise ticket queue;
- Student home centered on My Semester, Up Next and Due Soon rather than an enterprise work queue;
- global shell context, search, notifications and responsive navigation remain presentation concerns only and never security authority;
- dark mode, permanent numbering, exact token values and several implementation-library choices remain deferred.

The reviewed frontend implementation at authorization time is recorded as evidence in the decision document; it is not the architecture authority.

## Foundation Gate

The foundation gate is now **passed for controlled L6 authoring**, subject to the boundaries in `L6_Foundation_Controlled_Authorization_2026-09-16.md`.

The next governed work is:

1. derive the complete screen/workspace and user-journey inventory from approved upstream architecture;
2. validate persona/task navigation;
3. define production L6 slices using the controlled draft template;
4. implement frontend slices only after the corresponding L6 boundary is sufficiently approved/authorized;
5. feed material usability or feasibility findings back through controlled L6 amendment rather than silently changing architecture in code.

This authorization is not a Final L6 Corpus Lock and does not freeze a complete screen population.

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

`templates/L6_UIUX_Template.md` is the earlier lightweight repository placeholder. It remains repository history while the more complete `00_Governance/L6_UI_UX_Architecture_Template_v0.1_DRAFT.md` is the controlled working template under the current foundation authorization.

Do not silently delete or replace the earlier placeholder. Final template promotion/deprecation remains a separate governance action.

## Naming / Numbering

No final L6 naming and numbering scheme has been locked yet.

Permanent IDs shall be introduced only through an approved naming/numbering decision. Controlled authoring before that decision shall use descriptive governed filenames and explicit upstream traceability rather than inventing permanent identifiers.
