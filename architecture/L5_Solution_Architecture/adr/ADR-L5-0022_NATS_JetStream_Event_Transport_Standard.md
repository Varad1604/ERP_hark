# ADR-L5-0022 — NATS JetStream Event Transport Standard

**ADR ID:** `ADR-L5-0022`  
**Title:** NATS JetStream Event Transport Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-C-02 — Event & Messaging Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-C-02-001`…`L5-C-02-003`  
**Related ADRs:** `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context
A concrete implementation decision is required to remove an intentionally deferred L5 technology choice while preserving the current canonical technical owner.

## 2. Scope & Affected Architecture
This ADR applies only to the affected L5 technical responsibilities above. It does not redefine L1–L4 business ownership, rules, states, events, publishers, authority or outcomes.

## 3. Authoritative Constraints
Repository ownership remains canonical; on-prem/provider-neutral support is preserved where applicable; product/runtime choices may not create hidden cross-owner writes or business authority.

## 4. Decision Criteria
Durable consumers, acknowledgements, replay, consumer isolation, Rust support, operational simplicity and on-prem/cloud portability.

## 5. Alternatives Considered
NATS JetStream; Kafka; RabbitMQ; remain product-neutral.

## 6. Evaluation
The selected position was reviewed against the current L5 architecture, interoperability, operational complexity, security, portability and long-term maintainability. Mandatory repository constraints override product preference.

## 7. Decision
**NATS JetStream is the canonical durable event transport.**

NATS JetStream realizes C02 durable event transport. ADR-L5-0007 atomic publication intent and ADR-L5-0010 at-least-once-compatible delivery, idempotency, scoped ordering and replay remain authoritative. Broker subjects are technical routing identities, not canonical business-event ownership.

## 8. Consequences
Implementation may now rely on this standard. Product-specific details remain subordinate to the owning L5 specification and must not leak into business architecture.

## 9. Risks & Mitigations
Dependency/version drift is controlled through F01/F02 verification and governed upgrades. Product lock-in is bounded by canonical contracts, adapters, owner boundaries and explicit replacement governance.

## 10. Migration / Exit Position
A material replacement requires impact analysis and normally a superseding ADR. Canonical contracts/owner boundaries are the required migration seams.

## 11. Traceability & External Evidence
Affected architecture: `L5-C-02-001`…`L5-C-02-003`. Related authority: `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`.  
External evidence: https://docs.nats.io/nats-concepts/jetstream/consumers

## 12. Approval Record
**Approval source:** Explicit Lead Architect instruction on 2026-09-22 to update canonical `main` with the researched decisions as Locked information.  
**Formal Status:** `Locked`  
**Lock authority:** `L5_Technology_Decision_Wave_Lock_Decision_2026-09-22.md`.
