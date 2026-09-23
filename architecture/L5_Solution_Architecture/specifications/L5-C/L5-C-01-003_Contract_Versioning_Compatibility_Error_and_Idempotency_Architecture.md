# Contract Versioning, Compatibility, Error & Idempotency Architecture

## 1. Specification Metadata

**Specification ID:** `L5-C-01-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-01 — API & Contract Architecture  
**Canonical Technical Responsibility:** Contract evolution, compatibility, technical errors and idempotent request handling without inventing business recovery.  
**Technical Architecture Owner:** L5-C-01 — API & Contract Architecture  
**Assigned Author:** Pallav  
**Version:** `1.3.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-C-01-001`, `L5-C-01-002`, `L5-B-02-003`, `L4-A-029`, `L4-A-031`, `L4-A-032`.  
**Collaborates With / Referenced By:** API consumers, `L5-C-03-*`, `L5-E-04-005`.  
**Applicable ADRs:** existing C01 ADRs plus `ADR-L5-0021` and `ADR-L5-0034`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define safe contract evolution, compatibility classification, technical errors and duplicate-request handling while preserving business semantics and consumer migration evidence.

### In Scope

- Compatible/conditional/breaking change classification.
- Technical contract lifecycle and version coexistence.
- Consumer migration/dependency evidence.
- Stable machine-readable technical errors.
- Operation-specific idempotency/deduplication.
- Logical-operation versus transport-attempt identity.

### Out of Scope / Non-Goals

- Business lifecycle/version ownership.
- Business compensation/recovery.
- Transaction/locking implementation — B02.
- Event schema compatibility — C02.
- Deployment mechanics — E04-005.
- Exact version encoding/idempotency retention technology.

---

## 3. Upstream Applicability & Constraints

| Source | Exact Reference | L5 Use |
| --- | --- | --- |
| Candidate Register 105 | `L5-C-01-003` | Establishes canonical responsibility. |
| `L4-A-029` | `BH-001, BH-003, BH-004, BH-006, BH-007, BH-008` | Drives successor contract and coexistence rules. |
| `L4-A-031` | `BH-001, BH-002, BH-004, BH-005, BH-006, BH-007` | Drives deprecation/retirement gates. |
| `L4-A-032` | `BH-001, BH-003, BH-004, BH-005, BH-006, BH-007, BH-008` | Drives consumer compatibility validation. |
| `ADR-L5-0006` | Owner-local transaction/concurrency strategy | Idempotency persistence executes inside owner-local consistency boundaries. |
| `ADR-L5-0008` | Expand-Migrate-Transition-Contract schema evolution pattern | Establishes compatible evolution sequencing applicable to persisted/contracted structures. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### Authoritative Constraints

- Prior governed versions/history shall not be silently rewritten.
- Compatibility includes behavior/meaning and required context, not schema shape alone.
- Retirement requires dependency/migration evidence.
- Idempotency shall never create authority to repeat or compensate a business effect.
- Idempotency state and result replay shall respect owner-local transaction/concurrency boundaries.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Contract change classification.
- Technical Draft/Released/Deprecated/Retired contract status.
- Compatibility validation obligations.
- Technical error taxonomy.
- Idempotency admission/deduplication/result replay semantics.
- Logical-operation identity across attempts.

### Consumes / Collaborates With

| Specification | Relationship | Boundary |
| --- | --- | --- |
| `L5-C-01-001` | Dependency | Contract catalogue and ownership metadata. |
| `L5-C-01-002` | Dependency | Synchronous execution applies these semantics. |
| `L5-B-02-003` | Dependency | Durable deduplication/conflict handling remain owner-local. |
| `L5-E-04-005` | Collaboration / consumer | E04-005 consumes compatibility/coexistence status for release and upgrade planning. C01-003 does **not** depend on E04-005. |

This removes the former circular dependency `C01-003 ↔ E04-005` while preserving the technical handoff direction.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Responsibility | Output / Handoff |
| --- | --- | --- |
| Compatibility Classifier | Classify proposed change as compatible, conditional or breaking. | Change classification. |
| Contract Lifecycle Registry | Track released/deprecated/retired versions and lineage. | Consumer-visible support state. |
| Idempotency Guard | Bind logical-operation identity to request fingerprint and prior outcome. | Execute once / replay prior result / reject mismatch. |
| Error Mapper | Map technical conditions to stable machine-readable categories. | Safe bounded technical error. |
| Consumer Dependency Inventory | Record contract-version use and migration evidence. | Retirement decision input. |

### 5.2 Contract Evolution Flow

```text
Proposed change
→ compatibility classification
→ compatible change may advance under governed release
→ breaking change creates successor
→ publish support/deprecation state
→ identify dependent consumers
→ migrate/verify consumers
→ retire predecessor only when governed evidence permits.
```

### 5.3 Idempotent Request Flow

```text
Request + logical-operation identity
→ validate identity and request fingerprint
→ no prior record: execute in owner-local transaction boundary and persist outcome identity
→ matching prior record: return governed prior outcome when allowed
→ same identity with different request fingerprint: reject integrity conflict.
```

### 5.4 Material Failure Model

| Failure | Required Outcome | Forbidden Outcome |
| --- | --- | --- |
| Same idempotency identity, different content | Reject mismatch/integrity conflict. | Guess caller intent or execute again. |
| Unsupported/retired version | Reject or use explicitly governed migration path. | Silent reinterpretation. |
| Breaking change misclassified | Block release/retirement. | Mutate prior contract in place. |
| Internal technical failure | Stable error category + safe message. | Leak stack/storage/secret internals. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis | ADR |
| --- | --- | --- | --- | --- |
| AD-001 | Evolution model | Additive compatible evolution is preferred; breaking semantic change creates a successor rather than rewriting the released predecessor. | L4-A-029/A031/A032 + `ADR-L5-0008` | `ADR-L5-0008` |
| AD-002 | Compatibility scope | Compatibility includes schema, behavior/meaning, required context and consumer obligations. | A029/A032 | Local architecture decision |
| AD-003 | Idempotency | Idempotency is operation-specific; same logical identity with different request content is rejected. | C01-002 + `ADR-L5-0006` | Local architecture decision |
| AD-004 | Error model | Stable machine-readable technical categories; human-readable message is non-authoritative. | Security/operability | None |
| AD-005 | Retirement | No universal support duration is invented; dependency/migration evidence gates retirement. | A031 | None |

Named API-version encodings, schema/contract tooling and idempotency storage products remain technology selections, not hidden architecture facts.

---

## 7. Architecture Conformance Requirements

| Requirement ID | Requirement |
| --- | --- |
| CR-001 | A released contract shall not be silently rewritten when the change is breaking in schema, behavior, meaning or required context. |
| CR-002 | Deprecated contracts shall not retire until governed dependency/migration evidence satisfies retirement conditions. |
| CR-003 | Same logical idempotency identity with materially different request content shall be rejected. |
| CR-004 | Technical errors shall expose stable categories without requiring human-text parsing or protected-internal leakage. |
| CR-005 | E04 release/upgrade architecture shall consume C01 compatibility/coexistence status; C01 shall not depend on E04 deployment mechanics. |

---

## 8. Open Decisions & Deferred Design

| Item ID | Item | Classification | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- |
| OD-001 | **Resolved wire convention:** breaking majors use explicit `/api/vN/...` routing and released OpenAPI 3.2 is contract authority. Concrete code-generation/lint tooling remains F01/F02 implementation tooling. | ADR / tooling | Locked wire convention; tooling non-blocking | `ADR-L5-0034`. |
| OD-002 | Idempotency retention duration/storage technology | B02/E-family deployment decision | No | Define by approved data/operational profile; no invented numeric value here. |
| OD-003 | Material L4 parents remain Draft | Upstream | Yes for Architecture Approved | Resolve through L4 governance. |

---

## 9. Traceability Summary

### Primary L4 Applicability

- `L4-A-029-BH-001/BH-003/BH-004/BH-006/BH-007/BH-008`
- `L4-A-031-BH-001/BH-002/BH-004/BH-005/BH-006/BH-007`
- `L4-A-032-BH-001/BH-003/BH-004/BH-005/BH-006/BH-007/BH-008`

### Material Dependencies

`L5-C-01-001`, `L5-C-01-002`, `L5-B-02-003`, `L4-A-029`, `L4-A-031`, `L4-A-032`

### Material Collaborator / Consumer

`L5-E-04-005`

### Applicable Approved ADRs

`ADR-L5-0006`, `ADR-L5-0008`


## 2026-09-22 Backend Readiness Group 1 Synchronization

**Applicable locked decision:** `ADR-L5-0034`  
Breaking contract generations use URL major versioning; additive compatible changes stay within the major version. Technical errors use RFC 9457 Problem Details plus governed SutraERP extensions. Retry-sensitive mutating commands use `Idempotency-Key`; the same key with materially different content is rejected.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
