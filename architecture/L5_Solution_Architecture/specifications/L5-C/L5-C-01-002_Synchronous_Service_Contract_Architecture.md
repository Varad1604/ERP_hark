# Synchronous Service Contract Architecture

## 1. Specification Metadata

**Specification ID:** `L5-C-01-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-01 — API & Contract Architecture  
**Canonical Technical Responsibility:** Authoritative synchronous interaction, request/response and service-consumption architecture.  
**Technical Architecture Owner:** L5-C-01 — API & Contract Architecture  
**Assigned Author:** Pallav  
**Version:** `1.3.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-C-01-001`, `L5-A-04-002`, `L5-B-02-003`, `L4-A-015`, `L4-A-050`.  
**Collaborates With / Referenced By:** `L5-C-01-003`, `L5-D-02-*`, `L5-E-01-*`, `L5-E-05-*`, `L5-C-03-*`.  
**Applicable ADRs:** `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0019`; `ADR-L5-0021`; `ADR-L5-0034`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define synchronous request admission, authoritative provider execution, bounded failure/overload semantics, deadlines and reconciliation without propagating business authority or transactions across owners.

### In Scope

- Ordered request admission.
- Provider-owned request/response execution.
- Deadline/timeout semantics without invented numeric values.
- Overload/backpressure and dependency-failure handling.
- Unknown-outcome reconciliation.
- Retry ownership and call-correlation rules.

### Out of Scope / Non-Goals

- Contract lifecycle/idempotency details — C01-003.
- Persistence transactions — B02-003.
- Authorization-policy internals — D02.
- Resilience products/numeric thresholds — E01/E05.
- Business compensation/cancellation rules.
- Messaging — C02.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
| --- | --- | --- | --- | --- | --- |
| L5 Governance | Candidate Register 105 | v1.1.0 / Architecture Approved | `L5-C-01-002` row | Authoritative synchronous interaction, request/response and service-consumption architecture. | Establishes canonical responsibility. |
| L4 | `L4-A-015` | v1.0.0 / Draft | `BH-001, BH-004, BH-005, BH-006, BH-007, BH-008, BH-009, BH-010, BH-011` | Valid initiation, owner-authoritative calls, wait/suspend/completion/cancel/failure and evidence. | Defines synchronous execution and outcome boundaries. |
| L4 | `L4-A-050` | v1.0.0 / Draft | `BH-001, BH-003, BH-006, BH-008` | Scope context is resolved/revalidated separately from authorization. | Requires scope revalidation at call boundaries. |
| L5 ADR | `ADR-L5-0003` | Architecture Approved | Contract-Driven Modular Platform with Selective Isolation | Business-domain identity alone does not require a remote boundary; governed contracts survive placement choices. | Keeps synchronous contract semantics independent of deployment placement. |
| L5 ADR | `ADR-L5-0004` | Architecture Approved | Tenant and Institution Isolation Enforcement Pattern | Tenant/Institution isolation must be technically enforced. | Requires fail-closed scope handling. |
| L5 ADR | `ADR-L5-0006` | Architecture Approved | Owner-Local Transaction and Optimistic Concurrency Strategy | Transaction/concurrency remain owner-local. | Prohibits cross-owner transaction propagation. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-001 | Authoritative outcome comes from the owning capability. | L4-A-015-BH-004/005 | Consumer cannot synthesize provider result. |
| C-002 | Caller transaction shall not cross an owner boundary. | `ADR-L5-0006`; L5-B-02-003 | Target executes inside owner-local transaction/concurrency boundary. |
| C-003 | Missing scope/authority context shall not be defaulted. | `ADR-L5-0004`; A04/A050 | Fail closed. |
| C-004 | Placement choice shall not alter contract ownership or semantics. | `ADR-L5-0003` | In-process and selectively isolated realizations preserve the same logical contract. |
| C-005 | Timeout/disconnect is not proof of non-execution. | Distributed execution correctness | Unknown outcome remains explicit until reconciled where material. |

### 3.3 Assumptions

All product/framework/runtime choices remain candidates unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO or scaling target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Request admission/execution sequence.
- Success/business rejection/technical rejection/conflict/overload/unknown-outcome classification.
- Deadline-budget propagation semantics.
- Backpressure requirements at the synchronous contract boundary.
- Reconciliation requirement for ambiguous outcomes.
- Nested-call/retry coordination and correlation.

### Consumes / Collaborates With

| Exact L5 Specification | Relationship | Boundary |
| --- | --- | --- |
| `L5-C-01-001` | Dependency | Released contract/context exists before execution. |
| `L5-C-01-003` | Collaboration | C01-003 supplies error/idempotency/version semantics; this file does not depend on its physical realization to define synchronous execution. |
| `L5-B-02-003` | Dependency | Provider consistency is owner-local. |
| `L5-D-02-*` | Collaboration | Authorization enforcement is delegated to D02; C01 owns only admission ordering and handoff. |
| `L5-E-01-*` | Collaboration | E01 may realize resilience mechanisms without owning C01 request semantics. |
| `L5-E-05-*` | Collaboration | E05 consumes correlation/operational evidence without owning C01 behavior. |

Later-stage collaborators are deliberately not declared as prerequisite dependencies. Their absence shall not prevent C01 from defining its owned contract semantics.

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Failure Boundary |
| --- | --- | --- | --- | --- | --- | --- |
| Request Admission Gate | Control point | Structural, identity/scope and authorization admission before protected execution. | Contract + execution context | No business state ownership | Admitted request or rejection. | Reject before protected side effect. |
| Provider Execution Boundary | Service boundary | Invoke the owning capability while preserving ownership. | Admitted request | Owner-local state only | Authoritative result. | Explicit provider rejection/failure. |
| Deadline Budget | Execution context | Bound synchronous work without fixed repository-wide numbers. | Remaining call budget | Ephemeral technical context | Remaining budget / exhausted outcome. | Exhaustion is explicit. |
| Overload Gate | Control point | Enforce finite admission and avoid unbounded synchronous work queues. | Current capacity signal | Technical admission state only | Admit/reject/throttle. | Reject before partial effect. |
| Reconciliation Boundary | Contract capability | Resolve timeout/disconnect ambiguity where the operation may have executed. | Correlation/idempotency evidence | No duplicated business authority | Authoritative execution status/result. | Unknown remains explicit until resolved. |

### 5.2 Primary Technical Flow

```text
Receive released request
→ structural validation
→ identity/Tenant/Institution validation
→ authorization handoff/enforcement
→ provider precondition evaluation
→ owner-local transaction/processing
→ explicit success/rejection/conflict/overload/unknown outcome
→ correlation/telemetry handoff.
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Invalid request/context | Admission | Reject before side effect. | Partial mutation. | Caller/A04 |
| Authorization denied | Authorization enforcement | Protected rejection. | Mutation before authorization. | D02 |
| Concurrency conflict | Owner transaction | Explicit conflict. | Silent overwrite. | B02-003 |
| Deadline exhausted/disconnect | Call boundary | Timeout or unknown outcome as evidence permits. | Claim non-execution without evidence. | C01 reconciliation; C01-003 idempotency semantics |
| Overload | Admission | Reject/throttle before partial effect. | Unbounded synchronous queue. | C01 admission; E01/E05 may supply mechanisms/evidence |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | ADR |
| --- | --- | --- | --- | --- |
| AD-001 | Validation order | Structural → identity/scope → authorization → business preconditions → mutation. | A015/A050/D02 boundary | None |
| AD-002 | Transaction boundary | Caller transaction does not propagate across owner boundary. | `ADR-L5-0006` | `ADR-L5-0006` |
| AD-003 | Placement independence | Synchronous contract semantics remain stable for modular-core and selectively isolated execution. | `ADR-L5-0003` | `ADR-L5-0003` |
| AD-004 | Scope isolation | Missing/inconsistent Tenant/Institution context fails closed. | `ADR-L5-0004` | `ADR-L5-0004` |
| AD-005 | Unknown outcome | Timeout/disconnect may produce an unknown outcome; reconciliation is required where duplicate/contradictory effects are material. | Distributed execution correctness + C01-003 handoff | Local architecture decision; exact transport strategy may require ADR |
| AD-006 | Backpressure | Synchronous admission is finite; overload is explicit and shall not rely on unbounded queues. | Operational correctness | Local architecture decision; product-specific mechanism deferred |
| AD-007 | Retry ownership | Automatic retry is allowed only when operation semantics and C01-003 idempotency evidence make replay safe; multi-layer retry amplification is prohibited. | C01-003 boundary | Local architecture decision; shared retry standard may require ADR |

