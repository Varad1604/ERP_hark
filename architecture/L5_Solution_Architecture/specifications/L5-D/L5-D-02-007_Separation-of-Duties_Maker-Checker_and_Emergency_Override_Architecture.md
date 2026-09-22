# Separation-of-Duties, Maker-Checker & Emergency Override Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-02-007`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-02 — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD Architecture  
**Canonical Technical Responsibility:** Technical enforcement of upstream-defined Separation of Duties, maker-checker and emergency override controls without inventing approvers, authorities or business policy.  
**Technical Architecture Owner:** L5-D-02 — Authorization Architecture  
**Assigned Author:** Devang  
**Version:** `1.0.1`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-02-001_Authorization_Architecture_Baseline.md` v1.2.1 Architecture Approved; `L5-D-02-002_Permission_and_Capability_Model_Architecture.md` v1.0.1 Architecture Approved; `L5-D-02-003_Hierarchical_Scope_and_Resource_Context_Architecture.md` v1.0.2 Architecture Approved; `L5-D-02-004_Policy_Decision_and_Enforcement_Architecture.md` v1.2.1 Architecture Approved; `L5-D-02-005_Delegation_and_Temporary_Authority_Architecture.md` v1.0.1 Architecture Approved; `L5-D-02-006_Privileged_and_Administrative_Access_Architecture.md` v1.2.1 Architecture Approved; `L5-D-01-001` through `L5-D-01-004`; `L5-A-04-001`; `L5-A-04-002`; `L5-D-03-001`; `L4-A-051_Conflicting_Responsibility_Risk_to_Enforced_Separation_of_Duties.md` v1.0.0 Draft; `L4-A-013_Emergency_Access_Request_to_Audited_Emergency_Access_Closure.md` v1.0.0 Draft; `L4-A-011`; `L4-A-012`; `L4-A-052`; `L2-A04_Authorization_and_Policy_Workflows.md` v1.1 Locked; `ADR-PLAT-AUTH-001`; `ADR-L5-0001`; `ADR-L5-0004`.  
**Referenced By:** protected workflows; approval and maker-checker business processes; privileged administration; emergency-access interfaces; policy decision/enforcement; audit/evidence; high-risk transaction processing.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001` constrains canonical subject ownership; `ADR-L5-0001` constrains default-deny protected-boundary behavior; `ADR-L5-0004` constrains Tenant-root scope. No D02-007-specific policy engine, workflow product, approval product, timer, notification, audit product, cache, graph or rule-language ADR is selected here.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical architecture for three related but distinct authorization controls:

1. **Separation of Duties (SoD)** — detects and enforces prohibited combinations of effective responsibilities or execution contexts.
2. **Maker-Checker / Four-Eyes** — enforces source-defined requirements that a protected action must involve appropriately distinct authorized actors/responsibilities.
3. **Emergency Override / Break-Glass** — realizes exceptional, policy-approved, temporary emergency authority that remains scoped, auditable and subject to the normal authorization framework.

These controls converge at the central authorization boundary:

```text
effective Roles / Delegations / Temporary Access / Privilege
                      +
           workflow / resource context
                      +
               SoD configuration
                      +
       maker-checker context where required
                      +
       emergency context where applicable
                      ↓
              D02-004 evaluation
                      ↓
                Allow | Deny
```

The central rule is:

> No SoD, maker-checker or emergency mechanism creates an independent authorization bypass.

### 2.2 In Scope

- technical SoD-policy consumption.
- current effective-responsibility aggregation.
- conflict detection inputs.
- platform-default and permitted institution-override evaluation.
- deterministic SoD configuration validation.
- explicit-Deny interaction.
- maker-checker/four-eyes actor-distinction enforcement.
- same-actor/self-check prevention when upstream policy requires distinct actors.
- process/action/resource context binding.
- delegated/temp/privileged responsibility participation in SoD.
- technical permitted-continuation path when an already-approved policy explicitly allows it.
- final D02-004 handoff.
- emergency-access request admission prerequisites.
- emergency approval-reference consumption.
- temporary emergency authority.
- emergency scope/duration enforcement.
- emergency expiry/revocation exclusion.
- source-required administrator notification handoff.
- mandatory emergency audit-review evidence.
- emergency closure gating.
- evidence/provenance.
- stale-context invalidation.
- failure-safe behavior.
- separation of ordinary privilege from Break-Glass.

### 2.3 Out of Scope / Non-Goals

This specification does not define:

- which concrete business responsibilities conflict.
- a canonical enterprise-wide SoD conflict matrix not already approved upstream.
- maker/checker business role names.
- approver identity or checker authority where source architecture is silent.
- policy author/validator/approver.
- institution-override authority.
- permitted-continuation authority.
- Break-Glass requestor/subject/approver/revoker role names where undefined.
- emergency notification recipient role where undefined.
- emergency audit-review closing authority/criteria where undefined.
- SoD Policy lifecycle states.
- SoD Policy commands/events.
- Break-Glass lifecycle state names.
- Break-Glass commands/events.
- workflow-task business semantics.
- Role/Permission lifecycle.
- Delegation/Temporary Access lifecycle.
- privileged-access baseline.
- authentication/MFA lifecycle.
- audit storage product.
- workflow/rules product.
- numeric approval SLA.
- numeric emergency duration.
- universal two-person rule for every high-risk action.
- universal override capability.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority Matrix

