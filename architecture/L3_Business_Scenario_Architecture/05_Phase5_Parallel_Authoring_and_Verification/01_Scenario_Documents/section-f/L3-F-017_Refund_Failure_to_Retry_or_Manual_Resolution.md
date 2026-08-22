# Refund Failure to Retry or Manual Resolution

**Scenario ID:** L3-F-017  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Refund Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00; L2-F01; L2-F02; L2-F03 Refund Management  
**Referenced By:** Payment execution capability; Accounting & General Ledger; Treasury/Banking; Reporting and audit consumers

> **Source-control note:** Exact F17 identity, author and filename are confirmed by the Phase 5 workload guide. Frozen Stage 3 Type/Priority values should be mechanically reconciled before formal Review because the indexed Stage 3 row is not available in the current source extract.

## 2. Scenario Purpose

Govern an outbound Refund that cannot complete normally and requires a governed retry or manual business resolution, while keeping failure handling at business level and preserving the canonical Refund aggregate, lifecycle and event ownership.

## 3. Business Outcome

A failed Refund reaches a governed business resolution: it is successfully progressed through an authorised subsequent execution attempt where such progression is defined, or it is resolved manually into an authoritative non-completed outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an approved/published Refund has an authoritative execution failure or unresolved execution outcome.

### 4.2 End Boundary

The scenario ends when the Refund has either progressed to a successfully executed/completed outcome or reached an authoritative manual-resolution outcome within the Refund architecture.

### 4.3 In Scope

- Recognition of execution failure.
- Preservation of the failed Refund state and history.
- Governed business retry where permitted.
- Manual business resolution where retry is not sufficient or not applicable.
- State/lifecycle consequences.
- Canonical Refund events.
- Audit and financial lineage.

### 4.4 Out of Scope

- Technical retry counts or retry algorithms.
- Queues, message delivery, infrastructure failure, server errors, database rollback or exception handling.
- Payment execution implementation.
- Accounting posting mechanics.
- Creation of new Refund aggregates.

## 5. Ownership and Participating Domains

| Domain / Capability | Responsibility | Ownership |
|---|---|---|
| C — Student Lifecycle | Supplies student context where relevant. | Student context remains C-owned. |
| F — Refund Management | Owns Refund state, lifecycle, business resolution and Refund events. | Refund remains F03-owned. |
| Payment execution capability | Produces the business outcome of outbound monetary execution. | Execution does not transfer Refund ownership. |
| Accounting / Treasury & Banking | Consume or govern their own downstream financial consequences. | Their canonical artifacts remain separately owned. |

## 6. Business Actors, Roles and Responsibilities

L2-F03 does not define a canonical human actor for failed Refund resolution. No manual-resolution role is invented.

The scenario therefore uses domain/capability responsibilities only:

- Refund Management governs the Refund business outcome.
- The payment execution capability provides the execution result.
- Downstream Finance domains govern their own consequences.

## 7. Preconditions and Entry Criteria

- A canonical Refund exists.
- The Refund has reached an execution-eligible position.
- An authoritative execution failure or unresolved execution outcome exists.
- Refund financial lineage and references remain available.

## 8. Trigger

An authoritative Refund execution outcome indicates that the outbound monetary settlement did not complete successfully.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Record the authoritative failed execution outcome against the canonical Refund. | Refund Management | Refund is represented in the applicable failure state. |
| 2 | Determine whether the Refund is eligible for a governed subsequent execution attempt. | Refund Management / governing capability | A business resolution path is selected. |
| 3 | Where a subsequent attempt is permitted, re-establish the Refund as eligible for execution under its canonical business lifecycle/state rules. | Refund Management | Refund can progress toward execution again. |
| 4 | Where retry is not appropriate or does not resolve the business outcome, place the Refund into the applicable manual-resolution path. | Refund Management | A governed manual resolution is required. |
| 5 | Record the resulting business resolution and preserve all prior failure history. | Refund Management | Refund history remains reconstructable. |
| 6 | Publish the relevant canonical Refund business facts. | Refund Management | Downstream consumers receive authoritative outcomes. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Path | Result | Responsible Domain |
|---|---|---|---|---|
| A1 | Failure is resolved through an authorised subsequent execution attempt | Refund progresses through the normal execution lifecycle. | Refund may reach `Executed` and subsequently `Completed`. | Refund Management / execution capability |
| A2 | Failure cannot be resolved through normal execution | Refund enters an applicable manual business resolution path. | Refund remains outside `Completed` until a valid final outcome is established. | Refund Management |
| A3 | Failure is associated with cheque execution | Apply the canonical `Stale Cheque` state where applicable. | Refund remains unresolved pending governed resolution. | Refund Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Outbound Refund execution fails | Refund cannot be marked Executed or Completed. | Preserve `Execution Failed` and apply the governed business resolution. | Refund Management |
| A further execution attempt is permitted | Refund remains a valid business obligation to execute. | Progress through the authorised execution path. | Refund Management / execution capability |
| Manual resolution is required | Automated progression is insufficient to establish a financial outcome. | Resolve the Refund through an authorised business decision and preserve evidence. | Refund Management / governing authority |
| Cheque becomes stale | Refund remains financially unresolved. | Preserve `Stale Cheque` and use the governed business resolution. | Refund Management |

