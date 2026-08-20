# Workspace Definition to Governed Resolvable Experience Context

# 1. Scenario Metadata

**Scenario ID:** L3-A-050  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authorization & Policy  
**Participating Domains:** A  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Institutional experience-resolution and navigation consumers

---

# 2. Scenario Purpose

Define and govern a Workspace that represents an institutional operational context, map it to existing Roles under administrative policy, and make it resolvable for authorized users without granting Permissions, changing Authorization Decisions or prescribing a user-interface implementation.

---

# 3. Business Outcome

Workspaces are defined, mapped to roles under administrative policy, and made resolvable to authorized users, without granting permissions, modifying authorization decisions, or rendering user interfaces

Locked L2-A04 establishes the Workspace boundary and resolution principles but does not define the Workspace definition and mapping workflow, lifecycle states, commands, approval authority, canonical events or publishers. This scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an institutional user population requires a governed operational context that presents the responsibilities a Person holds.

## 4.2 End Boundary

The intended end boundary is reached when a governed Workspace definition and its Role mappings are approved and available for dynamic resolution to authorized users, without changing Role, Permission or Authorization Decision state. Exact completion cannot be certified until L2-A04 defines the missing lifecycle and event contracts.

## 4.3 In Scope

- Tenant and institution context.
- Definition of the Workspace's business purpose and experience context.
- Reference to existing Roles and responsibilities.
- Governed Workspace-to-Role mapping.
- Institution configuration and administrative-policy validation.
- Dynamic Workspace resolution for authorized users.
- Audit evidence for definition, mapping and governance decisions.

## 4.4 Out of Scope

- Creating or assigning Roles or Permissions.
- Granting or revoking access.
- Producing or changing an Authorization Decision.
- Authentication or session creation.
- Rendering screens or specifying UI components, routes, APIs, claims or storage.
- Workflow execution within the Workspace.
- Inventing Workspace states, commands, events, publishers or approval actors.

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
| Authorization & Policy / Section A | Primary owner. Owns Workspace definitions, Role mappings, administrative-policy validation and resolution rules. |
| Platform & Tenant / Section A | Supplies Tenant, institution and configuration context. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references used during later resolution. |
| Event Infrastructure / Section A | Distributes canonical events after such Workspace events are defined upstream. |
| Audit Infrastructure / Section A | Preserves immutable governance evidence. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized User | Receives a resolved Workspace corresponding to held responsibilities. | Consumer only; Workspace selection does not grant access. |
| Institutional user population | Provides the business need named by the trigger. | Does not imply configuration or approval authority. |

L2-A04 does not name the actor who requests, defines, approves, activates or changes a Workspace or its Role mappings. No Workspace administrator or approver is invented.

---

# 7. Preconditions and Entry Criteria

- Exactly one Tenant and applicable institution can be resolved.
- The required institutional experience context is known.
- Referenced Roles already exist under L2-A04 governance.
- Workspace mapping and institution configuration rules can be evaluated.
- Workspace definition will not alter Role or Permission ownership.
- The administrative policy expected by L2-A04 exists, although its approval actor and lifecycle contract are not defined.

---

# 8. Trigger