| Source | Status | Applicable Authority | D02-007 Effect |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | D02-007 owns technical enforcement of upstream-defined SoD, maker-checker and emergency override without inventing approvers. | Exact L5 responsibility. |
| D02-001 | Architecture Approved | Central authorization, explicit Deny, least privilege, fail closed. | All D02-007 controls feed the same decision model. |
| D02-002 | Architecture Approved | Permissions represent business capabilities; Roles/responsibilities remain distinct from UI. | SoD evaluates effective responsibilities/capabilities without inventing new ones. |
| D02-003 | Architecture Approved | Tenant/resource/scope context is authoritative. | Conflict and emergency evaluation remain scope-bound. |
| D02-004 | Architecture Approved | Final policy decision/enforcement; no positive prerequisite alone equals Allow. | SoD/maker-checker/emergency are decision inputs. |
| D02-005 | Architecture Approved | Active Delegations and Granted Temporary Access contribute temporary authority. | They must participate in SoD/conflict evaluation. |
| D02-006 | Architecture Approved | High-risk privilege remains centrally authorized; emergency is separate. | Break-Glass is not ordinary admin access. |
| L2-A04 | Locked | Separation of Duties Policy defines configurable Maker-Checker/Four-Eyes rules; Break-Glass is controlled emergency authorization. | Establishes the three control families. |
| L4-A-051 | Draft | SoD centrally governed; platform defaults + only permitted institution overrides; all current responsibility sources participate; conflict Denies by default; permitted continuation still requires full Allow. | Primary SoD technical obligations. |
| L4-A-013 | Draft | Emergency elevation requires context, justification, policy/SoD/Deny evaluation, approval, bounded scope/duration, notification, expiry/revocation removal and mandatory audit review. | Primary emergency obligations. |
| L4-A-011/A012 | Draft | Delegation/Temporary Access are temporary authority sources. | SoD sees them as effective responsibility inputs. |
| L4-A-052 | Draft | Decisions bind exact authoritative policy version; deterministic policy semantics and history. | SoD/emergency decision evidence binds exact policy version. |
| `ADR-L5-0001` | Architecture Approved | Default-deny protected-boundary crossing. | Missing control facts never become permissive. |
| `ADR-L5-0004` | Architecture Approved | Tenant-rooted isolation. | SoD/override cannot bridge Tenants. |

For this controlled Stage-2 validation program, the cited canonical L4 System Behavior Specifications are accepted as behavioral authority even where their own metadata remains `Draft` or `Review`; that metadata status is not an approval blocker for D02-007. This does not alter those L4 files' own status metadata.

### 3.2 SoD Invariants from L4-A-051

| ID | Invariant | Technical Effect |
|---|---|---|
| S-C01 | Tenant, institution, process/action/resource context and conflict definitions must resolve. | Missing required context prevents permissive evaluation. |
| S-C02 | Current Role Assignments, Delegations and Temporary Access participate. | Conflict engine uses effective responsibility facts, not only permanent Roles. |
| S-C03 | Platform defaults apply; institution overrides only where governing policy explicitly allows. | Institution cannot weaken controls arbitrarily. |
| S-C04 | SoD configuration must preserve determinism, Tenant isolation, least privilege and explicit-Deny precedence. | Invalid/ambiguous config is non-authoritative. |
| S-C05 | Required governance approval evidence is consumed without inventing approvers. | Technical layer validates evidence reference, not business authority. |
| S-C06 | No conflict detected is not an Allow. | Evaluation continues. |
| S-C07 | Prohibited conflict produces Deny by default. | Explicit-Deny precedence. |
| S-C08 | Approved permitted continuation still evaluates all remaining controls. | Exception is not bypass. |
| S-C09 | No canonical SoD policy lifecycle/events are defined. | D02-007 does not invent them. |
| S-C10 | Platform/institution policy authorities are unnamed. | Preserve gap. |

### 3.3 Emergency Invariants from L4-A-013

| ID | Invariant | Technical Effect |
|---|---|---|
| E-C01 | Tenant, subject, authenticated context, emergency justification, scope and protected operation must resolve. | No incomplete emergency elevation. |
| E-C02 | Break-Glass policy, least privilege, SoD and Explicit Deny are evaluated before elevation. | Emergency cannot bypass foundational controls. |
| E-C03 | Required emergency approval must exist before temporary elevation. | No self-declared emergency authority. |
| E-C04 | Emergency authority is limited to approved scope and governed duration. | No permanent/global elevation. |
| E-C05 | Roles, Permissions and Role Assignments are not permanently changed. | Emergency context is derivative. |
| E-C06 | Source-required administrator notification must be produced without inventing undefined recipient role. | Notification handoff required. |
| E-C07 | During elevation, normal Authorization Policy, SoD and Explicit Deny still participate. | No second authorization universe. |
| E-C08 | Expiry/revocation removes emergency authority from later evaluation. | Automatic cessation. |
| E-C09 | Mandatory audit review is required before emergency outcome is closed. | Closure gate. |
| E-C10 | Break-Glass lifecycle states/events/actors remain incomplete upstream. | Do not invent. |

### 3.4 Maker-Checker / Four-Eyes Constraint

Locked L2-A04 establishes:

> `Separation of Duties Policy` defines configurable Maker-Checker and Four-Eyes authorization rules.

Therefore D02-007 may define **technical actor-distinction enforcement**, but it shall not invent:

- which process requires maker-checker;
- who qualifies as maker;
- who qualifies as checker;
- whether two actors are sufficient for every scenario;
- whether checker must hold a named Role;
- whether checker may belong to another organization unit;
- which business state transition represents approval.

Those remain source-domain/Authorization Policy decisions.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 D02-007 Owns

- `SoDEvaluationContext`.
- effective-responsibility conflict evaluation.
- maker-checker actor-distinction context.
- checker/maker identity equality comparison.
- source-approved conflict-continuation consumption.
- SoD decision input to D02-004.
- emergency authority context.
- emergency elevation technical admission.
- emergency effective-period/scope enforcement.
- emergency expiry/revocation exclusion.
- mandatory emergency evidence correlation.
- emergency review-completion gate representation.
- stale SoD/emergency authority invalidation.
- fail-closed control behavior.

### 4.2 D02-007 Does Not Own

- business conflict definitions.
- organizational Role definitions.
- business approval/checker responsibility.
- SoD policy business lifecycle.
- emergency approver/revoker.
- notification recipient.
- emergency review authority.
- final authorization decision.
- workflow state.
- permission definition.
- scope hierarchy.
- audit storage.
- notification delivery product.

### 4.3 Collaborating Architecture

| Architecture | Supplies / Owns | D02-007 Use |
|---|---|---|
| D01 | authenticated actor/subject | identity distinction and attribution. |
| A04 | Tenant/Institution context | root isolation. |
| D02-002 | Permission/Role/responsibility resolution | effective capability/responsibility facts. |
| D02-003 | scope/resource/process context | conflict/emergency scope. |
| D02-004 | policy decision/enforcement | final Allow/Deny. |
| D02-005 | Delegation/Temporary Access | effective temporary responsibilities included in SoD. |
| D02-006 | privileged access | high-risk operation context; emergency remains separate. |
| A05 | workflow/process context | maker/checker stage/task references where upstream defines them. |
| D04 | audit/evidence | SoD/emergency evidence storage. |
| C02/H02 | event/notification infrastructure | source-required notification transport only when canonical contract exists. |
| B02 | consistency/state | current responsibility/resource context and race-safe execution. |

