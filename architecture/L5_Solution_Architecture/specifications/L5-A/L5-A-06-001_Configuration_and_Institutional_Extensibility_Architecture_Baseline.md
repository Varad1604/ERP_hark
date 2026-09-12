# Configuration & Institutional Extensibility Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-A-06-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-06 — Configuration, Feature & Institutional Extensibility Architecture  
**Canonical Technical Responsibility:** Governed configuration and extension mechanisms while upstream domains retain ownership of business policy and configuration meaning.  
**Technical Architecture Owner:** L5-A-06 — Configuration, Feature & Institutional Extensibility Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-12`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; `L5-A-01-001` v1.0.1 Draft; `L5-A-03-001` v1.0.1 Draft; `L5-A-04-001` v1.0.1 Draft; `L5-A-04-002` v1.0.1 Draft; `L2-A01` v1.0.0 Locked; applicable L4 sources identified in §3.  
**Referenced By:** `L5-A-06-002`; `L5-A-06-003`  
**Applicable ADRs:** `ADR-L5-0003` — Contract-Driven Modular Platform with Selective Isolation; `ADR-L5-0004` — Tenant and Institution Isolation Enforcement Pattern.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the common technical control plane through which SutraERP:

- registers configurable capabilities;
- governs configuration changes and versions;
- validates configuration without acquiring business-policy ownership;
- establishes safe effective-use and consumption boundaries;
- exposes governed extension points;
- prevents institutional extensibility from bypassing canonical application, ownership, security or persistence boundaries.

It provides the common baseline for `L5-A-06-002` and `L5-A-06-003`.

### In Scope

- versioned configuration-definition registration;
- configuration ownership binding;
- governed configuration-change admission;
- immutable configuration versions and predecessor/successor lineage;
- common technical validation and canonical-owner validation handoff;
- effective-use admission;
- concurrent configuration-publication protection;
- immutable configuration context for an execution;
- governed extension-point registration;
- extension contract and compatibility baseline;
- extension admission boundary;
- classification of configuration versus controlled extension;
- Tenant/Institution isolation across configuration and extension operations.

### Out of Scope / Non-Goals

- configuration scope hierarchy, inheritance, overrides and effective dating — `L5-A-06-002`;
- feature activation/deactivation and controlled extension execution mechanics — `L5-A-06-003`;
- business configuration meaning and business policy — canonical upstream owners;
- business approval authority;
- authorization decision implementation — L5-D-02;
- persistence technology, physical data model or transaction technology — L5-B-02;
- synchronous/event transport realization — L5-C;
- secrets, certificates and machine credentials — `L5-D-03-004`;
- audit-platform internals — L5-D-04;
- runtime/deployment topology — L5-E-04;
- UI/UX administration experience — L6.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-A-06-001 Use |
|---|---|---|---|---|---|
| L4 | `L4-A-056` | v1.0.0 / Draft | `BH-001`, `BH-002`, `BH-004`, `BH-007`, `BH-008`, `BH-009` | Configuration has one owner/scope, preserves predecessor versions, only governed candidates become effective, consumers receive read-only context and evidence remains traceable. | Primary basis for configuration definition, versions, validation, effective-use and read-only consumption. |
| L4 | `L4-A-056` | v1.0.0 / Draft | `BH-003`, `BH-005`, `BH-006` | Inheritance, eligible Active version and deterministic Tenant-aware resolution are required. | Detailed resolution delegated to `L5-A-06-002`. |
| L4 | `L4-B-011` | v1.0.0 / Review | `BH-001`–`BH-006` | Explicit scope, effective-version control, history and unambiguous governing configuration. | Demonstrates reusable common configuration architecture. |
| L4 | `L4-D-046` | v1.0.0 / Review | `BH-001`, `BH-003`–`BH-008` | Domain-owned configuration requires versioning, validation, approval gating and predecessor history. | Requires common mechanics without ownership transfer. |
| L4 | `L4-E-074` | v1.0.1 / Draft | `BH-001`–`BH-008` | Payroll configuration is governed and versioned while source policies stay externally owned. | Requires configuration framework to reference rather than absorb source-owned policy. |
| L4 | `L4-G-092` | v1.0.1 / Draft | `BH-001`–`BH-007`, `BH-009` | Operational configuration cannot silently become institution-wide policy. | Requires configuration/extension ownership boundaries. |
| L4 | `L4-G-123` | v1.0.1 / Draft | `BH-001`–`BH-006` | Governed configuration change, effective adoption, history and domain ownership preservation. | Reinforces reusable configuration infrastructure. |
| L2 | `L2-A01` | v1.0.0 / Locked | §§6.1–6.16 | Platform owns the common Configuration Framework; each layer/domain owns its configuration meaning; configuration is validated, version-consistent and read-only during consumption. | Canonical framework and consumption boundary. |
| L5 Corpus | `L5-A-06-001` | Frozen candidate baseline | Canonical responsibility | Governed configuration **and extension mechanisms** while upstream domains own business policy. | Requires both configuration and extensibility baseline architecture. |
| L5 Peer | `L5-A-01-001` | v1.0.1 / Draft | Application composition architecture | Modular Application Core with selective isolation. | A06 logical elements do not automatically imply separate deployment units. |
| L5 Peer | `L5-A-03-001` | v1.0.1 / Draft | §§5.2–5.5; `CR-004`, `CR-007`, `CR-008` | Cross-parent interaction must use governed public boundaries; cross-owner direct persistence is prohibited. | Constrains extension points and extension contracts. |
| L5 Peer | `L5-A-04-001` | v1.0.1 / Draft | isolation decisions/conformance | Tenant is the protected root scope. | Configuration and extension admission remain Tenant-safe. |
| L5 Peer | `L5-A-04-002` | v1.0.1 / Draft | §§5.1–5.7 | Validated Tenant/Institution scope survives execution boundaries. | Supplies execution-scope semantics. |
| L5 Peer | `L5-D-03-004` | v1.0.1 / Draft | §4.4; `AD-004` | Credential material is not ordinary configuration. | Prevents configuration propagation from becoming a credential mechanism. |

### 3.2 Authoritative Constraints

| ID | Constraint | Basis | L5 Effect |
|---|---|---|---|
| `C-001` | Every configuration item has one canonical owner. | L2-A01 §6; applicable L4 | Shared mechanics never acquire configuration meaning. |
| `C-002` | Historical configuration meaning shall not be destructively rewritten. | `L4-A-056-BH-002`; `L4-B-011-BH-005` | Version lineage is mandatory. |
| `C-003` | Only validated/governed configuration may enter effective use. | L2-A01 §§6.9–6.10 | Effective-use gate is mandatory. |
| `C-004` | Resolved configuration is Tenant-aware, version-consistent and read-only during execution. | L2-A01 §6.14 | Execution receives an immutable resolved configuration context. |
| `C-005` | Cross-owner technical interaction cannot target peer internals or persistence. | `L5-A-03-001` | Extensions operate only through registered public boundaries. |
| `C-006` | Tenant/Institution isolation remains authoritative. | A04 / `ADR-L5-0004` | Configuration/extension operations fail closed on invalid protected scope. |
| `C-007` | Secrets and machine credentials are not ordinary configuration. | `L5-D-03-004` | Protected credential material is excluded. |
| `C-008` | Detailed inheritance/effective dating and extension execution have separate A06 owners. | Frozen L5 corpus | A06-001 defines the common control plane; A06-002/A06-003 specialize it. |

### 3.3 Assumptions

No material architecture assumption is required.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

`L5-A-06-001` owns:

- Configuration Definition Registry semantics;
- Configuration Definition versioning and compatibility baseline;
- Configuration Change Boundary;
- Configuration Validation Coordinator;
- Configuration Version Repository logical boundary;
- Effective-Use Gate;
- immutable Execution Configuration Context;
- Extension Point Registry semantics;
- Extension Contract baseline;
- Extension Admission Boundary;
- configuration-versus-controlled-extension classification.

### Consumes / Collaborates With

| Exact L5 Specification | Capability | Boundary |
|---|---|---|
| `L5-A-01-001` | Application composition | Logical A06 elements inherit approved application composition. |
| `L5-A-03-001` | Module/public technical boundaries | Extension points may expose governed boundaries but cannot redefine module ownership/dependencies. |
| `L5-A-04-001/002` | Tenant/Institution isolation/context | Supplies protected scope. |
| `L5-A-06-002` | Scope/inheritance/effective dating | Resolves which eligible configuration applies. |
| `L5-A-06-003` | Feature activation/controlled extension execution | Executes admitted controlled extensions. |
| `L5-B-02-*` | Persistence/concurrency/transactions | Realizes version persistence and atomic conflict protection. |
| `L5-C-01-*` | Technical contracts | Realizes synchronous configuration/extension boundaries where needed. |
| `L5-D-02-*` | Authorization | Supplies protected-operation authorization decisions. |
| `L5-D-03-004` | Secrets/credentials | Owns protected technical credential material. |
| `L5-D-04-*` | Audit/evidence | Owns technical evidence infrastructure. |
| `L5-E-04-*` | Runtime/deployment | Owns physical runtime placement. |

### Must Not Own

A06-001 shall not own business rules, business approval authority, domain configuration meaning, hierarchy/precedence algorithms, feature activation, extension execution internals, module dependency ownership, persistence technology, secrets, deployment topology or UI.

---

# 5. Solution Architecture

## 5.1 Baseline Control-Plane Model

```text
                         A06-001

        ┌───────────────────────────────────┐
        │ Configuration Definition Registry │
        └──────────────────┬────────────────┘
                           │
                           ▼
              Configuration Change Boundary
                           │
                           ▼
             Configuration Validation Coordinator
                           │
                           ▼
              Configuration Version Repository
                           │
                           ▼
                    Effective-Use Gate
                           │
                           ▼
                     L5-A-06-002
                  Effective Resolution
                           │
                           ▼
            Execution Configuration Context


        ┌───────────────────────────────────┐
        │      Extension Point Registry     │
        └──────────────────┬────────────────┘
                           │
                           ▼
                 Extension Contract
                           │
                           ▼
               Extension Admission Boundary
                           │
                           ▼
                     L5-A-06-003
             Controlled Extension Execution
