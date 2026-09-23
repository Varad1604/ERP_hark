# Web Application Architecture

## 1. Specification Metadata

**Specification ID:** `L5-G-01-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-G — Client Platform Architecture  
**Major Area:** L5-G-01 — Client Platform Architecture — Web & Future Mobile  
**Canonical Technical Responsibility:** Web-client technical architecture for the immediate product direction without UI/UX design.  
**Technical Architecture Owner:** L5-G-01 — Client Platform Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.1`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-G-01-001` v1.0.1 Architecture Approved; `L5-C-01`; `L5-D-01`; `L5-D-02`; `L5-A-04`; `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0015`.  
**Collaborates With:** `L5-G-01-003` — client security/session/state/safe contract-consumption architecture used by the web realization; this collaboration is not an upstream approval dependency for G01-002.  
**Referenced By:** Future L6 web UI/UX architecture; web implementation standards; deployment/runtime consumers of the web artifact.  
**Applicable ADRs:** `ADR-L5-0015` — Web Client Platform, Rendering & Deployment Pattern; `ADR-L5-0003`; `ADR-L5-0004`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the immediate SutraERP web application architecture that specializes the approved G01 client-platform baseline into a concrete browser application using the technology and rendering pattern approved by `ADR-L5-0015`.

### In Scope

- React + TypeScript web application runtime structure;
- React Router Framework Mode application/routing boundary;
- SPA-first rendering model;
- application-shell and route/feature module composition;
- static web artifact boundary;
- web bootstrap and readiness flow;
- web specialization of G01 contract/context/security handoffs;
- route-level technical modularity and code-loading boundaries;
- browser-runtime failure containment;
- deployment-host obligations required by the SPA artifact.

### Out of Scope / Non-Goals

- screen, page, layout, navigation taxonomy, interaction flow or visual design;
- detailed client session/token storage, caching, mutation recovery or state-management product — `L5-G-01-003`;
- backend API protocol/endpoint architecture — `L5-C-01`;
- authentication-provider or credential/session lifecycle — `L5-D-01`;
- authorization policy/decision — `L5-D-02`;
- Tenant/Institution validation authority — `L5-A-04`;
- future mobile runtime — `L5-G-01-004`;
- web-server/CDN/cloud product selection;
- independently deployed micro-frontends.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Status | Exact applicability | G01-002 use |
|---|---|---|---|
| `L5-G-01-001` | Architecture Approved | Non-authoritative client; context coordination; governed contract consumption; authoritative-state boundary; web/mobile specialization contract | Establishes mandatory shared client invariants that the web architecture specializes. |
| G01 dependency-readiness decision 2026-09-15 | Architecture Approved | C01/D01/D02/A04 boundaries stable for G01 consumption | Allows web specialization to consume those peer boundaries without inventing them. |
| `ADR-L5-0015` | Architecture Approved | React + TypeScript + React Router Framework Mode; SPA-first; static artifact; modular single application; no micro-frontends by default | Establishes the concrete web platform and rendering/deployment pattern. |
| `ADR-L5-0003` | Architecture Approved | Contract-driven modular platform | Web modules target released contracts rather than backend placement/internal module structure. |
| `ADR-L5-0004` | Architecture Approved | Tenant/Institution isolation | Browser-provided scope never becomes authoritative; protected execution remains fail-closed server-side. |
| `L5-C-01` | Stable for G01 consumption | Governed released synchronous contract boundary | Owns transport/contract mechanics consumed by the web application. |
| `L5-D-01` | Stable for G01 consumption | Authenticated technical context | Supplies authentication/session context without authorization semantics. |
| `L5-D-02` | Architecture Approved | Authorization evaluation/enforcement | Server-side authorization remains authoritative. |
| `L5-A-04` | Stable for G01 consumption | Tenant-rooted scope/isolation | Web application carries but never authoritatively establishes protected scope. |

### 3.2 Architecture Constraints

**C-001 — Browser is untrusted for authority.**  
All browser-held identity, scope, permission, capability and business data is non-authoritative.

**C-002 — SPA correctness.**  
The authenticated ERP product shall not require runtime server-side rendering for correctness.

**C-003 — Static deployability.**  
The production web build shall be deployable as static web assets without requiring a persistent Node.js application server.

**C-004 — Modular single application.**  
Feature modularity shall remain inside one deployable application unless a future ADR explicitly establishes a distributed frontend topology.

**C-005 — Contract isolation.**  
Route/feature modules shall reach server capability only through the G01/C01 contract-consumption boundary.

**C-006 — G01-003 ownership.**  
Session/token storage, detailed local state, safe mutation/retry and contract-consumption security mechanics shall not be redefined here.

**C-007 — L6 separation.**  
Technical routes/modules do not establish user-facing navigation semantics, screen hierarchy or interaction design.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 G01-002 Owns

- **Web Application Shell** — browser runtime composition root for the SutraERP web application;
- **Web Route Runtime** — React Router Framework Mode route-module execution boundary;
- **Web Feature Module Boundary** — technical packaging/loading boundary for coherent client capability groups;
- **Web Platform Adapter Layer** — browser-specific adaptation of approved G01 shared services;
- **Static Web Artifact** — deployable browser assets produced by the governed build;
- **Web Bootstrap Boundary** — technical initialization sequence before route features may execute protected work;
- **Web Runtime Failure Boundary** — containment of route/module/client-runtime failures without fabricating business outcomes.

### 4.2 Consumed / Collaborating Architecture

| Owner | G01-002 relationship |
|---|---|
| `L5-G-01-001` | Upstream shared client host, context, contract and trust invariants. |
| `L5-G-01-003` | Collaborating sibling architecture that owns security/session/state/safe contract-consumption mechanics used by the web realization; it is not an upstream approval dependency for G01-002. |
| `L5-C-01` | Released synchronous contracts and compatibility semantics. |
| `L5-D-01` | Authenticated context. |
| `L5-D-02` | Authorization enforcement. |
| `L5-A-04` | Validated execution scope and isolation semantics. |
| deployment/runtime architecture | Hosting and serving of the static web artifact without changing its client ownership boundary. |

### 4.3 Must Not Own

G01-002 shall not own business rules, canonical state, authorization decisions, authentication/session lifecycle, API schemas, persistence, audit evidence, deployment-product selection or L6 UX semantics.

---

## 5. Solution Architecture

### 5.1 Selected Web Platform

Per `ADR-L5-0015`:

```text
Language          TypeScript
UI runtime        React
Routing/app mode  React Router — Framework Mode
Rendering         SPA-first / client-side runtime rendering
Runtime SSR       Disabled by default
Build integration React Router framework toolchain with Vite integration
Deployable unit   Static web application artifact
Topology          One modular web application
```

Framework/library version pinning belongs to implementation dependency management and shall remain within supported compatibility ranges; this specification does not invent version numbers.

### 5.2 Logical Runtime Structure

```text
Browser
  │
  ▼
