# Payment Cancellation to Controlled Terminal Outcome

**Scenario ID:** L3-F-012  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payment Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; L2-F02 Payment Management Architecture  
**Referenced By:** Student Charge & Fee Management, Accounts Receivable, Accounting & General Ledger

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-012 |
| Canonical Title | Payment Cancellation to Controlled Terminal Outcome |
| Primary Owner | Payment Management |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the cancellation of a Payment so that the Payment reaches a controlled terminal business position without falsely representing an incomplete or cancelled settlement as successfully completed.

The scenario preserves Payment ownership, protects historical financial evidence, and establishes the business boundary between Payment cancellation and any resulting Student Charge consequence.

## 3. Business Outcome

A Payment that is eligible for cancellation reaches the canonical Cancelled state under the applicable business authority, with its historical identity and financial lineage preserved and any resulting Student Charge consequence handled by the Student Charge owner.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Payment exists in a condition in which cancellation is permitted by the applicable business policy.

### 4.2 End Boundary

The scenario ends when the Payment has reached the canonical Cancelled state and the related financial obligation consequence, where applicable, has been handed to the canonical Student Charge owner.

### 4.3 In Scope

- Identification of a Payment eligible for cancellation.
- Business determination that cancellation is permitted.
- Controlled transition to the existing Cancelled Payment state.
- Preservation of Payment references and historical evidence.
- Communication of the completed cancellation fact using only defined canonical events.
- Student Charge consequence where cancellation changes the financial obligation's outstanding position.

### 4.4 Out of Scope

- Creating a new cancellation state.
- Defining cancellation approval authority not present upstream.
- Refund execution.
- Payment reversal or chargeback semantics.
- Accounting reversal mechanics.
- Technical cancellation requests, APIs, queues, retries or rollback.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Supplies authoritative student context where the Payment relates to a student. | Student context remains C-owned. |
| F — Payment Management | Owns Payment cancellation and Payment state. | Payment remains F02-owned. |
| F — Student Charge & Fee Management | Owns the financial obligation affected by cancellation where applicable. | Student Charge remains F01-owned. |

The frozen Stage 3 participation value is C, F and is preserved exactly. Participation does not transfer Payment or Student Charge ownership.

## 6. Business Actors, Roles and Responsibilities

L2-F02 has no authoritative actor catalogue and does not define a Payment Cancellation role. No cashier, finance officer, student, sponsor representative, approver, or cancellation officer is invented as an architectural role.

The scenario therefore expresses cancellation through canonical business responsibility and externally governed authorization.

## 7. Preconditions and Entry Criteria

- A canonical Payment exists.
- The Payment is not already in an irreversible historical position that the applicable policy prohibits from cancellation.
- Cancellation is permitted by the applicable business policy.
- The Payment's identity and references are available.
- Where the Payment relates to a Student Charge, the corresponding financial obligation is identifiable.

## 8. Trigger

An authorized business condition establishes that an existing Payment is to be cancelled.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Payment subject to cancellation and establish the business context for cancellation. | Payment Management | The canonical Payment is identified. |
| 2 | Confirm that cancellation is permitted under the applicable business policy and current Payment condition. | Payment Management / externally governed authority | Cancellation is authorized as a business outcome. |
| 3 | Payment Management transitions the Payment to the existing canonical Cancelled state where the state transition is permitted. | Payment Management | The Payment has a controlled terminal business condition. |
| 4 | Preserve the Payment identity, references, source, instrument, channel and historical state evidence. | Payment Management | The cancelled Payment remains fully traceable. |
| 5 | Publish the applicable completed Payment business fact using only the canonical Payment event vocabulary. | Payment Management | Downstream consumers receive authoritative information without an invented event. |
| 6 | Where cancellation changes the financial obligation, the Student Charge domain determines and maintains its own resulting obligation state. | Student Charge & Fee Management | Student Charge ownership and outstanding balance remain canonical. |

## 10. Alternate Business Flows

### A1 — Cancellation Before Allocation

Where the Payment has not yet been allocated, cancellation can terminate the Payment without claiming settlement against a Student Charge, subject to the authorized state transition.

### A2 — Cancellation After Publication but Before Settlement

Where the Payment has been published but has not reached settlement, the Payment may proceed to the Cancelled state if the applicable policy permits the transition.

### A3 — Student Charge Consequence

