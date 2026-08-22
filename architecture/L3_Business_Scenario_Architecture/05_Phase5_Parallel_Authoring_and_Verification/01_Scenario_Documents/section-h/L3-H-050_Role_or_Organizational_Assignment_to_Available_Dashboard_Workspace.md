# Role or Organizational Assignment to Available Dashboard Workspace

# 1. Scenario Metadata

**Scenario ID:** L3-H-050  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Enterprise Dashboard  
**Participating Domains:** A, B, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; L2-B02 — Organisation Workflows; L2-B03 — Campus Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Dashboard assignment, workspace governance, sharing, and dashboard availability scenarios

# 2. Scenario Purpose

Govern dashboard-workspace availability when role, organization, campus, or user assignment context changes while preserving canonical ownership of identity, authorization, organization, campus, and dashboard information.

# 3. Business Outcome

The authorized workspace and dashboard set becomes available or is withdrawn according to policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governed role, organization, campus, or user assignment changes.

## 4.2 End Boundary

The frozen outcome requires a canonical workspace and dashboard availability decision. Locked A04 and locked H07 both claim canonical ownership of `Workspace` with overlapping dashboard/navigation responsibilities, so L3 cannot determine the owning Workspace lifecycle and hand-off without an upstream resolution.

## 4.3 In Scope

- Canonical Section A role, assignment, identity, and authorization facts.
- Canonical Section B organization and institutional campus facts.
- H07 dashboard and dashboard-assignment metadata.
- Workspace availability only to the extent ownership is unambiguous.

## 4.4 Out of Scope

- Selecting one Workspace owner over another.
- Inventing a bridge between A04 Workspace and H07 Workspace.
- Re-owning Role Assignment, Organization, Campus, Person/Identity, or Dashboard.
- Technical navigation or UI design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / A | Owns Person/Identity references, Role, Role Assignment, Authorization Policy and Authorization Decision; A04 also claims Workspace and Workspace Switching. |
| Institution & Structure / B | B02 owns Organization and Organization Unit; B03 owns Institutional Campus identity and lifecycle. |
| Enterprise Dashboard / H | Owns Dashboard and dashboard composition/assignment metadata; H07 also claims Workspace and Workspace assignment/lifecycle. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Administrator | Performs H07 workspace/dashboard assignment actions where authorized. | Performs |
| Workspace Owner | Participates in H07 workspace/dashboard assignment where authorized. | Performs |
| Security Officer | Participates in H07 dashboard assignment governance where H07 defines that role. | Reviews / performs |

The canonical actor responsible for resolving the A04-versus-H07 Workspace ownership boundary is not defined upstream and is not invented here.

# 7. Preconditions and Entry Criteria

- Canonical Person/Identity and any applicable Section A Role Assignment already exist.
- Applicable Organization or Institutional Campus references exist under Section B where the trigger concerns those scopes.
- Any H07 dashboard to be assigned is published or otherwise explicitly assignable according to H07.
- No assumption is made that H07 Workspace ownership supersedes A04 Workspace ownership.

# 8. Trigger

A governed role, organization, campus, or user assignment changes

# 9. Main Business Flow

1. Receive the authoritative role, organization, campus, or user assignment change from its canonical owner.
2. Preserve Section A and Section B ownership of the changed fact.
3. Evaluate applicable authorization and visibility constraints without treating workspace visibility as permission ownership.
4. Determine the H07 dashboard set applicable to the governed context.
5. Stop before applying a canonical Workspace assignment/withdrawal outcome because A04 and H07 both claim ownership of Workspace with overlapping dashboard/navigation responsibilities.
6. Require upstream architecture resolution before the frozen available-or-withdrawn Workspace outcome is asserted.

# 10. Alternate Business Flows

