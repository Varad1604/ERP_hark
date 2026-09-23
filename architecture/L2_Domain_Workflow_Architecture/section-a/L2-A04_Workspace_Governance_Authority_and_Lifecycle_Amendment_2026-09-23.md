# L2-A04 — Workspace Governance Authority and Lifecycle — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-A04 — Authorization & Policy Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-23 |
| Approved Through | Explicit Lead Architect architecture decision recorded 2026-09-23 |
| Authority | SutraERP L5 Post-Validation Upstream Blocker Resolution programme — Option A selected for Workspace governance |

This amendment is part of the canonical L2-A04 architecture. It resolves the Workspace-governance omissions recorded by `L3-A-050` without changing A04 ownership of Workspace, Role, Permission or Authorization Decision and without transferring user-interface ownership into L2.

All unaffected L2-A04 ownership, authorization, policy, Role, Permission, delegation, temporary-access, break-glass, audit and resource-authorization architecture remains unchanged.

---

## 1. Canonical Aggregate and Ownership Decision

`Workspace` is the single governed aggregate for Workspace-definition governance.

Authorization & Policy / L2-A04 remains the canonical owner of:

- Workspace identity;
- Workspace lifecycle;
- Workspace definition/version governance;
- Workspace-to-Role mapping configuration;
- Workspace resolution rules;
- Workspace lifecycle business facts/events.

`Workspace Mapping` is governed configuration belonging to a Workspace definition/version. It is **not** an independently owned aggregate and does not receive an independent lifecycle.

A Workspace Mapping may reference existing governed Roles only. It shall never create, assign, modify, suspend, revoke or otherwise own a Role or Permission.

---

## 2. Canonical Workspace Lifecycle

The canonical Workspace lifecycle is:

```text
Draft
  ↓
Under Review
  ↓
Active
  ↓
Retired
```

with the governed return path:

```text
Under Review → Draft
```

### 2.1 State meanings

| State | Canonical Meaning |
|---|---|
| `Draft` | Workspace purpose, scope and Role mappings are being defined or revised and are not resolvable for governed use. |
| `Under Review` | The Workspace definition/version has been submitted for governance review and remains unavailable for governed resolution until approved. |
| `Active` | The Workspace definition/version has passed governance review and is eligible for governed Workspace resolution. |
| `Retired` | The Workspace definition/version is no longer eligible for new governed resolution but remains historically traceable. |

`Retired` is terminal for that Workspace version.

No other Workspace lifecycle state is canonical unless introduced through a later approved L2-A04 amendment.

---

## 3. Canonical Workspace Versioning and Change Rules

1. A material change to an `Active` Workspace shall not silently rewrite the active definition.
2. A material change creates a governed successor Workspace version beginning in `Draft`.
3. The active predecessor remains authoritative until the successor reaches `Active`.
4. When the governed successor becomes `Active`, the superseded predecessor version is moved to `Retired`.
5. Historical Workspace versions and their mappings remain traceable.
6. Role, Permission and Authorization Decision history is not rewritten by Workspace-version progression.
7. Workspace Mapping changes are governed as part of the applicable Workspace version and do not create an independent Mapping lifecycle.

---

## 4. Canonical Business Actors and Authority

| Business Actor / Authority | Canonical Responsibility | Boundary |
|---|---|---|
| Authorized Administrative Actor | May request a new Workspace, create or revise a `Draft` Workspace definition/version, and configure Workspace-to-Role mappings where permitted by institutional policy. | Abstract administrative actor category; not one universal institutional job title. Does not gain authority to grant Roles or Permissions. |
| Workspace Governance Authority | Makes the governed Workspace review decision. May return `Under Review → Draft`, approve `Under Review → Active`, and authorize `Active → Retired`. Also governs activation of a successor Workspace version. | Abstract business authority mapped locally through institutional policy. Does not own Role/Permission assignment or Authorization Decisions. |
| Authorization & Policy Domain | Owns the Workspace aggregate, Workspace lifecycle, Workspace Mapping configuration, validation rules, resolution semantics and Workspace lifecycle events. | Technical authorization enforcement and Workspace lifecycle ownership remain distinct from UI rendering/composition. |
| Authorized User | May consume or select an available Workspace context after resolution under applicable policy. | Consumer only. Workspace selection never grants access or changes Authorization Decisions. |

### 4.1 Authority mapping

`Workspace Governance Authority` is intentionally institution-independent.

Institutional policy may map this authority to an eligible office, committee, administrator or governance body appropriate to local operating structure. Such local mapping shall not redefine the canonical responsibility established here.

No universal requirement is introduced that the Authorized Administrative Actor and Workspace Governance Authority be different persons. Institution-specific policy may impose separation of duties where required.

---

## 5. Canonical Workspace Governance Operations

The following business operations are canonical:

| Operation | Required Entry Condition | Governed Result |
|---|---|---|
| Create Workspace Draft | Authorized administrative request and resolvable Tenant/institution context | New Workspace definition/version in `Draft`. |
| Submit Workspace for Review | `Draft` definition/version with required purpose, scope, Role mappings and validation evidence | Workspace moves `Draft → Under Review`. |
| Return Workspace for Revision | Workspace is `Under Review` and governance review requires correction | Workspace moves `Under Review → Draft`. |
| Activate Workspace | Workspace is `Under Review`, validations pass and Workspace Governance Authority approves | Workspace moves `Under Review → Active` and becomes eligible for governed resolution. |
| Create Workspace Successor Draft | Material change is required for an `Active` Workspace | Governed successor version begins in `Draft`; predecessor remains `Active` until successor activation. |
| Retire Workspace | `Active` Workspace is no longer eligible for continued use, or a governed successor has become `Active` | Workspace moves `Active → Retired`. |

