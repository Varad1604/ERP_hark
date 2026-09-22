# ADR-L5-0032 — Rust Decimal Monetary Computation Representation Standard

**ADR ID:** `ADR-L5-0032`  
**Title:** Rust Decimal Monetary Computation Representation Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-A-07 — Secure Computational Engine Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-A-07-002`; `L5-A-07-004`; monetary computation consumers  
**Related ADRs:** `ADR-L5-0018`; `ADR-L5-0020`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Exact base-10 arithmetic, PostgreSQL NUMERIC interoperability, Rust maturity and predictable serialization.

## 5. Alternatives Considered
rust_decimal; binary float; arbitrary precision everywhere; custom fixed-point.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**rust_decimal::Decimal is the default authoritative Rust monetary scalar.**

Authoritative Rust monetary computation uses rust_decimal::Decimal by default. No global precision, scale, currency minor-unit or rounding rule is introduced; each calculation binds the applicable authoritative policy. Requirements beyond supported precision require a governed exception rather than truncation.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-A-07-002`; `L5-A-07-004`; monetary computation consumers. Related authority: `ADR-L5-0018`; `ADR-L5-0020`.  
External evidence: https://docs.rs/rust_decimal/latest/rust_decimal/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
