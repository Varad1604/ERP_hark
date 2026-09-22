# Policy Decision & Enforcement Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-02-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-02 — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD Architecture  
**Canonical Technical Responsibility:** Technical policy evaluation and enforcement while business/institutional policy ownership remains upstream.  
**Technical Architecture Owner:** L5-D-02 — Authorization Architecture  
**Assigned Author:** Devang  
**Version:** `1.2.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-02-001_Authorization_Architecture_Baseline.md` v1.0.0 Architecture Approved; `L5-D-02-002_Permission_and_Capability_Model_Architecture.md` v1.0.0 Architecture Approved; `L5-D-02-003_Hierarchical_Scope_and_Resource_Context_Architecture.md` v1.0.0 Architecture Approved; `L5-D-01-001` through `L5-D-01-004`; `L5-A-04-001`; `L5-A-04-002`; `L5-D-03-001`; `L4-A-010` v1.0.0 Draft; `L4-A-011` v1.0.0 Draft; `L4-A-012` v1.0.0 Draft; `L4-A-013` v1.0.0 Draft; `L4-A-050` v1.0.0 Draft; `L4-A-051` v1.0.0 Draft; `L4-A-052` v1.0.0 Draft; `L2-A04_Authorization_and_Policy_Workflows.md` v1.1 Locked; `ADR-PLAT-AUTH-001`; `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0009`.  
**Referenced By:** `L5-D-02-005`; `L5-D-02-006`; `L5-D-02-007`; application/API/service/workflow/background/integration enforcement points; D04 Audit & Evidence; B02 persistence; C01/C02 contracts; E-family operational architecture.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001`, `ADR-L5-0001`, and `ADR-L5-0004` constrain subject, trust and Tenant-root scope. `ADR-L5-0009` governs non-human Technical Principal Authority Binding participation in policy evaluation and enforcement. No D02-004-specific product, policy-language, engine, cache, transport or deployment ADR is selected here.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical architecture that turns a complete authorization evaluation context into an enforceable Authorization Decision.

It is the D02 family control point between:

```text
identity + permission + scope + resource facts + policy
                         ↓
                 deterministic evaluation
                         ↓
                Authorization Decision
                         ↓
                 mandatory enforcement
                         ↓
                protected execution
