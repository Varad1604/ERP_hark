# ADR-L5-0028 — OCI, Kubernetes & Provider-Neutral Deployment Standard

**ADR ID:** `ADR-L5-0028`  
**Title:** OCI, Kubernetes & Provider-Neutral Deployment Standard  
**Scope:** Repository  
**Decision Owner:** L5-E-04 — Runtime Packaging & Deployment Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-E-04-001`…`L5-E-04-005`  
**Related ADRs:** `ADR-L5-0014`; `ADR-L5-0018`; `ADR-L5-0019`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Artifact portability, on-prem/cloud capability, operational maturity, isolation and avoidance of unnecessary small-install complexity.

## 5. Alternatives Considered
OCI+Kubernetes cluster + simple OCI profile; Kubernetes everywhere; cloud-managed-only; VM-only.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**OCI images are canonical; Kubernetes is the clustered orchestrator; non-clustered OCI deployment remains valid; cloud provider stays pluggable.**

Server workloads are packaged as OCI images. Kubernetes is the canonical clustered-production orchestration model. Kubernetes is not required for single-node/small on-prem profiles, which may run the same OCI artifacts on an approved OCI-compatible runtime. AWS/Azure/GCP remain deployment choices, not canonical architecture.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-E-04-001`…`L5-E-04-005`. Related authority: `ADR-L5-0014`; `ADR-L5-0018`; `ADR-L5-0019`.  
External evidence: https://opencontainers.org/ ; https://kubernetes.io/docs/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
