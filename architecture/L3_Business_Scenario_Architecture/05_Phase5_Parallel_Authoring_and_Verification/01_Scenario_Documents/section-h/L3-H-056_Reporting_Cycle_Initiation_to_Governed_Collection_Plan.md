# Reporting Cycle Initiation to Governed Collection Plan

# 1. Scenario Metadata

**Scenario ID:** L3-H-056  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Establish the governed plan for a statutory or accreditation reporting period while preserving the authority of each evidence-owning domain over the facts that the cycle will later consume.

# 3. Business Outcome

Scope, responsibilities, deadlines, source contracts, validations, and approvals are established in an active reporting cycle

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a statutory or accreditation reporting period opens.

## 4.2 End Boundary

The frozen end boundary requires an **active reporting cycle** with scope, responsibilities, deadlines, source contracts, validations, and approvals established. H08 defines a `Planned` Reporting Cycle and later `DataCollection`, but no canonical `Active` Reporting Cycle state.

## 4.3 In Scope

- Create the H08 Reporting Cycle for one published Reporting Framework.
- Associate the applicable published Report Template.
- Establish the reporting timeline and deadline context.
- Identify responsible reporting participation and the applicable authoritative evidence-owning domains.
- Preserve the approved reporting contracts and validation/approval requirements inherited from governed H08 framework/template configuration.
- Publish the governed reporting schedule.

## 4.4 Out of Scope

- Collecting source facts or creating the Reporting Snapshot.
- Changing facts in `VAR:Evidence-Owning Domain`.
- Inventing an `Active` lifecycle state or local activation transition.
- Selecting one concrete evidence-owning domain where the frozen canonical scenario intentionally preserves a variable participant.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| NEP, NAAC & AISHE Reporting / H08 | Owns Reporting Cycle, Reporting Schedule, reporting lifecycle, reporting metadata, validation governance, and reporting approval state. |
| VAR:Evidence-Owning Domain | Retains ownership of the authoritative facts/evidence later requested by the cycle and participates only through approved reporting contracts. |

# 6. Business Actors, Roles and Responsibilities

H08-W02 requires assignment of responsible officers but locked H08 does not provide a canonical business-actor section defining those responsibilities or the business authority that assigns them. The variable evidence-owning participant remains source-owner-specific at execution time.

# 7. Preconditions and Entry Criteria

- The applicable Reporting Framework is published.
- A reporting schedule and reporting period are defined.
- The applicable Report Template is published as required by BR-002.
- Relevant reporting contracts and validation/approval requirements are governed upstream in H08 configuration.

# 8. Trigger

A statutory or accreditation reporting period opens

# 9. Main Business Flow

1. H08 creates a Reporting Cycle for the applicable published Reporting Framework.
2. H08 associates the governed Report Template and reporting-period scope.
3. H08 establishes the reporting timeline and deadlines through the Reporting Schedule owned by the Reporting Cycle.
4. H08 identifies the required source participation and preserves each `VAR:Evidence-Owning Domain` as the owner of its own authoritative facts.
5. H08 associates the applicable governed reporting contracts, validation requirements, evidence requirements, and approval obligations for the cycle.
6. H08 assigns the reporting responsibilities described by H08-W02 and publishes the Reporting Schedule.

The locked lifecycle only establishes `Planned` at this point; L3 cannot assert a canonical `Active` state for the frozen end outcome.

# 10. Alternate Business Flows

- A cycle cannot proceed to collection if the required published framework or template is unavailable.
- Different cycles may involve different evidence-owning domains while preserving the same frozen `VAR:Evidence-Owning Domain` architecture boundary.
- Reporting schedule changes remain H08-owned and do not alter source-domain ownership.

# 11. Failure, Interruption and Business Recovery

If cycle scope, schedule, required reporting contracts, or required governance conditions are incomplete, the cycle remains at its last valid H08 lifecycle position. Missing source participation is resolved by the authoritative source owner; H08 does not create substitute source facts.

# 12. Business Rules and Decision Constraints

- BR-001 requires one published Reporting Framework per Reporting Cycle.
- BR-002 requires one published Report Template per Reporting Cycle.
- BR-003 requires reporting data to originate from canonical operational domains.
- H08 may orchestrate source participation but must not modify operational business data.
- Reporting Cycle owns its Reporting Schedule.

# 13. State and Lifecycle Impact

The H08 Reporting Cycle lifecycle is `Planned → DataCollection → Validation → SnapshotCreated → ApprovalPending → Approved/Rejected → SubmissionReady → Submitted → Accepted/RejectedByAuthority → Closed → Archived`, with correction paths returning to `DataCollection`. No canonical `Active` state exists despite H08 workflow preconditions sometimes describing a cycle as active.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H08 defines `ReportingCycleCreated`, `ReportingCycleScheduled`, and `ReportingCycleStarted` for cycle initiation/planning. The authoritative evidence-owning domains retain ownership of their own events and facts; no variable-domain event is invented here.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H08 defines approved reporting contracts for consuming authoritative source-domain information. Because the frozen participant is `VAR:Evidence-Owning Domain`, the specific upstream contract owner is resolved only when the applicable reporting definition/cycle identifies the actual source domain; L3 does not replace the variable with an invented domain.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H08 owns cycle creation and planning. Locked H08 does not canonically define the business actor that assigns responsible officers or the authority that declares the cycle `Active`, because no such lifecycle state is defined.

## 15.2 Separation of Duties, Overrides and Exceptions

Planning responsibilities do not transfer ownership of evidence or source facts to H08. No reporting participant may override a source-domain business rule through cycle configuration.

## 15.3 Business Audit and Evidence Obligations

Preserve cycle creation, framework/template version references, scope, reporting schedule, responsibility assignments, applicable source-contract references, validation/approval requirements, and planning changes.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The governed planning information required for collection is established and the H08 Reporting Cycle reaches the authoritative planning position. The frozen requirement that the cycle be in an `Active` state cannot be verified from the locked lifecycle.

## 16.2 Postconditions

The Reporting Cycle, schedule, and planning metadata remain H08-owned; source facts remain owned by their applicable evidence-owning domains. No local `Active` state is created.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**Material lifecycle gap:** frozen H056 requires the plan to be established in an **active reporting cycle**. Locked H08 defines `Planned` followed by `DataCollection` and never defines an `Active` Reporting Cycle state, although later workflow preconditions use the adjective “active.”

**Material actor/authority gap:** H08-W02 requires assignment of responsible officers, while the H08 canonical crosswalk states that a business Actors section is not present. The business authority for assigning those responsibilities is therefore not canonically defined.

**Architecture impact:** L3 cannot determine the authoritative lifecycle position that satisfies the frozen final outcome or invent the missing assignment authority.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions. `VAR:Evidence-Owning Domain` is preserved exactly as frozen.

## 17.3 Deferred Questions

Architecture change control must define the canonical meaning of an “active” Reporting Cycle and the business authority for assigning reporting-cycle responsibilities.
