# Transaction, Consistency & Concurrency Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-02-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-02 — Database & Persistence Architecture  
**Canonical Technical Responsibility:** Transaction boundaries, concurrency controls and consistency models preserving domain/event boundaries.  
**Technical Architecture Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Pilot ADR Candidate Disposition v1.0.0; `L5-B-02-001` v1.0.1 Draft; `L5-B-02-002` v1.0.1 Draft; `L5-A-04-002` v1.0.1 Draft; `L4-A-015` v1.0.0 Draft; `L4-A-019` v1.0.0 Draft; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** `L5-C-02-*` event/messaging architecture and downstream architectures requiring reliable owner-local state transitions.  
**Applicable ADRs:** `ADR-L5-0006` — Owner-Local Transaction and Optimistic Concurrency Strategy (`Architecture Approved`); `ADR-L5-0007` — Atomic State-to-Publication Intent Handoff Pattern (`Architecture Approved`).

---

## 2. Purpose, Scope & Non-Goals

This specification defines transaction, concurrency and state-to-publication consistency architecture inside canonical persistence-owner boundaries while preserving cross-owner/event ownership boundaries.

### In Scope

- Owner-local transaction boundaries.
- Optimistic concurrency as default conflict strategy.
- Justified stronger local locking/serialization.
- Atomic authoritative state + Durable Publication Intent persistence.
- B02→C02 durable publication handoff boundary.
- Failure/retry and idempotency expectations at that handoff.

### Out of Scope

- Distributed ACID as a repository-wide integration mechanism.
- C02 durable delivery/transport internals.
- Canonical business-event ownership/publisher authority.
- Database-product isolation-level/lock primitive selection.
- Schema evolution — B02-004.

---

## 3. Authoritative Basis & Constraints

| Source | B02-003 Effect |
|---|---|
| L1 v2.1 | Canonical owners remain independent; collaboration uses governed boundaries. |
| `L4-A-015` | Orchestration coordinates owner outcomes without taking ownership of participant state. |
| `L4-A-019` | Reliable event delivery preserves canonical publisher/consumer ownership. |
| B02-001/002 | Persistence and runtime identities remain owner-scoped. |
| pilot `ADR-C3` | Historical transaction/concurrency candidate provenance. |
| pilot `ADR-C4` | Historical state-to-event handoff candidate provenance. |
| `ADR-L5-0006` | Approves owner-local transaction + optimistic concurrency strategy. |
| `ADR-L5-0007` | Approves atomic state + Durable Publication Intent and B02→C02 handoff. |

### Constraints

1. Normal authoritative transactions remain inside one canonical owner boundary.
2. Distributed ACID across canonical owners is not the default collaboration model.
3. Long-running waits, remote calls and consumer acknowledgements shall not be held inside an owner-local database transaction to simulate global atomicity.
4. Concurrency conflicts shall be explicit and detectable.
5. Stronger local locking/serialization requires an invariant-based justification.
6. Durable Publication Intent is technical persistence state, not a canonical business event or new publisher.
7. C02 owns durable delivery after the B02 handoff exists.

---

## 4. Technical Ownership & Boundaries

### Owns

- Owner-local transaction boundary rules.
- Default concurrency strategy and conflict semantics.
- Atomic persistence of owner state plus Durable Publication Intent.
- B02 side of the state→delivery handoff.

### Consumes / Collaborates With

- B02-001/002 persistence ownership/isolation.
- A04 validated Tenant/Institution scope.
- C02 event/messaging delivery from the durable handoff onward.
- E05 observation where stuck/repeated handoff state requires operational evidence.

### Must Not Own

Business transaction semantics/rules, cross-owner business compensation, canonical event ownership, message transport/delivery, database product/locking primitives or schema migration.

---

## 5. Solution Architecture

### 5.1 Owner-Local Transaction Boundary

```text
validated owner + Tenant context
        ↓
owner-local business/technical mutation
        ↓
owner persistence transaction
        ↓
commit or fail atomically inside owner boundary
```

Cross-owner calls are not folded into this transaction merely to obtain global atomicity.

### 5.2 Concurrency Strategy

Optimistic concurrency is the default where concurrent mutation is possible:

```text
read state + version/precondition
        ↓
compute owner-local mutation
        ↓
conditional commit
   ↙             ↘
match → commit    conflict → explicit conflict outcome/retry policy
```

Pessimistic locking or stronger serialization is permitted only within the owner boundary where a documented invariant cannot be safely protected otherwise.

