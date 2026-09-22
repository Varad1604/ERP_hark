# CI/CD & Software Supply Chain Security Architecture Baseline

## 1. Specification Metadata
**Specification ID:** `L5-F-01-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-F — Engineering Delivery & Quality Architecture  
**Major Area:** L5-F-01 — CI/CD & Software Supply Chain Security Architecture  
**Canonical Technical Responsibility:** Engineering delivery, pipeline trust and artifact-promotion principles while branch workflow governance, deployment execution, security primitives, audit truth and verification semantics remain separately owned.  
**Technical Architecture Owner:** L5-F-01 — CI/CD & Software Supply Chain Security Architecture  
**Assigned Author:** Pallav  
**Version:** `1.1.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-E-04-*`; `L5-D-03-*`; `L5-D-02-001`; `L5-D-02-004`; `L5-F-02-*` where verification evidence is consumed; `L5-D-04-*`; `L5-E-05-*`; `L5-E-03-*` for backup/recovery ownership boundaries.  
**Referenced By:** `L5-F-01-002`, `L5-F-01-003`, `L5-E-04-*`, downstream release and operations architecture.  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0028`; `ADR-L5-0030`.

## 2. Purpose, Scope & Non-Goals
Define the shared technical architecture by which governed source changes become traceable, integrity-protected release artifacts.

**In Scope**
- pipeline-as-code control plane;
- trusted build execution;
- artifact identity and immutability;
- promotion eligibility and release evidence;
- environment/credential separation;
- supply-chain evidence handoff;
- CI/CD operational observability.

**Out of Scope**
- branch/PR governance policy;
- runtime deployment/install/rollback mechanics — E04;
- cryptographic key/credential ownership — D03;
- audit evidence ownership — D04;
- verification/test semantics — F02;
- exact CI/CD, registry, signing, SBOM or source-control product;
- invented numeric thresholds.

## 3. Upstream Applicability, Constraints & Assumptions
| Source | Status | Establishes | F01 Use |
|---|---|---|---|
| Candidate Corpus Register 105 | Architecture Approved | F01-001 pipeline trust/artifact-promotion boundary | Canonical scope |
| L5 execution timeline | Locked | Stage 3; F01 depends on `E04 + D03` | Dependency order |
| `L5-E-04-005` | Draft | E04 owns install/upgrade/rollback; F01 is collaborator | F01 stops at release eligibility/deployment handoff |
| `L5-D-03-004` | Draft | D03 owns secrets/certificates/machine credentials | Pipeline credentials are consumed, not owned |
| `L5-D-02-001` | Architecture Approved | D02 owns authorization and least privilege | Pipeline/service principals require governed authorization, not credential possession alone |
| `L5-D-02-004` | Architecture Approved | D02 owns policy decision and enforcement | Promotion/build/deployment-adjacent protected actions consume D02 authorization decisions |
| `ADR-L5-0009` | Architecture Approved | Non-human principal authority binding | Pipeline/workload identities require explicit technical authority binding |
| L5 Validation Standard | Locked | HF-09/HF-11/HF-12/HF-15 | ADR, completeness, trust and metadata gates |

Draft peer L5 material is consumed for dependency/boundary planning and is not treated as immutable upstream business authority.

### Constraints
1. Production artifacts originate from governed version-controlled source.
2. Production artifacts are created by controlled automation.
3. Promotion changes artifact eligibility/location state, not artifact bytes.
4. Pipeline credentials, certificates and cryptographic material are governed by D03; possession of credentials never implies execution authority.
5. Pipeline/service/workload authorization and least-privilege enforcement are governed by D02 and `ADR-L5-0009`.
6. F02 owns verification semantics; F01 consumes outcomes.
7. E04 owns runtime deployment and rollback.
8. E03 owns backup/restore/DR; F01 only identifies critical pipeline/artifact metadata dependencies.
9. GitHub Actions, GHCR and Cosign/Sigstore are governed by `ADR-L5-0030`; remaining pipeline policy/profile choices stay explicit.

### Assumptions
| ID | Assumption | Resolution |
|---|---|---|
| AS-001 | Governed source control exists for production code/config/pipeline definitions. | Resolve before Architecture Approved if not canonicalized. |
| AS-002 | F02 will expose machine-consumable verification outcomes. | Align before approval where materially required. |