---

## 7. Security, Privacy, Quality & Operational Impact

- Tenant/Institution scope and canonical ownership remain preserved across synchronous boundaries.
- Authentication/authorization, secrets/cryptography and trust remain D-family owned.
- E01/E05 may implement resilience/observability mechanisms but do not become prerequisites for C01-owned request semantics.
- No unavailable numeric target is fabricated.

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From |
| --- | --- | --- |
| CR-001 | Structural/context/authorization admission shall complete before protected mutation. | AD-001 |
| CR-002 | Cross-owner synchronous calls shall execute in the target owner's transaction boundary, not a caller-propagated transaction. | AD-002 / `ADR-L5-0006` |
| CR-003 | Placement changes shall not change canonical contract ownership or semantics. | AD-003 / `ADR-L5-0003` |
| CR-004 | Missing or inconsistent Tenant/Institution context shall fail closed. | AD-004 / `ADR-L5-0004` |
| CR-005 | Timeout/disconnect shall not be treated as proof that protected execution did not occur. | AD-005 |
| CR-006 | Synchronous admission shall be finite and overload explicit. | AD-006 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- |
| OD-001 | **Resolved:** HTTPS REST/JSON + OpenAPI 3.2; contract-first wire profile with URL-major versioning, RFC 9457 errors and `Idempotency-Key`. | ADR | C01 / Lead Architect | Locked | `ADR-L5-0021`; `ADR-L5-0034`. |
| OD-002 | **Resolved:** Rust + Axum + Tokio server realization; C01 contracts remain framework-independent. | ADR | `ADR-L5-0018`; `ADR-L5-0019` | Locked | No further selection required unless superseded. |
| OD-003 | Numeric deadline/retry/overload targets | Deployment/operations decision | E-family owners with C01 constraints | No | Define per approved deployment/service profile. |
| OD-004 | Material L4 parents remain Draft | Upstream | L4 governance | Yes for Architecture Approved | Resolve upstream approval; do not bypass from L5. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-015-BH-001/BH-004/BH-005/BH-006/BH-007/BH-008/BH-009/BH-010/BH-011` — synchronous execution and outcome boundaries.
- `L4-A-050-BH-001/BH-003/BH-006/BH-008` — scope revalidation at call boundaries.

### Material L5 Dependencies

`L5-C-01-001`, `L5-A-04-002`, `L5-B-02-003`, `L4-A-015`, `L4-A-050`

### Material Collaborators

`L5-C-01-003`, `L5-D-02-*`, `L5-E-01-*`, `L5-E-05-*`

### Applicable Approved ADRs

`ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0006`


## 2026-09-22 Backend Readiness Group 1 Synchronization

**Applicable locked decision:** `ADR-L5-0034`  
This specification inherits camelCase JSON, contract-first OpenAPI, `/api/v{major}` breaking-version routing, RFC 9457 Problem Details, selective `Idempotency-Key`, and REST/OpenAPI as the default synchronous interaction profile.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
