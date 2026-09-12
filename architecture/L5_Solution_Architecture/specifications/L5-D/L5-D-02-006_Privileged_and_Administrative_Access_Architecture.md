# Privileged & Administrative Access Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-02-006`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-02 — Authorization / RBAC / Permission / Scope / Delegation / Privileged Access / SoD Architecture  
**Canonical Technical Responsibility:** High-risk privileged/administrative access safeguards.  
**Technical Architecture Owner:** L5-D-02 — Authorization Architecture  
**Assigned Author:** Devang  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-12`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; working `L5-D-02-001_Authorization_Architecture_Baseline.md`; working `L5-D-02-002_Permission_and_Capability_Model_Architecture.md`; working `L5-D-02-003_Hierarchical_Scope_and_Resource_Context_Architecture.md`; working `L5-D-02-004_Policy_Decision_and_Enforcement_Architecture.md`; working `L5-D-02-005_Delegation_and_Temporary_Authority_Architecture.md`; `L5-D-01-001` through `L5-D-01-004`; `L5-D-03-001` and applicable D03 specializations; `L5-A-04-001`; `L5-A-04-002`; `L4-A-013_Emergency_Access_Request_to_Audited_Emergency_Access_Closure.md` v1.0.0 Draft as emergency-access boundary evidence; `L4-A-051`; `L4-A-052`; `L2-A04_Authorization_and_Policy_Workflows.md` v1.1 Locked; applicable protected-domain L2/L4 behaviors requiring privileged authorization; `ADR-PLAT-AUTH-001`; `ADR-L5-0001`; `ADR-L5-0004`.  
**Referenced By:** `L5-D-02-007`; administrative/platform operations; configuration and master-data administration; protected diagnostics; support/operations tooling; deployment/runtime operations where business authorization is required; verification/testing privileged execution; D04 Audit & Evidence.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001` for canonical authorization subject ownership; `ADR-L5-0001` for explicit trust boundaries/default-deny admission; `ADR-L5-0004` for Tenant/Institution isolation. No D02-006-specific privileged-access product, vault, session broker, workstation, network, MFA, approval-workflow, credential-management or recording product ADR is selected here.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical safeguards that apply when an operation has elevated administrative or privileged impact.

The architecture treats privilege as:

> **explicitly governed authorization to perform a high-risk protected operation within a bounded subject, Tenant, scope, resource and time context.**

Privilege is not:

- an implicit property of being an administrator;
- a property of an internal network;
- a property of a support workstation;
- a property of a database login;
- a property of a service account;
- a universal `superuser`;
- a bypass around D02 policy evaluation;
- an emergency Break-Glass mechanism by default.

The baseline flow is:

```text
Authenticated / attributable actor
        ↓
validated Tenant / Institution / resource scope
        ↓
privileged operation identified
        ↓
explicit privileged authority resolved
        ↓
required heightened safeguards satisfied
        ↓
D02-004 policy evaluation
        ↓
Allow | Deny
        ↓
privileged enforcement boundary
        ↓
high-risk operation
        ↓
enhanced evidence / audit
```

### 2.2 In Scope

This specification defines:

- privileged/administrative operation classification boundary;
- explicit privileged authority requirements;
- privileged Permission/capability consumption;
- privileged scope restrictions;
- privileged subject/actor attribution;
- standing versus temporary privileged authority as technical patterns;
- integration with D02-005 Temporary Access where source-governed;
- high-risk reauthorization/step-up control boundary where source policy requires it;
- additional approval/reference requirements where upstream policy defines them;
- high-risk enforcement-point requirements;
- privileged read/query/export controls;
- privileged configuration and mutation controls;
- privileged support/diagnostic access;
- privileged machine/workload access boundary;
- privileged session-context constraints;
- non-shared accountability;
- credential/secret non-ownership boundary;
- revocation/freshness behavior;
- privileged activity evidence;
- sensitive-data exposure minimization;
- production/non-production privileged separation principles;
- cross-Tenant prohibition;
- service-to-service privileged authorization;
- long-running privileged operation revalidation;
- privileged batch/bulk operation safeguards;
- boundary with emergency/Break-Glass access.

### 2.3 Out of Scope / Non-Goals

This specification does **not** own or define:

- canonical business roles such as “Platform Administrator” or “Security Administrator” unless already defined upstream;
- universal administrator role hierarchy;
- emergency/Break-Glass override semantics — D02-007;
- Separation of Duties or maker-checker detailed enforcement — D02-007;
- Delegation/Temporary Access lifecycle — D02-005;
- Permission definition — D02-002;
- scope hierarchy — D02-003;
- final decision engine — D02-004;
- Authentication Session, MFA, Credential or provider lifecycle — D01;
- cryptographic/key/secret storage — D03;
- operational-account creation;
- operating-system/database/cloud/IAM privilege models;
- privileged access management vendor/product;
- credential vault product;
- session-recording product;
- device-management product;
- network bastion product;
- approval workflow;
- mandatory universal MFA factor count;
- universal privileged-session duration;
- universal session-recording requirement where upstream does not establish it;
- universal “four-eyes” rule for all admin actions;
- numeric privileged-access TTL;
- named administrator approvers;
- canonical privileged-access business lifecycle aggregate not already defined upstream.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority Matrix

