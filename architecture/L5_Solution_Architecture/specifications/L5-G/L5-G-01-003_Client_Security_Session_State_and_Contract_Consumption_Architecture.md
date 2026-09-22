# Client Security, Session, State & Contract Consumption Architecture

## 1. Specification Metadata

**Specification ID:** `L5-G-01-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-G — Client Platform Architecture  
**Major Area:** L5-G-01 — Client Platform Architecture — Web & Future Mobile  
**Canonical Technical Responsibility:** Client security/session/state/safe contract consumption without becoming authorization authority.  
**Technical Architecture Owner:** L5-G-01 — Client Platform Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.2`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-G-01-001` v1.0.1 Architecture Approved; `L5-G-01-002` v1.0.0 Architecture Approved; `L5_G01_003_Client_Security_State_Architecture_Approval_Decision_2026-09-15.md`; G01 dependency-readiness decision 2026-09-15; `L5-C-01-001/002/003`; `L5-D-01-001/003`; `L5-D-02-001/004`; `L5-A-04-001/002`; `ADR-L5-0001`; `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0015`.  
**Referenced By:** `L5-G-01-002`, planned `L5-G-01-004`, future L6 client architecture and implementation standards.  
**Applicable ADRs:** `ADR-L5-0001`, `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0015`. D01 now defines a SutraERP custom username/password API with server-side secure-cookie sessions backed by Keycloak credential authority. Client state/storage product choices remain G01-specific and are not selected here.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the technical client control plane that safely consumes authenticated context, Tenant/Institution scope and released server contracts while keeping browser/mobile client state non-authoritative, scope-isolated and recoverable after session, network, version or context changes.

### In Scope

- client security/session projection boundary;
- client state classification and ownership rules;
- scope-partitioned server-state projections;
- client cache invalidation and context-change isolation;
- safe synchronous contract-consumption boundary;
- mutation admission, pending/indeterminate outcome handling and reconciliation;
- retry/idempotency consumption rules;
- protected browser persistence rules for the approved web client;
- stale-response containment across authentication or scope changes;
- session invalidation/logout consequences for client-held protected state;
- browser multi-tab stale-context safety;
- correlation/error handling required by client contract consumption.

### Out of Scope / Non-Goals

- Authentication Session, Credential, Refresh Token or provider lifecycle — `L5-D-01-003`;
- authentication/federation protocol or token format;
- authorization policy or Authorization Decision — `L5-D-02-*`;
- Tenant/Institution validation authority — `L5-A-04-*`;
- API protocol, endpoint design, technical error taxonomy, compatibility or idempotency semantics — `L5-C-01-*`;
- cryptography, key custody, secrets or browser security-header standards — `L5-D-03-*`;
- canonical business state, rules or business recovery;
- offline-first business mutation;
- UI/UX, navigation or interaction design;
- mandatory Redux, Zustand, TanStack Query or another state-management/cache product.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Reference Matrix

| Source | Status | Exact applicability | G01-003 use |
|---|---|---|---|
| `L5-G-01-001` | Architecture Approved | Non-authoritative client, context coordination, contract-consumption boundary, authoritative-state boundary, context-change isolation | Establishes the shared client invariants specialized here. |
| `L5-G-01-002` | Architecture Approved | React + TypeScript + React Router Framework Mode, SPA-first static web artifact | Establishes the immediate browser runtime in which G01-003 controls are realized. |
| G01 dependency-readiness decision 2026-09-15 | Architecture Approved | C01/D01/D02/A04 boundaries stable for G01 consumption | Authorizes consumption of the exact peer boundaries used here without promoting peer metadata. |
| `L5-D-01-003` | current / Review | Authentication Session authority with custom credential API, secure server-side cookie session and governed MFA profile; token/session internals remain D01-owned | Client consumes session outcome/context but does not own or redefine Authentication Session mechanics. |
| `L5-D-02-001/004` | Architecture Approved | Authorization remains separate and every protected operation is server-enforced | Client visibility/capability state remains advisory only. |
| `L5-A-04-001/002` | Draft / stable boundary for G01 consumption | Tenant-rooted isolation, validated scope propagation, fail-closed mismatch | Client state must be partitioned and invalidated by protected scope. |
| `L5-C-01-001/002/003` | Draft / stable boundary for G01 consumption | Released contract boundary; synchronous execution; compatibility/error/idempotency ownership | Client consumes these semantics and shall not invent retries, compatibility or idempotency rules. |
| `ADR-L5-0001` | Architecture Approved | Explicit trust boundaries, non-transitive trust and fail-closed protected crossing | Establishes the approval-level client security posture. |
| `L5-D-03-001` | Draft collaborator | Detailed trust/security baseline consistent with `ADR-L5-0001` | Collaboration reference only; not an approval dependency for G01-003. |
| `ADR-L5-0015` | Architecture Approved | SPA-first static web client, online-first, no micro-frontends by default | Constrains web-specific state/session realization and prohibits offline authoritative mutation. |

### 3.2 Architecture Constraints

**C-001 — Client state is never authority.**  
Client-held authentication, scope, authorization, contract or business data is always a projection/reference to a canonical owner.

**C-002 — Authentication Session authority remains D01.**  
G01 may maintain a client session projection but shall not manufacture, extend, revoke or validate the canonical Authentication Session independently of D01.

**C-003 — No client-side authorization authority.**  
Permission/capability data may affect presentation but shall never make a protected operation executable without D02 enforcement.

**C-004 — Scope isolation is mandatory.**  
Protected client state shall be bound to its validated Tenant/Institution context and shall not be reused across a context change without governed revalidation.

**C-005 — Script-readable durable browser storage is not a credential store.**  
The approved web client shall not persist authentication/session secrets, bearer-equivalent credentials, refresh/continuation secrets or credential material in `localStorage`, `sessionStorage`, IndexedDB or equivalent script-readable durable browser storage.

**C-006 — Protected server-state persistence is deny-by-default.**  
Protected business projections remain memory-scoped by default in the web client. Durable browser persistence requires an explicit classification/use-case decision proving it contains no authentication secret and does not weaken D03/A04/privacy constraints.

**C-007 — Contract semantics remain C01-owned.**  
G01-003 may coordinate request execution but shall not invent contract versions, endpoint fallbacks, technical error meanings, idempotency identities or retry safety.

**C-008 — Ambiguous mutation outcome is not success.**  
When the client cannot determine whether a mutation completed, it shall preserve an indeterminate technical outcome and reconcile with the authoritative owner before treating the business operation as complete or repeating it unsafely.

**C-009 — Offline business mutation is not part of the immediate architecture.**  
No queued offline mutation may create or claim an authoritative business outcome.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 G01-003 Owns

- **Client Security & State Coordinator** — common control plane for session projection, protected scope and state invalidation.
- **Session Projection Boundary** — non-secret client view of D01 authentication/session availability.
- **Client Context Epoch** — local technical generation marker used to invalidate asynchronous results after material authentication/scope changes.
- **Protected State Partition** — client state partition identified by applicable Tenant/Institution plus authenticated-principal/session context where required.
- **Server-State Projection Cache** — non-authoritative in-memory projection of authoritative contract results.
- **Safe Contract Client** — sole application-facing boundary for C01 request execution and technical outcome normalization.
- **Mutation Coordinator** — tracks one client mutation attempt through pending, authoritative success/rejection, or indeterminate technical outcome.
- **Browser Persistence Gate** — governs what client information may enter durable browser storage.
- **Client Invalidation/Reconciliation Control** — removes/quarantines stale state after session, scope, contract or application-version changes.

### 4.2 Consumed Architecture

| Owner | Consumed responsibility |
|---|---|
| `L5-D-01` | Canonical authenticated/session context and invalidation/continuation outcome. |
| `L5-D-02` | Authorization result/enforcement; permission/capability projection where exposed. |
| `L5-A-04` | Validated Tenant/Institution execution scope. |
| `L5-C-01` | Released contract, compatibility, technical error and idempotency semantics. |
| `L5-D-03` | Trust, secret-protection and security-control requirements. |
| `L5-G-01-002` | Browser/React Router runtime for immediate web realization. |

### 4.3 Must Not Own

G01-003 shall not own canonical Authentication Session state, token issuance, credential verification, authorization decisions, Tenant validation, business transactions, server idempotency persistence, business compensation/recovery, API schemas or L6 interaction semantics.

---

## 5. Solution Architecture

### 5.1 Logical Client Control Plane

```text
Route / Feature Module
        │
        ▼
