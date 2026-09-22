# SutraERP L5 Architecture Decision Record Register

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-09-22  
**Depends On:** L5 Technology Selection & ADR Methodology  
**Purpose:** Canonical allocation and lifecycle register for L5 Architecture Decision Record identifiers  

---

## 1. Register Authority

This is the single canonical allocation register for SutraERP L5 ADR identifiers.

Canonical identifier:

```text
ADR-L5-NNNN
```

ADR IDs shall not be independently claimed by:

- L5 specifications;
- author branches;
- local working copies;
- ADR draft filenames;
- conversation decisions.

An ADR ID becomes canonically allocated only when its allocation row exists in this register on canonical `main`.

---

## 2. Allocation Serialization Rule

Parallel authorship requires serialized allocation.

Therefore:

```text
Branch-local ADR number
        ≠
Reserved ADR number

Merged canonical register allocation
        =
Reserved ADR number
```

Before creating a canonical ADR:

1. verify whether an existing ADR already governs the issue;
2. determine that an ADR is required;
3. identify its scope and decision owner;
4. allocate the next canonical number in this register on `main`;
5. only then create/use that ADR identifier in authored architecture.

If two branches independently propose the same unallocated number, neither branch has canonical priority merely because it was authored first.

Canonical register state determines allocation.

---

## 3. Identifier Rules

1. One ID represents one ADR record.
2. Numbers are repository-wide.
3. Once allocated, an ID is never reused.
4. Rejected ADRs retain their IDs.
5. Superseded ADRs retain their IDs.
6. Retired ADRs retain their IDs.
7. Replacement decisions receive new IDs.
8. ADR identity remains stable even if its descriptive title is refined.
9. Gaps are permitted where an allocated ADR is later rejected or retired.
10. Never renumber historical ADRs to remove gaps.

The next new ADR number is determined from the highest canonical allocated ADR number, not from branch-local state.

---

## 4. Canonical Register Schema

