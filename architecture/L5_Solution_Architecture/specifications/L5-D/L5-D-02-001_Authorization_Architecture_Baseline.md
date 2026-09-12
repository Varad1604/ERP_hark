# Authorization Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-D-02-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-02 — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD Architecture  
**Canonical Technical Responsibility:** Authorization architecture separate from IAM, including policy decision/enforcement and least privilege.  
**Technical Architecture Owner:** L5-D-02 — Authorization Architecture  
**Assigned Author:** Devang  
**Version:** `1.0.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-12`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-01-001` through `L5-D-01-004`; `L5-D-03-001` and applicable D03 specializations; `L5-A-04-001`; `L5-A-04-002`; `L4-A-010` v1.0.0 Draft; `L4-A-011` v1.0.0 Draft; `L4-A-012` v1.0.0 Draft; `L4-A-013` v1.0.0 Draft; `L4-A-050` v1.0.0 Draft; `L4-A-051` v1.0.0 Draft; `L4-A-052` v1.0.0 Draft; `L2-A04_Authorization_and_Policy_Workflows.md` v1.1 Locked; applicable L2-A02/L2-A03 ownership decisions; `ADR-PLAT-AUTH-001`; `ADR-L5-0001`; `ADR-L5-0004`; `ADR-L5-0009`.  
**Referenced By:** `L5-D-02-002` through `L5-D-02-007`; protected business-domain APIs/services/jobs; workflow/orchestration architecture; client-platform architecture; audit/evidence architecture; verification/testing architecture.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001` for identity/authorization ownership; `ADR-L5-0001` for trust-boundary admission and non-transitive trust; `ADR-L5-0004` for Tenant/Institution isolation; `ADR-L5-0009` for non-human principal authorization binding.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification establishes the SutraERP L5 authorization baseline.

It defines the common technical authorization model that all protected business operations shall consume regardless of whether they originate from web applications, mobile applications, public or internal APIs, service-to-service calls, scheduled jobs, background workers, workflow/orchestration execution, integration services, or privileged administrative tooling.

The baseline sits after authentication and before protected execution.

```text
Authenticated / attributable technical actor
                ↓
Validated Tenant / Institution context
                ↓
Protected action + resource context
                ↓
Effective authorization facts
(Role Assignment / Permission / Policy / Delegation / Temporary Authority / SoD)
                ↓
Central authorization evaluation
                ↓
Authorization Decision
          ┌─────┴─────┐
        Allow        Deny
          ↓            ↓
Governed enforcement  Block execution
          ↓
Protected business operation
```

Authentication establishes who or what is acting.

Authorization determines whether that actor is permitted to perform the requested protected action in the applicable Tenant, Institution, resource, scope, time and policy context.

Successful authentication never implies authorization.

### 2.2 In Scope

This baseline defines:

- the D02 authorization family boundary;
- the mandatory centralized authorization control plane;
- separation of authentication and authorization;
- canonical authorization subject references;
- Role / Role Assignment consumption at L5;
- permission/capability resolution baseline;
- Tenant, Institution and lower-scope authorization context;
- protected resource/action context;
- policy-driven evaluation;
- explicit-deny precedence;
- least-privilege enforcement;
- effective-date/time validity;
- Delegation and Temporary Access participation in authorization;
- Separation of Duties participation in authorization;
- privileged/emergency access participation in authorization;
- decision/enforcement separation;
- authorization evidence requirements;
- fail-closed behavior;
- consistent behavior across channels;
- stale-context and partial-context handling;
- family-level contracts required by `L5-D-02-002` through `L5-D-02-007`.

### 2.3 Out of Scope / Non-Goals

This specification does not own or redefine:

