# Regulatory Framework Change to Active Reporting Definition

# 1. Scenario Metadata

**Scenario ID:** L3-H-055  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** Reporting-cycle planning, regulatory validation, snapshot, submission, correction, and archival scenarios

# 2. Scenario Purpose

Govern the creation or revision of an H08 Reporting Framework so that regulatory reporting metadata, validation requirements, evidence requirements, reporting periods, templates, and submission rules remain versioned and governed without moving ownership of canonical source-domain facts into H08.

# 3. Business Outcome

A versioned reporting definition is approved and made effective without changing canonical source-domain facts

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A NEP, NAAC, AISHE, or institutional reporting framework is introduced or revised.

## 4.2 End Boundary

The frozen outcome requires the versioned reporting definition to be approved and made effective. H08 supports framework creation, configuration, validation-state progression, publication, retirement, and a lifecycle containing `Active`, but does not define the business approval or activation action that moves a published framework to `Active`. The full frozen end boundary therefore cannot be completed authoritatively.

## 4.3 In Scope

- H08 Reporting Framework registration and revision.
- Versioned reporting metadata, reporting-period, template, validation, evidence, and submission-rule definition.
- Preservation of framework version history.
- Publication of a governed framework version where H08 requirements are satisfied.
- Preservation of source-domain ownership for every business fact referenced by reporting definitions.

## 4.4 Out of Scope

- Creation or modification of Student, Academic Delivery, Workforce, Finance, Examination, Research, Campus Operations, or institutional source facts.
- Inventing a framework approval authority, approval event, activation action, or activation event.
- Treating `Published` as identical to `Active` without upstream authority.
- Implementation mechanics or user-interface design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| NEP, NAAC & AISHE Reporting / H | Owns Reporting Framework metadata, framework version history, reporting configuration, validation/evidence requirements, and the H08 framework lifecycle. |

Operational domains remain canonical owners of the facts later consumed by reporting. A framework definition describes reporting requirements but does not transfer ownership of those facts to H08.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Reporting Administrator | Configures and governs Reporting Framework metadata where authorized. | Initiates / performs source-defined framework management |

L2-H08 explicitly records that its required Actors section is absent. It lists Reporting Administrator as a typical enterprise role, but does not define a canonical approver or activation authority for the frozen approved-and-effective outcome.

# 7. Preconditions and Entry Criteria

- The new or revised regulatory requirement is available for configuration.
- Required reporting-template context is available as required by H08 framework management.
- The proposed framework version can define reporting periods, validation rules, evidence requirements, and submission rules without owning source-domain facts.
- The framework action is governed under H08 reporting authority.

# 8. Trigger

A NEP, NAAC, AISHE, or institutional reporting framework is introduced or revised

# 9. Main Business Flow