### 5.3 State-to-Publication Handoff

Where committed state requires publication:

```text
authoritative state mutation
        +
Durable Publication Intent
        ↓
SAME owner-local transaction
        ↓
commit
        ↓
B02 durable handoff exists
        ↓
C02 durable delivery / transport
```

The intent carries sufficient technical correlation to permit safe delivery/retry, but it does not become a second canonical event definition or publisher.

### 5.4 Delivery Separation

C02 owns delivery after the durable handoff. Broker acknowledgement, consumer acknowledgement and remote delivery completion are not prerequisites for the owner transaction commit.

### 5.5 Failure Model

| Failure | Required Outcome |
|---|---|
| Optimistic version/precondition conflict | Do not overwrite silently; surface conflict/retry according to owning operation. |
| Owner-local transaction fails | Neither authoritative mutation nor associated publication intent is considered committed. |
| Process fails after commit before delivery | Durable intent remains recoverable for C02 handoff. |
| Delivery repeats after retry/recovery | Handoff/delivery processing must be idempotent where repetition is possible. |
| C02 unavailable | Preserve committed owner state and durable intent; do not mutate peer state directly. |
| Cross-owner global atomicity requested | Redesign collaboration using governed contracts/events/workflow rather than silently widening transaction ownership. |

---

## 6. Architecture Decisions

| Decision ID | Decision | Status | ADR |
|---|---|---|---|
| `AD-001` | Authoritative persistence mutations use owner-local transaction boundaries. | Architecture Approved | `ADR-L5-0006` |
| `AD-002` | Optimistic concurrency is the default; stronger local locking/serialization requires explicit invariant justification. | Architecture Approved | `ADR-L5-0006` |
| `AD-003` | Required publication handoff is made durable atomically with authoritative owner state through a Durable Publication Intent. | Architecture Approved | `ADR-L5-0007` |
| `AD-004` | C02 owns durable delivery after the B02 handoff; remote/consumer acknowledgement is outside the owner-local transaction. | Architecture Approved | `ADR-L5-0007` |

No database-specific isolation level, locking primitive, distributed transaction coordinator, broker or delivery technology is selected.

---

## 7. Security, Quality & Operational Impact

- Tenant/Institution scope remains enforced through A04/B02 boundaries.
- Transaction scoping limits cross-owner coupling and failure blast radius.
- Optimistic conflict outcomes require observable/retriable application handling where appropriate.
- Durable intents require operational observation so stalled handoffs do not remain invisible.
- No retry count, timeout or retention target is invented here.

---

## 8. Architecture Conformance Requirements

| ID | Requirement |
|---|---|
| `CR-001` | Authoritative state transactions shall remain inside one canonical owner boundary by default. |
| `CR-002` | Distributed ACID shall not be used as the normal cross-owner collaboration model. |
| `CR-003` | Concurrent mutation shall have explicit conflict detection where lost updates are possible. |
| `CR-004` | Stronger locking/serialization shall be justified by an owner-local invariant. |
| `CR-005` | When publication is required, authoritative state and Durable Publication Intent shall commit atomically. |
| `CR-006` | Durable Publication Intent shall remain technical state and shall not redefine canonical event/publisher ownership. |
| `CR-007` | C02 delivery begins after the durable B02 handoff exists. |
| `CR-008` | Remote delivery/consumer acknowledgement shall not extend the owner-local database transaction. |
| `CR-009` | Handoff/delivery repetition shall be safe where retries/recovery can repeat work. |
| `CR-010` | Cross-owner direct persistence shall not be used to repair consistency failures. |
| `CR-011` | Product-specific isolation/locking technology shall not become canonical without applicable technology governance. |

---

## 9. Open Decisions & Deferred Design

`ADR-L5-0006` resolves the former `ADR-C3` blocker. `ADR-L5-0007` resolves the former `ADR-C4` blocker while preserving B02/C02 ownership separation.

Product-specific isolation levels, lock primitives, durable-intent representation and delivery technology remain downstream implementation/technology decisions and do not block this product-neutral Draft.

---

## 10. Traceability Summary

- `AD-001`–`AD-002` → `ADR-L5-0006`.
- `AD-003`–`AD-004` → `ADR-L5-0007`.
- Historical pilot provenance: `ADR-C3`, `ADR-C4`.
- B02 owns state/publication-handoff consistency; C02 owns durable delivery after handoff.