- Person, Identity or User Account lifecycle — D01/upstream A02;
- authentication, Credential, MFA, Authentication Session or provider lifecycle — D01/upstream A03;
- Tenant/Institution isolation semantics or context propagation — A04;
- cryptography, key management, secrets or trust segmentation — D03;
- the detailed Permission/Capability technical model — `L5-D-02-002`;
- the detailed hierarchical scope/resource-context model — `L5-D-02-003`;
- detailed policy evaluation algorithms and enforcement mechanics — `L5-D-02-004`;
- detailed Delegation and Temporary Authority lifecycle realization — `L5-D-02-005`;
- detailed privileged/administrative safeguards — `L5-D-02-006`;
- detailed SoD, maker-checker or emergency override realization — `L5-D-02-007`;
- organizational position/appointment/assignment ownership in HR/workforce domains;
- business-domain approval rules;
- business-domain resource lifecycle;
- UI visibility, navigation, menu or button logic as a security boundary;
- Workspace as a source of authorization;
- physical persistence technology;
- authorization product/vendor/framework selection;
- policy-language selection;
- cache technology;
- protocol/token/claims format;
- numeric TTLs, refresh windows, evaluation timeouts or cache durations;
- invented business roles, approvers or policy authorities.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Status | Exact Authority Used by D02-001 | L5 Effect |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | `L5-D-02-001` is the Authorization Architecture Baseline; D02-002 through D02-007 own detailed permission, scope, policy, delegation, privileged-access and SoD architecture. | Establishes this baseline as family-level architecture rather than a monolithic implementation document. |
| L5 Dependency-Gated Parallel Authoring Timeline | Locked | D02 is Stage 2 and depends minimally on `D01 + D03 + A04`. | D02 authoring proceeds only after those foundations are present/stable enough to prevent invention of identity, trust or scope semantics. |
| `L5-D-01-001` through `L5-D-01-004` | Draft | D01 owns identity/account/authentication/session/credential boundaries and supplies authenticated/attributable actor context. | D02 consumes actor context; it never authenticates the actor or changes account/session state. |
| `L5-A-04-001` | v1.0.1 Draft | Tenant is mandatory root protected scope; Institution is subordinate; missing/mismatched/cross-Tenant scope fails closed. | Every D02 authorization request is evaluated inside validated A04 scope. |
| `L5-A-04-002` | Draft | Propagates Tenant/Institution/lower context across execution and persistence boundaries. | D02 consumes propagated scope and must not silently widen, replace or reconstruct it. |
| `L5-D-03-001` | v1.0.1 Draft | Explicit trust boundaries, non-transitive trust and fail-closed protected-boundary admission. | Authorization is one mandatory control consumed at protected boundaries where policy applies; D02 does not own D03 trust admission. |
| `L2-A04` | v1.1 Locked | Canonical owner of Role, Role Assignment, Permission, Authorization Policy, Authorization Decision, Authorization Context, Authorization Delegation, Temporary Access, SoD, Break-Glass and Resource Authorization. | Establishes D02 business-semantic source and mandatory separation from Identity/Authentication. |
| `L4-A-010` | v1.0.0 Draft | Active Role Assignment is available to Role/Permission Resolution only inside approved scope/effective period; assignment itself is not a final Authorization Decision. | D02 baseline separates entitlement facts from final protected-operation decision. |
| `L4-A-011` | v1.0.0 Draft | Governed Authorization Delegation is temporary and reverts at expiry/revocation; unresolved authority/contract details shall not be invented. | Delegation can contribute only when source-governed and effective. |
| `L4-A-012` | v1.0.0 Draft | Temporary Access is time-bounded, governed and expires/revokes according to source rules. | Temporary authority participates only while effective and within approved scope. |
| `L4-A-013` | v1.0.0 Draft | Emergency/Break-Glass access is exceptional, time-bound, auditable and non-permanent. | Emergency authority is an input to authorization, never a bypass around authorization. |
| `L4-A-050` | v1.0.0 Draft | Workspace is an experience context and does not grant/revoke Permissions or alter Authorization Decisions. | Workspace may influence UX/context resolution but cannot authorize execution. |
| `L4-A-051` | v1.0.0 Draft | SoD uses centralized effective responsibility facts; prohibited conflict results in Deny unless an already-approved policy permits continuation and all remaining controls Allow. | Establishes explicit-deny precedence and SoD participation in final decision. |
| `L4-A-052` | v1.0.0 Draft | Future decisions use the authoritative policy version and retain exact policy-version traceability; explicit deny cannot be weakened. | Authorization decisions bind the exact governed policy version evaluated. |
| `ADR-PLAT-AUTH-001` | Approved | Role Assignment subject is canonical A02 Person; User Account is authentication/runtime context and not the authorization assignment subject. | Prevents D02 from assigning authorization truth to User Account/session identity. |
| `ADR-L5-0001` | Architecture Approved | Explicit trust boundaries and fail-closed admission. | Authorization enforcement is a protected-boundary control, not optional application decoration. |
| `ADR-L5-0004` | Architecture Approved | Tenant-rooted isolation and subordinate Institution context. | D02 cannot authorize across Tenant boundaries or widen A04 scope. |
| `ADR-L5-0009` | Architecture Approved | Non-human service/workload/job/integration principals receive D02 authority through a Technical Principal Authority Binding rather than fabricated Person/Role Assignment semantics. | Makes non-human authorization implementable while preserving D01 identity ownership and D02 policy/scope enforcement. |

For this controlled Stage-2 validation program, the cited canonical L4 System Behavior Specifications are accepted as behavioral authority even where their repository metadata remains `Draft` or `Review`. Their metadata status is therefore not an approval blocker for this specification; the L4 files themselves are not silently promoted by this decision.

### 3.2 Core Authorization Invariants

| Constraint ID | Invariant | Basis |
|---|---|---|
| C-001 | Authentication and Authorization are independent; successful authentication never implies permission. | L2-A04 AP-02; D01/D03 boundaries. |
| C-002 | Every protected business operation shall receive a governed Authorization Decision before execution. | L2-A04 AP-01. |
| C-003 | Client-side visibility/navigation cannot be treated as authorization enforcement. | L2-A04 AP-03/AP-09. |
| C-004 | Authorization protects business actions/resources, not pages/widgets/routes as the canonical permission model. | L2-A04 AP-04. |
| C-005 | Authorization shall be policy-driven rather than independently hard-coded by each business domain. | L2-A04 AP-05/AP-13. |
| C-006 | Explicit Deny has precedence over Allow where governed rules conflict. | L2-A04 AP-06; L4-A-051; L4-A-052. |
| C-007 | Workspace never grants/removes authorization and workspace switching never independently changes the final authorization result. | L2-A04 AP-07/AP-09; L4-A-050. |
| C-008 | Least privilege is mandatory; effective authority shall be no broader than governed assignment/policy/scope permits. | L2-A04 AP-10; L4-A-010. |
| C-009 | Authorization execution is always bound to one valid Tenant root context. | L2-A04 AP-12; A04-001. |
| C-010 | Role Assignment subject is the canonical Person reference, not User Account or Authentication Session. | ADR-PLAT-AUTH-001; L4-A-010. |
| C-011 | Only source-valid `Active` Role Assignments within approved scope/effective period participate in later role/permission resolution. | L4-A-010-BH-007 through BH-010. |
| C-012 | Delegation, Temporary Access and emergency authority are additive governed inputs, not permanent mutation of underlying Role Assignment truth. | L2-A04; L4-A-011/A012/A013. |
| C-013 | Absence of a SoD conflict is not an independent Allow. | L4-A-051-BH-006. |
| C-014 | An SoD exception or emergency path never bypasses the remaining authorization evaluation. | L4-A-051-BH-008; L4-A-013 boundary. |
| C-015 | Authorization Decisions must remain traceable to the exact policy version and effective authority facts evaluated. | L4-A-052-BH-007/BH-008. |
| C-016 | Missing, inconsistent or untrusted mandatory authorization context shall not degrade to a permissive result. | D03-001 fail-closed principle; A04; L2-A04. |
| C-017 | Business domains supply protected-operation/resource/business context and retain business ownership; D02 determines authorization outcome without absorbing business lifecycle ownership. | L2-A04 §§5, 12–13. |
| C-018 | Authorization semantics shall be consistent across UI, API, internal service, job, workflow and integration channels. | L2-A04 AP-13. |
| C-019 | Non-human authority shall be represented through a D02 Technical Principal Authority Binding rooted in the same Tenant as the D01 non-human principal; the binding is not a human Role Assignment and never independently produces final Allow. | ADR-L5-0009. |

