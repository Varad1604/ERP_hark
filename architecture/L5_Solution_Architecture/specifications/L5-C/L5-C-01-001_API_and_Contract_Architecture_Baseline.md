# API & Contract Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-C-01-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-01 — API & Contract Architecture  
**Canonical Technical Responsibility:** Technical API/contract principles and ownership-preserving synchronous boundaries.  
**Technical Architecture Owner:** L5-C-01 — API & Contract Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-A-01-001`, `L5-A-03-001`, `L5-A-04-002`, `L5-B-02-001/002/003`, `L5-D-01-001`, `L5-D-03-001`, `L4-A-001`, `L4-A-015`, `L4-A-028`, `L4-A-050`.  
**Referenced By:** `L5-C-01-002`, `L5-C-01-003`, `L5-C-02-*`, `L5-C-03-*`.  
**Applicable ADRs:** Required but unresolved — Repository-wide synchronous interaction/API style; primary runtime/framework; API-management technology. No canonical ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the reusable synchronous contract boundary through which SutraERP capabilities collaborate without transferring canonical business ownership, bypassing Tenant/Institution scope, or exposing persistence internals.

### In Scope

- API/contract ownership and publication boundary.
- Internal versus external contract classification.
- Conceptual request/response context for identity, Tenant/Institution scope, correlation and authority.
- Contract-first definition, catalogue and lifecycle principles.
- External API-management/ingress boundary at architecture level.
- Contract semantics independent of runtime placement.

### Out of Scope / Non-Goals

- Business rules/lifecycles/canonical events.
- Authentication/authorization/cryptography/secrets internals — L5-D.
- Persistence/transaction/concurrency — L5-B-02.
- Event/messaging — L5-C-02.
- Exact language/framework/gateway/protocol/version encoding.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
| --- | --- | --- | --- | --- | --- |
| L5 Governance | Candidate Register 105 | v1.1.0 / Architecture Approved | `L5-C-01-001` row | Technical API/contract principles and ownership-preserving synchronous boundaries. | Establishes canonical responsibility. |
| L4 | `L4-A-001` | v1.0.0 / Draft | `BH-003, BH-005, BH-009` | Tenant/platform context is established and handed to downstream consumers without ownership transfer. | Contracts carry validated scope context. |
| L4 | `L4-A-015` | v1.0.0 / Draft | `BH-001, BH-004, BH-005, BH-008, BH-010, BH-011` | Cross-capability execution validates context, obtains owner-authoritative results, returns governed outcomes and evidence. | Defines owner-preserving request/response boundaries. |
| L4 | `L4-A-028` | v1.0.0 / Draft | `BH-003, BH-007, BH-008, BH-009` | Capabilities/versions/dependencies are governed while runtime deployment is separate. | Contracts bind to governed capabilities, not physical placement. |
| L4 | `L4-A-050` | v1.0.0 / Draft | `BH-001, BH-003, BH-006, BH-008` | Tenant/Institution context is resolvable and revalidated without becoming authorization. | Contract carries scope but does not decide access. |
| L5 Peer | See Section 4 | Draft/planned as identified | Exact peer IDs below | Peer-owned technical capabilities. | Consumed without ownership transfer. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-001 | API use shall not transfer canonical business ownership. | L4-A-015-BH-004/005 | Provider remains authoritative. |
| C-002 | Required Tenant/Institution context remains explicit and validated. | L4-A-001-BH-003/009; L4-A-050-BH-001/003 | Missing/inconsistent scope fails closed. |
| C-003 | Contracts shall not expose or authorize another owner's persistence internals. | L5-B-02-001/002/003 | Requests target the owning capability. |
| C-004 | Runtime placement shall not change supported contract semantics. | L5-A-01-001; L4-A-028-BH-009 | Local and isolated realizations preserve the same logical boundary. |

### 3.3 Assumptions

All product/framework/runtime choices discussed during authoring remain **preferred candidates only** unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO, scaling or similar target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Synchronous Contract Boundary.
- Internal/external contract classification.
- Contract catalogue structure and ownership/discovery metadata; contract lifecycle/version semantics remain C01-003-owned.
- Conceptual request/response context categories.
- External API-management boundary.
- Compatibility handoff to C01-003.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary |
| --- | --- | --- |
| `L5-A-01-001` | Application composition/placement | C01 contract semantics survive placement changes. |
| `L5-A-04-002` | Tenant/Institution context semantics | C01 carries scope; A04 defines it. |
| `L5-B-02-001/002/003` | Persistence/transactions | Provider processes inside owner-local boundary. |
| `L5-D-01/02/03` | Identity, authorization, trust | C01 carries validated context; D-family enforces. |
| `L5-C-01-002` | Synchronous execution | C01-002 owns admission, execution and synchronous failure semantics. |
| `L5-C-01-003` | Contract lifecycle/evolution | C01-003 owns versioning, compatibility, technical lifecycle, errors and idempotency semantics. |

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

**Draft decision gate:** Architecture in this section that depends on an `AD-*` whose ADR column contains `Required`, or on a blocking `OD-*`, is a provisional realization for architecture evaluation only. It is not a canonical implementation mandate until the required governance decision/dependency is resolved. Architecture supported independently by cited repository authority remains applicable.

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Contract Definition | Architecture artifact | Canonical technical request/response definition for a governed capability. | Governed context/configuration as applicable | Owned technical state only | Released contract or rejection. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Contract Catalogue | Registry/catalogue | Discoverable ownership, version and lifecycle status. | Governed context/configuration as applicable | Owned technical state only | Consumer discovery. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Synchronous Contract Boundary | Control point | Preserve scope, ownership and technical context across invocation. | Governed context/configuration as applicable | Owned technical state only | Provider request / governed response. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| External API Management Boundary | Control point | Expose only released external contracts; apply technical ingress policy. | Governed context/configuration as applicable | Owned technical state only | Routed request or technical rejection. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Contract Telemetry Hook | Observability point | Emit correlation/contract technical evidence without business ownership. | Governed context/configuration as applicable | Owned technical state only | E05 telemetry handoff. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |

### 5.2 Primary Technical Flow

```text
Consumer → resolve released contract → establish validated identity/scope/correlation → external API-management boundary when applicable → owning capability → provider validation/authorization delegation → owner-local processing → governed response/rejection → telemetry.
```

The flow is the current Draft technical realization. Portions dependent on unresolved ADRs or blocking peer/upstream decisions remain provisional and non-normative. Exact endpoint names, code classes, manifests, product configuration and numeric policies remain downstream implementation concerns unless later governed by ADR/technical standard.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Missing/invalid context | Contract boundary | Reject before protected mutation. | Never infer Tenant/Institution or authority. | A04/D-family |
| Unreleased contract | Catalogue/ingress | Reject unless governed compatibility path exists. | Do not use hidden/internal interface. | C01-003 |
| Provider unavailable/overloaded | Runtime boundary | Return explicit technical failure/degradation. | Do not fabricate business success. | E01/E05 |
| Internal error | Provider boundary | Return bounded technical error. | Do not leak stack/storage/secret internals. | D03/E05 |

### 5.4 State, Security, Runtime & Peer Boundaries

State mutation, persistence, transaction/concurrency, identity, authorization, cryptography, secrets, resilience, observability and deployment concerns are owned by the exact peer L5 specifications identified in Section 4. This specification constrains only the boundary it canonically owns and shall not duplicate those peer architectures.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
| --- | --- | --- | --- | --- | --- |
| AD-001 | Contract model | **UNRESOLVED — candidate for ADR evaluation:** Contract-first, capability-oriented synchronous boundary. | A015/A028/B02 | Preserves ownership and avoids CRUD coupling. | Required |
| AD-002 | External exposure | **UNRESOLVED — candidate for ADR evaluation:** External APIs pass governed API-management boundary; gateway owns no business logic. | D03/C01 boundary | Centralizes technical ingress control. | Required |
| AD-003 | Placement independence | Contract semantics unchanged by in-process vs isolated placement. | A01/A028 | Prevents topology from redefining architecture. | None |
| AD-004 | Preferred API candidates | **UNRESOLVED — candidate for ADR evaluation:** REST/HTTP + OpenAPI default; gRPC/GraphQL only when justified. | Working decision only | Candidate, not canonical. | Required |
| AD-005 | Preferred backend candidate | **UNRESOLVED — candidate for ADR evaluation:** Java LTS + Spring Boot; modular core. | Working decision only | Candidate, not canonical. | Required |

Any row whose ADR column contains `Required` is an **unresolved candidate position, not normative architecture**. It shall not become a conformance obligation until the required ADR is canonically allocated, evaluated and approved. Non-ADR rows remain valid only to the extent supported by the cited authority.

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
| CR-001 | Cross-owner synchronous integration shall invoke the owning capability through a governed boundary and shall not directly modify another owner's persistence/internal state. | §4–§5 | A015-BH-004/005; B02-001/002/003 |
| CR-002 | Protected requests shall carry validated Tenant/Institution scope where required and fail closed if missing/inconsistent. | §5 | A001; A050 |
| CR-003 | Contract semantics shall remain independent of local versus isolated runtime placement. | AD-003 | A028; A01 |
| CR-004 | Contract catalogue metadata shall identify ownership/status/version without persistence/framework internals. | §5 | A028/C01-003 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- | --- |
| OD-001 | Synchronous interaction/API style | ADR | No approved canonical resolution / source remains pending | Lead Architect / C01 | Yes | Allocate and evaluate ADR. |
| OD-002 | Primary runtime/language/framework | ADR | No approved canonical resolution / source remains pending | Lead Architect | Yes | Evaluate Java/Spring and alternatives. |
| OD-003 | API-management product | ADR | No approved canonical resolution / source remains pending | C01/D03/E04 + approver | Yes | Evaluate mature gateway alternatives. |
| OD-004 | Material L4 parents remain Draft | Upstream | No approved canonical resolution / source remains pending | L4 governance | Yes | Resolve upstream approval before Architecture Approved. |

These blockers prevent `Architecture Approved` status where applicable, but they do not prevent controlled Draft authoring or repository submission under the broad-authoring authorization.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-003/BH-005/BH-009` — Contracts carry validated scope context.
- `L4-A-015-BH-001/BH-004/BH-005/BH-008/BH-010/BH-011` — Defines owner-preserving request/response boundaries.
- `L4-A-028-BH-003/BH-007/BH-008/BH-009` — Contracts bind to governed capabilities, not physical placement.
- `L4-A-050-BH-001/BH-003/BH-006/BH-008` — Contract carries scope but does not decide access.

### Material L5 Dependencies

`L5-A-01-001`, `L5-A-03-001`, `L5-A-04-002`, `L5-B-02-001/002/003`, `L5-D-01-001`, `L5-D-03-001`, `L4-A-001`, `L4-A-015`, `L4-A-028`, `L4-A-050`

### Architecture Decisions

`AD-001`–`AD-005`

### Conformance Requirements

`CR-001`–`CR-004`

### Open / Blocking Items

`OD-001`–`OD-004`
