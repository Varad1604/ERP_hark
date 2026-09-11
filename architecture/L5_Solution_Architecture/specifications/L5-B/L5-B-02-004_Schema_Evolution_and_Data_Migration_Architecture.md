# Schema Evolution & Data Migration Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-02-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-02 — Database & Persistence Architecture  
**Canonical Technical Responsibility:** Controlled persistence-schema evolution and internal migration while preserving canonical ownership, isolation and compatibility; excludes operational governed business import/export.  
**Technical Architecture Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-B-02-001` v1.0.0 Draft; `L5-B-02-002` v1.0.0 Draft; `L5-B-02-003` v1.0.0 Draft; applicable `L5-E-03-*` recovery architecture; applicable `L5-E-04-005` release installation/upgrade/rollback architecture.  
**Referenced By:** Release/upgrade architecture, persistence implementation standards, recovery architecture, and owner-specific technical designs that evolve authoritative persistence structures.  
**Applicable ADRs:** Required but unresolved — reusable schema-evolution and migration-governance pattern. No canonical L5 ADR ID is currently allocated for this decision.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define how SutraERP evolves authoritative persistence structures and transforms already-persisted data without breaking supported runtimes, corrupting canonical business state, bypassing owner/Tenant isolation, or creating environment-specific schema histories.

The specification establishes a compatibility-preserving evolution lifecycle, governed migration execution, immutable migration history, and safe recovery boundaries. It does not define business import/export behavior or use schema migration as a mechanism for changing canonical business ownership.

### In Scope

- Persistence-schema evolution within an existing canonical Owner Persistence Boundary.
- Expand → Migrate → Transition → Contract evolution lifecycle.
- Internal persisted-data backfill/transformation required by a schema change.
- Dedicated migration execution authority separate from normal runtime persistence identities.
- Owner-scoped and Tenant-safe migration execution.
- Stable migration identity, sequencing and applied-state recording.
- Immutable applied migration definitions/history.
- Forward corrective migrations when an applied migration requires remediation.
- Compatibility gates between application/runtime versions and persistence versions.
- Destructive-change safeguards and recovery boundaries.
- Migration observability/evidence required to determine whether a governed environment reached the intended schema state.

### Out of Scope / Non-Goals

- Operational business import/export or user-operated bulk data exchange.
- Business onboarding/import semantics, validation, reconciliation or acceptance rules.
- Reassignment of canonical entity/domain ownership.
- Database product or migration-tool selection.
- Complete table/column/index/DDL design.
- General release orchestration, installation sequencing or application rollback — `L5-E-04-005`.
- Backup/restore/PITR/disaster-recovery mechanics — `L5-E-03-*`.
- Transaction/concurrency strategy — `L5-B-02-003`, except where migrations must preserve its invariants.
- Tenant/Institution scope semantics — `L5-A-04-*`.
- Runtime credentials/secrets management — `L5-D-03-*`.
- Human approval roles or operational change-management process.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-B-02-004 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-B-02-004` row | B02-004 owns controlled persistence-schema evolution/internal migration and excludes operational governed business import/export. | Establishes canonical scope. |
| L5 Peer | `L5-B-02-001_Database_and_Persistence_Architecture_Baseline.md` | v1.0.0 / Draft | B02 responsibility split | Authoritative persistence remains owner-aligned; database product and exact DDL remain separate decisions. | Schema evolution cannot transfer business ownership or assume a vendor. |
| L5 Peer | `L5-B-02-002_Domain_Persistence_Ownership_and_Isolation_Architecture.md` | v1.0.0 / Draft | privileged migration identity and owner isolation | Normal runtime identities are owner-scoped; privileged maintenance/migration access is separate and must not become a normal business write path. | Establishes migration authority separation and owner/Tenant isolation. |
| L5 Peer | `L5-B-02-003_Transaction_Consistency_and_Concurrency_Architecture.md` | v1.0.0 / Draft | owner-local transactions, concurrency and publication-intent consistency | Migration must not invalidate transaction/concurrency/state-publication invariants. | Requires compatible structural/data transitions. |
| L5 Governance | `L5_Technology_Selection_and_ADR_Methodology.md` | v1.0.1 / Locked | §1.1, §§2–3 | Reusable cross-specification patterns and material persistence/operational decisions require ADR governance. | Requires a formal ADR for the reusable migration pattern. |
| L5 Corpus | `L5-E-03-001/002/003` | frozen responsibility definitions | backup/restore/PITR/DR responsibilities | Destructive migration recovery shall rely on governed recovery architecture rather than assuming every migration can be reversed. | Establishes recovery boundary. |
| L5 Corpus | `L5-E-04-005` | frozen responsibility definition | controlled installation, upgrades and technical rollback | Release orchestration consumes B02 schema compatibility/migration requirements but remains E04-owned. | Prevents B02 from becoming release-management architecture. |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Source | L5 Effect |
|---|---|---|---|
| `C-001` | Schema evolution shall not transfer canonical business ownership. | L1/L5 governance; B02-001 | Structural migration remains inside the owning persistence boundary. |
| `C-002` | Normal runtime persistence identities shall not become unrestricted schema-administration identities. | B02-002 | Migration uses separate governed migration authority. |
| `C-003` | Shared physical infrastructure shall not allow one owner migration to modify another owner's authoritative namespace. | B02-002 | Migration scope is owner-scoped even on shared database infrastructure. |
| `C-004` | Tenant/Institution isolation must survive migration. | A04; B02-002 | Backfill/transformation shall not mix, widen or substitute Tenant scope. |
| `C-005` | Owner-local transaction/concurrency and publication-handoff invariants must remain valid during transition. | B02-003 | Compatibility windows cannot silently break write/concurrency/event guarantees. |
| `C-006` | Release rollback does not imply schema reversal is always safe. | E04/E03 responsibility split | Irreversible/data-destructive changes require governed recovery or forward correction. |
| `C-007` | Operational business import/export is not B02-004. | Frozen corpus register | Internal migration shall not become a backdoor bulk business-data integration mechanism. |
| `C-008` | Applied migration history shall remain deterministic across governed environments. | Lead Architect decision 2026-09-11 | Applied migration definitions cannot be edited in place. |

