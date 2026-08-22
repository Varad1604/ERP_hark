# Unidentified or Unallocated Receipt to Reconciled Payment

**Scenario ID:** L3-F-009  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00 Finance Domain Model & Foundation; L2-F02 Payment Management Architecture; L2-F07 Accounts Receivable Management; applicable L2 Treasury/Banking source where the receipt originates from bank settlement  
**Referenced By:** Accounts Receivable, Accounting & General Ledger, Financial Reporting and downstream Finance consumers

## 2. Scenario Purpose

Resolve a received monetary receipt whose payer, institutional reference, Student Charge relationship, or allocation context is initially insufficient for normal settlement, and bring that receipt to a reconciled Payment outcome without inventing financial identity or incorrectly allocating funds.

The scenario preserves Payment ownership and uses the canonical unallocated-advance architecture when the receipt cannot yet be matched to an obligation.

## 3. Business Outcome

The unidentified or unallocated receipt is reconciled into an authoritative Payment with sufficient financial identity and allocation context for its valid financial outcome, or remains explicitly in the governed unallocated/exception condition when authoritative information is still insufficient.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when the institution has received monetary value but the receipt cannot yet be reliably associated with the required Payment identity, payer context, or Student Charge allocation.

### 4.2 End Boundary

The scenario ends when the Payment has been reconciled to authoritative identity and allocation context, with the applicable settlement outcome established, or when it is formally retained in the governed unallocated/exception condition pending authoritative resolution.

### 4.3 In Scope

- Identification of an unidentified/unallocated receipt.
- Preservation of the receipt/payment financial value.
- Resolution of Payment Source and Payment Reference where possible.
- Resolution of the applicable Student Charge where possible.
- Classification as unallocated advance where no valid obligation can yet be identified.
- Reconciliation into the canonical Payment context.
- Allocation and settlement after authoritative matching.
- Preservation of financial lineage and exception history.

### 4.4 Out of Scope

- Bank execution.
- Payment reception mechanisms outside Payment Management's business boundary.
- Creation of Student Charges.
- Refund processing.
- Accounting posting mechanics.
- Treasury reconciliation workflow beyond the Payment business outcome.
- Technical matching algorithms, databases, queues, retries or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| F — Payment Management | Owns the Payment, Payment Reference, Payment Source, allocation and settlement representation. | Payment remains F02-owned. |
| F — Student Charge & Fee Management | Owns any Student Charge to which the reconciled Payment is allocated. | Student Charge remains F01-owned. |
| F — Accounts Receivable | Maintains the receivable representation and settlement references after authoritative settlement information exists. | Receivable remains F07-owned. |
| F — Treasury & Banking | May provide authoritative bank-origin context where the receipt originates from banking activity. | Treasury/Banking remains its canonical owner. |

The Stage 3 canonical participating-domain value is F only; other Finance capabilities are referenced here only where their canonical ownership is materially involved in the reconciliation boundary.

## 6. Business Actors, Roles and Responsibilities

L2-F02 does not provide an authoritative actor catalogue. No reconciliation officer, cashier, finance administrator, or other role is invented.

The scenario is therefore expressed through canonical Finance business responsibilities and ownership boundaries rather than unsupported human role assignments.

## 7. Preconditions and Entry Criteria

- Monetary value has been received by the institution.
- The receipt exists as a financially relevant business fact.
- Normal Payment identification or allocation is incomplete.
- Sufficient authoritative evidence may exist to reconcile the receipt.
- If the receipt cannot yet be matched, the Payment architecture supports an explicit Unallocated Advance.

## 8. Trigger

