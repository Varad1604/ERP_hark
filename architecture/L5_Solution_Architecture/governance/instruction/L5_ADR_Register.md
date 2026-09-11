# SutraERP L5 Architecture Decision Record Register

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-09-11  
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
- lifecycle visibility;
- supersession linkage.

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
**ADR IDs Currently Canonically Allocated:** 8  
**Next Canonical Allocation:** `ADR-L5-0009`
