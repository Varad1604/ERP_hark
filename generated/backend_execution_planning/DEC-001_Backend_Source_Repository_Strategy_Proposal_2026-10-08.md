# SutraERP Backend Planning — DEC-001: Source Repository Strategy

**Status:** RECOMMENDATION ACCEPTED IN PRINCIPLE by user on 2026-10-08; REPOSITORY NOT YET CREATED; detailed configuration and implementation still DRAFT / NOT APPROVED.  
**Date:** 2026-10-08  
**Purpose:** Record an independent, reviewable recommendation for where future backend runtime source should live. No repository is created here.  
**Evidence scope:** Only `DhruvNaik26/SutraERP_Architecture` at `main@933cf5d06f565cc52ef7e3b2a7f821606c2b9b6f` and `DhruvNaik26/SutraERP_frontend` at `main@372376d23ac6478fa4ac3f209cb97c71317d89d2` are SutraERP project evidence/authorities. Proposed future code location does not add an existing repository to decision-making.  
**Register link:** [P0 DEC-001 — draft](SutraERP_Backend_P0_Authority_and_Decision_Register_2026-10-08.md).  
**Decision owner:** User accepted the recommended private dedicated-backend repository approach on 2026-10-08. This does not approve backend code, locked-architecture changes, repo-main merging, runner execution or release.

## 1. Recommendation

**Recommend one new private GitHub repository dedicated to SutraERP backend runtime source**, proposed name `DhruvNaik26/SutraERP_backend` (name is provisional, not an approved canonical identity).

Maintain three distinct *roles*:

1. `SutraERP_Architecture` — governing L0–L6 business/technical architecture, approved source decisions, planning/governance evidence. **Do not add production runtime code** (architecture README §1 and CONTRIBUTING).
2. `SutraERP_frontend` — non-authoritative client, UX implementations and mock-operational review evidence; consume authentic released backend contracts when available. **Do not quietly convert the web-client repository into a mixed backend/runtime repository** (frontend README).
3. **Proposed future** backend repo — separately reviewed Rust platform implementation, physical owner-scoped PostgreSQL migrations, contract conformance, tests, deployment artifacts, backend CI. **No new business-rule or architecture authority** transfers to the new repo.

**Important distinction:** A single backend **Git repository** does not imply a single runtime process, a shared database schema, one service per business domain, or permission for cross-owner data access.

## 2. Why this is the lowest-risk choice

| Alternative | Assessment | Decision |
|---|---|---|
| Backend code inside the Architecture repo | Contradicts architecture repo's explicit planning-only responsibility; mixes governance and runtime permissions/builds. | REJECT |
| Backend subtree inside Frontend repo | Technically feasible without a third repo, but couples React/Node/browser CI with Rust/SQLx, database migrations, backend secrets and release workflows; increases AI workspace blast radius and makes repository protection/ownership harder to separate. | FALLBACK ONLY if strict two-repo hosting is explicitly required |
| New private backend repo with one modular Rust workspace | Keeps runtime permissions and release/build pipelines separately governable, while preserving A01/A03 logical modular architecture and a clean integration boundary to the frontend. Adds repo management and cross-repo contract synchronization work. | **RECOMMENDED, subject to approval** |
| One repository/service per domain | Confuses business ownership with deployment boundary and contradicts default modular-core/selective-isolation pattern. | REJECT |

**Verified architecture basis:**

