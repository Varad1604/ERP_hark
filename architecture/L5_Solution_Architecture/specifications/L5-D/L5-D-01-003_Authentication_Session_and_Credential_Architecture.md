# Authentication, Session & Credential Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-01-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-01 — IAM Architecture  
**Canonical Technical Responsibility:** Authentication/session/credential mechanisms separate from authorization policy.  
**Technical Architecture Owner:** L5-D-01 — IAM Architecture  
**Assigned Author:** Devang  
**Version:** `1.3.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-01-001` v1.3.0 Architecture Approved; `L5-D-01-002` v1.4.0 Architecture Approved; `L4-A-007` v1.0.1 Architecture Approved; `L4-A-008` v1.0.1 Architecture Approved; `L4-A-009` v1.0.1 Architecture Approved; `L4-A-053` v1.0.1 Architecture Approved; `L4-A-055` v1.0.1 Architecture Approved; `L2-A02` Locked; `L2-A03` v1.1.0 Locked; `ADR-PLAT-AUTH-001` Approved.  
**Referenced By:** Planned `L5-D-01-004`, `L5-D-02-*`, `L5-G-01-003`, and technical consumers requiring authenticated principal/session context.  
**Applicable ADRs:** `ADR-L5-0024`; `ADR-L5-0038`; `ADR-L5-0039`; `ADR-L5-0040`; `ADR-L5-0048`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the technical architecture that converts a valid D01 identity/account principal context into a governed authentication outcome and, on success, an Authentication Session and authenticated execution context.

It owns the detailed technical mechanisms for:

- authentication-attempt control;
- Authentication Policy application at the authentication boundary;
- provider/method selection and execution;
- Credential verification;
- MFA challenge integration;
- Authentication Session establishment, continuation and termination mechanics;
- Refresh Token handling as Authentication-owned session-continuation state;
- Credential rotation/recovery consequences at the authentication-mechanism boundary;
- non-secret authentication evidence and downstream authenticated-context handoff.

Authentication proves a principal under governed conditions. It does **not** determine what that principal may do. Role, Permission, resource scope, delegation, privileged access, SoD and the Authorization Decision remain `L5-D-02-*` responsibilities.

### In Scope

- Authentication-entry validation after D01-002 principal resolution.
- One-Tenant authentication execution and principal/account eligibility gating.
- Authentication Attempt technical control and correlation.
- Authentication Policy consumption for method/provider/factor requirements.
- Provider-independent Authentication Method / Authentication Provider execution boundaries.
- Credential association, verification and lifecycle-control mechanisms without exposing secret values.
- MFA challenge/verification mechanics using only eligible active verified factors.
- Successful Authentication Session creation/activation mechanics.
- Authenticated-context construction for D02 consumption.
- Session continuation/renewal using Authentication-owned continuation state such as Refresh Token.
- Session termination/invalidation mechanics when logout, account/identity closure, Credential recovery or policy requires it.
- Credential replacement/rotation cutover behavior needed to prevent superseded Credentials from continuing to authenticate.
- Failure containment, secret-safe evidence and provider availability/fallback execution.
- Persistence/consistency constraints for Authentication Attempt, Credential technical state, Authentication Session, Refresh Token and factor/provider execution references.
- Provider-neutral, product-neutral logical architecture sufficient for implementation.

### Out of Scope / Non-Goals

