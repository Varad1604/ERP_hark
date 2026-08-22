# Widget Definition to Governed Availability or Retirement

# 1. Scenario Metadata

**Scenario ID:** L3-H-054  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Dashboard  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A04 — Authorization & Policy Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Dashboard composition, dashboard publication, widget configuration, and widget governance scenarios

# 2. Scenario Purpose

Govern an H07 Widget Definition from a canonical projection need toward controlled dashboard availability and lifecycle governance while ensuring the widget remains a read-only presentation definition and never absorbs source-domain business logic or ownership.

# 3. Business Outcome

The widget definition is validated, approved, made available, versioned, suspended, or retired without embedding source-domain business logic

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A canonical projection requires a new or changed dashboard widget.

## 4.2 End Boundary

The supported H07 branches end when a widget definition has been governed into the catalog and made available, changed through a governed successor definition/version, or disabled from further selection. The frozen `suspended` and `retired` outcomes are not fully defined as canonical Widget Definition workflow/state/event transitions in locked H07.

## 4.3 In Scope

- H07 ownership of Widget Catalog metadata and Widget Definition governance.
- Validation, approval, registration, governed availability, change/version governance, deprecation constraints, and withdrawal from availability where H07 defines them.
- Section A authorization constraints on widget governance actions.
- Preservation of the source domain's canonical projection and business logic ownership.

## 4.4 Out of Scope

- Embedding source-domain business rules, commands, transactions, KPI computation, or operational state in the widget.
- Treating Widget Configuration lifecycle states as Widget Definition lifecycle states.
- Renaming H07 `Disabled` behavior to the frozen `Suspended` outcome without upstream authority.
- Inventing a Widget Definition retirement workflow, retirement authority, or retirement event.
- Technical rendering, packaging, code, deployment, or UI implementation.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization policies and decisions that constrain widget governance actions. |
| Enterprise Dashboard / H | Owns Widget Catalog metadata, Widget Configuration, widget governance, availability metadata, and dashboard composition references. The source domain retains ownership of the canonical projection and its business meaning. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Widget Publisher | Submits a widget definition for governed registration where authorized. | Initiates |
| Platform Administrator | Participates in H07 widget registration governance where authorized. | Performs / governs registration |
| Administrator | Enables or disables a registered widget where authorized. | Performs availability governance |
| Dashboard Owner | May participate in H07 widget enablement within permitted dashboard scope. | Participates |
| Governance Reviewer | Participates in H07 widget disablement/withdrawal governance where defined. | Reviews / performs |

H07 does not establish a distinct canonical actor/role for a Widget Definition retirement transition.

# 7. Preconditions and Entry Criteria

- A canonical projection or authoritative read source exists outside H07 and remains owned by its source domain.
- The proposed widget definition is limited to presentation metadata and approved read consumption.
- The actor is authorized under Section A and applicable H07 governance.
- Required source references and compatibility conditions can be validated without transferring ownership into H07.

# 8. Trigger

A canonical projection requires a new or changed dashboard widget

# 9. Main Business Flow

1. Receive the requirement for a new or changed dashboard widget against a canonical projection.
2. Preserve the source domain's ownership of the projection, business facts, KPI definitions, and business logic.
3. An authorized Widget Publisher submits the H07 Widget Definition for governance.
4. H07 validates the definition's identity, intended read source, compatibility, security classification, and declared governance metadata.
5. H07 registers the accepted widget definition in the governed Widget Catalog and records the registration outcome.
6. After applicable approval and availability checks, H07 enables the registered widget for the permitted dashboard scope and records the availability outcome.
7. Where the definition changes, H07 governs the changed definition/version without mutating historical dashboard references or transferring source-domain ownership.
8. Where further use must stop, H07 may disable availability while preserving historical references.
9. Stop before asserting the frozen `suspended` or `retired` Widget Definition outcome because H07 does not define a complete canonical business lifecycle, authority, and event path for those terms.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Scoped availability | Widget is valid only for a governed subset of dashboard contexts | H07 limits availability to the approved scope | Widget available only within governed scope | Enterprise Dashboard |
| New widget version | A changed definition requires successor governance | Register and govern the successor definition/version while preserving historical references | New governed version available if approved | Enterprise Dashboard |
| Controlled disablement | Widget must not be selectable for new compositions | H07 disables availability without deleting historical references | Widget unavailable for new governed use | Enterprise Dashboard |
| Retirement requested | The widget must reach the frozen terminal outcome | Do not infer retirement from disablement or deprecation | Upstream H07 resolution required | Enterprise Dashboard |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source reference, compatibility, or governance validation fails | Widget cannot become available | Reject or return the H07 definition for correction without altering the source projection | Enterprise Dashboard |
| Authorization does not permit registration or availability change | Widget governance action cannot proceed | Deny the action under Section A policy | Platform Foundation / A |
| Existing dashboards depend on a widget being withdrawn | Immediate removal may affect governed compositions | Apply the H07-defined disablement/fallback governance while retaining historical metadata | Enterprise Dashboard |
| Suspended or retired outcome is required | Frozen lifecycle branch is not fully defined | Escalate for upstream H07 lifecycle/authority/event resolution | Enterprise Dashboard |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Widgets are read-only presentation components and do not own operational state | Enterprise Dashboard | Source-domain business ownership remains external to H07 | L2-H07 §§1.8, 51.14 |
| Widgets consume approved read information and do not execute business-changing actions | Enterprise Dashboard / source-owning domain boundary | Widget Definition cannot embed source business execution | L2-H07 §§16.5, 51.1 |
| Disabled widgets do not erase historical dashboard references | Enterprise Dashboard | Withdrawal from new use preserves history | L2-H07 §51.17 |
| Deprecated or retired catalog entries are blocked from new publication | Enterprise Dashboard | H07 recognizes lifecycle governance beyond availability, but does not define the complete retirement transition | L2-H07 §53.6 |
| Authorization remains controlled by Section A | Platform Foundation | H07 widget governance cannot expand enterprise authorization | L2-A04 AP-01, AP-05 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Widget Definition / Widget Catalog entry | Proposed or not yet registered | Governed registration and availability | Registered and available/enabled where H07 governance succeeds | Enterprise Dashboard | L2-H07 §§51.14, 51.16, 54 |
| Widget Definition / Widget Catalog entry | Existing governed definition | Governed successor/version or withdrawal from new use | Successor version or disabled availability, with history preserved | Enterprise Dashboard | L2-H07 §§51.14, 51.17, 53.8 |
| Widget Catalog status | Supported / Deprecated / Retired vocabulary exists | Frozen retirement branch requires a governed transition | `Retired` exists as catalog status but no complete retirement workflow/event/actor path is defined | Enterprise Dashboard | L2-H07 §54.2 |
| Widget Configuration | Registered → Configured → Validated → Approved → Available → Updated → Deprecated | Separate configuration lifecycle; must not be used as the Widget Definition lifecycle | Remains a distinct H07 aggregate lifecycle | Enterprise Dashboard | L2-H07 §§12.4, 52.5 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| WidgetRegistered | Enterprise Dashboard | H07 dashboard governance consumers | Records governed widget registration | L2-H07 §23.5; §51.14 |
| WidgetEnabled | Enterprise Dashboard | H07 dashboard governance consumers | Records governed widget availability | L2-H07 §23.5; §51.16 |
| WidgetDisabled | Enterprise Dashboard | H07 dashboard governance consumers | Records governed withdrawal from availability | L2-H07 §23.5; §51.17 |

