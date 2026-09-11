# Domain Persistence Ownership & Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-02-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-02 — Database & Persistence Architecture  
**Canonical Technical Responsibility:** Enforce technical persistence ownership and prevent unauthorized cross-domain writes.  
**Technical Architecture Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Broad Authoring Authorization v1.0.0; L5 Dependency-Gated Parallel Authoring Execution Timeline v1.0.0; `L5-B-02-001` v1.0.0 Draft; `L5-A-03-001` v1.0.0 Draft; `L5-A-04-001` v1.0.0 Draft; `L5-A-04-002` v1.0.0 Draft; `L4-A-001` v1.0.0 Draft; `L4-A-015` v1.0.0 Draft; `L4-A-019` v1.0.0 Draft; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** Downstream persistence, privacy, reporting, secure-engine, recovery and integration architecture consuming owner-scoped persistence boundaries.  
**Applicable ADRs:** Required but unresolved — persistence-isolation strategy. ADR scope must be reviewed together with the unresolved persistence/topology decision in `L5-B-02-001`; no canonical L5 ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the enforceable persistence-isolation architecture that converts SutraERP's canonical single-owner business model into technical database boundaries. This specification ensures that shared physical persistence infrastructure never becomes shared authority, that Tenant-scoped data remains isolated inside the owning persistence boundary, and that another owner cannot bypass governed contracts by directly querying or mutating authoritative persistence.

### In Scope

- Owner-scoped persistence namespaces and logical persistence boundaries.
- Owner-scoped runtime database identities and privileges.
- Technical enforcement preventing cross-owner direct reads/writes.
- Hybrid Tenant persistence isolation inside an owner boundary.
- Promotion from shared Tenant-scoped storage to stronger owner+Tenant schema/database isolation when justified.
- Persistence placement resolution between shared and dedicated logical/physical boundaries.
- Separation of normal runtime persistence identities from privileged maintenance/migration identities.
- Fail-closed behavior for missing owner, Tenant, Institution or placement context.
- Enforcement rules that remain valid whether physical database infrastructure is shared or dedicated.

### Out of Scope / Non-Goals

