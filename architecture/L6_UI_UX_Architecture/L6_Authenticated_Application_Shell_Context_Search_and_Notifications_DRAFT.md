# SutraERP L6 — Authenticated Application Shell, Context, Search & Notifications — Draft

## 1. Specification Metadata

**L6 Identifier:** Provisional descriptive identifier only; permanent L6 numbering is not yet approved  
**Architecture Level:** L6 — UI / UX Architecture  
**Primary User Outcome:** An authenticated user can remain oriented in the correct institutional context, navigate to permitted work, discover authorized enterprise entities/workspaces and consume governed notifications without the client becoming authority for authentication, authorization, scope or business truth.  
**Primary Business Context:** Shared authenticated application shell; enterprise discovery; governed notification presentation  
**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Last Updated:** 2026-09-16  
**Depends On:** `00_Governance/L6_Foundation_Controlled_Authorization_2026-09-16.md`; `01_Foundation/L6_Application_Shell_and_Information_Architecture_DRAFT.md`; `01_Foundation/L6_Interaction_State_and_Feedback_Standard_DRAFT.md`; `L6_Persona_Journey_and_Navigation_Map_DRAFT.md`; `L6_Screen_Journey_Traceability_Register_DRAFT.md`; applicable L3/L4/L5 sources listed in §19.  
**Material Upstream Verification:** `L3/L4-A-007`; `L3/L4-A-050`; `L3/L4-B-001`; `L3/L4-B-027`; `L3/L4-B-028`; `L3/L4-H-010`; `L3/L4-H-013`; `L3/L4-H-014`; `L3/L4-H-017`; `L3/L4-H-018`; `L3/L4-H-019`.  
**Frontend Implementation Reference:** `DhruvNaik26/SutraERP_frontend` — current shell implementation is downstream evidence/target only and is not architecture authority.

> This slice defines interaction and presentation architecture only. It does not define routes, APIs, DTOs, database structures, security policy, session mechanics, notification-provider mechanics or search-engine implementation.

---

## 2. Purpose and User Outcome

This slice defines the shared authenticated shell that surrounds SutraERP workspaces.

The user shall be able to answer, without relying on hidden technical knowledge:

- who/which authenticated context is currently represented;
- which Tenant/Institution and applicable academic context is active;
- where the user is in the product;
- which work destinations are presented for the current responsibility context;
- how to discover authorized entities/workspaces;
- whether a search or notification result is informational, stale, unavailable or restricted;
- how to continue into the canonical owning workspace without treating shell visibility as authority.

The shell is a composition/orientation layer. It owns no canonical business record, authorization decision, Tenant/Institution fact, Searchable Projection, Notification or task lifecycle.

---

## 3. Scope and Boundary

### In Scope

- authenticated application-shell structure and orientation;
- responsibility-aware navigation presentation using the governed L6 journey map;
- current Tenant/Institution and applicable academic-context presentation;
- governed context-switch interaction and stale-state containment;
- global enterprise search/discovery entry and result presentation;
- notification entry, list/detail presentation and permitted preference entry;
- account/session presentation as a non-authoritative client projection;
- workspace header/context trail and content outlet responsibilities;
- loading, empty, unavailable, stale and partial-composition presentation;
- responsive and accessibility behavior for the shell/search/notification interactions.

### Out of Scope

- sign-in, credential recovery and MFA interaction design beyond the authenticated shell boundary;
- Role, Permission, policy, authorization or Workspace business-definition ownership;
- Tenant/Institution or Academic Calendar lifecycle management;
- notification creation, template authoring/approval, scheduling administration, provider selection, delivery retry/failover or provider receipt administration;
- workflow task/work-queue lifecycle (`S-06` remains a separate L6 surface);
- search-index administration, ranking configuration, synonym governance or projection onboarding;
- canonical entity editing from search results;
- final persona-home composition beyond the already authorized foundation direction;
- API/database/service/deployment design;
- permanent route names, L6 IDs, component library or exact responsive breakpoint values.

---

## 4. Personas, Actors and Operating Context