```

Both branches remain under A04 scope enforcement and A03 technical-boundary rules.

---

## 5.2 Technical Realization Model

| Element | Type | Responsibility | Material State / Context | Output / Handoff | Failure Boundary |
|---|---|---|---|---|---|
| **Configuration Definition Registry** | Logical component | Registers configurable capability identity, canonical owner, definition version and validation responsibility. | Definition versions and compatibility lineage. | Definition used for candidate validation. | Ambiguous owner or incompatible/unknown definition rejected. |
| **Configuration Change Boundary** | Control point | Admits a change only against a registered definition and protected execution scope. | Candidate, definition reference, predecessor/basis version, A04 context. | Candidate validation request. | Missing definition/scope/basis prevents continuation. |
| **Configuration Validation Coordinator** | Logical component | Applies generic structural validation and delegates domain-semantic validation to canonical owner. | Validation result/evidence references. | Eligible candidate or rejection. | Platform cannot override owner rejection or invent policy validation. |
| **Configuration Version Repository** | Logical state boundary | Preserves immutable configuration versions and lineage. | Definition-version reference, configuration version, predecessor relation, governance evidence reference. | Version supplied to effective-use gate. | Historical meaning cannot be rewritten. |
| **Effective-Use Gate** | Control point | Prevents unvalidated/unapproved/ineligible versions entering resolution. | Eligibility/governance result. | Eligible version to A06-002. | Missing eligibility preserves prior authoritative position. |
| **Execution Configuration Context** | Immutable execution context | Carries the resolved configuration version set used by one execution/resolution context. | Scope, resolved configuration-version references, resolution provenance. | Read-only context consumed by executing capability. | Context cannot silently mutate after establishment. |
| **Extension Point Registry** | Logical component | Registers explicitly supported places where governed extensions may attach. | Extension-point identity, canonical technical owner, governing contract/version, allowed extension category, scope requirements. | Registered public extension boundary. | Unknown/unregistered attachment point prohibited. |
| **Extension Contract** | Architecture contract | Defines the governed technical boundary between extension and owning capability. | Contract identity/version, owner, required execution context and permitted interaction categories. | Contract consumed by extension admission/A06-003. | Direct peer-internal/persistence access prohibited. |
| **Extension Admission Boundary** | Control point | Validates that an extension request targets a registered compatible extension point under valid scope/authorization before A06-003 execution. | Extension reference, extension-point/contract version, A04 scope, authorization outcome. | Admitted request to A06-003. | Incompatible/unregistered/unauthorized extension is not executed. |
| **Extensibility Classification Boundary** | Control point | Separates ordinary configuration from extension requirements. | Requested institutional variation. | Configuration path, A06-003 handoff or unsupported outcome. | Arbitrary application-logic customization is prohibited. |

These are logical solution elements, not mandated processes, services, containers or databases.

---

## 5.3 Configuration Definition Evolution

A configuration value is meaningful only against the configuration definition version under which it is governed.

```text
Configuration Definition vN
        │
        ├── Configuration Version 1
        ├── Configuration Version 2
        └── Configuration Version 3

