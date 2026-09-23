# Identity Provisioning, Recovery & Lifecycle Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-01-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-01 — IAM Architecture  
**Canonical Technical Responsibility:** Account provisioning/recovery/disablement synchronized with authoritative identity facts.  
**Technical Architecture Owner:** L5-D-01 — IAM Architecture  
**Assigned Author:** Devang  
**Version:** `1.2.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-D-01-001` v1.0.1 Draft; Review `L5-D-01-002` v1.0.0 Draft; Review `L5-D-01-003` v1.0.0 Draft; `L4-A-004` v1.0.1 Architecture Approved; `L4-A-005` v1.0.1 Architecture Approved; `L4-A-006` v1.0.1 Architecture Approved; `L4-A-007` v1.0.1 Architecture Approved; `L4-A-008` v1.0.1 Architecture Approved; `L4-A-053` v1.0.1 Architecture Approved; `L4-A-054` v1.0.1 Architecture Approved; `L2-A02` Locked; `L2-A03` v1.1.0 Locked; `ADR-PLAT-AUTH-001` Approved.  
**Referenced By:** Planned `L5-D-02-*`, account/identity lifecycle consumers, onboarding/integration architecture, and implementation specifications requiring governed IAM provisioning or lifecycle synchronization.  
**Applicable ADRs:** `ADR-PLAT-AUTH-001`; `ADR-L5-0039`; `ADR-L5-0048`. Product/workflow-engine/directory/transport/persistence choices remain governed separately.  

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the technical lifecycle architecture that keeps IAM technical capability synchronized with authoritative identity/account facts without transferring Person, Identity or User Account business ownership into L5 technical components.

It defines how SutraERP:

- receives authoritative identity/account lifecycle facts or governed lifecycle instructions;
- gates post-activation User Account provisioning where login capability is authoritatively required;
- coordinates technical principal/account binding after provisioning;
- coordinates Credential recovery with `L5-D-01-003` without changing Person, Identity or User Account lifecycle state;
- applies login-disablement and session-termination consequences when authoritative identity/account closure requires them;
- preserves lifecycle integrity through partial failures, duplicate/replayed inputs and asynchronous completion;
- processes bulk/onboarding identity outcomes without inventing a Migration Job business aggregate;
- blocks unsafe automatic synchronization when upstream merge/split or lifecycle semantics are unresolved;
- coordinates non-human principal provisioning/lifecycle only to the extent its authority is explicitly governed.

The architecture is deliberately **source-authority driven**. An upstream event name, record existence or technical observation does not by itself authorize creation, activation, recovery, disablement, deletion or reassociation of an identity/account.

### In Scope

- Technical lifecycle synchronization between upstream A02-owned identity/account facts and D01 technical identity/authentication capability.
- Post-activation User Account provisioning gate and execution coordination after authoritative eligibility exists.
- Account provisioning idempotency and duplicate-prevention at the technical orchestration boundary.
- Human principal/account binding handoff to `L5-D-01-002`.
- Credential/factor/session provisioning or recovery handoffs to `L5-D-01-003`.
- Recovery orchestration around Tenant, Identity, User Account eligibility, governed recovery method and Credential/session consequences.
- Login-disablement synchronization after authoritative Person/Identity/User Account lifecycle change.
- Active-session termination coordination and completion verification where required.
- Deprovisioning/closure technical consequences while retaining required stable historical references.
- Bulk identity-onboarding provisioning handoff using per-record authoritative outcomes.
- Lifecycle synchronization evidence and correlation.
- Fail-safe handling of stale, duplicate, contradictory or incomplete lifecycle inputs.
- Technology-neutral technical realization that does not require implementation teams to invent the core coordination model.
- Generic non-human principal lifecycle coordination boundary where D01-002 supplies an approved technical principal and authoritative provisioning basis.

### Out of Scope / Non-Goals

