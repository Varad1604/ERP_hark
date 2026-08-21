# New Evidence to Reopened Receivable

# 1. Scenario Metadata

**Scenario ID:** L3-F-043  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F02_Payment_Management_Architecture.md`; `L2-F03_Refund_Management.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs reactivation of a previously resolved receivable when new authoritative Finance evidence shows that an outstanding position exists again or that the prior resolution is no longer financially valid. It preserves the original settlement, write-off, closure, collection, adjustment, Accounting, and event history while reopening the same Accounts Receivable-owned aggregate through a traceable, validated business fact.

---

# 3. Business Outcome

The previously resolved receivable has been validly reopened from authoritative new evidence, its current outstanding balance and collection information are restored without erasing prior history, and `ReceivableReopened` provides an auditable completed fact for governed downstream use.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Accounts Receivable receives new authoritative Finance evidence affecting a receivable previously treated as fully settled, written off, closed, or otherwise resolved.

## 4.2 End Boundary

The scenario ends when the evidence has been validated and incorporated exactly once, the same receivable aggregate has a valid active outstanding and collection position, all prior resolution history remains immutable, and `ReceivableReopened` has been published.

## 4.3 In Scope

- Identification of the previously resolved receivable and its originating obligation.
- Validation of new Payment, Refund, Financial Adjustment, obligation, Accounting, or other authoritative Finance evidence.
- Determination that the evidence creates or restores a receivable outstanding position.
- Preservation of original settlement, write-off, closure, and collection history.
- Update of receivable balance, settlement references, aging, collection profile, and status information.
- Cross-domain traceability from the new evidence to the reopened receivable.
- Publication of the completed reopening fact.

## 4.4 Out of Scope

- Rewriting, deleting, or replacing the original receivable or its historical events.
- Creation of a second receivable for the same originating obligation solely because prior resolution changed.
- Payment reversal, Refund, Adjustment cancellation/reversal, obligation correction, or Accounting correction execution within Accounts Receivable.
- Informal reopening based on unsupported user assertion or non-authoritative evidence.
- Reopening of an Accounting Period, Fiscal Year, Journal, Ledger, or another domain's aggregate.
- Technical replay, message handling, persistence, or recovery mechanisms.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | `L3-F-041` Receipt or Adjustment to Receivable Settlement | Establishes a partial or full settlement that later evidence may invalidate or reduce. |
| Related | `L3-F-042` Uncollectible Receivable to Write-Off Decision | Establishes a write-off outcome that later authoritative evidence may affect. |
| Followed by | `L3-F-040` Overdue Receivable to Governed Dunning Outcome | A reopened outstanding receivable may re-enter governed collection activity. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounts Receivable / Section F | Owns the existing `Receivable`, reopening validation, resulting outstanding balance representation, settlement and evidence references, aging and collection visibility, and final reopened outcome. |
| Payment Management / Section F | Owns any Payment allocation, Settlement, reversal, cancellation, or other Payment fact supplied as new evidence. |
| Refund Management / Section F | Owns completed or cancelled Refund facts that may affect the receivable's current financial position. |
| Financial Adjustment Management / Section F | Owns Adjustment approval, publication, application, cancellation, and other Adjustment evidence affecting the prior resolution. |
| Accounting & General Ledger / Section F | Owns Accounting corrections, reversals, Journals, and Ledger consequences without determining the operational Receivable state. |
| Originating obligation owner / Section F | Retains ownership of the obligation and any authoritative revision, cancellation, restoration, or correction fact. |

Reopening never transfers source ownership. Accounts Receivable consumes authoritative evidence and modifies only its own aggregate after invariant validation.

---

# 6. Business Actors, Roles and Responsibilities

The locked L2 sources define no canonical business actors for requesting, validating, approving, rejecting, or confirming Receivable reopening. `L2-F07` states only that reactivation occurs through an authoritative Finance event. Responsibilities therefore remain assigned to canonical domains, with missing actor and authority definitions recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- One existing `Receivable` aggregate remains traceable to exactly one originating financial obligation.
- The receivable is in a previously resolved position such as fully settled, written off, closed, or another source-permitted terminal or resolved condition.
- Its complete balance, settlement, write-off, closure, collection, adjustment, event, and audit history is available and immutable.
- A new completed business fact has been published by the canonical owner of the affected Finance aggregate.
- The evidence identifies the prior receivable, originating obligation, source aggregate, financial effect, reason, and traceable reference.
- The evidence has not already been incorporated into the receivable.
- Reopening does not require direct modification of another bounded context's aggregate.

