# Module, Dependency & Technical Boundary Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-03-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-03 — Module, Dependency & Technical Boundary Architecture  
**Canonical Technical Responsibility:** Technical module boundaries aligned to canonical architecture ownership.  
**Technical Architecture Owner:** L5-A-03 — Module, Dependency & Technical Boundary Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-10`  
**Depends On:** `L5-A-01-001` v1.0.0 Draft; L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L2-A08_Module_Registry_Configuration_Workflows.md` v1.0.0 Locked; `L4-A-028` v1.0.0 Draft; `L4-A-029` v1.0.0 Draft; `L4-A-032` v1.0.0 Draft; SutraERP L1 Business Domain Architecture v2.1 Locked; `DEPENDENCY_MAP.md`.  
**Referenced By:** None yet  
**Applicable ADRs:** Required but unresolved — ownership-preserving logical-Module-to-technical-module decomposition and its relationship to the unresolved A01 application-decomposition ADR; see `OD-001`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines how a canonical Module Registry `Module` is realized as one or more cohesive L5 technical modules without changing the Module's canonical business ownership, lifecycle, capability semantics or Module Registry authority.

It establishes the technical-module boundary model, parent ownership mapping, technical dependency boundary classes, version traceability and cross-module interaction constraints required by the Contract-Driven Modular Platform direction in `L5-A-01-001`.

### In Scope

- Definition of the L5 **Technical Module** and **Technical Module Boundary**.
- Ownership-preserving `1:N` mapping from one canonical Module Registry Module to one or more technical modules.
- The requirement that each technical module traces to exactly one canonical parent Module and applicable governing Module Version context.
- Separation of canonical Module Dependency metadata, L5 technical dependencies and runtime contract/event consumption.
- Structural rules for same-parent versus cross-parent technical dependencies.
- Cross-parent encapsulation and public-boundary requirements.
- Technical-module evolution traceability when a material Module architectural change creates a successor Module Version.
- Boundary implications of A01 modular-core versus selectively isolated execution placement.

### Out of Scope / Non-Goals