Definition evolves
        ↓
Configuration Definition vN+1
        ↓
compatibility / migration relationship established
        ↓
future configuration versions use applicable definition
```

Rules:

1. Existing configuration versions retain traceability to their governing definition version.
2. A newer definition shall not silently reinterpret historical configuration.
3. Material definition evolution shall preserve compatibility information sufficient to determine whether existing configuration remains usable.
4. Where migration is required, the migration mechanism shall preserve lineage; exact migration implementation belongs to collaborating persistence/application architecture.
5. A configuration definition cannot change canonical business ownership.

This section defines compatibility architecture, not a physical schema language.

---

## 5.4 Configuration Change and Validation Flow

```text
Change request
      ↓
Validate A04 execution scope
      ↓
Resolve registered Configuration Definition + version
      ↓
Bind candidate to predecessor / current basis
      ↓
Generic technical validation
      ↓
Canonical owner semantic-validation contract
      ↓
Authoritative governance outcome available?
      ├── No → candidate remains outside effective use
      │
      └── Yes
            ↓
Preserve new immutable configuration version
            ↓
Concurrency / predecessor consistency check
            ↓
Effective-Use Gate
            ↓
A06-002 effective resolution
```

The common platform may validate structure, registration, ownership binding, version compatibility and protected scope.

It shall not invent domain business-policy semantics.

---

## 5.5 Concurrent Change and Publication Safety

Two changes derived from the same configuration basis shall not silently overwrite one another.

```text
Effective/Basis Version V1
       ├──────── Candidate A based on V1
       └──────── Candidate B based on V1