- Canonical Person, Identity or User Account business lifecycle ownership — upstream A02.
- Human-account, service/workload/job/integration principal technical identity models — `L5-D-01-002`.
- Account provisioning, identity/account recovery orchestration, disablement synchronization and lifecycle coordination — `L5-D-01-004`.
- Role, Permission, authorization-policy evaluation, resource scope, delegation, privileged access, maker-checker, SoD or Authorization Decision — `L5-D-02-*`.
- Cryptographic primitive selection, key management, secrets/certificate custody or machine-credential protection — `L5-D-03-*`.
- Audit/evidence persistence and tamper-evident storage — `L5-D-04-*`.
- Physical database/storage technology, DDL, locking primitive or persistence topology — `L5-B-02-*`.
- API endpoint/schema technology, event-broker technology or message-delivery implementation — `L5-C-01-*` / `L5-C-02-*`.
- Runtime/container/server/cloud topology — `L5-E-04-*`.
- L6 login, MFA, recovery or session-management screens and journeys.
- Selection of an IAM vendor, IdP product, directory product, federation protocol, token format, password-hashing algorithm, MFA technology, database, cache, broker, cloud service or session-store product.
- Invented authentication timeouts, Credential expiry periods, Refresh Token lifetimes, lockout thresholds, retry counts or numeric security targets.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-D-01-003` row | This specification owns authentication/session/credential mechanisms separate from authorization policy. | Establishes the exact technical responsibility and ownership ceiling. |
| L5 Peer | `L5-D-01-001_IAM_Architecture_Baseline.md` | v1.3.0 / Architecture Approved | §§2–5; C-002–C-008 | D01 baseline separates upstream identity/account truth, Authentication execution, authenticated-context handoff, Authorization and lifecycle synchronization. | Constrains this detailed realization to remain inside the D01 authentication boundary. |
| L5 Working Peer | `L5-D-01-002_Identity_Account_and_Service_Identity_Architecture.md` | v1.4.0 / Architecture Approved | §§4–5 | D01-002 supplies Tenant-bound human/non-human principal context and preserves distinction between principal identity and Credential secret/mechanism. | Supplies the principal/account context consumed by this specification without D01-003 becoming principal-lifecycle owner. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-007-BH-001`, `BH-002` | Authentication binds to exactly one Tenant and canonical Identity and shall not succeed when the applicable User Account status does not permit authentication. | Requires an authentication-entry gate before provider/Credential/MFA processing can produce success. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-007-BH-003`–`BH-005` | Authentication Policy, eligible method/provider/Credential and required MFA conditions gate success. | Requires policy-driven provider/method, Credential and MFA control points. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-007-BH-006`, `BH-007` | Successful authentication publishes the source-defined success fact and establishes/activates the Authentication Session with the canonical session facts. | Requires session creation only after completed authentication success and preserves canonical event meaning. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-007-BH-008`, `BH-009` | Authenticated context is supplied to Authorization without granting permission; attributable evidence covers policy, provider/method, Credential/MFA outcomes and session establishment. | Requires a permission-free authenticated-context contract and secret-safe evidence handoff. |
| L4 | `L4-A-008_Lost_or_Invalid_Credential_to_Restored_Authentication_Capability.md` | v1.0.1 / Architecture Approved | `L4-A-008-BH-001`–`BH-008` | Recovery requires resolvable Tenant/Identity/User Account context, governed recovery verification, policy-valid replacement Credential, covered previous Credentials incapable of authentication, optional policy-required session consequence and secret-safe evidence. | D01-003 owns the Credential/session mechanism consequences while D01-004 owns recovery/lifecycle orchestration. |
| L4 | `L4-A-009_Multi_Factor_Enrollment_to_Active_Verified_Authentication_Factor.md` | v1.0.1 / Architecture Approved | `L4-A-009-BH-001`, `BH-004`–`BH-009` | MFA enrollment depends on active authenticated context; unverified factors cannot authenticate; only active verified factors may later satisfy MFA; evidence must not expose factor secrets. | Requires factor eligibility verification and MFA challenge execution separate from factor-secret disclosure. |
| L4 | `L4-A-053_Person_Institutional_Exit_to_Governed_Identity_Closure.md` | v1.0.1 / Architecture Approved | `L4-A-053-BH-002`, `BH-004`, `BH-009` | Closure disables login capability, requires active-session termination, prevents authentication after archival and must not falsely claim closure while session termination remains incomplete. | Requires externally triggered session invalidation/termination and authentication-denial mechanisms. |
| L4 | `L4-A-055_Institutional_Authentication_Provider_Onboarding_to_Governed_Provider_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-055-BH-002`, `BH-005`, `BH-006`, `BH-009`, `BH-010` | Provider execution uses a uniform provider boundary; compatibility and policy scope govern deterministic provider eligibility/priority/fallback; invalid/unavailable providers are excluded from normal selection. | Requires provider adapters and policy-driven runtime selection without vendor semantics leaking into canonical architecture. |
| L2 | `L2-A02_Person_and_Identity_Workflows.md` | Locked | INT-301; 2026-08-20 activation sequencing; Authentication Preconditions | A02 owns Person, Identity and User Account; Authentication consumes their references and shall not own their lifecycles. | Prevents D01-003 from mutating or duplicating Person/Identity/User Account business state. |
| L2 | `L2-A03_Authentication_Workflows.md` | v1.1.0 / Locked | `Scope`; `Out of Scope`; `Canonical Ownership`; Credential Management; Authentication Session/Refresh Token ownership; Authentication Provider/Method; MFA architecture | Authentication owns Credential, Authentication Session, Refresh Token, Trusted Device, Authentication Provider/Method, MFA Enrollment, Authentication Policy and Authentication Attempt; Authorization remains outside A03. | Establishes the canonical Authentication-owned aggregates/mechanisms realized by this L5 specification. |
| ADR | `ADR-PLAT-AUTH-001.md` | Approved | §§2, 3.1, 3.2 | Authentication remains separate from authorization; User Account is authentication/runtime account context, not the Role Assignment subject. | Prevents D01-003 session/token state from becoming an authorization assignment or permission cache of record. |

All materially applicable L4 files currently carry `Draft` metadata on `main`. This specification therefore remains `Draft`; Section 9 records the approval gate rather than silently promoting Draft L4 content to approval-level authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Authentication is executed inside exactly one resolved Tenant and principal/Identity context. | `L4-A-007-BH-001`; D01-001 | Every Authentication Attempt, provider execution, Credential verification and session success path is Tenant-bound. |
| C-002 | Source account/principal eligibility must positively permit authentication. | `L4-A-007-BH-002`; D01-002 | No successful authentication/session may be created from unresolved or ineligible source account context. |
| C-003 | Authentication Policy, eligible method/provider/Credential and required MFA conditions all gate success. | `L4-A-007-BH-003`–`BH-005` | Authentication success is an ordered governed outcome, not the result of Credential verification alone. |
| C-004 | Authentication success and session activation do not grant business permission. | `L4-A-007-BH-008`; L2-A03; `ADR-PLAT-AUTH-001` | Authenticated context contains no authoritative Role/Permission/Authorization Decision semantics. |
| C-005 | Credential/factor secret material shall not be exposed through identity/session/evidence contracts. | `L4-A-008-BH-008`; `L4-A-009-BH-009`; D01-001 | Verification operates on protected secret/verifier references; evidence carries outcomes and references only. |
| C-006 | Covered previous Credentials shall cease being capable of authentication after successful governed replacement. | `L4-A-008-BH-006` | Credential cutover must prevent stale/superseded Credential success even though the exact upstream resulting state remains unresolved. |
| C-007 | Policy-required session consequences shall be enforceable after Credential recovery or identity/account closure. | `L4-A-008-BH-007`; `L4-A-053-BH-002/BH-009` | Session invalidation/termination is an explicit technical control, not an incidental cleanup action. |
| C-008 | Only active verified MFA factors may satisfy required MFA. | `L4-A-009-BH-004`–`BH-008` | Factor challenge verification must confirm source-governed factor eligibility before contributing to success. |
| C-009 | Runtime provider selection must respect governed provider eligibility/priority/availability/fallback without becoming vendor-specific architecture. | `L4-A-055-BH-002/BH-005/BH-006/BH-009/BH-010` | Provider adapters normalize provider execution under one D01 contract. |
| C-010 | Material technology/protocol/cryptographic choices require applicable ADR/technical governance. | L5 Technology Selection & ADR Methodology v1.0.1 | Logical architecture is concrete; Keycloak, the custom credential API/session profile and D03 cryptographic standards are governed by `ADR-L5-0024`, `ADR-L5-0038`, `ADR-L5-0039` and `ADR-L5-0040`. Numeric session lifetimes and profile-specific controls remain governed separately. |

### 3.3 Assumptions

No material assumption is promoted to fact. Numeric lifetimes, timeout/lockout values, protocol choices, cryptographic selections and provider-product decisions remain intentionally unresolved rather than assumed.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Authentication Attempt technical control and correlation.
- Authentication-entry eligibility enforcement after D01-002 principal resolution.
- Runtime application of Authentication-owned policy inputs to method/provider/Credential/MFA execution.
- Provider-independent Authentication Method and Authentication Provider execution adapters.
- Credential technical verification boundary and Credential lifecycle-control mechanisms required for authentication use.
- MFA challenge/verification execution using eligible source-governed factor state.
- Authentication Session creation/activation, continuation, invalidation and termination mechanisms.
- Refresh Token technical state and session-continuation mechanics.
- Construction of permission-free authenticated session/context for D02 consumption.
- Credential replacement/rotation cutover mechanics that prevent superseded Credential authentication.
- Secret-safe authentication evidence context and terminal outcome handoff.
- Authentication-side failure containment when provider, Credential, MFA, session or continuation validation fails.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-D-01-001` | IAM Architecture Baseline | Owns family-level IAM boundaries/invariants. | D01-003 must refine, not redefine, the D01 authentication boundary. |
| working `L5-D-01-002` | Identity Account & Service Identity Architecture | Owns human/non-human principal models and principal/account binding. | D01-003 authenticates a resolved principal; it does not create a substitute Person/User Account/service identity model. |
| `L5-D-01-004` | Identity Provisioning, Recovery & Lifecycle Architecture | Owns account/identity provisioning, recovery orchestration and lifecycle synchronization. | D01-003 supplies Credential/session mechanisms required by D01-004 consequences without owning recovery workflow authority. |
| `L5-D-02-001` / `L5-D-02-004` | Authorization baseline; policy decision/enforcement | Own authorization decisions and enforcement. | D01-003 emits authenticated context only; D02 decides whether protected operations are permitted. |
| `L5-A-04-001` / `L5-A-04-002` | Tenant/Institution isolation and context propagation | Own isolation/context mechanics. | Every authentication/session path must preserve Tenant/Institution boundary. |
| `L5-D-03-001` | Security Architecture & Trust Boundary Baseline | Owns material trust boundaries/security-control principles. | External provider, client and service authentication occurs inside D03-governed trust boundaries. |
| `L5-D-03-002` / `L5-D-03-003` / `L5-D-03-004` | Cryptography, key management, secrets/certificates/machine credentials | Own secret protection and cryptographic material lifecycle. | D01-003 references protected verification/signing/encryption capabilities without selecting primitives or custody. |
| `L5-D-04-001` / `L5-D-04-002` | Audit/evidence baseline and provenance model | Own evidence/provenance representation and persistence boundary. | D01-003 supplies authentication outcome/evidence context without becoming the immutable audit store. |
| `L5-B-02-001` / `L5-B-02-002` / `L5-B-02-003` | Persistence, data ownership/isolation, transaction/consistency/concurrency | Own physical persistence/consistency mechanisms. | Authentication state requires durable and concurrency-safe realization without D01-003 selecting a database. |
| `L5-C-01-*` | API / synchronous contract architecture | Own transport-level synchronous contracts. | Authentication control points may be exposed through governed synchronous boundaries. |
| `L5-C-02-*` | Event/messaging architecture | Own canonical event transport/delivery mechanics. | Authentication events retain upstream semantic publisher while C02 owns distribution infrastructure. |
| `L5-G-01-003` | Client Security, Session, State & Contract Consumption Architecture | Own client-side session/state/contract handling. | Client implementation consumes D01-003 session/authentication boundaries without becoming authentication authority. |
| `L5-E-05-*` | Observability/operations | Own technical operational telemetry. | Authentication health/failure telemetry remains separate from D04 audit truth. |

