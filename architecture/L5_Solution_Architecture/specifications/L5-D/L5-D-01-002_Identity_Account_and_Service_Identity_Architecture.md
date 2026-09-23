# Identity Account & Service Identity Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-01-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-01 — IAM Architecture  
**Canonical Technical Responsibility:** User-account, service/workload/job/integration identity technical models without duplicating Person.  
**Technical Architecture Owner:** L5-D-01 — IAM Architecture  
**Assigned Author:** Devang  
**Version:** `1.4.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-01-001` v1.3.0 Architecture Approved; `L4-A-004` v1.0.1 Architecture Approved; `L4-A-007` v1.0.1 Architecture Approved; `L4-A-008` v1.0.1 Architecture Approved; `L4-A-053` v1.0.1 Architecture Approved; `L2-A01` v1.0.0 Locked; `L2-A02` Locked; `L2-A03` Locked; `ADR-PLAT-AUTH-001` Approved.  
**Referenced By:** Planned `L5-D-01-003`, `L5-D-01-004`, `L5-D-02-*`, `L5-A-08-*`, `L5-C-03-*`, and other technical consumers requiring an authenticated human or non-human principal context.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001`; `ADR-L5-0039`; `ADR-L5-0048`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the technical identity model by which SutraERP represents an executable principal without duplicating upstream Person or Identity business truth.

It establishes two materially different identity paths:

1. **human account principals**, which bind IAM technical execution to authoritative Tenant, Person, Identity and User Account references owned upstream; and
2. **non-human principals**, which represent service, workload, job and integration execution identities as explicitly modeled, Tenant-aware technical identities.

The specification also defines the technical boundary required to preserve attribution when a human-triggered operation is executed through one or more technical principals. It does not grant authorization and does not own credential, session or authentication mechanism internals.

### In Scope

- Technical principal classification required to distinguish human account principals from non-human service/workload/job/integration principals.
- Technical binding of a human IAM principal to authoritative Tenant, Person, Identity and User Account references without duplicating their canonical business lifecycle.
- Canonical L5 technical model for non-human service, workload, job and integration identities.
- Tenant binding and technical ownership reference for every non-human principal.
- Separation of principal identity from credential secret material and authentication mechanism details.
- Execution-principal context that can preserve both an originating human principal and a calling/executing non-human principal when both materially participate.
- Identity/account eligibility handoff to authentication and authorization without granting permissions.
- Persistence and consistency boundaries for D01-002-owned technical identity records and upstream reference bindings.
- Failure behavior for unresolved, cross-Tenant, orphaned, ambiguous or ineligible principal context.

### Out of Scope / Non-Goals

- Canonical Person, Identity or User Account business lifecycle states, transitions, events or ownership — upstream A02.
- Credential material, authentication factors, Authentication Provider internals, Authentication Session internals or authentication protocol realization — `L5-D-01-003`.
- Account provisioning/recovery/disablement workflow and synchronization mechanics — `L5-D-01-004`.
- Role, Permission, capability, authorization policy, scope evaluation, delegation, privileged access or SoD — `L5-D-02-*`.
- Secrets, certificates, cryptographic algorithms, key custody or machine-credential protection — `L5-D-03-*`.
- Audit/evidence persistence and tamper-evident storage — `L5-D-04-*`.
- Physical database technology, schema/DDL or storage engine — `L5-B-02-*`.
- API/message schema and transport technology — `L5-C-01-*` / `L5-C-02-*`.
- Background-job scheduling/execution semantics — `L5-A-08-*`.
- External connector behavior — `L5-C-03-*`.
- Runtime/process/container/cloud topology — `L5-E-04-*`.
- L6 login/account/service-account administration screens or user journeys.
- Selection of an IAM product, directory, identity provider, service-account product, workload-identity platform, federation protocol, token format, database, cloud service or orchestration platform.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-D-01-002` row | This specification owns user-account and service/workload/job/integration identity technical models without duplicating Person. | Establishes the canonical L5 responsibility and permits a distinct non-human technical-principal model. |
| L4 | `L4-A-004_Person_Establishment_to_Active_Canonical_Identity.md` | v1.0.1 / Architecture Approved | `L4-A-004-BH-011`; §8 completion boundary | Active canonical Person/Identity references become available for governed downstream use without ownership transfer; A004 itself does not create User Account or authentication state. | Human principal binding consumes canonical upstream references and shall not reproduce Person/Identity truth. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-007-BH-001`, `BH-002` | Authentication is bound to exactly one Tenant and canonical Identity and shall not proceed when User Account status does not permit authentication. | Requires D01-002 to provide unambiguous Tenant/Identity/account technical principal context and source-governed eligibility to D01-003. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-007-BH-008`, `BH-009` | Successful authentication exposes authenticated context to Authorization without granting permission and preserves attributable authentication evidence. | Requires a principal context that is identifiable and attributable but carries no authorization grant. |
| L4 | `L4-A-008_Lost_or_Invalid_Credential_to_Restored_Authentication_Capability.md` | v1.0.1 / Architecture Approved | `L4-A-008-BH-001`, `BH-004`, `BH-008` | Recovery requires resolvable Tenant/Identity/User Account context; replacement Credential handling does not change Person, Identity or User Account lifecycle; evidence excludes Credential values. | Reinforces separation between authoritative account identity and D01-003 credential state. |
| L4 | `L4-A-053_Person_Institutional_Exit_to_Governed_Identity_Closure.md` | v1.0.1 / Architecture Approved | `L4-A-053-BH-001`, `BH-002`, `BH-004` | Closure resolves Person/Identity/User Account references, disables login capability and prevents authentication while preserving canonical identity ownership and historical references. | D01-002 must preserve stable principal/reference identity while allowing D01-004/D01-003 to enforce source-governed login consequences. |
| L2 | `L2-A01_Platform_Tenant_Foundation_Architecture.md` | v1.0.0 / Locked | `P4 Tenant Isolation`, `P5 Context-Driven Execution`, `AP-06 Tenant Awareness`; shared security governance for non-human identities | Every operation executes inside one Tenant; non-human identities are explicitly modeled and Tenant-aware; shared platform security contracts govern service-to-service trust. | Establishes Tenant binding for all human and non-human principal records and forbids context-free technical identities. |
| L2 | `L2-A02_Person_and_Identity_Workflows.md` | Locked | `Canonical Ownership Amendment — INT-301`; `Canonical Person–Identity Establishment and Activation Sequencing Decision — 2026-08-20`; `Canonical Ownership` | A02 owns Person, Identity and User Account; User Account is created only after Person/Identity activation where login is required; Credential/session/MFA/provider remain A03-owned. | D01-002 binds to, but never becomes, canonical owner of human Person/Identity/User Account business truth. |
| L2 | `L2-A03_Authentication_Workflows.md` | Locked | `Out of Scope`; `Canonical Ownership`; non-human identity/service-account security rules | Authentication owns credential/session/provider execution; service accounts and machine identities are non-human and Tenant-aware; API credentials are for system integrations and are not human login credentials. | Requires hard separation between human account principals, non-human principals and their D01-003 credential mechanisms. |
| ADR | `ADR-PLAT-AUTH-001.md` | Approved | §§2, 3.1, 3.2 | Person is Tenant-scoped; User Account is authentication/runtime account context and not the canonical Role Assignment subject; authentication and authorization remain separate. | Prevents D01-002 from treating User Account or service identity as a business-role assignment or permission grant. |
| L5 Working Dependency | `L5-D-01-001_IAM_Architecture_Baseline.md` | current / Review | §4; §5.1 Identity & Account Reference Boundary; `AD-001`–`AD-003` | Establishes the working D01 separation among upstream identity/account truth, authentication execution and downstream authorization. | D01-002 refines the identity/account technical model while remaining subordinate to repository governance and `OD-002`. |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Every principal context shall resolve to exactly one Tenant before it may participate in authentication or protected execution. | L2-A01 `P4`, `AP-06`; L4-A-007-BH-001 | Human and non-human technical principal records are Tenant-bound; cross-Tenant identity reuse is prohibited. |
| C-002 | D01-002 shall not create a duplicate canonical Person, Identity or User Account business record. | Frozen `L5-D-01-002` responsibility; L2-A02 INT-301; L4-A-004-BH-011 | Human principal records contain governed references/bindings only; upstream A02 remains source of truth. |
| C-003 | A User Account is authentication/runtime context, not the canonical subject of Role Assignment. | `ADR-PLAT-AUTH-001` §3.2 | D01-002 may identify the authenticating account but shall not encode Roles, Permissions or assignment ownership. |
| C-004 | Service accounts and machine identities are non-human, shall be explicitly modeled and shall be Tenant-aware. | L2-A01 shared security governance; L2-A03 non-human identity rules | Non-human principals require their own D01-002 technical records and shall not be represented by fake Person/User Account records. |
| C-005 | API/system-integration credentials shall not be treated as human login credentials. | L2-A03 non-human identity rules | Identity classification and credential binding must prevent accidental interchange of human and system credential paths. |
| C-006 | Identity/account eligibility must remain source-governed. | L4-A-007-BH-002; L4-A-008-BH-001/BH-004; L4-A-053-BH-002/BH-004 | D01-002 propagates eligibility/binding context but shall not invent an `Active`/`Enabled` mapping or override upstream lifecycle state. |
| C-007 | Principal identity and credential secret material are separate technical responsibilities. | Frozen D01 split; L2-A02 INT-301; L2-A03 canonical ownership | D01-002 owns principal/binding identity; D01-003 owns credential/session/authentication mechanisms; D03 owns secret/cryptographic controls. |
| C-008 | Authenticated identity context is not authorization. | L4-A-007-BH-008; ADR-PLAT-AUTH-001 | D01-002 outputs principal identity context only; D02 remains mandatory for permission/policy decisions. |
| C-009 | Technology/product selection requires canonical L5 governance. | L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0 | Keycloak 26.x is the canonical IAM platform and non-human service/workload authentication is governed by `ADR-L5-0039`; database/cloud identity details not otherwise selected remain with their canonical owners. |