- Module Registry ownership, Module lifecycle, Module Version lifecycle, certification, activation, suspension, retirement or canonical dependency-graph governance — upstream Module Registry architecture.
- Enforcement mechanisms for composition, dependency direction, cycle prevention, unauthorized coupling or repository validation — `L5-A-03-002`.
- Application-wide decomposition model — `L5-A-01-001`.
- Tenant/Institution isolation and execution-context propagation — `L5-A-04-*`.
- Persistence ownership, schema, transaction or consistency architecture — `L5-B-02-*`.
- API, synchronous-contract, event or messaging mechanics — `L5-C-01-*` / `L5-C-02-*`.
- Security/trust enforcement — L5-D.
- Runtime packaging, deployment-unit formation or process/container topology — `L5-E-04-*`.
- Technology-product selection — `L5-A-02-001` and applicable ADRs.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-A-03-001` row | A03-001 owns technical module boundaries aligned to canonical architecture ownership. | Establishes this specification's canonical responsibility. |
| L5 Peer | `L5-A-01-001_Application_and_Platform_Architecture.md` | v1.0.0 / Draft | `AD-001`, §§4–5, `CR-001`–`CR-004` | Draft direction is a Contract-Driven Modular Platform with Selective Isolation; A03 owns detailed technical module boundaries and a business-domain boundary does not automatically create a service/deployment boundary. | Provides the provisional application structure within which A03 technical modules compose. |
| L2 | `L2-A08_Module_Registry_Configuration_Workflows.md` | v1.0.0 / Locked | §§1.1, 5, 7.3, 10–12 | A Module is a logical enterprise capability; Module Registry owns Module/Module Version/dependency metadata; a Module is not a deployable package, runtime process, database schema, service instance or API. | Requires a distinct L5 technical-module concept without redefining the canonical Module. |
| L4 | `L4-A-028_Module_Proposal_to_Active_Governed_Enterprise_Capability.md` | v1.0.0 / Draft — explicitly authorized for controlled A03 authoring | `BH-001`, `BH-003`, `BH-005`, `BH-007`–`BH-009` | A Module has one canonical owner, versions, capabilities, governed dependencies and repository references; architectural activation is not software deployment/runtime execution. | Requires parent ownership binding, dependency traceability and separation of technical module from deployment/runtime identity. |
| L4 | `L4-A-029_Module_Architectural_Change_to_Governed_Active_Successor_Version.md` | v1.0.0 / Draft — explicitly authorized for controlled A03 authoring | `BH-001`, `BH-003`, `BH-004`, `BH-007`, `BH-008` | Material Module architectural change creates an immutable successor Module Version; prior versions remain unchanged; compatibility, dependency integrity and lineage are preserved. | Requires technical-module composition to remain traceable to the governing Module Version without inventing a parallel business lifecycle. |
| L4 | `L4-A-032_Module_Dependency_Change_to_Validated_Architectural_Dependency_Outcome.md` | v1.0.0 / Draft — explicitly authorized for controlled A03 authoring | `BH-001`–`BH-008` | Canonical source/target Modules and versions are resolved; dependency compatibility/circularity are governed; invalid or unapproved changes do not alter the canonical dependency graph. | Constrains technical dependency realization to preserve canonical Module Dependency semantics while delegating enforcement to A03-002. |
| L1 | `SutraERP_L1_Business_Domain_Architecture_Locked.md` | v2.1 / Locked | §§2.1, 2.4 | Every business capability has one canonical owner; only the owner changes authoritative business data; cross-domain collaboration occurs through contracts/events; direct cross-domain database access is prohibited. | Prevents technical decomposition from fragmenting ownership or creating implementation bypasses. |
| Repository Governance | `DEPENDENCY_MAP.md` | Current canonical `main` | `Runtime Contract and Event Consumption Register` | Runtime integration/consumption is distinct from foundational architectural dependency and does not by itself change ownership or upstream dependency direction. | Prevents A03 from treating every contract/event interaction as a Module Registry architectural dependency. |
| L5 Governance | `L5_Technology_Selection_and_ADR_Methodology.md` | v1.0.1 / Locked | §§2–5, especially §3 | Reusable cross-specification patterns and material module/service-boundary decisions require ADR governance. | Makes `AD-001` ADR-required. |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | A canonical Module is a logical enterprise capability, not a deployable/runtime/API/database artifact. | L2-A08 §§1.1, 7.3; L4-A-028-BH-009 | A Technical Module is a distinct L5 realization construct and shall not redefine the upstream Module. |
| C-002 | Canonical business ownership remains singular through technical decomposition. | L1 §§2.1, 2.4; L4-A-028-BH-001/003 | Technical modules cannot split, merge or transfer upstream business ownership. |
| C-003 | Module Registry owns Module, Module Version and canonical Module Dependency metadata. | L2-A08 §5; L4-A-032 | A03 consumes these as governing architecture facts and does not create a competing module/dependency registry. |
| C-004 | Prohibited dependency cycles cannot enter the governed Module dependency graph. | L4-A-032-BH-004 | A03-001 shall define boundaries that can be validated for acyclicity; detection/enforcement belongs to A03-002. |
| C-005 | Runtime contract/event consumption is not automatically a foundational architectural dependency. | `DEPENDENCY_MAP.md` Runtime Contract and Event Consumption Register | Technical/runtime interaction shall not silently create or reverse canonical dependency direction. |
| C-006 | Cross-domain direct persistence access is prohibited. | L1 §2.1 | Cross-parent technical composition cannot use shared runtime or code access to bypass B02/C01/C02 boundaries. |
| C-007 | Material Module architectural evolution preserves immutable predecessor/successor lineage. | L4-A-029-BH-001/003/004/007 | Technical-module composition changes must be traceable to the applicable governing Module Version; A03 creates no alternate Module lifecycle. |
| C-008 | Architectural activation is not deployment/runtime execution. | L4-A-028-BH-009 | A Technical Module boundary does not imply a process, service or deployment unit; E04 owns that realization. |

### 3.3 Assumptions

No material assumptions are required. The ownership-preserving `1:N` module-mapping position is an explicit architecture decision in `AD-001` and remains subject to canonical ADR resolution through `OD-001`.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The L5 definition of a **Technical Module** as a cohesive technical responsibility boundary that realizes part or all of exactly one canonical parent Module.
- The ownership-preserving `1:N` relationship between a canonical Module Registry Module and its technical modules.
- The requirement that each Technical Module has exactly one canonical parent Module and is traceable to the governing Module Version context applicable to its architecture.
- The distinction between:
  - canonical Module Registry architectural dependency;
  - L5 technical-module dependency; and
  - runtime contract/event consumption.
- Structural same-parent and cross-parent dependency boundaries.
- The rule that a cross-parent Technical Module cannot couple to another parent's internal implementation boundary.
- The rule that reusable/shared technical capability has one canonical technical owner and is consumed by other modules rather than becoming a multi-parent Technical Module.
- The requirement that Technical Module identity and mapping remain independent of runtime/deployment placement.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-A-01-001` | Application & Platform Architecture | A01 decides overall modular-core/selective-isolation structure; A03 defines the technical modules composing inside that structure. | Preserves separation between platform decomposition and module internals. |
| `L5-A-03-002` | Module Composition & Dependency Enforcement Architecture | A03-001 defines required relationships and invalid boundary forms; A03-002 defines how they are mechanically checked/enforced. | Prevents duplicate ownership of enforcement. |
| `L5-A-04-001` / `L5-A-04-002` | Tenant/Institution isolation and context propagation | A03 boundaries must carry the context required by A04 but do not define it. | Prevents decomposition from weakening scope isolation. |
| `L5-B-02-001` / `L5-B-02-002` / `L5-B-02-003` | Persistence ownership and transaction/consistency boundaries | A03 does not grant persistence access merely because technical modules share a parent or runtime. | Preserves data ownership. |
| `L5-C-01-*` | Synchronous technical contracts | Cross-parent synchronous collaboration targets governed capability contracts, not peer internals. | Preserves public technical boundaries. |
| `L5-C-02-*` | Event/messaging architecture | Cross-parent asynchronous collaboration consumes canonical events without changing publisher ownership. | Preserves event semantics and technical isolation. |
| `L5-D-03-001` | Security/trust boundary baseline | Technical Module boundary is not automatically a trust boundary. | Keeps trust decisions with D03. |
| `L5-E-04-001` / `L5-E-04-002` | Runtime, packaging and runtime-isolation realization | E04 may co-locate or separate technical modules subject to A01/A03/D03 constraints. | Prevents module boundary from becoming deployment topology. |

