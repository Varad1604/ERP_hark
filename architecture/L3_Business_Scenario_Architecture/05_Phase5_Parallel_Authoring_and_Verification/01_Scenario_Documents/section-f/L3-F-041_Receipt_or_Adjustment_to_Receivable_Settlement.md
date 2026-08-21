# Receipt or Adjustment to Receivable Settlement

# 1. Scenario Metadata

**Scenario ID:** L3-F-041  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F02_Payment_Management_Architecture.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs how Accounts Receivable incorporates a completed receipt allocation or authoritative financial adjustment into an existing receivable. It preserves separate ownership of Payment, Financial Adjustment, Accounting, the originating obligation, and the `Receivable` while producing a consistent partial or full settlement position with complete financial lineage.

---

# 3. Business Outcome

The authoritative receipt allocation or adjustment has been recorded against the correct receivable, its outstanding balance and settlement references are current, and the receivable is either partially settled or fully settled with an immutable, auditable completion fact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Accounts Receivable receives a completed authoritative business fact showing either that a Payment has been allocated to an obligation represented by a receivable or that an approved and applicable Financial Adjustment modifies that receivable's financial position.

## 4.2 End Boundary

The scenario ends when the source fact is validated and incorporated once, the receivable balance and settlement traceability are consistent, and Accounts Receivable has published `ReceivableSettlementRecorded` and, where no outstanding amount remains, `ReceivableFullySettled`.

## 4.3 In Scope

- Identification of the receivable affected by a receipt allocation or Financial Adjustment.
- Validation of source ownership, completion, amount, references, and applicability.
- Recording of settlement or adjustment references within the `Receivable` aggregate.
- Determination and recording of partial or full receivable settlement.
- Update of outstanding amount, aging, and collection visibility affected by settlement.
- Preservation of Payment, Adjustment, obligation, Accounting, and receivable lineage.
- Publication of canonical receivable settlement facts.

## 4.4 Out of Scope

