# Payment Failure or Timeout to Safe Recovery

**Scenario ID:** L3-F-010  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F00 Finance Domain Model & Foundation; L2-F02 Payment Management Architecture; applicable Treasury & Banking architecture where a banking-origin settlement fact is involved  
**Referenced By:** Accounting & General Ledger, Accounts Receivable, Financial Reporting, Treasury & Banking

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-010 |
| Canonical Title | Payment Failure or Timeout to Safe Recovery |
| Primary Owner | Payment Management |
| Participating Domains | F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Provide a controlled business outcome for a Payment whose normal progression cannot be completed because the financial result is unsuccessful or remains uncertain, while protecting the canonical Payment, preserving financial integrity, and ensuring that no unsupported successful settlement is claimed.

The scenario is business-level. It does not define technical timeout handling, retry mechanisms, queues, infrastructure recovery, or implementation error processing.

## 3. Business Outcome

A failed or outcome-uncertain Payment is brought to a safe, financially truthful state with its history preserved and with recovery, suspension, cancellation, or other terminal treatment applied only where supported by authoritative Payment architecture.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an attempted Payment cannot proceed to its expected successful financial outcome, or when the business outcome of the attempted Payment remains uncertain.

### 4.2 End Boundary

The scenario ends when the Payment has a known safe business position: successfully recovered into an authorized Payment state, explicitly suspended pending governed resolution, or placed into an authorized terminal outcome.

### 4.3 In Scope

- Recognition of unsuccessful or uncertain Payment outcome.
- Preservation of the canonical Payment identity and financial history.
- Business determination of whether the Payment can safely continue.
- Use of existing Payment states where permitted.
- Controlled suspension or cancellation where authorized.
- Preservation of references and audit lineage.
- Communication of only authoritative completed Payment facts.

### 4.4 Out of Scope

- Technical timeout detection.
- Technical retries, queues, dead-letter handling, server failures, rollback or infrastructure recovery.
- Bank reconciliation execution.
- Refund execution.
- Accounting posting mechanics.
- Creation of new Payment states or events.
- Changes to the Payment lifecycle architecture.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| F — Payment Management | Owns Payment reception, validation, state, lifecycle, references, exceptions and business recovery. | Payment remains Payment Management-owned. |
| F — Treasury & Banking | Participates only where an authoritative banking outcome is required to resolve the Payment's financial position. | Banking facts remain Treasury-owned. |
| F — Accounting & General Ledger | Consumes authoritative Payment facts where an accounting consequence exists. | Accounting records remain Accounting-owned. |

The frozen Stage 3 participation value for F-010 is F only. Other Finance domains are referenced only where their own canonical facts materially participate; participation does not transfer Payment ownership.

## 6. Business Actors, Roles and Responsibilities

L2-F02 explicitly records that an authoritative actor section is not present. No cashier, finance officer, gateway operator, approver, or other human role is invented.

Payment Management remains the canonical business owner of the Payment outcome. Enterprise authorization remains externally governed.

## 7. Preconditions and Entry Criteria

- A Payment context exists or an attempted settlement has generated a financially relevant business condition.
- The Payment cannot safely be treated as having completed its expected normal outcome.
- Existing Payment identity and references are preserved where available.
- The applicable business evidence is sufficient to determine whether recovery, suspension, cancellation, or another authorized outcome can be established.

## 8. Trigger

A Payment attempt results in a failed or uncertain financial outcome requiring governed recovery.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Payment Management recognizes that the Payment has not reached its expected successful business outcome. | Payment Management | The Payment enters exception handling at business level. |
| 2 | The Payment's existing identity, references, source, instrument and available financial context are preserved. | Payment Management | The financial history remains traceable. |
| 3 | Payment Management determines whether the Payment can be placed into an already-authorized safe state without falsely claiming settlement. | Payment Management | The permissible business position is identified. |
| 4 | Where the business evidence supports continued progression, the Payment proceeds through an existing authorized lifecycle/state path. | Payment Management | Recovery preserves the canonical Payment. |
| 5 | Where the Payment cannot safely continue, Payment Management applies an already-authorized exceptional state such as Suspended or Cancelled where enterprise policy permits. | Payment Management | The Payment is prevented from being represented as successfully settled. |
| 6 | Payment Management preserves the resulting business history and publishes only the canonical facts corresponding to completed state transitions. | Payment Management | Downstream consumers receive an authoritative financial position. |
| 7 | Where another Finance domain owns a resulting financial consequence, that domain processes its own aggregate from the authoritative Payment fact. | Owning Finance Domain | Cross-domain financial ownership remains intact. |

L2-F02 defines exception categories, controlled recovery, financial integrity, permanent exception history, and the canonical exceptional states Suspended and Cancelled. It does not define a dedicated technical timeout mechanism. 

## 10. Alternate Business Flows

### A1 — Payment Can Safely Continue

Where the business evidence establishes that the Payment remains valid and can continue under an existing authorized state transition, Payment Management continues the canonical Payment lifecycle.

### A2 — Payment Must Remain Suspended

Where the business outcome cannot yet be established and the architecture permits suspension, the Payment remains Suspended until a governed resolution is available.

