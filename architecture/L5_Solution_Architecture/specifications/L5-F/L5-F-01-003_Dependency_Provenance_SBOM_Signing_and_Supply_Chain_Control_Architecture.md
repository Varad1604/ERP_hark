# Dependency Provenance, SBOM, Signing & Supply-Chain Control Architecture

## 1. Specification Metadata
**Specification ID:** `L5-F-01-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-F — Engineering Delivery & Quality Architecture  
**Major Area:** L5-F-01 — CI/CD & Software Supply Chain Security Architecture  
**Canonical Technical Responsibility:** Dependency provenance, SBOM generation/binding, artifact provenance attestation, signing integration and software-supply-chain assurance.  
**Technical Architecture Owner:** L5-F-01 — CI/CD & Software Supply Chain Security Architecture  
**Assigned Author:** Pallav  
**Version:** `1.2.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-F-01-001/002`; `L5-D-03-*`; `L5-D-02-001`; `L5-D-02-004`; `L5-F-02-*`; `L5-E-04-*`; `L5-D-04-*`; `L5-E-05-*`.  
**Referenced By:** F01-002 promotion gates, E04 artifact verification, security/operations impact analysis.  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0027`; `ADR-L5-0030`; `ADR-L5-0040`.

## 2. Purpose, Scope & Non-Goals
Define how SutraERP proves what a release artifact contains, where its inputs came from, which controlled build produced it and whether its integrity/trust evidence remains valid.

**In Scope:** dependency provenance, lock/resolution evidence, SBOM, build provenance/attestation, signing integration, supply-chain gate results and reverse impact analysis.

**Out of Scope:** cryptographic algorithms/key custody — D03; deployment execution — E04; vulnerability risk ownership/legal policy; exact vendor/product; formal SLSA certification level unless separately demonstrated.

## 3. Upstream Applicability, Constraints & Assumptions
| Source | Status | Establishes | Use |
|---|---|---|---|
| Candidate Corpus Register 105 | Architecture Approved | F01-003 canonical responsibility | Scope authority |
| `L5-F-01-001/002` | Architecture Approved | Trusted build, immutable artifact, promotion gate model | Evidence attaches to exact artifact/release |
| `L5-D-03-003/004` | Architecture Approved | Key custody and machine credentials are D03-owned | Signing uses D03 services/credentials |
| `L5-D-02-001` / `L5-D-02-004` | Architecture Approved | Authorization and policy enforcement | Protected supply-chain actions require governed authority |
| `ADR-L5-0009` | Architecture Approved | Non-human authority binding | Build/sign/promotion automation authority remains explicit |
| `L5-E-04-005` | Architecture Approved | Deployment verifies release/artifact identity | E04 consumes F01 trust evidence |
| L5 Validation Standard | Locked | Security/ADR/implementability hard gates | No silent algorithm/vendor/threshold choices |

### Assumptions
- AS-001: build ecosystems expose enough resolved dependency metadata to generate a materially complete SBOM.
- AS-002: artifact storage preserves digest-addressable or equivalent immutable identity.

## 4. Technical Ownership & Dependencies
**Owns:** dependency-source/provenance controls, dependency lock/resolution evidence, SBOM generation/binding, build provenance/attestation, signing invocation/metadata binding, supply-chain gate results, affected-artifact inventory linkage.

**Consumes:** D03 signing/key/credential capability; D02 authorization for protected automated actions; vulnerability/license policy inputs; F02 verification; F01-002 artifact/release identity; E05 telemetry; D04 audit-grade evidence controls where applicable.

**Must Not Own:** authorization policy, cryptographic key lifecycle, legal/license policy, business risk acceptance, deployment execution or canonical security-event ownership.

## 5. Solution Architecture
### 5.1 Supply-Chain Evidence Flow
```text
Declared dependency sources + lock/resolution metadata
→ controlled dependency resolution
→ build output
→ resolved-component inventory
→ SBOM
→ build provenance attestation
→ artifact digest binding
→ D03-governed signing operation
→ signature/provenance/SBOM evidence
→ F01-002 promotion gate
→ E04 deployment verification
```

### 5.2 Dependency Controls
Production builds resolve dependencies from approved/governed sources or governed mirrors where available. Direct and transitive dependencies are visible. Floating/unbounded versions are avoided where ecosystem support allows governed locking. Third-party pipeline actions/plugins/scripts are treated as dependencies and pinned to governed immutable references where supported.

When the ecosystem provides authenticity/integrity metadata, dependency checksums, signatures or provenance are verified before the dependency is admitted to the trusted build. Supply-chain policy may maintain governed allow/deny controls for dependency sources, packages, publishers, licenses or risk classes. Automated dependency-update proposals are allowed only as change proposals and must pass the same review, verification and promotion controls as equivalent human changes.

Where relevant to the package/artifact ecosystem and approved security policy, the supply-chain analysis path supports malicious-package or malicious-artifact detection. Detection capability does not create a universal scanner/product mandate.

### 5.3 SBOM
SBOM is generated automatically from actual resolved production build content. It covers materially bundled direct/transitive components and is uniquely bound to artifact identity/digest. SPDX/CycloneDX are acceptable industry-standard capability candidates, but no format is canonically selected.

### 5.4 Provenance
Provenance records source revision, build identity, builder identity/class, pipeline definition, declared/resolved inputs and artifact digest. Signed provenance/attestation is supported for production trust paths. SLSA concepts may be used without claiming a formal level absent demonstrated conformance.

### 5.5 Signing & Authorization Boundary
F01-003 requests signing through D03-owned cryptographic/key/credential services. F01 does not own algorithms, keys, custody or rotation. Protected signing/release actions require D02 authorization and, for automation, `ADR-L5-0009` authority binding. Signatures bind to immutable artifact identity.

