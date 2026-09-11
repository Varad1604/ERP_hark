# Transaction, Consistency & Concurrency Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-02-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-02 — Database & Persistence Architecture  
**Canonical Technical Responsibility:** Transaction boundaries, concurrency controls and consistency models preserving domain/event boundaries.  
**Technical Architecture Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; L5 Pilot ADR Candidate Disposition v1.0.0; `L5-B-02-001` v1.0.0 Draft; `L5-B-02-002` v1.0.0 Draft; `L5-A-04-002` v1.0.0 Draft; `L4-A-015` v1.0.0 Draft; `L4-A-019` v1.0.0 Draft; SutraERP L1 Business Domain Architecture v2.1 Locked.  
**Referenced By:** `L5-C-02-*` event/messaging architecture and downstream architectures requiring reliable owner-local state transitions.  
**Applicable ADRs:** Required but unresolved — pilot candidate `ADR-C3` Persistence transaction, concurrency and consistency strategy; pilot candidate `ADR-C4` State-to-event durable publication and delivery strategy. No canonical L5 ADR ID is currently allocated.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Define how SutraERP changes authoritative state safely inside one canonical persistence owner, prevents conflicting concurrent updates, and preserves consistency between a committed business fact and the technical handoff required for later canonical event publication.

This specification keeps strong atomic consistency inside an owner boundary while preventing cross-owner persistence coupling. Cross-owner collaboration therefore composes owner-local transactions through governed contracts, events and workflows rather than one distributed database transaction.

### In Scope

- Owner-local transaction boundaries.
- Prohibition of distributed ACID transactions across canonical owners as the default architecture.
- Optimistic concurrency as the default owner-local conflict-control strategy.
- Stronger pessimistic locking/serialization only where an authoritative technical requirement justifies it.
- Detection and handling of stale-write / concurrent-update conflicts.
- Atomic commit of authoritative state plus durable publication intent where a committed fact requires event publication.
- Durable state-to-publication handoff responsibility up to the C02 delivery boundary.
- Idempotent technical handoff requirements necessary to avoid duplicate publication caused by technical replay.
- Failure behavior before commit, after commit/before handoff, and after handoff.

### Out of Scope / Non-Goals

