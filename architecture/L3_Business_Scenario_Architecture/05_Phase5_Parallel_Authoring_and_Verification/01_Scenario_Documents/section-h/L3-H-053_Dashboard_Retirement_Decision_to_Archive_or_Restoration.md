# Dashboard Retirement Decision to Archive or Restoration

# 1. Scenario Metadata

**Scenario ID:** L3-H-053  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Dashboard  
**Participating Domains:** A, H  
**Scenario Type:** Cross-domain  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A04 — Authorization & Policy Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Dashboard governance, retention, restoration, assignment, and audit scenarios

# 2. Scenario Purpose

Govern the withdrawal or restoration of an H07-owned Dashboard while preserving dashboard history, retention obligations, Section A authorization ownership, and source-domain business ownership.

# 3. Business Outcome

The dashboard is archived, restored, or permanently retired under retention and authorization rules

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A dashboard is no longer needed or an archived dashboard is required again.

## 4.2 End Boundary

The supported H07 branches end when a Dashboard is archived or restored under governed authority. The frozen permanently retired outcome cannot be completed from the locked H07 lifecycle because no canonical Dashboard `Retired` state, transition, or retirement event is defined, and H07's restoration workflow is not fully reconciled with its Dashboard state machine.

## 4.3 In Scope

- H07-owned Dashboard archival and restoration decisions.
- Retention, hold, governance, and authorization checks applicable to archival or restoration.
- Preservation of Dashboard history, version lineage, sharing evidence, and audit evidence.
- Section A authorization decisions governing the action.

## 4.4 Out of Scope

- Inventing a Dashboard `Retired` state, transition, authority, or event.
- Treating H07 soft deletion as equivalent to permanent retirement.
- Selecting a restoration lifecycle state not authoritatively reconciled by H07.
- Modification of source-domain business facts or KPI definitions.
- Technical storage, deployment, UI, or implementation mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns authorization policies and decisions that constrain archival and restoration authority. |
| Enterprise Dashboard / H | Owns the Dashboard, Dashboard lifecycle metadata, archival/restoration governance, version history, sharing evidence, and dashboard audit history. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Administrator | Performs H07 archival or restoration where authorized. | Performs |
| Dashboard Owner | May initiate or participate in H07 archival where authorized. | Initiates / participates |
| Governance Reviewer | Reviews archival or restoration where H07 governance requires review. | Reviews |

H07 does not define a canonical business actor or role for a distinct permanent-retirement action.

# 7. Preconditions and Entry Criteria

- The Dashboard exists under H07 ownership.
- For archival, no mandatory hold or retention condition prevents the action.
- For restoration, the Dashboard is archived and current retention and governance policy permits restoration.
- The acting party is authorized through Section A and the applicable H07 governance path.
- Historical versions and dashboard evidence remain preserved.

# 8. Trigger

A dashboard is no longer needed or an archived dashboard is required again

# 9. Main Business Flow

