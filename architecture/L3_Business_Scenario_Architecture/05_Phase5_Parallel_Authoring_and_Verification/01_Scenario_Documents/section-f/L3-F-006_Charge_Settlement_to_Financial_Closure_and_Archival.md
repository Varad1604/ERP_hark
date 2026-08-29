# Charge Settlement to Financial Closure and Archival

**Scenario ID:** L3-F-006  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Charge & Fee Management  
**Participating Domains:** C — Student Lifecycle; F — Finance  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-F00 Finance Domain Model & Foundation; L2-F01 Student Charge & Fee Management; L2-F02 Payment Management; L2-F07 Accounts Receivable Management; authoritative Student Lifecycle context where relevant  
**Referenced By:** Accounting, Financial Reporting, Audit and downstream financial history consumers

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-006 |
| Canonical Title | Charge Settlement to Financial Closure and Archival |
| Primary Owner | Student Charge & Fee Management |
| Participating Domains | C — Student Lifecycle; F — Finance |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Complete the institutional lifecycle of a Student Charge after its financial obligation has been fully settled, bringing the charge to its governed financial closure and eventual archival while preserving the complete historical lineage of the obligation and its settlement.

The scenario distinguishes settlement ownership from Student Charge ownership: Payment Management owns payment settlement, while Student Charge & Fee Management owns the Student Charge lifecycle and its transition to Fully Settled and Archived.

## 3. Business Outcome

A Student Charge whose financial obligation has been fully settled is recognized as **Fully Settled**, subsequently reaches **Archived** in accordance with the canonical lifecycle, and remains historically accessible and reconstructable.

Settlement itself is performed by downstream Finance capabilities. This scenario governs the Student Charge's financial closure after the authoritative settlement fact is available.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when authoritative downstream Finance information establishes that the Student Charge's financial obligation has been fully settled.

### 4.2 End Boundary

The scenario ends when the Student Charge has completed its governed lifecycle to Archived while remaining historically accessible and traceable.

### 4.3 In Scope

- Recognition of the completed settlement outcome.
- Confirmation that the Student Charge has no remaining payable obligation under the authoritative settlement position.
- Transition of the Student Charge to Fully Settled.
- Preservation of settlement lineage.
- Completion of the Student Charge lifecycle.
- Transition to Archived.
- Publication of applicable Student Charge lifecycle events.
- Preservation of historical financial accessibility.

### 4.4 Out of Scope

- Payment reception.
- Payment allocation.
- Bank execution.
- Payment reconciliation execution.
- Accounting posting.
- Refund processing.
- Financial Adjustment execution.
- Receivable creation or independent settlement processing.
- Technical archival, persistence, storage or infrastructure mechanics.

## 5. Ownership and Participating Domains

| Domain / Capability | Responsibility | Ownership |
|---|---|---|
| C — Student Lifecycle | Supplies student context where relevant to the originating obligation. | Student concepts remain C-owned. |
| F — Student Charge & Fee Management | Owns Student Charge lifecycle and closure/archival. | Student Charge remains F01-owned. |
| F — Payment Management | Owns payment settlement and publishes settlement facts. | Payment remains F02-owned. |
| F — Accounts Receivable | Owns receivable representation and settlement references where applicable. | Receivable remains F07-owned. |
| F — Accounting | Consumes the financial outcome for accounting responsibilities. | Accounting remains F05-owned. |

L2-F01 explicitly states that settlement processing remains outside the Student Charge architecture and that the Student Charge lifecycle includes Fully Settled and Archived.

## 6. Business Actors, Roles and Responsibilities

The applicable L2 sources do not define an authoritative actor catalogue for Student Charge settlement closure. No actor or role is invented.

Payment Management remains responsible for the business settlement outcome; Student Charge & Fee Management is responsible for applying the resulting lifecycle impact to its own canonical aggregate.

## 7. Preconditions and Entry Criteria

- A canonical Student Charge exists.
- The charge has reached a financially active state.
- Authoritative settlement information exists.
- The settlement information is sufficient to establish that the obligation is fully settled.
- Any relevant receivable representation is consistent with the settlement outcome.
- No unresolved financial adjustment or refund consequence prevents closure.

## 8. Trigger

A canonical downstream Finance settlement outcome establishes that the Student Charge's financial obligation is fully settled.

L2-F02 defines Payment as the canonical settlement aggregate and identifies `PaymentSettlementCompleted` as a canonical Payment lifecycle event.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Payment Management completes settlement against the relevant Student Charge. | F — Payment Management | An authoritative settlement fact exists. |
| 2 | The settlement outcome is made available to Student Charge & Fee Management. | F — Payment Management → F01 | F01 can determine the charge's current financial position. |
| 3 | Student Charge & Fee Management evaluates the charge's outstanding financial position using canonical settlement information. | F — Student Charge | The charge is determined to be fully settled or not yet fully settled. |
| 4 | Where the obligation is fully settled, the Student Charge progresses to the canonical Fully Settled lifecycle stage/state permitted by F01. | F — Student Charge | The financial obligation is closed from an outstanding-settlement perspective. |
| 5 | The Student Charge's settlement lineage remains traceable to the underlying payment/receivable financial facts. | F | The closed obligation remains explainable. |
| 6 | The Student Charge completes its operational lifecycle and progresses to Archived under the canonical F01 lifecycle. | F — Student Charge | The charge reaches its terminal historical lifecycle state. |
| 7 | The canonical Student Charge closure and archival events are published. | F — Student Charge | Downstream consumers can recognize the completed lifecycle. |

The flow ends at Archived. It does not continue into accounting, reporting, or technical archival mechanics.

## 10. Alternate Business Flows

### A1 — Partially Settled

