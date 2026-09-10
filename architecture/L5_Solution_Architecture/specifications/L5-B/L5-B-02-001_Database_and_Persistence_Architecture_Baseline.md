# Database & Persistence Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-B-02-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-02 — Database & Persistence Architecture  
**Canonical Technical Responsibility:** Physical persistence architecture and principles without changing domain ownership.  
**Technical Architecture Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-10`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Architecture Family Register; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Broad Authoring Authorization v1.0.0; L5 Dependency-Gated Parallel Authoring Execution Timeline v1.0.0; `L5-A-01-001` v1.0.0 Draft; `L5-A-03-001` v1.0.0 Draft; `L5-A-04-001` v1.0.0 Draft; `L5-A-04-002` v1.0.0 Draft; `L4-A-001` v1.0.0 Draft; `L4-A-019` v1.0.0 Draft; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** Dependency-gated downstream areas including L5-B-03, L5-C-02, L5-B-05, L5-E-02, L5-A-07, L5-B-01, L5-B-04, L5-B-06 and L5-E-03.  
**Applicable ADRs:** Required but unresolved — persistence strategy / hybrid topology and primary relational database technology. No canonical L5 ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the SutraERP baseline persistence architecture: the default persistence model, ownership-preserving persistence boundaries, logical-to-physical database topology, authoritative-versus-derived data position, and the rules that allow physical infrastructure to be shared without creating shared ownership.

This specification establishes the common persistence foundation for `L5-B-02-002` to `L5-B-02-004`. It does not decide business ownership and does not treat a database, schema, engine or infrastructure unit as the owner of a business fact merely because it stores that fact.

### In Scope

- Relational-first persistence for authoritative ERP data.
- A governed direction toward one primary relational database technology for SutraERP, with the actual product selected only through ADR governance.
- Authoritative Persistence Boundaries aligned to exactly one canonical owner.
- A hybrid topology in which logical persistence boundaries remain separate while physical database infrastructure may be shared where safe.
- Criteria for when physical database separation must be evaluated.
- The rule that direct cross-owner database access is prohibited.
- Tenant/Institution scope requirements at protected persistence boundaries.
- Separation of authoritative data from caches, search indexes, analytics projections and other derived stores.
- The rule that secure computational-engine isolation does not automatically create new data ownership or require a separate authoritative database.

### Out of Scope / Non-Goals

- Exact domain-to-database or Tenant-to-database mapping — `L5-B-02-002`.
- Transaction boundaries, consistency models and concurrency mechanisms — `L5-B-02-003`.
- Schema evolution, structural migration and persistence migration mechanics — `L5-B-02-004`.
- Search/index internals — `L5-B-01-*`.
- Reporting/analytics/projection internals — `L5-B-06-*`.
- Document/object-storage architecture — `L5-B-04-*`.
- Privacy classification, masking or sensitive-data controls — `L5-B-05-*`.
- Tenant/Institution isolation semantics and context propagation — `L5-A-04-*`.
- Security/trust-control implementation — L5-D.
- Backup/restore/disaster-recovery mechanics — `L5-E-03-*`.
- Runtime/deployment topology — `L5-E-04-*`.
- Complete DDL, tables, columns, indexes, ORM design or SQL implementation.
- A fixed number of physical databases before the isolation and deployment analysis is complete.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-B-02-001 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-B-02-001` row | B02-001 owns physical persistence architecture and principles without changing domain ownership. | Establishes canonical responsibility. |
| L5 Governance | `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md` | v1.0.0 / Locked | Stage 1; Dhruv execution order | B02 is a foundation dependency for multiple later L5 areas. | Requires a reusable baseline rather than domain-specific database design. |
| L4 | `L4-A-001_Tenant_Provisioning_to_Active_Tenant_Availability.md` | v1.0.0 / Draft | `BH-003`, `BH-005`, `BH-006`, `BH-011`, `BH-013`, `BH-015` | Tenant environment/isolation must be established; valid lifecycle state and evidence must survive success/failure/continuation. Persistence implementation is deferred to L5. | Requires persistence capable of preserving authoritative Tenant-scoped state without owning Tenant lifecycle semantics. |
| L4 | `L4-A-019_Committed_Business_Fact_to_Reliable_Consumer_Delivery.md` | v1.0.0 / Draft | `BH-001`, `BH-003`, `BH-004`, `BH-010` | Event publication follows a committed domain-owned business fact; event identity/immutability/traceability must be preserved. | Requires an authoritative persistence boundary that can distinguish committed source state from derived/event infrastructure; transaction-to-publication consistency remains B02-003/C02. |
| L5 Peer | `L5-A-01-001_Application_and_Platform_Architecture.md` | v1.0.0 / Draft | §§4–5; persistence boundary references | Application co-location or selective isolation shall not transfer data ownership or permit persistence bypass. | Persistence boundaries remain valid regardless of application placement. |
| L5 Peer | `L5-A-03-001_Module_Dependency_and_Technical_Boundary_Architecture.md` | v1.0.0 / Draft | §§4–5; B02 collaboration boundary | Technical-module composition does not grant access to another owner's persistence. | Prevents shared runtime/module composition from becoming shared database authority. |
| L5 Peer | `L5-A-04-001` / `L5-A-04-002` | v1.0.0 / Draft | A04 isolation and persistence-context boundaries | Tenant is the root protected execution scope; validated Tenant/Institution context must reach protected persistence. Physical separation is not automatically required per Tenant. | B02 consumes valid scope and realizes the persistence side without redefining A04 semantics. |
| L1 | `SutraERP_L1_Business_Domain_Architecture_Locked.md` | v2.1 / Locked | §§2.1, 2.4 | Canonical ownership is singular; only the owner changes authoritative business data; cross-domain direct database access is prohibited. | Establishes the non-negotiable persistence ownership boundary. |
| L5 Governance | `L5_Technology_Selection_and_ADR_Methodology.md` | v1.0.1 / Locked | §§1–6, especially mandatory ADR triggers | Persistence strategy and database technology selection require governed ADR handling when material. | Keeps the selected Draft direction provisional until ADR approval. |
| L5 Governance | `L5_ADR_Register.md` | v1.0.0 / Locked | §§1–8 | ADR IDs may only be allocated on canonical `main`; no L5 ADR is currently allocated. | Prevents this Draft from inventing an ADR identifier. |