---

# 8. Trigger

Accounts Receivable consumes a new authoritative Finance event whose valid financial effect means that a previously resolved receivable again has an outstanding amount or otherwise requires reactivation.

`L2-F07` explicitly cites `PaymentReversed` and also consumes Refund and Financial Adjustment facts, but the complete canonical trigger set and exact reopening guards are not defined upstream.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Publish a completed authoritative Finance fact that changes or invalidates a prior receivable resolution. | Canonical source-owning Finance domain | New immutable reopening evidence becomes available. |
| 2 | Identify the existing resolved `Receivable`, originating obligation, prior resolution, and source facts affected by the new evidence. | Accounts Receivable | The correct historical aggregate and financial context are established. |
| 3 | Validate the evidence's publisher, source aggregate, lifecycle completion, amount, reason, currency and institutional context, uniqueness, and relationship to the prior resolution. | Accounts Receivable | The evidence is confirmed as authoritative and applicable. |
| 4 | Determine whether incorporating the evidence creates or restores an outstanding receivable amount and whether reopening is permitted. | Accounts Receivable | A governed reopen or no-reopen decision is reached. |
| 5 | Record the new immutable source reference while preserving every prior settlement, write-off, closure, collection, and Accounting reference unchanged. | Accounts Receivable | Complete before-and-after financial lineage is retained. |
| 6 | Recalculate the receivable's current outstanding representation from the authoritative source effect and re-establish applicable aging, collection, classification, and status information. | Accounts Receivable | The same aggregate reflects the current active financial position. |
| 7 | Validate aggregate identity, balance, source traceability, consistency, and all applicable invariants. | Accounts Receivable | The reopening is financially valid and auditable. |
| 8 | Publish `ReceivableReopened` and make the updated outstanding and collection visibility available to governed consumers. | Accounts Receivable | Reopening completes without ownership transfer or historical erasure. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Payment settlement reversed | An authoritative Payment fact reverses or invalidates an allocation or Settlement that previously satisfied the receivable. | Record the Payment evidence, restore only the affected outstanding amount, and preserve the original settlement and reversal lineage. | The receivable is reopened to the valid remaining amount. | Payment Management and Accounts Receivable |
| Refund changes the resolved position | A completed Refund creates a receivable consequence under the authoritative financial rules. | Validate the Refund relationship and effect, record its reference, and reopen only when the resulting receivable amount is established. | The receivable is reopened or remains resolved according to the authoritative effect. | Refund Management and Accounts Receivable |
| Write-off or Adjustment later changes | An Adjustment is cancelled, reversed, corrected, or superseded by another authoritative Adjustment fact. | Preserve the original Write-Off/Adjustment history and incorporate the later source-owned fact once. | The receivable reopens to the amount supported by the new Adjustment evidence. | Financial Adjustment Management and Accounts Receivable |
| Originating obligation revised or restored | The obligation owner publishes a valid fact increasing or restoring the obligation after receivable resolution. | Validate the relationship to the same obligation and existing receivable, then update and reopen the same aggregate. | The receivable is reopened without duplicate registration. | Originating obligation owner and Accounts Receivable |
| Evidence does not restore an outstanding amount | The valid new fact changes history but leaves the receivable fully resolved. | Record the evidence where required without publishing `ReceivableReopened`. | The receivable remains in its prior resolved position. | Accounts Receivable |
| Evidence requires review | Amount, ownership, timing, reference, or relationship to the prior resolution is uncertain. | Preserve the evidence and maintain the last valid receivable position until the canonical source owner resolves the uncertainty. | No premature reopening occurs. | Accounts Receivable and affected source owner |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| New evidence is not published by an authoritative Finance owner. | The receivable cannot be reopened without bypassing ownership and integrity controls. | Reject the reopening basis and require a canonical completed source fact. | Accounts Receivable and claimed source owner |
| The evidence cannot identify the existing receivable or originating obligation. | Reopening could affect the wrong aggregate or create duplicate receivables. | Withhold incorporation and resolve the missing immutable relationship with the source owner. | Accounts Receivable and source owner |
| The same evidence was already incorporated. | Duplicate application would overstate the outstanding balance. | Preserve the prior application and reject duplicate financial effect while retaining the repeated fact for investigation. | Accounts Receivable |
| Amount, currency, institution, or prior-resolution references conflict. | The reopened financial position would not be reconstructable or reliable. | Place the evidence under review and reconcile authoritative facts before any state or balance change. | Accounts Receivable and affected source owners |
| Historical settlement, write-off, closure, or Accounting evidence is missing. | The institution cannot prove why the receivable was resolved or why it should reopen. | Retain the last valid position and reconstruct or obtain the missing evidence under canonical ownership before reopening. | Accounts Receivable and affected owner |
| Reopening would require a negative, duplicated, or otherwise invalid balance. | Aggregate and monetary integrity would fail. | Reject the proposed effect and return the conflict to the source owner for correction. | Accounts Receivable and source owner |
| Aggregate validation fails after evidence receipt. | No valid reopened outcome exists. | Do not publish `ReceivableReopened`; preserve the previous resolved position and all received evidence for governed correction. | Accounts Receivable |