### 3.3 Assumptions

No business authority is inferred merely to complete technical architecture.

Where current L4 leaves policy lifecycle, Delegation approval authority, Temporary Access approver, SoD override authority or emergency approval semantics unresolved, D02 carries those as explicit governed inputs or feature-realization gates rather than inventing actors/states/events.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 D02-001 Owns

This baseline owns the family-wide technical authorization invariants and the common authorization-control model:

- centralized authorization request/decision boundary;
- mandatory pre-execution authorization;
- final Allow/Deny decision contract at family level;
- separation of authenticated actor context from authorization truth;
- common effective-authority resolution sequence;
- decision/enforcement separation;
- explicit-deny precedence;
- least privilege;
- fail-closed authorization behavior;
- consistent multi-channel authorization behavior;
- family-level evidence/correlation requirements;
- technical boundaries among D02-002 through D02-007.

### 4.2 D02 Family Specialization

| Specification | Canonical Detailed Responsibility | D02-001 Boundary |
|---|---|---|
| `L5-D-02-002` | Permission & Capability Model Architecture | Defines technical Permission/capability representation and resolution model; baseline only establishes that permissions describe governed business actions/resources and are not UI artifacts. |
| `L5-D-02-003` | Hierarchical Scope & Resource Context Architecture | Defines Tenant/Institution/organizational-unit/resource authorization scope; baseline only requires scope-aware evaluation and A04 compatibility. |
| `L5-D-02-004` | Policy Decision & Enforcement Architecture | Defines detailed policy evaluation, decision and enforcement mechanics; baseline only establishes mandatory centralized decision and fail-closed enforcement. |
| `L5-D-02-005` | Delegation & Temporary Authority Architecture | Defines detailed technical realization of approved delegated/time-bounded authority; baseline only establishes participation rules/invariants. |
| `L5-D-02-006` | Privileged & Administrative Access Architecture | Defines high-risk privileged/admin safeguards; baseline requires privileged access to remain governed authorization, never implicit trust. |
| `L5-D-02-007` | Separation-of-Duties, Maker-Checker & Emergency Override Architecture | Defines detailed SoD/maker-checker/emergency technical controls; baseline establishes deny precedence and no-bypass invariant. |

### 4.3 Collaborating L5 Architecture

| L5 Area | Supplies / Owns | D02 Consumption |
|---|---|---|
| D01 | Person/account technical references, human/non-human principal identity, authenticated context/session | D02 resolves subject/actor but never mutates D01 state. |
| A04 | Tenant/Institution isolation and context propagation | D02 binds every decision to valid A04 scope. |
| D03 | trust-boundary/security admission, cryptography/secrets/segmentation | D02 decision is one security prerequisite consumed by D03-protected execution. |
| D04 | audit/evidence architecture | D02 emits decision/evaluation facts required for evidence. |
| C01 | request/response/API contract architecture | Carries authorization request/decision/enforcement contracts. |
| C02 | event/messaging architecture | Carries invalidation/change notifications where later governed. |
| B02 | persistence/consistency architecture | Stores authorization aggregates/technical projections according to canonical ownership. |
| A05 | workflow/orchestration | Consumes D02 for protected workflow actions and maker-checker/assignment policy references; A05 does not own authorization truth. |
| A06 | governed configuration/extensibility | Supplies permitted institutional configuration without widening D02 policy authority. |
| E05 | observability/operations | Receives operational health/latency/failure signals without replacing D04 authorization evidence. |
| G01 | client platform | May consume visibility hints but cannot replace server-side D02 enforcement. |

### 4.4 Must Not Own

D02-001 and the D02 family shall not own:

- authentication;
- Credential verification;
- User Account lifecycle;
- Person lifecycle;
- business-domain state transitions;
- organizational appointment/position authority;
- Tenant/Institution lifecycle;
- UI navigation security;
- raw security secret material;
- business workflow approval rules;
- policy approval authority where upstream is silent;
- deployment trust;
- persistence implementation;
- transport technology.

---

## 5. Solution Architecture

### 5.1 Family-Level Authorization Control Plane

```text
                D01
   authenticated / attributable actor
                 │
                 ▼
               A04
     validated Tenant / Institution
                 │
                 ▼
Protected request ───────► Authorization Request Boundary
(action + resource)                  │
                                    ▼
                         Subject / Actor Resolver
                                    │
                                    ▼
                         Effective Authority Resolver
                    ┌───────────────┼─────────────────┐
                    │               │                 │
              Role Assignment   Delegation       Temporary /
                    │               │            Privileged
                    └───────┬───────┴─────────────┬───┘
                            ▼                     ▼
                      Permission /           SoD / Emergency /
                      Capability             Policy constraints
                            \                     /
                             \                   /
                              ▼                 ▼
                          Policy Evaluation Boundary
                                   │
                                   ▼
                         Authorization Decision
                           Allow / Deny
                                   │
                    ┌──────────────┴──────────────┐
                    ▼                             ▼
             Enforcement Point                D04 Evidence
                    │
             Allow? │
               ┌────┴────┐
               ▼         ▼
            Execute     Reject
            business    protected
            operation   operation
```

No element above owns the business operation itself.

### 5.2 Logical Components