| UX profile / actor basis | Shell responsibility | Upstream authority |
|---|---|---|
| Authenticated User | Receive authenticated shell only after authoritative session context exists | `L3/L4-A-007`; L5-D01; L5-G-01-003 |
| Student | Receive Student-oriented navigation composition where upstream responsibility permits | L6 persona map + applicable owning-domain authorization |
| Faculty / Teacher | Receive teaching-oriented navigation composition where upstream responsibility permits | L6 persona map + applicable owning-domain authorization |
| Guardian / Authorized Representative | Receive relationship-scoped composition only where upstream relationship/authority permits | L6 persona map + upstream Guardian/authorization architecture |
| Institutional Administrator / specialist | Receive applicable operational work families without assuming all institutional work is visible | L6 persona map + D02 authorization |
| Multi-responsibility user | Receive composed navigation without duplicating destinations or treating a persona label as a Role | `L3/L4-A-050`; L5-D02 |

A UX profile is presentation composition only. It is not a Role, Permission or Authorization Decision.

---

## 5. Information Architecture and Navigation Placement

### 5.1 Shell Regions

The shared authenticated shell shall provide these conceptual regions where applicable:

```text
Product identity / current scope / global discovery / notifications / account
                                ↓
Primary navigation + current workspace orientation
                                ↓
Workspace header / context trail / workspace actions
                                ↓
Workspace content
```

This is structural architecture, not a fixed pixel layout.

### 5.2 Navigation Composition

- Primary navigation shall use the approved persona/task journey map rather than architecture sections A–J.
- Navigation visibility may be informed by upstream capability/responsibility projections for usability but never becomes an authorization decision.
- Deep links and search results may bypass primary hierarchy and enter an owning workspace directly where supported.
- Duplicate destinations caused by multiple responsibilities shall be merged into one understandable destination where the user outcome is the same.
- The prototype persona switcher is not part of the production authority model.

### 5.3 Orientation

The user shall always be able to identify:

- current product/workspace location;
- current confirmed Institution context when applicable;
- current academic context when applicable;
- whether the currently displayed protected content belongs to the confirmed context;
- an understandable path back to a broader workspace or Home.

---

## 6. Workspace / Screen Responsibilities

| Surface | Responsibility | Authoritative facts presented | Actions exposed |
|---|---|---|---|
| Shared shell | Product identity, orientation, navigation, status and shared entry points | Session availability projection; validated context projection; current navigation composition | Navigate; invoke context/search/notification/account surfaces |
| Context control | Show current validated context and permitted alternatives | Tenant/Institution and applicable academic-context references returned through approved contracts | Request context switch; cancel/close selector |
| Global search/discovery | Discover permitted enterprise references without owning source facts | Search projections, result type, canonical reference/context and freshness metadata where supplied | Search/filter within supported scope; open owning workspace/detail |
| Notification entry/center | Present governed Notification information distinctly from workflow tasks | Notification identity/content/classification/outcome information supplied by H02 contracts | Open notification/detail; navigate to referenced destination; preference action only where upstream supports it |
| Account/session affordance | Present current user/account/session context and supported account actions | Non-secret session/account projection | Open account/session actions supplied by owning architecture |
| Workspace frame | Present current workspace title/context and bounded actions | Source-owned facts supplied to the workspace | Workspace-specific navigation/actions defined by that workspace's L6 slice |

A shell composition may present facts from several owners but does not transfer ownership.

---

## 7. Primary User Journey

### 7.1 Authenticated Entry

```text
authoritative authenticated context available
        ↓
client reconstructs non-secret session projection
        ↓
validated Tenant/Institution context established
        ↓
responsibility/navigation composition resolved
        ↓
shared shell becomes available
        ↓
user enters Home or a valid deep-linked workspace
```

Authentication success shall not be presented as business-operation authorization.

### 7.2 Context Switch

```text
current confirmed context
        ↓
user opens context selector
        ↓
selects another presented context
        ↓
client treats selection as non-authoritative request
        ↓
prior protected projections are quarantined/invalidated as required
        ↓
authoritative context revalidation
        ├─ confirmed → new context becomes active; valid destination restored
        └─ rejected/unavailable → requested context is not shown as active
```

A failed switch shall never leave data from the previous context visually active under the newly requested context label.

### 7.3 Global Discovery

```text
search/discovery entry
        ↓
user supplies supported discovery criteria
        ↓
authorized H03 search/discovery projection returned
        ↓
results grouped/distinguished by understandable entity/workspace type
        ↓
user opens one result
        ↓
canonical reference is resolved by the owning workspace
```