### Must Not Own

- Person, Identity or User Account canonical business lifecycle.
- Service/workload/job/integration identity model ownership assigned to D01-002.
- Account provisioning/recovery workflow authority or lifecycle synchronization owned by D01-004.
- Role, Permission, resource scope, delegation, privileged access, SoD or Authorization Decision.
- Cryptographic algorithms, keys, certificates, secret custody or machine-credential protection internals.
- Audit/evidence persistence.
- Physical persistence/locking/database technology.
- API/message transport technology.
- Runtime/deployment topology.
- L6 authentication user experience.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| Authentication Entry Gate | Control Point | Validates resolved Tenant/principal/account context before Authentication-owned execution begins. | D01-002 principal context; Tenant context; source account eligibility; correlation. | Authentication Attempt correlation/reference; no upstream lifecycle ownership. | Eligible Authentication Attempt context or rejection. | D01-002; A04; B02 | Missing Tenant/principal, cross-Tenant context or ineligible account cannot proceed to successful authentication. | `L4-A-007-BH-001/BH-002`; C-001/C-002. |
| Authentication Policy Resolver Boundary | Control Point / Adapter | Resolves the applicable Authentication-owned policy inputs that define required method/provider/factor conditions for the attempt. | Tenant/principal context; Authentication Attempt; source-governed policy reference. | Policy decision input/reference only; no D02 authorization policy. | Required authentication conditions to execution coordinator. | A03 upstream; A04; B02 | Missing/ambiguous policy prevents success when policy is required; D01-003 shall not invent a permissive default. | `L4-A-007-BH-003`; C-003/C-004. |
| Authentication Execution Coordinator | Component / Control Point | Coordinates provider/method selection, Credential verification, MFA challenge requirements and terminal authentication outcome. | Eligible attempt; Authentication Policy result; provider/method candidates; principal context. | Attempt progress/outcomes necessary for Authentication ownership and evidence. | Success outcome to Session Authority or failure outcome/evidence. | Provider adapters; Credential Verifier; MFA Challenge Controller; D04 | Any required failed/unresolved condition prevents success/session establishment. | `L4-A-007-BH-003`–`BH-007`. |
| Authentication Provider Selector | Control Point | Applies source-governed provider eligibility, priority, availability, fallback/failover conditions without embedding vendor semantics. | Authentication Policy result; enabled/eligible provider references; method requirement. | Provider-selection execution context; no provider business lifecycle ownership. | Selected eligible provider adapter or controlled no-provider failure. | A03 upstream; D03; B02 | Disabled/ineligible/unavailable provider shall not be selected for normal authentication. | `L4-A-055-BH-002/BH-005/BH-006/BH-009/BH-010`. |
| Authentication Provider Adapter Boundary | Adapter | Normalizes provider-specific execution behind a common D01 authentication result boundary. | Selected provider; method; principal context; protected authentication material reference/challenge input. | Provider execution/correlation state only as needed. | Normalized verification/challenge result to coordinator. | D03; C01; external provider owner where applicable | Provider-specific success cannot bypass D01 policy/Credential/MFA/session gates. Provider failure remains a failed/alternate governed path. | C-003/C-009. |
| Credential Verification Boundary | Component / Control Point | Verifies an eligible Credential through its governed method without exposing secret values and returns a typed verification outcome. | Principal ref; Credential ref; method/provider context; protected verifier/secret capability reference. | Credential lifecycle/control metadata and verification outcome; secret material remains D03-protected. | Credential result to coordinator; evidence reference to D04. | D01-002; D03; B02 | Unknown/ineligible/superseded/revoked-or-equivalent source condition or failed verification cannot contribute to success. Exact upstream state mappings are not invented. | `L4-A-007-BH-004`; `L4-A-008-BH-004`–`BH-006`; C-005/C-006. |
| MFA Challenge Controller | Component / Control Point | Executes required challenge(s), validates that the factor is eligible/active/verified under source semantics, and returns factor outcomes. | Authentication Policy requirement; principal/session/attempt context; factor reference; protected factor capability. | Challenge correlation/outcome; factor secret remains protected. | MFA satisfaction/failure to coordinator. | A03 upstream; D03; B02 | Unverified/ineligible factor or failed required challenge prevents authentication success. | `L4-A-007-BH-005`; `L4-A-009-BH-004`–`BH-008`. |
| Authentication Session Authority | Component / State Owner | Creates and activates Authentication Session only after terminal authentication success; owns session validity/invalidation/termination mechanics and authenticated-context production. | Successful authentication result; Tenant/principal context; Authentication method/provider/factor outcomes; applicable session policy inputs. | Authentication Session state/reference; session continuation association; source-defined contextual references. | Active authenticated context to D02/client/technical consumers; canonical session facts through C02. | B02; D04; C01/C02; A04 | Failed/incomplete authentication cannot create/activate a successful session; invalidated/terminated session cannot be emitted as active. | `L4-A-007-BH-006`–`BH-009`; C-004/C-007. |
| Refresh / Session Continuation Controller | Component / Control Point | Validates Authentication-owned continuation state (including Refresh Token where applicable), current session/principal eligibility and policy before continuing an authenticated session. | Existing session ref; Refresh Token/continuation ref; Tenant/principal context; current eligibility/policy basis. | Refresh Token and continuation-control state owned by Authentication; no authorization state. | Continued/renewed session context or denial/termination consequence. | D01-002; D01-004; B02; D03 | Stale, mismatched, invalidated or ineligible continuation state cannot produce active authenticated context. | L2-A03 Refresh Token ownership; C-002/C-004/C-007. |
| Session Invalidation / Termination Control | Control Point | Applies authentication-side session consequences for logout, Credential recovery, source identity/account closure or other governed termination basis. | Session refs or principal-scope termination request; governed reason/basis; Tenant context. | Session invalidation/termination state and correlation/evidence. | No-longer-active session result; evidence; canonical events only where upstream-defined. | D01-004; B02; D04; C02 | Partial/failed termination must not be represented as completed closure where upstream requires termination. | `L4-A-008-BH-007`; `L4-A-053-BH-002/BH-009`. |
| Credential Cutover Control | Control Point | Applies the authentication-side consequence of governed Credential replacement/rotation so covered previous Credentials cannot continue authenticating. | Replacement Credential ref and validated activation basis from D01-004/recovery flow; previous Credential scope; Tenant/principal context. | Credential control metadata/association state; exact upstream lifecycle transition remains source-governed. | New eligible Credential available for later authentication; covered previous Credential blocked from successful verification. | D01-004; D03; B02 | The replacement path is incomplete if a covered previous Credential can still authenticate. | `L4-A-008-BH-004`–`BH-006`; C-006. |
| Authentication Evidence Handoff | Contract Boundary | Emits non-secret authentication attribution/outcome information for D04 evidence and E05 operational observability. | Attempt/session/provider/method/Credential/MFA outcome references; Tenant/principal; correlation; terminal outcome. | No secret values; no immutable evidence store owned here. | D04 evidence context; E05 operational signals. | D04; E05; C01/C02 | Secret material or raw Credential/factor values shall not enter audit/telemetry payloads. | `L4-A-007-BH-009`; `L4-A-008-BH-008`; `L4-A-009-BH-009`. |

