# Synchronous Service Contract Architecture

## 1. Specification Metadata

**Specification ID:** `L5-C-01-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-01 — API & Contract Architecture  
**Canonical Technical Responsibility:** Authoritative synchronous interaction, request/response and service-consumption architecture.  
**Technical Architecture Owner:** L5-C-01 — API & Contract Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-C-01-001`, `L5-A-04-002`, `L5-B-02-003`, `L5-D-02-*`, `L5-E-01-*`, `L5-E-05-*`, `L4-A-015`, `L4-A-050`.  
**Referenced By:** `L5-C-01-003`, `L5-C-03-*`.  
**Applicable ADRs:** Required but unresolved — Repository-wide synchronous interaction strategy and selected transport/runtime technology. No canonical ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define synchronous request admission, authoritative provider execution, bounded failure/overload semantics, deadlines and reconciliation without propagating business authority or transactions across owners.

### In Scope

- Ordered request admission.
- Provider-owned request/response execution.
- Deadline/timeout semantics without numeric values.
- Overload/backpressure and dependency-failure handling.
- Unknown-outcome reconciliation.
- Retry ownership and call-correlation rules.

### Out of Scope / Non-Goals

- Contract lifecycle/idempotency details — C01-003.
- Persistence transactions — B02-003.
- Authorization policy — D02.
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
| L5 Peer | See Section 4 | Draft/planned as identified | Exact peer IDs below | Peer-owned technical capabilities. | Consumed without ownership transfer. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-001 | Authoritative outcome comes from the owning capability. | L4-A-015-BH-004/005 | Consumer cannot synthesize provider result. |
| C-002 | Caller transaction shall not cross an owner boundary. | L5-B-02-003 | Target opens owner-local transaction. |
| C-003 | Missing scope/authority context shall not be defaulted. | A04/A050 | Fail closed. |
| C-004 | Timeout/disconnect is not proof of non-execution. | Defensible distributed-systems derivation | Unknown outcome remains explicit until reconciled. |

### 3.3 Assumptions

All product/framework/runtime choices discussed during authoring remain **preferred candidates only** unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO, scaling or similar target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Request admission/execution sequence.
- Success/business rejection/technical rejection/conflict/overload/unknown outcome classification.
- Deadline budget propagation semantics.
- Backpressure requirements.
- Reconciliation requirement for ambiguous outcomes.
- Nested-call/retry coordination and correlation.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary |
| --- | --- | --- |
| `L5-C-01-001` | Released contract/context | Execution happens inside that contract. |
| `L5-C-01-003` | Errors/idempotency/versioning | Safe replay and error semantics. |
| `L5-B-02-003` | Owner-local transaction/concurrency | Provider consistency boundary. |
| `L5-D-02-*` | Authorization enforcement | Before protected mutation. |
| `L5-E-01-* / L5-E-05-*` | Resilience and telemetry | Failure containment/operability. |

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Request Admission Gate | Control point | Structural, identity/scope and authorization admission before protected execution. | Governed context/configuration as applicable | Owned technical state only | Admitted request or rejection. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Provider Execution Boundary | Service boundary | Invoke owner capability and owner-local processing. | Governed context/configuration as applicable | Owned technical state only | Authoritative result. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Deadline Budget | Execution context | Constrain downstream synchronous work without fixed numbers. | Governed context/configuration as applicable | Owned technical state only | Remaining budget. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Overload Gate | Control point | Bound finite capacity before partial effect. | Governed context/configuration as applicable | Owned technical state only | Admit/reject/throttle. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Reconciliation Boundary | Contract capability | Resolve timeout/disconnect ambiguity where material. | Governed context/configuration as applicable | Owned technical state only | Authoritative status/result. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |

### 5.2 Primary Technical Flow

```text
Receive released request → structural validation → identity/Tenant/Institution validation → authorization enforcement → owner business preconditions → owner-local transaction/processing → explicit success/rejection/conflict/overload/unknown outcome → correlation evidence.
```

