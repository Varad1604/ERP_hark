# Permission & Capability Model Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-02-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-02 — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD Architecture  
**Canonical Technical Responsibility:** Technical permission/capability representation without encoding business ownership into generic roles.  
**Technical Architecture Owner:** L5-D-02 — Authorization Architecture  
**Assigned Author:** Devang  
**Version:** `1.0.1`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-02-001_Authorization_Architecture_Baseline.md` v1.0.0 Architecture Approved; `L5-D-01-001` through `L5-D-01-004`; `L5-A-04-001`; `L5-A-04-002`; `L5-D-03-001`; `L4-A-010` v1.0.0 Draft; `L4-A-011` v1.0.0 Draft; `L4-A-012` v1.0.0 Draft; `L4-A-013` v1.0.0 Draft; `L4-A-050` v1.0.0 Draft; `L4-A-051` v1.0.0 Draft; `L4-A-052` v1.0.0 Draft; `L2-A04_Authorization_and_Policy_Workflows.md` v1.1 Locked; `ADR-PLAT-AUTH-001`; `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0009`.  
**Referenced By:** `L5-D-02-003`; `L5-D-02-004`; `L5-D-02-005`; `L5-D-02-006`; `L5-D-02-007`; protected APIs/services/jobs/workflows; role-resolution and policy-evaluation implementations.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001`, `ADR-L5-0001`, and `ADR-L5-0004` constrain identity, trust, and Tenant scope. `ADR-L5-0009` governs the non-human Technical Principal Authority Binding used to make governed Permissions/capabilities effective for service/workload/job/integration principals.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the L5 technical model for representing and resolving SutraERP authorization Permissions and capability references.

It refines the D02 baseline by answering:

```text
What business operation is being protected?
                +
What authorization atom represents that operation?
                +
How is that atom grouped into governed authority?
                +
How is the atom preserved across channels without becoming UI logic,
business ownership, scope ownership, or a final authorization decision?
```

The canonical upstream model is:

```text
Permission
   =
Business Resource
   +
Business Action
```

with the governed naming form:

```text
<Resource>.<Action>
```

This L5 specification turns that locked authorization model into an implementation-ready technical representation without inventing new business authorization concepts.

### 2.2 In Scope

- technical Permission identity and descriptor model;
- canonical Resource + Action semantic decomposition;
- stable permission-key semantics;
- technical capability references consumed by services, APIs, workflows, jobs, integrations, and client visibility helpers;
- Role-to-Permission technical mapping boundary;
- permission-resolution semantics;
- indirect authority through Role Assignment;
- participation of Delegation, Temporary Access, privileged/emergency authority;
- permission catalogs/registries as technical projections of authorization-owned truth;
- duplicate/conflict detection;
- permission compatibility across human and non-human callers;
- least-privilege granularity principles;
- resource/action ownership registration boundary;
- permission deprecation/change safety without inventing unsupported lifecycle states/events;
- projection/cache consistency constraints;
- evidence/provenance needed by D02-004/D04;
- conformance requirements for business modules defining new protected capabilities.

### 2.3 Out of Scope / Non-Goals

This specification does not define:

- final Authorization Decision semantics — D02-001/D02-004;
- detailed hierarchical scope, Institution/department/resource-scope inheritance — D02-003;
- detailed policy evaluation/enforcement — D02-004;
- Delegation/Temporary Access lifecycle — D02-005;
- privileged/admin safeguards — D02-006;
- SoD, maker-checker, emergency override — D02-007;
- Person, Identity, User Account, Authentication Session or Credential — D01;
- Tenant/Institution isolation ownership — A04;
- Role Assignment business lifecycle — upstream A04/L4-A-010;
- business-resource lifecycle;
- business-domain ownership of operations;
- organizational job/position models;
- UI pages, menus, buttons or navigation as canonical Permissions;
- API route names or HTTP methods as canonical permission semantics;
- database schema;
- policy engine/vendor/framework;
- claims/token format;
- cache technology;
- numeric permission limits or performance targets;
- a new canonical `Capability` business aggregate unless upstream governance later defines one.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority Matrix