- Persistence ownership, owner namespaces, Tenant placement and database access identities — `L5-B-02-001/002`.
- Database product, transaction isolation-level name, lock primitive, ORM or framework selection.
- Schema evolution/migration mechanics — `L5-B-02-004`.
- Business compensation rules or recovery decisions — owning business/workflow architecture.
- Workflow orchestration mechanics — `L5-A-05-*`.
- Event transport, broker, subscription routing, ordering, dead-letter policy or consumer delivery retries — `L5-C-02-*`.
- API request idempotency — `L5-C-01-003`.
- Background-worker implementation or scheduling technology — `L5-A-08-*`.
- Numeric retry counts, lock timeouts, transaction timeouts or database-specific tuning values.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-B-02-003 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-B-02-003` row | B02-003 owns transaction boundaries, concurrency controls and consistency models preserving domain/event boundaries. | Establishes canonical responsibility. |
| L5 Governance | `L5_Pilot_ADR_Candidate_Disposition_2026-09-09.md` | v1.0.0 / Locked | `ADR-C3`, `ADR-C4`, §5 | Persistence transaction/concurrency/consistency and state-to-event handoff are material ADR candidates; B02-003 owns state/publication-handoff consistency while C02-002 owns durable delivery after handoff exists. | Establishes ADR requirement and exact B02/C02 ownership split. |
| L5 Peer | `L5-B-02-001_Database_and_Persistence_Architecture_Baseline.md` | v1.0.0 / Draft | §§4–6; B02 responsibility split | Authoritative persistence is owner-scoped; direct cross-owner database access is prohibited; B02-003 owns transactions/consistency/concurrency. | Provides persistence and ownership foundation. |
| L5 Peer | `L5-B-02-002_Domain_Persistence_Ownership_and_Isolation_Architecture.md` | v1.0.0 / Draft | §§4–6 | Every authoritative persistence operation resolves to one Owner Persistence Boundary with owner/Tenant isolation; transaction mechanics are delegated here. | Defines the boundary within which a transaction may commit authoritative state. |
| L5 Peer | `L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md` | v1.0.0 / Draft | §§4–5 | Protected persistence operations carry validated Tenant/Institution scope and fail closed if scope is missing, widened or inconsistent. | Transaction context shall not cross Tenant scope implicitly. |
| L4 | `L4-A-015_Workflow_Initiation_to_Completed_Orchestrated_Execution.md` | v1.0.0 / Draft | `BH-004`, `BH-005`, `BH-006`, `BH-007`, `BH-010` | Workflow coordinates domain-owned actions/results and may wait/suspend/fail without directly changing participating-domain state. | Requires cross-owner work to compose local owner transactions rather than one shared distributed transaction. |
| L4 | `L4-A-019_Committed_Business_Fact_to_Reliable_Consumer_Delivery.md` | v1.0.0 / Draft | `BH-001`, `BH-003`, `BH-004`, `BH-007`, `BH-010` | Event publication occurs only for a committed domain-owned fact; stable event identity and immutable publication/traceability are required while publisher ownership remains with the source domain. | Requires reliable consistency between owner-local commit and later publication without making the database the semantic publisher. |
| L1 | `SutraERP_L1_Business_Domain_Architecture_Locked.md` | v2.1 / Locked | §§2.1, 2.4 | Canonical ownership is singular; only the owner changes authoritative business state; cross-domain database access is prohibited. | Prevents transactions from spanning ownership by directly mutating multiple owners. |
| L5 Governance | `L5_Technology_Selection_and_ADR_Methodology.md` | v1.0.1 / Locked | §3 mandatory ADR triggers | Persistence-isolation/consistency strategy and reusable cross-specification patterns require ADR governance. | Keeps the selected Draft transaction/concurrency patterns provisional. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Source | L5 Effect |
|---|---|---|---|
| `C-001` | A transaction shall not obtain write authority over another canonical owner's persistence merely to achieve atomicity. | L1 §§2.1/2.4; B02-001/002 | Authoritative transactions are owner-local by default. |
| `C-002` | Tenant/Institution scope remains valid throughout the protected transaction. | A04-002; B02-002 | A transaction cannot widen or switch Tenant scope mid-flight. |
| `C-003` | A canonical event may represent only a committed business fact. | L4-A-019-BH-001 | Publication handoff shall not expose an uncommitted business fact as published. |
| `C-004` | Event publisher/fact ownership remains with the originating domain. | L4-A-019-BH-004/007 | Publication-intent storage is technical state, not a new semantic publisher. |
| `C-005` | State-to-publication consistency is B02-003-owned; durable delivery after handoff is C02-002-owned. | Pilot ADR Candidate Disposition §5 | B02 ends at a durable accepted handoff to C02 and does not absorb broker/delivery internals. |
| `C-006` | Business recovery/compensation policy remains upstream-owned. | L4-A-015; L5 level separation | Technical conflicts/failures may be surfaced or technically retried only without inventing business decisions. |
| `C-007` | Product-specific isolation levels/locking primitives remain unresolved until technology selection. | L5 Technology Selection methodology | Architecture defines semantics, not vendor syntax. |

### 3.3 Assumptions

No database vendor, broker, ORM, lock primitive, isolation-level name, queue technology, retry count or timeout is assumed.

The Lead Architect selected the following Draft directions on 2026-09-11:

- authoritative transactions are owner-local rather than distributed across canonical owners;
- optimistic concurrency is the default owner-local conflict-control strategy, with stronger locking/serialization permitted only where justified;
- authoritative state plus durable publication intent commit atomically inside the owner-local transaction when later canonical event publication is required.

These are material architecture choices and remain provisional pending `ADR-C3` / `ADR-C4` formal ADR governance.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- The **Owner-Local Transaction Boundary** around authoritative state owned by one persistence owner.
- The rule that cross-owner collaboration does not create a shared ACID transaction by default.
- The **Concurrency Guard** used to detect stale/conflicting owner-local updates.
- The default optimistic-concurrency strategy and the controlled use of stronger serialization/locking where justified.
- The **Durable Publication Intent** technical record representing a committed obligation to hand a canonical event publication request to C02 after the underlying fact commits.
- Atomic state + publication-intent commit inside the owner-local transaction.
- The **Publication Handoff Boundary** that preserves committed intent until C02 accepts the handoff.
- Technical idempotency/correlation requirements needed to prevent handoff replay from creating duplicate publication for the same committed fact.
- Transaction/handoff failure-state boundaries and recovery invariants.

### Consumes / Collaborates With

| Exact L5 Architecture | Consumed Responsibility | B02-003 Boundary |
|---|---|---|
| `L5-B-02-001` | Persistence baseline | Transaction semantics operate only inside approved authoritative persistence boundaries. |
| `L5-B-02-002` | Ownership/isolation | Resolves owner/Tenant persistence target before transaction execution. |
| `L5-B-02-004` | Schema evolution/migration | Changes persistence structures without weakening transaction/concurrency invariants. |
| `L5-A-04-002` | Validated Tenant/Institution scope | Scope remains invariant or validly narrowed before transaction start; B02 does not own scope semantics. |
| `L5-A-05-*` | Workflow/orchestration | Coordinates multiple owner-local outcomes; B02 does not define business saga/compensation policy. |
| `L5-A-08-*` | Background execution | May execute technical publication handoff work; B02-003 retains state/handoff consistency ownership. |
| `L5-C-01-*` | Synchronous contracts | Cross-owner synchronous work starts a separate transaction in the target owner rather than extending the caller's transaction. |
| `L5-C-02-001/002/003` | Event/messaging baseline, durable delivery, schema compatibility | C02 receives a committed publication handoff and owns publication/delivery mechanics after acceptance. |
| `L5-D-04-*` | Audit/evidence | Consumes transaction/conflict/handoff evidence where required without becoming business-record owner. |
| `L5-E-01-*` / `L5-E-05-*` | Resilience/operations | Detects and recovers technical handoff failures without changing authoritative business outcomes. |

### Must Not Own

- Business validation, approval, compensation, rollback policy or retry decisions.
- Another owner's authoritative persistence.
- Cross-owner distributed commit coordination as the default architecture.
- Event semantic ownership, Event Definition/Contract or canonical publisher identity.
- Event broker, routing, consumer delivery, acknowledgement, dead-letter or ordering mechanics after C02 accepts the handoff.
- API idempotency semantics.
- Database product, isolation-level syntax or vendor lock implementation.
- Schema migration mechanics.

---

## 5. Solution Architecture

### 5.1 Owner-Local Transaction Boundary

A transaction that changes authoritative state shall execute entirely inside one canonical Owner Persistence Boundary.

```text
Validated owner + Tenant scope
        ↓
