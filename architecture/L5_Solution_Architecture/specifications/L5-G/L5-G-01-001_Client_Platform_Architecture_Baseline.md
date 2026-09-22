# Client Platform Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-G-01-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-G — Client Platform Architecture  
**Major Area:** L5-G-01 — Client Platform Architecture — Web & Future Mobile  
**Canonical Technical Responsibility:** Shared technical client-platform principles; excludes L6 screens, journeys, navigation and layout.  
**Technical Architecture Owner:** L5-G-01 — Client Platform Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.2`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Production Path:** `architecture/L5_Solution_Architecture/specifications/L5-G/L5-G-01-001_Client_Platform_Architecture_Baseline.md`

**Depends On:**  
L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; frozen L5 Candidate Document Corpus Register 105 v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5_G01_Upstream_Dependency_Readiness_and_Architecture_Approval_Decision_2026-09-15.md`; `L5-C-01-001`; applicable `L5-C-01` specializations; `L5-D-01-001`; applicable `L5-D-01` specializations; `L5-D-02-001`; applicable `L5-D-02` specializations; `L5-A-04-001`; `L5-A-04-002`.

**Referenced By:** Planned `L5-G-01-002`, `L5-G-01-003`, `L5-G-01-004`; future L6 architecture.

**Applicable ADRs:** `ADR-L5-0003` and `ADR-L5-0004`. No client framework, client runtime, rendering model, mobile technology or client-state product is selected by this specification. Any later shared client-platform/framework selection shall follow ADR governance.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the common technical architecture boundary that all SutraERP client applications shall preserve when interacting with the SutraERP solution.

It establishes how a client participates in the solution without becoming an authority for:

- business state;
- Tenant or Institution identity;
- authentication;
- authorization;
- business-rule execution;
- canonical business outcomes;
- persistence.

The baseline is intentionally technology-neutral and client-platform-neutral. Web-specific realization belongs to `L5-G-01-002`; detailed client security, session, local state and safe contract-consumption mechanics belong to `L5-G-01-003`; future mobile specialization belongs to `L5-G-01-004`.

### 2.2 In Scope

This specification owns:

- the shared logical client-platform boundary;
- the common client initialization and context-establishment model;
- the common contract-consumption boundary;
- the rules for consuming authenticated and scope context;
- the client-side relationship to authorization decisions;
- the treatment of authoritative versus client-local state;
- shared failure-containment principles;
- common specialization requirements for web and future mobile clients;
- the architecture handoff from L5 client architecture to L6 UI/UX.

### 2.3 Out of Scope / Non-Goals

This specification does not own:

- screen, page, form, layout or navigation architecture;
- user journeys or interaction design;
- web rendering model or web framework;
- mobile framework or native/hybrid choice;
- detailed client session/token storage;
- detailed client cache/state-management implementation;
- authentication-provider implementation;
- authorization policy evaluation;
- API protocol or endpoint design;
- server persistence;
- business validation;
- business lifecycle state;
- business events;
- domain-owned business data;
- Workspace lifecycle, commands, events, publishers or approval authority.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority & Dependency Matrix

| Source | Version / Status | Exact applicability | L5-G-01-001 use |
|---|---|---|---|
| L5 Candidate Corpus Register 105 | v1.1.0 / Architecture Approved | `L5-G-01-001` | Establishes shared client-platform ownership and L6 exclusion. |
| L5 Architecture Family Register | Architecture Approved | L5-G family | Establishes Client Platform as the canonical L5 family. |
| L5 Dependency-Gated Authoring Timeline | Locked | Stage 4 `L5-G-01`; minimum `C01 + D01 + D02 + A04` | Establishes mandatory peer dependency set. |
| `L5_G01_Upstream_Dependency_Readiness_and_Architecture_Approval_Decision_2026-09-15.md` | Architecture Approved | G01 scoped dependency-readiness and approval decision | Removes invalid direct A007/A050 approval dependencies and declares the consumed C01/D01/D02/A04 boundaries sufficiently stable for G01. |
| `L5-C-01-001` | v1.2.1 / Review | Contract Boundary and catalogue principles | Client calls cross the solution only through governed contracts. |
| `L5-D-01-001` | v1.2.1 / Review | IAM baseline | Supplies authenticated technical context without authorization semantics. |
| `L5-D-02-001` | v1.2.1 / Architecture Approved | Authorization baseline | Supplies authoritative authorization evaluation/enforcement. |
| `L5-A-04-001` / `L5-A-04-002` | v1.0.1 / Review | Tenant/Institution isolation and validated scope propagation | Client-supplied scope is not authoritative merely because it was presented by the client. |
| `ADR-L5-0003` | Architecture Approved | Contract-driven modular platform with selective isolation | Client architecture targets governed contracts rather than physical runtime placement or internal module structures. |
| `ADR-L5-0004` | Architecture Approved | Tenant and Institution isolation enforcement | Client boundaries preserve Tenant/Institution isolation and fail closed on invalid scope. |