- Overall relational-first persistence strategy or database-product selection — `L5-B-02-001` and applicable ADRs.
- Transaction boundaries, isolation levels, concurrency control or state/event consistency — `L5-B-02-003`.
- Schema evolution, structural migration or migration tooling — `L5-B-02-004`.
- Tenant/Institution isolation semantics or context-carrier mechanics — `L5-A-04-*`.
- Authorization policy semantics — `L5-D-02-*`.
- Cryptography, secrets/key technology or network trust controls — `L5-D-03-*`.
- Runtime/deployment topology — `L5-E-04-*`.
- Backup/restore/DR mechanics — `L5-E-03-*`.
- Complete database DDL, table/column/index design, connection strings or ORM configuration.
- A fixed physical database count.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-B-02-002 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-B-02-002` row | B02-002 owns technical persistence ownership/isolation and prevention of unauthorized cross-domain writes. | Establishes canonical responsibility. |
| L5 Peer | `L5-B-02-001_Database_and_Persistence_Architecture_Baseline.md` | v1.0.0 / Draft | §§4–6; `AD-003`–`AD-006`; `CR-001`–`CR-012` | Authoritative persistence maps to one canonical owner; hybrid logical/physical topology; direct cross-owner access prohibited; exact mapping belongs here. | Supplies the baseline this specification makes enforceable. |
| L5 Peer | `L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md` | v1.0.0 / Draft | §§4–5 | Tenant-scoped persistence shall consume validated Tenant/Institution scope; unscoped, widened or substituted scope fails closed; physical schema/database design belongs to B02. | Supplies mandatory persistence-scope input without transferring A04 ownership. |
| L5 Peer | `L5-A-03-001_Module_Dependency_and_Technical_Boundary_Architecture.md` | v1.0.0 / Draft | `C-006`; B02 collaboration boundary | Technical-module composition shall not create cross-owner persistence access merely because modules share a parent/runtime. | Requires isolation to survive application/module co-location. |
| L4 | `L4-A-001_Tenant_Provisioning_to_Active_Tenant_Availability.md` | v1.0.0 / Draft | `BH-003`, `BH-005`, `BH-009` | Tenant environment/isolation is established and Active Tenant context becomes available downstream without ownership transfer. | Requires persistence isolation to preserve the single Tenant boundary. |
| L4 | `L4-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md` | v1.0.0 / Draft | `BH-001`, `BH-004`, `BH-005`, `BH-011` | Workflow executes within one Tenant, does not directly change participating-domain state, and obtains authoritative results from owning domains. | Prevents orchestration from using direct database access to bypass domain ownership. |
| L4 | `L4-A-019_Committed_Business_Fact_to_Reliable_Consumer_Delivery.md` | v1.0.0 / Draft | `BH-001`, `BH-002`, `BH-005`, `BH-007`, `BH-010` | Business facts remain domain-owned; publication/routing is Tenant-scoped; consumers receive facts without ownership transfer. | Requires source persistence to remain owner/Tenant scoped across asynchronous collaboration. |
| L1 | `SutraERP_L1_Business_Domain_Architecture_Locked.md` | v2.1 / Locked | §§2.1, 2.4 | Each business capability has one canonical owner; only the owner changes authoritative business data; cross-domain collaboration uses governed boundaries; direct cross-domain database access is prohibited. | Establishes the non-negotiable ownership and bypass prohibition. |
| L5 Governance | `L5_Technology_Selection_and_ADR_Methodology.md` | v1.0.1 / Locked | §§2–4, mandatory ADR triggers | A material persistence-isolation strategy requires ADR governance. | Keeps `AD-001`–`AD-003` provisional until ADR resolution. |
| L5 Governance | `L5_ADR_Register.md` | v1.0.0 / Locked | §§1–8 | ADR IDs are allocated only through the canonical register; none are currently allocated. | Prevents this Draft from inventing an ADR identifier. |

Draft L4 sources support controlled Draft authoring only; they are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Source | L5 Effect |
|---|---|---|---|
| `C-001` | Authoritative persistence belongs to exactly one canonical owner. | L1 §§2.1/2.4; B02-001 | Every runtime persistence path shall resolve to one owner-scoped persistence boundary. |
| `C-002` | Direct cross-owner database access is prohibited. | L1; B02-001; A03-001 | Shared infrastructure shall not expose another owner's authoritative objects to a normal runtime identity. |
| `C-003` | Tenant is the root protected persistence scope; Institution may only narrow it. | A04-001/002 | Persistence access shall consume validated Tenant/Institution scope and fail closed on mismatch. |
| `C-004` | Shared physical infrastructure does not create shared write authority. | L5 Governance; B02-001 | Logical namespace, identity and privilege isolation remain mandatory on a shared database platform. |
| `C-005` | Exact Tenant physical separation is not universal. | A04; B02-001 | Shared owner-scoped storage is permitted by default; stronger isolation is introduced only when justified. |
| `C-006` | Technical persistence placement shall not create or alter business ownership. | L1; L5 Validation HF-04 | Dedicated schemas/databases remain technical placement, not new business owners. |
| `C-007` | Privileged technical access shall not become an alternate normal business write path. | L5 Validation HF-07/HF-12; D02/D03 ownership | Maintenance/migration identities require separate controlled use and shall not substitute for owner runtime identities. |
| `C-008` | Transaction/concurrency and schema evolution remain peer responsibilities. | Frozen B02 split | This specification constrains access/isolation only. |

### 3.3 Assumptions

No database product, server topology, schema naming convention, connection-pool product, row-level security feature or cloud service is assumed.

The Lead Architect selected the following Draft architecture directions on 2026-09-11:

- hybrid Tenant persistence isolation;
- owner-scoped runtime database identities;
- separate owner-scoped schema/namespace boundaries when owners share a relational database platform.

Because these choices establish a reusable persistence-isolation strategy, they remain provisional pending ADR governance.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The **Owner Persistence Boundary** mapping one authoritative persistence scope to one canonical owner.
- The **Owner Persistence Namespace** used to prevent one owner's authoritative objects from becoming another owner's internal database surface.
- The **Owner Runtime Persistence Identity** and least-privilege relationship to its owner namespace.
- The **Persistence Scope Guard** that consumes validated Tenant/Institution scope before Tenant-scoped persistence access.
- The hybrid Tenant placement model inside an owner boundary.
- The requirement for stronger owner+Tenant isolation where authoritative security, sensitivity, scale, resilience, recovery or contractual constraints justify it.
- The technical prohibition on normal cross-owner database access.
- The separation of privileged maintenance/migration persistence identities from normal runtime persistence identities.

### Consumes / Collaborates With

| Exact L5 Architecture | Consumed Responsibility | B02-002 Boundary |
|---|---|---|
| `L5-B-02-001` | Persistence baseline and hybrid logical/physical topology | B02-002 realizes ownership/isolation without changing the baseline technology decision. |
| `L5-B-02-003` | Transaction, consistency & concurrency | Executes only after owner/scope isolation is established; B02-002 does not define transaction semantics. |
| `L5-B-02-004` | Schema evolution & migration | Evolves owner-scoped namespaces while preserving B02-002 ownership/isolation. |
| `L5-A-03-001/002` | Technical module/dependency boundaries | Module composition shall not create a persistence bypass. |
| `L5-A-04-001/002` | Tenant/Institution isolation and propagated validated scope | B02-002 consumes scope and enforces persistence placement/access; A04 owns scope meaning. |
| `L5-C-01-*` / `L5-C-02-*` | Governed synchronous/asynchronous collaboration | Cross-owner consumers use contracts/events rather than direct SQL/database access. |
| `L5-D-02-*` | Authorization / privileged access | Supplies authorization controls around administrative/privileged operations; B02 does not own authorization policy. |
| `L5-D-03-*` | Security/trust/secrets | Protects database credentials and may require stronger trust isolation. |
| `L5-B-05-*` | Sensitive-data handling | May justify stronger physical or Tenant isolation without B02 redefining privacy classification. |
| `L5-E-02-*` | Performance/capacity | May justify dedicated placement where workload isolation is material. |
| `L5-E-03-*` | Backup/restore/DR | Recovery shall preserve owner/Tenant placement and authority boundaries. |
| `L5-E-04-*` | Runtime/deployment | Realizes database deployment/placement without weakening B02-002 controls. |

### Must Not Own

- Business domains, entity ownership, business rules, lifecycle states, events or institutional authority.
- Tenant/Institution hierarchy or scope semantics.
- User/business-role authorization decisions.
- Database-product selection.
- Transaction isolation/concurrency algorithms.
- Schema migration procedure/tooling.
- Backup/recovery procedure.
- Network segmentation, encryption, key or secret technology.
- Complete table/column/index/foreign-key design.

---

## 5. Solution Architecture

### 5.1 Persistence Isolation Stack

Every protected authoritative persistence operation shall traverse this conceptual enforcement chain:

```text
Validated execution scope
        ↓