The authoritative settlement position does not fully satisfy the Student Charge. The charge remains financially active and does not progress to Fully Settled.

### A2 — Settlement Reversed or Otherwise No Longer Final

If authoritative downstream Finance information establishes that the previous settlement position is no longer the valid final position, the Student Charge does not remain incorrectly closed. The appropriate downstream payment/adjustment process governs the new financial position before closure is reconsidered.

### A3 — Financial Adjustment Prevents Closure

A valid financial adjustment changes the outstanding position. The Student Charge remains outside final closure until the canonical financial position is resolved.

### A4 — Refund Consequence

A settlement outcome creates a valid refund consequence. Refund Management owns the refund process; Student Charge closure occurs only when the authoritative financial position supports it.

### A5 — Multiple Installments

A Student Charge with multiple installments reaches Fully Settled only after the authoritative financial position establishes that the entire Student Charge obligation has been settled, not merely one installment.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Resolution |
|---|---|
| Settlement information is incomplete | Do not close the Student Charge; await authoritative settlement information. |
| Outstanding balance remains | Keep the charge financially active and do not progress to Fully Settled. |
| Settlement is disputed or reversed | Keep the charge from final closure until the canonical financial position is resolved. |
| Receivable position is inconsistent | Resolve the receivable position through Accounts Receivable ownership before final closure. |
| Adjustment or refund consequence remains unresolved | Do not archive the Student Charge until the resulting financial position is authoritative. |
| Historical lineage cannot be established | Do not treat the obligation as safely closed; preserve the charge in its existing lifecycle until lineage is restored through the governing business process. |

No technical rollback, retry, queue, server, database, or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

Authoritative constraints include:

- Student Charge is the canonical financial obligation.
- Settlement is a downstream responsibility and does not transfer Student Charge ownership.
- Outstanding Balance is derived from canonical financial information.
- A Student Charge progresses to Fully Settled only when its associated financial obligation has been completely settled.
- Archived Student Charges remain accessible and historically traceable.
- Historical lifecycle stages remain immutable.
- Payment Management owns Payment and Settlement.
- Accounts Receivable owns the Receivable representation.
- Accounting remains downstream of the Student Charge and Payment domains.

L2-F01 explicitly states that settlement processing is outside the Student Charge architecture, defines Fully Settled and Archived lifecycle stages, and requires archived charges to remain accessible.

L2-F02 defines Payment as the canonical settlement aggregate and includes `PaymentSettlementCompleted` in its canonical event set.

L2-F07 defines Receivable as the canonical Accounts Receivable aggregate and includes `ReceivableSettled` and `ReceivableClosed` among its domain events.

## 13. State and Lifecycle Impact

### Student Charge Lifecycle

The authoritative L2-F01 lifecycle is:

`Business Obligation → Charge Identification → Charge Generation → Charge Validation → Published → Financially Active → Fully Settled → Archived`

This scenario covers the final two lifecycle stages.

### Student Charge State

The canonical F01 state architecture includes `Closed` and `Archived` among its canonical states, with the lifecycle concept remaining distinct from current operational state. The scenario does not invent a new state.

### Payment State

L2-F02 includes `Fully Settled` and `Archived` among canonical Payment states. Payment remains Payment Management-owned.

### Receivable State / Outcome

Accounts Receivable maintains its own Receivable lifecycle and settlement references. It does not own the Student Charge lifecycle.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

**Payment Management publisher**

- `PaymentSettlementCompleted` — canonical Payment event published by Payment Management.

**Student Charge publisher**

L2-F01 identifies:

- `ChargeClosureCompleted`
- `ChargeArchivalCompleted`

as canonical Student Charge lifecycle events.

**Accounts Receivable publisher**

L2-F07 identifies:

- `ReceivableSettled`
- `ReceivableClosed`

as canonical Accounts Receivable events.

Each event remains published by its canonical owning domain.

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Payment Management | Student Charge Management | Authoritative completed settlement fact | Allows F01 to determine whether the Student Charge is fully settled. |
| Accounts Receivable | Student Charge Management | Authoritative receivable settlement position where applicable | Supports final financial-position determination. |
| Student Charge Management | Accounting | Published financial closure facts | Allows Accounting to consume the authoritative business outcome. |
| Student Charge Management | Reporting / Analytics | Closed historical financial fact | Supports downstream financial history and reporting. |

Events remain distinct from these synchronous/business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

No scenario-specific settlement-closure approval authority is defined in the accessible F01/F02/F07 architecture. Closure follows the authoritative financial facts rather than an invented approval role.

### 15.2 Separation of Duties

No additional scenario-specific separation-of-duty rule is introduced.

### 15.3 Business Audit and Evidence Obligations

The completed lifecycle must remain traceable through:

- the original Student Charge;
- settlement information;
- any relevant receivable settlement reference;
- the Fully Settled outcome;
- closure and archival lifecycle events.

L2-F01 explicitly requires complete financial lineage, immutable historical states and reconstructable lifecycle history.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The authoritative Student Charge is identified.
- Authoritative settlement information establishes that the obligation is fully settled.
- Any material remaining financial consequence has been resolved or routed to its canonical owner.
- Student Charge reaches the canonical Fully Settled lifecycle stage.
- Student Charge completes its lifecycle to Archived.
- Closure and archival history remain traceable.
- Downstream Finance consumers can recognize the final financial outcome.

### 16.2 Postconditions

- The Student Charge is no longer an outstanding financial obligation.
- The Student Charge remains historically accessible.
- Settlement lineage is reconstructable.
- Payment, Receivable and Accounting ownership remains unchanged.
- No new financial obligation has been created by closure or archival.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material scenario-specific upstream issue was identified for the settlement-to-closure boundary in the accessible locked F01/F02/F07 architecture.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