F01 establishes production eligibility only when its required signature/provenance verification succeeds and hands the verified evidence with the release manifest to E04. E04 remains owner of deployment-admission and runtime verification behavior; F01 does not prescribe E04 internals.

### 5.6 Vulnerability & License Controls
Supply-chain analysis includes direct/transitive dependencies before production eligibility. Findings use governed severity/exploitability/context policy; no universal CVSS threshold is invented. License policy is consumed from authoritative governance. Suppression/exception is explicit, reviewable and time-bounded where applicable; expiration removes future suppression authority.

### 5.7 Post-Release Re-evaluation
Released/deployed artifacts remain linked to SBOM/dependency versions so newly disclosed vulnerabilities or compromised dependencies can be mapped to affected artifacts/releases/environments without rebuilding first.

### 5.8 Compromise Response
1. identify affected trust scope;
2. restrict/reject affected outputs where required;
3. perform reverse impact analysis;
4. delegate key/credential actions to D03;
5. rebuild/re-sign/re-release through governed flow;
6. preserve evidence without exposing secrets;
7. notify operations through E05 and hand audit-grade evidence to D04 where required.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Dependency visibility | Direct and transitive dependencies are included |
| AD-002 | Dependency determinism | Lockfiles/equivalent resolved manifests are governed where supported |
| AD-003 | SBOM binding | SBOM derives from resolved build output and binds to exact artifact |
| AD-004 | Provenance | Production artifacts carry source→builder→artifact provenance |
| AD-005 | Signing ownership | F01 invokes signing; D03 owns algorithms/keys/credentials/custody |
| AD-006 | Re-evaluation | Released artifacts remain queryable against new vulnerabilities/compromise |
| AD-007 | Policy | Gates use governed policy and explicit exceptions; no invented universal thresholds |
| AD-008 | Portability | Evidence semantics remain product-neutral |
| AD-009 | Dependency source trust | Production dependencies resolve from governed sources/mirrors and verify available integrity/authenticity metadata |
| AD-010 | Policy admission | Package/source/license/publisher allow/deny controls may be enforced from governed policy |
| AD-011 | Automated updates | Dependency bots/automation create proposals; they do not bypass review/verification/promotion gates |
| AD-012 | Malware control | Ecosystems with material malicious-package/artifact risk support governed detection before release eligibility |

## 7. Conformance Requirements
- CR-001: Every production artifact shall have dependency/provenance records sufficient for reverse impact analysis.
- CR-002: SBOM evidence shall bind to the exact immutable artifact/release it describes.
- CR-003: Required signatures/provenance shall be verified before F01 marks an artifact production-eligible; verified evidence shall be handed to E04 without redefining E04 deployment-admission behavior.
- CR-004: F01 shall not own/embed cryptographic keys or secret material outside D03 boundaries.
- CR-005: Third-party pipeline components shall be governed as software dependencies.
- CR-006: Material dependency/security findings shall not be silently suppressed.
- CR-007: Evidence shall support artifact→source and dependency→affected-release traceability.
- CR-008: Where available and governed, dependency integrity/authenticity metadata shall be verified before admission to trusted production builds.
- CR-009: Automated dependency-update proposals shall pass the same required review, verification and promotion gates as equivalent human changes.
- CR-010: Protected signing/release actions performed by automation shall consume D02 authorization and `ADR-L5-0009` authority binding.

## 8. Quality, Security & Operations
Evidence retention preserves historical verification after key/credential rotation through D03-compatible trust records. Pipeline/signing/registry integrity failures and release-blocking supply-chain findings are observable to E05. Critical evidence is handed to D04 where audit-grade integrity/retention is required. The architecture remains portable across CI/CD, registry, SBOM and signing products.

## 9. Open Decisions
| ID | Item | Blocking? |
|---|---|---|
| OD-001 | Canonical SBOM serialization (`SPDX`, `CycloneDX`, other) | No for Review |
| OD-002 | **Partially resolved:** Cosign/Sigstore is canonical for signing/attestation; exact keyless/identity-backed signing mode remains security/profile governed. | No for Review |
| OD-003 | Formal SLSA target level | No; cannot be claimed without demonstrated evidence |
| OD-004 | Vulnerability/license thresholds and exception durations | No; require authoritative policy |
| OD-005 | **Resolved:** material D03/E04 peers are Architecture Approved; F01 parent pipeline architecture is Architecture Approved. | Closed for lifecycle promotion; preserve impact review on material peer change |
| OD-006 | Exact dependency mirror/allow-deny/malware-detection implementation | No for Review; product/policy selection remains governed |

## 10. Traceability Summary
Canonical scope: `L5-F-01-003`. Parent architecture: `L5-F-01-001/002`. Authorization owner: `L5-D-02-001` / `L5-D-02-004` with `ADR-L5-0009`. Security/key/credential owner: `L5-D-03-*` with `ADR-L5-0001` / `ADR-L5-0002`. Deployment consumer: `L5-E-04-*`. Operations/evidence collaborators: `L5-E-05-*`, `L5-D-04-*`.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0030`  
GitHub Actions is canonical while source remains on GitHub; GHCR is the default OCI registry; Cosign/Sigstore handles signing/attestation; high-trust paths use ephemeral self-hosted runners where required.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled signing/registry product wording with `ADR-L5-0030`; SBOM format remains legitimately open.


## 2026-09-23 Post-R8 Lifecycle Closure

This specification is promoted from Review to Architecture Approved after F02 verification architecture and all material E04/D03 peer dependencies reached approval-level authority. Existing release/promotion, deployment, verification, credential and supply-chain ownership boundaries are unchanged. No new technology selection or business authority is introduced. Final corpus lock remains separate governance.