### A3 — Payment Reaches a Controlled Cancellation Outcome

Where cancellation is permitted by the applicable business policy, Payment Management moves the Payment to the canonical Cancelled state and preserves the resulting history.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Payment outcome is unsuccessful | Payment cannot be treated as completed settlement | Preserve the Payment and apply only an authorized recovery or exceptional state. | Payment Management |
| Payment outcome remains uncertain | Financial settlement cannot safely be asserted | Preserve the Payment as unresolved/suspended where the authorized state architecture permits. | Payment Management |
| Payment cannot be safely recovered | Normal progression cannot continue | Apply an authorized terminal outcome where policy permits; otherwise retain the governed exception. | Payment Management |
| Banking evidence is required to establish the outcome | Payment position cannot be finalized from Payment context alone | Consume the authoritative banking business fact without transferring Payment ownership. | Treasury & Banking / Payment Management |

No technical retry, timeout algorithm, queue, rollback or infrastructure recovery is specified.

## 12. Business Rules and Decision Constraints

The scenario applies the following source-grounded constraints:

- Payment is the canonical settlement aggregate and remains exclusively owned by Payment Management.
- A Payment has exactly one active operational state.
- State transitions are deterministic.
- Historical states and completed lifecycle stages are immutable.
- Exceptions must preserve aggregate consistency, financial integrity and permanent traceability.
- Payment References remain immutable after publication.
- Settlement derives from Payment Allocation; a failed or uncertain Payment must not be represented as a completed settlement without the required business evidence.
- Authorization remains externally governed.
- Published events represent completed business facts and must remain immutable.

L2-F02 defines these constraints but does not define a dedicated “Payment Timeout”, “Payment Failed”, or “Payment Recovery” state/rule.

## 13. State and Lifecycle Impact

### Payment Lifecycle

The canonical lifecycle remains:

`Payment Received → Validated → Published → Allocated → Settled → Archived`

The scenario does not introduce a new lifecycle stage. 

### Payment State

Authoritative Payment states include:

`Draft`, `Pending Validation`, `Validated`, `Published`, `Gateway Settlement Pending`, `Allocated`, `Partially Settled`, `Fully Settled`, `Suspended`, `Cancelled`, `Cheque Dishonoured`, and `Archived`.

The scenario may use only these existing states. It must not create a new “Failed” or “Timeout” state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

L2-F02 defines the following canonical Payment events:

- `PaymentReceptionInitiated`
- `PaymentValidationCompleted`
- `PaymentPublicationCompleted`
- `GatewaySettlementConfirmed`
- `ChequeDishonourRecorded`
- `PaymentAllocationCompleted`
- `PaymentSettlementCompleted`
- `PaymentArchivalCompleted`

No dedicated `PaymentFailed`, `PaymentTimedOut`, `PaymentRecovered`, or `PaymentCancelled` event is defined in L2-F02. None is invented here.

### 14.2 Synchronous Business Contracts / Hand-offs

Where an immediate authoritative financial answer is required, the Payment Domain may participate through the enterprise's governed business-contract model. Where a completed financial fact has occurred, the canonical Payment event vocabulary applies.

A banking-origin fact remains owned by Treasury & Banking; it does not transfer Payment ownership.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Payment authorization is externally governed. L2-F02 does not define a dedicated Payment Failure or Cancellation approver.

### 15.2 Separation of Duties

No scenario-specific Payment Failure separation-of-duty rule is defined in the accessed L2-F02 architecture.

### 15.3 Audit and Evidence

The scenario preserves:

- Payment identity;
- Payment References;
- Payment Source;
- Payment Instrument and Channel;
- applicable state transition;
- business evidence supporting the recovery or terminal position;
- complete exception history;
- downstream financial references where applicable.

L2-F02 requires permanent auditability, immutable history and reconstructable Payment lineage.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

The scenario is complete when:

- the Payment is no longer represented as an unresolved normal-success outcome;
- its safe business position is established using an existing authorized state;
- no unsupported successful settlement is claimed;
- Payment history and references remain reconstructable;
- any resulting downstream Finance fact is handed to its canonical owner.

### 16.2 Postconditions

- Payment remains owned by Payment Management.
- Historical Payment information remains immutable.
- No new lifecycle or state concept is introduced.
- Downstream domains receive only authoritative completed facts.
- Any unresolved condition remains explicitly governed rather than silently treated as success.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Material Upstream Gap — Recovery Semantics

The frozen scenario title explicitly requires “Payment Failure or Timeout to Safe Recovery”, but L2-F02 does not define a canonical Payment Failure/Timeout state, a dedicated recovery transition, or a dedicated failure/recovery event. It defines generic exception architecture and the existing states Suspended and Cancelled, but does not establish which business conditions map to which recovery outcome.

**Impact:** The L3 document can preserve the safe business boundary and existing authorized states, but cannot author a definitive failure-to-recovery transition without inventing upstream architecture.

**Required governance:** Confirm the authoritative business meaning of Payment Failure/Timeout and its permitted recovery state(s)/transition(s), and whether a canonical event is required.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Until the upstream gap is governed, the scenario remains a complete source-grounded Draft rather than an Architecture Approved or Locked scenario.