The flow is authoritative at architecture level only. Exact endpoint names, code classes, manifests, product configuration and numeric policies remain downstream implementation concerns unless later governed by ADR/technical standard.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Invalid request/context | Admission | Reject before side effect. | No partial mutation. | Caller/A04 |
| Authorization denied | Provider enforcement | Protected rejection. | No mutation before authorization. | D02 |
| Concurrency conflict | Owner transaction | Explicit conflict. | No silent overwrite. | B02-003 |
| Deadline exhausted | Call boundary | Return timeout/unknown as appropriate. | Do not claim non-execution without evidence. | Reconciliation |
| Overload | Admission | Reject/throttle before partial effect. | No unbounded synchronous queue. | E01/E05 |

### 5.4 State, Security, Runtime & Peer Boundaries

State mutation, persistence, transaction/concurrency, identity, authorization, cryptography, secrets, resilience, observability and deployment concerns are owned by the exact peer L5 specifications identified in Section 4. This specification constrains only the boundary it canonically owns and shall not duplicate those peer architectures.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
| --- | --- | --- | --- | --- | --- |
| AD-001 | Validation order | Structural → identity/scope → authorization → business preconditions → mutation. | A015/A050/D02 | Prevents invalid/unauthorized side effects. | None |
| AD-002 | Transaction boundary | Caller transaction does not propagate across owner boundary. | B02-003 | Preserves persistence ownership. | None |
| AD-003 | Timeout semantics | Timeout may be unknown outcome; reconciliation where material. | Distributed execution derivation | Prevents duplicate/contradictory action. | Required |
| AD-004 | Backpressure | Finite admission and explicit overload; no unbounded sync queues. | Operational correctness | Protects system under load. | Required |
| AD-005 | Retry ownership | Retry only when proven safe/idempotent; avoid multi-layer amplification. | C01-003/E01 | Prevents duplicate effects. | Required |

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
| CR-001 | Structural/context/authorization admission shall complete before protected mutation. | AD-001 | A015/A050 |
| CR-002 | Cross-owner synchronous calls shall execute in target owner transaction boundary, not caller transaction. | AD-002 | B02-003 |
| CR-003 | Timeout shall not be represented as definitive non-execution absent authoritative evidence. | AD-003 | Distributed correctness |
| CR-004 | Ambiguous operations shall support governed reconciliation where duplicate re-execution would be unsafe. | §5 | C01-003 |
| CR-005 | Synchronous admission shall not use unbounded queues; overload shall be explicit before partial effect. | AD-004 | E01/E05 |
| CR-006 | Retry ownership shall prevent independent amplification across layers. | AD-005 | C01-003/E01 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- | --- |
| OD-001 | Repository synchronous interaction strategy | ADR | No approved canonical resolution / source remains pending | Lead Architect / C01 | Yes | Allocate/evaluate ADR. |
| OD-002 | Exact transport/runtime implementation | ADR | No approved canonical resolution / source remains pending | Lead Architect | Yes | Resolve after C01-001 style decision. |
| OD-003 | Numeric deadline/retry/overload targets | Dependency | No approved canonical resolution / source remains pending | E01/E02 owners | No | Define by deployment profile. |
| OD-004 | Material L4 parents remain Draft | Upstream | No approved canonical resolution / source remains pending | L4 governance | Yes | Resolve before Architecture Approved. |

These blockers prevent `Architecture Approved` status where applicable, but they do not prevent controlled Draft authoring or repository submission under the broad-authoring authorization.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-015-BH-001/BH-004/BH-005/BH-006/BH-007/BH-008/BH-009/BH-010/BH-011` — Defines synchronous execution and outcome boundaries.
- `L4-A-050-BH-001/BH-003/BH-006/BH-008` — Requires scope revalidation at call boundaries.

### Material L5 Dependencies

`L5-C-01-001`, `L5-A-04-002`, `L5-B-02-003`, `L5-D-02-*`, `L5-E-01-*`, `L5-E-05-*`, `L4-A-015`, `L4-A-050`

### Architecture Decisions

`AD-001`–`AD-005`

### Conformance Requirements

`CR-001`–`CR-006`

### Open / Blocking Items

`OD-001`–`OD-004`
