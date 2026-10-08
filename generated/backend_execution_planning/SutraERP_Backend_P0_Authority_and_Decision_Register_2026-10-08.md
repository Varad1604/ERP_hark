# SutraERP Backend — Gate P0 Authority, Scope and Decision Register

**Date:** 2026-10-08 (Asia/Kolkata)  
**Status:** **DRAFT FOR REVIEW — P0 evidence gathered; P0 NOT formally approved or closed**  
**Work mode:** Documentation-only planning, authorized 2026-10-08. This draft is proposed on a review branch; no approval of canonical architecture changes or production code is implied. No implementation, tests or CI have been executed as part of this planning update.  
**Authority constraint:** `SutraERP_Architecture` governs canonical architecture; `SutraERP_frontend` is current client evidence; **`DhruvNaik26/SutraERP_backend` is now the user-authorized new implementation target**, created 2026-10-08 with README at `main@213c323303bf9fff4284c15b94f9d4735d68759a`. No historical/legacy backend code is an authorized baseline. Public vendor standards may be consulted for technical due diligence but do not create business authority. Public vendor standards may be consulted later for tool due diligence but are not business-architecture authority.

## 1. Verified repositories and baseline

| Repository | Verified branch | Head SHA on audit | Governs |
|---|---|---|---|
| [SutraERP_Architecture](https://github.com/DhruvNaik26/SutraERP_Architecture) | `main` | `933cf5d06f565cc52ef7e3b2a7f821606c2b9b6f` | L0–L6 approved architecture, domain ownership, design invariants, technical and programme governance |
| [SutraERP_frontend](https://github.com/DhruvNaik26/SutraERP_frontend) | `main` | `372376d23ac6478fa4ac3f209cb97c71317d89d2` | Verified current frontend implementation, workflow representation and integration expectations; does **not** define backend business contracts |

Architecture's [final L5 lock decision (2026-09-23)](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/governance/reference/L5_Final_Corpus_Lock_Decision_2026-09-23.md) records **105 locked L5 specifications, 48 ADRs and 571 L4→L5 relationships**. Subsequent domain-specific issues can remain open; the L5 lock does not prove every domain contract has operationally published schema or backend implementation.

The [Stage-5 terminal evidence](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L6_UI_UX_Architecture/00_Governance/L6_Stage5_Batch20_Final_Validation_Cleanup_and_Stage6_Readiness_2026-10-06.md) records completion of the mock-operational frontend and Stage-6 readiness. The [frontend README](https://github.com/DhruvNaik26/SutraERP_frontend/blob/main/README.md) explicitly excludes authentic released backend integration and production validation. The frontend root has no dedicated `backend/` source directory at the verified head. **No released SutraERP backend implementation is verified by this two-repository P0 review.** Do not turn absence of evidence into a claim that no backend exists anywhere.

Branch endpoint metadata reported `protected: false` for both `main` branches. **Effective repo rulesets / merge protections are NOT fully verified**, so audit and configure applicable controls before AI write access and merging.

### 2026-10-08 execution clarification — user approved, review still pending

- **Backend target:** the new private [SutraERP_backend](https://github.com/DhruvNaik26/SutraERP_backend) exists, is connected and has README only; it is the sole designated target for **new** backend implementation. Architecture remains canonical; code cannot create business authority.
- **Prior backend code:** any old/pre-existing backend repository/code is **user-designated LEGACY/OBSOLETE as an implementation baseline**, not a usable dependency, compliance evidence or automatic migration candidate. Do not copy, run, deploy, delete, or promote that code by default. A later migration exception requires a separate explicitly approved review with independent tests. This is a prospective programme rule, not a claim that old code was exhaustively inspected.
- **Frontend API status:** authentic frontend↔backend integrations/production API contracts are **not configured/released** as project implementation evidence. Do not infer released APIs from UI mocks, paths, fixtures or examples.
- **Protection responsibility:** user will arrange GitHub protections. The connected API reported `protected:false` and ruleset verification was restricted; **enforcement is UNVERIFIED**, so no AI autonomous merge/write to protected base, no production coding/CI until checks are demonstrably operational.
- **Execution order:** (1) update decision/status truth, (2) choose smallest low-dependency slice, (3) approve its owner and released contract, (4) reproducible local backend/security baseline, (5) real protected read with frontend integration, (6) first governed write, (7) only then durable cross-owner event path if applicable. Logical entity coverage across A–J proceeds in parallel as **planning**; no need for all owner physical DDL before first slice.

See [integration gate plan](DEC-011_Frontend_Backend_Integration_Safety_and_Acceptance_Plan_2026-10-08.md) and [execution issue-resolution addendum](DEC-012_Execution_Risk_Resolution_and_Decision_Packet_2026-10-08.md).

## 2. Non-negotiable L1 logical design constraints

Primary authority: [L1 locked business domain architecture](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md), §§2.1–2.4 and 3.1–3.5, **as amended** by the [Locked L1 Person Scope Amendment (2026-08-26)](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md). Where the base L1 and amendment disagree, the later Locked amendment controls.

1. Each business entity has exactly one canonical owner; the owner alone may create/change its authoritative business state. Cross-owner integration uses approved contracts/events, not shared-table reads/writes.
2. Distinguish **Platform → Tenant → Institution → College/School → Department**. A Tenant may have one or multiple Institutions; **Tenant and Institution are not synonyms**. Physical Campus is not an organizational unit.
3. **CRITICAL LOCKED AMENDMENT: Person is Tenant-scoped, NOT Institution-scoped.** Every real human has exactly one canonical Person within a Tenant, and may participate in multiple Institutions under that Tenant through institution-scoped relationships. Cross-Tenant Person sharing is NOT established. The amendment expressly supersedes the earlier L1 §3.4 wording saying Person was per Institute. Platform Foundation / Person & Identity retains canonical identity ownership; institution and operational owners retain their own relationships/lifecycles. A false Institution-level uniqueness constraint or duplicate Person-per-Institution table model would directly violate this canonical amendment. A Person may hold Student, Faculty, Parent, Alumni or Employee personas as governed by their owners.
4. **Student Context** is owned by Student Lifecycle. One Student persona may have historical Student Contexts, but **at most one ACTIVE Student Context within an Institution**. Transition eligibility includes explicit terminal-state rules; do not implement a simplistic universal boolean without the approved lifecycle model.
5. Example canonical entity owners: Person → Platform Foundation / Person & Identity (Tenant-scoped); Institution → Institution; Student Context → Student Lifecycle; Programme → Academic Structure; Academic Year → Academic Calendar; Examination Result → Examination; Financial Ledger → Finance. These examples do not replace owner-by-owner L2–L4 verification.
6. Shared identity never grants shared operational data. Operational scope is narrower and source-owned. Exact key/reference cardinality and relational representation require separate approved P1/P3 design.

**P1 mandatory validation:** Build the complete entity→owner→scope→business ID→technical ID→lifecycle→retention→correlation matrix before physical DDL for each slice. Mark missing upstream semantics **BLOCKED**, not presumed.

## 3. Locked L5 implementation constraints relevant to backend

| Concern | Verified canonical decision | Primary source |
|---|---|---|
| Composition | Contract-driven modular application core with selectively justified isolated units; one domain does not imply one microservice | [L5-A-01-001](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-01-001_Application_and_Platform_Architecture.md) |
| Module enforcement | Declared parent/dependency graph; no forbidden peer-internal coupling, cycles or cross-owner persistence | [L5-A-03-002](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-03-002_Module_Composition_and_Dependency_Enforcement_Architecture.md) |
| Tenant execution | Validated Tenant root; Institution and lower scopes subordinate; missing/widened/switching scope fails closed | [L5-A-04-002](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md) |
| Rust backend | Rust, Axum and Tokio primary runtime | ADR-L5-0018, -0019 |
| Relational persistence | PostgreSQL 18.x + SQLx; owner-scoped schema/roles or equivalent boundaries | ADR-L5-0020, [ADR-L5-0036](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/adr/ADR-L5-0036_Persistence_Implementation_Conventions_and_Technical_Identity_Standard.md) |
| Technical identity & time | UUIDv7 stored as `uuid` by default; `timestamptz` UTC for technical instants; business-local timezone preserved where needed | ADR-L5-0036 |
| Persistence semantics | No cross-owner database access; owner-local transactions and optimistic concurrency by default; durable publication intent atomically with state | ADR-L5-0005, -0006, -0007 |
| Evolution | Explicit immutable versioned migrations, Expand → Migrate → Transition → Contract; separate migration authority; no universal soft delete | ADR-L5-0008, -0036 |
| Synchronous API | OpenAPI 3.2 contract-first; JSON `camelCase`; major `/api/v1`; RFC 9457 errors; scoped `Idempotency-Key` for retry-sensitive commands | [ADR-L5-0034](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/adr/ADR-L5-0034_Synchronous_API_Wire_Versioning_Error_and_Idempotency_Profile.md) |
| Events | NATS JetStream; CloudEvents-compatible JSON event envelopes, JSON Schema 2020-12, explicit approved versions, idempotent consumers | ADR-L5-0022, [ADR-L5-0035](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/adr/ADR-L5-0035_Event_Wire_Envelope_and_Schema_Serialization_Profile.md) |
| Auth | Keycloak 26.x credential authority; D01 custom credential API via supported adapter (NOT Resource Owner Password/Direct Access Grant), secure opaque browser session; D02 authorizes | ADR-L5-0024, [ADR-L5-0038](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/adr/ADR-L5-0038_Custom_Credential_API_Server_Session_and_MFA_Profile.md) |
| Workflow/background | Flowable isolated workflow orchestration does not own domain state; Rust background workers with JetStream | ADR-L5-0023, -0041, -0042 |
| CI & supply chain | GitHub Actions + GHCR + Sigstore/Cosign; trusted production build/signing requires appropriate isolation, prefer ephemeral self-hosted | [ADR-L5-0030](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/main/architecture/L5_Solution_Architecture/adr/ADR-L5-0030_GitHub_Actions_GHCR_and_Sigstore_Supply_Chain_Standard.md) |

**Caution:** A 'locked' L5 technical pattern is not an approved business field, service endpoint, event name, or released versioned operational contract.

## 4. Verified outstanding architectural and frontend dependency groups

The following open items are real dependency gates, **not reasons to block unrelated foundation work**:

| Group | Open authoritative issues | Affected backend slice | Stop condition |
|---|---|---|---|
| Qualification / academic completion | [Architecture #136](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/136), [#137](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/137); [frontend #50](https://github.com/DhruvNaik26/SutraERP_frontend/issues/50), [#57](https://github.com/DhruvNaik26/SutraERP_frontend/issues/57) | Official I08→I09→I10 evidence, C04/I13→C05 qualification, clearance and exit | Do not create qualification authority/approval/events from frontend fixtures or unresolved L3 ownership. Require approved owner contracts and actual source history. |
| Receipt correction / financial holds | [Architecture #140](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/140), [#144](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/144), [#145](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/145), [#146](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/146); [frontend #67–69](https://github.com/DhruvNaik26/SutraERP_frontend/issues/67) | J05 finalized correction, F08/F14 holds, uncertain NOT_APPLIED handling | No invented correction/hold semantics, success claims or resolution shortcut. |
| Event identity / active registry | [Architecture #152](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/152) | J05 variance event and consumers | Do not assume recommended document naming is an active event registry entry/version. Publisher/consumer compatibility evidence required. |
| Backend code placement / governance | [Architecture #147](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/147) | All backend implementation work | This plan treats only the existence of the source-repository governance question as evidence. Its analysis of any other existing repository is **out of scope**, must not govern this plan. Independently decide code target using only the two authorized repositories and your explicit approval. |

Additional implementation standard gaps confirmed in L5 texts:
- `L5-A-03-002` §9 `OD-002`: Technical Composition Declaration concrete schema/repository convention is deferred. Must select before machine-enforced dependency checks.
- `L5-A-04-002` §9 `OD-002`: exact context-carrier representations and enforcement mechanics remain implementation design.
- `L5-B-02-002` §9 `OD-002`: precise triggers for Tenant/owner escalation to dedicated storage remain deployment policy work.
- `L5-C-02-003`: an actual registry *product* is unselected; concept of authoritative active contract does not prove its operational deployment.

## 5. First decision register: approvals required before backend implementation

| ID | Proposed decision / question | Proposed position, NOT approved | Gate / acceptance |
|---|---|---|---|
| DEC-001 | Where will backend source live? | Architectural rules stay in Architecture repo. Frontend stays client authority only. If strictly two repositories must host source, use a clearly separate Rust workspace under frontend with independent controls **only if explicitly approved**. A dedicated new backend repository would be cleaner for ownership, CI and release autonomy **but requires explicit expansion of the current repo scope**. Do not create one now. | Approve code location, branch model and maintainer/reviewer authorities. |
| DEC-002 | P1 identity and scope model | **Tenant-scoped Person** per the later Locked L1 amendment (not the superseded Institute-scoped rule); Tenant-rooted scope, Institution hierarchy, distinct College/School vs Campus, explicit institution-governed Student Context lifecycle; do not assume all physical tables contain identical scope columns. | Trace L1→L2/L3/L4 per entity; verify legal constraints/correlation and dedicated history. |
| DEC-003 | SQL conventions | Propose `snake_case` schema/table/column names; owner-scoped namespaces and DB roles; UUIDv7 technical key, separate business IDs; no blanket `deleted_at`, `status`, `tenant_id`, or timestamps. | Approve naming/type/constraint/nullability/index and versioned migration standards separately from business semantics. |
| DEC-004 | Tenant DB enforcement | Evaluate owner-scoped credentials, narrowed PostgreSQL roles, transaction/session scope, and RLS where it strengthens defense-in-depth without undermining isolation, pooling or migration. No assertion RLS alone is sufficient. | Implement repeatable negative access/missing-context/pool-leak tests in first vertical slice. |
| DEC-005 | Technical module declaration | Propose machine-readable declaration of canonical owner, parent, dependencies, public contract consumption and allowed imports; select format/version in P4. | A03 static checks reject prohibited dependencies on every PR. |
| DEC-006 | Contract catalogue and release status | Separate `proposed`, `approved`, `released`, `active`, `deprecated`, `retired` governance semantics as defined by source owners; do not invent registry records or event names. | Verify canonical source + schema + status/version + consumers + compatibility for first slice. |
| DEC-007 | Local vs two GitHub runners | Target two isolated workers on the second PC, not two unrestricted services sharing a host. Validate CPU, memory, disk, virtualization, port/data isolation, credentials, cancellation and cleanup. | One safe job first, then measured parallel tests; PRs do not receive signing credentials. |
| DEC-008 | AI agent and model | DeepSeek and Ponytail are candidates, not approved production engineering dependencies. Select agent harness and pinned versions; require least-privilege repo workspace and no secret/student/employee data sent to external models. | Vendor privacy/security terms review + benchmark against fixed, source-derived Rust/SQLx acceptance tests. |
| DEC-009 | Human/AI merge safeguards | Protected PRs; architecture-owner signoff for source semantics, code-owner signoff, independent review and test evidence; no auto-merge, force-push or CI reruns by default. | Effective GitHub branch rulesets/permissions and workflow trust model independently verified. |
| DEC-010 | First golden slice | A narrow end-to-end demonstration of session/tenant scope → owner-local state → durable publication intent → released event consumption → non-authoritative frontend read; choose business action only after owner evidence. | Approved model, exact DDL+OpenAPI+event schema, failure/negative tests and traceability before coding. |

## 6. Pre-coding delivery gates and completion evidence

| Gate | Product | Must prove | Current status |
|---|---|---|---|
| P0 — Authority & blockers | Snapshot, source matrix, blocker/dependency map, code-target decision register | Canonical Architecture + Frontend authority, new Backend target, current refs, L1/L5 rules, blockers, reviewers/decisions | **IN PROGRESS**; repository target selected, branch protection and other gates open |
| P1 — Complete logical design | Whole-system conceptual/ER/ownership/scope model; primary data dictionary | No duplicate authority; clear Person, Tenant, Student Context, history, effective policy and read-only cross-owner projections | NOT STARTED |
| P2 — Communications | API/event/workflow/consumer catalogue with owner, contract version, failure semantics | No invented released endpoints/events; cross-domain command/idempotency/reconciliation/error model | NOT STARTED |
| P3 — Per-slice physical schema | Full table/column/type/nullability/PK/FK/index/constraint/RLS policy/DDL/migrations and roll-forward/backfill | L1–L5 traceability, accepted domain invariants, fail-closed negative tests | NOT STARTED |
| P4 — Engineering/infra | Rust crates, composition rules, local environment, two-runner strategy, CI trust, D01/D02 security and observability | Reproducible secure local validation before scarce Actions use | NOT STARTED |
| P5 — AI execution protocol | Benchmarked model/agent/Ponytail configuration, prompts, work packages, review/checkpointing, change limits | Measured pass/fail/cost, authorized data transmission, independently accepted changes | NOT STARTED |

### P0 completion checks still outstanding

- [x] Verified main branch heads of both authorized repositories.
- [x] Verified L1 base AND the later Locked Person Scope Amendment. **Amendment supersedes Institute-scoped Person wording: Tenant-scoped Person is authoritative.**
- [x] Verified L5 lock and key database/contract/worker/security decisions.
- [x] Retrieved open issues from only the two authorized repositories.
- [x] Verified the frontend Stage-5 mock boundary and current frontend workflow properties.
- [ ] Complete exhaustive L1/L2 owner/entity/capability and canonical event register, including all amended authoritative sources.
- [ ] Complete target-by-target dependency mapping including event/consumer chronology and all policy-source owners.
- [x] User selected and created dedicated private backend target; legacy backend baselines excluded. **[ ]** PR/branch governance and effective protections still require confirmation.
- [ ] Confirm effective GitHub rulesets, runner resource specs and security controls.
- [ ] Choose and record the first nonconflicted golden slice with approved business behavior.
- [ ] Obtain explicit Lead Architect acceptance to close P0 and authorize P1/P2 drafting.

## 7. AI coding verification benchmark — design only

Benchmark **identical** approved tasks under fixed model/harness/Ponytail variants and tool permissions. Representative safe tasks: OpenAPI-generated Rust adapter with contract conformance; owner-local SQLx persistence with tenant-negative tests; outbox crash/restart and idempotent replay tests; cross-owner access denial. Use synthetic non-sensitive data and inspect all test code. Track actual cost/token count, tool/retry count, runtime, first-pass passing tests, human corrections, security defects and architectural violations. A cheaper model that repeatedly violates owner/state boundaries is not economical.

**No benchmark has yet been run, no API credentials were used, and no vendor data terms were approved in this gate.**

## 8. Progress and integrity statement

This artifact is a working decision/evidence register, **not** a promoted canonical governance decision or accepted backend design. Current branch evidence must be refreshed when a change is considered. No database schema, field name, API, event, business state, permission, code path, or release is declared approved by this file alone.