```

The objective is to guarantee that:

- every protected operation is evaluated before execution;
- the authoritative current policy version is used;
- explicit Deny retains precedence;
- Permission possession alone never becomes execution authority;
- Role Assignment or Technical Principal Authority Binding alone never becomes final execution authority;
- SoD, delegated authority, temporary authority, privileged/emergency context and resource state can participate without bypass;
- Allow is bound to the exact subject/principal/action/resource/scope/policy context evaluated;
- Deny cannot be weakened by clients, downstream services or alternate channels;
- policy/enforcement failure does not degrade into permissive execution.

### 2.2 In Scope

- policy decision boundary;
- policy-context resolution;
- authoritative policy-version resolution;
- deterministic evaluation;
- explicit-deny precedence;
- evaluation ordering constraints;
- policy reference validation;
- Permission/Capability input consumption;
- Role Assignment and Technical Principal Authority Binding effective-authority consumption;
- scope/resource-context input consumption;
- SoD/delegation/temp/privileged/emergency input consumption;
- final Authorization Decision generation;
- decision provenance;
- decision freshness;
- policy-change invalidation/re-evaluation;
- authority-binding change/revocation invalidation/re-evaluation;
- decision caching constraints;
- enforcement-point architecture;
- enforcement at API, service, workflow, background, scheduled, integration and persistence-sensitive boundaries;
- collection/query enforcement semantics;
- bulk-operation enforcement;
- read/write/action enforcement;
- time-of-check/time-of-use concerns;
- nested service-call authorization;
- decision propagation versus re-evaluation;
- fail-closed handling;
- technical policy observability;
- decision/audit handoff.

### 2.3 Out of Scope / Non-Goals

This specification does not own:

- business/institutional policy content;
- policy authorship/approval authority where upstream is silent;
- policy business lifecycle states/events;
- Role Assignment lifecycle;
- non-human principal identity or credential lifecycle — D01/D03;
- Permission model definition — D02-002;
- hierarchical/resource scope semantics — D02-003;
- Delegation/Temporary Access lifecycle — D02-005;
- privileged/admin-access safeguards — D02-006;
- SoD/maker-checker/emergency technical specialization — D02-007;
- authentication;
- Tenant/Institution lifecycle;
- business resource state;
- organizational hierarchy;
- business workflow approval semantics;
- UI navigation;
- persistence/database technology;
- policy engine vendor;
- policy language;
- claims/token format;
- cache product;
- service mesh/API gateway product;
- numeric decision TTL, timeout, retry count, latency target or availability target.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority Matrix

| Source | Status | Applicable Authority | D02-004 Effect |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | D02-004 owns technical policy evaluation/enforcement while business/institutional policy ownership remains upstream. | Establishes exact technical responsibility. |
| `L5-D-02-001` | Architecture Approved | Authorization mandatory; centralized; policy-driven; explicit Deny; final Allow/Deny; decision/enforcement separation. | Supplies family-level invariants. |
| `L5-D-02-002` | Architecture Approved | Permission is Resource + Action, indirect authority input, not final decision. | D02-004 consumes resolved Permission/Capability. |
| `L5-D-02-003` | Architecture Approved | Tenant-rooted effective scope and resource state/ownership context. | D02-004 consumes authoritative scope/resource context. |
| L2-A04 AP-01 | Locked | Every protected business operation is authorized before execution. | Enforcement is mandatory. |
| L2-A04 AP-02 | Locked | Authentication and authorization are independent. | Authenticated context never substitutes for policy decision. |
| L2-A04 AP-03 | Locked | Server-side enforcement is mandatory. | Client/UI decisions are non-authoritative. |
| L2-A04 AP-05 | Locked | Authorization decisions are determined through policy evaluation, not independent hard-coded security logic. | Centralized policy decision model. |
| L2-A04 AP-06 | Locked | Explicit Deny overrides Allow. | Deny precedence is invariant. |
| L2-A04 AP-10 | Locked | Least privilege. | Policy may restrict otherwise available authority. |
| L2-A04 AP-11 | Locked | Authorization activity/decisions are auditable. | Decision provenance/evidence required. |
| L2-A04 AP-12 | Locked | No cross-Tenant authorization. | Tenant root is mandatory decision context. |
| L2-A04 AP-13 | Locked | Same authorization model across web/mobile/API/internal service/jobs/workers/integrations. | Enforcement semantics are channel-independent. |
| L4-A-010 | Draft | Active Role Assignment participates in later Role/Permission resolution but is not final Authorization Decision. | Entitlement fact must pass policy decision. |
| L4-A-011 | Draft | Delegated authority is temporary and governed. | Delegation is evaluation input, not bypass. |
| L4-A-012 | Draft | Temporary Access remains subject to later policy/resource/scope/SoD/Explicit Deny evaluation. | Temporary grant does not equal Allow. |
| L4-A-013 | Draft | Break-glass/emergency access remains bounded and auditable. | Emergency authority remains within central policy decision. |
| L4-A-050 | Draft | Workspace does not alter authorization outcome. | Workspace cannot influence final decision except as non-authoritative UX context. |
| L4-A-051 | Draft | SoD facts participate in centralized evaluation; no-conflict is not Allow; prohibited conflict Denies unless approved continuation and all remaining controls Allow. | SoD is one policy input; exception never bypasses full evaluation. |
| L4-A-052 | Draft | Current authoritative policy version must be used; deterministic ordering and explicit Deny preserved; decision binds exact policy version; history remains immutable. | Defines policy-version and historical-decision traceability requirements. |
| `ADR-L5-0001` | Architecture Approved | Default-deny protected boundary admission; explicit logical trust boundaries. | Policy/enforcement failure does not create bypass. |
| `ADR-L5-0004` | Architecture Approved | Tenant-rooted validated execution context. | Policy decision is bound to validated Tenant/Institution scope. |
| `ADR-L5-0009` | Architecture Approved | Non-human service/workload/job/integration principals receive bounded authority through a D02 Technical Principal Authority Binding. | D02-004 consumes the binding as an effective-authority input while still producing the final policy-governed Allow/Deny. |

For this controlled Stage-2 validation program, the cited canonical L4 System Behavior Specifications are accepted as behavioral authority even where their own metadata remains `Draft` or `Review`; that metadata status is not an approval blocker for D02-004.

### 3.2 Locked Policy/Decision Invariants

| Constraint ID | Invariant | Effect |
|---|---|---|
| C-001 | Every protected operation is evaluated before execution. | No bypass path. |
| C-002 | Authentication never implies authorization. | D01 context is input only. |
| C-003 | Permission grant alone is insufficient. | Policy/resource/scope/SoD/Deny remain applicable. |
| C-004 | Explicit Deny always overrides conflicting Allow. | Deny precedence cannot be configured away by L5. |
| C-005 | Policy evaluation is centralized and consistent across channels. | Business domains cannot implement weaker independent authorization. |
| C-006 | Evaluation order must be deterministic enough to avoid ambiguous outcomes. | Policy version cannot become authoritative if ordering is ambiguous. |
| C-007 | Tenant isolation and scope validity are mandatory prerequisites. | Cross-Tenant cannot be overruled by ordinary policy Allow. |
| C-008 | Least privilege is mandatory. | Policy may restrict broad Role/Permission authority. |
| C-009 | SoD absence is not an Allow. | Full evaluation continues. |
| C-010 | SoD permitted continuation is not a bypass. | Full evaluation still required. |
| C-011 | Current authoritative policy version is used for future requests. | Stale policy must not remain silently authoritative. |
| C-012 | Every Authorization Decision retains exact evaluated policy-version reference. | Historical reproducibility. |
| C-013 | Historical decisions and predecessor policy versions are not rewritten. | Audit integrity. |
| C-014 | Undefined successor activation/rollback semantics are not invented. | D02-004 consumes only authoritative version status/basis. |
| C-015 | Mandatory context uncertainty does not become Allow. | Fail closed. |
| C-016 | Enforcement is server-side/technical-boundary controlled. | Client claims are hints, not authority. |
| C-017 | Allow is context-bound, not a transferable blanket capability. | Changed material context requires new/revalidated decision. |
| C-018 | Protected-boundary security controls outside authorization still apply. | Authorization Allow does not bypass D03. |
| C-019 | A Technical Principal Authority Binding is an effective-authority input only; it is valid only for the matching D01 non-human principal, Tenant, capability and applicable scope/validity context. | Machine authority remains bounded and cannot bypass policy. |

### 3.3 Explicit Non-Assumptions

D02-004 does not assume:

- a named policy language;
- a specific RBAC/ABAC engine;
- a particular “PDP/PEP” product;
- a service-mesh authorization implementation;
- a gateway-only enforcement model;
- JWT-embedded authoritative permissions;
- a fixed policy cache TTL;
- an “Indeterminate” canonical business Authorization Decision outcome.

At the architecture boundary, protected execution resolves to **Allow or Deny**. Internal technical evaluation failures may have diagnostic classifications, but they shall not become permissive business outcomes.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 D02-004 Owns

- Authorization Policy Decision Boundary.
- Policy-version resolution interface.
- technical policy-evaluation orchestration.
- deterministic rule evaluation.
- explicit-deny combination semantics.
- decision-generation architecture.
- decision-binding context.
- decision freshness/revalidation semantics.
- Technical Principal Authority Binding participation in final evaluation.
- Policy Enforcement Boundary.
- enforcement-point obligations.
- query/bulk enforcement semantics.
- decision-cache safety constraints.
- authorization failure behavior.
- authorization health/operational semantics.
- decision evidence handoff.

### 4.2 D02-004 Does Not Own

- policy business content/authority;
- policy lifecycle business commands/events;
- Permission definition;
- scope hierarchy;
- human or non-human principal identity/authentication;
- business resource lifecycle;
- Delegation/Temporary Access lifecycle;
- SoD business rules;
- privileged emergency approval authority;
- deployment/network trust;
- persistence engine;
- API gateway or runtime technology.

### 4.3 Collaborating Architecture

| Architecture | Supplies / Owns | D02-004 Use |
|---|---|---|
| D01 | authenticated/attributable human/non-human actor | subject/principal/actor input. |
| A04 | validated Tenant/Institution execution context | root scope prerequisite. |
| D02-002 | Permission/Capability resolution and TPAB permission references | protected capability/effective-authority input. |
| D02-003 | effective scope/resource state/ownership context and TPAB scope compatibility | scope/resource policy input. |
| D02-005 | Delegation/Temporary Authority | effective authority input. |
| D02-006 | privileged/admin authority | privileged context input. |
| D02-007 | SoD/maker-checker/emergency controls | specialized deny/continuation inputs. |
| D03 | trust-boundary/security admission | authorization is one mandatory boundary control. |
| D04 | audit/evidence | receives decision/evaluation provenance. |
| C01 | synchronous contracts | transports request/decision context. |
| C02 | change/invalidation messaging | may propagate authoritative change notifications. |
| B02 | persistence/consistency | supports policy/authority/resource state and enforcement consistency. |
| A05 | workflow/orchestration | calls D02-004 for protected tasks/actions. |
| E01/E02/E05 | resilience/performance/observability | technical operational realization without weakening fail-closed semantics. |

---

## 5. Solution Architecture

### 5.1 Authorization Decision Pipeline

```text
Protected Operation Request
        ↓