The elements above are logical ownership/control boundaries. They do not imply separate services, processes, hosts, containers or products.

### 5.2 Primary Technical Flows

#### A. Primary Human Authentication to Active Session

```text
D01-002 resolved human principal/account context
        ↓
Authentication Entry Gate
        ↓
Tenant/principal/account eligibility valid?
        ├─ No → terminal authentication rejection
        ↓ Yes
Authentication Policy Resolver Boundary
        ↓
Required method/provider/Credential/MFA conditions
        ↓
Authentication Provider Selector
        ↓
Provider Adapter Boundary
        ↓
Credential Verification Boundary
        ↓
MFA required?
        ├─ Yes → MFA Challenge Controller → all required challenges succeed?
        │                                  ├─ No → terminal authentication failure
        │                                  └─ Yes
        └─ No
        ↓
Authentication Execution Coordinator
recognizes terminal authentication success
        ↓
Authentication Session Authority
        ↓
Session created / activated under source-defined semantics
        ↓
AuthenticatedSessionContext
        ↓
D02 authorization evaluation for protected operations
```

Credential verification alone shall never bypass required provider, policy, MFA or account-eligibility conditions.

#### B. Provider Selection / Fallback Execution

```text
Authentication Policy + available provider references
        ↓
Filter by source-governed eligibility / enabled availability / method compatibility
        ↓
Apply governed relative priority
        ↓
Select eligible provider
        ↓
Provider execution
        ├─ success/valid response → continue Authentication flow
        └─ unavailable/failure
              ↓
        Is fallback permitted by source policy?
              ├─ No → authentication failure
              └─ Yes → select next governed eligible provider
```

D01-003 does not invent provider priorities, fallback order or availability rules. It executes the source-governed policy.

#### C. Session Continuation

```text
Session continuation request
        ↓
Resolve Tenant + session + principal
        ↓
Validate session is eligible for continuation
        ↓
Validate Authentication-owned continuation / Refresh Token state
        ↓
Re-check source account/principal eligibility and applicable Authentication policy
        ↓
All required continuation conditions satisfied?
        ├─ No → deny continuation; apply termination/invalidation where governed
        └─ Yes → issue/return continued authenticated session context
        ↓
D02 remains responsible for authorization on later protected operations
```