### 3.3 Assumptions

No material implementation assumptions are treated as facts.

The specification deliberately avoids defining non-human identity lifecycle states, credential formats, external-subject mapping protocols or account-status mappings that are not established by authoritative sources. Any such requirement is carried as an open item in Section 9 rather than guessed.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The D01 technical principal model distinguishing **human account principal** and **non-human principal** without changing upstream business identity semantics.
- Human principal binding from a D01 technical principal reference to exactly one Tenant and the relevant authoritative Person/Identity/User Account references.
- Canonical L5 technical identity representation for service, workload, job and integration principals.
- The rule that non-human principal identity shall be explicitly modeled rather than represented through a fabricated Person or reused human User Account.
- The rule that every non-human principal shall carry a resolvable Tenant binding and technical ownership/association context sufficient to prevent orphaned execution identity.
- Principal-class separation required to prevent system-integration credentials from being treated as human-login credentials.
- Construction of an execution-principal context that can preserve the originating human identity separately from the calling/executing technical principal where both are present.
- Principal-identity persistence semantics and reference-integrity requirements inside the D01-002 boundary.
- The principal-context handoff to authentication, authorization, jobs, connectors and audit/provenance peers without granting authority or exposing credential secrets.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-D-01-001` | IAM Architecture Baseline | Supplies family-level IAM ownership and authentication/authorization separation. | D01-002 refines only the identity/account technical model. Current artifact is in Review and must be consumed according to its live validated boundaries. |
| `L5-D-01-003` | Authentication, Session & Credential Architecture | Owns authentication mechanisms, Credential, MFA, provider and session technical realization. | D01-002 supplies the principal/account identity to be authenticated; it does not own secret or session mechanics. |
| `L5-D-01-004` | Identity Provisioning, Recovery & Lifecycle Architecture | Owns account provisioning/recovery/disablement synchronization mechanics. | D01-002 provides stable principal identity/binding records on which lifecycle synchronization operates. |
| `L5-D-02-001`–`L5-D-02-004` | Authorization baseline, permission model, scope/context and policy enforcement | Own permission/capability representation and authorization decision/enforcement. | D01-002 supplies principal identity only; D02 determines what that principal may do. |
| `L5-A-04-001` / `L5-A-04-002` | Tenant/Institution isolation and context propagation | Own isolation/context propagation. | Every principal and principal-context handoff must preserve Tenant/Institution isolation. |
| `L5-D-03-001` / `L5-D-03-004` | Security/trust baseline; secrets/certificates/machine credential architecture | Own trust controls and machine credential/secret protection. | Non-human identity is separate from the secret or certificate used to authenticate it. |
| `L5-D-04-001` / `L5-D-04-002` | Audit/evidence baseline and provenance model | Own attributable evidence/provenance representation. | D01-002 supplies human/non-human principal attribution and actor-chain context without owning evidence storage. |
| `L5-B-02-001` / `L5-B-02-002` / `L5-B-02-003` | Persistence baseline, ownership/isolation, transaction/consistency/concurrency | Own physical persistence and consistency mechanics. | D01-002 requires durable unique principal bindings and Tenant-safe reference integrity without selecting a database. |
| `L5-C-01-001` / `L5-C-01-002` | API/contracts baseline and synchronous contract architecture | Own synchronous contract mechanics. | Provides governed principal-context interfaces where synchronous interaction is selected. |
| `L5-C-02-001` / `L5-C-02-002` | Event/messaging baseline and durable messaging | Own asynchronous transport and delivery. | Propagates principal/lifecycle context where a governed asynchronous boundary is selected. |
| `L5-A-08-*` | Background Processing & Job Execution | Own job execution/scheduling mechanics. | Jobs consume D01-002 job/workload principal identity without D01-002 becoming job scheduler. |
| `L5-C-03-*` | External Integration & Connector Architecture | Own connector/provider integration mechanics. | Integrations consume D01-002 integration/service identity while D01-002 does not own connector behavior. |

### Must Not Own

- Person, Identity or User Account business lifecycle truth.
- A business Persona, organizational responsibility or institutional relationship.
- Role, Role Assignment, Permission, capability, policy decision, delegation or SoD.
- Authentication protocol, Credential secret, MFA, session, refresh token or Authentication Provider internals.
- Credential/secret cryptography, key management or certificate lifecycle.
- Job scheduling, connector behavior or business-domain workflow execution.
- Audit/evidence storage or business-record ownership.
- Physical persistence technology, API/message transport technology or runtime topology.
- L6 administration screens, forms, navigation or user journeys.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| Principal Classification Boundary | Control Point | Classifies the technical subject as a human account principal or one of the frozen non-human categories: service, workload, job or integration identity. The classification determines which authoritative reference and credential path may be used. | Tenant context; request/execution origin; available authoritative identity/account or technical-owner reference. | D01-002 principal type/classification and immutable technical principal reference. | Routed human binding or non-human registration/binding path. | A04; D01-001; B02 | Ambiguous or conflicting principal type prevents successful principal-context construction. | Frozen D01-002 responsibility; C-004/C-005. |
| Human Account Principal Binding | Adapter / Identity Binding | Binds a D01 technical principal reference to authoritative Tenant, Person, Identity and User Account references where login capability is required, while preserving upstream ownership and source-governed eligibility. | One Tenant; canonical Person/Identity references; User Account reference when applicable; source eligibility/lifecycle reference. | D01-owned technical binding/reference metadata only; no duplicate Person/Identity/User Account business state. | Human principal context to D01-003, D02 or other governed consumers. | A02 source; D01-004; A04; B02 | Missing, cross-Tenant, stale or contradictory references shall not be repaired by creating local canonical identity truth. | L4-A-004-BH-011; L4-A-007-BH-001/BH-002; C-001/C-002/C-006. |
| Non-Human Principal Registry | Technical Identity Store / Boundary | Provides the canonical L5 technical identity record for service, workload, job and integration principals. Each record is Tenant-bound and associated with the technical capability/execution owner that is accountable for its use. | Tenant context; non-human principal category; governed technical owner/association reference; provisioning/lifecycle basis when available. | D01-002-owned non-human principal identity and binding metadata; no Person/User Account business record. | Stable non-human principal reference for D01-003 authentication and D02 authorization. | A01/A04; B02; D03; D01-004 where lifecycle coordination applies | Orphaned owner, unresolved Tenant, duplicate/ambiguous identity or unsupported category prevents normal activation/use. Lifecycle state and transition semantics are governed by `ADR-L5-0048`. | Frozen D01-002 boundary; L2-A01/A03 non-human identity rules. |
| Credential Association Boundary | Contract Boundary | Associates a human or non-human principal reference with D01-003-owned authentication credential/mechanism references without owning or exposing credential secret material. | D01-002 principal reference; D01-003 credential/authentication-mechanism reference; Tenant context. | Binding reference only; secret material remains outside D01-002. | Principal-to-authentication handoff. | D01-003; D03; B02 | Missing/mismatched Tenant or wrong principal class prevents the credential association from being used for authentication. | C-005/C-007; L4-A-008-BH-004/BH-008. |
| Execution Principal Context Builder | Control Point / Context Builder | Constructs the principal identity context used by downstream execution while preserving the distinction between original human actor and current calling/executing non-human principal when both participate. | Tenant context; authenticated human principal when present; calling service/workload/job/integration principal when present; correlation reference. | Context only; no business lifecycle or authorization decision. | `ExecutionPrincipalContext` to D02, A08, C03, D04 and governed technical consumers. | D01-003; A04; D04; C01/C02 | The builder shall not replace the originating human identity with a service identity or collapse distinct actors into one principal. Missing required attribution prevents claims of attributable execution. | Frozen D01-002 responsibility; L2 platform/non-human identity rules; D04 collaboration. |
| Principal Eligibility Reference Boundary | Adapter / Control Point | Carries source-governed account/identity eligibility into authentication and lifecycle processing without inventing status mappings. | Human upstream eligibility/account status reference or governed non-human principal availability/lifecycle basis. | Reference to source fact/decision; no duplicated business state. | Eligibility signal/context to D01-003/D01-004. | A02; D01-004; C01/C02 | Unknown or contradictory eligibility is treated as non-eligible for success paths that require positive eligibility; D01-002 shall not infer `Active`/`Enabled` equivalence. | L4-A-007-BH-002; L4-A-008-BH-001; C-006. |
| Principal Evidence / Attribution Handoff | Contract Boundary | Supplies non-secret principal identity and actor-chain categories required for attributable evidence/provenance while keeping immutable evidence storage D04-owned. | Tenant; human principal ref if present; non-human principal ref if present; correlation; technical action context. | No credential secret and no authoritative audit store. | Attribution context to `L5-D-04-*`. | D04; C01/C02; D03 | Missing attribution or cross-Tenant mismatch shall be surfaced; D01-002 shall not silently substitute a generic system actor when a specific principal should be known. | L4-A-007-BH-009; D04 responsibility; C-001. |

The elements above are logical architecture boundaries. They do not imply separate processes, services, containers or vendors.

### 5.2 Primary Technical Flows

#### A. Human Account Principal Resolution

```text
IAM / authentication request
        ↓