- Canonical Person, Identity or User Account business lifecycle ownership, states, transitions, approvers, business rules or canonical business events — upstream A02 and applicable L4.
- Definition of the missing business trigger/authority that decides *when* a User Account shall be provisioned after Person/Identity activation.
- Human/non-human principal identity model and binding structure — `L5-D-01-002`.
- Credential verification, provider execution, MFA challenge mechanics, Authentication Session internals, Refresh Token mechanics or Credential cutover implementation — `L5-D-01-003`.
- Authorization, Role/Permission assignment, scope, delegation, privileged access, SoD or Authorization Decision — `L5-D-02-*`.
- Cryptographic primitives, key/secret/certificate custody or machine-credential protection — `L5-D-03-*`.
- Audit evidence storage/provenance internals — `L5-D-04-*`.
- Physical persistence, transaction engine, database, cache or locking technology — `L5-B-02-*`.
- API/message transport technology — `L5-C-01-*` / `L5-C-02-*`.
- Deployment/runtime topology — `L5-E-04-*`.
- User-facing onboarding, recovery, account-administration or closure journeys — L6.
- Invented User Account lifecycle states/events not explicitly governed upstream.
- Invented Identity merge/split effects that current L4 explicitly leaves unresolved.
- Invented Migration Job aggregate, state machine or canonical migration events.
- Product selection for IAM, directory, provisioning, workflow/orchestration, recovery, messaging or storage.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-D-01-004` row | D01-004 owns account provisioning/recovery/disablement synchronization with authoritative identity facts. | Establishes this specification's canonical technical responsibility and permits technical lifecycle architecture without transferring upstream business ownership. |
| L5 Peer | `L5-D-01-001_IAM_Architecture_Baseline.md` | v1.0.1 / Draft | §§2–5; C-001–C-008 | D01 separates upstream Person/Identity/User Account truth, detailed principal models, Authentication mechanisms and lifecycle synchronization. | Constrains D01-004 to lifecycle coordination and governed handoffs rather than identity/authentication redefinition. |
| L5 Working Peer | `L5-D-01-002_Identity_Account_and_Service_Identity_Architecture.md` | v1.0.0 / Draft working artifact | §§4–5; `OD-003` | D01-002 owns human/non-human principal records/bindings and explicitly delegates provisioning/recovery/disablement synchronization to D01-004; non-human lifecycle authority is governed by `ADR-L5-0048`. | D01-004 coordinates lifecycle changes against D01-002 principal bindings but does not redefine principal identity. |
| L5 Working Peer | `L5-D-01-003_Authentication_Session_and_Credential_Architecture.md` | v1.0.0 / Draft working artifact | §§4–6; `AD-007`, `AD-008`; `CR-010`–`CR-015` | D01-003 owns Credential/session mechanisms, Credential cutover and session invalidation controls while delegating recovery/lifecycle orchestration to D01-004. | D01-004 orchestrates recovery and closure consequences through D01-003 rather than duplicating Credential/session internals. |
| L4 | `L4-A-004_Person_Establishment_to_Active_Canonical_Identity.md` | v1.0.1 / Architecture Approved | `L4-A-004-BH-008`–`BH-011`; §8 | Person and Identity must reach source-defined activation before canonical references are made available; User Account creation is explicitly outside A004. | Supplies the prerequisite identity facts for later provisioning while proving that D01-004 must not reinterpret A004 as an automatic User Account-creation trigger. |
| L4 | `L4-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md` | v1.0.1 / Architecture Approved | `L4-A-007-BH-001`, `BH-002` | Authentication requires one resolved Tenant/Identity context and a User Account status that permits authentication; no unsupported status mapping may be inferred. | Provisioning/lifecycle synchronization must produce source-governed account eligibility references and fail closed when eligibility is unresolved. |
| L4 | `L4-A-008_Lost_or_Invalid_Credential_to_Restored_Authentication_Capability.md` | v1.0.1 / Architecture Approved | `L4-A-008-BH-001`–`BH-008` | Governed recovery resolves Tenant/Identity/User Account, selects permitted recovery method, validates replacement Credential, prevents old Credential reuse, applies policy-required session consequence and preserves secret-safe evidence without changing Person/Identity/User Account lifecycle state. | Establishes recovery orchestration responsibility and the boundary between D01-004 lifecycle coordination and D01-003 Credential/session mechanics. |
| L4 | `L4-A-053_Person_Institutional_Exit_to_Governed_Identity_Closure.md` | v1.0.1 / Architecture Approved | `L4-A-053-BH-001`, `BH-002`, `BH-004`, `BH-009`; §§5, 8, 10 | Closure resolves Person/Identity/User Account context, disables login capability, requests active-session termination, prevents authentication after archival, and cannot claim completion while required session termination remains incomplete; exact archive/disable ordering is unresolved. | Establishes disablement/deprovisioning synchronization and a required completion barrier without allowing D01-004 to invent upstream lifecycle ordering. |
| L4 | `L4-A-054_Existing_Institutional_Population_to_Governed_Canonical_Identity_Onboarding.md` | v1.0.1 / Architecture Approved | `L4-A-054-BH-001`–`BH-010`; §10 | Bulk onboarding validates records, prevents duplicates, uses normal Person/Identity establishment, preserves independent per-record outcomes and stage-appropriate recovery, and explicitly defines no Migration Job aggregate/events. | D01-004 may consume successful authoritative per-identity outcomes for later account provisioning but shall not create a migration business lifecycle. |
| L4 | `L4-A-005_Duplicate_Person_Identification_to_Canonical_Identity_Reconciliation.md` | v1.0.1 / Architecture Approved | `L4-A-005-BH-004`–`BH-008`; §10 | Person merge preserves canonical survivor/history but associated Identity effects and downstream-reference transfer remain unresolved. | D01-004 shall not automatically rebind/provision/disable accounts on `PersonMerged` where the required Identity/account consequence is undefined. |
| L4 | `L4-A-006_Erroneous_Person_Consolidation_to_Restored_Identity_Separation.md` | v1.0.1 / Architecture Approved | `L4-A-006-BH-004`–`BH-007`; §10 | Person split preserves identifiers/history but exact Person/Identity lifecycle effects and downstream redistribution contracts remain unresolved. | D01-004 blocks automatic account/principal reassociation until an authoritative Identity/account consequence exists. |
| L2 | `L2-A02_Person_and_Identity_Workflows.md` | v1.0 / Locked | INT-301; Canonical Person–Identity Establishment and Activation Sequencing Decision (2026-08-20); User Account ownership/rules | A02 owns Person, Identity and User Account lifecycle; a User Account may be created only after Person and Identity activation where login capability is required; a Person may remain active without a User Account. | Establishes provisioning prerequisites and ownership boundary. D01-004 realizes technical provisioning but cannot decide that login is required. |
| L2 | `L2-A03_Authentication_Workflows.md` | v1.1.0 / Locked | Out of Scope; Canonical Ownership; Credential/session/recovery sections | A03 owns Credential, Authentication Session, Refresh Token, provider/method, MFA and Authentication policy; A02 retains User Account lifecycle. | D01-004 coordinates A02 account lifecycle with D01-003 mechanisms without transferring Credential/session ownership. |
| ADR | `ADR-PLAT-AUTH-001.md` | Approved | §§2, 3.1, 3.2 | Person is Tenant-scoped; User Account is authentication/runtime account context, not the canonical Role Assignment subject; Authentication remains distinct from Authorization. | Prevents provisioning from creating authorization grants or using User Account as authorization identity truth. |

All materially applicable L4 parents cited above are Architecture Approved under the R3A/R3B governance decisions and current-version revalidation. Approval-level reliance is therefore authorized for the exact cited BHs; ownership remains upstream.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Person, Identity and User Account lifecycle truth remains A02-owned. | L2-A02 INT-301; L4-A-053 | D01-004 coordinates technical consequences but never writes an alternative canonical identity/account lifecycle. |
| C-002 | User Account creation may occur only after Person and Identity activation and only where login capability is authoritatively required. | L2-A02 2026-08-20 sequencing decision | Provisioning has two distinct gates: activated canonical identity and an authoritative login-required basis. |
| C-003 | `IdentityActivated` or `PersonActivated` alone does not prove that a User Account shall be created. | L2-A02 sequencing: Person may remain active without User Account; L4-A-004 explicitly excludes User Account creation | Lifecycle events may make provisioning *eligible* but do not independently authorize provisioning. |
| C-004 | Account status/eligibility mapping shall remain source-governed. | `L4-A-007-BH-002`; D01-002 | D01-004 shall not infer `Active`, `Enabled` or equivalent technical/business state mappings not explicitly defined. |
| C-005 | Recovery shall not alter Person, Identity or User Account lifecycle merely because Credential recovery succeeds. | `L4-A-008-BH-004` | Recovery coordination delegates Credential/session consequences to D01-003 and preserves upstream account/identity state. |
| C-006 | Covered previous Credentials must cease authenticating and policy-required session consequences must complete before recovery can be asserted complete. | `L4-A-008-BH-006/BH-007`; D01-003 | D01-004 owns the completion barrier across recovery verification, Credential cutover and required session consequence. |
| C-007 | Identity/account closure must make login unavailable and prevent authentication; required session termination is a completion dependency. | `L4-A-053-BH-002/BH-004/BH-009` | Disablement is fail-safe and coordinated with D01-003. |
| C-008 | Stable historical identity references shall survive archival/closure where upstream requires them. | `L4-A-053-BH-004/BH-008`; A02 | Deprovisioning shall not delete or rewrite identity references merely to remove access. |
| C-009 | Bulk onboarding uses normal Person/Identity lifecycles and independent per-record outcomes; no Migration Job aggregate/event is canonical. | `L4-A-054-BH-006`–`BH-010` | D01-004 may coordinate technical account provisioning per accepted identity, but must not create a business migration state machine. |
| C-010 | Merge/split Identity effects are unresolved upstream. | L4-A-005 §10; L4-A-006 §10 | `PersonMerged`/`PersonSplit` shall not trigger automatic account rebinding or lifecycle mutation unless an authoritative Identity/account consequence exists. |
| C-011 | Every lifecycle synchronization is Tenant-bound. | L2-A01; L4-A-004/A054/A053 | Cross-Tenant account/principal reassociation is prohibited. |
| C-012 | Technical lifecycle processing shall be retry/resume safe without creating duplicate User Accounts or replaying already-completed security consequences. | Defensible L5 derivation from C-001/C-002/C-006/C-007 plus frozen D01-004 responsibility | D01-004 requires durable correlation and idempotent application at its owned technical boundary, without inventing transport keys or retry counts. |
| C-013 | Product, workflow-engine, transport, persistence and numeric policy choices require governed selection. | L5 Template v1.0.3; L5 Technology Selection & ADR Methodology v1.0.1 | D01-004 remains technology-neutral and allocates no ADR number. |

### 3.3 Assumptions

No material business or technology assumption is promoted to fact. Missing User Account provisioning trigger/authority, merge/split Identity effects and exact unresolved Credential recovery semantics remain explicit open items in Section 9.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The D01 technical lifecycle synchronization architecture connecting authoritative identity/account facts to IAM technical consequences.
- The post-activation User Account provisioning gate and provisioning coordination mechanism.
- Technical enforcement that provisioning requires both activated canonical identity context and an authoritative login-required/provisioning basis.
- Technical duplicate/replay protection for provisioning and lifecycle synchronization.
- Coordination of User Account provisioning outcome with D01-002 principal binding and D01-003 authentication capability preparation.
- Recovery orchestration across authoritative identity/account context, D01-003 recovery mechanisms, Credential cutover and policy-required session consequences.
- Disablement/deprovisioning coordination when authoritative identity/account lifecycle facts remove login eligibility.
- Completion barriers proving required authentication consequences have completed before lifecycle synchronization is reported complete.
- Lifecycle exception/reconciliation handling for contradictory, stale, incomplete or currently undefined upstream consequences.
- Bulk/onboarding account-provisioning handoff using per-identity outcomes without inventing migration business semantics.
- Generic lifecycle coordination boundary for D01-002 non-human principals once authoritative provisioning/lifecycle basis exists.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-D-01-001` | IAM Architecture Baseline | Owns D01 family boundaries/invariants. | D01-004 must preserve identity/account ownership and authentication/authorization separation. |
| Review `L5-D-01-002` | Identity Account & Service Identity Architecture | Owns human User Account principal binding and service/workload/job/integration principal records. | D01-004 provisions/synchronizes lifecycle against those technical identities without redefining them. |
| Review `L5-D-01-003` | Authentication, Session & Credential Architecture | Owns Credential/factor/provider/session mechanisms, Credential cutover and session invalidation controls. | D01-004 orchestrates recovery/closure consequences by invoking D01-003-owned mechanisms. |
| `L5-D-02-*` | Authorization architecture | Owns Roles, Permissions, policy decision/enforcement and authorization scope. | Provisioning or account activation shall never create authorization grants. |
| `L5-A-04-001` / `L5-A-04-002` | Tenant/Institution isolation and context propagation | Owns technical isolation/context mechanisms. | Lifecycle processing must remain inside one Tenant and preserve context. |
| `L5-D-03-*` | Security, cryptography, keys, secrets and machine credentials | Owns protection of secret/cryptographic material. | Recovery/provisioning may need protected Credential or machine-credential capability without D01-004 owning secret internals. |
| `L5-D-04-*` | Audit/evidence architecture | Owns immutable evidence/provenance model and storage. | D01-004 supplies lifecycle coordination evidence and correlations. |
| `L5-B-02-*` | Persistence/ownership/transaction/consistency architecture | Owns concrete persistence and concurrency mechanisms. | D01-004 requires durable, retry-safe coordination state without selecting persistence technology. |
| `L5-C-01-*` / `L5-C-02-*` | Synchronous contracts and event/messaging architecture | Own transport and delivery mechanics. | Authoritative lifecycle facts/instructions and consequence results cross governed boundaries without direct persistence access. |
| `L5-A-05-*` | Workflow & Orchestration Architecture | Owns reusable workflow/orchestration platform mechanisms where selected. | D01-004 defines IAM lifecycle coordination semantics; A05 may realize reusable orchestration mechanics after its dependency gate is satisfied. |
| `L5-E-01-*` | Availability/resilience and recovery | Owns generic fault recovery/resilience architecture. | Lifecycle coordination must resume safely after technical interruption. |
| `L5-E-05-*` | Observability/operations | Owns operational telemetry. | D01-004 exposes technical health/failure without replacing D04 evidence. |

