# L6 UI / UX Architecture

**Architecture Level:** L6 — UI / UX Architecture  
**Current State:** Foundation Drafts Under Review  
**Broad Production Authoring:** Not Yet Authorized  
**Downstream:** SutraERP frontend implementation repository after foundation approval  

L6 defines user interaction, navigation, screens, workspaces and user journeys derived from approved L5 Solution Architecture without owning business logic.

```text
L5 Solution Architecture
        ↓
L6 UI / UX Architecture
        ↓
Frontend Implementation
```

## Current Draft Foundation Package

The first L6 foundation review package is located under:

```text
00_Governance/
├── L6_Governance_and_Architecture_Standard_DRAFT.md
├── L6_UI_UX_Architecture_Template_v0.1_DRAFT.md
└── L6_Validation_Standard_DRAFT.md

01_Foundation/
├── L6_Design_System_Foundation_DRAFT.md
├── L6_Application_Shell_and_Information_Architecture_DRAFT.md
└── L6_Interaction_State_and_Feedback_Standard_DRAFT.md
```

All six artifacts are `Draft`. Their presence does not establish a Locked L6 baseline, canonical L6 numbering scheme or broad production-authoring authorization.

## Foundation Gate

Before broad frontend production implementation begins, repository governance should approve or explicitly authorize the L6 foundation covering:

- L6 ownership/boundary and cross-repository governance;
- canonical L6 artifact template;
- L6 validation/proceed-gate rules;
- Sutra design-system foundation;
- application shell and information architecture;
- interaction/state/feedback semantics.

After the foundation gate, L6 architecture and frontend implementation may proceed in a controlled pipeline where implementation never works ahead of an approved L6 boundary.

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

`templates/L6_UIUX_Template.md` is the earlier lightweight repository placeholder. It remains repository history while the more complete `00_Governance/L6_UI_UX_Architecture_Template_v0.1_DRAFT.md` is under review.

Do not treat the Draft replacement as canonical until it receives governance approval. If approved, repository governance should then reconcile or deprecate the older placeholder explicitly rather than silently.

## Naming / Numbering

No final L6 naming and numbering scheme has been locked yet. Draft foundation files therefore do not allocate permanent L6 specification IDs.

Permanent IDs shall be introduced only through an approved naming/numbering decision.