These L4 references establish selective material applicability for this reusable baseline. They are not a replacement for the canonical repository-wide BH-to-L5 traceability store.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Source | L5 Effect |
|---|---|---|---|
| `C-001` | A persistence mechanism shall never transfer canonical business ownership. | L1 §§2.1/2.4; L5 Governance | Every authoritative persistence boundary maps to one canonical owner. |
| `C-002` | Direct cross-owner database access is prohibited. | L1 §§2.1/2.4; A01/A03 | Another owner consumes data through governed contracts, events, workflow or approved projections, not direct table/database access. |
| `C-003` | Protected Tenant-scoped persistence shall consume validated Tenant/Institution scope. | A04-001/A04-002 | Missing, contradictory or widened scope shall not fall back to unscoped persistence. |
| `C-004` | Physical co-location does not create shared ownership or write authority. | L5 Governance; A01/A03 | Several logical persistence boundaries may share physical infrastructure only while isolation remains enforceable. |
| `C-005` | Derived stores do not become canonical owners merely because they contain copies. | Frozen B01/B06 responsibility boundaries; L5 ownership rules | Cache/index/projection data remains derived from authoritative owners. |
| `C-006` | Database product selection is not yet canonical. | L5 Technology Selection & ADR Methodology; ADR Register | No database vendor/product is named as approved architecture in this Draft. |
| `C-007` | Exact database count is a consequence of isolation/deployment decisions, not business-domain count alone. | Frozen B02 split; A04 physical-isolation boundary | B02-001 defines the mapping model; B02-002 and collaborating security/runtime/recovery specs determine exact physical separation. |
| `C-008` | Transaction/concurrency and schema-evolution responsibilities remain separate. | Frozen `L5-B-02-003` / `L5-B-02-004` boundaries | B02-001 shall not absorb their mechanics. |

### 3.3 Assumptions

No exact database product, physical database count, schema count, capacity value, replication pattern or storage topology is assumed.

