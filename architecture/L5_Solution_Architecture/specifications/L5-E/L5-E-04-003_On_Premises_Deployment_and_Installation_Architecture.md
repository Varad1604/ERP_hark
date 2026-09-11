# On-Premises Deployment & Installation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-04-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** Repeatable, supportable institution-controlled on-premises deployment and installation.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-E-04-001/002`, `L5-D-03-*`, `L5-B-02-*`, `L5-E-01-*`, `L5-E-05-*`, `L5-F-01-*`, `L4-A-001`, `L4-A-028`.  
**Referenced By:** Installation operations, E03, E05, F01.  
**Applicable ADRs:** Required but unresolved — Supported OS/runtime/container profile; reverse-proxy/ingress product; secrets/certificate integration products. No canonical ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define repeatable, supportable on-premises installation including restricted-network operation, artifact mirroring, prerequisite validation and technical conformance evidence.

### In Scope

- Installation manifest and prerequisite validation.
- Restricted/offline installation.
- Local artifact/dependency mirroring.
- Process-managed and containerized profiles.
- Ingress/reverse-proxy and certificate integration boundary.
- Installation validation/supportability classification.
- Topology/dependency inventory.

### Out of Scope / Non-Goals

- Exact OS distribution/version or proxy product.
- Database product/topology.
- Secret/PKI internals.
- HA targets.
- Business configuration/activation.
- CI/CD build system.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
| --- | --- | --- | --- | --- | --- |
| L5 Governance | Candidate Register 105 | v1.1.0 / Architecture Approved | `L5-E-04-003` row | Repeatable, supportable institution-controlled on-premises deployment and installation. | Establishes canonical responsibility. |
| L4 | `L4-A-001` | v1.0.0 / Draft | `BH-003, BH-005, BH-010, BH-011, BH-012, BH-013, BH-015` | Tenant environment is established; failed provisioning remains unavailable; remediation/re-entry is governed and evidenced. | Installation validates technical readiness without changing Tenant lifecycle. |
| L4 | `L4-A-028` | v1.0.0 / Draft | `BH-003, BH-007, BH-008, BH-009` | Technical deployment is separate from module activation. | Installer cannot activate module/business lifecycle. |
| L5 Peer | See Section 4 | Draft/planned as identified | Exact peer IDs below | Peer-owned technical capabilities. | Consumed without ownership transfer. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
| --- | --- | --- | --- |
| C-002 | Installation success is not Tenant/module activation. | A001/A028 | Report technical readiness only. |
| C-003 | Secrets/certificates remain D03-owned. | D03 | Installer references/injects, never defines security internals. |
| C-004 | Persistence topology remains B02-owned. | B02 | Installer validates approved connectivity only. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
| --- | --- | --- | --- | --- |
| AS-001 | Restricted/offline-capable on-premises operation is a desired support profile for core SutraERP capability. | It is part of the approved working decision set but is not repository authority. | Artifact mirroring and disconnected installation architecture may become optional or change. | Resolve through E04 deployment-pattern/support-profile ADR or technical standard. |

All product/framework/runtime choices discussed during authoring remain **preferred candidates only** unless an approved ADR or technical standard makes them canonical. No numeric timeout, retry, capacity, availability, retention, RPO/RTO, scaling or similar target is invented here.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Installation Manifest/Prerequisite Validator.
- Artifact/dependency import/mirroring flow.
- On-Prem Deployment Profile.
- Installation Validation Report/supportability classification.
- Local runtime installation flow.
- Technical ingress integration.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary |
| --- | --- | --- |
| `L5-E-04-001/002` | Runtime/package model | Instantiates on-prem profile. |
| `L5-D-03-*` | PKI/secrets/trust | Consumes institution-approved security services. |
| `L5-B-02-*` | Persistence | Validates required dependencies. |
| `L5-E-01/05` | HA/readiness/operations | Profile and validation evidence. |
| `L5-F-01-*` | Signed artifacts/SBOM/provenance | Consumes verified release. |

### Must Not Own

This specification shall not redefine peer-L5 internals, upstream business ownership/rules/states/events/authority, or material technology selections that require ADR governance. Cross-boundary interaction uses governed contracts, events, workflows or peer-owned technical boundaries.

---

## 5. Solution Architecture

**Draft decision gate:** Architecture in this section that depends on an `AD-*` whose ADR column contains `Required`, or on a blocking `OD-*`, is a provisional realization for architecture evaluation only. It is not a canonical implementation mandate until the required governance decision/dependency is resolved. Architecture supported independently by cited repository authority remains applicable.

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Installation Manifest | Artifact metadata | List required artifacts, prerequisites, profile and compatibility. | Governed context/configuration as applicable | Owned technical state only | Installation plan. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Prerequisite Validator | Control point | Validate platform/resources/network/persistence/security prerequisites. | Governed context/configuration as applicable | Owned technical state only | Pass/fail findings. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Artifact Mirror Boundary | Repository integration | Import approved artifacts/dependencies into local controlled registry/repo. | Governed context/configuration as applicable | Owned technical state only | Offline-ready source. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Installer/Profile Executor | Deployment controller | Apply selected on-prem deployment profile and external config. | Governed context/configuration as applicable | Owned technical state only | Running units. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Installation Validator | Operational control | Check version, dependency reachability and readiness. | Governed context/configuration as applicable | Owned technical state only | Validation report. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |
| Supportability Classifier | Control point | Classify conformant supported profile vs approved/custom deviation. | Governed context/configuration as applicable | Owned technical state only | Support status. | Peer L5 boundaries | Explicit failure/not-ready/rejection | L4/L5 basis |

