# ADR-L5-0027 — OpenBao Secrets/Crypto Service & rustls TLS Standard

**ADR ID:** `ADR-L5-0027`  
**Title:** OpenBao Secrets/Crypto Service & rustls TLS Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-D-03-001`…`L5-D-03-004`; protected Rust services  
**Related ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0019`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Self-hosting, secret lifecycle, transit crypto, HSM/KMS integration, auditability, Rust safety and provider neutrality.

## 5. Alternatives Considered
OpenBao+rustls; cloud-only KMS/secret manager; custom secret service; OpenSSL default.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**OpenBao is the canonical secrets/transit service and rustls the default Rust TLS implementation.**

OpenBao is the canonical application/integration secret and centrally governed transit-cryptography service. D03-003 retains key-custody authority and external HSM/KMS integration remains supported. rustls is the default in-process Rust TLS implementation unless an approved compliance/vendor constraint requires an exception.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-D-03-001`…`L5-D-03-004`; protected Rust services. Related authority: `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0019`.  
External evidence: https://openbao.org/docs/ ; https://docs.rs/rustls/latest/rustls/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
