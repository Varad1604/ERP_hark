# Student or Sponsor Payment to Receipt and Accounting Completion

**Scenario ID:** L3-F-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payment Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; L2-F02 Payment Management Architecture; L2-F05 Accounting & General Ledger Management  
**Referenced By:** Student Charge Management, Accounts Receivable, Financial Reporting and downstream Finance consumers

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-007 |
| Canonical Title | Student or Sponsor Payment to Receipt and Accounting Completion |
| Primary Owner | Payment Management |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Complete the institutional processing of a valid monetary payment from a student or sponsor from receipt by Finance through establishment of the canonical Payment, creation of its institutional receipt reference, settlement/allocability information, and completion of the corresponding accounting outcome.

The scenario preserves the separation between Payment ownership and Accounting ownership: Payment Management owns the Payment and its receipt/reference lifecycle, while Accounting & General Ledger owns accounting posting and the official ledger record.

## 3. Business Outcome

A valid student or sponsor payment is represented as an authoritative Payment, carries an immutable institutional receipt reference, is available for the applicable settlement/allocation outcome, and has its corresponding accounting outcome completed by the Accounting & General Ledger domain.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a monetary payment from a student or sponsor is received by the institution through an accepted payment channel.

### 4.2 End Boundary

The scenario ends when the Payment has reached its applicable completed financial settlement outcome, the institutional receipt reference is established, and the corresponding accounting posting is confirmed by the Accounting & General Ledger domain.

### 4.3 In Scope

- Payment reception.
- Identification of the student or sponsor as Payment Source.
- Identification of the Payment Instrument and Payment Channel.
- Payment validation.
- Creation/publication of the canonical Payment.
- Establishment of the institutional Payment Reference / Receipt Reference.
- Allocation or settlement against the applicable Student Charge where the payment context permits.
- Publication of canonical Payment facts.
- Accounting consumption and completion of the corresponding accounting posting.
- Preservation of financial traceability.

### 4.4 Out of Scope

- Creation of the underlying Student Charge.
- Definition of fee policy or charge calculation.
- Refund processing.
- Financial adjustment processing.
- Bank reconciliation as a separate Treasury responsibility.
- General Ledger ownership or accounting rule redesign.
- Technical APIs, persistence, queues, retries, database transactions, or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Business Responsibility | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Supplies authoritative student context where the payment concerns a student. | Student Context remains C-owned. |
| F — Payment Management | Receives, validates, represents, references, allocates and settles the Payment. | Payment remains F02-owned. |
| F — Student Charge & Fee Management | Owns the financial obligation against which the payment may be allocated. | Student Charge remains F01-owned. |
| F — Accounting & General Ledger | Completes the accounting outcome from authoritative Payment facts. | Accounting Ledger and posted accounting records remain F05-owned. |

L2-F02 identifies Student Lifecycle and other operational domains as upstream business-context providers and Accounting as a downstream consumer of canonical settlement information. It explicitly states that downstream domains consume Payment information without assuming ownership. 

## 6. Business Actors, Roles and Responsibilities

L2-F02 records that an authoritative actor catalogue is not present. No payment actor, cashier, sponsor representative, accountant, approver, or other role is invented here.

The business participation of the payer is represented by the canonical Payment Source, which may be Student or Sponsor. Payment authorization remains externally governed.

## 7. Preconditions and Entry Criteria

- A valid business context exists for the payment.
- Where payment is against a Student Charge, the relevant canonical obligation is identifiable.
- The payer can be represented through the Payment Source architecture.
- An accepted Payment Instrument and Payment Channel are available.
- The payment can pass the applicable Payment validation requirements.
- Accounting can consume the canonical financial settlement information.

## 8. Trigger