### 3.3 Assumptions

No database vendor, schema-migration product, ORM migration framework, deployment orchestrator, SQL dialect, DDL syntax, lock primitive or backup technology is assumed.

The Lead Architect selected the following Draft architecture directions on 2026-09-11:

- potentially breaking structural changes use a compatibility-preserving **Expand → Migrate → Transition → Contract** lifecycle unless a specifically governed exception proves an atomic change safe;
- production schema/data migrations execute through a dedicated governed migration path and identity rather than normal application startup identities;
- once a migration has been applied to a governed environment, its definition/history is immutable and corrections are introduced as new forward migrations.

These decisions establish a reusable cross-specification migration pattern and therefore remain provisional pending ADR governance.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The **Persistence Evolution Lifecycle** for controlled owner-local structural change.
- The **Migration Package** concept: a stable, ordered technical change definition with migration identity, compatibility intent and validation expectations.
- The **Dedicated Migration Execution Boundary** and requirement for non-runtime migration authority.
- The **Migration State Record** identifying which migration definitions are applied to a governed persistence boundary.
- The rule that applied migration definitions are immutable.
- Forward corrective migration strategy for already-applied migrations.
- Internal data backfill/transformation required to move from old persistence representation to new representation.
- Schema/runtime compatibility requirements during transition.
- Destructive-change classification and recovery-precondition requirements.

### Consumes / Collaborates With

