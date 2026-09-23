# Application & Platform Architecture

## 1. Specification Metadata

**Specification ID:** `L5-A-01-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-01 — Application & Platform Architecture  
**Canonical Technical Responsibility:** Overall solution/application structure and platform responsibility boundaries without redefining business domains.  
**Technical Architecture Owner:** L5-A-01 — Application & Platform Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.1.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L4-A-001` v1.0.1 Architecture Approved; `L4-A-015` v1.0.1 Architecture Approved; `L4-A-019` v1.0.1 Architecture Approved; `L4-A-028` v1.0.1 Architecture Approved; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** None yet  
**Applicable ADRs:** `ADR-L5-0003` — Contract-Driven Modular Platform with Selective Isolation (`Architecture Approved`).  

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the current approved logical structure of the SutraERP application platform: how platform and domain-aligned technical capabilities compose into one governed modular application model, how selectively isolated execution units may coexist with that model, and where A01 stops so that module composition, persistence, contracts, security and deployment remain owned by their canonical peer L5 specifications.

The term **platform** in this specification denotes the L5 technical application/platform structure. It does not transfer or redefine the upstream business ownership of the Platform & Tenant domain or any other business domain.

### In Scope

- The top-level SutraERP application composition model.
- The default modular-core composition position for application capabilities.
- The rule permitting selectively isolated execution units only where explicit technical architecture justifies isolation.
- The distinction between business-domain ownership, technical-module boundary, execution boundary and deployment boundary.
- Platform-level rules governing interaction between the modular core and separately isolated execution units.
- Structural constraints required to preserve canonical business ownership, Tenant/Institution context and peer-L5 technical ownership across application boundaries.

### Out of Scope / Non-Goals