- Payment reception, validation, allocation decision, or ownership.
- Financial Adjustment initiation, validation, approval, publication, application, or ownership.
- Modification of the originating financial obligation.
- Refund execution, Payment reversal, adjustment reversal, write-off, or reopening.
- Accounting Journal creation or Ledger posting.
- Technical allocation, persistence, interface, or transaction design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-039` Published Financial Obligation to Accounts Receivable Recognition | Establishes the receivable that later receives settlement information. |
| Related | `L3-F-040` Overdue Receivable to Governed Dunning Outcome | Settlement may stop or change collection activity for an overdue receivable. |
| Related | `L3-F-043` New Evidence to Reopened Receivable | A later reversal or authoritative change may require a previously resolved receivable to reopen. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounts Receivable / Section F | Owns the receivable representation, outstanding balance, settlement references, aging and collection visibility, and final partial or full receivable-settlement outcome. |
| Payment Management / Section F | Owns the Payment, allocation, and monetary settlement fact and preserves its immutable allocation lineage. |
| Financial Adjustment Management / Section F | Owns the Financial Adjustment lifecycle and publishes the authoritative adjustment fact affecting the financial position. |
| Accounting & General Ledger / Section F | Owns any Accounting consequence and references the source financial facts without owning the operational settlement. |
| Originating obligation owner / Section F | Retains ownership of the financial obligation represented by the receivable. |

Settlement participation never transfers aggregate ownership. Accounts Receivable records references to completed source facts; it does not execute Payment allocation, approve Adjustments, or perform Accounting.

---

# 6. Business Actors, Roles and Responsibilities

The applicable L2 sources do not define canonical business actors for incorporating Payment allocation or Adjustment facts into Accounts Receivable, reviewing settlement exceptions, or confirming full settlement. This scenario therefore assigns responsibilities to canonical domains and records the actor and authority gap in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid `Receivable` exists with exactly one originating financial-obligation reference.
- The receivable has a positive outstanding amount before the new source fact is incorporated.
- A Payment allocation or applicable Financial Adjustment fact has completed the required source-owned lifecycle stage.
- The source fact identifies its canonical aggregate, amount, financial obligation or receivable relationship, and immutable reference.
- The source amount has not already been incorporated into the receivable.
- The source fact and receivable use a financially compatible institutional, obligation, currency, and accounting context.
- The receivable is not in a terminal position that prohibits the update unless a separately governed reopening has occurred.

---

# 8. Trigger

Accounts Receivable consumes a completed Payment allocation/settlement fact or a published and applicable Financial Adjustment fact that changes the amount outstanding on a represented financial obligation.

The exact canonical event names and the Adjustment lifecycle point that authorizes receivable impact are inconsistent across the locked L2 sources and remain recorded in Section 17.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Complete the Payment allocation or Financial Adjustment milestone under the source aggregate's own validation, approval, and lifecycle rules. | Payment Management or Financial Adjustment Management | An authoritative immutable financial fact is available to Accounts Receivable. |
| 2 | Receive the source fact and identify the `Receivable` through its originating obligation and source references. | Accounts Receivable | The correct receivable and settlement basis are identified. |
| 3 | Validate source ownership, lifecycle completion, amount, currency context, applicability, uniqueness, and financial traceability. | Accounts Receivable | The fact is eligible for incorporation exactly once. |
| 4 | Record the immutable Payment allocation or Adjustment reference within the `Receivable` aggregate without modifying the source aggregate. | Accounts Receivable | Settlement lineage is preserved across bounded contexts. |
| 5 | Apply the authoritative financial effect to the receivable's outstanding balance and preserve monetary integrity. | Accounts Receivable | The receivable reflects the current remaining obligation. |
| 6 | Re-evaluate aging and collection visibility affected by the new outstanding position. | Accounts Receivable | Collection information remains consistent with the receivable balance. |
| 7 | Determine whether the result is partial settlement or complete settlement. | Accounts Receivable | The receivable has a valid settlement outcome. |
| 8 | Validate aggregate consistency, references, balance, and financial lineage. | Accounts Receivable | The settlement update is complete and auditable. |
| 9 | Publish `ReceivableSettlementRecorded`; if the outstanding amount is fully satisfied, also publish `ReceivableFullySettled`. | Accounts Receivable | Governed consumers receive the completed receivable settlement facts. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Partial receipt settlement | The allocated Payment is less than the receivable's outstanding amount. | Record the allocation reference, reduce the outstanding balance, and retain the receivable in an active unsettled position. | The receivable is partially settled and remains available for later settlement or collection. | Payment Management and Accounts Receivable |
| Full receipt settlement | The valid allocation exactly satisfies the outstanding amount. | Record the allocation, reduce the outstanding balance to zero, complete applicable collection activity, and publish the full-settlement fact. | The receivable becomes fully settled. | Payment Management and Accounts Receivable |
| Partial adjustment settlement | An applicable credit adjustment reduces but does not eliminate the outstanding amount. | Record the adjustment reference and effect while keeping the remaining receivable active. | The receivable is partially settled through an authoritative adjustment. | Financial Adjustment Management and Accounts Receivable |
| Full adjustment settlement | An applicable approved adjustment eliminates the remaining outstanding amount. | Record the adjustment reference, reduce the outstanding amount to zero, and publish the full-settlement fact. | The receivable becomes fully settled without representing the Adjustment as a Payment. | Financial Adjustment Management and Accounts Receivable |
| Combined settlement | Multiple valid Payment allocations and/or Adjustments together satisfy the receivable. | Incorporate each authoritative fact once in its completion order and verify the resulting cumulative balance. | The receivable is partially or fully settled with reconstructable multi-source lineage. | Accounts Receivable and relevant Finance source owners |
| Excess amount | A source fact would reduce the receivable below zero. | Do not force a negative balance; withhold the conflicting portion and return the excess treatment to the canonical source-owned advance, refund, allocation, or adjustment governance. | The receivable remains financially valid pending source resolution. | Accounts Receivable and relevant source owner |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The source fact lacks a valid receivable or obligation relationship. | Settlement cannot be assigned without risking the wrong financial position. | Withhold incorporation and require the source owner to correct or complete the authoritative reference. | Payment Management or Financial Adjustment Management |
| The source fact has already been incorporated. | Reapplication would duplicate settlement and understate the receivable. | Preserve the existing reference and reject duplicate financial effect while retaining the repeated fact for investigation. | Accounts Receivable |
| Amount, currency, institution, or obligation context conflicts. | Monetary integrity and traceability would be compromised. | Place the settlement update under review and reconcile the source-owned and receivable facts before incorporation. | Accounts Receivable and affected source owner |
| The source fact exceeds the outstanding amount. | The receivable could become negative or absorb an amount it does not own. | Apply no unsupported excess treatment; resolve it through the canonical Payment allocation, advance, refund, or Adjustment process. | Relevant source owner and Accounts Receivable |
| The receivable is already fully settled, closed, written off, or otherwise ineligible. | The new fact cannot be applied within the current receivable lifecycle. | Preserve the new evidence and route it to the applicable correction, reversal, or reopening scenario before any receivable modification. | Accounts Receivable and source owner |
| Aggregate validation fails after the source fact is received. | No valid receivable settlement outcome exists. | Do not publish settlement events; retain the last valid receivable position and resolve the inconsistent source or reference under its canonical owner. | Accounts Receivable and affected source owner |
| A later Payment reversal or Adjustment reversal invalidates prior settlement. | The receivable may no longer be fully or partially satisfied. | Preserve the original settlement history and process the new authoritative evidence through the governed reopening or correction scenario. | Payment/Adjustment owner and Accounts Receivable |

No failed-settlement or rejected-settlement state/event is defined for the `Receivable` in `L2-F07`; none is introduced here.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Payments represent settlement and settle receivables; they do not establish obligations. | Payment Management | A receipt path must originate from a canonical Payment and valid allocation rather than an Accounts Receivable-created settlement. | `L2-F02`, Sections 1, 6 and 13 |
| Payment allocation associates one Payment with one or more obligations, preserves monetary integrity, is auditable, and remains reconstructable. | Payment Management | Accounts Receivable may incorporate only a valid allocation relationship and immutable reference. | `L2-F02`, Section 12 |
| Payment Settlement always derives from successful allocation, supports partial or complete settlement, preserves lineage, and is immutable after completion. | Payment Management | A received Payment alone is insufficient; the applicable allocation/settlement fact must be complete. | `L2-F02`, Section 13 |
| Financial Adjustment is the canonical representation of an approved modification to an existing financial position. | Financial Adjustment Management | An Adjustment affects the receivable only through its authoritative source-owned lifecycle and event. | `L2-F04`, Sections 2-4 |
| Only validated and approved Adjustments may be published; publication preserves immutability, auditability, and ownership. | Financial Adjustment Management | Accounts Receivable cannot use a Draft or unapproved adjustment basis. | `L2-F04`, Sections 15-16 |
| Receivable Balance is authoritative within the aggregate, while balance effects are governed by upstream financial events. | Accounts Receivable and source Finance owners | The receivable incorporates source facts but does not invent or re-own them. | `L2-F07`, Section 10.4 and `INV-AR-02` |
| Settlement references never invalidate aggregate consistency. | Accounts Receivable | Every Payment or Adjustment reference must preserve the receivable boundary and financial integrity. | `L2-F07`, `INV-AR-05` |
| Accounts Receivable does not own Payment execution or Accounting entries. | Payment Management and Accounting & General Ledger | Settlement recording cannot execute payment or post the Ledger. | `L2-F07`, Sections 10.8 and 9 |
| Events publish only after completed valid aggregate transitions. | Publishing domain | No source or receivable settlement event may represent a partial or invalid update. | `L2-F02`, Section 17; `L2-F04`, Section 17; `L2-F07`, Sections 11.1-11.4 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Payment | Successfully received and eligible for allocation | Allocation and Settlement complete within Payment Management; Accounts Receivable records only their immutable references and effect. | Source-defined allocated/settled lifecycle position | Payment Management | `L2-F02`, Sections 12, 13 and 16 |
| Financial Adjustment | Validated and approved; publication/application stage applicable to receivable impact | Its authoritative financial effect is referenced and incorporated without transferring ownership. | Source-defined `Published`, `Applied`, or `Completed` position, subject to the open sequencing issue | Financial Adjustment Management | `L2-F04`, Sections 15-17 |
| `Receivable` | Active with positive outstanding amount | Records settlement reference, updates balance, aging, and collection visibility, and determines partial or full satisfaction. | Partially settled or `Fully Settled`; exact canonical aggregate state transition is not separately defined | Accounts Receivable | `L2-F07`, Sections 6.5, 10.4, 10.8, 11.3 and States appendix |
| Originating Financial Obligation | Existing source-owned lifecycle position | Remains referenced; Accounts Receivable settlement does not directly modify it. | Unchanged unless its canonical owner defines a related effect | Originating obligation owner | `L2-F07`, Sections 2.2 and 6.7 |

`L2-F07` implies Settled through events but does not define a complete receivable state machine or a named partial-settlement state. The exact lifecycle remains open in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PaymentAllocationCompleted` / `PaymentSettlementCompleted` | Payment Management | Accounts Receivable and other governed Finance consumers | Supplies the owning Payment domain's completed allocation or settlement fact. | `L2-F02`, Section 17 |
| `PaymentAllocated` | Payment Management | Accounts Receivable | Event name expected by Accounts Receivable for settlement participation. | `L2-F07`, Section 11.5 |
| `AdjustmentApproved`, `AdjustmentPublicationCompleted`, `AdjustmentApplicationCompleted`, or `AdjustmentFinancialPositionUpdated` | Financial Adjustment Management | Accounts Receivable and other governed Finance consumers, where applicable | Supplies the Adjustment domain's approved, published, applied, or financial-position fact. | `L2-F04`, Sections 17.4-17.5 |
| `FinancialAdjustmentApproved` | Financial Adjustment Management | Accounts Receivable | Event name expected by Accounts Receivable for adjustment participation. | `L2-F07`, Section 11.5 |
| `ReceivableSettlementRecorded` | Accounts Receivable | Financial Reporting, Treasury, dashboards, Audit, Enterprise Analytics, and other governed consumers | Announces incorporation of settlement information into the receivable. | `L2-F07`, Sections 11.3-11.4 and 12.4 |
| `ReceivableFullySettled` | Accounts Receivable | Financial Reporting, Treasury, dashboards, Audit, Enterprise Analytics, and other governed consumers | Announces complete satisfaction of the receivable's outstanding amount. | `L2-F07`, Sections 11.3-11.4 and 12.4 |

