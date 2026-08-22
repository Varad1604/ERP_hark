# Payment Population to Governed Bulk Collection Outcome

**Scenario ID:** L3-F-013  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Payment Management  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-F00 Finance Domain Model & Foundation; L2-F02 Payment Management Architecture; applicable Student Lifecycle architecture for the population context  
**Referenced By:** Student Charge & Fee Management; Accounting & General Ledger; Accounts Receivable; Financial Reporting

## 2. Scenario Purpose

Establish a governed institutional outcome for collecting a defined population of Payments as a coordinated business activity while preserving the canonical Payment aggregate, its financial lineage, and the ownership of each individual settlement.

The scenario covers the institutional outcome of a bulk collection activity, not a technical batch-processing mechanism.

## 3. Business Outcome

A defined Payment population is governed as a bulk collection outcome, with the participating Payments remaining individually canonical, traceable, and subject to the same Payment ownership and financial-integrity constraints as all other Payments.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an institutionally defined population of Payments is identified for coordinated collection.

### 4.2 End Boundary

The scenario ends when the population has reached the authoritative governed bulk-collection outcome supported by the applicable Payment architecture, with individual Payment financial facts preserved.

### 4.3 In Scope

- Identification of the Payment population.
- Preservation of individual Payment identity and ownership.
- Governance of the collection population as one institutional business activity.
- Determination of the resulting business position for the population.
- Preservation of individual Payment history and traceability.
- Communication of completed Payment facts through canonical Payment events where applicable.

### 4.4 Out of Scope

- Technical batch jobs or file formats.
- Queueing, retry counts, infrastructure recovery, database transactions, or implementation mechanics.
- Creation of a new bulk-payment aggregate unless separately established upstream.
- Direct modification of Student Charges by Payment Management.
- Accounting posting mechanics.
- Reconciliation execution.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| C — Student Lifecycle | Provides authoritative student context where the collection population is defined from student context. | Student context remains C-owned. |
| F — Payment Management | Owns the individual Payment aggregates and the Payment settlement architecture. | Payment remains F-owned. |
| F — Student Charge & Fee Management | Owns Student Charges that may be the financial obligations underlying collected Payments. | Student Charge remains F01-owned. |

The frozen Stage 3 participating domains are preserved as `C, F`; participation does not transfer Payment or Student Charge ownership.

## 6. Business Actors, Roles and Responsibilities

L2-F02 does not contain an authoritative actor section. No collection officer, cashier, finance administrator, student, sponsor, or batch operator is invented.

Business authority for any population-selection or collection-governance decision must come from the applicable upstream business architecture.

## 7. Preconditions and Entry Criteria

- A population of Payments is identifiable from authoritative business context.
- Each Payment retains its canonical Payment identity and financial lineage.
- The participating Student Lifecycle context, where applicable, is authoritative.
- The business purpose for coordinated collection is established.

## 8. Trigger

An institutionally governed collection requirement identifies a population of Payments for coordinated bulk collection treatment.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Establish the business population of Payments subject to coordinated collection. | Payment Management with applicable C context | A governed Payment population is identified. |
| 2 | Confirm that the population consists of canonical Payments and preserve each Payment's source, instrument, channel, reference and current business position. | Payment Management | Individual Payment integrity remains intact. |
| 3 | Apply the applicable business governance to the collection population without redefining Payment ownership or Student Charge ownership. | Payment Management / governing business authority | The population is treated as one institutional collection activity while individual aggregates remain canonical. |
| 4 | Determine the collection outcome for the population using only authoritative Payment states, lifecycle positions, and business facts. | Payment Management | The collection activity reaches its supported business outcome. |
| 5 | Preserve the resulting history and communicate completed Payment facts using the canonical Payment event architecture where an event is defined for the completed fact. | Payment Management | Downstream Finance domains can consume authoritative settlement information. |
| 6 | Where individual collection outcomes affect Student Charges, the Student Charge domain remains responsible for its own financial obligation state. | Student Charge & Fee Management | Financial obligation ownership remains intact. |

## 10. Alternate Business Flows

No scenario-specific alternate bulk-collection path is explicitly defined in the authoritative L2-F02 architecture.

Where a governed collection population contains Payments at different valid business positions, the scenario must preserve those individual Payment positions rather than inventing a population-level state that replaces them.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| A Payment in the population cannot participate in the governed collection outcome | The individual Payment cannot be represented as successfully completed solely because the population is being processed collectively. | Preserve the Payment's authoritative business position and apply only an existing Payment exception/recovery path. | Payment Management |
| Population membership is not authoritative | The collection outcome cannot safely be attributed to the intended Payments. | Do not infer membership; resolve the authoritative population definition before treating the collection activity as complete. | Payment Management / governing domain |
| Individual Payments reach different valid outcomes | A single population result would conceal individual financial truth. | Preserve each Payment's canonical outcome and financial lineage. | Payment Management |
| A required downstream financial consequence is not established | The bulk activity cannot be treated as fully completed for that consequence. | Preserve the authoritative Payment facts and allow the owning downstream domain to govern its own position. | Canonical owning domain |