- Detailed module/dependency design or enforcement — `L5-A-03-001` / `L5-A-03-002`.
- Tenant/Institution isolation mechanisms and context propagation — `L5-A-04-001` / `L5-A-04-002`.
- Database, persistence ownership, transaction or concurrency design — `L5-B-02-*`.
- API, synchronous-contract, event or messaging mechanics — `L5-C-01-*` / `L5-C-02-*`.
- Identity, authorization, cryptography, trust-control implementation or secrets — L5-D.
- Internal architecture of secure/specialized computation engines — `L5-A-07-*`.
- Runtime packaging, process/container/host topology, deployment or orchestration technology — `L5-E-04-*`.
- Programming language, framework, database, broker, cloud or other technology selection — `L5-A-02-001` and applicable ADRs.
- L6 client interaction, screen, navigation or UI/UX design.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-A-01-001` row | A01 owns overall solution/application structure and platform responsibility boundaries without redefining business domains. | Establishes this specification's canonical responsibility. |
| L4 | `L4-A-001_Tenant_Provisioning_to_Active_Tenant_Availability.md` | v1.0.0 / Draft — explicitly authorized for controlled A01 authoring | `BH-003`, `BH-005`, `BH-009` | Tenant provisioning establishes canonical Tenant environment/Platform context; Active Tenant context is made available to permitted downstream setup without ownership transfer. | Requires an application structure capable of carrying platform/Tenant context across technical capability boundaries without collapsing ownership. |
| L4 | `L4-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md` | v1.0.0 / Draft — explicitly authorized for controlled A01 authoring | `BH-004`, `BH-005` | Workflow execution coordinates defined steps and obtains authoritative domain results without changing participating-domain state directly. | Requires cross-capability composition that preserves the boundary between orchestration and domain-owned execution. |
| L4 | `L4-A-019_Committed_Business_Fact_to_Reliable_Consumer_Delivery.md` | v1.0.0 / Draft — explicitly authorized for controlled A01 authoring | `BH-007` | An immutable event is made available to eligible consumers without transferring publisher ownership or consumer business-decision ownership. | Requires application/execution boundaries that do not alter upstream semantic ownership when communication crosses them. |
| L4 | `L4-A-028_Module_Proposal_to_Active_Governed_Enterprise_Capability.md` | v1.0.0 / Draft — explicitly authorized for controlled A01 authoring | `BH-003`, `BH-008`, `BH-009` | Modules expose governed capabilities/dependencies and become discoverable when active; registry activation is not software deployment or runtime execution. | Requires explicit separation between logical module governance/composition and runtime/deployment realization. |
| L1 | `SutraERP_L1_Business_Domain_Architecture_Locked.md` | v2.1 / Locked | §§2.1, 2.4 | Every business capability has one owner; only the owner modifies authoritative business data; cross-domain collaboration uses contracts/events; direct database access across domains is prohibited. | Preserves canonical ownership regardless of co-location or execution separation. |
| L5 Governance | `L5_Technology_Selection_and_ADR_Methodology.md` | v1.0.1 / Locked | §§2–5, especially §3 | Reusable cross-specification structure, material module/service boundary decisions and major runtime patterns require ADR governance. | Makes `AD-001` ADR-governed. |
| L5 ADR | `ADR-L5-0003_Contract_Driven_Modular_Platform_with_Selective_Isolation.md` | Architecture Approved | Decision §§7–11 | Establishes the Contract-Driven Modular Platform with Selective Isolation as the canonical application-composition pattern. | Canonical decision authority for `AD-001`. |
| L5 Governance | `L5_ADR_Register.md` | v1.0.0 / Locked | Canonical register row `ADR-L5-0003` | Allocates and tracks the approved application-composition ADR. | Confirms canonical identity and lifecycle state. |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Technical placement shall not transfer or duplicate canonical business ownership. | L1 v2.1 §§2.1, 2.4; L4-A-015-BH-004/005; L4-A-019-BH-007 | A module or isolated execution unit executes capability; it does not become owner of another domain's business fact or lifecycle merely because it hosts code or state. |
| C-002 | Cross-domain direct persistence access is prohibited. | L1 v2.1 §2.1 | Co-location inside the modular core shall not permit direct modification of another domain's authoritative persistence. |
| C-003 | Tenant/Institution context and isolation must remain valid across application boundaries. | L4-A-001-BH-003/005/009 | Both core-local and isolated-unit execution consume the canonical A04 isolation/context architecture. |
| C-004 | Module Registry activation is logically distinct from software deployment or runtime execution. | L4-A-028-BH-008/009 | Active module status may participate in composition eligibility but shall not itself define packaging, process creation or deployment. |
| C-005 | Physical/runtime packaging and deployment are not owned by A01. | Frozen L5 corpus: `L5-E-04-*` | A01 defines logical application/execution categories; E04 determines their concrete packaging/deployment realization. |
| C-006 | No product/framework/runtime topology may be selected without applicable governance/ADR authority. | L5 Template v1.0.3; L5 Technology Selection & ADR Methodology v1.0.1 | A01 remains technology-neutral while still defining concrete logical structure. |

### 3.3 Assumptions

No material architecture assumptions are required for this specification. `ADR-L5-0003` establishes the selected decomposition model recorded as `AD-001`.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The overall SutraERP application-structure decision established by `ADR-L5-0003`: **Contract-Driven Modular Platform with Selective Isolation**.
- The top-level logical distinction between the **Modular Application Core** and **Selectively Isolated Execution Units**.
- The rule that a business-domain boundary does not automatically create a network, process, service or deployment boundary.
- The rule that execution isolation requires explicit technical justification from the canonical L5 owner and applicable ADR/governance, rather than preference or business-domain identity alone.
- The platform-level prohibition on using co-location or separation to bypass upstream ownership or peer-L5 technical boundaries.
- The structural handoff requirement between core composition and peer-owned contract, context, persistence, security and deployment architectures.

### Consumes / Collaborates With

The peers below are referenced by their frozen canonical responsibility boundaries. A01 does not assume un-authored peer internals or treat them as already stable technical implementations.

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-A-03-001` | Module, Dependency & Technical Boundary Architecture | A03 defines detailed technical module boundaries; A01 defines the overall application structural model in which they compose. | Prevents A01 from becoming the module-design owner. |
| `L5-A-03-002` | Module Composition & Dependency Enforcement Architecture | A03 defines enforceable dependency/composition rules. | Makes modular-core composition enforceable without duplicating dependency mechanics here. |
| `L5-A-04-001` | Tenant & Institutional Isolation Architecture | A04 owns isolation enforcement. | Ensures application placement cannot weaken Tenant/Institution separation. |
| `L5-A-04-002` | Tenant and Institution Context Propagation & Data Isolation Architecture | A04 owns context propagation/data isolation across execution/persistence. | Required whenever execution crosses a module or isolated-unit boundary. |
| `L5-B-02-001` / `L5-B-02-002` / `L5-B-02-003` | Persistence baseline, ownership/isolation, transaction/consistency/concurrency | B02 owns persistence and consistency realization. | Prevents application composition from implying shared-write authority or inventing transaction mechanics. |
| `L5-C-01-001` / `L5-C-01-002` / `L5-C-01-003` | API and synchronous contract architecture | C01 owns synchronous technical contract realization. | Used where a governed synchronous boundary is required. |
| `L5-C-02-001` / `L5-C-02-002` / `L5-C-02-003` | Event and messaging architecture | C02 owns event transport, durable delivery and event-schema compatibility. | Used where a governed asynchronous/event boundary is required. |
| `L5-D-03-001` | Security Architecture & Trust Boundary Baseline | D03 owns security/trust-boundary realization. | Selective isolation may create or strengthen technical trust boundaries but A01 does not define their controls. |
| `L5-E-04-001` / `L5-E-04-002` | Runtime/deployment baseline and application packaging/runtime isolation | E04 owns runtime, packaging and deployment realization. | Converts A01 logical core/isolated categories into deployable units without A01 mandating topology. |
| `L5-A-07-001`–`L5-A-07-004` | Secure computational-engine baseline and Finance/Examination/Payroll specialized engines | A07 owns specialized engine internals and isolation requirements. | Provides canonical consumers of the selective-isolation option without making every domain a service. |