Where cancellation affects an obligation that had previously been treated as settled or partially settled, the resulting Student Charge consequence is maintained by Student Charge Management. Payment Management does not modify the Student Charge directly.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Cancellation is not permitted in the current Payment condition | Payment cannot enter the Cancelled state | Preserve the existing Payment state and follow the applicable authorized business path. | Payment Management |
| Cancellation authority is not established | Terminal outcome cannot safely be claimed | Do not invent authority; preserve the Payment and record the governance gap. | Payment Management / governing authority |
| Payment has already reached a completed historical condition for which cancellation semantics are undefined | The effect of cancellation is ambiguous | Preserve the original Payment history and escalate the missing transition definition rather than rewriting history. | Payment Management |
| Cancellation affects a Student Charge | Obligation position may require a financial consequence | Student Charge Management governs its own resulting obligation state. | Student Charge & Fee Management |

No technical cancellation retry, rollback, queue, or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

The scenario applies:

- Payment is the canonical settlement aggregate.
- External domains cannot directly modify Payment.
- Payment has exactly one active operational state.
- State transitions must be deterministic.
- Historical states are immutable.
- Payment lifecycle progression is distinct from operational state.
- Published Payment References remain immutable.
- Cancellation is an existing canonical Payment state, not a newly invented state.
- Settlement derives from Payment Allocation.
- Any Student Charge consequence remains Student Charge-owned.
- Published events represent completed business facts.

L2-F02 explicitly includes Cancelled among canonical Payment states and allows exceptional states such as Suspended and Cancelled where permitted by enterprise policies.

## 13. State and Lifecycle Impact

### Payment Lifecycle

The canonical lifecycle remains:

`Payment Received → Validated → Published → Allocated → Settled → Archived`

L2-F02 does not define cancellation as a new lifecycle stage. Cancellation is an operational state consequence and must not be represented as a newly invented lifecycle stage.

### Payment State

`Cancelled` is a canonical Payment state.

The authoritative state architecture does not provide a complete explicit transition matrix for every cancellation condition. It states that exceptional states such as Cancelled may occur where permitted by enterprise policies.

Accordingly, this L3 uses the existing Cancelled state but does not invent unsupported predecessor transitions.

### Student Charge

Where cancellation changes a Student Charge's outstanding position, Student Charge Management owns the resulting obligation state and balance.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

L2-F02's canonical Payment events are:

- `PaymentReceptionInitiated`
- `PaymentValidationCompleted`
- `PaymentPublicationCompleted`
- `GatewaySettlementConfirmed`
- `ChequeDishonourRecorded`
- `PaymentAllocationCompleted`
- `PaymentSettlementCompleted`
- `PaymentArchivalCompleted`

There is **no canonical `PaymentCancelled` event defined by L2-F02**. The scenario therefore does not invent one.

The cancellation outcome must be communicated using an authoritative event only if and when the upstream Payment event architecture defines one.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C — Student Lifecycle | Payment Management | Student context | Supports identification of the relevant Payment context where applicable. |
| Payment Management | Student Charge & Fee Management | Authoritative cancellation consequence | Allows the obligation owner to determine the resulting financial position. |

Events remain distinct from these business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

L2-F02 states that Payment authorization is externally governed. It does not define a Payment Cancellation approval role or specific approval chain.

### 15.2 Separation of Duties

No Payment Cancellation-specific separation-of-duty rule is defined in the accessed authoritative Payment architecture.

### 15.3 Audit and Evidence

Cancellation must preserve:

- Payment identity;
- Payment Reference and Receipt Reference where applicable;
- Payment Source;
- Payment Instrument and Channel;
- cancellation business evidence;
- state-transition history;
- resulting Student Charge consequence where applicable;
- complete historical lineage.

L2-F02 requires immutable Payment history, auditable state transitions and complete reconstruction of financial history.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The Payment is identified.
- Cancellation is permitted by the applicable business authority/policy.
- The Payment reaches the existing Cancelled state through an authorized transition.
- Historical Payment information remains immutable and traceable.
- No unsupported cancellation event or state is introduced.
- Any Student Charge consequence is handled by the canonical Student Charge owner.

### 16.2 Postconditions

- Payment remains Payment Management-owned.
- Student Charge remains Student Charge Management-owned.
- Cancelled Payment history remains immutable.
- Downstream consumers receive only authoritative completed facts.
- No financial obligation is silently deleted or rewritten.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Material Upstream Gap — Cancellation Transition and Event

L2-F02 defines `Cancelled` as a canonical Payment state but does not provide a complete explicit transition matrix identifying every permitted predecessor state for cancellation. It also does not define a `PaymentCancelled` event.

**Impact:** The L3 can safely preserve the existing Cancelled state and the ownership/boundary model, but cannot assert a complete predecessor-to-Cancelled transition or invent a cancellation event.

**Required governance:** Confirm the permitted cancellation predecessor states, cancellation authority/policy, and canonical business event for cancellation if downstream consumers require an explicit completed fact.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Until the cancellation transition/event semantics are confirmed, this remains a source-grounded Draft rather than an Architecture Approved or Locked scenario.
