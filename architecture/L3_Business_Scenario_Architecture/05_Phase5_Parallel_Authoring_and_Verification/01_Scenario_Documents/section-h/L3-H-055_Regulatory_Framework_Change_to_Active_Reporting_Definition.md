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
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Govern introduction or revision of an H08 Reporting Framework so future regulatory reporting uses a versioned, effective reporting definition without changing canonical source-domain facts.

# 3. Business Outcome

A versioned reporting definition is approved and made effective without changing canonical source-domain facts

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a NEP, NAAC, AISHE, or institutional reporting framework is introduced or revised.

## 4.2 End Boundary

The frozen end boundary requires a versioned reporting definition to be both approved and made effective. Locked H08 defines publication and activation of a Reporting Framework but does not define a framework approval state, approval transition, or approval event.

## 4.3 In Scope

- Establish or revise H08-owned Reporting Framework metadata.
- Validate the framework under H08 governance.
- Version and publish the governed framework definition.
- Make a published framework version active where the H08 lifecycle permits.
- Preserve historical framework-version lineage.

## 4.4 Out of Scope

- Changing operational facts owned by source domains.
- Creating a local framework approval state or approval authority absent from H08.
- Treating publication as approval without an upstream decision.
- Redefining regulator-owned requirements outside the governed H08 reporting definition.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| NEP, NAAC & AISHE Reporting / H08 | Owns Reporting Framework definition, metadata, versioning, lifecycle, reporting configuration, validation policy, evidence requirements, and submission policy used for regulatory reporting. |
| Source-owning domains | Retain ownership of operational business facts subsequently consumed by reporting cycles; H08 does not alter those facts through framework governance. |

# 6. Business Actors, Roles and Responsibilities

Locked H08 does not provide a canonical business-actor section. H08-W01 defines framework-management activity and requires governed authority, but it does not define the business approval authority required by the frozen outcome.

# 7. Preconditions and Entry Criteria

- A new or revised reporting framework requirement exists.
- Applicable framework metadata and reporting requirements are available for governance.
- Historical framework versions remain preserved.
- No source-domain fact is transferred into H08 ownership.

# 8. Trigger

A NEP, NAAC, AISHE, or institutional reporting framework is introduced or revised

# 9. Main Business Flow

1. H08 establishes or revises the Reporting Framework definition in `Draft`.
2. H08 configures the framework metadata, applicable reporting periods, validation policies, evidence requirements, templates, and submission policy.
3. The framework progresses through `Configured` and `Validated` under the H08 lifecycle.
4. H08 publishes the validated framework version as an immutable published version.
5. The published version may progress to `Active` under the H08 lifecycle and become available to future Reporting Cycles.

The flow cannot establish the frozen approval outcome because H08 defines no Reporting Framework approval state or approval decision path.

# 10. Alternate Business Flows

- An active framework may later progress to `Deprecated` and then `Retired` under H08 governance.
- A revised framework is represented by version evolution; historical Reporting Cycles remain bound to their original framework version.
- If validation does not succeed, publication does not proceed.

# 11. Failure, Interruption and Business Recovery

If required framework information or validation conditions are incomplete, the reporting definition remains at its last valid H08 lifecycle position until the governing business condition is resolved. L3 does not substitute publication for the missing approval decision.

# 12. Business Rules and Decision Constraints

- BR-001 requires every Reporting Cycle to reference one published Reporting Framework.
- Framework versions are immutable after publication.
- Framework retirement does not invalidate historical Reporting Cycles.
- Operational business data remains owned by its canonical source domain.
- Historical reporting must remain reproducible against the framework version originally used.

# 13. State and Lifecycle Impact

The H08 Reporting Framework lifecycle is `Draft → Configured → Validated → Published → Active → Deprecated → Retired`. No `Approved` state or approval transition is defined for Reporting Framework.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H08 defines `ReportingFrameworkCreated`, `ReportingFrameworkPublished`, and `ReportingFrameworkRetired`. No canonical Reporting Framework approval event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The active framework governs how later H08 reporting cycles consume authoritative source-owned information through approved reporting contracts. This scenario does not transfer source-data ownership to H08.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H08 governs Reporting Framework management. The specific business approval authority required to satisfy the frozen outcome is not defined in the locked H08 business-actor or workflow architecture.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 participant may treat framework publication as equivalent to business approval or bypass the H08 lifecycle. Framework governance cannot override source-domain ownership.

## 15.3 Business Audit and Evidence Obligations

Preserve framework creation/change history, validation evidence, version lineage, publication history, activation/deprecation/retirement history, and the governing requirements used for the definition.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported H08 lifecycle can produce a versioned, published, and active Reporting Framework. The frozen requirement for an approved reporting definition cannot be verified from the locked source.

## 16.2 Postconditions

No invented approval state, event, or authority is introduced. Source-domain facts remain outside H08 ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**Material lifecycle and authority gap:** frozen H055 requires a versioned reporting definition to be **approved and made effective**. Locked H08 defines `Draft → Configured → Validated → Published → Active → Deprecated → Retired` and H08-W01 defines create/configure/publish/retire actions, but no Reporting Framework approval state, approval transition, approval event, or business approval authority.

**Architecture impact:** L3 cannot equate `Published` with `Approved` or invent a framework approval decision, so the frozen final outcome cannot be completed safely.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Architecture change control must define whether Reporting Framework approval is a distinct H08 business decision and, if so, its authority, lifecycle position, and canonical event.