Client Security & State Coordinator
        │
        ├── Session Projection Boundary ───────→ D01
        ├── Protected State Partition ─────────→ A04 context
        ├── Server-State Projection Cache
        ├── Mutation Coordinator
        ├── Browser Persistence Gate
        └── Safe Contract Client ──────────────→ C01
                                             ↓
                                      server D01/D02/A04
                                             ↓
                                      canonical owner
```

The control plane is a client technical mechanism only. It owns no canonical business fact.

### 5.2 Client State Classification

| State class | Authority | Default lifetime | Durable browser persistence |
|---|---|---|---|
| Session projection | D01 | Current authenticated client context | Non-secret projection only; never session/continuation secret. |
| Protected execution scope | A04 | Current validated context | Preference/reference may persist only as non-authoritative input; must be revalidated before protected use. |
| Authorization/capability projection | D02 | Advisory snapshot | Not authority; invalidate with session/scope change and when source semantics require refresh. |
| Server-state projection | Canonical business owner through C01 | Bounded client cache | Memory by default for protected data. |
| Mutation execution state | G01 technical state | Until authoritative resolution/reconciliation | No durable offline business queue in current architecture. |
| Ephemeral presentation state | G01/L6 technical use | Route/component lifetime | May persist only when non-sensitive and non-authoritative. |
| User/client preference | Applicable client/settings owner | Cross-session when permitted | Allowed only when safe for browser exposure and validated on read. |
| Compatibility/application metadata | G01/C01 | Current build/contract context | May persist where non-sensitive and never treated as server authority. |

### 5.3 Session Projection Boundary

The client session projection communicates only what the client needs to determine whether protected execution may be attempted.

It may contain non-secret references such as:

```text
session availability / authentication status projection
principal reference safe for client use
current Tenant/Institution reference
assurance/context classification where D01 exposes it
session/context version or freshness reference where available
```

It shall not contain raw Credential material, refresh/continuation secrets, private keys or another secret merely so client code can inspect it.

For the web client, a browser reload shall reconstruct session projection from D01/C01-governed authoritative context rather than trusting a previously persisted client claim of authentication.

### 5.4 Client Context Epoch

G01-003 defines a local monotonic **Client Context Epoch**.

The epoch changes whenever a material protected client context becomes invalid or is replaced, including:

- authenticated principal/session change;
- logout or authoritative session invalidation;
- Tenant/Institution context change;
- transition from authenticated to unauthenticated context;
- client reset required by incompatible application/contract state.

Every protected asynchronous read/mutation captures the epoch under which it began.

When a result returns under a no-longer-current epoch:

```text
result may be used for diagnostics/reconciliation if safe
result shall not populate the current protected state partition
result shall not restore prior scope/session capability
```

This prevents late responses from one Tenant/session context contaminating another client context.

### 5.5 Protected State Partition

A protected state partition is keyed conceptually by the current validated execution context.

At minimum, the partition distinguishes:

```text
Tenant
Institution where applicable
authenticated principal/session context where applicable
contract/application compatibility context where material
```

Exact key structure is implementation detail.

When the protected context changes, the prior partition is quarantined from active use and then cleared or retired according to its state class. No route/feature module may maintain a hidden independent protected-scope cache outside this control.

### 5.6 Server-State Projection Cache

Server-state projections are non-authoritative copies of C01 contract results.

The cache shall:

- remain scope-partitioned;
- preserve enough provenance to determine which context produced an entry;
- support invalidation after authoritative mutation outcomes;
- never convert age/freshness into business authority;
- never use stale cached authorization as a substitute for D02 enforcement;
- discard/quarantine entries produced under a stale Client Context Epoch;
- fail closed rather than reuse data across Tenant/Institution mismatch.

For the immediate web architecture, protected server-state caching is memory-scoped by default. No separate state/cache library is mandatory; the approved React/React Router runtime may provide route data/revalidation mechanics behind this boundary.

### 5.7 Browser Persistence Gate

Every value proposed for durable browser storage must be classified before persistence.

The gate shall reject:

- authentication/session identifiers or bearer-equivalent credentials readable by application JavaScript;
- refresh/continuation secrets;
- Credential/MFA secret material;
- private keys or machine credentials;
- unclassified protected business payloads;
- authorization decisions intended to survive source revalidation.

Permitted durable values are limited to non-secret, non-authoritative client information whose exposure or modification by the browser user does not grant authority. All persisted values are treated as untrusted on read.

This specification does not select the D01 session-carrier mechanism. A later D01/C01 protocol decision may use a browser mechanism that is inaccessible to application JavaScript; G01-003 consumes that mechanism without changing D01 ownership.

### 5.8 Safe Contract Client

All G01 client modules shall invoke backend capability through one **Safe Contract Client** abstraction over released C01 contracts.

Responsibilities:

1. resolve the released contract supported by the current client/application context;
2. capture current Client Context Epoch and protected state partition;
3. attach/preserve the peer-governed authentication, scope and correlation context applicable to the contract;
4. execute through C01 without bypass to internal endpoints;
5. normalize C01 technical outcome categories for client handling;
6. reject applying a response to current protected state when epoch/scope no longer matches;
7. expose authoritative success/rejection distinctly from transport/technical uncertainty;
8. preserve idempotency/logical-operation semantics supplied by C01 for safe repeat execution.

Feature modules shall not embed independent raw network clients that bypass this boundary for protected SutraERP contracts.

### 5.9 Read Flow

```text
Feature requires authoritative data
        ↓