No technical retry, queue, timeout, rollback, or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Payment is the canonical settlement aggregate. | Payment Management | Every collected settlement remains an individual Payment. | L2-F02 |
| External domains shall not modify Payments directly. | Payment Management | Population coordination cannot transfer Payment ownership. | L2-F02 |
| Every Payment has one Payment Source, at least one Payment Instrument, and one Payment Channel. | Payment Management | Collection population must preserve individual Payment context. | L2-F02 |
| Payment Allocation does not own monetary settlement. | Payment Management | Bulk collection cannot create a population-level substitute for Payment/Settlement ownership. | L2-F02 |
| Settlement derives from successful Payment Allocation. | Payment Management | Only supported settled outcomes may be represented as completed settlement. | L2-F02 |
| Payment history and references remain traceable and immutable. | Payment Management | Bulk collection cannot rewrite individual Payment history. | L2-F02 |

## 13. State and Lifecycle Impact

The canonical Payment lifecycle remains:

`Payment Received → Validated → Published → Allocated → Settled → Archived`

The scenario does not introduce a bulk-collection lifecycle.

Relevant Payment operational states remain the existing F02 states, including `Draft`, `Pending Validation`, `Validated`, `Published`, `Gateway Settlement Pending`, `Allocated`, `Partially Settled`, `Fully Settled`, `Suspended`, `Cancelled`, `Cheque Dishonoured`, and `Archived`.

A bulk collection activity coordinates individual Payment outcomes; it does not replace the Payment lifecycle or state model.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Payment Events

L2-F02 defines:

- `PaymentReceptionInitiated`
- `PaymentValidationCompleted`
- `PaymentPublicationCompleted`
- `GatewaySettlementConfirmed`
- `ChequeDishonourRecorded`
- `PaymentAllocationCompleted`
- `PaymentSettlementCompleted`
- `PaymentArchivalCompleted`

These remain Payment-domain events. No new `BulkCollectionCompleted` or `BulkPaymentProcessed` event is invented.

### 14.2 Synchronous Business Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C — Student Lifecycle | Payment Management | Authoritative student/population context | Establishes the business population where student context determines membership. |
| Payment Management | Student Charge & Fee Management | Individual authoritative Payment consequence | Allows the obligation owner to maintain its own financial position. |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

No authoritative Payment-specific bulk-collection actor or approval authority is defined in L2-F02. No role or approval chain is invented.

### 15.2 Separation of Duties

No scenario-specific bulk-collection separation-of-duty rule is defined by the accessed Payment architecture.

### 15.3 Audit and Evidence

The bulk collection outcome must remain reconstructable from:

- the governed population definition;
- individual Payment references;
- individual Payment state/lifecycle positions;
- collection outcome evidence;
- resulting Payment events where defined;
- downstream financial consequences.

F02 requires auditable Payment history, immutable references, and permanent lifecycle/state traceability.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The governed Payment population is established.
- Each participating Payment remains individually identifiable and owned by Payment Management.
- The collection outcome does not falsely collapse individual Payment states into an invented population state.
- All completed Payment facts use the canonical Payment event vocabulary.
- Any Student Charge consequence remains Student Charge-owned.

### 16.2 Postconditions

- Payment ownership remains exclusively with Payment Management.
- Individual Payment financial lineage remains reconstructable.
- No new bulk Payment aggregate, lifecycle, state, or event has been introduced.
- Downstream Finance domains consume only authoritative business facts.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Material Upstream Gap — Bulk Collection Definition

The frozen Stage 3 scenario explicitly requires **“Payment Population to Governed Bulk Collection Outcome”**, but the accessed locked L2-F02 Payment Management architecture defines Payment reception, allocation, settlement, lifecycle, state, events and exceptions without defining a dedicated bulk-collection capability, population aggregate, bulk-collection state, bulk-collection event, or explicit bulk-collection decision path.

**Impact:** The L3 can define the institutional boundary and preserve individual Payment ownership, but it cannot safely specify the detailed governed bulk-collection mechanics or a canonical population-level terminal state without inventing upstream architecture.

**Required governance:** Confirm whether bulk collection is an existing Payment-domain capability, a capability of another Finance domain, or an L3 orchestration outcome composed from existing Payment business facts. If a population-level state/event is required, its canonical owner and publisher must be established upstream.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Until the bulk-collection semantics are explicitly governed, this scenario remains a source-grounded Draft and must not be treated as Architecture Approved or Locked.
