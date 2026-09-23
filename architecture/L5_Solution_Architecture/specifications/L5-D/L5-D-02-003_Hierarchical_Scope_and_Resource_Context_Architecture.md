# Hierarchical Scope & Resource Context Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-02-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-02 — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD Architecture  
**Canonical Technical Responsibility:** Tenant, Institution, organisational-unit and resource-context authorization scope.  
**Technical Architecture Owner:** L5-D-02 — Authorization Architecture  
**Assigned Author:** Devang  
**Version:** `1.0.2`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-02-001_Authorization_Architecture_Baseline.md` v1.2.1 Architecture Approved; `L5-D-02-002_Permission_and_Capability_Model_Architecture.md` v1.0.1 Architecture Approved; `L5-A-04-001` v1.0.1 Review; `L5-A-04-002` v1.0.1 Review; `L5-D-01-001` through `L5-D-01-004`; `L5-D-03-001`; `L4-A-010` v1.0.0 Draft; `L4-A-011` v1.0.0 Draft; `L4-A-012` v1.0.0 Draft; `L4-A-013` v1.0.0 Draft; `L4-A-050` v1.0.0 Draft; `L4-A-051` v1.0.0 Draft; `L4-A-052` v1.0.0 Draft; `L2-A04_Authorization_and_Policy_Workflows.md` v1.1 Locked; `ADR-PLAT-AUTH-001`; `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0009`.  
**Referenced By:** `L5-D-02-004`; `L5-D-02-005`; `L5-D-02-006`; `L5-D-02-007`; workflow/orchestration, API, service, background-job, client and business-domain architectures requiring scoped authorization.  
**Applicable ADRs:** `ADR-L5-0004` canonically constrains Tenant/Institution isolation and validated execution scope; `ADR-L5-0001` constrains trust-boundary admission; `ADR-PLAT-AUTH-001` constrains authorization subject ownership; `ADR-L5-0009` governs Tenant/scope binding for non-human Technical Principal Authority Bindings. No additional D02-003 product/technology ADR is selected here.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical authorization scope and resource-context architecture used by D02.

It answers four distinct questions that a Permission alone cannot answer:

```text
Permission       → What business action is being attempted?
Scope            → Where is that authority valid?
Resource Context → What exact protected resource / state / ownership is involved?
Policy           → Under these facts, is the action allowed?
```

The D02 model therefore separates:

```text
Permission / Capability
        ↓
Authorization Scope
        ↓
Resource Context
        ↓
Policy Evaluation
        ↓
