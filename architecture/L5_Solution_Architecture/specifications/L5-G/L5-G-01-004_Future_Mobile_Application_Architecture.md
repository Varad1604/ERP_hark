# Future Mobile Application Architecture

## 1. Specification Metadata

**Specification ID:** `L5-G-01-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-G — Client Platform Architecture  
**Major Area:** L5-G-01 — Client Platform Architecture — Web & Future Mobile  
**Canonical Technical Responsibility:** Future mobile technical architecture aligned to shared backend contracts; no offline-first assumption or L6 design.  
**Technical Architecture Owner:** L5-G-01 — Client Platform Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.3`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-G-01-001` v1.0.1 Architecture Approved; `L5-G-01-003` v1.0.3 Architecture Approved; `L5_G01_004_Future_Mobile_Application_Architecture_Approval_Decision_2026-09-15.md`; G01 dependency-readiness decision 2026-09-15; `L5-C-01-001/002/003`; `L5-D-01`; `L5-D-02`; `L5-A-04`; `ADR-L5-0001`; `ADR-L5-0003`; `ADR-L5-0004`.  
**Referenced By:** Future mobile implementation architecture, future L6 mobile UX architecture, mobile release/runtime architecture.  
**Applicable ADRs:** `ADR-L5-0001`, `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0038`. No mobile framework, language, app-shell product, push-provider, local database, secure-storage product or offline-sync platform is selected by this specification.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the future SutraERP mobile-client architecture as a specialization of the approved G01 client platform, preserving the same canonical backend contracts, authentication/authorization separation, Tenant/Institution isolation and non-authoritative client-state model while introducing only the mobile-specific technical adaptation boundaries needed by a native-device application.

### In Scope

- future mobile application shell and lifecycle boundary;
- mobile specialization of G01-003 session/state/contract-consumption controls;
- device/platform adapter boundary;
- secure mobile persistence classification;
- foreground/background transition handling;
- application upgrade/compatibility boundary;
- device capability mediation;
- notification/deep-link ingress as non-authoritative triggers;
- mobile network interruption handling;
- online-first mobile execution;
- framework-selection and future offline-expansion gates.

### Out of Scope / Non-Goals

- mobile screens, navigation taxonomy, journeys, interaction or visual design;
- selection of React Native, Expo, Flutter, Swift, Kotlin or another mobile framework/runtime;
- Android/iOS deployment-store processes;
- mobile-device-management product selection;
- push-notification provider/product selection;
- Authentication Session/token format or credential mechanism — D01;
- authorization policy/decision — D02;
- Tenant/Institution validation authority — A04;
- API protocol/schema/versioning/idempotency — C01;
- canonical notification business semantics — C04;
- offline-first business execution, durable offline mutation queues or disconnected authoritative state.

---

## 3. Canonical Constraints

| Source | Constraint on G01-004 |
|---|---|
| `L5-G-01-001` | Mobile remains a non-authoritative client consuming governed contracts and peer-owned context. |
| `L5-G-01-003` | Client Context Epoch, protected state partition, safe contract client, indeterminate mutation handling and client-security boundaries apply to mobile specialization. |
| `L5-C-01` | Mobile consumes released synchronous contracts; it does not create mobile-only bypass APIs or redefine compatibility/idempotency. |
| `L5-D-01` | Authentication/session authority and credential/token mechanics remain D01-owned. |
| `L5-D-02` | Authorization remains server-authoritative; mobile visibility/state is advisory only. |
| `L5-A-04` | Tenant/Institution context is validated server-side and mobile state must remain scope-isolated. |
| `ADR-L5-0001` | Mobile/device execution is an explicit trust boundary; trust is non-transitive and protected crossings fail closed. |
| Frozen corpus | Future mobile architecture has no offline-first assumption. |

### Architecture Constraints

**C-001 — Same backend contract model.**  
Mobile shall consume the same governed C01 contract architecture as other SutraERP clients. A mobile form factor does not create a separate business API authority.

**C-002 — Non-authoritative mobile state.**  
Device-held business data, capability projections, session projections and scope context remain non-authoritative.

**C-003 — Mobile platform is a distinct trust context.**  
Device security features may strengthen protection but shall not cause the server to trust client assertions as canonical facts.

**C-004 — Online-first execution.**  
Authoritative protected mutations require governed backend interaction. No durable disconnected mutation queue is approved.

**C-005 — Platform lifecycle cannot widen authority.**  
App backgrounding, process death, restore, notification activation, deep link or application upgrade shall not silently reactivate stale protected context.

**C-006 — Framework choice is deferred and ADR-governed.**  
A material mobile framework/runtime selection shall follow L5 technology-selection governance when implementation becomes active.

---

## 4. Technical Ownership & Boundaries

### 4.1 G01-004 Owns

- **Mobile Application Shell** — future mobile runtime composition boundary.
- **Mobile Lifecycle Coordinator** — handles foreground/background/process restore/restart consequences without creating business authority.
- **Mobile Platform Adapter Layer** — isolates OS/device-specific capabilities from G01 shared logic.
- **Mobile Protected State Container** — platform-specific realization of G01-003 protected client-state partitioning.
- **Mobile Secure Persistence Gate** — governs admission of sensitive/protected client material into device storage.
- **Mobile Contract Adapter** — platform realization of the G01 Safe Contract Client.
- **Mobile Ingress Adapter** — normalizes notification/deep-link/device invocation into non-authoritative client intent/context.
- **Mobile Compatibility Boundary** — validates application build/contract compatibility before protected capability execution.

### 4.2 Consumes / Collaborates With

| Owner | Mobile dependency |
|---|---|
| `L5-G-01-001` | Shared client invariants. |
| `L5-G-01-003` | Client security/session/state/safe contract-consumption architecture. |
| `L5-C-01` | Released contracts, errors, compatibility and idempotency. |
| `L5-D-01` | Authenticated/session context. |
| `L5-D-02` | Authorization enforcement. |
| `L5-A-04` | Tenant/Institution scope/isolation. |
| `L5-C-04` | Notification/communication technical delivery when mobile notifications are later realized. |
| `L5-D-03` | Security, secrets and trust-boundary controls. |

### 4.3 Must Not Own

G01-004 shall not own business rules, notification business meaning, canonical mobile-only business state, authorization decisions, Authentication Session lifecycle, API schemas, backend orchestration, persistent server data or L6 UX semantics.

---

## 5. Solution Architecture

### 5.1 Logical Mobile Structure

```text
Mobile OS / Device
        ↓
