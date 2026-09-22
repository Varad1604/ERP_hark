# ADR-L5-0033 — External Provider Pluggability & No Repository-Wide Vendor Default

**ADR ID:** `ADR-L5-0033`  
**Title:** External Provider Pluggability & No Repository-Wide Vendor Default  
**Scope:** Repository  
**Decision Owner:** L5-C-03 / L5-C-04 — External Integration & Communication Provider Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-C-03-001`…`L5-C-03-009`; `L5-C-04-001`…`L5-C-04-003`  
**Related ADRs:** `ADR-L5-0016`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Institutional/jurisdictional variability, portability, commercial independence, failover and normalized integration boundaries.

## 5. Alternatives Considered
Provider-pluggable/no default; one canonical vendor; unconstrained provider-specific implementations.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**Payment, banking and communication providers remain pluggable; no repository-wide vendor default is canonical.**

No single payment gateway, banking/treasury network, SMS, email or push provider is canonical. Providers are deployment/institution configuration behind C03/C04 adapters. Provider-specific semantics shall not become business architecture. A mandatory provider later requires a scope-specific ADR.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-C-03-001`…`L5-C-03-009`; `L5-C-04-001`…`L5-C-04-003`. Related authority: `ADR-L5-0016`.  
External evidence: Primary authority: ADR-L5-0016 and C03/C04 architecture.

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