| ADR ID | Title | Scope | Decision Owner | Status | ADR Path | Affected L5 Specifications | Allocated On | Decision Date | Supersedes | Superseded By | Notes |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `ADR-L5-0001` | Canonical Trust Boundary & Segmentation Pattern | Shared / Family | L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0001_Canonical_Trust_Boundary_and_Segmentation_Pattern.md` | `L5-D-03-001`; `L5-D-03-005` | 2026-09-11 | 2026-09-11 | — | — | Approved after controlled independent architecture review; not Locked. |
| `ADR-L5-0002` | Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern | Shared / Family | L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0002_Cryptographic_Protection_Key_Custody_and_Machine_Credential_Responsibility_Pattern.md` | `L5-D-03-002`; `L5-D-03-003`; `L5-D-03-004` | 2026-09-11 | 2026-09-11 | — | — | Approved after controlled independent architecture review; not Locked. |
| `ADR-L5-0003` | Contract-Driven Modular Platform with Selective Isolation | Repository | L5-A-01 — Application & Platform Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0003_Contract_Driven_Modular_Platform_with_Selective_Isolation.md` | `L5-A-01-001`; `L5-A-03-001`; `L5-A-03-002` | 2026-09-11 | 2026-09-11 | — | — | Consolidates A01/A03 application and module-boundary decision scope; not Locked. |
| `ADR-L5-0004` | Tenant and Institution Isolation Enforcement Pattern | Shared / Family | L5-A-04 — Tenant & Institution Isolation Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0004_Tenant_and_Institution_Isolation_Enforcement_Pattern.md` | `L5-A-04-001`; `L5-A-04-002` | 2026-09-11 | 2026-09-11 | — | — | Canonical allocation of pilot candidate ADR-C1; not Locked. |
| `ADR-L5-0005` | Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern | Shared / Family | L5-B-02 — Database & Persistence Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0005_Persistence_Ownership_Hybrid_Topology_and_Tenant_Isolation_Pattern.md` | `L5-B-02-001`; `L5-B-02-002` | 2026-09-11 | 2026-09-11 | — | — | Consolidates B02 persistence-topology and isolation decision scope; database product remains separate. |
| `ADR-L5-0006` | Owner-Local Transaction and Optimistic Concurrency Strategy | Shared / Family | L5-B-02 — Database & Persistence Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0006_Owner_Local_Transaction_and_Optimistic_Concurrency_Strategy.md` | `L5-B-02-003` | 2026-09-11 | 2026-09-11 | — | — | Canonical allocation of pilot candidate ADR-C3; not Locked. |
| `ADR-L5-0007` | Atomic State-to-Publication Intent Handoff Pattern | Repository | L5-B-02 — Database & Persistence Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0007_Atomic_State_to_Publication_Intent_Handoff_Pattern.md` | `L5-B-02-003`; materially constrains `L5-C-02-002` | 2026-09-11 | 2026-09-11 | — | — | Resolves pilot candidate ADR-C4 with B02 owning atomic handoff and C02 retaining delivery ownership. |
| `ADR-L5-0008` | Expand-Migrate-Transition-Contract Schema Evolution Pattern | Shared / Family | L5-B-02 — Database & Persistence Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0008_Expand_Migrate_Transition_Contract_Schema_Evolution_Pattern.md` | `L5-B-02-004` | 2026-09-11 | 2026-09-11 | — | — | Reusable B02 schema-evolution pattern; not Locked. |
| `ADR-L5-0009` | Non-Human Principal Authorization Binding Pattern | Shared / Family | L5-D-02 — Authorization Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0009_Non_Human_Principal_Authorization_Binding_Pattern.md` | `L5-D-02-001`; `L5-D-02-002`; `L5-D-02-003`; `L5-D-02-004`; `L5-D-02-006`; consumes `L5-D-01-002` | 2026-09-12 | 2026-09-12 | — | — | Approved Stage-2 remediation decision resolving machine/service/workload/job/integration authority binding without fabricating Person/Role Assignment semantics; not Locked. |
| `ADR-L5-0010` | Durable Event Delivery, Subscription, Ordering & Recovery Pattern | Shared / Family | L5-C-02 — Event & Messaging Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0010_Durable_Event_Delivery_Subscription_Ordering_and_Recovery_Pattern.md` | `L5-C-02-001`; `L5-C-02-002` | 2026-09-12 | 2026-09-12 | — | — | Approved Stage-2 remediation decision governing C02 delivery, subscription, acknowledgement, ordering and recovery semantics; no broker/product selection. |
| `ADR-L5-0011` | Event Contract Versioning & Consumer Compatibility Pattern | Shared / Family | L5-C-02 — Event & Messaging Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0011_Event_Contract_Versioning_and_Consumer_Compatibility_Pattern.md` | `L5-C-02-003`; collaborates with `L5-C-02-001` | 2026-09-12 | 2026-09-12 | — | — | Approved Stage-2 remediation decision governing event-contract evolution and compatibility semantics; no registry/serialization product selection. |
| `ADR-L5-0012` | Tamper-Evident Audit Evidence Storage, Integrity Stream, Checkpoint & Anchor Pattern | Specification | L5-D-04-003 — Tamper-Evident Audit Storage, Access & Export Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0012_Tamper_Evident_Audit_Evidence_Storage_Integrity_Stream_Checkpoint_and_Anchor_Pattern.md` | `L5-D-04-003` | 2026-09-12 | 2026-09-12 | — | — | Approved Stage-3 decision for canonical/query topology, Tenant-local versioned integrity streams, sequential linkage, checkpoint sealing and independently protected checkpoint anchoring; no product/algorithm selection. |
| `ADR-L5-0013` | Governed Background Job, Work-Unit & Technical Retry Execution Pattern | Shared / Family | L5-A-08 — Background Processing & Job Execution Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0013_Governed_Background_Job_Work_Unit_and_Technical_Retry_Execution_Pattern.md` | `L5-A-08-001`; `L5-A-08-002`; constrains consumers including `L5-A-05-003` | 2026-09-15 | 2026-09-15 | — | — | Approved Varad Stage-3 remediation decision for product-neutral asynchronous execution, work-unit, attempt, retry and A05/A08 handoff semantics; not Locked. |
| `ADR-L5-0014` | Repository Resilience, Health, Degradation & Recovery Pattern | Repository | L5-E-01 — Availability & Resilience Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0014_Repository_Resilience_Health_Degradation_and_Recovery_Pattern.md` | `L5-E-01-001`; `L5-E-01-002`; constrains reliability-aware L5 consumers | 2026-09-15 | 2026-09-15 | — | — | Approved Varad Stage-3 remediation decision for repository health, degradation, containment, validation and recovery semantics; not Locked. |
| `ADR-L5-0015` | Web Client Platform, Rendering & Deployment Pattern | Specification | L5-G-01 — Client Platform Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0015_Web_Client_Platform_Rendering_and_Deployment_Pattern.md` | `L5-G-01-002` | 2026-09-15 | 2026-09-15 | — | — | Approved immediate web-client stack: TypeScript + React + React Router Framework Mode, SPA-first static artifact and one modular web application; not Locked. |
| `ADR-L5-0016` | External Connector Adapter, Configuration & Failure-Isolation Pattern | Shared / Family | L5-C-03 — External Integration & Connector Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0016_External_Connector_Adapter_Configuration_and_Failure_Isolation_Pattern.md` | `L5-C-03-001`–`L5-C-03-009` | 2026-09-16 | 2026-09-16 | — | — | Approved Stage-5 C03 family pattern; provider/product selection remains outside this ADR. |
| `ADR-L5-0017` | Recovery Classification, Protected Copy, Restore & Disaster-Recovery Validation Pattern | Repository | L5-E-03 — Backup, Restore & Disaster Recovery Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0017_Recovery_Classification_Protected_Copy_Restore_and_Disaster_Recovery_Validation_Pattern.md` | `L5-E-03-001`; `L5-E-03-002`; `L5-E-03-003`; constrains recovery-aware L5 consumers | 2026-09-16 | 2026-09-16 | — | — | Approved Stage-5 E03 recovery strategy; provider/product and numeric RPO/RTO selection remain outside this ADR. |
| `ADR-L5-0018` | Primary Server Platform, Rust Standardization & Governed Runtime Exception Pattern | Repository | L5-A-02 — Technology Platform & Standards Architecture | Architecture Approved | `architecture/L5_Solution_Architecture/adr/ADR-L5-0018_Primary_Server_Platform_Rust_Standardization_and_Governed_Runtime_Exception_Pattern.md` | `L5-A-02-001`; constrains server-side SutraERP implementations and technology selections | 2026-09-16 | 2026-09-16 | — | — | Approved Stage-6 primary server-platform decision: Rust primary, governed runtime exceptions, pinned stable toolchain; framework/product selections remain separately governed. |

| `ADR-L5-0019` | Rust Web Service Framework & Async Runtime Standard | Repository | L5-A-02 — Technology Platform & Standards Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0019_Rust_Web_Service_Framework_and_Async_Runtime_Standard.md` | `L5-A-02-001`; Rust server implementations | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0020` | PostgreSQL & SQLx Primary Relational Persistence Standard | Shared / Family | L5-B-02 — Database & Persistence Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0020_PostgreSQL_and_SQLx_Primary_Relational_Persistence_Standard.md` | `L5-B-02-001`…`L5-B-02-004` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0021` | REST/JSON/OpenAPI Synchronous Service API Profile | Shared / Family | L5-C-01 — API & Contract Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0021_REST_JSON_OpenAPI_Synchronous_Service_API_Profile.md` | `L5-C-01-001`…`L5-C-01-003` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0022` | NATS JetStream Event Transport Standard | Shared / Family | L5-C-02 — Event & Messaging Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0022_NATS_JetStream_Event_Transport_Standard.md` | `L5-C-02-001`…`L5-C-02-003` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0023` | BPMN & Flowable Workflow Runtime Standard | Shared / Family | L5-A-05 — Workflow & Orchestration Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0023_BPMN_and_Flowable_Workflow_Runtime_Standard.md` | `L5-A-05-001`…`L5-A-05-003` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0024` | Keycloak Authentication Platform Standard | Shared / Family | L5-D-01 — IAM Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0024_Keycloak_Authentication_Platform_Standard.md` | `L5-D-01-001`…`L5-D-01-004` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0025` | OpenSearch Enterprise Search Projection Standard | Specification | L5-B-01 — Search & Indexing Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0025_OpenSearch_Enterprise_Search_Projection_Standard.md` | `L5-B-01-001`; `L5-B-01-002` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0026` | S3-Compatible Object Storage & Ceph RGW Reference Standard | Shared / Family | L5-B-04 — Document, Media & Object Platform Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0026_S3_Compatible_Object_Storage_and_Ceph_RGW_Reference_Standard.md` | `L5-B-04-001`; `L5-B-04-002` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0027` | OpenBao Secrets/Crypto Service & rustls TLS Standard | Shared / Family | L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0027_OpenBao_Secrets_Crypto_Service_and_Rustls_TLS_Standard.md` | `L5-D-03-001`…`L5-D-03-004` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0028` | OCI, Kubernetes & Provider-Neutral Deployment Standard | Repository | L5-E-04 — Runtime Packaging & Deployment Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0028_OCI_Kubernetes_and_Provider_Neutral_Deployment_Standard.md` | `L5-E-04-001`…`L5-E-04-005` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0029` | OpenTelemetry, Prometheus, Loki, Tempo & Grafana Observability Standard | Shared / Family | L5-E-05 — Observability & Operations Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0029_OpenTelemetry_Prometheus_Loki_Tempo_Grafana_Observability_Standard.md` | `L5-E-05-001`…`L5-E-05-004` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0030` | GitHub Actions, GHCR & Sigstore Supply-Chain Standard | Shared / Family | L5-F-01 — CI/CD & Software Supply Chain Security Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0030_GitHub_Actions_GHCR_and_Sigstore_Supply_Chain_Standard.md` | `L5-F-01-001`…`L5-F-01-003` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0031` | CEL Constrained Rules & Formula Language Standard | Specification | L5-A-07 — Secure Computational Engine Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0031_CEL_Constrained_Rules_and_Formula_Language_Standard.md` | `L5-A-07-005` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0032` | Rust Decimal Monetary Computation Representation Standard | Shared / Family | L5-A-07 — Secure Computational Engine Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0032_Rust_Decimal_Monetary_Computation_Representation_Standard.md` | `L5-A-07-002`; `L5-A-07-004` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |
| `ADR-L5-0033` | External Provider Pluggability & No Repository-Wide Vendor Default | Repository | L5-C-03 / L5-C-04 — External Integration & Communication Provider Architecture | Locked | `architecture/L5_Solution_Architecture/adr/ADR-L5-0033_External_Provider_Pluggability_and_No_Repository_Wide_Vendor_Default.md` | `L5-C-03-*`; `L5-C-04-*` | 2026-09-22 | 2026-09-22 | — | — | Locked by 2026-09-22 technology decision wave. |

