# IAM Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-D-01-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-01 — IAM Architecture  
**Canonical Technical Responsibility:** Technical identity/authentication/account architecture; canonical Person/business relationships remain upstream-owned.  
**Technical Architecture Owner:** L5-D-01 — IAM Architecture  
**Assigned Author:** Devang  
**Version:** `1.3.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L4-A-004` v1.0.1 Architecture Approved; `L4-A-007` v1.0.1 Architecture Approved; `L4-A-008` v1.0.1 Architecture Approved; `L4-A-009` v1.0.1 Architecture Approved; `L4-A-053` v1.0.1 Architecture Approved; `L4-A-055` v1.0.1 Architecture Approved; `L2-A02` Locked; `L2-A03` Locked; `ADR-PLAT-AUTH-001` Approved.  
**Referenced By:** Planned canonical dependents `L5-D-01-002`, `L5-D-01-003`, `L5-D-01-004`, and `L5-D-02-001`.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001`; `ADR-L5-0024`; `ADR-L5-0038`; `ADR-L5-0039`; `ADR-L5-0048`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution-wide IAM architecture baseline that connects authoritative Tenant, Person, Identity and User Account context to governed authentication execution and produces an authenticated technical context for later authorization without transferring upstream ownership or granting permission.

Within SutraERP L5, **IAM Architecture** in `L5-D-01` covers technical identity/account integration and authentication architecture. Authorization policy, RBAC, Permission resolution, delegated authority, privileged access and separation-of-duties enforcement are owned by `L5-D-02` and are not part of this baseline.

The baseline establishes common technical boundaries and handoffs that the detailed `L5-D-01-002`, `L5-D-01-003` and `L5-D-01-004` specifications shall refine without contradicting.

### In Scope

- The family-level logical IAM composition and control boundaries.
- Intake and use of authoritative Tenant, Person, Identity and User Account references without duplicating canonical ownership.
- Authentication-entry gating before credential, provider, MFA or session processing.
- The technical boundary between identity/account context and Authentication-owned credentials, factors, providers and sessions.
- The authenticated-context handoff from IAM to Authorization without permission semantics.
- Provider-independent authentication integration boundaries required by governed provider onboarding and selection behavior.
- Lifecycle synchronization boundaries that consume authoritative identity/account closure or eligibility facts and apply authentication-side consequences without changing Person/Identity business state.
- Shared security, Tenant-isolation, evidence and failure invariants that all `L5-D-01` detailed specifications shall preserve.

### Out of Scope / Non-Goals

- Canonical Person, Identity or User Account business lifecycle ownership, states, transitions or business events — upstream `L2-A02` / applicable L4 behavior.
- Detailed user-account, service/workload/job/integration identity technical models — `L5-D-01-002`.
- Detailed credential, MFA, authentication-provider, Authentication Session or authentication-protocol architecture — `L5-D-01-003`.
- Detailed account provisioning, recovery, disablement and lifecycle synchronization mechanics — `L5-D-01-004`.
- Authorization, RBAC, Permission/capability evaluation, scope evaluation, delegation, privileged access, maker-checker or SoD — `L5-D-02-*`.
- Cryptographic primitives, key management, secrets/certificate architecture or cross-solution trust segmentation — `L5-D-03-*`.
- Audit-evidence persistence, provenance model or tamper-evident storage — `L5-D-04-*`.
- Physical database technology, transaction engine or persistence topology — `L5-B-02-*`.
- API transport, event-broker technology or message-delivery implementation — `L5-C-01-*` / `L5-C-02-*`.
- Process/container/server/cloud topology — `L5-E-04-*`.
- L6 screens, navigation, login UI, recovery UI or other interaction design.
- Selection of an IAM vendor, identity provider product, federation protocol, authentication framework, token format, credential algorithm, session store, database, broker or cloud service.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-D-01-001` row | This specification owns the technical identity/authentication/account architecture baseline while canonical Person/business relationships remain upstream-owned. | Establishes the canonical responsibility and ownership ceiling for this file. |
| L4 | `L4-A-004_Person_Establishment_to_Active_Canonical_Identity.md` | v1.0.1 / Architecture Approved | `L4-A-004-BH-011`; completion boundary in §8 | Active canonical Person/Identity references become available for governed downstream use without ownership transfer; User Account/authentication are outside A004 completion. | IAM consumes canonical identity references through a governed boundary and shall not recreate or own Person/Identity business truth. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `BH-001`, `BH-002` | Every authentication attempt is bound to exactly one Tenant and canonical Identity, and account eligibility gates authentication. | Requires a Tenant/subject/account eligibility gate before credential or session success. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `BH-003`–`BH-005` | Authentication Policy, method/provider, Credential and required MFA conditions determine whether authentication may succeed. | Requires a dedicated Authentication-owned execution boundary that composes policy, provider/method, credential and MFA checks without becoming Authorization. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `BH-006` | Successful authentication produces the source-defined authentication-success outcome and canonical success fact. | Requires the D01 authentication-control boundary to expose a successful authentication outcome only after all required checks succeed; canonical event transport/delivery mechanics remain `L5-C-02-*` owned. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `BH-007`, `BH-008` | Successful authentication establishes an `Active` Authentication Session and makes authenticated context available to Authorization without granting permission. | Requires an authenticated-context boundary that is technically distinct from Authorization Decision/Permission evaluation. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `BH-009` | Authentication evidence must preserve attributable Tenant/Identity, policy, method/provider, credential/MFA outcomes, session establishment and terminal result. | Requires a non-secret IAM evidence handoff to the canonical audit architecture. |
| L4 | `L4-A-008_Lost_or_Invalid_Credential_to_Restored_Authentication_Capability.md` | v1.0.1 / Architecture Approved | `BH-001`, `BH-003`–`BH-008` | Recovery requires resolvable Tenant/Identity/User Account context; failed recovery verification or replacement-Credential validation cannot restore authentication; covered previous Credentials must cease authenticating; policy may require session consequences; evidence must not expose Credential values. | Establishes the family-level recovery/lifecycle failure and handoff invariants while detailed recovery mechanics remain `L5-D-01-004` owned and Credential/session mechanisms remain `L5-D-01-003` owned. |
| L4 | `L4-A-009_Multi_Factor_Enrollment_to_Active_Verified_Authentication_Factor.md` | v1.0.1 / Architecture Approved | `BH-001`, `BH-004`, `BH-008`, `BH-009` | MFA enrollment requires an active authenticated Tenant/Identity context; unverified factors cannot authenticate; only active verified factors may be used; evidence must not expose factor secrets. | Requires the baseline to preserve session-bound enrollment context and secret-isolation rules while delegating factor mechanics to `L5-D-01-003`. |
| L4 | `L4-A-053_Person_Institutional_Exit_to_Governed_Identity_Closure.md` | v1.0.1 / Architecture Approved | `BH-002`, `BH-004`, `BH-009` | Governed closure disables login capability, requires termination of active Authentication sessions, prevents later authentication after archival, and must not falsely claim completion while session termination is incomplete. | Requires an authoritative lifecycle-to-authentication consequence handoff without IAM mutating Person/Identity lifecycle state. |
| L4 | `L4-A-055_Institutional_Authentication_Provider_Onboarding_to_Governed_Provider_Availability.md` | v1.0.1 / Architecture Approved | `BH-002`, `BH-005`, `BH-006`, `BH-009`, `BH-010` | Providers conform to a uniform boundary, configuration/method compatibility is validated, selection is policy-governed and deterministic, and invalid/unavailable providers cannot enter normal selection. | Requires a provider-independent IAM boundary; vendor/protocol details remain implementation/ADR-governed. |
| L2 | `L2-A02_Person_and_Identity_Workflows.md` | v1.0 / Locked | `Canonical Ownership Amendment — INT-301`; `Canonical Person–Identity Establishment and Activation Sequencing Decision — 2026-08-20`; `Canonical Ownership` | Person, Identity and User Account are A02-owned; Credential, Authentication Session, MFA and Authentication Provider are A03-owned; authorization objects are A04-owned; User Account creation occurs only after Person/Identity activation where login is required. | Establishes canonical business ownership and prevents IAM technical realization from duplicating Person/Identity or authorization truth. |
| L2 | `L2-A03_Authentication_Workflows.md` | v1.1.0 / Locked | `Scope`; `Out of Scope`; `Architectural Position`; `Canonical Ownership` | Authentication owns Credential, Authentication Session, Refresh Token, Trusted Device, Authentication Provider/Method, MFA Enrollment, Authentication Policy and Authentication Attempt; it consumes A02 identity/account references and supplies authenticated context to A04. | Establishes the Authentication-side business ownership that the D01 technical family realizes without absorbing Authorization. |
| Upstream ADR | `governance/adr/ADR-PLAT-AUTH-001.md` | Approved | §2; §3.1; §3.2 | Authentication remains separate from authorization; canonical Person is Tenant-scoped; User Account is authentication/runtime context and is not the canonical Role Assignment subject. | Constrains authenticated-context design and prevents D01 from encoding authorization assignments into account/session state. |

