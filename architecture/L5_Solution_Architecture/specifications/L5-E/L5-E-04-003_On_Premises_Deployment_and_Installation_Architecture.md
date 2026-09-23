# On-Premises Deployment & Installation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-04-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** On-premises deployment and installation architecture preserving portability, security boundaries and supportability.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.3.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-E-04-001`, `L5-E-04-002`, `L5-D-03-*`, `L5-B-02-*`, `L4-A-001`, `L4-A-028`.  
**Collaborates With / Referenced By:** `L5-E-01-*`, `L5-E-05-*`, `L5-F-01-*`, `L5-E-04-005`.  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0005`; `ADR-L5-0028`; `ADR-L5-0043`; `ADR-L5-0044`.

---

## 2. Purpose, Scope & Non-Goals

Define a supportable on-premises installation model for SutraERP without making cloud-only services mandatory or silently promising fully disconnected operation where repository authority does not yet exist.

### In Scope

- On-premises installation topology and prerequisites.
- Artifact/configuration/credential delivery boundaries.
- Local dependency and ingress expectations.
- Installation validation and supportability evidence.
- Restricted-connectivity/offline-capable extension points where later governed.

### Out of Scope

- Business activation/lifecycle.
- Database technology selection.
- Exact operating-system support matrix.
- Exact ingress/load-balancer/container/orchestrator products.
- CI/CD implementation internals.
- Numeric availability/capacity targets.

---

## 3. Authoritative Basis

| Source | Effect |
| --- | --- |
| `L5-E-04-001` | Supplies baseline runtime/deployment model. |
| `L5-E-04-002` | Supplies package composition and runtime-isolation rules. |
| `ADR-L5-0002` | Credentials/secrets are externally governed and not embedded in installers/artifacts. |
| `ADR-L5-0003` | On-prem placement does not imply one service per business domain; modular-core default remains valid. |
| `ADR-L5-0004` | Tenant/Institution isolation is enforced on-prem as in any other profile. |
| `ADR-L5-0005` | On-prem co-location does not transfer persistence ownership or bypass tenant isolation. |
| `L4-A-001`, `L4-A-028` | Deployment readiness/availability remains distinct from business lifecycle and module governance. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### Assumption Boundary

Restricted-network or fully offline operation is **not** treated as a locked repository requirement. The architecture shall avoid unnecessary continuous vendor-cloud dependence where practical, but any mandatory disconnected-support promise requires explicit governance.

---

## 4. Technical Ownership & Dependencies

### Owns

- On-prem installation profile structure.
- Installation prerequisite and validation model.
- Local artifact/configuration/credential placement rules.
- Supportability evidence for installed environments.
- Installation handoff to release/upgrade architecture.

### Relationships

| Specification | Relationship | Boundary |
| --- | --- | --- |
| `L5-E-04-001/002` | Dependency | Baseline runtime and packaging rules. |
| `L5-D-03-*` | Dependency | Secrets/certificates/credentials. |
| `L5-B-02-*` | Dependency | Persistence connectivity and topology remain B02-owned. |
| `L5-E-01-*` | Collaboration | Later resilience architecture consumes on-prem profile hooks. |
| `L5-E-05-*` | Collaboration | Later observability/operations architecture consumes installation/health evidence. |
| `L5-F-01-*` | Collaboration | Delivery pipeline may produce/promote installation artifacts but is not a prerequisite authority. |

E01/E05/F01 are deliberately not encoded as prerequisites for the on-prem architecture.

---

## 5. Solution Architecture

### 5.1 Installation Model

```text
verified release artifact
+ approved on-prem deployment profile
+ environment configuration
+ D03-managed credential references
+ B02-owned persistence connectivity
→ prerequisite validation
→ installation/deployment
→ runtime startup
→ readiness/health validation
→ supportability evidence
→ operational handoff.
```

### 5.2 Installation Rules

- Installation shall use the same governed release identity used by other supported profiles; site-specific rebuilding is not the default architecture.
- Environment configuration is external to the release artifact.
- Secrets, certificates and machine credentials are referenced/resolved through D03-owned mechanisms.
- On-prem packaging shall not grant cross-owner persistence access.
- Installation success is a technical deployment outcome and does not itself activate a business/module lifecycle state.
- Where artifact mirroring or disconnected distribution is later approved, provenance and release identity must remain verifiable.

### 5.3 Failure Model

| Failure | Required Outcome | Forbidden Outcome |
| --- | --- | --- |
| Missing prerequisite/configuration | Installation blocks or runtime remains not-ready with evidence. | Silent default of security/ownership-critical values. |
| Credential unavailable | Protected startup/path fails closed. | Embedded/shared fallback secret. |
| Persistence dependency unavailable | Readiness/install validation reports dependency failure. | Local ad-hoc persistence replacing B02 owner. |
| Unsupported platform/profile | Reject as unsupported configuration. | Implicit support promise. |

---

## 6. Architecture Decisions

| Decision ID | Decision | Position | Authority |
| --- | --- | --- | --- |
| AD-001 | On-prem profile | On-premises is a first-class deployment profile under E04, not a separate business architecture. | E04 canonical responsibility |
| AD-002 | Artifact integrity | Use governed release identity with external environment configuration; avoid environment-specific rebuild as default. | E04-001 |
| AD-003 | Credential boundary | Installers/artifacts do not embed canonical secrets or machine credentials. | `ADR-L5-0002` |
| AD-004 | Modular placement | Modular-core default/selective isolation rules remain valid on-prem. | `ADR-L5-0003` |
| AD-005 | Isolation/persistence | Tenant isolation and persistence ownership remain unchanged by on-prem co-location. | `ADR-L5-0004`, `ADR-L5-0005` |
| AD-006 | OS/ingress/container/orchestrator | Podman is default non-clustered OCI runtime; Kubernetes clustered profile uses Envoy Gateway/Gateway API. OS matrix remains profile-governed. | `ADR-L5-0043` |
| AD-007 | Offline/restricted-network support | Optional architecture extension until explicitly governed as a mandatory support requirement. | Requires governance if made normative |

---

## 7. Conformance Requirements

| Requirement ID | Requirement |
| --- | --- |
| CR-001 | On-prem deployment shall not redefine business/domain/module ownership or lifecycle. |
| CR-002 | Installation artifacts shall preserve verifiable release identity and shall not embed ordinary secrets/credentials. |
| CR-003 | Shared host/process placement shall not grant cross-owner persistence access. |
| CR-004 | Unsupported OS/runtime/topology combinations shall be rejected or explicitly classified unsupported; they shall not become accidental commitments. |
| CR-005 | E01/E05/F01 shall remain collaborators/consumers, not prerequisite authorities for E04-003. |
| CR-006 | Fully offline operation shall not be claimed as mandatory until repository governance explicitly establishes that requirement. |

---

## 8. Open Decisions

| Item ID | Item | Blocking? | Resolution |
| --- | --- | --- | --- |
| OD-001 | Supported OS matrix | Yes before normative support commitment | Govern through E04 technical standard/ADR where material. |
| OD-002 | **Resolved clustered default:** Envoy Gateway + Kubernetes Gateway API. | Locked | `ADR-L5-0043`. |
| OD-003 | **Resolved:** Podman non-clustered; Kubernetes clustered. | Locked | `ADR-L5-0028`; `ADR-L5-0043`. |
| OD-004 | Mandatory restricted/offline operating profile | Yes only if claimed as supported requirement | Establish authoritative repository decision. |
| OD-005 | Material L4 parents remain Draft | Yes for Architecture Approved | Resolve through L4 governance. |

---

## 9. Traceability Summary

**Material Dependencies:** `L5-E-04-001`, `L5-E-04-002`, `L5-D-03-*`, `L5-B-02-*`, `L4-A-001`, `L4-A-028`  
**Material Collaborators:** `L5-E-01-*`, `L5-E-05-*`, `L5-F-01-*`, `L5-E-04-005`  
**Applicable Approved ADRs:** `ADR-L5-0002`, `ADR-L5-0003`, `ADR-L5-0004`, `ADR-L5-0005`


## 2026-09-22 Backend Readiness Group 5 Synchronization

**Applicable locked decision:** `ADR-L5-0043`  
On-premises deployments may remain non-clustered using Podman; Kubernetes is not mandatory for small installations. OpenTofu manages infrastructure and configuration remains external/typed with OpenBao secrets.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