A student or sponsor monetary payment is received by the institution.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | The institution receives the monetary payment through an accepted channel. | F — Payment Management | Payment reception has occurred. |
| 2 | The payer is identified as the canonical Payment Source. | F — Payment Management | Financial origin is established. |
| 3 | The Payment Instrument and Payment Channel are established. | F — Payment Management | The settlement mechanism and institutional entry point are known. |
| 4 | Payment validation establishes that the received payment satisfies the applicable financial integrity requirements. | F — Payment Management | The payment is eligible for canonical Payment representation. |
| 5 | Payment Management establishes the canonical Payment and its institutional Payment Reference, including the applicable Receipt Reference. | F — Payment Management | The payment has an authoritative institutional identity and receipt traceability. |
| 6 | Where the payment is allocable to a Student Charge, Payment Allocation establishes the relationship between the Payment and the applicable obligation. | F — Payment Management | The payment can satisfy the relevant financial obligation. |
| 7 | The resulting settlement information is established from successful allocation. | F — Payment Management | The applicable financial settlement outcome is known. |
| 8 | Payment Management publishes the canonical Payment facts required by downstream Finance domains. | F — Payment Management | Accounting and other consumers receive authoritative settlement information. |
| 9 | Accounting & General Ledger consumes the authoritative financial transaction and completes the governed accounting posting. | F — Accounting & General Ledger | The accounting outcome is recorded in the official ledger. |
| 10 | The completed Payment, receipt reference, settlement and accounting lineage remain reconstructable. | F | The institutional payment has a complete financial history. |

L2-F02 defines Payment Reception before Payment publication, Payment References including Receipt Reference, Allocation, Settlement, and the downstream Accounting relationship.

## 10. Alternate Business Flows

### A1 — Sponsor Payment on Behalf of a Student

The payer is represented as Sponsor while the Payment may be allocated to the Student Charge associated with the student. Payment Source ownership remains with Payment Management; Student Charge ownership remains with Student Charge Management.

### A2 — Payment Satisfies One Student Charge

The Payment is allocated to one Student Charge and produces the corresponding settlement outcome.

### A3 — Payment Satisfies Multiple Student Charges

The Payment proceeds through the multi-obligation allocation path defined by Payment Management. This scenario stops once the resulting allocation/settlement and accounting outcome are complete; the detailed multi-obligation allocation scenario is separately covered by L3-F-008.

### A4 — Payment Remains Unallocated

Where the received payment cannot yet be associated with a Student Charge, the payment follows the governed unallocated/advance path. The payment is not falsely treated as settled against an obligation.

### A5 — Accounting Outcome Not Yet Completed

The Payment remains financially traceable and the accounting completion is not claimed until Accounting & General Ledger establishes the authoritative posting outcome.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Resolution |
|---|---|
| Payment cannot be validated | Do not treat the payment as a completed valid Payment outcome; retain the governed Payment exception condition until resolved. |
| Payer/source context is insufficient | Do not invent payer identity; resolve the authoritative Payment Source context. |
| Institutional receipt reference cannot be established | Do not claim receipt completion until the canonical Payment Reference requirement is satisfied. |
| Payment cannot yet be allocated | Preserve the Payment and follow the governed unallocated/advance path rather than assigning it to an inferred Student Charge. |
| Accounting rejects or cannot complete the financial posting | Preserve the canonical Payment and its financial lineage; resolve the accounting outcome under Accounting ownership before claiming accounting completion. |
| Conflicting financial facts exist | Preserve unambiguous facts and route the conflict to the owning Finance capability; do not locally redefine the financial position. |

No technical retry, queue, rollback, server failure, database recovery, or infrastructure mechanism is defined at L3.

## 12. Business Rules and Decision Constraints

The scenario applies these source-supported constraints:

- Payment is the canonical settlement aggregate.
- Every Payment has one canonical Payment Source.
- Payment Reception precedes Payment publication.
- Every Payment possesses a unique institutional reference; Receipt Reference is a canonical Payment Reference category.
- Payment Allocation connects Payments to Student Charges.
- Settlement is derived from Payment Allocation and does not independently own Payment or Student Charge.
- Payment lifecycle progression is forward and completed lifecycle stages are immutable.
- Payment state is independently owned by Payment Management.
- Accounting Ledger is the authoritative source of posted financial information.
- Accounting posting requires successful validation and an approved Journal Entry.
- Posted accounting records are immutable and corrections use reversal rather than editing history.
- Aggregate ownership boundaries cannot be bypassed.