### 5.2 Primary Technical Flow

```text
Obtain approved release/mirror → verify artifact integrity/provenance → validate platform/prerequisites → validate network/persistence/security dependencies → apply profile → inject external config/secret references → start runtime → health/readiness checks → emit validation report/supportability classification.
```

The flow is the current Draft technical realization. Portions dependent on unresolved ADRs or blocking peer/upstream decisions remain provisional and non-normative. Exact endpoint names, code classes, manifests, product configuration and numeric policies remain downstream implementation concerns unless later governed by ADR/technical standard.

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner |
| --- | --- | --- | --- | --- |
| Unsupported prerequisite | Prerequisite Validator | Stop installation/readiness. | Do not continue best-effort into unsupported state. | E04 |
| Missing mandatory dependency | Validator | Not-ready with actionable finding. | No partial operational acceptance. | E05 |
| Missing secret/certificate | D03 boundary | Protected failure. | No embedded fallback secret. | D03 |
| Post-install validation failure | Installation Validator | Installation not accepted. | Do not report success from process exit alone. | E05 |

### 5.4 State, Security, Runtime & Peer Boundaries

State mutation, persistence, transaction/concurrency, identity, authorization, cryptography, secrets, resilience, observability and deployment concerns are owned by the exact peer L5 specifications identified in Section 4. This specification constrains only the boundary it canonically owns and shall not duplicate those peer architectures.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
| --- | --- | --- | --- | --- | --- |
| AD-001 | On-prem profiles | **UNRESOLVED — candidate for ADR evaluation:** Standard, containerized and HA-capable profiles. | E04 responsibility | Accommodates different institutions. | Required |
| AD-002 | Restricted network | **UNRESOLVED — candidate for ADR evaluation:** local registry/repository mirroring for restricted/offline profiles. | AS-001 | Enables controlled/offline sites if the support profile is approved. | Required |
| AD-003 | OS candidate | **UNRESOLVED — candidate for ADR evaluation:** Linux-first approved support matrix; Windows secondary/optional. | Working decision | Operational standardization candidate. | Required |
| AD-004 | Ingress | **UNRESOLVED — candidate for ADR evaluation:** Dedicated reverse-proxy/ingress boundary. | D03/E04 | Avoids direct runtime exposure. | Required |
| AD-005 | Supportability | Every install emits validation evidence and profile classification. | E05/E04 | Controlled support. | None |

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
| CR-001 | On-prem installation shall validate prerequisites and mandatory dependencies before technical readiness. | §5 | A001 |
| CR-002 | Installation shall not treat deployment success as Tenant/module activation. | C-002 | A001/A028 |
| CR-003 | Management/health interfaces shall not be exposed as ordinary public business APIs. | §5 | D03/E05 |
| CR-004 | Installation evidence shall identify artifact/profile/config lineage without secret values. | AD-005 | D03/E05 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
| --- | --- | --- | --- | --- | --- | --- |
| OD-001 | Supported OS distribution/version matrix | ADR/standard | No approved canonical resolution / source remains pending | E04 | Yes | Evaluate enterprise Linux/Windows scope. |
| OD-002 | Reverse-proxy/ingress product | ADR | No approved canonical resolution / source remains pending | E04/D03 | Yes | Evaluate NGINX/HAProxy/Envoy class. |
| OD-003 | Container runtime/profile technology | ADR | No approved canonical resolution / source remains pending | E04 | Yes | Resolve packaging/orchestration ADR. |
| OD-004 | Restricted/offline support profile and mirroring requirement | ADR / working assumption | `AS-001` is conversation-derived and not repository authority | E04 / Lead Architect | Yes for making this normative | Resolve in deployment/support-profile ADR or technical standard. |
| OD-005 | Material L4 parents remain Draft | Upstream | No approved canonical resolution / source remains pending | L4 governance | Yes | Resolve before Architecture Approved. |

These blockers prevent `Architecture Approved` status where applicable, but they do not prevent controlled Draft authoring or repository submission under the broad-authoring authorization.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-001-BH-003/BH-005/BH-010/BH-011/BH-012/BH-013/BH-015` — Installation validates technical readiness without changing Tenant lifecycle.
- `L4-A-028-BH-003/BH-007/BH-008/BH-009` — Installer cannot activate module/business lifecycle.

### Material L5 Dependencies

`L5-E-04-001/002`, `L5-D-03-*`, `L5-B-02-*`, `L5-E-01-*`, `L5-E-05-*`, `L5-F-01-*`, `L4-A-001`, `L4-A-028`

### Architecture Decisions

`AD-001`–`AD-005`

### Conformance Requirements

`CR-001`–`CR-004`

### Open / Blocking Items

`OD-001`–`OD-005`