`L4-A-007` and `L4-A-050` are not direct approval authorities for this reusable G01 baseline. Authentication behavior remains upstream/D01-owned, and Workspace business behavior remains upstream/D02-owned. G01 consumes peer-owned technical results without redefining their business semantics.

### 3.2 Architecture Constraints

**C-001 — Non-authoritative client.**  
A client shall never become the authoritative source of a protected business fact, business decision, Tenant/Institution identity, authentication result or Authorization Decision.

**C-002 — Explicit protected scope.**  
Protected operations shall carry sufficient Tenant/Institution context for the owning server-side architecture to validate the operation within the correct protected scope.

**C-003 — Contract-only server interaction.**  
Client applications shall consume released technical contracts owned by `L5-C-01`; they shall not depend on another module's persistence or internal implementation boundary.

**C-004 — Authentication is not authorization.**  
Authenticated context received from `L5-D-01` shall never be interpreted by the client as authority to perform a protected operation.

**C-005 — Authorization remains authoritative outside the client.**  
Client visibility or enablement information may improve client behavior but shall never replace `L5-D-02` enforcement.

**C-006 — Client-held business information is a projection.**  
Client-local representations of server-owned business data remain non-authoritative copies of canonical facts.

**C-007 — Platform specialization remains downstream.**  
Web and mobile specializations shall preserve this baseline without redefining its security, contract or scope principles.

**C-008 — L6 remains independent.**  
Technical client architecture shall not prescribe the screen, navigation, interaction or visual model.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 G01-001 Owns

- Client Platform Host boundary.
- Client Context Coordination boundary.
- Client Contract Consumption boundary.
- shared client execution invariants;
- client-side authoritative/non-authoritative data boundary;
- common failure and protected scope-change behavior;
- shared technical specialization contract for G01-002 through G01-004.

### 4.2 Consumed Architecture

| Owner | Capability consumed by G01 |
|---|---|
| `L5-C-01` | Released synchronous contracts, contract identity/version/compatibility and governed invocation boundary. |
| `L5-D-01` | Authenticated technical principal/session context. |
| `L5-D-02` | Authorization decisions, permission/capability semantics and protected-operation enforcement. |
| `L5-A-04` | Validated Tenant/Institution scope semantics and isolation rules. |
| `L5-A-01/A03` | Application/module technical boundaries and contract-driven composition. |

### 4.3 Must Not Own

G01-001 shall not own:

- authorization policy or Authorization Decision;
- User Account, Credential or Authentication Session lifecycle;
- Tenant/Institution validation authority;
- contract protocol/versioning internals;
- domain persistence;
- canonical domain data;
- Workspace business lifecycle or authority;
- UI composition semantics;
- web/mobile implementation details.

---

## 5. Solution Architecture

### 5.1 Logical Client Architecture

```text
Client Application
      │
      ▼
Client Platform Host
      │
      ├── Context Coordination Boundary
      │       ├── Tenant / Institution Context → L5-A-04
      │       ├── Authenticated Context → L5-D-01
      │       └── Authorization Context → L5-D-02
      │
      ├── Contract Consumption Boundary
      │       └── Released Contracts → L5-C-01
      │
      ├── Client Result / Failure Boundary
      │
      └── Telemetry / Correlation Handoff
              ↓
        Canonical server-side architecture
```

No element above owns business truth.

### 5.2 Client Platform Host

The **Client Platform Host** is the common runtime boundary inside a SutraERP client.

It shall:

- initialize shared client-platform services;
- coordinate access to validated execution context;
- expose the governed contract-consumption mechanism to client feature code;
- isolate feature implementations from server physical placement;
- preserve a stable handoff boundary for G01-002, G01-003 and G01-004.