A received monetary receipt is identified as unidentified or unallocated and requires governed reconciliation.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the receipt as requiring reconciliation because payer, Payment Reference, Student Charge or allocation context is incomplete. | F — Payment Management | The receipt enters a governed reconciliation condition. |
| 2 | Preserve the received financial value and its available financial references without assigning an unsupported identity. | F — Payment Management | Financial value remains protected and traceable. |
| 3 | Resolve the canonical Payment Source from authoritative evidence where available. | F — Payment Management | The payer context is established when evidence supports it. |
| 4 | Resolve the institutional Payment Reference and relevant Receipt Reference where available. | F — Payment Management | The receipt can be connected to a canonical Payment identity. |
| 5 | Determine whether an authoritative Student Charge can receive the Payment. | F — Payment Management / F01 context | Allocation eligibility is established. |
| 6 | If a valid Student Charge is identified, establish the Payment Allocation and resulting settlement. | F — Payment Management | The receipt becomes financially allocated. |
| 7 | If no Student Charge can yet be identified, classify the receipt as an Unallocated Advance and preserve it as a Liability until consumed by a future Charge Allocation. | F — Payment Management | The receipt has a governed interim financial meaning without false allocation. |
| 8 | Publish the canonical Payment facts and applicable allocation/settlement facts once the reconciled outcome is valid. | F — Payment Management | Downstream Finance domains can consume authoritative facts. |
| 9 | Where applicable, Accounts Receivable incorporates the settlement reference into the Receivable representation. | F — Accounts Receivable | Receivable visibility reflects the reconciled settlement. |
| 10 | Preserve the complete reconciliation history. | F | The institutional financial lineage is reconstructable. |

L2-F02 explicitly defines Unallocated Advance as a Payment received without an associated Student Charge, or where the Payment exceeds the outstanding obligation, and requires it to be classified as a Liability until consumed by a future Charge Allocation. fileciteturn146file0

## 10. Alternate Business Flows

### A1 — Payer Identified, Charge Unidentified

The Payment Source and Payment Reference are resolved, but no valid Student Charge can yet be identified. The Payment remains an Unallocated Advance until a future authoritative Charge Allocation consumes it.

### A2 — Charge Identified, Payer Evidence Incomplete

The Payment cannot be assigned to an inferred payer merely because a Student Charge is identifiable. Payment Source remains unresolved until authoritative evidence exists.

### A3 — Payment Amount Exceeds Current Obligation

The applicable portion may be allocated to the Student Charge while the excess follows the canonical Unallocated Advance/Liability treatment.

### A4 — Multiple Possible Student Charges

The receipt is not allocated arbitrarily. The authoritative financial context must establish the correct allocation relationship before settlement is claimed.

### A5 — Reconciled to a Single Student Charge

Once Payment identity and obligation context are authoritative, the receipt proceeds through the normal Payment Allocation and Settlement path.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Resolution |
|---|---|
| Payer cannot be reliably identified | Preserve the receipt without inventing Payment Source identity. |
| Payment Reference is insufficient or conflicting | Preserve available financial evidence and resolve the reference before claiming reconciliation. |
| No Student Charge can be established | Use the canonical Unallocated Advance/Liability treatment rather than false allocation. |
| Several obligations appear plausible | Do not select one without authoritative business context. |
| Reconciliation evidence conflicts | Preserve the unambiguous facts and escalate to the canonical Finance owner of the disputed information. |
| Receipt is determined to require another Finance outcome | Hand the resulting business fact to the canonical owning domain; do not create a parallel local aggregate. |

Payment exceptions must preserve financial integrity, aggregate consistency and traceability. fileciteturn166file0

No technical retry, queue, rollback, database, or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

The scenario applies the following authoritative constraints:

- Payment is the canonical settlement aggregate.
- Payment ownership remains exclusive to Payment Management.
- Payment References are unique, immutable and auditable.
- Allocation must reference a canonical Payment and one or more Student Charges.
- Settlement always derives from Payment Allocation.
- Unallocated Advance is a canonical Payment allocation model.
- Unallocated Advance is classified as a Liability until consumed by a future Charge Allocation.
- Payment exceptions must preserve financial integrity and traceability.
- External domains do not directly modify Payment.
- Accounts Receivable does not accept payments or execute payment reconciliation; it consumes authoritative settlement information.
- Receivable traceability includes originating financial obligation, settlement references, adjustment references, refund references and accounting references.