Candidate A becomes accepted successor
                 ↓
V1 → V2

Candidate B attempts publication
                 ↓
its expected basis no longer matches
                 ↓
conflict detected
                 ↓
B does NOT silently replace V2
```

The architecture requires:

- each successor candidate to retain its predecessor/basis reference;
- publication/effective-use admission to verify that required predecessor/basis consistency still holds;
- a conflict to result in rejection or governed re-evaluation rather than lost-update behavior;
- B02 to supply the physical transaction/concurrency mechanism.

A06-001 does not prescribe locks, isolation levels or database technology.

---

## 5.6 Effective-Use Boundary

Only a configuration version with all required:

- registered-definition validity;
- protected scope validity;
- technical validation;
- canonical-owner validation where applicable;
- upstream governance eligibility;

may enter A06-002 resolution.

A06-001 does not decide scope precedence, override hierarchy or effective dates.

Those belong to `L5-A-06-002`.

---

## 5.7 Immutable Execution Configuration Context

Locked L2-A01 requires resolved configuration to remain validated, Tenant-aware, version-consistent and immutable during execution.

A06 realizes this as an **Execution Configuration Context**.

```text
Execution starts
      ↓
A06-002 resolves eligible configuration
      ↓
Execution Configuration Context established
      ↓
exact resolved configuration-version references fixed
for this execution context
      ↓
business / platform processing
      ↓
