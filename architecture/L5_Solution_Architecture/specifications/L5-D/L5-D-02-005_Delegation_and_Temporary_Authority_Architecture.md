# Delegation & Temporary Authority Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-02-005`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-02 — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD Architecture  
**Canonical Technical Responsibility:** Technical realization of approved delegated and time-bounded authority without creating permanent authorization state.  
**Technical Architecture Owner:** L5-D-02 — Authorization Architecture  
**Assigned Author:** Devang  
**Version:** `1.0.1`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-02-001_Authorization_Architecture_Baseline.md` v1.2.1 Architecture Approved; `L5-D-02-002_Permission_and_Capability_Model_Architecture.md` v1.0.1 Architecture Approved; `L5-D-02-003_Hierarchical_Scope_and_Resource_Context_Architecture.md` v1.0.2 Architecture Approved; `L5-D-02-004_Policy_Decision_and_Enforcement_Architecture.md` v1.2.1 Architecture Approved; `L5-D-01-001` through `L5-D-01-004`; `L5-A-04-001`; `L5-A-04-002`; `L5-D-03-001`; `L4-A-011_Responsibility_Delegation_to_Automatic_Authority_Reversion.md` v1.0.0 Draft; `L4-A-012_Temporary_Access_Request_to_Governed_Access_Expiry.md` v1.0.0 Draft; `L4-A-010`; `L4-A-013`; `L4-A-051`; `L4-A-052`; `L2-A04_Authorization_and_Policy_Workflows.md` v1.1 Locked; `ADR-PLAT-AUTH-001`; `ADR-L5-0001`; `ADR-L5-0004`.  
**Referenced By:** `L5-D-02-004`; `L5-D-02-006`; `L5-D-02-007`; workflow/orchestration; protected business services; audit/evidence; background processing; client/admin experiences that request or display temporary authority.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001` constrains authorization subject ownership; `ADR-L5-0001` constrains trust/fail-closed behavior; `ADR-L5-0004` constrains Tenant/Institution scope. No D02-005-specific product, scheduler, workflow, policy-engine, notification, cache, or storage ADR is selected here.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines how SutraERP technically realizes **temporary authorization authority** after the upstream Authorization & Policy domain has established the governing facts.

It specializes two distinct patterns:

```text
A. Authorization Delegation
   existing responsibility
          ↓
   approved delegate + scope + period
          ↓
   temporary additional authorization context
          ↓
   automatic reversion on expiry / governed revocation

B. Temporary Access
   exceptional access requirement
          ↓
   approved Permissions + scope + period
          ↓
   temporary additional authorization context
          ↓
   automatic cessation on expiry / governed revocation
```

Both patterns add **temporary evaluation context**.

Neither pattern:

- permanently changes Role Assignment;
- permanently changes Role;
- permanently changes Permission definitions;
- bypasses Authorization Policy;
- bypasses Separation of Duties;
- bypasses Explicit Deny;
- grants cross-Tenant authority;
- automatically produces an `Allow`.

### 2.2 In Scope

- technical representation of Authorization Delegation.
- technical representation of Temporary Access Grant.
- source-authority preservation.
- delegator/delegate attribution.
- Requestor/access-subject distinction.
- canonical lifecycle-state consumption.
- effective-date and expiry-date enforcement.
- activation/grant eligibility gates.
- automatic exclusion after expiry.
- early-revocation exclusion.
- new-approval requirement for Temporary Access extension.
- least-privilege scope/Permission constraints.
- Role/responsibility-derived delegated authority.
- Permission-derived Temporary Access.
- effective-authority projection into D02-004.
- invalidation of stale effective authority.
- overlapping temporary-authority sources.
- audit/evidence handoff.
- retry/idempotency requirements for technical activation/expiry handling.
- long-running workflow/job consequences.
- client/UI visibility boundaries.
- failure and recovery behavior without inventing unsupported business workflows.
- clear separation from organizational Assignment Delegation.

### 2.3 Out of Scope / Non-Goals

This specification does not own:

- canonical Delegation approver identity/role.
- canonical Temporary Access approver identity/role.
- Delegation activation/revocation business authority where upstream is silent.
- Temporary Access grant/revocation business authority where upstream is silent.
- canonical lifecycle command names.
- canonical Delegation or Temporary Access event names/publishers.
- E02 organizational Assignment Delegation.
- Role/Role Assignment lifecycle.
- Permission definition.
- Permission/catalog model.
- Tenant/Institution hierarchy.
- final Authorization Decision.
- policy content.
- SoD policy definition.
- privileged/admin architecture.
- break-glass architecture.
- workflow-engine task assignment.
- notification/email delivery.
- persistence product/schema.
- scheduler/queue product.
- numeric maximum delegation duration or Temporary Access duration.
- numeric timer frequency or revocation SLA.
- universal chained-delegation rule where upstream does not define one.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority Matrix

| Source | Status | Applicable Authority | D02-005 Effect |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | D02-005 owns technical realization of approved delegated/time-bounded authority. | Establishes exact L5 responsibility. |
| D02-001 | Architecture Approved | Delegation/Temporary Access are governed authorization inputs, not permanent role mutation or final decision. | D02-005 supplies effective temporary-authority facts. |
| D02-002 | Architecture Approved | Permissions remain stable business capability atoms; temporary sources do not mutate Permission definitions. | Temporary Access references existing governed Permissions. |
| D02-003 | Architecture Approved | temporary authority is Tenant-rooted and scope-bounded. | Delegation/Grant scope participates in effective-scope intersection. |
| D02-004 | Architecture Approved | final Authorization Decision still evaluates policy, scope, SoD, Deny and freshness. | D02-005 never returns final Allow. |
| L2-A04 §10.5 / Delegation | Locked | Delegation is temporary transfer of business responsibility, policy-controlled, time-bound, auditable and never permanently changes assigned Roles. | Core Delegation semantic. |
| L2-A04 §10.6 / Temporary Access | Locked | Temporary Access is short-lived authorization elevation; every grant includes Reason, Scope, Duration and Approval. | Core Temporary Access semantic. |
| `L4-A-011` | Draft | Canonical Delegation lifecycle `Requested → Approved → Active → Expired/Revoked`; Active contributes only approved delegated scope; expiry/revocation removes it; permanent authorization definitions remain unchanged. | Technical lifecycle realization boundary. |
| `L4-A-012` | Draft | Canonical Temporary Access lifecycle `Requested → Approved → Granted → Expired/Revoked`; Granted contributes only approved Permissions/scope; extension requires new approval; permanent authorization state unchanged. | Technical Grant realization boundary. |
| L4-A-051 | Draft | current Delegations and Temporary Access facts participate in SoD evaluation. | D02-005 must expose current temporary-authority facts to D02-007/D02-004. |
| L4-A-052 | Draft | decisions bind authoritative policy version and history remains reproducible. | Temporary authority provenance must be preserved in decision evidence. |
| `ADR-PLAT-AUTH-001` | Approved | Person is canonical human authorization subject; User Account/session is runtime authentication context. | Delegation/Grant binds canonical subject references, not account ownership. |
| `ADR-L5-0004` | Architecture Approved | Tenant-root scope cannot be widened. | All temporary authority remains Tenant-bound. |

