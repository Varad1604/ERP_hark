# Runtime, Packaging & Deployment Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-E-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** Technology-neutral runtime/deployment architecture supporting on-premises primary and cloud-capable deployment.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-A-01-001`, `L5-A-03-001`, `L5-A-04-*`, `L5-B-02-*`, `L5-D-03-*`, `L5-E-01-*`, `L5-E-03-*`, `L5-E-05-*`, `L5-F-01-*`, `L4-A-001`, `L4-A-028`, `L4-A-029`, `L4-A-030`, `L4-A-031`.  
**Referenced By:** `L5-E-04-002`–`005`, `L5-E-05-*`, `L5-F-01-*`.  
**Applicable ADRs:** Required but unresolved — Primary runtime/language/framework; packaging standard; container/orchestration strategy; major deployment pattern. No canonical ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the reusable runtime/deployment model that supports institution-controlled on-premises and cloud-capable operation without changing business ownership or equating domains with services.

### In Scope

- Runtime Unit, Deployable Artifact, Deployment Profile and Environment concepts.
- Build-once/promote-same-artifact principle.
- Externalized configuration and separate secret boundary.
- Environment separation and deployment metadata.
- Health/readiness/lifecycle hooks.
- Placement independence from business ownership.

### Out of Scope / Non-Goals

- Exact container/orchestrator/cloud/provider products.
- Database/storage topology.
- Security/secret/PKI internals.
- HA/RPO/RTO targets.
- CI/CD internals.
- Business activation/module lifecycle.
- Exact capacity/topology counts.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
| --- | --- | --- | --- | --- | --- |
| L5 Governance | Candidate Register 105 | v1.1.0 / Architecture Approved | `L5-E-04-001` row | Technology-neutral runtime/deployment architecture supporting on-premises primary and cloud-capable deployment. | Establishes canonical responsibility. |
| L4 | `L4-A-001` | v1.0.0 / Draft | `BH-003, BH-005, BH-011, BH-012, BH-013` | Tenant environment/context is established; failed provisioning remains unavailable and can re-enter after remediation. | Runtime supports ready/not-ready technical environments without redefining Tenant lifecycle. |
| L4 | `L4-A-028` | v1.0.0 / Draft | `BH-003, BH-007, BH-008, BH-009` | Module governance and activation are separate from runtime deployment. | Deployment shall not become business/module activation. |
| L4 | `L4-A-029` | v1.0.0 / Draft | `BH-001, BH-006, BH-007` | Successor/coexistence is governed independently of runtime mechanism. | Runtime can host governed coexistence without creating states. |
| L4 | `L4-A-030` | v1.0.0 / Draft | `BH-003, BH-005, BH-006, BH-007` | Suspension/restoration controls availability while preserving history. | Runtime can enforce availability after authoritative decision. |
| L4 | `L4-A-031` | v1.0.0 / Draft | `BH-001, BH-004, BH-005, BH-006, BH-007` | Retirement is governed; runtime decommissioning is deferred. | E04 decommissions only after authoritative retirement. |
| L5 Peer | See Section 4 | Draft/planned as identified | Exact peer IDs below | Peer-owned technical capabilities. | Consumed without ownership transfer. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-001 | Business/domain boundary does not automatically become a process/service/deployment boundary. | L5-A-01-001 | Default modular co-location. |
| C-002 | Deployment does not itself activate/deactivate business capability. | A028-BH-009 | Deployment and business/module lifecycle remain separate. |
| C-003 | Core on-prem operation shall not require continuous vendor-cloud connectivity. | E04 responsibility + approved authoring decision | Restricted/offline profile supported. |
| C-004 | Shared runtime/infrastructure does not change persistence/business ownership. | B02/L1 | No cross-owner write authority by co-location. |

### 3.3 Assumptions

All product/framework/runtime choices discussed during authoring remain **preferred candidates only** unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO, scaling or similar target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Runtime Unit/Deployable Artifact/Deployment Profile/Environment concepts.
- Runtime placement/isolation rules at deployment level.
- Artifact/configuration separation.
- Health/readiness/lifecycle hooks.
- Deployment metadata/support profile classification.
- Common on-prem/cloud logical deployment model.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary |
| --- | --- | --- |
| `L5-A-01-001 / L5-A-03-001` | Composition/dependencies | E04 maps approved composition into runtime units. |
| `L5-A-04-*` | Tenant/Institution isolation | Runtime preserves isolation. |
| `L5-B-02-*` | Persistence | E04 hosts connectivity only. |
| `L5-D-03-*` | Secrets/certificates/trust | Runtime consumes security services. |
| `L5-E-01/03/05` | Resilience/recovery/operations | Profiles realize reliability requirements. |
| `L5-F-01-*` | CI/CD supply chain | Produces/promotes artifacts. |

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Deployable Artifact | Release artifact | Immutable application/package identity produced by F01. | Governed context/configuration as applicable | Owned technical state only | Promotion input. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Deployment Profile | Architecture profile | Supported topology class and dependency/configuration expectations. | Governed context/configuration as applicable | Owned technical state only | Runtime realization. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Runtime Unit | Execution boundary | Run one deployable package/process with defined health/identity. | Governed context/configuration as applicable | Owned technical state only | Operational instance(s). | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Configuration Boundary | Control point | Inject versioned environment configuration separately from artifact. | Governed context/configuration as applicable | Owned technical state only | Configured runtime. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Secret/Credential Boundary | Security integration | Reference D03-managed secrets/certificates, not embed them. | Governed context/configuration as applicable | Owned technical state only | Runtime credentials. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Runtime Health Interface | Operational interface | Expose liveness/readiness/version/config lineage without business API exposure. | Governed context/configuration as applicable | Owned technical state only | E05 evidence. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |

### 5.2 Primary Technical Flow

```text
Verified immutable artifact + externalized config + D03-managed secret references → select supported deployment profile → instantiate runtime unit(s) → validate mandatory dependencies/config → expose readiness/liveness/version evidence → admit workload only when ready.
```

The flow is authoritative at architecture level only. Exact endpoint names, code classes, manifests, product configuration and numeric policies remain downstream implementation concerns unless later governed by ADR/technical standard.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Invalid configuration | Startup/readiness | Remain not-ready and expose actionable technical evidence. | Do not start accepting protected work. | E05 |
| Mandatory dependency unavailable | Readiness | Remain not-ready. | Do not fabricate degraded success. | E01 |
| Optional dependency unavailable | Runtime | Explicit degraded state only when consuming contract allows. | No silent incomplete result. | E01/C01 |
| Secret/certificate unavailable | Security boundary | Fail protected readiness/path. | No fallback to embedded/shared secret. | D03 |

### 5.4 State, Security, Runtime & Peer Boundaries

State mutation, persistence, transaction/concurrency, identity, authorization, cryptography, secrets, resilience, observability and deployment concerns are owned by the exact peer L5 specifications identified in Section 4. This specification constrains only the boundary it canonically owns and shall not duplicate those peer architectures.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
| --- | --- | --- | --- | --- | --- |
| AD-001 | Deployment profiles | Small/standard on-prem, containerized on-prem, HA enterprise and cloud-capable profiles. | E04 responsibility | Supports institutions of different scale. | Required |
| AD-002 | Artifact model | Build once; promote same immutable artifact with external config. | Release integrity | Prevents environment drift. | Required |
| AD-003 | Placement model | Modular core default; selective isolated units only with technical justification. | A01 | Avoids microservice proliferation. | Required |
| AD-004 | Packaging/orchestration candidates | OCI images; Kubernetes-compatible for larger/HA; simpler runtime permitted. | Working decision | Candidate direction only. | Required |
| AD-005 | Config/secrets | Externalize config; secrets stay under D03. | D03/F01 | Secure repeatable deployment. | None |

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
| CR-001 | Deployment shall not map every business domain/module to a mandatory independent runtime unit. | AD-003 | A01/A028 |
| CR-002 | Production deployment shall identify immutable artifact and configuration revision. | AD-002 | Release integrity |
| CR-003 | Secrets shall not be embedded in deployable artifacts or ordinary config/logs. | AD-005 | D03 |
| CR-004 | Managed runtime units shall expose distinct readiness and liveness where operationally applicable. | §5 | E01/E05 |
| CR-005 | Co-location shall not grant cross-owner persistence access. | C-004 | B02/L1 |
| CR-006 | Supported on-prem core operation shall not require continuous vendor-cloud connectivity. | C-003 | E04 responsibility |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- | --- |
| OD-001 | Primary runtime/language/framework | ADR | No approved canonical resolution / source remains pending | Lead Architect | Yes | Evaluate Java/Spring and alternatives. |
| OD-002 | Packaging/container/orchestration standard | ADR | No approved canonical resolution / source remains pending | Lead Architect / E04 | Yes | Evaluate OCI/Kubernetes position. |
| OD-003 | Exact profile sizing/availability targets | Dependency | No approved canonical resolution / source remains pending | E01/E02 | No | Define by profile. |
| OD-004 | Material L4 parents remain Draft | Upstream | No approved canonical resolution / source remains pending | L4 governance | Yes | Resolve before Architecture Approved. |

These blockers prevent `Architecture Approved` status where applicable, but they do not prevent controlled Draft authoring or repository submission under the broad-authoring authorization.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-003/BH-005/BH-011/BH-012/BH-013` — Runtime supports ready/not-ready technical environments without redefining Tenant lifecycle.
- `L4-A-028-BH-003/BH-007/BH-008/BH-009` — Deployment shall not become business/module activation.
- `L4-A-029-BH-001/BH-006/BH-007` — Runtime can host governed coexistence without creating states.
- `L4-A-030-BH-003/BH-005/BH-006/BH-007` — Runtime can enforce availability after authoritative decision.
- `L4-A-031-BH-001/BH-004/BH-005/BH-006/BH-007` — E04 decommissions only after authoritative retirement.

### Material L5 Dependencies

`L5-A-01-001`, `L5-A-03-001`, `L5-A-04-*`, `L5-B-02-*`, `L5-D-03-*`, `L5-E-01-*`, `L5-E-03-*`, `L5-E-05-*`, `L5-F-01-*`, `L4-A-001`, `L4-A-028`, `L4-A-029`, `L4-A-030`, `L4-A-031`

### Architecture Decisions

`AD-001`–`AD-005`

### Conformance Requirements

`CR-001`–`CR-006`

### Open / Blocking Items

`OD-001`–`OD-004`
