# ADR-L5-0023 — BPMN & Flowable Workflow Runtime Standard

**ADR ID:** `ADR-L5-0023`  
**Title:** BPMN & Flowable Workflow Runtime Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-A-05 — Workflow & Orchestration Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-A-05-001`…`L5-A-05-003`  
**Related ADRs:** `ADR-L5-0013`; `ADR-L5-0018`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Standards-based modeling, human tasks, timers, versioning, compensation, self-hosting and separation from business authority.

## 5. Alternatives Considered
BPMN+Flowable; custom Rust engine; code-first workflow runtime; remain unselected.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**BPMN 2.x + Flowable OSS 8.x are the canonical workflow notation/runtime.**

BPMN 2.x is the canonical orchestration-definition notation and Flowable OSS 8.x the canonical runtime. Flowable is an isolated governed JVM runtime exception under the Rust-primary decision. Domains retain business rules/state and D01/D02 retain authentication/authorization authority.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-A-05-001`…`L5-A-05-003`. Related authority: `ADR-L5-0013`; `ADR-L5-0018`.  
External evidence: https://www.omg.org/spec/BPMN/2.0.2/ ; https://www.flowable.com/open-source

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