| Source | Status | Applicable Authority | D02-006 Effect |
|---|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | `L5-D-02-006` owns high-risk privileged/administrative access safeguards. | Establishes exact L5 responsibility. |
| working D02-001 | Draft | All protected operations use centralized authorization, least privilege and fail-closed enforcement. | Privileged operations cannot bypass family baseline. |
| working D02-002 | Draft | Permissions represent business capabilities independent of UI/account. | Privileged authority is expressed through explicit governed capabilities, not vague admin status. |
| working D02-003 | Draft | Authorization is Tenant-rooted and resource/scope bounded. | Privileged authority cannot become global/cross-Tenant by implication. |
| working D02-004 | Draft | Policy decision/enforcement is mandatory across channels. | Privileged access remains subject to final D02-004 Allow/Deny. |
| working D02-005 | Draft | Time-bounded approved authority can be represented without permanent Role mutation. | Temporary privileged elevation may use D02-005 only where source-governed. |
| L2-A04 | Locked | Least privilege, policy-driven authorization, explicit Deny, server-side enforcement, Temporary Access, Break-Glass, SoD and auditable decisions. | Supplies privileged authorization invariants. |
| applicable business/platform L2 | Locked/Draft by source | Administrative operations may explicitly require privileged authorization and audit. | D02-006 provides reusable technical safeguards without taking business ownership. |
| `L4-A-013` | Draft | Emergency elevation is exceptional, least-privilege, scoped, time-bound, policy/SoD/Deny evaluated, and fully evidenced. | Defines the D02-007 emergency boundary and confirms no emergency mechanism may become permanent privilege. |
| L4-A-051 | Draft | Current effective responsibility sources participate in SoD; conflict Deny precedence applies. | Privileged authority must remain visible to SoD. |
| L4-A-052 | Draft | Policy decision binds exact policy version; historical traceability preserved. | Privileged decisions retain exact policy-version provenance. |
| `ADR-L5-0001` | Architecture Approved | Explicit trust boundaries/default-deny protected-boundary crossing. | Internal placement never implies privilege. |
| `ADR-L5-0004` | Architecture Approved | Tenant is mandatory protected root. | Privileged access cannot cross Tenant. |
| `ADR-PLAT-AUTH-001` | Approved | Person is canonical human authorization subject; User Account/session is runtime authentication context. | Permanent privileged authority does not attach canonically to session/account identity. |

### 3.2 Privileged Access Invariants

| Constraint ID | Invariant | Architectural Effect |
|---|---|---|
| P-C01 | No privileged operation bypasses authorization evaluation. | All administrative paths consume D02-004. |
| P-C02 | Privilege is explicit; internal placement/account type does not imply authority. | No implicit admin trust. |
| P-C03 | Least privilege applies to privileged authority. | Grant only required operation/scope. |
| P-C04 | Tenant isolation applies equally to privileged operations. | No cross-Tenant superuser by default. |
| P-C05 | Explicit Deny still overrides privileged Allow facts. | Privilege is not Deny bypass. |
| P-C06 | SoD remains applicable. | Privileged authority participates in D02-007 conflict evaluation. |
| P-C07 | Privileged actions are attributable. | Anonymous/shared execution context is not sufficient for high-risk human operations. |
| P-C08 | Protected resource/action/scope remain bound to the decision. | Admin permission for one operation cannot silently widen to another. |
| P-C09 | Authentication and Authorization remain separate. | Strong authentication may be a prerequisite but never replaces privileged authorization. |
| P-C10 | Emergency access is a distinct exceptional path. | Ordinary privileged access shall not silently become Break-Glass. |
| P-C11 | High-risk operations require stronger evidence than ordinary invisible background authorization where source/control policy requires it. | D04 evidence integration. |
| P-C12 | Privileged authority changes/revocation must not leave stale effective access. | Freshness/revalidation mandatory. |
| P-C13 | Business domains retain ownership of protected operations. | D02-006 safeguards access only. |
| P-C14 | Privileged technical access to infrastructure does not grant business authority to change canonical business state outside governed contracts. | Technical/admin and business authorization remain distinct. |

### 3.3 Explicit Non-Assumptions

This specification does not assume:

- all privileged access is temporary;
- all privileged access is standing;
- all administrators require the same Permissions;
- all privileged operations require approval at execution time;
- all privileged operations require MFA step-up;
- all privileged sessions require recording;
- every admin operation requires a separate workstation/network zone;
- privileged humans may use shared accounts;
- infrastructure root/database administrator access is itself sufficient business authorization;
- a universal platform-wide super-admin role exists;
- all privileged actions are emergency actions;
- Break-Glass lifecycle states exist beyond those explicitly established upstream;
- a named PAM/bastion/vault technology.

Where stronger controls are required, the source policy or collaborating D01/D03 architecture must establish them rather than D02-006 inventing business rules.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 D02-006 Owns

- privileged operation classification boundary;
- privileged authorization context;
- explicit privileged-capability requirement;
- privileged scope/resource binding;
- heightened technical control hooks;
- privileged execution admission;
- privileged authority freshness requirements;
- privileged human/non-human attribution;
- privileged evidence requirements;
- privileged read/export/query safeguards;
- no-shared-authority invariant for attributable human execution;
- standing-vs-temporary privileged technical patterns;
- boundary with emergency override;
- high-risk enforcement/revalidation requirements.

### 4.2 D02-006 Does Not Own

- privileged business-role definition;
- privilege approval business workflow;
- Credential/MFA factor lifecycle;
- secret/key storage;
- machine identity lifecycle;
- Role Assignment lifecycle;
- Temporary Access lifecycle;
- Break-Glass lifecycle;
- SoD policy definition;
- business-domain state transitions;
- infrastructure administrator account provisioning;
- operating system/database/cloud privilege;
- network segmentation technology;
- session-recording technology.

### 4.3 Collaborating Architecture

| Architecture | Supplies / Owns | D02-006 Use |
|---|---|---|
| D01 | authenticated actor, account/session/credential context | Verifies attributable execution context; D02-006 may request stronger auth posture only where policy requires. |
| D03 | trust boundaries, secrets, cryptography, security controls | Privileged access consumes D03 protections; no implicit trust. |
| A04 | Tenant/Institution scope | Bounds privileged operations. |
| D02-002 | Permission/Capability | Defines privileged business/technical action capability. |
| D02-003 | resource/scope context | Binds admin authority to exact Tenant/resource scope. |
| D02-004 | policy decision/enforcement | Generates final Allow/Deny. |
| D02-005 | temporary authority | Realizes temporary privileged access where source-governed. |
| D02-007 | SoD, maker-checker, emergency override | Applies high-risk conflict/approval/emergency controls. |
| D04 | audit/evidence | Stores privileged decision/action provenance. |
| B05 | sensitive-data handling | Governs privileged exposure of sensitive data. |
| E05 | operations/diagnostics | Consumes privileged safeguards for mutating/admin diagnostics. |
| F02 | verification/testing | Ensures test/verification privileged execution is controlled and evidenced. |