### Must Not Own

- Business-domain ownership, rules, lifecycle states, canonical events or publisher authority.
- Detailed module composition/dependency graphs.
- Cross-domain persistence, transaction or consistency mechanisms.
- API/event transport schemas and delivery policies.
- Authorization/security control internals.
- Specialized engine internals.
- Concrete process/container/server/cloud/orchestration topology.
- Technology-product selection.

---

## 5. Solution Architecture

The realization below implements the approved application-composition direction in `ADR-L5-0003` and `AD-001`.

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| SutraERP Application Composition Boundary | Logical application boundary | Defines the top-level technical composition within which platform capabilities, domain-aligned modules and selectively isolated units participate without changing business ownership. | Governed capability/module identities; applicable Tenant/Institution context. | Owns no canonical business data. | Establishes placement relationship to Modular Application Core or an explicitly justified isolated execution boundary. | A03, A04, B02, C01/C02, D03, E04 | Invalid/unsupported placement shall not be repaired by bypassing a canonical owner. | Frozen A01 responsibility; C-001–C-006; `ADR-L5-0003`. |
| Modular Application Core | Logical composition | Default application composition for capabilities that do not have an explicit architecture requirement for separate execution isolation. Permits strong module boundaries without requiring a network/service boundary for every business domain. | A03-defined module composition; A04 context; peer-owned contracts. | Domain/business state remains owned by canonical domains and B02 persistence boundaries. | Core-local governed module invocation or peer-owned handoff. | A03, A04, B02, C01/C02 | Co-location shall not authorize cross-owner persistence writes, implicit trust or dependency bypass. | L1 §§2.1/2.4; `ADR-L5-0003`; AD-001. |
| Selectively Isolated Execution Unit | Logical execution boundary | Provides a separately executable boundary only where the canonical technical owner or approved ADR requires isolation for security, workload, resilience or other material architecture reason. | Governed input/contract and propagated context. | State ownership remains with the canonical capability/domain and its persistence owner. | Governed synchronous or asynchronous handoff defined by C01/C02 and the owning L5 architecture. | A04, C01/C02, D03, E04; specialized owner such as A07 | Unit failure shall not be recovered through direct cross-boundary state modification or ownership transfer. Exact retry/recovery is delegated. | `ADR-L5-0003`; AD-001; L4-A-015-BH-004/005; L4-A-019-BH-007. |
| Core-to-Isolated Handoff Boundary | Architecture boundary | Requires every core↔isolated interaction to cross a governed technical contract/event boundary while preserving source/target ownership and execution context. | Contract/event context defined by canonical peers. | No independent business state ownership. | C01 synchronous handoff or C02 asynchronous/event handoff. | C01/C02, A04, D03 | Missing/invalid context or incompatible/unavailable boundary prevents bypass execution. | C-001–C-003; L4-A-019-BH-007. |
| Deployment Realization Handoff | Architecture boundary | Transfers the logical placement model to E04 for packaging/runtime/deployment realization without prescribing process, container, server or cloud topology. | A01 logical placement plus owner-specific isolation requirement. | No runtime/deployment state owned by A01. | E04 packaging/isolation/deployment architecture. | E04 | A deployment choice shall not collapse a required A01 isolated boundary or create an unapproved one that changes ownership semantics. | C-005; `ADR-L5-0003`; AD-001. |

