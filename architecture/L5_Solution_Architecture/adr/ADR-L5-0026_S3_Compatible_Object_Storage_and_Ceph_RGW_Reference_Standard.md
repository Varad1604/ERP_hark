# ADR-L5-0026 — S3-Compatible Object Storage & Ceph RGW Reference Standard

**ADR ID:** `ADR-L5-0026`  
**Title:** S3-Compatible Object Storage & Ceph RGW Reference Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-B-04 — Document, Media & Object Platform Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-B-04-001`; `L5-B-04-002`  
**Related ADRs:** `ADR-L5-0002`; `ADR-L5-0005`; `ADR-L5-0014`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Portability, S3 ecosystem, self-hosting, HA, object semantics and migration feasibility.

## 5. Alternatives Considered
S3+Ceph RGW; cloud-vendor-only object storage; MinIO default; filesystem-only.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**S3-compatible storage is canonical; Ceph RGW is the default clustered/on-prem reference.**

B04 uses an S3-compatible object-storage contract. Ceph RGW is the default clustered/on-prem reference backend. Cloud S3-compatible implementations are allowed by deployment profile. MinIO is not the repository default. Object references remain opaque and provider extensions do not become business architecture.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-B-04-001`; `L5-B-04-002`. Related authority: `ADR-L5-0002`; `ADR-L5-0005`; `ADR-L5-0014`.  
External evidence: https://docs.ceph.com/en/latest/radosgw/s3/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