### Must Not Own

- Canonical Module, Module Version, Module lifecycle, Module Dependency or dependency-type ownership.
- Business-domain ownership, business rules, lifecycle states, events or institutional authority.
- Dependency/cycle enforcement implementation.
- Contract/event schemas or transports.
- Persistence schema, transaction or consistency internals.
- Trust-boundary/security-control internals.
- Deployment, process, container, host or cloud topology.
- A separate technical-module business lifecycle.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| Canonical Parent Binding | Architecture mapping | Associates each Technical Module with exactly one canonical Module and the governing Module Version context applicable to that technical realization. | Canonical Module identity, ownership and Module Version metadata. | No business state; mapping/traceability only. | Technical Module Boundary definition. | Module Registry architecture | Missing, ambiguous or multi-parent binding makes the Technical Module non-conformant. | C-001–C-003; AD-001. |
| Technical Module Boundary | Technical module | Encapsulates one cohesive technical responsibility underneath its canonical parent Module and exposes only the boundary required for composition/consumption. | Parent binding; owned technical responsibility; governed capability/interaction references. | State access remains governed by B02; A03 owns no business state. | Same-parent technical dependency or governed cross-parent handoff. | A01, A03-002, B02, C01/C02 | Internal implementation shall not become a cross-parent dependency surface. | Frozen A03 responsibility; AD-001. |
| Same-Parent Technical Dependency | Technical relation | Represents a technical dependency between Technical Modules sharing the same canonical parent Module. | Source/target Technical Module identities and parent binding. | No canonical Module Dependency ownership implied. | Internal composition relation subject to A03-002 enforcement and B02/C01 rules where applicable. | A03-002; B02 | Relationship is invalid if parent bindings differ or it bypasses another canonical technical owner. | AD-001; C-002/C-003. |
| Cross-Parent Capability Boundary | Architecture boundary | Prevents a Technical Module from depending directly on another canonical Module's internal Technical Module implementation. Cross-parent collaboration targets an upstream-authorized public capability, contract, event, workflow or other governed boundary. | Source parent, target canonical owner, applicable governed interaction/dependency context. | No transferred ownership. | C01/C02/workflow or other canonical peer handoff. | C01/C02 and relevant capability owner | Direct peer-internal reference or cross-owner state access is non-conformant. | L1 §§2.1/2.4; C-006. |
| Architectural Dependency Alignment | Control relation | Relates a technical dependency to canonical Module Dependency metadata when the technical relationship realizes that governed architectural dependency, without treating every runtime interaction as such a dependency. | Module Registry dependency metadata; technical source/target mapping. | Canonical Module Dependency remains Module Registry-owned. | A03-002 validation/enforcement input. | Module Registry; DEPENDENCY_MAP; A03-002 | Contradictory direction/type/version applicability or invented dependency is non-conformant. | L4-A-032; C-003–C-005. |
| Module-Version Technical Mapping | Traceability relation | Associates the technical-module composition that realizes a Module architecture with the governing Module Version so material successor changes remain additive and predecessor mappings remain traceable. | Parent Module Version lineage; technical-module composition. | No new Module lifecycle/state. | Successor technical mapping when upstream governance establishes a successor Module Version. | Module Registry | Rewriting prior-version technical mapping as if it were the successor architecture is non-conformant. | L4-A-029-BH-001/003/004/007; C-007. |
| Shared Technical Capability Consumption Boundary | Architecture boundary | Ensures a reusable technical capability has one canonical technical owner and is consumed by multiple parents rather than represented as one multi-parent Technical Module. | Canonical technical owner and governed consumption boundary. | Provider retains technical ownership. | Governed consumer interaction. | Applicable L5 owner; C01/C02 where relevant | Multi-parent ownership or consumer mutation of provider internals is non-conformant. | Repository single-owner principle; AD-001. |

