# ADR-L5-0015 — Web Client Platform, Rendering & Deployment Pattern

**ADR ID:** `ADR-L5-0015`  
**Title:** Web Client Platform, Rendering & Deployment Pattern  
**Scope:** Specification  
**Decision Owner:** L5-G-01 — Client Platform Architecture  
**Assigned Author:** Dhruv  
**Reviewers:** Lead Architecture / repository governance review  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-15  
**Affected L5 Specifications:** `L5-G-01-002`  
**Related ADRs:** `ADR-L5-0003`, `ADR-L5-0004`  
**Supersedes:** —  
**Superseded By:** —

---

## 1. Decision Context

`L5-G-01-002 — Web Application Architecture` owns the web-client technical architecture for SutraERP's immediate product direction while excluding UI/UX design. The approved `L5-G-01-001` baseline requires the web specialization to preserve a non-authoritative client model, governed contract consumption, Tenant/Institution isolation, authentication/authorization separation and server-owned business truth.

A concrete web platform decision is required so implementation does not independently choose the primary language, framework, rendering strategy, route/application composition model or deployable web artifact. The L5 Technology Selection & ADR Methodology requires ADR governance when selecting a major framework or establishing a shared client platform pattern.

---

## 2. Scope & Affected Architecture

This ADR decides only the immediate SutraERP web application platform governed by `L5-G-01-002`.

It decides:

- primary web language;
- primary UI application framework;
- routing/application framework mode;
- browser/server rendering position;
- deployable web artifact model;
- default web modularity boundary.

It does not decide:

- L6 screens, page composition, navigation taxonomy, visual design or user journeys;
- detailed session/token storage, client state or contract-consumption mechanisms owned by `L5-G-01-003`;
- future mobile architecture owned by `L5-G-01-004`;
- API transport, authentication protocol or backend runtime technology;
- deployment product, CDN, web-server vendor or cloud provider.

---

## 3. Authoritative Constraints

1. `L5-G-01-001` requires a non-authoritative client that consumes released C01 contracts and preserves D01/D02/A04 boundaries.
2. The web architecture must remain deployable in both provider-neutral/on-premises and cloud-capable environments without making a cloud-specific managed frontend service mandatory.
3. The web client must support highly interactive authenticated application behavior without requiring server-side rendering for correctness.
4. The web implementation shall not create a new business-authority or persistence boundary.
5. Technology selection must not force future mobile architecture into the same runtime model.
6. A future material change to the selected framework or rendering/deployment pattern requires ADR impact review.

---

## 4. Decision Criteria

The decision was evaluated against:

- fit with the approved G01 non-authoritative client architecture;
- suitability for large interactive ERP applications;
- type safety and maintainability;
- client modularity and route-level composition;
- static/provider-neutral deployment;
- on-premises operability;
- ability to defer server rendering until a real requirement exists;
- ecosystem maturity and current official framework support;
- operational simplicity;
- replacement/migration cost;
- avoidance of unnecessary runtime infrastructure.

---

## 5. Alternatives Considered

### Option A — React + TypeScript + React Router Framework Mode, SPA-first

A single modular React application written in TypeScript, using React Router Framework Mode with runtime server-side rendering disabled for the authenticated ERP application. Build output is a static deployable web artifact. Route-level code splitting and modular feature composition are permitted; independently deployed micro-frontends are not the default.

### Option B — React + TypeScript + Next.js hybrid SSR/CSR

A React application using Next.js with server/client rendering capabilities and a persistent web runtime where required.

### Option C — Angular + TypeScript SPA

A single Angular application using Angular's integrated application framework, router and dependency-injection model.

### Option D — Remain technology-neutral

Define only logical web components and postpone the framework, language and rendering decision to implementation.

---

## 6. Evaluation

| Criterion | Option A | Option B | Option C | Option D |
|---|---|---|---|---|
| G01 architectural fit | Strong | Strong | Strong | Acceptable |
| Interactive ERP fit | Strong | Strong | Strong | Undecided |
| Static/provider-neutral deployment | Strong | Acceptable/Strong | Strong | Undecided |
| Operational simplicity | Strong | Acceptable | Strong | Undecided |
| Type safety | Strong | Strong | Strong | Undecided |
| Future optional SSR path | Strong | Strong | Acceptable | Undecided |
| Framework governance burden | Acceptable | Higher | Acceptable | Deferred to implementation |
| Avoids unnecessary server runtime | Strong | Weaker when full-stack features used | Strong | Undecided |
| Implementation readiness | Strong | Strong | Strong | Weak |

Official React guidance recommends starting new production React applications with a framework and explicitly states that recommended frameworks can run as CSR/SPAs and static deployments without a runtime server. React Router Framework Mode provides typed route modules, code splitting and SPA/SSR/static rendering strategies. Its SPA mode disables runtime SSR and produces an `index.html` plus client assets suitable for static hosting. TypeScript adds static type syntax and tooling while compiling to JavaScript.

