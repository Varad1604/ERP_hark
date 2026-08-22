# Partial or Multi-Obligation Payment to Governed Allocation

**Scenario ID:** L3-F-008  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payment Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; L2-F02 Payment Management Architecture; L2-F07 Accounts Receivable Management  
**Referenced By:** Student Charge Management, Accounts Receivable, Accounting & General Ledger, Financial Reporting

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-008 |
| Canonical Title | Partial or Multi-Obligation Payment to Governed Allocation |
| Primary Owner | Payment Management |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the allocation of a valid Payment where one payment partially satisfies an obligation, one payment satisfies multiple obligations, multiple payments satisfy one obligation, or multiple payments satisfy multiple obligations.

The scenario establishes the correct financial relationship between Payment and Student Charge while preserving separate aggregate ownership and ensuring that settlement is derived from allocation.

## 3. Business Outcome

The Payment is allocated against the applicable Student Charge or Student Charges according to the authoritative allocation context, producing a financially valid allocation and the corresponding partial, complete, or composite settlement outcome without merging the Payment and Student Charge aggregates.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a validated/published Payment is eligible for allocation to one or more Student Charges.

### 4.2 End Boundary

The scenario ends when the governed Payment Allocation has been established and the resulting settlement/outstanding-balance consequence is authoritative.

### 4.3 In Scope

- Identification of the Payment available for allocation.
- Identification of one or more Student Charges.
- Resolution of the applicable allocation relationship.
- One-to-one allocation.
- One-to-many allocation.
- Many-to-one allocation.
- Many-to-many/composite allocation.
- Partial settlement.
- Complete settlement.
- Unallocated advance treatment where no applicable charge can yet be allocated.
- Preservation of allocation and settlement traceability.

### 4.4 Out of Scope

- Payment reception and initial validation.
- Creation or modification of Student Charges.
- Refund processing.
- Financial adjustment execution.
- Accounting posting.
- Bank reconciliation.
- Technical allocation algorithms, APIs, databases, queues, retries or transaction mechanics.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Provides authoritative student context where required to identify the relevant obligations. | Student Context remains C-owned. |
| F — Payment Management | Owns Payment, Payment Allocation and Settlement representation. | Payment and allocation remain F02-owned. |
| F — Student Charge & Fee Management | Owns Student Charge and its financial obligation. | Student Charge remains F01-owned. |
| F — Accounts Receivable | Consumes settlement information and maintains the receivable representation. | Receivable remains F07-owned. |

L2-F02 explicitly defines Payment Allocation as the relationship between Payment and one or more Student Charges and states that allocation never merges the aggregates.

## 6. Business Actors, Roles and Responsibilities

L2-F02 does not define an authoritative actor catalogue. No allocation role, cashier, finance officer, or other actor is invented.

Payment allocation is a Finance business responsibility governed within Payment Management; student context remains supplied by the authoritative Student Lifecycle domain where required.

## 7. Preconditions and Entry Criteria

- A canonical Payment exists and is eligible for allocation.
- The relevant Student Charge or Student Charges exist where allocation is intended.
- The Payment and financial obligations are identifiable.
- The allocation context is sufficient to determine a valid relationship.
- Monetary integrity can be preserved.
- Any unallocated amount can be represented through the canonical unallocated-advance architecture if necessary.

## 8. Trigger

A validated/published Payment becomes ready for allocation to one or more Student Charges.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Payment available for allocation. | F — Payment Management | A canonical settlement source is established. |
| 2 | Identify the Student Charge or Student Charges eligible to receive the Payment. | F — Payment Management / F01 context | Candidate financial obligations are established. |
| 3 | Resolve the applicable allocation relationship using the authoritative financial context. | F — Payment Management | The Payment-to-Charge relationship is determined. |
| 4 | Establish the Payment Allocation without transferring ownership of either aggregate. | F — Payment Management | Allocation becomes authoritative. |
| 5 | Determine whether the resulting settlement is partial, complete, multi-charge, multi-payment or composite. | F — Payment Management | The settlement outcome is classified by the canonical allocation model. |
| 6 | Update the derived outstanding financial position through the owning Finance architecture. | F — Payment Management / F01 | Outstanding financial position reflects the allocation outcome. |
| 7 | Publish the canonical allocation and settlement facts. | F — Payment Management | Downstream consumers can recognize the completed allocation. |
| 8 | Where applicable, Accounts Receivable incorporates the settlement reference into the Receivable representation. | F — Accounts Receivable | Receivable visibility reflects the authoritative settlement fact. |

L2-F02 explicitly supports one-to-one, one-to-many, many-to-one, many-to-many and zero-allocation advance models.

## 10. Alternate Business Flows

### A1 — One Payment to One Student Charge

The Payment satisfies a single Student Charge, producing partial or complete settlement depending on the amount.

### A2 — One Payment to Multiple Student Charges

The Payment is allocated across multiple Student Charges according to the governed allocation context.

### A3 — Multiple Payments to One Student Charge