## 4. Technical Ownership & Dependencies
**Owns:** pipeline control/execution, trusted build boundary, artifact identity/manifest/evidence generation, promotion eligibility enforcement, CI/CD policy execution, pipeline operational signals.

**Consumes:** D03 secrets/certificates/machine-credential/signing services; D02 authorization decisions and non-human authority bindings; E04 deployment target/profile; F02 verification outcomes; D04 evidence retention where audit-grade evidence is required; E05 observability; E03 recovery services for critical CI/CD metadata where applicable.

**Must Not Own:** business facts, human or non-human authorization policy, deployment semantics, rollback safety, cryptographic primitives/key lifecycle, audit canonical truth, test design, backup/restore/DR behavior, or branch workflow governance.

## 5. Solution Architecture
### 5.1 Logical Elements
| Element | Responsibility |
|---|---|
| Pipeline Definition Repository | Version-controlled pipeline-as-code source and policy bindings |
| Pipeline Controller | Orchestrates build, verification, packaging, evidence and promotion |
| Trusted Build Executor | Isolated build environment with bounded identity/permissions/network/toolchain; ephemeral execution is preferred for high-trust production builds where supported |
| Artifact Registry Boundary | Stores immutable build outputs and promotion metadata |
| Release Manifest Builder | Binds artifact digests, config references, source revision and evidence |
| Promotion Gate | Evaluates mandatory checks and governed exceptions |
| Supply-Chain Evidence Adapter | Consumes F01-003 SBOM/provenance/signing outputs |
| Deployment Handoff | Supplies E04 with exact eligible release manifest/artifacts |
| Telemetry Adapter | Emits pipeline health/failure evidence to E05 |

### 5.2 Primary Flow
```text
governed source revision
→ controlled pipeline definition
→ isolated trusted build
→ immutable artifact + identity
→ F02 verification evidence
→ F01-003 SBOM/provenance/signature evidence
→ promotion policy evaluation
→ exact release manifest
→ eligible release handoff
→ E04 deployment
```

### 5.3 Failure Model
| Failure | Required Outcome |
|---|---|
| Build input/toolchain unresolved | Fail build; no releasable artifact |
| Mandatory verification fails | Block promotion |
| Digest/signature/provenance mismatch | Reject/quarantine artifact |
| Required credential unavailable | Fail closed |
| Promotion exception missing/expired | Block promotion |
| CI/CD outage | Preserve accepted artifacts/evidence; do not fabricate eligibility |
| Suspected runner/pipeline compromise | Suspend trust in affected outputs and assess impact |

### 5.4 Security, Authorization & Isolation
Build/promotion identities are non-human service/workload identities. Identity and credential validity are necessary but not sufficient for execution: protected pipeline actions consume D02 authorization decisions and `ADR-L5-0009` authority bindings. Privileges are stage-scoped and least-privilege. High-trust build/signing execution is isolated from general CI workload where required; ephemeral runners are preferred for production build/signing workloads where supported. Secrets are obtained through D03 controls and never embedded in source or artifacts. Production and non-production credentials/execution contexts are separated.

High-trust build/signing jobs use controlled outbound network access where practical so arbitrary dependency or command retrieval cannot silently bypass supply-chain policy.

### 5.5 Source & Release Reference Integrity
Production release paths consume protected source references or equivalent governed refs rather than arbitrary developer refs. Release tags/version references are immutable or equivalently protected after release creation. Where source/tag signing or equivalent authenticity evidence is supported by the source platform and governed trust model, high-trust release paths preserve and validate that evidence.

### 5.6 Release Evidence Bundle
Each production release produces a coherent evidence bundle linking source revision, pipeline definition version, builder identity, artifact digest, release manifest, F02 verification outcomes, F01-003 SBOM/provenance/signature evidence, promotion authorization, exceptions and deployment handoff reference. D04 remains owner of audit-grade evidence integrity/retention where such retention is required.