A Technical Module is therefore **not** a synonym for the upstream Module Registry `Module`. The canonical Module remains the governed logical enterprise capability. The Technical Module is its subordinate L5 technical realization boundary.

The mapping invariant is:

```text
Canonical Module Registry Module
        │
        ├── Technical Module 1
        ├── Technical Module 2
        └── Technical Module N

N >= 1

Each Technical Module
        ↓
exactly one canonical parent Module
        ↓
one unchanged canonical ownership context
```

No Technical Module may simultaneously realize portions of unrelated canonical parent Modules.

### 5.2 Boundary and Dependency Model

A03 distinguishes three different relationships that shall not be conflated:

| Relationship | Meaning | Ownership Effect | A03 Rule |
|---|---|---|---|
| Canonical Module Dependency | Governed architectural relationship between Module Registry Modules, including source/target, version/compatibility and upstream-defined dependency metadata. | None; Module Registry owns the relationship metadata and Module owners remain unchanged. | A03 consumes it as an architectural constraint; it does not recreate or rename it. |
| Technical Module Dependency | L5 implementation-architecture reliance between Technical Module boundaries. | None; each Technical Module keeps exactly one parent. | Same-parent dependency may remain internal; cross-parent dependency cannot target peer internals. A03-002 enforces allowed composition. |
| Runtime Contract / Event Consumption | Runtime collaboration through a governed contract, event, workflow or other canonical interaction. | None; runtime consumption does not by itself establish foundational ownership/dependency direction. | Use the relevant C/A05 boundary; do not invent a Module Registry dependency solely because runtime communication exists. |