Search result visibility does not imply edit/action permission.

### 7.4 Notification Consumption

```text
notification entry
        ↓
current governed notifications presented
        ↓
user opens one notification
        ↓
source context / message / classification presented
        ↓
optional supported destination opened
```

Notifications remain informational communication outcomes. They are not silently converted into workflow tasks.

---

## 8. Alternate and Exception Journeys

### 8.1 No Governed Workspace Context

Where `L4-A-050` cannot resolve a governed Workspace context, L6 shall not manufacture a default unless an existing upstream policy explicitly provides one. The shell may present only the safe minimal authenticated context and supported recovery/navigation options.

### 8.2 Context Revalidation Rejected

- do not show the requested scope as active;
- clear/quarantine data associated with the unconfirmed context;
- show source-supported denial/unavailability;
- return to the last still-valid confirmed context only when it remains valid; otherwise require context/session re-establishment.

### 8.3 Search Empty Result

An empty authorized result shall be presented as legitimate absence. It shall not hint that restricted results exist.

### 8.4 Search Degraded / Stale

Where search architecture reports stale/degraded/unavailable state:

- identify the limitation without exposing internal infrastructure;
- preserve known current shell context;
- do not fabricate canonical data;
- allow authoritative owner resolution only where an explicitly governed owner lookup path exists.

### 8.5 Notification Unavailable

The shell may remain usable when notification projection is independently unavailable. Notification unavailability shall be local to the notification surface unless upstream context/session failure makes the whole shell unsafe.

### 8.6 Emergency/Critical Notification

When H02 supplies an authoritative Emergency/Critical classification, L6 may elevate visual prominence and persistence. L6 shall not create or infer emergency classification itself.

### 8.7 Session Invalidated

When authoritative session context is invalidated:

- protected shell content shall not remain active;
- protected state shall be cleared/quarantined according to G01-003;
- navigation shall transition to the appropriate unauthenticated/session-recovery boundary supplied by the owning architecture;
- no stale protected result may repopulate the shell afterward.

---

## 9. User Actions and Interaction Contract

| Action | Preconditions visible to user | L6 behavior | Authoritative outcome source |
|---|---|---|---|
| Navigate to destination | Destination is presented/discoverable | Move to destination while preserving current confirmed scope and orientation | Owning workspace + D02 enforcement |
| Request context switch | Alternative context is presented | Mark switch as pending, prevent mixed-context presentation, await authoritative revalidation | A04/D01/D02/C01 via G01-003 |
| Search/discover | Authenticated/current context available | Submit discovery request; present only returned authorized projections | H03 / L5-B-01 + owning source on resolution |
| Open search result | Result includes canonical reference/context | Navigate to owning workspace/detail; re-resolve authoritative current state when required | Canonical owning domain |
| Open notification | Notification projection available | Present source-owned communication content/context | H02 / C04 |
| Change optional notification preference | H02 explicitly exposes recipient-control eligibility | Present pending/authoritative result; preserve higher-policy precedence | H02 `L3/L4-H-013` / C04-002 |
| Open account/session area | Session projection available | Present supported account/session actions without exposing secret material | D01 |

No control creates authority by being enabled or visible.

---

## 10. Forms, Input and Validation Experience

### 10.1 Context Selector

- selections are client input, never authoritative scope;
- the currently confirmed context must be visually distinguishable from a pending requested context;
- search/filter inside a long context list is permitted as presentation assistance if source data supports it;
- no synthetic institution/campus/period option may be invented.

### 10.2 Search

- accept only interaction fields supported by the H03 search/discovery contract;
- client input assistance may normalize presentation but shall not alter authorization/search semantics;
- invalid/unsupported criteria should produce understandable local or authoritative feedback as applicable;
- user input shall remain available after recoverable technical failure where safe.

### 10.3 Notification Preferences

Only preference/subscription actions explicitly supported by H02 may be rendered. Mandatory notifications and Emergency/Critical policy overrides shall not be represented as user-controllable when upstream says otherwise.

---

## 11. Data Presentation

### 11.1 Current Context

Context presentation should be compact but persistent enough to avoid accidental cross-institution work. Institution and Academic Period are visually quiet but unmistakable where they materially affect the current workspace.