| Exact L5 Architecture | Consumed Responsibility | B02-004 Boundary |
|---|---|---|
| `L5-B-02-001` | Persistence platform/boundaries | Evolution applies within approved authoritative persistence architecture. |
| `L5-B-02-002` | Owner/Tenant isolation and migration-identity separation | Migration may alter only the authorized owner persistence scope. |
| `L5-B-02-003` | Transaction/concurrency/state-handoff invariants | Migration preserves operational consistency semantics while old/new structures coexist. |
| `L5-A-04-*` | Tenant/Institution scope semantics | B02-004 preserves scope during data transformation but does not redefine it. |
| `L5-D-02-*` / `L5-D-03-*` | Privileged authorization, credentials/secrets | Controls who/what may obtain migration authority and protects its credentials. |
| `L5-E-03-*` | Backup/restore/PITR/DR | Supplies recovery capability when a migration is destructive or forward correction cannot safely restore service. |
| `L5-E-04-005` | Release installation/upgrade/technical rollback | Coordinates when compatible runtime versions and B02 migrations are executed. |
| `L5-E-05-*` | Operations/observability | Detects migration execution, failure, drift and incomplete transition conditions. |

### Must Not Own

- Business entity/domain ownership or lifecycle semantics.
- Operational business import/export.
- User-facing data correction or business reconciliation policy.
- Release approval or general deployment sequencing.
- Backup/restore/DR execution.
- Database/vendor migration-tool selection without ADR authority.
- Human roles/change-control process.
- Complete physical table/column/index design.

---

## 5. Solution Architecture

### 5.1 Persistence Evolution Lifecycle

Potentially breaking persistence change follows four controlled phases:

```text
1. EXPAND
   introduce backward-compatible new structure
        ↓
2. MIGRATE
   backfill / transform persisted data
        ↓
3. TRANSITION
   supported runtime changes authoritative use to new representation
        ↓
4. CONTRACT
   remove obsolete structure only after compatibility proof
```

The lifecycle is architecture-level and product-neutral. A change may combine phases only where evidence proves the combination cannot break a supported runtime, corrupt data, violate owner/Tenant isolation or invalidate recovery requirements.

### 5.2 Expand Phase

Expand introduces new persistence capability while preserving compatibility with runtime versions that are still legitimately active during the governed upgrade window.

Examples of architectural intent include:

- introducing a new optional representation before making it mandatory;
- introducing a replacement structure while the old representation remains readable/writable as required by the transition plan;
- adding supporting structure before moving existing data.

Expand shall not:

- remove a structure still required by a supported runtime;
- transfer ownership to another namespace;
- silently widen Tenant visibility;
- require an unavailable future runtime to make existing state valid.

### 5.3 Migrate Phase

Migrate transforms/backfills already-persisted owner data from the old representation to the new representation.

```text
owner-scoped source data
        ↓
validated migration transformation
        ↓
owner-scoped target representation
        ↓
post-migration validation
```

Internal migration shall preserve:

- canonical owner;
- Tenant and applicable Institution association;
- authoritative entity identity;
- business state meaning established upstream;
- transaction/concurrency requirements applicable to the target representation;
- state/publication integrity where event-related technical state is migrated.

A migration shall not invent missing business meaning. If transformation requires a business decision not derivable from authoritative state/rules, that decision is outside B02-004 and must be resolved by the appropriate owning architecture/process before technical migration proceeds.

### 5.4 Transition Phase

Transition moves supported application/runtime behavior to the new persistence representation while compatibility is still protected.

During transition:

- the release architecture shall know the minimum schema state required by each runtime version;
- mixed supported runtime versions shall not be permitted to corrupt each other's persisted state;
- write paths shall have one clear authoritative representation or a governed temporary compatibility mechanism;
- temporary compatibility logic shall have an explicit retirement condition;
- runtime startup shall not implicitly obtain schema-administration authority merely because migration is pending.

This specification defines compatibility requirements; `L5-E-04-005` owns release/upgrade orchestration.

### 5.5 Contract Phase

Contract removes obsolete persistence structures only after evidence proves they are no longer required by any supported runtime, migration/backfill, recovery process or approved compatibility window.

A contract action shall be treated as potentially destructive when it can remove data or eliminate the ability of an older runtime/recovery point to interpret persisted state.

Before destructive contract:

- required data transformation/backfill must be complete and validated;
- supported runtime dependency must be zero;
- recovery implications must be evaluated;
- rollback assumptions must be explicit;
- required backup/restore/PITR protection must exist under E03 where applicable.

