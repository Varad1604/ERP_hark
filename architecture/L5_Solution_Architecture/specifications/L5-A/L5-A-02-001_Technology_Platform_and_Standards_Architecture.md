# L5-A-02-001 — Technology Platform & Standards Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-02-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-02 — Technology Platform & Standards Architecture  
**Canonical Technical Responsibility:** Governed technology-selection criteria and approved standards after ADR decisions; no silent framework/product lock.  
**Technical Architecture Owner:** L5-A-02 — Technology Platform & Standards Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.1`  
**Status:** `Locked`  
**Last Updated:** `2026-09-16`  
**Depends On:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; L5 Dependency-Gated Parallel Authoring Execution Timeline; `L5-A-01-001`; applicable material peer L5 specifications and approved ADRs  
**Referenced By:** Server-side implementation architecture and technology-governed L5 consumers  
**Applicable ADRs:** `ADR-L5-0003`; `ADR-L5-0015`; `ADR-L5-0018`  
**Governance Authorization:** `L5_Dhruv_Stage6_A02_ADR0018_Independent_Architecture_Approval_Decision_2026-09-16.md`  
**Governance Lock:** `L5_Dhruv_Stage6_Final_Lock_and_Completion_Decision_2026-09-16.md`

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification is the repository consolidation point for governed SutraERP technology standards.

It converts approved technology ADRs and peer-owned technology decisions into one implementation-facing standards position without re-deciding another L5 owner's architecture.

### In Scope

A02 owns:

- primary platform/language standards;
- classification of approved, unresolved and exception technologies;
- repository-wide technology-standardization rules;
- toolchain support/version-policy principles for A02-owned platforms;
- consolidation of approved peer technology decisions;
- governance boundary for technology exceptions.

### Out of Scope / Non-Goals

A02 does not own another area's technical architecture merely because that architecture uses a named product.

Specifically, A02 does not independently select database, messaging, search, object storage, workflow/orchestration, IAM, cryptography, observability, deployment/orchestration, CI/CD or client technology where another canonical L5 owner owns the decision.

A02 does not define L6 UI/UX.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Status | Exact Responsibility | A02 Use |
|---|---|---|---|
| L5 Candidate Document Corpus Register | Architecture Approved | `L5-A-02-001` owns governed technology-selection criteria and approved standards after ADR decisions | Establishes A02 canonical ownership |
| L5 Technology Selection & ADR Methodology | Locked | Programming language, runtime, major framework and material shared technology selections require ADR governance | Governs technology admission |
| L5 Dependency-Gated Parallel Authoring Timeline | Locked | A02 is Stage-6 consolidation after major architecture and ADR decisions mature | Prevents premature product selection |
| `ADR-L5-0003` | Architecture Approved | Contract-Driven Modular Platform with Selective Isolation | Technology standards must preserve approved application structure |
| `L5-A-01-001` | Draft | Programming language/framework/product selection is delegated to A02 and applicable ADRs | Establishes A01/A02 boundary |
| `L5-E-04-001` | Draft | Runtime packaging/deployment remains product-neutral; primary runtime and orchestration technologies require separate governance | Prevents A02 from redefining E04 deployment ownership |
| `ADR-L5-0015` | Architecture Approved | Immediate web stack is TypeScript + React + React Router Framework Mode, SPA-first static artifact | Imported as an existing approved technology standard |
| `ADR-L5-0018` | Architecture Approved | Rust primary server platform + governed runtime exceptions + pinned stable toolchain | Establishes normative backend technology standard |

### 3.2 Authoritative Constraints

**C-001 — ADR-first technology governance.**  
A material technology shall not become a repository standard before the required ADR or canonical owner decision is approved.

**C-002 — Consolidation does not transfer ownership.**  
A02 records approved peer technology decisions but does not become owner of the underlying persistence, messaging, security, deployment, client or operational architecture.

**C-003 — Technology standards shall preserve architecture boundaries.**  
Named technologies shall not weaken module, contract, Tenant/Institution, persistence, security or deployment boundaries.

**C-004 — Product absence is explicit.**  
Where the canonical owner has not approved a product, A02 shall record the position as unresolved rather than inferring a technology.

**C-005 — Client/backend independence.**  
Server platform selection shall not force web or mobile clients into the same programming language/runtime.

### 3.3 Assumptions

No material assumption is used to establish an approved technology standard.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

A02 owns the consolidated technology-standard status of a technology after the appropriate architecture authority has decided it.

A02 also owns the repository-wide primary server-platform standard and its exception/version policy established by `ADR-L5-0018`.

### Consumes / Collaborates With

| Owner | Technology responsibility consumed by A02 |
|---|---|
| A01/A03 | application/module architecture constraints |
| B01 | search/index technology decisions |
| B02 | database/persistence technology decisions |
| B04 | document/media/object-storage technology decisions |
| C02 | broker/messaging technology decisions |
| D01 | IAM technology decisions |
| D03 | security/cryptographic technology decisions |
| A05 | workflow/orchestration technology decisions |
| E04 | packaging/container/orchestration technology decisions |
| E05 | observability technology decisions |
| F01 | build/supply-chain/release tooling constraints |
| G01 | web/mobile client platform decisions |

A02 shall not select these products on behalf of their owners.

---

## 5. Technology Standards Architecture

### 5.1 Standard Classification

Every material technology position consolidated by A02 shall have one of four states:

```text
APPROVED STANDARD
    Approved by canonical owner / required ADR
    and normative for its declared scope.

