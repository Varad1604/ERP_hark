# Durable Event Delivery, Ordering & Failure Handling Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-02-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-02 — Event & Messaging Architecture`  
**Canonical Technical Responsibility:** Durable delivery, technical retries, ordering, failure/dead-letter treatment and transactional publication after the durable publication handoff.  
**Technical Architecture Owner:** `L5-C-02 — Event & Messaging Architecture`  
**Assigned Author:** Pallav  
**Version:** `1.0.2`  
**Status:** `Review`  
**Last Updated:** `2026-09-12`  
**Depends On:** `L5-B-02-003`; `L5-C-02-001`; `ADR-L5-0007`; `ADR-L5-0010`  
**Referenced By:** downstream event-consuming L5 specifications requiring durable delivery/recovery semantics  
**Applicable ADRs:** `ADR-L5-0007`; `ADR-L5-0010`  
**Governing Sources:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology; L5 ADR Register.

## 2. Purpose, Scope & Non-Goals
Defines durable delivery after the B02-owned Durable Publication Intent exists. Owns acknowledgement, retry classification, bounded redelivery, ordering scope, dead-message isolation, replay and backpressure. It does not own source transaction commit, business retry/reversal policy, event schema evolution, or consumer-domain business logic.

## 3. Upstream Applicability, Constraints & Assumptions
`ADR-L5-0007` requires authoritative state and Durable Publication Intent to be committed atomically in B02; C02 starts after that handoff and remote consumer acknowledgement shall not extend the owner-local transaction. `ADR-L5-0010` governs the shared C02 durable delivery/subscription/ordering/recovery pattern.

### 3.1 Authoritative L4 Traceability
For this production validation/remediation cycle, canonical L4 specifications are treated as behavioral authority regardless of repository metadata status. These relations are therefore **Validated architectural realization mappings**.

| L4 BH | Contribution | State |
|---|---|---|
| `L4-A-019-BH-006/008/009/010` | Independent delivery, acknowledgement, failure isolation and traceability. | Validated |
| `L4-A-020-BH-001/002/003/004` | Consumer-specific recovery context, preserved identity and governed recovery entry. | Validated |

## 4. Technical Ownership, Boundaries & Dependencies
**Owns:** delivery-attempt lifecycle, ack/redelivery semantics, retryability classification, dead-message state, replay workflow, ordering scope, backpressure.  
**Consumes:** B02-003 + `ADR-L5-0007` durable publication intent; C02-001 routing/subscription; D04 audit capability for privileged replay; E05 telemetry; E01 resilience.  
**Must not own:** owner-local state mutation, business compensation, consumer business decision, business-event semantics.

References to D04/E05 are consumed peer capabilities only. This specification does not invent their internal implementation before those peer architectures are stable.

## 5. Solution Architecture
### 5.1 Delivery Lifecycle
```text
AVAILABLE → DELIVERED → ACKNOWLEDGED
                 ↓ failure/no ack
          RETRY-ELIGIBLE / NON-RETRYABLE
                 ↓ bounded backoff
          REDELIVERED
                 ↓ exhausted/permanent
          QUARANTINED / DEAD-MESSAGE
                 ↓ governed replay
          normal consumer validation/idempotency path
```

These are technical delivery states owned by C02, not business lifecycle states.

### 5.2 Reliability Rules
- Critical consumer acknowledgement occurs only after successful owned processing outcome is known.
- Duplicate delivery is possible; critical consumers are idempotent.
- Retry distinguishes transient from permanent failures and uses bounded backoff; numeric schedules remain configuration/profile decisions.
- Poison messages are isolated per consumer/subscription.
- Replay preserves original event identity plus replay provenance and never bypasses authorization, validation or idempotency.
- Shared delivery/recovery semantics conform to `ADR-L5-0010`.

### 5.3 Ordering
No global order is promised. Ordering is declared only where correctness requires it, normally by aggregate/business stream key. Partition/group ordering permits parallelism across independent scopes. Wall-clock timestamp alone is not causality. Exclusive/single-active processing is allowed only for genuinely serialized streams.

### 5.4 Backpressure & Recovery
Independent consumer backlog prevents one slow subscriber from blocking others. Operational signals include lag/backlog age, redelivery, retry exhaustion and dead-message age. Bulk replay is rate-controlled. Recovery capacity must be sufficient to clear governed backlog without destabilizing normal ingress; numeric thresholds are externalized and are not invented here.

## 6. Architecture & Technology Decisions
| ID | Decision | Position | Authority |
|---|---|---|---|
| AD-001 | Handoff | B02 ends at Durable Publication Intent; C02 owns delivery thereafter. | `ADR-L5-0007`. |
| AD-002 | Delivery | At-least-once-compatible durable delivery for critical consumers. | `ADR-L5-0010`. |
| AD-003 | Ack | Ack only after successful consumer processing. | `ADR-L5-0010`. |
| AD-004 | Retry | Classified, bounded, backoff-based. | `ADR-L5-0010`. |
| AD-005 | Ordering | Scoped/partitioned; no global serialization. | `ADR-L5-0010`. |
| AD-006 | Replay | Privileged, scoped, observable, rate-controlled and idempotency-safe. | `ADR-L5-0010`. |
| AD-007 | Exactly-once | Not a platform-wide guarantee; requires separate justified ADR/profile if selected. | `ADR-L5-0010`. |

## 7. Security, Privacy, Quality & Operations
Replay/dead-message operations are privileged, least-privilege, Tenant-aware and auditable. Diagnostic tooling defaults read-only. Retention of sensitive dead-message content is governed by classification/retention architecture. Large binary content is referenced rather than embedded without bound.

## 8. Architecture Conformance Requirements
`CR-001` C02 begins after B02 durable handoff.  
`CR-002` Critical deliveries are not acknowledged before owned processing success.  
`CR-003` Duplicate delivery is tolerated by idempotent consumers.  
`CR-004` Automated retry is bounded and failure-class aware.  
`CR-005` Ordering scope is explicit; global ordering is prohibited as an assumption.  
`CR-006` Replay is privileged, scoped and normal-path/idempotency preserving.  
`CR-007` One consumer's failure/backlog does not block unrelated consumers.  
`CR-008` Shared delivery, ordering and recovery behavior conforms to `ADR-L5-0010`.  
`CR-009` Delivery lifecycle states shall remain technical C02 state and shall not redefine upstream business lifecycle state.

## 9. Open Decisions & Deferred Design
- Broker/queue product: separate ADR required if canonically selected; not required for Review.
- Numeric retry/backoff/TTL/retention thresholds: externalized to governed operational profiles.
- Peer-control internals from D04/E05 remain consumed capabilities until their canonical architecture is stable; C02 shall not invent them.

No unresolved L4-authority blocker remains for this validation cycle because canonical L4 behavior is being treated as authoritative by explicit review instruction.

## 10. Traceability Summary
Primary decision authority: `ADR-L5-0007` and `ADR-L5-0010`; candidate corpus boundary; locked hybrid/event durability constraint. Validated BHs: L4-A-019 and L4-A-020. Dependencies: B02-003, C02-001/003, E01, E05, D04.