Authorization Decision
```

This document owns the second and third layers above.

### 2.2 In Scope

- Tenant-rooted authorization scope.
- Institution as subordinate authorization scope.
- Lower authorization scope such as organizational unit, Campus, Department, Academic Session, Financial Year or other source-defined operational scope.
- Hierarchical scope relationships.
- Scope narrowing.
- Scope matching between subject authority and protected resource.
- Role Assignment scope consumption.
- Technical Principal Authority Binding scope consumption for non-human principals.
- Delegation/Temporary Access/privileged authority scope consumption.
- Resource identity/context references.
- Resource ownership relationship context.
- Resource state context.
- Scope provenance.
- Resource-context provenance.
- Multi-scope evaluation where upstream business semantics require more than one scope dimension.
- Scope intersection and effective-scope derivation.
- Prevention of implicit scope widening.
- Context preservation across synchronous, asynchronous, workflow, job and integration execution.
- Fail-closed handling of missing, ambiguous, incompatible or stale scope.
- Family contracts consumed by D02-004.

### 2.3 Out of Scope / Non-Goals

This specification does not own:

- Tenant or Institution business lifecycle.
- Organizational-unit business lifecycle.
- Campus/Department/Academic Session/Financial Year business definition or hierarchy ownership.
- Resource business lifecycle.
- Permission identity or Resource.Action representation — D02-002.
- Final policy evaluation / Allow / Deny engine — D02-004.
- Role Assignment, Delegation or Temporary Access business lifecycle.
- Non-human principal identity/credential lifecycle — D01/D03.
- Privileged/admin authority lifecycle.
- SoD/maker-checker/emergency semantics.
- Authentication or identity lifecycle.
- A04 execution-scope propagation internals.
- Persistence technology.
- API/event payload representation.
- graph/database/index implementation.
- universal hard-coded hierarchy for every business domain.
- invented inheritance where upstream business architecture does not define parent/child scope semantics.
- UI filters as security boundaries.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority Matrix

| Source | Status | Applicable Authority | D02-003 Effect |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | D02-003 owns Tenant, Institution, organisational-unit and resource-context authorization scope. | Establishes this specification’s exact L5 responsibility. |
| `L5-D-02-001` | Architecture Approved | Authorization is mandatory, Tenant-rooted, scope-aware, fail-closed and policy-driven. | D02-003 supplies scope/resource context into the D02 baseline. |
| `L5-D-02-002` | Architecture Approved | Permission answers “what”; scope is intentionally separate and delegated to D02-003. | Prevents Permission-key explosion and scope encoding. |
| `L5-A-04-001` | v1.0.1 Draft | Tenant is mandatory root protected scope; Institution is subordinate; missing/mismatched/cross-Tenant scope fails closed. | Establishes the outer isolation envelope D02-003 must never widen. |
| `L5-A-04-002` | v1.0.1 Draft | Validated Execution Scope Context carries Tenant, Institution and lower scope across execution/persistence; narrowing is permitted when authoritative; widening requires revalidation. | D02-003 consumes and refines lower authorization scope while preserving A04 semantics. |
| L2-A04 | v1.1 Locked | Role Assignment contains scope; policy may evaluate Institution, Department, Campus, time, resource state, ownership and attributes; Resource Scope Evaluation includes Tenant, Institution, Organization, Campus, Department, Academic Session and Financial Year; out-of-scope operations are denied. | Supplies the canonical authorization-scope and resource-context semantics. |
| L4-A-010 | v1.0.0 Draft | Role Assignment is available to later Role/Permission resolution only inside approved scope and effective period. | Scope is a first-class constraint on effective authority. |
| L4-A-011 | v1.0.0 Draft | Delegated authority is bounded and temporary. | Delegation cannot broaden source authority beyond governed delegated scope. |
| L4-A-012 | v1.0.0 Draft | Temporary Access remains bounded and still subject to resource/scope/policy evaluation. | Temporary grants contribute scoped authority rather than global authority. |
| L4-A-013 | v1.0.0 Draft | Emergency authority remains exceptional, bounded and non-permanent. | Emergency path cannot silently widen Tenant/resource scope. |
| L4-A-050 | v1.0.0 Draft | Workspace is experience context, not authorization authority. | Workspace selection cannot set authoritative authorization scope. |
| L4-A-051 | v1.0.0 Draft | Protected request resolves Tenant, institution, business-process/action/resource context and effective responsibilities for SoD evaluation. | D02-003 must provide deterministic resource/scope context to later SoD/policy evaluation. |
| L4-A-052 | v1.0.0 Draft | Policy version resolves exact Tenant/institutional scope and validates Role/Permission/scope references. | Scope references must be stable/provenance-aware enough for deterministic policy evaluation. |
| `ADR-L5-0004` | Architecture Approved | Tenant-rooted scope, subordinate Institution, propagation and fail-closed scope handling. | Governs the root isolation/scope model. |
| `ADR-L5-0009` | Architecture Approved | A non-human Technical Principal Authority Binding is Tenant-bound and may include Institution/lower-scope constraints that only narrow the principal’s valid authorization scope. | Gives machine principals a canonical scoped-authority model without human Role Assignment fabrication. |

For this controlled Stage-2 validation program, the cited canonical L4 System Behavior Specifications are accepted as behavioral authority even where their own metadata remains `Draft` or `Review`; that metadata status is not an approval blocker for D02-003.

### 3.2 Locked Scope and Resource Rules

| Constraint ID | Rule | Architectural Effect |
|---|---|---|
| C-001 | Tenant is the mandatory protected root. | Every D02-003 context is Tenant-bound. |
| C-002 | Institution is subordinate to Tenant. | Institution cannot replace/widen Tenant. |
| C-003 | Lower scope is conditional and source-owned. | D02-003 references lower scope but does not invent its business hierarchy. |
| C-004 | Role Assignment contains scope. | Effective human authority is constrained by assignment scope. |
| C-005 | Role Assignment scope and effective period both constrain authority. | Correct Permission with wrong scope is insufficient. |
| C-006 | Resource Scope Evaluation may include Tenant, Institution, Organization, Campus, Department, Academic Session and Financial Year. | D02-003 supports multi-dimensional operational scope. |
| C-007 | Business operations outside authorized operational scope shall be denied. | Out-of-scope is a non-permissive result. |
| C-008 | Scope evaluation always preserves Tenant isolation. | No lower-level relationship can bridge Tenants. |
| C-009 | Resource context may include Business Resource, Requested Action, Resource State and Resource Ownership. | D02-003 models these as authoritative inputs to D02-004. |
| C-010 | Resource ownership evaluation is policy-driven rather than hard-coded application ownership logic. | D02-003 supplies ownership fact; D02-004 decides. |
| C-011 | Resource state evaluation is policy-driven. | D02-003 supplies state fact; D02-004 decides. |
| C-012 | Scope may narrow under authoritative hierarchy but cannot silently widen. | Effective scope is intersection/narrowing oriented. |
| C-013 | Scope loss or mismatch across async/workflow/background handoff fails closed. | Ambient/global context is prohibited. |
| C-014 | Workspace does not grant authorization or scope. | Workspace selection is not a source of authority. |
| C-015 | Permission identity should not encode organizational scope instances merely to avoid proper scope evaluation. | Permission and scope remain orthogonal. |
| C-016 | A Technical Principal Authority Binding shall be rooted in the same Tenant as its D01 non-human principal; Institution/lower scope may only narrow authority and shall never bridge Tenant boundaries. | ADR-L5-0009. |

### 3.3 Explicit Non-Assumptions

This specification does not assume that every institution has the same lower hierarchy.

It does not assert a universal chain such as:

```text
Tenant → Institution → Campus → Department → Program → Resource
```

for every resource.

Instead, it establishes a technical scope framework capable of representing source-governed relationships such as:

```text
Tenant
 ├─ Institution
 │   ├─ Campus
 │   └─ Department
 ├─ Academic Session
 └─ Financial Year
