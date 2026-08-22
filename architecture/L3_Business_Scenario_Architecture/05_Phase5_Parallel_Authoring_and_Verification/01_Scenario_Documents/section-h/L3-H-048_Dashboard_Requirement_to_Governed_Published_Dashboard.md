# Dashboard Requirement to Governed Published Dashboard

# 1. Scenario Metadata

**Scenario ID:** L3-H-048  
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
**Referenced By:** Dashboard template, workspace assignment, sharing, versioning, retirement, and widget scenarios

# 2. Scenario Purpose

Establish a governed Enterprise Dashboard composition for an authorized institutional requirement while preserving Section A authorization ownership and preserving every source domain's ownership of business facts and KPI definitions.

# 3. Business Outcome

H07 creates, approves, publishes, or rejects the dashboard composition without owning its source facts or KPI definitions

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized institutional role requires a dashboard composed from canonical information.

## 4.2 End Boundary

The scenario ends when H07 has produced a governed published dashboard composition or the requested composition has been rejected by the H07 validation/publication path.

## 4.3 In Scope

- Creation and configuration of H07-owned Dashboard composition metadata.
- Validation, review, approval, publication, or rejection of the dashboard composition.
- Use of canonical authorization context from Section A.
- Use of source-owned read information and KPI definitions without transferring ownership to H07.

## 4.4 Out of Scope

- Creation, modification, or recomputation of source-domain business facts or KPIs.
- Technical rendering, APIs, persistence, caching, deployment, or user-interface design.
- Redefinition of Section A authorization policy.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization roles, assignments, policies, decisions, and access constraints used to govern dashboard actions. |
| Enterprise Dashboard / H | Owns Dashboard composition, layout, widget configuration references, Dashboard Version, publication metadata, and dashboard governance. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Dashboard Administrator | Creates and administers the H07 dashboard composition where authorized. | Initiates / performs |
| Workspace Owner | Participates in creation where the dashboard is established for a governed workspace. | Initiates / performs |
| Governance Reviewer | Reviews the governed dashboard composition where H07 requires review. | Reviews |
| Dashboard Owner | Owns the dashboard publication request within H07 governance. | Initiates publication |
| Approver | Provides the H07-defined approval required before publication. | Approves |
| Administrator | Performs H07 publication administration where authorized. | Performs |

# 7. Preconditions and Entry Criteria

- The governing H07 workspace context exists.
- The requester is authorized under Section A policy for the intended dashboard action and scope.
- Required registered widgets and authoritative read contracts are available for the intended composition.
- H07 does not take ownership of the underlying business facts or KPI definitions.

# 8. Trigger

An authorized institutional role requires a dashboard composed from canonical information

# 9. Main Business Flow

1. The authorized requirement is accepted into H07 dashboard governance.
2. H07 establishes a Dashboard draft in the applicable workspace and defines only presentation/composition metadata.
3. The composition references registered widgets and authoritative source-domain read information without copying business ownership into H07.
4. H07 validates structure, widget compatibility, read-contract compatibility, authorization scope, and publication eligibility.
5. The governed composition is reviewed and receives the required H07 approval.
6. H07 publishes the validated Dashboard version as an immutable governed publication.
7. H07 makes the published dashboard available only within its authorized scope and stops at the published composition outcome.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Rejection | The composition fails validation, lacks required approval, or has an incompatible widget/read contract | H07 rejects progression to publication and preserves the existing governed state for correction or resubmission | Dashboard composition is rejected rather than published | Enterprise Dashboard |
| Staged publication | H07 governance permits publication to a limited approved scope | The validated composition is published to that governed scope | Published dashboard within the approved scope | Enterprise Dashboard |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid composition or dependency | Publication cannot proceed | Correct the H07-owned composition or dependency reference and resubmit through the governed path | Enterprise Dashboard |
| Missing required approval | Publication cannot proceed | Obtain the required H07 approval before publication | Enterprise Dashboard |
| Authorization does not permit the requested action or scope | Dashboard action is not permitted | The action is denied under Section A authorization policy | Platform Foundation / A |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H07 owns dashboard composition only and does not own business data or KPI computation | Enterprise Dashboard | Source facts and KPI definitions remain with their canonical owners | L2-H07 §§1, 2, 6 |
| Only published dashboards may be assigned to users | Enterprise Dashboard | Assignment cannot precede governed publication | L2-H07 BR-DASH-004 |
| Dashboard publication creates an immutable published version | Enterprise Dashboard | Published content is not edited in place | L2-H07 BR-DASH-010; BR-VER-001 |
| Authorization remains governed by Section A | Platform Foundation | H07 visibility and publication actions cannot expand authorization | L2-A04; L2-H07 §17 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Dashboard | Draft / Under Configuration / Validation / Ready for Review / Approved | Governed creation, validation, review, approval and publication | Published; may become Active under H07 lifecycle governance | Enterprise Dashboard | L2-H07 §§12.1, 52.2 |
| Dashboard Version | Candidate or Created as described upstream | Validation and publication of the version | Published / Locked position as defined by the applicable H07 version path | Enterprise Dashboard | L2-H07 §§12.6, 52.7 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| DashboardCreated | Enterprise Dashboard | H07 governance participants | Records creation of the Dashboard aggregate | L2-H07 §23.2; §55 |
| DashboardPublished | Enterprise Dashboard | Authorized dashboard consumers | Records successful governed publication | L2-H07 §23.2; §51.7 |

H07 contains an internal naming inconsistency for the version-publication event; this scenario does not rename or select between the disputed names and records the issue in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A provides authoritative authorization context and decisions. Source-owning domains provide only their approved read information or projections to H07. H07 composes those sources into a dashboard without modifying or re-owning them.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Create/configure dashboard | Dashboard Administrator / Workspace Owner as defined by H07 | Must satisfy applicable Section A authorization and H07 workspace scope | Enterprise Dashboard |
| Review composition | Governance Reviewer | Reviews H07 composition against governed requirements | Enterprise Dashboard |
| Approve/publish | Approver / Dashboard Owner / Administrator as defined by H07 | Publication requires completed validation and required approval | Enterprise Dashboard |

## 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is defined beyond H07's required review/approval path and Section A authorization governance.

## 15.3 Business Audit and Evidence Obligations

Preserve the dashboard creation and configuration history, validation result, approval evidence, publication lineage, scope, and H07 audit history without duplicating source-domain business evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A governed H07 Dashboard composition has either been published after validation and approval or rejected by the defined governance path.
- H07 has not assumed ownership of source facts, KPI definitions, or Section A authorization policy.
- Any published version remains immutable.

## 16.2 Postconditions

- A published dashboard is available only within authorized scope, or the rejected composition remains unpublished.
- Source-domain facts and KPI definitions remain authoritative outside H07.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- L2-H07 §51.7 and workflow/state mapping use `DashboardVersionLocked`, while the §23.8 version-event catalogue instead lists `DashboardVersionPublished` and does not list `DashboardVersionLocked`. The unambiguous `DashboardPublished` event is sufficient for this L3 outcome, so the conflict does not prevent this Draft, but the H07 version-event vocabulary requires upstream reconciliation before scenario locking.
- H07 also differs on the Dashboard Version lifecycle entry vocabulary (`Created` in §12.6 versus `Candidate` in §52.7, with `Locked` additionally present in §52.7). This Draft does not depend on the disputed entry label, but upstream reconciliation is required before locking.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