---

## 5. Solution Architecture

### 5.1 Three-Control Model

```text
Separation of Duties
    detects prohibited effective-responsibility combinations

Maker-Checker / Four-Eyes
    requires appropriately distinct actors/responsibilities for
    source-defined protected business action sequence

Emergency Override / Break-Glass
    provides exceptional, approved, temporary authority while
    preserving normal policy/SoD/Deny and mandatory review
```

These controls may interact, but remain semantically distinct.

### 5.2 Central Evaluation Integration

```text
Authenticated actor / subject
        ↓
Permission / capability
        ↓
Tenant / scope / resource / process context
        ↓
Effective Roles
+ Delegations
+ Temporary Access
+ Privileged context
        ↓
SoD / Maker-Checker evaluation
        ↓
Emergency context where applicable
        ↓
D02-004 policy evaluation
        ↓
Explicit Deny precedence
        ↓
Allow | Deny
```

### 5.3 SoD Evaluation Context

Conceptually:

```text
SoDEvaluationContext
{
  tenant_reference
  institution_reference
  process_reference
  protected_action_reference
  resource_reference

  subject_reference
  actor_reference

  effective_role_assignment_references[]
  active_delegation_references[]
  granted_temporary_access_references[]
  privileged_authority_references[]

  maker_checker_context
  applicable_sod_policy_reference
  applicable_configuration_reference
  policy_version_reference

  effective_time
  correlation_reference
}
```

This is not a prescribed schema.

### 5.4 Effective Responsibility Set

The effective responsibility set is derived from current authoritative sources.

It may include:

- active Role Assignments;
- Active Delegations;
- Granted Temporary Access;
- privileged authority sources;
- emergency authority where applicable and source policy requires it to participate.

It shall not include:

- expired/revoked temporary authority;
- requested/approved-but-not-effective temporary authority;
- UI Workspace;
- stale Role Assignment;
- unverified client claims.

### 5.5 Conflict Definition Ownership

D02-007 never creates organizational responsibility definitions merely to implement a conflict rule.

Conflict definitions reference existing upstream responsibility concepts.

Conceptually:

```text
Conflict Definition
    responsibility A
    responsibility B
    process / action / resource scope
    governed policy conditions
```

The technical layer can evaluate the relation.

It cannot decide that two responsibilities are conflicting without source authority.

### 5.6 Platform Defaults and Institution Overrides

Conceptual policy composition:

```text
Platform SoD Defaults
        ↓
permitted institution-specific configuration
        ↓
validated governed SoD configuration
```

An institution override may only apply if governing policy allows the override scope.

The implementation shall reject configuration that:

- weakens explicit-Deny precedence;
- violates Tenant isolation;
- creates ambiguous ordering;
- references invalid Roles/Permissions/scopes;
- exceeds allowed override boundaries.

### 5.7 Deterministic Conflict Evaluation

Given the same materially relevant:

- effective responsibility set;
- process/action/resource context;
- Tenant/institution configuration;
- policy version;

the conflict evaluation shall be deterministic.

Rule evaluation shall not depend on:

- UI order;
- dictionary/map iteration order;
- last-loaded config;
- client order;
- unspecified precedence.

### 5.8 No-Conflict Result

```text
No prohibited conflict detected
        ↓
continue D02-004 evaluation
```

It does not mean:

```text
Allow
```

A request may still be denied because of:

- missing Permission;
- scope mismatch;
- resource state;
- explicit Deny;
- Temporary Access expiry;
- privileged-access control;
- other policy.

### 5.9 Prohibited Conflict

When an applicable prohibited conflict exists:

```text
Conflict Detected
        ↓
Deny
```

unless:

- an already-approved policy explicitly defines a permitted continuation case.

D02-007 does not invent such a continuation case.

### 5.10 Permitted Continuation

A permitted continuation is not a conflict deletion.

The evidence should retain:

- conflict detected;
- policy reference;
- continuation basis;
- applicable conditions.

Then:

```text
approved continuation basis
        ↓
evaluate remaining authorization controls
        ↓
Allow only if complete D02-004 decision = Allow
```

### 5.11 Maker-Checker Technical Pattern

Maker-checker is modeled as a policy-driven actor-separation condition.

Conceptually:

```text
Maker Action
   actor = A
   process/resource = R
        ↓
Checker Action
   actor = B
   same governed process/resource context
        ↓
policy requires A ≠ B
        ↓
checker also requires its own valid authority
```

D02-007 enforces actor distinctness only where upstream policy requires it.

### 5.12 Four-Eyes Technical Pattern

Four-Eyes is represented as:

> a protected action/process condition requiring independent participation by at least two appropriately authorized actors according to upstream policy.

D02-007 does not define a universal “exactly two” rule.

The source may require:

- two actors;
- more than two actors;
- role separation;
- organizational separation;
- sequential approval;
- concurrent approval.

D02-007 consumes only the source-defined semantics.

### 5.13 Maker Identity

A Maker is the actual attributable actor who performed the source-defined maker operation.

The Maker shall not be inferred from:

- record owner only;
- current session user at checker time;
- workflow assignee if different from executing actor;
- UI selection.

Actual execution attribution is required.

### 5.14 Checker Identity

A Checker is the actual attributable actor executing the source-defined checker operation.

Checker authorization is independently evaluated.

Possession of “checker” workflow assignment alone is not enough.

### 5.15 Same-Actor Prevention

Where policy requires distinct actors:

```text
maker_actor_reference == checker_actor_reference
        ↓
Deny / prevent checker completion
```

The check is performed against canonical actor identity, not display name.

### 5.16 Actor vs Subject

Where actual actor and authorization subject can differ:

- actual actor remains attributable;
- subject remains attributable;
- maker-checker identity comparison uses the policy-defined identity dimension;
- D02-007 does not assume `actor == subject`.

For normal human execution, canonical Person identity is the primary human authorization subject.

### 5.17 Delegation and Maker-Checker

