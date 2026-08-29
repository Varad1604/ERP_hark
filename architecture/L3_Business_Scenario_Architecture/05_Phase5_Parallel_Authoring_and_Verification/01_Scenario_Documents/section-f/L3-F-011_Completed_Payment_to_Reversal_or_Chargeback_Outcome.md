# Completed Payment to Reversal or Chargeback Outcome

**Scenario ID:** L3-F-011  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F00 Finance Domain Model & Foundation; L2-F02 Payment Management Architecture; L2-F03 Refund Management; L2-F04 Financial Adjustment Management; L2-F05 Accounting & General Ledger Management  
**Referenced By:** Accounts Receivable, Student Charge & Fee Management, Accounting & General Ledger, Refund Management, Financial Adjustment Management

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-011 |
| Canonical Title | Completed Payment to Reversal or Chargeback Outcome |
| Primary Owner | Payment Management |
| Participating Domains | F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the institutional financial consequence when a previously completed Payment must be reversed or is subject to a chargeback outcome, while preserving the original Payment history and ensuring that any resulting financial correction remains under the canonical owning Finance domain.

The scenario distinguishes Payment ownership from downstream financial correction. A Payment is never silently overwritten to erase the historical settlement.

## 3. Business Outcome

A completed Payment is brought to an authoritative reversal or chargeback-related financial outcome, with the original Payment history preserved and all resulting financial consequences represented by their canonical owning Finance aggregates.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authoritative business condition requires a previously completed Payment to be reversed or when a chargeback outcome affecting that Payment is established.

### 4.2 End Boundary

The scenario ends when the Payment's authoritative historical position and the resulting financial consequence are established and traceable under the applicable Finance ownership boundaries.

### 4.3 In Scope

- Recognition of an authoritative reversal/chargeback business condition.
- Preservation of the original Payment.
- Determination of the Payment's allowed resulting state under existing Payment architecture.
- Financial consequences for the related Student Charge or receivable where applicable.
- Use of Refund or Financial Adjustment domains where their aggregates own the resulting correction.
- Accounting consequence where applicable.
- Complete financial traceability.

### 4.4 Out of Scope

- Editing or deleting the historical Payment.
- Inventing a new Payment reversal state.
- Defining chargeback provider mechanics.
- Refund execution.
- Financial adjustment approval rules not defined upstream.
- Accounting implementation mechanics.
- Technical integration protocols, retries, queues or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| F — Payment Management | Owns the historical Payment and any Payment state/lifecycle consequences defined by F02. | Payment remains Payment Management-owned. |
| F — Student Charge & Fee Management | Owns the underlying financial obligation and its outstanding-balance consequence. | Student Charge remains F01-owned. |
| F — Refund Management | Owns Refund where the resulting financial action is a Refund. | Refund remains F03-owned. |
| F — Financial Adjustment Management | Owns financial adjustments where the correction is an adjustment. | Adjustment remains F04-owned. |
| F — Accounting & General Ledger | Owns accounting correction and ledger history. | Accounting remains F05-owned. |

The frozen Stage 3 participation value is F only. These adjacent Finance capabilities are not added to the metadata as participating domains unless the concrete reversal/chargeback outcome invokes their canonical aggregate. The Stage 3 participation value remains unchanged.

## 6. Business Actors, Roles and Responsibilities

The accessed L2 documents do not provide an authoritative actor catalogue for Payment reversal or chargeback. No chargeback officer, payment administrator, approver, or bank operator is invented.

Authorization for Payment remains externally governed; any approval or adjustment authority belongs to the canonical owning domain if explicitly defined there.

## 7. Preconditions and Entry Criteria

- A completed Payment exists.
- An authoritative business condition indicates that the completed Payment's financial outcome must be reversed or is affected by chargeback.
- The original Payment and its references remain available.
- The appropriate downstream financial owner can be identified for any resulting correction.

## 8. Trigger

An authoritative reversal or chargeback condition is established against a completed Payment.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | An authoritative business condition establishes that a completed Payment requires reversal or is affected by chargeback. | Canonical Finance source / Payment Management | A reversal-related business condition exists. |
| 2 | Payment Management identifies the canonical Payment and preserves its historical identity and references. | Payment Management | The original settlement remains reconstructable. |
| 3 | Payment Management determines the applicable consequence using only the Payment states and lifecycle transitions authorized by F02. | Payment Management | No unsupported Payment state is introduced. |
| 4 | Where the consequence affects the underlying financial obligation, the Student Charge domain receives the authoritative Payment consequence and maintains its own obligation state. | Student Charge & Fee Management | Outstanding financial position remains Student Charge-owned. |
| 5 | Where the consequence requires a Refund or Financial Adjustment, the corresponding owning domain governs that aggregate according to its own architecture. | Refund / Financial Adjustment | Correction ownership remains canonical. |
| 6 | Accounting & General Ledger records any required accounting consequence under its own immutable posting/reversal architecture. | Accounting & General Ledger | Financial books remain historically correct. |
| 7 | The resulting business facts and references remain fully traceable across the Payment and downstream financial consequences. | Participating Finance domains | The institutional financial position is reconstructable. |

## 10. Alternate Business Flows

### A1 — Reversal Produces a Student Charge Consequence

The Payment consequence affects the amount previously settled against a Student Charge. Student Charge Management maintains the resulting obligation/outstanding-balance position; Payment ownership is unchanged.

### A2 — Reversal Requires a Financial Adjustment

Where the correction is an adjustment rather than a Refund, Financial Adjustment Management owns the resulting adjustment aggregate.

### A3 — Reversal Requires a Refund