### Must Not Own

- Canonical Person, Identity or User Account business lifecycle state or authority.
- The business decision that login capability is required.
- Authentication Credential/session/factor/provider internals.
- Authorization grants or Role/Permission assignment.
- Person merge/split business semantics or undefined Identity consequences.
- Migration business aggregate/lifecycle/events.
- Secret/cryptographic custody.
- Audit store.
- Cross-domain persistence.
- Transport/deployment technology.
- L6 administrative/recovery screens.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| Lifecycle Synchronization Intake | Control Point | Accepts only attributable authoritative lifecycle facts/instructions, resolves Tenant/principal/account target and classifies the requested technical consequence without treating event arrival as business authority. | Tenant; Person/Identity/User Account or non-human principal refs; authoritative source fact/instruction reference; requested consequence category; correlation. | Durable processing/correlation metadata only; no duplicate upstream business lifecycle. | Validated lifecycle work context or blocked/rejected result. | D01-002; A04; C01/C02; B02 | Unattributed, cross-Tenant, contradictory or non-authoritative input cannot trigger lifecycle mutation. | C-001/C-003/C-010/C-011. |
| Account Provisioning Gate | Control Point | Determines whether technical User Account provisioning may proceed by requiring authoritative active Person/Identity prerequisites and a separately authoritative basis that login capability/account creation is required. | Tenant; Person/Identity refs and source states; login-required/provisioning authority reference; existing User Account reference if any. | Gate result/correlation; no A02 lifecycle ownership. | Provisioning-authorized context or no-op/blocked result. | A02; D01-002; B02 | `IdentityActivated` alone, missing login-required authority, inactive identity or contradictory existing account blocks creation. | L2-A02 sequencing; L4-A-004 boundary; C-002/C-003. |
| User Account Provisioning Adapter | Adapter / Technical Executor | Realizes the technical create/bind/update invocation against the A02-owned User Account lifecycle boundary using only source-authorized operation semantics. | Provisioning-authorized context; source-governed account operation; Tenant/Identity refs. | Technical request/result correlation; does not become User Account system of record. | Authoritative User Account result/reference returned from A02 boundary. | C01/C02; B02; A02 implementation owner | Partial/failed account creation is not represented as successful provisioning; no direct write into A02 persistence. | Frozen D01-004 responsibility; C-001/C-002. |
| Principal Binding Synchronizer | Adapter / Control Point | Synchronizes D01-002 human principal binding to the authoritative User Account reference/result without copying account lifecycle truth. | D01-002 principal ref; authoritative User Account ref/eligibility; Tenant/Identity refs. | D01-002 binding/reference metadata. | Updated principal context eligible for later Authentication when source status permits. | D01-002; B02 | Stale/mismatched/cross-Tenant account binding is rejected; binding shall not create an account substitute. | D01-002 ownership; C-004/C-011. |
| Authentication Capability Provisioning Handoff | Coordination Boundary | Requests D01-003-owned Credential/factor/session-preparation consequences only when source policy requires authentication capability preparation after account provisioning. | User Account/principal refs; governed Authentication/recovery policy basis; requested D01-003 capability. | D01-004 coordination result references only. | D01-003 Credential/factor capability result or explicit incomplete/failed consequence. | D01-003; D03 | D01-004 shall not directly create Credential/factor/session state or claim authentication readiness before required D01-003 result. | L2-A02 INT-301; D01-003. |
| Recovery Orchestrator | Component / Coordinator | Coordinates governed recovery from resolved Tenant/Identity/User Account through permitted recovery method verification, D01-003 replacement-Credential preparation/cutover and any required session consequence. | Recovery request basis; Tenant/Identity/User Account; applicable recovery/Auth policy; correlation. | Durable step/outcome references sufficient to resume/reconcile; no secret values. | Restored-authentication-capability result only when all source-required consequences complete; otherwise explicit incomplete/failed result. | D01-003; D03; D04; B02 | Failed verification, invalid account, incomplete Credential cutover or required session consequence prevents completion. | `L4-A-008-BH-001`–`BH-008`; C-005/C-006. |
| Login Eligibility Synchronizer | Control Point | Propagates authoritative account/identity eligibility consequences into D01 technical principal/authentication gates without inventing status equivalence. | Authoritative A02 lifecycle/eligibility fact; Tenant/principal/account refs. | Source reference and technical eligibility projection/reference only. | Updated D01-002 eligibility context; D01-003 prevent/permit-authentication input according to source fact. | D01-002; D01-003; B02/C02 | Unknown/ambiguous eligibility fails closed for success paths; local state may not override source. | L4-A-007-BH-002; C-004. |
| Disablement & Closure Coordinator | Component / Coordinator | Coordinates authentication-side consequences of authoritative inactivation/archive/closure: block new authentication, request session termination, verify required consequences and return completion/incomplete status. | Authoritative closure/disablement basis; Tenant; Person/Identity/User Account refs; affected principal refs. | Durable consequence correlation/evidence; no upstream lifecycle mutation. | D01-002 eligibility update; D01-003 session termination/authentication prevention result; lifecycle consequence result to source owner. | D01-002/003; B02; D04; C01/C02 | Incomplete required session termination or unresolved target prevents a false completed-closure result. | `L4-A-053-BH-002/BH-004/BH-009`; C-007/C-008. |
| Lifecycle Reconciliation Control | Control Point | Detects stale, duplicate, out-of-order or contradictory lifecycle inputs; compares current authoritative source state before replaying a technical consequence and routes unresolved semantic conflicts to governance/manual resolution. | Incoming lifecycle work; previously processed correlation; current authoritative refs/state/version where exposed by source. | Processing history/correlation only. | No-op, safe replay/resume, or blocked reconciliation finding. | B02; C01/C02; D04 | Shall not use stale events to reactivate disabled access, duplicate accounts or undo later authoritative closure. | C-012; D01-003 stale-state rules. |
| Bulk Onboarding Provisioning Handoff | Coordinator / Adapter | Consumes per-record accepted canonical identity outcomes from A054 and evaluates each independently for later account provisioning eligibility; does not own source migration/onboarding business state. | Per-record Tenant/Person/Identity outcome; legacy mapping reference; provisioning/login-required authority if available. | Technical per-record correlation/result. | Individual Account Provisioning Gate invocation, link/no-op, or evidenced blocked/rejected outcome. | A02; D01-002; D04; B02 | No job-level invented business status/event; failed records do not corrupt successful records. | `L4-A-054-BH-006`–`BH-010`; C-009. |
| Merge/Split Lifecycle Hold | Safety Control Point | Prevents automatic account/principal rebinding, duplicate suppression, disablement or reassociation when a Person merge/split arrives without authoritative Identity/User Account consequences. | `PersonMerged`/`PersonSplit` context; associated principal/account refs; current authoritative Identity/account consequence if any. | Block/hold/reconciliation reference only. | Proceed only when authoritative Identity/account effect exists; otherwise explicit blocked lifecycle item. | A02; D01-002; D04 | Guessing Identity merge/split, transferring account ownership or rewriting account bindings is prohibited. | L4-A-005 §10; L4-A-006 §10; C-010. |
| Non-Human Lifecycle Coordination Boundary | Control Point | Applies the same source-authority, Tenant, idempotency and evidence principles to D01-002 service/workload/job/integration principals once an authoritative technical provisioning/lifecycle basis is available. | Tenant; non-human principal ref; accountable technical owner; approved provisioning/lifecycle instruction. | D01-004 coordination metadata only; non-human lifecycle semantics are governed by `ADR-L5-0048`. | D01-002 principal availability/binding update and D01-003 machine-auth capability handoff where authorized. | D01-002; D01-003; D03 | No invented non-human states/events/owner; no human Person/User Account fabrication for machine identities. | Frozen D01-002/D01-004 responsibilities; `OD-007`. |
| Lifecycle Evidence Handoff | Contract Boundary | Supplies non-secret provisioning/recovery/disablement/reconciliation facts, actors/source authority references, step outcomes and correlation to D04. | Lifecycle processing context and outcomes. | No immutable audit store; no Credential secrets. | D04 evidence/provenance input and E05 operational signal where applicable. | D04; E05 | Evidence failure shall not be hidden where evidence is required to assert completion. | L4-A-008-BH-008; L4-A-053 §7; L4-A-054 §7. |