Where a cross-parent technical dependency **realizes a canonical Module Dependency**, the technical design shall remain aligned to the registered source/target Module relationship and applicable version/compatibility constraints. A03-001 does not define or change the upstream dependency type.

Where two Modules merely collaborate at runtime through a contract or event, the interaction shall not be promoted to a foundational Module Dependency unless upstream architecture actually defines such a dependency.

### 5.3 Primary Technical Flow

```text
Canonical Module and owner resolved from Module Registry
        ↓
Applicable governing Module Version resolved
        ↓
Cohesive technical responsibility identified
        ↓
Technical Module defined
        ↓
Bind Technical Module to exactly one parent Module
        ↓
Declare same-parent technical dependencies
and externally consumed/provided boundaries
        ↓
For each cross-parent relationship:
        │
        ├─ Is this realization of a canonical Module Dependency?
        │       ↓ Yes
        │   Preserve registered direction/version/compatibility context
        │
        └─ Is this runtime contract/event consumption only?
                ↓ Yes
            Preserve as runtime collaboration;
            do not invent foundational dependency
        ↓
Cross-parent interaction targets governed public boundary
        ↓
A03-002 validates/enforces composition and dependency rules
        ↓
A01/E04 determine allowed runtime placement without changing parent binding
```

### 5.4 Module Evolution and Version Traceability

A03 does not create states such as `Draft`, `Active`, `Deprecated` or `Retired` for Technical Modules. Those upstream states belong to Module Registry concepts.

When upstream governance determines that a material Module architectural change requires a successor Module Version:

1. the predecessor Module Version and its technical-module mapping remain traceable;
2. the successor Module Version may establish a different technical-module composition;
3. a Technical Module may be introduced, removed, split, combined or have its responsibility changed only as part of an architecture change traceable to the applicable parent Module Version and repository governance;
4. such a change shall not rewrite prior-version architecture or transfer business ownership; and
5. actual compatibility validation/enforcement remains delegated to Module Registry governance and A03-002/C01/C02/B02 as applicable.

A03 does not invent a separate Technical Module Version lifecycle. If downstream implementation requires a software artifact/release version, that concern belongs to the applicable release/runtime architecture rather than this specification.

### 5.5 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Missing or ambiguous canonical parent | Canonical Parent Binding | Treat Technical Module definition as invalid/unresolved. | Creating an ownerless technical module or inferring ownership from code location/team. | Architecture author + Module Registry governance. | C-001–C-003. |
| Multi-parent Technical Module | A03 boundary review | Reject the boundary; split responsibilities or establish one separate canonical technical owner as appropriate. | One technical module silently implementing authoritative responsibilities of multiple canonical owners. | Architecture governance; upstream owner if business split is actually needed. | AD-001; C-002. |
| Cross-parent internal implementation dependency | Cross-Parent Capability Boundary | Reject the dependency and require a governed public boundary. | Direct call/reference into peer internals or peer persistence. | A03-002 + C01/C02/B02 as applicable. | L1 §§2.1/2.4; C-006. |
| Technical dependency contradicts canonical Module Dependency | Architectural Dependency Alignment | Reject/hold technical composition until architecture is reconciled through canonical governance. | Reversing dependency direction or changing upstream dependency semantics locally. | Module Registry + A03-002. | L4-A-032. |
| Runtime consumption misclassified as foundational dependency | Dependency classification review | Preserve runtime collaboration without changing canonical dependency direction. | Creating ownership/upstream dependency solely because a contract/event is consumed. | Architecture governance; DEPENDENCY_MAP authority. | C-005. |
| Prohibited canonical dependency cycle | Module Registry/A03-002 validation boundary | Technical composition shall not be accepted as conformant. | Bypassing the cycle through an undeclared technical-module relation. | Module Registry + A03-002. | L4-A-032-BH-004. |
| Predecessor Module Version mapping overwritten | Module-Version Technical Mapping | Preserve predecessor mapping; model changed composition against the governed successor context. | Mutating architectural history to make the successor appear as the predecessor. | Module Registry governance + repository change governance. | L4-A-029; C-007. |
| Deployment topology used to redefine module ownership | A01/A03/E04 boundary | Preserve Technical Module parent mapping regardless of co-location/separation. | Treating a process/service/container as a new canonical business owner. | A01/A03/E04/D03 as applicable. | C-008; A01 AD-001. |