Resolved Tenant context
        ↓
Canonical Person + Identity reference available?
        ├─ No → no human principal success
        ↓ Yes
User Account required for this authentication path?
        ├─ Yes → resolve canonical User Account reference + source eligibility
        └─ No  → preserve upstream boundary; do not fabricate account
        ↓
Human Account Principal Binding
        ↓
D01 technical principal reference
        ↓
Credential Association Boundary → D01-003
        ↓
Execution/authenticated principal context
        ↓
D02 performs later authorization
```

A human principal binding shall not copy authoritative Person/Identity attributes merely to make the technical principal self-contained.

#### B. Non-Human Service / Workload / Job / Integration Principal Resolution

```text
Technical execution requirement
        ↓
Resolved Tenant context
        ↓
Principal Classification Boundary
        ↓
service / workload / job / integration
        ↓
Governed technical owner/association reference available?
        ├─ No → reject/orphaned principal
        ↓ Yes
Non-Human Principal Registry
        ↓
Stable Tenant-bound technical principal reference
        ↓
Credential Association Boundary → D01-003/D03
        ↓
Execution Principal Context Builder
        ↓
D02 authorization / A08 job execution / C03 integration as applicable
```

No Person or human User Account is created merely to make a non-human execution identity fit the human identity model.

#### C. Human-Initiated Operation Executed by a Technical Principal

```text
Authenticated human principal
        +