The Lead Architect has selected a **relational-first, single-primary-technology, hybrid logical/physical persistence direction** for Draft authoring on 2026-09-10. Because this is a material persistence strategy, it remains provisional pending governed ADR handling.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The concept of an **Authoritative Persistence Boundary**: a technical persistence boundary containing authoritative data for exactly one canonical owner.
- The **Persistence Access Boundary** through which an owning capability performs authoritative persistence operations.
- The distinction between a logical persistence boundary and its physical database deployment.
- The hybrid logical-to-physical mapping model.
- The relational-first default for authoritative ERP persistence, subject to ADR approval.
- The requirement that any non-relational authoritative persistence proposal has explicit architectural justification and applicable ADR review.
- The rule that derived stores never gain business ownership or become an alternate source of truth by replication alone.
- The criteria that trigger evaluation of stronger physical separation.

### Consumes / Collaborates With

| Exact L5 Architecture | Consumed / Collaborating Responsibility | B02-001 Boundary |
|---|---|---|
| `L5-A-01-001` | Application/platform placement | Persistence ownership remains invariant across modular-core and selectively isolated execution placement. |
| `L5-A-03-001/002` | Technical module/dependency boundaries | Technical co-location shall not create database bypass paths. |
| `L5-A-04-001/002` | Tenant/Institution isolation and validated scope propagation | B02 consumes validated scope at persistence boundaries; A04 owns scope semantics/carriage. |
| `L5-B-02-002` | Domain persistence ownership & isolation | Determines detailed logical/physical mapping and stronger separation requirements. |
| `L5-B-02-003` | Transaction, consistency & concurrency | Owns commit boundaries, concurrency and state/event consistency. |
| `L5-B-02-004` | Schema evolution & data migration | Owns controlled structural evolution and internal migration. |
| `L5-B-01-*` | Search/indexing | Indexes consume authorized canonical facts and remain derived. |
| `L5-B-06-*` | Reporting/analytics/projection | Reporting/projection stores remain derived unless upstream architecture explicitly assigns canonical ownership elsewhere. |
| `L5-C-01-*` / `L5-C-02-*` | Synchronous/asynchronous contracts | Cross-owner data consumption uses governed technical boundaries rather than direct persistence access. |
| `L5-D-03-*` / `L5-B-05-*` | Security/trust and sensitive-data controls | May require stronger persistence isolation; B02 does not redefine security/privacy semantics. |
| `L5-E-02-*` | Performance/capacity | Supplies material workload evidence that may justify physical separation or scaling. |
| `L5-E-03-*` | Backup/restore/disaster recovery | Recovery must preserve authoritative ownership and logical boundary mapping. |
| `L5-E-04-*` | Runtime/deployment | Realizes physical deployment without weakening B02 ownership/isolation rules. |
| `L5-A-07-*` | Secure computational engines | Engine isolation does not itself transfer data ownership or automatically require a new authoritative database. |

### Must Not Own

- Business-domain/entity ownership, rules, lifecycle states, transitions, events or institutional authority.
- Tenant/Institution isolation semantics.
- Authorization decisions or security policy.
- Search, analytics or cache ownership.
- Transaction/concurrency algorithms.
- Schema-evolution/migration procedure.
- Backup/recovery procedure.
- Runtime deployment topology.
- Database vendor/product selection without approved ADR authority.
- Physical database count before B02-002 and collaborating architecture determine the required separation.

---

## 5. Solution Architecture

### 5.1 Canonical Persistence Model

For authoritative ERP state, the Draft direction is **relational-first**.

```text
Canonical business owner
        ↓
Owning technical capability
        ↓
Persistence Access Boundary
        ↓
Authoritative Persistence Boundary
        ↓
Logical Persistence Unit
        ↓
Physical Persistence Mapping
        ↓
Shared physical database platform
        OR
Dedicated physical database boundary
```

A logical persistence boundary and a physical database are deliberately different concepts.

A logical boundary exists to preserve ownership and access isolation. A physical database boundary exists for technical isolation, scale, resilience, performance or operational reasons. One shall not be inferred from the other.

### 5.2 Authoritative Persistence Boundary

Each authoritative persisted data set shall resolve to exactly one canonical owner and one corresponding owner-scoped persistence boundary.

The boundary shall ensure:

- authoritative writes originate only through the owning technical capability;
- another domain/module cannot acquire write authority because it shares a process, host, schema server or database platform;
- direct cross-owner reads or writes do not become an integration mechanism;
- Tenant/Institution-scoped access consumes the valid A04 execution scope;
- database-level placement never redefines upstream entity ownership.

An owner may require more than one logical persistence unit where later architecture justifies separation by sensitivity, scale, recovery or other material technical constraints. Multiple units do not create multiple business owners.