Actor / Subject / Technical Principal Context ← D01
        ↓
Validated Tenant / Institution                ← A04
        ↓
Permission / Capability                       ← D02-002
        ↓
Scope / Resource Context                      ← D02-003
        ↓
Effective Authority Inputs
(Role Assignment / Technical Principal Authority Binding /
 Delegation / Temporary / Privileged)
        ↓
Applicable Policy Version Resolver
        ↓
Policy Context Validation
        ↓
Deterministic Policy Evaluation
        ↓
Explicit Deny Combination
        ↓
Final Authorization Decision
      Allow | Deny
        ↓
Policy Enforcement Boundary
        ↓
Protected Operation Executes
only when decision is valid Allow
```

### 5.2 Logical Components

| Component | Responsibility | Failure Rule |
|---|---|---|
| Authorization Request Gate | Accept complete protected-operation context and reject structurally invalid requests. | Missing mandatory context → no Allow. |
| Policy Applicability Resolver | Determines which governed policy/version applies to Tenant/scope/action/resource. | Ambiguous/unresolved applicable policy → no permissive default. |
| Policy Version Authority Adapter | Obtains the authoritative current policy version/basis without owning policy lifecycle. | Successor not authoritatively active → use established authoritative version or deny if none exists. |
| Context Completeness Validator | Verifies human subject or non-human principal, Permission, scope, resource facts, authority sources, time and policy references. | Missing mandatory fact → Deny/reject. |
| Policy Evaluation Boundary | Evaluates applicable rules in deterministic order. | Non-deterministic/invalid evaluation → no Allow. |
| Deny Combination Control | Ensures explicit Deny precedence. | Any applicable authoritative explicit Deny → final Deny unless source-defined exceptional continuation semantics explicitly require further governed evaluation; never silent override. |
| Specialized-Control Adapter | Incorporates SoD, delegation/temp, privileged/emergency controls from D02-005/006/007. | Unavailable mandatory control → no Allow. |
| Authorization Decision Generator | Produces context-bound Allow/Deny plus provenance. | Cannot establish complete authoritative basis → Deny. |
| Decision Freshness Validator | Ensures decision remains valid against policy/authority/resource/binding changes. | Material staleness → re-evaluate or reject. |
| Policy Enforcement Boundary | Blocks protected operation unless valid Allow matches actual execution context. | Mismatch or missing decision → block. |
| Evidence Handoff | Emits non-secret decision/evaluation facts to D04. | Mandatory evidence failure must not be hidden. |

### 5.3 Authorization Evaluation Context

Conceptually:

```text
AuthorizationEvaluationContext
{
  tenant_reference
  institution_reference
  lower_scope_references[]

  subject_reference where human authority applies
  technical_principal_reference where non-human authority applies
  actor_reference
  actor_subject_relationship

  permission_reference
  action_reference
  resource_reference

  resource_scope_context
  resource_state_context
  resource_ownership_context

  effective_role_assignment_references[]
  technical_principal_authority_binding_reference(s)
  delegation_reference(s)
  temporary_authority_reference(s)
  privileged_or_emergency_reference(s)

  sod_context
  workflow_or_process_context where required

  authoritative_policy_version_reference
  institution_configuration_reference(s)

  evaluation_time
  request_provenance
  correlation_reference
}
```

The exact data representation is not prescribed.

### 5.4 Policy Applicability Resolution

Policy applicability is resolved from governed context such as:

- Tenant;
- Institution;
- Permission/action;
- resource/resource type;
- lower scope;
- business-process/workflow context where source policy requires it;
- effective time;
- permitted institutional configuration.

D02-004 does not invent which policy applies. It implements the technical resolution of upstream-owned policy applicability.

### 5.5 Authoritative Policy Version Resolution

L4-A-052 requires future requests to use the authoritative current policy version.

Therefore:

1. the evaluator shall resolve the authoritative policy version before decision generation;
2. a draft/proposed/successor technical artifact is not authoritative merely because it exists;
3. predecessor policy definitions remain immutable for history;
4. historical decisions retain their evaluated version;
5. rollback uses only a future source-defined canonical rollback contract;
6. no L5 component may invent `PolicyActivated`, `PolicySuperseded`, or similar canonical events where upstream has not established them.

### 5.6 Deterministic Evaluation

For identical materially relevant authoritative inputs, the evaluation architecture shall produce the same governed result.

Determinism requires:

- unambiguous applicable policy version;
- deterministic rule ordering/combination semantics;
- stable Permission references;
- stable scope/resource inputs;
- stable effective-authority references including Technical Principal Authority Binding where applicable;
- explicit handling of Deny precedence;
- no dependency on unordered implementation behavior;
- no dependency on ambient client/session UI state;
- no random or time-varying decision input unless time itself is an explicit governed policy attribute.

The architecture does not require a specific policy language.

### 5.7 Policy Evaluation Sequence

A conceptual evaluation order is:

```text
1. Validate attributable human subject / non-human principal / actor.
2. Validate Tenant / Institution context.
3. Validate requested Permission / action / resource.
4. Validate effective authority source(s), including TPAB when non-human.
5. Validate effective scope / resource compatibility.
6. Resolve authoritative policy version.
7. Resolve mandatory policy attributes / resource facts.
8. Evaluate Delegation / Temporary / Privileged controls as applicable.
9. Evaluate SoD / maker-checker / emergency constraints as applicable.
10. Evaluate remaining policy conditions.
11. Apply Explicit Deny precedence.
12. Generate final Authorization Decision.
13. Bind decision to exact evaluation context.
14. Enforce before protected execution.
15. Preserve evidence.
```

This is architecture-level ordering. A compliant implementation may internally optimize evaluation provided the observable semantics remain equivalent.

### 5.8 Explicit Deny Precedence

Deny precedence is non-negotiable.

```text
governed effective authority
      +