For this controlled Stage-2 validation program, the cited canonical L4 System Behavior Specifications are accepted as behavioral authority even where their own metadata remains `Draft` or `Review`; that metadata status is not an approval blocker for D02-005.

### 3.2 Delegation Invariants from L4-A-011

| ID | Invariant | Technical Effect |
|---|---|---|
| D-C01 | Tenant, delegator, delegate, delegated responsibility, scope, effective date and expiry date must resolve. | Missing mandatory context prevents activation path. |
| D-C02 | Delegation policy, least privilege and SoD are evaluated before approval. | Technical realization consumes approved/gated business outcome. |
| D-C03 | Lifecycle is `Requested → Approved → Active`, then `Expired` or `Revoked`. | Technical state handling must preserve these canonical states. |
| D-C04 | `Approved` contributes no authority before permitted effective date. | Effective-authority projection excludes pre-effective Delegation. |
| D-C05 | `Active` contributes only approved delegated scope. | No role-wide/global expansion beyond approved responsibility/scope. |
| D-C06 | Active Delegation remains subject to Policy, SoD and Explicit Deny. | Not an Allow. |
| D-C07 | `Expired` and `Revoked` contribute no authority. | Revocation/expiry invalidates derived authority. |
| D-C08 | Delegation does not alter Role Assignments, Roles, Permissions or E02 Assignment Delegation. | Temporary layer remains separate. |
| D-C09 | History must preserve delegator, delegate, scope, period, evaluation, approval, activation/use and cessation. | D04 evidence handoff mandatory. |
| D-C10 | Event contracts/publishers are not canonically defined. | D02-005 shall not invent business events. |

### 3.3 Temporary Access Invariants from L4-A-012

| ID | Invariant | Technical Effect |
|---|---|---|
| T-C01 | Tenant, identity context, justification, Permissions, scope and effective period must resolve. | Missing context prevents progression/effective Grant. |
| T-C02 | Temporary Access policy, least privilege, duration, scope and SoD are evaluated before approval. | Technical realization consumes source-governed approval. |
| T-C03 | Lifecycle is `Requested → Approved → Granted`, then `Expired` or `Revoked`. | Canonical Grant states are preserved. |
| T-C04 | `Approved` cannot become effective before effective date or when prerequisites no longer hold. | Grant activation gate must revalidate prerequisites. |
| T-C05 | `Granted` contributes only approved Permissions and scope. | No hidden Role creation or unbounded elevation. |
| T-C06 | Granted access remains subject to Policy, SoD and Explicit Deny. | Not an Allow. |
| T-C07 | `Expired` and `Revoked` contribute no authority. | Derived authority is invalidated. |
| T-C08 | Extension requires a new approval process. | Existing Grant cannot silently move expiry forward. |
| T-C09 | Grant does not permanently modify Role Assignment, Role or Permission definitions. | Temporary layer remains separate. |
| T-C10 | History includes Requestor, access subject where distinct, justification, Permissions, scope, period, approval and cessation. | Full attributable evidence. |
| T-C11 | Canonical events/publishers are undefined. | No invented lifecycle event contract. |

### 3.4 Explicit Non-Assumptions

This specification does not infer:

- that delegator may delegate every responsibility they possess;
- that a delegate may re-delegate;
- that Temporary Access can be self-approved;
- that Requestor and access subject are always the same Person;
- that any specific role is a universal approver;
- that Delegation transfers organizational position/assignment;
- that expiry must be implemented by a cron job;
- that active sessions must be terminated on temporary-authority expiry in every case;
- that a named event is emitted on expiry or revocation.

Where these matters require canonical business authority, they remain governed upstream or explicitly deferred.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 D02-005 Owns

- `DelegationAuthorityContext`.
- `TemporaryAccessAuthorityContext`.
- effective-authority inclusion/exclusion rules.
- temporal eligibility evaluation.
- activation/grant readiness boundary.
- expiry/revocation technical cessation.
- temporary-authority projection invalidation.
- provenance and attribution.
- source-authority non-mutation.
- context supplied to D02-004.
- idempotent technical realization around repeated activation/expiry/revocation processing.
- overlap/conflict handling at technical aggregation boundary.
- failure-safe exclusion when state is unresolved/stale.

### 4.2 D02-005 Does Not Own

- canonical approval decision/actor.
- organizational responsibility creation.
- business justification acceptance.
- canonical commands/events.
- policy definition.
- Role or Permission mutation.
- final Allow/Deny.
- SoD policy.
- break-glass.
- workflow lifecycle.
- session lifecycle.

### 4.3 Dependency Collaboration

| Architecture | Responsibility | D02-005 Use |
|---|---|---|
| D01 | canonical Person/technical actor context | Resolves delegator/delegate/Requestor/access subject/actor. |
| A04 | Tenant/Institution execution scope | Root isolation boundary. |
| D02-002 | Permission/Capability | Temporary Access Permission references; delegated effective capability resolution. |
| D02-003 | scope/resource context | Intersects temporary-authority scope with requested/resource scope. |
| D02-004 | policy decision/enforcement | Consumes current valid temporary authority and returns final decision. |
| D02-007 | SoD/emergency | Evaluates temporary responsibility conflicts. |
| D04 | audit/evidence | Stores attribution/history/provenance. |
| A05/A08 | workflow/background/timer execution | May drive technical time-based work, but does not own authority semantics. |
| C02 | messaging/change propagation | May propagate invalidation facts when canonically defined; no invented business event. |
| B02 | persistence/consistency | Stores/projections under canonical ownership; implements concurrency. |
| E05 | operations | Observes stuck expiry/invalidation without changing authorization state. |