execution completes
```

A later configuration change shall not silently replace configuration inside an already-established execution context.

```text
Execution X → Configuration Snapshot V4
Configuration V5 becomes eligible

Execution X continues → V4 context
Newly resolved Execution Y → applicable V5 context
```

This is a technical consistency boundary, not a business rule about when a business workflow must start a new execution.

Long-running/deferred processing that begins a new technical execution boundary shall resolve configuration according to its governing execution architecture; it shall not rely on ambient mutable configuration.

---

## 5.8 Resolved Configuration Consumption

Consumers receive configuration through a read-only boundary.

A consumer shall not:

- change authoritative configuration through the read path;
- independently calculate hierarchy/precedence owned by A06-002;
- substitute a local/global fallback for failed resolution;
- reinterpret the configuration under another definition version.

Optional caches/projections may exist only if their implementation preserves the exact scope, version and effective-resolution semantics of A06.

No cache technology or TTL is selected here.

---

# 5.9 Institutional Extensibility Baseline

Institutional variation is divided into three technical categories.

### A. Governed Configuration

Variation changes supported values/parameters inside a registered Configuration Definition.

```text
Registered configurable capability
        ↓
configuration value/version
        ↓
A06-001 governance
        ↓
A06-002 resolution
```

### B. Governed Controlled Extension

Variation requires executable or externally supplied behavior through an explicitly registered extension boundary.

```text
Registered Extension Point
        ↓
compatible Extension Contract
        ↓
Extension Admission Boundary
        ↓
A06-003
```

### C. Unsupported Core Modification

Variation requires bypassing a governed configuration or extension boundary, directly modifying another technical owner's internals or changing business architecture.

```text
No registered configuration
+
No registered extension point
        ↓
NOT an authorized extension path
```

A06 shall not legitimize this third category as customization.

---

## 5.10 Extension Point Registry

An **Extension Point** declares that one technical capability intentionally permits governed extension through a specific public boundary.

Conceptually:

```text
ExtensionPoint
{
  stable extension-point identity
  canonical technical owner
  governing contract/version
  supported extension category
  required execution/scope context
  compatibility responsibility
}
```

Rules:

1. Every extension point has exactly one canonical technical owner.
2. Registering an extension point does not transfer the owner's business responsibility.
3. An extension may attach only through a registered extension point.
4. Extension-point registration cannot authorize cross-owner persistence access.
5. Extension points respect A03 parent-module and public-boundary rules.
6. Extension-point identity does not imply independent deployment.

---

## 5.11 Extension Contract Boundary

An Extension Contract defines what interaction an admitted extension may perform through the owner-provided public boundary.

The contract establishes only the categories required for architectural compatibility, including:

- extension-point identity/version;
- owning technical capability;
- required execution/scope context;
- permitted input/output interaction;
- compatibility relationship.

The contract shall not grant:

- canonical business ownership;
- unrestricted access to owner internals;
- direct access to another owner's persistence;
- Tenant scope widening;
- authorization beyond D02 decisions;
- access to secrets merely because an extension is admitted.

Exact API/schema/wire representation remains with the applicable C-family architecture.

---

## 5.12 Extension Contract Evolution

Extension contracts are version-governed.

A materially changed extension contract shall not silently reinterpret an extension built against an incompatible predecessor.

```text
Extension Contract v1
        ↓
registered compatible extensions

Contract evolves
        ↓
Extension Contract v2
        ↓
compatibility assessed
        ├── compatible → governed continuation possible
        └── incompatible → existing extension cannot be silently admitted as v2
```

Exact compatibility calculation and feature activation behavior belong to A06-003 where execution/activation is involved.

A06-001 establishes the requirement that compatibility be explicit and traceable.

---

## 5.13 Extension Admission Flow

```text
Extension request / registration reference
        ↓
Extension Point exists?
        ├── No → reject as governed extension
        ↓
Contract version compatible?
        ├── No → reject / require governed adaptation
        ↓
A04 scope valid?
        ├── No → fail closed
        ↓
D02 authorization outcome permits operation?
        ├── No → reject
        ↓
A03 technical-boundary rules satisfied?
        ├── No → reject
        ↓