Continuation does not preserve or cache an authoritative Authorization Decision.

#### D. Credential Recovery / Replacement Cutover Handoff

```text
D01-004 governed recovery/provisioning orchestration
        ↓
Replacement Credential prepared and policy-validated
        ↓
Credential Cutover Control
        ↓
Replacement becomes eligible under source-defined conditions
        ↓
Covered previous Credential(s) made incapable of authentication
        ↓
Policy requires existing-session consequence?
        ├─ Yes → Session Invalidation / Termination Control
        └─ No
        ↓
Recovery mechanism outcome returned to D01-004
```

The exact business recovery-completion event and exact previous-Credential resulting lifecycle state are not invented here.

#### E. Identity / Account Closure to Session Termination

```text
D01-004 receives authoritative closure / disablement consequence
        ↓
Resolve affected Tenant + principal/account + active session references
        ↓
Prevent new successful authentication for ineligible subject
        ↓
Session Invalidation / Termination Control
        ↓
All required active sessions terminated/inactivated?
        ├─ No → return incomplete consequence; do not claim closure complete
        └─ Yes → return completed authentication-side consequence
```

D01-003 never changes Person or Identity lifecycle state.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Tenant/principal/account unresolved or cross-Tenant | Authentication Entry Gate | Reject the authentication success path; record attributable non-secret failure context. | Guessing Tenant/Identity or creating a session from ambiguous principal context. | D01-002/A04 source resolution. | `L4-A-007-BH-001/BH-002`. |
| Authentication Policy missing/ambiguous where required | Policy Resolver | Prevent success until authoritative policy can be resolved. | Permissive default invented by D01-003. | Upstream A03 policy governance / D01-004 where synchronization applies. | `L4-A-007-BH-003`. |
| Provider ineligible, disabled or unavailable | Provider Selector / Adapter | Exclude provider; apply only source-permitted fallback; otherwise fail authentication. | Unapproved provider fallback or vendor-specific bypass. | Provider lifecycle owner / operational recovery. | `L4-A-055-BH-006/BH-009/BH-010`. |
| Credential missing/ineligible/failed verification | Credential Verification Boundary | Return failed Credential outcome; no authentication success/session activation. | Treating provider response or account existence as Credential success. | D01-004 for governed recovery; D03 for secret/capability failure. | `L4-A-007-BH-004`; `L4-A-008`. |
| Required MFA challenge fails or factor is not eligible active verified | MFA Controller | Authentication fails/incomplete; no session activation. | Bypassing MFA due to provider success or previous session history. | D01-004 where factor recovery/provisioning is governed. | `L4-A-007-BH-005`; `L4-A-009-BH-004`–`BH-008`. |
| Session creation/activation persistence fails after authentication verification | Session Authority | Do not expose active authenticated context; preserve failure evidence and recover under B02/E01 architecture. | Returning authentication success as an active session when session state is not durably established. | B02 persistence; E01 resilience; D04 evidence. | `L4-A-007-BH-007/BH-009`. |
| Continuation / Refresh state stale, mismatched, invalidated or cross-Tenant | Refresh Controller | Reject continuation; do not emit active session context. | Silent continuation based only on previously successful authentication. | D01-004/session governance; D03 if protected token capability is involved. | L2-A03 Refresh Token/session ownership; C-001/C-007. |
| Covered old Credential still verifies after replacement cutover | Credential Cutover Control | Treat recovery/cutover as incomplete; block successful use of the covered old Credential. | Declaring recovery complete while old Credential still authenticates. | D01-004 recovery coordination; B02 consistency. | `L4-A-008-BH-006`. |
| Required session termination incomplete | Session Termination Control | Report incomplete consequence; affected session shall not be falsely represented as terminated. | Claiming identity/account closure complete when required session consequence has not completed. | D01-004 + B02/E01. | `L4-A-053-BH-009`. |
| Evidence path attempts to include raw secret/factor material | Evidence Handoff | Reject/redact at the architecture boundary and emit only reference/classification/outcome information. | Password/API secret/private key/token secret/MFA secret in audit or ordinary telemetry. | D03/D04. | C-005; L4 evidence BHs. |

### 5.4 Conceptual Contracts / Context

The following structures define information categories, not transport schemas or exact field names.

```text
AuthenticationAttemptContext
{
  resolved Tenant reference
  D01 principal reference
  canonical Identity / User Account reference where applicable
  source account/principal eligibility reference
  Authentication Policy reference
  correlation / attempt reference
}
```

```text
AuthenticationRequirementSet
{
  permitted / required Authentication Method category
  eligible Authentication Provider set / selection basis
  Credential requirement
  MFA requirement and required assurance/challenge basis
  session consequence / continuation policy reference where applicable
}
```

The requirement set contains governed authentication conditions only. It contains no Role, Permission or authorization-policy decision.

```text
CredentialVerificationContext
{
  resolved Tenant reference
  principal reference
  Credential reference / classification
  Authentication Method reference
  Authentication Provider reference when applicable
  protected verification-capability reference
  correlation / attempt reference
}
```

No raw Credential secret is part of the conceptual cross-boundary contract.

```text
AuthenticationResult
{
  Tenant reference
  principal reference
  attempt reference
  terminal authentication outcome
  Authentication Method / Provider references
  Credential verification outcome reference
  MFA outcome reference(s) where applicable
  evidence / correlation reference
}
```

```text
AuthenticatedSessionContext
{
  Tenant reference
  authenticated principal reference
  Authentication Session reference
  authentication time / assurance context required by source semantics
  method / provider / factor outcome references required for downstream trust evaluation
  source-defined contextual references where applicable
  correlation / evidence reference
}
```

`AuthenticatedSessionContext` is proof-of-authentication context, not an authorization grant. It shall contain no canonical Permission set or Authorization Decision.

```text
SessionContinuationContext
{
  Tenant reference
  principal reference
  Authentication Session reference
  Authentication-owned continuation / Refresh Token reference
  current source eligibility reference
  applicable Authentication Policy reference
  correlation reference
}
```

### 5.5 State / Persistence / Consistency

