# ADR-L5-0018 — Primary Server Platform, Rust Standardization & Governed Runtime Exception Pattern

**ADR ID:** `ADR-L5-0018`  
**Title:** Primary Server Platform, Rust Standardization & Governed Runtime Exception Pattern  
**Scope:** Repository  
**Decision Owner:** L5-A-02 — Technology Platform & Standards Architecture  
**Assigned Author:** Dhruv  
**Reviewers:** Lead Architecture / repository governance review  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-16  
**Affected L5 Specifications:** `L5-A-02-001`; constrains server-side SutraERP implementations and technology selections  
**Related ADRs:** `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0013`; `ADR-L5-0014`; `ADR-L5-0015`  
**Supersedes:** None  
**Superseded By:** None

---

## 1. Decision Context

SutraERP requires a canonical primary server-side technology position so implementation teams do not independently select incompatible programming languages and runtime ecosystems.

The decision must preserve the repository's approved Contract-Driven Modular Platform with Selective Isolation, support provider-neutral and on-premises-capable deployment, support high-concurrency and computation-heavy workloads, remain compatible with background execution and resilience architecture, and avoid unnecessary technology proliferation.

The decision does not select persistence, messaging, search, workflow, IAM, observability, deployment-orchestration or other peer-owned products.

---

## 2. Scope & Affected Architecture

This decision governs:

- primary server-side programming language/toolchain;
- repository-wide default language position;
- governance for alternative server runtimes;
- production toolchain-version policy;
- language-specific production safety constraints.

It does not select:

- backend HTTP/application framework;
- Rust asynchronous runtime;
- ORM/database access library;
- database product;
- cache product;
- message broker;
- search/index product;
- object-storage product;
- IAM product;
- observability product;
- container/orchestration platform;
- cloud provider;
- mobile-client platform;
- CI/CD vendor.

Those decisions remain with their canonical L5 owners and applicable ADR governance.

---

## 3. Authoritative Constraints

1. `ADR-L5-0003` establishes a Contract-Driven Modular Platform with Selective Isolation.
2. Server technology shall not change canonical business ownership, persistence ownership, Tenant/Institution isolation or contract boundaries.
3. A business-domain boundary does not automatically require a distinct runtime technology.
4. Technology selection shall remain compatible with provider-neutral and on-premises-capable runtime realization.
5. The primary technology shall support interactive workloads, background execution, concurrent processing and specialized computational workloads without changing A08/A07 ownership.
6. Alternative languages shall not be introduced merely by team preference.
7. Client platforms remain independently governed; selecting Rust for the backend shall not require web or mobile clients to use Rust.
8. Exact database, broker, deployment and other peer-owned products remain outside this ADR.

---

## 4. Decision Criteria

The primary server platform was evaluated against:

- memory and concurrency safety;
- predictable long-running production operation;
- resource efficiency;
- high-concurrency server workloads;
- computation-heavy workloads;
- deployment portability;
- on-premises operability;
- modular application composition;
- background processing suitability;
- security and supply-chain governability;
- maintainability over the expected ERP lifetime;
- ecosystem maturity;
- engineering-team capability;
- ability to preserve future technology exceptions without uncontrolled polyglot proliferation.

---

## 5. Alternatives Considered

### Option A — Rust primary platform with ADR-governed exceptions

Rust is the canonical server-side language/toolchain. Another language/runtime may be introduced only where a material architecture need justifies the deviation and applicable ADR governance approves it.

**Selected.**

### Option B — .NET primary platform

Provides strong enterprise ecosystem, long-term servicing model, mature tooling and broad staffing availability.

Not selected as the primary platform because the chosen architecture prioritizes Rust's memory/thread-safety characteristics, efficient native execution, predictable resource behavior and suitability for concurrency/computation-heavy workloads.

### Option C — Java primary platform

Provides mature enterprise libraries, broad integration capability and extensive engineering availability.

Not selected as the primary platform for the same repository-specific reasons as Option B.

### Option D — Fully polyglot server platform

Allows individual components to choose independent languages.

Rejected because it increases build, security, observability, staffing, deployment and maintenance complexity and conflicts with the repository goal of avoiding unnecessary technology proliferation.

### Option E — Rust mandatory without exceptions

Requires every server-side capability to use Rust.

Rejected because future specialized requirements or external technology constraints may justify another runtime. Such deviations must be governed rather than categorically prohibited.

---

## 6. Decision

SutraERP shall adopt **Rust as its primary server-side programming language and toolchain platform**.

### 6.1 Primary Platform

```text
Primary server language: Rust
Toolchain channel: Stable
Production toolchain: Repository-approved pinned stable version
Upgrade model: Controlled verification and promotion
Alternative server runtime: Exception only through governed architecture decision
```

This decision does not select Axum, Tokio or any other Rust framework/runtime library.

### 6.2 Technology Exception Model

A server-side capability may use a non-Rust implementation only when a material requirement demonstrates that the primary Rust platform is unsuitable or creates disproportionate architecture risk.

A technology exception shall identify:

- affected canonical L5 owner;
- requirement that cannot reasonably be met by the primary platform;
- interoperability boundary with the Rust platform;
- operational and security consequences;
- deployment consequences;
- migration/exit position.

Where the alternative introduces a material runtime/platform dependency, repository precedent, portability impact or significant technology lock-in, ADR approval is mandatory.

### 6.3 Stable Toolchain Policy

Production builds shall use a repository-approved pinned **stable** Rust toolchain.

Production shall not automatically advance merely because a new Rust stable release exists.

