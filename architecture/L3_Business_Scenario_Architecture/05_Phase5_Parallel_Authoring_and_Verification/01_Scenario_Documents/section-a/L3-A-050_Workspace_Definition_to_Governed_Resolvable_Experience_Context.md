# Workspace Definition to Governed Resolvable Experience Context

# 1. Scenario Metadata

**Scenario ID:** L3-A-050  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; `L2-A04_Workspace_Governance_Authority_and_Lifecycle_Amendment_2026-09-23.md`; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Institutional experience-resolution and navigation consumers

---

# 2. Scenario Purpose

Define and govern a Workspace that represents an institutional operational context, govern its Workspace-to-Role mappings as part of the Workspace definition/version, and make the Active Workspace resolvable for authorized users without granting Permissions, changing Authorization Decisions or prescribing a user-interface implementation.

---

# 3. Business Outcome

A governed Workspace definition/version is created, reviewed and activated under Authorization & Policy authority; its governed Role mappings become eligible for Workspace resolution; and authorized users may receive the resulting experience context without any Role, Permission or Authorization Decision change.

Workspace Mapping remains governed configuration of the Workspace rather than an independently owned aggregate or lifecycle.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an institutional user population requires a governed operational context that presents responsibilities already represented by existing governed Roles.

## 4.2 End Boundary

The scenario completes when the applicable Workspace definition/version reaches `Active`, `WorkspaceActivated` is published by Authorization & Policy, and the Workspace becomes eligible for governed resolution using existing assigned Roles, institution configuration, Workspace mappings and administrative policy.

## 4.3 In Scope

- Tenant and institution context.
- Definition of Workspace business purpose and experience context.
- Reference to existing Roles.
- Workspace-to-Role mapping configuration owned by the Workspace version.
- Workspace lifecycle `Draft → Under Review → Active → Retired`.
- Governed return `Under Review → Draft`.
- Successor-version governance for material change to an Active Workspace.
- Workspace Governance Authority review/activation/retirement.
- Dynamic Workspace resolution for authorized users.
- `WorkspaceActivated` and `WorkspaceRetired` publication.
- Audit evidence for definition, mapping, validation, governance and lifecycle outcomes.

## 4.4 Out of Scope

- Creating or assigning Roles or Permissions.
- Granting or revoking access.
- Producing or changing an Authorization Decision.
- Authentication or session creation.
- Rendering screens or specifying UI components, routes, APIs, claims or storage.
- Business workflow execution inside the Workspace.
- Separate Workspace Mapping aggregate/lifecycle.
- Event transport or technical implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access | Owns Role Assignment; a Workspace may reflect assigned Roles but cannot grant them. |
| Related | L3-A-051 — Conflicting Responsibility Risk to Enforced Separation of Duties | SoD remains an authorization control independent of Workspace selection. |
| Related | L3-A-052 — Authorization Policy Change to Governed Active Policy Version | Governs Authorization Policy versions rather than Workspace definitions. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authorization & Policy / Section A | Primary owner. Owns Workspace identity, lifecycle, Workspace Mapping configuration, governance authority semantics, resolution rules and Workspace lifecycle events. |
| Platform & Tenant / Section A | Supplies authoritative Tenant/institution scope and configuration context. |
| Person & Identity / Section A | Supplies canonical Person/Identity/User Account references used during later resolution. |
| Event Infrastructure / Section A | Distributes Workspace lifecycle events without becoming their semantic publisher. |
| Audit Infrastructure / Section A | Preserves immutable governance evidence without acquiring Workspace ownership. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized Administrative Actor | Requests a new Workspace or material change, defines/revises a Draft Workspace version and configures governed Role mappings where institutional policy permits. | May configure Draft definition/mapping data but does not approve itself merely by configuration and does not gain Role/Permission authority. |
| Workspace Governance Authority | Reviews the Workspace version and makes the governed return, activation and retirement decisions. | May return `Under Review → Draft`, approve `Under Review → Active`, and authorize `Active → Retired`; abstract institutional authority mapped locally through policy. |
| Authorization & Policy Domain | Owns Workspace aggregate/lifecycle, validation rules, mapping governance, resolution semantics and lifecycle events. | Canonical business owner. |
| Authorized User | Receives or selects an available Workspace after governed resolution. | Consumer only; selection never grants access or changes authorization. |