These constraints are inherited from L2-F02 and L2-F05 and are not redefined locally.

## 13. State and Lifecycle Impact

### Payment Lifecycle

The canonical Payment lifecycle is:

`Payment Received → Validated → Published → Allocated → Settled → Archived`

This scenario covers the active progression through the applicable settlement outcome. It does not invent a new Payment lifecycle.

### Payment State

The Payment state architecture includes:

- Draft
- Pending Validation
- Validated
- Published
- Gateway Settlement Pending
- Allocated
- Partially Settled
- Fully Settled
- Suspended
- Cancelled
- Cheque Dishonoured
- Archived

Only the state transitions defined by L2-F02 may be applied.

### Accounting Lifecycle

Accounting Journal Entries progress through `Draft → Validated → Approved → Posted → Reversed/Archived` as applicable. Posted financial records remain immutable.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

Payment Management is the canonical publisher of:

- `PaymentReceptionInitiated`
- `PaymentValidationCompleted`
- `PaymentPublicationCompleted`
- `GatewaySettlementConfirmed`
- `PaymentAllocationCompleted`
- `PaymentSettlementCompleted`
- `PaymentArchivalCompleted`

The exact event names and publisher remain those defined by L2-F02.

Accounting & General Ledger publishes its own accounting facts after the accounting outcome is completed. Accounts Receivable explicitly consumes `AccountingPosted` from Accounting & General Ledger, establishing Accounting as its publisher.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C — Student Lifecycle | F — Payment Management | Student business context | Establishes the relevant student context for the payment. |
| Payer / source context | F — Payment Management | Payment Source context | Establishes who provided the settlement. |
| F — Payment Management | F — Student Charge | Allocation/settlement context | Associates payment settlement with the canonical obligation without transferring ownership. |
| F — Payment Management | F — Accounting & General Ledger | Authoritative financial settlement information | Enables accounting treatment and posting. |
| F — Accounting & General Ledger | downstream Finance consumers | Completed accounting fact | Communicates the authoritative accounting outcome. |

Events remain distinct from these business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

L2-F02 states that Payment authorization is externally governed. L2-F05 defines accounting authorization and segregation-of-duty principles within Accounting. No new payment approval authority is introduced.

### 15.2 Separation of Duties

Accounting & General Ledger requires maker/checker separation where applicable and records approval decisions. These rules remain F05-owned and are not transferred to Payment Management.

### 15.3 Business Audit and Evidence

The completed outcome must preserve:

- canonical Payment identity;
- Payment Source;
- Payment Instrument and Channel;
- institutional Payment/Receipt Reference;
- allocation and settlement lineage;
- accounting posting reference/outcome;
- immutable historical financial records.

L2-F02 requires permanent reconstruction of financial history, while L2-F05 requires traceability and immutable accounting history.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The payment has been received and validated.
- A canonical Payment exists.
- The payer is represented through the canonical Payment Source.
- An institutional Payment Reference, including the applicable Receipt Reference, exists.
- Applicable allocation and settlement have been completed or the payment has entered its explicitly governed unallocated path.
- Accounting & General Ledger has completed the accounting outcome before accounting completion is claimed.
- Financial lineage is reconstructable across Payment, settlement and accounting.

### 16.2 Postconditions

- The Payment remains owned by Payment Management.
- Any Student Charge remains owned by Student Charge Management.
- Accounting Ledger remains owned by Accounting & General Ledger.
- The institutional receipt/payment reference is immutable.
- The completed financial outcome is available to downstream Finance consumers.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material scenario-specific upstream ambiguity was identified for the payment → receipt → accounting boundary in the accessed locked L2-F02/F05 architecture.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