The **Modular Application Core is the default composition model, not a declaration that all capabilities must execute in one process or one deployment unit**. Conversely, the existence of an isolated execution unit does not convert its represented business domain into a separately owned business service. Runtime realization remains governed by E04.

### 5.2 Primary Technical Flows

#### A. Capability Placement / Composition

```text
Canonical technical capability
        ↓
A03 technical module / boundary definition
        ↓
Is separate execution isolation explicitly required
by the capability's canonical L5 architecture / ADR?
        ↓
   No                         Yes
   ↓                           ↓
Modular Application Core   Selectively Isolated Execution Unit
   ↓                           ↓
A03 composition             C01/C02 governed boundary
   └──────────────┬────────────┘
                  ↓
        E04 runtime / packaging realization
```

Business-domain identity alone shall not answer the isolation question.

#### B. Runtime Cross-Boundary Invocation

```text
Governed invocation / event
        ↓
Canonical Tenant / Institution context applied by A04
        ↓
A03-defined target technical boundary
        ↓
Target is core-local? ─────── Yes ──→ governed module invocation
        │
        No
        ↓
C01 synchronous contract or C02 event/messaging boundary
        ↓
Selectively Isolated Execution Unit
        ↓
Canonical capability executes under unchanged business ownership
        ↓
B02-owned persistence / consistency boundary where state is changed
        ↓
Governed result / acknowledgement / event handoff
```

A01 does not select synchronous versus asynchronous interaction globally. The appropriate mechanism is owned by the relevant C-family architecture and the collaborating capability specification.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Unsupported or ambiguous capability placement | A01/A03 composition boundary | Treat placement as unresolved; do not create an ad-hoc service or collapse a required isolated boundary. | Preference-driven topology or business-domain-to-service assumption. | Canonical capability owner + A01/A03 architecture governance. | AD-001; C-005/C-006. |
| Missing/invalid Tenant or Institution context | A04 boundary before target execution | Prevent target execution until canonical context requirements are satisfied. | Executing under default, inferred or cross-Tenant context. | `L5-A-04-001` / `L5-A-04-002`. | C-003. |
| Invalid dependency or prohibited cross-owner persistence path | A03/B02 boundary | Reject/break the invalid composition path while preserving canonical ownership. | Direct modification of another owner's persistence because components are co-located. | A03/B02. | C-001/C-002. |
| Core↔isolated contract unavailable or incompatible | C01/C02 boundary | Fail at the governed boundary and preserve source/target ownership. | Direct internal-state/persistence bypass around the failed boundary. | C01/C02 and the isolated capability owner. | L4-A-019-BH-007; C-001. |
| Isolated execution unit unavailable | Execution boundary | Contain the failure to the dependent interaction and expose failure through the owning peer architecture. | Transferring business authority to the core or silently executing alternate unapproved logic. | Capability owner; `L5-E-01-001` / `L5-E-01-002`, `L5-E-04-*` and C01/C02 as applicable. | AD-001; C-001. |

No retry count, timeout, failover topology or recovery interval is defined by A01.

### 5.4 State / Persistence / Consistency

A01 owns no business persistence, cross-domain transaction strategy or canonical domain state. Application composition shall preserve B02 ownership and shall not treat a shared runtime, shared process, shared database technology or shared infrastructure as permission for cross-owner writes.

Any technical metadata required to enforce module composition or runtime packaging is owned by the applicable A03/E04 architecture rather than by A01 unless a later canonical decision explicitly assigns otherwise.

### 5.5 Security / Trust / Isolation

Co-location inside the Modular Application Core shall not imply implicit authorization, common business ownership or unrestricted persistence access. Tenant/Institution context enforcement remains mandatory through A04, and security/trust controls remain owned by the applicable D-family architecture.

A Selectively Isolated Execution Unit represents a stronger logical execution boundary than ordinary core composition, but A01 does not prescribe the mechanism used to enforce that boundary. D03 and E04 determine the trust/runtime realization, while specialized owners such as A07 define capability-specific requirements.

### 5.6 Runtime / Deployment / Operations