No current repository requirement justifies making server-side rendering, React Server Components or a persistent frontend server runtime mandatory for authenticated SutraERP ERP execution.

---

## 7. Decision

SutraERP `L5-G-01-002` shall use the following immediate web platform:

```text
Language:          TypeScript
UI Framework:      React
Routing Framework: React Router — Framework Mode
Rendering:         SPA-first / client-side runtime rendering
Runtime SSR:       Disabled by default
Build/Framework integration: React Router framework toolchain using its Vite integration
Deployable artifact: Static web application artifact
Application model: One modular web application shell
Micro-frontends:   Not the default architecture
```

### 7.1 Rendering Position

The authenticated ERP application shall operate as a client-side SPA by default.

Runtime SSR is not required for correctness and shall not be introduced merely as a framework default. React Router SPA mode shall be used so the production artifact can be served through provider-neutral static/web hosting.

A later route or product surface may introduce SSR/pre-rendering only after demonstrating a material architectural requirement and completing impact review under this ADR.

### 7.2 Application Modularity

The web application is one deployable application shell with internal route/feature modularity.

Feature modules may be loaded independently within the application and may use route-level code splitting, but application modularity shall not be equated with independently deployed micro-frontends.

A micro-frontend topology would materially change deployment, compatibility, client security and operational boundaries and therefore requires a new architecture decision.

### 7.3 Online-First Position

The immediate web client is online-first.

Local browser state may support presentation and bounded technical continuity, but no offline mechanism may create authoritative business mutation or bypass C01/D01/D02/A04 boundaries. Detailed client state, safe mutation and recovery mechanics remain owned by `L5-G-01-003`.

---

## 8. Consequences

### Positive

- concrete and implementation-ready web platform;
- static deployment supports on-premises and cloud-capable hosting without a mandatory Node.js application server;
- TypeScript provides compile-time contract/type tooling for a large client codebase;
- React Router Framework Mode provides governed route modules, code splitting and an upgrade path if rendering needs change;
- React preserves a strong component ecosystem without coupling business authority to the client;
- one modular application avoids premature distributed frontend deployment complexity.

### Costs / Trade-offs

- React leaves some application concerns to explicit SutraERP standards rather than prescribing all structure itself;
- SPA-first delivery can have a larger initial client payload than server-rendered approaches if modular loading is not governed;
- browser execution means client security controls must assume all client code/state is observable or modifiable by the user;
- static deployment requires host fallback/rewrite behavior so application routes resolve to the SPA entry artifact;
- future SSR adoption will require architecture impact assessment rather than being assumed from day one.

---

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Client bundle growth | Use route/feature modularity and code splitting; performance budgets remain owned by applicable performance architecture when numeric targets are available. |
| Client-side authorization drift | G01-001/D02 remain authoritative; visibility checks are advisory only. |
| Tenant/session state leakage | G01-003 owns detailed state/session isolation; G01-002 provides only the web runtime boundary. |
| Framework-specific coupling | Contract, identity, authorization and scope boundaries remain outside React-specific components through G01 shared boundaries. |
| Static-host route failures | Deployment shall provide SPA fallback to the built entry artifact for valid client routes. |
| Premature micro-frontends | One deployable application remains default; distributed frontend topology requires separate ADR review. |

---

## 10. Migration / Exit Position

The selected stack shall preserve migration seams at:

- C01 contract-consumption boundary;
- G01-003 security/session/state boundary;
- route/feature module boundaries;
- UI component boundary from business/server authority.

A future replacement of React or React Router shall not require changing canonical backend contracts, authorization semantics, Tenant isolation or business ownership.

---

## 11. Traceability

- `L5-G-01-001` — Architecture Approved shared client-platform baseline.
- `L5-G-01-002` — frozen responsibility for immediate web-client technical architecture.
- `L5-C-01` — governed synchronous contract boundary.
- `L5-D-01` — authenticated technical context.
- `L5-D-02` — authorization decision/enforcement.
- `L5-A-04` — Tenant/Institution isolation and scope propagation.
- `ADR-L5-0003` — contract-driven modular platform.
- `ADR-L5-0004` — Tenant/Institution isolation enforcement.

External product evidence is feasibility/technology evidence only and does not override repository architecture.

---

## 12. Approval Record

**Decision requested:** 2026-09-15  
**Decision selected:** Option A — React + TypeScript + React Router Framework Mode, SPA-first static web application.  
**Approval source:** Explicit repository-authority authorization in the SutraERP architecture workflow to proceed with the recommended decision.  
**Architecture review result:** No conflict identified with G01-001, C01, D01, D02, A04, ADR-L5-0003 or ADR-L5-0004.  
**Formal Status:** `Architecture Approved`  
**Lock Status:** Not Locked; lock requires separate final governance authorization.