Capture current epoch + protected partition
        ↓
Check eligible non-authoritative cache entry
        ├─ usable projection → render projection; revalidate when required
        └─ no usable projection
                ↓
        Safe Contract Client
                ↓
        C01 authoritative read
                ↓
        epoch/scope still current?
                ├─ no → discard from active partition
                └─ yes → store projection + expose result
```

Read retry behavior shall follow C01 technical-error/retry safety. No fixed retry count is invented here.

### 5.10 Mutation Flow

```text
Feature requests protected mutation
        ↓
Capture epoch + protected partition
        ↓
Mutation Coordinator creates client attempt context
        ↓
Safe Contract Client invokes released C01 mutation contract
        ↓
Server performs D01/A04/D02 and canonical-owner enforcement
        ↓
Outcome
   ├─ authoritative success
   │      ↓
   │   mark client attempt resolved
   │   invalidate/reconcile affected projections
   │
   ├─ authoritative rejection
   │      ↓
   │   preserve rejection; no local success
   │
   └─ technical/transport ambiguity
          ↓
       mark client attempt indeterminate
       reconcile authoritative state before unsafe repeat
```

An optimistic visual projection may be shown only as provisional client state. It shall not be recorded as canonical success and must be reversible/reconciled against the authoritative result.

### 5.11 Retry and Idempotency Safety

G01-003 does not define idempotency semantics; it consumes C01-003.

Rules:

- client mutation retry is **not automatic by default** merely because a transport request failed;
- repeat execution is allowed only when the applicable C01 contract declares/provides safe repeat or idempotency semantics;
- where C01 supplies a logical-operation identity, retries shall preserve that identity as required by C01 rather than creating a new business effect;
- an idempotency mismatch/integrity conflict is surfaced as a failure, never resolved by guessing intent;
- client read retries may occur only for operations whose C01 semantics make repetition safe and shall remain centrally governed rather than feature-specific;
- no numeric retry count/backoff is defined here.

### 5.12 Contract Compatibility and Client Build Drift

The Safe Contract Client consumes C01 compatibility/lifecycle status.

If the current application build has no usable released contract for a capability:

- the affected capability becomes technically unavailable;
- the client shall not call an internal/retired fallback;
- cached prior success does not authorize a new mutation;
- application refresh/upgrade may be requested by implementation, but exact UX remains L6 and exact release compatibility belongs C01/E04.

### 5.13 Session Invalidation, Logout and Expiry

On authoritative D01 session invalidation/logout/expiry indication:

1. advance the Client Context Epoch;
2. mark current session projection unavailable/stale;
3. block new protected contract execution under the old context;
4. quarantine/clear protected state partitions associated with the invalidated context;
5. preserve only non-sensitive diagnostics/correlation needed for technical handling;
6. resolve a new D01 session context before protected execution resumes.

If logout/session termination request outcome is technically ambiguous, the client shall not continue assuming the prior session is valid; protected execution must be revalidated with D01.

### 5.14 Tenant / Institution Context Change

A protected scope change is a security boundary change, not merely a presentation switch.

The client shall:

- obtain the new context through the canonical A04/D01/D02 collaboration;
- advance Client Context Epoch before the new partition becomes active;
- stop applying late responses from the prior partition;
- clear/quarantine prior protected server-state projections;
- re-resolve capability/authorization projections for the new context;
- never copy a prior Authorization Decision into the new context.

### 5.15 Multi-Tab / Concurrent Browser Context

Multiple browser tabs/windows may hold stale client projections concurrently.

Therefore:

- server D01/D02/A04 enforcement remains authoritative for every protected request;
- one tab shall not assume another tab has synchronized its local session/scope state;
- browser coordination mechanisms may improve responsiveness but shall never become session authority;
- when a tab observes authoritative session/scope rejection or returns from a materially stale period, it shall revalidate before continuing protected execution;
- secrets shall not be broadcast through script-readable cross-tab storage merely to synchronize session state.

No specific cross-tab API is mandated.

### 5.16 React / React Router Web Realization

For the approved G01-002 web platform:

- React Router `clientLoader` may coordinate authoritative reads through the Safe Contract Client;
- React Router `clientAction` may coordinate protected mutations through the Mutation Coordinator and Safe Contract Client;
- route revalidation may refresh affected server-state projections after authoritative mutation outcomes;
- React component/local state may hold ephemeral presentation state;
- shared security/session/scope state shall be provided through the G01 control plane rather than independently recreated by each route;
- no additional global state-management/cache library is mandatory in this baseline.

A future dedicated state/cache product may be adopted only if it preserves this architecture and passes applicable technology/ADR review where material.

### 5.17 Error and Security Handling

Client control flow shall depend on stable C01 technical outcome categories and peer-owned security outcomes, not human-readable message parsing.

Conceptual client handling categories include:

- authentication unavailable/invalid → D01-owned re-establishment path;
- authorization denied → terminal for that invocation under D02;
- scope invalid/mismatched → A04 fail-closed and context re-resolution;
- contract incompatible/unavailable → affected capability unavailable;
- authoritative business rejection → display/handling defined by the owning contract/L6 without converting it to technical retry;
- technical ambiguity after mutation → indeterminate + reconciliation;
- client runtime failure → no fabricated server success.

Exact HTTP status codes, endpoint schemas and error payloads remain C01-owned.

### 5.18 Correlation, Telemetry and Privacy

The Safe Contract Client shall preserve applicable correlation/provenance references supplied by the architecture and may emit client technical telemetry.

Client telemetry shall not include raw authentication/session/credential secrets or protected payloads merely for diagnostics. Canonical audit evidence remains D04-owned; operational telemetry remains E05-owned.

---

## 6. Architecture Decisions

| Decision ID | Decision | Classification / Authority |
|---|---|---|
| `AD-001` | Client session state is a non-secret projection of D01 authority; canonical Authentication Session mechanics remain server-owned. | Direct ownership preservation from D01/G01-001. |
| `AD-002` | Script-readable durable browser storage shall not hold authentication/session/continuation secrets or bearer-equivalent credentials. | Defensible client-security control under `ADR-L5-0001`/D03 trust model; does not select D01 token/protocol format. |
| `AD-003` | Protected client state is partitioned by validated execution context and invalidated through Client Context Epoch changes. | G01-owned technical mechanism required to preserve A04 isolation during asynchronous client execution. |
| `AD-004` | Protected server-state projections are memory-scoped by default in the web client; durable protected caching requires explicit review. | Local security/privacy decision within G01-003; reversible without peer ownership change. |
| `AD-005` | All protected backend interaction uses one Safe Contract Client; feature modules shall not bypass it. | `ADR-L5-0003`, G01-001 and C01 boundary. |
| `AD-006` | A technically ambiguous mutation result becomes `indeterminate` client technical state until authoritative reconciliation. | Defensible realization of G01-001 non-authoritative client and C01 ownership. |
| `AD-007` | Mutations are not blindly retried; repeat execution depends on C01-declared/provided idempotency/retry safety and preserves logical-operation identity where applicable. | C01-003 ownership-preserving consumption. |
| `AD-008` | Immediate web implementation adds no mandatory external global state/cache product; React/React Router primitives may realize the approved state architecture. | Local technology-minimization decision; no new framework dependency required. |
| `AD-009` | Offline authoritative business mutation is excluded from current G01 architecture. | `ADR-L5-0015` online-first decision and G01-001 authoritative-state boundary. |

No new canonical ADR identifier is required by these local decisions. Any future decision that introduces a shared client security framework, persistent protected-data subsystem, offline mutation platform or materially different trust boundary must follow ADR governance.

---

## 7. Security, Quality & Operational Implications

### Security

- browser/mobile client execution is untrusted for canonical authority;
- protected state cannot cross Tenant/Institution/session context through stale asynchronous results;
- script-readable durable browser storage is excluded for authentication secrets;
- D02 enforcement remains mandatory even when cached capability information exists;
- no contract/client error may trigger an ungoverned endpoint fallback.

### Consistency & Recovery

- successful client mutation state is derived from authoritative contract outcome;
- ambiguous mutation completion is explicitly represented and reconciled;
- cache invalidation follows authoritative success and context changes;
- stale client projections never repair or overwrite canonical server state.

### Maintainability

- features share one contract/security/state control plane;
- route modules remain independent of token format, API transport and server deployment;
- state-management product choice is not spread across feature modules.

### Operability

- client correlation is preserved across contract calls;
- security-sensitive values are excluded from ordinary telemetry;
- no invented numeric cache TTL, retry, timeout, refresh or session target is introduced.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Requirement |
|---|---|
| `CR-001` | Client authentication/session state shall remain a projection of D01 authority and shall not create or extend an Authentication Session locally. |
| `CR-002` | The web client shall not persist authentication/session/continuation secrets or bearer-equivalent credentials in script-readable durable browser storage. |
| `CR-003` | Every protected asynchronous result shall be associated with the client context under which it began and shall not populate current protected state after a material context epoch change. |
| `CR-004` | Protected client state shall be partitioned by the applicable Tenant/Institution and authenticated context and shall fail closed on mismatch. |
| `CR-005` | Protected server-state projections shall be treated as non-authoritative and memory-scoped by default in the web client. |
| `CR-006` | All protected client-to-server interaction shall use the G01 Safe Contract Client over released C01 contracts. |
| `CR-007` | A client feature shall not create a private endpoint fallback, independent authentication path or independent authorization enforcement path. |
| `CR-008` | A technically ambiguous mutation outcome shall not be presented internally as authoritative success until reconciled with the canonical owner. |
| `CR-009` | Mutation retry shall occur only when C01 semantics establish safe repetition/idempotency and shall preserve the required logical-operation identity. |
| `CR-010` | Cached Permission/capability information shall remain advisory and shall never replace D02 enforcement. |
| `CR-011` | Logout, authoritative session invalidation or protected scope change shall invalidate the old Client Context Epoch and prevent continued use of its protected state partition. |
| `CR-012` | Browser multi-tab synchronization shall not use script-readable storage to distribute authentication secrets or become session authority. |
| `CR-013` | React Router route data/actions used by G01-002 shall invoke the shared G01 contract/state control plane rather than bypassing it. |
| `CR-014` | No mandatory third-party global state/cache library shall be introduced without architecture review against this specification. |
| `CR-015` | Offline/disconnected client state shall not create, confirm or queue an authoritative business mutation under the current architecture. |
| `CR-016` | Client telemetry/correlation shall not expose raw credentials, session/continuation secrets or protected payloads without explicit evidence/security authority. |
| `CR-017` | Contract incompatibility shall make the affected capability unavailable rather than silently invoking a retired/internal/unversioned contract. |

---

## 9. Assumptions, Open Decisions & Dependency Position

### AS-001 — Peer Stability

G01-003 consumes only the stable client-facing responsibility boundaries of C01, D01 and A04 authorized by the G01 dependency-readiness decision. If those boundaries materially change, G01-003 requires impact review.

### OD-001 — Authentication Protocol / Session Carrier

D01 has selected the default custom credential API and server-side secure-cookie session profile; exact internal token/session representation and federation-provider details remain D01-owned.

**Blocking for G01-003 architecture:** No.  
**Blocking before implementation of a concrete authentication carrier:** Yes, owned by D01/ADR governance.

G01-003 therefore defines what the client may safely do with session authority without selecting how D01 transports that authority.

### OD-002 — Dedicated State / Server-Cache Product

No separate state/cache product is required by current architecture. React/React Router primitives are sufficient to implement the defined boundaries initially.

**Blocking:** No.

If measured implementation complexity, scale or performance later justifies a dedicated product, evaluate it against this architecture before adoption.

### OD-003 — Durable Protected Client Cache

No protected business-data persistence subsystem is approved for the web client.

**Blocking:** No for online-first architecture.  
A future offline/long-lived protected cache requires privacy, security, consistency and reconciliation review and likely ADR governance.

---

## 10. Traceability & Current Disposition

### Primary parent / peer traceability

```text
L5-G-01-001 → non-authoritative client, state/context isolation, governed contract boundary
L5-G-01-002 → React/React Router SPA runtime
L5-D-01     → authenticated/session authority
L5-D-02     → authorization authority
L5-A-04     → Tenant/Institution isolation
L5-C-01     → contract/version/error/idempotency authority
L5-D-03     → trust/security boundary
```

### Current disposition

**Architecture completeness:** complete for the G01-003 canonical responsibility.  
**Technology position:** uses the approved G01-002 web platform; adds no new mandatory state/cache/authentication technology.  
**Security position:** fail-closed, non-authoritative client with scope/session invalidation and no script-readable durable authentication-secret storage.  
**L6 leakage:** none intended.  
**Current Status:** `Architecture Approved`.  
**Downstream Readiness:** ready for `L5-G-01-004` and downstream L6/implementation consumption.  
**Lock Status:** not Locked; lock requires separate final governance authorization.

## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled client-session references with `ADR-L5-0038`; client state-management/mobile framework choices remain outside this change.