The conflicting source-event names and Adjustment authorization point are retained in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Payment Management | Accounts Receivable | Immutable Payment allocation/Settlement fact with Payment, obligation, amount, and lineage references | Enables partial or complete receivable settlement without transferring Payment ownership. | `L2-F02`, Sections 12-13; `L2-F07`, Sections 9 and 12.3 |
| Financial Adjustment Management | Accounts Receivable | Immutable approved and applicable Adjustment fact with source, reason, amount, reference, lifecycle, and audit lineage | Enables a governed receivable financial-position change without transferring Adjustment ownership. | `L2-F04`, Sections 15-17 and 25; `L2-F07`, Section 12.3 |
| Accounts Receivable | Accounting & General Ledger | Completed receivable settlement fact and source references | Supports Accounting traceability while Accounting retains ownership of Journal and Ledger consequences. | `L2-F07`, Sections 5.1, 9 and 12 |
| Accounts Receivable | Governed downstream consumers | Updated outstanding amount and partial/full settlement visibility | Supports reporting, collection, treasury, analytics, dashboards, and audit. | `L2-F07`, Sections 5.2 and 12.4 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Complete Payment allocation/Settlement | No canonical actor defined in `L2-F02` | Authority and decision must remain within Payment Management's deterministic, auditable allocation governance. | Payment Management |
| Approve, publish, and apply Financial Adjustment | No canonical actor defined in the applicable source crosswalk | Only validated and approved Adjustments may be published; exact receivable-impact authority and stage remain open. | Financial Adjustment Management |
| Incorporate the source fact and determine receivable settlement | No canonical actor defined in `L2-F07` | The decision is constrained by authoritative source facts, uniqueness, monetary integrity, and Receivable invariants. | Accounts Receivable |
| Confirm full settlement | No canonical actor defined in `L2-F07` | Full settlement requires zero remaining outstanding amount after all applicable valid source facts are incorporated. | Accounts Receivable |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Allocation versus receivable update | Payment Management and Accounts Receivable | Payment owns allocation and Settlement; Accounts Receivable owns its balance and settlement visibility. |
| Adjustment approval versus receivable update | Financial Adjustment Management and Accounts Receivable | Adjustment owns approval and applicable financial effect; Accounts Receivable owns incorporation into the receivable. |
| Accounting participation | Source Finance domains and Accounting & General Ledger | Accounting records consequences but does not re-own Payment, Adjustment, obligation, or Receivable aggregates. |
| Excess, duplicate, or conflicting source fact | Source owner and Accounts Receivable | No local override may force an invalid negative, duplicated, or untraceable settlement position. |
| Later reversal | Source reversal authority and Accounts Receivable | Original settlement evidence remains immutable; reversal and reopening occur through separately governed facts and scenarios. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove Payment settlement basis | Payment identity, payer/source, allocation, obligation relationship, amount, completion event, and audit history | Payment Management |
| Prove Adjustment basis | Adjustment identity, source, category, reason, references, validation, approval, publication/application evidence, amount, and event history | Financial Adjustment Management |
| Prove receivable incorporation | Receivable identity, originating obligation, unique source reference, previous balance, applied effect, resulting balance, and validation evidence | Accounts Receivable |
| Prove partial or full outcome | Outstanding amount calculation from authoritative facts, settlement classification, aging/collection impact, and applicable settlement events | Accounts Receivable |
| Preserve cross-domain lineage | Immutable relationship from obligation through Payment or Adjustment to Receivable and Accounting references | Each canonical owner for its aggregate |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The source Payment allocation/Settlement or Financial Adjustment fact is authoritative, completed to the required source-owned stage, immutable, and traceable.
- The correct receivable and originating obligation are identified.
- The source fact is incorporated exactly once and does not make the receivable negative.
- The receivable settlement reference, outstanding balance, aging, and collection visibility are mutually consistent.
- Partial versus full settlement is determined from the remaining authoritative outstanding amount.
- Aggregate consistency, financial integrity, and complete lineage are validated.
- `ReceivableSettlementRecorded` is published and `ReceivableFullySettled` is published when the balance is completely satisfied.