| Component / Boundary | Responsibility | Key Inputs | Key Output | Failure Rule |
|---|---|---|---|---|
| Authorization Request Boundary | Normalizes a protected-operation authorization request without losing actor, scope, action or resource context. | authenticated actor/principal; Tenant/Institution scope; action; resource; business context; correlation | canonical technical Authorization Evaluation Context | Missing required context → no Allow. |
| Subject & Actor Resolver | Resolves the canonical authorization subject and acting technical principal while preserving distinction between Person subject, authenticated user, delegated actor and workload identity. | D01 actor; canonical Person; subject references | attributable subject/actor context | User Account/session shall not replace canonical Role Assignment subject. |
| Scope Admission Boundary | Confirms the requested authorization scope is within validated A04 Tenant/Institution context. | A04 scope; requested resource scope | admitted scope or rejection | Cross-Tenant/mismatched/widened scope → Deny/reject. |
| Effective Authority Resolver | Resolves source-valid Role Assignments, Technical Principal Authority Bindings, and governed delegated/temporary authority that may contribute to evaluation. | active Role Assignments; valid technical-principal bindings; period/scope; delegation/temp access references | effective authority facts | Inactive/expired/out-of-scope authority does not participate. |
| Permission / Capability Resolution Boundary | Resolves the business action/resource capability represented by effective authority. | Roles/Permissions/capabilities; requested action/resource | candidate permitted capabilities | Detailed model belongs D02-002. |
| Resource Context Resolver | Supplies governed resource state/classification/ownership/scope attributes needed by policy without transferring resource ownership to D02. | business-domain resource reference/state context | immutable evaluation snapshot/reference | Missing mandatory resource context → no permissive inference. |
| Policy Version Resolver | Resolves the authoritative policy version applicable to Tenant/Institution/action/resource/time. | policy scope/version; configuration | exact governed policy reference | Undefined/unapproved successor must not silently replace current authority. |
| Authorization Policy Evaluator | Evaluates applicable rules with deterministic deny precedence. | subject; authority; permission; scope; resource; policy; SoD; temporal context | final policy result | Conflict/indeterminate mandatory control → no Allow. Detailed mechanics D02-004. |
| Authorization Decision Authority | Produces final `Allow` or `Deny` for the protected operation and binds it to evaluated context/version. | complete policy evaluation | Authorization Decision | No decision reuse outside governed context. |
| Enforcement Boundary | Ensures protected execution occurs only after valid Allow and only for exactly authorized operation/resource/scope. | Authorization Decision; protected invocation | execute or reject | Client-side/UI checks never satisfy this boundary. |
| Decision Evidence Handoff | Supplies non-secret evaluation/decision facts to D04. | decision + provenance | audit/evidence input | Evidence requirements must not be silently omitted. |
| Decision Invalidation / Freshness Boundary | Prevents materially stale authority/policy/scope from being reused after revocation, expiry, policy change or scope change. | policy/authority version references; source change state | valid/re-evaluate outcome | Stale permissive authority must not survive authoritative revocation. |

### 5.3 Authorization Evaluation Context

The conceptual context is:

```text
AuthorizationEvaluationContext
{
  Tenant reference
  Institution / lower-scope reference where applicable

  authenticated / attributable actor reference
  canonical authorization subject reference
  actor-vs-subject relationship if distinct

  protected action / capability reference
  protected resource reference
  authoritative resource context required by policy

  effective Role Assignment references
  effective Technical Principal Authority Binding reference where applicable
  effective Delegation references if any
  effective Temporary / privileged authority references if any

  applicable SoD / maker-checker / emergency context
  exact Authorization Policy version reference

  effective-time context
  trust-boundary / request provenance reference
  correlation reference
}
```

This is a semantic contract, not an API payload or claims format.

### 5.4 Canonical Subject and Actor Model

For human authorization:

- Role Assignment is bound to the canonical A02 Person subject.
- User Account and Authentication Session are technical authentication/runtime context.
- A valid session may identify the acting user but is not the canonical owner of authorization assignment.
- A delegated actor may act using approved delegated authority while the original source authority remains traceable.
- A privileged/emergency actor remains attributable; emergency elevation does not replace identity.
- The business-domain target/resource remains owned by its business domain.

For non-human execution:

- D01 supplies the attributable Tenant-bound service/workload/job/integration principal.
- D02 binds machine authority through the `ADR-L5-0009` **Technical Principal Authority Binding** rather than fabricating a Person, human User Account or human Role Assignment.
- The binding references the D01 technical principal, governed Permission/capability authority, Tenant and applicable narrower scope, provenance/governance reference, and validity/freshness information.
- A Technical Principal Authority Binding contributes candidate effective authority only; D02-003 scope/resource evaluation and D02-004 policy/deny enforcement still determine the final Authorization Decision.
- A machine identity shall never acquire human Person authority merely to reuse the human RBAC path.

### 5.5 Role Assignment Participation

D02-001 establishes these baseline rules from L4-A-010:

1. Role Assignment is not the final Authorization Decision.
2. Only source-valid effective Role Assignment facts participate.
3. Assignment scope and effective period constrain authority.
4. Permissions are not directly assigned to a Person by this baseline where upstream Role Assignment semantics require Role-mediated authority.
5. Role/Permission definition ownership remains A04 authorization-domain truth.
6. Source business eligibility remains source-domain truth.
7. Organizational assignment/position ownership does not transfer into authorization.
8. Least privilege and SoD constraints apply before authority becomes effective.

Detailed Permission representation is D02-002.

Detailed scope semantics are D02-003.

### 5.6 Permission / Capability Baseline

A Permission/Capability represents authorization for a governed business action on a governed business resource or capability.

Baseline constraints:

- permissions describe business actions/capabilities, not UI pages;
- naming/identifier conventions remain D02-002;
- a Permission alone does not establish a Person's effective authority;
- permission resolution considers effective Role Assignment, Technical Principal Authority Binding where applicable, and any governed temporary/delegated authority;
- resource/scope constraints remain part of evaluation;
- policy can further restrict an otherwise available permission;
- Deny can override otherwise permitting authority;
- UI presentation may consume authorization-derived hints but cannot become source authority.

### 5.7 Scope and Resource Context Baseline

