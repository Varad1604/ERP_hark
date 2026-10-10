# P1-005 — A01/B01 Column-Level Persistence Blueprint (REVIEW DRAFT)

**Date:** 2026-10-10 | **Status: DESIGN CANDIDATES — NOT APPROVED DDL/MIGRATIONS**

## Scope and source authority
The user's requested design depth is **table and column level**. This document proposes exact *candidate* table/column names and PostgreSQL types for a bounded Platform Tenant + Institution read slice, **without implementing SQL**. These identifiers are NOT canonical approved names.

- L2-A01 Platform Tenant Foundation: Tenant is a platform (not an Institution), root isolation, lifecycle and Context Resolution owner; inactive Tenants fail normal protected context.
- L2-B01 Institution Workflow + locked activation amendment: B01 owns Institution identity/lifecycle, Profile and versioned Policy; one Institution has one Tenant, Code unique within Tenant, Archived is read-only/terminal, Suspended retains history.
- L4-B-001/002/003, approved: activation/suspension/archival history and governed outcomes; Archived preserves identity/history, prevents identifier reuse.
- **Locked L3-B-004 already establishes** preservation of prior approved Institution Profile history, unchanged Institution identity and effective-authority behavior. The L4-B-004 file currently has `Status: Review`, so its additional behavior/approval status must be reconciled before implementing profile mutation; **do not reopen or reapprove established L2/L3 business rules**.
- ADR-L5-0020 and ADR-L5-0036: PostgreSQL 18.x, SQLx, owner-scoped namespaces/roles; UUIDv7 `uuid` technical IDs; UTC `timestamptz` technical instants; no global soft delete; explicit migrations; owner- and Tenant-scope restrictions.
- L5-B-02-002/003/004: no direct cross-owner SQL (even FK enforcement strategy must not create runtime cross-owner access), optimistic owner-local concurrency, historical/transaction correctness, migrations through separate privileged identity.
- L5-B-05-003: retention/hold decisions and purge are externally governed, not a universal deleted column.

## Physical grouping proposals (names illustrative)

**PostgreSQL namespaces**: `platform_tenant` for A01 and `institution` for B01. These names are **candidate technical namespaces**, not new business domains. Runtime identities are separate, with least-privilege access. Do not grant B01 runtime SQL read of A01 tables simply because both live in one database.

### Table A — `platform_tenant.tenants` (A01 owner; provisioning/lifecycle storage proposal)

| Candidate column | SQL type | Null | Proposed rule / purpose |
|---|---|---|---|
| `id` | `uuid` | NO | Primary key, new technical UUIDv7 |
| `tenant_reference` | `text` | YES* | Optional business identifier/namespace if A01 confirms its official format and uniqueness |
| `lifecycle_state` | `text` | NO | Current A01 source state, e.g. Requested, Provisioning, Provisioning Failed, Active, Maintenance, Suspended, Upgrade, Archived; exact storage encoding to be approved |
| `row_version` | `bigint` | NO | Positive optimistic-concurrency version; internal to A01 |
| `created_at` | `timestamptz` | NO | Technical creation instant |
| `updated_at` | `timestamptz` | NO | Technical latest-change instant |

**Candidate constraints:** `PRIMARY KEY(id)`, `CHECK(row_version > 0)`, uniqueness on `tenant_reference` **only after A01 defines its lifecycle and nullability**. Do not add a fictional Tenant name, owner, `institution_id`, default Tenant fallback, or fixed numeric retention. A01 provisioning may require more tables/columns; this is a **minimal slice subset**, not a full Tenant schema.

### Table B — `institution.institutions` (B01 owner; canonical identity + current state)

| Candidate column | SQL type | Null | Proposed rule / purpose |
|---|---|---|---|
| `id` | `uuid` | NO | Candidate technical UUIDv7 primary key. The **immutable canonical B01 Institution business UUID already exists as a locked business identity**. Its lossless persistence mapping (same field or separate unique column) must be demonstrated against ADR-L5-0036 before DDL; do not treat business identity itself as undecided |
| `tenant_id` | `uuid` | NO | Immutable **A01-issued reference**, not B01-owned Tenant; scope filter and authoritative mapping resolution through A01 contract |
| `institution_code` | `text` | NO | Immutable source-owned Institution Code; uniqueness within Tenant |
| `lifecycle_state` | `text` | NO | Source-defined current B01 state; cannot be mutated by a read; enforce valid source transitions in B01 application/domain layer |
| `row_version` | `bigint` | NO | Internal version for optimistic concurrency |
| `created_at` | `timestamptz` | NO | Technical creation instant |
| `updated_at` | `timestamptz` | NO | Technical current record change instant |