| Source | Status | Applicable Authority | D02-002 Effect |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | D02-002 owns “technical permission/capability representation without encoding business ownership into generic roles.” | Establishes exact L5 responsibility. |
| `L5-D-02-001` | v1.0.0 Architecture Approved | Permission/Capability is an effective-authority input, not final Authorization Decision; detailed model delegated to D02-002. | D02-002 refines representation/resolution only. |
| L2-A04 AP-04 | Locked | Permissions protect business resources and authorize business actions; they are not UI-page definitions. | Resource + Action is the semantic basis. |
| L2-A04 §§34–38 | Locked | Permission consists of Business Resource + Business Action; naming format `<Resource>.<Action>`; Permissions are assigned to Roles; users receive them indirectly via assigned/delegated/temporary/emergency authority. | Establishes identity, naming, and mapping rules. |
| L2-A04 §42 | Locked | Permission invariants: independent of UI/navigation/dashboard/URL; Roles rather than users; decisions evaluate Permissions with Policies; Explicit Deny overrides granted Permissions. | Prevents permission checks from becoming final decision shortcuts. |
| L4-A-010 | Draft | An Active Role Assignment makes the Role available for later Role/Permission Resolution only within approved scope/effective period; direct Permission assignment to Person is prohibited by that flow; active assignment is not final decision. | Permission resolution must consume active authority and preserve later policy evaluation. |
| L4-A-011 | Draft | Delegation contributes temporary authority and reverts/ends under governed rules. | Permission availability through delegation is derived/effective, not permanent remapping. |
| L4-A-012 | Draft | Temporary Access can make a Permission available, but granted Permission does not automatically authorize execution; later policy/resource/scope/SoD/Deny evaluation still applies. | Temporary permission contribution remains contextual. |
| L4-A-013 | Draft | Emergency authorization is exceptional, bounded, auditable and not permanent. | Emergency-derived Permissions/Capabilities cannot mutate baseline catalog or permanent Role mappings. |
| L4-A-050 | Draft | Workspace represents responsibilities but does not create/alter Role or Permission ownership and does not change authorization. | Workspace mappings may reference Permissions/Roles for UX but never own them. |
| L4-A-051 | Draft | Current Roles, Delegations and Temporary Access participate in SoD evaluation; Deny precedence remains final. | Permission availability must retain authority provenance for later SoD/policy evaluation. |
| L4-A-052 | Draft | Policy versions reference/validate Roles and Permissions and decisions retain exact policy-version traceability. | Permission identifiers/references must be stable enough for deterministic policy/history reconstruction. |
| `ADR-PLAT-AUTH-001` | Approved | Role Assignment subject is canonical Person; User Account/session is runtime authentication context. | Permission model does not attach permanent business authority directly to User Account/session. |
| `ADR-L5-0004` | Architecture Approved | Tenant is mandatory protected root. | Permission resolution occurs inside valid Tenant scope without encoding Tenant widening into Permission identity. |
| `ADR-L5-0009` | Architecture Approved | Non-human principals receive authorization authority through a D02 Technical Principal Authority Binding. | Machine principals can receive governed Permissions/capabilities without fabricated Person or human Role Assignment semantics. |

For this controlled Stage-2 validation program, the cited canonical L4 System Behavior Specifications are accepted as behavioral authority even where their own metadata remains `Draft` or `Review`; that metadata status is not an approval blocker for D02-002.

### 3.2 Locked Permission Rules

The following upstream rules are treated as architectural constraints:

| Constraint ID | Rule | Effect |
|---|---|---|
| C-001 | A Permission protects a Business Resource. | Permission identity begins with a governed resource concept, not screen/controller/table naming. |
| C-002 | A Permission authorizes a Business Action. | Permission identity includes the operation/capability being protected. |
| C-003 | Canonical Permission shape is `<Resource>.<Action>`. | D02-002 preserves this naming semantic unless upstream governance changes it. |
| C-004 | Permissions are the smallest assignable authorization unit described by L2-A04. | Role/policy design should compose Permissions rather than invent opaque “access levels” as substitutes. |
| C-005 | Permissions are assigned to Roles; people receive normal authority indirectly through Role Assignments. | Permanent direct Person/User Permission grants are prohibited by the baseline model. |
| C-006 | Delegation, Temporary Access and emergency authorization may make Permissions available without permanently changing underlying Role/Permission ownership. | Derived/effective permission sources retain provenance and expiry/closure semantics. |
| C-007 | A granted/resolved Permission is not a final Authorization Decision. | D02-004 still evaluates scope/resource/policy/SoD/Deny. |
| C-008 | Permissions are independent of UI pages, navigation, dashboards and URLs. | UI/transport artifacts may map to protected capabilities but cannot define canonical Permission semantics. |
| C-009 | Explicit Deny overrides a granted Permission. | Permission resolution returns candidate/effective authority, never unconditional execution rights. |
| C-010 | Business domains retain ownership of business resources and actions. | D02 registers/represents authorization atoms but does not take ownership of domain behavior. |
| C-011 | Tenant/Institution scope is separately governed. | Scope shall not be silently embedded into generic Permission names as a substitute for D02-003. |
| C-012 | Historical policy/decision reconstruction requires stable references. | Permission identifiers shall not be silently repurposed to mean a materially different business action. |
| C-013 | A non-human principal receives candidate Permission/capability authority only through a valid Tenant-bound Technical Principal Authority Binding; credential possession or machine identity alone does not grant authority. | ADR-L5-0009. |

### 3.3 Assumptions and Explicit Non-Assumptions

This specification does not assume a canonical Permission lifecycle state machine because no applicable upstream behavior establishes one.

It therefore does not invent states such as:

- Draft;
- Active;
- Deprecated;
- Disabled;
- Retired;
- Superseded.

Technical implementations may need governance/status metadata, but canonical state names and business events must come from approved upstream authority or a separately governed L5 technical decision where permitted.

The term **Capability** in this specification means a technical reference to an authorization-relevant Permission/business capability. It does not establish a new upstream business aggregate.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 D02-002 Owns