### 11.2 Search Results

Each result should present, where source data provides it:

- understandable result label;
- entity/workspace type;
- institution/scope disambiguation when needed;
- canonical reference/context sufficient for destination resolution;
- freshness/staleness indication when material and available.

Search ranking is upstream/technical search behavior; L6 shall not hard-code a business ranking rule.

### 11.3 Notifications

Notification presentation should preserve source-provided:

- message/content;
- source/causation context where useful;
- Emergency/Critical or other governed classification where supplied;
- timing/effective context;
- destination/reference where supplied.

Provider-specific delivery internals are not ordinary recipient-facing UI content.

### 11.4 Notifications vs Tasks

The shell shall maintain a visible conceptual distinction:

```text
Notification = communicated information
Task / Work Item = upstream-created action or decision obligation
```

One may link to the other only where upstream architecture provides that relationship.

---

## 12. Interaction States and Feedback

The shell and its shared surfaces shall support the L6 state standard.

| State | Shell/search/notification treatment |
|---|---|
| Initial loading | Preserve product/orientation skeleton; do not show unvalidated protected facts as current |
| Legitimate empty | Explain that no applicable result/notification/context option exists without implying hidden records |
| Pending context switch | Keep requested context visibly pending; quarantine prior protected content |
| Authoritative rejection | Present the authoritative denial/reason where available; do not label as technical failure |
| Technical failure | Keep safe known context; localize failure where independent; allow retry only where safe |
| Stale / invalidated | Remove/quarantine stale protected projection; re-establish current context |
| Partial composition | Keep unaffected shell/workspace regions available and identify the unavailable region |
| Indeterminate mutation | Applies to preference/account mutations if outcome cannot be established; never show success until reconciled |

Search and notification reads must not elevate stale projection age/freshness into canonical business authority.

---

## 13. Authorization and Capability Presentation

- Server-side D02 enforcement remains authoritative.
- Search filtering must fail closed when authorization context is unavailable.
- Navigation/search/notification visibility does not grant a protected action.
- Hidden/disabled controls are usability treatment only.
- A Workspace or persona composition may organize presented work but may not change Role, Permission or Authorization Decision facts.
- A deep link shall re-establish current session/scope/authorization context before protected content/action is trusted.

---

## 14. Accessibility Requirements

The supported shell journey targets the controlled WCAG 2.2 AA direction.

Mandatory L6 behavior:

- skip-to-content mechanism;
- application/navigation/main landmarks with meaningful names;
- logical heading hierarchy and page/workspace title;
- keyboard-operable primary navigation, context selector, search and notification surfaces;
- visible focus and predictable focus order;
- current-page/current-section semantics that do not depend on color;
- accessible naming for icon-only shell controls;
- notification count/status conveyed in text semantics, not color alone;
- context change success/rejection announced where necessary;
- focus restoration when search/context/notification dialog or drawer closes;
- search result groups/list semantics and keyboard traversal;
- reduced-motion behavior for shell transitions;
- reflow/text scaling without loss of current context or primary actions.

A keyboard shortcut for global search may be provided, but no shortcut is mandatory until separately governed. The visible search entry remains discoverable without a shortcut.

---

## 15. Responsive and Density Behavior

### Desktop

- persistent or collapsible primary navigation where width permits;
- current Institution/academic context visible without dominating workspace content;
- search/notification/account entry available in the global shell;
- dense institutional work may use wider workspace content regions.

### Tablet / Narrow Desktop

- primary navigation may collapse to a drawer/rail;
- current context and current location remain visible;
- search and notifications remain directly discoverable.

### Mobile-Supported Tasks

- shell navigation may use a drawer or bottom-navigation adaptation consistent with the controlled foundation;
- context shall not disappear simply to gain space;
- complex specialist work may remain intentionally limited where its own L6 slice says so;
- search and notification details shall reflow rather than require desktop-width tables.

Exact breakpoints are implementation-level/deferred until governed separately.

---

## 16. Design-System Dependencies

Use the controlled L6 design-system foundation for:

- navigation item states;
- context badges/selectors;
- buttons/icon buttons;
- search input/dialog/drawer patterns;
- list/result items;
- notification/status indicators;
- loading/skeleton/empty/unavailable states;
- focus rings and semantic status treatment;
- drawer/dialog surfaces;
- responsive spacing/density.

