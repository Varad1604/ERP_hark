# Completed Refund to Reversal or Recovery

**Scenario ID:** L3-F-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Refund Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00; L2-F01; L2-F02; L2-F03 Refund Management; applicable downstream Accounting/Treasury architecture  
**Referenced By:** Accounting & General Ledger; Treasury & Banking; Bank Reconciliation; Financial Reporting; audit consumers

> **Source-control note:** Exact F18 identity, author and filename are confirmed by the Phase 5 workload guide. The current indexed source does not expose the separate Stage 3 row containing the frozen Type/Priority values; those values must be mechanically reconciled before formal Review.

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-018 |
| Canonical Title | Completed Refund to Reversal or Recovery |
| Primary Owner | Refund Management |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the business outcome when a Refund that has already completed subsequently requires reversal or recovery, while preserving the completed Refund as historical fact and keeping any successor financial outcome under its canonical owning domain.

## 3. Business Outcome

A completed Refund is identified as requiring a governed reversal or recovery, and the resulting financial position is established through an authoritative business resolution without overwriting the original completed Refund.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Refund is in the canonical `Completed` position and a subsequent business fact establishes that reversal or recovery is required.

### 4.2 End Boundary

The scenario ends when the reversal/recovery outcome is established and the resulting financial responsibility is transferred to the canonical owning business process for any successor obligation or recovery position.

### 4.3 In Scope

- Identification of a completed Refund requiring reversal/recovery.
- Preservation of the original Refund and its completion history.
- Business determination of the reversal/recovery outcome.
- Cross-domain financial consequences.
- Canonical Refund event publication.
- Audit and evidence preservation.

### 4.4 Out of Scope

- Original Refund creation, approval or execution.
- Technical reversal implementation.
- Accounting journal mechanics.
- Bank reconciliation mechanics.
- Creation of a new Refund lifecycle/state not defined upstream.
- Technical retries, rollback or infrastructure recovery.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership |
|---|---|---|
| C — Student Lifecycle | Provides authoritative student context where relevant. | Student context remains C-owned. |
| F — Refund Management | Owns the historical Refund and its Refund lifecycle/events. | Refund remains F03-owned. |
| Accounting & General Ledger | Owns accounting consequences of a financial reversal/recovery. | Accounting remains F05-owned. |
| Treasury & Banking / Bank Reconciliation | Owns banking/reconciliation consequences where applicable. | Banking/reconciliation remain separately owned. |

A downstream reversal or recovery consequence does not transfer ownership of the original Refund.

## 6. Business Actors, Roles and Responsibilities

L2-F03 does not define a specific human actor for post-completion reversal/recovery.

The authoritative responsibilities are therefore expressed by domain:

- Refund Management preserves the original Refund and its business history.
- The domain owning the resulting financial consequence governs that consequence.
- Accounting/Treasury/Bank Reconciliation own their respective artifacts and outcomes.

## 7. Preconditions and Entry Criteria

- A canonical Refund exists.
- The Refund has reached `Completed`.
- A subsequent authoritative business condition indicates that the completed Refund requires reversal or recovery.
- The original financial lineage remains reconstructable.

## 8. Trigger

New authoritative evidence establishes that a completed Refund must be reversed or that financial recovery is required.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the completed Refund and the subsequent evidence requiring reversal/recovery. | Refund Management | A post-completion financial issue is established against a historical Refund. |
| 2 | Preserve the completed Refund as an immutable historical business fact. | Refund Management | Original Refund history remains intact. |
| 3 | Determine the authoritative business consequence of the reversal/recovery. | Refund Management with applicable Finance domain | A governed successor financial outcome is established. |
| 4 | Where the consequence belongs to another Finance capability, hand the resulting business context to that canonical owner. | Refund Management / owning Finance domain | Ownership of the successor financial consequence remains correct. |
| 5 | Record and publish the canonical Refund fact associated with the reversal/recovery. | Refund Management | Enterprise consumers can reconstruct the relationship between original Refund and subsequent outcome. |
| 6 | Preserve the complete financial lineage across the original Refund and the successor outcome. | Refund Management / successor owner | Historical and current financial positions remain distinguishable. |

## 10. Alternate Business Flows

### A1 — Recovery Is Required Without Reversing the Original Historical Fact

The original Completed Refund remains unchanged; the recovery obligation is represented by the canonical owning Finance process.

### A2 — Evidence Does Not Establish a Reversal/Recovery Outcome

The Refund remains historically Completed and no reversal is recorded until authoritative evidence exists.