### 5.3 Hybrid Logical / Physical Topology

The selected Draft topology is **hybrid**:

```text
Many owner-scoped logical persistence boundaries
                    ↓
        placement / isolation decision
             ↙                 ↘
shared physical platform     dedicated physical boundary
```

Shared physical infrastructure is permitted only when the implementation can preserve all logical ownership, Tenant/Institution isolation and access-control boundaries.

Physical separation shall be evaluated when one or more of the following is materially present:

- a stronger security or trust boundary is required by L5-D;
- sensitive-data controls require stronger isolation under L5-B-05;
- a secure computational-engine architecture requires independently protected persistence interaction;
- materially different performance/capacity characteristics require workload isolation;
- availability/resilience or recovery architecture requires independent failure/recovery scope;
- an authoritative compliance or deployment constraint requires stronger physical separation.

This baseline intentionally does not assign a fixed physical database count.

### 5.4 Tenant / Institution Persistence Scope

B02 consumes the A04 validated execution scope.

For protected Tenant-scoped persistence:

```text
Validated Tenant / Institution scope
        ↓
Owner-scoped Persistence Access Boundary
        ↓
Scope and owner consistency check
        ↓
Authoritative Persistence Boundary
```

Missing, conflicting, widened or substituted scope shall fail closed. A database connection, shared schema or physical co-location shall not be treated as evidence of valid Tenant scope.

SutraERP does **not** require a separate physical database per Tenant by default. Stronger Tenant-specific physical isolation may be introduced only where B02-002 and collaborating security/deployment architecture justify it.

### 5.5 Cross-Owner Data Access

Cross-owner direct database access is prohibited for both mutation and integration-style querying.

```text
Owner A needs Owner B data
        ↓
NOT: direct SQL / table access
        ↓
Use canonical contract / event / workflow / approved projection
owned by the applicable L5 architecture
```

This applies even when both owners are hosted in the same application process or on the same physical database platform.

### 5.6 Authoritative vs Derived Stores

The authoritative persistence boundary is the source of truth for the business facts owned by its canonical owner.

The following are derived when used as copies/projections of authoritative facts:

- caches;
- search indexes;
- analytics/reporting projections;
- read-optimized copies;
- technical replicas that do not receive canonical ownership through upstream architecture.

A derived store shall not:

- accept a business correction as an authoritative write path;
- independently redefine canonical state;
- become the recovery source of truth merely because the primary source is unavailable;
- transfer ownership to its technical platform.

### 5.7 Secure Computational Engines

Finance, Examination and Payroll secure computational-engine separation does not by itself create separate canonical data ownership or mandate one physical database per engine.

Their authoritative data remains inside the persistence boundary of the canonical owner. B02-002, A07 and applicable D/E architecture shall determine whether stronger physical persistence isolation is required.

### 5.8 Primary Write Flow

```text
Validated execution context
        ↓
Owning technical capability
        ↓
Persistence Access Boundary
        ↓
Owner + scope validation
        ↓
B02-003 governed transaction / concurrency path
        ↓
Authoritative persistence commit
        ↓
Post-commit contract/event handoff where required
```

B02-001 defines the persistence ownership/topology boundary only. Commit atomicity, consistency and publication-handoff mechanics are owned by `L5-B-02-003` and collaborating `L5-C-02-*` architecture.

### 5.9 Material Failure Boundaries