---

## 5. Solution Architecture

### 5.1 Two Distinct Temporary-Authority Models

D02-005 shall not collapse Delegation and Temporary Access into one undifferentiated object.

#### Delegation

Delegation starts from an existing authorization-layer **responsibility** eligible for delegation.

```text
Delegator
   ↓
existing governed responsibility
   ↓
Delegate + approved scope + period
   ↓
Active Delegation
   ↓
additional temporary authorization context
```

#### Temporary Access

Temporary Access starts from an **exceptional approved Permission request**.

```text
Requestor / Access Subject
   ↓
business justification
   ↓
approved Permissions + scope + period
   ↓
Granted Temporary Access
   ↓
additional temporary authorization context
```

The two can share technical infrastructure, but their canonical semantics and lifecycles remain distinct.

### 5.2 Delegation Technical Context

Conceptually:

```text
DelegationAuthorityContext
{
  delegation_reference
  tenant_reference

  delegator_subject_reference
  delegate_subject_reference

  delegated_responsibility_reference
  derived_permission_reference(s) where resolvable

  approved_scope_reference(s)
  effective_from
  expires_at

  lifecycle_state
  approval_reference

  policy / SoD provenance
  source_authority_reference

  freshness / version_reference
  correlation_reference
}
```

This is an architecture model, not a prescribed data schema.

### 5.3 Temporary Access Technical Context

Conceptually:

```text
TemporaryAccessAuthorityContext
{
  temporary_access_reference
  tenant_reference

  requestor_subject_reference
  access_subject_reference

  justification_reference
  approved_permission_reference(s)
  approved_scope_reference(s)

  effective_from
  expires_at

  lifecycle_state
  approval_reference

  policy / SoD provenance
  freshness / version_reference
  correlation_reference
}
```

### 5.4 Identity and Subject Binding

Human authorization references follow upstream canonical identity ownership.

Rules:

- Delegator and delegate resolve to canonical Person/Identity context as defined upstream.
- Requestor and access subject resolve to canonical subject context.
- User Account/session may establish current actor but is not permanent authorization subject ownership.
- If Requestor differs from access subject, both references remain attributable.
- If actor exercising authority differs from subject/source authority, the distinction must remain available to D02-004/D04.
- unresolved identity context prevents effective authority.

### 5.5 Tenant Root

Temporary authority never changes Tenant.

```text
source authority Tenant = temporary authority Tenant = protected resource Tenant
```

Cross-Tenant Delegation/Temporary Access is not inferred.

A matching Role/Permission name in another Tenant does not make the Grant/Delegation portable.

### 5.6 Delegation Lifecycle Technical Realization

Canonical lifecycle:

```text
Requested
   ↓ required approval
Approved
   ↓ effective date + activation prerequisites
Active
  ├── governed expiry → Expired
  └── governed early revocation → Revoked
```

D02-005 preserves the source-defined states exactly.

#### Requested

- contributes no temporary authority;
- mandatory references must be resolvable;
- remains subject to approval process outside D02-005 technical ownership.

#### Approved

- still contributes no authority before permitted effective date;
- technical activation gate may validate that prerequisites remain valid;
- approval itself is not execution permission.

#### Active

- contributes only approved responsibility/scope to later authorization;
- remains subject to Policy, SoD, scope, effective period and Explicit Deny;
- does not alter underlying Role Assignment.

#### Expired / Revoked

- excluded from subsequent effective-authority resolution;
- historical record/evidence preserved;
- permanent authorization context remains unchanged.

### 5.7 Temporary Access Lifecycle Technical Realization

Canonical lifecycle:

```text
Requested
   ↓ required approval
Approved
   ↓ effective date + grant prerequisites
Granted
  ├── governed expiry → Expired
  └── governed early revocation → Revoked
```

#### Requested

- contributes no Permission;
- requires resolvable justification, Permissions, scope and period.

#### Approved

- contributes no Permission before permitted effective date;
- grant prerequisites must still hold before effective Grant.

#### Granted

- approved Permissions/scope become additional authorization context;
- not a final Allow;
- remains subject to policy/SoD/Explicit Deny.

#### Expired / Revoked

- approved temporary Permissions no longer contribute;
- historical approval/use/cessation evidence remains.

### 5.8 Activation / Grant Gate

Technical realization must ensure that an approved record does not become effective merely because the time boundary has arrived if source-required prerequisites no longer hold.

Conceptually:

```text
Approved temporary authority
        ↓
effective date reached?
        ↓
mandatory current prerequisites valid?
        ↓
source-governed activation/grant condition satisfied?
        ↓
Active / Granted
```

If prerequisites cannot be established:

- do not add effective authority;
- do not invent a recovery transition;
- surface the condition to the owning process/operations architecture.

### 5.9 Least-Privilege Delegation

Delegation can contribute no more than the source-governed delegated responsibility and approved scope.

The technical model shall prevent accidental expansion such as:

```text
Delegator has:
  DepartmentHead responsibility for Department CSE

Delegation approves:
  selected delegated responsibility for CSE

System must NOT derive:
  all Delegator Roles
  all Delegator Permissions
  Institution-wide authority
```

unless upstream explicitly approved those exact semantics.

### 5.10 Delegation Source Authority

D02-005 preserves the relationship to the original authority source.

Conceptually:

```text
Permanent Source Authority
        ↓
Delegation
        ↓
temporary derivative authority
```

If source authority ceases to be valid and upstream policy requires Delegation eligibility to depend on source validity, the derived temporary authority must not remain permissive merely because its own expiry time has not arrived.

The exact upstream rule for every such case is not fully defined; D02-005 therefore requires source-validity linkage rather than inventing a universal cancellation transition.

### 5.11 Temporary Access Least Privilege

Temporary Access uses explicit approved Permission references and scope.

It shall not use vague elevation such as:

```text
all_access = true
temporary_admin = unlimited
same_as_superuser
```

unless upstream explicitly defines a governed capability with those semantics.

Each approved Permission remains a normal D02-002 capability input and is evaluated by D02-004.

### 5.12 Required Temporary Access Metadata

Locked L2 requires every Temporary Access Grant to include:

- Reason.
- Scope.
- Duration.
- Approval.

L4 additionally requires:

- Tenant.
- identity context.
- requested Permissions.
- effective date/expiry date.
- policy/SoD validation.
- approval reference.

The technical model shall preserve these as authoritative references rather than reducing the Grant to only a list of Permissions.

### 5.13 Delegation Versus Assignment Delegation

A04 Authorization Delegation is not E02 organizational Assignment Delegation.

D02-005 shall never interpret temporary authorization Delegation as:

- position change;
- reporting-line change;
- acting designation;
- employment Assignment transfer;
- permanent business ownership transfer.

If an organizational delegation creates authorization consequences, E02 remains the source of organizational truth and A04 remains the source of authorization delegation/decision semantics.

### 5.14 Effective Authority Projection

D02-005 exposes a derived projection for D02-004.

Conceptually:

```text
EffectiveTemporaryAuthority
{
  source_type: Delegation | TemporaryAccess
  source_reference
  subject_reference
  actor_reference where applicable
  permission / responsibility_reference(s)
  scope_reference(s)
  effective_from
  expires_at
  lifecycle_state
  source_authority_reference
  provenance_reference
}
```

Only:

```text
Delegation = Active
Temporary Access = Granted
```

may contribute authority.

`Requested`, `Approved`, `Expired`, and `Revoked` contribute no effective authority.

### 5.15 Final Authorization Integration

D02-005 never produces final Allow.

```text
Active Delegation / Granted Temporary Access
                ↓
additional authority input
                ↓
D02-002 Permission Resolution
                ↓
D02-003 Scope / Resource Context
                ↓
D02-007 SoD where applicable
                ↓
D02-004 Policy Decision
                ↓
Allow | Deny
```

### 5.16 Explicit Deny

Explicit Deny remains dominant.

A temporary grant cannot be used to weaken:

- Tenant isolation;
- policy Deny;
- SoD Deny;
- resource-state restriction;
- resource-ownership restriction;
- privileged-access safeguards.

### 5.17 Time Eligibility

Temporal evaluation is mandatory.

A temporary-authority source participates only if:

```text
current governed evaluation time
   >= permitted effective date
and
   < / within governed expiry boundary
and
lifecycle state is effective
```

The exact endpoint inclusivity/clock semantics shall follow canonical upstream/time architecture if defined; D02-005 does not invent time-zone/precision rules.

### 5.18 Automatic Expiry

“Automatic” authority cessation means:

> once the governed expiry condition is reached, expired authority must no longer contribute to subsequent authorization evaluation without requiring a user to manually remove permanent Role/Permission assignments.

It does **not** require one particular scheduler technology.

Implementations may realize expiry using:

- evaluation-time temporal filtering;
- scheduled transition processing;
- durable timers;
- background jobs;
- event-driven time control;
- a combination.

Whatever implementation is selected, no protected request after effective expiry may receive authority solely from the expired source.

### 5.19 Expiry Transition and Effective Authority

The business lifecycle requires transition to `Expired`.

Technical authorization safety shall not depend solely on a delayed asynchronous state-write if the expiry timestamp is already authoritative.

Therefore a compliant resolver must not keep granting from a logically expired source merely because the persistence transition to `Expired` is delayed.

Conceptually:

```text
state = Active / Granted
but expires_at has passed
        ↓
DO NOT contribute authority
        ↓
reconcile lifecycle transition to Expired
```

This avoids stale privilege.

The exact reconciliation mechanism is implementation-specific.

### 5.20 Early Revocation

When governed early revocation occurs:

- authority contribution stops;
- derived caches/projections are invalidated;
- future D02-004 decisions exclude the source;
- historical record is retained;
- permanent Role/Permission state is untouched.

D02-005 does not invent who may revoke.

### 5.21 Revocation Freshness

Revocation is security-sensitive.

The architecture shall ensure stale effective-authority projections do not continue indefinitely after authoritative revocation.

Possible techniques include:

- version checks;
- authoritative resolution;
- invalidation messages;
- cache eviction;
- short bounded reuse;
- re-evaluation at sensitive boundaries.

No numeric revocation propagation SLA or cache TTL is defined.

### 5.22 Temporary Access Extension

L4-A-012 explicitly prohibits silent extension.

Therefore:

```text
existing Grant
   + later desired expiry
   ≠
silent expires_at edit
```

A new approval process is required.

Technical implementations shall not provide a privileged “extend” shortcut that changes expiry while preserving the old approval as if it covered the new period.

### 5.23 Delegation Extension

L4-A-011 does not establish an equivalent explicit extension rule.

D02-005 therefore does not invent one.

If Delegation duration needs extension:

- use source-governed behavior when defined;
- otherwise preserve the gap;
- do not assume Temporary Access extension semantics automatically apply to Delegation.

### 5.24 Overlapping Temporary Access Grants

A subject may have multiple active Grants.

D02-005 maintains each source separately.

Example:

```text
Grant G1: Student.View, Dept CSE
Grant G2: Student.Update, Dept CSE
```

The resolver may present both valid authority sources to D02-004.

It shall not merge their audit identity into one anonymous broad Grant.

### 5.25 Overlapping Delegations

A delegate may receive more than one active Delegation.

Each Delegation retains:

- delegator;
- delegated responsibility;
- scope;
- period;
- approval;
- provenance.

Overlapping Delegations do not create a new permanent Role.

### 5.26 Duplicate Temporary Authority

Duplicate or overlapping authority does not multiply authorization.

If two sources both provide the same Permission within the same scope:

- the Permission remains one capability;
- both provenance sources may be retained for evidence;
- revocation of one source does not remove authority still valid through another independent source;
- D02-004 evaluates the remaining source normally.

### 5.27 Authority Union and Scope

Temporary authority from separate sources is not blindly flattened into an unrestricted union.

Each source remains scoped.

Conceptually:

```text
Source S1 → Permission P → Scope A
Source S2 → Permission P → Scope B
```

This means P may be effective in A and B through separate sources.

It does not mean:

```text
P → every scope between/above A and B
```

### 5.28 Chained Delegation

Whether a delegate may re-delegate delegated authority is not canonically defined by the current source.

Therefore:

- D02-005 does not enable chained delegation by architecture assumption;
- it preserves source authority lineage if such capability is later governed;
- it never lets a chain widen original responsibility/scope/duration;
- an implementation requiring chained delegation needs explicit upstream authorization-policy authority.