Authorization scope shall never widen A04 context.

```text
Tenant
  ↓
Institution
  ↓
governed organizational / functional scope
  ↓
resource / record / operation context
```

The exact hierarchy is D02-003-owned.

D02-001 requires that:

- the authorization request carries sufficient scope to identify the protected operation boundary;
- scope is validated against A04 Tenant/Institution context;
- resource context comes from the business/resource owner;
- authorization evaluation does not mutate resource state merely to decide access;
- a Role Assignment or Technical Principal Authority Binding valid at one scope does not automatically grant authority at another scope;
- a missing lower scope cannot be replaced by a broader scope merely to obtain an Allow;
- Tenant mismatch is always non-permissive.

### 5.8 Policy Evaluation Baseline

The policy decision sequence is conceptually:

```text
1. validate actor/subject
2. validate Tenant/Institution scope
3. resolve protected action/resource
4. resolve effective authority
5. resolve Permission/Capability
6. resolve applicable policy version
7. resolve SoD/delegation/temp/privileged constraints
8. evaluate deterministic policy rules
9. apply explicit-deny precedence
10. produce final Authorization Decision
11. enforce exactly that decision/context
12. preserve evidence
```

D02-004 owns detailed evaluator/enforcement architecture.

### 5.9 Authorization Decision

The baseline final decision contract is:

```text
AuthorizationDecision
{
  outcome: Allow | Deny
  Tenant / scope reference
  subject / actor reference
  protected action reference
  protected resource reference
  evaluated policy version reference
  relevant effective-authority references
  material deny / decision basis reference
  decision time / effective-time context
  correlation / evidence reference
}
```

The contract shall not expose sensitive policy internals unnecessarily, but it shall retain sufficient provenance for auditability and deterministic reconstruction under D04 governance.

No additional canonical business outcome state is invented here.

### 5.10 Decision and Enforcement Separation

Decision and enforcement are distinct responsibilities.

```text
Decision Point
   determines whether operation is permitted
            ↓
Enforcement Point
   prevents execution unless the decision permits it
```

This distinction allows consistent policy use across API, service, workflow and background execution boundaries.

However:

- an Enforcement Point shall not reinterpret a Deny into Allow;
- an Allow is valid only for its bound context;
- changing action, resource, Tenant, Institution, subject or material policy context requires new/revalidated authorization according to D02-004;
- the client cannot self-assert an Allow;
- a cached/propagated decision cannot outlive its governed validity/freshness constraints.

### 5.11 Delegation and Temporary Authority Participation

D02-001 establishes only family invariants:

- delegated/temporary authority must originate from source-governed authorization-domain facts;
- authority is limited by approved scope, action/capability and effective period;
- authority expiry/revocation removes that contribution;
- delegation shall not permanently mutate source Role Assignment;
- temporary access shall not become a hidden permanent permission assignment;
- delegated authority does not widen Tenant scope;
- the original source, delegate/recipient and actual actor remain attributable;
- detailed realization is D02-005.

### 5.12 Privileged and Administrative Access Participation

Privileged/admin authority is not implicit in technical placement, role title, internal network position or account type.

D02 baseline requires:

- explicit governed privileged authority;
- least-privilege scope;
- attributable actor;
- protected trust-boundary admission through D03;
- stronger evidence where upstream/control policy requires;
- no unrestricted “superuser” semantic invented by L5;
- no bypass of normal authorization merely because an operation is administrative.

Detailed safeguards belong D02-006.

### 5.13 SoD / Maker-Checker / Emergency Participation

D02-001 establishes:

- applicable SoD conflict evaluation participates in authorization;
- prohibited conflict results in Deny unless an already-approved policy defines a permitted continuation path;
- absence of conflict does not independently produce Allow;
- maker-checker/business approval semantics remain upstream-owned;
- emergency/break-glass authority is exceptional and time/scope bounded;
- emergency authority does not suppress final centralized authorization evaluation;
- no approver/override authority is invented where upstream is silent;
- detailed technical controls belong D02-007.

### 5.14 Workspace Separation

Workspace is explicitly non-authoritative for access.

Workspace may:

- select an experience context;
- influence navigation/dashboard/menu presentation;
- help users choose among responsibilities they already possess.

Workspace shall not:

- grant a Permission;
- activate a Role Assignment;
- bypass policy;
- change Deny to Allow;
- broaden scope;
- replace server-side authorization.

### 5.15 Multi-Channel Consistency

The same family-level authorization semantics apply to:

```text
Browser / Mobile
        │
        ├──── API
        ├──── Internal Service
        ├──── Workflow
        ├──── Background Job
        └──── Integration
                ↓
        D02 Authorization
                ↓
        protected execution
```

Each channel may use different transport/runtime realization, but it shall not implement a weaker authorization model.

### 5.16 Decision Freshness and Change

The architecture shall prevent stale permissive authority from surviving material authoritative change.

Material changes include, where applicable:

- Role Assignment activation/suspension/expiry/revocation;
- Technical Principal Authority Binding validity/expiry/revocation or scope change;
- Delegation expiry/revocation;
- Temporary Access expiry/revocation;
- policy-version change;
- SoD rule/configuration change;
- Tenant/Institution scope change;
- resource-state change relevant to policy;
- privileged/emergency authority closure.

D02-001 does not select cache invalidation technology, event delivery model, polling interval or TTL.

D02-004/C02/B02 will realize freshness mechanisms under governed design.

### 5.17 Failure Model