- D01-003 owns Authentication technical state corresponding to Authentication Attempt, Credential control metadata, Authentication Session, Refresh Token and authentication-factor/provider execution references within its canonical responsibility.
- Person, Identity and User Account business state remains upstream-owned. D01-003 stores references necessary for authentication and shall not create an independent canonical copy of those lifecycles.
- Physical secret material and cryptographic key/certificate custody remain D03-owned. D01-003 may persist only protected verifier/secret references or D03-governed protected material necessary for its owned authentication mechanism.
- An active authenticated context may be exposed only after the Authentication Session's required state change is durably established under B02 persistence/consistency architecture.
- Credential replacement/cutover shall be consistency-safe: after the governed cutover completes, a covered prior Credential must not succeed merely because another replica/cache/process retains stale eligibility.
- Session invalidation/termination shall be consistency-safe: once the governed invalidation/termination is authoritative, downstream session validation shall not treat stale technical state as active.
- Exact database technology, index model, transaction isolation level, lock primitive and replication mechanism remain B02-owned.
- Exact Credential/session/Refresh Token numeric lifetimes, lockout thresholds and retry counters are not defined here.
- Where upstream lifecycle terminology is unresolved, D01-003 shall model the necessary technical eligibility/ineligibility outcome without inventing a new canonical business state name.

### 5.6 Asynchronous / Messaging Behavior

- Canonical Authentication events retain their source-defined semantic publisher. D01-003 does not rename or republish a new business fact merely to match a technical component boundary.
- C02 owns event transport, durable delivery, ordering and message-recovery architecture where canonical Authentication events cross asynchronous boundaries.
- Session termination or Credential cutover may require fan-out to multiple technical session/verification enforcement points; exact messaging topology, retry policy, timeout and delivery thresholds are not invented here.
- A stale event or message shall not be allowed to resurrect a Credential/session state that has already been made ineligible by a later authoritative authentication state change.
- Where ordering/consistency is material, D01-003 supplies the semantic requirement and B02/C02 own the concrete consistency/delivery mechanism.

### 5.7 Security / Trust / Isolation

- Every Authentication Attempt, Credential association, provider execution, session and continuation operation is Tenant-bound.
- Authentication Provider adapters are trust boundaries; provider-specific responses are untrusted until normalized and accepted by D01 policy/verification gates.
- Credential verification shall minimize secret exposure. Raw secrets shall not be copied into principal context, session context, business-domain requests, audit evidence or ordinary operational telemetry.
- MFA factor secret/protected material remains isolated from session/authenticated-context contracts.
- Session and continuation artifacts are authentication-sensitive security material even though exact token technology is unresolved; their protection follows D03.
- Session identifiers or continuation artifacts shall not themselves be treated as permission. D02 must still authorize each protected action as applicable.
- Authentication success for a service/workload/job/integration principal preserves that non-human identity classification; it shall not be converted into a human User Account identity.
- Provider fallback shall not weaken required method/MFA/security conditions.
- Credential recovery shall not reactivate a Person/Identity/User Account whose source-governed eligibility does not permit authentication.
- D01-003 shall not log or expose raw Credential, Refresh Token, API secret, private-key or MFA secret values.

### 5.8 Runtime / Deployment / Operations

D01-003 defines logical authentication components/control points only. Authentication Entry Gate, Policy Resolver, Provider Selector/Adapters, Credential Verification, MFA Challenge Controller, Session Authority, Refresh Controller and Session Termination Control may be co-located or separately realized only under A01/E04 architecture and applicable ADR governance.

Runtime realization must preserve:

- one-Tenant authentication isolation;
- no bypass around Credential/MFA/session gates;
- secret isolation;
- authoritative session invalidation;
- provider-adapter boundary integrity;
- permission-free authenticated context;
- observable health/failure signals without leaking secrets.

Keycloak is the canonical IAM platform; Valkey is the canonical shared cache where a cache is appropriate, but session authority remains D01-owned. No microservice topology or cloud vendor is selected here.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| AD-001 | Authentication coordination boundary | Authentication success is produced through one logical D01 execution coordinator that composes entry eligibility, policy, provider/method, Credential and required MFA outcomes before session establishment. | `L4-A-007-BH-001`–`BH-007`; frozen D01-003 responsibility | Prevents independent sub-mechanisms from declaring authentication success prematurely. | None — local architecture within frozen responsibility. |
| AD-002 | Authentication vs authorization separation | D01-003 emits permission-free authenticated session context; D02 alone owns later Authorization Decision/Permission evaluation. | `L4-A-007-BH-008`; L2-A03; ADR-PLAT-AUTH-001 | Prevents session/token state from becoming an implicit RBAC store or access grant. | None — direct authoritative constraint. |
| AD-003 | Credential secret isolation | D01-003 owns Credential verification/control semantics while raw secret/cryptographic custody is isolated behind D03-governed protection capability; cross-boundary contracts use references/outcomes. | L4 evidence constraints; D03 corpus boundary | Limits secret exposure and prevents identity/session stores from duplicating secret material. | None for boundary; technology selection may require ADR. |
| AD-004 | Provider normalization | All provider execution enters through a provider-independent adapter/result boundary; provider eligibility/priority/fallback is source-policy driven. | `L4-A-055-BH-002/BH-005/BH-006/BH-009/BH-010` | Supports multiple governed providers without vendor semantics redefining Authentication architecture. | None for adapter boundary; provider/protocol selection requires ADR when material. |
| AD-005 | Session issuance gate | An Authentication Session may be exposed as active only after all required authentication conditions and durable session establishment succeed. | `L4-A-007-BH-005`–`BH-008` | Prevents partial authentication from becoming usable authenticated context. | None. |
| AD-006 | Continuation is revalidated Authentication state | Refresh/session continuation validates current session, continuation artifact, Tenant/principal eligibility and applicable Authentication policy before emitting continued authenticated context; it does not carry authoritative authorization state. | L2-A03 session/Refresh Token ownership; `L4-A-007-BH-008`; C-002/C-004 | Prevents stale authentication or old authorization results from being perpetuated by continuation alone. | None for architecture; token technology/lifetime is unresolved. |
| AD-007 | Credential cutover fail-safe | Governed Credential replacement is not technically complete until covered previous Credentials are incapable of successful verification; exact upstream state/event names remain source-governed. | `L4-A-008-BH-004`–`BH-006` | Realizes the observable security outcome without silently repairing upstream lifecycle ambiguity. | None. |
| AD-008 | Explicit session invalidation control | Account/identity closure and policy-required recovery consequences use a first-class session invalidation/termination control rather than incidental expiry. | `L4-A-008-BH-007`; `L4-A-053-BH-002/BH-009` | Allows upstream lifecycle closure to obtain a verifiable Authentication-side consequence. | None. |
| AD-009 | MFA factor eligibility at challenge time | Required MFA challenge execution verifies that the selected factor is source-eligible/active/verified before it may satisfy Authentication policy. | `L4-A-009-BH-004`–`BH-008`; `L4-A-007-BH-005` | Prevents stale or unverified factor enrollment from satisfying MFA. | None. |