**Candidate constraints:** `PRIMARY KEY(id)`, `UNIQUE(tenant_id, institution_code)`, `CHECK(row_version > 0)`; `tenant_id` mandatory; B01-owned state transition validation; avoid global `UNIQUE(institution_code)`. **Do not place a database-level cross-schema FOREIGN KEY to A01 by default**: consider reference validation under governed A01 contract plus B01 scope assurance, and decide migrations/reconciliation mechanics with B02. This is an *open design choice*, not an architectural statement that foreign keys are universally forbidden.

**Important:** row-version `bigint` and physical names are proposals. **No new Institution identity-model decision is needed.** The only pending issue is whether the B01 business UUID can legitimately serve as the SQL primary key under ADR-L5-0036 or needs an explicit separate technical identifier; preserve canonical business identity either way.

### Table C — `institution.institution_profile_versions` (B01 owner; separate profile and effective history candidate)

| Candidate column | SQL type | Null | Proposed rule / purpose |
|---|---|---|---|
| `id` | `uuid` | NO | Technical UUIDv7 for the profile version record |
| `institution_id` | `uuid` | NO | B01-local FK to `institution.institutions(id)` |
| `version_number` | `integer` | NO | Positive per-Institution version; **technical candidate**, not source business policy version |
| `institution_name` | `text` | NO* | B01 §7 Identity; requiredness across incomplete Draft vs configured state needs review |
| `short_name` | `text` | YES | B01 §7 Identity; may be mandatory before Configured |
| `institution_type` | `text` | YES | B01 §7 Identity; approved code system not selected |
| `effective_from` | `timestamptz` | YES | Candidate effective instant; valid only if B01/L3/L4 authorize precise semantics |
| `effective_until` | `timestamptz` | YES | Candidate expiry of version; mutually exclusive current versions per effective context must be governed |
| `created_at` | `timestamptz` | NO | Technical version record creation |
| `decision_reference` | `uuid` | YES | Candidate link to governed profile decision if canonical reference shape supports UUID; **may be a different type** |

**Candidate constraints:** `PRIMARY KEY(id)`, `FOREIGN KEY(institution_id) REFERENCES institution.institutions(id)`, `UNIQUE(institution_id, version_number)`. The locked L3-B-004 already requires approved current-profile authority and preserved predecessor history. Physical enforcement remains to be designed: Draft completeness; future effective dating; no-overlap; pending/returned/rejected proposal isolation; atomic effective selection; decision evidence. **L4-B-004 metadata is Review**; this exploratory table is **outside the first identity/code/status read** and shall not be migrated just to support that read.

### Table D — `institution.institution_lifecycle_history` (B01 owner; durable prior/status transition record candidate)

| Candidate column | SQL type | Null | Proposed rule / purpose |
|---|---|---|---|
| `id` | `uuid` | NO | Technical UUIDv7 for transition/evidence record |
| `institution_id` | `uuid` | NO | B01-local FK to canonical Institution |
| `from_state` | `text` | YES | Previous B01 state, null only for source-authorized establishment |
| `to_state` | `text` | NO | Resulting source-authorized B01 state |
| `effective_at` | `timestamptz` | NO* | When governed state change becomes effective; if scheduled/future effective, separate proposal/effect design required |
| `recorded_at` | `timestamptz` | NO | When system records event/evidence; not necessarily identical to effective time |
| `decision_reference` | `uuid` | YES* | Authoritative approval/evidence reference type and mandatory cases to confirm |
| `sequence_number` | `bigint` | NO | Positive per-Institution sequence proposed for ordering/provenance |

**Candidate constraints:** `PRIMARY KEY(id)`, `FOREIGN KEY(institution_id) REFERENCES institution.institutions(id)`, `UNIQUE(institution_id, sequence_number)`; permitted transitions/authority from locked L2/L3/L4 not a generic `UPDATE lifecycle_state`; restrict deletion. Do not treat this journal as the D04 cross-platform technical audit store or the C02 broker outbox. Separate business history from distributed technical audit and event delivery.

### Table E — `institution.institution_policy_versions` (**DEFERRED**, B01 owner)