The cited canonical L4 parents are Architecture Approved under R3A/R3B and have been rebound to their current approved versions for this review. The former Draft-L4 approval blocker is closed.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Person, Identity and User Account canonical business truth remains upstream-owned; IAM technical components consume references rather than recreating that truth. | L2-A02 INT-301; L4-A-004-BH-011; frozen `L5-D-01-001` responsibility | D01 may own technical bindings/adapters but cannot become canonical Person/Identity/business-account owner. |
| C-002 | Authentication success is not authorization. | L4-A-007-BH-008; L2-A03 `Out of Scope`; ADR-PLAT-AUTH-001 §2 | Authenticated context contains no permission grant and must be evaluated by `L5-D-02` before protected operations are authorized. |
| C-003 | Authentication execution is bound to exactly one resolved Tenant and canonical Identity before success. | L4-A-007-BH-001 | Tenant/Identity resolution is a mandatory entry invariant for the authentication path. |
| C-004 | Account eligibility, Authentication Policy, provider/method, Credential and required MFA conditions gate successful session establishment. | L4-A-007-BH-002–005, BH-007 | No active session/authenticated context may be emitted if required checks do not succeed. |
| C-005 | Identity/account closure or archival cannot be bypassed by IAM recovery or session continuation. | L4-A-053-BH-002/BH-004/BH-009; L4-A-008-BH-004 | Lifecycle synchronization may apply authentication consequences but shall not reactivate or alter upstream Person/Identity state. |
| C-006 | Authentication Provider selection remains provider-independent at the architecture boundary and follows source-governed eligibility/priority/fallback behavior. | L4-A-055-BH-002/BH-005/BH-006/BH-009/BH-010 | Provider adapters shall not leak vendor-specific semantics into canonical business architecture or bypass policy. |
| C-007 | Authentication, MFA, recovery and session evidence must be attributable without exposing Credential/factor secret values. | L4-A-007-BH-009; L4-A-008-BH-008; L4-A-009-BH-009 | IAM evidence contracts contain references/classification/outcomes, not secret material. |
| C-008 | Product/protocol/cryptographic/runtime choices require authoritative architecture or applicable ADR governance. | L5 Template v1.0.3; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0 | This baseline remains concrete but technology-neutral and allocates no ADR number. |

