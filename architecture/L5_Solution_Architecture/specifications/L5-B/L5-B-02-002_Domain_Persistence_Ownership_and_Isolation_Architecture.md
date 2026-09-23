# Domain Persistence Ownership & Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-02-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-02 — Database & Persistence Architecture  
**Canonical Technical Responsibility:** Enforce technical persistence ownership and prevent unauthorized cross-domain writes.  
**Technical Architecture Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.3.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Broad Authoring Authorization v1.0.0; L5 Dependency-Gated Parallel Authoring Execution Timeline v1.0.0; `L5-B-02-001` v1.3.0 Architecture Approved; `L5-A-03-001` v1.1.0 Architecture Approved; `L5-A-04-001` v1.1.0 Architecture Approved; `L5-A-04-002` v1.1.0 Architecture Approved; `L4-A-001` v1.0.1 Architecture Approved; `L4-A-015` v1.0.1 Architecture Approved; `L4-A-019` v1.0.1 Architecture Approved; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** Downstream persistence, privacy, reporting, secure-engine, recovery and integration architecture consuming owner-scoped persistence boundaries.  
**Applicable ADRs:** `ADR-L5-0005`; `ADR-L5-0036`.

---

## 2. Purpose, Scope & Non-Goals

This specification defines the enforcement architecture that preserves canonical persistence ownership and Tenant isolation inside the B02 baseline. It prevents shared physical infrastructure, shared runtime or technical convenience from creating unauthorized cross-owner persistence access.

### In Scope

- Owner Persistence Boundary and Namespace.
- Owner Runtime Persistence Identity.
- Persistence Scope Guard.
- Hybrid Tenant placement inside an owner boundary.
- Separation of normal runtime identity from migration/maintenance identity.
- Prohibition of direct cross-owner authoritative read/write paths.

### Out of Scope

- Database product and physical database count — B02-001/future technology ADR.
- Transaction/concurrency — B02-003.
- Schema evolution/migration — B02-004.
- Tenant scope semantics — A04.
- Authorization/trust internals — D02/D03.
- Recovery/deployment topology — E03/E04.

---

## 3. Authoritative Basis & Constraints

| Source | B02-002 Effect |
|---|---|
| L1 v2.1 | Only canonical owner modifies authoritative data; direct cross-domain database access prohibited. |
| `L5-B-02-001` | Establishes authoritative owner persistence boundary and hybrid physical topology. |
| `L5-A-04-001/002` | Tenant-rooted validated execution scope must reach protected persistence. |
| `ADR-L5-0005` | Approves owner-scoped namespaces/identities and hybrid Tenant placement. |

### Constraints

1. Every protected persistence operation resolves to one canonical owner boundary.
2. Normal runtime persistence identities are owner-scoped.
3. Cross-owner direct authoritative read/write access is prohibited as normal collaboration.
4. Tenant scope is enforced within owner persistence boundaries.
5. Shared physical database infrastructure does not imply shared namespace/write authority.
6. Privileged migration/maintenance access is separated from normal runtime and shall not become a business-write bypass.
7. Dedicated Tenant/database placement is selective and constraint-driven, not mandatory for every Tenant.

---

## 4. Technical Ownership & Boundaries

### Owns

- **Owner Persistence Boundary** — logical authoritative persistence scope for one canonical owner.
- **Owner Persistence Namespace** — technical namespace/partition boundary preserving owner separation.
- **Owner Runtime Persistence Identity** — normal runtime authority constrained to its owner scope.
- **Persistence Scope Guard** — validates owner plus applicable Tenant/Institution scope before protected data access.
- Tenant placement model inside an owner boundary.

### Consumes / Collaborates With

- A04 validated execution scope.
- B02-001 physical/logical baseline.
- B02-003 transaction/concurrency.
- B02-004 migration execution.
- D02/D03 privileged authorization/trust/credential controls.
- B05 privacy and E02/E03/E04 constraints for selective dedicated placement.

### Must Not Own

Business data ownership, Tenant lifecycle, authorization policy, transaction semantics, migration lifecycle, backup/recovery or deployment topology.

---

## 5. Solution Architecture

### 5.1 Persistence Access Path

```text
validated execution context
        ↓
resolve canonical owner
        ↓
Owner Runtime Persistence Identity
        ↓
Persistence Scope Guard
        ↓
Owner Persistence Namespace
        ↓
authoritative owner data
```

Failure to establish owner or required Tenant scope blocks protected persistence.

### 5.2 Owner Runtime Identity

Normal runtime identity shall be bounded to the authoritative persistence namespace(s) owned by that technical owner. Co-location in one process/database/server shall not grant peer-owner authority.