### 5.6 Conceptual Technical Boundary Descriptors

These are conceptual information categories required for architecture traceability; they do not define a database schema, API payload or source-code manifest.

```text
TechnicalModuleBoundary
{
  stable technical-module identity
  canonical parent Module reference
  governing Module Version context
  owned technical responsibility
  provided governed boundary references
  consumed technical dependency / interaction references
}
```

```text
TechnicalDependencyRelation
{
  source Technical Module reference
  target boundary reference
  relation classification:
      same-parent technical dependency
      OR cross-parent governed consumption
  canonical Module Dependency reference when one actually governs the relationship
  applicable compatibility/version-governance reference where upstream defines it
}
```

No lifecycle state, deployment address, endpoint URL, persistence schema, transport protocol or technology product is introduced by these descriptors.

### 5.7 State / Persistence / Consistency

A03 owns no business persistence. A Technical Module boundary does not automatically own a database/schema and does not gain authority to read or write another owner's authoritative state.

B02 determines persistence isolation and transaction/consistency mechanics. A03-001 provides only the structural owner/parent boundary needed for B02 to apply those controls.

### 5.8 Security / Trust / Isolation

A Technical Module Boundary is an encapsulation and ownership-alignment boundary; it is **not automatically a security trust boundary**.

D03 determines trust boundaries and controls. A01/E04 may realize one or more Technical Modules inside a shared runtime or a Selectively Isolated Execution Unit. Such placement shall not alter the canonical parent binding or ownership semantics defined here.

### 5.9 Runtime / Deployment

Technical Module identity is independent of physical/runtime placement.

```text
Technical Module
      ≠
Service
      ≠
Process
      ≠
Container
      ≠
Deployment Unit
```

A03 does not require one deployment unit per Technical Module and does not require all Technical Modules under one parent to share a deployment unit. A01 provides the modular-core/selective-isolation application structure; E04 owns concrete runtime/package/deployment realization.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| AD-001 | Relationship between canonical Module Registry Modules and L5 Technical Modules | **Provisional Draft direction — ownership-preserving hierarchical `1:N` mapping.** One canonical Module may be realized by one or more Technical Modules. Every Technical Module belongs to exactly one canonical parent Module and applicable governing Module Version context. A Technical Module shall not span multiple unrelated canonical owners. | Frozen A03-001 responsibility; L2-A08 §§1.1/5/7.3; L4-A-028; L4-A-029; L1 ownership rules; Lead Architect selection on 2026-09-10. | Allows cohesive technical decomposition without fragmenting business ownership, conflating logical Modules with services/deployments, or creating ambiguous multi-owner components. | **Required — canonical ADR not yet allocated; ADR scope must be coordinated with A01; see OD-001.** |
| AD-002 | Cross-parent technical dependency surface | A Technical Module may depend on another parent Module only through a governed public capability/contract/event/workflow or other canonical boundary; it shall not target the peer's internal Technical Module implementation or authoritative persistence. | L1 §§2.1/2.4; L4-A-032; A01 boundary model. | Preserves independent ownership/evolution and prevents co-location from becoming unauthorized coupling. | None separately; derived from upstream ownership and A03 responsibility. |
| AD-003 | Relationship between technical dependency and repository dependency classes | Technical-module dependency and runtime consumption are distinct from canonical Module Registry/foundational dependency. A03 shall preserve, not infer or rewrite, the upstream dependency classification. | `DEPENDENCY_MAP.md`; L2-A08 §12; L4-A-032. | Prevents false dependency direction, unnecessary coupling and accidental ownership changes caused by runtime interactions. | None separately; preserves established repository semantics. |

