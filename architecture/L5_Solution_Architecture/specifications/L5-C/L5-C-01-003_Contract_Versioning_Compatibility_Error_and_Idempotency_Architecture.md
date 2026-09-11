# Contract Versioning, Compatibility, Error & Idempotency Architecture

## 1. Specification Metadata

**Specification ID:** `L5-C-01-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-01 — API & Contract Architecture  
**Canonical Technical Responsibility:** Contract evolution, compatibility, technical errors and idempotent request handling without inventing business recovery.  
**Technical Architecture Owner:** L5-C-01 — API & Contract Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-C-01-001`, `L5-C-01-002`, `L5-B-02-003`, `L5-E-04-005`, `L4-A-029`, `L4-A-031`, `L4-A-032`.  
**Referenced By:** API consumers, `L5-C-03-*`, `L5-E-04-005`.  
**Applicable ADRs:** Required but unresolved — Shared contract-versioning/compatibility/idempotency standard and selected contract tooling. No canonical ADR ID is currently allocated.

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

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
| --- | --- | --- | --- | --- | --- |
| L5 Governance | Candidate Register 105 | v1.1.0 / Architecture Approved | `L5-C-01-003` row | Contract evolution, compatibility, technical errors and idempotent request handling without inventing business recovery. | Establishes canonical responsibility. |
| L4 | `L4-A-029` | v1.0.0 / Draft | `BH-001, BH-003, BH-004, BH-006, BH-007, BH-008` | Successor versions are additive/traceable and compatibility governs activation/coexistence. | Drives successor contract and coexistence rules. |
| L4 | `L4-A-031` | v1.0.0 / Draft | `BH-001, BH-002, BH-004, BH-005, BH-006, BH-007` | Deprecation/retirement requires successor, dependency/migration evidence and preserved history. | Drives contract deprecation/retirement gates. |
| L4 | `L4-A-032` | v1.0.0 / Draft | `BH-001, BH-003, BH-004, BH-005, BH-006, BH-007, BH-008` | Dependency changes require version compatibility and governed approval. | Drives consumer compatibility validation. |
| L5 Peer | See Section 4 | Draft/planned as identified | Exact peer IDs below | Peer-owned technical capabilities. | Consumed without ownership transfer. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-001 | Prior governed versions/history shall not be silently rewritten. | A029-BH-001; A031-BH-007 | Breaking evolution creates successor. |
| C-002 | Compatibility includes semantics/behavior, not schema shape alone. | A029-BH-003/004; A032-BH-003 | Semantic break can be breaking despite additive schema. |
| C-003 | Retirement waits for dependency/migration evidence. | A031-BH-004 | Consumers are migrated/dispositioned first. |
| C-004 | Promised idempotency prevents repeat technical submission from repeating one logical effect. | C01-002 + B02 | Transport retries map to one logical operation. |

### 3.3 Assumptions

All product/framework/runtime choices discussed during authoring remain **preferred candidates only** unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO, scaling or similar target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Contract change classification.
- Technical Draft/Released/Deprecated/Retired status.
- Compatibility validation obligations.
- Technical error taxonomy.
- Idempotency admission/deduplication/result replay semantics.
- Logical-operation identity across attempts.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary |
| --- | --- | --- |
| `L5-C-01-001` | Contract catalogue | Lifecycle/version metadata. |
| `L5-C-01-002` | Synchronous execution | Errors/idempotency applied during execution. |
| `L5-B-02-003` | Transactions/concurrency | Durable deduplication and conflict behavior. |
| `L5-E-04-005` | Release/upgrade | Consumes compatibility/coexistence status. |

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Compatibility Classifier | Control point | Classify proposed change as compatible/conditional/breaking. | Governed context/configuration as applicable | Owned technical state only | Change classification. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Contract Lifecycle Registry | Registry | Track released/deprecated/retired supported versions and lineage. | Governed context/configuration as applicable | Owned technical state only | Consumer-visible support state. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Idempotency Guard | Control point | Bind logical operation identity to request identity and prior outcome. | Governed context/configuration as applicable | Owned technical state only | Execute once / return prior outcome / reject mismatch. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Error Mapper | Contract boundary | Map technical conditions to stable machine-readable categories. | Governed context/configuration as applicable | Owned technical state only | Bounded error response. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Consumer Dependency Inventory | Registry/evidence | Record supported contract versions and migration evidence. | Governed context/configuration as applicable | Owned technical state only | Retirement decision input. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |

### 5.2 Primary Technical Flow

```text
Proposed contract change → classify compatibility → breaking change creates successor → publish support/deprecation status → inventory dependent consumers → migrate/verify consumers → retire only when governed evidence allows.
```