A01 defines only two application-structure categories:

```text
1. Modular Application Core
2. Selectively Isolated Execution Unit
```

These are logical architecture categories. E04 owns their realization into packages, processes, containers, hosts, clusters, on-premises installations or cloud-capable environments. A01 therefore does not mandate microservices, a single binary, containers, Kubernetes, physical servers or a cloud provider.

### 5.7 Architecture Diagram

```text
                         ┌──────────────────────────────────────┐
                         │      SutraERP Application Platform   │
                         │                                      │
                         │  ┌────────────────────────────────┐  │
                         │  │   Modular Application Core     │  │
                         │  │                                │  │
                         │  │ A03-governed technical modules │  │
                         │  │ Domain ownership unchanged     │  │
                         │  └──────────────┬─────────────────┘  │
                         │                 │                    │
                         │       C01 / C02 governed boundary    │
                         │                 │                    │
                         │  ┌──────────────▼─────────────────┐  │
                         │  │ Selectively Isolated           │  │
                         │  │ Execution Unit(s)              │  │
                         │  │                                │  │
                         │  │ Only when canonical L5 / ADR   │  │
                         │  │ explicitly justifies isolation │  │
                         │  └────────────────────────────────┘  │
                         └──────────────────────────────────────┘

        A04 Tenant/Institution context applies across all execution boundaries
        B02 preserves persistence ownership and consistency boundaries
        D-family preserves authorization/security/trust boundaries
        E04 owns concrete runtime, packaging and deployment realization
```

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| AD-001 | Overall SutraERP application decomposition model | **Architecture Approved — Contract-Driven Modular Platform with Selective Isolation**: the Modular Application Core is the default technical composition; separately executable isolation is introduced only where explicit canonical L5 architecture/ADR justifies it. A business-domain boundary does not automatically become a network/service/deployment boundary. | `ADR-L5-0003`; frozen `L5-A-01-001` responsibility; L1 §§2.1/2.4; L4-A-001-BH-003/009; L4-A-015-BH-004/005; L4-A-019-BH-007; L4-A-028-BH-003/009. | Preserves domain ownership and governed contracts while avoiding unjustified distributed-system boundaries; permits stronger isolation for specialized capabilities without forcing the entire ERP into microservices or permanently coupling all capabilities to one runtime. | `ADR-L5-0003` |

No programming language, framework, database, broker, cloud platform, container/orchestration platform or vendor product is selected by this specification.

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant/Institution isolation | Placement inside the core or an isolated unit must preserve identical upstream Tenant/Institution semantics; placement cannot weaken scope enforcement. | L4-A-001-BH-003/005/009 | `L5-A-04-001`, `L5-A-04-002` |
| Security / trust | Co-location does not imply trust; selective isolation may strengthen a technical trust boundary when justified. | AD-001; canonical ownership constraints | `L5-D-03-001` and applicable D-family specs |
| Failure containment | A separately isolated unit creates a boundary at which its failure can be contained without permitting direct-state bypass or ownership transfer. | AD-001 | Capability owner; `L5-E-01-001` / `L5-E-01-002`; `L5-E-04-*`; C01/C02 |
| Maintainability / evolvability | Domain-aligned technical modules can remain inside the modular core or later move behind an isolated boundary without changing canonical business ownership, provided A03/C01/C02/B02 constraints are preserved. | AD-001 | A03, B02, C01/C02, E04 |
| Deployment portability | A01 logical structure is not tied to a process/container/cloud topology. | C-005/C-006 | `L5-E-04-*`, `L5-A-02-001` |