Where the canonical financial outcome is an outbound monetary return, Refund Management owns the Refund aggregate. Refund Management explicitly states that Refunds do not create or modify Payments.

### A4 — Accounting Correction Required

Accounting & General Ledger applies its own reversal architecture while preserving the original accounting history.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Reversal evidence is insufficient | Completed Payment cannot safely be reversed | Preserve the original Payment and do not claim a reversal outcome until authoritative evidence exists. | Payment Management |
| Chargeback condition conflicts with existing Payment history | Financial position is disputed | Preserve the historical Payment and resolve the authoritative business fact before changing downstream financial position. | Payment Management / owning Finance domain |
| Payment consequence requires another Finance aggregate | Payment alone cannot represent the entire correction | Hand the authoritative financial fact to the owning Refund, Adjustment, Student Charge or Accounting domain. | Canonical owning domain |
| Historical record would need to be overwritten | Financial traceability would be lost | Do not overwrite; represent the correction through the canonical downstream correction mechanism. | Payment Management / Accounting |

No technical chargeback protocol, retry, queue, rollback or provider handling is defined.

## 12. Business Rules and Decision Constraints

The following are source-supported:

- Historical Payments shall never be overwritten.
- Payment ownership remains exclusive to Payment Management.
- Payment lifecycle progression is forward and completed stages are immutable.
- Payment state has exactly one active operational state.
- Settlement derives from Payment Allocation.
- Published Payment events are immutable.
- Refunds represent outbound monetary return and do not modify Payments.
- Accounting corrections use reversal Journals; historical accounting transactions are not edited.
- Aggregate ownership boundaries must not be bypassed.

L2-F03 explicitly states that Refunds never create or modify Payments. L2-F05 requires corrections through reversal Journals and preservation of original accounting history.

## 13. State and Lifecycle Impact

### Payment Lifecycle

L2-F02 defines:

`Payment Received → Validated → Published → Allocated → Settled → Archived`

It does not define a reversal/chargeback lifecycle branch. No new lifecycle stage is introduced in this L3.

### Payment State

L2-F02 defines `Draft`, `Pending Validation`, `Validated`, `Published`, `Gateway Settlement Pending`, `Allocated`, `Partially Settled`, `Fully Settled`, `Suspended`, `Cancelled`, `Cheque Dishonoured`, and `Archived`.

It does not define `Reversed`, `Chargeback`, or `Payment Reversal` as canonical Payment states. This scenario therefore cannot assert such a state without upstream change control.

### Accounting

Accounting correction, when required, follows the Accounting Journal lifecycle and reversal rules owned by L2-F05.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

L2-F02 defines:

- `PaymentReceptionInitiated`
- `PaymentValidationCompleted`
- `PaymentPublicationCompleted`
- `GatewaySettlementConfirmed`
- `ChequeDishonourRecorded`
- `PaymentAllocationCompleted`
- `PaymentSettlementCompleted`
- `PaymentArchivalCompleted`

No `PaymentReversed`, `PaymentChargebackRecorded`, or `PaymentReversalCompleted` event is defined by L2-F02. None is invented here.

Refund Management has its own canonical Refund event architecture; those events do not transfer ownership of the Payment.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Payment Management | Student Charge Management | Authoritative Payment consequence | Enables the obligation owner to determine its resulting outstanding position. |
| Payment Management | Refund Management | Authoritative financial consequence | Enables Refund ownership where an outbound return is required. |
| Payment Management | Financial Adjustment Management | Authoritative financial correction context | Enables Adjustment ownership where applicable. |
| Payment Management | Accounting & General Ledger | Authoritative Payment consequence | Enables accounting correction under Accounting ownership. |

Events remain distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

No Payment-specific reversal/chargeback authority is defined in L2-F02. No approval authority is invented.

### 15.2 Separation of Duties

Accounting's existing authorization and maker/checker rules remain F05-owned where accounting reversal is required. They are not transferred to Payment Management.

### 15.3 Audit and Evidence

The outcome must preserve:

- original Payment identity and references;
- evidence supporting the reversal/chargeback condition;
- resulting Student Charge, Refund, Adjustment and/or Accounting references;
- complete chronological financial lineage;
- immutable historical records.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The original Payment remains historically reconstructable.
- The authoritative reversal/chargeback condition is established.
- Any resulting financial consequence is owned by the correct Finance aggregate.
- Accounting consequences, where applicable, are represented in the Accounting domain.
- No unsupported Payment reversal/chargeback state or event has been invented.

### 16.2 Postconditions

- Payment ownership remains with Payment Management.
- Refund ownership remains with Refund Management where applicable.
- Adjustment ownership remains with Financial Adjustment Management where applicable.
- Student Charge ownership remains with Student Charge Management.
- Accounting history remains immutable and auditable.
- Financial lineage remains reconstructable end to end.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Material Upstream Gap — Payment Reversal / Chargeback Architecture

The frozen scenario explicitly requires a “Reversal or Chargeback Outcome”, but L2-F02 does not define a Payment reversal/chargeback state, lifecycle transition, business rule, recovery path, or dedicated Payment reversal/chargeback event.

L2-F03 defines Refund as a separate outbound monetary aggregate and explicitly prohibits Refunds from modifying Payments. L2-F05 defines accounting reversal, but that is an Accounting concern and does not establish a Payment reversal state.

**Impact:** A definitive Payment-level reversal/chargeback E2E path cannot be authored without either an existing authoritative source not yet identified or an upstream architecture decision.

**Required governance:** Confirm the canonical owner and business representation of Payment reversal/chargeback, including permitted Payment state/lifecycle treatment and canonical event publisher.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

The scenario remains a source-grounded Draft pending governance of the Payment-level reversal/chargeback semantics.