The host shall not:

- approve business actions;
- infer authorization from client state;
- define domain business rules;
- prescribe L6 navigation or page composition.

### 5.3 Context Coordination Boundary

The **Context Coordination Boundary** coordinates the technical context required by client-originated operations.

Its conceptual context categories are:

```text
Tenant / Institution scope
Authenticated principal/session context, when applicable
Authorization-relevant resource/action context
Correlation context
Applicable contract identity/version
```

These categories are references to peer-owned authority.

The client does not become canonical owner of any category merely because it carries or temporarily stores it.

### 5.4 Contract Consumption Boundary

All server-side capability access shall flow through a governed **Contract Consumption Boundary**.

Conceptually:

```text
Client feature
      ↓
Contract Consumption Boundary
      ↓
Resolve released contract
      ↓
Attach applicable validated execution context
      ↓
Invoke canonical owning capability
      ↓
Receive authoritative response / rejection / technical failure
```

The boundary shall prevent feature code from depending directly on:

- database structure;
- internal server module implementation;
- physical service placement;
- unreleased/internal contracts not authorized for that client.

Transport, payload format, endpoint structure, contract compatibility and request idempotency remain owned by `L5-C-01`.

### 5.5 Initialization Flow

```text
Client artifact starts
      ↓
Initialize Client Platform Host
      ↓
Resolve available Tenant / Institution execution context
      ↓
Consume authenticated context when required
      ↓
Establish access to released contracts
      ↓
Bind authorization-aware capability context
      ↓
Client platform becomes technically ready
```

An unauthenticated client context may exist only for capabilities whose canonical server-side contracts permit unauthenticated consumption.

A protected capability shall not be treated as technically ready merely because client initialization completed.

### 5.6 Protected Operation Flow

```text
L6 interaction requests an operation
      ↓
Client feature requests canonical contract
      ↓
Context Coordination Boundary supplies current context
      ↓
Contract Consumption Boundary issues request
      ↓
Server-side authentication / scope / authorization enforcement
      ↓
Owning capability executes or rejects
      ↓
Authoritative result returned
      ↓
Client projection is updated from the authoritative result
```

The client may perform presentation-related checks or suppress obviously unavailable actions, but those checks are advisory.

They are never authorization enforcement.

### 5.7 Protected Scope Change

When a Tenant, Institution or another peer-defined protected execution scope changes:

```text
Requested scope change
      ↓
Validate / resolve through canonical owner
      ↓
New protected scope accepted
      ↓
Previous scope-bound client context becomes ineligible for reuse
      ↓
G01-003 applies required state/session consequences
      ↓
Applicable capabilities/contracts are resolved for the new scope
```

A protected scope change shall not:

- transfer authority from the previous scope;
- reuse protected information from the prior scope without revalidation;
- create Permissions;
- alter an Authorization Decision merely because client context changed.

Detailed local-state invalidation and session mechanics belong to `L5-G-01-003`. Business concepts such as Workspace remain owned by their canonical upstream/D02 architecture and are not defined by G01.

### 5.8 Authoritative State Boundary

Client state belongs to one of two categories:

**Technical client state**

Examples include:

- currently resolved technical context;
- in-flight operation status;
- client-local projections;
- local presentation preferences;
- compatibility metadata.

**Canonical business state**

Owned only by authoritative server-side business domains.

The client shall never treat successful local mutation as proof of canonical business mutation.

A protected business operation is successful only when the applicable authoritative contract reports the governed successful outcome.

### 5.9 Contract Evolution

The client shall consume only a contract version recognized as usable under `L5-C-01` compatibility rules.

If no compatible released contract exists:

```text
do not guess
do not call an internal fallback
do not reinterpret another contract
        ↓
fail the affected client capability explicitly
```

This failure shall remain contained to the affected capability where possible.

### 5.10 Material Failure Behavior

