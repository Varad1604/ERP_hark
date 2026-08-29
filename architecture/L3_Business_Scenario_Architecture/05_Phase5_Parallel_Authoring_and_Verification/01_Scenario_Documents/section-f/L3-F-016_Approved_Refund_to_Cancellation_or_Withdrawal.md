# Approved Refund to Cancellation or Withdrawal

**Scenario ID:** L3-F-016  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Refund Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; L2-F02 Payment Management; L2-F03 Refund Management  
**Referenced By:** downstream Refund, Accounting, Reporting and audit consumers

> **Source-control note:** The Phase 5 workload guide confirms the exact final ID, title, author and filename for F16. The currently indexed Stage 3 semantic extract does not expose a separate row for F16's frozen Type/Priority values; the classification above follows the locked Finance/Refund classification context and must be mechanically checked against the Stage 3 row before approval.

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-016 |
| Canonical Title | Approved Refund to Cancellation or Withdrawal |
| Primary Owner | Refund Management |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the business outcome in which an already approved Refund is cancelled or withdrawn before it completes its outbound financial responsibility, while preserving the Refund aggregate, approval history, financial lineage and canonical Refund lifecycle/state ownership.

This scenario does not redesign approval, payment execution, accounting, or cancellation policy.

## 3. Business Outcome

An approved Refund is prevented from progressing as an active payable refund when a governed cancellation or withdrawal condition applies, and the Refund's resulting business position is recorded without transferring ownership or rewriting historical approval evidence.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins with a Refund in the canonical `Approved` position and a business condition that requires cancellation or withdrawal before financial completion.

### 4.2 End Boundary

The scenario ends when the Refund has reached the authoritative governed terminal/interrupted position permitted by the Refund architecture, or when the cancellation/withdrawal request is rejected and the Refund remains governed in its prior valid position.

### 4.3 In Scope

- Identification of an approved Refund requiring cancellation or withdrawal.
- Preservation of Refund Source, Reason, Reference and financial amounts.
- Business decision to cancel or withdraw.
- Resulting Refund state/lifecycle consequences where canonically defined.
- Preservation of approval and audit evidence.
- Publication of a canonical cancellation fact where defined upstream.

### 4.4 Out of Scope

- Creation or approval of the original Refund request.
- Outbound payment execution.
- Accounting reversal or recovery after a completed Refund.
- Technical cancellation mechanics.
- Technical retries, queues, rollback, infrastructure recovery or implementation details.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership |
|---|---|---|
| C — Student Lifecycle | Provides authoritative student context where the Refund originated from a student lifecycle circumstance. | Student context remains C-owned. |
| F — Refund Management | Owns the Refund aggregate, lifecycle, state, references, validation and Refund events. | Refund remains F03-owned. |
| Enterprise Governance | Provides approval/governance capability where a cancellation decision is governed externally. | Approval governance remains externally owned. |

Participation does not transfer Refund ownership.

## 6. Business Actors, Roles and Responsibilities

L2-F03 does not define a canonical actor list for Refund cancellation or withdrawal. No specific finance officer, approver, committee or user role is invented.

The authoritative responsibilities are therefore expressed at capability/domain level:

- Refund Management governs the Refund business record.
- Enterprise Governance owns externally governed approval workflow where applicable.
- Student Lifecycle supplies authoritative student context where applicable.

## 7. Preconditions and Entry Criteria

- A canonical Refund exists.
- The Refund has reached `Approved` or another applicable non-completed position from which cancellation/withdrawal is permitted.
- The Refund's financial context remains traceable.
- A business condition requiring cancellation or withdrawal has been established.

## 8. Trigger

A governed business condition or request requires an approved Refund to be cancelled or withdrawn before it reaches financial completion.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the approved Refund and the business condition requiring cancellation or withdrawal. | Refund Management | A cancellation/withdrawal case is established against the canonical Refund. |
| 2 | Preserve the Refund's source, reason, references, amounts and prior approval history. | Refund Management | Financial lineage remains intact. |
| 3 | Apply the authoritative cancellation/withdrawal governance applicable to the Refund. | Refund Management / Enterprise Governance | A governed decision is reached. |
| 4 | Where cancellation/withdrawal is accepted, move the Refund only through an already-defined canonical state/lifecycle transition. | Refund Management | The Refund no longer proceeds as an active approved Refund. |
| 5 | Preserve the cancellation/withdrawal outcome and associated evidence. | Refund Management | The historical Refund record remains reconstructable. |
| 6 | Publish the canonical Refund cancellation fact where the authoritative event architecture defines one. | Refund Management | Downstream consumers receive an authoritative business fact. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result | Responsible Domain |
|---|---|---|---|---|
| A1 | Cancellation/withdrawal request is not accepted | The Refund remains in its current valid business position and continues under the existing lifecycle. | No cancellation is recorded as completed. | Refund Management / Enterprise Governance |
| A2 | Cancellation is identified after execution has already completed | The request is outside this scenario boundary and proceeds to the separate completed-Refund reversal/recovery scenario. | F18 governs the completed-Refund outcome. | Refund Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Cancellation authority is not established | No authoritative cancellation outcome can be claimed. | Preserve the Refund and resolve the governing authority before changing its business position. | Enterprise Governance / Refund Management |
| Cancellation condition is not satisfied | Refund remains eligible for its existing lifecycle progression. | Continue under the existing Refund lifecycle; do not manufacture a cancellation state. | Refund Management |
| Refund has already reached financial completion | Cancellation is no longer the appropriate business path. | Use the completed-Refund reversal/recovery scenario. | Refund Management |