---

## 5. Solution Architecture

### 5.1 Privileged Access Position

```text
Normal protected operation
        ↓
D02 authorization
        ↓
Allow / Deny

High-risk privileged operation
        ↓
D02 authorization
        +
privileged safeguards
        +
D03 trust/security prerequisites
        +
D02-007 controls where applicable
        ↓
Allow / Deny
```

Privileged access is not a parallel authorization system.

### 5.2 Privileged Operation Classification

D02-006 uses a technical concept:

```text
PrivilegedOperationContext
```

to mark a protected operation as requiring heightened safeguards.

This is not a new canonical business aggregate.

A domain/platform owner remains responsible for identifying whether an operation is administratively high-risk according to source architecture/policy.

Examples may include:

- security configuration change;
- authorization-policy administration;
- high-impact master/configuration administration;
- protected data export;
- tenant/institution administration;
- privileged diagnostic mutation;
- account/session administrative control;
- bulk administrative correction;
- protected recovery operation;
- sensitive audit/evidence access.

These are examples, not a complete canonical list.

### 5.3 Conceptual Privileged Authorization Context

```text
PrivilegedAuthorizationContext
{
  tenant_reference
  institution / lower_scope_reference(s)

  subject_reference
  actor_reference
  authentication_context_reference

  privileged_permission_reference
  protected_operation_reference
  resource_reference

  privileged_authority_source_reference
  authority_scope_reference
  effective_period_reference where applicable

  required_control_references[]
  satisfied_control_evidence_references[]

  policy_version_reference
  correlation_reference
}
```

This is semantic architecture, not a prescribed API or database schema.

### 5.4 Explicit Privileged Authority

Privileged authority shall be explicitly resolvable.

The system shall not infer privileged authority from:

- email domain;
- network address;
- server location;
- workstation type;
- infrastructure group name alone;
- User Account type;
- service account status;
- database ownership;
- operating system privilege;
- “internal employee” status;
- application route;
- UI menu visibility.

The protected operation must resolve a governed D02 Permission/capability and authority source.

### 5.5 No Universal Superuser Assumption

D02-006 does not define:

```text
SuperAdmin = all permissions in all Tenants
```

as a canonical model.

If upstream architecture later defines a broad administrative responsibility, it remains:

- Tenant/scope governed;
- policy evaluated;
- explicit-Deny constrained;
- auditable;
- subject to D03;
- subject to D02-007 where applicable.

### 5.6 Privileged Capability Granularity

Privileged operations should remain explicit business/technical capabilities under D02-002.

Prefer conceptually:

```text
AuthorizationPolicy.Manage
UserAccount.AdministrativelyDisable
AuditEvidence.Export
InstitutionConfiguration.Manage
MasterData.Correct
```

over opaque capability:

```text
AdminEverything
```

unless upstream explicitly defines the broader capability.

Names above are illustrative only.

### 5.7 Privileged Scope

Privileged authority is bounded by D02-003.

Conceptually:

```text
Privileged Permission
       +
Tenant
       +
Institution / lower scope
       +
resource / operation
       =
candidate privileged authority
```

A Platform/Institution administrator role in one Tenant shall not become authority in another Tenant.

### 5.8 Standing Privileged Authority

Some source-governed responsibilities may carry standing privileged capability.

Where such authority exists:

- it still resolves through Role/Role Assignment or other approved authority;
- it remains scoped;
- it remains policy/SoD/Deny controlled;
- it remains attributable;
- it does not create an emergency session;
- high-risk operations may still require additional control at execution where upstream policy requires.

D02-006 does not decide which Roles have standing privilege.

### 5.9 Temporary Privileged Authority

Where source policy requires temporary administrative elevation:

- D02-005 Temporary Access is the normal technical time-bounded mechanism unless upstream defines another authority source;
- approved Permission/scope/period is explicit;
- expiry/revocation removes authority;
- temporary privilege does not mutate permanent Roles.

D02-006 adds privileged safeguards around use of that temporary authority.

### 5.10 Emergency Privileged Authority Boundary

Emergency/Break-Glass is distinct.

```text
ordinary privileged access
    → D02-006

emergency policy-approved elevation
    → D02-007
       consuming L4-A-013 semantics
```

D02-006 shall not use a Break-Glass label merely because an administrator needs elevated access.

Emergency semantics include exceptional need, governed approval, bounded scope/duration, notification/evidence and mandatory audit-review closure. Those remain D02-007-owned.

### 5.11 Authentication Posture Boundary

Privileged authorization may require stronger authentication posture according to source policy.

D02-006 may consume D01 facts such as:

- authentication method;
- MFA satisfaction;
- session age/freshness reference;
- trusted-device/security posture where D01 supports it.

D02-006 does not:

- define MFA enrollment;
- define a universal factor count;
- set a universal step-up timeout;
- define password rules.

If required posture is absent:

```text
privileged operation
    ↓
policy requires stronger authentication
    ↓
current auth context insufficient
    ↓
no privileged Allow
```

The precise reauthentication/step-up flow belongs D01 plus policy.

### 5.12 Privileged Session Context

An authenticated session used for privileged action remains a D01 session.

D02-006 may mark that a privileged decision occurred in that session context, but it shall not invent a second session aggregate.

Privileged authority shall not be cached in session state in a way that survives:

- authority revocation;
- scope change;
- policy change;
- temporary-access expiry;
- mandatory reauthentication condition.