```

or other valid domain-specific structures.

Where upstream architecture does not establish whether one lower scope contains, overlaps or is independent of another, D02-003 shall not invent that relationship.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 D02-003 Owns

- Authorization Scope Context.
- Resource Authorization Context.
- Scope-reference semantics.
- Effective-scope intersection/narrowing rules.
- Scope-match / scope-compatibility semantics.
- Technical Principal Authority Binding scope compatibility and narrowing semantics under `ADR-L5-0009`.
- Resource scope extraction boundary.
- Resource ownership-context extraction boundary.
- Resource state-context extraction boundary.
- Cross-context Tenant consistency.
- Lower-scope provenance.
- Multi-dimensional scope context.
- Scope/resource freshness requirements.
- family-level handoff into D02-004.

### 4.2 D02-003 Does Not Own

- Permission identity.
- final policy decision.
- non-human principal identity or credentials.
- business hierarchy lifecycle.
- authoritative Department/Campus/etc master data.
- business resource lifecycle.
- resource ownership mutation.
- resource state mutation.
- Role Assignment lifecycle.
- delegation lifecycle.
- Tenant/Institution lifecycle.
- persistence isolation implementation.
- context-carrier schema.
- UI filter logic.

### 4.3 Collaborating Architecture

| Architecture | Owns / Supplies | D02-003 Use |
|---|---|---|
| A04-001 | Tenant/Institution isolation envelope | Sets root and subordinate scope invariants. |
| A04-002 | Validated Execution Scope Context | Supplies propagated Tenant/Institution/lower execution context. |
| D01-002 | Human/non-human principal identity | Supplies Tenant-bound non-human principal identity referenced by Technical Principal Authority Binding. |
| D02-001 | Authorization baseline | Defines fail-closed scope-aware authorization. |
| D02-002 | Permission/Capability model | Supplies protected business action/resource capability. |
| D02-004 | Policy decision/enforcement | Consumes scope/resource facts and returns final decision. |
| D02-005 | Delegated/temporary authority | Supplies grant-specific scope/effective constraints. |
| D02-006 | Privileged/admin authority | Supplies privileged scope bounds. |
| D02-007 | SoD/emergency | Consumes process/resource/effective-responsibility scope. |
| Business domains | Resource identity/state/ownership and domain hierarchy truth | D02-003 references authoritative facts; does not duplicate them. |
| B02 | persistence/data ownership/isolation | Realizes state lookups/projections and isolation. |
| C01/C02 | contract/message propagation | Carries/revalidates scope/context. |
| D04 | audit/evidence | Preserves evaluated scope/resource provenance. |

---

## 5. Solution Architecture

### 5.1 Authorization Scope Model

D02-003 distinguishes three layers:

```text
1. Isolation Scope
   Tenant → Institution
   owned by A04

2. Authorization Operational Scope
   Organization / Campus / Department /
   Academic Session / Financial Year /
   other source-defined lower scopes

3. Resource Context
   exact business resource
   resource state
   ownership / relationship attributes
```

The D02 authorization decision consumes all applicable layers.

### 5.2 Conceptual Authorization Scope Context

```text
AuthorizationScopeContext
{
  tenant_reference                     // mandatory

  institution_reference                // when applicable
  lower_scope_references[]             // source-defined

  scope_dimension_types[]              // e.g. Campus, Department,
                                        // Academic Session, Financial Year

  assignment_or_technical_binding_scope_reference(s)
  delegated_or_temporary_scope_reference(s)

  requested_operation_scope_reference
  target_resource_scope_reference(s)

  validation_provenance
  effective_time_context
  correlation_reference
}
```

This is an architecture contract and not a required DTO/class/schema.

### 5.3 Conceptual Resource Authorization Context

```text
ResourceAuthorizationContext
{
  business_resource_reference
  requested_action_reference

  tenant_reference
  institution_reference where applicable
  lower_scope_reference(s)

  resource_state_reference / state facts where required
  resource_ownership_reference / ownership facts where required

  authoritative_source_reference
  fact_version / freshness reference where exposed
  correlation_reference
}
```

D02-003 collects/references these facts.

D02-004 evaluates them.

### 5.4 Scope Is Not Permission

A subject or non-human technical principal may possess:

```text
Student.Update
```

but the Role Assignment or Technical Principal Authority Binding may be scoped only to:

```text
Institution A
Department CSE
```

A Student resource in another Department or Institution is therefore not authorized merely because the Permission is resolved.

```text
Permission match
      +
Scope match
      +
Resource context
      +
Policy
      =
possible Allow
```

### 5.5 Tenant Root Invariant

Every authorization scope begins with one Tenant.

```text
Tenant T1
   ↓
all subordinate authorization scope
```

Rules:

1. All lower scope references must be compatible with the same Tenant.
2. A resource in Tenant T2 cannot be reached through authority rooted in Tenant T1.
3. A Role Assignment or Technical Principal Authority Binding from T1 cannot become valid in T2 through a matching Permission key.
4. Delegation/Temporary Access/privileged authority cannot cross Tenant by omission.
5. A cross-Tenant target is rejected before any lower-scope inheritance is evaluated.
6. Shared physical storage or shared services never create shared authorization scope.

### 5.6 Institution Scope

Institution is subordinate to Tenant.

Institution scope can constrain:

- Role Assignment.
- Technical Principal Authority Binding.
- delegated authority.
- temporary authority.
- protected operation.
- policy applicability.
- target resource.

A valid Tenant does not imply every Institution inside that Tenant is authorized.

Conceptually:

```text
Tenant Authority
     ≠
Institution Authority
```

unless upstream policy/assignment/binding explicitly provides the broader scope.

### 5.7 Lower-Scope Dimensions

L2-A04 allows authorization scope to include dimensions such as:

- Organization;
- Campus;
- Department;
- Academic Session;
- Financial Year.

D02-003 treats these as source-governed **scope dimensions**, not as one hard-coded universal tree.

Example:

```text
Authorization scope:
  Tenant = T1
  Institution = I1
  Department = CSE
  Academic Session = 2026-27
```

A protected resource may need all applicable dimensions to match.

### 5.8 Hierarchical Versus Orthogonal Scope

Some scopes are hierarchical:

```text
Tenant
  ↓
Institution
  ↓
Department
```

where authoritative source architecture explicitly defines containment.

Other dimensions may be orthogonal:

```text
Department = CSE
Academic Session = 2026-27
Financial Year = 2026-27
```

D02-003 shall not infer that one orthogonal dimension contains another.

The effective scope is evaluated by compatibility across each applicable dimension.

### 5.9 Scope Relationship Types

A technical scope relationship may conceptually express:

```text
same
contains
contained-by
disjoint
overlaps
unknown
```

However, these are technical relationship outcomes, not new business states.

The architecture only uses a relationship when the source-owned hierarchy can prove it.

`unknown` is not converted to `contains`.

For protected execution, unknown mandatory scope compatibility is non-permissive.

### 5.10 Scope Narrowing

Scope narrowing is allowed when the target context is demonstrably inside authorized scope.

Example:

```text
Authority Scope:
  Institution I1

Requested Resource:
  Department CSE inside Institution I1

