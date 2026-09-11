# Database & Persistence Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-B-02-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-02 — Database & Persistence Architecture  
**Canonical Technical Responsibility:** Physical persistence architecture and principles without changing domain ownership.  
**Technical Architecture Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Architecture Family Register; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Broad Authoring Authorization v1.0.0; L5 Dependency-Gated Parallel Authoring Execution Timeline v1.0.0; `L5-A-01-001` v1.0.1 Draft; `L5-A-03-001` v1.0.1 Draft; `L5-A-04-001` v1.0.1 Draft; `L5-A-04-002` v1.0.1 Draft; `L4-A-001` v1.0.0 Draft; `L4-A-019` v1.0.0 Draft; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** Dependency-gated downstream areas including L5-B-03, L5-C-02, L5-B-05, L5-E-02, L5-A-07, L5-B-01, L5-B-04, L5-B-06 and L5-E-03.  
**Applicable ADRs:** `ADR-L5-0005` — Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern (`Architecture Approved`). Primary relational database technology remains a separate unresolved technology-selection decision and is not selected here.

---

## 2. Purpose, Scope & Non-Goals

This specification establishes the product-neutral persistence baseline for authoritative SutraERP data while preserving canonical business ownership. It defines logical persistence boundaries and their allowed relationship to physical database infrastructure; it does not select a database product or fixed database count.

### In Scope

- Relational-first authoritative persistence baseline.
- Authoritative Persistence Boundary aligned to exactly one canonical owner.
- Hybrid logical/physical persistence topology.
- Shared versus dedicated physical placement principles.
- Derived-store non-authority rules.
- Cross-owner persistence prohibition.

### Out of Scope

- Concrete database product/engine selection.
- Table/column/index schema design.
- Owner/Tenant identity mechanics — B02-002.
- Transaction/concurrency — B02-003.
- Schema evolution/migration — B02-004.
- Backup/restore/DR — E03.
- Runtime/deployment topology — E04.

---

## 3. Authoritative Basis & Constraints

| Source | B02-001 Effect |
|---|---|
| L1 v2.1 | One canonical business owner; direct cross-domain database access is prohibited. |
| `L4-A-019` | Committed business facts/events retain publisher/owner authority across consumer delivery. |
| A04-001/002 | Tenant is root protected scope and validated Tenant/Institution context must reach protected persistence. |
| `ADR-L5-0005` | Canonically approves owner-scoped persistence with hybrid physical placement. |
| ADR methodology | Database product selection requires separate governed technology ADR. |

### Constraints

1. Every authoritative persistence boundary maps to one canonical owner.
2. Shared infrastructure shall not create shared business write authority.
3. Cross-owner direct database access is not a collaboration mechanism.
4. Tenant isolation shall remain enforceable independent of physical placement.
5. Physical database count is not derived from domain/module count.
6. Derived/index/cache/projection stores shall not become hidden authoritative sources.
7. Specialized/isolated computation does not automatically create new canonical persistence ownership.

---

## 4. Technical Ownership & Boundaries

### Owns

- **Authoritative Persistence Boundary** concept.
- Relational-first authoritative persistence position.
- Hybrid logical/physical persistence topology.
- Rule that physical separation is constraint-driven, not business-domain-count driven.
- Derived-store non-authority rule.

### Consumes / Collaborates With

- A03 technical owner/module boundaries.
- A04 Tenant/Institution isolation.
- B02-002 ownership/isolation mechanics.
- B02-003 transaction/consistency.
- B02-004 schema evolution.
- D03 security/trust requirements.
- E02/E03/E04 operational, recovery and deployment constraints.

### Must Not Own

Business ownership/rules/lifecycles/events, database-product choice, exact physical schema, transaction mechanics, migration mechanics, recovery or deployment topology.

---

## 5. Solution Architecture

### 5.1 Authoritative Persistence Boundary

Each canonical owner that persists authoritative state uses one logical authoritative persistence boundary. The boundary defines write authority and ownership; it is not synonymous with one physical database instance.

```text
Canonical owner
      ↓
Authoritative Persistence Boundary
      ↓
owner-scoped logical namespace / access boundary
      ↓
physical relational placement selected under approved constraints
```

### 5.2 Relational-First Baseline

Authoritative transactional ERP state is relational-first because the corpus requires integrity, consistency, ownership traceability and governed transactional mutation. This is an architecture class, not a vendor selection.