H07 defines temporary and inherited workspace assignments and time-bound/inherited dashboard assignments. These paths remain subordinate to the unresolved Workspace ownership conflict and cannot be selected as the canonical cross-domain path.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid or unauthorized target assignment | Availability must not change | Reject the requested H07 assignment under the owning authorization policy | Platform Foundation / A and Enterprise Dashboard / H within their unambiguous boundaries |
| Workspace ownership cannot be resolved | Final workspace availability/withdrawal cannot be authoritatively completed | Escalate the A04/H07 ownership conflict for upstream architecture resolution | Architecture governance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A04 owns Role, Role Assignment, Authorization Policy, Authorization Decision and Workspace | Platform Foundation | H07 cannot silently assume those authorization-owned responsibilities | L2-A04 §§3, 9, 10.4 |
| A04 Workspace controls user-experience context and visibility but never grants permissions | Platform Foundation | Workspace availability must not be treated as new authority | L2-A04 AP-07, AP-09 |
| B02 is authoritative for Organization and Organization Unit | Institution & Structure | H07 references organizational scope without re-owning it | L2-B02 |
| B03 is authoritative for Institutional Campus identity/status | Institution & Structure | H07 references campus scope without re-owning it | L2-B03 |
| H07 owns Dashboard composition and claims Workspace/Workspace assignment | Enterprise Dashboard | Creates a direct ownership collision with A04 that L3 cannot resolve | L2-H07 §§2.3, 8.4, 51.13 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Role Assignment / authorization facts | Existing governed state | Provides changed assignment/authorization context | Remains owned and governed by Section A | Platform Foundation | L2-A04 |
| Organization / Institutional Campus | Existing governed state | Provides changed organizational/campus context | Remains owned and governed by Section B | Institution & Structure | L2-B02; L2-B03 |
| Workspace | Existing applicable state | Frozen outcome requires availability or withdrawal | Cannot be resolved because A04 and H07 both claim Workspace lifecycle/ownership | Unresolved conflict | L2-A04; L2-H07 |
| Dashboard assignment | Published/assignable dashboard | May change presentation availability once Workspace ownership is resolved | Assigned/available or withdrawn according to governed H07 path | Enterprise Dashboard | L2-H07 §51.9 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H07 publishes `DashboardAssigned` and `WorkspaceAssigned` for its assignment workflows. H07 also names consumed events `UserRoleChanged`, `UserGroupChanged`, and `OrganisationRestructured`, but those exact event names were not established in the reviewed locked A04/B02 sources. L3 therefore does not claim them as canonical cross-domain events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A and Section B must provide their canonical assignment, authorization, organization, and campus facts through approved collaboration mechanisms. The exact Workspace hand-off cannot be defined until the A04/H07 ownership collision is resolved.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H07 defines Administrator and Workspace Owner participation for workspace assignment and Administrator/Workspace Owner/Security Officer participation for dashboard assignment. Section A remains authoritative for authorization decisions. The disputed Workspace ownership prevents a complete final decision-authority model.

## 15.2 Separation of Duties, Overrides and Exceptions

Workspace availability must never expand permissions beyond Section A authorization. No additional override or ownership-transfer rule is invented.

## 15.3 Business Audit and Evidence Obligations

Preserve the authoritative assignment-change reference, organization/campus context where applicable, authorization decision, H07 assignment history, and ownership boundaries. No duplicate Workspace evidence model is created.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The complete frozen outcome cannot be claimed until one canonical Workspace owner and collaboration boundary are established upstream.

## 16.2 Postconditions

Section A and Section B source facts remain unchanged and canonically owned. H07 dashboard metadata is not used to resolve the Workspace ownership conflict locally.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Locked L2-A04 explicitly owns `Workspace` and `Workspace Switching` and defines Workspace as the user-experience context containing navigation, dashboards, widget visibility, and workspace configuration.
- Locked L2-H07 also explicitly owns `Workspace`, defines a Workspace aggregate/lifecycle, and owns workspace assignment, navigation, landing dashboards, and dashboard membership.
- The responsibilities materially overlap; no reviewed higher-authority source disambiguates an Authorization Workspace from an Enterprise Dashboard Workspace.
- H07 names `UserRoleChanged`, `UserGroupChanged`, and `OrganisationRestructured` as consumed events, but those exact publisher contracts are not established in the reviewed locked A04/B02 sources.

This affects canonical ownership, lifecycle, cross-domain hand-off, and the frozen final outcome.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream architecture must establish one canonical Workspace ownership model or explicitly distinguish the two concepts and define their governed relationship.