resolved Permission
      +
valid scope
      +
other Allow rules
      +
applicable Explicit Deny
      =
DENY
```

D02-004 shall not provide a generic “Allow beats Deny”, “most-specific wins”, or “last rule wins” configuration that can weaken the locked invariant.

If upstream specifically defines an exceptional continuation path, such as a source-governed SoD continuation case, that path still undergoes the complete remaining authorization evaluation. It is not an override of the central authorization architecture.

### 5.9 No-Conflict / Positive-Fact Semantics

The following facts are not independent Allows:

- authenticated human user;
- authenticated non-human principal;
- active account;
- active Role Assignment;
- valid Technical Principal Authority Binding;
- resolved Permission;
- matching scope;
- no SoD conflict;
- valid Delegation;
- valid Temporary Access;
- valid privileged authority;
- approved emergency basis;
- matching Workspace.

They are inputs/prerequisites only.

Final Allow exists only when the complete policy evaluation returns Allow.

### 5.10 Authorization Decision Contract

Conceptually:

```text
AuthorizationDecision
{
  outcome: Allow | Deny

  tenant_reference
  institution / lower_scope_reference(s)

  subject_reference where applicable
  technical_principal_reference where applicable
  actor_reference

  permission / action_reference
  resource_reference

  evaluated_policy_version_reference
  evaluated_authority_source_references[]
  evaluated_scope_reference(s)

  decision_basis_reference(s)
  deny_reason_reference(s) where applicable

  evaluation_time
  freshness / validity_reference
  correlation_reference
  evidence_reference
}
```

This contract is semantic, not a prescribed API schema.

### 5.11 Decision Binding

An Allow is valid only for the exact context it was generated for.

Materially changing any of the following invalidates direct reuse unless a governed decision contract explicitly supports the new context:

- Tenant;
- Institution;
- human subject or non-human technical principal;
- actor when actor/subject distinction matters;
- Permission/action;
- resource;
- lower scope;
- resource state/ownership relevant to policy;
- policy version;
- Role Assignment or Technical Principal Authority Binding basis;
- Delegation/Temporary/privileged/emergency basis;
- SoD/process context;
- effective time constraints.

An Allow for one resource shall not authorize a different resource by client substitution.

### 5.12 Decision Freshness

Authorization facts change.

Examples:

- Role Assignment revoked/expired;
- Technical Principal Authority Binding expires, is revoked/disabled, narrowed, or becomes stale;
- Permission removed from Role or Technical Principal Authority Binding;
- Delegation revoked;
- Temporary Access expired;
- emergency authority closed;
- policy version changed;
- resource state changed;
- ownership/relationship changed;
- scope changed.

D02-004 requires a technical freshness strategy that prevents stale permissive authority from surviving material source change.

The exact mechanism may include:

- direct current-state resolution;
- version comparison;
- invalidation;
- event-driven change propagation;
- bounded caching;
- re-evaluation.

No specific technology or numeric TTL is selected here.

### 5.13 Decision Caching

Decision caching is optional, not mandatory.

If used:

- cache key/context must include all materially decision-relevant identifiers/versions, including TPAB reference/version/freshness where applicable;
- Tenant must be part of the isolation boundary;
- Deny precedence must remain intact;
- policy/authority change must invalidate or supersede stale decisions;
- cross-resource reuse is prohibited unless resource context is proven equivalent by design;
- client-provided cached Allow is never authoritative;
- cache outage/miss does not imply Allow;
- historical decision evidence is not rewritten when cache entries expire.

### 5.14 Permission / Effective-Authority Caching

Caching an effective Permission set is not equivalent to caching the final decision.

Even when the Permission set or Technical Principal Authority Binding is current, the request may still be denied due to:

- scope;
- resource state;
- ownership;
- SoD;
- policy;
- time;
- explicit Deny;
- emergency/privileged restrictions.

### 5.15 Enforcement Boundary

Every protected operation has at least one authoritative technical enforcement boundary.

The enforcement boundary:

1. obtains or invokes an authoritative D02 Authorization Decision;
2. confirms the decision matches actual subject/principal/action/resource/scope;
3. blocks execution for Deny;
4. blocks execution for missing/invalid/stale decision;
5. executes only on valid Allow;
6. does not reinterpret policy;
7. preserves evidence/correlation.

### 5.16 Decision Point and Enforcement Point Separation

Conceptually:

```text
Policy Decision Boundary
        ↓ decision
Policy Enforcement Boundary
        ↓
Protected Resource / Operation
```

The two may be implemented in the same runtime component or separately.

Architecture requires semantic separation, not physical deployment separation.

### 5.17 Enforcement Placement

Authorization may need enforcement at multiple technical boundaries:

- application service;
- API/service operation;
- domain application command;
- workflow task execution;
- background/scheduled job operation;
- integration command;
- sensitive query/data-access boundary;
- privileged administrative operation.

A gateway-only check is insufficient where internal calls can reach protected business operations without passing the gateway.

### 5.18 Defense Against Internal Bypass

Internal placement does not imply authorization.

If an internal service or worker invokes a protected operation:

- D01 non-human principal remains attributable;
- the applicable Technical Principal Authority Binding is resolved and must be valid;
- Tenant/scope remains validated;
- required Permission/policy evaluation still occurs;
- the internal network, service identity or credential alone does not create Allow.

D03 trust and D02 authorization both apply.

### 5.19 Client-Side Authorization Hints

Clients may request authorization hints for UX purposes, such as whether to display an action.

Such hints:

- may improve usability;
- may be derived from current Permission/policy context;
- are not authoritative;
- may become stale;
- cannot replace server-side enforcement.

A hidden button is not access control.

### 5.20 API Enforcement

For API/service operations:

```text
API Request
   ↓
authenticate / establish actor or technical principal
   ↓
validate Tenant/scope
   ↓
resolve authority + action/resource
   ↓
D02 decision
   ↓