## 16.2 Postconditions

- Accounts Receivable retains ownership of the updated `Receivable` and its settlement representation.
- Payment Management or Financial Adjustment Management retains ownership of the incorporated source fact.
- The originating obligation remains owned by its authoritative Finance bounded context.
- A partially settled receivable remains available for later settlement, collection, adjustment, or write-off governance.
- A fully settled receivable is protected from further ordinary collection and awaits only permitted correction, reversal, reopening, closure, or archival behaviour.
- Accounting and downstream consumers may use the completed facts without modifying or re-owning source aggregates.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Event naming is inconsistent: `L2-F07` consumes `PaymentAllocated`, while `L2-F02` defines `PaymentAllocationCompleted` and `PaymentSettlementCompleted`. | Section 8 and Section 14 cannot lock one canonical Payment trigger name without overriding a locked source. | Payment Management, Accounts Receivable, and Lead Architect must align the canonical event contract. |
| Event naming and lifecycle sequencing are inconsistent: `L2-F07` consumes `FinancialAdjustmentApproved`, while `L2-F04` defines `AdjustmentApproved`, `AdjustmentPublicationCompleted`, `AdjustmentApplicationCompleted`, and `AdjustmentFinancialPositionUpdated`. | The Draft cannot determine whether approval, publication, application, or completion authorizes the receivable balance effect. | Financial Adjustment Management, Accounts Receivable, and Lead Architect must define the canonical event and authoritative receivable-impact stage. |
| `L2-F07` has no canonical actor definitions, while the Payment and Adjustment sources also leave material settlement/application authority unspecified. | Sections 6 and 15 cannot identify initiators, reviewers, approvers, exception authorities, or full-settlement confirmation roles. | Relevant Finance domain owners must define canonical business actors and decision authority. |
| `L2-F07` implies Created, Settled, Written Off, Reopened, and Closed positions through events but defines no canonical Receivable state machine or named partial-settlement state. | Section 13 cannot lock exact state transitions for partial and full settlement. | Accounts Receivable owner and Lead Architect must define the canonical lifecycle and transition guards. |
| The cross-source business rule for cumulative settlement ordering, effective dates, currency conversion, rounding, tolerance, and simultaneous Payment/Adjustment facts is not defined. | The Draft can require deterministic monetary integrity but cannot specify all balance-decision rules. | Payment, Financial Adjustment, Accounts Receivable, and Finance foundation owners must define the calculation and ordering policy upstream. |
| Exact excess-settlement treatment and hand-off are incomplete across receivable, Payment advance, refund, and adjustment architecture. | Section 10 and Section 11 cannot determine whether excess becomes an unallocated liability, reallocation, refund basis, or another governed outcome in every case. | Payment, Refund, Financial Adjustment, and Accounts Receivable owners must define the canonical excess treatment contract. |
| `L2-F07` defines `ReceivableSettlementRecorded` and `ReceivableFullySettled` but not a settlement-rejected, settlement-deferred, or partial-settlement event. | Downstream consumers may lack a precise fact for partial or exceptional settlement outcomes. | Accounts Receivable owner must define any additional events required or confirm that the generic event set is sufficient. |
| The sources do not define when a fully settled receivable becomes `Closed`, whether collection completion is automatic, or which later facts mandate reopening. | Post-settlement lifecycle and relationship to F-040 and F-043 cannot be locked. | Accounts Receivable owner must define full-settlement, collection-completion, closure, and reopening semantics. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-041**