| Failure | Required architecture behavior |
|---|---|
| Tenant/Institution context absent or inconsistent | Do not initiate the protected operation. Require canonical context resolution. |
| Authentication context absent/invalid for a protected operation | Do not treat the caller as authenticated. Route through D01-owned behavior. |
| Authorization denied | Treat the authoritative denial as terminal for that invocation. Client logic shall not override it. |
| Contract unavailable | Surface technical unavailability; do not synthesize business success. |
| Contract incompatible | Stop the affected capability until a governed compatible contract exists. |
| Network interruption after mutation request | Do not infer success or automatically create a second business effect. Retry/reconciliation follows C01/G01-003 governed semantics. |
| Client projection stale | Re-resolve from authoritative sources before using stale data for a protected decision. |
| Protected scope changes during active use | Prevent prior scope-bound context from being silently reused. |

### 5.11 Trust Boundary

Client execution is outside the authoritative server-side trust boundary.

Therefore:

```text
client assertion
      ≠
authoritative fact

client authenticated-context copy
      ≠
authorization decision

client visibility rule
      ≠
access-control enforcement

client local data
      ≠
canonical business state
```

Server-side canonical owners shall validate all material protected context before executing protected behavior.

### 5.12 Telemetry and Evidence Handoff

The client platform shall provide technical correlation sufficient for client-originated operations to participate in solution-wide operational tracing.

G01-001 does not own:

- canonical audit evidence;
- server observability architecture;
- audit retention;
- trace-storage technology.

Detailed evidence is handed to the applicable L5-D-04 and L5-E-05 architecture.

No sensitive client/session value shall be exposed merely to improve diagnostics.

### 5.13 Web and Mobile Specialization

`L5-G-01-002` and `L5-G-01-004` shall specialize this baseline.

They may define platform-specific:

- runtime structure;
- packaging;
- rendering/application execution;
- platform lifecycle integration;
- delivery/update considerations.

They shall not weaken:

- contract boundaries;
- Tenant/Institution isolation;
- authentication/authorization separation;
- non-authoritative client semantics;
- canonical data ownership.

---

## 6. Architecture Decisions

| Decision ID | Decision | Classification / Authority |
|---|---|---|
| `AD-001` | Treat every client as a non-authoritative technical participant; protected truth and decisions remain with canonical server-side owners. | Defensible L5 derivation from D01/D02/A04 and upstream ownership. |
| `AD-002` | Require one governed logical Contract Consumption Boundary between client feature code and SutraERP server capabilities. | Inherits contract-driven architecture from `ADR-L5-0003` and `L5-C-01`. |
| `AD-003` | Coordinate Tenant/Institution, authenticated and authorization-relevant context explicitly rather than allowing each client feature to invent context handling. | Defensible shared client-baseline realization; peer authority retained. |
| `AD-004` | Keep this baseline platform-neutral and specialize web/mobile architecture in G01-002/G01-004. | Frozen corpus/granularity decision. |
| `AD-005` | Client-side permission/capability information may guide presentation but never constitutes authoritative authorization enforcement. | Inherited `L5-D-02` boundary. |
| `AD-006` | Client-held business information remains a projection; protected mutation is complete only after authoritative server outcome. | Preserves upstream ownership and prevents client-side state transfer. |

No new canonical ADR number is allocated by this specification.

A future choice of a repository-wide/shared client framework or client platform technology is ADR-governed.

---

## 7. Quality, Security & Operational Implications

### Security

- protected execution fails closed where required context cannot be validated;
- client assertions never create protected authority;
- authentication and authorization remain independent;
- Tenant/Institution context is validated outside the client;
- server authorization cannot be bypassed by client feature logic.

### Portability

The baseline does not depend on:

- browser-only primitives;
- mobile-only primitives;
- one language;
- one framework;
- one rendering model;
- one cloud provider.

This permits both G01-002 and G01-004 to consume the same architectural invariants.

### Maintainability

Client features depend on shared boundaries rather than independently integrating IAM, Authorization, Tenant scope and server contracts.

This reduces architecture drift and prevents feature-specific security semantics.

### Resilience

Failure of one optional capability should not force unrelated client capabilities to fabricate state or results.

Capability degradation must remain explicit and bounded.

### Observability

Client-originated operations preserve technical correlation without transferring audit or observability ownership into the client family.

### Performance

This specification introduces no numeric latency, caching, payload-size or throughput target.

Performance-specific architecture shall consume authoritative targets and `L5-E-02` where material.

---

## 8. Assumptions, Open Decisions & Dependency Readiness

### AS-001 — Peer Boundary Change Revalidation

The consumed C01, D01, D02 and A04 boundaries are declared sufficiently stable for G01 by `L5_G01_Upstream_Dependency_Readiness_and_Architecture_Approval_Decision_2026-09-15.md`.