### 5.13 Reauthorization

High-risk operations may require re-evaluation close to execution.

Reasons include:

- current authority may have changed;
- resource state may have changed;
- approval/control evidence may be operation-specific;
- privileged consequence may be irreversible or high impact.

Exact reauthorization frequency is not numerically prescribed.

### 5.14 Privileged Decision Binding

A privileged Allow is bound to:

- subject;
- actual actor;
- Tenant;
- Institution/lower scope;
- Permission/action;
- target resource;
- policy version;
- authority source;
- satisfied heightened controls where applicable;
- effective time.

It is not transferable to another privileged operation.

### 5.15 No Internal-Network Bypass

D03 trust establishes whether a request can cross a protected technical boundary.

It does not establish business privilege.

```text
inside trusted network
        ≠
authorized administrator
```

Internal tools, maintenance consoles and service endpoints remain D02-controlled.

### 5.16 Infrastructure Privilege vs Business Privilege

Technical control of infrastructure does not automatically authorize business changes.

Example:

```text
Database administrator can technically modify rows
```

does not mean:

```text
Database administrator is authorized to alter Student, Finance,
Examination or Identity business state outside governed application contracts.
```

D02-006 requires operational architecture to minimize and govern such bypass capability.

Where unavoidable technical access exists, it requires D03/D04 controls and must not be treated as ordinary business-operation authority.

### 5.17 Privileged Support Access

Support/operations personnel may need access to diagnose incidents.

D02-006 distinguishes:

**diagnostic read** from **administrative mutation**.

Read access may itself be privileged when sensitive.

Mutation/correction requires explicit protected Permission and full D02 evaluation.

A support role shall not gain broad business write authority merely because it can view diagnostics.

### 5.18 Sensitive Data Access

Privileged access does not nullify privacy requirements.

Privileged data access may require:

- explicit protected read/export Permission;
- scope/resource restrictions;
- masking/minimization where B05 requires;
- reason/purpose context where source policy requires;
- stronger evidence.

D02-006 does not create a universal right for administrators to view all sensitive data.

### 5.19 Privileged Export / Download

High-risk export operations are separately protected.

An actor authorized to view records is not automatically authorized to export them.

Conceptually:

```text
Resource.View
    ≠
Resource.Export
```

unless upstream Permission semantics explicitly combine them.

Exports remain Tenant/scope/policy/audit controlled.

### 5.20 Privileged Bulk Operations

Bulk privileged operations amplify risk.

A bulk operation requires:

- explicit capability;
- proven authorized resource set;
- no mixed-Tenant execution;
- current policy evaluation;
- bounded scope;
- source-defined approvals/SoD where applicable;
- evidence of affected scope/resources or governed batch reference.

D02-006 does not invent partial-success business semantics.

### 5.21 Privileged Configuration Change

Configuration change may affect future authorization/system behavior.

For privileged configuration operations:

- change authority is explicit;
- applicable scope is explicit;
- target configuration owner remains canonical;
- change does not become active merely because client submits it if upstream governance requires approval/activation;
- evidence preserves actor, scope, before/after references where peer architecture supplies them.

### 5.22 Authorization Administration

Administration of:

- Roles;
- Permissions;
- Role Assignments;
- policies;
- SoD configuration;
- Delegation/Temporary Access;

is itself privileged.

D02-006 safeguards the administrative operation, but each D02 child/source owner retains semantic ownership of the changed object.

No actor can use “authorization administrator” status to bypass the rules governing the authorization objects being changed.

### 5.23 Identity/Account Administration

Administrative actions on User Accounts, credentials, sessions or recovery remain D01-owned business/technical behavior.

D02-006 contributes privileged authorization safeguards.

It does not directly:

- reset Credential secrets;
- terminate sessions;
- unlock accounts;
- create accounts.

These operations require explicit permissions/policy and D01 execution contracts.

### 5.24 Audit/Evidence Administration

Access to D04 evidence is privileged where sensitive.

D02-006 requires:

- separate read/export/manage capabilities;
- no ability to rewrite immutable evidence merely because an actor is privileged;
- D04 tamper-evidence/retention controls remain authoritative;
- audit administrators cannot use D02 authority to violate D04 invariants.

### 5.25 Operational Diagnostics

Operational diagnostic tools may expose sensitive runtime state.

D02-006 requires:

- authenticated attributable actor;
- explicit diagnostic capability;
- scope/target service context;
- D03 boundary controls;
- D02-004 decision;
- evidence.

Mutation through diagnostics requires separately authorized privileged mutation capability.

### 5.26 Production vs Non-Production

Environment placement does not erase privilege.

Production operations are typically higher risk, but D02-006 does not invent environment-specific business rules.

If environment is a source-defined policy attribute, D02-004 can evaluate it.

A non-production administrator shall not be inferred to have production privilege.

### 5.27 Shared Human Accounts

For high-risk human privileged activity, the architecture requires attributable actor identity.

Shared human credentials/accounts undermine attribution and shall not be used as the normal privileged authorization model.

If a legacy/shared operational mechanism cannot provide individual attribution, it is a governance/security exception requiring explicit treatment outside this baseline.

### 5.28 Non-Human Privileged Actors

Service/workload/job/integration principals may need privileged capabilities.

D02-006 requires:

- canonical D01 non-human identity;
- explicit capability;
- Tenant/scope binding;
- least privilege;
- workload/service trust under D03;
- no use of human Role Assignment fiction merely to obtain privilege;
- D02-004 decision/equivalent governed machine authorization path;
- full technical attribution.

Exact machine-authority binding remains a known D01/D02 design gap where upstream is Person-focused.

### 5.29 Human Impersonation

D02-006 does not assume administrators may impersonate users.

If a product requires support impersonation:

- it needs explicit upstream semantics;
- original administrator identity must remain attributable;
- impersonated subject and actual actor must be distinct;
- scope/time/purpose constraints are required;
- policy/SoD/evidence still apply.