Canonical technical owner resolution
        ↓
Owner Runtime Persistence Identity
        ↓
Owner Persistence Namespace
        ↓
Persistence Scope Guard
        ↓
Resolved Tenant placement
        ↓
Authoritative persistence operation
```

No layer may infer missing owner or Tenant scope from physical database location alone.

### 5.2 Owner Persistence Namespace

When multiple canonical owners share one relational database platform, each owner shall have a separate technical namespace boundary for its authoritative persistence.

```text
Shared Relational Database Platform
│
├── Owner A namespace
│   └── Owner A authoritative persistence
│
├── Owner B namespace
│   └── Owner B authoritative persistence
│
└── Owner C namespace
    └── Owner C authoritative persistence
```

The namespace may be implemented by the selected relational technology's approved isolation mechanism after ADR/product selection. This Draft uses **schema/namespace** as an architecture concept and does not prescribe vendor-specific DDL.

An owner may have multiple namespaces or dedicated databases where later architecture justifies stronger isolation, scale, recovery or migration independence. Additional namespaces do not create additional business ownership.

### 5.3 Owner-Scoped Runtime Persistence Identity

Each Owner Persistence Boundary shall have a normal runtime persistence identity whose privileges are constrained to the persistence resources owned by that boundary.

```text
Owner A runtime
      ↓