### 5.3 Cross-Owner Collaboration

When one owner needs another owner's business fact, collaboration uses the canonical contract/event/workflow/read-model boundary. Direct mutation of the peer owner's authoritative persistence is prohibited. Direct peer reads that bypass the canonical collaboration architecture are also prohibited as normal owner integration.

### 5.4 Hybrid Tenant Placement

Default physical placement may share an owner-scoped persistence namespace/infrastructure across multiple Tenants where the A04 scope guard remains enforceable.

Escalation may use:

- stronger owner+Tenant namespace isolation; or
- dedicated physical database/placement,

when approved security, privacy, workload, resilience, recovery, compliance or deployment constraints justify it.

No numeric threshold or universal database-per-Tenant rule is defined here.

### 5.5 Privileged Migration / Maintenance

Schema/data migration or governed maintenance may require authority beyond normal runtime permissions. That authority shall be separated, attributable and limited to the approved operation. It shall not become an alternate ordinary business-write path.

### 5.6 Failure Model

| Failure | Required Outcome |
|---|---|
| Owner cannot be resolved | Reject persistence operation. |
| Runtime identity attempts peer-owner namespace access | Deny. |
| Tenant scope missing/mismatched | Fail closed. |
| Shared placement cannot preserve required isolation | Escalate placement; do not relax isolation. |
| Privileged identity used as normal runtime | Non-conformant; reject/remove broad path. |
| Direct cross-owner data path discovered | Treat as architecture violation and route through governed collaboration. |

---

## 6. Architecture Decisions

| Decision ID | Decision | Status | ADR |
|---|---|---|---|
| `AD-001` | Authoritative persistence is partitioned by canonical owner through owner-scoped logical boundaries/namespaces. | Architecture Approved | `ADR-L5-0005` |
| `AD-002` | Normal runtime persistence identity is owner-scoped; privileged migration/maintenance authority is separated. | Architecture Approved | `ADR-L5-0005` |
| `AD-003` | Tenant placement is hybrid: shared owner-scoped storage is permitted by default where isolation holds, with stronger/dedicated placement when justified. | Architecture Approved | `ADR-L5-0005` |

---

## 7. Security, Privacy & Operational Impact

- Reduces blast radius of accidental or compromised persistence access.
- Makes cross-owner negative-access tests mandatory.
- Preserves Tenant isolation on shared physical infrastructure.
- Allows privacy/security/recovery constraints to require dedicated placement without turning it into universal topology.
- Vendor-specific database roles/features remain downstream choices.

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Every authoritative persistence operation shall resolve one canonical owner boundary. |
| `CR-002` | Normal runtime identities shall be owner-scoped. |
| `CR-003` | Direct cross-owner authoritative persistence access shall be denied. |
| `CR-004` | Tenant/Institution scope shall be enforced before protected persistence access. |
| `CR-005` | Shared physical placement shall preserve logical owner and Tenant isolation. |
| `CR-006` | Dedicated Tenant/database placement shall be justified by approved material constraints rather than preference. |
| `CR-007` | Privileged migration/maintenance authority shall remain separate from normal runtime authority. |
| `CR-008` | Privileged technical paths shall not become ordinary business-write bypasses. |
| `CR-009` | B02-002 shall not redefine transaction/concurrency or migration mechanics owned by B02-003/B02-004. |
| `CR-010` | Vendor-specific isolation features shall not be canonical without applicable technology/ADR authority. |

---

## 9. Open Decisions & Deferred Design

`ADR-L5-0005` resolves the former blocking persistence-isolation ADR item.

| ID | Item | Blocking Status | Resolution Path |
|---|---|---|---|
| `OD-002` | Exact governed criteria for escalating an owner/Tenant from shared placement to stronger namespace/dedicated database. | Does not block product-neutral Draft; required before operational placement automation/implementation. | Consume approved B05/D03/E02/E03/E04 constraints; do not invent numeric thresholds here. |

---

## 10. Traceability Summary

Canonical architecture: `AD-001`–`AD-003` → `ADR-L5-0005`.

A04 owns Tenant scope semantics; B02-002 enforces those semantics at persistence boundaries. B02-001 owns the persistence baseline; B02-003/004 own transaction and migration mechanics.


## 2026-09-22 Backend Readiness Group 2 Synchronization

**Applicable locked decision:** `ADR-L5-0036`  
Normal runtime code shall not directly read or write another canonical owner's authoritative tables/schema. Co-location in the same PostgreSQL instance does not weaken owner boundaries.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