- Permission technical identity/descriptor architecture.
- Resource + Action semantic representation.
- Permission key normalization/validation against locked naming semantics.
- technical Permission catalog/registry view.
- Role-to-Permission resolution boundary.
- effective Permission-set projection semantics.
- permission-source provenance.
- duplicate/collision detection.
- safe change/deprecation compatibility requirements.
- cross-channel capability reference.
- technical model for non-human actors to request protected capabilities without fabricating human Roles.
- permission/capability participation in `ADR-L5-0009` Technical Principal Authority Bindings.
- contracts supplied to D02-003/D02-004.

### 4.2 D02-002 Does Not Own

- final Allow/Deny;
- policy precedence/evaluation engine;
- resource-scope hierarchy;
- Role Assignment lifecycle;
- Delegation/Temporary Access lifecycle;
- Person/Identity/User Account;
- non-human principal identity or credentials;
- business resource definitions or lifecycle;
- UI navigation;
- workflow approval logic;
- Tenant lifecycle/isolation;
- trust/cryptography;
- persistence technology.

### 4.3 Dependency Collaboration

| Dependency | Responsibility | D02-002 Use |
|---|---|---|
| D02-001 | family authorization baseline | Supplies mandatory authorization invariants and decision boundary. |
| D01 | actor/principal identity | Supplies attributable human/non-human caller context; no Permission ownership. |
| A04 | Tenant/Institution context | Bounds resolution; D02-002 cannot widen scope. |
| D02-003 | hierarchical scope/resource context | Applies where/which resource instances a Permission may govern. |
| D02-004 | policy decision/enforcement | Consumes Permission resolution as one input and produces final decision. |
| D02-005 | delegation/temp authority | Supplies additional governed permission-source relationships. |
| D02-006 | privileged/admin access | Supplies high-risk effective authority source/control. |
| D02-007 | SoD/emergency | Consumes effective Permissions/responsibilities and applies conflict/override rules. |
| B02 | persistence/index/projection mechanics | Realizes catalog/mapping storage consistently. |
| C01/C02 | contracts/change propagation | Carries Permission refs and invalidation facts without changing semantics. |
| D04 | evidence | Records Permission references/provenance evaluated. |

---

## 5. Solution Architecture

### 5.1 Permission Semantic Model

The canonical semantic unit is:

```text
Permission
  ├── Business Resource
  └── Business Action
```

Canonical technical key shape:

```text
<Resource>.<Action>
```

Examples already consistent with upstream convention include patterns such as:

```text
Student.View
Attendance.Mark
Examination.PublishResults
FinanceVoucher.Approve
```

These are examples of the locked convention, not a universal enumeration.

A Permission answers:

> **What kind of business operation may be attempted?**

It does not by itself answer:

- who may execute it;
- in which Tenant;
- for which Institution;
- on which exact resource instance;
- at what time;
- whether SoD permits it;
- whether resource state permits it;
- whether a policy Deny applies;
- whether execution is finally authorized.

Those answers belong to effective authority, D02-003 scope/resource context, and D02-004 policy decision.

### 5.2 Technical Permission Descriptor

A conceptual Permission descriptor may contain:

```text
PermissionDescriptor
{
  permission_reference
  permission_key              // <Resource>.<Action>
  business_resource_reference
  business_action_reference
  owning_business_domain_reference
  semantic_description
  provenance / governance_reference
  compatibility / change_reference where governed
}
```

This is a semantic contract, not a prescribed database schema.

The descriptor shall not require:

- UI page ID;
- route;
- HTTP verb;
- frontend component;
- dashboard;
- menu item;
- database table name;
- internal class/method name.

Those implementation artifacts can map to a protected Permission at enforcement boundaries, but they are not the canonical Permission identity.

### 5.3 Resource Model Boundary

A Business Resource is an authorization-relevant protected business concept owned by its canonical domain.

Examples may include:

- Student;
- Employee;
- Examination;
- FinanceVoucher;
- Vendor;
- Asset;
- Document.

D02-002 does not own the business definition or lifecycle of those resources.

The Permission catalog stores only the minimum authoritative reference/semantic information needed to define the protected capability.

#### Resource registration rules

1. Resource identity shall refer to a real governed business capability/resource.
2. Two resource names shall not represent the same business concept merely because different channels expose it differently.
3. One UI page containing multiple protected business operations may map to multiple Permissions.
4. One Permission may protect the same business action across multiple technical channels.
5. A resource identifier shall not silently change meaning after being used in Roles, Policies or historical Authorization Decisions.
6. Resource instance identity and hierarchical resource scope are D02-003 concerns, not part of the Permission key itself unless explicitly required by upstream semantics.

### 5.4 Business Action Model Boundary

A Business Action describes the operation being protected.

Common upstream examples include actions such as:

```text
View
Create
Update
Delete
Approve
Publish
Mark
```

D02-002 does not impose a fixed global verb enumeration beyond upstream-governed semantics.

Action design shall prefer the business meaning of the protected operation over transport implementation.

For example:

```text
FinanceVoucher.Approve
```

is stronger architecture than:

```text
POST./api/v1/voucher/approve
```

because the former expresses stable authorization intent while the latter couples authorization to a technical route.

### 5.5 Permission Key

The canonical naming form is:

```text
<Resource>.<Action>
```

The Permission Key is a stable technical identifier for an authorization atom.

#### Permission Key invariants

- unique within the governed authorization namespace in which the upstream model expects uniqueness;
- semantically attributable to one protected business Resource + Action combination;
- independent of UI/channel;
- independent of a particular user;
- independent of a specific Authentication Session;
- does not encode a Role name;
- does not encode a Person name;
- does not encode a temporary assignee;
- does not use Tenant ID as a substitute for scope architecture;
- does not contain time-bound grant state;
- does not imply final Allow.

#### Namespace / collision boundary

Where two business domains use the same natural resource word for materially different concepts, the architecture shall resolve ambiguity through governed resource identity/naming rather than allowing two Permission Keys to collide semantically.

The exact cross-domain namespace convention beyond the locked `<Resource>.<Action>` shape remains an open design item unless already governed.

### 5.6 Capability Reference

D02-002 uses **Capability Reference** as a technical integration term.

A Capability Reference is:

> a stable technical reference to the Permission/business capability that a protected operation requires.

It may be consumed by:

- API enforcement;
- service boundaries;
- workflow tasks;
- background jobs;
- integration operations;
- client visibility helpers;
- policy rules.

It shall not create an independent authority source.

Conceptually:

```text
Protected Operation
       ↓ requires
Capability Reference
       ↓ resolves to
Permission Descriptor
       ↓ participates in
D02 Authorization Evaluation
```

A Capability Reference is therefore a reference/projection, not a new business aggregate.

### 5.7 Permission Assignment Model

Normal permanent human authority follows:

```text
Permission(s)
      ↑
     Role
      ↑
Active Role Assignment
      ↑
Canonical Person subject
```

The Permission is assigned to the Role.

The Person receives effective Role authority through the governed Role Assignment.

D02-002 shall not create:

```text
Person ── permanent direct Permission
```

as a competing default authorization model.

For non-human principals, the approved model is distinct:

```text
D01 Non-Human Principal
        ↓
Technical Principal Authority Binding
        ↓
Permission / Capability references
```

The binding is D02 authorization technical state, not a human Role Assignment and not an identity/credential record.

### 5.8 Effective Permission Sources

A Permission may become effective for an authorization evaluation from one or more governed authority sources:

```text
Active Role Assignment
        │
        ├── Role → Permission mapping
        │
Technical Principal Authority Binding
        │
Authorization Delegation
        │
Temporary Access Grant
        │
Privileged / Emergency authority
        │
        ▼
Effective Permission Set
```

The effective set is **derived**.

It is not a new canonical aggregate and shall retain provenance for each contributing Permission.

Conceptual structure:

```text
EffectivePermission
{
  permission_reference
  source_authority_type
  source_authority_reference
  subject_or_technical_principal_reference
  actor_reference where distinct
  effective_period_reference
  scope_reference
  provenance_reference
}
```

Detailed source lifecycles remain D02-005/006/007. Technical Principal Authority Binding semantics are governed by `ADR-L5-0009`.

### 5.9 Role-to-Permission Mapping

Role groups business Permissions representing an organizational responsibility.

D02-002 owns the technical mapping semantics but not the upstream organizational/business decision about which Permissions a Role should contain.

Technical rules:

1. Role-to-Permission mapping references governed Role and Permission identities.
2. A mapping cannot create a new Permission implicitly.
3. Duplicate identical mappings do not create extra authority.
4. A mapping does not override policy Deny.
5. A mapping does not widen Role Assignment scope.
6. Removal/change of a mapping shall invalidate/reconcile derived effective Permission projections.
7. Historical Authorization Decisions retain sufficient references to reconstruct what Permission/policy context was evaluated.
8. Exact Role/Permission governance events are not invented if upstream does not define them.

### 5.10 Permission Resolution

Conceptual resolution:

```text
Input:
  subject / actor or D01 non-human principal
  authoritative effective authority references
  requested Capability Reference
  time
  Tenant context

Resolve:
  applicable Role Assignments for human subjects
  applicable Technical Principal Authority Binding for non-human principals
  applicable Role→Permission mappings
  delegated / temporary / privileged sources
  requested Permission descriptor
  source provenance

Output:
  resolved candidate/effective Permission facts
  NOT final Allow
```

The resolved result is passed to D02-003/D02-004.

Conceptual contract:

```text
PermissionResolutionResult
{
  requested_permission_reference
  matched_permission_reference(s)
  source_authority_reference(s)
  source_role_reference(s) where applicable
  subject_or_technical_principal_reference
  actor_reference
  effective_period_reference(s)
  scope_reference(s)
  resolution_provenance
}
```

### 5.11 Permission Resolution Does Not Equal Authorization

This rule is fundamental.

```text
Permission resolved
        ≠
Operation authorized
```

A resolved/granted Permission still requires the complete authorization context:

- Tenant/Institution;
- lower scope;
- resource instance/context;
- resource state where policy uses it;
- policy version;
- SoD;
- explicit Deny;
- Delegation/Temporary Access conditions;
- privileged/emergency controls;
- trust prerequisites.

D02-004 owns the final evaluation and enforcement.