No canonical `WidgetSuspended` or `WidgetRetired` event exists in the H07 event catalogue. H07 references `WidgetDeprecated` in some workflow material but does not define it in the canonical Widget event catalogue; L3 does not invent or normalize that event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source-owning domain supplies the canonical projection/read information and retains its business meaning, rules, and fact ownership. H07 stores and governs only widget/dashboard presentation metadata. Section A supplies the authoritative authorization decision and constraints.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register Widget Definition | Widget Publisher / Platform Administrator as defined by H07 | Requires successful H07 governance validation and Section A authorization | Enterprise Dashboard |
| Enable widget availability | Administrator / Dashboard Owner as defined by H07 | Only a governed registered widget may become available within approved scope | Enterprise Dashboard |
| Disable widget availability | Administrator / Governance Reviewer as defined by H07 | Withdrawal must preserve historical references and applicable governance evidence | Enterprise Dashboard |
| Suspend or retire Widget Definition | Undefined as a complete canonical path | H07 does not define the full frozen transition/authority/event semantics | Enterprise Dashboard |

## 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is introduced. H07 governance and Section A authorization remain authoritative; source-domain ownership cannot be overridden by Widget Definition governance.

## 15.3 Business Audit and Evidence Obligations

Preserve the widget definition identity, accountable ownership reference, source projection reference, validation outcome, approval/governance evidence, availability changes, version history, disablement reason, affected historical references, and authorization outcome.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported registration/availability branch completes when the accepted Widget Definition is governed into the H07 catalog and made available within approved scope. A changed definition can proceed through governed successor/version handling, and a widget can be disabled from new use. The complete frozen scenario cannot be source-complete until `suspended` and `retired` Widget Definition semantics are authoritatively defined.

## 16.2 Postconditions

The source projection remains canonically owned outside H07; H07 retains only governed presentation metadata and history; no unsupported suspension or retirement outcome is fabricated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen semantics require Widget Definition outcomes including **suspended** and **retired**.
- H07 defines enable/disable workflows and events, but does not define a canonical `Suspended` Widget Definition state or `WidgetSuspended` event. L3 cannot silently rename `Disabled` to `Suspended`.
- H07's Widget Catalog recognizes `Supported`, `Deprecated`, and `Retired` status values and blocks deprecated/retired widgets from new publication, but it provides no complete Widget Definition retirement workflow, canonical retirement actor, or `WidgetRetired` event.
- H07's explicit state machine is for **Widget Configuration**, not Widget Definition; its terminal state is `Deprecated`, so that lifecycle cannot be reused to fill the Widget Definition retirement gap.
- H07 references `WidgetDeprecated` outside the canonical Widget event catalogue, but §23.5 defines only `WidgetRegistered`, `WidgetConfigurationUpdated`, `WidgetEnabled`, and `WidgetDisabled`.

These issues materially affect the frozen final outcome, lifecycle, authority, and event model.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H07 architecture must define or reconcile the canonical Widget Definition lifecycle for suspension, deprecation, retirement, governing authority, and canonical events before this scenario can be completed.