**Impact if a peer boundary changes materially:** G01 contract/context adapters and conformance requirements require impact review.

**Resolution:** Revalidate this specification whenever a consumed peer boundary materially changes.

### OD-001 — Shared Client Technology Selection

No client framework, runtime, rendering framework, client-state library or mobile framework is selected.

A future material shared client-platform/framework selection is ADR-governed.

**Blocking for this product-independent architectural baseline:** No.  
**Blocking before technology-specific implementation where a concrete platform selection is required:** Yes.

No ADR identifier is allocated locally by this specification.

### Dependency Readiness Position

- C01 contract boundary: stable for G01 consumption; transport/style/product decisions remain peer-owned.
- D01 authenticated-context boundary: stable for G01 consumption; protocol/provider/session representation remains peer-owned.
- D02 authorization boundary: Architecture Approved.
- A04 Tenant/Institution isolation and validated scope propagation: stable for G01 consumption under `ADR-L5-0004`.

No G01-owned blocking dependency remains.

---

## 9. Conformance Requirements

| CR | Requirement |
|---|---|
| `CR-001` | No client component shall be the authoritative enforcement point for a protected business authorization decision. |
| `CR-002` | Client-to-server capability access shall use a released `L5-C-01` governed contract boundary. |
| `CR-003` | Client code shall not directly access or depend on server persistence internals. |
| `CR-004` | A protected operation shall carry a Tenant/Institution context capable of authoritative A04 validation. |
| `CR-005` | Authentication success shall not be interpreted by the client as authorization success. |
| `CR-006` | Client-side visibility or capability checks shall remain advisory and shall not replace D02 enforcement. |
| `CR-007` | Protected client state bound to one Tenant/Institution context shall not be silently reused after a protected scope change without governed revalidation. |
| `CR-008` | Contract incompatibility shall prevent the affected capability from silently using an ungoverned fallback. |
| `CR-009` | Network, client or server technical failure shall never be converted by the client into a fabricated successful business outcome. |
| `CR-010` | Client-local business representations shall remain projections of source-owned canonical data. |
| `CR-011` | Web and mobile specializations shall conform to G01-001 client trust, scope, authorization and contract boundaries. |
| `CR-012` | G01 implementation shall not encode screens, navigation, page structure or interaction journeys as L5 architectural requirements. |
| `CR-013` | No programming language, framework, rendering model, state-management product or mobile platform shall become mandatory without applicable governance/ADR approval. |
| `CR-014` | Client telemetry shall preserve correlation while not assuming ownership of audit evidence or exposing protected session/security material. |

---

## 10. Traceability & Current Disposition

### Approval authority

`L5_G01_Upstream_Dependency_Readiness_and_Architecture_Approval_Decision_2026-09-15.md` establishes that:

- direct `L4-A-007` dependency is unnecessary for this reusable client-platform baseline;
- direct `L4-A-050` dependency is invalid while its parent Workspace architecture remains blocked;
- G01 shall consume authentication, authorization and protected-scope outcomes through their canonical L5 peer owners;
- the required C01, D01, D02 and A04 boundaries are sufficiently stable for G01 consumption.

### Primary peer dependencies

```text
L5-C-01 — API & Contract Architecture
L5-D-01 — IAM Architecture
L5-D-02 — Authorization Architecture
L5-A-04 — Tenant & Institutional Isolation Architecture
```

### Current disposition

**Technical Architecture:** coherent and implementable at product-neutral L5 depth.  
**Technology Selection:** intentionally unresolved and non-blocking for this baseline.  
**L6 Leakage:** none.  
**Current Status:** `Architecture Approved`.  
**Downstream Readiness:** ready for `L5-G-01-002`, `L5-G-01-003` and `L5-G-01-004` consumption.  
**Lock Status:** not Locked; lock requires separate final governance authorization.


## 2026-09-22 Batch 7C Impact Revalidation Synchronization

Non-material dependency/ADR metadata synchronization only. Current peer lifecycle/version labels and applicable authentication/session governance references now match canonical repository state. Client non-authority, server-side authorization, Tenant/Institution validation, safe contract consumption, online-first posture, L6 separation and future mobile technology deferral are unchanged. No lifecycle promotion is performed.