Delegation complicates checker independence.

A delegated responsibility does not erase:

- original delegator;
- actual delegate actor;
- delegated source.

D02-007 evaluates the source-defined conflict rules using full provenance.

It does not automatically decide:

- delegate is equivalent to delegator for all conflicts;
- delegate is always independent from delegator.

Those semantics need source policy.

### 5.18 Temporary Access and Maker-Checker

Temporary Access may make a Permission available to a checker/maker.

It does not exempt the actor from SoD.

A temporarily elevated actor still participates in conflict evaluation.

### 5.19 Privileged Access and Maker-Checker

Administrative privilege does not exempt an actor from maker-checker.

A high-risk administrator may still require:

- separate checker;
- source-governed approval;
- conflict-free responsibility set.

### 5.20 Workflow Context

A05 owns workflow execution.

D02-007 may consume:

- workflow instance reference;
- task/stage reference;
- prior actor reference;
- source-defined maker/checker marker;
- business resource/process reference.

It does not invent workflow states.

### 5.21 Business-State Context

Maker-checker may depend on the resource being in a source-defined state.

D02-003 resolves state context.

D02-007 does not own the resource lifecycle.

### 5.22 Repeated Approval Attempts

If a same-actor checker attempt is denied:

- no checker authority is created;
- workflow/business state shall not be advanced through D02;
- evidence may record the denied attempt;
- a later valid independent actor can be evaluated according to source workflow.

### 5.23 Multiple Checkers

Where upstream allows/requires multiple checker actors:

- each actor is independently authorized;
- distinctness requirements are evaluated according to source policy;
- D02-007 does not collapse multiple approvals into one shared identity.

### 5.24 SoD Configuration Reference

A protected decision should retain:

- SoD policy/configuration version/reference;
- platform default reference;
- institution override reference where applied;
- conflict-definition references;
- continuation basis where used.

This supports historical reconstruction.

### 5.25 SoD Configuration Change

Future protected requests use the authoritative current SoD configuration/policy basis.

Historical Authorization Decisions retain the prior evaluated references.

D02-007 does not rewrite historic conflict results after configuration changes.

### 5.26 SoD Cache / Projection

A technical engine may cache:

- conflict definitions;
- effective-responsibility projections;
- maker/checker context;
- institution overrides.

Caches are derivative.

Stale cache must not permit a prohibited conflict after authoritative change.

### 5.27 Emergency Override Position

Break-Glass is an exceptional path:

```text
ordinary authority insufficient
        +
exceptional institutional need
        +
governed emergency policy
        +
required approval
        ↓
temporary emergency context
        ↓
normal centralized authorization
```

Break-Glass is not:

- a hidden superuser;
- a permanent Role;
- a permission to bypass all Deny rules;
- a generic administrator convenience.

### 5.28 Emergency Request Context

Conceptually:

```text
EmergencyAccessContext
{
  tenant_reference
  subject_reference
  actor_reference
  authenticated_context_reference

  emergency_justification_reference
  requested_scope_reference
  protected_operation_reference

  emergency_approval_reference
  effective_period_reference

  applicable_break_glass_policy_reference
  sod_evaluation_reference
  explicit_deny_context_reference

  notification_reference
  audit_review_reference
  correlation_reference
}
```

### 5.29 Emergency Admission Gate

Before elevation:

1. resolve Tenant;
2. resolve subject/actor/authentication context;
3. resolve exceptional justification;
4. resolve protected operation;
5. resolve requested scope;
6. evaluate Break-Glass policy;
7. evaluate least privilege;
8. evaluate SoD;
9. evaluate Explicit Deny;
10. verify required emergency approval.

If any mandatory prerequisite is unresolved or prohibited, no elevation.

### 5.30 Emergency Approval

D02-007 consumes the required approval outcome/reference.

It does not define:

- approver role;
- approval command;
- approval workflow;
- approval event.

### 5.31 Emergency Authority Scope

Approved emergency authority is limited to:

- approved Tenant;
- approved scope;
- approved protected operation/capability;
- governed duration;
- approved subject/actor context.

It shall not expand to unrelated resources/actions.

### 5.32 Emergency Least Privilege

Break-Glass elevation is not equivalent to “all admin rights.”

The minimum approved emergency capability/scope is supplied to D02-004.

### 5.33 Emergency Authority Is Temporary

Emergency access never mutates:

- Role definition;
- Permission definition;
- Role Assignment.

It is an additional temporary authorization context.

### 5.34 Emergency Authorization During Elevation

While emergency authority is available:

```text
Break-Glass context
        +
normal Permission / scope
        +
Authorization Policy
        +
SoD
        +
Explicit Deny
        ↓
D02-004 final decision
```

The emergency context may satisfy source-defined exceptional conditions, but it never disables the central decision architecture.

### 5.35 Emergency Explicit Deny

If source policy defines an Explicit Deny applicable even in emergency context, the Deny remains authoritative.

D02-007 does not introduce a universal “Break-Glass overrides Deny” rule.

### 5.36 Emergency SoD

Emergency context still participates in SoD according to L4-A-013.

If source policy allows a specific emergency continuation, it must be explicitly governed.

No implicit exception is created.

### 5.37 Emergency Notification

When elevation becomes available, source-required administrator notification must be produced.

D02-007 supplies a notification handoff containing only approved, necessary context.

It shall not invent the recipient role.

If recipient authority remains unresolved upstream:

- preserve evidence that notification is required/unresolved;
- do not fabricate recipient identity.

### 5.38 Notification Failure

Upstream requires notification.

D02-007 shall preserve notification attempt/status evidence.

Whether notification-delivery failure immediately revokes elevation is not universally defined in current L4 and is not invented here.

If policy later defines notification as a precondition, D02-004 enforces it.

### 5.39 Emergency Expiry

At governed expiry:

- emergency authority is excluded from future authorization evaluation;
- no permanent Role cleanup is needed;
- stale cache/session context cannot preserve it;
- lifecycle/evidence reconciliation may continue.

### 5.40 Emergency Early Revocation

When governed revocation occurs:

- emergency authority stops contributing;
- future decisions exclude it;
- stale projections are invalidated/revalidated;
- history remains.

D02-007 does not invent revoker authority.

### 5.41 Emergency Session Boundary

