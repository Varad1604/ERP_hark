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
**Last Updated:** 2026-08-23  
**Depends On:** Locked L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A04 — Authorization & Policy Workflows; L2-B02 — Organisation Workflows; L2-B03 — Campus Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Describe how authoritative role, organization, campus, or user-assignment changes affect dashboard-workspace availability while preserving ownership of authorization and organizational facts.

# 3. Business Outcome

The authorized workspace and dashboard set becomes available or is withdrawn according to policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed role, organization, campus, or user assignment changes.

## 4.2 End Boundary

The frozen end boundary is the resulting availability or withdrawal of the authorized dashboard workspace and dashboard set. Locked A04 and H07 both claim canonical ownership of `Workspace`, preventing a safe owner assignment for this outcome.

## 4.3 In Scope

- Preserve A04 ownership of Role and Role Assignment facts and authorization decisions.
- Preserve B02 ownership of Organization facts and B03 ownership of Institutional Campus facts.
- Apply relevant authoritative assignment/context changes to dashboard availability only through a defined cross-domain contract.
- Preserve H07 ownership of dashboard composition.

## 4.4 Out of Scope

- Re-owning Role, Role Assignment, Organization, Campus, or authorization decisions in H07.
- Inventing an H07 workspace owner boundary that contradicts A04.
- Inventing unassignment, withdrawal, or cross-domain event contracts absent from locked sources.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Platform Foundation / A04 | Explicitly owns Role, Role Assignment, authorization decisions, and `Workspace`. |
| Institution & Structure / B02 | Owns Organization and organizational-structure facts. |
| Institution & Structure / B03 | Owns Institutional Campus identity and lifecycle facts. |
| Enterprise Dashboard / H07 | Owns dashboard composition but also independently claims `Workspace`, creating a locked-source ownership conflict with A04. |

# 6. Business Actors, Roles and Responsibilities

A04 governs role assignment and authorization authority; B02/B03 govern organization and campus facts. H07 identifies administrative actors for dashboard workspace assignment, but L3 cannot assign final workspace authority while A04 and H07 both claim the same aggregate.

# 7. Preconditions and Entry Criteria

- A role/user assignment change is authoritative under A04, or an organization/campus change is authoritative under B02/B03.
- Dashboard composition remains H07-owned.
- No L3 assumption resolves the A04/H07 Workspace ownership conflict.

# 8. Trigger

A governed role, organization, campus, or user assignment changes

# 9. Main Business Flow

1. The owning upstream domain records the authoritative role, assignment, organization, campus, or user-context change.
2. H07 must determine whether the changed authoritative context affects dashboard availability without changing the upstream fact.
3. If an applicable dashboard-assignment relationship remains valid, the relevant dashboard set may remain available.
4. If the authoritative context no longer permits availability, the dashboard-workspace relationship must be withdrawn under the canonical Workspace owner and a defined cross-domain hand-off.

The scenario cannot safely complete because locked sources disagree on who owns Workspace and H07 does not define a canonical withdrawal/unassignment event for the frozen outcome.

# 10. Alternate Business Flows

- Organization or campus context may change while dashboard composition remains unchanged.
- A role assignment may become inactive, expired, suspended, or revoked under A04 without transferring that lifecycle to H07.
- L3 does not map these upstream changes to an invented H07 withdrawal transition.

# 11. Failure, Interruption and Business Recovery

If ownership or cross-domain assignment evidence is ambiguous, dashboard-workspace availability must not be changed through an invented local rule. The business issue remains pending architecture resolution.

# 12. Business Rules and Decision Constraints

- A04 exclusively owns Role, Role Assignment, and authorization decisions.
- B02 owns Organization facts; B03 owns Institutional Campus facts.
- `DEPENDENCY_MAP.md` assigns H07 dashboard composition ownership but does not transfer A04 authorization or B organizational ownership.
- Cross-domain participation does not transfer canonical ownership.

# 13. State and Lifecycle Impact

A04 owns the Role Assignment lifecycle. B02/B03 own their respective organizational lifecycles. H07 defines a Workspace lifecycle, but A04 also declares Workspace as its own aggregate. L3 cannot decide which Workspace lifecycle governs the frozen outcome.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H07 defines `WorkspaceAssigned` and states that its assignment workflow consumes `UserGroupChanged` and `OrganisationRestructured`. Those consumed event names are not defined in the applicable locked A04/B02 sources reviewed for this scenario. B03 instead publishes canonical campus events such as `CampusUpdated`, `CampusActivated`, `CampusDecommissioned`, and `CampusArchived`. L3 does not invent a mapping.

## 14.2 Business Contracts and Cross-Domain Hand-offs

A canonical contract is required from A04/B02/B03 authoritative changes to the owner of dashboard-workspace availability. That contract cannot be finalized while Workspace ownership itself conflicts and the H07 consumed-event names do not resolve to the reviewed upstream sources.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Role assignment and authorization decisions remain A04-owned; organization/campus decisions remain B-owned. Final Workspace assignment/withdrawal authority is unresolved because of the A04/H07 ownership conflict.

## 15.2 Separation of Duties, Overrides and Exceptions

H07 must not reinterpret an A04 authorization decision or a B-owned organization/campus fact. No L3 override resolves duplicate Workspace ownership.

## 15.3 Business Audit and Evidence Obligations

Preserve the authoritative upstream change, authorization evidence, dashboard assignment history, and any resulting availability decision without rewriting the source record.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen completion criterion requires an authoritative determination that the dashboard workspace/set is available or withdrawn. That authority cannot be assigned consistently from the locked sources.

## 16.2 Postconditions

No new Workspace ownership, withdrawal state, or synthetic cross-domain event is introduced at L3.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**Material ownership conflict:** locked A04 explicitly owns `Workspace`; locked H07 also explicitly owns `Workspace` and its lifecycle.

**Material contract gap:** H07 cites `UserGroupChanged` and `OrganisationRestructured` as consumed assignment events, but the reviewed locked A04/B02 sources do not define those canonical event names, and no H07 withdrawal/unassignment event is defined for the frozen end outcome.

**Architecture impact:** canonical owner, lifecycle, and mandatory cross-domain contract for dashboard-workspace availability/withdrawal are unresolved.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Architecture change control must establish the canonical Workspace boundary between A04 and H07 and the authoritative upstream assignment-change contracts consumed by dashboard availability governance.