No numeric availability, performance, capacity or recovery target is established by A01.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| CR-001 | Every application capability governed by A01 shall have an explicit architectural placement relationship to the Modular Application Core or to an explicitly justified Selectively Isolated Execution Unit; business-domain identity alone shall not mandate separate execution. | Section 5.1; AD-001 | Frozen A01 responsibility; `ADR-L5-0003` |
| CR-002 | Introducing a Selectively Isolated Execution Unit shall require explicit support from the capability's canonical L5 architecture and applicable ADR/governance; implementation teams shall not create isolated services solely by preference. | Section 5.1; AD-001 | L5 ADR methodology §3; `ADR-L5-0003` |
| CR-003 | Co-located modules shall not use co-location to modify another canonical business owner's persistence directly or bypass A03/B02 ownership controls. | Sections 5.1, 5.4 | L1 v2.1 §2.1; C-001/C-002 |
| CR-004 | Every core↔isolated interaction shall use a governed C01/C02 technical boundary and shall preserve the canonical source/target business ownership. | Sections 5.1–5.3 | L1 v2.1 §§2.1/2.4; L4-A-015-BH-004/005; L4-A-019-BH-007 |
| CR-005 | Tenant/Institution execution context shall be preserved and enforced across both core-local and isolated execution according to A04; no A01 placement shall introduce an inferred/default cross-Tenant context. | Sections 5.2, 5.5 | L4-A-001-BH-003/005/009; C-003 |
| CR-006 | Module Registry `Active` status shall not be interpreted by A01 as equivalent to deployment, process creation or runtime execution; concrete composition/deployment shall remain governed by A03/E04. | Sections 3.2, 5.1 | L4-A-028-BH-008/009 |
| CR-007 | Concrete runtime packaging shall preserve any A01/A03/D03-required execution boundary while remaining owned by E04; A01 conformance shall not require Kubernetes, microservices, a single binary, a server count or a cloud provider. | Sections 5.5–5.6 | C-005/C-006; `ADR-L5-0003` |
| CR-008 | No platform-level technical element shall become canonical owner of a business fact, lifecycle or event merely because it hosts, executes, routes or persists the corresponding technical behavior. | Sections 4, 5.1 | L1 v2.1 §§2.1/2.4; L4-A-019-BH-007 |

---

## 9. Open Decisions & Deferred Design

`ADR-L5-0003` resolves the former blocking application-decomposition ADR item. The remaining item concerns promotion evidence for cited upstream L4 sources and is not altered by this remediation.

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| OD-002 | Canonical repository evidence for controlled use of the cited Draft L4 sources | Governance / Upstream | `L4-A-001`, `L4-A-015`, `L4-A-019` and `L4-A-028` remain `Draft` on `main`; explicit Lead-Architect authorization was supplied for this authoring session. | Lead Architect / repository governance | **Yes — blocks promotion unless governance evidence or approved L4 state is canonicalized** | Before Architecture Approved status, either promote the applicable L4 material through its governance path or record the explicit Draft-L4 usage authorization in canonical repository governance. |

Detailed module boundaries, isolation mechanics, persistence/transaction design, synchronous/asynchronous contract mechanics, security controls, runtime packaging and technology products are intentionally delegated to their canonical L5 owners and are not open A01 decisions.

---

## 10. Traceability Summary

### Primary L4 Applicability
- `L4-A-001-BH-003`, `BH-005`, `BH-009` — Tenant/platform context across application boundaries.
- `L4-A-015-BH-004`, `BH-005` — cross-capability orchestration without ownership transfer.
- `L4-A-019-BH-007` — communication without publisher/consumer ownership transfer.
- `L4-A-028-BH-003`, `BH-008`, `BH-009` — governed modules/capabilities and separation of activation from runtime deployment.

### Material Higher-Level / Governance Constraints
- L1 v2.1 §§2.1, 2.4 — canonical domain ownership, contracts/events and prohibition of cross-domain direct database access.
- L5 Candidate Corpus Register v1.1.0 — A01 canonical responsibility.
- L5 Technology Selection & ADR Methodology v1.0.1 — ADR requirement for AD-001.
- `ADR-L5-0003` — approved application-composition decision.
- L5 ADR Register v1.0.0 — canonical identity/lifecycle allocation.

### Material L5 Dependencies
- `L5-A-03-001` / `L5-A-03-002` — module boundaries and composition enforcement.
- `L5-A-04-001` / `L5-A-04-002` — Tenant/Institution isolation and context propagation.
- `L5-B-02-*` — persistence ownership and consistency.
- `L5-C-01-*` / `L5-C-02-*` — synchronous and event/messaging boundaries.
- `L5-D-03-001` — security/trust boundary baseline.
- `L5-E-04-001` / `L5-E-04-002` — runtime, packaging and deployment realization.
- `L5-A-07-001`–`L5-A-07-004` — specialized secure computation consumers of selective isolation.

### Architecture Decisions
- `AD-001` → `ADR-L5-0003`

### Conformance Requirements
- `CR-001`–`CR-008`

### Open / Blocking Items
- `OD-002` — canonical repository evidence for authorized Draft-L4 usage before promotion.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
