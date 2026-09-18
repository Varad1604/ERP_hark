# SutraERP L6 UI / UX Governance and Architecture Standard — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Governance  
**Last Updated:** 2026-09-16  
**Depends On:** Locked/approved L0–L5 architecture; `L5-G-01-001`; `L5-G-01-002`; `L5-G-01-003`; Architecture Level Validation Standards; Repository Constitution.  
**Downstream:** SutraERP frontend implementation repository and UI test artifacts.  

> This document is a review draft. It does not create a locked L6 numbering scheme and does not authorize broad L6 production authoring until approved by repository governance.

---

## 1. Purpose

L6 defines the user interaction, navigation, presentation, workspace, screen, journey, accessibility and experience architecture required to realize approved L5 solution architecture without owning or redefining business logic.

L6 is the final architecture layer before frontend implementation.

```text
L0 Business Vision
  ↓
L1 Business Domain Architecture
  ↓
L2 Domain Workflow Architecture
  ↓
L3 Business Scenario Architecture
  ↓
L4 System Behavior Specification
  ↓
L5 Solution Architecture
  ↓
L6 UI / UX Architecture
  ↓
Frontend Implementation
```

## 2. L6 Owns

L6 may define:

- information architecture and user-facing navigation semantics;
- application-shell experience and workspace composition;
- screen and page responsibilities;
- user journeys and interaction sequences;
- visual hierarchy and information density;
- design-system semantics and reusable interaction patterns;
- form, table, filter, search and task interaction patterns;
- loading, empty, unavailable, failure, rejection, pending and success presentation;
- accessibility, responsive behavior and keyboard interaction requirements;
- presentation of canonical lifecycle/business states without redefining them;
- presentation of authoritative decisions, events, audit/evidence and cross-domain facts;
- persona-sensitive presentation where supported by upstream architecture;
- implementation-independent acceptance criteria for UI/UX behavior.

## 3. L6 Must Not Own

L6 shall not invent, redefine or transfer:

- canonical business entities or ownership;
- business rules, thresholds, calculations or approval authority;
- business lifecycle states or transitions;
- business events or publishers;
- authorization policy or authoritative Authorization Decisions;
- authentication/session lifecycle;
- Tenant/Institution validation authority;
- API contracts, payload schemas or endpoint design;
- database/persistence design;
- backend service/component boundaries;
- infrastructure/deployment architecture;
- technology selections already owned by L5.

A screen, route, button, hidden control or disabled state is never authorization authority.

## 4. Mandatory L5 Inheritance

All SutraERP L6 web architecture shall preserve the currently approved L5 client constraints:

- React + TypeScript web realization;
- React Router Framework Mode;
- SPA-first client rendering;
- static deployable web artifact;
- one modular web application unless future ADR governance changes the topology;
- client/browser state is non-authoritative;
- protected operations execute through governed contract-consumption boundaries;
- Tenant/Institution context remains server-validated and fail-closed;
- authentication and authorization remain separate;
- protected server-state projections are memory-scoped by default;
- UI capability/visibility data is advisory and does not replace server enforcement;
- indeterminate mutation outcomes must not be presented as confirmed business success;
- no offline authoritative business mutation is assumed by L6.

L6 may choose how these constraints are experienced by users, but it may not weaken them.

## 5. Repository and Implementation Separation

The architecture repository is authoritative for L6 decisions. The frontend repository implements approved L6 architecture.

```text
SutraERP_Architecture
      ↓ approved L6
SutraERP_frontend
      ↓ implementation evidence
UX / implementation findings
      ↓
Controlled L6 amendment when architecture changes are required
```

Frontend implementation may reveal usability or feasibility findings. Such findings do not silently change architecture.

## 6. Architecture-Led Parallel Delivery Model

SutraERP shall use a gated pipeline:

```text
L6 Foundation
  ↓ review / approval
Approved L6 slice N
  ↓
Frontend implementation slice N
  ↓
Usability + implementation review
  ↓
L6 amendment if materially required
  ↓
Scale pattern to next approved slice
```

Once the foundation is stable, architecture, implementation and review may operate in parallel on different slices, provided implementation never works ahead of an approved L6 boundary.

## 7. Required L6 Artifact Content

A production L6 artifact should contain, where applicable:

1. Metadata and upstream traceability.
2. Purpose and user outcome.
3. L6 boundary and non-goals.
4. Personas/actors and context.
5. Information architecture and navigation placement.
6. Workspace/screen responsibilities.
7. Primary and alternate user journeys.
8. Data presentation and authoritative-source indication where material.
9. User actions and interaction states.
10. Validation, rejection, pending and recovery presentation.
11. Accessibility and responsive requirements.
12. Audit/evidence/history presentation where required.
13. Design-system components/patterns used.
14. Observable UX completion/acceptance conditions.
15. Traceability to L5 and material L4/L3 sources.
16. Open issues and deferred implementation details.

## 8. Workspace-First Principle

SutraERP is an Institution Operating Platform, not a collection of unrelated CRUD pages.

L6 should prefer coherent task/workspace architecture over exposing backend entities as isolated Create/Edit/View pages.