### 3.3 Assumptions

No material architecture assumption is promoted to fact in this approved baseline. Applicable L4 authority is Architecture Approved under R3A/R3B and current-bound under R3D; peer-L5 stabilization and intentionally deferred/profile technology details remain explicit in Section 9.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The canonical **family-level IAM technical baseline** for `L5-D-01`.
- The logical boundary between upstream-owned Person/Identity/User Account truth and D01-owned technical identity/account/authentication realization.
- The mandatory Tenant/Identity/account eligibility gate before successful authentication processing.
- The family-level separation between identity/account reference handling, authentication execution, authenticated-context handoff and lifecycle synchronization.
- The authenticated-context boundary delivered to `L5-D-02` without Role/Permission/Authorization Decision semantics.
- The provider-independent authentication integration boundary required by L4-A-055.
- Cross-D01 invariants for secret isolation, evidence handoff, lifecycle consequences and fail-safe rejection of unsupported authentication success.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-D-01-002` | Identity Account & Service Identity Architecture | Owns detailed user-account and service/workload/job/integration identity technical models. D01-001 establishes only the family boundary. | Prevents the baseline from duplicating detailed identity/account technical models. |
| `L5-D-01-003` | Authentication, Session & Credential Architecture | Owns detailed authentication, Credential, factor, provider and Authentication Session mechanisms. | Realizes the detailed execution behind the baseline Authentication Control Boundary. |
| `L5-D-01-004` | Identity Provisioning, Recovery & Lifecycle Architecture | Owns detailed provisioning/recovery/disablement synchronization with authoritative identity/account facts. | Realizes lifecycle-to-access consequences without changing upstream lifecycle ownership. |
| `L5-D-02-001` | Authorization Architecture Baseline | Owns authorization decision/enforcement architecture. | Consumes authenticated context; D01 never returns permission as an authentication result. |
| `L5-A-04-001` / `L5-A-04-002` | Tenant & Institutional Isolation; context propagation/data isolation | Owns technical Tenant/Institution isolation and context propagation. | IAM must preserve one-Tenant execution and authenticated-context isolation without inventing A04 mechanisms. |
| `L5-D-03-001` | Security Architecture & Trust Boundary Baseline | Owns cross-solution trust boundaries and security-control principles. | Constrains IAM trust boundaries, especially external provider and client/service interaction. |
| `L5-D-03-002` / `L5-D-03-004` | Cryptography/data protection; secrets/certificates/machine credentials | Own cryptographic and secret-protection architecture. | D01 requires protected credentials/factors/secrets but does not select primitives or custody mechanisms. |
| `L5-D-04-001` / `L5-D-04-002` | Audit & Evidence baseline; provenance/evidence model | Own audit/evidence capability and technical provenance model. | IAM emits attributable evidence without becoming evidence-storage owner. |
| `L5-B-02-001` / `L5-B-02-002` | Database/persistence baseline and ownership/isolation | Own physical persistence, isolation and repository architecture. | Detailed D01 state must persist without cross-owner direct writes or Tenant leakage. |
| `L5-C-01-001` / `L5-C-01-002` | API/contract baseline and synchronous contract architecture | Own synchronous technical contract mechanics. | Provides governed contract realization for IAM entry/handoff boundaries where synchronous interaction is selected. |
| `L5-C-02-001` / `L5-C-02-002` | Event/messaging baseline and durable messaging | Own event transport/durable delivery mechanics. | Provides governed transport for lifecycle/event/evidence handoffs where asynchronous interaction is selected. |
| `L5-E-04-001` / `L5-E-04-002` | Runtime/deployment baseline and runtime isolation | Own packaging/runtime/deployment realization. | D01 logical control boundaries do not imply a service/process/container topology. |

These peers are consumed by canonical responsibility. This specification does not assume unverified implementation details from those peers.

### Must Not Own

- Canonical Person, Identity, User Account or institutional relationship business ownership.
- Business Role, Role Assignment, Permission, Authorization Policy, Authorization Decision, delegation or SoD semantics.
- Cryptographic algorithms, key custody, secrets architecture or certificate topology.
- Audit/evidence retention, integrity storage or business-record ownership.
- Physical database/persistence technology or cross-domain direct writes.
- API/event transport technology or message-delivery policy.
- Runtime/deployment topology.
- L6 UI/UX design.
- Vendor-specific authentication-provider semantics or unapproved technology standards.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| IAM Entry Boundary | Control Point | Accepts an authentication or IAM-family request only after required execution context can be associated with one Tenant and an appropriate canonical subject/account reference. It routes the request to the D01-owned detailed capability without granting access. | Tenant context; canonical Identity reference; User Account reference where login is required; request correlation. | Owns no canonical Person/Identity/User Account business state. | Governed handoff to identity/account, authentication or lifecycle sub-capability. | A04; D01-002/003/004 | Unresolved Tenant/Identity/account context prevents successful authentication processing. | L4-A-007-BH-001/BH-002; C-001/C-003. |
| Identity & Account Reference Boundary | Adapter / Contract Boundary | Resolves and consumes authoritative Person/Identity/User Account references and eligibility facts without duplicating or directly mutating their canonical lifecycle. | Canonical A02 references and source-governed account/identity eligibility facts. | May maintain only D01-owned technical binding/reference state defined by `L5-D-01-002`; canonical business truth remains upstream. | Subject/account context usable by Authentication Control Boundary. | D01-002; A04; C01/C02; B02 | Missing, stale, contradictory or ineligible canonical context shall not be repaired by creating local Person/Identity truth. | L4-A-004-BH-011; L2-A02 INT-301; C-001. |
| Authentication Control Boundary | Control Point | Coordinates the technical authentication decision path across applicable Authentication Policy, provider/method, Credential and required MFA checks and establishes an authentication result only when source-required conditions succeed. | Tenant-bound subject/account context; applicable Authentication Policy; provider/method availability; Credential/factor verification outcomes. | Detailed attempt/Credential/session/factor/provider state is owned by `L5-D-01-003` and persisted through B02 architecture. | Successful session establishment and authenticated context, or explicit authentication failure/rejection. | D01-003; D03; B02; C01/C02 | Any required failed/unknown check prevents an active-session/authenticated-context success result. | L4-A-007-BH-002–BH-007; C-004. |
| Authentication Provider Boundary | Adapter Boundary | Presents provider-independent authentication capabilities and normalizes provider participation behind the Authentication-owned contract while preserving policy-governed eligibility, priority, availability and fallback semantics. | Tenant Authentication Policy; provider identifier/type; supported method/configuration classifications; provider availability/result. | Provider technical state is detailed by `L5-D-01-003`; secrets remain D03-governed. | Normalized provider capability/result to Authentication Control Boundary. | D01-003; D03; C01; E04 | Invalid/ineligible/unavailable provider cannot enter normal selection; fallback occurs only when upstream policy permits it. | L4-A-055-BH-002/BH-005/BH-006/BH-009/BH-010; C-006. |
| Authenticated Context Boundary | Contract / Control Point | Exposes the minimum authenticated technical context required for downstream authorization while making no authorization decision. | Successful Authentication Session; Tenant; canonical Identity/account reference; authentication outcome/MFA satisfaction context as applicable; correlation/evidence reference. | Owns no Role, Permission, Delegation or Authorization Decision state. | Authenticated context supplied to `L5-D-02-001` and permitted technical consumers. | D01-003; D02-001; A04; C01 | No context is emitted as authenticated unless authentication/session establishment succeeded; consumers may not interpret the context itself as permission. | L4-A-007-BH-007/BH-008; ADR-PLAT-AUTH-001 §2; C-002. |
| IAM Lifecycle Synchronization Boundary | Control Point | Consumes authoritative identity/account lifecycle or closure facts and applies the corresponding D01-owned login/session/credential consequences without modifying upstream Person/Identity lifecycle state. | Tenant; canonical Person/Identity/User Account references; authoritative lifecycle/closure fact; affected authentication context. | Detailed synchronization/recovery/disablement state is owned by `L5-D-01-004`; session/Credential consequences collaborate with `L5-D-01-003`. | Login disablement, session-termination request/result, recovery eligibility or other source-governed D01 consequence. | D01-004; D01-003; A04; C01/C02 | IAM shall not reactivate or invent upstream lifecycle state to resolve a failed recovery or closure. Incomplete required session termination cannot be represented as completed closure. | L4-A-053-BH-002/BH-004/BH-009; L4-A-008-BH-004/BH-007; C-005. |
| IAM Evidence Handoff | Contract Boundary | Emits attributable, non-secret authentication/recovery/lifecycle evidence categories to the canonical audit/evidence architecture. | Tenant/Identity context; actor/request context where available; policy/method/provider classification; verification outcomes; session/result references; correlation. | Does not own immutable evidence storage or business records. | Evidence handoff to `L5-D-04-*` using C01/C02 mechanisms as governed. | D04; C01/C02; D03 | Secret material shall not be exposed; failed/uncertain evidence handoff shall not be silently treated as durable evidence. Final durability/recovery semantics remain peer-owned. | L4-A-007-BH-009; L4-A-008-BH-008; L4-A-009-BH-009; C-007. |

The elements above are **logical architecture boundaries**, not declarations of separate microservices, processes, containers or servers. Runtime realization remains owned by `L5-E-04-*` and technology selection remains governed by L5 ADR methodology.

### 5.2 Primary Technical Flows

#### A. Authentication to Authenticated Context

```text
Authentication request
        ↓