Result:
  narrowed evaluation may continue
```

if and only if the authoritative hierarchy establishes that relationship and policy permits it.

Narrowing does not create additional Permission.

### 5.11 Scope Widening

Scope widening cannot occur implicitly.

Example:

```text
Authority Scope:
  Department CSE

Request:
  Institution I1 entire population
```

D02-003 shall not treat the narrower Department authority as Institution-wide authority.

A broader request requires separately governed broader authority.

### 5.12 Effective Scope Intersection

Where multiple authority sources constrain scope, the effective scope is bounded by their intersection rather than their union unless upstream explicitly establishes a unioning rule.

Conceptually:

```text
Base Authority Scope
          ∩
Delegation / Temporary Scope
          ∩
Requested Operation Scope
          ∩
Resource Scope
          =
Candidate Effective Scope
```

Examples:

**Role Assignment:** Institution I1  
**Delegation:** Department CSE only  
**Request:** Department CSE  
→ Candidate scope may continue as CSE.

**Role Assignment:** Department CSE  
**Temporary Grant:** Department Mechanical  
→ no automatic union is inferred.

Exact union behavior for multiple independent valid Role Assignments or Technical Principal Authority Bindings is handled as a set of separately attributable authority sources and later policy evaluation, not by silently widening one source.

### 5.13 Role Assignment and Technical Principal Authority Scope

For human authority, L4-A-010 establishes Role Assignment scope as part of the governed assignment.

D02-003 requires that:

- every effective Role Assignment retain its approved scope reference;
- Role Resolution not discard assignment scope;
- Permission Resolution not convert Role authority into unscoped Permission;
- policy evaluation receive the source assignment scope;
- expiration/effective period and scope both remain constraints;
- multiple assignments to the same Role at different scopes remain distinguishable.

For non-human authority, `ADR-L5-0009` requires that:

- the Technical Principal Authority Binding Tenant matches the D01 principal Tenant;
- Institution/lower-scope references only narrow the binding;
- missing/ambiguous scope fails closed;
- the binding scope remains distinct from principal identity and credential material;
- policy evaluation receives the binding scope and freshness/provenance reference.

Conceptual result:

```text
EffectiveAuthority
{
  authority_source_type
  authority_source_reference
  permission_reference(s)
  scope_reference(s)
  effective_period_or_validity_reference
  source_basis_reference
}
```

### 5.14 Delegation and Temporary Scope

Delegation and Temporary Access can narrow or otherwise govern effective authority according to source semantics.

D02-003 shall preserve:

- source authority scope;
- delegated/granted scope;
- effective period;
- recipient/actor;
- Permission/capability;
- Tenant root.

It shall not silently treat:

```text
delegated one Department
```

as:

```text
delegated entire Institution
```

Detailed grant/delegation lifecycle remains D02-005.

### 5.15 Privileged / Emergency Scope

Privileged and emergency authority remains scoped.

Even where upstream allows exceptional elevation:

- Tenant root remains mandatory.
- explicit governed target/scope remains required where source policy requires it.
- emergency authority does not become cross-Tenant.
- emergency context does not turn all resources into “owned”.
- later D02-004/D02-007 policy evaluation remains mandatory.

### 5.16 Protected Resource Context

The resource context begins with:

```text
Business Resource
Requested Action
```

and may add:

```text
Resource State
Resource Ownership
Resource Scope
other source-owned policy attributes
```

D02-003 does not copy or redefine the underlying resource.

It resolves only the minimum context required for authorization.

### 5.17 Resource Identity

A protected resource must be attributable to an authoritative business-domain reference.

Examples:

```text
Student record
Finance Voucher
Examination
Employee
Vendor
Asset
Document
```

The resource reference used by D02 shall be sufficient to:

- identify the target;
- resolve Tenant;
- resolve applicable Institution/lower scope;
- resolve required state/ownership facts;
- correlate authorization evidence.

D02-003 does not define domain resource IDs.

### 5.18 Resource Scope Extraction

The resource owner/domain supplies authoritative scope facts.

Conceptual flow:

```text
Protected Resource Reference
          ↓
Owning Domain
          ↓
Resource Scope Facts
          ↓
D02-003 Scope Compatibility
          ↓
D02-004 Policy Evaluation
```

D02-003 shall not infer resource Department/Campus/Institution merely from a UI route, client-selected filter or caller-supplied untrusted field.

### 5.19 Resource Ownership Context

L2-A04 allows ownership-based authorization.

Examples include scenarios such as:

- Faculty accessing assigned students.
- Mentor accessing allocated mentees.
- Librarian managing issued books.
- Hostel Warden managing assigned hostels.
- Department Head approving departmental activities.

D02-003 represents ownership/relationship facts but does not decide them through application-local shortcuts.

Conceptual contract:

```text
ResourceOwnershipContext
{
  resource_reference
  relationship_type_reference
  related_subject / organizational reference
  authoritative_source_reference
  effective_time / validity reference
}
```

Whether that relationship permits the requested action is a D02-004 policy decision.

### 5.20 Resource State Context

Authorization may depend on current resource state.

Examples conceptually include policy such as:

```text
can approve only if resource is in approvable state
can publish only after prerequisites are met
cannot modify finalized/archived resource
```

D02-003 shall:

- obtain/reference authoritative state;
- preserve state provenance;
- not invent a duplicate authorization-owned lifecycle;
- not change resource state;
- hand state context to D02-004.

### 5.21 Resource State Freshness

Because resource state can change authorization outcome, stale state cannot be assumed valid indefinitely.

Requirements:

- the evaluation must use state fresh enough for the protected operation/control;
- a state-changing operation may require revalidation close to enforcement;
- exact consistency/locking/transaction strategy belongs B02/D02-004;
- caching cannot turn a previously editable resource into permanently editable authority after authoritative finalization.

### 5.22 Ownership Freshness

Ownership/relationship facts can change.

Examples:

- student reassigned;
- employee transferred;
- asset custody changed;
- mentor allocation ended.

A stale ownership projection must not create durable access after authoritative relationship change.

D02-003 exposes freshness/provenance needs; implementation mechanics belong B02/C02/D02-004.

### 5.23 Multi-Dimensional Scope Example

Consider:

```text
Permission:
  Examination.PublishResults