| Failure Class | Required Outcome | Forbidden Outcome | Delegated / Collaborating Owner |
|---|---|---|---|
| Missing/invalid Tenant scope | Reject protected persistence access. | Default/unscoped access. | A04-002 / B02-002 |
| Cross-owner persistence attempt | Reject the direct access path. | Convenience SQL/table access into another owner. | B02-002 / A03 / C01/C02 |
| Physical database boundary unavailable | Fail within the affected owner/boundary and use governed resilience/recovery. | Writing to another owner's store or promoting a derived copy to authority. | E01/E03/E04 |
| Derived store stale/unavailable | Treat it as derived and recover/rebuild through its canonical owner. | Treating stale derived state as authoritative. | B01/B06/applicable owner |
| Ambiguous physical placement/mapping | Fail closed and raise operational/configuration fault. | Guessing another schema/database or widening scope. | B02-002 / E04 |
| Commit/concurrency failure | Defer to governed transaction failure semantics. | Inventing transaction behavior in this baseline. | B02-003 |
| Migration/schema incompatibility | Defer to governed evolution/migration handling. | Ad-hoc structural repair. | B02-004 |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Status | Basis | Consequence | ADR |
|---|---|---|---|---|---|
| `AD-001` | Authoritative ERP persistence is relational-first. A non-relational authoritative store requires explicit architecture justification. | **Provisional Draft direction** | B02-001 canonical responsibility; Lead Architect selection 2026-09-10. | Gives SutraERP one integrity-oriented default without forbidding justified specialized stores. | **Required — persistence-strategy ADR scope review; see OD-001.** |
| `AD-002` | SutraERP shall converge on one primary relational database technology standard; exceptions require governed justification. The product is not selected by this Draft. | **Provisional Draft direction** | Lead Architect selection 2026-09-10; L5 Technology Selection & ADR Methodology. | Reduces uncontrolled technology proliferation while keeping product evaluation evidence-based. | **Required — Repository ADR for product selection; see OD-002.** |
| `AD-003` | Use a hybrid topology: owner-scoped logical persistence boundaries are mandatory; physical database sharing is allowed where safe; physical separation is introduced only when justified by material isolation/quality constraints. | **Provisional Draft direction** | A04 physical-isolation boundary; B02 responsibility; Lead Architect selection 2026-09-10. | Avoids both one giant shared ownership model and unnecessary database-per-domain proliferation. | **Required — persistence/topology ADR scope review; see OD-001.** |
| `AD-004` | Physical co-location shall never grant cross-owner persistence access. | Established / derived from locked ownership rules | L1 §§2.1/2.4; L5 Governance; A01/A03. | Allows shared infrastructure without shared authority. | No new ADR required unless a future exception changes the ownership/isolation strategy. |
| `AD-005` | Derived copies are not alternate sources of truth. | Established / defensible L5 derivation | B01/B06 responsibility boundaries; single-owner rule. | Prevents cache/index/reporting state from silently becoming canonical. | No new ADR required for the baseline principle. |
| `AD-006` | Secure computational-engine separation does not automatically mandate a dedicated authoritative database. | Defensible boundary decision | A07 secure-engine responsibility; B02 ownership boundary; Lead Architect selection 2026-09-10. | Physical separation remains evidence-driven and is decided in B02-002 with security/runtime peers. | No separate ADR unless the resulting isolation choice is material under ADR methodology. |

No database vendor, ORM, cloud database service or deployment technology is selected by this document.

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Owning / Collaborating L5 Spec |
|---|---|---|
| Tenant/Institution isolation | Every protected persistence operation must remain inside validated A04 scope; physical sharing shall not weaken this. | A04-001/002, B02-002 |
| Data ownership | Credentials, schemas/databases and access paths must make direct cross-owner persistence bypass impossible by design. | B02-002, A03, D02/D03 |
| Sensitive data | Stronger physical isolation may be required where authoritative privacy/security architecture demands it. | B05, D03, B02-002 |
| Availability / resilience | Physical separation may be used to contain failure where justified; exact HA/recovery mechanism is not owned here. | E01, E03, E04 |
| Performance / capacity | High-contention or materially distinct workloads may justify dedicated physical placement; no threshold is invented here. | E02, B02-002 |
| Operability | The primary-technology direction reduces unnecessary platform diversity; exact product and operations model remain ADR/E04 decisions. | A02, E04, E05 |
| Portability | The baseline is product-neutral and shall not assume a cloud-only persistence service. | Technology Selection methodology, E04 |
| Recoverability | Backup/restore must preserve owner and Tenant boundary mapping and must not merge canonical authority. | E03, B02-002 |

---

## 8. Architecture Conformance Requirements