Locked L2 refers to a `Break-Glass Session`, but L4 explicitly leaves the canonical Break-Glass state model incomplete.

D02-007 therefore uses **Emergency Access Context** technically without inventing unsupported lifecycle state names.

It does not conflate Break-Glass Session with D01 Authentication Session.

### 5.42 Authentication Session Interaction

A normal authenticated session can remain technically valid while emergency authority expires.

At the next protected operation:

```text
authentication session valid
break-glass authority expired
        ↓
no emergency authority
```

Whether session termination is additionally required is source-policy/D01 behavior, not universally assumed.

### 5.43 Emergency Closure

Emergency access outcome is not complete merely when elevation ends.

L4-A-013 requires mandatory audit review.

Conceptually:

```text
elevation ceased
        ↓
evidence complete
        ↓
mandatory audit review
        ↓
source-governed closure
```

### 5.44 Mandatory Audit Review

The review must have access to preserved evidence including:

- justification;
- scope;
- policy/SoD evaluation;
- approval;
- elevation period;
- protected Authorization Decisions;
- notification;
- expiry/revocation;
- actor/time/correlation.

D02-007 does not define the reviewer or closing authority.

### 5.45 Closure Gate

If mandatory audit review is incomplete:

- the emergency-access outcome shall not be treated as governed closed;
- this does not imply emergency authority remains active;
- authorization cessation and governance closure are separate facts.

This distinction is essential.

### 5.46 Emergency Reuse

A previous emergency approval/elevation shall not be reused as standing future authority unless upstream explicitly defines such behavior.

New exceptional circumstances require the source-defined emergency process.

### 5.47 Emergency Chaining

One emergency access context shall not automatically authorize another emergency request or broaden another subject's authority.

### 5.48 Emergency Bulk Operations

Emergency authority for a protected operation does not automatically authorize a bulk set beyond approved scope.

D02-003/004 bulk/resource controls remain applicable.

### 5.49 Emergency Non-Human Actors

Current L4-A-013 is expressed around canonical emergency-access subject/authenticated context and does not fully define machine emergency elevation.

D02-007 does not invent machine Break-Glass semantics. `ADR-L5-0009` defines ordinary and privileged non-human authorization binding, but it does not silently create machine emergency/Break-Glass authority.

If machine emergency access is required, upstream governance must define that behavior before realization.

### 5.50 Conflict + Emergency Interaction

A possible evaluation flow:

```text
prohibited SoD conflict detected
        ↓
is there an already-approved policy-permitted continuation?
        ├─ no → Deny
        └─ yes
             ↓
       is emergency context also required/valid?
             ↓
       evaluate all remaining controls
             ↓
          final Allow/Deny
```

D02-007 does not assume every permitted continuation is Break-Glass.

### 5.51 Maker-Checker + Emergency Interaction

Emergency authority does not automatically waive maker-checker.

If source policy explicitly permits emergency continuation without normal checker separation, that must be represented as an approved policy condition.

Otherwise normal maker-checker remains applicable.

### 5.52 Privileged + Emergency Interaction

D02-006 ordinary privilege and D02-007 Break-Glass remain separate authority sources.

A privileged administrator claiming “emergency” gains no additional authority without the governed emergency basis.

### 5.53 Temporal Freshness

SoD and emergency controls are time-sensitive.

Changes that may require re-evaluation include:

- Role Assignment activation/revocation;
- Delegation activation/expiry/revocation;
- Temporary Access grant/expiry/revocation;
- privileged authority change;
- SoD configuration change;
- institution override change;
- emergency expiry/revocation;
- policy version change;
- workflow actor change;
- resource/process state change.

### 5.54 Race Conditions

Security-sensitive races include:

```text
maker action vs actor-role change
checker action vs maker identity update
emergency expiry vs protected mutation
revocation vs cached Allow
SoD policy change vs pending approval
```

D02-007 requires current/versioned context sufficient to avoid stale permissive execution.

Exact transaction/locking design belongs B02/D02-004/A05.

### 5.55 Idempotency

Repeated technical processing shall not:

- duplicate conflict facts;
- create multiple emergency authorities;
- re-enable expired emergency access;
- duplicate checker completion;
- invent extra approvals.

Canonical business transitions remain source-owned.

### 5.56 Policy Engine Failure

If mandatory SoD or emergency control evaluation is unavailable:

- do not infer no conflict;
- do not infer valid emergency authority;
- no protected Allow.

### 5.57 Workflow Engine Failure

Workflow availability does not determine authorization truth.

If workflow/checker context required for policy cannot be reliably resolved:

- no permissive fallback;
- preserve operational failure separately from business Deny.

### 5.58 Client-Side Control

Client UI may show:

- maker/checker status;
- conflict warning;
- emergency request status;
- expiry countdown;
- review status.

None are authoritative security controls.

### 5.59 API / Service Enforcement

Every checker/emergency-protected operation remains server-side authorized.

Direct API invocation cannot bypass:

- maker-checker;
- conflict detection;
- emergency approval;
- emergency expiry.

### 5.60 Background Execution

A background worker completing a source-defined protected checker/action must have an authorized non-human execution model if upstream permits automation.

It shall not impersonate a checker human without explicit semantics.

### 5.61 Evidence Model

SoD evidence may include:

- subject/actor;
- Tenant/Institution;
- process/action/resource;
- effective Role Assignments;
- Delegations;
- Temporary Access;
- privileged authority;
- SoD policy/configuration version;
- detected conflict;
- continuation basis;
- final Authorization Decision.

Maker-checker evidence may include:

- maker actor;
- checker actor;
- maker operation reference;
- checker operation reference;
- process/resource correlation;
- actor-distinction result;
- checker authority;
- final decision.

Emergency evidence may include all L4-A-013-BH-009 facts.

### 5.62 Privacy / Evidence Minimization

Conflict/evidence records shall avoid copying unnecessary business data.

References and decision facts should be used where sufficient.

Credentials/secrets are never evidence.

### 5.63 Operational Observability

Operational telemetry may expose:

- SoD evaluation failures;
- conflict Denies;
- permitted-continuation attempts;
- same-actor maker-checker Denies;
- unresolved checker context;
- emergency requests blocked by missing prerequisites;
- active emergency authority counts;
- expired emergency authority detected in stale cache;
- notification handoff failures;
- emergency outcomes awaiting mandatory review.

