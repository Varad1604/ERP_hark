# Dashboard Template Request to Effective Governed Template

# 1. Scenario Metadata

**Scenario ID:** L3-H-049  
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
**Referenced By:** Dashboard creation, workspace rollout, dashboard versioning, and template retirement scenarios

# 2. Scenario Purpose

Govern creation and use of reusable H07 dashboard templates while preserving Section A authorization ownership and ensuring a template remains presentation metadata rather than a source of business data or KPI logic.

# 3. Business Outcome

A versioned template is created, approved, published, deprecated, or retired under H07 governance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An institution requires a reusable dashboard arrangement.

## 4.2 End Boundary

The frozen outcome requires H07 to support a governed template through creation, approval, publication, deprecation, or retirement. H07 authoritatively defines the template lifecycle states, but its workflow/event material does not fully define the governed deprecation and retirement path; that material gap is recorded in Section 17.

## 4.3 In Scope

- H07-owned Dashboard Template creation and versioned metadata.
- Validation, approval, publication, deprecation, and retirement lifecycle positions defined by H07.
- Section A authorization constraints applicable to template governance.
- Preservation of source-domain business ownership.

## 4.4 Out of Scope

- Embedding operational business data or KPI logic in templates.
- Inventing a deprecation/retirement actor, workflow, command, event, or transition semantics not established by H07.
- Technical UI, persistence, deployment, or rendering design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization policy, roles, decisions, and access constraints governing template actions. |
| Enterprise Dashboard / H | Owns Dashboard Template, template versions, template lifecycle, and template publication governance. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Dashboard Designer | Creates the reusable template blueprint where authorized. | Initiates / performs |
| Administrator | Performs H07 template administration where authorized. | Performs |
| Template Owner | Initiates governed publication of a validated template. | Initiates publication |
| Approver | Provides H07-defined approval before publication. | Approves |

H07 does not identify a canonical business actor or role for the frozen deprecation and retirement branches.

# 7. Preconditions and Entry Criteria

- The template requirement belongs within H07 dashboard presentation governance.
- The actor is authorized under applicable Section A policy.
- Required widget definitions are registered and valid for the intended reusable arrangement.
- The template contains presentation metadata only.

# 8. Trigger

An institution requires a reusable dashboard arrangement

# 9. Main Business Flow

1. The reusable dashboard arrangement requirement enters H07 template governance.
2. An authorized Dashboard Designer or Administrator creates the H07 Dashboard Template draft.
3. H07 validates template structure, registered widget compatibility, naming, inheritance, and applicable scope.
4. The template progresses through the H07-defined Configured and Validated lifecycle positions.
5. The required H07 approval is completed.
6. H07 publishes the approved template as immutable reusable presentation metadata.
7. The published template becomes available for controlled creation or cloning of dashboards.
8. The scenario cannot authoritatively continue into the frozen Deprecated or Retired outcome because H07 does not fully define the governing business workflow and actor authority for those transitions.

# 10. Alternate Business Flows

H07 permits creation from an existing dashboard or institutional standard arrangement and permits publication to a limited governed scope. Those paths do not resolve the deprecation/retirement gap.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid template structure or widget compatibility | Template cannot progress to publication | Correct the H07-owned template metadata and resubmit for validation | Enterprise Dashboard |
| Required approval is incomplete | Publication cannot occur | Complete the defined H07 approval path | Enterprise Dashboard |
| Deprecated/retired outcome is required | Frozen outcome branch cannot be completed from the available workflow semantics | Resolve upstream H07 lifecycle-workflow/authority gap | Enterprise Dashboard |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Templates contain presentation blueprint metadata only | Enterprise Dashboard | No business data is embedded | L2-H07 §51.10 |
| Published templates are immutable | Enterprise Dashboard | Changes require a successor version rather than in-place mutation | L2-H07 BR-TMP-003, BR-TMP-004 |
| Template lifecycle is Draft → Configured → Validated → Approved → Published → Deprecated → Retired | Enterprise Dashboard | Frozen lifecycle terms must not be renamed | L2-H07 §§12.2, 52.4 |
| Authorization remains controlled by Section A | Platform Foundation | Template actions cannot expand enterprise authorization | L2-A04; L2-H07 §17 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Dashboard Template | Draft | Configure, validate, approve, publish | Published | Enterprise Dashboard | L2-H07 §§12.2, 52.4 |
| Dashboard Template | Published | Frozen semantics also require possible deprecation and retirement | Deprecated / Retired are defined lifecycle positions, but the governing transition workflow/authority is incomplete | Enterprise Dashboard | L2-H07 §§12.2, 52.4 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| DashboardTemplateCreated | Enterprise Dashboard | H07 template governance participants | Records creation of the reusable template | L2-H07 §23.4; §55 |
| DashboardTemplatePublished | Enterprise Dashboard | Authorized H07 dashboard creators | Records governed template publication | L2-H07 §23.4; §51.11 |

H07 §52.4 names `DashboardTemplateRetired` as a transition event, but the canonical template-event catalogue in §23.4 does not define that event and instead defines `DashboardTemplateArchived`. No canonical deprecation event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies authorization decisions and context. H07 owns the reusable dashboard-template metadata. No source-domain business data is transferred into template ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Create template | Dashboard Designer / Administrator | Requires H07 template-creation authority constrained by Section A | Enterprise Dashboard |
| Approve/publish template | Template Owner / Approver / Administrator | Requires completed validation and approval | Enterprise Dashboard |
| Deprecate/retire template | Undefined upstream | H07 state machine requires template governance authority but does not establish the canonical actor/role or complete workflow for the frozen terminal branches | Enterprise Dashboard |

## 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is defined. L3 does not invent deprecation or retirement authority.

## 15.3 Business Audit and Evidence Obligations

Preserve template creation history, validation, approval, version lineage, publication evidence, and any upstream-defined lifecycle transition evidence. No unsupported retirement evidence is invented.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The created/approved/published branch can complete when an immutable H07 template is published. The complete frozen scenario cannot be claimed while the Deprecated and Retired branches lack a fully defined governing business path and unambiguous event semantics.

## 16.2 Postconditions

A supported published template is available for controlled reuse and remains immutable. Unsupported deprecation/retirement semantics are not approximated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H07 defines `Deprecated` and `Retired` as canonical Template lifecycle states, but its workflow catalogue defines Create Template and Publish Template without a distinct governed deprecation or retirement workflow.
- H07 does not establish a canonical business actor/role for the deprecation and retirement branches required by the frozen outcome.
- H07 §52.4 names `DashboardTemplateRetired`, while §23.4 does not define that event and instead lists `DashboardTemplateArchived`; no canonical deprecation event is defined.

These gaps materially affect frozen final-outcome branches and business authority.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H07 architecture must reconcile template deprecation/retirement workflow, authority, and event semantics before this scenario can become source-complete.