1. Receive the governed retirement or restoration decision for the H07-owned Dashboard.
2. Confirm the Dashboard's current lifecycle position and the acting party's authority.
3. Evaluate applicable retention, hold, approval, and governance constraints.
4. If the Dashboard is no longer needed and archival is permitted, H07 removes it from active use, preserves its historical versions and sharing evidence, closes active dashboard assignments as defined upstream, and records the archival outcome.
5. If an archived Dashboard is required again and restoration is permitted, H07 restores the Dashboard metadata and governed references without altering historical versions and records the restoration outcome.
6. Preserve the Dashboard audit history and Section A authorization evidence for the completed supported action.
7. Stop before claiming permanent retirement because locked H07 defines no canonical Dashboard retirement lifecycle or event distinct from archive or soft deletion.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Policy-driven archival | H07 lifecycle policy determines archival is due | Apply the same retention, hold, and authorization checks before archiving | Dashboard archived with history preserved | Enterprise Dashboard |
| Restoration requires current-scope validation | The archived Dashboard's prior context is no longer valid | H07 validates the current governed context before restoration; L3 does not select an unresolved Workspace ownership model | Restoration proceeds only if current governance permits it | Enterprise Dashboard / Platform Foundation within their canonical boundaries |
| Permanent retirement requested | Institutional decision requires a terminal outcome beyond archive | Do not equate archive, delete, or soft deletion with `Retired` | Upstream architecture resolution required | Enterprise Dashboard |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Retention hold, open approval, or other mandatory preservation condition prevents archival | Dashboard cannot be archived | Preserve the current governed state until the blocking condition is resolved | Enterprise Dashboard |
| Restoration is no longer permitted by current retention or governance policy | Archived Dashboard cannot return to use | Reject the restoration request and preserve archival history | Enterprise Dashboard |
| Acting party lacks authority | Archival or restoration cannot proceed | Deny the action under Section A authorization policy | Platform Foundation / A |
| Permanent retirement is required | Frozen terminal branch is undefined upstream | Escalate for H07 architecture resolution rather than substituting another state | Enterprise Dashboard |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Archival preserves historical versions and sharing evidence | Enterprise Dashboard | Archive withdraws active use without erasing history | L2-H07 §51.5 |
| Archived dashboards do not appear in normal user navigation | Enterprise Dashboard | Archived Dashboard is withdrawn from normal availability | L2-H07 BR-DASH-008 |
| H07 deletion is retained through soft deletion | Enterprise Dashboard | `Deleted` cannot be silently interpreted as permanent retirement | L2-H07 BR-DASH-009 |
| Restoration does not alter historical versions | Enterprise Dashboard | Restored availability preserves prior history | L2-H07 §51.6 |
| Authorization is mandatory and business domains do not redefine it | Platform Foundation | H07 archival/restoration actions remain constrained by Section A | L2-A04 AP-01, AP-05 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Dashboard | Active or another archival-eligible H07 position | Governed archival | Archived | Enterprise Dashboard | L2-H07 §§12.1, 51.5, 52.2 |
| Dashboard | Archived | Governed restoration | H07 workflow says return to a managed active lifecycle state, but §52.2 does not define the canonical restore transition/resulting state | Enterprise Dashboard | L2-H07 §§51.6, 52.2 |
| Dashboard | Archived or other terminal-candidate position | Frozen permanent-retirement branch | No canonical `Retired` Dashboard state exists | Enterprise Dashboard | L2-H07 §§12.1, 52.2 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| DashboardArchived | Enterprise Dashboard | Authorized H07 governance consumers | Records successful Dashboard archival | L2-H07 §23.2; §51.5 |
| DashboardRestored | Enterprise Dashboard | Authorized H07 governance consumers | Records successful Dashboard restoration | L2-H07 §23.2; §51.6 |

H07 also defines `DashboardDeleted`, but its own rule states deleted dashboards are retained through soft deletion. No canonical `DashboardRetired` event is defined, so L3 does not use `DashboardDeleted` as a substitute for permanent retirement.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A supplies the authoritative authorization decision and constraints. H07 owns the Dashboard archival/restoration action and preserves only dashboard-owned metadata and history. Source-domain business facts remain outside H07 ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Archive Dashboard | Administrator / Dashboard Owner / Governance Reviewer as defined by H07 | Must satisfy H07 archival eligibility and Section A authorization | Enterprise Dashboard |
| Restore Dashboard | Administrator / Governance Reviewer as defined by H07 | Dashboard must be archived and restoration permitted under current policy | Enterprise Dashboard |
| Permanently retire Dashboard | Undefined upstream | No distinct H07 retirement action or authority is defined | Enterprise Dashboard |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule beyond source-defined governance and Section A authorization is introduced. Retention or hold conditions are not overridden locally.

## 15.3 Business Audit and Evidence Obligations

Preserve the action initiator, governance/authorization outcome, archival or restoration reason, retention classification, relevant approval evidence, historical version lineage, sharing evidence, and Dashboard audit history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported archive branch completes when the Dashboard is governed into `Archived` with required history preserved. The supported restoration branch may record `DashboardRestored`, but its exact lifecycle landing state remains unresolved in H07. The complete frozen scenario cannot be source-complete until permanent retirement and restoration-state semantics are resolved.

## 16.2 Postconditions

Archived Dashboard history remains preserved; a supported restored Dashboard may return to controlled use under current authorization; no permanent-retirement outcome is fabricated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen semantics require a **permanently retired** Dashboard outcome, but H07's Dashboard lifecycle ends at `Archived` and defines no `Retired` state or `DashboardRetired` event.
- H07 defines `DashboardDeleted`, but BR-DASH-009 makes deletion a soft-deletion retention mechanism; L3 cannot silently equate that with permanent retirement.
- H07 §51.6 defines a Restore Dashboard workflow and `DashboardRestored`, while §52.2's allowed Dashboard transitions do not define an `Archived` restoration transition or canonical restored state; the state diagram and workflow therefore are not fully reconciled.
- H07 restoration alternatives can involve a Workspace, while locked A04 and H07 both claim Workspace ownership; this does not alter Dashboard archival ownership but reinforces the need not to invent a restoration hand-off.

These issues materially affect the frozen final outcome and lifecycle.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H07 architecture must define the canonical Dashboard permanent-retirement outcome and reconcile the Restore Dashboard workflow with the Dashboard state machine before this scenario can be completed.
