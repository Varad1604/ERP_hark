# ADR-L5-0030 — GitHub Actions, GHCR & Sigstore Supply-Chain Standard

**ADR ID:** `ADR-L5-0030`  
**Title:** GitHub Actions, GHCR & Sigstore Supply-Chain Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-F-01 — CI/CD & Software Supply Chain Security Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-F-01-001`…`L5-F-01-003`  
**Related ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0028`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Repository integration, ephemeral runner support, OCI registry, provenance/signing and migration seams.

## 5. Alternatives Considered
GitHub stack; separate Jenkins/GitLab control plane; persistent privileged runners; unsigned artifacts.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**GitHub Actions + ephemeral trusted runners + GHCR + Cosign/Sigstore are canonical.**

GitHub Actions is canonical while source remains GitHub-hosted. High-trust production build/signing uses ephemeral self-hosted runners where required. GHCR is the default OCI registry. Cosign/Sigstore is the canonical signing/attestation toolchain.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-F-01-001`…`L5-F-01-003`. Related authority: `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0028`.  
External evidence: https://docs.github.com/actions ; https://docs.github.com/packages ; https://docs.sigstore.dev/cosign/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