Mobile Application Shell
        ├── Mobile Lifecycle Coordinator
        ├── G01-003 Client Security & State Coordinator
        │      ├── Session Projection
        │      ├── Client Context Epoch
        │      ├── Protected State Partition
        │      ├── Mutation Coordinator
        │      └── Safe Contract Client
        ├── Mobile Platform Adapter Layer
        │      ├── secure-storage adapter
        │      ├── notification ingress adapter
        │      ├── deep-link / app-link adapter
        │      ├── device capability adapters
        │      └── lifecycle/network adapters
        └── Mobile Feature Modules
                 ↓
              L5-C-01
                 ↓
          Canonical backend owners
```

Mobile-specific components adapt the device/runtime. They do not duplicate backend ownership.

### 5.2 Mobile Application Shell

The shell shall:

- initialize shared G01 client controls;
- establish platform adapters;
- restore only classified client-local state;
- require authoritative revalidation before protected execution when prior session/scope freshness is uncertain;
- prevent direct feature access to ungoverned network/security/storage paths;
- expose device capabilities through controlled adapters rather than feature-specific platform calls where the capability materially affects security or state.

### 5.3 Mobile Lifecycle Coordinator

Mobile operating systems may suspend, terminate or restore application execution outside the application's control.

Therefore, lifecycle transitions are explicit architecture events for client-local state:

```text
Foreground
Background/Suspended
Process Terminated
Cold Start / Restore
```

These are technical runtime conditions only, not business lifecycle states.

On return from background, cold start or restored process state, the application shall revalidate material session/scope/compatibility assumptions before protected mutation execution where freshness cannot be established.

A restored local snapshot shall never itself prove:

- authenticated session validity;
- current Tenant/Institution authority;
- authorization permission;
- current canonical business state.

### 5.4 Mobile Protected State Container

G01-003 state classes remain authoritative for the mobile specialization.

Mobile adds one additional classification dimension: **device persistence class**.

| Class | Mobile treatment |
|---|---|
| Authentication/session secret | Only through D01/D03-approved secure mechanism; never generic application storage. |
| Protected server-state projection | Memory by default; persistent storage requires explicit security/privacy/consistency approval. |
| Non-secret session/scope projection | May be retained only as non-authoritative restore hint and must be revalidated. |
| User/client preference | May persist when non-sensitive and safe for local exposure. |
| Compatibility/build metadata | May persist when non-sensitive. |
| Pending/indeterminate mutation state | May not become a durable offline execution queue under current architecture. |

Exact OS secure-storage technology remains a future framework/platform decision.

### 5.5 Mobile Secure Persistence Gate

Every value entering durable device storage shall be classified against:

- secret sensitivity;
- Tenant/Institution scope;
- business-data sensitivity;
- canonical authority;
- required revalidation;
- deletion/invalidation consequences;
- platform backup/export exposure where applicable.

Generic durable storage shall not be treated as secure merely because it is local to the device.

A future framework implementation shall provide an adapter boundary so platform-secure storage, ordinary preferences and protected-data storage cannot be interchanged by feature code.

### 5.6 Safe Contract Consumption

Mobile feature modules shall consume backend capability only through the G01-003 Safe Contract Client specialization.

The mobile adapter shall preserve:

- current Client Context Epoch;
- Tenant/Institution scope context;
- D01 session/authentication context;
- C01 contract version/compatibility semantics;
- logical-operation/idempotency semantics where supplied;
- correlation/provenance context;
- authoritative success/rejection/indeterminate separation.

No mobile-only hidden endpoint fallback is permitted.

### 5.7 Network Loss & Intermittency

Mobile connectivity may disappear during a protected operation.

Required behavior:

```text
Before request sent
→ no business effect assumed