APPROVED EXCEPTION
    A governed deviation from a broader standard.

UNRESOLVED
    Architecture exists, but named technology selection
    has not received required authority.

NON-AUTHORITATIVE / HISTORICAL
    Evidence or prior implementation only;
    not an active architecture standard.
```

Implementation shall not promote `UNRESOLVED` or `NON-AUTHORITATIVE / HISTORICAL` technology into a repository standard by convention.

### 5.2 Current Consolidated Standards Matrix

| Technology Area | Current A02 Position | Canonical Decision Owner / Authority |
|---|---|---|
| Primary server language/toolchain | **Rust — APPROVED STANDARD** | `ADR-L5-0018` |
| Server technology policy | **Rust primary, governed exceptions — APPROVED STANDARD** | `ADR-L5-0018` |
| Rust toolchain lifecycle | **Pinned stable toolchain with controlled promotion — APPROVED STANDARD** | `ADR-L5-0018` |
| Backend framework | `UNRESOLVED` | A02 / applicable framework ADR |
| Rust async runtime | `UNRESOLVED` | A02 / applicable runtime ADR if material |
| Database product | `UNRESOLVED` | B02 + ADR |
| Database access/ORM technology | `UNRESOLVED` unless B02 establishes a shared standard | B02 |
| Messaging/broker product | `UNRESOLVED` | C02 + ADR |
| Search/index product | `UNRESOLVED` | B01 + ADR |
| Object-storage product | `UNRESOLVED` | B04 + ADR |
| Workflow/orchestration product | `UNRESOLVED` | A05 + ADR |
| IAM product | `UNRESOLVED` | D01 + ADR |
| Security/cryptographic products | `UNRESOLVED` unless explicitly approved by D03 | D03 + ADR |
| Container/orchestration platform | `UNRESOLVED` | E04 + ADR |
| Observability platform | `UNRESOLVED` | E05 + ADR |
| CI/CD/tooling platform | `UNRESOLVED` unless F01 approves one | F01 |
| Web client language | **TypeScript — APPROVED STANDARD** | `ADR-L5-0015` |
| Web application framework | **React — APPROVED STANDARD** | `ADR-L5-0015` |
| Web routing/application mode | **React Router Framework Mode — APPROVED STANDARD** | `ADR-L5-0015` |
| Web rendering/deployment position | **SPA-first static web artifact — APPROVED STANDARD** | `ADR-L5-0015` |
| Mobile client platform | `UNRESOLVED` | G01 mobile specialization |

### 5.3 Primary Server Standard

The approved primary server standard is:

```text
Primary server platform
        ↓
Rust

Default rule
        ↓
New server-side implementation uses Rust

Exception required?
        ↓
Demonstrate material technical reason
        ↓
Canonical-owner review
        ↓
ADR where material
        ↓
