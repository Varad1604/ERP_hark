# ADR-L5-0024 — Keycloak Authentication Platform Standard

**ADR ID:** `ADR-L5-0024`  
**Title:** Keycloak Authentication Platform Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-01 — IAM Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-D-01-001`…`L5-D-01-004`  
**Related ADRs:** `ADR-L5-0004`; `ADR-L5-0009`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
OIDC/OAuth/SAML maturity, MFA/federation, self-hosting, administration, security ecosystem and provider neutrality.

## 5. Alternatives Considered
Keycloak; cloud-only managed IAM; custom authentication server; remain unselected.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**Keycloak 26.x is the canonical authentication/federation platform.**

Keycloak 26.x is canonical; initial implementation uses repository-approved 26.x patch level (26.7.x baseline). SutraERP remains canonical owner of Person/Identity/User Account semantics and D02 remains authorization authority. Keycloak technical identities reference SutraERP canonical identity.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-D-01-001`…`L5-D-01-004`. Related authority: `ADR-L5-0004`; `ADR-L5-0009`.  
External evidence: https://www.keycloak.org/documentation ; https://www.keycloak.org/docs/latest/release_notes/

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