### 5.6 Dedicated Governed Migration Execution

Normal application runtime credentials shall not be the default authority for production schema migration.

```text
approved migration package
        ↓
governed release/deployment trigger
        ↓
dedicated migration identity
        ↓
owner-scoped migration boundary
        ↓
execute migration
        ↓
validate result
        ↓
record applied state
```

The migration identity shall:

- be distinct from normal runtime persistence identities;
- receive only the privileged capabilities necessary for the approved migration scope;
- remain constrained to the intended owner persistence boundary and approved technical metadata;
- preserve Tenant/isolation constraints during data transformation;
- not become an alternate path for ordinary business-state mutation;
- be governed by applicable D02/D03 controls.

Multiple application instances shall not independently race to alter the same production schema during normal startup.

### 5.7 Migration Package and Stable Identity

Each migration shall have a stable technical identity/order sufficient to determine whether it has been applied to a given governed persistence boundary.

A migration package shall provide enough technical evidence to establish:

- migration identity/order;
- owner persistence scope;
- predecessor/schema-state expectation;
- structural/data transformation intent;
- compatibility phase(s) affected;
- whether the migration is potentially destructive;
- required post-migration validation;
- whether forward correction or E03 recovery would be required if the migration causes unacceptable outcomes.

Exact file format, checksum algorithm, tool syntax and migration framework remain implementation/technology decisions.

### 5.8 Immutable Applied Migration History

Once a migration definition has been applied to any governed environment, that applied definition shall not be silently rewritten.

```text
Migration N applied
        ↓
problem found
        ↓
DO NOT mutate Migration N
        ↓
create Migration N+1 corrective change
```

This prevents two environments from reporting the same migration identity while containing materially different structures or transformations.

Corrections shall use a new forward migration unless the environment is explicitly disposable and repository governance allows replacement before the migration becomes part of governed history.

### 5.9 Forward Correction vs Reverse Migration

B02-004 does not assume every schema/data migration is safely reversible.

A reverse migration is permitted only when it can demonstrably preserve authoritative information and required compatibility. If reversing would discard, misinterpret or corrupt authoritative state, the preferred technical recovery path is:

```text
new forward corrective migration
        OR
E03 governed restore / PITR / recovery
```

Release rollback under E04-005 therefore shall not automatically execute destructive reverse database changes.

### 5.10 Schema/Runtime Compatibility Gate

A runtime may operate only when the target persistence boundary is within a schema state that the runtime explicitly supports.

Conceptually:

```text
runtime version
     +
required schema compatibility range/state
        ↓
compatibility check
        ↓
compatible → proceed
incompatible → fail deployment/startup safely
```

The architecture shall prevent a runtime from continuing against a known-incompatible schema merely because a database connection succeeds.

Exact compatibility metadata representation belongs to the selected implementation/tooling standard.

### 5.11 Migration Failure Boundaries

| Failure Point | Required Outcome | Forbidden Outcome |
|---|---|---|
| Migration cannot obtain intended owner boundary | Fail before unauthorized change. | Falling back to broader/shared privileged scope. |
| Precondition/schema state differs from expectation | Stop and surface drift/conflict. | Blindly applying migration against unknown structure. |
| Structural migration fails before valid completion | Preserve/recover to a known governed state according to database/tool capability and migration design. | Reporting migration as applied without verified completion. |
| Data backfill partially fails | Remain explicitly incomplete and recover/resume/correct under governed migration logic. | Silently treating partial data transformation as complete. |
| Post-migration validation fails | Prevent dependent transition/contract step from proceeding. | Removing old representation despite failed validation. |
| Destructive contract causes unacceptable production result | Use approved forward correction or E03 recovery path according to recoverability analysis. | Assuming a reverse migration is inherently safe. |

### 5.12 Drift Prevention

Governed environments shall be able to determine whether their recorded migration state matches the expected canonical migration sequence.

Manual structural changes that bypass the governed migration history create schema drift and shall not be treated as normal architecture.