| Failure / Ambiguity | Required Outcome |
|---|---|
| No authenticated/attributable actor where required | No Allow. |
| Canonical subject cannot be resolved | No Allow. |
| Non-human principal lacks a valid Technical Principal Authority Binding for the requested capability/scope | No Allow. |
| Tenant scope missing or inconsistent | Fail closed. |
| Institution/resource scope conflicts with Tenant | Fail closed. |
| Protected action/resource cannot be resolved | Reject evaluation / Deny; never guess a broader permission. |
| Role Assignment is inactive, expired, premature or out of scope | Exclude it from effective authority. |
| Technical Principal Authority Binding is expired, revoked, stale or out of scope | Exclude it from effective authority. |
| Delegation/temporary authority is expired/revoked/out of scope | Exclude it. |
| Applicable policy version cannot be reliably resolved | No permissive default. |
| Policy evaluation is ambiguous/non-deterministic | No Allow. |
| Explicit Deny is present | Deny takes precedence. |
| SoD conflict is prohibited | Deny unless source-approved continuation applies and full evaluation then Allows. |
| Authorization service/control unavailable | Protected operation shall not bypass authorization. |
| Client claims an Allow without authoritative evaluation | Ignore/reject. |
| Decision is stale relative to material authority/policy/resource change | Re-evaluate or reject; do not reuse stale Allow. |
| Evidence handoff fails where evidence is mandatory for the protected operation/control | Do not silently claim complete controlled execution. |

### 5.18 Persistence / Consistency Boundaries

- Canonical Role, Role Assignment, Permission, Delegation, Temporary Access, SoD Policy and Authorization Policy state remain authorization-domain-owned.
- D02-owned Technical Principal Authority Bindings are authorization technical state and shall not become D01 identity or credential truth.
- D02-001 defines no database schema.
- Derived effective-authority/index/cache/projection structures may exist only as technical derivatives and may not become canonical authority.
- Historical Authorization Decisions shall retain the policy-version reference evaluated.
- Derived projections shall be invalidated/reconciled when authoritative entitlement/policy facts change.
- Cross-domain resource attributes remain source-owned and shall not be copied into D02 as independent business truth merely for authorization convenience.
- B02 owns physical transaction/isolation/locking/replication decisions.

### 5.19 Security / Trust Boundary

D02 operates inside the D03 security model.

```text
Identity established by D01
        ↓
Scope validated by A04
        ↓
Trust-boundary prerequisites validated by D03
        ↓
Authorization decided by D02
        ↓
Protected boundary admitted only when all mandatory controls succeed
```

An Authorization `Allow` does not remove other security prerequisites.

A D03 trust admission does not itself produce authorization.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Rationale / Basis | ADR |
|---|---|---|---|
| AD-001 | Authorization is a mandatory centralized control for every protected business operation. | L2-A04 AP-01/AP-13; prevents domain/channel-specific bypass models. | None — direct locked architecture derivation. |
| AD-002 | Authentication and authorization remain technically and semantically separate. | L2-A04 AP-02; D01/D02 ownership. | `ADR-PLAT-AUTH-001`. |
| AD-003 | Canonical Role Assignment subject is Person; User Account/session are runtime authentication context, not assignment truth. | L4-A-010; ADR-PLAT-AUTH-001. | `ADR-PLAT-AUTH-001`. |
| AD-004 | D02 authorization is Tenant-rooted and cannot widen A04 scope. | A04-001 and `ADR-L5-0004`. | `ADR-L5-0004`. |
| AD-005 | Authorization is policy-driven and explicit Deny has precedence over Allow. | L2-A04 AP-05/AP-06; L4-A-051/A052. | None; direct upstream rule. |
| AD-006 | Role Assignment, Permission, Technical Principal Authority Binding, Delegation, Temporary Access, SoD and policy are effective-authority inputs; none alone is automatically the final Authorization Decision. | L4-A-010/A051; L2-A04; ADR-L5-0009. | `ADR-L5-0009` for non-human binding. |
| AD-007 | Decision and enforcement are distinct but inseparable controls: protected execution requires a valid final Allow. | L2-A04 AP-01/AP-03. | None. |
| AD-008 | Workspace and client-side visibility are not security boundaries and never authorize execution. | L2-A04 AP-03/AP-07/AP-09; L4-A-050. | None. |
| AD-009 | Authorization decisions bind exact evaluated scope, action/resource and policy-version provenance. | L4-A-052-BH-007/BH-008. | None. |
| AD-010 | Missing/invalid mandatory authorization context fails closed; no permissive fallback is allowed. | D03 fail-closed baseline; A04 isolation; L2-A04. | `ADR-L5-0001` + `ADR-L5-0004` constraints. |
| AD-011 | D02 family uses one common authorization semantics across interactive, API, service, workflow, job and integration channels. | L2-A04 AP-13. | None. |
| AD-012 | Detailed D02 responsibilities remain split across D02-002 through D02-007 rather than being duplicated in the baseline. | Frozen L5 corpus. | None. |
| AD-013 | No authorization vendor, policy language, RBAC framework, ABAC engine, PDP/PEP product, cache, database or protocol is selected by this baseline. | L5 Technology Selection & ADR Methodology. | Future ADR only if material selection is required. |
| AD-014 | Non-human service/workload/job/integration authority uses a D02-owned Technical Principal Authority Binding instead of fabricated human Person/Role Assignment semantics. | D01 non-human identity model plus D02 authorization ownership. | `ADR-L5-0009`. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owning / Collaborating Spec |
|---|---|---|
| Unauthorized access | Protected operations execute only after valid Allow. | D02-001 / D02-004 |
| Tenant isolation | Every decision remains inside A04 Tenant root and compatible subordinate scope. | A04 + D02-003 |
| Identity integrity | Role Assignment subject remains canonical Person; actor remains attributable through D01. | D01 + D02 |
| Machine authorization integrity | Non-human principals receive bounded D02 authority without fabricated Person/User Account semantics. | D01-002 + D02 + ADR-L5-0009 |
| Least privilege | Effective authority limited by Role Assignment or Technical Principal Authority Binding, Permission, scope, period and policy. | D02-002/003/004 |
| Policy consistency | Central policy evaluation prevents business modules from diverging. | D02-004 |
| SoD | Effective responsibilities participate in conflict evaluation; conflict Deny precedence preserved. | D02-007 |
| Privileged access | Administrative/privileged execution requires explicit governed authority and trust controls. | D02-006 + D03 |
| Emergency access | Exceptional authority remains bounded, attributable and non-bypass. | D02-007 |
| Delegation | Actual actor, subject and source authority remain distinguishable. | D02-005 |
| Historical reproducibility | Authorization Decision retains policy-version provenance. | D02-004 + D04 |
| Client security | UI visibility is never treated as enforcement. | G01 + D02-004 |
| Availability failure | Authorization-control unavailability does not become permissive bypass. | D02-004 + E01 |
| Performance | Caching/projection may optimize evaluation only if freshness and canonical-authority invariants are preserved. | D02-004 + B02 + E02 |
| Auditability | Material decision/effective-authority/policy/scope facts are handed to D04. | D04 |
| Privacy | Decision/evidence contracts expose only information required for control/evidence; resource personal data remains source-owned. | B05 + D04 |