**No proposed columns yet.** B01 §2.1 explicitly owns distinct **versioned Institution Policy**, and policy lifecycle/historical rules cannot be collapsed into Profile or Authorization Policy. Not required to satisfy first protected Institution summary. A future separate owner-backed policy/approval model needs policy content, applicability, version, effective validity, issuer/decision and retention/compatibility design. Do not create a universal JSON policy blob or invent fields now.

### Tables F/G — authentication, authorization, durable publication intent

**Not part of B01 schema:** A02/D01 account/session/credentials, D02 permission/policy/role assignment, A04 execution context machinery and D04 audit are independently owned. Any eventual B01 state mutation requiring public business event must follow B02-003 owner-local **durable publication intent/outbox** and released C02 schema. No such outbox or event table is necessary to implement a truthful B01-only read. Plan cross-owner references/contracts rather than unrestricted B01 SQL joins.

## Relationship and scope sketch

```text
A01 Platform Tenant [owner A01, separate PostgreSQL role/schema]
    tenant UUID (authoritative reference)
        |
        | A01-governed contract/reference, NOT unrestricted cross-owner SQL
        v
B01 Institution [owner B01]
    institutions.id (UUID)
      | 1:n B01-local referential integrity
      +-- institution_profile_versions (current/effective historical concept, proposed)
      +-- institution_lifecycle_history (source-derived business lifecycle evidence, proposed)
      +-- institution_policy_versions (separate B01 aggregate, deferred)
```

**Runtime isolation:** B01 validates requested Tenant/Institution through A04/D02 and A01-sourced association, and restricts all reads to allowed scope. B01's cached authoritative association does not create ownership of Tenant. No cross-Tenant unscoped read, enumeration or inferred Tenant default.

## Open physical design decisions / blocking checks

| ID | Decision/question | Gate |
|---|---|---|
| DB-01 | **Implementation mapping only:** preserve the already-locked immutable B01 business UUID; establish ADR-L5-0036-conformant technical PK / unique business-UUID storage mapping | Before B01 DDL |
| DB-02 | A01 tenant business reference format, code/namespace and required provisioning columns | Before Tenant migration |
| DB-03 | Explicit schema and SQL runtime role names, strict owner-local database privileges | Before any shared PostgreSQL runtime |
| DB-04 | Tenant/Institution A04 isolation enforcement: RLS or equivalent, transaction binding, pool reset and negative tests | Before protected persistence |
| DB-05 | Source-approved lifecycle state literal mapping, allowed transitions, future effective approval evidence | Before B01 mutations |
| DB-06 | **Deferred from first read:** physical profile-version representation, approved-current selection, draft nullability, effective overlap and proposal separation; reconcile L4-B-004 `Review` without reopening locked L3-B-004 history obligations | Before Profile DDL |
| DB-07 | B01 policy lifecycle/versions and owner event evidence; separate from D02 permissions | Future policy slice |
| DB-08 | ID-reference contract across A01/B01, loss-of-upstream source/reconciliation | Before read integration |
| DB-09 | Archived history, privacy classification, legal hold, retention + recovery; inactive-Tenant exceptional access | Before historical reporting |
| DB-10 | Exact target physical indexing/query/exposure patterns and status freshness semantics | Before performance/sign-off |
| DB-11 | Migration identity/order/expand-contract strategy and rollback/restore rehearsals | Before production DDL |
| DB-12 | Test Tenant/Institution records: source-valid setup vs synthetic historically impossible `Active` rows | Before authenticity claims |

## Recommended testing from this blueprint (NOT executed)

- Unique Institution Code is scoped by Tenant, not globally.
- One Institution cannot silently change Tenant or be referenced under a different Tenant.
- B01 runtime cannot SELECT/UPDATE A01 authoritative tables.
- B01 role cannot SELECT another owner namespace or bypass A04 Tenant scope.
- Every record update that requires concurrency checking detects stale `row_version`.
- Archive retains identity and history; does not reuse codes/UUIDs or delete dependent business history.
- Profile versions are immutable once effective except source-governed successor/revision mechanism; no simultaneous overlapping authoritative effective profile.
- Technical UTC instants are distinguishable from institutional local business dates.
- Migration credentials are separate from runtime privileges.
- No frontend or mobile client can supply a fake `lifecycle_state` to the read endpoint.

**Integrity:** Column types/names/constraints are **proposed candidate DDL semantics**, not validated SQL; no tables exist, no queries, no migrations, no tests and no production authorization. Broader A–J database entity inventory is still incomplete, and these tables are not the full SutraERP physical model.