IAM Entry Boundary
        ↓
Resolve exactly one Tenant + canonical Identity/account context
        ↓
Identity/account eligible for authentication?
        ├─ No / Unknown → Reject authentication success
        ↓ Yes
Authentication Control Boundary
        ↓
Evaluate applicable Authentication Policy
        ↓
Select only eligible provider/method under policy
        ↓
Credential verification + required MFA verification
        ├─ Any required condition fails/unknown → No Active Session / no authenticated context
        ↓ All required conditions succeed
L5-D-01-003 establishes source-governed Authentication Session outcome
        ↓
Authenticated Context Boundary
        ↓
Authenticated context available to L5-D-02
        ↓
Authorization remains a separate downstream decision
```

This flow does not define Credential algorithms, MFA method technology, token format, protocol, timeout, retry, session-store technology or provider product.

#### B. Identity / Account Closure to Authentication Consequence

```text
Authoritative upstream identity/account/closure fact
        ↓
IAM Lifecycle Synchronization Boundary
        ↓
Validate Tenant + canonical subject/account reference
        ↓
Determine only the source-governed D01 consequence
        ↓
Disable/prevent login capability where required
        +
request/coordinate active Authentication Session termination where required
        ↓
Preserve non-secret evidence handoff
        ↓
Do not change Person/Identity business lifecycle ownership
```

Recovery or lifecycle synchronization shall not create a path that makes an upstream-ineligible identity/account authenticatable merely to complete an IAM operation.

#### C. Authentication Provider Participation

```text
Tenant Authentication Policy + provider candidates
        ↓