- [Architecture README](../../README.md): repository is planning/architecture only; implementation is downstream.
- [ADR-L5-0003](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0003_Contract_Driven_Modular_Platform_with_Selective_Isolation.md): contract-driven modular core is default; selectively isolated execution units require material justification.
- [L5-A-03-002](../../architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-03-002_Module_Composition_and_Dependency_Enforcement_Architecture.md): implementation dependencies must be declared and validated; physical packaging is not architecture authority.
- [L5-E-04-002](../../architecture/L5_Solution_Architecture/specifications/L5-E/L5-E-04-002_Application_Packaging_and_Runtime_Isolation_Architecture.md): distinguishes modular core from justified isolated service/worker runtimes.
- [L5-F-01-001](../../architecture/L5_Solution_Architecture/specifications/L5-F/L5-F-01-001_CI_CD_and_Software_Supply_Chain_Security_Architecture_Baseline.md): controlled source, isolated build, reproducible evidence and governed artifacts.
- [Frontend README](https://github.com/DhruvNaik26/SutraERP_frontend/blob/main/README.md): web-client-only and current Stage-5 mock operational; no released authentic backend contract/production validation.

## 3. Proposed backend repository structure (illustrative only)

```text
SutraERP_backend/          # PROPOSED, DOES NOT EXIST OR HAVE APPROVAL
  Cargo.toml               # One Rust Cargo workspace
  apps/
    api/                   # Axum HTTP adapter and controlled composition
  crates/
    platform/              # Strictly shared technical facilities (not a business owner)
    owners/                # Owner-specific modules, individually approved and traceable
  workers/                 # Only justified isolated job/process packages
  contracts/
    api/                   # Versioned, governed/released OpenAPI consumable copies
    events/                # Versioned, governed/released JSON Schema consumable copies
  migrations/
    <owner>/               # Owner-local SQLx/PostgreSQL migration packages
  tests/
    contract/
    isolation/
    integration/
    recovery/
  deploy/                  # Approved OCI/runtime profile configuration
  docs/                    # Implementation traceability and operations, not redefinition of business rules
  .github/workflows/       # Backend-specific bounded PR/build/security gates
```

This example is a **developer layout proposal**, not the Technical Composition Declaration, domain ownership registry, SQL schema or approved implementation topology. An L5-A-03-derived machine-checkable composition declaration remains required, and actual module grouping must trace to approved owners, not section labels by convenience.

**Contract authority:** machine-readable API/event files may be stored as versioned **consumable** artifacts in backend code, but copying a schema to a repository does **not** make it a released/active authoritative contract. Source owner approval, version, active registry/catalog evidence and consumer compatibility remain independently governed. Frontend clients must be generated/implemented against **verified released** contracts only. A06/J05 unresolved event registry evidence (architecture issue #152) remains a blocker for the affected positive event path.

## 4. Coding, integration and CI safeguards (proposed)

- **Private repository:** least-privilege access; named code reviewers; PR-only `main` merges and required checks. Confirm actual branch protection and ruleset enforcement before AI agent write access; current ruleset coverage is not verified.
- **No cross-repo architecture drift:** every backend change cites exact Architecture source paths/versions and applicable decisions; CI/validation records a pinned Architecture commit or approved released contract catalog version, never silently assumes latest `main`.
- **Independent local feedback:** `cargo fmt`, `cargo check`, `clippy`, unit/domain/contract checks, isolated PostgreSQL SQLx migration/tests; use CI for attributable PR and integration evidence, not every AI edit.
- **AI agents:** no default access to architecture `main`, frontend `main`, PR merges, runner secrets, signing credentials or real sensitive student/employee data. A bounded branch/worktree is the normal write target.
- **Protected runner boundaries:** backend Rust tests and frontend browser tests may use isolated runners or VMs on second PC; no untrusted PR jobs with production signing secrets. Two runner registrations on one bare host are not security isolation.
- **Cross-repo integration:** a reviewed frontend PR consumes a specific released backend contract version and tests real owner responses; frontend mock success is not integration evidence.
- **Independent deployability:** separate backend release/artifact/rollback process, subject to E04/F01 governance; this does not force one backend service per module.

## 5. Costs and risks to accept explicitly

- Extra repository administration, GitHub permissions, branch governance and secret/access policies.
- Need a versioned contract distribution/synchronization strategy between backend and frontend.
- Running backend and frontend together for integration requires orchestrated test environments.
- Backend target must be authorized as a *new, initially empty source repository* before implementing any runtime code.
- Existing A01/A03 logical module boundaries, owner-local PostgreSQL access, Tenant/Institution isolation and external Flowable runtime boundary are **not** relaxed by the source-code repository choice.

## 6. Decision request / stop rule

**Decision disposition:** The user approved proceeding with the **new private dedicated-backend repository** approach on 2026-10-08 and authorized going ahead with creation. Proposed owner/name: `DhruvNaik26/SutraERP_backend`. Creation **has not occurred**, because this GitHub connector supports writing within accessible repositories but does not expose a create-repository action. User-side GitHub UI creation (or a separately authorized creation capability) is required. Rust Cargo workspace shape, branch rules, credentials, workflows, runner isolation and implementation still require subsequent approval/verification.

**Creating the explicitly authorized empty private backend repository is permitted, but it is NOT yet accomplished and confers no new architecture/governance authority. Do not add code to Architecture or Frontend as a shortcut. No API, data table, event, business rule, permission, scope or physical topology may be inferred from this proposal. Do not merge this proposal without explicit review.**

### Follow-up acceptance checklist

- [x] User selected dedicated backend repository in principle (2026-10-08).
- [x] User authorized recommended private `DhruvNaik26/SutraERP_backend` repository creation; exact spelling can be adjusted before creation.
- [ ] Branch protection, CI/runner trust, AI agent identity, approvals and secret policies specified.
- [ ] Machine-readable module boundary/conformance mechanism selected.
- [ ] Contract provenance/version/registry distribution approach approved.
- [ ] Empty private backend repository actually created and GitHub connector granted access; no production code or CI authorized yet.