No universal institutional job title is imposed for the Workspace Governance Authority.

---

# 7. Preconditions and Entry Criteria

- Exactly one Tenant and applicable institution context can be resolved.
- The institutional experience need is known.
- Referenced Roles already exist under L2-A04 governance.
- The Authorized Administrative Actor is permitted to configure the applicable Workspace scope.
- Workspace mapping and institution configuration rules can be evaluated.
- The proposed Workspace definition/mapping does not create, grant, revoke or modify Role/Permission ownership.
- Required administrative-policy and governance evidence can be established.

---

# 8. Trigger

An institutional user population requires a new governed Workspace context, or a material change requires a successor version of an existing Active Workspace.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the requirement for a governed operational context or material change to an existing Active Workspace. | Authorized Administrative Actor; Authorization & Policy | A new Workspace or successor-version need is established. |
| 2 | Resolve Tenant, institution and applicable configuration. | Platform & Tenant; Authorization & Policy | Workspace scope is authoritative. |
| 3 | Create a new Workspace definition/version in `Draft`. | Authorized Administrative Actor; Authorization & Policy | Draft Workspace version exists. |
| 4 | Define Workspace purpose and responsibility-oriented experience context. | Authorized Administrative Actor; Authorization & Policy | Business purpose is explicit without prescribing UI implementation. |
| 5 | Reference existing governed Roles and configure Workspace mappings as part of the Draft Workspace version. | Authorized Administrative Actor; Authorization & Policy | Governed mapping configuration exists without creating Roles. |
| 6 | Validate Tenant isolation, institution configuration, Role availability, mapping consistency and authorization boundaries. | Authorization & Policy | Invalid or cross-Tenant configuration is rejected. |
| 7 | Submit the valid Draft Workspace for governance review, causing `Draft → Under Review`. | Authorized Administrative Actor; Authorization & Policy | Workspace version is awaiting governance decision. |
| 8 | Workspace Governance Authority reviews the version and either returns it `Under Review → Draft` for correction or approves activation. | Workspace Governance Authority | Governed decision is attributable. |
| 9 | On approval and satisfactory validation, transition `Under Review → Active`. | Authorization & Policy under Workspace Governance Authority decision | Workspace becomes eligible for governed resolution. |
| 10 | Publish `WorkspaceActivated` as an Authorization & Policy business fact. | Authorization & Policy | Active Workspace availability is externally observable without ownership transfer. |
| 11 | Resolve the Active Workspace for an authorized user from assigned Roles, institution configuration, Workspace mappings and administrative policy. | Authorization & Policy | Governed experience context is returned without granting access. |
| 12 | Preserve definition, mapping, validation, decision, version lineage and lifecycle/event evidence. | Authorization & Policy; Audit Infrastructure | Outcome is historically traceable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Review requires correction | Workspace is `Under Review` and governance requires revision. | Return `Under Review → Draft`; revise and resubmit. | No Active availability until later approval. | Workspace Governance Authority; Authorization & Policy |
| Multiple assigned Roles | A user holds Roles mapped to more than one Active Workspace. | Apply governed resolution and institution configuration. | Multiple eligible contexts may be presented without changing authorization. | Authorization & Policy |
| No applicable mapping | No Active Workspace mapping corresponds to held Roles. | Return no governed Workspace context, except an institutional default only where policy explicitly defines one. | No Permission or authorization change. | Authorization & Policy |
| Material Active change | An Active Workspace requires a material definition/mapping change. | Create governed successor in `Draft`; predecessor remains Active until successor activation. | No silent rewrite of the Active version. | Authorized Administrative Actor; Authorization & Policy |
| Successor activation | Approved successor reaches `Active`. | Make successor Active, transition superseded predecessor `Active → Retired`, publish applicable lifecycle events. | New version becomes resolvable and predecessor remains historical. | Workspace Governance Authority; Authorization & Policy |
| Workspace retirement | Active Workspace is no longer eligible for use. | Authorize `Active → Retired` and publish `WorkspaceRetired`. | Retired version is no longer eligible for new resolution. | Workspace Governance Authority; Authorization & Policy |
| Workspace switch | Authorized user selects another available Active Workspace. | Revalidate selected context against current mappings and policy. | Experience context changes; permissions and authorization do not. | Authorization & Policy |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant/institution scope is unresolved. | Workspace governance cannot preserve isolation. | Reject progression and retain current valid Workspace lifecycle position. | Platform & Tenant; Authorization & Policy |
| Referenced Role is unavailable or invalid. | Mapping lacks a valid responsibility reference. | Reject/correct mapping without creating a Role. | Authorization & Policy |
| Mapping would imply Role/Permission grant or authorization change. | Workspace boundary would be violated. | Reject mapping and preserve Role/Permission/Authorization Decision ownership. | Authorization & Policy |
| Validation fails before review/activation. | Workspace cannot progress. | Retain `Draft` or return to `Draft` as applicable. | Authorization & Policy |
| Governance approval is absent. | Workspace cannot become resolvable. | Remain `Under Review` or return to `Draft`; do not expose as Active. | Workspace Governance Authority; Authorization & Policy |
| Material Active change is attempted in place. | Active historical definition would be rewritten. | Reject in-place material rewrite and require a successor Draft version. | Authorization & Policy |
| Retired Workspace is requested for new resolution. | Terminal historical version would be reused. | Reject new resolution against Retired version. | Authorization & Policy |