### 5.29 Self-Delegation / Self-Grant

The current sources do not define universal rules for:

- delegator = delegate;
- Requestor = access subject;
- Requestor self-approval.

D02-005 does not invent blanket acceptance or rejection beyond existing SoD/approval/policy rules.

The owning policy layer must resolve such cases.

### 5.30 SoD Integration

Temporary authority can create new conflicts.

D02-005 exposes all current temporary responsibility/Permission sources to D02-007.

SoD evaluation shall consider:

- base Role Assignments;
- Active Delegations;
- Granted Temporary Access;
- applicable privileged/emergency context.

No temporary source is invisible merely because it is short-lived.

### 5.31 Maker-Checker Context

Where a business process requires different actors/responsibilities:

- Delegation must not erase actor attribution;
- Temporary Access must not erase original Requestor/access-subject distinction;
- D02-007/D02-004 evaluate maker-checker constraints using actual effective responsibility facts.

Temporary authority is not a generic way to collapse maker/checker separation.

### 5.32 Workflow Context

A05 may orchestrate:

- request;
- approval;
- activation/grant;
- review;
- expiry/revocation operational follow-up.

But A05 does not own authorization semantics.

D02-005 may consume workflow outcome references without turning workflow status into authorization truth unless upstream explicitly defines it.

### 5.33 Long-Running Workflows

If a workflow starts while temporary authority is valid and continues after expiry/revocation:

- later protected actions re-evaluate current authority where D02-004 requires authorization;
- the original earlier Allow does not automatically extend the temporary authority lifetime;
- workflow runtime does not freeze temporary permissions indefinitely.

### 5.34 Background / Deferred Work

A job scheduled while temporary authority is valid does not automatically inherit that authority at later execution.

At execution time the system must use the governing authorization model:

- attributable actor/workload identity;
- current valid temporary-authority basis if applicable;
- Tenant/scope;
- current policy.

Exact delegated execution semantics are upstream/process-specific.

### 5.35 Session Interaction

Authentication Session lifetime and temporary-authority lifetime are distinct.

An existing valid session does not preserve an expired/revoked Delegation/Grant.

At later protected operation evaluation:

```text
Session still valid
Temporary authority expired
        ↓
temporary authority excluded
```

Whether expiry/revocation also requires session termination is not universally defined and remains D01/D02 policy integration, not an assumption here.

### 5.36 Client / UI Behavior

UI may display:

- active Delegations;
- Temporary Access status;
- effective period;
- requested/approved scope;
- upcoming expiry;
- reason/justification where authorized.

But:

- UI status does not establish authority;
- hidden expiry UI does not revoke authority;
- visible “Granted” label does not replace server-side temporal/policy evaluation;
- client timer is not authoritative expiry enforcement.

### 5.37 Policy-Change Interaction

A temporary authority may have been approved under an earlier policy version.

Future protected requests still use the authoritative current policy version as required by D02-004/L4-A-052.

Therefore:

```text
Grant approved yesterday
policy tightened today
request today
        ↓
current authorization policy evaluated
```

unless upstream policy explicitly defines another governed version-binding rule.

Historical approval evidence remains tied to its actual governance context.

### 5.38 Scope-Change Interaction

If underlying organizational/resource scope changes while temporary authority remains time-valid:

- D02-003 supplies current authoritative scope context;
- temporary source does not automatically migrate to broader scope;
- D02-004 evaluates compatibility;
- stale scope projections cannot preserve overbroad authority.

### 5.39 Source Permission / Role Change

If a Permission definition or Role mapping changes while Delegation/Grant remains time-valid:

- historical approval provenance remains;
- current protected request uses current authoritative D02-002/D02-004 semantics unless upstream explicitly freezes a definition/version;
- a deleted/invalid Permission reference cannot be treated as a wildcard;
- implementation must avoid silently reinterpreting old identifiers.

### 5.40 Cessation Barrier

A temporary authority source is considered technically safe for cessation only when:

- it no longer contributes to effective authority;
- derived caches/projections are invalidated or provably non-permissive;
- future decisions no longer rely on it;
- historical evidence remains intact.

A delayed notification is not itself a reason to keep authority active.

### 5.41 Idempotency

Repeated technical processing of the same effective outcome must not multiply authority.

Examples:

- repeated activation handling for same Delegation;
- repeated Grant realization;
- repeated expiry reconciliation;
- repeated revocation invalidation.

Requirements:

- same source reference resolves to one canonical temporary-authority source;
- duplicate processing does not create duplicate Permissions;
- terminal-state reprocessing remains non-permissive;
- history can record attempts without fabricating multiple business transitions.

### 5.42 Concurrency

Potential races include:

```text
expiry vs protected request
revocation vs protected request
grant activation vs policy change
revocation vs duplicate activation retry
```

D02-005 requires authoritative state/time/version checks sufficient to prevent stale privilege.

Exact locking/transaction/optimistic concurrency mechanics belong B02/D02-004 implementation.

### 5.43 Time Source

Temporary authority depends on effective time.

The implementation must use a governed server/platform time basis appropriate for authorization.

Client device time shall not determine:

- activation;
- grant;
- expiry;
- revocation effectiveness.

Exact clock synchronization technology is outside this specification.

### 5.44 Time Zone / Calendar Semantics

If business approval captures date/time in a local institutional time zone, technical normalization must preserve the approved instant/period semantics.

D02-005 does not invent default time-zone rules where upstream is silent.

### 5.45 Recovery from Missed Expiry Processing

If scheduled expiry processing fails:

- an expired source must still be excluded at authorization resolution based on governed expiry;
- operations tooling may reconcile persisted lifecycle state;
- no manual Role removal should be required;
- no protected operation may rely on “expiry worker has not run yet” as authority.

### 5.46 Recovery from Invalidation Failure

If a cache/message invalidation fails after revocation:

- an enforcement path must have a way to detect stale authority according to D02-004 freshness architecture;
- system must not intentionally favor availability over revoked authority unless explicitly governed;
- operational evidence should identify stale-projection risk.

### 5.47 Persistence Boundary

Canonical temporary-authority records remain A04 Authorization & Policy-owned.

Technical storage may maintain:

- lifecycle state;
- subject references;
- responsibility/Permission references;
- scope;
- period;
- approval reference;
- provenance;
- version/freshness metadata.

D02-005 does not prescribe tables, document models, event stores or indexes.

### 5.48 Derived Indexes and Caches

Implementations may create:

- active Delegation lookup by delegate;
- active Temporary Access lookup by subject;
- Permission→temporary-source index;
- scope→temporary-source index;
- expiry scheduling index.

These are derivative.

They cannot redefine canonical state.

### 5.49 Audit / Evidence

Delegation evidence must support:

- Tenant;
- delegator;
- delegate;
- responsibility;
- scope;
- period;
- policy/SoD evaluation reference;
- approval;
- activation;
- use;
- expiry/revocation;
- return to non-delegated context.

Temporary Access evidence must support:

- Tenant;
- Requestor;
- access subject where distinct;
- justification;
- Permissions;
- scope;
- period;
- policy/SoD evaluation;
- approval reference;
- grant;
- use;
- expiry/revocation;
- final access cessation.

D04 owns evidence storage and tamper-evidence details.

### 5.50 Privacy

Temporary-authority evidence may contain justification or operational context.

D02-005 requires:

- only necessary authorization data be propagated to decision/enforcement paths;
- detailed justification need not be exposed to every downstream caller;
- secrets/credentials are never part of Delegation/Grant authority context;
- D04/B05 governance applies to retained evidence.

### 5.51 Operational Observability

Operational telemetry may include:

- Approved records waiting for effective time.
- Active/Granted temporary sources.
- sources approaching expiry.
- expired sources not yet reconciled to terminal state.
- revocation invalidation failures.
- stale cache/version detections.
- unresolved identity/scope references.
- repeated activation/expiry processing failures.

No thresholds or alert timings are prescribed.

### 5.52 Failure Model

| Failure / Condition | Required Outcome |
|---|---|
| Tenant unresolved/mismatched | Temporary authority not effective. |
| delegator/delegate unresolved | Delegation not effective. |
| Requestor/access subject unresolved | Temporary Access not effective. |
| responsibility/Permission unresolved | No effective temporary authority. |
| scope invalid/ambiguous | No effective temporary authority. |
| effective period invalid | No effective temporary authority. |
| required approval absent | Remain outside Active/Granted. |
| approval exists but effective date not reached | No authority contribution. |
| prerequisites no longer hold at activation/grant | No authority contribution. |
| source is Expired/Revoked | Exclude from evaluation. |
| expiry processor delayed | Expiry time still prevents authority contribution. |
| cache invalidation uncertain after revocation | Revalidate/fail closed according to D02-004. |
| policy/SoD produces Deny | Final result remains Deny. |
| attempted silent Temporary Access extension | Reject; require new approval process. |
| event contract not defined | Do not invent event; use governed technical reconciliation/contract only. |
| exact approver/revoker undefined | Do not invent actor/authority. |
| duplicate processing | Idempotent; no duplicate authority. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Rationale / Basis | ADR |
|---|---|---|---|
| AD-001 | Delegation and Temporary Access remain separate canonical temporary-authority patterns. | L2-A04 + distinct L4-A-011/A012 lifecycles. | None. |
| AD-002 | Delegation lifecycle is consumed as `Requested → Approved → Active → Expired/Revoked`. | L4-A-011. | None. |
| AD-003 | Temporary Access lifecycle is consumed as `Requested → Approved → Granted → Expired/Revoked`. | L4-A-012. | None. |
| AD-004 | Only Active Delegation and Granted Temporary Access contribute temporary authority. | L4-A-011/A012. | None. |
| AD-005 | Temporary authority never permanently mutates Role Assignment, Role or Permission definitions. | L4-A-011-BH-009; L4-A-012-BH-010. | None. |
| AD-006 | Temporary authority is Tenant-bound, scope-bounded, time-bounded and least-privilege. | L2-A04 + L4-A-011/A012. | `ADR-L5-0004` constraint. |
| AD-007 | Temporary authority is an input to D02-004, never a final Allow. | L4-A-011-BH-006; L4-A-012-BH-006. | None. |
| AD-008 | Expired or Revoked authority is excluded even if a derivative expiry/invalidation worker is delayed. | Security-safe interpretation of automatic cessation. | None. |
| AD-009 | Temporary Access extension requires new approval and cannot silently edit the original expiry. | L4-A-012-BH-009. | None. |
| AD-010 | Delegation is not E02 organizational Assignment Delegation. | L4-A-011 boundary. | None. |
| AD-011 | Temporary-authority sources retain separate provenance when overlapping; they are not flattened into anonymous broad access. | auditability / least privilege. | None. |
| AD-012 | Chained Delegation is not assumed because upstream does not canonically define it. | zero-guessing. | None. |
| AD-013 | No canonical Delegation/Temporary Access lifecycle event names or publishers are invented. | L4-A-011/A012 event gaps. | None. |
| AD-014 | No workflow engine, scheduler, queue, cache, policy product, timer mechanism, database, session-revocation mechanism or numeric duration is selected here. | L5 technology governance. | Future ADR only if material. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Privilege persistence | Expiry/revocation removes temporary authority without Role cleanup. | D02-005/D02-004 |
| Over-delegation | Delegated authority cannot exceed approved responsibility/scope. | D02-005/003 |
| Temporary over-privilege | Temporary Access uses explicit Permission + scope + period. | D02-002/003/005 |
| SoD | Temporary responsibility sources remain visible to conflict evaluation. | D02-007 |
| Tenant isolation | Temporary authority cannot move across Tenant. | A04 + D02 |
| Stale caches | Revoked/expired authority must be invalidated/revalidated. | D02-004/B02/C02 |
| Session persistence | Valid session does not preserve expired authority. | D01 + D02 |
| Workflow longevity | Later protected workflow steps re-evaluate current authority. | A05 + D02-004 |
| Auditability | Full source, subject, scope, period, approval and cessation history. | D04 |
| Privacy | Justification is propagated only where needed. | B05/D04 |
| Availability | Missed expiry jobs cannot extend authority. | A08/E01 + D02 |
| Operational recovery | State/projection reconciliation can occur without restoring expired privilege. | E05/B02 |