Role Assignment:
  Tenant T1
  Institution I1
  Campus Main
  Department CSE

Resource:
  Examination E100
  Tenant T1
  Institution I1
  Campus Main
  Department CSE
  Academic Session 2026-27
  State = source-owned state X

Request:
  PublishResults
```

D02-003 can establish:

- same Tenant;
- compatible Institution;
- compatible Campus;
- compatible Department;
- Academic Session context;
- resource state context.

It does **not** decide Allow.

D02-004 still evaluates the applicable policy, SoD, explicit Deny, emergency/temporary authority and any other conditions.

### 5.24 Multi-Authority Scope Resolution

A Person may have multiple Role Assignments, and a non-human principal may have multiple separately governed Technical Principal Authority Bindings.

Example:

```text
Authority A → Department CSE
Authority B → Department AI
```

D02-003 does not merge these into:

```text
all departments
```

as a generic broad scope.

It maintains separately attributable authority sources and allows D02-004 to evaluate whether the requested resource matches one or more applicable authority sources.

This preserves least privilege and auditability.

### 5.25 Scope Inheritance

Scope inheritance is permitted only where source hierarchy explicitly establishes it.

A technical implementation may derive:

```text
authority at parent scope
        ↓
child resource scope
```

only if:

1. parent-child relationship is authoritative;
2. assignment/binding/policy semantics permit descendant coverage;
3. Tenant root remains identical;
4. no explicit Deny/child-specific constraint overrides;
5. resource context remains within the authorized operation.

D02-003 does not establish universal descendant inheritance for every Role, binding or scope.

### 5.26 Scope Overrides

Institution-specific or policy-specific scope exceptions are not implemented as arbitrary local bypasses.

Any override must be:

- source-governed;
- Tenant-compatible;
- attributable;
- visible to policy evaluation;
- non-destructive to historical traceability;
- incapable of bypassing explicit Deny unless upstream specifically defines such behavior.

Detailed override semantics belong D02-004/D02-007.

### 5.27 Scope Switching

Switching between valid authorization contexts requires an explicit new/revalidated scope context.

For example:

```text
Institution I1 → Institution I2
```

inside the same Tenant cannot be inferred merely because the actor has some authority in both.

Each protected request shall be evaluated against the requested/active governed context.

Workspace switching alone is insufficient.

### 5.28 Workspace Separation

Workspace may help a user choose an operational context, but:

```text
Workspace selection
      ≠
Authorization Scope Grant
```

D02-003 treats Workspace-provided scope hints as untrusted/non-authoritative until reconciled against A04 context and D02 authority.

### 5.29 Client Filter Separation

A UI filter such as:

```text
Department = CSE
```

does not prove the caller is authorized for CSE.

The filter can express the requested context, but the server must:

1. resolve authoritative Tenant;
2. validate Institution/lower scope;
3. validate effective authority scope;
4. resolve target resource scope;
5. evaluate policy.

### 5.30 Synchronous Execution

For protected synchronous calls:

```text
Request
  ↓
A04 Validated Execution Scope
  ↓
D02 Actor + Permission
  ↓
D02-003 Scope/Resource Context
  ↓
D02-004 Decision
  ↓
Enforcement
```

The receiving service shall not substitute ambient/default Tenant or Institution.

### 5.31 Asynchronous / Event Execution

For asynchronous protected consumption:

- scope correlation must survive publication/consumption according to A04/C02.
- consumer revalidates applicable resource/scope before mutable execution.
- event receipt does not itself grant scope.
- source event Tenant/resource facts do not bypass current authorization requirements where authorization is required at consumption/execution time.
- stale context shall not widen authority.

### 5.32 Workflow / Background / Scheduled Execution

Long-lived execution shall not depend on the current interactive user's later active context.

The workflow/job shall use:

- attributable actor/workload identity;
- captured/referenced governed authorization basis as applicable, including a Technical Principal Authority Binding for non-human execution where required;
- validated Tenant scope;
- authoritative resource context;
- current policy requirements where reauthorization is required.

Exact reauthorization frequency/strategy belongs D02-004/A05.

### 5.33 Resource Collections / Queries

Collection/list/query authorization needs special care.

A broad query such as:

```text
List all Students
```

shall not become a data-leak path merely because `Student.View` is resolved.

D02-003 requires a scope restriction model so the query execution cannot return resources outside authorized scope.

Conceptually:

```text
Requested collection
      ∩
Authorized operational scope
      ∩
Policy/resource constraints
      =