No technical retry, rollback or implementation recovery is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Workspace is owned by Authorization & Policy. | Authorization & Policy | Workspace governance cannot transfer to UI/business modules. | L2-A04 + Workspace Governance Amendment |
| Workspace Mapping is configuration of a Workspace version, not a separate aggregate. | Authorization & Policy | Mapping uses the Workspace lifecycle/version boundary. | L2-A04 Workspace Governance Amendment |
| Workspace lifecycle is `Draft → Under Review → Active → Retired` with `Under Review → Draft`. | Authorization & Policy | No local state invention is permitted. | L2-A04 Workspace Governance Amendment |
| Only Active Workspace versions are eligible for resolution. | Authorization & Policy | Draft/Under Review/Retired versions cannot be returned as current governed context. | L2-A04 Workspace Governance Amendment |
| Material Active change creates a successor version. | Authorization & Policy | Active history is not silently rewritten. | L2-A04 Workspace Governance Amendment |
| Workspace does not grant/revoke Permissions. | Authorization & Policy | Mapping cannot create access rights. | L2-A04 AP-07/AP-09 + Amendment |
| Workspace selection does not change Authorization Decisions. | Authorization & Policy | Every protected action remains independently authorized. | L2-A04 AP-07/AP-09 |
| Resolution uses assigned Roles, institution configuration, Active Workspace Mapping and administrative policy. | Authorization & Policy | Inputs govern availability. | L2-A04 Workspace Resolution + Amendment |
| Tenant isolation must be preserved. | Platform & Tenant; Authorization & Policy | Cross-Tenant mapping is prohibited. | L1; L2-A04 |
| Workspace governance must be auditable. | Authorization & Policy; Audit Infrastructure | Definition/version/mapping/decision history remains traceable. | L2-A04 + Amendment |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Position | Canonical Owner |
|---|---|---|---|---|
| Workspace | New definition/version required | Create and configure. | `Draft` | Authorization & Policy |
| Workspace | `Draft` and valid for review | Submit for governance. | `Under Review` | Authorization & Policy |
| Workspace | `Under Review`; revision required | Return for correction. | `Draft` | Authorization & Policy |
| Workspace | `Under Review`; governance approves and validation passes | Activate. | `Active` | Authorization & Policy |
| Workspace | `Active`; retirement authorized or governed successor activated | Retire. | `Retired` | Authorization & Policy |
| Workspace Mapping | Applicable Workspace version | Configure references to existing Roles. | Governed configuration within Workspace version | Authorization & Policy |
| Role | Existing | Referenced only. | Unchanged | Authorization & Policy |
| Permission | Existing | No assignment/modification. | Unchanged | Authorization & Policy |
| Authorization Decision | Independent protected operation | Not created/modified by Workspace lifecycle. | Unchanged | Authorization & Policy |
| Person / Identity / User Account | Existing | Used only during resolution. | Unchanged | Person & Identity |