Allow → execute
Deny  → reject
```

HTTP status/error mapping is C01/API implementation concern, not D02 business semantics.

### 5.21 Workflow Enforcement

A workflow being valid/running does not automatically authorize the human or machine actor to perform every workflow task.

Protected workflow actions shall evaluate:

- current actor / technical principal;
- applicable Role Assignment or Technical Principal Authority Binding;
- Permission;
- scope/resource;
- workflow/process context if policy requires;
- other authority source(s);
- SoD/maker-checker constraints;
- current policy version.

A05 owns workflow execution. D02 owns authorization decision.

### 5.22 Background / Scheduled Execution

A scheduled job or worker is not exempt from authorization.

Where a job performs protected business action:

- D01 supplies attributable non-human principal;
- A04 supplies Tenant scope;
- D02 resolves the applicable `ADR-L5-0009` Technical Principal Authority Binding;
- D02-002 resolves governed Permission/capability authority;
- D02-003 validates binding/request/resource scope;
- D02-004 evaluates policy and produces Allow/Deny;
- enforcement occurs before mutation/disclosure.

The interactive user who originally scheduled the job does not automatically remain the valid authority at future execution unless upstream semantics explicitly establish such delegated authority.

### 5.23 Integration Enforcement

External integration credentials establish technical identity/trust, not authorization.

Protected integration operations require:

- valid D01 integration principal;
- valid Technical Principal Authority Binding;
- Tenant/scope;
- required Permission/capability;
- resource context;
- applicable policy.

An integration credential cannot grant every operation merely by authentication success.

### 5.24 Persistence-Sensitive Enforcement

D02-004 does not own database access-control technology, but protected reads/writes must not bypass application authorization through unrestricted persistence paths.

B02 shall collaborate so that:

- query result domain respects authorized scope;
- writes cannot target resources outside authorized context;
- shared storage does not create cross-Tenant access;
- persistence-layer optimizations do not weaken authorization.

### 5.25 Collection / Search / List Enforcement

Collection requests are especially sensitive.

A Permission such as:

```text
Student.View
```

does not imply:

```text
view every Student in the Tenant
```

unless scope/policy explicitly says so.

The enforcement architecture shall constrain:

```text
candidate dataset
  ∩ authorized scope
  ∩ resource/policy constraints
  =
permitted result set
```

Applying authorization only after retrieving an unrestricted dataset is unacceptable where it can cause disclosure or bypass.

Exact query-rewrite/filter technology belongs B02/D02 implementation.

### 5.26 Bulk Mutation Enforcement

Bulk operations shall not rely on one sample item.

For:

```text
approve R1 ... Rn
```

the enforcement architecture must establish that every affected resource is covered by the decision model or by a proven homogeneous authorization context.

Mixed-Tenant mutation is prohibited.

Mixed-resource outcomes must follow explicit contract semantics; D02-004 does not invent partial-success business behavior.

### 5.27 Read vs Write vs Administrative Action

All protected operation types may require authorization.

The architecture does not assume reads are harmless.

Protected categories may include:

- read/view;
- create;
- update;
- delete/archive;
- approve;
- publish;
- export;
- download;
- configure;
- assign;
- delegate;
- administer;
- recover;
- override.

Permission definitions remain D02-002.

### 5.28 Resource State and TOCTOU

If policy depends on current resource state, there is a risk that the state changes between decision and mutation.

D02-004 requires the implementation to preserve policy correctness across the decision/execution window.

Possible architecture techniques may include:

- revalidation at mutation;
- transactional condition checks;
- optimistic concurrency/version checks;
- resource-version binding;
- short-lived decision validity.

The exact strategy belongs D02-004/B02 implementation and technology governance.

No numeric window is prescribed.

### 5.29 Scope TOCTOU

Scope/ownership relationships and Technical Principal Authority Binding scope/validity may also change.

Example:

- authority valid when request begins;
- subject/resource reassigned or machine binding narrowed/revoked before mutation;
- stale Allow would now be too broad.

For high-risk operations, enforcement must use a consistency strategy appropriate to the authoritative scope/resource/authority source.

### 5.30 Nested Service Calls

A protected request may cross multiple services.

D02-004 distinguishes:

**decision propagation** from **authority propagation**.

A downstream service shall not trust an upstream “allowed=true” assertion unless the technical contract guarantees:

- source authenticity;
- Tenant binding;
- human subject or non-human technical principal binding;
- authority-source binding where material;
- action/resource binding;
- policy-version/freshness validity;
- non-replay beyond intended context;
- downstream operation equivalence.

Otherwise, the downstream protected boundary re-evaluates.

No token/claims format is chosen here.

### 5.31 Re-Evaluation vs Decision Reuse

Re-evaluation is required when:

- protected action changes;
- resource changes;
- Tenant/Institution changes;
- material lower scope changes;
- policy version changes;
- Role Assignment or Technical Principal Authority Binding changes;
- another authority source changes;
- resource state/ownership changes materially;
- downstream service operation is broader/different;
- freshness cannot be proven.

Decision reuse may be permitted only when the implementation can prove the original decision contract exactly covers the continued operation.

### 5.32 Long-Lived Processes

Long-lived workflows/jobs shall not assume an initial Allow remains valid forever.

Reauthorization may be required at protected execution points based on:

- current actor / technical principal;
- current authority including TPAB validity where applicable;
- current policy;
- current resource state;
- current scope.

Exact cadence is process/implementation-specific and is not numerically defined.

### 5.33 SoD Integration

D02-007 supplies detailed SoD/maker-checker/emergency semantics.

D02-004 guarantees:

- SoD evaluation participates before final Allow when applicable;
- detected prohibited conflict produces Deny by default;
- absence of conflict is not Allow;
- approved continuation still evaluates all other controls;
- SoD control unavailable where mandatory → no Allow;
- decision evidence records applicable SoD outcome/basis.

### 5.34 Delegation and Temporary Access Integration

D02-005 supplies effective delegated/temp authority.

D02-004 guarantees:

- expired/revoked authority is excluded;
- scope/effective period is respected;
- source provenance remains visible;
- delegated/temp grant does not bypass policy;
- a granted Permission can still be Denied.

### 5.35 Privileged / Administrative Integration

D02-006 supplies privileged authority/safeguards.

D02-004 guarantees:

- privileged human or non-human operation remains explicitly authorized;
- non-human privileged execution uses the applicable Technical Principal Authority Binding plus D02-006 safeguards;
- “admin” or infrastructure status is not blanket bypass;
- protected operation/scope remain bound to decision;
- privileged evidence requirements can be enforced.

### 5.36 Emergency / Break-Glass Integration

Emergency access does not create a second independent authorization system.

```text
Emergency Authority
        ↓
D02-007 controls
        ↓
D02-004 central evaluation
        ↓