calling service/workload/job/integration principal
        +
resolved Tenant + correlation
        ↓
Execution Principal Context Builder
        ↓
Preserve separately:
  original human principal reference
  current executing technical principal reference
        ↓
Downstream execution
        ↓
D02 evaluates authorization for the applicable principal/context
        ↓
D04 receives attributable actor-chain evidence context
```

The presence of a non-human execution principal shall not erase or replace a known originating human identity.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Human canonical references unresolved or cross-Tenant | Human Account Principal Binding | Prevent valid human principal construction and authentication handoff. | Creating a local Person/Identity/User Account substitute or guessing Tenant. | Upstream A02/A04 resolution; D01-004 where lifecycle synchronization is involved. | L4-A-004-BH-011; L4-A-007-BH-001/BH-002. |
| User Account source eligibility does not permit authentication | Principal Eligibility Reference Boundary | Mark the authentication path ineligible; no successful D01-003 authentication may be produced from that account context. | Reinterpreting source status or mapping unsupported states to `Active`/`Enabled`. | Upstream A02; D01-004. | L4-A-007-BH-002. |
| Non-human principal lacks Tenant or technical owner/association reference | Non-Human Principal Registry | Prevent normal registration/use and surface orphaned-principal failure. | Global unscoped service identity or anonymous shared machine principal. | A01/A04 governance; lifecycle/provisioning resolution under `ADR-L5-0048`. | L2-A01/A03 non-human identity rules; C-001/C-004. |
| Human credential path used for system integration or system credential path used as human-login identity | Principal Classification / Credential Association Boundary | Reject the mismatched association/use. | Treating system API credentials as human credentials or a human User Account as the implicit service identity. | D01-003/D03 detailed credential architecture. | L2-A03 non-human identity rules; C-005/C-007. |
| Principal binding references stale or contradictory upstream identity/account truth | Human Binding / Eligibility Boundary | Stop the affected success path and require source re-resolution/synchronization. | Silently keeping a stale local canonical copy as authoritative. | D01-004 + upstream A02. | C-002/C-006. |
| Original human actor lost during service/job/integration execution | Execution Principal Context Builder | Mark attribution context incomplete and prevent any claim that the technical principal is the human actor. | Replacing original user identity with service identity or fabricating an initiator. | D04 provenance; A08/C03 execution owner. | D01-002 attribution responsibility; D04 collaboration. |
| Principal-to-credential association Tenant/class mismatch | Credential Association Boundary | Reject association/use and emit non-secret failure evidence where required. | Cross-Tenant credential reuse or class-confused credential binding. | D01-003/D03/A04. | C-001/C-005/C-007. |

### 5.4 Conceptual Contracts / Context

The following are conceptual information categories only. Exact transport schemas belong to C01/C02 and concrete authentication artifacts belong to D01-003.

```text
HumanAccountPrincipalBinding
{
  resolved Tenant reference
  canonical Person reference
  canonical Identity reference
  canonical User Account reference when required
  D01 technical principal reference
  source-governed eligibility/lifecycle reference
  correlation/reference metadata where required
}
```

This binding contains references necessary for technical execution. It is not a duplicate Person, Identity or User Account record.

```text
NonHumanPrincipal
{
  resolved Tenant reference
  D01 technical principal reference
  principal category: service | workload | job | integration
  governed technical owner/association reference
  lifecycle/provisioning basis reference where available
  authentication-mechanism binding reference
}
```

`authentication-mechanism binding reference` points to D01-003/D03-owned material and shall not contain credential, secret or private-key values.

```text
ExecutionPrincipalContext
{
  resolved Tenant reference
  effective executing principal reference
  original human principal reference when present
  calling non-human principal reference when present
  correlation/evidence reference
}
```

`ExecutionPrincipalContext` carries identity and attribution only. It contains no Role, Permission, delegated authority or Authorization Decision.

### 5.5 State / Persistence / Consistency

- D01-002 may persist its own **technical principal identity**, classification and binding metadata, subject to B02 persistence/isolation architecture.
- For human principals, authoritative Person, Identity and User Account business state remains upstream A02-owned. D01-002 stores references and technical binding metadata only.
- For non-human principals, D01-002 is the canonical L5 technical owner of the service/workload/job/integration principal identity model within the frozen corpus boundary.
- The exact lifecycle states for non-human principals are governed by `ADR-L5-0048`: `Provisioned → Active ↔ Suspended → Retired`, with `Retired` terminal. This specification owns the technical-principal identity/state representation while D01-004 coordinates transition execution and D01-003/D03 retain authentication/credential mechanics.
- Principal references shall be unique within their governed Tenant scope and shall not be silently rebound across Tenants.
- A human principal binding shall not be reassigned to a different canonical Person/Identity merely as a technical shortcut; upstream identity reconciliation remains upstream-owned.
- Credential secret state, Authentication Session state and provider/factor state are not stored as D01-002 principal state; only governed association references may be retained.
- Physical indexing, foreign-key strategy, transaction isolation, locking and repository technology are B02-owned and are not selected here.
- Updates that change a principal's Tenant binding or canonical subject binding are architecture-sensitive and shall not be implemented as ordinary mutable metadata without the applicable D01-004/upstream governance path.

### 5.6 Asynchronous / Messaging Behavior

- Principal identity context may cross asynchronous boundaries only through C02-governed messaging while preserving Tenant and principal references required for execution and attribution.
- A job or integration message carrying a non-human principal reference does not grant permission; D02 authorization remains separately applicable where the protected action requires it.
- Canonical upstream Person/Identity/User Account events retain their source-defined semantic publisher. D01-002 consumption does not create replacement identity lifecycle events.
- D01-002 shall not invent canonical non-human identity lifecycle event names without governed upstream/L5 authority.
- Exact retry, ordering, deduplication, timeout and delivery semantics are delegated to C02/A08 and are not invented here.

### 5.7 Security / Trust / Isolation

- Human and non-human principals are Tenant-bound and shall never be resolved through cross-Tenant fallback.
- Non-human identity shall not be implemented as a shared global credential merely because the same software capability exists in multiple Tenants.
- D01-002 principal records shall not contain password, API secret, token secret, private key or MFA secret values.
- Credential/machine-secret protection belongs to D01-003/D03; D01-002 retains only the association necessary to identify which principal is being authenticated.
- Principal type/classification is security-relevant. Implementations shall prevent type confusion that would allow a system credential to impersonate a human account or vice versa.
- A service/workload/job/integration identity shall not acquire human business attributes merely to pass downstream authorization. D02 must evaluate permissions/scopes against the actual principal/context architecture.
- When a human request causes downstream execution by a technical principal, the architecture shall preserve both identities when material for attribution instead of collapsing them into one actor.

### 5.8 Runtime / Deployment / Operations

D01-002 defines logical identity boundaries only. The Principal Classification Boundary, Human Account Principal Binding, Non-Human Principal Registry, Credential Association Boundary and Execution Principal Context Builder may be co-located or separately realized only under A01/E04 architecture and applicable ADR governance.

This specification does not mandate a central directory server, dedicated identity microservice, sidecar, cloud workload-identity service, service mesh, certificate authority or any other product/topology.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| AD-001 | Separate human account principals from non-human technical principals | Human execution binds to canonical Person/Identity/User Account references; service/workload/job/integration identities use explicit non-human D01 technical principal records. | Frozen D01-002 responsibility; L2-A01/A03 non-human identity rules; L2-A02 ownership | Prevents fabricated Person records, shared ambiguous service users and human/system credential confusion. | None — directly within canonical D01-002 responsibility. |
| AD-002 | Tenant-bound non-human identity | Every non-human principal is bound to exactly one resolved Tenant for execution context; no global cross-Tenant service principal is created by this architecture. | L2-A01 Tenant isolation/awareness; L2-A03 non-human identity rules | Preserves the platform isolation invariant and prevents identity reuse from collapsing Tenant boundaries. | None — direct authoritative constraint. |
| AD-003 | Principal identity is separate from credential/authentication material | D01-002 owns principal identity/binding; D01-003/D03 own credential/authentication and secret protection. | Frozen D01 split; L2-A02 INT-301; L2-A03 canonical ownership | Allows credentials to change/recover without changing the identity of the principal and prevents secret material from becoming identity master data. | None — canonical ownership realization. |
| AD-004 | Preserve human initiator and technical executor separately | Where an operation has both a known human initiator and a service/workload/job/integration executor, execution context preserves both references rather than substituting one for the other. | D01-002 attribution responsibility; L4-A-007-BH-009; D04 provenance collaboration | Enables attributable execution and prevents service identity from erasing human initiation context. | None — local technical derivation; subject to D04 consistency review. |
| AD-005 | D01-002 carries identity, not authorization | Principal contexts emitted by D01-002 contain no Role, Permission, delegation or Authorization Decision semantics. | L4-A-007-BH-008; ADR-PLAT-AUTH-001 §3.2; frozen D02 responsibility | Maintains the authentication/authorization boundary and prevents account/service identity from becoming an access grant. | None — direct upstream/corpus constraint. |

No technology selection is made by these decisions.

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant isolation | Every human/non-human principal and binding is Tenant-scoped; context propagation shall preserve that boundary. | L2-A01; L4-A-007-BH-001 | `L5-A-04-*`; D01-002 |
| Human/non-human separation | A service/workload/job/integration identity cannot masquerade as a human User Account merely to reuse login architecture. | L2-A03 non-human identity rules; AD-001 | D01-002; `L5-D-01-003` |
| Secret minimization | Principal records carry credential references, never credential secret values. | L4-A-008-BH-008; AD-003 | `L5-D-01-003`; `L5-D-03-*` |
| Attribution | When both human initiator and technical executor exist, both references remain distinguishable for provenance. | L4-A-007-BH-009; AD-004 | D01-002; `L5-D-04-*` |
| Authorization separation | Principal identity alone never establishes permission. | L4-A-007-BH-008; ADR-PLAT-AUTH-001 | `L5-D-02-*` |
| Lifecycle integrity | Human account eligibility remains source-governed; non-human principal lifecycle is governed by ADR-L5-0048. | L2-A02; L4-A-007-BH-002; ADR-L5-0048 | `L5-D-01-004`; upstream A02 / D01 governance |
| Reference integrity | Stale, orphaned or cross-Tenant principal bindings fail closed rather than being silently repaired locally. | Canonical ownership + Tenant constraints | D01-002; B02; A04 |

No numeric credential lifetime, inactivity period, session duration, identity-retention period, rotation interval, retry count, timeout or capacity target is introduced here.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| CR-001 | Every D01-002 principal used for authentication or protected execution shall resolve to exactly one Tenant before a success path proceeds. | AD-002; §5.2 | L2-A01 `P4`/`AP-06`; `L4-A-007-BH-001` |
| CR-002 | A human principal implementation shall reference authoritative Person/Identity/User Account records and shall not create a competing canonical Person/Identity/User Account store. | AD-001; §5.1 Human Account Principal Binding | `L4-A-004-BH-011`; L2-A02 INT-301 |
| CR-003 | Non-human service/workload/job/integration principals shall be explicitly represented as non-human technical identities and shall not be modeled as fabricated Persons or implicit human User Accounts. | AD-001; §5.1 Non-Human Principal Registry | Frozen `L5-D-01-002` boundary; L2-A01/A03 non-human identity rules |
| CR-004 | A non-human principal shall not be usable for normal execution unless its Tenant and governed technical owner/association context can be resolved. | §5.1; §5.3 | C-001/C-004; L2-A01/A03 |
| CR-005 | API/system-integration credential associations shall not be accepted as human-login credential associations, and human login credentials shall not implicitly define service identity. | §5.3; AD-003 | L2-A03 non-human identity rules; C-005 |
| CR-006 | D01-002 persistence shall not contain credential secret, token secret, private-key or MFA-secret values; secret/authentication material remains D01-003/D03-owned. | AD-003; §5.5/§5.7 | Frozen D01 split; L4-A-008-BH-008 |
| CR-007 | User Account/source eligibility that does not positively permit authentication shall prevent the human account principal from being used to produce a successful authentication path; unsupported status mappings shall not be inferred. | §5.1 Principal Eligibility Boundary | `L4-A-007-BH-002` |
| CR-008 | Principal contexts emitted by D01-002 shall contain no Role, Permission, Delegation or Authorization Decision semantics and shall require D02 evaluation where authorization is applicable. | AD-005; §5.4 | `L4-A-007-BH-008`; ADR-PLAT-AUTH-001 §3.2 |
| CR-009 | When both a human initiator and a technical executing principal are known, the execution context shall preserve them as distinct references rather than replacing one with the other. | AD-004; §5.2C | `L4-A-007-BH-009`; D04 provenance collaboration |
| CR-010 | A principal binding with cross-Tenant, unresolved, orphaned or contradictory references shall fail the affected success path and shall not be silently repaired by local identity creation or rebinding. | §5.3 | C-001/C-002/C-006 |
| CR-011 | D01-002 shall not publish newly invented canonical business lifecycle events for User Account or non-human principal state. | §5.6 | L5 zero-guessing rule; upstream canonical event ownership |
| CR-012 | Implementations shall not infer a named directory, IdP, workload-identity product, credential technology, database or deployment topology from this specification. | §5.8 | L5 Technology Selection & ADR Methodology v1.0.1 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| OD-001 | Material human-principal/account L4 authority was previously Draft. | **Resolved** | R3A/R3B approval decisions; R3D traceability validation | L4/repository governance | **No** | Closed: applicable L4 parents are Architecture Approved and current-version relations are validated. |
| OD-002 | D01-001 and material A04/B02/D03/D04/C01/C02 peers are now canonical repository dependencies; later material changes still require impact review. | Dependency | Current canonical repository state | Lead Architect + respective L5 owners | **No standalone blocker after this reconciliation** | Maintain impact review; OD-001 L4 authority blocker is resolved and later material upstream changes require governed revalidation. |
| OD-003 | Non-human principal lifecycle/provisioning authority was previously unresolved. | Resolved L5-owned architecture decision | `ADR-L5-0048` | D01 / Lead Architect | **No** | Closed by the canonical D01 technical-principal lifecycle and event/ownership model. |
| OD-004 | **Partially resolved:** Keycloak 26.x is canonical and non-human service/workload identity uses the governed `ADR-L5-0039` profile. Exact federation/provider-specific details remain D01/D03/profile-governed where not otherwise selected. | ADR / Technology | `ADR-L5-0024`; `ADR-L5-0039` | Applicable canonical owners | Product/profile resolved; remaining provider details governed | Preserve D01/D02 ownership. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-004-BH-011` — canonical Person/Identity reference consumption without ownership transfer.
- `L4-A-007-BH-001`, `BH-002` — Tenant/Identity/account context and account eligibility.
- `L4-A-007-BH-008`, `BH-009` — authenticated identity context remains separate from authorization and attributable evidence is required.
- `L4-A-008-BH-001`, `BH-004`, `BH-008` — recovery resolves User Account context while preserving identity/account lifecycle ownership and secret-safe evidence.
- `L4-A-053-BH-001`, `BH-002`, `BH-004` — closure resolves stable identity/account references, disables login capability and preserves historical identity ownership.