Local decisions inherit the locked Keycloak/custom-login/session/security technology profiles; exact token/session representation and numeric lifetimes remain D01/D03-governed.

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant isolation | All authentication/session/Credential operations require one resolved Tenant; cross-Tenant fallback is forbidden. | `L4-A-007-BH-001`; D01-001 | D01-003; `L5-A-04-*` |
| Authentication integrity | No successful session until account, policy, provider/method, Credential and required MFA conditions succeed. | `L4-A-007-BH-002`–`BH-007`; AD-001/AD-005 | D01-003 |
| Authorization separation | Authenticated context carries identity/trust context, not Permission/Authorization Decision truth. | `L4-A-007-BH-008`; AD-002 | `L5-D-02-*` |
| Secret minimization | Authentication contracts/evidence use references/outcomes; raw secrets stay behind D03 protection boundaries. | L4 evidence BHs; AD-003 | `L5-D-03-*`; D01-003 |
| Session revocation integrity | Governed invalidation must take effect across enforcement points without stale state reactivating a session. | `L4-A-053-BH-009`; AD-008 | D01-003; B02/C02/E01 |
| Credential recovery integrity | Previous covered Credentials cannot remain usable after completed replacement cutover. | `L4-A-008-BH-006`; AD-007 | D01-003; D01-004 |
| Provider resilience | Only source-governed eligible fallback paths may be attempted; provider failure does not bypass Authentication requirements. | L4-A-055; AD-004 | D01-003; D03/E01 |
| Auditability | Attempt, policy, provider/method, Credential/MFA outcome and session result are attributable without secret disclosure. | `L4-A-007-BH-009`; L4-A-008/009 evidence | `L5-D-04-*` |
| Operational telemetry | Technical provider/session/authentication health may be observed without becoming business audit truth. | D04/E05 ownership split | `L5-E-05-*`; `L5-D-04-*` |