B02-002 Owner Persistence Boundary
        ↓
Owner-local transaction
        ├── read owner state
        ├── validate technical concurrency condition
        ├── write owner authoritative state
        └── write durable publication intent where required
        ↓
commit / rollback
```

A cross-owner workflow is therefore a composition of separate owner-local commits:

```text
Owner A local transaction → commit
        ↓
governed contract / event / workflow
        ↓
Owner B local transaction → commit
```

One owner shall not enlist another owner's authoritative database in its local ACID transaction merely to create repository-wide atomicity.

### 5.2 Cross-Owner Consistency Model

SutraERP provides strong transaction atomicity **inside** an owner boundary. Across owners, consistency is achieved by governed coordination of separately committed owner outcomes.

This means a cross-owner process may temporarily observe different progress points while coordination is in flight. That condition shall not be "fixed" by:

- direct writes into another owner's persistence;
- one shared cross-owner transaction;
- database triggers/procedures that silently mutate another owner's authoritative state;
- treating orchestration state as authority over participating-domain state.

Where a cross-owner process requires rollback, reversal, compensation or another business recovery outcome, the owning upstream workflow/domain architecture must define that behavior. B02-003 supplies transaction primitives; it does not invent compensating business actions.

### 5.3 Optimistic Concurrency Guard

Optimistic concurrency is the default owner-local strategy for mutable authoritative state.

Conceptually:

```text
read authoritative state + concurrency token
        ↓
perform owner-governed decision
        ↓
at commit, verify token/current-state expectation
        ↓
match? ── yes → commit
   │
   no
   ↓
concurrency conflict
```

The concurrency token is an architecture concept representing sufficient technical evidence that the state being changed is still the state on which the operation was based. Its concrete representation is product/implementation dependent.

On conflict:

- the stale write shall not silently overwrite the newer committed state;
- the conflicting transaction shall not partially commit;
- the conflict shall be surfaced to the owning technical/business execution path for governed re-evaluation;
- an automatic technical retry is permitted only when repeating the operation cannot repeat or invent a business decision and the applicable owner architecture allows it;
- blind retry of approvals, postings, publications or other business decisions is prohibited unless the owning behavior explicitly makes such replay safe.

### 5.4 Stronger Serialization / Locking

Pessimistic locking or stronger serialization may be used for a specific owner-local operation when an authoritative technical requirement demonstrates that optimistic conflict detection is insufficient, for example because the operation requires strict sequencing or experiences materially harmful contention.

Such strengthening shall remain:

- owner-local;
- Tenant-scope preserving;
- explicit and reviewable;
- limited to the smallest necessary consistency boundary;
- free of cross-owner lock coupling.

This specification does not select a database isolation-level name, lock syntax or timeout.

### 5.5 Atomic State + Durable Publication Intent

When a successful owner-local state change creates an obligation to publish a canonical business event, the owner transaction shall atomically persist:

```text
A. authoritative business state change
+
B. durable publication intent
```

inside the same owner-local transaction.

```text
BEGIN owner-local transaction
        ↓