These are business operations. This amendment does not define APIs, endpoints, software commands, message schemas or persistence mechanisms.

---

## 6. Validation and Mapping Rules

Before `Under Review → Active`, the Workspace definition/version shall satisfy all applicable governance checks:

1. Tenant and institution scope are authoritative and unambiguous.
2. Every referenced Role already exists under L2-A04 governance.
3. Workspace mappings remain within the applicable Tenant/institution scope.
4. Workspace mappings do not create, grant, revoke or mutate Roles or Permissions.
5. Workspace definition or selection cannot change an Authorization Decision.
6. Workspace resolution remains based on existing assigned Roles, governed mappings, institution configuration and administrative policy.
7. Material validation findings block activation.
8. Required governance evidence is attributable and preserved.

An `Active` Workspace is resolvable context only. It remains independent from authorization outcome.

---

## 7. Canonical Workspace Events

The following Workspace lifecycle events are canonical business facts:

| Business Event | Canonical Publisher | Publication Meaning |
|---|---|---|
| `WorkspaceActivated` | Authorization & Policy / L2-A04 | A governed Workspace definition/version has successfully reached `Active` and is eligible for Workspace resolution. |
| `WorkspaceRetired` | Authorization & Policy / L2-A04 | A previously `Active` Workspace definition/version has reached terminal `Retired` and is no longer eligible for new governed resolution. |

Rules:

1. L2-A04 is the sole semantic publisher of both events.
2. Event Infrastructure may distribute these facts but does not become their semantic publisher.
3. No canonical Workspace-created, review-submitted, returned-for-revision or mapping-approved event is required by this amendment.
4. Existing human-readable audit labels such as `Workspace Changed`, `Workspace Switch Requested` and `Workspace Validation Failure` remain operational/audit labels unless separately promoted through architecture governance.
5. Event payloads, transport, topics, queues and implementation protocols remain outside L2.

---

## 8. Workspace Resolution Contract

An `Active` Workspace may be returned as governed experience context when:

- the authenticated subject is resolved through existing A02/A03 ownership;
- applicable assigned Roles are authoritative;
- a current Workspace Mapping relates those Roles to the `Active` Workspace;
- Tenant/institution configuration permits availability;
- applicable administrative policy allows the Workspace context.

Resolution of a Workspace:

- does not assign a Role;
- does not grant or revoke a Permission;
- does not produce a privileged Authorization Decision;
- does not bypass centralized authorization;
- does not transfer UI/navigation/dashboard composition ownership into L2-A04.

---

## 9. Audit and Historical Obligations

The following shall remain attributable and historically traceable:

- Workspace identity and version;
- Tenant/institution scope;
- Workspace purpose;
- Role mappings;
- validation findings;
- administrative actor making/requesting changes;
- Workspace Governance Authority decision;
- lifecycle transitions;
- predecessor/successor version lineage;
- `WorkspaceActivated` and `WorkspaceRetired` publication where applicable.

L2-A07 retains ownership of canonical audit infrastructure. L2-A04 retains ownership of Workspace governance meaning and Workspace lifecycle facts.

---

## 10. Locked Decisions

The following decisions are Locked by this amendment:

1. Workspace is the single governed aggregate for Workspace-definition governance.
2. Workspace Mapping is governed Workspace configuration, not a separate aggregate or lifecycle.
3. Authorization & Policy / L2-A04 owns Workspace identity, lifecycle, mapping configuration, resolution semantics and Workspace lifecycle events.
4. The Workspace lifecycle is exactly `Draft → Under Review → Active → Retired`, with `Under Review → Draft` as the governed return path.
5. Material change to an `Active` Workspace creates a successor version rather than silently rewriting the active version.
6. `Workspace Governance Authority` is the canonical abstract business authority for activation, return-for-revision and retirement decisions.
7. Authorized administrative actors may configure Draft Workspace definitions and mappings under institutional policy but do not thereby gain Role/Permission authority.
8. Only `Active` Workspace versions are eligible for governed resolution.
9. Workspace and Workspace Mapping never grant, revoke or modify Roles or Permissions and never alter Authorization Decisions.
10. `WorkspaceActivated` and `WorkspaceRetired` are canonical events published solely by Authorization & Policy / L2-A04.
11. Workspace rendering, navigation design, dashboards, routes and UI implementation remain outside L2-A04.
12. Downstream L3/L4/L5/L6 architecture shall consume these decisions without introducing alternative Workspace ownership, lifecycle, actors or publishers.

---

## 11. Downstream Effect

This amendment resolves the three blocking omissions recorded by `L3-A-050`:

- Workspace workflow/lifecycle and governed operations;
- Workspace lifecycle events and sole publisher;
- Workspace configuration and governance authority.

The original `L2-A04_Authorization_and_Policy_Workflows.md` and this amendment shall be read together as the canonical L2-A04 architecture.