1. H08 receives the new or revised regulatory framework requirement.
2. The Reporting Administrator establishes or revises the H08 Reporting Framework metadata while preserving the prior framework version history.
3. H08 configures the reporting periods, applicable report templates, validation requirements, evidence requirements, and governed submission rules supported by the regulatory definition.
4. H08 verifies that the reporting definition references canonical source-domain information without taking ownership of that information.
5. The framework progresses through the source-defined `Draft`, `Configured`, and `Validated` lifecycle positions as its H08 metadata becomes complete and valid.
6. H08 publishes the governed framework version and records the source-defined publication event.
7. Stop before asserting institutional approval or the `Published → Active` transition because H08 does not define the required business decision, authority, action, or event for that part of the frozen outcome.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Existing framework revised | Regulatory requirements change while historical cycles exist | Create and govern a successor framework version while historical Reporting Cycles remain bound to the version originally used | Historical reporting remains reproducible | NEP, NAAC & AISHE Reporting |
| Framework retirement | A published/active framework is no longer to govern new cycles | Use the H08-defined retirement action while preserving historical cycle references | Reporting Framework reaches the source-defined retired outcome | NEP, NAAC & AISHE Reporting |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Framework metadata or required reporting configuration is incomplete | Framework cannot progress to governed publication | Correct the H08-owned framework definition before a later publication attempt | NEP, NAAC & AISHE Reporting |
| Proposed definition would duplicate or change canonical source facts | Ownership boundary would be violated | Remove the duplicated business fact and retain only the H08 reporting definition/reference | NEP, NAAC & AISHE Reporting |
| Approved/effective outcome is required | Source architecture does not define the approval/activation path | Escalate the lifecycle/authority gap for upstream resolution | NEP, NAAC & AISHE Reporting |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Reporting metadata belongs exclusively to H08 | NEP, NAAC & AISHE Reporting | The framework may define reporting requirements but may not re-own operational facts | L2-H08 §§5.2, 5.7 |
| Framework versions are immutable after publication | NEP, NAAC & AISHE Reporting | Revisions require versioned successor governance rather than rewriting historical versions | L2-H08 §6.5 |
| Framework retirement must not invalidate historical Reporting Cycles | NEP, NAAC & AISHE Reporting | Historical cycle/version references remain valid | L2-H08 §6.5; §23.1 |
| Every Reporting Cycle references one published Reporting Framework | NEP, NAAC & AISHE Reporting | A framework must reach the supported published position before new cycles depend on it | L2-H08 BR-001 |
| Operational ownership does not migrate into H08 | Source-owning domains | Framework metadata cannot alter the source system of record | L2-H08 O-002; ADR-003; `DEPENDENCY_MAP.md` |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reporting Framework | Draft / Configured / Validated | Configure and publish a governed framework version | Published | H08 | L2-H08 §§6.5, 7 H08-W01 |
| Reporting Framework | Published | Frozen outcome requires it to become effective | `Active` exists in the lifecycle, but the governing transition/authority is not defined | H08 | L2-H08 §6.5 |
| Reporting Framework | Active / Deprecated | Supported retirement may occur when appropriate | Retired | H08 | L2-H08 §6.5; H08-W01 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ReportingFrameworkCreated | H08 | Authorized reporting consumers | Records framework creation | L2-H08 §10.2 |
| ReportingFrameworkPublished | H08 | Reporting-cycle governance | Records governed framework publication | L2-H08 §10.2 |
| ReportingFrameworkRetired | H08 | Reporting governance consumers | Records framework retirement | L2-H08 §10.2 |

No canonical Reporting Framework approval or activation event is defined by H08.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The Reporting Framework may identify approved reporting contracts and source categories required by future Reporting Cycles. Those contracts remain references to source-owned facts; no business ownership is transferred to H08 by the framework definition.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Reporting Administrator is source-supported for Reporting Framework configuration. H08 does not establish the canonical business approver or activation authority needed to satisfy the frozen approved-and-effective outcome.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific approval hierarchy or separation-of-duty model for Reporting Framework activation is defined. L3 does not invent one.

## 15.3 Business Audit and Evidence Obligations

Preserve framework creation/revision history, configuration changes, version lineage, publication evidence, retirement evidence where applicable, and references to the regulatory requirement that caused the change.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported branch can complete through governed publication of a versioned H08 Reporting Framework. The full frozen scenario cannot be completed until the approval and `Active`-state transition are defined upstream.

## 16.2 Postconditions

A source-supported published framework version may be referenced by future Reporting Cycles; historical versions remain preserved; canonical source-domain facts remain unchanged; no unsupported active-state claim is made.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen semantics require the reporting definition to be **approved and made effective**.
- H08 defines the Reporting Framework lifecycle as `Draft → Configured → Validated → Published → Active → Deprecated → Retired`.
- H08-W01 defines create/configure/publish/retire actions and events but no framework approval action, approval authority, approval event, activation action, or activation event.
- Therefore L3 cannot determine how `Published → Active` occurs or who authoritatively approves that transition.

This materially affects the frozen final outcome, lifecycle, and decision authority.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H08 architecture must define the Reporting Framework approval and activation semantics, including authority and canonical event treatment, before this scenario can become source-complete.
