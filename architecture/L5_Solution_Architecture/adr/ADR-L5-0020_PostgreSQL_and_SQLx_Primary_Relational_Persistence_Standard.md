# ADR-L5-0020 — PostgreSQL & SQLx Primary Relational Persistence Standard

**ADR ID:** `ADR-L5-0020`  
**Title:** PostgreSQL & SQLx Primary Relational Persistence Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-B-02 — Database & Persistence Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-B-02-001`…`L5-B-02-004`  
**Related ADRs:** `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0007`; `ADR-L5-0008`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
ACID maturity, relational integrity, Rust support, portability, migration tooling, operations and long-term maintainability.

## 5. Alternatives Considered
PostgreSQL+SQLx; PostgreSQL+mandatory ORM; MySQL/MariaDB; remain product-neutral.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**PostgreSQL 18.x + SQLx are the canonical primary relational persistence stack.**

PostgreSQL 18.x, on a repository-approved supported minor, is the canonical primary relational database. Rust persistence uses SQLx by default; no repository-wide full ORM is mandated. Logical owner boundaries, no direct cross-owner DB collaboration and no fixed physical DB count remain unchanged.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-B-02-001`…`L5-B-02-004`. Related authority: `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0007`; `ADR-L5-0008`.  
External evidence: https://www.postgresql.org/docs/18/ ; https://docs.rs/sqlx/latest/sqlx/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