Technical retries are intentionally not specified.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Application |
|---|---|---|
| Refund has one canonical aggregate owner. | Refund Management | Failure handling does not create a second Refund. |
| Refund state is canonical and singular. | Refund Management | Failure and recovery are represented through existing states only. |
| `Execution Failed` and `Stale Cheque` are canonical F03 states. | Refund Management | These states may be used where their business conditions apply. |
| Lifecycle milestones remain immutable once completed. | Refund Management | Failed attempts do not erase prior history. |
| Execution is externally performed; Refund Management records the resulting milestone. | Refund Management / execution capability | Execution ownership remains separate from Refund ownership. |

## 13. State and Lifecycle Impact

The canonical Refund operational states include:

`Pending Execution → Executed → Completed`

with `Execution Failed` and `Stale Cheque` as defined exceptional states.

The scenario may therefore move from an execution-eligible state into `Execution Failed` or `Stale Cheque`, followed by a governed business resolution. It must not invent a state such as `Retrying`, `Manual Review`, or `Retry Exhausted`.

The Refund lifecycle remains:

`Refund Initiated → Validated → Approved → Published → Executed → Completed → Archived`

Failure handling does not create a new lifecycle.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Events

L2-F03 defines:

- `RefundExecutionStarted`
- `RefundExecutionFailed`
- `RefundExecuted`
- `RefundCompleted`
- `RefundChequeDishonourRecorded`
- `RefundSuspended`
- `RefundCancelled`

Only materially relevant events are used in this scenario.

The Refund Domain remains the canonical publisher.

### 14.2 Business Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Refund Management | Payment execution capability | Refund execution context | Enables outbound execution. |
| Payment execution capability | Refund Management | Execution outcome | Establishes failure or success. |
| Refund Management | downstream Finance domains | Canonical Refund events/facts | Preserves enterprise financial synchronization. |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

L2-F03 does not define the human authority for manual Refund resolution. No role is invented.

### 15.2 Approval / Separation of Duties

Where manual resolution requires governance, the existing enterprise approval capability applies. L3 does not define a new approval hierarchy.

### 15.3 Audit / Evidence

The scenario must preserve:

- original Refund reference;
- execution outcome;
- failure reason where authoritative;
- state history;
- subsequent resolution;
- resulting execution/completion fact where applicable;
- Refund event history.

The evidence remains part of the Refund business history; canonical audit infrastructure remains platform-owned.

## 16. Completion Criteria and Postconditions

### Completion Criteria

One of the following is reached:

1. Refund progresses successfully to `Executed` and, where its business responsibilities are complete, `Completed`; or
2. Refund reaches an authoritative manual/non-completed resolution permitted by the upstream architecture.

### Postconditions

- Failed execution history remains immutable and reconstructable.
- No technical retry mechanism is defined.
- Refund ownership remains with Refund Management.
- Downstream domains receive only authoritative business facts.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| F03 defines `Execution Failed` and `Stale Cheque` but does not specify the exact business eligibility criteria for another execution attempt versus manual resolution. | L3 cannot safely define retry eligibility rules. | Establish the authoritative business retry/manual-resolution policy upstream. |
| F03 does not identify the canonical owner of the outbound Refund execution capability. | The L3 must preserve the external execution boundary without inventing an owner. | Confirm the canonical execution capability/owner and its business outcome contract. |
| Frozen Stage 3 Type/Priority row is not available in the indexed source extract. | Classification metadata requires mechanical reconciliation before approval. | Reconcile against the frozen Stage 3 row. |

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No generic implementation questions are deferred to later levels.
