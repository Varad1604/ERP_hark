# Build, Artifact, Promotion & Release Pipeline Architecture

## 1. Specification Metadata
**Specification ID:** `L5-F-01-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-F — Engineering Delivery & Quality Architecture  
**Major Area:** L5-F-01 — CI/CD & Software Supply Chain Security Architecture  
**Canonical Technical Responsibility:** Controlled build, artifact creation, release-manifest formation, environment promotion and release-eligibility gates.  
**Technical Architecture Owner:** L5-F-01 — CI/CD & Software Supply Chain Security Architecture  
**Assigned Author:** Pallav  
**Version:** `1.3.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-F-01-001`; `L5-E-04-*`; `L5-D-03-*`; `L5-D-02-001`; `L5-D-02-004`; `L5-F-02-*`; applicable B02/C01 compatibility evidence where releases include persistence/contract changes; `L5-E-03-*` for recovery ownership boundaries.  
**Referenced By:** E04 release/deployment architecture and operational acceptance.  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0008`; `ADR-L5-0009`; `ADR-L5-0030`; `ADR-L5-0043`; `ADR-L5-0044`; `ADR-L5-0046`.

## 2. Purpose, Scope & Non-Goals
Define the technical path from governed source revision to an immutable release candidate that E04 may deploy.

**In Scope:** build inputs, build execution, artifact identity, release manifest, promotion eligibility, environment separation, release gates, emergency release control and deployment handoff.

**Out of Scope:** deployment mechanics/rollback — E04; test semantics — F02; SBOM/signing internals — F01-003; persistence migration internals — B02; branch workflow governance.

## 3. Upstream Applicability, Constraints & Assumptions
| Source | Status | Establishes | Use |
|---|---|---|---|
| Candidate Corpus Register 105 | Architecture Approved | F01-002 build/artifact/promotion/release boundary | Canonical scope |
| `L5-F-01-001` | Architecture Approved | Pipeline trust baseline | Parent architecture |
| `L5-E-04-005` | Architecture Approved | E04 owns install/upgrade/rollback | Deployment handoff |
| `L5-D-03-004` | Architecture Approved | Machine credential/secret lifecycle | Pipeline credential consumption |
| `L5-D-02-001` / `L5-D-02-004` | Architecture Approved | Non-human authorization and policy enforcement | Build/promotion actions require governed authorization, not credential possession alone |
| `ADR-L5-0009` | Architecture Approved | Non-human principal authority binding | Promotion automation authority is explicit and attributable |
| B02/C01 peer architecture | peer | Migration/compatibility evidence | Promotion gate consumes evidence; does not own internals |

### Assumptions
- AS-001: each deployable component can be represented by immutable artifact identity/digest.
- AS-002: deployment environments expose a stable target profile identifier.

## 4. Technical Ownership & Dependencies
**Owns:** build input resolution, trusted build execution, artifact creation/metadata, release manifests, promotion state/eligibility and release evidence packaging.

**Collaborates:** F01-003 for SBOM/provenance/signing; D02 authorization; D03 credentials/trust; F02 verification; E04 deployment; B02/C01 migration/contract compatibility; E05 pipeline operations; E03 recovery.

**Must Not Own:** application/module business lifecycle, authorization policy, deployment runtime sequencing, rollback safety, schema migration internals, backup/restore/DR behavior or test definitions.

## 5. Solution Architecture
### 5.1 Release Object Model
```text
Source Revision
 + Pipeline Definition Version
 + Declared Toolchain/Build Inputs
        ↓
Controlled Build
        ↓
Immutable Artifact(s)
        ↓
Artifact Set / Release Manifest
        ↓
Verification + Security + Provenance Evidence
        ↓
Promotion Eligibility
        ↓
E04 Deployment Handoff
```

A release manifest binds exact artifact identities/digests, source revision, pipeline/build identity, configuration references (not secret values), compatibility/migration evidence references, F02 verification evidence, F01-003 SBOM/provenance/signature references, and promotion/exception evidence.

### 5.2 Build Execution
Builds run from declared source and inputs in controlled execution contexts. Production artifacts shall not depend on undeclared workstation state. Build scripts/plugins/actions are supply-chain dependencies and are versioned/pinned/reviewed as applicable.

Build inputs explicitly identify source revision, dependency resolution state, toolchain version, pipeline definition/version and approved secret references. Production release builds consume protected/governed source references. Release tags/version references are immutable or equivalently protected after release creation.

High-trust build execution is isolated from general workloads. Ephemeral runners are preferred where supported. Production build/signing execution uses controlled outbound network access where practical and may resolve dependencies only through governed sources/mirrors as established by F01-003.

### 5.3 Artifact Lifecycle
Technical engineering states only:
```text
Built → Verified → Releasable → Promoted → Deployment-Eligible
          ↘ Rejected
```
Any artifact-byte change creates a new artifact identity. Promotion changes metadata/eligibility, not bytes.

### 5.4 Multi-Artifact Releases
The release manifest binds all included component versions and compatibility expectations. Partial promotion is never implicit; if supported, it must be explicit and compatible with E04 rollout rules.

### 5.5 Promotion Gates
Mandatory categories:
- artifact integrity;
- F02 verification;
- dependency/provenance/SBOM/signature checks;
- required contract/schema/migration readiness;
- policy/authorization;
- target-environment eligibility.

Any blocking gate failure prevents production eligibility. Emergency release paths remain explicit, authorized, evidenced and subject to post-release review. Any exception has scope, authority, reason and expiry; expiration automatically removes future bypass authority. Human approval does not substitute for mandatory automated checks.