The elements above are logical architecture boundaries. They do not prescribe service/process/container decomposition.

### 5.2 Primary Technical Flows

#### A. Post-Activation User Account Provisioning

```text
Authoritative A02 Person + Identity reach source-governed active outcome
        ↓
Immutable canonical references available
        ↓
Lifecycle Synchronization Intake
        ↓
Is there an authoritative login-required / account-provisioning basis?
        ├─ No → no User Account provisioning
        │       (active Person may legitimately remain without User Account)
        └─ Yes
             ↓
Account Provisioning Gate
             ↓
Tenant / Person / Identity prerequisites valid?
Existing User Account already satisfies requested outcome?
             ├─ Invalid / contradictory → blocked reconciliation
             ├─ Already satisfied → idempotent no-op / binding reconciliation
             └─ Provisioning required
                    ↓
User Account Provisioning Adapter
                    ↓
A02-owned User Account lifecycle operation
                    ↓
Authoritative User Account reference/result
                    ↓
D01-002 Principal Binding Synchronizer
                    ↓
Authentication capability preparation required by source policy?
                    ├─ No → provisioning technical outcome complete
                    └─ Yes → D01-003 handoff
                                  ↓
                           D01-003 result
                                  ↓
                         lifecycle evidence/result
```

No automatic account creation is inferred solely from `PersonActivated` or `IdentityActivated`.

#### B. Governed Authentication Recovery

```text
Governed recovery request
        ↓
Lifecycle Synchronization Intake
        ↓
Resolve exactly one Tenant + canonical Identity + eligible User Account
        ↓
Recovery Orchestrator
        ↓
Select only source-permitted recovery method / Authentication policy
        ↓
D01-003 recovery verification
        ├─ fails → recovery fails; upstream account/identity unchanged
        └─ succeeds
             ↓
D01-003 replacement Credential preparation + policy validation
             ↓
Replacement becomes source-eligible for authentication?
        ├─ No → incomplete / failed recovery
        └─ Yes
             ↓
D01-003 Credential cutover:
covered previous Credential(s) cannot authenticate
             ↓
Policy requires existing-session consequence?
        ├─ Yes → D01-003 session invalidation/termination
        │          ↓
        │      consequence complete?
        │          ├─ No → recovery incomplete
        │          └─ Yes
        └─ No
             ↓
Recovery Orchestrator verifies all required consequences
             ↓
Restored-authentication-capability result + D04 evidence
```

D01-004 does not create a new authenticated session as part of recovery; later authentication remains D01-003/A007 behavior.

#### C. Authoritative Identity/Account Disablement and Closure

```text
A02-authoritative inactivation / archival / account-disablement basis
        ↓
Lifecycle Synchronization Intake
        ↓
Resolve affected Tenant + Person/Identity/User Account + D01 principal binding
        ↓
Login Eligibility Synchronizer
        ↓
Prevent new successful authentication
        ↓
Disablement & Closure Coordinator
        ↓
D01-003 terminate/invalidate active Authentication Sessions where required
        ↓
Required session consequence complete?
        ├─ No → return incomplete lifecycle consequence
        │       do not assert technical closure complete
        └─ Yes
             ↓
D01-002 technical principal/account binding reflects non-eligible source context
             ↓
Preserve stable historical references
             ↓
Return completed IAM-side lifecycle consequence to authoritative source
```