Static Web Artifact
  │
  ▼
Web Application Shell
  │
  ├── Web Bootstrap Boundary
  │     └── G01-001 / G01-003 shared client services
  │
  ├── React Router Web Route Runtime
  │     ├── Route / Feature Module A
  │     ├── Route / Feature Module B
  │     └── Route / Feature Module N
  │
  ├── Web Platform Adapter Layer
  │     ├── Browser lifecycle adapter
  │     ├── Browser storage adapter → G01-003 policy
  │     └── Browser network/runtime adapter
  │
  └── G01 Contract Consumption Boundary
         ↓
      L5-C-01
         ↓
 Canonical backend owners
```

React components and route modules are client implementation structure; they do not become business-domain owners.

### 5.3 Web Application Shell

The Web Application Shell is the composition root for the browser application.

It shall:

- initialize the G01 shared client-platform services required by the web runtime;
- instantiate the React Router route runtime;
- expose approved shared technical context to route/feature modules through governed client boundaries;
- establish top-level technical error/failure containment;
- coordinate application-version/build identity needed for compatibility and diagnostics;
- remain independent of domain business rules and server physical placement.

The shell shall not encode L6 screen hierarchy or use route presence as authorization authority.

### 5.4 Web Bootstrap & Readiness

```text
Static assets load
      ↓
Web Application Shell initializes
      ↓
G01 shared services initialize
      ↓
G01-003 establishes permitted client security/session/state boundary
      ↓
Current Tenant/Institution and authenticated context are resolved when applicable
      ↓