No numeric duration, expiry lead time, revocation SLA, timer interval, cache TTL or retry count is defined.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Conformance Requirement | Basis |
|---|---|---|
| CR-001 | Authorization Delegation shall preserve the canonical lifecycle `Requested → Approved → Active → Expired/Revoked`. | L4-A-011. |
| CR-002 | Temporary Access shall preserve the canonical lifecycle `Requested → Approved → Granted → Expired/Revoked`. | L4-A-012. |
| CR-003 | Requested/Approved Delegation shall not contribute authority before Active. | L4-A-011-BH-004/BH-005. |
| CR-004 | Requested/Approved Temporary Access shall not contribute Permissions before Granted. | L4-A-012-BH-004/BH-005. |
| CR-005 | Delegation creation/effectiveness shall require resolvable Tenant, delegator, delegate, responsibility, scope and period. | L4-A-011-BH-001. |
| CR-006 | Temporary Access progression/effectiveness shall require resolvable Tenant, identity, justification, Permissions, scope and period. | L4-A-012-BH-001. |
| CR-007 | Temporary authority shall remain subject to least privilege, Authorization Policy, SoD and Explicit Deny. | L4-A-011-BH-002/BH-006; A012-BH-002/BH-006. |
| CR-008 | Active Delegation shall contribute only the approved delegated responsibility/scope. | L4-A-011-BH-005. |
| CR-009 | Granted Temporary Access shall contribute only approved Permissions/scope. | L4-A-012-BH-005. |
| CR-010 | Expired Delegation shall be excluded from later authorization evaluation. | L4-A-011-BH-007. |
| CR-011 | Revoked Delegation shall be excluded from later authorization evaluation. | L4-A-011-BH-008. |
| CR-012 | Expired Temporary Access shall be excluded from later authorization evaluation. | L4-A-012-BH-007. |
| CR-013 | Revoked Temporary Access shall be excluded from later authorization evaluation. | L4-A-012-BH-008. |
| CR-014 | Delegation shall not modify permanent Role Assignment, Role, Permission or E02 Assignment Delegation state. | L4-A-011-BH-009. |
| CR-015 | Temporary Access shall not permanently modify Role Assignment, Role or Permission definitions. | L4-A-012-BH-010. |
| CR-016 | Temporary Access expiry shall not be silently extended; a new approval process is required. | L4-A-012-BH-009. |
| CR-017 | D02-005 shall not infer a Delegation extension rule from the Temporary Access rule. | zero-guessing. |
| CR-018 | Temporary authority shall remain within one Tenant and shall never widen A04 scope. | `ADR-L5-0004`. |
| CR-019 | Temporary-authority scope shall remain attached to the authority source through D02-004 evaluation. | D02-003. |
| CR-020 | Active/Granted authority shall not itself be treated as final Allow. | D02-004; L4-A-011/A012. |
| CR-021 | Existing Authentication Session validity shall not preserve expired/revoked temporary authority. | D01/D02 separation. |
| CR-022 | Missed/delayed expiry processing shall not allow authority beyond governed expiry. | AD-008. |
| CR-023 | Revocation shall invalidate or cause revalidation of derived effective-authority projections/caches. | freshness/security. |
| CR-024 | Duplicate activation/grant/expiry/revocation processing shall not create duplicate authority. | idempotency. |
| CR-025 | Multiple temporary-authority sources shall retain separate provenance. | AD-011. |
| CR-026 | Revoking one authority source shall not remove equivalent authority still valid through a distinct independent source. | provenance correctness. |
| CR-027 | Separate scoped sources shall not be flattened into broader implied organizational scope. | least privilege. |
| CR-028 | Chained Delegation shall not be enabled solely by D02-005 assumption. | AD-012. |
| CR-029 | Delegation shall remain distinct from organizational Assignment Delegation. | L4-A-011 boundary. |
| CR-030 | Requestor and access subject shall remain separately attributable when upstream facts distinguish them. | L4-A-012-BH-011. |
| CR-031 | Delegator and delegate shall remain attributable for Delegation use/evidence. | L4-A-011-BH-010. |
| CR-032 | SoD evaluation shall be able to see current Active Delegations and Granted Temporary Access. | L4-A-051. |
| CR-033 | Long-running workflow/job execution shall not freeze temporary authority beyond expiry/revocation. | D02-004/A05 boundary. |
| CR-034 | Client timers/status labels shall not be authoritative for activation, grant, expiry or revocation. | server-side authorization. |
| CR-035 | Current authoritative policy shall remain applicable to protected operations using temporary authority. | L4-A-052/D02-004. |
| CR-036 | Policy change shall not rewrite historical approval/use evidence. | historical traceability. |
| CR-037 | D02-005 shall not invent canonical Delegation or Temporary Access event names/publishers. | L4-A-011/A012. |
| CR-038 | D02-005 shall not invent approver, revoker, grantor or activation authority where upstream is silent. | zero-guessing. |
| CR-039 | Temporary-authority evidence shall preserve scope, period, approval and cessation provenance without storing credentials/secrets. | D04/D03. |
| CR-040 | No implementation shall infer a mandatory scheduler, workflow engine, queue, cache, database, policy product, session-control mechanism or numeric duration from this specification. | AD-014. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Source / Gap | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| OD-001 | Applicable L4-A-011/A012 remain `Draft` in their own metadata. | Governance disposition | Controlled Stage-2 validation rule | No | Resolved for D02-005 approval interpretation; source L4 metadata remains unchanged. |
| OD-002 | D02-001 through D02-004 were working authoring dependencies. | Peer stabilization | D02 family reconciliation decision | No | Closed: all four are committed and Architecture Approved; reopen only on material peer change. |
| OD-003 | Canonical Delegation approver is undefined. | Upstream feature-realization gate | L4-A-011 §10 | No for reusable D02 architecture; yes only for the exact approval workflow | Do not invent; source architecture must govern before that workflow is realized. |
| OD-004 | Delegation activation/revocation authority and lifecycle command contracts are undefined. | Upstream feature-realization gate | L4-A-011 §10 | No for reusable D02 architecture; yes for command/API ownership of the affected flow | Preserve lifecycle result; define business authority upstream. |
| OD-005 | Canonical Delegation events/publishers, including expiry event, are undefined. | Upstream event feature gate | L4-A-011 §5/§10 | No for architecture approval; yes only for event integration requiring a canonical event | Do not invent; define through upstream/event governance. |
| OD-006 | Temporary Access approver eligibility/role is undefined. | Upstream feature-realization gate | L4-A-012 §10 | No for reusable D02 architecture; yes only for exact approval workflow implementation | Do not invent; resolve upstream. |
| OD-007 | Temporary Access access-subject definition where distinct from Requestor is incomplete. | Upstream semantic feature gate | L4-A-012 §10 | No for the reference-preserving technical model; yes for an edge-case flow requiring the missing business distinction | Preserve both references where supplied; resolve exact semantics upstream. |
| OD-008 | Temporary Access grant/revocation authorities and lifecycle command contracts are undefined. | Upstream feature-realization gate | L4-A-012 §10 | No for reusable D02 architecture; yes for command/API ownership of the affected flow | Do not invent; resolve upstream. |
| OD-009 | Canonical Temporary Access events/publishers are undefined. | Upstream event feature gate | L4-A-012 §5/§10 | No for architecture approval; yes only for event integration requiring a canonical event | Do not promote audit labels into events; define through governance. |
| OD-010 | Chained/re-delegation semantics are undefined. | Authorization policy feature gate | current upstream | No for reusable D02 architecture; yes if the product requires re-delegation | Require explicit upstream policy/behavior. |
| OD-011 | Delegation extension/renewal semantics are undefined. | Upstream behavior feature gate | L4-A-011 does not state A012-style renewal rule | No for reusable D02 architecture; yes if extension is required | Do not infer; obtain canonical rule. |
| OD-012 | Effect of source Role Assignment/responsibility revocation on an otherwise Active Delegation is not fully specified as lifecycle transition behavior. | Upstream behavior feature gate | current L4 | No for fail-safe effective-authority exclusion; yes for an exact automatic business transition | Link source validity and exclude invalid authority; define canonical transition upstream. |
| OD-013 | Whether expiry/revocation mandates Authentication Session termination for specific high-risk cases. | D01/D02 policy feature gate | not universal upstream | No for temporary-authority exclusion; yes only for a flow requiring session-control policy | Resolve in D01/D02-006/007 or source policy. |
| OD-014 | Exact timer/scheduler/expiry reconciliation mechanism. | Technology / A08 | not selected | No for architecture approval; yes before implementation | Select under canonical L5 technology/ADR process. |
| OD-015 | Exact revocation/expiry cache invalidation mechanism and propagation target. | D02-004/B02/C02 | not selected | No for architecture approval; yes before distributed implementation | Select without weakening fail-closed/freshness requirements. |
| OD-016 | Exact concurrency/transaction semantics for expiry/revocation racing protected mutations. | B02/D02-004 | implementation design | No for architecture approval; yes for high-risk implementation | Define state/version/revalidation strategy. |
| OD-017 | Machine-principal Delegation/Temporary Access semantics. | Upstream feature-realization gate | human A011/A012 semantics dominate current upstream | No for current human Delegation/Temporary Access architecture; yes if temporary machine authority is required | Do not reuse human semantics without explicit governance; `ADR-L5-0009` does not silently create machine Delegation/Temporary Access semantics. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-D-02-005` realizes:

> Technical realization of approved delegated and time-bounded authority without creating permanent authorization state.

### 10.2 Primary Locked L2 Traceability

- Delegation represents temporary transfer of business responsibility.
- Delegation is policy-controlled, time-bound and auditable.
- Delegation never permanently changes assigned Roles.
- Temporary Access is short-lived authorization elevation.
- Every Temporary Access Grant includes Reason, Scope, Duration and Approval.
- Delegation/Temporary Access are authorization-evaluation inputs, not independent final decisions.

### 10.3 L4-A-011 Traceability

- `BH-001`: mandatory Tenant/delegator/delegate/responsibility/scope/period.
- `BH-002`: Delegation policy, least privilege and SoD before approval.
- `BH-003`: `Requested → Approved` requires approval.
- `BH-004`: no authority before effective date.
- `BH-005`: `Approved → Active`, approved delegated scope only.
- `BH-006`: Active remains subject to Policy/SoD/Explicit Deny.
- `BH-007`: `Active → Expired`, remove from evaluation.
- `BH-008`: `Active → Revoked`, remove from evaluation.
- `BH-009`: no permanent authorization/organizational state mutation.
- `BH-010`: complete attributable temporary-authority evidence.

### 10.4 L4-A-012 Traceability

- `BH-001`: mandatory Tenant/identity/justification/Permissions/scope/period.
- `BH-002`: Temporary Access policy, least privilege, duration, scope and SoD.
- `BH-003`: `Requested → Approved` requires approval.
- `BH-004`: no `Granted` before effective date/prerequisite validity.
- `BH-005`: `Approved → Granted`, approved Permissions/scope only.
- `BH-006`: Granted remains subject to Policy/SoD/Explicit Deny.
- `BH-007`: `Granted → Expired`, remove from evaluation.
- `BH-008`: `Granted → Revoked`, remove from evaluation.
- `BH-009`: new approval required instead of silent extension.
- `BH-010`: no permanent Role/Role Assignment/Permission mutation.
- `BH-011`: complete attributable approval/use/cessation evidence.

### 10.5 D02 Family Handoff

```text
D02-001 Authorization Baseline
        ↓
D02-002 Permission & Capability
        ↓
D02-003 Scope & Resource Context
        ↓
D02-005 Delegation & Temporary Authority
        ├───────────────┐
        ↓               ↓
D02-007 SoD       D02-004 Policy Decision
                        ↓
                 final Allow / Deny
```

`D02-006` may additionally constrain privileged temporary authority, but D02-005 does not own privileged/admin semantics.

### 10.6 Architecture Decisions

`AD-001` through `AD-014`.

### 10.7 Conformance Requirements

`CR-001` through `CR-040`.

### 10.8 Open Decisions

`OD-001` through `OD-017`.

### 10.9 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-D/
            └── L5-D-02-005_Delegation_and_Temporary_Authority_Architecture.md
```

No nested `L5-D/L5-D-02/` folder is introduced.


## 2026-09-22 Batch 7B Impact Revalidation Synchronization

Non-material dependency-metadata synchronization only. Current peer versions/statuses are reconciled where explicitly stated; the controlled Stage-2 authorization for cited Draft L4 behavior remains unchanged. No authorization ownership, business authority, policy semantics, conformance requirement, traceability relation state, or Architecture Approved basis changed.