D01-004 never performs the Person/Identity lifecycle transition itself.

#### D. Bulk / Existing-Population Onboarding Handoff

```text
A054 governed onboarding
        ↓
per-record canonical outcome
  ├─ rejected / unresolved → no D01 account provisioning
  ├─ existing canonical match → evaluate only source-authorized account need
  └─ new canonical Person/Identity established through normal lifecycle
                           ↓
                identity becomes eligible input
                           ↓
        authoritative login-required basis exists?
             ├─ No → no account provisioning
             └─ Yes → Account Provisioning Gate
                           ↓
                  normal D01-004 flow
```

D01-004 records per-identity technical outcomes. It does not introduce `MigrationJobStarted`, `MigrationJobCompleted`, `MigrationJobFailed` or any equivalent canonical business event/state.

#### E. Merge / Split Safety Flow

```text
PersonMerged or PersonSplit observed
        ↓
Lifecycle Synchronization Intake
        ↓
Is authoritative Identity/User Account consequence explicitly defined?
        ├─ No → Merge/Split Lifecycle Hold
        │       preserve current safe access state
        │       raise blocked reconciliation
        └─ Yes
             ↓
Validate same-Tenant target and authoritative consequence
             ↓
Apply only that governed D01 binding/provisioning/disablement consequence
             ↓
Evidence + correlation
```

D01-004 does not derive `IdentityMerged`, `IdentitySplit`, account transfer or credential reassignment from Person merge/split alone.

#### F. Non-Human Principal Provisioning / Disablement

```text
Approved technical owner / provisioning authority
        ↓
Tenant-bound non-human principal basis
        ↓
Lifecycle Synchronization Intake
        ↓
D01-002 non-human principal exists / may be provisioned under governed semantics?
        ├─ No / lifecycle preconditions fail → blocked / remain non-active
        └─ Yes
             ↓
Apply authorized D01-002 principal lifecycle/binding consequence
             ↓
Authentication capability required?
        ├─ No
        └─ Yes → D01-003 machine-auth / credential handoff
             ↓
Evidence / completion result
```

No Person or human User Account is fabricated for a machine principal.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| No authoritative account-provisioning basis after identity activation | Account Provisioning Gate | Do not create a User Account; retain eligible identity references only. | Auto-provisioning because `IdentityActivated` was observed. | Upstream A02/L4 governance resolves trigger/authority. | C-002/C-003; `OD-002`. |
| Cross-Tenant or unresolved principal/account target | Lifecycle Intake / Provisioning Gate | Reject/block lifecycle processing. | Guessing Tenant or moving/rebinding identity across Tenant. | A02/A04 resolution. | C-011. |
| Duplicate/replayed provisioning input | Reconciliation Control | Resolve current authoritative account state; perform no duplicate creation; safely resume incomplete work if needed. | Multiple User Accounts or repeated irreversible consequences from one governed requirement. | B02/C01/C02. | C-012. |
| A02 account creation fails/partially completes | User Account Provisioning Adapter | Preserve explicit incomplete result; reconcile against authoritative A02 state before retry. | Claiming provisioning success or independently creating a local account record. | A02 owner; B02/E01. | C-001/C-012. |
| D01-002 principal binding cannot synchronize to authoritative User Account | Principal Binding Synchronizer | Prevent claim that IAM provisioning is technically complete; retain authoritative account reference for reconciliation. | Replacing source User Account with a D01 local surrogate. | D01-002/B02. | C-001/C-004. |
| Required D01-003 authentication capability preparation fails | Authentication Capability Handoff | Provisioning may record A02 account creation but must not claim authentication capability ready if source requires the failed consequence. | Fabricating Credential/factor/session state inside D01-004. | D01-003/D03. | INT-301; D01-003. |
| Recovery verification fails | Recovery Orchestrator | Stop recovery without changing Person/Identity/User Account lifecycle or exposing replacement Credential as usable. | Account/identity activation or reset by technical recovery shortcut. | D01-003 / upstream recovery authority. | `L4-A-008-BH-003/BH-004`. |
| Old covered Credential remains usable | Recovery completion barrier | Treat recovery as incomplete and block completed result. | Declaring restored authentication while old Credential still authenticates. | D01-003/B02. | `L4-A-008-BH-006`. |
| Policy-required session consequence incomplete | Recovery/Closure Coordinator | Return incomplete consequence; continue/reconcile without false completion. | Claiming recovery/closure complete while required sessions remain active. | D01-003/E01/B02. | `L4-A-008-BH-007`; `L4-A-053-BH-009`. |
| Stale lifecycle signal arrives after later authoritative closure | Reconciliation Control | Compare current authoritative eligibility; suppress stale reactivation/provisioning effect. | Re-enabling login or recreating access from historical event ordering. | C02/B02/A02 source. | C-012; fail-safe lifecycle integrity. |
| Person merge/split received without authoritative Identity/account consequence | Merge/Split Lifecycle Hold | Block automatic account/principal reassociation and expose unresolved lifecycle item. | Inventing Identity merge/split, transferring User Account or Credential association. | Upstream A02/L4 governance. | C-010; `OD-004`. |
| Bulk onboarding has mixed successful/failed records | Bulk Handoff | Preserve independent technical outcomes and continue only for source-approved records. | All-or-nothing invented migration rollback or job-level canonical state. | A054 source; B02/E01. | `L4-A-054-BH-009/BH-010`. |
| Evidence/correlation unavailable where completion requires evidence | Evidence Handoff | Do not hide the evidence gap; completion status remains subject to D04/governed evidence requirement. | Unattributable provisioning/recovery/disablement completion. | D04. | L4-A008/A053/A054 evidence obligations. |

### 5.4 Conceptual Contracts / Context

These structures define semantic information categories only. Exact APIs, message schemas and field names remain C01/C02 implementation concerns.

```text
LifecycleSynchronizationContext
{
  resolved Tenant reference
  target principal / Person / Identity / User Account reference(s)
  authoritative source fact or governed instruction reference
  requested IAM technical consequence category
  source authority / policy basis where required
  correlation / evidence reference
}
```

```text
AccountProvisioningEligibilityContext
{
  Tenant reference
  active Person reference
  active Identity reference
  authoritative login-required / provisioning basis
  existing User Account reference if any
  source-governed eligibility reference
  correlation reference
}
```

The presence of an active Identity without the separate login-required/provisioning basis is insufficient to authorize account creation.

```text
LifecycleConsequenceRequest
{
  Tenant reference
  principal/account target reference
  authoritative lifecycle basis
  required D01 consequence(s)
  source policy reference where consequence is conditional
  correlation reference
}
```

```text
LifecycleConsequenceResult
{
  Tenant reference
  target reference
  requested consequence reference
  per-consequence outcome references
  incomplete / blocked dependency reference where applicable
  authoritative source recheck reference where used
  evidence / correlation reference
}
```

No new canonical Person/Identity/User Account lifecycle state is defined by `LifecycleConsequenceResult`.

```text
RecoveryCoordinationContext
{
  Tenant reference
  canonical Identity reference
  eligible User Account reference
  governed recovery request / method / policy reference
  D01-003 verification and Credential consequence references
  session consequence reference when policy requires
  correlation / evidence reference
}
```