No alert thresholds are defined.

### 5.64 Failure Model

| Failure / Condition | Required Outcome |
|---|---|
| Tenant/resource/process context unresolved | No permissive SoD/emergency result. |
| effective responsibilities unresolved | Do not infer no conflict. |
| SoD configuration ambiguous | Non-authoritative; no Allow based on it. |
| prohibited conflict | Deny unless approved policy explicitly permits continuation. |
| no conflict | Continue; not Allow. |
| continuation basis missing | Deny. |
| maker/checker distinctness required but same actor | Deny checker-protected action. |
| checker lacks authority | Deny. |
| emergency justification missing | No elevation. |
| emergency approval missing | No elevation. |
| emergency scope excessive | No elevation / restrict according to governed approved scope. |
| explicit Deny applies | Deny. |
| emergency expired/revoked | Exclude emergency authority. |
| notification recipient undefined | Do not invent recipient; preserve unresolved governance/evidence. |
| mandatory audit review incomplete | Emergency outcome not closed. |
| review authority undefined | Do not invent closure actor. |
| stale conflict/emergency cache | Revalidate / fail closed. |
| client claims conflict-free/emergency-approved | Ignore as authority. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Rationale / Basis | ADR |
|---|---|---|---|
| AD-001 | SoD, maker-checker and Break-Glass are distinct controls integrated through one centralized authorization decision architecture. | L2-A04 + frozen corpus. | None. |
| AD-002 | All current effective responsibility sources participate in SoD, including Role Assignments, Delegations and Temporary Access. | L4-A-051-BH-005. | None. |
| AD-003 | No-conflict is not Allow. | L4-A-051-BH-006. | None. |
| AD-004 | A prohibited SoD conflict Denies by default under Explicit Deny precedence. | L4-A-051-BH-007. | None. |
| AD-005 | Approved conflict continuation never bypasses remaining authorization controls. | L4-A-051-BH-008. | None. |
| AD-006 | Maker-checker is implemented as source-policy-driven actor/responsibility distinction, not a universal hard-coded two-role model. | L2-A04 SoD Policy definition. | None. |
| AD-007 | Same-actor checker execution is denied only where upstream policy requires actor distinction; D02-007 does not invent applicability. | ownership preservation. | None. |
| AD-008 | Institution SoD overrides apply only where governing policy explicitly permits them. | L4-A-051-BH-002. | None. |
| AD-009 | Break-Glass is temporary exceptional authority, not a permanent administrator or generic Deny bypass. | L4-A-013. | None. |
| AD-010 | Emergency elevation requires approval, bounded scope/duration, least privilege, policy, SoD and Explicit Deny evaluation. | L4-A-013-BH-001–006. | None. |
| AD-011 | Emergency expiry/revocation removes authority independently of later governance-review closure. | L4-A-013-BH-007/BH-008 distinction. | None. |
| AD-012 | Mandatory emergency audit review is a closure gate, not an authority-extension mechanism. | L4-A-013-BH-008. | None. |
| AD-013 | No undefined SoD/emergency approver, override authority, lifecycle state, command, event, notification recipient or review closer is invented. | explicit L4 gaps. | None. |
| AD-014 | SoD/emergency context remains Tenant-rooted. | A04. | `ADR-L5-0004`. |
| AD-015 | Control failure is non-permissive. | fail-closed baseline. | `ADR-L5-0001`. |
| AD-016 | No policy engine, rules language, workflow engine, timer, notification product, graph engine, cache, database or numeric emergency duration is selected here. | L5 technology governance. | Future ADR if material. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Toxic responsibility combinations | Central conflict evaluation before protected execution. | D02-007 |
| Delegated conflict bypass | Delegations remain visible in effective responsibility set. | D02-005/007 |
| Temporary privilege conflict | Granted Temporary Access participates in SoD. | D02-005/007 |
| Self-approval | Maker-checker actor distinctness enforced where policy requires. | A05/D02-007 |
| Admin bypass | Privileged users remain subject to SoD. | D02-006/007 |
| Emergency abuse | Exceptional justification + approval + scope/time + policy controls. | D02-007 |
| Emergency persistence | Expiry/revocation removes authority regardless of review completion. | D02-007/004 |
| Cross-Tenant bypass | Prohibited. | A04 |
| Stale conflict data | Current/versioned effective responsibility context required. | D02/B02/C02 |
| Audit integrity | Full conflict/continuation/emergency evidence preserved. | D04 |
| Privacy | Evidence minimized to needed references/facts. | B05/D04 |
| Notification governance | Required notification preserved without fabricated recipient. | H02/C02/A06 boundary |
| Operations | Pending review/stale elevation failures observable without weakening authorization. | E05 |