`L2-F07` defines `ReceivableReopened` but no reopening-requested, under-review, rejected, or failed state/event. No local exception lifecycle is invented.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A previously resolved receivable may be reactivated only through an authoritative Finance event. | Accounts Receivable and source Finance owner | Unsupported assertions, direct edits, and non-authoritative evidence cannot reopen the aggregate. | `L2-F07`, Section 11.3 |
| Accounts Receivable never creates obligations and every receivable references exactly one originating financial obligation. | Accounts Receivable and obligation owner | Reopening updates the same representation and preserves the original obligation relationship. | `L2-F07`, Sections 2.2 and 6.7; `INV-AR-03` |
| Receivable Aggregates cannot exist without complete financial traceability. | Accounts Receivable | New evidence, prior resolution, restored amount, and resulting position must remain reconstructable. | `L2-F07`, `INV-AR-04` |
| Receivable balance is authoritative within the aggregate and its changes are governed by upstream financial facts. | Accounts Receivable and source Finance owners | Reopening amount is derived only from the authoritative source effect. | `L2-F07`, Section 10.4 and `INV-AR-02` |
| Settlement references shall never invalidate aggregate consistency. | Accounts Receivable | Original settlement and later reversal/correction references remain together and consistent. | `L2-F07`, `INV-AR-05` |
| External bounded contexts never directly modify Receivable entities. | Accounts Receivable | Source owners publish facts; Accounts Receivable alone incorporates them. | `L2-F07`, `INV-AR-08` |
| Published events represent completed valid aggregate transitions and preserve ownership. | Publishing domain | `ReceivableReopened` follows successful validation and never represents an incomplete review. | `L2-F07`, Sections 11.1-11.4 and `INV-AR-06` |
| Financial Adjustment milestones and history remain immutable, and publication never transfers Adjustment ownership. | Financial Adjustment Management | A later Adjustment fact supplements rather than overwrites the original write-off or adjustment history. | `L2-F04`, Sections 14-17 |
| Posted Accounting history is corrected through governed reversal or compensating evidence rather than historical modification. | Accounting & General Ledger | Receivable reopening cannot rewrite Journals or Ledger history. | `L2-F05` Accounting immutability and reversal constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| `Receivable` | Fully settled, `Written Off`, `Closed`, or another source-supported resolved position | Incorporates authoritative new evidence, restores a valid outstanding position, and preserves all prior lifecycle history. | `Reopened` is implied by `ReceivableReopened`; subsequent active collection/aging position is not canonically defined | Accounts Receivable | `L2-F07`, Sections 6.5, 10.6-10.8, 11.3 and States appendix |
| Source Payment, Refund, Adjustment, or obligation aggregate | Source-defined completed, reversed, cancelled, corrected, or revised position | Supplies immutable evidence but is not modified by Accounts Receivable. | Unchanged by this scenario | Canonical source Finance owner | Applicable L2-F02, L2-F03, L2-F04, or obligation-owner lifecycle |
| Accounting records | Existing posted or reversed history | Retain prior resolution evidence and incorporate any separately governed Accounting correction references. | Source-defined immutable Journal and Ledger lifecycle positions | Accounting & General Ledger | `L2-F05` |