No technical recovery mechanism is defined.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Application |
|---|---|---|
| Refund is the canonical outbound monetary aggregate. | Refund Management | Cancellation modifies the business position of the Refund; it does not create another refund aggregate. |
| External domains do not modify Refunds directly. | Refund Management | Participating domains cannot directly change the Refund. |
| Refund history and references remain traceable. | Refund Management | Cancellation cannot erase prior approval or financial evidence. |
| Lifecycle and state remain owned by Refund Management. | Refund Management | No new cancellation lifecycle/state is introduced by L3. |
| Approval workflow, routing, rules and hierarchy are externally owned. | Enterprise Governance | L3 does not invent a cancellation approver or approval hierarchy. |

## 13. State and Lifecycle Impact

L2-F03 defines the Refund lifecycle as:

`Refund Initiated → Validated → Approved → Published → Executed → Completed → Archived`

The canonical operational states include `Draft`, `Pending Validation`, `Validated`, `Pending Approval`, `Approved`, `Published`, `Pending Execution`, `Executed`, `Execution Failed`, `Stale Cheque`, `Completed`, `Suspended`, `Cancelled` and `Archived`.

The authoritative architecture therefore supports a `Cancelled` operational state, but does not define a separate `Withdrawn` state or a dedicated cancellation lifecycle milestone.

This scenario uses only the canonical `Cancelled` state where the upstream cancellation outcome is applicable. It does not invent `Withdrawn` as a new state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Events

L2-F03 defines `RefundCancelled` among its canonical Refund lifecycle events.

The Refund Domain is the canonical publisher.

No `RefundWithdrawn` event is invented because it is not defined in the accessed L2 event vocabulary.

### 14.2 Business Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Student Lifecycle | Refund Management | Authoritative student context | Establishes relevant financial context where applicable. |
| Refund Management | Enterprise Governance | Refund cancellation decision context | Obtains the externally governed decision where required. |
| Enterprise Governance | Refund Management | Cancellation/decision outcome | Determines whether the Refund can be cancelled. |
| Refund Management | downstream Finance consumers | `RefundCancelled` business fact | Communicates the completed cancellation fact. |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

No specific cancellation actor or authority is defined in L2-F03. The scenario therefore does not assign a human role.

### 15.2 Approval and Separation of Duties

Enterprise approval is externally owned. No cancellation-specific maker/checker rule is invented.

### 15.3 Audit / Evidence

The business record must preserve:

- original Refund reference;
- Refund Source and Reason;
- approved status/history;
- cancellation/withdrawal decision;
- resulting Refund state;
- relevant Refund events;
- financial lineage.

Section A remains the owner of canonical audit infrastructure; the Refund Domain owns the underlying Refund business facts and evidence obligations.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- The Refund is identified as eligible for cancellation/withdrawal.
- The authoritative business decision is established.
- If accepted, the Refund reaches the applicable canonical `Cancelled` position.
- `RefundCancelled` is published as the canonical cancellation fact.
- Historical Refund evidence remains intact.

### Postconditions

- The Refund no longer progresses as an active approved refund when cancellation is accepted.
- Previous approval and financial history remain reconstructable.
- No new Refund aggregate is created.
- Completed Refund reversal/recovery remains outside this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| F03 defines `Cancelled` as a canonical state but does not specify the exact business conditions under which an Approved Refund may be cancelled versus withdrawn. | The L3 can preserve the cancellation boundary but cannot invent eligibility criteria. | Confirm the authoritative cancellation/withdrawal policy and permitted transitions. |
| F03 event vocabulary includes `RefundCancelled` but does not define a separate `RefundWithdrawn` event. | Withdrawal cannot be represented as a distinct event without upstream architecture. | Confirm whether withdrawal is semantically represented by `RefundCancelled` or requires an upstream event definition. |
| Frozen Stage 3 Type/Priority row was not available in the indexed source extract used for this draft. | Metadata classification must be mechanically verified before Review/Approval. | Reconcile against the frozen Stage 3 canonical row. |

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No scenario-specific L4/L5/L6 question is required at this stage.