| CR ID | Conformance Requirement | Evidence Expected |
|---|---|---|
| `CR-001` | Every authoritative persisted data set shall map to exactly one canonical owner and an owner-scoped Authoritative Persistence Boundary. | Persistence ownership mapping. |
| `CR-002` | No domain/module may directly read or write another owner's authoritative database/schema as an integration shortcut. | Dependency/access review showing governed contracts/events/projections instead. |
| `CR-003` | Protected Tenant-scoped persistence shall require valid A04 Tenant/Institution context and fail closed when that scope is invalid or absent. | Persistence-boundary scope enforcement evidence. |
| `CR-004` | Shared physical database infrastructure shall preserve logical owner boundaries; co-location shall not create shared credentials or shared write authority that bypass canonical ownership. | Access/isolation design evidence. |
| `CR-005` | Authoritative ERP persistence shall follow the relational-first Draft direction until superseded by governed ADR outcome; any proposed non-relational authoritative store shall be explicitly justified. | ADR/architecture reference for exception. |
| `CR-006` | Cache, search, reporting and projection stores containing copies of canonical facts shall remain non-authoritative unless upstream architecture explicitly assigns ownership differently. | Data-flow/source-of-truth mapping. |
| `CR-007` | Physical separation shall be justified by material security, sensitivity, performance, resilience, recovery, compliance or operational constraints rather than by business-domain count alone. | Placement/isolation decision evidence. |
| `CR-008` | A separate secure computational engine shall not automatically receive a new authoritative database or new data ownership. | A07/B02 ownership and placement mapping. |
| `CR-009` | No database product shall be represented as canonical until the applicable Repository ADR is allocated and approved. | ADR Register / approved ADR reference. |
| `CR-010` | Transaction, concurrency and state/event consistency mechanics shall be delegated to `L5-B-02-003`. | B02-003 dependency evidence. |
| `CR-011` | Schema evolution and structural migration mechanics shall be delegated to `L5-B-02-004`. | B02-004 dependency evidence. |
| `CR-012` | The implementation shall not infer a fixed physical database count from this baseline; exact mapping shall follow `L5-B-02-002` and collaborating D/E architecture. | Logical-to-physical persistence mapping. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Owner | Blocking Status | Resolution Path |
|---|---|---|---|---|
| `OD-001` | Formalize or revise the relational-first and hybrid persistence-topology strategy. | L5-B-02 / Lead Architect | **Blocks Architecture Approved for this Draft direction.** | Apply ADR-necessity/scope review, allocate canonical ADR ID on `main`, evaluate alternatives and obtain approval. |
| `OD-002` | Select the primary relational database technology. | L5-B-02 with Repository ADR governance; A02 later consolidates the approved standard | Does not block continued Draft architecture work; blocks canonical technology standardization. | Define criteria, evaluate viable products, allocate/approve Repository ADR, then propagate into B02/A02. |
| `OD-003` | Determine exact logical persistence boundaries and which require dedicated physical separation, including Finance/Examination/Payroll where justified. | `L5-B-02-002` with A04/A07/B05/D03/E02/E03/E04 | Not a B02-001 Draft blocker. | Resolve in B02-002 using this baseline. |
| `OD-004` | Transaction, concurrency, consistency and state/publication-handoff strategy. | `L5-B-02-003` with C02 boundary | Not a B02-001 Draft blocker; blocks affected B02-003 approval. | Resolve under the existing material ADR-candidate process for B02-003/C02. |

No canonical L5 ADR number is allocated by this specification.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-003/005/006/011/013/015` — provides the persistence foundation required to preserve Tenant-scoped authoritative state, valid continuation state and evidence while leaving Tenant lifecycle meaning upstream-owned.
- `L4-A-019-BH-001/003/004/010` — provides the authoritative committed-state/source-of-truth boundary on which event publication and traceability depend; transaction/publication mechanics are delegated to B02-003/C02.

Both cited L4 specifications are currently Draft. They support controlled Draft L5 authoring but are not treated as immutable approved behavioral authority.

### Higher-Level Ownership Verification

- SutraERP L1 Business Domain Architecture v2.1 §§2.1/2.4 — singular canonical ownership and prohibition on direct cross-domain authoritative persistence access.
- `L5-A-01-001` / `L5-A-03-001` — application/module placement cannot bypass persistence ownership.
- `L5-A-04-001` / `L5-A-04-002` — validated Tenant/Institution scope must remain effective at protected persistence boundaries while physical infrastructure separation is not automatic.

### Canonical B02 Responsibility Split

```text
L5-B-02-001
Persistence baseline + logical/physical topology
        ↓
L5-B-02-002
Detailed ownership/isolation + physical-separation mapping
        ↓
L5-B-02-003
Transaction + consistency + concurrency
        ↓
L5-B-02-004
Schema evolution + data migration
```

This document shall remain `Draft` while `OD-001` is unresolved and shall not be treated as an approved database-product selection.