Owner A persistence identity
      ↓
Owner A namespace only

Owner B runtime
      ↓
Owner B persistence identity
      ↓
Owner B namespace only
```

A normal runtime identity shall not receive broad platform-level privileges merely because several owners share the same database process or server.

The architecture shall prevent:

- Owner A runtime identity mutating Owner B authoritative objects;
- Owner A runtime identity using Owner B objects as an integration/read surface;
- one shared broad runtime credential being used as the normal access identity for unrelated canonical owners;
- a selectively isolated execution unit acquiring broader database authority than the canonical owner whose capability it realizes.

Exact credential issuance, rotation and secret protection are delegated to D03.

### 5.4 Hybrid Tenant Persistence Isolation

Inside an Owner Persistence Boundary, the default Tenant model is shared owner-scoped persistence with mandatory Tenant isolation.

```text
Owner Persistence Namespace
        ↓
Tenant T1 authoritative records
Tenant T2 authoritative records
Tenant T3 authoritative records
        ↓
Every protected access constrained by validated Tenant scope
```

This is a **logical model**, not a mandated table design. Implementation shall represent Tenant ownership strongly enough that an operation scoped to Tenant `T1` cannot read or mutate Tenant `T2` records through a valid normal runtime path.

A stronger isolation tier may be used where justified:

```text
Default
Owner-scoped shared namespace + mandatory Tenant scope

Escalated
Owner + Tenant dedicated namespace

Strongest governed placement
Owner + Tenant dedicated database boundary
```

Promotion to a stronger tier shall be based on an authoritative technical requirement such as:

- stronger security/trust isolation;
- sensitive-data handling obligations;
- contractual/regulatory isolation;
- materially different scale/performance characteristics;
- independent failure/recovery requirements;
- deployment or operational isolation requirements.

Tenant count alone does not require database-per-Tenant architecture.

### 5.5 Persistence Scope Guard

Before a Tenant-scoped persistence operation reaches authoritative data, the persistence path shall prove:

- exactly one canonical owner for the target authoritative data;
- one validated Tenant root from A04;
- applicable Institution scope, where required by the operation;
- consistency between the requested resource and validated scope;
- a valid owner/Tenant placement target.

The following shall fail closed:

```text
owner missing
Tenant missing
Tenant ambiguous
Institution belongs to another Tenant
owner/namespace mismatch
placement unresolved
scope widened after validation
caller-supplied scope contradicts validated scope
```

The persistence boundary shall not repair invalid scope by selecting a default Tenant, default schema, first matching database or administrator-wide connection.

### 5.6 Persistence Placement Resolution

Because B02-001 permits hybrid physical topology and this specification permits hybrid Tenant placement, a protected persistence operation requires a deterministic mapping from:

```text
canonical owner
+
validated Tenant / applicable Institution scope
+
approved isolation tier
        ↓
Owner Persistence Boundary
        ↓