No programming language, framework, package system, module loader, build tool, database, broker, service mesh, container platform or deployment technology is selected here.

---

## 7. Security, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Ownership integrity | Technical decomposition cannot split or combine canonical business ownership. | AD-001; L1; L4-A-028 | A03 with upstream Module Registry authority |
| Encapsulation / maintainability | Cross-parent consumers depend on governed public boundaries, allowing internal Technical Module composition to evolve without creating peer-internal coupling. | AD-002 | A03-002; C01/C02 |
| Dependency integrity | Technical dependency declarations must preserve canonical dependency direction/classification and be enforceable against cycles/unauthorized coupling. | AD-003; L4-A-032 | A03-002 |
| Evolution traceability | Material technical composition changes remain associated with the governed successor Module Version and do not rewrite predecessor architecture. | L4-A-029 | Module Registry + A03 |
| Isolation portability | Technical Module identity remains stable whether runtime placement is co-located or selectively isolated. | A01 AD-001; C-008 | E04, D03 |
| Tenant/Institution isolation | Module decomposition shall not weaken context propagation or isolation. | A01/A04 boundary | A04 |

No numeric performance, availability, capacity or deployment targets are established by A03-001.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| CR-001 | Every Technical Module shall identify exactly one canonical parent Module; an ownerless or multi-parent Technical Module is non-conformant. | AD-001; §5.1 | L2-A08; L4-A-028; L1 ownership rules |
| CR-002 | A canonical Module may be realized by one or more Technical Modules without creating additional business-domain ownership, Module Registry ownership or business lifecycle. | AD-001; §5.1 | Frozen A03 responsibility; L2-A08 §7.3 |
| CR-003 | Every Technical Module shall be traceable to the governing Module Version context applicable to its architecture; material successor architecture shall not overwrite predecessor mappings. | §5.4; AD-001 | L4-A-029-BH-001/003/004/007 |
| CR-004 | A cross-parent Technical Module dependency shall target a governed public capability/contract/event/workflow or other canonical boundary and shall not depend on peer-internal implementation or authoritative persistence. | AD-002; §§5.1–5.3 | L1 §§2.1/2.4; L4-A-032 |
| CR-005 | A technical/runtime interaction shall not be represented as a foundational or Module Registry dependency solely because a contract, event or workflow is consumed; upstream dependency classification shall be preserved. | AD-003; §5.2 | `DEPENDENCY_MAP.md`; L2-A08 §12 |
| CR-006 | Where a technical relationship realizes a canonical Module Dependency, its source/target Module relationship and applicable upstream version/compatibility constraints shall not contradict the governed Module Registry relationship. | §5.2; AD-003 | L4-A-032-BH-001–008 |
| CR-007 | A03-001 boundary definitions shall expose enough stable identity, parent mapping and dependency metadata for A03-002 to validate composition, cycles and unauthorized coupling without inferring ownership. | §§5.1, 5.6 | Frozen A03-002 boundary; L4-A-032 |
| CR-008 | Reusable technical capability consumed by multiple canonical parents shall have one canonical technical owner and shall be consumed through governed boundaries; it shall not become a multi-parent Technical Module. | §5.1; AD-001/AD-002 | Repository single-owner principle |
| CR-009 | Technical Module identity shall remain independent of service/process/container/deployment placement; runtime separation or co-location shall not change canonical parent ownership. | §§5.8–5.9 | L2-A08 §7.3; L4-A-028-BH-009; A01 AD-001 |
| CR-010 | Technical Module composition shall not create a path that bypasses B02 persistence ownership, A04 Tenant/Institution context or applicable C01/C02/D03 boundaries. | §§4–5 | L1 ownership rules; A01 boundary model |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| OD-001 | Canonical ADR for ownership-preserving Module→Technical Module decomposition and its relationship to A01's Contract-Driven Modular Platform decision | ADR / Dependency | A01 `AD-001` and A03 `AD-001` are materially coupled reusable boundary decisions; the canonical ADR Register has no allocated ADR and no approved ADR currently governs them. | Lead Architect through L5 ADR governance | **Yes — blocks Architecture Approved/Locked status** | Before ADR allocation, determine whether A01 and A03 are one Repository/Shared ADR decision scope rather than duplicate ADRs; allocate canonically on `main`, evaluate/approve, then reference the approved ADR here. |
| OD-002 | Canonical repository evidence for controlled use of `L4-A-028`, `L4-A-029` and `L4-A-032` while they remain Draft | Governance / Upstream | The cited L4 specifications remain `Draft` on `main`; explicit Lead-Architect authorization for A03 authoring was supplied on 2026-09-10. | Lead Architect / repository governance | **Yes — blocks promotion unless governance evidence or approved L4 state is canonicalized** | Before Architecture Approved status, either approve the applicable L4 material through its governance path or record the explicit Draft-L4 usage authorization in canonical repository governance. |