An institutional user population requires a governed operational context that presents the responsibilities a Person holds

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the requirement for a governed operational context. | Authorization & Policy | A Workspace-definition need is identified. |
| 2 | Resolve the Tenant, institution and applicable institution configuration. | Platform & Tenant; Authorization & Policy | The Workspace boundary is established. |
| 3 | Define the Workspace purpose and the responsibility-oriented experience it is intended to present. | Authorization & Policy — Workspace | The intended operational context is described without defining UI implementation. |
| 4 | Identify existing Roles eligible to be represented by the Workspace. | Authorization & Policy — Role | The Workspace references governed Roles without modifying them. |
| 5 | Define Workspace mappings from those Roles under administrative policy. | Authorization & Policy — Workspace | Intended resolution mappings are established. |
| 6 | Validate Tenant isolation, institution configuration and mapping consistency. | Authorization & Policy | Invalid or cross-Tenant mappings are rejected. |
| 7 | Confirm that the Workspace and its mappings neither grant Permissions nor alter Authorization Decisions. | Authorization & Policy | The authorization boundary is preserved. |
| 8 | Complete the applicable administrative review and approval. | Authorization & Policy | Intended governance approval is obtained; the actor and transition are undefined upstream. |
| 9 | Make the approved Workspace definition and mappings available for resolution. | Authorization & Policy — Workspace | Intended resolvable availability is reached; its state and event are undefined upstream. |
| 10 | Resolve the Workspace dynamically for an authorized user from assigned Roles, institution configuration, Workspace mappings and administrative policies. | Authorization & Policy | A governed experience context is returned without granting access. |
| 11 | Preserve immutable evidence of definition, mapping, validation, approval and availability. | Authorization & Policy; Audit Infrastructure | The intended outcome is traceable. |

Steps 8–9 cannot be finalized as canonical L3 behaviour because L2-A04 omits the required actors, commands, lifecycle transitions and events.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Multiple assigned Roles | A user holds Roles mapped to more than one Workspace. | Apply governed resolution and institution configuration. | Applicable contexts may be presented without changing authorization. | Authorization & Policy |
| No applicable mapping | No Workspace mapping corresponds to held Roles. | Return no governed Workspace context or an established institutional default where policy explicitly defines one. | No Permission or authorization change occurs. | Authorization & Policy |
| Mapping change | Institutional responsibility presentation changes. | Govern a successor mapping without changing underlying Roles. | Historical mapping evidence remains traceable; exact version lifecycle is undefined. | Authorization & Policy |
| Workspace switch | An authorized user selects another available Workspace. | Validate the selected context against current mappings and policies. | Experience context changes; permissions and authorization do not. | Authorization & Policy |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant or institution is unresolved. | Workspace governance cannot preserve scope. | Reject the definition or resolution request. | Platform & Tenant; Authorization & Policy |
| Referenced Role is unavailable. | The mapping has no valid responsibility reference. | Reject or correct the mapping without creating a Role. | Authorization & Policy |
| Mapping would imply Permission grant. | Workspace ownership boundary would be violated. | Reject the mapping and retain authorization ownership in Role, Permission and Policy. | Authorization & Policy |
| Administrative approval is absent. | Intended governed availability cannot be established. | Do not expose the Workspace as approved. | Authorization & Policy |
| Definition or mapping publication cannot be identified. | Consumers lack a canonical business fact. | Do not invent an event; obtain an L2-A04 definition. | Authorization & Policy |

No technical retry, rollback or implementation recovery is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Workspace is owned by Authorization & Policy. | Authorization & Policy | Workspace governance cannot be transferred to UI or business modules. | L2-A04 — Workspace |
| Workspace represents responsibilities held by a Person. | Authorization & Policy | Resolution is responsibility-oriented. | L2-A04 — Workspace |
| Workspace does not grant or revoke Permissions. | Authorization & Policy | Mapping cannot create access rights. | L2-A04 — Workspace Invariants |
| Workspace selection does not change authorization outcomes. | Authorization & Policy | Every protected action remains subject to centralized authorization. | L2-A04 — AP-07 and AP-09 |
| Workspace is independent of Role and Permission definitions. | Authorization & Policy | Definitions remain unchanged by Workspace governance. | L2-A04 — Workspace Invariants |
| Resolution uses assigned Roles, institution configuration, Workspace Mapping and Administrative Policies. | Authorization & Policy | These inputs govern dynamic resolution. | L2-A04 — Workspace Resolution |
| Tenant isolation must be preserved. | Platform & Tenant; Authorization & Policy | Cross-Tenant mapping is prohibited. | L1; L2-A04 |
| Workspace governance must be auditable. | Authorization & Policy; Audit Infrastructure | Definition and mapping history must remain traceable. | L2-A04 — Authorization Audit |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Workspace | Definition does not yet exist or requires change | Define, validate, approve and make resolvable. | Governed resolvable context; canonical states are undefined. | Authorization & Policy |
| Workspace Mapping | Mapping does not yet exist or requires change | Associate existing Roles under policy. | Approved mapping; canonical states are undefined. | Authorization & Policy |
| Role | Existing | Referenced only. | Unchanged. | Authorization & Policy |
| Permission | Existing | No assignment or modification. | Unchanged. | Authorization & Policy |
| Authorization Decision | None for a protected request | No decision is created by Workspace definition or selection. | Unchanged. | Authorization & Policy |
| Person / Identity / User Account | Existing | Used only during later resolution. | Unchanged. | Person & Identity |

