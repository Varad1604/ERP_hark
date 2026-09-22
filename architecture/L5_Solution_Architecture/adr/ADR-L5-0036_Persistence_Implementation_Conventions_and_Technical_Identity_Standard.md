# ADR-L5-0036 — Persistence Implementation Conventions & Technical Identity Standard

**ADR ID:** `ADR-L5-0036`  
**Title:** Persistence Implementation Conventions & Technical Identity Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-B-02 — Database & Persistence Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-B-02-001`; `L5-B-02-002`; `L5-B-02-003`; `L5-B-02-004`; constrains `L5-B-06-001/002`  
**Related ADRs:** `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0008`; `ADR-L5-0020`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

PostgreSQL 18.x and SQLx are already canonical, while B02 already locks owner-scoped logical persistence, hybrid physical placement, owner-local transactions and controlled schema evolution. Backend preparation still requires deterministic conventions for technical identifiers, time representation, default co-located persistence realization, deletion semantics, connection pooling and PostgreSQL extension governance.

## 2. Scope & Ownership

This ADR governs persistence implementation conventions only. It does not rename aggregates, invent entity ownership, create business identifiers, change lifecycle/retention rules, or require one physical database per business domain.

## 3. Decision

1. **Owner-scoped logical persistence:** when multiple canonical owners are co-located in one PostgreSQL database, the default realization is owner-scoped PostgreSQL schema/role boundaries or an equivalently enforceable logical namespace. Physical database separation remains selective and justified by isolation, scale, recovery or operational requirements under ADR-L5-0005.
2. **No cross-owner database collaboration:** normal runtime code shall not directly read or write another canonical owner's authoritative tables/schema. Cross-owner collaboration uses owner contracts, events or governed B06 projections/read models.
3. **Technical identifiers:** new implementation-level persistent identifiers use **UUID version 7** by default and PostgreSQL's native `uuid` type. UUIDv7 is a technical identifier only; source-owned business identifiers remain separate.
4. **Time:** authoritative technical instants use timezone-aware timestamps and are persisted as PostgreSQL `timestamptz` with service/session operation normalized to UTC. Business-local dates, cut-off times, academic dates and timezone-dependent semantics preserve the explicit applicable timezone/business-date context rather than being inferred from UTC alone.
5. **Deletion:** there is **no universal soft-delete convention**. Archive, deactivate, withdraw, terminate, delete, retain and purge semantics come from the owning lifecycle/retention architecture.
6. **Schema migration:** production schema evolution uses version-controlled explicit migrations following ADR-L5-0008 Expand → Migrate/Backfill → Transition → Contract. Automatic ORM/schema synchronization is prohibited as a production migration mechanism.
7. **Connection pooling:** Rust persistence uses bounded SQLx connection pools as the application baseline. **PgBouncer** may be introduced by deployment/capacity profile where connection pressure or topology justifies it; it is not mandatory for every deployment.
8. **Extensions:** PostgreSQL extensions are deny-by-default for repository standardization. An extension may be enabled only after review of portability, security, upgrade, backup/recovery and operational consequences.
9. **Read-heavy cross-domain queries:** dashboards, reporting and cross-domain query views use governed B06 read models/projections rather than cross-owner joins against authoritative tables.

## 4. Conformance Rules

- A physical PostgreSQL schema name does not create or rename a business domain.
- UUIDv7 generation may occur in PostgreSQL 18 or in an approved application library, but storage uses the PostgreSQL `uuid` type.
- UUIDv7 ordering shall not be treated as an authoritative business timestamp.
- UTC normalization shall not erase source business timezone where the rule depends on institutional local time.
- Migration/maintenance identities remain separately governed from normal runtime identities.
- Direct DBA/support access does not become an application integration mechanism.
- Cross-owner projections remain non-authoritative and rebuildable.

## 5. Consequences

Implementation teams receive one consistent persistence baseline without collapsing canonical owners into a shared-table model.

## 6. External Evidence

- PostgreSQL 18 UUID type: https://www.postgresql.org/docs/18/datatype-uuid.html
- PostgreSQL 18 UUID functions: https://www.postgresql.org/docs/18/functions-uuid.html
- PgBouncer: https://www.pgbouncer.org/

## 7. Approval Record

**Decision selected:** owner-scoped PostgreSQL logical boundaries; UUIDv7 technical IDs; UTC timezone-aware technical instants with explicit business timezone context; no universal soft delete; no normal cross-owner SQL; B06 projections for cross-owner reads; explicit migrations; SQLx pools with profile-driven PgBouncer; approved extensions only.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