No numeric security, performance, availability, retention, timeout, TTL, lockout or retry target is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| CR-001 | Every authentication success path shall be bound to exactly one resolved Tenant and one D01 principal context before provider/Credential/MFA success is accepted. | §5.1 Authentication Entry Gate; AD-001 | `L4-A-007-BH-001` |
| CR-002 | Authentication shall not succeed when source account/principal eligibility does not positively permit authentication. | §5.1; C-002 | `L4-A-007-BH-002` |
| CR-003 | Credential verification alone shall not establish authentication success when Authentication Policy also requires provider/method or MFA conditions. | AD-001; §5.2A | `L4-A-007-BH-003`–`BH-005` |
| CR-004 | A required MFA challenge shall be accepted only from a source-eligible active verified factor; failed/ineligible factors shall prevent success. | AD-009; §5.1 MFA Controller | `L4-A-007-BH-005`; `L4-A-009-BH-004`–`BH-008` |
| CR-005 | An Authentication Session shall not be exposed as active until terminal authentication success and durable session establishment both succeed. | AD-005; §5.1 Session Authority | `L4-A-007-BH-006/BH-007` |
| CR-006 | Authenticated session/context emitted by D01-003 shall contain no authoritative Role, Permission, Delegation or Authorization Decision semantics. | AD-002; §5.4 | `L4-A-007-BH-008`; ADR-PLAT-AUTH-001 |
| CR-007 | Authentication evidence and ordinary telemetry shall not contain raw Credential, Refresh Token, private-key, API-secret or MFA-secret values. | AD-003; §5.7 | `L4-A-007-BH-009`; `L4-A-008-BH-008`; `L4-A-009-BH-009` |
| CR-008 | Runtime provider selection shall exclude source-ineligible/disabled/unavailable providers and shall use fallback only when permitted by source-governed provider policy. | AD-004; §5.2B | `L4-A-055-BH-006/BH-009/BH-010` |
| CR-009 | Provider-specific execution shall terminate at a common D01 authentication-result boundary and shall not bypass policy, Credential, MFA or session gates. | AD-004; §5.1 Provider Adapter | `L4-A-055-BH-002/BH-005`; `L4-A-007` |
| CR-010 | After completed governed Credential replacement, every Credential covered by that replacement decision shall be incapable of successful authentication. | AD-007; §5.2D | `L4-A-008-BH-006` |
| CR-011 | D01-003 shall not invent the unresolved canonical prior-Credential resulting state or recovery-completion event merely to implement cutover. | §5.2D; OD-005 | `L4-A-008` open semantics |
| CR-012 | Session continuation shall revalidate the Authentication-owned session/continuation artifact and current Tenant/principal eligibility before emitting continued active authenticated context. | AD-006; §5.2C | L2-A03 session/Refresh Token ownership; D01 constraints |
| CR-013 | Session continuation shall not treat a previous Authorization Decision or cached Permission set as authoritative authorization for future protected operations. | AD-002/AD-006 | `L4-A-007-BH-008`; D02 ownership |
| CR-014 | When recovery or source identity/account closure requires session consequence, D01-003 shall expose an explicit invalidation/termination operation and shall report incomplete termination rather than falsely claiming completion. | AD-008; §5.2E | `L4-A-008-BH-007`; `L4-A-053-BH-002/BH-009` |
| CR-015 | Credential/session validity enforcement shall not allow stale technical state to resurrect a Credential or Session already made ineligible by a later authoritative Authentication state change. | §5.5/§5.6 | AD-007/AD-008; B02/C02 handoff |
| CR-016 | D01-003 shall preserve human/non-human principal classification supplied by D01-002 and shall not convert a service/workload/job/integration authentication into a fabricated human User Account identity. | §5.7 | working D01-002 boundary; L2-A03 non-human identity rules |
| CR-017 | No implementation may infer a specific IAM/IdP product, federation/authentication protocol, token format, Credential algorithm, session store, database, cache, broker or cloud service from this specification. | §5.8 | L5 Technology Selection & ADR Methodology |
| CR-018 | Canonical Authentication event meanings and publishers shall remain source-defined; D01-003 technical components shall not create replacement canonical event names. | §5.6 | L5 zero-guessing; L4-A-007 source event semantics |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| OD-001 | Material authentication L4 authority was previously Draft. | **Resolved** | R3A/R3B approval decisions; R3D traceability validation | L4/repository governance | **No** | Closed: applicable L4 parents are Architecture Approved and current-version relations are validated. |
| OD-002 | D01 peer stabilization was previously pending. | **Resolved** | D01-001/002/004 current R3D approved versions | D01 / Lead Architect | **No** | Closed by targeted D01 family reconciliation. |
| OD-003 | Exact session/Refresh Token lifetimes, renewal limits, inactivity/absolute expiry rules, replay controls, token representation and continuation artifact format are not selected here. | L5 / ADR / Security | No approved canonical L5 ADR/standard with exact choices or numeric targets is identified. | Lead Architect + D01/D03 architecture governance | **No for logical Draft; Yes before implementation where concrete values/mechanisms are required** | Resolve through governed security/technology decisions; allocate ADR only when mandatory under methodology. |
| OD-004 | **Partially resolved:** Keycloak 26.x + SutraERP custom username/password API, secure server-side session, privileged/high-risk MFA, WebAuthn/passkeys preferred where approved/stable with TOTP fallback. Exact provider-specific federation, token representation, verifier implementation and session-store internals remain governed details. | ADR / Technology | `ADR-L5-0024`; `ADR-L5-0038`; `ADR-L5-0040` | D01/D03 | Core profile resolved | Preserve numeric/profile deferrals. |
| OD-005 | Prior-Credential invalidation and recovery completion semantics were previously unresolved. | **Resolved** | A03 R3B amendment; L3-A-008 v1.1; L4-A-008 v1.0.1 Architecture Approved | Authentication architecture governance | **No** | Closed: replacement Credential activates via `CredentialActivated`; covered previous Credentials are revoked via `CredentialRevoked`; required session consequences remain completion barriers. |
| OD-006 | Exact canonical lifecycle/event semantics for session renewal/continuation beyond the explicitly sourced session creation/activation and closure consequences are not sufficiently established at L4 for new business event naming. | Upstream / L5 | L2-A03 owns Refresh Token/session lifecycle, but this specification shall not manufacture missing L4 canonical event semantics. | Authentication architecture governance | **No for continuation mechanics; Yes for introducing new canonical event names** | Use technical continuation state without new canonical business events until upstream/event semantics are explicitly governed. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-007-BH-001`–`BH-005` — Tenant/principal/account, Authentication Policy, provider/method, Credential and MFA success gates.
- `L4-A-007-BH-006`–`BH-009` — authentication success, session establishment, authorization separation and evidence.
- `L4-A-008-BH-001`–`BH-008` — Credential recovery/replacement verification, old-Credential cutover, session consequence and secret-safe evidence.
- `L4-A-009-BH-001`, `BH-004`–`BH-009` — session-bound MFA enrollment context, active verified factor usage and evidence.
- `L4-A-053-BH-002`, `BH-004`, `BH-009` — authentication prevention and active-session termination during identity closure.
- `L4-A-055-BH-002`, `BH-005`, `BH-006`, `BH-009`, `BH-010` — uniform provider boundary, deterministic selection/fallback and invalid-provider exclusion.

### Material Higher-Level / ADR Constraints

- `L2-A02` — Person/Identity/User Account remain upstream-owned.
- `L2-A03` — Credential, Authentication Session, Refresh Token, provider/method, MFA, Authentication Policy and Authentication Attempt ownership.
- `ADR-PLAT-AUTH-001` — authentication/runtime account context remains separate from Role Assignment and authorization.

### Material L5 Dependencies

- `L5-D-01-001` — IAM family baseline.
- working `L5-D-01-002` — human/non-human principal architecture.
- `L5-D-01-004` — provisioning/recovery/lifecycle coordination.
- `L5-D-02-*` — authorization.
- `L5-A-04-*` — Tenant/Institution isolation/context.
- `L5-D-03-*` — trust, cryptography, secrets and machine credentials.
- `L5-D-04-*` — audit/evidence.
- `L5-B-02-*` — persistence/consistency/concurrency.
- `L5-C-01-*` / `L5-C-02-*` — technical contracts/event transport.
- `L5-G-01-003` — client session/security consumption.
- `L5-E-05-*` — operational observability.

### Architecture Decisions

- `AD-001`–`AD-009`

### Conformance Requirements

- `CR-001`–`CR-018`

### Open / Blocking Items

- `OD-001` — **resolved by R3A/R3B approval + R3D validation**.
- `OD-002` — D01-002 and peer-L5 stabilization.
- `OD-003` — session/Refresh Token lifetime and continuation security decisions.
- `OD-004` — product/protocol/cryptographic technology selection.
- `OD-005` — prior-Credential invalidation/recovery event semantics unresolved upstream.
- `OD-006` — continuation/renewal canonical event semantics deferred.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0038`  
Human authentication uses the SutraERP custom credential API backed by Keycloak credential authority, server-side secure cookie sessions, bounded JWT use, privileged/high-risk MFA, WebAuthn/passkeys preferred where approved/stable, and TOTP fallback.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0039`  
System/service credentials are distinct from human credentials; non-human principals authenticate explicitly and remain separately revocable.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled authentication/session product wording with the locked custom credential API, Keycloak authority and D03 security profile. No OAuth Direct Access Grant is made canonical.


## 2026-09-23 R3D Targeted Revalidation

**Review result:** PASS — Architecture Approved.

The R3D review revalidated this specification against the current approved R3A/R3B L4 parent versions, the Locked R3B upstream amendments, applicable Locked/Approved ADRs including `ADR-L5-0048`, and the D01 peer set.

No applicable HF-01…HF-15 failure remains. Exact L4 BH→L5 relations are recorded in the canonical Section-A traceability partition under the R3D validation decision.

This approval does not authorize Final L5 Corpus Lock and does not resolve unrelated intentionally deferred implementation/profile choices.