A toolchain upgrade shall pass the governed build, verification, security, compatibility and performance gates before promotion.

Nightly Rust features are not part of the standard production baseline. Material dependence on nightly-only capability requires explicit architecture exception review.

### 6.4 Rust Safety Position

Safe Rust is the default implementation boundary.

Use of `unsafe` Rust shall be:

- limited to a justified technical boundary;
- isolated from ordinary domain/application code where feasible;
- reviewable;
- accompanied by explicit safety invariants;
- included in security and verification evidence.

A dependency containing `unsafe` code is not automatically prohibited, but supply-chain and verification controls shall account for the reduced compiler-enforced safety boundary.

### 6.5 Bounded Resource Position

Rust's concurrency efficiency shall not be used to justify unbounded work admission.

Request concurrency, background execution, database access, external integrations and computational workloads remain subject to the canonical capacity, persistence, background-execution and resilience architectures.

---

## 7. Integration Boundaries

### A02 → A01/A03
Rust realizes the approved modular application architecture without redefining module or ownership boundaries.

### A02 → B02
Rust does not select or alter the persistence product, transaction model or concurrency ownership defined by B02.

### A02 → C01/C02
Rust implementations consume canonical synchronous and asynchronous contracts. This ADR does not select protocol libraries or messaging products.

### A02 → A08/A07
Rust is available to background-processing and computation-engine implementations while A08/A07 retain execution semantics and isolation ownership.

### A02 → E04
E04 retains packaging, runtime-unit, deployment and orchestration ownership. Rust shall remain compatible with supported deployment profiles.

### A02 → F01/F02
Toolchain and dependency changes pass governed supply-chain, build, verification and release controls.

### A02 → G01
Rust is a backend decision only. Web and mobile clients remain independently governed through G01 and communicate through canonical contract boundaries.

---

## 8. Consequences

### Positive

- one primary server platform reduces uncontrolled technology proliferation;
- Rust provides strong compiler-enforced memory and thread-safety properties;
- native execution supports efficient long-running server and worker workloads;
- the platform is suitable for high-concurrency and computation-heavy workloads;
- native deployable artifacts support portable/on-premises-capable runtime models;
- exceptions remain possible without turning SutraERP into an unmanaged polyglot platform.

### Costs / Trade-offs

- Rust has a steeper engineering learning curve than common enterprise managed runtimes;
- enterprise-specific libraries and vendor SDK coverage may be narrower than Java/.NET;
- large Rust workspaces require build-performance discipline;
- the Rust release model requires SutraERP to operate its own controlled toolchain-promotion policy rather than depending on a conventional multi-year language LTS branch;
- production observability/integration maturity must be validated for selected libraries before implementation adoption;
- recruiting and maintaining sufficient Rust expertise is an organizational requirement.

---

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Rust expertise becomes scarce | Maintain platform conventions, reusable libraries, architecture documentation and training; allow governed exceptions where justified. |
| Build times degrade at repository scale | Preserve modular workspace/dependency boundaries and use F01-controlled build optimization/caching. |
| Vendor SDK unavailable in Rust | Prefer standards-based integration or a governed adapter/exception rather than contaminating core ownership. |
| Toolchain upgrades introduce regression | Pin production stable toolchain and promote upgrades only through F01/F02 evidence. |
| Unsafe code weakens guarantees | Restrict and isolate `unsafe`; require explicit safety invariants and review. |
| Concurrency overwhelms dependencies | Apply E02/A08/B02 bounded capacity and admission controls. |
| Rust-specific implementation leaks into contracts | C01/C02 contracts remain technology-independent. |
| Primary platform becomes irreversible lock-in | Preserve contract/module boundaries and ADR-governed exception/replacement path. |

---

## 10. Migration / Exit Position

A future replacement of Rust as the primary server platform shall require a superseding repository ADR.

The architecture shall preserve migration seams at module, synchronous-contract, asynchronous-contract, persistence, deployment and external-integration boundaries so changing the implementation language does not require changing canonical business ownership or L4 behavior.

---

## 11. Traceability

Primary realization: `L5-A-02-001 — Technology Platform & Standards Architecture`.

Material collaborating architecture:

`L5-A-01-*`, `L5-A-03-*`, `L5-A-07-*`, `L5-A-08-*`, `L5-B-02-*`, `L5-C-01-*`, `L5-C-02-*`, `L5-E-02-*`, `L5-E-04-*`, `L5-F-01-*`, `L5-F-02-*`, `L5-G-01-*`.

Applicable approved decisions include:

- `ADR-L5-0003` — Contract-Driven Modular Platform with Selective Isolation.
- `ADR-L5-0013` — Governed Background Job, Work-Unit & Technical Retry Execution Pattern.
- `ADR-L5-0014` — Repository Resilience, Health, Degradation & Recovery Pattern.
- `ADR-L5-0015` — Web Client Platform, Rendering & Deployment Pattern.

---

## 12. Approval Record

**Decision requested:** 2026-09-16.  
**Decision selected:** Rust primary server platform; Rust-primary with ADR-governed exceptions; pinned stable Rust toolchain with controlled upgrades.  
**Approval source:** Explicit Lead-Architect decision selection and repository-write authorization in the SutraERP architecture workflow on 2026-09-16.  
**Architecture review result:** No conflict identified with canonical L5 ownership, `ADR-L5-0003`, `ADR-L5-0013`, `ADR-L5-0014`, `ADR-L5-0015`, or peer-owned technology boundaries.  
**Formal Status:** `Architecture Approved`  
**Lock Status:** Not Locked; lock requires separate final governance authorization.