A workspace may compose read-only or actionable projections from multiple domains while preserving each domain's canonical ownership and authority.

## 9. Navigation Principle

Architecture sections A–J, backend modules, L5 families and React feature-module boundaries do not automatically become user-facing navigation categories.

Navigation shall be derived from:

- user goals and institutional tasks;
- persona responsibilities;
- frequency and urgency;
- workflow continuity;
- cross-domain task composition;
- discoverability and cognitive load;
- institutional scope/context.

## 10. Design-System Principle

The Sutra design system shall define reusable experience semantics without becoming a business-rule owner.

Design tokens and components shall support at minimum:

- semantic status communication;
- high-density enterprise data presentation;
- accessible focus/keyboard behavior;
- forms and validation;
- tables, filters and bulk actions;
- workflow state and decision presentation;
- audit/history presentation;
- loading/empty/error/unavailable states;
- responsive adaptation;
- consistent destructive/irreversible action treatment.

## 11. State Presentation Integrity

L6 shall distinguish at least:

- authoritative success;
- authoritative rejection/denial;
- pending/in-progress client operation;
- indeterminate technical outcome;
- unavailable/technical failure;
- stale or invalidated projection;
- no data / legitimate empty state.

Technical uncertainty shall never be visually collapsed into business success.

## 12. Accessibility Baseline

L6 shall target WCAG 2.2 AA as the minimum product accessibility objective unless repository governance later establishes a stricter requirement.

Every interaction pattern shall account for:

- keyboard operation;
- visible focus;
- semantic structure;
- non-color-only status communication;
- accessible names and descriptions;
- error identification and recovery;
- scalable text and responsive reflow;
- motion sensitivity where animation is used.

This target is a proposed L6 requirement in this Draft and becomes binding only when approved.

## 13. Responsive and Density Model

SutraERP is desktop-priority for complex institutional operations but shall remain responsive rather than desktop-only.

L6 shall define:

- comfortable default density;
- compact enterprise-density variants where justified;
- tablet adaptation for operational workflows;
- mobile behavior for supported tasks;
- explicit unsupported/degraded behavior where a complex task cannot responsibly fit a small viewport.

Responsive behavior shall preserve task meaning and authority boundaries rather than merely compressing layouts.

## 14. Traceability

Every production L6 artifact shall trace to one or more approved L5 sources.

Material L4/L3 references may be included where they are necessary to verify user-visible system behavior, business outcome, authority, lifecycle state, or event semantics.

L6 shall not bypass L5 to invent implementation architecture.

## 15. Status and Review

Recommended lifecycle:

```text
Draft → Review → Architecture Approved → Locked
```

- Authors may draft but may not self-lock.
- Material cross-domain UX shall receive affected-domain review where necessary.
- Frontend implementation evidence may inform review but cannot replace architecture approval.

## 16. Foundation Gate

Broad frontend production implementation should begin only after the following L6 foundation is Architecture Approved or otherwise explicitly authorized:

- L6 Governance and Architecture Standard;
- canonical L6 artifact template;
- Sutra Design System Foundation;
- Application Shell and Information Architecture;
- Interaction, State and Feedback Standard.

A controlled prototype used to validate these drafts may be authorized separately, but it shall not become the de facto architecture baseline.

## 17. Open Governance Decisions

The following are intentionally not locked by this Draft:

- canonical L6 numbering and naming scheme;
- final production folder structure beyond the current L6 root;
- component library selection;
- styling utility/framework selection;
- icon library;
- state-management/cache library;
- Storybook or equivalent tooling;
- exact responsive breakpoints;
- final visual brand language.

These require review and, where technology/material implementation choices are involved, alignment with L5/ADR governance.

---

**Draft verdict:** suitable for architecture review; not yet canonical or Locked.


---

## 16. Delivery Experience Slice Model

The repository-wide workspace inventory exists for architecture coverage and sustained context. It shall not be treated as a one-row-per-frontend-task plan.

For implementation-authorized L6 packages, delivery may be decomposed into **experience slices**:

> An experience slice is a bounded, traceable increment in which a specified user, in a specified context, can achieve one meaningful observable outcome—or reach a clearly defined handoff—with the material alternate, failure, accessibility and responsive behavior reviewable.

Rules:

1. A delivery experience slice is not a new architecture level.
2. One L3 scenario does not automatically equal one experience slice.
3. One workspace family does not automatically equal one implementation task.
4. One component, tab, modal, route or API call does not automatically equal one experience slice.
5. A UX journey may cross workspace-family boundaries while canonical ownership remains unchanged.
6. A delivery experience slice may inherit an existing approved L6 package boundary; implementation-local refactors, fixtures and review cases do not require a new formal architecture approval unless they materially change approved L6 semantics.
7. Flow descriptions, state matrices, fixture sets and acceptance evidence support a slice but shall not become separate central architecture registers.
8. Delivery status shall distinguish architecture authorization, frontend implementation, fixture-based UX acceptance, released-contract integration and integrated production validation.

`L6_Experience_Slice_Register_DRAFT.md` may be used as a thin delivery locator for selected work. It shall link—not duplicate—canonical traceability evidence.