These constraints are directly supported by L2-F02 and L2-F07. fileciteturn146file0turn155file0turn156file0

## 13. State and Lifecycle Impact

### Payment Lifecycle

The canonical Payment lifecycle remains:

`Payment Received → Validated → Published → Allocated → Settled → Archived`

An unidentified/unallocated receipt must not be described as fully settled until the applicable allocation/settlement condition is actually established. fileciteturn147file0

### Payment State

L2-F02 defines `Suspended`, `Cancelled`, and other exceptional states in addition to the normal validation/publication/allocation/settlement states. The scenario does not invent a dedicated “Unreconciled” state; any exceptional state used must be one already authorized by F02's state architecture. fileciteturn148file0

### Receivable

Accounts Receivable records settlement references but does not own Payment execution or reconciliation. Its Receivable lifecycle changes only from authoritative Finance facts. fileciteturn155file0turn156file0

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

Payment Management owns and publishes the canonical Payment event vocabulary, including:

- `PaymentReceptionInitiated`
- `PaymentValidationCompleted`
- `PaymentPublicationCompleted`
- `PaymentAllocationCompleted`
- `PaymentSettlementCompleted`
- `PaymentArchivalCompleted`

No dedicated “ReceiptReconciled” event is invented because the accessed L2-F02 event architecture does not define one. fileciteturn148file0

Accounts Receivable publishes `ReceivableSettlementRecorded` and `ReceivableFullySettled` when its own aggregate reaches those conditions. fileciteturn156file0

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| F — Treasury & Banking, where applicable | F — Payment Management | Authoritative bank-origin financial context | Supports identification of the received monetary value. |
| F — Payment Management | F — Student Charge | Candidate/confirmed obligation context | Establishes whether the Payment can be allocated. |
| F — Payment Management | F — Accounts Receivable | Completed settlement reference | Allows the receivable representation to reflect the reconciled outcome. |
| F — Payment Management | F — Accounting | Authoritative financial fact | Enables accounting treatment after reconciliation. |

Events remain distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

No reconciliation-specific human authority is defined in L2-F02. The scenario therefore does not invent an approving or investigating role.

### 15.2 Separation of Duties

No reconciliation-specific separation-of-duty rule is defined in L2-F02 or L2-F07.

### 15.3 Business Audit and Evidence

The reconciliation must preserve:

- original receipt/payment evidence;
- Payment Source resolution;
- Payment Reference/Receipt Reference;
- Student Charge allocation context where established;
- unallocated-advance classification where applicable;
- resulting settlement;
- downstream receivable/accounting references;
- complete exception and resolution history.

L2-F02 requires permanent availability of exception history and complete financial traceability. L2-F07 requires complete financial lineage including settlement and accounting references. fileciteturn166file0turn156file0

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

A reconciled outcome is complete when:

- the Payment identity is authoritative;
- Payment Source and required Payment References are sufficiently resolved;
- the applicable Student Charge allocation is established, or the receipt is explicitly retained as an Unallocated Advance;
- the applicable settlement outcome is authoritative where allocation has occurred;
- downstream Finance consumers can consume the completed financial fact;
- reconciliation history is fully traceable.

### 16.2 Postconditions

- No unsupported payer or obligation identity has been invented.
- No duplicate Payment or Student Charge has been created.
- Any unallocated value retains its canonical Liability treatment until consumed by a future Charge Allocation.
- Payment ownership remains with Payment Management.
- Receivable and Accounting ownership remain unchanged.
- Financial history remains reconstructable.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material scenario-specific upstream ambiguity was identified for the unidentified/unallocated receipt boundary in the accessed L2-F02/F07 architecture.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