Authoritative rejection received
→ rejection is final for that invocation

Authoritative success received
→ success may update client projections

Connection lost while outcome unknown
→ client mutation state = indeterminate
→ reconcile with canonical owner before unsafe repeat
```

The application may preserve non-authoritative read projections for bounded presentation continuity, but connectivity loss shall not convert cached data into canonical truth.

No numeric retry policy is defined.

### 5.8 No Offline-First Mutation Architecture

Current G01 mobile architecture deliberately excludes:

- durable offline command queues;
- local canonical business aggregates;
- automatic disconnected mutation replay;
- client-side conflict resolution that changes business semantics;
- offline authorization decisions treated as server-equivalent authority.

A future offline-first mode would require explicit architecture for:

- conflict/reconciliation semantics;
- idempotency and duplicate suppression;
- authorization freshness;
- Tenant/scope continuity;
- data protection at rest;
- revocation while disconnected;
- business-rule/version drift;
- event/audit consequences.

That future architecture requires separate review and likely ADR governance.

### 5.9 Notification / Deep-Link Ingress

A notification, deep link, app link or OS launch intent is **untrusted invocation context**, not proof of authorization or business state.

The Mobile Ingress Adapter shall:

1. normalize the platform invocation;
2. treat supplied identifiers/context as untrusted input;
3. resolve/revalidate D01/A04/D02 context as required;
4. use released C01 contracts to load authoritative data;
5. hand resulting presentation/navigation intent to future L6 design.

Notification delivery semantics remain C04-owned.

### 5.10 Device Capabilities

Future mobile features may consume device capabilities such as camera, file selection, biometrics, location or local notification facilities only through governed platform adapters when they materially affect security, privacy or data handling.

This specification does not authorize any particular device capability for a business workflow.

A device capability shall never silently substitute for canonical business approval, identity, authorization or evidence requirements.

### 5.11 Mobile Authentication Integration

Mobile may consume D01-approved authentication mechanisms and platform security capabilities.

G01-004 does not decide:

- OAuth/OIDC/SAML or another protocol;
- access-token/refresh-token format;
- biometric authentication semantics;
- device binding;
- certificate-based client identity;
- secure enclave/keystore technology.

Where D01 later selects a mobile-capable authentication mechanism, G01-004 shall expose it through the session projection/security boundary defined by G01-003.

Platform biometrics, if later used, may unlock or mediate access to a D01-approved client credential/session mechanism but shall not independently become SutraERP authorization authority.

### 5.12 Application Upgrade & Compatibility

A mobile application version may remain installed longer than a web build remains cached.

Therefore mobile must explicitly handle client-version drift.

Before executing a protected capability, the application shall rely on C01 compatibility/lifecycle semantics and applicable release policy.

If no compatible released contract remains:

- the affected capability becomes unavailable;
- no internal/retired endpoint fallback is permitted;
- cached prior success cannot authorize a mutation;
- required application upgrade handling remains a release/L6 concern.

No minimum supported OS/app version period is invented here.

### 5.13 Background Execution

Background execution is not assumed to be continuously available on mobile platforms.

Background tasks, if later introduced, shall:

- remain non-authoritative client execution;
- re-establish required D01/A04/D02 context before protected work;
- consume C01 contracts through the G01 control plane;
- not use stale UI/session state as authority;
- comply with A08/E-family runtime/operations architecture where server-side asynchronous execution is the correct owner.

A mobile client shall not become a substitute background-processing platform for SutraERP enterprise workflows.

### 5.14 Mobile Telemetry & Privacy

Mobile telemetry may report technical application/device/runtime health and correlation context.

It shall not expose:

- raw credentials/session secrets;
- protected payloads merely for diagnostics;
- device data unrelated to the technical need;
- canonical audit evidence under G01 ownership.

D04/E05 and privacy architecture retain their canonical responsibilities.

---

## 6. Architecture Decisions

| Decision ID | Decision | Classification / Authority |
|---|---|---|
| `AD-001` | Mobile uses the same G01/C01 canonical backend contract boundary; no mobile-specific backend authority is created. | G01-001 + `ADR-L5-0003`. |
| `AD-002` | Mobile specializes G01-003 client security/state controls rather than defining an independent mobile session/state architecture. | Ownership-preserving family decision. |
| `AD-003` | Future mobile is online-first; disconnected authoritative business mutation is excluded. | Frozen corpus boundary + G01-003. |
| `AD-004` | OS/device lifecycle restore never restores canonical session/scope/business authority without revalidation. | G01-003 + A04/D01 trust boundary. |
| `AD-005` | Notification/deep-link ingress is untrusted invocation context requiring canonical revalidation. | `ADR-L5-0001`, D01/D02/A04 ownership. |
| `AD-006` | No mobile framework/runtime is selected while mobile remains future architecture. | Technology-selection governance; avoids premature lock-in. |
| `AD-007` | Mobile framework/runtime selection is blocking only when concrete mobile implementation is authorized. | Future ADR gate. |
| `AD-008` | Mobile platform storage is split into governed secure/persistent/nonpersistent classes behind adapters; exact products/APIs remain future decisions. | Security architecture derivation; no technology selection. |

No new ADR is allocated by G01-004. A future selection of React Native, Expo, Flutter, native Swift/Kotlin or another shared mobile platform is a mandatory ADR trigger under the L5 Technology Selection & ADR Methodology.

---

## 7. Architecture Conformance Requirements

| Requirement ID | Requirement |
|---|---|
| `CR-001` | Future mobile implementations shall preserve G01-001 non-authoritative-client invariants. |
| `CR-002` | Mobile shall consume released C01 contracts and shall not create feature-local hidden backend endpoints. |
| `CR-003` | Mobile session/authentication state shall remain a projection of D01 authority. |
| `CR-004` | Mobile authorization/capability state shall remain advisory and shall not replace D02 enforcement. |
| `CR-005` | Mobile protected state shall remain partitioned by applicable Tenant/Institution/authenticated context and follow G01-003 Client Context Epoch invalidation semantics. |
| `CR-006` | Application background/restore/restart shall not reactivate protected execution from stale session or scope state without required revalidation. |
| `CR-007` | Mobile storage shall classify data before persistence and shall not use ordinary storage as an authentication-secret store. |
| `CR-008` | Network ambiguity after mutation shall become an indeterminate client outcome until authoritative reconciliation. |
| `CR-009` | Mutation retry shall consume C01 idempotency/retry semantics and shall not be implemented as unconditional reconnect replay. |
| `CR-010` | No durable offline authoritative mutation queue shall be introduced under the current architecture. |
| `CR-011` | Notification/deep-link/device launch data shall be treated as untrusted input and shall not grant authorization or establish canonical business state. |
| `CR-012` | Mobile framework/runtime selection shall not occur as an unreviewed implementation choice; it requires the applicable ADR process before canonical adoption. |
| `CR-013` | Mobile-specific device capabilities shall be mediated through controlled adapters when they affect security, privacy or protected data handling. |
| `CR-014` | An incompatible/outdated mobile build shall make affected protected capability unavailable rather than silently bypassing C01 compatibility controls. |
| `CR-015` | Mobile background processing shall not substitute for canonical server workflow/background-processing ownership. |
| `CR-016` | Mobile telemetry shall not expose raw credentials, session secrets or protected payloads without explicit canonical authority. |

---

## 8. Open Decisions & Future Activation Gates

### OD-001 — Mobile Framework / Runtime

Candidate technology is intentionally not selected.

Possible future candidates may include cross-platform or native approaches, but no candidate has repository authority today.

**Blocking for G01-004 architecture:** No.  
**Blocking for concrete mobile implementation:** Yes.

Resolution requires technology evaluation and ADR allocation at the time mobile implementation is authorized.

### OD-002 — Mobile Authentication Carrier

D01 authentication/session authority is now governed by `ADR-L5-0038`; mobile-capable client realization/carrier details remain D01/G01-governed when mobile implementation becomes active.

**Blocking for architecture:** No.  
**Blocking for concrete authenticated implementation:** Yes.

### OD-003 — Durable Protected Mobile Cache / Offline Mode

No durable protected business cache or offline mutation architecture is approved.

**Blocking for online-first mobile:** No.  
**Blocking for offline-first/disconnected business capability:** Yes; separate security, consistency, privacy and ADR review required.

### OD-004 — Push/Device Notification Technology

Notification provider, push infrastructure and device-registration technology are C04/mobile implementation decisions, not G01-owned facts.

**Blocking for core mobile architecture:** No.

---

## 9. Traceability & Current Disposition

```text
L5-G-01-001 → shared client invariants
L5-G-01-003 → client security/session/state/safe contract consumption
L5-C-01     → shared backend contract authority
L5-D-01     → authentication/session authority
L5-D-02     → authorization authority
L5-A-04     → Tenant/Institution isolation
L5-C-04     → notification delivery architecture
ADR-L5-0001 → trust boundary
ADR-L5-0003 → contract-driven modular platform
ADR-L5-0004 → Tenant/Institution isolation enforcement
```

**Architecture completeness:** complete for future-mobile structural architecture without selecting implementation technology.  
**Technology position:** mobile framework/runtime intentionally deferred until implementation activation.  
**Offline position:** online-first; no offline authoritative business mutation.  
**L6 leakage:** none intended.  
**Current Status:** `Architecture Approved`.  
**Implementation Activation:** future mobile framework/runtime ADR, D01 mobile session carrier and any C04 push technology remain activation gates.  
**Lock Status:** not Locked; lock requires separate final governance authorization.

## 2026-09-22 Full-Corpus Post-Decision Synchronization

Mobile framework/language/storage remain intentionally deferred; only the stale D01 authentication/session dependency wording was reconciled.


## 2026-09-22 Batch 7C Impact Revalidation Synchronization

Non-material dependency/ADR metadata synchronization only. Current peer lifecycle/version labels and applicable authentication/session governance references now match canonical repository state. Client non-authority, server-side authorization, Tenant/Institution validation, safe contract consumption, online-first posture, L6 separation and future mobile technology deferral are unchanged. No lifecycle promotion is performed.