### ADR Path

Once the ADR file exists, record its canonical repository path.

### Allocated On

Records when the ADR identifier became canonically reserved.

### Decision Date

Remains empty until an architecture decision is formally approved.

---

## 5. Allowed Scope

Use exactly:

```text
Specification
Shared / Family
Repository
```

Scope changes require architecture review because they can materially change decision authority and impact.

---

## 6. ADR Lifecycle

Use the locked ADR methodology states:

```text
Proposed
Under Evaluation
Architecture Approved
Locked
Rejected
Superseded
Retired
```

The canonical register mirrors the actual ADR state.

It does not grant approval merely by changing a table value.

### Initial Allocation State

A newly allocated ADR normally enters the register as:

```text
Proposed
```

The associated ADR document should then be created using the allocated ID.

---

## 7. Canonical ADR Repository Structure

```text
architecture/
└── L5_Solution_Architecture/
    ├── governance/
    │   └── L5_ADR_Register.md
    └── adr/
        ├── ADR-L5-0001_<Decision_Slug>.md
        ├── ADR-L5-0002_<Decision_Slug>.md
        └── ...
```

The register governs:

- identifier allocation;
- collision prevention;
- lifecycle indexing;
- supersession visibility.

Individual ADR documents govern:

- decision context;
- constraints;
- alternatives;
- evidence;
- decision;
- consequences;
- approval record.