Hand off to A06-003
for feature activation / controlled execution
```

A06-001 performs the common admission architecture.

It does not execute extension code or define sandbox/runtime mechanics owned by A06-003 and security/runtime peers.

---

## 5.14 Configuration vs Extension Classification

```text
Institutional requirement
        ↓
Can it be represented inside a registered
Configuration Definition without changing
application/business logic?
        ├── Yes
        │     ↓
        │  Configuration path
        │
        └── No
              ↓
Is there a registered Extension Point
whose contract legitimately supports it?
              ├── Yes → A06-003 path
              │
              └── No → unsupported by A06
```

Ordinary configuration shall not contain executable behavior merely to bypass controlled extension governance.

A domain may define configuration whose **business meaning** influences domain execution, but the configuration framework itself shall not become an arbitrary executable-code transport.

---

# 5.15 Material Failure Model

| Failure | Detection Boundary | Required Outcome | Forbidden Outcome | Delegated Owner |
|---|---|---|---|---|
| Unknown/ambiguous configuration owner | Definition Registry | Reject registration/admission. | Guess an owner. | Upstream owner governance |
| Unknown/incompatible configuration-definition version | Validation Boundary | Reject candidate/use. | Reinterpret under another definition. | A06 |
| Invalid Tenant/Institution scope | A04 | Fail closed. | Default/global Tenant. | A04 |
| Generic validation failure | Validation Coordinator | Candidate stays non-effective. | Effective publication. | A06 |
| Owner semantic validation failure | Owner handoff | Candidate stays non-effective. | Platform overrides owner. | Canonical owner |
| Missing/rejected governance outcome | Effective-Use Gate | Preserve existing valid position. | Infer approval. | Upstream governance |
| Competing successor changed the basis | Publication boundary | Detect conflict and re-evaluate/reject. | Lost update/silent overwrite. | A06 + B02 |
| Attempt to mutate historical version | Repository boundary | Reject; require new successor. | In-place history rewrite. | A06/B02 |
| Resolved context changes mid-execution | Execution Configuration Context | Preserve established context. | Silent hot replacement inside execution. | A06 |
| A06-002 cannot produce unambiguous resolution | Resolution boundary | Do not invent fallback. | Consumer-selected precedence. | A06-002 |
| Secret supplied as ordinary configuration | Change boundary | Exclude protected material/use D03 mechanism. | Secret propagation. | D03-004 |
| Unregistered extension point | Extension Admission | Reject governed extension admission. | Attach to internal implementation. | A06 |
| Extension contract incompatible | Extension Admission | Reject/adapt through governed process. | Silent incompatible execution. | A06/A06-003 |
| Extension attempts peer-internal/direct persistence access | A03/A06 boundary | Reject path. | Boundary bypass. | A03/B02 |
| Extension scope/authorization invalid | A04/D02 | Fail closed. | Scope or privilege widening. | A04/D02 |
| Requirement fits neither configuration nor registered extension | Classification boundary | Remain unsupported/escalate architecture. | Ad-hoc core modification. | Architecture governance |

---

## 5.16 Persistence & Consistency Boundary

A06 owns logical version, predecessor and compatibility semantics.

B02 owns:

- physical persistence;
- schema realization;
- transactional atomicity;
- concurrency-control mechanism;
- storage isolation.

The B02 realization must be able to enforce A06's requirements that:

- historical configuration versions remain identifiable;
- predecessor lineage is durable;
- stale-basis successor publication cannot silently overwrite a newer authoritative version;
- configuration scope/version associations remain consistent.

---

## 5.17 Security & Trust Boundary

A06 consumes security capabilities rather than duplicating them.

- A04 establishes Tenant/Institution execution scope.
- D02 establishes authorization.
- D03-004 protects secrets/certificates/machine credentials.
- D04 preserves technical evidence.
- A03 prevents cross-owner internal/persistence bypass.
- A06-003 owns extension execution protection together with applicable D/E-family architecture.

Extension registration itself grants none of those privileges.

---

## 5.18 Runtime / Deployment Position

A06-001 defines a shared logical capability.

Under `ADR-L5-0003`:

```text
logical A06 component
        ≠