### 5.12 Scope Separation

Permission answers **what**.

D02-003 answers **where / for which governed scope or resource context**.

```text
Permission: Student.Update
Scope:      Institution X / Department Y / permitted resource context
Policy:     source-governed conditions
Decision:   D02-004
```

The architecture shall avoid Permission explosion such as:

```text
Student.Update.DepartmentA
Student.Update.DepartmentB
Student.Update.DepartmentC
```

when the varying concept is actually authorization scope and not a different business action.

This does not prohibit distinct Permissions where business semantics truly differ.

### 5.13 Resource-State Separation

A Permission shall not encode transient resource lifecycle state into its identity merely to express policy.

Example:

```text
Examination.PublishResults
```

may be subject to a policy requiring the Examination/resource to be in an appropriate upstream-owned state.

D02-002 does not invent keys such as:

```text
Examination.PublishResults.WhenFinalized
```

unless upstream explicitly defines that as a distinct business capability.

Resource-state evaluation belongs to D02-003/D02-004 using authoritative domain context.

### 5.14 UI / Navigation / Endpoint Independence

The following are not canonical Permissions:

```text
OpenStudentPage
ShowApproveButton
AccessDashboardWidget
POST_/voucher/approve
Route_Admin_Settings
```

unless the protected concept behind them is separately defined as a real business action/resource.

UI and API implementations may map:

```text
UI button / API method / service command
                 ↓
        Capability Reference
                 ↓
             Permission
```

but not the reverse.

This ensures the same business capability is protected consistently across all channels.

### 5.15 Permission Catalog / Registry

A technical Permission Catalog may expose the authoritative set of Permission descriptors for discovery/validation by D02 components and build-time/runtime consumers.

The catalog may support:

- lookup by Permission reference/key;
- resource/action validation;
- Role mapping validation;
- policy-reference validation;
- duplicate/collision detection;
- integration capability lookup;
- documentation/discovery;
- evidence-friendly description/provenance.

The catalog shall not:

- become a second business-resource registry;
- invent business actions;
- silently activate undefined Permissions;
- own final Role Assignment;
- own final Authorization Decision;
- permit applications to register unrestricted Permissions at runtime without governance.

Concrete persistence/distribution technology remains outside D02-002.

### 5.16 Permission Definition Admission

Before a new Permission is considered technically usable, the implementation shall be able to establish:

1. a governed business resource/action semantic exists;
2. ownership of the business resource/action is attributable;
3. the Permission key follows `<Resource>.<Action>`;
4. the key does not collide with a materially different existing Permission;
5. it is not merely a UI/route/dashboard artifact;
6. its scope is not improperly encoded to bypass D02-003;
7. it is suitable for policy evaluation;
8. Role/policy references can resolve it deterministically;
9. governance/approval requirements defined upstream have been satisfied.

D02-002 does not invent the missing business approver if upstream is silent.

### 5.17 Permission Change / Compatibility

Because Permissions may be referenced by:

- Roles;
- policies;
- Technical Principal Authority Bindings;
- Delegations;
- Temporary Access;
- SoD definitions;
- Authorization Decisions/evidence;
- protected operation mappings;

silent semantic repurposing is unsafe.

Therefore:

- a Permission identifier already used for one material business meaning shall not be silently reused for a different meaning;
- changes shall preserve historical reconstructibility;
- dependent mappings/policies shall be impact-assessed;
- derived projections shall be invalidated/reconciled;
- exact canonical Permission version/lifecycle/event semantics remain upstream/governance dependent.

D02-002 does not declare whether a material change must create a new Permission ID/version because upstream does not yet define that rule.

### 5.18 Deprecation / Removal Safety

Where a Permission is intended to stop being used:

- new protected operation mappings shall not silently continue to require an obsolete semantic;
- active Role/policy/technical-principal/delegation/temp-access references shall be detected before destructive removal;
- historical decisions/evidence shall preserve the old reference;
- removal shall not reinterpret old Permission IDs;
- exact deprecation state names/events are not invented here.

### 5.19 Human and Non-Human Actors

For human subjects:

- normal permanent Permission availability derives through Role/Role Assignment;
- temporary/delegated/emergency mechanisms remain governed D02 authority sources.

For non-human principals:

- D01 supplies the service/workload/job/integration identity;
- the same Permission semantics protect business capabilities;
- a machine principal shall not require fabrication of a Person merely to receive authorization;
- D02 uses the `ADR-L5-0009` Technical Principal Authority Binding to associate the Tenant-bound non-human principal with explicit Permissions/capabilities and applicable scope/governance/validity references;
- the binding contributes candidate effective authority only and remains subject to D02-003 scope/resource and D02-004 policy/Deny evaluation.

### 5.20 Least-Privilege Granularity

Permission granularity shall support least privilege.

Too broad:

```text
Finance.All
ERP.AdminEverything
Student.FullAccess
```

may collapse materially different business actions into one opaque entitlement.

Too implementation-specific:

```text
StudentControllerMethod17
POST_Student_Endpoint_4
```

couples authority to technical implementation.

The target is a stable business-capability atom:

```text
Student.View
Student.Update
FinanceVoucher.Approve
Examination.PublishResults
```

The specification does not prescribe a universal maximum/minimum granularity; business semantics and least privilege govern the correct boundary.

### 5.21 Duplicate and Conflict Handling

| Condition | Required Technical Outcome |
|---|---|
| Same key + same semantic meaning registered repeatedly | Treat as same governed Permission reference; do not create extra authority. |
| Same key + materially different resource/action meaning | Reject as semantic collision; require governance resolution. |
| Different keys representing indistinguishable same capability | Flag duplication/normalization risk; do not silently merge historical references. |
| UI route proposes a new Permission with no distinct business capability | Reject as canonical Permission definition. |
| Permission embeds Tenant/department instance to avoid scope architecture | Reject/rework through D02-003 unless upstream explicitly defines a distinct capability. |
| Unknown Permission referenced by Role/Policy/Technical Principal Authority Binding | Treat reference as invalid; no permissive inference. |
| Permission removed while active dependencies exist | Block destructive removal or require governed migration/correction. |
| Permission source expires/revokes | Remove that source contribution from effective resolution; do not delete Permission catalog identity. |
| Permission is granted but policy Deny applies | D02-004 returns Deny; granted Permission does not override policy. |

### 5.22 Consistency / Caching / Projection

Technical implementations may maintain:

- Permission lookup indexes;
- Role→Permission projections;
- Technical Principal Authority Binding projections;
- effective Permission projections;
- compiled capability maps;
- distributed read caches.

They remain derivative.

Requirements:

- canonical authority remains authorization-owned;
- stale projections cannot create enduring authority after removal/revocation/change;
- projection keys retain source references/version/freshness metadata sufficient for safe invalidation;
- missing projection data shall not be treated as proof of Allow;
- exact caching, invalidation, polling, messaging or TTL strategy is D02-004/B02/C02 territory.

### 5.23 Evidence / Observability

Permission-resolution evidence may include:

- requested Permission reference/key;
- resolved resource/action;
- contributing Role/authority/Technical Principal Authority Binding references;
- subject/actor or non-human principal references;
- scope references;
- effective-time reference;
- catalog/provenance reference;
- resolution failure/collision reason;
- correlation identifier.

This evidence is not itself an Authorization Decision.

D04 owns evidence storage/provenance mechanics.

Operational metrics may track:

- invalid Permission references;
- mapping-resolution failures;
- catalog collisions;
- stale-projection detections;
- high-frequency Permission lookup failures.

No metric threshold is defined here.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Rationale / Basis | ADR |
|---|---|---|---|
| AD-001 | Permission semantics are modeled as governed Business Resource + Business Action. | Locked L2-A04 §§34–36/AP-04. | None; direct upstream rule. |
| AD-002 | Canonical Permission key shape is `<Resource>.<Action>`. | Locked L2-A04 §37. | None. |
| AD-003 | Permission identity is channel/UI independent. | L2-A04 AP-04/§42. | None. |
| AD-004 | Permission is the smallest assignable authorization unit in the D02 model; normal permanent human authority is composed through Roles/Role Assignments. | L2-A04 Permission model/assignment; L4-A-010. | `ADR-PLAT-AUTH-001` constrains subject ownership. |
| AD-005 | Capability Reference is a technical reference/projection of Permission semantics, not a new canonical business aggregate. | Frozen D02-002 responsibility + ownership preservation. | None. |
| AD-006 | Permission resolution produces candidate/effective authority facts, never final Allow. | D02-001; L4-A-010/A012/A051. | None. |
| AD-007 | Permission key shall not encode Tenant/Institution/lower-scope instances as a substitute for D02-003. | A04 separation; D02 corpus decomposition. | `ADR-L5-0004` constraint. |
| AD-008 | Role→Permission mapping cannot create business ownership or final authorization. | L2-A04; L4-A-010. | None. |
| AD-009 | Silent semantic repurposing of a referenced Permission identifier is prohibited because policy/decision history must remain reconstructible. | L4-A-052 historical traceability; architectural integrity. | None. |
| AD-010 | Derived Permission catalogs/projections/caches remain non-authoritative. | Canonical ownership + D02-001 freshness invariant. | None. |
| AD-011 | Human and machine actors use the same governed business Permission/capability semantics, while machine authority is carried through a distinct Technical Principal Authority Binding rather than fabricated Person/Role Assignment truth. | D01 non-human identity boundary; ownership preservation. | `ADR-L5-0009`. |
| AD-012 | No Permission management product, RBAC library, policy engine, claims format, cache, database or namespace technology is selected here. | L5 technology/ADR governance. | Future ADR only if required. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Least privilege | Business actions are represented as explicit Permission atoms rather than opaque broad access classes. | D02-002 |
| Cross-channel consistency | One Permission semantic is reusable across UI/API/service/job/workflow. | D02-002 + C01/G01/A05 |
| Tenant safety | Permission does not widen A04 scope; actual scope evaluated separately. | A04 + D02-003 |
| Policy safety | Resolved Permission never bypasses policy/SoD/Deny. | D02-004/007 |
| Identity integrity | Permanent human Permission is not attached directly to User Account/session. | D01 + D02 |
| Temporary authority | Delegation/temp/emergency sources remain derived and time/governance bounded. | D02-005/007 |
| Historical integrity | Permission reference meaning is not silently repurposed. | D02-002 + D04 |
| Business ownership | Resource/action semantics remain source-domain owned. | Business domains + D02 |
| Machine access | Machine principal receives bounded Permission/capability authority through the Tenant-bound Technical Principal Authority Binding without false Person ownership. | D01 + D02-002/003/004 + ADR-L5-0009 |
| Operational resilience | Derived maps/caches may optimize resolution but cannot become canonical authority. | B02/C02/D02-004 |
| Auditability | Resolution provenance feeds later decision/evidence reconstruction. | D04 |
| Privacy | Permission descriptors should not copy business-resource personal/sensitive content. | B05/D04 |