`Retired` is terminal for the affected Workspace version.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `WorkspaceActivated` | Authorization & Policy / L2-A04 | Authorized experience/navigation/dashboard consumers and other explicitly governed consumers | Communicate that a governed Workspace version reached Active and is eligible for resolution. | L2-A04 Workspace Governance Amendment |
| `WorkspaceRetired` | Authorization & Policy / L2-A04 | Authorized experience/navigation/dashboard consumers and other explicitly governed consumers | Communicate that a previously Active Workspace version is no longer eligible for new resolution. | L2-A04 Workspace Governance Amendment |

Existing audit labels such as `Workspace Changed`, `Workspace Switch Requested` and `Workspace Validation Failure` remain audit/operational labels and are not substituted for these canonical lifecycle events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant/institution context | Platform & Tenant | Workspace governance | Scope and configuration. |
| Assigned Role references | Role Assignment / Role Resolution | Workspace Resolution | Current governed responsibilities. |
| Active Workspace definition and mappings | Authorization & Policy | Workspace Resolution / authorized experience consumers | Eligible experience contexts. |
| Resolved Workspace context | Authorization & Policy | Authorized user experience | Context only, without access grant. |
| Workspace lifecycle event | Authorization & Policy | Event Infrastructure / authorized consumers | Active or Retired lifecycle fact without publisher transfer. |
| Audit input | Authorization & Policy | Audit Infrastructure | Definition, mapping, governance and lifecycle evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Workspace availability cannot grant a Role or Permission.
- Workspace selection cannot override an Authorization Decision.
- Protected operations remain subject to centralized authorization and explicit-deny precedence.

## 15.2 Approval and Separation of Duty

- Authorized Administrative Actor may request/configure Draft Workspace definitions under institutional policy.
- Workspace Governance Authority decides return, activation and retirement.
- Workspace Governance Authority is an abstract business authority rather than one universal job title.
- No universal separation-of-duties requirement is introduced between configurer and governance authority; institutional policy may impose one.

## 15.3 Audit Obligations

Tenant, institution, Workspace identity/version, purpose, referenced Roles, mappings, validation, actor, governance decision, lifecycle transition, predecessor/successor lineage, event publication and time/correlation shall remain attributable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Tenant/institution scope is authoritative.
- A Workspace definition/version exists with governed purpose and Role mappings.
- Validation passed without creating or changing access rights.
- Workspace Governance Authority approved activation.
- Workspace reached `Active`.
- `WorkspaceActivated` was published by Authorization & Policy.
- The Active Workspace is eligible for governed resolution.
- Required governance evidence is preserved.

## 16.2 Postconditions

- Authorized users may receive the Active Workspace where current Role mappings/configuration/policy permit.
- Role, Permission and Authorization Decision state remains unchanged by Workspace activation.
- Material later changes require a successor Workspace version.
- Retired Workspace versions remain historical and are not eligible for new resolution.
- Workspace ownership remains with Authorization & Policy.

---

# 17. Open Issues, Assumptions and Deferred Questions

No blocking issues remain for this scenario after the locked `L2-A04_Workspace_Governance_Authority_and_Lifecycle_Amendment_2026-09-23.md`.

No assumptions.

UI rendering, navigation composition, dashboard composition and implementation technology remain deferred to their owning downstream architecture.