---

## 8. Allocation Procedure

Before a new ADR is allocated:

### Step 1 — Confirm ADR Necessity

Apply the locked Technology Selection & ADR Methodology.

### Step 2 — Search Existing Decisions

Confirm that no active ADR already governs the same material decision.

### Step 3 — Resolve Scope & Owner

Identify:

- Specification / Shared / Repository scope;
- canonical technical decision owner.

### Step 4 — Allocate on Canonical Main

Add the next canonical ADR row.

The allocation commit is the authoritative reservation point.

### Step 5 — Create ADR File

Create:

```text
ADR-L5-NNNN_<Decision_Slug>.md
```

using exactly the allocated ID.

### Step 6 — Maintain Register

Update:

- path;
- affected specifications;
- lifecycle status;
- decision date;
- supersession fields

as governance progresses.

---

## 9. Parallel Authoring & Collision Handling

### Same Number Proposed on Multiple Branches

Only the allocation already canonical on `main` owns the number.

Other branches must rebase/re-fetch and allocate a new number.

### Duplicate ADR Purpose

If two authors identify the same architectural decision independently:

- do not allocate two ADRs merely because both were started;
- consolidate under one canonical ADR where the decision scope is genuinely the same.

### Similar But Distinct Decisions

Separate ADRs are appropriate only where architecture scope, technical owner or decision consequences are materially distinct.