### Material Higher-Level / ADR Constraints

- `L2-A01` — Tenant isolation/context-driven execution and explicit Tenant-aware non-human identities.
- `L2-A02` INT-301 + 2026-08-20 sequencing — Person/Identity/User Account ownership and post-activation User Account creation boundary.
- `L2-A03` — authentication ownership; non-human service/machine identity and system-credential separation.
- `ADR-PLAT-AUTH-001` — Tenant-scoped Person and User Account separation from Role Assignment/authorization ownership.

### Material L5 Dependencies

- working `L5-D-01-001` — IAM family baseline.
- `L5-D-01-003` — credential/session/authentication mechanisms.
- `L5-D-01-004` — provisioning/recovery/disablement synchronization.
- `L5-D-02-*` — authorization.
- `L5-A-04-*` — Tenant/Institution isolation/context.
- `L5-D-03-*` — trust/secrets/machine credentials.
- `L5-D-04-*` — audit/provenance.
- `L5-B-02-*` — persistence/consistency.
- `L5-C-01-*` / `L5-C-02-*` — technical contracts/transport.
- `L5-A-08-*` — job execution.
- `L5-C-03-*` — external integrations.

### Architecture Decisions

- `AD-001`–`AD-005`

### Conformance Requirements

- `CR-001`–`CR-012`