One-off shell patterns require justification and must not encode business authority.

---

## 17. Audit, Evidence and History Presentation

This shell does not become the enterprise audit owner.

Where upstream behavior requires attributable evidence:

- context-switch or Workspace-resolution evidence remains source-owned;
- search/discovery request/outcome traceability remains H03-owned;
- Notification evidence remains H02/C04/D04-owned;
- L6 may provide a user-facing history/evidence link only when the owning workspace explicitly supports it.

Ordinary shell telemetry shall not be represented as canonical business audit evidence.

---

## 18. Observable UX Completion Conditions

This L6 slice is acceptable when a conforming implementation demonstrates all of the following:

1. authenticated shell does not appear as proof of business authorization;
2. current Institution and applicable academic context are understandable before protected work;
3. context switching never mixes stale old-context data with the requested new context;
4. navigation presentation can vary by resolved responsibility without creating Roles/Permissions;
5. global discovery returns only source-provided, authorization-filtered results and communicates empty/degraded state safely;
6. opening a search result resolves into its owning workspace rather than editing the search projection;
7. notifications remain distinct from workflow tasks;
8. Emergency/Critical visual prominence is driven only by source classification;
9. independently unavailable search/notification content does not unnecessarily destroy otherwise safe shell orientation;
10. keyboard-only users can operate shell navigation, context, search and notifications with correct focus behavior;
11. mobile/narrow layouts preserve current context and user location;
12. no secret/session credential or authoritative permission is persisted/presented as client-owned truth;
13. no technical uncertainty is rendered as authoritative success.

---

## 19. Traceability

### 19.1 Direct L3 / L4 Behavioral Sources

| L3/L4 source | Material behavior used by this L6 slice |
|---|---|
| `L3/L4-A-007` | Authenticated Session becomes available while authentication does not grant business authorization. |
| `L3/L4-A-050` | Workspace is governed experience-context metadata; mapping/selection cannot create permissions; alternative Workspace selection requires revalidation. |
| `L3/L4-B-001` | Institution activation/current institutional context remains B01-owned. |
| `L3/L4-B-027` | Open Academic Context originates in Academic Calendar governance. |
| `L3/L4-B-028` | Calendar assignment/effective governing scope remains upstream-owned. |
| `L3/L4-H-010` | Governed Notification exists as an H02 communication outcome without re-owning source business facts. |
| `L3/L4-H-013` | Optional recipient preference/subscription changes are governed and subject to higher-policy precedence. |
| `L3/L4-H-014` | Emergency/Critical communication classification and permitted overrides remain H02/policy-owned. |
| `L3/L4-H-017` | Authenticated discovery returns only permitted source-derived references; restricted relationships/entities must not leak. |
| `L3/L4-H-018` | Search projection remains a derived representation, not canonical fact. |
| `L3/L4-H-019` | Missing/corrupt/stale search projection recovery is source-derived; L6 must tolerate degraded/unavailable search. |

### 19.2 Indirect / Operational L4 Verification

The following upstream behaviors constrain the connected capability but do not create additional shell actions in this slice:

- `L4-H-011` — Notification scheduling;
- `L4-H-012` — Notification Template publication;
- `L4-H-015` — delivery recovery/failure;
- `L4-H-016` — provider receipt/final notification evidence;
- `L4-H-020` — search-domain projection onboarding;
- `L4-H-021` — search configuration change.

Their administrative workflows are out of scope here.

### 19.3 Exact L5 Technical Sources