No numeric SoD matrix size, cache TTL, emergency duration, review SLA, notification SLA or maker-checker count is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Conformance Requirement | Basis |
|---|---|---|
| CR-001 | SoD evaluation shall use authoritative Tenant, institution, process/action/resource context. | L4-A-051 context. |
| CR-002 | Active Role Assignments, Active Delegations and Granted Temporary Access shall participate in effective responsibility resolution. | L4-A-051-BH-005. |
| CR-003 | Expired/revoked temporary authority shall not participate in SoD evaluation. | D02-005. |
| CR-004 | Platform SoD defaults shall apply unless an institution override is explicitly permitted by governing policy. | L4-A-051-BH-002. |
| CR-005 | Institution override shall not exceed source-governed boundaries. | L4-A-051. |
| CR-006 | SoD configuration shall preserve determinism, Tenant isolation, least privilege and Explicit Deny precedence. | L4-A-051-BH-003. |
| CR-007 | D02-007 shall not invent SoD policy approvers, override authorities, lifecycle states, commands or events. | L4-A-051 open gaps. |
| CR-008 | No-conflict result shall not independently produce Allow. | L4-A-051-BH-006. |
| CR-009 | Prohibited conflict shall produce Deny unless an already-approved policy explicitly permits continuation. | L4-A-051-BH-007. |
| CR-010 | Permitted continuation shall still evaluate all remaining D02-004 authorization controls. | L4-A-051-BH-008. |
| CR-011 | SoD evidence shall retain detected conflict and continuation basis where applicable. | L4-A-051 §7. |
| CR-012 | Maker-checker applicability shall come from upstream policy/process semantics, not D02-007 assumption. | ownership boundary. |
| CR-013 | Where actor distinction is required, maker and checker canonical actor references shall be compared server-side. | maker-checker technical enforcement. |
| CR-014 | Same display name, username or UI identity label shall not be used as the canonical actor-distinction test. | identity integrity. |
| CR-015 | Checker shall independently possess valid current authorization for the checker operation. | D02-004. |
| CR-016 | Workflow assignment alone shall not create checker authorization. | A05/D02 separation. |
| CR-017 | Delegated or temporary checker authority shall remain visible to SoD evaluation. | D02-005/007. |
| CR-018 | Privileged actors shall not be exempt from maker-checker/SoD where governed. | D02-006. |
| CR-019 | Break-Glass elevation shall require resolvable Tenant, subject, authenticated context, justification, requested scope and protected operation. | L4-A-013-BH-001. |
| CR-020 | Break-Glass shall evaluate source-defined policy, least privilege, SoD and Explicit Deny before elevation. | L4-A-013-BH-002. |
| CR-021 | Required emergency approval shall exist before temporary elevation. | L4-A-013-BH-003. |
| CR-022 | Emergency authority shall be limited to approved scope and governed duration. | L4-A-013-BH-004. |
| CR-023 | Emergency access shall not permanently alter Roles, Permissions or Role Assignments. | L4-A-013-BH-004. |
| CR-024 | Source-required administrator notification shall be handed off without inventing undefined recipient role. | L4-A-013-BH-005. |
| CR-025 | During emergency elevation, normal Authorization Policy, SoD and Explicit Deny shall still participate. | L4-A-013-BH-006. |
| CR-026 | Emergency expiry/revocation shall exclude emergency authority from subsequent authorization evaluation. | L4-A-013-BH-007. |
| CR-027 | Expired emergency authority shall not remain effective because a timer/job/cache transition is delayed. | security-safe cessation. |
| CR-028 | Existing Authentication Session shall not preserve expired/revoked emergency authority. | D01/D02 separation. |
| CR-029 | Mandatory emergency audit review shall be required before the emergency outcome is treated as closed. | L4-A-013-BH-008. |
| CR-030 | Incomplete audit review shall not reactivate or extend emergency authority. | closure/elevation separation. |
| CR-031 | D02-007 shall not invent the emergency review-closing authority/criteria. | L4-A-013 open gap. |
| CR-032 | D02-007 shall not invent Break-Glass lifecycle states, requestor/approver/revoker roles, commands or canonical events. | L4-A-013 §10. |
| CR-033 | `Break-Glass Activated` / `Expired` activity labels shall not be promoted to canonical event contracts without A06/C02 governance. | L4-A-013 §5. |
| CR-034 | Break-Glass shall not be treated as an ordinary privileged-access synonym. | D02-006 boundary. |
| CR-035 | A privileged actor claiming emergency shall receive no emergency authority without governed emergency basis. | D02-006/007. |
| CR-036 | Emergency context shall remain within one Tenant. | A04 / `ADR-L5-0004`. |
| CR-037 | Client/UI conflict-free, checker-approved or emergency-approved assertions shall not be authoritative. | server-side enforcement. |
| CR-038 | Mandatory SoD/emergency control unavailability shall not default to Allow. | fail closed. |
| CR-039 | Stale SoD/emergency projections shall not permit access after authoritative responsibility/policy/revocation change. | freshness. |
| CR-040 | Historical decisions shall retain the exact SoD/emergency policy/configuration references evaluated. | L4-A-052/D04. |
| CR-041 | SoD/emergency configuration change shall not rewrite historical decision/evidence. | historical integrity. |
| CR-042 | Emergency evidence shall preserve justification, scope, policy/SoD evaluation, approval, elevation period, protected decisions, notification, expiry/revocation and mandatory review activity. | L4-A-013-BH-009. |
| CR-043 | SoD/maker-checker evidence shall preserve actor/responsibility/process/resource/conflict provenance sufficient for reconstruction. | auditability. |
| CR-044 | Credentials/secrets shall not be copied into SoD/emergency evidence. | D03/D04. |
| CR-045 | Background/machine execution shall not masquerade as a checker human without explicit upstream semantics. | actor integrity. |
| CR-046 | Machine Break-Glass shall not be introduced solely by analogy to human emergency access. | zero-guessing. |
| CR-047 | Bulk emergency operations shall remain bounded by approved resource/scope context. | D02-003/004. |
| CR-048 | No implementation shall infer a mandatory rule engine, workflow engine, notification system, timer, graph store, cache, database, or numeric emergency duration from this specification. | AD-016. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Source / Gap | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| OD-001 | L4-A-051 and L4-A-013 remain `Draft` in their own metadata. | Governance disposition | Controlled Stage-2 validation rule | No | Resolved for D02-007 approval interpretation; source L4 metadata remains unchanged. |
| OD-002 | D02-001 through D02-006 were working authoring dependencies. | Peer stabilization | D02 family reconciliation decision | No | Closed: D02-001 through D02-006 are committed and Architecture Approved; reopen only on material peer change. |
| OD-003 | Canonical SoD Policy lifecycle states/transitions are undefined. | Upstream feature-realization gate | L4-A-051 open issue | No for reusable runtime enforcement; yes only for a feature requiring exact lifecycle UI/API semantics | Do not invent; define upstream before realizing the affected lifecycle feature. |
| OD-004 | SoD policy creation/configuration/approval commands/workflows are undefined. | Upstream feature-realization gate | L4-A-051 | No for reusable enforcement architecture; yes for the affected governance workflow | Define business authority/workflow upstream without L5 invention. |
| OD-005 | Canonical SoD policy events/publishers are undefined. | Upstream event feature gate | L4-A-051 | No for architecture approval; yes for canonical event integration | Define through upstream/event governance; do not promote activity labels into events. |
| OD-006 | Platform-default authority, institution-override authority, policy approver and continuation authority are unnamed. | Upstream authority feature gate | L4-A-051 | No for reusable enforcement architecture; yes for the exact governance workflow requiring those authorities | Preserve the gap and resolve upstream; do not invent roles. |
| OD-007 | Complete enterprise catalog of maker-checker/four-eyes applicability is not centrally defined. | Cross-domain feature gate | L2-A04 defines mechanism, not all process mappings | No for reusable framework; yes per protected process | Source domains/policies identify applicable actions. |
| OD-008 | Exact maker/checker qualification semantics per process are source-specific. | Business behavior feature gate | current upstream | No for reusable framework; yes per affected process | Preserve source-defined actor/responsibility requirements. |
| OD-009 | Delegator-vs-delegate equivalence semantics for specific SoD conflicts are not universally defined. | Policy feature gate | L4-A-051 requires Delegations participate but does not define all equivalence rules | No for reusable conflict framework; yes where such equivalence affects a concrete conflict | Define explicit source SoD policy semantics. |
| OD-010 | Break-Glass canonical lifecycle state names are undefined. | Emergency feature-realization gate | L4-A-013 | No for reusable emergency enforcement; yes for stateful emergency UI/API | Do not invent; resolve upstream before that feature is realized. |
| OD-011 | Break-Glass requestor/subject/approver/revoker roles are undefined. | Upstream authority feature gate | L4-A-013 | No for reusable emergency control architecture; yes for exact workflow implementation | Define upstream. |
| OD-012 | Break-Glass commands/events/publishers are undefined. | Event/contract feature gate | L4-A-013 | No for architecture approval; yes for canonical event integration | Do not invent; resolve through upstream/A06/C02 governance. |
| OD-013 | Administrator-notification recipient is undefined. | Governance feature gate | L4-A-013 | No for reusable notification handoff; yes for exact routing | Preserve required notification and unresolved recipient authority. |
| OD-014 | Mandatory audit-review closing authority/criteria are undefined. | Governance feature gate | L4-A-013 | No for reusable closure-gate architecture; yes for automated closure | D04/upstream must define; do not invent. |
| OD-015 | Whether notification failure must revoke/block an already approved emergency elevation is not universally defined. | Policy feature gate | L4-A-013 requires notification but does not define failure semantics | No for reusable emergency architecture; yes for the affected strict runtime branch | Source emergency policy must define. |
| OD-016 | Machine-principal Break-Glass behavior is undefined. | Upstream feature-realization gate | current emergency behavior is human-centric | No for current human-centric emergency architecture; yes if machine emergency access is required | Define new upstream behavior rather than analogy; `ADR-L5-0009` does not create machine Break-Glass semantics. |
| OD-017 | Exact SoD policy/rule-engine technology. | Technology | not selected | No for architecture approval; yes before concrete implementation if material | Select through L5 technology ADR governance. |
| OD-018 | Exact effective-responsibility/conflict cache and invalidation mechanism. | D02/B02/C02 implementation design | not selected | No for architecture approval; yes before distributed implementation | Preserve fail-closed freshness. |
| OD-019 | Exact race-safe transaction strategy for maker-checker/emergency execution. | D02-004/B02/A05 implementation design | not selected | No for architecture approval; yes for high-risk implementation | Define version/revalidation/transaction mechanism. |
| OD-020 | Exact notification and emergency-review workflow technical orchestration. | A05/H02/D04 implementation design | not selected | No for architecture approval; yes after source actors/contracts are resolved | Implement only after source actors/contracts are defined. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-D-02-007` realizes:

> Technical enforcement of upstream-defined Separation-of-Duties, Maker-Checker and Emergency Override controls without inventing approvers.

### 10.2 Locked L2 Traceability

- Separation of Duties Policy is a canonical A04 authorization aggregate.
- It defines configurable Maker-Checker and Four-Eyes authorization rules.
- Separation of Duties remains policy driven.
- Break-Glass is reserved for emergency situations.
- Break-Glass represents controlled emergency authorization.
- Authorization remains centralized, least-privilege, explicit-Deny preserving and auditable.

### 10.3 L4-A-051 Traceability

- `BH-001`: source-governed conflict semantics/process scope.
- `BH-002`: platform defaults + permitted institution overrides.
- `BH-003`: determinism, Tenant isolation, least privilege, Deny validation.
- `BH-004`: required approval evidence without invented authority.
- `BH-005`: current Roles, Delegations, Temporary Access, workflow context and SoD config participate.
- `BH-006`: no conflict is not Allow.
- `BH-007`: prohibited conflict Denies by default.
- `BH-008`: approved continuation still requires complete Authorization Allow.
- Open issues preserve undefined lifecycle/events/authorities.

### 10.4 L4-A-013 Traceability

- `BH-001`: emergency admission context.
- `BH-002`: policy, least privilege, SoD and Deny before elevation.
- `BH-003`: required approval.
- `BH-004`: scoped/time-bounded non-permanent emergency authority.
- `BH-005`: source-required administrator notification.
- `BH-006`: normal policy/SoD/Deny during elevation.
- `BH-007`: expiry/revocation removes authority.
- `BH-008`: mandatory review before closure.
- `BH-009`: complete emergency evidence.
- Open issues preserve undefined actors, states, commands, events, recipients and closure authority.

### 10.5 D02 Family Completion Flow

```text
D02-001 Authorization Architecture Baseline
        ↓
D02-002 Permission & Capability Model
        ↓
D02-003 Hierarchical Scope & Resource Context
        ↓
D02-004 Policy Decision & Enforcement
        ↑
D02-005 Delegation & Temporary Authority
        ↑
D02-006 Privileged & Administrative Access
        ↑
D02-007 SoD / Maker-Checker / Emergency Override
```

Together, D02-001 through D02-007 establish the Stage-2 authorization architecture family.

### 10.6 Architecture Decisions

`AD-001` through `AD-016`.

### 10.7 Conformance Requirements

`CR-001` through `CR-048`.

### 10.8 Open Decisions

`OD-001` through `OD-020`.

### 10.9 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-D/
            └── L5-D-02-007_Separation-of-Duties_Maker-Checker_and_Emergency_Override_Architecture.md
```

No nested `L5-D/L5-D-02/` folder is introduced.


## 2026-09-22 Batch 7B Impact Revalidation Synchronization

Non-material repository synchronization only. Current dependency metadata and/or authority wording is aligned to canonical governance without changing business authority, technical ownership, behavioral semantics, conformance requirements, traceability relation state, or lifecycle approval/lock basis.