Permitted result domain
```

Exact query-filter/persistence enforcement belongs D02-004/B02, but scope semantics are defined here.

### 5.34 Bulk Operations

Bulk operation authorization shall not assume that one authorized resource proves all items are authorized.

For a bulk set:

```text
R1, R2, R3 ... Rn
```

each item or a proven homogeneous governed scope must be compatible with the effective authorization scope.

A mixed-Tenant bulk operation is prohibited.

Mixed-scope behavior shall follow policy and must not silently widen to the broadest item.

### 5.35 Parent / Child Resource Context

A child resource may derive some context from its authoritative parent relationship.

Example conceptually:

```text
Voucher Line → Voucher → Institution / Financial Year
```

D02-003 may consume the parent relationship to resolve authorization context only when that relationship is authoritative.

It shall not infer parentage from identifier shape or client payload alone.

### 5.36 Cross-Domain Resource References

A protected operation may reference resources from multiple domains.

D02-003 requires:

- each authoritative resource reference to preserve Tenant compatibility;
- the operation’s primary authorization scope to be explicit;
- secondary resource context not to broaden authority;
- policy to evaluate any required cross-resource relationship.

No cross-domain direct database coupling is implied.

### 5.37 Scope Conflict Model

| Conflict | Required Outcome |
|---|---|
| Actor authority Tenant ≠ resource Tenant | Deny/reject immediately. |
| Technical Principal Authority Binding Tenant ≠ D01 principal Tenant | Reject binding / no authority contribution. |
| Institution mismatch inside same Tenant | No permissive inheritance; deny unless separate authority applies. |
| Role Assignment or Technical Principal Authority Binding lower scope ≠ resource lower scope | That authority source does not authorize the resource. |
| Delegation scope broader than source authority | Reject/ignore invalid broadening; governance correction required. |
| Temporary grant scope missing where source requires scope | No permissive default. |
| Resource has ambiguous Institution/Department | No Allow until authoritative context resolves. |
| Client scope conflicts with authoritative resource scope | Authoritative resource/A04 scope wins; suspicious mismatch is evidenced. |
| Scope hierarchy relationship unknown | Do not infer containment. |
| Resource state unavailable where policy requires it | No permissive decision. |
| Ownership relation unavailable where policy requires it | No permissive decision. |
| Workspace-selected scope conflicts with authority | Workspace cannot override authority. |

### 5.38 Effective Scope Derivation

Conceptually:

```text
Validated A04 Tenant/Institution Context
        ∩
Authority Source Scope
        ∩
Delegated/Temporary/Privileged Scope (if applicable)
        ∩
Requested Operation Scope
        ∩
Target Resource Scope
        =
Candidate Effective Authorization Scope
```

A non-empty candidate scope is not an Allow.

It only means D02-004 may continue evaluation.

### 5.39 Resource Context Resolution Pipeline

```text
Protected Resource Reference
        ↓
Resolve owning domain / authoritative source
        ↓
Resolve Tenant / Institution
        ↓
Resolve lower scope dimensions
        ↓
Resolve state / ownership facts required by policy
        ↓
Validate freshness/provenance
        ↓
Compare with effective authority scope
        ↓