Several Payments contribute to one Student Charge. Each Payment remains independently owned by Payment Management; the Student Charge remains F01-owned.

### A4 — Multiple Payments to Multiple Student Charges

The institution resolves a composite allocation relationship while preserving each Payment and Student Charge as separate canonical aggregates.

### A5 — Partial Settlement

Only part of the obligation is satisfied. The outstanding balance continues to exist.

### A6 — Unallocated Advance

No applicable Student Charge can yet receive the Payment, or the Payment exceeds the currently applicable obligation. The Payment remains an explicit unallocated advance and is classified as a Liability until consumed by a future Charge Allocation.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Resolution |
|---|---|
| No valid Student Charge can be identified | Do not allocate against an inferred obligation; retain the Payment in the governed unallocated/advance condition. |
| Payment amount does not match the intended obligation | Preserve monetary integrity and apply the canonical partial or unallocated model rather than inventing a balancing amount. |
| Multiple eligible obligations exist but allocation context is insufficient | Do not make an unsupported allocation choice; obtain the authoritative financial context. |
| Allocation would violate Payment or Student Charge ownership | Reject that path; allocation must preserve bounded-context ownership. |
| Settlement information conflicts with the allocation | Preserve the authoritative Payment and Student Charge facts and resolve the allocation inconsistency through the owning Finance capability. |
| Downstream receivable representation is inconsistent | Accounts Receivable resolves its own representation using the canonical settlement fact; Payment ownership remains unchanged. |

No technical retry, rollback, queue or database recovery is defined.

## 12. Business Rules and Decision Constraints

Authoritative L2-F02 constraints include:

- Every allocation references one canonical Payment.
- Every allocation references one or more Student Charges.
- Allocation preserves monetary integrity.
- Allocation remains auditable.
- Allocation does not merge Payment and Student Charge aggregates.
- Settlement always derives from Payment Allocation.
- Settlement never independently owns Payment or Student Charge.
- Partial settlement leaves an Outstanding Balance.
- Unallocated advances are explicitly classified as a Liability until consumed by a future Charge Allocation.
- Payment state has one active operational state.
- Completed lifecycle stages are immutable.

L2-F01 separately retains ownership of Student Charge and its Outstanding Balance. L2-F07 consumes settlement information without taking ownership of Payment.

## 13. State and Lifecycle Impact

### Payment Lifecycle

The Payment progresses through the canonical lifecycle toward `Allocated` and, where applicable, `Settled`. The scenario does not introduce a new lifecycle.

### Payment State

The applicable canonical states include `Published`, `Allocated`, `Partially Settled`, and `Fully Settled`. The scenario uses only transitions defined by L2-F02.

### Student Charge

The Student Charge remains under its own canonical lifecycle and state architecture. Allocation does not transfer Student Charge ownership to Payment Management.

### Receivable

Accounts Receivable maintains its own Receivable balance and settlement references. It consumes authoritative settlement information rather than executing Payment allocation.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

Payment Management publishes:

- `PaymentAllocationCompleted`
- `PaymentSettlementCompleted`

These are exact canonical Payment events defined by L2-F02.

Accounts Receivable publishes its own settlement facts, including `ReceivableSettlementRecorded` and `ReceivableFullySettled`, where its aggregate reaches those conditions.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C — Student Lifecycle | F — Payment Management | Student context | Supports identification of the relevant financial obligations. |
| F — Student Charge Management | F — Payment Management | Authoritative Student Charge context | Provides the financial obligation that may receive allocation. |
| F — Payment Management | F — Accounts Receivable | Completed allocation/settlement fact | Allows receivable representation to reflect settlement. |
| F — Payment Management | F — Accounting | Authoritative settlement information | Enables downstream accounting treatment. |

Events remain distinct from synchronous/business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

L2-F02 states that Payment authorization is externally governed. No allocation-specific approval authority is defined and none is invented.

### 15.2 Separation of Duties

No allocation-specific separation-of-duty rule is defined in the accessed L2-F02 material.

### 15.3 Business Audit and Evidence

Every allocation must remain reconstructable from:

- Payment;
- Payment Reference;
- Student Charge;
- allocation relationship;
- resulting settlement;
- outstanding balance consequence.

L2-F02 requires allocation traceability and permanent financial reconstruction.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The Payment is identified.
- The applicable Student Charge relationship is established or the payment is explicitly classified as unallocated/advance.
- The allocation preserves monetary integrity.
- The applicable partial, complete, multi-charge or composite settlement outcome is established.
- The allocation and settlement facts are published.
- Downstream receivable/accounting consumers can consume the authoritative financial outcome.

### 16.2 Postconditions

- Payment remains Payment Management-owned.
- Student Charge remains Student Charge Management-owned.
- Allocation remains Payment Management-owned.
- Settlement remains derived from allocation.
- Any remaining outstanding amount remains represented by the canonical financial architecture.
- No duplicate financial obligation is created.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material scenario-specific upstream ambiguity was identified in the accessed L2-F02/F01/F07 allocation boundary.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