No numeric performance, availability, cache TTL, Permission-count or catalog-size target is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Conformance Requirement | Basis |
|---|---|---|
| CR-001 | Every canonical Permission shall represent a governed Business Resource + Business Action combination. | L2-A04 §§34–36. |
| CR-002 | Permission keys shall follow the governed `<Resource>.<Action>` semantic format unless upstream architecture formally changes the convention. | L2-A04 §37. |
| CR-003 | Canonical Permission definitions shall not be based on UI pages, navigation, dashboards, buttons or URLs. | AP-04; §42. |
| CR-004 | Transport/API route/method identifiers shall not replace business Permission semantics. | AD-003. |
| CR-005 | Normal permanent human authority shall not be implemented as direct permanent Permission assignment to Person/User Account/session where the governed model requires Role/Role Assignment. | L2-A04 §§38/42; L4-A-010. |
| CR-006 | Permission resolution shall retain the authority source/provenance through which the Permission became effective. | §§5.8–5.10. |
| CR-007 | A resolved/granted Permission shall never by itself be treated as final authorization to execute a protected operation. | L4-A-010/A012/A051; AD-006. |
| CR-008 | Explicit Deny and later policy/SoD/resource/scope evaluation shall remain capable of denying execution despite a resolved Permission. | L2-A04 §42; L4-A-051. |
| CR-009 | Permission identity shall not silently encode Tenant/Institution/lower-scope instances as a substitute for D02-003 scope evaluation. | AD-007. |
| CR-010 | Permission identity shall not encode subject/person/account identity. | Ownership separation. |
| CR-011 | Permission identity shall not encode transient Delegation, Temporary Access or emergency-grant state. | D02-005/007 separation. |
| CR-012 | Role→Permission mapping shall reference existing governed Permission identities and shall not create new Permissions implicitly. | §5.9. |
| CR-013 | Duplicate Role→Permission mappings shall not multiply effective authority. | §5.9. |
| CR-014 | Unknown/unresolvable Permission references in Role/Policy/protected-operation configuration shall not be treated as Allow. | Fail-closed baseline. |
| CR-015 | Same Permission key shall not represent materially different business capabilities. | §5.21. |
| CR-016 | Material semantic change shall not silently repurpose a historically referenced Permission identifier. | AD-009; L4-A-052 traceability. |
| CR-017 | Historical decisions/evidence shall retain sufficient Permission references to reconstruct evaluated capability semantics. | L4-A-052; D04 boundary. |
| CR-018 | Permission catalogs/registries/projections shall remain derived/authorization-owned technical views and shall not take ownership of business resources/actions. | §5.15. |
| CR-019 | Permission catalog admission shall validate business ownership/provenance and reject UI-only or ambiguous capability definitions. | §5.16. |
| CR-020 | Permission removal/deprecation shall not destructively erase references needed by historical decisions/evidence. | §§5.17–5.18. |
| CR-021 | Derived Permission caches/projections shall not remain permissive after authoritative mapping/grant expiry, revocation or removal. | §5.22. |
| CR-022 | D02-002 shall not define final lower-scope inheritance/resource-instance scope semantics owned by D02-003. | Family boundary. |
| CR-023 | D02-002 shall not define final policy decision/enforcement algorithms owned by D02-004. | Family boundary. |
| CR-024 | Delegation/Temporary/Privileged/Emergency authority shall not permanently mutate base Permission catalog semantics. | D02-005/006/007. |
| CR-025 | Workspace mapping or switching shall not create/remove Permissions or alter Permission ownership. | L4-A-050. |
| CR-026 | Non-human principals shall not be modeled as human Person/User Account subjects merely to reuse Permission assignment mechanics. | D01 boundary; AD-011. |
| CR-027 | Permission descriptors shall not require copying protected business-resource data into authorization storage. | Ownership/privacy boundary. |
| CR-028 | Business domains shall use the shared Permission/capability model rather than inventing channel-specific parallel permission systems. | L2-A04 AP-13 / D02-001. |
| CR-029 | Permission-resolution output shall be usable as input to D02-003/D02-004 without itself asserting final Allow. | AD-006. |
| CR-030 | No implementation shall infer a mandatory authorization library/vendor, policy language, claims representation, cache, database or numeric limit from this specification. | AD-012. |
| CR-031 | A non-human Permission/capability shall contribute to effective authority only through a valid, matching Tenant-bound Technical Principal Authority Binding and shall remain subject to D02-003/D02-004 evaluation. | ADR-L5-0009. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Source / Gap | Blocking? | Resolution |
|---|---|---|---|---|---|
| OD-001 | Applicable L4 authorization behaviors remain `Draft` in their own metadata. | Governance disposition | Controlled Stage-2 validation rule | No | Resolved for D02-002 approval interpretation; source L4 metadata remains unchanged. |
| OD-002 | D02-001 was a working authoring dependency. | Peer stabilization | D02 family reconciliation decision | No | Closed: D02-001 is committed and Architecture Approved; reopen only on material baseline change. |
| OD-003 | Exact global Permission namespace rules when two domains use the same Resource word. | L5 design | L2 fixes `<Resource>.<Action>` shape but not all cross-domain collision semantics. | No for architecture approval; yes before an enterprise catalog must resolve a real collision | Resolve through architecture governance; avoid local ad hoc prefixes without governance. |
| OD-004 | Canonical Permission lifecycle/state/commands/events for creation, change, deprecation or retirement are not sufficiently established upstream. | Upstream feature-realization gate | current L2/L4 coverage | No for reusable D02 architecture; yes for a feature requiring those exact lifecycle/event semantics | Preserve references/history and do not invent state/event names. |
| OD-005 | Whether every material Permission semantic change requires a new Permission identity/version is not defined upstream. | Upstream/L5 governance | L4-A-052 governs policy versions, not Permission versions. | No for architecture approval; yes before automated semantic-version governance | Require non-repurposing now; obtain explicit governance before automating version-change rules. |
| OD-006 | Exact machine-principal authority-assignment model for service/workload/job/integration identities. | Resolved cross-D01/D02 decision | `ADR-L5-0009` | No | Resolved by the approved Technical Principal Authority Binding pattern. |
| OD-007 | Exact Permission catalog distribution, caching and invalidation technology. | Technology / child design | B02/C02/D02-004 responsibility | No for architecture approval; yes before concrete implementation where material | Select under L5 technology/ADR governance. |
| OD-008 | Exact source/governance workflow that approves new domain Permission definitions. | Upstream feature-realization gate | A04 owns Permission but applicable L4 does not fully define Permission-definition governance. | No for reusable model; yes for self-service runtime Permission creation | Use governed architecture/configuration process until upstream workflow is defined. |
| OD-009 | Exact compatibility contract for removing a Permission with active Role/Policy/Technical Principal Authority Binding/Delegation/Temporary Access references. | L5/upstream | no complete upstream lifecycle contract | No for architecture approval; yes for destructive automated removal | Require dependency detection and explicit governance; do not infer cascade delete. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-D-02-002` realizes the frozen responsibility:

> Technical permission/capability representation without encoding business ownership into generic roles.

### 10.2 Primary Locked L2 Traceability

- L2-A04 AP-04: Permissions protect business resources and business actions, not UI.
- §34: Permission = Business Resource + Business Action.
- §§35–36: Resource and Action semantics.
- §37: `<Resource>.<Action>` naming convention.
- §38: Permissions are assigned to Roles and may become available through governed authority sources.
- §42: Permission invariants, indirect assignment, policy evaluation and Explicit Deny precedence.

### 10.3 Primary L4 Traceability

- `L4-A-010-BH-007` through `BH-010`: effective Role Assignment participation and no direct/final-decision shortcut.
- L4-A-011: delegated authority is temporary/derived.
- L4-A-012: temporary granted Permission does not automatically authorize execution.
- L4-A-013: emergency authority is bounded/non-permanent.
- L4-A-050: Workspace does not create/alter Permission authority.
- `L4-A-051-BH-005` through `BH-008`: effective authority + SoD + Deny.
- `L4-A-052-BH-004/BH-007/BH-008`: Permission references validated in policy and decisions retain exact policy-version history.

### 10.4 D02 Family Handoff

```text
D02-001 Authorization Baseline
        ↓
D02-002 Permission & Capability Model
        ├──► D02-003 Scope & Resource Context
        ├──► D02-004 Policy Decision & Enforcement
        ├──► D02-005 Delegation & Temporary Authority
        ├──► D02-006 Privileged & Administrative Access
        └──► D02-007 SoD / Maker-Checker / Emergency Override
```

### 10.5 Architecture Decisions

`AD-001` through `AD-012`.

### 10.6 Conformance Requirements

`CR-001` through `CR-031`.

### 10.7 Open Decisions

`OD-001` through `OD-009`.

### 10.8 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-D/
            └── L5-D-02-002_Permission_and_Capability_Model_Architecture.md
```

No nested `L5-D/L5-D-02/` folder is introduced.


## 2026-09-22 Batch 7B Impact Revalidation Synchronization

Non-material dependency-metadata synchronization only. Current peer lifecycle/version metadata is reconciled where explicitly stated; the controlled Stage-2 authorization for cited Draft L4 behavior remains unchanged. No authorization ownership, business authority, policy semantics, conformance requirement, traceability relation state, or Architecture Approved basis changed.