validate owner/Tenant scope
        ↓
validate concurrency condition
        ↓
change authoritative owner state
        ↓
record durable publication intent
        ↓
COMMIT BOTH
or
ROLL BACK BOTH
```

The durable publication intent is **technical persistence state**. It records that, after successful commit, a canonical event publication handoff is required. It is not itself the published canonical Event Instance and does not change the semantic publisher defined upstream.

If the transaction rolls back, neither the business state change nor its publication intent may survive as committed state.

### 5.6 Publication Handoff Boundary

After the owner-local transaction commits, the publication intent becomes eligible for handoff to `L5-C-02-*`.

```text
Committed owner fact
        +
Committed publication intent
        ↓
B02-003 durable publication handoff
        ↓
C02 accepts handoff
        ↓
C02 publication / routing / delivery architecture
```

B02-003 shall preserve the committed publication obligation until successful handoff acceptance is recorded or otherwise made provably durable under the approved C02 contract.

A process/runtime crash after the owner commit but before C02 acceptance shall not erase the publication obligation and shall not require re-execution of the business transaction merely to reconstruct it.

Technical handoff re-attempt may occur without re-running the business decision, subject to the approved B02/C02 architecture. Exact worker, scheduling, retry/backoff and broker mechanics are delegated to A08/C02/E-family architecture as applicable.

### 5.7 Stable Handoff Identity / Technical Idempotency

A durable publication intent shall carry a stable technical association to the committed source fact/event-publication obligation sufficient for B02/C02 to distinguish replay of the same handoff from creation of a new business fact.

Therefore:

- retrying the handoff shall reuse the same source publication obligation identity;
- a technical retry shall not generate a second business fact;
- B02 shall not invent a new canonical event name or publisher;
- C02 shall preserve the upstream canonical event identity/contract rules when turning the accepted handoff into publication/delivery work.

Exact event-contract fields remain C02-owned.

### 5.8 Commit / Handoff Failure Boundaries

| Failure Point | Required State | Required Recovery Boundary | Forbidden Outcome |
|---|---|---|---|
| Before owner-local commit | No authoritative state change and no committed publication intent. | Owner transaction may be safely retried/re-evaluated only under owning behavior. | Partial authoritative commit. |
| Concurrency conflict at commit | Stale/conflicting transaction does not overwrite newer state. | Surface conflict; re-evaluate/retry only through safe owner logic. | Last-write-wins by accident. |
| Business state write succeeds internally but transaction fails before commit | Entire local transaction rolls back. | Standard owner-local transaction recovery. | Business state committed without required intent. |
| Commit succeeds; process fails before C02 handoff | Business state and publication intent remain committed. | Resume/re-attempt technical handoff from durable intent. | Re-running the business transaction to recreate the event obligation. |
| C02 handoff attempt is ambiguous/fails | Publication intent remains eligible for safe re-attempt until acceptance can be established. | B02/C02 idempotent handoff boundary. | Creating a new business fact/event obligation on every retry. |
| C02 has accepted handoff | B02 state/handoff consistency responsibility is satisfied. | C02-002 owns durable publication/delivery/retry/failure treatment after acceptance. | B02 directly managing consumer delivery. |
| Consumer delivery fails after publication | Owner transaction remains committed. | C02-002/A020-equivalent delivery recovery path. | Rolling back the already committed source business fact solely because a consumer delivery failed. |

### 5.9 Read Consistency Boundary

A transaction shall read authoritative data only through the owner/scope boundary established by B02-002. Where one owner requires another owner's current fact, that fact shall be obtained through the applicable governed contract/projection/event boundary rather than by extending the local transaction into the peer database.

This specification does not promise one instantaneous globally consistent snapshot across all canonical owners.

### 5.10 Transaction Scope Discipline

Owner-local transactions shall contain only work necessary to preserve the atomic integrity of the owner's authoritative state and required technical publication intent.

Long-running human waits, cross-owner workflows, external-provider calls or consumer delivery acknowledgement shall not be kept inside an open database transaction merely to simulate end-to-end atomicity.

Where those activities are required, they occur outside the local commit boundary under their owning workflow/contract/event architecture.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Status | Basis | Consequence | ADR |
|---|---|---|---|---|---|
| `AD-001` | Authoritative transactions are owner-local; distributed ACID transactions across canonical owners are not the default SutraERP consistency model. | **Provisional Draft direction** | L1 single-owner rule; B02-001/002; Lead Architect selection 2026-09-11. | Preserves ownership and fault isolation; cross-owner consistency is coordinated through contracts/events/workflows. | **Required — `ADR-C3` formal scope/decision; see OD-001.** |
| `AD-002` | Optimistic concurrency is the default owner-local conflict-control strategy; stronger locking/serialization is allowed only for explicitly justified owner-local cases. | **Provisional Draft direction** | B02-003 responsibility; Lead Architect selection 2026-09-11. | Prevents silent stale overwrites without imposing universal pessimistic locking. | **Required — `ADR-C3`; see OD-001.** |
| `AD-003` | A state change requiring canonical event publication atomically commits authoritative state plus durable publication intent in the same owner-local transaction. | **Provisional Draft direction** | L4-A-019-BH-001/003/004; Lead Architect selection 2026-09-11. | Eliminates the committed-state/no-publication-obligation gap without requiring a distributed DB+broker transaction. | **Required — `ADR-C4`; see OD-002.** |
| `AD-004` | Publication intent is technical state, not the canonical event itself; actual publication occurs only after owner commit and preserves the upstream semantic publisher. | Defensible L5 technical derivation | L4-A-019-BH-001/004/007; ADR-C4 boundary. | Maintains upstream event meaning while allowing reliable handoff. | Governed with `ADR-C4`. |
| `AD-005` | Consumer delivery failure does not roll back an already committed source-domain business fact. | Established boundary consequence | L4-A-019-BH-006/008/009; C02 ownership. | Source state remains authoritative; delivery recovery is independently governed. | No separate ADR beyond C3/C4 boundary. |

No transaction isolation-level name, database lock primitive, broker transaction, distributed transaction coordinator, retry count or timeout is selected by this specification.

---

## 7. Security, Privacy, Quality & Operational Impact

| Concern | Architectural Impact | Owning / Collaborating L5 Spec |
|---|---|---|
| Ownership isolation | Transactions never gain peer-owner write authority to achieve atomicity. | B02-001/002/003 |
| Tenant isolation | A transaction and its publication intent remain inside the validated Tenant/Institution scope of the source operation. | A04-002, B02-002 |
| Data integrity | Optimistic conflict detection prevents silent stale overwrite; stronger serialization is available when justified. | B02-003 |
| Reliability | Atomic state+intent commit removes the post-commit publication gap; durable intent survives process failure until handoff. | B02-003, C02-002, E01 |
| Scalability | Owner-local transactions avoid repository-wide distributed lock/commit coupling. | B02-003, E02 |
| Operability | Pending/failed publication handoffs must be observable enough to detect a committed intent not yet accepted by C02. | B02-003, E05, D04 as applicable |
| Recoverability | Recovery shall preserve committed state, concurrency evidence and pending publication intent consistently. | E03, B02-003 |
| Portability | Architecture does not depend on a database+broker distributed transaction feature. | A02/E04/ADR governance |

---

## 8. Architecture Conformance Requirements

| CR ID | Conformance Requirement | Evidence Expected |
|---|---|---|
| `CR-001` | Every authoritative write transaction shall resolve to exactly one canonical Owner Persistence Boundary. | Transaction-to-owner mapping. |
| `CR-002` | A normal authoritative transaction shall not directly enlist another canonical owner's persistence in the same ACID commit. | Transaction/dependency review. |
| `CR-003` | Cross-owner workflows shall compose separately committed owner-local outcomes through governed boundaries rather than direct database mutation. | Workflow/contract/event data-flow evidence. |
| `CR-004` | Mutable authoritative state shall use concurrency-conflict detection sufficient to prevent silent stale overwrite; optimistic concurrency is the default Draft strategy. | Concurrency-control design and negative conflict tests. |
| `CR-005` | Stronger locking/serialization shall be explicitly justified, owner-local and limited to the necessary consistency boundary. | Architecture rationale and scope evidence. |
| `CR-006` | A concurrency conflict shall not partially commit the stale transaction or automatically repeat a non-idempotent business decision. | Failure-path verification. |
| `CR-007` | When a committed state change requires event publication, authoritative state and durable publication intent shall commit or roll back atomically. | Transaction design / failure-injection evidence. |
| `CR-008` | The durable publication intent shall not be treated as the published canonical Event Instance or as a transfer of semantic publisher ownership. | B02/C02 mapping and event-ownership review. |
| `CR-009` | A process failure after state+intent commit but before C02 acceptance shall leave a recoverable committed publication obligation. | Crash/recovery verification. |
| `CR-010` | Re-attempt of the state-to-C02 handoff shall be technically idempotent with respect to the same committed publication obligation. | Stable handoff identity / replay verification. |
| `CR-011` | Once C02 accepts the handoff, consumer routing/delivery/retry/acknowledgement responsibility shall remain C02-owned. | Cross-L5 responsibility review. |
| `CR-012` | Consumer-delivery failure shall not automatically roll back the already committed source-domain business fact. | Event failure-path verification. |
| `CR-013` | Long-running waits, cross-owner calls and consumer acknowledgement shall not be held inside one owner database transaction merely to simulate global atomicity. | Transaction-boundary review. |
| `CR-014` | Product-specific isolation levels, locking primitives or distributed transaction technology shall not be treated as canonical until applicable technology/ADR governance approves them. | ADR/technical-standard reference. |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Owner | Blocking Status | Resolution Path |
|---|---|---|---|---|
| `OD-001` | Formalize the owner-local transaction and optimistic-concurrency strategy represented by pilot candidate `ADR-C3`. | L5-B-02 / Lead Architect | **Blocks Architecture Approved for AD-001/AD-002.** | Reconfirm ADR necessity/scope, allocate canonical ADR ID on `main`, evaluate alternatives, approve/revise and propagate. |
| `OD-002` | Formalize the atomic state + durable publication-intent and B02→C02 handoff strategy represented by pilot candidate `ADR-C4`. | L5-B-02 / L5-C-02 governed boundary; Lead Architect for ADR scope | **Blocks Architecture Approved for AD-003/AD-004.** | Resolve Shared vs Repository scope, preserve B02/C02 ownership split, allocate canonical ADR ID and approve/revise. |
| `OD-003` | Select concrete database concurrency/isolation primitives after the primary relational database technology is approved. | L5-B-02 / A02 via technology ADR | Does not block product-neutral Draft architecture; blocks implementation standardization. | Map approved product capabilities to CR-004/005 without weakening semantics. |
| `OD-004` | Define exact C02 acceptance contract and post-handoff durable-delivery mechanics when `L5-C-02-002` is authored/stabilized. | L5-C-02 | Does not block Draft B02 boundary; required for end-to-end implementation. | C02-002 consumes this handoff boundary and owns delivery thereafter. |

No canonical L5 ADR number is allocated by this specification.

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-A-015-BH-004/005/006/007/010` — cross-domain orchestration coordinates owner results without direct participating-domain state mutation, including waiting/suspension/failure; this supports separately committed owner-local transactions rather than one long cross-owner database transaction.
- `L4-A-019-BH-001/003/004/007/010` — publication must represent a committed owner fact, preserve stable identity/traceability and retain the originating semantic publisher; this drives the atomic state+publication-intent and post-commit handoff architecture.