Authentication Provider Boundary
        ↓
Provider contract/configuration/method eligibility validation
        ↓
Policy-governed availability / priority / fallback evaluation
        ↓
Eligible provider selected for Authentication Control Boundary
        ├─ none eligible → authentication cannot succeed through provider selection
        ↓
Normalized provider result returned to authentication execution
```

Provider-specific protocol or vendor semantics shall not become canonical business behavior through this boundary.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Tenant/Identity/account context unresolved or ineligible | IAM Entry / Identity & Account Reference Boundary | Prevent authentication success and active-session/authenticated-context emission. Preserve attributable failure evidence where source requires. | Guessing Tenant, creating local canonical identity truth or continuing as authenticated. | Upstream A02/A04 context resolution; D01-002 detailed handling. | L4-A-007-BH-001/BH-002. |
| Authentication policy/provider/method/Credential/MFA condition fails | Authentication Control Boundary | Produce a non-success authentication result; no active session/authenticated context. | Bypassing a failed required condition, silently downgrading MFA or treating partial verification as success. | D01-003; provider recovery/fallback only where policy permits. | L4-A-007-BH-003–BH-007. |
| Provider invalid, unavailable or policy-ineligible | Authentication Provider Boundary | Exclude provider from normal selection; use fallback only if governed policy permits; otherwise authentication cannot succeed via that provider path. | Selecting an unvalidated/ineligible provider or inventing fallback order. | D01-003 with D03/E04/C01 as applicable. | L4-A-055-BH-005/BH-006/BH-009/BH-010. |
| Recovery verification or replacement-Credential validation fails | Lifecycle / Authentication Boundary | Keep authentication capability unrestored; do not expose replacement Credential as valid. | Reactivating upstream Identity/User Account or leaving covered previous Credential usable contrary to source conditions. | D01-004 + D01-003. | L4-A-008-BH-003–BH-007. |
| Upstream closure requires login/session termination | Lifecycle Synchronization Boundary | Prevent later authentication and coordinate required active-session termination; retain incomplete status if required termination is not confirmed. | Reporting closure complete while required active session remains usable. | D01-004 + D01-003; C01/C02 transport as governed. | L4-A-053-BH-002/BH-004/BH-009. |
| Audit/evidence handoff is unavailable or uncertain | IAM Evidence Handoff | Do not claim durable evidence was preserved; expose the dependency failure to the canonical evidence/recovery architecture. | Silently dropping required evidence or logging secret Credential/factor values. | D04 + C01/C02; exact durability coupling remains `OD-001`. | L4-A-007-BH-009; L4-A-008-BH-008; L4-A-009-BH-009. |

### 5.4 Conceptual Contracts / Context

The following are conceptual information categories only. Exact API/message schemas belong to `L5-C-01-*` / `L5-C-02-*` and the detailed D01 specifications.

```text
IAMSubjectContext
{
  resolved Tenant reference
  canonical Person/Identity reference as applicable
  User Account reference where login capability is required
  source-governed eligibility/lifecycle reference
  request/correlation reference
}
```

`IAMSubjectContext` does not duplicate Person, Identity or User Account canonical attributes. It carries only the references/context necessary for the D01 technical operation.

```text
AuthenticatedContext
{
  resolved Tenant reference
  authenticated canonical Identity/account reference
  Authentication Session reference
  source-governed authentication outcome context
  MFA satisfaction/verification context where applicable
  correlation/evidence reference
}
```

`AuthenticatedContext` shall not contain or imply a Role Assignment, Permission grant, Authorization Decision, delegated authority or business-operation approval.

```text
AuthenticationLifecycleHandoff
{
  resolved Tenant and canonical subject/account references
  authoritative upstream lifecycle/closure/recovery basis reference
  D01-owned authentication consequence/result reference
  session consequence reference where required
  correlation/evidence reference
}
```

The handoff carries the basis needed to apply an authentication-side consequence; it does not authorize D01 to write upstream Person/Identity lifecycle state.

### 5.5 State / Persistence / Consistency

- `L5-D-01-001` owns no canonical business persistence and no physical persistence technology.
- Canonical Person, Identity and User Account business state remains upstream-owned under A02 semantics.
- Detailed D01 technical identity/account binding belongs to `L5-D-01-002`.
- Detailed Authentication Attempt, Credential, MFA, provider and Authentication Session technical state belongs to `L5-D-01-003` within its canonical boundary.
- Detailed lifecycle synchronization/recovery/disablement state belongs to `L5-D-01-004` within its canonical boundary.
- Physical repositories, isolation, transaction and concurrency architecture are delegated to `L5-B-02-*`.
- No D01 component may directly modify another canonical technical owner's internal persistence merely because the data participates in authentication.
- Tenant/Institution isolation of all D01 persistence and context is delegated to `L5-A-04-*` and must remain preserved across every handoff.

### 5.6 Asynchronous / Messaging Behavior

- Canonical upstream business events retain their source-defined publisher and meaning; D01 consumption does not republish them as new canonical identity events.
- Authentication-owned events may be emitted only where the upstream Authentication architecture defines the corresponding lifecycle outcome; this baseline does not rename or create events.
- Lifecycle, provider, session and evidence handoffs may use synchronous or asynchronous transport only through the canonical C01/C02 architecture. This baseline does not select the interaction strategy where the authoritative peer has not yet decided it.
- Exact retry counts, backoff, timeout, queue depth, dead-letter policy and delivery technology are intentionally not invented.

### 5.7 Security / Trust / Isolation

- Authentication success requires a resolved one-Tenant context and shall not be transferable across Tenant boundaries.
- Secret Credential/factor values shall not appear in `AuthenticatedContext`, generic lifecycle handoffs or audit/evidence payloads.
- External or institutional Authentication Provider participation crosses a security/trust boundary whose detailed controls belong to `L5-D-03-*`; provider output must pass through the Authentication Provider Boundary before it can influence authentication success.
- Authenticated context is evidence of successful authentication only. Its integrity and confidentiality shall be protected according to D03/C01/G01 architecture, but it shall never be interpreted as authorization by possession alone.
- Canonical authorization evaluation remains `L5-D-02` responsibility even if IAM and Authorization are co-located at runtime.
- Identity/account closure facts shall prevent authentication-side bypass or reactivation.

### 5.8 Runtime / Deployment / Operations

The IAM Entry Boundary, Identity & Account Reference Boundary, Authentication Control Boundary, Provider Boundary, Authenticated Context Boundary, Lifecycle Synchronization Boundary and Evidence Handoff are logical architecture elements. They may be co-located or separately realized only through `L5-A-01` / `L5-E-04` architecture and applicable ADR governance.

This specification does not mandate a microservice, monolith, sidecar, gateway, container, cloud identity service or on-premises identity product.

---

## 6. Inherited Architecture Constraints & Technology Position

This baseline establishes no local `AD-*` decision. The material positions that govern D01-001 are already authoritative constraints in Section 3.2 and are not locally changeable architecture choices:

- `C-001` — canonical Person/Identity/User Account business truth remains upstream-owned.
- `C-002` — authentication success is not authorization.
- `C-003` — successful authentication requires one resolved Tenant/Identity execution context.
- `C-006` — Authentication Provider participation remains provider-independent and policy-governed.

Section 5 defines the D01-owned technical realization of those constraints. Section 8 makes that realization objectively conformable.

Technology decisions already locked by `ADR-L5-0024`, `ADR-L5-0038`, `ADR-L5-0039` and `ADR-L5-0040` are inherited here; remaining provider-specific, numeric/session and deployment details remain with their canonical owners.

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant isolation | Every D01 request, session and authenticated context must remain bound to the resolved Tenant across internal and cross-boundary handoffs. | L4-A-007-BH-001; L4-A-055-BH-001/BH-006 | `L5-A-04-*`; D01 family |
| Credential / factor secrecy | Secret values remain outside generic context/evidence contracts and require D03-governed protection. | L4-A-008-BH-008; L4-A-009-BH-009 | `L5-D-03-*`; `L5-D-01-003` |
| Authentication vs authorization | Protected operations require D02 evaluation even after valid authentication. | L4-A-007-BH-008; ADR-PLAT-AUTH-001 | `L5-D-02-*` |
| Identity lifecycle integrity | IAM consumes authoritative closure/eligibility facts and cannot repair or reactivate upstream Person/Identity lifecycle state. | L4-A-053; L2-A02 INT-301 | `L5-D-01-004`; upstream A02 |
| Auditability | Authentication/recovery/MFA/lifecycle actions require attributable evidence without secret exposure. | L4-A-007-BH-009; L4-A-008-BH-008; L4-A-009-BH-009 | `L5-D-04-*` |
| Provider availability/fallback | Provider failure may use only source-governed eligible fallback; no implicit fail-open provider path exists. | L4-A-055-BH-006/BH-009/BH-010 | `L5-D-01-003`; `L5-E-01-*` where applicable |

No numeric availability, latency, timeout, session duration, credential lifetime, MFA lifetime, retry or recovery target is introduced by this baseline.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| CR-001 | An implementation shall not produce successful authentication unless the execution is associated with exactly one resolved Tenant and canonical Identity/account context as applicable. | C-003; §5.2A | `L4-A-007-BH-001`, `BH-002` |
| CR-002 | Failure or indeterminate result of any source-required account eligibility, Authentication Policy, provider/method, Credential or MFA condition shall prevent emission of an active authenticated context. | §5.1 Authentication Control Boundary; §5.3 | `L4-A-007-BH-002`–`BH-007` |
| CR-003 | The authenticated context passed from D01 shall contain no Role, Permission, Delegation or Authorization Decision semantics and shall require D02 evaluation for protected access. | C-002; §5.4 | `L4-A-007-BH-008`; ADR-PLAT-AUTH-001 §2 |
| CR-004 | D01 technical components shall consume canonical Person/Identity/User Account references without creating a competing canonical Person/Identity/account business lifecycle store. | C-001; §5.5 | `L4-A-004-BH-011`; L2-A02 INT-301 |
| CR-005 | Provider implementations shall participate through the provider-independent Authentication Provider Boundary and shall not bypass source-governed eligibility, priority, availability or fallback rules. | C-006; §5.2C | `L4-A-055-BH-002/BH-005/BH-006/BH-009/BH-010` |
| CR-006 | A source-governed identity/account closure that disables login or requires session termination shall be reflected through D01 lifecycle synchronization without reactivating or altering upstream Person/Identity business state. | §5.2B; §5.3 | `L4-A-053-BH-002/BH-004/BH-009` |
| CR-007 | Recovery handling shall not expose a replacement Credential as valid when recovery verification/policy validation fails and shall not leave covered previous Credentials usable contrary to the source-governed recovery result. | §5.3 | `L4-A-008-BH-003`–`BH-007` |
| CR-008 | Credential and authentication-factor secret values shall not be emitted in authenticated-context, lifecycle-handoff or generic audit/evidence contracts. | §5.4; §5.7 | `L4-A-008-BH-008`; `L4-A-009-BH-009` |
| CR-009 | IAM evidence production shall preserve attributable Tenant/Identity/account, policy/provider/method, verification/session/result and correlation categories required by applicable L4 behavior while delegating evidence persistence/integrity to D04. | IAM Evidence Handoff; §5.3 | `L4-A-007-BH-009`; `L4-A-008-BH-008`; `L4-A-009-BH-009` |
| CR-010 | No implementation shall infer a service/process/container/cloud topology or select an IAM/provider/protocol/cryptographic product from this baseline; such decisions require the canonical runtime/technology/ADR process. | §5.8; C-008 | L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0 |
| CR-011 | `L5-D-01-002`, `L5-D-01-003` and `L5-D-01-004` shall preserve the Section 3.2 authoritative constraints and Section 4/5 family boundaries and shall not duplicate responsibilities assigned to D02, D03, D04, A04, B02 or C01/C02. | §3.2; §4; §5 | Frozen L5 corpus responsibility boundaries |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| OD-001 | Material D01-001 L4 authority was previously Draft. | **Resolved** | R3A/R3B L4 approval decisions; current-version revalidation | L4/repository governance + D01 review | **No** | Closed: all material cited L4 parents are Architecture Approved and exact D01 relations are validated by R3D. |
| OD-002 | **Partially resolved:** Keycloak 26.x is the Locked IAM/authentication platform. Federation protocol profiles, token/session representation, credential protection and runtime placement remain detailed-owner decisions where not otherwise governed. | L5 / ADR / Technology | `ADR-L5-0024` | Applicable canonical owners | Product resolved | Preserve canonical Person/Identity and D02 authorization boundaries. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-004-BH-011` — canonical identity references without ownership transfer.
- `L4-A-007-BH-001`–`BH-009` — family-level Tenant-bound authentication, policy/provider/Credential/MFA gating, successful-authentication outcome boundary, session/authenticated-context handoff and evidence; detailed mechanisms remain with `L5-D-01-003` and event transport with `L5-C-02-*`.
- `L4-A-008-BH-001`, `BH-003`–`BH-008` — family-level recovery eligibility/failure boundaries, replacement/covered-Credential and session consequences, and non-secret evidence; detailed recovery mechanics remain with `L5-D-01-004` and Credential/session mechanisms with `L5-D-01-003`.
- `L4-A-009-BH-001`, `BH-004`, `BH-008`, `BH-009` — MFA/session context, factor eligibility and secret-safe evidence.
- `L4-A-053-BH-002`, `BH-004`, `BH-009` — login disablement, session termination and closure completion boundary.
- `L4-A-055-BH-002`, `BH-005`, `BH-006`, `BH-009`, `BH-010` — provider-independent contract, policy-governed selection/fallback and failure exclusion.

