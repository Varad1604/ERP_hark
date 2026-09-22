# Runtime, Packaging & Deployment Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-E-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** Technology-neutral runtime/deployment architecture supporting on-premises and cloud-capable deployment without transferring business ownership.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.2.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-A-01-001`, `L5-A-03-001`, `L5-A-04-*`, `L5-B-02-*`, `L5-D-03-*`, `L4-A-001`, `L4-A-028`, `L4-A-029`, `L4-A-030`, `L4-A-031`.  
**Collaborates With / Referenced By:** `L5-E-04-002`–`005`, `L5-E-01-*`, `L5-E-03-*`, `L5-E-05-*`, `L5-F-01-*`.  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0005`; `ADR-L5-0018`; `ADR-L5-0019`; `ADR-L5-0028`; `ADR-L5-0043`; `ADR-L5-0044`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the reusable runtime/deployment model through which SutraERP can be packaged and operated while preserving business ownership, technical isolation requirements and deployment portability.

### In Scope

- Runtime Unit, Deployable Artifact, Deployment Profile and Environment concepts.
- Build-once/promote-same-artifact principle.
- Externalized configuration and separate secret boundary.
- Environment separation and deployment metadata.
- Health/readiness/lifecycle hooks.
- Placement independence from business ownership.
- Required handoffs to resilience, recovery, observability and delivery architecture without making those later-stage areas prerequisites.

### Out of Scope / Non-Goals

- Exact container/orchestrator/cloud/provider products.
- Database/storage topology.
- Security/secret/PKI internals.
- HA/RPO/RTO targets.
- CI/CD implementation internals.
- Business activation/module lifecycle.
- Exact capacity/topology counts.

---

## 3. Upstream Applicability & Constraints

| Source | Exact Reference | L5 Use |
| --- | --- | --- |
| Candidate Register 105 | `L5-E-04-001` | Establishes canonical responsibility. |
| `L4-A-001` | `BH-003, BH-005, BH-011, BH-012, BH-013` | Runtime supports ready/not-ready technical environments without redefining Tenant lifecycle. |
| `L4-A-028` | `BH-003, BH-007, BH-008, BH-009` | Deployment remains distinct from governed capability/module activation. |
| `L4-A-029` | `BH-001, BH-006, BH-007` | Runtime can host governed successor/coexistence states without creating those states. |
| `L4-A-030` | `BH-003, BH-005, BH-006, BH-007` | Runtime may enforce availability after authoritative suspension/restoration decisions. |
| `L4-A-031` | `BH-001, BH-004, BH-005, BH-006, BH-007` | Decommissioning follows authoritative retirement. |
| `ADR-L5-0003` | Contract-Driven Modular Platform with Selective Isolation | Default modular-core placement; isolation only when justified by approved architecture. |
| `ADR-L5-0004` | Tenant and Institution Isolation Enforcement Pattern | Runtime must preserve enforced Tenant/Institution isolation. |
| `ADR-L5-0005` | Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern | Deployment/co-location does not transfer persistence ownership. |
| `ADR-L5-0002` | Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern | Secrets and machine credentials remain externally governed rather than embedded in artifacts. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### Authoritative Constraints

- Business-domain identity does not itself create a process/service/network/deployment boundary.
- Deployment shall not itself activate/deactivate a business capability.
- Shared runtime/infrastructure shall not create cross-owner persistence access.
- Runtime placement shall preserve Tenant/Institution isolation and credential boundaries.
- Later-stage resilience/recovery/observability/CI-CD specifications may consume and specialize E04 hooks but are not prerequisite authorities for E04's baseline runtime model.

### Assumption Boundary

A continuously vendor-cloud-independent on-premises profile is a working direction only until repository governance makes that requirement authoritative. E04 shall remain capable of expressing on-premises deployment without treating continuous offline/vendor-independent operation as a locked requirement.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Runtime Unit / Deployable Artifact / Deployment Profile / Environment concepts.
- Baseline placement and artifact/configuration separation.
- Runtime identity, version/configuration lineage and health/readiness hooks.
- Deployment metadata/support-profile classification.
- Common logical deployment model used by on-premises and cloud-capable specializations.

### Consumes / Collaborates With

| Specification | Relationship | Boundary |
| --- | --- | --- |
| `L5-A-01-001` | Dependency | E04 realizes approved application composition/placement. |
| `L5-A-03-001` | Dependency | Runtime placement preserves governed module/dependency boundaries. |
| `L5-A-04-*` | Dependency | Runtime preserves Tenant/Institution isolation semantics. |
| `L5-B-02-*` | Dependency | Runtime hosts connectivity but does not own persistence topology/transactions. |
| `L5-D-03-*` | Dependency | Runtime references governed secrets/certificates/credentials. |
| `L5-E-04-002` | Child specialization | Owns package composition and runtime-isolation detail. |
| `L5-E-01-*` | Collaboration / consumer | Consumes E04 health/readiness/placement hooks for resilience realization. |
| `L5-E-03-*` | Collaboration / consumer | Consumes E04 deployment identity/state for recovery realization. |
| `L5-E-05-*` | Collaboration / consumer | Consumes E04 operational evidence/health hooks. |
| `L5-F-01-*` | Collaboration / producer-consumer | Produces/promotes artifacts conforming to E04 artifact contracts. |

The E01/E03/E05/F01 relationships are deliberately collaborations, not prerequisite dependencies, preventing later-stage dependency inversion.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Responsibility | Required Property |
| --- | --- | --- |
| Deployable Artifact | Immutable versioned application/package identity. | Same artifact identity is promoted across environments; environment-specific configuration is external. |
| Deployment Profile | Supported topology class and dependency/configuration expectations. | Profile does not redefine business ownership. |
| Runtime Unit | Executable placement boundary for one governed deployable unit. | Exposes technical identity, version and health/readiness state. |
| Configuration Boundary | Inject environment-specific non-secret configuration. | Configuration is separately governed/versioned from artifact content. |
| Secret/Credential Boundary | Resolve D03-governed secret/certificate references. | Secrets are not embedded in ordinary artifact/config/log content. |
| Runtime Health Interface | Expose liveness/readiness/version/configuration lineage. | Health evidence does not become a business status or business API. |

### 5.2 Primary Technical Flow

```text
Verified artifact
+ externalized configuration
+ governed credential/secret references
→ select supported deployment profile
→ instantiate runtime unit(s)
→ validate required dependencies and configuration
→ expose health/readiness/version evidence
→ admit workload only when required readiness conditions hold.
```

### 5.3 Failure Model

| Failure | Required Outcome | Forbidden Outcome |
| --- | --- | --- |
| Invalid configuration | Runtime remains not-ready with actionable technical evidence. | Accept protected work using guessed/default critical config. |
| Mandatory dependency unavailable | Runtime remains not-ready or explicitly unavailable according to owning contract. | Fabricated success. |
| Optional dependency unavailable | Explicit degraded behavior only if the consuming contract permits it. | Silent incomplete result. |
| Secret/certificate unavailable | Protected path/readiness fails closed. | Fallback to embedded/shared secret. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis | ADR |
| --- | --- | --- | --- | --- |
| AD-001 | Placement model | Modular-core placement is the default; selective isolation is used only for explicit security/workload/resilience/integrity/operational reasons. | `ADR-L5-0003` | `ADR-L5-0003` |
| AD-002 | Artifact/configuration separation | Deployable artifact identity is immutable for a release; environment configuration is externalized. | Release integrity / portability | Local architecture decision |
| AD-003 | Credential handling | Runtime resolves D03-governed machine credentials/secrets externally; artifacts do not embed them. | `ADR-L5-0002` | `ADR-L5-0002` |
| AD-004 | Persistence boundary | Co-location or shared infrastructure shall not grant cross-owner persistence access. | `ADR-L5-0005` | `ADR-L5-0005` |
| AD-005 | Isolation preservation | Tenant/Institution isolation remains enforced independent of deployment profile. | `ADR-L5-0004` | `ADR-L5-0004` |
| AD-006 | Packaging/orchestration technology | OCI images are canonical; Kubernetes is the clustered-production model; non-clustered profiles may use an approved OCI-compatible runtime. | `ADR-L5-0028` | Locked |

---

## 7. Architecture Conformance Requirements

| Requirement ID | Requirement |
| --- | --- |
| CR-001 | Business-domain/module identity alone shall not force an independent runtime/service/deployment boundary. |
| CR-002 | Secrets and machine credentials shall not be embedded in deployable artifacts or ordinary configuration/logs. |
| CR-003 | Managed runtime units shall expose technical readiness/liveness/version evidence where operationally applicable. |
| CR-004 | Runtime co-location shall not grant cross-owner persistence access. |
| CR-005 | Deployment profile changes shall not change Tenant/Institution isolation obligations. |
| CR-006 | E01/E03/E05/F01 collaboration shall not be encoded as prerequisite authority for E04 baseline architecture. |

---

## 8. Open Decisions & Deferred Design

| Item ID | Item | Classification | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- |
| OD-001 | **Resolved:** Rust + Axum + Tokio primary server runtime/framework. | ADR | Locked. | `ADR-L5-0018`; `ADR-L5-0019`. |
| OD-002 | **Resolved:** OCI images + Kubernetes clustered profile; simple OCI non-clustered profile retained. | ADR | Locked. | `ADR-L5-0028`. |
| OD-003 | Exact profile sizing/availability targets | E01/E02 dependency | No | Define by approved reliability/capacity profiles. |
| OD-004 | Continuous vendor-cloud independence for core on-premises operation | Architecture/ADR decision | Yes only if this is to become a mandatory support promise | Establish repository authority before making normative. |
| OD-005 | Material L4 parents remain Draft | Upstream | Yes for Architecture Approved | Resolve through L4 governance. |

---

## 9. Traceability Summary

### Material Dependencies

`L5-A-01-001`, `L5-A-03-001`, `L5-A-04-*`, `L5-B-02-*`, `L5-D-03-*`, `L4-A-001`, `L4-A-028`, `L4-A-029`, `L4-A-030`, `L4-A-031`

### Material Collaborators

`L5-E-04-002`–`005`, `L5-E-01-*`, `L5-E-03-*`, `L5-E-05-*`, `L5-F-01-*`

### Applicable Approved ADRs

`ADR-L5-0002`, `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0005`


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0028`  
OCI is the artifact standard; Kubernetes is the clustered orchestrator; non-clustered on-prem may use an approved OCI runtime; cloud vendor remains pluggable.


## 2026-09-22 Backend Readiness Group 5 Synchronization

**Applicable locked decision:** `ADR-L5-0043`  
Distribution-neutral Kubernetes is the clustered baseline; Podman is the default non-clustered on-prem OCI runtime; Envoy Gateway + Gateway API, OpenTofu and Helm are canonical deployment tools; GitOps controller remains optional; supporting stateful platforms are independently operated.


## 2026-09-22 Backend Readiness Group 5 Synchronization

**Applicable locked decision:** `ADR-L5-0044`  
Runtime units use governed immutable artifacts/configuration, separate startup/liveness/readiness semantics and graceful draining/shutdown; emergency mutation is exceptional and reconciled.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled the stale primary-runtime open decision with the Rust/Axum/Tokio baseline. Deployment sizing/availability values remain E01/E02 profile decisions.