Pass context to D02-004
```

### 5.40 Scope Context Contract

Conceptual contract:

```text
ScopedAuthorizationCandidate
{
  tenant_reference
  institution_reference

  authority_source_reference
  permission_reference

  authority_scope_references[]
  requested_scope_references[]
  resource_scope_references[]

  matched_scope_dimensions[]
  unresolved_scope_dimensions[]

  resource_reference
  resource_state_reference
  resource_ownership_reference

  effective_time_context
  provenance_reference
  correlation_reference
}
```

No field above is intended as a prescribed JSON/schema name.

### 5.41 Failure / Rejection Model

| Failure Class | Required Technical Outcome |
|---|---|
| Tenant absent | Fail closed. |
| Tenant mismatch | Fail closed. |
| Technical Principal Authority Binding Tenant mismatch | Reject binding; no machine authority contribution. |
| Institution incompatible | Deny/reject. |
| Required lower scope absent | No permissive fallback. |
| Scope hierarchy unresolved | Do not infer parent/child. |
| Authority source scope expired/revoked/stale | Exclude source. |
| Resource scope cannot be resolved | No Allow. |
| Required resource state cannot be resolved | No Allow. |
| Required resource ownership cannot be resolved | No Allow. |
| Stale resource context detected | Refresh/re-evaluate before protected execution. |
| Context dropped across technical boundary | Stop protected mutable execution. |
| Client attempts wider scope than effective authority | Restrict/reject; never widen. |
| Cross-Tenant batch | Reject. |
| Collection query lacks enforceable scope boundary | Do not execute unrestricted protected query. |

### 5.42 Persistence / Projection Boundaries

D02-003 may use derivative technical projections of:

- organizational hierarchy;
- resource-to-scope mapping;
- ownership relationships;
- scope membership;
- authority-scope relationships.

These remain derivative.

Requirements:

- source domain remains canonical;
- D02-owned Technical Principal Authority Binding state remains separate from D01 identity state;
- projections shall retain source/provenance;
- stale projections shall not create enduring broadened access;
- cross-Tenant hierarchy edges are invalid;
- direct ownership of Campus/Department/etc does not transfer to D02;
- B02 owns storage/indexing/transaction mechanics.

### 5.43 Evidence

Scope/resource evaluation evidence may include:

- Tenant;
- Institution;
- applicable lower scope;
- authority-source scope;
- Technical Principal Authority Binding reference where applicable;
- target resource scope;
- resource state reference;
- resource ownership reference;
- matched/unmatched scope dimensions;
- hierarchy/provenance reference;
- effective time;
- actor/subject/non-human principal;
- Permission;
- correlation;
- reason for scope failure.

D04 owns evidence persistence/provenance architecture.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Rationale / Basis | ADR |
|---|---|---|---|
| AD-001 | Tenant is the mandatory authorization-scope root. | A04-001/A04-002; L2-A04 AP-12. | `ADR-L5-0004`. |
| AD-002 | Institution is a subordinate scope and shall not widen or replace Tenant. | A04 architecture. | `ADR-L5-0004`. |
| AD-003 | Lower authorization scopes are source-defined dimensions, not one universal hard-coded hierarchy. | L2-A04 supports multiple scope concepts; ownership preservation. | None. |
| AD-004 | Effective authority scope is determined by compatible/intersecting authority, request and resource contexts rather than permissive implicit union. | Least privilege; L4-A-010. | None. |
| AD-005 | Scope narrowing is allowed only through authoritative hierarchy; implicit widening is prohibited. | A04-002 scope-change rules. | `ADR-L5-0004`. |
| AD-006 | Resource state and ownership are authoritative policy inputs, not application-local authorization shortcuts. | L2-A04 §§71–72. | None. |
| AD-007 | Workspace/client filters may request or hint context but never establish authorization scope. | L2-A04 AP-07/AP-09; L4-A-050. | None. |
| AD-008 | Multiple effective authority sources retain separate scope provenance rather than being flattened into an unscoped union. | L4-A-010 + least privilege. | None. |
| AD-009 | Scope/resource context is resolved close enough to authorization/enforcement to prevent materially stale permissive access. | Resource-state/ownership semantics and D02 freshness invariant. | None; implementation mechanics D02-004/B02. |
| AD-010 | Collection/bulk authorization must constrain every returned/modified resource to proven authorized scope. | Scope integrity and Tenant isolation. | None. |
| AD-011 | D02-003 does not encode lower scope into Permission identity. | D02-002/D02-003 responsibility split. | None. |
| AD-012 | No hierarchy database, graph engine, policy product, cache, query-filter technology or context-carrier format is selected here. | L5 technology governance. | Future ADR if material. |
| AD-013 | Non-human authorization scope is carried through the Tenant-bound Technical Principal Authority Binding and may only narrow from that Tenant root. | D01/D02 ownership and Tenant isolation. | `ADR-L5-0009`. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Cross-Tenant access | Rejected before lower-scope evaluation. | A04 + D02-003 |
| Institution isolation | Institution mismatch is non-permissive. | A04 + D02-003 |
| Machine scope integrity | Technical Principal Authority Binding must match D01 principal Tenant and cannot become global/cross-Tenant authority. | D01-002 + D02-003 + ADR-L5-0009 |
| Least privilege | Scope intersection/narrowing prevents broad authority from being inferred. | D02-003 |
| Horizontal access | Resource ownership/context can participate in policy. | D02-003 + D02-004 |
| State-based protection | Resource lifecycle state can constrain authorization without D02 owning lifecycle. | Business domain + D02-003/004 |
| Delegation | Delegated scope cannot exceed governed source/delegation bounds. | D02-005 |
| Privileged access | Admin/emergency authority remains scoped. | D02-006/007 |
| Bulk/query data leakage | Result domain must be constrained by authorization scope. | D02-003/004 + B02 |
| Background execution | No ambient/current-user scope fallback. | A04-002 + A05/D02 |
| Stale relationships | Ownership/hierarchy projections cannot remain permissive after source change. | B02/C02/D02-004 |
| Auditability | Exact evaluated scope/resource context is attributable. | D04 |
| Privacy | D02 should reference minimum resource facts required for policy rather than duplicate sensitive records. | B05 + source domains |

No numeric freshness, TTL, depth, hierarchy size, batch size or latency target is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Conformance Requirement | Basis |
|---|---|---|
| CR-001 | Every protected Tenant-scoped authorization evaluation shall resolve one unambiguous Tenant root. | A04 / AD-001. |
| CR-002 | No authority, scope relation or resource context shall bridge different Tenants. | Tenant isolation. |
| CR-003 | Institution scope, where applicable, shall be validated as compatible with the Tenant root. | A04 / AD-002. |
| CR-004 | Lower scope relationships shall be consumed from authoritative source semantics rather than invented by D02. | AD-003. |
| CR-005 | A Role Assignment’s approved scope shall remain attached to its effective authority through Permission Resolution and policy evaluation. | L4-A-010. |
| CR-006 | Human Role/Permission resolution shall not convert scoped authority into an unscoped Permission grant. | L4-A-010; D02-002 boundary. |
| CR-007 | Scope narrowing shall require an authoritative compatible parent/child or equivalent relationship. | A04-002 / AD-005. |
| CR-008 | Scope widening shall not occur implicitly. | A04-002 / AD-005. |
| CR-009 | Workspace switching shall not by itself change or grant authorization scope. | L4-A-050. |
| CR-010 | UI/client filters shall not be treated as authoritative scope. | §5.29. |
| CR-011 | Resource scope shall be resolved from authoritative business-domain/A04 context rather than caller-controlled values alone. | §5.18. |
| CR-012 | Resource state shall remain source-owned and shall only be referenced as policy context. | L2-A04 §72. |
| CR-013 | Resource ownership/relationship shall remain source-owned and shall only be referenced as policy context. | L2-A04 §71. |
| CR-014 | Failure to resolve mandatory resource state/ownership context shall not produce a permissive default. | Fail-closed baseline. |
| CR-015 | Business operations outside the authorized operational scope shall be denied. | L2-A04 §73. |
| CR-016 | Tenant isolation shall be preserved during all lower-scope evaluation. | L2-A04 §73. |
| CR-017 | Multiple authority sources at different scopes shall retain separate source/scope provenance. | AD-008. |
| CR-018 | Delegated or Temporary Authority shall not widen beyond source-governed scope. | D02-005 boundary. |
| CR-019 | Privileged/emergency authority shall not become implicit cross-Tenant/global scope. | D02-006/007 boundary. |
| CR-020 | Permission identity shall not encode scope instances merely to bypass D02-003. | D02-002 / AD-011. |
| CR-021 | Unknown hierarchy relationship shall not be treated as containment. | §5.9. |
| CR-022 | Required scope context shall survive synchronous, async, workflow, job and integration handoffs or be authoritatively re-resolved. | A04-002. |
| CR-023 | Background/deferred execution shall not use ambient user/Tenant scope as authority. | A04-002. |
| CR-024 | Collection/query execution shall not expose resources outside the proven authorized result domain. | AD-010. |
| CR-025 | Bulk protected operations shall validate all applicable resource scope or a proven homogeneous governed scope; mixed-Tenant bulk execution is prohibited. | §5.34. |
| CR-026 | Cross-domain resource context shall preserve Tenant compatibility and shall not let secondary references broaden primary authority. | §5.36. |
| CR-027 | Scope/resource projections/caches shall remain derivative and shall not become canonical hierarchy/resource truth. | §5.42. |
| CR-028 | Materially stale scope, state or ownership context shall not be reused to create permissive access after authoritative change. | AD-009. |
| CR-029 | D02-003 shall not itself return final Allow merely because scope compatibility succeeds. | D02-004 ownership. |
| CR-030 | No implementation shall infer a required graph engine, hierarchy database, cache, query-filter library, claims format or numeric target from this specification. | AD-012. |
| CR-031 | Every Technical Principal Authority Binding shall match its D01 non-human principal Tenant and shall preserve any Institution/lower-scope constraints through D02-004 evaluation; no missing scope may be replaced by broader implicit scope. | ADR-L5-0009; AD-013. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Source / Gap | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| OD-001 | Material applicable L4 authorization behaviors remain `Draft` in their own metadata. | Governance disposition | Controlled Stage-2 validation rule | No | Resolved for D02-003 approval interpretation; source L4 metadata remains unchanged. |
| OD-002 | D02-001 and D02-002 were working authoring dependencies. | Peer stabilization | D02 family reconciliation decision | No | Closed: both are committed and Architecture Approved; reopen only on material peer change. |
| OD-003 | Complete enterprise lower-scope hierarchy across Organization, Campus, Department, Academic Session, Financial Year and domain-specific structures is not defined as one canonical tree. | Upstream/domain design | L2-A04 lists scope dimensions but does not make one universal hierarchy. | No for reusable architecture; yes for a domain-specific inheritance rule that lacks authoritative relationships | Each domain/source must provide authoritative relationships; do not invent universal containment. |
| OD-004 | Exact inheritance semantics for a parent scope covering descendants are not universally defined. | Upstream feature-realization gate | source-specific | No for reusable D02 architecture; yes only for a feature that attempts generic descendant inheritance without source authority | Require explicit authoritative hierarchy + policy semantics before enabling that inheritance. |
| OD-005 | Exact technical resource-scope lookup contracts for every business domain. | Cross-L5/domain integration | business domains own resources | No for architecture approval; yes for concrete implementation of each domain integration | Define C01/domain contracts preserving ownership and provenance. |
| OD-006 | Exact resource-state/ownership consistency and revalidation strategy at enforcement time. | D02-004/B02 implementation design | implementation concern | No for D02-003 architecture approval; yes before sensitive implementation | Resolve in D02-004/B02, including transaction/freshness semantics. |
| OD-007 | Exact collection/query authorization filter enforcement pattern. | D02-004/B02 technology/design | not selected | No for architecture approval; yes before query implementation | Define policy-enforcement + persistence-filter architecture under canonical ownership. |
| OD-008 | Exact machine-principal scope assignment/binding semantics. | Resolved D01/D02 decision | `ADR-L5-0009` | No | Resolved by the approved Tenant-bound Technical Principal Authority Binding pattern. |
| OD-009 | Exact scope relation storage/indexing technology. | Technology | no product selection | No for architecture approval | Select under L5 technology/ADR methodology only if material. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-D-02-003` realizes:

> Tenant, Institution, organisational-unit and resource-context authorization scope.

### 10.2 Primary Locked L2 Traceability

- L2-A04 Role Assignment includes scope.
- L2-A04 Role Assignment constraints include Institution, Department and Campus boundaries.
- Authorization policies may evaluate Institution, Campus, Department, time and resource attributes.
- Resource Context includes Business Resource, Requested Action, Resource State and Resource Ownership.
- §71: Resource Ownership Evaluation is policy-driven.
- §72: Resource State Evaluation is policy-driven.
- §73: Resource Scope Evaluation can include Tenant, Institution, Organization, Campus, Department, Academic Session and Financial Year.
- Operations outside authorized operational scope are denied.
- Tenant isolation is always preserved.

### 10.3 A04 Technical Traceability

- `L5-A-04-001`: Tenant root, subordinate Institution, fail-closed isolation.
- `L5-A-04-002`: Validated Execution Scope Context, propagation, narrowing, no implicit widening and no default/global Tenant fallback.
- `ADR-L5-0004`: canonical isolation/propagation decision.
- `ADR-L5-0009`: non-human authorization binding remains rooted in the D01 principal Tenant and may only narrow Institution/lower scope.

### 10.4 Primary L4 Traceability

- `L4-A-010-BH-001/BH-003/BH-007/BH-009/BH-010`: Role Assignment scope and effective authority boundary.
- L4-A-011: delegated authority remains bounded.
- L4-A-012: Temporary Access remains subject to scope/resource evaluation.
- L4-A-013: emergency access remains bounded/non-permanent.
- L4-A-050: Workspace does not grant or alter authorization.
- `L4-A-051-BH-005` through `BH-008`: resource/process/effective-responsibility context participates in centralized evaluation.
- `L4-A-052-BH-001/BH-004/BH-007`: Tenant/institutional scope and policy-version context.

### 10.5 D02 Family Handoff

```text
D02-001 Authorization Baseline
        ↓
D02-002 Permission & Capability Model
        ↓
D02-003 Hierarchical Scope & Resource Context
        ↓
D02-004 Policy Decision & Enforcement
        ↓
D02-005 / D02-006 / D02-007 specialized authority controls
```

### 10.6 Architecture Decisions

`AD-001` through `AD-013`.

### 10.7 Conformance Requirements

`CR-001` through `CR-031`.

### 10.8 Open Decisions

`OD-001` through `OD-009`.

### 10.9 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-D/
            └── L5-D-02-003_Hierarchical_Scope_and_Resource_Context_Architecture.md
```

No nested `L5-D/L5-D-02/` folder is introduced.


## 2026-09-22 Batch 7B Impact Revalidation Synchronization

Non-material dependency-metadata synchronization only. Current peer lifecycle/version metadata is reconciled where explicitly stated; the controlled Stage-2 authorization for cited Draft L4 behavior remains unchanged. No authorization ownership, business authority, policy semantics, conformance requirement, traceability relation state, or Architecture Approved basis changed.