Approved non-Rust exception
```

This standard does not select a backend framework or async runtime.

### 5.4 Rust Toolchain Standard

1. Production uses a repository-approved pinned stable Rust toolchain.
2. Production does not automatically consume each newly released stable compiler.
3. Toolchain changes pass F01/F02 build, security, compatibility and verification gates.
4. Nightly Rust is outside the standard production baseline.
5. Material nightly dependency requires an approved exception.
6. Exact compiler patch/version belongs to governed build/release configuration rather than being frozen permanently in this L5 document.

### 5.5 Rust Safety Standard

Safe Rust is the default code boundary.

Material use of `unsafe` shall be isolated and justified through explicit safety invariants and review evidence.

This requirement protects the architectural rationale for selecting Rust without making A02 owner of code-review procedures or implementation-level coding style.

### 5.6 Technology Exception Boundary

An exception does not authorize an alternative architecture.

A technology exception must continue to comply with canonical:

- business/domain ownership;
- A01/A03 module boundaries;
- A04 Tenant/Institution isolation;
- B02 persistence ownership;
- C01/C02 contracts;
- D01/D02/D03 identity/security controls;
- E01/E02/E04 reliability/capacity/deployment architecture;
- F01/F02 build and verification gates.

An exception changes the implementation technology, not the architectural authority.

---

## 6. Architecture Decisions

| Decision | Position | Status |
|---|---|---|
| `AD-A02-001` | A02 is the consolidation authority for approved technology standards, not the decision owner for every technology family. | Established from corpus/governance |
| `AD-A02-002` | Technology states are explicitly classified as approved, exception, unresolved or non-authoritative/historical. | A02 technical derivation |
| `AD-A02-003` | Rust is the primary server platform with ADR-governed exceptions. | `ADR-L5-0018` |
| `AD-A02-004` | Production Rust uses a pinned stable toolchain with controlled upgrade promotion. | `ADR-L5-0018` |
| `AD-A02-005` | Existing approved web standard from `ADR-L5-0015` is inherited without re-selection. | Approved ADR-derived |
| `AD-A02-006` | Peer-owned products remain unresolved until their canonical owners approve them. | Governance-derived |

---

## 7. Conformance Requirements

**CR-A02-001** — No material technology requiring ADR governance shall be represented as an approved standard before the applicable ADR is approved.

**CR-A02-002** — A02 shall not select a peer-owned technology product without the canonical owner's decision authority.

**CR-A02-003** — An approved technology exception shall preserve all canonical ownership, isolation, persistence, contract, security and deployment boundaries.

**CR-A02-004** — Historical implementation technologies shall not be treated as canonical standards unless separately approved through current L5 governance.

**CR-A02-005** — Backend technology selection shall not require web or mobile clients to share the backend programming language/runtime.

**CR-A02-006** — New server-side implementations shall use Rust unless an applicable governed exception is approved.

**CR-A02-007** — Production Rust builds shall use the repository-approved stable toolchain rather than an uncontrolled toolchain version.

**CR-A02-008** — A toolchain upgrade shall pass the applicable build, security, compatibility and verification gates before production promotion.

**CR-A02-009** — `unsafe` Rust shall not become an uncontrolled general implementation mechanism; its safety boundary shall be explicit and reviewable.

**CR-A02-010** — Unresolved database, broker, search, object-storage, IAM, orchestration, observability and deployment products shall remain visibly unresolved until their canonical decisions exist.

---

## 8. Open Decisions & Deferred Design

| ID | Item | Owner | Blocking? |
|---|---|---|---|
| `OD-A02-001` | Backend framework selection | A02 / ADR governance if material | No |
| `OD-A02-002` | Rust asynchronous runtime selection | A02 / ADR governance if material | No |
| `OD-A02-003` | Database product | B02 | No; must remain unresolved |
| `OD-A02-004` | Broker/messaging product | C02 | No; must remain unresolved |
| `OD-A02-005` | Container/orchestration platform | E04 | No; must remain unresolved |
| `OD-A02-006` | Other peer-owned product selections | Applicable canonical owner | No; must remain unresolved |

These unresolved items do not block A02's approved standards architecture because A02 explicitly preserves their canonical ownership and does not invent a product choice.

---

## 9. Traceability

A02 does not claim ownership of a business behavior or independent L4 behavioral outcome.

Its authoritative purpose is the frozen `L5-A-02-001` repository responsibility and Stage-6 requirement to consolidate approved technology decisions after architecture and ADR maturity.

L4→L5 relations shall be recorded only where an approved L4 BH materially depends on an A02 technology-standard contribution. A02 shall not self-map broadly to L4 merely because all implementation uses technology.

Material downstream architecture links include:

`L5-A-01-*`, `L5-A-03-*`, `L5-B-01-*`, `L5-B-02-*`, `L5-B-04-*`, `L5-C-02-*`, `L5-D-01-*`, `L5-D-03-*`, `L5-E-04-*`, `L5-E-05-*`, `L5-F-01-*`, `L5-G-01-*`.

---

## 10. Approval Disposition

`L5-A-02-001` is `Locked` under `L5_Dhruv_Stage6_Final_Lock_and_Completion_Decision_2026-09-16.md`.

The lock preserves the primary server-platform decision established by `ADR-L5-0018` and does not convert unresolved peer-owned products into standards. Those choices remain explicitly deferred to their canonical technical owners and applicable ADR governance.

Material change requires governed impact assessment and controlled relock.
