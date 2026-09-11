# ADR-L5-0007 — Atomic State-to-Publication Intent Handoff Pattern

**ADR ID:** `ADR-L5-0007`  
**Title:** Atomic State-to-Publication Intent Handoff Pattern  
**Scope:** Repository  
**Decision Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Architecture remediation for Dhruv Stage-1 corpus  
**Reviewers:** SAOS architecture review; promotion explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-11  
**Affected L5 Specifications:** `L5-B-02-003`; materially constrains `L5-C-02-002`  
**Related ADRs:** `ADR-L5-0006`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

A committed business-state change that requires publication must not create a gap in which state commits but the publication handoff is silently lost. At the same time, B02 must not absorb C02 ownership of durable messaging/delivery. Pilot candidate `ADR-C4` identified this as a material cross-family decision requiring explicit scope resolution.

## 2. Scope & Affected Architecture

This ADR governs only the boundary between authoritative owner-local state commitment and creation of a durable publication intent. B02 owns atomic state + publication-intent persistence inside the owner transaction. C02 owns durable event/message delivery after that handoff exists. This ADR does not define broker technology, delivery retry schedules, consumer acknowledgement, event-schema ownership or business-event publisher authority.

## 3. Authoritative Constraints

1. Canonical business-event publisher ownership remains upstream and unchanged.
2. State and its required durable publication intent must not diverge because of a process/runtime failure between two unrelated writes.
3. Durable Publication Intent is technical persistence state, not a new canonical business event.
4. C02 begins delivery responsibility after the B02 handoff exists.
5. Consumer acknowledgement and remote delivery shall not be part of the owner-local state transaction.
6. Delivery mechanisms must support idempotent handoff/processing where repetition is possible.

## 4. Decision Criteria

No lost publication handoff, preservation of publisher ownership, separation of persistence and messaging responsibility, failure recoverability, idempotency, technology neutrality and cross-family clarity.

## 5. Alternatives Considered

### A — Commit state, then publish directly as a second best-effort action
Rejected: process failure can leave committed state without a durable publication handoff.

### B — Hold owner transaction open until broker/consumer acknowledgement
Rejected: turns remote delivery into distributed transaction coupling and violates owner-local transaction boundaries.

### C — Atomically persist state and durable publication intent, then hand off to C02
Selected.

## 6. Evaluation

| Criterion | A | B | C |
|---|---|---|---|
| Lost-handoff prevention | Weak | Strong | Strong |
| Owner isolation | Acceptable | Weak | Strong |
| Operational coupling | Strong | Weak | Strong |
| Failure recoverability | Weak | Weak/Acceptable | Strong |
| Technology neutrality | Strong | Weak | Strong |

## 7. Decision

When a committed owner-local state transition requires publication, SutraERP shall persist the authoritative state mutation and a **Durable Publication Intent** atomically inside the same owner-local transaction.

After commit:

1. the durable publication intent forms the B02→C02 technical handoff;
2. C02 is responsible for durable delivery/transport behavior from that handoff onward;
3. retries/re-reads of the intent shall be safe and attributable;
4. the intent shall not redefine the upstream canonical event or publisher;
5. consumer acknowledgement shall not extend the owner-local transaction.

## 8. Consequences

Positive: closes the state/publication loss window, preserves B02/C02 ownership, supports restart recovery and idempotent delivery.  
Cost: additional durable technical state and lifecycle/evidence are required; delivery is eventually completed rather than globally atomic with consumers.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Intent treated as a business event | Explicitly classify it as technical persistence state only. |
| B02 duplicates C02 delivery logic | B02 stops at durable handoff; C02 owns delivery. |
| Duplicate delivery after retry | Require idempotent handoff/consumer mechanics in C02. |
| Stuck intents | C02/E05 operational observation and governed recovery; no direct state bypass. |

## 10. Migration / Exit Position

No broker/database product is selected. A replacement strategy must preserve the no-lost-handoff invariant and B02/C02 ownership split. Replacement requires explicit supersession.

## 11. Traceability

Primary realization: `L5-B-02-003_Transaction_Consistency_and_Concurrency_Architecture.md`.  
Material downstream collaboration: `L5-C-02-002`.  
Pilot provenance: `ADR-C4`.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** Repository scope is justified by the cross-family B02/C02 boundary. B02 remains the single decision owner for atomic state/publication-intent creation while C02 retains delivery ownership. Controlled remediation was explicitly authorized on 2026-09-11. This approval does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| B02 technical decision owner | Recommended | 2026-09-10 | B02-003 architecture |
| SAOS architecture reviewer | PASS | 2026-09-11 | Stage-1 production architecture review + pilot ADR-C4 boundary note |
| Architecture promotion authorization | Approved | 2026-09-11 | Explicit remediation authorization |