C01 contract compatibility/readiness is established for required capability
      ↓
React Router route runtime activates feature module
```

A route may render a technical loading, unavailable or unauthenticated condition before protected readiness exists. Such presentation does not grant permission and is not a business outcome.

Protected contract execution shall not begin merely because a route module has loaded.

### 5.5 Route / Feature Module Boundary

A **Web Feature Module** is a technical code-organization and loading boundary for a coherent set of web capabilities.

It may contain:

- route modules;
- React components;
- client-side adapters/view models;
- module-local technical orchestration;
- module-local non-authoritative presentation state.

It shall not contain:

- server business rules as client authority;
- direct persistence access;
- independent authentication/authorization implementations;
- module-specific bypasses around G01/C01/A04/D01/D02 boundaries.

Module boundaries shall be capable of independent code loading where useful, but independent loading does not imply independent deployment.

### 5.6 Route Runtime

React Router Framework Mode is the canonical route/application framework for G01-002.

Its route configuration is a technical mapping from browser locations to web feature execution units. It shall not be treated as canonical institutional navigation or a business workflow definition.

Route-level capabilities may use framework-supported code splitting. Route modules shall resolve data/mutations through the G01/C01 boundary rather than direct backend-internal calls.

Runtime SSR remains disabled by default under `ADR-L5-0015`.

### 5.7 Rendering Boundary

The authenticated ERP application shall use client-side runtime rendering after the initial static artifact is delivered.

The browser may pre-render or hydrate framework-generated root content produced at build time, but no business authority or protected server processing shall execute in that build artifact.

A later SSR/SSG specialization may be introduced only when:

1. a material requirement exists;
2. the additional runtime/trust/deployment boundary is assessed;
3. `ADR-L5-0015` impact governance is completed.

### 5.8 Static Web Artifact

The web build produces a static deployable artifact containing the generated SPA entry document and browser assets required by the application.

The hosting environment shall:

- serve the artifact over the institution's approved web delivery boundary;
- preserve correct content/media types and asset addressing;
- route valid application locations to the SPA entry artifact where necessary;
- avoid embedding environment secrets into public browser assets;
- not become a business-logic or authorization owner merely because it hosts the application.

Hosting may be on-premises or cloud-capable. No vendor is selected here.

### 5.9 Browser Configuration Boundary

Environment-specific web configuration shall contain only information safe for browser exposure.

Anything delivered to browser code shall be treated as observable by the user. Credentials, server secrets, private keys and privileged machine credentials shall not be placed in the static artifact or browser configuration.

Detailed secret/security mechanisms remain D03/G01-003 owned.

### 5.10 Contract Consumption

```text
Route / Feature Module
       ↓
G01-003 safe client-consumption mechanism
       ↓
G01-001 Contract Consumption Boundary
       ↓
Released C01 contract
       ↓
Server scope/authentication/authorization enforcement
       ↓
Authoritative owner result
       ↓