### 5.6 Environment Separation
Environment-specific configuration is external to immutable artifacts. Production/non-production credentials and contexts are separated. Secrets are never packaged into immutable application artifacts.

### 5.7 Deployment Handoff
F01 emits an exact eligible release manifest to E04. E04 owns installation/upgrade sequencing, coexistence, rollback safety and runtime acceptance. F01 preserves sufficient artifact/configuration history and release evidence for E04 to select a governed rollback or roll-forward path but does not decide runtime rollback safety.

### 5.8 Release Evidence & Risk Context
The release evidence bundle binds source, build, toolchain, artifacts, configuration references, verification outcomes, SBOM/provenance/signatures, authorization/approval and exceptions. Release-risk context may include affected components, security sensitivity, persistence/contract migration impact, dependency changes and rollback complexity. These signals may select stronger governed release gates but do not create unapproved business risk policy.

### 5.9 Failure and Recovery Boundary
A failed build, verification gate or promotion does not mutate the previously accepted release reference. CI/CD or registry outage does not fabricate eligibility. F01 identifies critical release/artifact metadata required for continuity; E03 owns backup, restore and disaster-recovery implementation.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Build provenance | Release maps to exact source, pipeline and build execution |
| AD-002 | Immutable promotion | Build once; promote unchanged |
| AD-003 | Release manifest | Component/config/evidence relationships are machine-readable and explicit |
| AD-004 | Environment config | Config is separated from artifact bytes; secrets are excluded |
| AD-005 | Gate model | Production eligibility requires all mandatory gates for the release class |
| AD-006 | Failure semantics | Failed build/gate/promotion does not alter previously accepted release state |
| AD-007 | Rollback boundary | F01 preserves history/evidence; E04 owns rollback execution/safety |
| AD-008 | Risk classes | Controls may vary by governed release class without invented numeric thresholds |
| AD-009 | Source refs | Production release builds use protected/governed source refs and immutable/equivalently protected release refs |
| AD-010 | Exception expiry | Expired exceptions cease authorizing future promotion automatically |
| AD-011 | Roll-forward | Pipeline preserves governed roll-forward as well as rollback artifact history/evidence |
| AD-012 | Recovery | E03 owns backup/restore/DR; F01 only identifies required CI/CD metadata dependencies |

## 7. Conformance Requirements
- CR-001: Every release manifest shall identify exact immutable artifact(s) and source/build provenance.
- CR-002: No environment promotion shall rebuild a verified artifact.
- CR-003: Mandatory gate failure shall prevent production eligibility.
- CR-004: Environment-specific secrets shall not be embedded in release artifacts.
- CR-005: Multi-artifact releases shall preserve explicit component/version binding.
- CR-006: F01 shall consume B02/C01/E04 readiness evidence rather than redefine it.
- CR-007: Emergency release override shall be explicit, authorized, scoped, evidenced, time-bounded and non-permanent.
- CR-008: Protected production promotion/build actions shall consume D02 authorization for human or non-human principals; credential possession alone shall not authorize them.
- CR-009: Expired exceptions shall cease authorizing subsequent promotions.
- CR-010: Release tags/version refs used for production shall be immutable or equivalently protected after release creation.

## 8. Quality, Failure & Operations
Build/registry/promotion failures are observable through E05, including repeated gate failure, registry-integrity failure, unavailable signing/provenance controls and release-blocking supply-chain faults. Artifact/manifest history supports rollback/roll-forward selection by E04, reverse impact analysis and release reproducibility evidence. Platform outage shall not mutate accepted artifact state or fabricate eligibility.

## 9. Open Decisions
| ID | Item | Blocking? |
|---|---|---|
| OD-001 | Exact release-manifest serialization/schema standard | No for Review |
| OD-002 | **Resolved baseline progression:** Local/Development → Integration/Test → Staging/Pre-production → Production; ephemeral verification environments remain CI-managed. | `ADR-L5-0043` |
| OD-003 | Default promotion implementation | No; must preserve immutability |
| OD-004 | **Resolved:** E04/D03 material peers are Architecture Approved and F02 verification semantics are Architecture Approved. | Closed for lifecycle promotion; preserve impact review on material peer change |
| OD-005 | Exact source-ref protection/tag-authenticity mechanism | No for Review; depends on selected source-control platform |

## 10. Traceability Summary
Parent baseline: `L5-F-01-001`. Deployment boundary: `L5-E-04-005`. Authorization boundary: `L5-D-02-001` / `L5-D-02-004` with `ADR-L5-0009`. Security/credential boundary: `L5-D-03-*` with `ADR-L5-0001` / `ADR-L5-0002`. Persistence evolution: `ADR-L5-0008` where applicable. Verification boundary: `L5-F-02-*`. Recovery boundary: `L5-E-03-*`.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0030`  
GitHub Actions is canonical while source remains on GitHub; GHCR is the default OCI registry; Cosign/Sigstore handles signing/attestation; high-trust paths use ephemeral self-hosted runners where required.


## 2026-09-22 Backend Readiness Group 5 Synchronization

**Applicable locked decision:** `ADR-L5-0043`  
Promotion targets use the governed four-stage environment progression while preserving F01's existing build-once/promote-unchanged rule.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled pipeline/registry wording with `ADR-L5-0030`. The existing immutable build-once/promote-unchanged architecture remains authoritative.


## 2026-09-23 Post-R8 Lifecycle Closure

This specification is promoted from Review to Architecture Approved after F02 verification architecture and all material E04/D03 peer dependencies reached approval-level authority. Existing release/promotion, deployment, verification, credential and supply-chain ownership boundaries are unchanged. No new technology selection or business authority is introduced. Final corpus lock remains separate governance.