No impersonation mechanism is defined here.

### 5.30 “Run As” / Delegated Administration

Similarly, “run as” or delegated administration is not equivalent to changing authentication identity.

Where source policy allows acting on behalf of another subject:

- actual actor remains visible;
- source authority remains visible;
- target subject remains visible;
- D02-005 may apply where this is true Delegation;
- policy remains final.

### 5.31 Approval Hooks

Some privileged operations may require source-governed approval or maker-checker.

D02-006 provides an enforcement hook:

```text
privileged operation
     ↓
required approval/control reference?
     ↓
valid source-governed evidence exists?
     ↓
continue to D02-004
```

It does not invent approver roles or approval workflows.

Detailed maker-checker is D02-007.

### 5.32 SoD Integration

Privileged authority participates in SoD evaluation.

An administrator shall not be exempt merely because the action is administrative.

D02-007 may deny actions where:

- actor holds conflicting responsibilities;
- maker/checker separation is violated;
- policy prohibits self-approval;
- emergency continuation requirements are unmet.

### 5.33 Temporary Privileged Elevation

Where policy requires time-bounded elevation, D02-006 + D02-005 cooperate:

```text
approved temporary access
        ↓
privileged Permission + scope + period
        ↓
D02-006 privileged safeguards
        ↓
D02-004 decision
```

Expiry/removal semantics remain D02-005.

### 5.34 Privileged Access Revocation

Revocation of privileged authority must become effective for future protected operations.

Potential sources:

- Role Assignment suspension/revocation;
- Role→Permission change;
- Temporary Access expiry/revocation;
- policy change;
- account/session control under D01;
- organizational source change where applicable.

D02-006 requires stale privilege to be invalidated/revalidated through D02-004 freshness architecture.

### 5.35 Policy Change

A previously valid privileged actor can be denied after policy change.

Future decisions use the authoritative current policy version.

Historic privileged decisions retain original policy-version reference.

### 5.36 Privileged Operation TOCTOU

For high-impact mutation:

```text
authorization check
        ↓
resource / authority changes
        ↓
mutation
```

can create risk.

D02-006 requires D02-004/B02 to use appropriate revalidation/version/transaction safeguards where material.

No universal locking mechanism is prescribed.

### 5.37 Long-Running Privileged Operations

Long-running jobs such as administrative bulk correction, reindexing with protected mutation, migration, or repair may outlive initial authorization context.

The architecture shall not assume one initial Allow authorizes all future privileged mutations indefinitely.

Each material protected step or governed batch execution must preserve current valid authority according to D02-004.

### 5.38 Administrative Automation

Automated privileged maintenance requires a non-human principal and explicit machine capability.

It shall not rely on:

- embedded human credentials;
- a developer’s session;
- anonymous scheduler identity;
- unrestricted database owner credentials as business authority.

### 5.39 Read-Only Does Not Mean Non-Privileged

Read-only access can still expose:

- personal data;
- financial data;
- examination data;
- credentials/security configuration;
- audit evidence;
- system diagnostics.

Therefore privileged classification is based on risk and policy, not only mutation capability.

### 5.40 Direct Persistence Access

Direct database/storage access is outside normal application authorization semantics and is inherently high risk.

Where operational necessity requires it:

- D03 controls technical access;
- D02 policy should govern the business/admin authorization basis where possible;
- D04 evidence must preserve accountable activity;
- canonical business state should be changed through owned services/contracts rather than ad hoc direct mutation wherever architecture permits.

D02-006 does not define DBA tooling.

### 5.41 Privileged API/Console Consistency

A privileged action exposed through multiple channels must use equivalent authorization semantics.

```text
Admin UI
Admin API
CLI / tool
Internal service
Background operation
```

shall not each implement independent privilege rules.

### 5.42 Enforcement Point

A privileged enforcement point shall verify:

1. valid attributable actor;
2. valid Tenant/scope;
3. required privileged capability;
4. authority source;
5. any required heightened authentication/control evidence;
6. resource/action match;
7. current policy decision;
8. SoD/maker-checker result where applicable;
9. freshness;
10. D03 protected-boundary prerequisites.

Only then may the privileged operation execute.

### 5.43 Privileged Decision Cache Safety

Caching is optional.

If used:

- privilege state is never cached as `isAdmin=true` without authority context;
- cache keys bind Tenant, subject, capability, resource/scope and policy/authority version;
- revocation/expiry/policy change invalidates stale privilege;
- cache miss does not default Allow;
- cross-Tenant cache contamination is prohibited.

### 5.44 Privileged Error Handling

Error responses should not reveal unnecessary sensitive control information.

Externally visible response may state access denied or control requirement failure.

Internal evidence can retain the governed diagnostic reason.

D02-006 does not prescribe HTTP status codes.

### 5.45 Evidence Model

Privileged evidence may include:

- actor/subject;
- Tenant/Institution/scope;
- privileged capability;
- protected resource/operation;
- authority source;
- authentication posture reference where policy-relevant;
- temporary authority reference if used;
- required approval/control references;
- SoD result reference;
- policy version;
- final decision;
- enforcement point;
- operation correlation;
- outcome reference;
- time;
- reason for Deny/control failure.

Credentials/secrets shall not be logged.

### 5.46 Before/After Evidence

Where source domain/D04 requires change evidence for administrative mutation, D02-006 may correlate authorization evidence with source-provided:

- prior state reference;
- resulting state reference;
- change-set/reference;
- operation identifier.

D02-006 does not duplicate business records into the authorization store.

### 5.47 Privileged Evidence Immutability

Being privileged shall not grant permission to rewrite historical privileged evidence.

D04 owns tamper-evident storage/access/export.

Any evidence-management capability is separately authorized.

### 5.48 Emergency Boundary with D02-007

L4-A-013 defines emergency elevation with:

- exceptional justification;
- approval;
- approved scope/duration;
- normal policy/SoD/Explicit Deny evaluation;
- administrator notification;
- automatic/early cessation;
- mandatory audit review before closure.

D02-006 consumes none of those as ordinary privileged defaults.

Instead:

```text
normal admin need
   → D02-006

exceptional emergency need
   → D02-007 + L4-A-013
```

### 5.49 Failure Model

| Failure / Condition | Required Outcome |
|---|---|
| privileged capability unresolved | Deny / no execution. |
| actor not attributable | No human privileged execution. |
| Tenant missing/mismatched | Fail closed. |
| scope broader than authority | Deny/restrict; no widening. |
| authentication posture required by policy but not satisfied | No privileged Allow. |
| privileged authority expired/revoked | Exclude from evaluation. |
| explicit Deny applies | Deny. |
| SoD conflict applies | D02-007/D02-004 determines governed Deny/continuation; privilege does not bypass. |
| privileged control evidence missing | No Allow where mandatory. |
| internal service is trusted but lacks business authority | Deny protected operation. |
| client asserts administrator status | Ignore as authority. |
| cache says admin but source authority is stale | Revalidate / fail closed. |
| direct DB/admin technical access exists | Does not create business authorization; separate D03/D04 controls required. |
| emergency condition claimed without governed Break-Glass basis | Do not treat as emergency authority. |
| audit/evidence write fails where mandatory | Do not silently claim controlled privileged completion. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Rationale / Basis | ADR |
|---|---|---|---|
| AD-001 | Privileged/administrative access remains part of the normal centralized D02 authorization model. | L2-A04 mandatory policy-driven authorization. | None. |
| AD-002 | Privilege must be explicit through governed capability/authority; internal location/account type does not imply privilege. | Least privilege + D03 separation. | `ADR-L5-0001` constraint. |
| AD-003 | No universal cross-Tenant `superuser` model is introduced. | Tenant isolation + zero-guessing. | `ADR-L5-0004`. |
| AD-004 | Privileged authority is bound to subject/actor, capability, scope, resource, policy and effective authority source. | D02-002/003/004. | None. |
| AD-005 | Standing privileged authority and temporary privileged authority are supported as distinct technical patterns where source-governed. | Avoids forcing all admin access into one model. | None. |
| AD-006 | Temporary privileged elevation reuses D02-005 where applicable and does not mutate permanent Roles. | D02-005 invariants. | None. |
| AD-007 | Authentication posture may be a privileged policy input, but D02-006 does not invent universal MFA/session rules. | D01 ownership + zero-guessing. | None. |
| AD-008 | Privileged human execution must remain individually attributable; shared human privilege is not the normal architecture. | Auditability/security. | None. |
| AD-009 | Privileged non-human execution requires explicit machine identity/capability/scope and cannot masquerade as human Person authority. | D01 non-human identity boundary. | None. |
| AD-010 | Infrastructure privilege and business authorization are distinct. | Canonical business ownership and least privilege. | None. |
| AD-011 | Privileged read/export can require separate capability from ordinary read. | Sensitive-data/least-privilege boundary. | None. |
| AD-012 | Emergency/Break-Glass remains D02-007-owned and is not treated as a synonym for privileged access. | L4-A-013 + frozen corpus. | None. |
| AD-013 | Privileged authorization failure is non-permissive and cannot fall back to a weaker “internal admin” path. | D02-004 + D03 fail closed. | `ADR-L5-0001`. |
| AD-014 | Privileged actions require enhanced evidence/correlation where applicable without logging credentials/secrets. | AP-11 + D04/D03. | None. |
| AD-015 | No PAM, vault, bastion, privileged workstation, session recorder, IAM vendor, MFA product, database, cache or numeric privileged-session target is selected here. | L5 Technology Selection & ADR Methodology. | Future ADR only if material. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaborator |
|---|---|---|
| Admin bypass | All admin operations remain D02-004 controlled. | D02-004/006 |
| Cross-Tenant privilege | Prohibited; no universal superuser assumption. | A04/D02 |
| Privilege escalation | Explicit capability + authority source + least privilege. | D02-002/006 |
| Authentication strength | Policy can require stronger posture without D02 owning MFA. | D01/D02 |
| Internal service misuse | Trusted network/service does not equal business privilege. | D03/D02 |
| Sensitive-data exposure | Privileged read/export separately governed; B05 still applies. | B05/D02 |
| Shared accounts | Individual human attribution required for high-risk activity. | D01/D04 |
| Machine privilege | Explicit workload identity and machine authority. | D01/D02 |
| Stale privilege | Revalidation/invalidation after authority/policy change. | D02-004/B02/C02 |
| SoD | Privileged authority remains visible to conflict evaluation. | D02-007 |
| Emergency misuse | Emergency path is separate and governed. | D02-007 |
| Audit integrity | Privileged users cannot implicitly rewrite evidence. | D04 |
| Support diagnostics | Read/mutation differentiated and individually authorized. | E05/D02 |
| Bulk admin operations | Explicit scope and affected resource-set control. | D02-003/004/006 |
| Direct persistence | Treated as exceptional technical risk, not normal business authorization. | D03/D04/B02 |
| Availability | Outage does not create admin bypass. | E01/D02-004 |