### Higher-Level / Peer Verification

- SutraERP L1 Business Domain Architecture v2.1 §§2.1/2.4 — singular canonical ownership and no direct cross-owner authoritative persistence access.
- `L5-B-02-001` — owner-scoped authoritative persistence baseline and B02-003 responsibility allocation.
- `L5-B-02-002` — owner/Tenant isolation boundary within which local transactions execute.
- `L5-A-04-002` — validated Tenant/Institution context remains mandatory at protected persistence.
- `L5_Pilot_ADR_Candidate_Disposition_2026-09-09.md` — `ADR-C3` and `ADR-C4` decision candidates and the B02-003/C02-002 handoff ownership split.

### Resulting Consistency Model

```text
ONE CANONICAL OWNER
        ↓
Owner-local transaction
        ├── authoritative state
        ├── optimistic concurrency guard
        └── durable publication intent (when required)
        ↓
atomic commit
        ↓
B02 durable handoff
        ↓
C02 accepted handoff
        ↓
C02 publication / routing / durable consumer delivery

CROSS-OWNER PROCESS
Owner A commit
        ↓
governed coordination
        ↓
Owner B commit
```

This specification remains `Draft` while `OD-001` and `OD-002` are unresolved and shall not be treated as an approved transaction/consistency standard until required ADR governance is complete.