The repository may later select one primary relational database technology through governed technology evaluation/ADR. Until then, no vendor is canonical.

### 5.3 Hybrid Physical Topology

Owner persistence boundaries may share physical relational infrastructure when all logical ownership, Tenant isolation, security, recovery and operational constraints remain enforceable.

Physical separation may be introduced where justified by material constraints such as:

- security/privacy/trust;
- specialized engine isolation;
- workload/performance/capacity;
- resilience/recovery;
- compliance;
- deployment/operational independence.

No canonical fixed physical database count is defined.

### 5.4 Cross-Owner Access

A consumer shall not bypass the canonical owner through direct database read/write access. Cross-owner collaboration uses governed contracts, events, workflows or approved read-model/projection boundaries. Co-location in one database technology or server does not alter this rule.

### 5.5 Derived Stores

Caches, indexes, search structures, projections, analytics stores or similar derived persistence may exist under their canonical owners, but they shall not silently become authoritative write sources for business facts owned elsewhere.

### 5.6 Failure Model

| Failure | Required Outcome |
|---|---|
| Owner boundary cannot be resolved | Fail before authoritative mutation. |
| Attempted cross-owner direct persistence | Reject. |
| Tenant scope missing/invalid | Fail closed under A04/B02-002. |
| Shared physical placement cannot preserve required isolation | Escalate placement/isolation; do not weaken logical boundaries. |
| Derived store diverges from source authority | Treat canonical owner as authoritative; recover derived state through its owning architecture. |

---

## 6. Architecture Decisions

| Decision ID | Decision | Status | ADR |
|---|---|---|---|
| `AD-001` | Authoritative ERP persistence is relational-first and aligned to canonical owner boundaries. | Architecture Approved for pattern; database product remains unresolved | `ADR-L5-0005` |
| `AD-002` | Logical owner persistence boundaries may share physical database infrastructure when constraints permit; physical separation is selectively justified. | Architecture Approved | `ADR-L5-0005` |
| `AD-003` | No fixed physical database count is canonical. | Architecture Approved | `ADR-L5-0005` |
| `AD-004` | Primary relational database technology selection is deferred to separate Repository ADR/technology evaluation. | Unresolved technology selection; non-blocking for product-neutral Draft | Future governed ADR |

---

## 7. Security, Privacy & Operational Impact

- Shared physical placement requires enforceable owner and Tenant isolation.
- Physical separation may strengthen security/resilience but does not create business ownership.
- Database product and vendor-specific isolation features remain non-canonical until selected through ADR governance.
- No numeric performance/capacity/recovery target is invented here.

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Every authoritative persisted business fact shall resolve to one canonical owner persistence boundary. |
| `CR-002` | Shared physical infrastructure shall not create cross-owner write authority. |
| `CR-003` | Direct cross-owner database access shall not be used as normal collaboration. |
| `CR-004` | Tenant/Institution isolation shall be preserved at protected persistence boundaries. |
| `CR-005` | Physical database count shall not be inferred from business-domain/module count. |
| `CR-006` | Dedicated physical placement shall be justified by approved material constraints. |
| `CR-007` | Derived stores shall not become hidden authoritative sources. |
| `CR-008` | Specialized computation isolation shall not automatically create new canonical business/persistence ownership. |
| `CR-009` | Product/vendor-specific database behavior shall not become canonical without applicable ADR authority. |
| `CR-010` | Transaction and migration mechanics remain delegated to B02-003/B02-004. |

---

## 9. Open Decisions & Deferred Design

`ADR-L5-0005` resolves the former blocking persistence-topology/isolation strategy item.

| ID | Item | Blocking Status | Resolution Path |
|---|---|---|---|
| `OD-002` | Select the primary relational database technology. | Does not block product-neutral Draft; blocks canonical technology standardization/implementation standard. | Define criteria, evaluate viable products and approve a Repository technology ADR; A02 then consolidates the approved technology standard as applicable. |

---

## 10. Traceability Summary

Canonical architecture pattern: `AD-001`–`AD-003` → `ADR-L5-0005`.

Technology choice remains intentionally unresolved: `AD-004` / `OD-002`.

Downstream consumers shall depend on the owner-scoped persistence boundary, not a presumed physical database count or vendor.