No numeric SLA, privileged-session duration, reauthentication interval, approval timeout, log retention, cache TTL or review interval is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Conformance Requirement | Basis |
|---|---|---|
| CR-001 | Every privileged/administrative protected operation shall pass D02-004 authorization before execution. | AP-01/AP-05. |
| CR-002 | Internal network location, infrastructure placement or application trust shall not independently grant privileged business authority. | D03/D02 separation. |
| CR-003 | A client-supplied `admin`/`superuser` flag shall not be accepted as authoritative privileged context. | server-side authorization. |
| CR-004 | Privileged authority shall resolve an explicit governed Permission/capability. | D02-002. |
| CR-005 | Privileged authority shall remain Tenant-bound and shall not authorize cross-Tenant execution. | A04 / `ADR-L5-0004`. |
| CR-006 | Privileged authority shall remain constrained by Institution/lower scope/resource where applicable. | D02-003. |
| CR-007 | Explicit Deny shall remain effective against privileged actors. | L2-A04 AP-06. |
| CR-008 | SoD/maker-checker controls shall remain applicable to privileged actors where governed. | D02-007. |
| CR-009 | Privileged authority shall not by itself bypass D03 trust/cryptographic/secret/security controls. | D03. |
| CR-010 | Stronger authentication posture may be required only through governed D01/policy semantics; D02-006 shall not invent universal factor/session rules. | D01 boundary. |
| CR-011 | Valid authentication shall not independently grant privilege. | AuthN/AuthZ separation. |
| CR-012 | Privileged human execution shall retain individual actor attribution. | audit/security. |
| CR-013 | Permanent privileged business authority shall not be canonically attached to a User Account/session in place of Person/Role Assignment semantics. | `ADR-PLAT-AUTH-001`. |
| CR-014 | Privileged non-human execution shall use explicit service/workload/job/integration identity and shall not fabricate human Person authority. | D01 boundary. |
| CR-015 | Temporary privileged authority shall use governed temporary authority semantics and shall not permanently mutate Role/Permission state. | D02-005. |
| CR-016 | Expired/revoked privileged authority shall not remain effective through stale cache/session context. | freshness. |
| CR-017 | A privileged Allow shall be bound to the actual operation/resource/scope and shall not be reused as generic administrator authority. | D02-004. |
| CR-018 | Privileged query/list/export shall be constrained to authorized result scope before disclosure. | D02-003/004. |
| CR-019 | Privileged export/download shall not be inferred solely from ordinary read capability unless upstream Permission semantics explicitly define that equivalence. | least privilege. |
| CR-020 | Privileged bulk operations shall validate all affected resources or a proven governed batch scope. | D02-003/004. |
| CR-021 | Mixed-Tenant privileged bulk operations shall be prohibited. | A04. |
| CR-022 | Privileged support/diagnostic mutation shall require separate explicit authorization from diagnostic visibility. | §5.17/5.25. |
| CR-023 | Infrastructure administrator/root/database-owner technical ability shall not be treated as canonical business authorization. | §5.16/5.40. |
| CR-024 | Canonical business state should be mutated through owning services/contracts rather than ad hoc privileged persistence modification where architecture permits. | ownership boundary. |
| CR-025 | Privileged configuration administration shall not bypass configuration owner/governance activation semantics. | canonical ownership. |
| CR-026 | Authorization administration itself shall be privileged and shall not bypass rules of the authorization aggregate being changed. | D02 ownership. |
| CR-027 | Identity/account administrative execution shall remain D01-owned and merely consume D02-006 safeguards. | D01 boundary. |
| CR-028 | Audit/evidence administration shall remain D04-owned and privileged authority shall not permit historical evidence rewrite. | D04 boundary. |
| CR-029 | Privileged operations shall preserve policy-version and authority-source provenance. | L4-A-052/D04. |
| CR-030 | Privileged actions shall not rely on hidden/visible UI state as security enforcement. | AP-03. |
| CR-031 | Privileged APIs, consoles, CLIs, internal services and jobs shall use equivalent D02 authorization semantics. | AP-13. |
| CR-032 | Long-running privileged operations shall not assume the initial Allow remains valid indefinitely. | D02-004 freshness. |
| CR-033 | Privileged decision caches shall be context/version bound and shall fail closed on staleness/uncertainty. | D02-004. |
| CR-034 | Shared human privileged accounts shall not be the normal architecture for operations requiring individual accountability. | attribution. |
| CR-035 | Human impersonation or “run as” shall not be introduced without explicit upstream semantics and actor/subject separation. | zero-guessing. |
| CR-036 | Privileged read-only access may still be classified and controlled as privileged where policy/data sensitivity requires. | security/privacy. |
| CR-037 | Emergency/Break-Glass claims shall not create authority unless D02-007/L4-A-013 governed conditions are satisfied. | emergency boundary. |
| CR-038 | D02-006 shall not invent Break-Glass lifecycle states, approvers, revokers, notification recipients, commands or events. | L4-A-013 open gaps. |
| CR-039 | Privileged evidence shall not include credentials, secrets, secret material or unnecessary sensitive payload. | D03/D04/B05. |
| CR-040 | Failure of privileged-control validation shall not route the operation through a weaker local administrator bypass. | fail closed. |
| CR-041 | Environment differences shall not automatically imply privilege transfer from non-production to production. | scope/least privilege. |
| CR-042 | Privileged authority changes/revocation shall invalidate or force revalidation of derived effective access. | freshness. |
| CR-043 | Policy changes shall apply to future privileged decisions using the authoritative current policy version. | L4-A-052. |
| CR-044 | Historic privileged decisions shall retain the exact policy version and authority context evaluated. | L4-A-052/D04. |
| CR-045 | No implementation shall infer a mandatory PAM, vault, bastion, workstation, recorder, IAM product, database, cache, MFA product, or numeric duration from this specification. | AD-015. |

---

## 9. Open Decisions & Deferred Design