### A3 — Reversal Impacts Accounting or Banking

The Refund Domain preserves the originating Refund fact while Accounting/Treasury/Bank Reconciliation governs its own downstream consequence.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Evidence is insufficient to establish reversal/recovery | Completed Refund cannot safely be treated as reversed. | Preserve historical completion and resolve the evidence question through the owning business authority. | Refund Management / governing domain |
| Successor financial owner is not established | Reversal/recovery cannot be assigned without violating ownership. | Preserve the Refund and escalate the ownership question; do not invent a successor aggregate. | Finance architecture governance |
| Downstream financial consequence is unresolved | Original Refund remains completed, but successor consequence is not complete. | Allow the canonical downstream owner to govern its own resolution. | Owning Finance domain |

No technical rollback is defined. The original Completed Refund is not overwritten.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Application |
|---|---|---|
| Completed Refund milestones are historical and immutable. | Refund Management | Reversal cannot erase the original Refund completion fact. |
| Refund remains the canonical outbound refund aggregate. | Refund Management | A reversal does not create a second Refund aggregate by default. |
| External domains do not modify Refunds directly. | Refund Management | Successor domains consume/refine their own business consequences. |
| Financial truth remains owned by Finance. | Section F | Any recovery consequence remains within the canonical Finance architecture. |
| Accounting and banking artifacts remain independently owned. | F05/F10 | L3 does not redefine accounting or banking ownership. |

## 13. State and Lifecycle Impact

L2-F03 defines `Completed` as a canonical Refund lifecycle/state position and does not define a separate `Reversed` Refund lifecycle stage.

Therefore:

- the original Refund remains `Completed` as the historical fact;
- L3 does not invent a `Reversed` state;
- a subsequent recovery/reversal consequence must be represented by the canonical downstream business capability that owns that consequence.

This preserves the distinction between historical Refund completion and a later financial recovery event.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Refund Events

L2-F03 defines canonical Refund events including:

- `RefundCompleted`
- `RefundCancelled`
- `RefundSuspended`
- `RefundArchived`

The accessed F03 event architecture does **not** define a canonical `RefundReversed` or `RefundRecoveryRequired` event.

Therefore no new reversal event is invented in this L3.

### 14.2 Business Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Refund Management | Accounting & General Ledger | Completed Refund reversal/recovery context | Enables accounting-owned financial consequence. |
| Refund Management | Treasury & Banking / Bank Reconciliation | Completed Refund financial evidence | Enables banking/reconciliation-owned consequence where applicable. |
| Refund Management | applicable Finance owner | Original Refund lineage and recovery context | Establishes the successor financial outcome without transferring Refund ownership. |

Events and synchronous hand-offs remain distinct.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

No specific human reversal/recovery authority is defined in L2-F03. No actor or approval hierarchy is invented.

### 15.2 Separation of Duties

No Refund-specific post-completion separation-of-duty rule is defined upstream.

### 15.3 Audit / Evidence

The scenario preserves:

- original Refund reference;
- original approval and execution history;
- completion evidence;
- subsequent evidence establishing reversal/recovery need;
- relationship between original Refund and successor financial outcome;
- all applicable canonical business events.

The original Completed Refund must remain reconstructable independently of the successor outcome.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- The completed Refund is identified as requiring reversal/recovery based on authoritative evidence.
- The original Refund remains historically intact.
- The successor financial consequence has a canonical owner.
- The resulting relationship between the original Refund and successor financial outcome is traceable.
- No unsupported `Reversed` Refund state/event is introduced.

### Postconditions

- Original Refund remains a completed historical fact.
- Any recovery/reversal financial consequence is governed by its canonical Finance owner.
- Accounting/banking ownership remains independent.
- Audit reconstruction can distinguish original completion from later recovery.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-F03 does not define a canonical Refund reversal/recovery lifecycle state or event. | The L3 cannot create a Refund-level `Reversed` state or `RefundReversed` event. | Confirm the canonical owner and representation of post-completion Refund reversal/recovery. |
| The accessed F03 architecture does not define the exact downstream Finance process for recovery after a Completed Refund. | Detailed successor outcome cannot be authored without risking ownership leakage. | Identify the authoritative downstream L2 capability and its business contract/event for recovery. |
| Frozen Stage 3 Type/Priority row is not available in the indexed source extract. | Classification metadata requires mechanical reconciliation before formal approval. | Reconcile against the frozen Stage 3 canonical row. |

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No generic implementation questions are deferred.