Non-authoritative web projection
```

Web modules shall not select alternative endpoint/transport behavior outside C01 governance merely because browser APIs allow it.

### 5.11 Tenant / Institution Context

The web client may carry and present the current protected scope, but A04 remains authoritative.

When scope changes, G01-003 shall apply state/session consequences before prior protected projections or mutable operations are reused. G01-002 shall ensure route/feature modules consume only the current G01-approved context and do not maintain independent hidden scope authorities.

### 5.12 Authentication & Authorization

The web runtime may react to D01 authentication state and D02 authorization/capability information for technical presentation behavior.

It shall not:

- convert route access into authorization;
- infer permission from authenticated status;
- treat hidden/disabled UI as enforcement;
- fabricate a successful protected operation after an authoritative Deny.

Server-side D02 enforcement remains mandatory.

### 5.13 Client State Boundary

G01-002 permits module-local ephemeral presentation state necessary to operate the React application.

Canonical rules for:

- session state;
- token handling;
- contract/result caching;
- mutation state;
- cross-route shared state;
- persistence in browser storage;
- stale-data handling;
- retries/reconciliation;

belong to `L5-G-01-003`.

No state-management product is selected by G01-002.

### 5.14 Online-First Execution

The immediate web application assumes governed backend connectivity for authoritative protected business execution.

Loss of connectivity may preserve bounded technical/presentation state as permitted by G01-003, but shall not create offline authoritative business mutation or locally confirm server success.

An offline-first business execution model would materially change consistency, security and reconciliation architecture and requires separate governance.

### 5.15 Failure Containment

| Failure class | Required behavior |
|---|---|
| Static artifact cannot load completely | Application does not claim technical readiness; hosting/runtime diagnostics may report failure. |
| Route/feature module load failure | Contain failure to affected web capability where possible; do not fabricate business outcome. |
| Client runtime/component failure | Top-level technical boundary prevents unrelated client state from being interpreted as successful server execution. |
| C01 contract unavailable/incompatible | Affected feature becomes unavailable according to G01-001/C01 semantics; no hidden fallback to internal endpoints. |
| D01 context absent/expired | Protected execution does not proceed as authenticated; G01-003/D01 own recovery/session handling. |
| D02 Deny | Invocation remains denied; route/UI behavior cannot override it. |
| A04 scope invalid/mismatched | Protected execution fails closed; web context is re-resolved before reuse. |
| Connectivity lost after mutation request | Web client does not infer final success; reconciliation follows G01-003/C01 semantics. |
| New application build deployed while browser is active | Compatibility handling must prevent silent use of an unusable contract/client combination; exact refresh/version strategy belongs to implementation/G01-003 where material. |

### 5.16 Modularity and Micro-Frontend Boundary

The canonical immediate topology is one web application artifact.

Internal feature modules may be independently developed and code-split but share:

- one G01 application shell;
- one client security/session boundary;
- one contract-consumption architecture;
- one framework/runtime compatibility baseline.

Independently deployed micro-frontends are prohibited as an unreviewed local implementation choice because they create additional deployment, versioning, trust and runtime-composition boundaries.

### 5.17 L6 Handoff

L6 may design pages, screens, flows, navigation, visual hierarchy and interaction behavior using the technical capabilities exposed by this web architecture.

L6 shall not be forced to mirror source folders, route-module names or code-splitting boundaries as user-visible information architecture unless separately justified.

---

## 6. Architecture Decisions

| Decision ID | Decision | Authority |
|---|---|---|
| `AD-001` | React + TypeScript is the immediate SutraERP web application platform. | `ADR-L5-0015` |
| `AD-002` | React Router Framework Mode is the canonical web route/application framework. | `ADR-L5-0015` |
| `AD-003` | Authenticated ERP execution is SPA-first with runtime SSR disabled by default. | `ADR-L5-0015` |
| `AD-004` | The production web deployable is a static artifact, independent of a mandatory persistent frontend application server. | `ADR-L5-0015` |
| `AD-005` | One modular web application shell is the default topology; micro-frontends require new architecture governance. | `ADR-L5-0015` |
| `AD-006` | Route/feature modules are technical boundaries and do not define L6 navigation or business ownership. | G01-001 + G01-002 responsibility boundary |
| `AD-007` | Detailed browser session/state/storage/mutation safety remains G01-003-owned. | Frozen G01 corpus decomposition |

---

## 7. Quality, Security & Operational Implications

### Security / Trust

- all browser code and browser-delivered configuration are considered observable/modifiable by the user;
- protected authorization remains server-side;
- secrets shall not be embedded in the static artifact;
- Tenant/Institution context remains subject to A04 validation;
- framework route guards or component visibility are never access-control authorities.

### Portability / Deployment

- static artifact deployment permits provider-neutral web hosting;
- no cloud-specific frontend platform is required;
- no persistent Node.js runtime is required for the canonical SPA path;
- hosting may be strengthened by E-family deployment/security controls without changing G01 ownership.

### Maintainability

- one TypeScript/React/React Router compatibility baseline reduces frontend technology proliferation;
- feature modules can evolve internally while sharing G01 boundaries;
- C01 contracts prevent coupling route modules to server implementation topology.

### Performance

Route-level code splitting is permitted to control browser payload growth. No numeric performance budget is invented here; applicable E02/performance architecture may establish measurable targets when authoritative requirements exist.

---

## 8. Assumptions, Open Decisions & Deferred Design

### OD-001 — Detailed Client State & Data-Fetching Mechanism

Exact state-management, server-state caching/data-fetching, browser persistence and safe mutation/retry mechanisms are intentionally not selected here because `L5-G-01-003` owns them.

**Blocking for G01-002:** No.  
**Blocking for complete web implementation:** Yes where implementation reaches those concerns; resolve in G01-003.

### OD-002 — Web Hosting Product

No CDN, web server, cloud hosting product or reverse-proxy product is selected.

**Blocking for G01-002:** No.  
**Decision owner:** applicable deployment/runtime architecture when concrete deployment is designed.

### OD-003 — SSR / Pre-rendering Expansion

Runtime SSR is not part of the immediate architecture. If a later web surface demonstrates a material need for SSR/SSG beyond the approved SPA pattern, `ADR-L5-0015` shall undergo impact review.

**Blocking for immediate authenticated ERP:** No.

---

## 9. Architecture Conformance Requirements

| CR | Requirement |
|---|---|
| `CR-001` | The immediate SutraERP web application shall be implemented in TypeScript using React under `ADR-L5-0015`. |
| `CR-002` | The web route/application framework shall be React Router Framework Mode unless `ADR-L5-0015` is superseded or a governed exception exists. |
| `CR-003` | Runtime SSR shall not be required for authenticated ERP correctness in the canonical immediate architecture. |
| `CR-004` | The production web application shall be deployable as a static web artifact without a mandatory persistent frontend application server. |
| `CR-005` | Valid browser application locations shall be resolvable by the hosting environment to the SPA entry artifact as required by the route runtime. |
| `CR-006` | Route/feature modules shall invoke server capability only through the G01/C01 governed contract-consumption boundary. |
| `CR-007` | Client route guards, component visibility and client capability checks shall not be treated as authoritative authorization enforcement. |
| `CR-008` | Browser-delivered configuration/static assets shall contain no server secret, private key or privileged machine credential. |
| `CR-009` | Tenant/Institution context carried by the web application shall remain non-authoritative until A04/server validation. |
| `CR-010` | Web feature modules shall not independently implement authentication, authorization or Tenant-scope authority. |
| `CR-011` | Independently deployed micro-frontends shall not be introduced without a new approved architecture decision. |
| `CR-012` | Route/feature modules may use code splitting, but code-loading boundaries shall not transfer canonical business ownership. |
| `CR-013` | Detailed session/token storage, client-state persistence, server-state cache and mutation-recovery semantics shall conform to G01-003 and shall not be invented locally by G01-002 implementations. |
| `CR-014` | Connectivity or browser-runtime failure shall not be converted into a fabricated successful business outcome. |
| `CR-015` | L6 UI/UX shall remain free to define user-facing navigation and interaction independently of technical route/source-code organization. |

---

## 10. Traceability & Current Disposition

### Canonical parent

`L5-G-01-001` — Architecture Approved.

### Material peer dependencies and collaboration

```text
L5-G-01-003 — collaborating sibling; client security/session/state/safe consumption
L5-C-01     — released contract boundary
L5-D-01     — authenticated context
L5-D-02     — authorization enforcement
L5-A-04     — Tenant/Institution isolation
```

### Governing architecture decisions

```text
ADR-L5-0015 — Web Client Platform, Rendering & Deployment Pattern
ADR-L5-0003 — Contract-Driven Modular Platform with Selective Isolation
ADR-L5-0004 — Tenant and Institution Isolation Enforcement Pattern
```

### Disposition

**Canonical responsibility:** satisfied.  
**Technology decision:** resolved by `ADR-L5-0015`.  
**Direct L4 dependency:** none required; this is a technology specialization of the approved G01 shared client architecture and does not claim new business behavior. Repository-level BH→L5 coverage remains governed by the canonical traceability methodology and shall not be fabricated by reintroducing rejected direct G01 mappings.  
**G01-002-owned blocking findings:** none.  
**Status:** `Architecture Approved`.  
**Lock status:** not Locked; lock requires separate final governance authorization.


## 2026-09-22 Batch 7C Impact Revalidation Synchronization

Non-material dependency/ADR metadata synchronization only. Current peer lifecycle/version labels and applicable authentication/session governance references now match canonical repository state. Client non-authority, server-side authorization, Tenant/Institution validation, safe contract consumption, online-first posture, L6 separation and future mobile technology deferral are unchanged. No lifecycle promotion is performed.
