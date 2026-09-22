# ADR-L5-0021 — REST/JSON/OpenAPI Synchronous Service API Profile

**ADR ID:** `ADR-L5-0021`  
**Title:** REST/JSON/OpenAPI Synchronous Service API Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-C-01 — API & Contract Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-C-01-001`…`L5-C-01-003`  
**Related ADRs:** `ADR-L5-0003`; `ADR-L5-0019`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Interoperability, tooling, client reach, documentation, provider neutrality and implementation simplicity.

## 5. Alternatives Considered
REST/OpenAPI; repository-wide gRPC; GraphQL default; mandatory heavyweight API management.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**HTTPS REST/JSON + OpenAPI 3.2 is the default synchronous API profile.**

Synchronous owner contracts use HTTPS REST/JSON documented by OpenAPI 3.2. gRPC is exception-only where a material streaming/high-throughput requirement justifies it. No heavyweight API-management product is mandatory by default; deployment ingress may provide routing/TLS/policy enforcement without owning business logic.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-C-01-001`…`L5-C-01-003`. Related authority: `ADR-L5-0003`; `ADR-L5-0019`.  
External evidence: https://spec.openapis.org/oas/v3.2.0.html

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