### Material Higher-Level / ADR Constraints

- `L2-A02` INT-301 and 2026-08-20 sequencing decision — Person/Identity/User Account ownership and post-activation account boundary.
- `L2-A03` canonical ownership — Credential/session/MFA/provider Authentication ownership and Authorization separation.
- `ADR-PLAT-AUTH-001` §§2, 3.1, 3.2 — Tenant-scoped Person alignment and User Account/authentication separation from Role Assignment.

### Material L5 Dependencies

- `L5-D-01-002`, `L5-D-01-003`, `L5-D-01-004` — detailed D01 realization.
- `L5-D-02-001` — downstream Authorization architecture.
- `L5-A-04-*` — Tenant/Institution isolation and context.
- `L5-D-03-*` — trust, cryptography and secrets.
- `L5-D-04-*` — audit/evidence.
- `L5-B-02-*` — persistence.
- `L5-C-01-*` / `L5-C-02-*` — governed technical handoffs.
- `L5-E-04-*` — runtime/deployment realization.

### Local Architecture Decisions

- None in this baseline. Section 3.2 records inherited authoritative constraints; Section 5 records their D01-owned technical realization.

### Conformance Requirements

- `CR-001`–`CR-011`

### Open / Blocking Items

- `OD-001` — **resolved by R3A/R3B approval + R3D validation**.
- `OD-002` — core IAM technology is resolved; remaining provider/profile details stay governed.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0024`  
Keycloak 26.x is canonical for authentication/federation. It does not become the canonical Person/Identity master and does not own authorization policy.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0038`  
The default human login surface is a SutraERP D01-owned username/password API backed by Keycloak credential authority. Passwords are transient only, never persisted by SutraERP, and OAuth Direct Access Grant is not the canonical realization. Browser sessions are server-side/opaque-cookie based with privileged/high-risk MFA.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0039`  
Non-human principals use explicit governed identities and Keycloak/service-account or approved workload authentication; D02 remains authorization authority and internal network location never substitutes for authentication.


## 2026-09-23 R3D Targeted Revalidation

**Review result:** PASS — Architecture Approved.

The R3D review revalidated this specification against the current approved R3A/R3B L4 parent versions, the Locked R3B upstream amendments, applicable Locked/Approved ADRs including `ADR-L5-0048`, and the D01 peer set.

No applicable HF-01…HF-15 failure remains. Exact L4 BH→L5 relations are recorded in the canonical Section-A traceability partition under the R3D validation decision.

This approval does not authorize Final L5 Corpus Lock and does not resolve unrelated intentionally deferred implementation/profile choices.