### Open / Blocking Items

- `OD-001` — **resolved by R3A/R3B approval + R3D validation**.
- `OD-002` — working D01 baseline and peer-L5 stabilization gate.
- `OD-003` — **resolved by ADR-L5-0048**.
- `OD-004` — material technology/ADR selections intentionally deferred.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0039`  
Every protected service/workload/job/integration execution uses an explicit non-human principal. Shared ambient API keys and implicit internal-network trust are not the default. Human initiator and technical executor are preserved separately.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Non-human lifecycle/provisioning semantics are now governed by `ADR-L5-0048`; unrelated upstream human identity/account semantics remain unchanged.


## 2026-09-23 R3C Non-Human Principal Lifecycle Synchronization

**Applicable locked decision:** `ADR-L5-0048`

The Non-Human Principal Registry owns the canonical technical lifecycle:

```text
Provisioned → Active ↔ Suspended → Retired
```

with `Retired` terminal.

The registry also owns the technical lifecycle facts `TechnicalPrincipalProvisioned`, `TechnicalPrincipalActivated`, `TechnicalPrincipalSuspended`, `TechnicalPrincipalResumed`, and `TechnicalPrincipalRetired`.

This lifecycle does not create Person/User Account records and does not grant authority. D02 remains authorization owner; D01-003/D03 remain authentication/credential/secret owners; D01-004 coordinates lifecycle execution and reconciliation.


## 2026-09-23 R3D Targeted Revalidation

**Review result:** PASS — Architecture Approved.

The R3D review revalidated this specification against the current approved R3A/R3B L4 parent versions, the Locked R3B upstream amendments, applicable Locked/Approved ADRs including `ADR-L5-0048`, and the D01 peer set.

No applicable HF-01…HF-15 failure remains. Exact L4 BH→L5 relations are recorded in the canonical Section-A traceability partition under the R3D validation decision.

This approval does not authorize Final L5 Corpus Lock and does not resolve unrelated intentionally deferred implementation/profile choices.