The upstream `L4-A-029` gaps concerning version-specific validation/certification commands and a distinct Transitional event do not block A03-001 because this specification neither invents those commands nor introduces a new event.

---

## 10. Traceability Summary

### Primary L4 Applicability
- `L4-A-028-BH-001`, `BH-003`, `BH-005`, `BH-007`–`BH-009` — canonical Module ownership, capability/dependency metadata and separation from deployment/runtime.
- `L4-A-029-BH-001`, `BH-003`, `BH-004`, `BH-007`, `BH-008` — immutable successor-version architecture, compatibility/dependency validation and lineage.
- `L4-A-032-BH-001`–`BH-008` — canonical dependency endpoints, compatibility, circularity and governed graph changes.

### Material Higher-Level / Governance Constraints
- L2-A08 §§1.1, 5, 7.3, 10–12 — Module Registry meaning and ownership.
- L1 v2.1 §§2.1, 2.4 — singular business ownership and governed cross-domain interaction.
- `DEPENDENCY_MAP.md` Runtime Contract and Event Consumption Register — runtime consumption does not by itself create foundational dependency direction.
- `L5-A-01-001` `AD-001` — provisional modular-core/selective-isolation application structure.
- L5 Technology Selection & ADR Methodology §3 — ADR requirement.

### Material L5 Dependencies
- `L5-A-01-001` — application structure.
- `L5-A-03-002` — composition/dependency enforcement.
- `L5-A-04-*` — Tenant/Institution isolation/context.
- `L5-B-02-*` — persistence ownership and consistency.
- `L5-C-01-*` / `L5-C-02-*` — governed interaction boundaries.
- `L5-D-03-001` — trust boundary baseline.
- `L5-E-04-001` / `L5-E-04-002` — runtime/deployment realization.

### Architecture Decisions
- `AD-001`–`AD-003`

### Conformance Requirements
- `CR-001`–`CR-010`

### Open / Blocking Items
- `OD-001` — canonical ADR scope/allocation/approval for A01/A03 decomposition pattern.
- `OD-002` — canonical evidence for authorized Draft-L4 usage before promotion.