### 5.7 Exception Expiry & Compromise Handling
Policy exceptions are scoped and time-bounded; expiry automatically removes future authorization. Suspected CI/build/signing compromise invalidates trust in affected outputs as appropriate, triggers reverse impact analysis and requires governed rebuild/re-sign/re-release before trust is restored. F01 identifies critical pipeline/artifact metadata for E03 recovery but does not own backup/restore/DR.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Pipeline definition | Pipeline-as-code is version-controlled and reviewable |
| AD-002 | Artifact trust | Production artifacts come from controlled automated builds |
| AD-003 | Artifact promotion | Same immutable artifact is promoted; environment stages do not rebuild it |
| AD-004 | Build isolation | Production builds execute in isolated controlled environments |
| AD-005 | Identity & authorization | CI/CD uses governed non-human identities; D02 authorization and `ADR-L5-0009` bind allowed actions with least privilege |
| AD-006 | Separation | F01 owns eligibility/promotion; E04 deployment; F02 verification semantics; D03 credentials/crypto |
| AD-007 | Exceptions | Bypasses are explicit, scoped, authorized, evidenced and time-bounded |
| AD-008 | Technology | GitHub Actions + GHCR + Cosign/Sigstore are canonical under `ADR-L5-0030`; high-trust paths use ephemeral self-hosted runners where required. |
| AD-009 | Source references | Production release paths use protected/governed source refs; release refs are immutable or equivalently protected |
| AD-010 | Runner trust | High-trust production build/signing workloads are segmented from general CI; ephemeral execution is preferred where supported |
| AD-011 | Evidence | Production release evidence is coherently bound across source, build, artifact, verification, provenance, authorization and deployment handoff |
| AD-012 | Recovery boundary | F01 identifies critical CI/CD metadata; E03 owns backup/restore/DR implementation |

## 7. Conformance Requirements
- CR-001: Every production release artifact shall trace to an exact governed source revision and controlled build.
- CR-002: A verified artifact's bytes shall not change during promotion.
- CR-003: Mandatory verification/security/provenance gate failure shall block promotion.
- CR-004: CI/CD credentials shall be consumed through D03-owned controls and never embedded in source/artifacts.
- CR-005: Production promotion shall be attributable to an authorized human or governed non-human principal whose authority is admitted through D02/`ADR-L5-0009`.
- CR-006: F01 shall hand off an eligible release to E04 without redefining deployment/rollback behavior.
- CR-007: Vendor/product selection shall not become canonical without governed selection.
- CR-008: Production release references shall be protected and release tags/version refs shall be immutable or equivalently protected after release creation.
- CR-009: Expired exceptions shall not authorize future promotions.
- CR-010: High-trust runner compromise shall cause affected outputs to be treated as untrusted until impact assessment and governed remediation complete.

## 8. Quality, Operations & Portability
Capacity dimensions include concurrent pipelines, queue time, build duration, artifact throughput/storage, registry availability and runner saturation; no numeric targets are invented. Pipeline health/failure telemetry flows to E05, including repeated pipeline failure, unavailable signing/provenance controls, registry-integrity failures and release-blocking supply-chain failures. Source, manifests, digests, SBOM/provenance and signature semantics remain portable enough for platform replacement where feasible.

## 9. Open Decisions
| ID | Item | Blocking? |
|---|---|---|
| OD-001 | **Resolved:** GitHub Actions + GHCR + Cosign/Sigstore. | Locked by `ADR-L5-0030` |
| OD-002 | Artifact retention durations and numeric pipeline SLOs | No for Review |
| OD-003 | Final F02 machine-readable gate contract | No for Review; align before approval if material |
| OD-004 | Draft status of material E04/D03 peers | Yes for approval-level reliance unless governance authorizes |
| OD-005 | Exact runner isolation/ephemerality/network-egress implementation | No for Review; product/topology selection remains governed |

## 10. Traceability Summary
Canonical scope: `L5-F-01-001` in Candidate Corpus Register 105.  
Primary peer boundaries: `L5-E-04-*`, `L5-D-03-*`, `L5-D-02-001`, `L5-D-02-004`, `L5-F-02-*`, `L5-D-04-*`, `L5-E-05-*`, `L5-E-03-*`. Applicable approved ADRs: `ADR-L5-0001`, `ADR-L5-0002`, `ADR-L5-0009`.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0030`  
GitHub Actions is canonical while source remains on GitHub; GHCR is the default OCI registry; Cosign/Sigstore handles signing/attestation; high-trust paths use ephemeral self-hosted runners where required.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled F01 product status with the locked delivery/supply-chain standard.