The flow is authoritative at architecture level only. Exact endpoint names, code classes, manifests, product configuration and numeric policies remain downstream implementation concerns unless later governed by ADR/technical standard.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Same idempotency identity, different content | Idempotency Guard | Reject mismatch. | Do not guess caller intent. | Integrity |
| Unsupported/retired version | Contract boundary | Reject or governed migration path. | Do not silently reinterpret. | A031 |
| Breaking change misclassified | Review/compatibility gate | Block release/retirement. | Do not mutate prior contract. | A029/A032 |
| Technical internal failure | Error Mapper | Stable category + safe message. | No sensitive internals. | D03 |

### 5.4 State, Security, Runtime & Peer Boundaries

State mutation, persistence, transaction/concurrency, identity, authorization, cryptography, secrets, resilience, observability and deployment concerns are owned by the exact peer L5 specifications identified in Section 4. This specification constrains only the boundary it canonically owns and shall not duplicate those peer architectures.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
| --- | --- | --- | --- | --- | --- |
| AD-001 | Evolution model | Additive compatibility preferred; breaking changes create successor. | A029/A031/A032 | Preserves consumers/history. | Required |
| AD-002 | Compatibility scope | Behavioral meaning/context are part of compatibility. | A029/A032 | Schema-only validation is insufficient. | Required |
| AD-003 | Idempotency | Operation-specific deduplication; same key/different content rejects. | C01-002/B02 | Prevents repeat effects. | Required |
| AD-004 | Error model | Stable machine-readable technical categories; human message non-authoritative. | Security/operability | Automatable and non-leaky. | None |
| AD-005 | Retirement | No universal support duration; dependency/migration evidence gates retirement. | A031 | Avoids invented dates/orphan consumers. | None |

Any row marked `Required` remains provisional while no canonical ADR ID is allocated on `main`.

---

## 7. Security, Privacy, Quality & Operational Impact

- Tenant/Institution scope and canonical ownership shall remain preserved across this specification's technical boundaries.
- Authentication/authorization, secrets/cryptography and trust remain D-family owned where applicable.
- Operational health, resilience, performance/capacity and observability are consumed from E-family owners rather than redefined here.
- No unavailable numeric target is fabricated; deployment/profile-specific targets remain deferred to their canonical owners.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
| --- | --- | --- | --- |
| CR-001 | Breaking contract changes shall create governed successors rather than silently mutate supported prior contracts. | AD-001 | A029 |
| CR-002 | Compatibility validation shall include behavioral semantics and required context. | AD-002 | A029/A032 |
| CR-003 | Deprecated contracts shall not retire until dependency/migration evidence satisfies governed conditions. | AD-005 | A031 |
| CR-004 | For idempotent operations, repeat submission of the same logical request shall not repeat the authoritative effect. | AD-003 | C01-002/B02 |
| CR-005 | Same idempotency identity with materially different content shall be rejected. | AD-003 | Integrity |
| CR-006 | Technical errors shall expose stable categories without requiring human-text parsing or protected-internal leakage. | AD-004 | Security/operability |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- | --- |
| OD-001 | Contract-versioning/compatibility standard | ADR | No approved canonical resolution / source remains pending | Lead Architect / C01 | Yes | Allocate/evaluate ADR. |
| OD-002 | Concrete API version encoding/tooling | ADR | No approved canonical resolution / source remains pending | C01 | Yes | Resolve after API style decision. |
| OD-003 | Idempotency retention/storage implementation | Dependency | No approved canonical resolution / source remains pending | B02/C01/E01 | No | Resolve by implementation profile. |
| OD-004 | Material L4 parents remain Draft | Upstream | No approved canonical resolution / source remains pending | L4 governance | Yes | Resolve before Architecture Approved. |

These blockers prevent `Architecture Approved` status where applicable, but they do not prevent controlled Draft authoring or repository submission under the broad-authoring authorization.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-029-BH-001/BH-003/BH-004/BH-006/BH-007/BH-008` — Drives successor contract and coexistence rules.
- `L4-A-031-BH-001/BH-002/BH-004/BH-005/BH-006/BH-007` — Drives contract deprecation/retirement gates.
- `L4-A-032-BH-001/BH-003/BH-004/BH-005/BH-006/BH-007/BH-008` — Drives consumer compatibility validation.

### Material L5 Dependencies

`L5-C-01-001`, `L5-C-01-002`, `L5-B-02-003`, `L5-E-04-005`, `L4-A-029`, `L4-A-031`, `L4-A-032`

### Architecture Decisions

`AD-001`–`AD-005`

### Conformance Requirements

`CR-001`–`CR-006`

### Open / Blocking Items

`OD-001`–`OD-004`