If emergency/manual repair is necessary, it must be reconciled into canonical migration history through a governed corrective migration or other explicitly approved technical recovery procedure so later environments do not inherit an undocumented divergence.

### 5.13 Migration and Tenant Isolation

For shared owner namespaces containing multiple Tenants, internal data migration shall process data in a way that preserves Tenant association and prevents cross-Tenant transformation/mixing.

For owner+Tenant dedicated placement, the same migration definition may be applied across multiple governed placement targets, but each application remains independently traceable and shall not infer completion for one Tenant from another Tenant's migration state.

This specification does not mandate database-per-Tenant migration architecture.

### 5.14 Operational Business Import/Export Boundary

Internal persistence migration exists to evolve the technical representation of already-authoritative owner data.

It shall not be used to bypass governed business ingestion by directly loading external operational records into authoritative tables.

```text
Schema/internal migration
    = technical representation evolution

Operational import/export
    = governed business-data interaction
```

The latter remains outside `L5-B-02-004`.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Status | Basis | Consequence | ADR |
|---|---|---|---|---|---|
| `AD-001` | Potentially breaking persistence changes use Expand → Migrate → Transition → Contract unless a governed exception proves atomic change safe. | **Provisional Draft direction** | Lead Architect selection 2026-09-11; compatibility/recovery requirements. | Allows rolling/controlled upgrades without coupling schema and runtime to one instantaneous change. | **ADR required — reusable cross-specification migration pattern; no ID allocated.** |
| `AD-002` | Production schema/data migrations execute through a dedicated governed migration identity/path, not normal runtime startup credentials. | **Provisional Draft direction** | B02-002 privileged-identity separation; Lead Architect selection 2026-09-11. | Prevents runtime privilege expansion and migration races. | **ADR scope review required as part of migration pattern; no ID allocated.** |
| `AD-003` | Applied migration definitions/history are immutable; corrections use new forward migrations. | **Provisional Draft direction** | Lead Architect selection 2026-09-11; deterministic environment requirement. | Prevents same migration identity from representing different schemas across environments. | **ADR scope review required as part of migration pattern; no ID allocated.** |
| `AD-004` | Database rollback is not assumed to be universally reversible; destructive failures use safe forward correction or E03 recovery when reversal would lose/corrupt data. | Defensible technical derivation | E03/E04 responsibility split; authoritative data integrity. | Avoids unsafe automatic reverse migrations. | Govern with migration/recovery ADR scope if material. |

No migration tool, ORM, database vendor, SQL dialect, checksum algorithm or deployment technology is selected here.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owning / Collaborating L5 Spec |
|---|---|---|
| Privilege | Migration authority is distinct from normal runtime and narrowly scoped. | B02-002, D02, D03 |
| Tenant isolation | Internal transformation preserves Tenant/Institution association and placement. | A04, B02-002/004 |
| Data integrity | Compatibility lifecycle and post-migration validation prevent premature destructive change. | B02-004 |
| Availability | Expand/contract separation reduces need for one instantaneous stop-the-world schema/runtime switch. | B02-004, E04-005 |
| Recoverability | Destructive migrations require recovery analysis; reverse migration is not assumed safe. | E03, B02-004 |
| Operability | Migration state, success/failure and drift must be observable. | B02-004, E05 |
| Portability | Pattern is vendor-neutral and does not rely on one database migration product. | B02-004, ADR governance |
| Auditability | Applied migration identity and result remain traceable and immutable. | B02-004, D04 where audit evidence is required |

---

## 8. Architecture Conformance Requirements

