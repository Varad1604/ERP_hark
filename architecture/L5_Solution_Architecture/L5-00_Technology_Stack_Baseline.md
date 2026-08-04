# L5-00 — Technology Stack Baseline

**Status:** Draft
**Architecture Level:** L5 — Solution Architecture
**Document Owner:** Dhruv
**Created:** 2026-08-04
**Source:** Extracted verbatim from `architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md` section 3 under integrity finding INT-002.

---

# 1. Purpose

This document records the implementation technology baseline and the modular
monolith decision for SutraERP.

It exists because L0 Business Vision must remain business-facing and
technology-independent, as required by `governance/constitution/Volume_1_Governance.md`
lines 152-168. L0 previously prescribed a concrete stack — Rust, Axum, Tokio,
PostgreSQL, SQLx, PgBouncer, Redis, React, Vite, TypeScript, Zustand and Axios —
which falsely elevated implementation choices to immutable business vision.

# 2. Status of This Document

**This content has not been re-reviewed as L5 architecture.** It was moved here
verbatim to remove a level violation from L0, not because it has been validated
against an approved L4 System Behavior Specification. Per
`architecture/L5_Solution_Architecture/README.md`, L5 documents must trace to
approved L4 artifacts, and none exist yet.

Treat this as a *recorded observation of the audited implementation as of
2026-07-05*, not as an approved solution architecture. It must be re-authored
against the L5 template before it can be cited as canonical.

# 3. Recorded Technology Baseline

The audited repository is currently an **Early Modular ERP** implemented
as a **Modular Rust Monolith**.

## 3.1 Current Stack

-   Backend: Rust.
-   Web framework: Axum 0.7.
-   Async runtime: Tokio.
-   Database: PostgreSQL 16.
-   Database access: SQLx raw asynchronous SQL.
-   Pooling: PgBouncer.
-   Cache/support infrastructure: Redis.
-   Frontend: React 19.
-   Build tool: Vite.
-   Language: TypeScript.
-   Frontend state: Zustand for authentication.
-   HTTP client: Axios.
-   File infrastructure declared: MinIO, but complete Rust S3
    integration is not confirmed.
-   Authentication: JWT HS256 with refresh tokens and Argon2id password
    hashing.
-   Authorization: mixed database-backed RBAC and hardcoded role-name
    checks.

## 3.2 Current Backend Model

``` text
One Rust Binary
      ↓
Axum Router
      ↓
Internal Domain Modules
      ↓
Shared AppState
      ↓
Single PostgreSQL Database
```

Current backend modules include:

-   Auth.
-   Student.
-   Academics.
-   Examination.
-   Finance.
-   Library.
-   Hostel.
-   Transport.
-   Placement.
-   Medical.
-   HR.
-   Health.

The current database uses one PostgreSQL database and one public schema
with logical domain separation.

## 3.3 Architecture Decision

**SutraERP will not be redesigned as microservices at this stage.**

The current modular monolith will evolve into a:

> **Contract-Driven Modular ERP Platform**

Reasons:

-   Current scale does not justify distributed-service operational
    complexity.
-   Separate teams can be supported using domain ownership and explicit
    contracts.
-   A modular monolith permits transactional consistency where
    appropriate.
-   Future extraction into services remains possible if domain
    boundaries are disciplined.

Risk:

-   Without enforcement, developers may continue using direct
    inter-domain SQL and shared files, creating a distributed-monolith
    style coupling inside one binary.

---

# 4. Traceability

| Item | Value |
|---|---|
| Extracted from | L0 section 3, "Current Repository Architecture" |
| Extraction date | 2026-08-04 |
| Integrity finding | INT-002 (Type 6 — Structural Drift, High) |
| Review | `generated/reviews/integrity-2026-08-03-repository-wide-review.md` |
| Template to adopt | `templates/L5_SolutionArchitecture_Template.md` |