---

## 10. Supersession

When an ADR is replaced:

```text
Old ADR
Status = Superseded
Superseded By = ADR-L5-NNNN

New ADR
Supersedes = old ADR ID
```

Both records remain permanently visible.

Do not delete or rewrite historical allocation evidence.

---

## 11. Register Change Governance

This is a **living canonical operational register**.

Its schema and allocation rules are governance-controlled.

Its rows are expected to change as ADRs are allocated and progress through their lifecycle.

Therefore:

```text
Locked register rules
≠
Frozen register contents
```

After the register structure is formally approved:

- normal row additions/status updates are governed operational maintenance;
- changing identifier format, allocation semantics, lifecycle vocabulary, or canonical ownership requires architecture governance review.

The register shall never be treated as immutable merely because its governance structure has been approved.

---

## 12. Integrity Failures

The following are repository-integrity defects:

- duplicate ADR ID;
- reused historical ID;
- ADR file with no canonical register allocation;
- register row referencing the wrong ADR file;
- conflicting lifecycle state between register and ADR;
- supersession links that are not reciprocal;
- author-created number not allocated on canonical `main`;
- separate ADRs owning the same material decision without justified scope separation.

Material defects shall be corrected before affected ADR approval.

---

## 13. Governance Effect

Approval of this register structure establishes:

- canonical ADR numbering;
- centralized allocation;
- collision prevention;
- lifecycle indexing;
- supersession visibility.

It does not:

- approve an ADR;
- select technology;
- authorize implementation;
- authorize broad L5 authoring.

---

**Status:** Locked  
**Authority:** Lead Architect — L5 ADR Allocation and Lifecycle Register Baseline  
**ADR IDs Currently Canonically Allocated:** 35  
**Next Canonical Allocation:** `ADR-L5-0036`