```text
BulkProvisioningRecordContext
{
  Tenant reference
  source/onboarding record correlation
  canonical Person / Identity result reference
  legacy mapping reference where applicable
  authoritative account-provisioning basis if any
  per-record technical outcome reference
}
```

There is intentionally no canonical Migration Job state or event contract.

### 5.5 State / Persistence / Consistency

- D01-004 owns **technical coordination state**, correlation, deduplication evidence and consequence progress sufficient to resume/reconcile provisioning, recovery and disablement work. It does not own Person, Identity or User Account business state.
- A02 remains authoritative for User Account existence, identity association, account status and lifecycle. D01-004 shall re-read/re-resolve authoritative state when retrying after uncertain partial completion rather than assuming the previous request failed.
- D01-002 remains authoritative within L5 for its technical principal/binding records; D01-003 remains authoritative within L5 for Authentication-owned Credential/session mechanisms.
- Cross-owner lifecycle coordination shall not depend on a single shared database transaction across A02, D01-002 and D01-003. D01-004 shall tolerate partial completion, record attributable consequence outcomes and reconcile safely.
- Technical lifecycle operations shall be idempotent at the D01-004 responsibility boundary: replaying the same authoritative requirement shall not create a second User Account, duplicate principal binding, repeated Credential cutover or unintended session resurrection.
- Exact idempotency-key schema, transaction isolation level, locking mechanism, database, journal/store or retry algorithm is not defined here.
- Lifecycle synchronization shall be monotonic with respect to current authoritative eligibility: stale historical inputs shall not override a later authoritative closure/disablement fact.
- Stable historical Person/Identity/User Account references required by upstream architecture shall not be physically erased merely to represent access disablement.
- Exact retention periods, archival storage technology and physical deletion mechanisms remain outside D01-004.

### 5.6 Asynchronous / Messaging Behavior

- D01-004 may receive authoritative lifecycle facts/instructions through synchronous or asynchronous governed boundaries; C01/C02 own transport realization.
- Event observation does not create business authority. The intake boundary validates the semantic source and current authoritative target state before applying a material lifecycle consequence.
- Duplicate delivery shall be safe under §5.5 idempotency requirements.
- Out-of-order delivery shall not permit stale activation/provisioning to override later disablement/closure.
- Canonical upstream events retain their canonical publisher. D01-004 shall not rename `PersonActivated`, `IdentityActivated`, `PersonMerged`, `PersonSplit`, `PersonInactivated`, `PersonArchived`, `IdentityDisabled`, `IdentityArchived` or other upstream facts.
- D01-004 shall not create account-provisioning, migration-job, merge/split or recovery canonical business event names merely to simplify implementation.
- Where D01-004 needs internal technical work signals, their representation remains non-canonical implementation detail unless separately governed.

### 5.7 Security / Trust / Isolation

- Lifecycle work is Tenant-bound from intake through final consequence.
- Provisioning requires positive authoritative eligibility; absence/ambiguity fails closed.
- Recovery shall never bypass source account eligibility, Authentication Policy or D01-003 Credential/factor verification.
- Disablement/closure has security priority over stale provisioning/recovery work. A later authoritative non-eligible/closed state prevents reactivation by earlier in-flight work.
- User Account provisioning shall not create Role/Permission assignments or any Authorization Decision.
- Recovery evidence shall not include Credential/factor secret values.
- Non-human identity lifecycle shall not be represented by a human Person/User Account merely for implementation convenience.
- Merge/split ambiguity is treated as a security-sensitive identity-binding conflict and is blocked rather than guessed.
- Historical identity references retained for audit/business traceability shall not themselves permit authentication after source-governed closure.
- D01-004 does not select cryptographic or secret-protection mechanisms; D03 controls those boundaries.

### 5.8 Runtime / Deployment / Operations

D01-004 defines logical coordination/control boundaries, not deployment units.

Any runtime realization shall preserve:

- source-authority validation;
- Tenant isolation;
- durable retry/resume safety;
- duplicate/replay suppression;
- stale-event protection;
- explicit partial-completion visibility;
- separation of A02 business lifecycle, D01-002 principal identity and D01-003 Authentication mechanisms;
- fail-safe disablement;
- non-secret observability;
- no direct cross-owner persistence modification.

Whether these controls run in one application process, separate execution units or a reusable A05 orchestration runtime is governed elsewhere.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| AD-001 | Source-authority-driven lifecycle synchronization | D01-004 applies material provisioning/recovery/disablement consequences only from attributable authoritative facts/instructions plus current source eligibility, not from technical observation alone. | C-001/C-003/C-004; L4-A-004/A053 | Prevents technical components/events from becoming business lifecycle authority. | None — direct ownership-preserving architecture. |
| AD-002 | Two-gate User Account provisioning | Technical account provisioning requires both activated canonical Person/Identity prerequisites and a separately authoritative login-required/provisioning basis. | L2-A02 2026-08-20 sequencing; L4-A-004 boundary | A Person may remain active without a User Account; activation alone is insufficient. | None; exact business trigger remains `OD-002`. |
| AD-003 | Retry/resume-safe lifecycle coordinator | D01-004 records durable correlation/outcomes and reconciles authoritative source state so duplicate or interrupted processing cannot duplicate accounts or replay security consequences. | Frozen D01-004 responsibility; C-012 | Lifecycle work spans multiple owners and cannot safely assume all-or-nothing execution. | None for local semantic architecture; technology selection remains governed. |
| AD-004 | A02 remains account system of record | User Account provisioning executes through a governed A02 boundary and returns authoritative account result/reference; D01-004 owns only technical coordination. | INT-301; C-001 | Prevents duplicate User Account ownership in IAM technical stores. | None. |
| AD-005 | Recovery completion barrier | D01-004 reports recovery complete only after governed verification, replacement Credential eligibility, prior-Credential disablement and any policy-required session consequence complete. | `L4-A-008-BH-001`–`BH-008`; D01-003 AD-007/008 | Makes recovery outcome verifiable across owner boundaries without absorbing Credential/session internals. | None. |
| AD-006 | Fail-safe disablement | Authoritative non-eligibility/closure blocks new authentication before D01-004 may report lifecycle consequence complete; required active-session termination is explicitly verified. | `L4-A-053-BH-002/BH-004/BH-009` | Prevents stale technical access from surviving authoritative closure. | None. |
| AD-007 | Merge/split automatic synchronization is prohibited while Identity/account effects are undefined | Person merge/split enters lifecycle hold unless an explicit authoritative Identity/User Account consequence is supplied. | L4-A-005 §10; L4-A-006 §10 | Avoids inventing identity transfer/rebinding semantics. | None — upstream blocker. |
| AD-008 | Bulk onboarding stays per-identity, not job-canonical | A054 accepted identity outcomes may independently enter normal account-provisioning flow; D01-004 creates no Migration Job business lifecycle/events. | `L4-A-054-BH-006`–`BH-010` | Preserves A054 ownership and allows partial success without invented business semantics. | None. |
| AD-009 | Human and non-human lifecycle coordination share control invariants while non-human states/events are governed by ADR-L5-0048 | Non-human principals use the same Tenant/source-authority/idempotency/evidence controls and the canonical D01 lifecycle `Provisioned → Active ↔ Suspended → Retired`. | D01-002 responsibility; `ADR-L5-0048`; frozen D01-004 responsibility | Reuses safe technical coordination without fabricating human account semantics for machines. | None. |
| AD-010 | No cross-owner shared-write shortcut | D01-004 never directly updates A02, D01-002 or D01-003 internal persistence outside their governed boundaries, including during recovery/closure. | L5 Validation HF-07; canonical ownership | Preserves owner boundaries through partial failure and future implementation changes. | None. |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Identity integrity | Lifecycle processing cannot manufacture or transfer Person/Identity/User Account ownership. | INT-301; AD-001/004 | D01-004 + A02 |
| Provisioning correctness | Account creation requires activated identity plus authoritative login-required basis. | L2-A02 sequencing; AD-002 | D01-004 |
| Tenant isolation | Every lifecycle target and consequence remains inside one Tenant. | L2-A01; C-011 | `L5-A-04-*` |
| Recovery security | Recovery completion requires all source-required Credential/session consequences. | L4-A-008; AD-005 | D01-004 + D01-003 |
| Closure security | New authentication is fail-safe blocked; required session termination must complete. | L4-A-053; AD-006 | D01-004 + D01-003 |
| Replay/stale-input safety | Lifecycle coordinator reconciles current authority and prevents duplicate/reversed consequences. | AD-003; §5.5 | D01-004 + B02/C02 |
| Privacy/history | Closure/deprovisioning preserves stable references and governed history; technical disablement is not destructive erasure. | L4-A-053 | A02 + D01-004 + D04 |
| Merge/split safety | Undefined Identity effects produce blocked reconciliation, not guessed rebinding. | L4-A-005/A006 | A02 + D01-004 |
| Bulk onboarding integrity | Per-record outcomes remain independent; no migration business lifecycle is invented. | L4-A-054 | A02 + D01-004 |
| Auditability | Every material provisioning/recovery/disablement result is attributable without secret disclosure. | L4-A008/A053/A054 | `L5-D-04-*` |
| Operational resilience | Partial work remains recoverable/resumable without false completion. | AD-003 | B02/E01/E05 |