| Item ID | Open / Deferred Item | Classification | Source / Gap | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| OD-001 | Applicable authorization L4 sources remain Draft. | Upstream approval | L4-A-013/A051/A052 and applicable protected-domain L4 | **Yes for Architecture Approved; No for Draft** | Promote/authorize applicable sources before formal D02 approval. |
| OD-002 | D02-001 through D02-005 remain working authoring artifacts pending stabilization. | Peer dependency | current D02 sequence | Yes before family approval if contracts materially change | Reconcile D02-006 with committed canonical peers. |
| OD-003 | Canonical enterprise taxonomy of privileged/high-risk operations is not defined in one upstream source. | Cross-domain governance | protected domains define administrative operations independently | No for framework Draft; yes for complete enterprise catalog | Define/register privileged operation classification through canonical owners without moving business ownership into D02. |
| OD-004 | Whether specific privileged capabilities require standing Role authority versus Temporary Access. | Policy/governance | not universally defined | Yes per capability before implementation | Source Authorization Policy must decide; D02-006 supports both patterns. |
| OD-005 | Which privileged operations require stronger authentication/step-up and what posture qualifies. | D01/D02 policy | no universal upstream rule | Yes for high-risk implementation requiring step-up | Define through Authentication/Authorization policy; do not hard-code here. |
| OD-006 | Universal privileged-session duration/reauthentication rules are undefined. | D01/policy | not established | No for Draft; yes if product requires such session policy | Resolve in D01 + Authorization Policy. |
| OD-007 | Whether privileged session recording is mandatory for any operation class. | Security/compliance policy | not universally established | No for Draft | Decide under D03/D04/compliance governance; select product only via ADR if material. |
| OD-008 | Shared/legacy operational account exception handling. | Security governance | normal architecture requires attribution | Yes if legacy shared access exists | Define explicit exception/migration control; do not normalize shared human privilege. |
| OD-009 | Exact non-human privileged authority binding model. | D01/D02 upstream gap | non-human principal supported; Role Assignment Person-focused | Yes for production machine privileged actions | Govern machine authority without fabricated Person semantics. |
| OD-010 | Human support impersonation / “run as” semantics. | Upstream behavior gap | not canonically defined | Yes if feature is required | Create explicit upstream scenario/behavior and preserve actor/subject evidence. |
| OD-011 | Exact privileged-access approval hooks / maker-checker requirements per operation. | D02-007/upstream | operation-specific | No for baseline; yes per governed operation | Resolve through source policy and D02-007. |
| OD-012 | Exact privileged cache/invalidation mechanism. | D02-004/B02/C02 | technology not selected | No for Draft; yes before optimization | Select with fail-closed freshness. |
| OD-013 | Exact infrastructure-access integration between D02 business authorization and D03/cloud/OS/database technical privilege. | Cross-L5 architecture | platform-specific | No for product-neutral Draft; yes before operational implementation | Define canonical technical control handoff without treating infrastructure privilege as business authority. |
| OD-014 | Exact privileged evidence capture/recording granularity for high-risk operations. | D04/B05 | source/compliance-specific | No for Draft; yes before implementation | D04 defines evidence schema/retention; B05 governs sensitive content. |
| OD-015 | Break-Glass lifecycle states, roles/authorities, commands/events and audit-review closure authority remain undefined. | Upstream emergency gap | L4-A-013 §10 | Not D02-006 blocking; **D02-007 blocking for exact emergency implementation** | Preserve boundary and resolve in upstream/D02-007. |
| OD-016 | Exact privileged-access management, vault, bastion, device-trust or session-control technology. | Technology | no ADR selected | No for Draft | Select through L5 Technology Selection & ADR Methodology if required. |

---

## 10. Traceability Summary

### 10.1 Canonical Responsibility

`L5-D-02-006` realizes the frozen L5 responsibility:

> **High-risk privileged/administrative access safeguards.**

### 10.2 Primary D02 Traceability

- D02-001: authorization mandatory, least privilege, fail closed.
- D02-002: privileged action uses explicit Permission/Capability.
- D02-003: privileged access remains Tenant/resource/scope bounded.
- D02-004: final policy decision/enforcement remains mandatory.
- D02-005: time-bounded privileged elevation may use governed Temporary Access.
- D02-007: SoD/maker-checker/emergency override remains separate.

### 10.3 Locked L2 Traceability

- Authorization remains centralized and policy-driven.
- Explicit Deny overrides Allow.
- Least privilege applies.
- Server-side enforcement is mandatory.
- Privileged/administrative operations remain auditable.
- No privileged operation bypasses authorization evaluation.
- Break-Glass is separate controlled emergency authorization rather than permanent administrator privilege.

### 10.4 L4-A-013 Boundary Traceability

D02-006 uses `L4-A-013` only to define the emergency boundary:

- `BH-001`: emergency access requires resolvable Tenant, subject/authentication, justification, scope and operation.
- `BH-002`: Break-Glass policy, least privilege, SoD and Explicit Deny apply.
- `BH-003`: emergency approval required.
- `BH-004`: emergency authority is scoped/time-bounded and does not alter permanent Role/Permission/Role Assignment.
- `BH-006`: normal Authorization Policy, SoD and Explicit Deny still apply.
- `BH-007`: emergency authority ceases on expiry/revocation.
- `BH-008/BH-009`: mandatory audit review/evidence are part of emergency closure.
- Break-Glass lifecycle state names/roles/commands/events remain unresolved and are not invented in D02-006.

### 10.5 Architecture Flow

```text
D01 actor/authentication
        +
A04 Tenant/scope
        +
D02-002 privileged capability
        +
D02-003 resource/scope context
        +
D02-005 temporary authority when applicable
        +
D02-006 privileged safeguards
        +
D02-007 SoD/maker-checker/emergency when applicable
        ↓
D02-004 policy decision & enforcement
        ↓
Allow | Deny
        ↓
privileged protected operation
        ↓
D04 evidence
```

### 10.6 Architecture Decisions

`AD-001` through `AD-015`.

### 10.7 Conformance Requirements

`CR-001` through `CR-045`.

### 10.8 Open Decisions

`OD-001` through `OD-016`.

### 10.9 Canonical Repository Placement

```text
architecture/
└── L5_Solution_Architecture/
    └── specifications/
        └── L5-D/
            └── L5-D-02-006_Privileged_and_Administrative_Access_Architecture.md
```

No nested `L5-D/L5-D-02/` folder is introduced.