No numeric latency, throughput, availability, cache TTL, policy evaluation timeout or evidence retention target is defined.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Conformance Requirement | Basis |
|---|---|---|
| CR-001 | Every protected business operation shall be authorized before execution. | L2-A04 AP-01. |
| CR-002 | Successful authentication shall not by itself authorize any protected operation. | AP-02; AD-002. |
| CR-003 | Client-side menus, buttons, routes, URLs, dashboards or Workspace state shall not be accepted as security enforcement. | AP-03/AP-07/AP-09; L4-A-050. |
| CR-004 | Authorization shall evaluate one valid Tenant-rooted context and shall not authorize cross-Tenant or scope-widened execution. | A04; AD-004. |
| CR-005 | Role Assignment subject shall resolve to the canonical Person reference according to upstream ownership; User Account/session shall not replace the assignment subject. | L4-A-010; ADR-PLAT-AUTH-001. |
| CR-006 | Only effective source-valid Role Assignments inside approved scope/effective period shall participate in Role/Permission Resolution. | L4-A-010-BH-007 through BH-010. |
| CR-007 | Active Role Assignment shall not itself be treated as the final Authorization Decision. | L4-A-010-BH-010. |
| CR-008 | Permission/capability semantics shall represent governed business action/resource capability rather than UI presentation. | L2-A04 AP-04. |
| CR-009 | Authorization shall be policy-driven and all channels shall use the same family-level decision semantics. | AP-05/AP-13. |
| CR-010 | Explicit Deny shall take precedence over conflicting Allow. | AP-06; L4-A-051/A052. |
| CR-011 | Missing, invalid or ambiguous mandatory subject, scope, policy, action or resource context shall not produce a permissive fallback. | AD-010. |
| CR-012 | A SoD no-conflict result shall not independently create Allow. | L4-A-051-BH-006. |
| CR-013 | A prohibited SoD conflict shall result in Deny unless an already-approved policy explicitly permits continuation and the complete authorization evaluation then Allows. | L4-A-051-BH-007/BH-008. |
| CR-014 | Delegation and Temporary Access shall participate only while source-valid, in scope and within effective time. | L4-A-011/A012. |
| CR-015 | Delegated or Temporary Authority shall not silently become permanent Role/Permission assignment. | L2-A04; D02-005 boundary. |
| CR-016 | Privileged/admin authority shall not be inferred from internal placement, technical account type or infrastructure role alone. | D03 trust model; D02-006 boundary. |
| CR-017 | Emergency/break-glass authority shall remain exceptional, attributable and bounded and shall not bypass the remaining centralized authorization evaluation. | L4-A-013; D02-007 boundary. |
| CR-018 | Authorization Decision shall bind the exact policy version evaluated. | L4-A-052-BH-007/BH-008. |
| CR-019 | Historical Authorization Decisions shall retain their original policy-version references after later policy changes. | L4-A-052-BH-008. |
| CR-020 | A policy successor shall not become effective merely because a technical deployment/configuration exists when upstream governance has not established it as authoritative. | L4-A-052-BH-005/BH-006. |
| CR-021 | Protected execution shall enforce exactly the authorized action/resource/scope and shall not widen the decision at the enforcement boundary. | AD-007/009. |
| CR-022 | Stale permissive authority shall not survive authoritative expiry, revocation, policy replacement or material resource/scope change. | §5.16. |
| CR-023 | Authorization-derived caches/projections shall remain derivative and shall not become canonical Role/Permission/Policy authority. | §5.18. |
| CR-024 | D02 shall not directly modify business-domain resource state as part of deciding authorization. | Ownership boundary. |
| CR-025 | D02 shall not create Person, User Account, Authentication Session or Credential state. | D01 ownership. |
| CR-026 | D02 shall preserve D03 trust-boundary prerequisites; an authorization Allow shall not bypass cryptographic/credential/segmentation/security controls. | D03-001. |
| CR-027 | Material authorization outcomes shall expose sufficient non-secret evidence context to D04 for attributable reconstruction. | AP-11; L4-A-010/A051/A052. |
| CR-028 | Business domains shall not implement an independent weaker authorization model for alternate channels. | AP-13. |
| CR-029 | No detailed child-spec responsibility shall be duplicated in D02-001 in a way that creates conflicting technical ownership with D02-002 through D02-007. | Frozen corpus / AD-012. |
| CR-030 | No implementation shall infer a mandatory authorization product, policy language, token/claims format, cache technology, database or numeric target from this baseline. | AD-013. |
| CR-031 | Every non-human protected operation shall resolve a valid D01 non-human principal and a matching Tenant-bound D02 Technical Principal Authority Binding before its authority can participate in final policy evaluation. | ADR-L5-0009; AD-014. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Source / Gap | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| OD-001 | L4 authorization specifications remain `Draft` in their own metadata. | Governance disposition | Controlled Stage-2 validation rule and `L5_Devang_Stage2_Architecture_Remediation_and_Approval_Decision_2026-09-12.md` | No | Resolved for this specification's approval interpretation; source L4 metadata remains unchanged. |
| OD-002 | Detailed Permission/Capability representation and non-human capability binding. | Resolved D02 child design | `L5-D-02-002`; `ADR-L5-0009` | No | Resolved by the approved D02-002 model and Technical Principal Authority Binding pattern. |
| OD-003 | Exact lower-scope hierarchy/resource-context technical model and inheritance semantics. | Resolved D02 child design | `L5-D-02-003` | No | Resolved at reusable architecture level by `L5-D-02-003`; source-owned lower-scope semantics remain authoritative. |
| OD-004 | Detailed policy evaluator/enforcement architecture and machine-protected-operation participation. | Resolved D02 child design / technology deferral | `L5-D-02-004`; `ADR-L5-0009` | No for architecture approval | Resolved at architecture level; product/topology/cache selections remain separately governed implementation decisions. |
| OD-005 | Authorization Policy lifecycle states, activation/rollback commands/events and policy governance actors remain undefined upstream. | Upstream feature-realization gate | L4-A-052 open issues | No for reusable D02 architecture; Yes only for a feature that requires the undefined lifecycle/governance semantics | Keep evaluator bound to an authoritative policy reference and resolve missing business authority upstream before realizing that feature. |
| OD-006 | Delegation canonical approving authority, some lifecycle contracts/events and exact creation/expiry/revocation handoffs remain incompletely governed upstream. | Upstream feature-realization gate | L4-A-011 / parent gaps | No for reusable D02 architecture; Yes only for the affected Delegation approval/lifecycle path | Preserve D02-005 mechanism; resolve business approver/command/event authority upstream without L5 invention. |
| OD-007 | Temporary Access approving role/authority and exact lifecycle contracts/events remain incompletely governed upstream. | Upstream feature-realization gate | L4-A-012 / parent gaps | No for reusable D02 architecture; Yes only for the affected Temporary Access approval/lifecycle path | Preserve D02-005 mechanism; resolve business authority upstream. |
| OD-008 | SoD configuration lifecycle/events and platform/institution override authorities remain undefined. | Upstream feature-realization gate | L4-A-051 `A051-L4-OPEN-*` | No for reusable D02 architecture; Yes only where the missing policy-governance authority is required | Preserve Deny/no-bypass semantics and resolve missing governance upstream before that feature is realized. |
| OD-009 | Detailed business authority for selected privileged/admin and emergency approval/override paths remains source-governed. | Upstream feature-realization gate | D02-006/D02-007; L4-A-013 | No for reusable D02 architecture; Yes only for a flow requiring an undefined business authority | Use approved D02-006/D02-007 technical safeguards and obtain source authority before enabling the affected flow. |
| OD-010 | Exact audit evidence storage/schema/retention for Authorization Decisions. | Cross-L5 dependency | D04 responsibility | No | D02 supplies semantic evidence facts; D04 owns storage/provenance details. |
| OD-011 | Exact authorization decision transport, claims representation, policy language, engine/product, deployment and persistence technology. | Technology/ADR | No product selection is made here | No for architecture approval; governed before concrete implementation where material | Use L5 Technology Selection & ADR Methodology and ADR governance for material selections. |