The exact allowed transitions into `Reopened`, the status after reopening, and whether `Closed` is reversible are not defined by a canonical Receivable state machine and remain open in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PaymentReversed` | Payment Management | Accounts Receivable | Expected by Accounts Receivable as an authoritative fact that may invalidate prior settlement. | `L2-F07`, Section 11.5; exact owning-source definition is unresolved |
| `RefundCompleted` | Refund Management | Accounts Receivable | Supplies a completed Refund fact that may alter the resolved receivable position. | `L2-F03`, Section 17; `L2-F07`, Section 11.5 |
| `RefundCancelled` | Refund Management | Accounts Receivable, where contractually applicable | Supplies a cancelled Refund fact whose receivable consequence requires evaluation. | `L2-F03`, Section 17 |
| `AdjustmentCancelled`, `AdjustmentApplicationCompleted`, or `AdjustmentFinancialPositionUpdated` | Financial Adjustment Management | Accounts Receivable | Supplies later authoritative Adjustment evidence affecting the prior resolution. | `L2-F04`, Sections 17.4-17.5 |
| Authoritative obligation revision/restoration event | Originating obligation owner | Accounts Receivable | Communicates a valid change to the obligation represented by the existing receivable. | `L2-F07`, Sections 6.7 and 11.5; exact event name not defined |
| `ReceivableReopened` | Accounts Receivable | Accounting, Financial Reporting, Treasury, dashboards, Audit, Enterprise Analytics, and other governed consumers | Announces completed reactivation of a previously resolved receivable. | `L2-F07`, Sections 11.3-11.4 and 12.4 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Payment, Refund, Financial Adjustment, or originating obligation owner | Accounts Receivable | Immutable completed source fact with aggregate identity, prior reference, amount/effect, reason, lifecycle milestone, and audit lineage | Provides the only permitted basis for evaluating reopening. | `L2-F07`, Sections 5.1, 11.5 and 12.3 |
| Accounts Receivable | Accounting & General Ledger | Reopened receivable fact with prior resolution and new source references | Supports separately governed Accounting consequences without transferring Receivable ownership. | `L2-F07`, Sections 9 and 12 |
| Accounts Receivable | Governed downstream consumers | `ReceivableReopened`, restored outstanding position, aging/collection visibility, and traceable prior resolution | Enables reporting, treasury, collection, analytics, dashboards, and audit to use the current position. | `L2-F07`, Sections 5.2 and 12.4 |

The exact evidence contract, event correlation, effective ordering, and source-specific reopening trigger names remain open in Section 17.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Publish new Finance evidence | No canonical actor defined in the applicable L2 sources | Only the canonical aggregate owner may complete and publish its source fact. | Payment, Refund, Financial Adjustment, Accounting, or obligation owner |
| Determine evidence applicability | No canonical actor defined in `L2-F07` | Decision must preserve source ownership, uniqueness, monetary integrity, prior history, and Receivable invariants. | Accounts Receivable |
| Approve or confirm reopening | No canonical role or separate approval requirement defined | Reopening requires an authoritative Finance event and completed aggregate validation; whether human approval is additionally required remains open. | Accounts Receivable and institutional governance |
| Publish `ReceivableReopened` | No canonical actor defined | Permitted only after a positive outstanding position and valid reopened lifecycle result are established. | Accounts Receivable |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Source correction versus Receivable reopening | Source Finance owner and Accounts Receivable | Source owner changes only its aggregate and publishes evidence; Accounts Receivable alone changes the receivable. |
| Historical preservation | Prior settlement/write-off/closure owners and Accounts Receivable | New evidence supplements the immutable history and never overwrites the original resolution. |
| Reopening versus new registration | Accounts Receivable | The existing aggregate is reopened when it represents the same originating obligation; no duplicate receivable is created. |
| Reopening versus Accounting correction | Accounts Receivable and Accounting & General Ledger | Operational reopening and Accounting correction remain separate governed outcomes. |
| Conflicting or uncertain evidence | Accounts Receivable and affected source owners | No local override may select among unresolved authoritative facts or force an invalid reopening. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove prior resolution | Original obligation, receivable, settlement/write-off/closure facts, collection outcome, approval, Accounting, and prior events | Each canonical owner |
| Prove new evidence | Source aggregate identity, completed event, financial effect, reason, prior-reference relationship, and immutable audit history | Publishing Finance owner |
| Prove reopening decision | Applicability review, uniqueness check, prior and resulting balance, transition basis, and exception evidence | Accounts Receivable |
| Prove restored position | Updated receivable balance, aging, collection, settlement/evidence references, and invariant validation | Accounts Receivable |
| Prove completion | `ReceivableReopened` and cross-domain lineage from prior resolution through new evidence to the active receivable | Accounts Receivable and source owner |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- One existing previously resolved receivable and its originating obligation are identified.
- New evidence is an immutable completed fact published by the canonical Finance owner.
- The evidence is applicable, unique, financially consistent, and traceable to the prior resolution.
- Incorporating the evidence establishes a valid outstanding receivable amount or another source-permitted reopening basis.
- Original settlement, write-off, closure, collection, Adjustment, Accounting, event, and audit history remains unchanged and available.
- The receivable balance, aging, collection information, source references, and lifecycle position are mutually consistent.
- Aggregate validation completes before `ReceivableReopened` is published.

## 16.2 Postconditions

- Accounts Receivable retains ownership of the same reopened `Receivable` aggregate and immutable identity.
- The originating obligation remains owned by its canonical source domain.
- Each new evidence source retains ownership of its Payment, Refund, Adjustment, Accounting, or obligation fact.
- The receivable is available for governed collection, settlement, adjustment, write-off, closure, and reporting based on its restored position.
- Downstream consumers can distinguish the prior resolution from the later reopening and reconstruct the full chronology.
- No historical financial evidence has been deleted, replaced, or silently edited.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F07` defines `ReceivableReopened` but no canonical Receivable state machine, permitted origin states, transition guards, target active state, or relationship to `ReceivableClosed`. | Sections 9 and 13 cannot lock which resolved positions may reopen or what state follows reopening. | Accounts Receivable owner and Lead Architect must define the canonical reopening lifecycle. |
| `L2-F07` consumes `PaymentReversed`, but `L2-F02` does not list this exact canonical Payment event in its event architecture. | Section 8 and Section 14 cannot establish an aligned Payment reversal trigger contract. | Payment Management and Accounts Receivable owners must align the canonical event and reversal semantics. |
| Financial Adjustment architecture defines cancellation but no canonical adjustment-reversal aggregate, lifecycle, or event for an already applied Write-Off or other Adjustment. | The scenario cannot determine how an applied Adjustment is authoritatively undone before reopening. | Financial Adjustment Management owner must define post-application correction/reversal governance. |
| Refund architecture defines `RefundCompleted` and `RefundCancelled`, but the exact circumstances and financial effect under which either reopens a receivable are not defined. | The Draft cannot lock a source-specific Refund reopening rule. | Refund Management and Accounts Receivable owners must define the cross-domain effect contract. |
| No exact obligation revision/restoration event set or cross-source evidence contract is defined for every obligation-owning Finance context. | Section 14 cannot enumerate the complete canonical trigger set. | Obligation-owning Finance domains and Accounts Receivable must define event names and required evidence. |
| No actor, reviewer, approver, exception authority, or separation-of-duty rule is defined for reopening. | Sections 6 and 15 cannot determine whether authoritative event consumption alone is sufficient or human approval is mandatory. | Accounts Receivable, Enterprise Governance, and Lead Architect must define reopening authority. |
| Ordering, effective-date, currency conversion, tolerance, and conflict rules are missing when multiple new facts affect the same resolved receivable. | Section 9 cannot deterministically resolve simultaneous or contradictory reopening evidence. | Finance foundation and participating domain owners must define the calculation and precedence policy. |
| The sources do not define whether reopening a written-off receivable restores a collectible balance, a legal claim, an Accounting asset, or all three, nor the required Accounting sequence. | The business and financial meaning of the reopened position remains incomplete. | Accounts Receivable, Financial Adjustment, Accounting, Legal/Compliance, and Finance policy owners must define the effect upstream. |
| No reopening-requested, under-review, rejected, failed, or no-reopen event exists. | Exceptional and negative outcomes cannot be communicated using canonical Receivable facts. | Accounts Receivable owner must define required events or approve evidence-only handling. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-043**