| CR ID | Conformance Requirement | Evidence Expected |
|---|---|---|
| `CR-001` | Every schema/data migration shall resolve to one authorized owner persistence boundary. | Migration-to-owner mapping. |
| `CR-002` | Potentially breaking changes shall use the compatibility lifecycle or document an approved exception proving an atomic change safe. | Migration plan / ADR or exception evidence. |
| `CR-003` | Normal production runtime credentials shall not be the default schema-migration authority. | Credential/privilege design. |
| `CR-004` | Migration execution shall not gain write authority over another canonical owner's authoritative namespace. | Privilege and negative-access tests. |
| `CR-005` | Internal data migration shall preserve Tenant/Institution isolation and authoritative identity. | Transformation validation / cross-Tenant negative tests. |
| `CR-006` | Applied migration identity/order and completion state shall be durably recorded per governed persistence boundary. | Migration-state evidence. |
| `CR-007` | An applied migration definition shall not be silently edited; correction shall use a new migration. | Repository/history validation. |
| `CR-008` | Post-migration validation shall complete before dependent transition/contract steps proceed. | Deployment/migration gate evidence. |
| `CR-009` | Obsolete persistence structures shall not be contracted while any supported runtime or required migration/recovery path still depends on them. | Compatibility/dependency evidence. |
| `CR-010` | Destructive migration shall have explicit recoverability analysis and required E03 protection before execution where applicable. | Recovery readiness evidence. |
| `CR-011` | Release rollback shall not automatically imply destructive reverse database migration. | E04/B02 rollback mapping. |
| `CR-012` | Known schema drift or unexpected predecessor state shall fail safe rather than be overwritten blindly. | Drift/precondition tests. |
| `CR-013` | Operational business import/export shall not be implemented as a B02-004 schema/internal migration bypass. | Architecture boundary review. |
| `CR-014` | No database/migration product shall be treated as canonical until applicable technology/ADR governance approves it. | ADR/technical-standard reference. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Owner | Blocking Status | Resolution Path |
|---|---|---|---|---|
| `OD-001` | Formal ADR for the reusable schema-evolution/migration pattern covering expand–migrate–transition–contract, dedicated migration execution and immutable migration history. | L5-B-02 / Lead Architect | **Blocks Architecture Approved for AD-001–AD-003.** | Confirm ADR scope, allocate canonical ID on `main`, evaluate alternatives, approve/revise and propagate. |
| `OD-002` | Select concrete database migration tooling/format after primary relational database technology/runtime choices are governed. | L5-B-02 with applicable technology owner | Does not block product-neutral Draft; blocks implementation standardization. | Technology evaluation + ADR if mandatory trigger applies. |
| `OD-003` | Define exact schema/runtime compatibility metadata and execution gate with `L5-E-04-005`. | L5-B-02 / L5-E-04 | Does not block Draft; required before release implementation. | Cross-specification technical contract. |
| `OD-004` | Define destructive-migration recovery evidence and restore/PITR integration once `L5-E-03-*` is stabilized. | L5-B-02 / L5-E-03 | Does not block Draft; required before destructive production migrations. | E03/B02 recovery mapping and validation. |

No canonical L5 ADR number is allocated by this specification.

---

## 10. Traceability Summary

### Canonical Responsibility

`L5-B-02-004` realizes the frozen corpus responsibility: controlled persistence-schema evolution/internal migration while excluding operational governed business import/export.

### Peer Architecture Preservation

- `L5-B-02-001` — persistence ownership and platform baseline remain unchanged by migration.
- `L5-B-02-002` — owner/Tenant isolation and privileged migration-identity separation remain mandatory.
- `L5-B-02-003` — transaction/concurrency/state-publication invariants remain valid during structural transition.
- `L5-E-03-*` — backup/restore/PITR/DR owns recovery mechanics.
- `L5-E-04-005` — release installation/upgrade/rollback owns deployment sequencing; B02-004 supplies schema compatibility and migration constraints.

### Resulting Evolution Model

```text
Approved persistence change
        ↓
EXPAND compatible structure
        ↓
MIGRATE / backfill authoritative owner data
        ↓
validate migration
        ↓
TRANSITION supported runtimes
        ↓
prove old structure no longer required
        ↓
CONTRACT obsolete structure
        ↓
immutable applied migration history

Problem after applied migration?
        ↓
new forward corrective migration
        OR
E03 governed recovery
```

This specification remains `Draft` while `OD-001` is unresolved and shall not be treated as the final approved repository-wide migration standard until the required ADR is allocated and approved.