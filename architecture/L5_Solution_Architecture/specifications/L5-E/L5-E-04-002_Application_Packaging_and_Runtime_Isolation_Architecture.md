# Application Packaging & Runtime Isolation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-04-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** Application packaging, runtime isolation and service-boundary realization without mandating Kubernetes.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-E-04-001`, `L5-A-01-001`, `L5-A-03-001`, `L5-C-01-*`, `L5-C-02-*`, `L5-D-03-005`, `L5-B-02-*`, `L4-A-028`, `L4-A-029`, `L4-A-032`.  
**Referenced By:** `L5-E-04-003/004/005`, E01/E05/F01.  
**Applicable ADRs:** Required but unresolved — Packaging/container standard and runtime isolation/orchestration strategy. No canonical ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define how SutraERP application capabilities are composed into deployable runtime units and when isolation is justified, without turning every domain/module into a service.

### In Scope

- Deployable artifact composition.
- Runtime unit boundary and isolation criteria.
- Independent scale/restart/replace semantics.
- Co-location versus isolation constraints.
- Logical service addressing/discovery.
- Runtime-local state restrictions and lifecycle behavior.

### Out of Scope / Non-Goals

- Business module lifecycle.
- API/event semantics.
- Orchestrator product selection.
- Network trust policy internals.
- Persistence topology.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
| --- | --- | --- | --- | --- | --- |
| L5 Governance | Candidate Register 105 | v1.1.0 / Architecture Approved | `L5-E-04-002` row | Application packaging, runtime isolation and service-boundary realization without mandating Kubernetes. | Establishes canonical responsibility. |
| L4 | `L4-A-028` | v1.0.0 / Draft | `BH-003, BH-007, BH-008, BH-009` | Module/version/capability governance is independent of runtime deployment. | Packaging cannot define business/module ownership or activation. |
| L4 | `L4-A-029` | v1.0.0 / Draft | `BH-001, BH-003, BH-004, BH-006, BH-007` | Successor/coexistence requires governed compatibility and lineage. | Packaging may support compatible side-by-side runtime. |
| L4 | `L4-A-032` | v1.0.0 / Draft | `BH-001, BH-003, BH-004, BH-005, BH-006` | Dependencies require governed compatibility and graph integrity. | Isolation shall not bypass dependency architecture. |
| L5 Peer | See Section 4 | Draft/planned as identified | Exact peer IDs below | Peer-owned technical capabilities. | Consumed without ownership transfer. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-001 | Business domain/module is not automatically a deployment unit. | A01/A028 | Co-located modular packaging is valid. |
| C-002 | Isolation requires a technical justification. | A01 | Separate runtime only for security/scaling/resilience/workload/operations/integration reasons. |
| C-003 | Isolated runtime does not create a new business owner. | L1/A028 | Ownership unchanged. |
| C-004 | Process/node-local filesystem is not canonical business persistence absent B02/B04 authority. | B02/B04 | Runtime units remain replaceable. |

### 3.3 Assumptions

All product/framework/runtime choices discussed during authoring remain **preferred candidates only** unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO, scaling or similar target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Package composition.
- Isolation criteria/runtime-unit identity.
- Independent restart/scale/replace characteristics.
- Logical addressing requirements.
- Runtime-local state constraints.
- Graceful startup/shutdown/health behavior.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary |
| --- | --- | --- |
| `L5-E-04-001` | Baseline profiles | E04-002 specializes packaging. |
| `L5-A-01-001 / L5-A-03-001` | Composition/dependencies | Packaging respects approved modular graph. |
| `L5-C-01-* / L5-C-02-*` | Cross-runtime communication | E04 provides placement, not communication semantics. |
| `L5-D-03-005` | Trust segmentation | Isolation may realize security boundary. |
| `L5-B-02-*` | Persistence | Local runtime state cannot replace authoritative persistence. |

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Modular Core Runtime | Runtime unit | Primary co-located runtime preserving enforceable module boundaries. | Governed context/configuration as applicable | Owned technical state only | Hosted application. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Isolated Service/Worker Runtime | Runtime unit | Separate execution only for justified technical need. | Governed context/configuration as applicable | Owned technical state only | Independent runtime. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Edge/Adapter Runtime | Runtime unit | Controlled connector/adapter placement where required. | Governed context/configuration as applicable | Owned technical state only | Integration execution. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Service Discovery/Addressing | Control point | Resolve isolated units by logical identity, not code-hardcoded machine address. | Governed context/configuration as applicable | Owned technical state only | Runtime endpoint. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Lifecycle Controller | Operational control | Validate startup/readiness and graceful shutdown/scale-down. | Governed context/configuration as applicable | Owned technical state only | Safe instance lifecycle. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |

### 5.2 Primary Technical Flow

```text
Approved application/module composition → evaluate isolation need → package into modular core or justified isolated unit → bind contracts/dependencies → deploy instances → resolve by logical service identity → health/lifecycle control.
```

The flow is authoritative at architecture level only. Exact endpoint names, code classes, manifests, product configuration and numeric policies remain downstream implementation concerns unless later governed by ADR/technical standard.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Unjustified isolation | Architecture review | Reject separate unit. | Do not create service solely from domain naming. | A01 |
| Hard-coded machine coupling | Deployment validation | Reject non-portable configuration. | No endpoint in code. | E04 |
| Node-local canonical state | Runtime review | Move authoritative state to B02/B04 boundary. | No correctness tied to one instance. | B02/B04 |
| Unsafe shutdown/scale-down | Lifecycle control | Withdraw readiness and drain governed in-flight work. | No abrupt loss of accepted work. | E01 |

### 5.4 State, Security, Runtime & Peer Boundaries

State mutation, persistence, transaction/concurrency, identity, authorization, cryptography, secrets, resilience, observability and deployment concerns are owned by the exact peer L5 specifications identified in Section 4. This specification constrains only the boundary it canonically owns and shall not duplicate those peer architectures.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
| --- | --- | --- | --- | --- | --- |
| AD-001 | Packaging topology | Modular core default; selective isolation only by technical need. | A01/A028 | Avoids distributed complexity. | Required |
| AD-002 | Isolated unit behavior | Independently restartable/scalable and communicates via governed contracts. | A01/C01/C02 | Failure/scale containment. | Required |
| AD-003 | Package format candidate | OCI-compatible image preferred; non-container on-prem profile allowed. | Working decision | Portability. | Required |
| AD-004 | Orchestration position | Kubernetes-compatible for larger/HA profile; not universal dependency. | Working decision + corpus boundary | Balances operability and small-site simplicity. | Required |

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
| CR-001 | A domain/module shall not receive a separate runtime unit solely because it is a domain/module. | AD-001 | A01/A028 |
| CR-002 | Every isolated unit shall document its technical justification and governed communication boundary. | AD-001/002 | A01/C01/C02 |
| CR-003 | Isolated units shall use logical service addressing rather than code-hardcoded machine endpoints. | §5 | Portability |
| CR-004 | Runtime-local state shall not become canonical business persistence absent canonical owner approval. | C-004 | B02/B04 |
| CR-005 | Startup/readiness/shutdown/scale-down shall protect mandatory dependencies and accepted in-flight work. | §5 | E01/E05 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- | --- |
| OD-001 | Package/container standard | ADR | No approved canonical resolution / source remains pending | E04/Lead Architect | Yes | Evaluate OCI/container alternatives. |
| OD-002 | Orchestration platform | ADR | No approved canonical resolution / source remains pending | E04/Lead Architect | Yes | Evaluate Kubernetes and alternatives. |
| OD-003 | Material L4 parents remain Draft | Upstream | No approved canonical resolution / source remains pending | L4 governance | Yes | Resolve before Architecture Approved. |

These blockers prevent `Architecture Approved` status where applicable, but they do not prevent controlled Draft authoring or repository submission under the broad-authoring authorization.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-028-BH-003/BH-007/BH-008/BH-009` — Packaging cannot define business/module ownership or activation.
- `L4-A-029-BH-001/BH-003/BH-004/BH-006/BH-007` — Packaging may support compatible side-by-side runtime.
- `L4-A-032-BH-001/BH-003/BH-004/BH-005/BH-006` — Isolation shall not bypass dependency architecture.

### Material L5 Dependencies

`L5-E-04-001`, `L5-A-01-001`, `L5-A-03-001`, `L5-C-01-*`, `L5-C-02-*`, `L5-D-03-005`, `L5-B-02-*`, `L4-A-028`, `L4-A-029`, `L4-A-032`

### Architecture Decisions

`AD-001`–`AD-004`

### Conformance Requirements

`CR-001`–`CR-005`

### Open / Blocking Items

`OD-001`–`OD-003`
