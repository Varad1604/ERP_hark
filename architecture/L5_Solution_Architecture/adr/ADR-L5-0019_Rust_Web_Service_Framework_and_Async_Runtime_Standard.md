# ADR-L5-0019 — Rust Web Service Framework & Async Runtime Standard

**ADR ID:** `ADR-L5-0019`  
**Title:** Rust Web Service Framework & Async Runtime Standard  
**Scope:** Repository  
**Decision Owner:** L5-A-02 — Technology Platform & Standards Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-A-02-001`; Rust server implementations  
**Related ADRs:** `ADR-L5-0003`; `ADR-L5-0018`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Rust-first compatibility, middleware composability, ecosystem maturity, performance, observability integration and maintainability.

## 5. Alternatives Considered
Axum/Tokio; Actix Web; hyper-only/custom composition; remain unselected.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**Axum + Tokio are the canonical Rust backend framework/runtime.**

Axum is selected for the HTTP/application layer and Tokio for asynchronous execution. Tower-compatible middleware is the default composition model. Framework types shall not leak into canonical C01 contracts. A materially different runtime/framework requires a governed exception.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-A-02-001`; Rust server implementations. Related authority: `ADR-L5-0003`; `ADR-L5-0018`.  
External evidence: https://docs.rs/axum/latest/axum/ ; https://tokio.rs/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
