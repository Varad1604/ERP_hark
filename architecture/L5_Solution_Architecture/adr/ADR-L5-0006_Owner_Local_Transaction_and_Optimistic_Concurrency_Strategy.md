# ADR-L5-0006 — Owner-Local Transaction and Optimistic Concurrency Strategy

**ADR ID:** `ADR-L5-0006`  
**Title:** Owner-Local Transaction and Optimistic Concurrency Strategy  
**Scope:** Shared / Family  
**Decision Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Architecture remediation for Dhruv Stage-1 corpus  
**Reviewers:** SAOS architecture review; promotion explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-11  
**Affected L5 Specifications:** `L5-B-02-003`  
**Related ADRs:** `ADR-L5-0005`; `ADR-L5-0007`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

SutraERP needs a reusable transaction/concurrency strategy that preserves canonical owner boundaries. `L5-B-02-003` defines owner-local transaction boundaries, optimistic concurrency as the default conflict-management approach, and explicit rejection of cross-owner distributed ACID as the normal integration model. Pilot candidate `ADR-C3` identified this as a material Shared / Family persistence decision.

## 2. Scope & Affected Architecture

This ADR governs transaction-boundary placement and default concurrency strategy for authoritative owner-local persistence. It does not select product-specific isolation levels, locking primitives, distributed transaction coordinators or database products.

## 3. Authoritative Constraints

1. Canonical ownership shall be preserved by transaction boundaries.
2. Cross-owner collaboration shall not rely on one distributed ACID transaction by default.
3. Long-running waits, remote calls and consumer acknowledgement shall not be held inside one owner-local database transaction merely to simulate global atomicity.
4. Concurrency conflicts must be explicit and detectable.
5. Stronger pessimistic locking is allowed only where a justified invariant requires it.
6. Product-specific mechanisms remain downstream technology/implementation choices until governed.

## 4. Decision Criteria

Correctness, owner isolation, recoverability, scalability, deadlock/contention risk, portability, failure transparency and implementation neutrality.

## 5. Alternatives Considered

### A — Distributed ACID across owners by default
Rejected: couples owners and infrastructure, obscures failure boundaries and conflicts with repository collaboration rules.

### B — Pessimistic locking as universal default
Rejected: unnecessary contention and operational coupling for many workflows.

### C — Owner-local transactions with optimistic concurrency by default
Selected.

## 6. Evaluation

| Criterion | A | B | C |
|---|---|---|---|
| Ownership preservation | Weak | Strong | Strong |
| Portability | Weak | Acceptable | Strong |
| Contention/deadlock risk | Weak | Weak | Strong |
| Failure transparency | Weak | Acceptable | Strong |
| Scalability | Weak | Weak/Acceptable | Strong |

## 7. Decision

SutraERP shall use **owner-local transactions** for authoritative state changes, with **optimistic concurrency as the default concurrency strategy**.

- A transaction shall not cross canonical owner boundaries as the normal collaboration mechanism.
- Version/precondition conflict detection shall be explicit where concurrent mutation is possible.
- Pessimistic locking or stronger serialization may be used only for a justified invariant and only within the canonical owner boundary.
- Cross-owner progression uses contracts/events/workflows and explicit failure/retry/compensation behavior owned by the relevant collaborating architecture.
- Product-specific isolation levels or locking primitives are not selected by this ADR.

## 8. Consequences

Positive: clear ownership, reduced coupling, scalable concurrency, technology neutrality, explicit conflict handling.  
Cost: applications must handle optimistic-conflict outcomes and cannot assume global atomicity across owners.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Lost updates | Explicit version/precondition checks. |
| Overuse of pessimistic locking | Require invariant-based justification and local scope. |
| Hidden distributed transaction | Conformance rejects remote waits/calls inside owner-local transaction as global-atomicity substitutes. |
| Cross-owner inconsistency | Use governed asynchronous/synchronous collaboration and ADR-L5-0007 where publication handoff applies. |

## 10. Migration / Exit Position

No product-specific migration is implied. A future strategy change must preserve canonical ownership and explicitly re-evaluate coupling, failure and portability consequences.

## 11. Traceability

Primary realization: `L5-B-02-003_Transaction_Consistency_and_Concurrency_Architecture.md`.  
Pilot provenance: `ADR-C3`.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** ADR necessity and B02 Shared / Family scope were confirmed; the decision remains product-neutral and preserves repository owner boundaries. Controlled remediation was explicitly authorized on 2026-09-11. This approval does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| B02 technical decision owner | Recommended | 2026-09-10 | B02-003 architecture |
| SAOS architecture reviewer | PASS | 2026-09-11 | Stage-1 production architecture review |
| Architecture promotion authorization | Approved | 2026-09-11 | Explicit remediation authorization |