No numeric availability, retry, timeout, TTL, retention, throughput, lockout or recovery target is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| CR-001 | D01-004 shall not create, change or reinterpret canonical Person, Identity or User Account business lifecycle state outside an A02-governed boundary. | AD-001/AD-004 | INT-301; L4-A-053 |
| CR-002 | User Account provisioning shall require a resolved single Tenant, active canonical Person/Identity prerequisites and an authoritative login-required/provisioning basis. | AD-002; §5.2A | L2-A02 sequencing; L4-A-004 boundary |
| CR-003 | `PersonActivated` or `IdentityActivated` alone shall not automatically trigger User Account creation. | AD-002 | L2-A02 explicitly allows active Person without User Account |
| CR-004 | Replayed or duplicate provisioning requirements shall not create more than the one authoritative User Account outcome permitted by A02. | AD-003; §5.5 | C-012; canonical ownership |
| CR-005 | After uncertain/partial A02 account provisioning, D01-004 shall reconcile against authoritative A02 state before retrying rather than assume the prior request failed. | AD-003/004 | §5.5 |
| CR-006 | D01-004 shall not fabricate a local User Account record when A02 account provisioning fails or is unavailable. | AD-004 | INT-301 |
| CR-007 | D01-004 shall bind a provisioned human principal only to the authoritative A02 User Account reference and shall preserve Tenant/Identity consistency. | §5.1 Principal Binding Synchronizer | D01-002; C-011 |
| CR-008 | Account provisioning shall not create Role, Permission, Delegation or Authorization Decision data. | §4 Must Not Own | ADR-PLAT-AUTH-001; `L5-D-02-*` ownership |
| CR-009 | Credential, MFA and Authentication Session state required during provisioning/recovery shall be produced only through D01-003-owned mechanisms. | §5.1 Authentication Handoff | INT-301; D01-003 |
| CR-010 | Recovery shall not proceed unless Tenant, canonical Identity and eligible User Account context are resolvable and the recovery method is source-permitted. | §5.2B | `L4-A-008-BH-001/BH-002` |
| CR-011 | Failed recovery verification shall not make a replacement Credential available or change Person/Identity/User Account lifecycle state. | AD-005 | `L4-A-008-BH-003/BH-004` |
| CR-012 | D01-004 shall not report recovery complete until the replacement Credential satisfies source eligibility and all previous Credentials covered by the recovery decision are incapable of authentication. | AD-005 | `L4-A-008-BH-005/BH-006` |
| CR-013 | Where Authentication Policy requires an existing-session consequence during recovery, D01-004 shall not report completion until D01-003 returns the required consequence as complete. | AD-005 | `L4-A-008-BH-007` |
| CR-014 | Lifecycle disablement/closure shall prevent new successful authentication according to authoritative source eligibility without D01-004 inventing account-status mappings. | AD-006 | `L4-A-053-BH-002/BH-004`; `L4-A-007-BH-002` |
| CR-015 | Where closure requires active-session termination, incomplete termination shall yield an incomplete lifecycle consequence result rather than a completed IAM closure result. | AD-006 | `L4-A-053-BH-009` |
| CR-016 | Deprovisioning/access closure shall not erase stable Person/Identity references that upstream architecture requires for historical/audit resolution. | §5.5 | `L4-A-053-BH-004/BH-008` |
| CR-017 | Stale or out-of-order activation/provisioning inputs shall not reactivate access after a later authoritative disablement/closure fact. | AD-003/006; §5.6 | C-012 |
| CR-018 | `PersonMerged` or `PersonSplit` without an authoritative Identity/User Account consequence shall not automatically rebind, transfer, provision, disable or delete IAM principal/account associations. | AD-007 | L4-A-005 §10; L4-A-006 §10 |
| CR-019 | Bulk onboarding shall feed D01-004 only through per-record authoritative canonical identity outcomes; no Migration Job business aggregate/state/event shall be introduced by D01-004. | AD-008 | `L4-A-054-BH-006`–`BH-010` |
| CR-020 | Failed bulk records shall not roll back or corrupt independently successful canonical identity/account outcomes unless an explicit upstream corrective workflow requires it. | §5.2D | `L4-A-054-BH-009/BH-010` |
| CR-021 | Non-human lifecycle processing shall use the `ADR-L5-0048` lifecycle, require a Tenant-bound D01-002 principal and accountable technical owner, and shall not create a human Person/User Account surrogate. | AD-009; ADR-L5-0048 | D01-002 / D01-004 responsibility |
| CR-022 | Lifecycle synchronization evidence shall preserve source authority, target references, requested consequence, material outcome and correlation without raw Credential/factor secret values. | §5.4/§5.7 | L4-A008/A053/A054 evidence obligations |
| CR-023 | D01-004 shall not directly modify another L5/A02 owner's internal persistence as a shortcut for provisioning, recovery, disablement or reconciliation. | AD-010 | L5 Validation HF-07 |
| CR-024 | No implementation may infer a specific IAM/directory/provisioning/workflow product, database, broker, cloud service, retry count, timeout or lifecycle numeric target from this specification. | §5.8 | L5 Technology Selection & ADR Methodology |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| OD-001 | Material L4 approval authority was previously Draft. | **Resolved** | R3A/R3B approval decisions; R3D traceability validation | L4/repository governance | **No** | Closed: applicable L4 parents are Architecture Approved. |
| OD-002 | Post-activation User Account provisioning trigger/authority was previously undefined. | **Resolved** | Locked A02 R3B amendment; L3-A-004 v1.1; L4-A-004 v1.0.1 Architecture Approved | A02/L3/L4 governance | **No** | Closed: authoritative login-capability requirement plus User Account Provisioning Authority is required; `IdentityActivated` alone is insufficient. |
| OD-003 | Exact User Account lifecycle transitions/events used for create/enable/disable/lock/archive synchronization are not mapped by a current applicable L4 behavior set in a way sufficient for this specification to canonicalize them. | Upstream traceability | A02 owns Account Status/User Account lifecycle, but L5 must not manufacture missing L4 behavior. | A02/L4 governance | **Yes for any new canonical state/event assertions; No for source-reference-based coordination** | Bind D01-004 to exact approved L4 BHs once authored/approved; until then carry opaque source-governed eligibility/lifecycle references. |
| OD-004 | Person merge/split Identity/User Account consequences were previously unresolved. | **Resolved** | Locked A02 R3B amendment; L3/L4-A-005/A-006 current approved versions | A02/L3/L4 governance | **No** | Closed by explicit per-association disposition and owner-preserving downstream reference reconciliation. |
| OD-005 | Exact coordination ordering among `PersonArchived`, `IdentityDisabled` and `IdentityArchived` is not defined upstream. | Upstream | L4-A-053 `A053-L4-OPEN-02` | A02/L4 governance | **No for fail-safe authentication prevention; Yes for claiming a canonical ordered lifecycle sequence** | Resolve upstream ordering. D01-004 may enforce “no authentication + required session termination” without inventing aggregate transition order. |
| OD-006 | Previous-Credential invalidation and recovery completion semantics were previously unresolved. | **Resolved** | Locked A03 R3B amendment; L3-A-008 v1.1; L4-A-008 v1.0.1 Architecture Approved | A03/L3/L4 governance | **No** | Closed by replacement Credential activation, covered prior-Credential revocation and policy-required session completion barriers. |
| OD-007 | Non-human principal lifecycle semantics were previously unresolved. | Resolved L5-owned architecture decision | `ADR-L5-0048` | D01 / Lead Architect | **No** | Closed by canonical technical-principal states, transitions, lifecycle events and ownership boundaries. |
| OD-008 | D01-002 and D01-003 are now canonical Review peers and have been reconciled to the current IAM/security ADR baseline. | Dependency | Current canonical repository state | Lead Architect / D01 owner | **No standalone blocker after reconciliation** | Continue normal impact review. |
| OD-009 | Core platform technology is governed by the Locked Rust/Axum/Tokio, PostgreSQL/SQLx, NATS JetStream, Flowable, Keycloak and Group-3 security decisions. Exact lifecycle-policy semantics and numeric retry/timeout values remain upstream/profile-governed. | Technology / ADR | `ADR-L5-0019`; `ADR-L5-0020`; `ADR-L5-0022`; `ADR-L5-0023`; `ADR-L5-0024`; `ADR-L5-0038`; `ADR-L5-0039`; `ADR-L5-0040`; `ADR-L5-0041` | Canonical owners | **Core technology resolved; no standalone blocker** | Do not use technical resolution to invent missing account/non-human lifecycle business semantics. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-004-BH-008`–`BH-011` — active canonical identity prerequisite and downstream-reference boundary; User Account creation remains outside A004.
- `L4-A-007-BH-001/BH-002` — Tenant/Identity resolution and source-governed account eligibility for Authentication.
- `L4-A-008-BH-001`–`BH-008` — governed Credential recovery, previous-Credential invalidation, session consequence and evidence.
- `L4-A-053-BH-001/BH-002/BH-004/BH-009` — closure target resolution, login disablement, authentication prevention and session-termination completion barrier.
- `L4-A-054-BH-001`–`BH-010` — governed bulk identity onboarding, normal lifecycle reuse, per-record outcomes and non-invention of Migration Job semantics.
- `L4-A-005-BH-004`–`BH-008` / §10 — Person merge with unresolved Identity effects.
- `L4-A-006-BH-004`–`BH-007` / §10 — Person split with unresolved Identity effects.

### Material Higher-Level / ADR Constraints

- `L2-A02` INT-301 — Person/Identity/User Account lifecycle remains A02-owned.
- `L2-A02` 2026-08-20 sequencing — User Account only after Person/Identity activation and only where login capability is required.
- `L2-A03` — Authentication/Credential/session lifecycle is separate from A02 account lifecycle.
- `ADR-PLAT-AUTH-001` — User Account is authentication/runtime context, not authorization assignment subject.

### Material L5 Dependencies

- `L5-D-01-001` — IAM baseline.
- Review `L5-D-01-002` — principal/account/service-identity models.
- Review `L5-D-01-003` — Authentication, Credential and session mechanisms.
- `L5-D-02-*` — authorization.
- `L5-A-04-*` — Tenant/Institution isolation/context.
- `L5-D-03-*` — security/cryptography/secrets.
- `L5-D-04-*` — audit/evidence.
- `L5-B-02-*` — persistence/consistency/concurrency.
- `L5-C-01-*` / `L5-C-02-*` — governed technical handoffs.
- `L5-A-05-*` — reusable orchestration mechanics where applicable.
- `L5-E-01-*` / `L5-E-05-*` — resilience and operations.

### Architecture Decisions

- `AD-001`–`AD-010`

### Conformance Requirements

- `CR-001`–`CR-024`

### Open / Blocking Items

- `OD-001` — **resolved by R3A/R3B approval + R3D validation**.
- `OD-002` — missing L4 User Account provisioning trigger/authority.
- `OD-003` — exact User Account lifecycle BH mapping incomplete.
- `OD-004` — **resolved by the Locked A02 R3B amendment and approved A005/A006 behavior**.
- `OD-005` — closure aggregate ordering unresolved.
- `OD-006` — **resolved by the Locked A03 R3B amendment and approved A008 behavior**.
- `OD-007` — **resolved by ADR-L5-0048**.
- `OD-008` — D01 peer stabilization reconciled; ordinary impact review remains.
- `OD-009` — core technology resolved; policy/profile values remain governed.


## 2026-09-23 R3C Non-Human Lifecycle Coordination Synchronization

**Applicable locked decision:** `ADR-L5-0048`

D01-004 coordinates non-human principal lifecycle transitions against the D01-002-owned canonical state model:

```text
Provisioned → Active ↔ Suspended → Retired
```

with `Retired` terminal.

Coordination rules:

- provisioning establishes the technical principal and required Tenant/technical-owner context before activation;
- activation requires current authentication/credential readiness and applicable authorization-binding readiness;
- suspension blocks new authenticated execution and coordinates required credential/session consequences;
- resume revalidates current Tenant, owner, authentication and authorization-binding conditions;
- retirement is permanent, preserves history, and coordinates credential/session revocation;
- duplicate/replayed requests are idempotent;
- stale activation cannot override later suspension or retirement;
- D01-004 never fabricates Person/User Account records or grants D02 authority.


## 2026-09-23 R3D Targeted Revalidation

**Review result:** PASS — Architecture Approved.

The R3D review revalidated this specification against the current approved R3A/R3B L4 parent versions, the Locked R3B upstream amendments, applicable Locked/Approved ADRs including `ADR-L5-0048`, and the D01 peer set.

No applicable HF-01…HF-15 failure remains. Exact L4 BH→L5 relations are recorded in the canonical Section-A traceability partition under the R3D validation decision.

This approval does not authorize Final L5 Corpus Lock and does not resolve unrelated intentionally deferred implementation/profile choices.