L2-A04 provides no canonical Workspace or Workspace Mapping lifecycle-state model; no state names are introduced here.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Event Determination

| Required Business Fact | Source Resolution Result | Scenario Treatment |
|---|---|---|
| Workspace defined | No canonical event or publisher defined. | No event is invented. |
| Workspace mapping approved | No canonical event or publisher defined. | No event is invented. |
| Workspace made resolvable | No canonical event or publisher defined. | No event is invented. |

L2-A04's human-readable audit labels `Workspace Changed`, `Workspace Switch Requested` and `Workspace Validation Failure` are not canonical Workspace lifecycle events and do not identify sole publishers.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant and institution context | Platform & Tenant | Workspace | Scope and configuration. |
| Assigned Role references | Role Assignment / Role Resolution | Workspace Resolution | Current governed responsibilities. |
| Workspace definition and mappings | Workspace | Workspace Resolution | Eligible experience contexts. |
| Resolved Workspace context | Workspace Resolution | Authorized user experience | Context only, without access grant. |
| Audit input | Authorization & Policy | Audit Infrastructure | Definition, mapping, approval and resolution evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Workspace availability cannot grant a Role or Permission.
- Workspace selection cannot override an Authorization Decision.
- Protected operations still require centralized policy evaluation with explicit-deny precedence.

## 15.2 Approval and Separation of Duty

- L2-A04 requires administrative-policy governance but does not name the requestor, configuration authority or approver.
- No approval actor or SoD rule is invented.

## 15.3 Audit Obligations

Tenant, institution, Workspace purpose, referenced Roles, mappings, policy validation, approval outcome, availability, actor, time and correlation must be auditable without recording new access rights.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- Tenant and institution scope were validated.
- The Workspace purpose was defined.
- Existing Roles were mapped under administrative policy.
- The mapping was validated and approved.
- The Workspace became resolvable to authorized users.
- No Role, Permission or Authorization Decision was changed.
- Governance evidence was preserved.

These criteria cannot be validated canonically because L2-A04 does not define the Workspace workflow, states, commands, approval actors, events or publishers. The scenario remains `BLOCKED`.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Precise Scenario Impact | Required Resolution |
|---|---|---|---|---|
| A050-BLOCKER-01 | Blocking | L2-A04 defines Workspace principles and resolution inputs but no definition, mapping or activation workflow, commands or lifecycle states. | The exact progression to governed resolvable availability cannot be specified. | Define the canonical Workspace and Workspace Mapping workflows, commands and states. |
| A050-BLOCKER-02 | Blocking | No canonical Workspace creation, mapping approval or availability events and publishers are defined. | Completion cannot produce an authoritative published fact. | Define exact events, conditions and sole publishers. |
| A050-BLOCKER-03 | Blocking | L2-A04 does not name the Workspace requestor, configuration authority, approver or activation authority. | Business responsibility and approval cannot be allocated without invention. | Define the authoritative actors and approval boundary. |

No unsupported actor, state, command, event, publisher, approval rule or implementation mechanism is assumed.