logical namespace / dedicated database target
```

The mapping is an L5 architecture responsibility, not an invitation to hard-code topology in business logic.

Placement configuration shall be governed by the appropriate configuration/deployment architecture. B02-002 requires only that placement resolution be deterministic, owner-preserving and fail closed when missing or inconsistent.

### 5.7 Cross-Owner Data Consumption

Another owner shall not query an authoritative namespace directly, even for read-only integration convenience.

```text
Owner A requires Owner B fact
        ↓
NOT
Owner A DB identity → Owner B namespace
        ↓
USE
C01 governed contract
or C02 governed event
or approved derived projection/read model
```

A consumer-owned derived projection may exist under the canonical projection/reporting architecture, but it remains non-authoritative and shall not become a backdoor write path into the source owner.

### 5.8 Privileged Maintenance and Migration Access

Privileged maintenance, operational or schema-migration access may require broader authority than normal runtime access. Such identities shall therefore be technically distinct from normal runtime persistence identities.

They shall:

- be used only for governed administrative/migration purposes;
- not become the application's normal business persistence path;
- preserve owner and Tenant boundaries during execution;
- be subject to D02/D03 privileged-access, secrets and trust controls;
- produce the evidence required by D04 where authoritative governance requires it.

B02-002 does not define human roles, approval workflow, credential technology or migration procedure.

### 5.9 Physical Co-Location Invariant

The following shall remain equivalent from an ownership perspective:

```text
Owner A namespace + Owner B namespace
on one physical database platform
```

and

```text
Owner A database + Owner B database
on separate physical database platforms
```

In both cases:

- Owner A owns only Owner A authoritative persistence;
- Owner B owns only Owner B authoritative persistence;
- cross-owner direct access remains prohibited;
- Tenant/Institution scope remains mandatory;
- deployment placement does not redefine business ownership.

### 5.10 Material Failure Boundaries

| Failure Class | Required Outcome | Forbidden Outcome | Collaborating Owner |
|---|---|---|---|
| Owner identity cannot be resolved | Reject authoritative persistence access. | Guessing a namespace from runtime/module location. | A03 / B02-002 |
| Runtime persistence identity has excessive privilege | Treat as architecture/security non-conformance; prevent use until corrected. | Relying only on application convention. | D02/D03 / B02-002 |
| Tenant scope missing/conflicting | Fail closed. | Unscoped query or default-Tenant fallback. | A04-002 |
| Owner/Tenant placement missing | Fail closed and surface configuration/operational fault. | Querying another namespace/database. | A06/E04 as applicable |
| Cross-owner direct query/write attempted | Reject the access path. | Temporary bypass because the target is co-located. | B02-002 / A03 |
| Dedicated Tenant placement unavailable | Apply governed resilience/recovery; do not redirect to another Tenant/owner store. | Authority transfer or cross-Tenant failover. | E01/E03/E04 |
| Privileged identity invoked as business runtime | Reject/non-conformant. | Treating administrator access as normal data access. | D02/D03 |
| Transaction/concurrency fault | Delegate to B02-003. | Inventing alternative consistency behavior here. | B02-003 |
| Schema/migration fault | Delegate to B02-004. | Ad-hoc cross-owner structural repair. | B02-004 |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Status | Basis | Consequence | ADR |
|---|---|---|---|---|---|
| `AD-001` | Tenant persistence isolation is hybrid: shared owner-scoped persistence with mandatory Tenant isolation is the default; dedicated owner+Tenant namespace/database placement is permitted when justified by material constraints. | **Provisional Draft direction** | A04 physical-separation boundary; B02-001 hybrid topology; Lead Architect selection 2026-09-11. | Supports scale without preventing stronger isolation for high-risk/high-demand Tenants. | **Required — persistence-isolation ADR scope review; see OD-001.** |
| `AD-002` | Every Owner Persistence Boundary uses an owner-scoped normal runtime persistence identity rather than one unrestricted shared application credential. | **Provisional Draft direction** | L1 ownership; B02-001 no cross-owner access; Lead Architect selection 2026-09-11. | Makes persistence ownership enforceable at the database privilege boundary. | **Required as part of the persistence-isolation strategy; see OD-001.** |
| `AD-003` | When owners share one relational database platform, authoritative persistence is separated by owner-scoped schema/namespace boundaries. | **Provisional Draft direction** | B02-001 hybrid topology; L1 single-owner rule; Lead Architect selection 2026-09-11. | Reduces accidental coupling and permits later physical separation without ownership change. | **Required as part of the persistence-isolation strategy; see OD-001.** |
| `AD-004` | Normal cross-owner direct database reads and writes are prohibited; governed contracts/events/projections are the collaboration path. | Established / technical realization of locked ownership | L1 §§2.1/2.4; B02-001; A03-001. | Shared physical infrastructure cannot become an integration shortcut. | No separate ADR unless a future exception changes the strategy. |
| `AD-005` | Privileged maintenance/migration identities are distinct from normal runtime persistence identities. | Defensible L5 technical derivation | HF-07/HF-12; owner-scoped runtime identity requirement; D02/D03 responsibility. | Prevents broad administrative privilege from becoming an alternate business write path. | Included in isolation ADR scope if materially governed; no independent ADR assumed. |

No database product, database-native row-security feature, vendor-specific privilege mechanism or schema naming convention is selected by this specification.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owning / Collaborating L5 Spec |
|---|---|---|
| Tenant isolation | Every protected persistence operation consumes validated Tenant/Institution scope; stronger Tenant placement is available where justified. | A04-001/002, B02-002 |
| Least privilege | Runtime database authority is owner-scoped; broad shared runtime credentials are non-conformant. | B02-002, D02/D03 |
| Sensitive data | B05/D03 may require stronger owner/Tenant physical isolation; B02-002 provides the placement mechanism. | B05, D03 |
| Secure engines | A07 execution isolation does not grant cross-owner persistence authority; any dedicated database placement preserves canonical ownership. | A07, B02-002 |
| Performance/capacity | Dedicated placement may isolate materially different workloads without forcing database-per-owner universally. | E02, E04 |
| Recovery | Backup/restore/failover must preserve owner/Tenant placement mappings and never merge canonical authority. | E03 |
| Maintainability | Owner namespaces and identities make ownership explicit and allow controlled movement to dedicated databases. | B02-002, B02-004, E04 |
| Testability | Isolation can be verified by negative cross-owner/cross-Tenant access tests without depending on UI behavior. | F02 |

---

## 8. Architecture Conformance Requirements

| CR ID | Conformance Requirement | Evidence Expected |
|---|---|---|
| `CR-001` | Every authoritative persistence resource shall map to exactly one canonical owner and one Owner Persistence Boundary. | Persistence ownership map. |
| `CR-002` | A normal runtime persistence identity shall have data access only to the authoritative persistence resources owned by its boundary, plus explicitly governed technical metadata required for that boundary. | Database privilege/identity design evidence. |
| `CR-003` | One broad shared runtime credential shall not be used as the normal unrestricted persistence identity for unrelated canonical owners. | Credential-to-owner mapping. |
| `CR-004` | When multiple owners share a relational database platform, authoritative objects shall be separated by owner-scoped schema/namespace boundaries or a stronger approved isolation mechanism. | Logical database namespace design. |
| `CR-005` | Protected Tenant-scoped persistence access shall consume validated A04 Tenant/Institution scope and reject missing, conflicting, widened or substituted scope. | Persistence-scope enforcement evidence. |
| `CR-006` | The default shared Tenant storage model shall ensure an operation scoped to one Tenant cannot access another Tenant through a valid normal runtime path. | Tenant-isolation verification. |
| `CR-007` | Dedicated owner+Tenant namespace/database placement shall preserve the same canonical owner and shall not create new business ownership. | Placement/ownership mapping. |
| `CR-008` | Cross-owner direct database reads and writes shall be prohibited as an integration mechanism, including when owners share one database server/process. | Negative access tests / privilege review. |
| `CR-009` | Cross-owner data consumption shall use governed contracts/events or approved derived projections/read models rather than peer authoritative persistence. | Dependency/data-flow review. |
| `CR-010` | Persistence placement resolution shall be deterministic for owner + validated scope and shall fail closed when no valid mapping exists. | Placement resolution architecture/configuration evidence. |
| `CR-011` | Privileged maintenance/migration persistence identities shall remain distinct from normal business runtime identities and shall not be usable as the default application data path. | Privileged identity/access design. |
| `CR-012` | Physical co-location shall not weaken owner, Tenant or privilege isolation requirements. | Deployment/persistence boundary review. |
| `CR-013` | B02-002 shall not define transaction/concurrency semantics or schema-evolution mechanics owned by B02-003/B02-004. | Cross-specification ownership review. |
| `CR-014` | No vendor-specific database isolation feature shall be treated as canonical until applicable ADR/technology selection is approved. | ADR/technical-standard reference. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Owner | Blocking Status | Resolution Path |
|---|---|---|---|---|
| `OD-001` | Formalize the persistence-isolation strategy covering hybrid Tenant placement, owner-scoped runtime identities and owner-scoped namespace boundaries; determine whether it is consolidated with the B02-001 persistence/topology ADR or separated by governed scope. | L5-B-02 / Lead Architect | **Blocks Architecture Approved for the provisional decisions in this Draft.** | Perform ADR necessity/scope review, avoid duplicate ADR ownership, allocate canonical ADR ID on `main`, evaluate alternatives and approve/revise. |
| `OD-002` | Establish exact criteria and governed placement rules for when a Tenant/owner moves from shared placement to dedicated namespace/database. | B02-002 with B05/D03/E02/E03/E04 | Does not block Draft authoring; required before operational implementation. | Consume approved security/privacy/performance/recovery/deployment constraints; do not invent numeric thresholds here. |
| `OD-003` | Determine the concrete database-product mechanism for namespaces, runtime identities and Tenant isolation after the primary database technology is selected. | B02/A02 via Repository ADR | Does not block product-neutral Draft architecture; blocks implementation standardization. | Resolve primary database technology ADR first, then specialize this architecture without weakening CRs. |

No canonical L5 ADR number is allocated by this specification.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-003/005/009` — Tenant environment/isolation and Active Tenant context must persist through downstream technical boundaries without ownership transfer.
- `L4-A-015-BH-001/004/005/011` — workflow remains Tenant-scoped, may not directly mutate participating-domain state, and consumes authoritative owner results rather than persistence internals.
- `L4-A-019-BH-001/002/005/007/010` — committed source facts remain domain-owned, Tenant-scoped publication/routing is preserved, and consumer delivery does not transfer ownership.

### Higher-Level / Peer Verification

- SutraERP L1 Business Domain Architecture v2.1 §§2.1/2.4 — singular canonical ownership and prohibition on direct cross-domain database access.
- `L5-B-02-001` — relational-first hybrid persistence baseline and distinction between logical owner boundaries and physical databases.
- `L5-A-04-001/002` — Tenant root isolation and mandatory propagation/enforcement of validated scope at persistence boundaries.
- `L5-A-03-001` — module/runtime co-location shall not grant persistence access across canonical ownership boundaries.

### Resulting Isolation Model

```text
Canonical Owner
      ↓
Owner Runtime Persistence Identity
      ↓
Owner Persistence Namespace
      ↓
Validated Tenant / Institution Scope Guard
      ↓
Shared Tenant Placement
      OR
Dedicated Owner+Tenant Namespace
      OR
Dedicated Owner+Tenant Database
```

The specification remains `Draft` while `OD-001` is unresolved and shall not be treated as an approved persistence-isolation standard until the required ADR governance is complete.