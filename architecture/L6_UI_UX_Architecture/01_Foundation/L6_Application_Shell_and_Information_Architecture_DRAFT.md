# SutraERP L6 — Application Shell and Information Architecture — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** L6 Governance and Architecture Standard (Draft); L6 Design System Foundation (Draft); `L5-G-01-001`; `L5-G-01-002`; `L5-G-01-003`; applicable Identity/Authorization/Tenant-Institution L5 architecture.  

---

## 1. Purpose

Define the user-facing application shell and top-level information architecture for SutraERP without treating backend architecture sections, route modules or permissions as the navigation model.

The shell must keep users oriented across institutional scope, workspaces and task context while preserving L5 client-security and non-authoritative-state constraints.

## 2. Shell Responsibilities

The application shell may provide:

- SutraERP product identity;
- current Tenant/Institution context presentation;
- applicable College/School/Campus/academic context selectors where upstream-supported;
- global navigation;
- global search/discovery entry point;
- notifications/task inbox entry point where supported;
- user/account/session presentation;
- breadcrumbs/context trail;
- workspace title/context/actions region;
- global status/unavailable presentation;
- responsive navigation behavior.

The shell does not own canonical institution, identity, authorization, notification or workflow data.

## 3. Shell Must Not Do

The shell shall not:

- infer authorization from the presence of a navigation item;
- create its own Tenant/Institution truth;
- retain stale protected scope after governed context change;
- present a route load as proof of permission;
- redefine business lifecycle state;
- collapse domain ownership into a shell-owned mega-model;
- expose architecture sections A–J merely because they exist technically.

## 4. Conceptual Shell Regions

```text
┌───────────────────────────────────────────────────────────────┐
│ Product / Scope / Search / Tasks / User                      │
├────────────────┬──────────────────────────────────────────────┤
│ Primary        │ Context trail / workspace title / actions    │
│ navigation     ├──────────────────────────────────────────────┤
│                │                                              │
│                │ Workspace content                            │
│                │                                              │
│                │                                              │
└────────────────┴──────────────────────────────────────────────┘
```

This is structural architecture, not a final visual layout.

## 5. Information Architecture Principles

### 5.1 User Goal First

Primary navigation shall group capabilities by recognizable institutional work rather than internal technical structure.

### 5.2 Stable Orientation

Users should always be able to identify:

- where they are;
- which institution/scope they are acting within;
- the current workspace/task;
- whether they are viewing or acting on authoritative/current data;
- how to return to a broader context.

### 5.3 Progressive Disclosure

High-frequency work should remain directly reachable. Lower-frequency administration, history and configuration should be discoverable without dominating the primary shell.

### 5.4 Cross-Domain Workspaces

A workspace may compose facts/actions from multiple canonical owners when the user task spans domains. Composition does not transfer ownership.

### 5.5 No Module Dump

The shell shall not expose every L2 domain, L5 family or internal service as a navigation item.

## 6. Proposed Top-Level Navigation Families

The following are **candidate user-facing families for validation**, not locked canonical labels:

- Home / Work Queue;
- Students;
- Academics;
- Examinations;
- Finance;
- People;
- Campus Operations;
- Resources / Procurement & Inventory;
- Institutional Services;
- Reports / Intelligence;
- Administration / Settings.

These labels require persona/task validation before approval. Section ownership remains unchanged regardless of navigation grouping.

## 7. Home / Work Queue Concept

The default authenticated landing experience should prioritize actionable institutional work rather than a generic chart dashboard.

Candidate composition:

- tasks requiring user action;
- approvals/reviews where the user has upstream-supported authority;
- exceptions requiring attention;
- recently accessed workspaces;
- relevant institutional notices;
- concise KPI/context summaries where meaningful.

Dashboard content remains a read/composition surface and does not own operational facts.

## 8. Scope and Context Switching

The shell may allow a user to select among contexts they are permitted to access, such as Tenant/Institution/College/School/Campus/academic period where applicable.

L5 constraints apply:

1. client-selected context is non-authoritative input;
2. server-side validation remains mandatory;
3. material protected-scope change triggers the G01-003 invalidation/reconciliation boundary;
4. prior protected projections must not remain active under the new context;
5. UI shall communicate context change before users continue protected work.

## 9. Global Search

Global search is a discovery surface, not a canonical data owner.

Search results shall:

- respect applicable authorization/privacy boundaries;
- preserve entity/context identity;
- avoid cross-Tenant/Institution leakage;
- deep-link to the appropriate workspace/detail context;
- distinguish result types clearly;
- not imply search availability equals mutation permission.

## 10. Notifications and Task Entry

Notifications and actionable tasks should be distinct concepts where architecture supports both.

- Notification: information delivered to the user.
- Task/work item: an action or decision the user may need to perform.

The shell may surface both, but shall not invent workflow authority or task lifecycle.

## 11. Workspace Architecture

A workspace is the preferred top-level unit for sustained institutional work.

A workspace should contain:

- identity/context header;
- current state/summary;
- primary task actions;
- related sections/tabs only when they support the coherent user goal;
- history/evidence where material;
- cross-domain projections clearly contextualized;
- exception/recovery information.

### Example Pattern — Student Workspace

Candidate sections might include:

```text
Overview
Identity & Institutional Context
Academic Journey
Registration
Attendance
Finance
Examination
Documents
Guardian Relationships
Progression / Qualification
Activity / Audit History
```

This is illustrative only; final composition must be derived from approved upstream behavior and persona needs.

## 12. Detail vs Workspace vs Task Screen

Use:

- **Detail surface** when the user primarily needs to inspect one coherent record/context.
- **Workspace** when the user performs sustained multi-step or cross-domain work around one institutional subject/task.
- **Task screen** when a bounded decision/action requires focused completion.

Do not force every interaction into a full workspace.

## 13. Navigation State and Deep Links

The UX architecture should support stable deep links to meaningful workspace/task contexts where security and state semantics allow.

A URL/route may identify presentation context, but it does not establish:

- authorization;
- Tenant/Institution validity;
- business state;
- ownership.

## 14. Breadcrumb and Orientation Model

Breadcrumbs should represent user-facing information hierarchy, not necessarily React route nesting.

Candidate form:

```text
Students > Student Workspace > Academic Journey
```

Context selectors (Institution, Campus, Academic Year, etc.) remain separate from breadcrumbs where mixing them would create ambiguity.

## 15. Action Placement

Actions should be placed by scope:

- global actions in the application shell only when truly global;
- workspace actions near workspace title/context;
- section actions inside their section;
- row/item actions near the relevant item;
- destructive actions separated visually and semantically from routine actions.

Primary action prominence shall not imply business authority.

## 16. Responsive Shell

### Desktop

Persistent primary navigation is preferred where viewport permits.

### Tablet / Narrow Desktop

Navigation may collapse while retaining current location and scope visibility.

### Mobile

Only supported mobile tasks should be optimized. Complex enterprise workflows may use a simplified navigation model but shall not lose essential context or state meaning.

## 17. Accessibility

The shell shall provide:

- skip navigation mechanism;
- landmark structure;
- keyboard-operable navigation;
- visible focus;
- accessible current-page indication;
- announced scope/context changes where necessary;
- logical focus restoration after dialogs/drawers/navigation transitions;
- meaningful page/workspace titles.

## 18. Frontend Implementation Handoff

The frontend repository should implement the shell as part of the single modular React application established by L5-G-01-002.

Technical React route/module structure may differ from user-facing navigation. The implementation shall preserve this distinction.

## 19. Open Decisions

- final top-level navigation labels/order;
- role/persona-specific navigation adaptation;
- home/work-queue composition;
- global search scope and interaction pattern;
- notification/task inbox composition;
- exact scope-selector hierarchy;
- mobile supported task set;
- command palette / keyboard shortcut requirement;
- breadcrumb behavior for cross-domain workspaces.

---

**Draft verdict:** suitable for information-architecture review and first shell prototype; not yet canonical.
