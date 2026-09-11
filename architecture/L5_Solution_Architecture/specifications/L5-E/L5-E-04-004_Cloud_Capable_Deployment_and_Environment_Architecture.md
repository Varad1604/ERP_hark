# Cloud-Capable Deployment & Environment Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-04-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** Provider-neutral cloud-capable deployment and environment architecture without hyperscaler assumption.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-E-04-001/002`, `L5-A-04-*`, `L5-B-02-*`, `L5-B-04-*`, `L5-D-03-*`, `L5-E-01/02/05-*`, `L4-A-001`, `L4-A-028`, `L4-A-029`.  
**Referenced By:** Cloud profiles, E01/E02/E05/F01.  
**Applicable ADRs:** Required but unresolved — Container/orchestration platform; cloud-specific managed services; provider-specific infrastructure choices. No canonical ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define provider-neutral cloud-capable deployment that preserves SutraERP logical architecture and on-prem portability while allowing managed cloud capabilities behind canonical boundaries.

### In Scope

- Provider-neutral logical cloud profile.
- Managed-service consumption boundaries.
- Cloud environment separation.
- Horizontal scaling and multi-zone capability.
- Provider-specific dependency inventory.
- Cloud ingress/secrets/observability abstraction boundaries.
- Migration/exit visibility.

### Out of Scope / Non-Goals

- Selecting AWS/Azure/GCP or Kubernetes distribution.
- DB/object-store/secret/APM vendor selection.
- Mandatory multi-region.
- Numeric scaling/availability targets.
- Tenant placement/business multitenancy semantics.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
| --- | --- | --- | --- | --- | --- |
| L5 Governance | Candidate Register 105 | v1.1.0 / Architecture Approved | `L5-E-04-004` row | Provider-neutral cloud-capable deployment and environment architecture without hyperscaler assumption. | Establishes canonical responsibility. |
| L4 | `L4-A-001` | v1.0.0 / Draft | `BH-003, BH-005, BH-009, BH-011, BH-013` | Tenant context/environment and downstream availability preserve isolation and failure semantics. | Cloud profile preserves Tenant isolation/readiness. |
| L4 | `L4-A-028` | v1.0.0 / Draft | `BH-003, BH-007, BH-008, BH-009` | Capability/module governance is separate from deployment. | Cloud platform cannot redefine module activation. |
| L4 | `L4-A-029` | v1.0.0 / Draft | `BH-003, BH-004, BH-006, BH-007` | Compatibility/transitional coexistence are governed. | Cloud rollout realizes coexistence only when compatible. |
| L5 Peer | See Section 4 | Draft/planned as identified | Exact peer IDs below | Peer-owned technical capabilities. | Consumed without ownership transfer. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-001 | Cloud-capable is not cloud-mandatory. | E04 corpus boundary | On-prem remains supported. |
| C-002 | Provider-specific services shall not redefine canonical responsibilities. | Repository ownership rules | Managed service is technical implementation only. |
| C-003 | Portability means governed migration/exit, not identical physical infrastructure. | Defensible deployment derivation | Provider dependencies are bounded/documented. |
| C-004 | Multi-region is not assumed without E01/E03/business target. | Validation standard | Capability only, not mandate. |

### 3.3 Assumptions

All product/framework/runtime choices discussed during authoring remain **preferred candidates only** unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO, scaling or similar target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Cloud deployment profile/environment model.
- Provider-neutral workload/runtime boundary.
- Provider-specific dependency inventory.
- Scaling/zone-placement hooks.
- Managed-service consumption boundary.
- Cloud exit/migration visibility.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary |
| --- | --- | --- |
| `L5-E-04-001/002` | Runtime/package model | Cloud profile specializes baseline. |
| `L5-A-04-*` | Tenant isolation | Cloud infra preserves scope/isolation. |
| `L5-B-02-* / L5-B-04-*` | Database/object platform | Managed services only as conformant realizations. |
| `L5-D-03-*` | Secrets/crypto/trust | Provider services behind D03 boundary. |
| `L5-E-01/02/05-*` | Availability/capacity/operations | Zones/scaling/telemetry. |

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Cloud Deployment Profile | Architecture profile | Provider-neutral topology/dependency model. | Governed context/configuration as applicable | Owned technical state only | Cloud environment. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Managed-Service Adapter Boundary | Control point | Consume provider capability behind canonical owner responsibility. | Governed context/configuration as applicable | Owned technical state only | Managed dependency. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Scaling Controller Interface | Operational integration | Scale eligible runtime units from governed metrics. | Governed context/configuration as applicable | Owned technical state only | Replica/instance change. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Zone Placement Policy | Deployment policy | Spread eligible units across failure zones when E01 requires. | Governed context/configuration as applicable | Owned technical state only | Placement result. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Provider Dependency Inventory | Evidence registry | Record provider-specific service, owner, portability/exit impact. | Governed context/configuration as applicable | Owned technical state only | Migration/support evidence. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |

### 5.2 Primary Technical Flow

```text
Select cloud profile → resolve provider-neutral runtime units → bind canonical persistence/security/observability dependencies → optionally map approved provider-managed services → deploy eligible units → apply governed scaling/zone placement → record provider dependency/exit evidence.
```

The flow is authoritative at architecture level only. Exact endpoint names, code classes, manifests, product configuration and numeric policies remain downstream implementation concerns unless later governed by ADR/technical standard.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Provider service unavailable | Managed-service boundary | Apply owner/E01-defined failure behavior. | No provider-specific business fallback. | Canonical owner/E01 |
| Scaling unsuitable for stateful unit | Scaling gate | Do not autoscale that unit. | No correctness-breaking replica changes. | B02/E02 |
| Provider lock-in not recorded | Architecture review | Block approval of profile. | No hidden dependency. | E04 |
| Secret/telemetry bypass | D03/E05 boundary | Reject direct app coupling. | No bypass of canonical security/ops owner. | D03/E05 |

### 5.4 State, Security, Runtime & Peer Boundaries

State mutation, persistence, transaction/concurrency, identity, authorization, cryptography, secrets, resilience, observability and deployment concerns are owned by the exact peer L5 specifications identified in Section 4. This specification constrains only the boundary it canonically owns and shall not duplicate those peer architectures.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
| --- | --- | --- | --- | --- | --- |
| AD-001 | Cloud portability | Same logical architecture; provider-specific realization behind canonical boundaries. | E04 responsibility | Avoids hyperscaler business architecture. | Required |
| AD-002 | Managed services | Allowed when owner boundary and portability impact stay explicit. | B02/B04/D03 | Operational leverage without ownership transfer. | Required when materially selected |
| AD-003 | Orchestration candidate | Kubernetes-compatible managed/self-managed platform for larger cloud profile. | Working decision | Portable enterprise operations candidate. | Required |
| AD-004 | Autoscaling | Eligible runtime units may scale from governed metrics; not universal. | E02/E04 | Avoids blanket scaling. | None |
| AD-005 | Zones/regions | Multi-zone supported when E01 requires; multi-region not assumed. | E01/E03 | No invented resilience target. | None |

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
| CR-001 | Cloud deployment shall preserve canonical L5 ownership and logical contract boundaries used on-prem. | AD-001 | Repository ownership |
| CR-002 | Provider-specific managed services shall be mapped to canonical owner and portability/exit impact. | AD-002 | E04 |
| CR-003 | Autoscaling shall be enabled only for runtime units whose state/execution model supports independent scaling. | AD-004 | E02/B02 |
| CR-004 | Multi-region shall not be mandatory absent authoritative E01/E03/business requirement. | AD-005 | No-invented-target rule |
| CR-005 | Provider observability/secrets services shall not require application bypass of E05/D03 boundaries. | §5 | E05/D03 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- | --- |
| OD-001 | Cloud/orchestration platform | ADR | No approved canonical resolution / source remains pending | E04/Lead Architect | Yes | Evaluate Kubernetes/provider options. |
| OD-002 | Provider-native managed services | ADR | No approved canonical resolution / source remains pending | Canonical owner + approver | Yes when selected | Evaluate per service. |
| OD-003 | Zone/region/scaling numeric targets | Dependency | No approved canonical resolution / source remains pending | E01/E02/E03 | No | Define by profile. |
| OD-004 | Material L4 parents remain Draft | Upstream | No approved canonical resolution / source remains pending | L4 governance | Yes | Resolve before Architecture Approved. |

These blockers prevent `Architecture Approved` status where applicable, but they do not prevent controlled Draft authoring or repository submission under the broad-authoring authorization.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-003/BH-005/BH-009/BH-011/BH-013` — Cloud profile preserves Tenant isolation/readiness.
- `L4-A-028-BH-003/BH-007/BH-008/BH-009` — Cloud platform cannot redefine module activation.
- `L4-A-029-BH-003/BH-004/BH-006/BH-007` — Cloud rollout realizes coexistence only when compatible.

### Material L5 Dependencies

`L5-E-04-001/002`, `L5-A-04-*`, `L5-B-02-*`, `L5-B-04-*`, `L5-D-03-*`, `L5-E-01/02/05-*`, `L4-A-001`, `L4-A-028`, `L4-A-029`

### Architecture Decisions

`AD-001`–`AD-005`

### Conformance Requirements

`CR-001`–`CR-005`

### Open / Blocking Items

`OD-001`–`OD-004`