final Allow / Deny
```

The emergency basis may change applicable policy conditions only where upstream explicitly allows it.

### 5.37 Policy Configuration Integrity

Institution configuration may affect policy only within permitted upstream boundaries.

D02-004 shall reject or refuse authoritative use of configuration that:

- breaks Tenant isolation;
- creates nondeterministic evaluation;
- weakens explicit Deny;
- references invalid Role/Permission/scope/Technical Principal Authority Binding;
- exceeds permitted institutional override boundaries;
- lacks required upstream governance evidence.

### 5.38 Policy Successor Handling

A successor policy definition may exist before it is authoritatively usable.

D02-004 shall not decide based on:

- file modification time;
- deployment presence;
- latest database row merely by sequence;
- client-requested policy version;

unless those are part of a governed authoritative activation contract.

Current authoritative version selection must follow upstream policy governance.

### 5.39 Policy Rollback

Rollback semantics are upstream-deferred.

Therefore D02-004:

- preserves policy history;
- can consume a prior version only when governance establishes it as authoritative;
- does not rewrite decisions made under later/other versions;
- does not invent rollback commands/events.

### 5.40 Decision Reason Model

A Deny should be technically attributable to a stable internal reason classification sufficient for troubleshooting/evidence.

Conceptual categories may include:

```text
missing_identity_context
technical_principal_binding_missing_or_invalid
tenant_mismatch
scope_mismatch
permission_not_effective
policy_explicit_deny
sod_conflict
delegation_expired
temporary_access_expired
resource_state_restriction
resource_ownership_restriction
policy_context_incomplete
policy_version_unresolved
authorization_control_unavailable
```

These are technical diagnostic categories, not new canonical business events.

External/user-facing error disclosure may be more limited for security/privacy reasons.

### 5.41 Decision Evidence

D04 may receive:

- outcome;
- human subject/actor or non-human technical principal/actor;
- Technical Principal Authority Binding reference where applicable;
- Tenant/Institution;
- Permission/action;
- resource;
- effective authority references;
- applicable scope;
- evaluated policy version;
- applicable SoD/delegation/temp/privileged/emergency facts;
- decision reason;
- evaluation time;
- enforcement point;
- correlation/provenance.

Secrets/credentials shall not be copied into decision evidence.

### 5.42 Policy Evaluation Failure

| Failure | Required Outcome |
|---|---|
| Policy service unavailable | Protected operation does not bypass; deny/fail closed. |
| Applicable policy unresolved | No Allow. |
| Policy version ambiguous | No Allow. |
| Policy reference invalid | No Allow. |
| Required non-human Technical Principal Authority Binding missing/invalid | No Allow. |
| Evaluation throws/aborts | No Allow. |
| Required resource fact unavailable | No Allow. |
| Required SoD/delegation/temp control unavailable | No Allow. |
| Decision serialization/transport invalid | Enforcement rejects. |
| Decision mismatches actual action/resource/scope/principal | Enforcement rejects. |
| Evidence correlation missing where mandatory | Do not silently report controlled success. |

### 5.43 Enforcement Failure

If enforcement infrastructure fails:

- protected execution shall not proceed through an unguarded alternate path;
- retry/failover may be used if it preserves exact policy semantics;
- fallback to a weaker local allow-list is prohibited unless formally governed as equivalent architecture;
- operational failure is distinguishable from business Deny but remains non-permissive for the operation.

### 5.44 High Availability and Failover

Availability design may replicate decision/enforcement components, but:

- replicas must use compatible authoritative policy versions;
- replicas must not use stale permissive authority-binding state;
- failover cannot switch to stale broad policy silently;
- Tenant isolation remains;
- explicit Deny remains;
- evidence remains attributable;
- exact topology belongs E01/E04/E05 and technology ADRs.

### 5.45 Version Compatibility

During rolling change or distributed deployment, multiple component versions may temporarily coexist.

D02-004 requires that:

- policy/decision contracts remain compatible;
- a consumer that cannot interpret required policy/decision/TPAB semantics fails safely;
- old code must not reinterpret new Deny semantics as Allow;
- unsupported new Permission/scope/authority attributes do not default permissively.

### 5.46 Observability

Operational observability may include:

- decision request counts;
- Allow/Deny counts;
- evaluation failures;
- policy-resolution failures;
- invalid/stale Technical Principal Authority Binding detections;
- stale-decision detections;
- cache hit/miss;
- enforcement rejection due to context mismatch;
- policy-version skew;
- dependency latency/errors.

Operational telemetry is not a substitute for D04 evidence.

No alert thresholds are defined.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Rationale / Basis | ADR |
|---|---|---|---|
| AD-001 | Every protected operation passes a centralized authorization decision boundary before execution. | L2-A04 AP-01/AP-05/AP-13. | None; direct locked rule. |
| AD-002 | Policy evaluation consumes D02-002 Permission and D02-003 scope/resource context; neither independently grants access. | D02 family separation. | None. |
| AD-003 | Explicit Deny has precedence over conflicting Allow. | L2-A04 AP-06; L4-A-051/A052. | None. |
| AD-004 | Evaluation semantics are deterministic for identical materially relevant authoritative inputs. | L4-A-052-BH-003. | None. |
| AD-005 | Future decisions bind the exact authoritative policy version evaluated. | L4-A-052-BH-007/BH-008. | None. |
| AD-006 | D02-004 consumes authoritative policy-version status but does not invent policy lifecycle/activation/rollback business semantics. | L4-A-052 open issues. | None. |
| AD-007 | Final Authorization Decision is context-bound Allow or Deny. Technical uncertainty/failure is non-permissive. | D02 baseline + fail-closed security. | `ADR-L5-0001` constraint. |
| AD-008 | Decision generation and enforcement are semantically separate; enforcement cannot reinterpret Deny or widen Allow. | L2-A04 AP-03. | None. |
| AD-009 | Server-side/service-side enforcement is mandatory; client/UI authorization is advisory only. | AP-03/AP-09. | None. |
| AD-010 | Decision caching is optional and derivative; stale permissive decisions must be invalidated/re-evaluated. | L4-A-052 + D02 freshness. | None; implementation ADR if needed. |
| AD-011 | Protected internal service/job/integration calls remain subject to authorization; internal trust is not authorization. | AP-13 + D03. | `ADR-L5-0001`. |
| AD-012 | Collection and bulk enforcement constrains all affected resources to authorized scope. | D02-003 scope model. | None. |
| AD-013 | Resource/scope/authority state affecting policy must be sufficiently current at enforcement to avoid stale permissive access. | Policy correctness / TOCTOU. | None. |
| AD-014 | No policy product, language, engine, gateway, service mesh, claims format, cache, database, TTL, timeout or deployment topology is selected by this specification. | L5 technology governance. | Future ADR only where material. |
| AD-015 | Non-human protected actions consume an applicable `ADR-L5-0009` Technical Principal Authority Binding as an effective-authority input; that binding never bypasses D02-004 policy evaluation or final Allow/Deny generation. | D01 non-human identity + D02 authorization ownership. | `ADR-L5-0009`. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Unauthorized execution | Mandatory decision + enforcement prevents bypass. | D02-004 |
| Cross-Tenant access | Tenant mismatch cannot be allowed by policy. | A04 + D02-003/004 |
| Machine authorization | Non-human principal requires matching Tenant-bound TPAB and full policy evaluation; credential/internal placement alone never grants authority. | D01 + D02-002/003/004 + ADR-L5-0009 |
| Over-privilege | Policy can restrict granted Permission using scope/state/SoD. | D02-004 |
| Stale access | Policy/authority/resource change triggers invalidation/re-evaluation requirements. | D02-004 + B02/C02 |
| Client tampering | Client Allow/hints are ignored as authority. | D02-004 + G01 |
| Internal bypass | Service/job/integration paths remain protected. | D02-004 + D03 |
| Query leakage | Collection results constrained before disclosure. | D02-003/004 + B02 |
| Bulk mutation | Every affected item or proven homogeneous scope must be authorized. | D02-004 |
| Privileged misuse | Admin/emergency remains policy-controlled. | D02-006/007 + D02-004 |
| SoD | Prohibited conflict Denies by default. | D02-007 + D02-004 |
| Auditability | Exact policy-version and decision context preserved. | D04 |
| Privacy | Deny reasons exposed externally may be minimized while full internal evidence remains governed. | D04/B05 |
| Availability | Authorization outage is non-permissive. | E01/E05 + D02 |
| Performance | Caching/compilation allowed only without weakening freshness/semantics. | E02/B02/D02-004 |

No numeric SLA, SLO, latency, throughput, availability, cache duration or retry target is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Conformance Requirement | Basis |
|---|---|---|
| CR-001 | Every protected business operation shall receive an authoritative D02 Authorization Decision before execution. | AP-01. |
| CR-002 | Authentication success shall never be accepted as authorization success. | AP-02. |
| CR-003 | A resolved Permission, active Role Assignment or valid Technical Principal Authority Binding shall never by itself be treated as final Allow. | L4-A-010; D02-002; ADR-L5-0009. |
| CR-004 | Authorization policy evaluation shall remain centralized/shared across protected channels. | AP-05/AP-13. |
| CR-005 | Explicit Deny shall override conflicting Allow. | AP-06. |
| CR-006 | Policy evaluation shall use one unambiguous authoritative policy version. | L4-A-052. |
| CR-007 | Future Authorization Decisions shall bind the exact policy version evaluated. | L4-A-052-BH-007. |
| CR-008 | Historical Authorization Decisions shall preserve original policy-version references. | L4-A-052-BH-008. |
| CR-009 | Policy predecessor/history shall not be rewritten as part of successor activation or rollback. | L4-A-052. |
| CR-010 | D02-004 shall not invent missing policy lifecycle states, activation commands, rollback commands or canonical events. | L4-A-052 open issues. |
| CR-011 | Evaluation ordering/combination shall be deterministic enough to avoid ambiguous outcomes. | L4-A-052-BH-003. |
| CR-012 | Tenant, scope, effective-authority, Permission and SoD references required by policy shall be validated before Allow. | L4-A-052-BH-004; ADR-L5-0009. |
| CR-013 | Absence of SoD conflict shall not independently generate Allow. | L4-A-051-BH-006. |
| CR-014 | Prohibited SoD conflict shall Deny unless an already-approved policy explicitly permits continuation and the complete remaining evaluation Allows. | L4-A-051-BH-007/BH-008. |
| CR-015 | Delegation/Temporary Access/privileged/emergency authority shall remain policy inputs and shall not bypass final evaluation. | D02-005/006/007 boundaries. |
| CR-016 | Mandatory context uncertainty or control unavailability shall not default to Allow. | Fail-closed baseline. |
| CR-017 | Final Allow shall be bound to the evaluated human subject or non-human principal, action/Permission, resource, Tenant/scope and policy version. | §5.11. |
| CR-018 | Enforcement shall reject decisions whose context does not match the actual protected execution. | §5.15. |
| CR-019 | Client/UI authorization hints shall never replace server-side enforcement. | AP-03/AP-09. |
| CR-020 | Internal service, background, workflow and integration paths shall not be exempt from authorization. | AP-13. |
| CR-021 | Gateway-level authorization alone shall not be accepted if protected internal operations remain reachable through bypass paths. | §5.17–5.18. |
| CR-022 | Decision caching, when used, shall include sufficient context/versioning to prevent cross-resource/cross-Tenant or stale permissive reuse. | §5.13. |
| CR-023 | Cache miss/outage shall not produce Allow. | Fail-closed. |
| CR-024 | Material authority/policy/resource-state/scope change shall cause stale permissive decisions to be rejected or re-evaluated. | §5.12. |
| CR-025 | Collection/query enforcement shall constrain results to authorized scope before protected disclosure. | §5.25. |
| CR-026 | Bulk mutation shall validate all affected resources or a proven homogeneous authorization context; cross-Tenant bulk mutation is prohibited. | §5.26. |
| CR-027 | Resource-state/ownership facts used by policy shall remain source-owned and sufficiently current for the protected operation. | D02-003 / §5.28–5.29. |
| CR-028 | Decision propagation across services shall not be trusted unless principal/subject/action/resource/scope/policy/freshness binding is verifiable; otherwise downstream re-evaluates. | §5.30–5.31. |
| CR-029 | Authorization Allow shall not bypass D03 trust, cryptographic, secret, credential or segmentation controls. | D03 boundary. |
| CR-030 | Authorization-control failure shall not trigger a weaker local fallback that changes governed decision semantics. | §5.43. |
| CR-031 | Policy-version or authority-binding skew across replicas/failover shall not result in silent broader authority. | §5.44. |
| CR-032 | Unsupported/new policy or authority-binding semantics shall fail safely rather than being interpreted permissively by older components. | §5.45. |
| CR-033 | Material Authorization Decisions shall expose non-secret provenance to D04. | AP-11. |
| CR-034 | External-facing Deny messages need not expose sensitive policy internals; internal evidence shall remain sufficient for governed reconstruction. | Security/privacy. |
| CR-035 | D02-004 shall not mutate upstream business resource state as part of authorization evaluation. | Ownership boundary. |
| CR-036 | D02-004 shall not become owner of Role/Permission/scope/Delegation/SoD business lifecycles. | D02 family boundaries. |
| CR-037 | No implementation shall infer a mandatory policy language, PDP/PEP product, gateway, service mesh, token format, cache, database, numeric TTL or deployment topology from this specification. | AD-014. |
| CR-038 | A protected non-human action shall not reach Allow unless the D01 technical principal is valid, an applicable same-Tenant Technical Principal Authority Binding is valid for the requested Permission/scope, and the complete D02-004 policy evaluation Allows. | ADR-L5-0009; AD-015. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Source / Gap | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| OD-001 | Applicable L4 authorization specifications remain `Draft` in their own metadata. | Governance disposition | Controlled Stage-2 validation rule | No | Resolved for D02-004 approval interpretation; source L4 metadata remains unchanged. |
| OD-002 | D02-001/002/003 were working authoring dependencies. | Peer stabilization | D02 family reconciliation decision | No | Closed: D02-001/002/003 are committed and Architecture Approved; reopen only on material peer change. |
| OD-003 | Authorization Policy lifecycle states, activation/rollback commands/events and governance actors remain undefined. | Upstream feature-realization gate | L4-A-052 open issues | No for reusable D02 architecture; yes only for a feature requiring the undefined lifecycle/governance mechanics | Consume an authoritative policy version only; resolve missing business governance upstream without L5 invention. |
| OD-004 | SoD policy lifecycle/events and override authorities remain undefined. | Upstream feature-realization gate | L4-A-051 open issues | No for reusable D02 architecture; yes only for a feature requiring the undefined governance authority | D02-007 preserves the gap and uses only source-approved continuation. |
| OD-005 | Exact policy evaluation language/runtime/product. | Technology | not selected | No for architecture approval; yes before concrete implementation | Evaluate/select through L5 technology ADR methodology. |
| OD-006 | Exact deployment topology of decision and enforcement components. | Technology/runtime | not selected | No for architecture approval | Resolve with E04/C01/D03 and ADR if material. |
| OD-007 | Exact authorization decision propagation/claims format across service boundaries. | C01/D02 technology | not selected | No for architecture approval; yes before distributed implementation | Define authenticated context-bound contract; avoid unsigned/self-asserted Allow. |
| OD-008 | Exact decision/effective-authority cache strategy, TTL and invalidation mechanism. | D02/B02/C02/E02 | not selected | No for architecture approval; yes before optimization | Select without weakening freshness/fail-closed semantics. |
| OD-009 | Exact transaction/TOCTOU strategy for state-sensitive authorization and mutation. | D02/B02 | implementation design | No for architecture approval; yes for high-risk mutable flows | Choose revalidation/version/transaction pattern per operation class. |
| OD-010 | Exact collection/query authorization enforcement technique. | D02/B02 | implementation design | No for architecture approval; yes before search/list implementation | Define secure scoped-query/filter enforcement. |
| OD-011 | Exact retry/failover behavior and availability targets for authorization control. | E01/E05 | operational design | No for architecture approval | Define later; no bypass fallback allowed. |
| OD-012 | Exact machine-principal authority binding model. | Resolved D01/D02 decision | `ADR-L5-0009` | No | Resolved by the approved Technical Principal Authority Binding pattern. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-D-02-004` realizes:

> Technical policy evaluation/enforcement; business/institutional policy ownership remains upstream.

### 10.2 Primary Locked L2 Traceability

- AP-01: Authorization is mandatory.
- AP-02: Authentication and Authorization are independent.
- AP-03: Server-side enforcement.
- AP-05: Policy-driven authorization.
- AP-06: Explicit Deny precedence.
- AP-10: Least privilege.
- AP-11: Complete auditability.
- AP-12: Multi-Tenant isolation.
- AP-13: Consistent authorization across all channels.
- Permission invariants: Authorization decisions evaluate Permissions together with policies; Explicit Deny overrides granted Permissions.

### 10.3 Primary L4 Traceability

**L4-A-010**
- active Role Assignment is available to Role/Permission resolution within scope/effective period;
- assignment is not final Authorization Decision.

**L4-A-051**
- `BH-005`: resolve current Roles, Delegations, Temporary Access, workflow context and applicable SoD configuration;
- `BH-006`: no conflict is not Allow;
- `BH-007`: prohibited conflict produces Deny under explicit-deny precedence;
- `BH-008`: approved continuation still requires complete authorization Allow.

**L4-A-052**
- `BH-002`: immutable predecessor/successor lineage;
- `BH-003`: deterministic ordering and explicit-deny preservation;
- `BH-004`: validate Tenant/Role/Permission/SoD/scope/config references;
- `BH-005/BH-006`: governance required; no invented activation mechanics;
- `BH-007`: use authoritative current policy version and bind decision to exact version;
- `BH-008`: preserve historical decision/version references;
- `BH-009`: rollback only through source-defined contract.

### 10.4 D02 / ADR Handoff

```text
D01 Non-Human Principal
        ↓
ADR-L5-0009 Technical Principal Authority Binding
        ↓
D02-002 Permission & Capability
        ↓
D02-003 Scope & Resource Context
        ↓
D02-004 Policy Decision & Enforcement
        ├── consumes D02-005 Delegation & Temporary Authority
        ├── consumes D02-006 Privileged & Administrative Access
        └── consumes D02-007 SoD / Maker-Checker / Emergency controls
```

Human authority continues through canonical Person / Role Assignment semantics defined upstream.

### 10.5 Architecture Decisions

`AD-001` through `AD-015`.

### 10.6 Conformance Requirements

`CR-001` through `CR-038`.

### 10.7 Open Decisions

`OD-001` through `OD-012`.

### 10.8 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-D/
            └── L5-D-02-004_Policy_Decision_and_Enforcement_Architecture.md
```

No nested `L5-D/L5-D-02/` folder is introduced.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0039`  
D01-authenticated human/non-human principal context is an input only. D02 remains the canonical authority for permissions, scope, delegation, SoD/maker-checker and emergency-access decisions; Keycloak roles/claims do not become permanent authorization truth.