---

## 10. Traceability Summary

### 10.1 Primary Upstream Behavioral Traceability

- `L4-A-010-BH-001` through `BH-011`: Role Assignment request, validation, approval, activation, scope/effective period and separation from final Authorization Decision.
- `L4-A-011`: governed Authorization Delegation and automatic authority reversion.
- `L4-A-012`: temporary access and governed expiry/revocation.
- `L4-A-013`: emergency/break-glass authorization closure and auditability.
- `L4-A-050`: Workspace is a resolvable experience context and does not grant/alter authorization.
- `L4-A-051-BH-005` through `BH-008`: centralized SoD evaluation, explicit-deny precedence, absence-of-conflict not equal to Allow, exception not equal to bypass.
- `L4-A-052-BH-002` through `BH-009`: immutable policy version lineage, deterministic/deny-preserving evaluation, exact decision-to-policy-version traceability.

### 10.2 Locked L2 / ADR Constraints

- L2-A04 AP-01: authorization mandatory.
- AP-02: authentication and authorization independent.
- AP-03: server-side enforcement.
- AP-04: permissions protect business resources/actions.
- AP-05: policy-driven authorization.
- AP-06: Explicit Deny precedence.
- AP-07/AP-09: Workspace does not grant authorization.
- AP-10: least privilege.
- AP-11: complete auditability.
- AP-12: Tenant isolation.
- AP-13: consistent authorization across channels.
- `ADR-PLAT-AUTH-001`: Person is the authorization assignment subject; User Account is runtime/authentication context.
- `ADR-L5-0001`: protected boundaries are explicit and fail closed.
- `ADR-L5-0004`: Tenant is mandatory root protected scope.
- `ADR-L5-0009`: non-human authority uses D02 Technical Principal Authority Binding without fabricated Person/Role Assignment semantics.

### 10.3 D02 Family Decomposition

```text
L5-D-02-001  Authorization Architecture Baseline
        ↓
L5-D-02-002  Permission & Capability Model
        ↓
L5-D-02-003  Hierarchical Scope & Resource Context
        ↓
L5-D-02-004  Policy Decision & Enforcement
        ↓
L5-D-02-005  Delegation & Temporary Authority
        ↓
L5-D-02-006  Privileged & Administrative Access
        ↓
L5-D-02-007  SoD / Maker-Checker / Emergency Override
```

The arrows above indicate authoring specialization/order for the D02 family, not mandatory runtime call sequencing.

### 10.4 Architecture Decisions

`AD-001` through `AD-014`.

### 10.5 Conformance Requirements

`CR-001` through `CR-031`.

### 10.6 Open / Deferred Items

`OD-001` through `OD-011`.

### 10.7 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-D/
            └── L5-D-02-001_Authorization_Architecture_Baseline.md
```

No `L5-D/L5-D-02/` nested major-area folder is introduced by this specification.