| L5 source | Material constraint / evidence |
|---|---|
| `L5-G-01-001` | Shared non-authoritative client-platform invariants. |
| `L5-G-01-002` | Single modular React/TypeScript web application and web runtime boundary; L6 owns final navigation/presentation. |
| `L5-G-01-003` | Client state non-authoritative; Client Context Epoch; protected state partitions; scope invalidation; safe contract consumption; indeterminate outcome handling; deny-by-default protected persistence. |
| `L5-D-01-003` | Authentication Session remains D01-owned; client only consumes session outcome/context. |
| `L5-D-02-001` | Authorization architecture baseline. |
| `L5-D-02-004` | Server-side policy decision/enforcement; UI/navigation never establishes authority. |
| `L5-A-04-001` | Tenant is mandatory protected root; Institution is subordinate; invalid/mismatched scope fails closed. |
| `L5-A-04-002` | Tenant/Institution context propagation; switching/widening requires revalidation; no implicit global/default protected scope. |
| `L5-C-01-001/002/003` | Released synchronous contract/error/compatibility/idempotency semantics consumed through G01-003. |
| `L5-B-01-001` | Search is a read-only disposable projection; results retain canonical references; authorization filtering fails closed; degraded search cannot bypass canonical ownership. |
| `L5-B-01-002` | Search projection freshness/synchronization/access filtering; unavailable policy context fails closed. |
| `L5-C-04-001` | Notification/communication baseline and H02 Notification identity/lifecycle orchestration. Current document metadata: `Review`. |
| `L5-C-04-002` | Template/preference/policy execution; recipient control remains policy-governed. Current document metadata: `Review`. |
| `L5-C-04-003` | Multi-channel/provider/delivery evidence boundary; provider mechanics stay out of L6. Current document metadata: `Review`. |

### 19.4 L5 Dependency Maturity Note

- `L5-G-01-002` and `L5-G-01-003` are Architecture Approved.
- `L5-B-01-001/002` are Locked.
- `L5-D-02-004` is Architecture Approved.
- `L5-A-04-001/002` remain `Review`, but `L5-G-01-003` explicitly records the A04 boundary as stable for G01 consumption and is itself Architecture Approved.
- `L5-C-04-001/002/003` remain `Review`. Their ownership and technical boundaries are usable for controlled L6 derivation, but L6 shall not freeze provider/transport/API details and frontend live integration shall consume only released contracts.

### 19.5 L6 Foundation Dependencies

- `L6_Foundation_Controlled_Authorization_2026-09-16.md`;
- `L6_Design_System_Foundation_DRAFT.md`;
- `L6_Application_Shell_and_Information_Architecture_DRAFT.md`;
- `L6_Interaction_State_and_Feedback_Standard_DRAFT.md`;
- `L6_Persona_Journey_and_Navigation_Map_DRAFT.md`;
- `L6_Screen_Journey_Traceability_Register_DRAFT.md`.

---

## 20. Open Issues / Deferred Decisions

| ID | Item | Treatment |
|---|---|---|
| `L6-SHELL-OPEN-01` | Permanent L6 identifier and production folder taxonomy | Deferred by controlled foundation authorization; descriptive filename remains valid. |
| `L6-SHELL-OPEN-02` | Exact navigation labels/order for every responsibility combination | Use persona journey map as controlled working composition; validate per later workspace slices. |
| `L6-SHELL-OPEN-03` | Mandatory command-palette keyboard shortcut | Not required; visible search entry is mandatory; shortcut may be added as implementation convenience. |
| `L6-SHELL-OPEN-04` | Exact search facets/result types for each canonical domain | Resolve from H03 indexing/search contracts and owning-domain projection participation; do not invent globally. |
| `L6-SHELL-OPEN-05` | Notification read/acknowledgement semantics | Not defined by this slice. No mark-read/acknowledge lifecycle is invented. |
| `L6-SHELL-OPEN-06` | Notification live-integration maturity while C04 specifications remain Review | UI architecture may proceed; connected implementation shall use only released contracts and shall not harden unresolved provider/transport mechanics. |
| `L6-SHELL-OPEN-07` | Exact mobile-supported specialist tasks | Defined by each workspace slice, not globally by the shell. |

---

## Review Checklist Result

Internal architecture review confirms:

- correct L6 responsibility boundary;
- explicit L3/L4/L5 traceability;
- no business-rule, lifecycle, event, authorization or Tenant/Institution ownership transfer;
- no navigation-as-permission assumption;
- search and notification ownership preserved;
- context-switch stale-state behavior aligned with G01-003/A04;
- error/empty/stale/partial states defined;
- accessibility and responsive behavior defined;
- observable frontend acceptance conditions defined;
- unresolved notification technical maturity recorded rather than guessed.

**Draft authoring verdict:** Ready for independent L6 validation. Live notification integration remains constrained to released upstream contracts while C04 lifecycle metadata remains `Review`.