mandatory microservice/process/container
```

Co-location does not weaken A03/A04/A06 boundaries.

Physical isolation, where justified, remains E04-owned.

---

# 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis | ADR |
|---|---|---|---|---|
| `AD-001` | Shared configuration control plane | One reusable configuration architecture; business meaning remains owner-controlled. | L2-A01 §6 + corpus boundary | None |
| `AD-002` | Configuration history | Material changes create distinct versions with preserved predecessor lineage. | Applicable L4 | None |
| `AD-003` | Versioned configuration definitions | Configuration versions remain bound to the definition version under which they were validated; newer definitions cannot silently reinterpret history. | Version consistency + L2-A01 compatibility | None |
| `AD-004` | Validation separation | Generic platform validation and owner semantic validation remain separate. | Ownership integrity | None |
| `AD-005` | Concurrent successor protection | Candidate publication verifies its expected basis/predecessor so stale candidates cannot silently overwrite newer authority. | Technical consistency derivation | None |
| `AD-006` | Immutable execution context | One resolved execution context retains its exact configuration-version set for that execution. | L2-A01 §6.14 | None |
| `AD-007` | Read-only consumption | Consumers cannot mutate authoritative configuration through resolved-context access. | L2-A01 §6.14; L4-A-056 | None |
| `AD-008` | Governed extension points | Executable/institutional extension is permitted only through explicitly registered owner-provided public extension points. | Frozen A06 responsibility + A03 | None |
| `AD-009` | Versioned extension contracts | Extension admission requires an explicitly compatible governing contract/version. | Compatibility and boundary integrity derivation | None |
| `AD-010` | Configuration is not arbitrary executable customization | Executable behavior outside a registered configuration semantic belongs to the controlled-extension path. | L2-A01 P7 + A06 boundary | None |
| `AD-011` | Credentials excluded | Secret/private credential material is not ordinary configuration. | D03-004 | `ADR-L5-0002` through D03 |
| `AD-012` | Deployment independence | A06 logical elements do not mandate separate deployment units. | A01/A03 | `ADR-L5-0003` |

No programming language, framework, configuration server, database, broker, cache, scripting engine, plugin framework, container platform or vendor technology is selected.

---

# 7. Security, Quality & Operational Impact

| Concern | Architectural Impact | Owner / Collaboration |
|---|---|---|
| Isolation | Configuration and extensions operate only under validated scope. | A04 |
| Integrity | Version/definition lineage prevents historical reinterpretation. | A06/B02 |
| Concurrent safety | Stale-basis publication cannot silently overwrite a newer successor. | A06/B02 |
| Execution determinism | Running execution uses one immutable resolved configuration context. | A06/A06-002 |
| Extensibility containment | Extensions attach only at registered public boundaries. | A06/A03/A06-003 |
| Least privilege | Extension registration does not imply authorization or broader scope. | D02/A04 |
| Confidentiality | Credentials excluded from ordinary configuration. | D03-004 |
| Auditability | Definition/version, validation, governance, resolution and extension-admission evidence is externally preservable. | D04 |
| Maintainability | Institutional variation uses governed configuration/extension rather than uncontrolled core modification. | A06 |
| Portability | Architecture remains product/deployment neutral. | A01/E04 |

No unsupported numeric target is introduced.

---

# 8. Architecture Conformance Requirements

| ID | Architecture Conformance Requirement | Derived From |
|---|---|---|
| `CR-001` | Every configurable capability shall reference a registered Configuration Definition with one canonical owner. | `AD-001` |
| `CR-002` | Configuration Definitions shall be version traceable. | `AD-003` |
| `CR-003` | A configuration version shall remain associated with the definition version under which it was validated. | `AD-003` |
| `CR-004` | Definition evolution shall not silently reinterpret historical configuration. | `AD-003` |
| `CR-005` | Configuration change admission shall require valid protected scope where applicable. | A04 |
| `CR-006` | Domain/business semantic validation shall remain under the canonical owner. | `AD-004` |
| `CR-007` | Material configuration changes shall preserve predecessor/successor lineage. | `AD-002` |
| `CR-008` | Stale-basis successor publication shall be detected rather than silently overwriting a newer valid successor. | `AD-005` |
| `CR-009` | Configuration lacking required validation/governance eligibility shall not enter effective resolution. | §5.6 |
| `CR-010` | Scope/inheritance/override/effective-date calculation shall remain A06-002-owned. | Boundary |
| `CR-011` | A resolved execution context shall identify the exact configuration versions governing that context. | `AD-006` |
| `CR-012` | An established Execution Configuration Context shall not silently change because a newer configuration becomes eligible. | `AD-006` |
| `CR-013` | Configuration consumption shall be read-only through the resolved-context boundary. | `AD-007` |
| `CR-014` | Consumers shall not invent fallback hierarchy/precedence when A06-002 resolution fails. | A06-002 boundary |
| `CR-015` | Every governed extension attachment shall target a registered Extension Point. | `AD-008` |
| `CR-016` | Every Extension Point shall identify one canonical technical owner and governing contract/version. | `AD-008/009` |
| `CR-017` | Extension admission shall verify contract compatibility before controlled execution. | `AD-009` |
| `CR-018` | Extension boundaries shall not permit direct peer-internal or cross-owner persistence access. | A03 |
| `CR-019` | Extension admission shall preserve A04 scope and applicable D02 authorization decisions. | §5.13 |
| `CR-020` | Extension registration/admission shall not grant business authority, persistence ownership or Tenant scope widening. | §5.11–5.13 |
| `CR-021` | Feature activation/deactivation and extension execution mechanics shall remain A06-003-owned. | Boundary |
| `CR-022` | Executable behavior shall not be hidden inside ordinary configuration merely to bypass controlled-extension governance. | `AD-010` |
| `CR-023` | Raw secrets/private credential material shall not propagate as ordinary configuration. | `AD-011` |
| `CR-024` | Physical persistence/concurrency implementation shall remain B02-owned while satisfying A06 version/lineage/conflict semantics. | §5.16 |
| `CR-025` | Logical A06 components shall remain independent of deployment placement unless separately governed. | `AD-012` |
| `CR-026` | Configuration and extension governance evidence shall remain attributable without transferring business/configuration ownership to D04. | §7 |

---

# 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|
| `OD-002` | Exact canonical configuration request, approval/activation contracts and configuration lifecycle event/publisher semantics are absent upstream in `L4-A-056`. | Upstream | Applicable upstream architecture owner | No for this baseline; blocks their invention | Keep undefined until established upstream. |
| `OD-003` | Configuration/extensibility product or framework selection is not established. | Technology | L5 ADR governance if materially required | No | Remain product-neutral until a real selection decision is necessary. |

No additional Lead-Architect decision is required to complete the A06-001 baseline architecture.

---

# 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-056-BH-001/002/004/007/008/009`
- `L4-B-011-BH-001`–`006`
- `L4-D-046-BH-001/003`–`008`
- `L4-E-074-BH-001`–`008`
- `L4-G-092-BH-001`–`007/009`
- `L4-G-123-BH-001`–`006`

### Higher-Level / ADR Constraints

- `L2-A01` §§6.1–6.16
- `ADR-L5-0003`
- `ADR-L5-0004`

### Material L5 Dependencies

- `L5-A-01-001`
- `L5-A-03-001`
- `L5-A-04-001`
- `L5-A-04-002`
- `L5-A-06-002`
- `L5-A-06-003`
- `L5-B-02-*`
- `L5-D-03-004`
- `L5-D-04-*`

### Architecture Decisions

`AD-001`–`AD-012`

### Conformance Requirements

`CR-001`–`CR-026`

### Open / Deferred Items

- `OD-002` — upstream approval/activation/event semantics remain undefined.
- `OD-003` — product selection intentionally deferred.

### Review Position

The specification is **Architecture Approved** following full Stage-2 validation at repository baseline `2876bde9994bcd0a9ae17d9fb09c104a7be86d27`. The remaining open items are non-blocking and prevent unsupported invention rather than preventing this architecture from being implemented. The specification is lock-ready but shall not be marked `Locked` until distinct final governance authorization is recorded.