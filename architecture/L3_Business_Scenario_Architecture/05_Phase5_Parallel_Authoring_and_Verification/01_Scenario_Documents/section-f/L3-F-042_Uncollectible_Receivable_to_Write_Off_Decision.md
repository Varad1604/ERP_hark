# Uncollectible Receivable to Write-Off Decision

# 1. Scenario Metadata

**Scenario ID:** L3-F-042  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs the institutional decision to approve or reject writing off a receivable that is considered uncollectible. It ensures that collection evidence, outstanding balance, obligation lineage, policy eligibility, approval participation, Financial Adjustment ownership, Accounting consequences, and the Accounts Receivable outcome remain controlled, reconstructable, and separately owned.

---

# 3. Business Outcome

A complete, authorized, and auditable write-off decision has been reached for the uncollectible receivable: either an approved Write-Off Financial Adjustment proceeds to authoritative application and the receivable is recorded as written off, or the request is rejected and the receivable remains in its prior valid outstanding collection position.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an outstanding receivable is presented for write-off consideration with evidence that collection is no longer reasonably expected or that another institutionally governed write-off condition applies.

## 4.2 End Boundary

The scenario ends when the write-off request has received a governed approval outcome and either: the approved Write-Off Financial Adjustment has become applicable to the financial position and Accounts Receivable has published `ReceivableWrittenOff`; or the rejected decision and its reasons are preserved while the receivable remains outstanding.

## 4.3 In Scope

- Identification of the receivable and outstanding amount proposed for write-off.
- Collection, aging, settlement, adjustment, obligation, dispute, and exception evidence supporting the request.
- Validation of financial traceability and applicable institutional policy.
- Creation and classification of the proposed modification as a Write-Off Financial Adjustment.
- Participation in externally governed approval and preservation of the approval outcome.
- Application of an approved write-off fact to the receivable.
- Recording of approved or rejected decision evidence.
- Publication of the completed receivable write-off fact after valid application.

## 4.4 Out of Scope

- Dunning execution, reminder activity, or legal-recovery procedure.
- Payment reception, allocation, refund, or settlement execution.
- Creation, cancellation, or silent deletion of the originating obligation.
- Physical deletion or rewriting of the receivable or its collection history.
- Independent approval by Accounts Receivable or Financial Adjustment Management where Enterprise Governance owns approval execution.
- Accounting Journal or Ledger posting design or execution.
- Later reopening when new evidence arises, which belongs to `L3-F-043`.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-040` Overdue Receivable to Governed Dunning Outcome | Provides the governed collection and escalation history supporting possible uncollectibility. |
| Related | `L3-F-041` Receipt or Adjustment to Receivable Settlement | Confirms completed settlements and adjustments used to establish the current outstanding amount. |
| Followed by | `L3-F-043` New Evidence to Reopened Receivable | Governs reactivation if later authoritative evidence invalidates or changes the resolved position. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounts Receivable / Section F | Owns the `Receivable`, its outstanding and collection information, write-off evidence reference, and final written-off receivable outcome. |
| Financial Adjustment Management / Section F | Exclusively owns the Write-Off Financial Adjustment, its classification, validation, lifecycle, publication, application milestone, and adjustment events. |
| Enterprise Governance | Owns approval policies, workflows, hierarchies, routing, rules, execution, and the immutable approval outcome consumed by Financial Adjustment Management. |
| Accounting & General Ledger / Section F | Owns the Accounting consequence of an approved applied write-off and preserves Journal and Ledger traceability. |
| Originating obligation owner / Section F | Retains ownership and history of the financial obligation referenced by the receivable. |

Approval does not transfer ownership. A write-off changes the institution's receivable financial position through a canonical Financial Adjustment; it does not erase the original obligation, receivable, or collection evidence.

---

# 6. Business Actors, Roles and Responsibilities

The locked L2 sources define no canonical actors for proposing, reviewing, approving, rejecting, applying, or auditing a receivable write-off. Financial Adjustment architecture explicitly places approval execution, matrices, routing, hierarchy, and rules outside its ownership. This Draft therefore assigns responsibilities to canonical domains and records the unresolved actor and authority definitions in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid Accounts Receivable-owned `Receivable` exists with a positive outstanding amount and complete originating-obligation traceability.
- All authoritative Payment, Refund, and Financial Adjustment facts already applicable to the receivable have been incorporated.
- The current aging and collection position and prior dunning, review, suspension, or escalation evidence are available.
- A documented uncollectibility or other governed write-off basis has been presented.
- The receivable is not already fully settled, written off, closed, or otherwise ineligible for an ordinary write-off decision.
- The proposed write-off amount does not exceed the current authoritative outstanding amount.
- Applicable institutional policy and enterprise approval participation are available.

---

# 8. Trigger

A governed write-off consideration request is raised for an outstanding receivable with evidence supporting uncollectibility or another institutionally permitted write-off basis.

The L2 sources do not define the canonical requesting actor, mandatory eligibility event, approval threshold, or required minimum collection history. These remain open in Section 17.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the receivable, originating obligation, current outstanding amount, aging position, collection status, and proposed write-off amount. | Accounts Receivable | A bounded write-off consideration case is established. |
| 2 | Assemble the collection history, prior settlement and adjustment references, dispute or exception evidence, and stated write-off reason. | Accounts Receivable | The proposed decision has complete receivable evidence and lineage. |
| 3 | Validate that the receivable remains outstanding, the proposed amount is valid, all relevant source facts are current, and the request satisfies the applicable institutional policy basis. | Accounts Receivable and policy owner | The request is eligible to enter Financial Adjustment governance or is returned as ineligible. |
| 4 | Establish one canonical Financial Adjustment classified as `Write-Off`, referencing the receivable, originating obligation, amount, reason, source evidence, and audit lineage. | Financial Adjustment Management | The proposed financial modification has an authoritative aggregate and identity. |
| 5 | Validate the Financial Adjustment and submit its approval context to Enterprise Governance. | Financial Adjustment Management | A complete write-off proposal is available for governed decision. |
| 6 | Execute the applicable approval process and return an immutable approved or rejected outcome with decision evidence. | Enterprise Governance | A governed write-off decision exists without transferring adjustment ownership. |
| 7 | If approved, publish and apply the Write-Off Financial Adjustment according to its canonical lifecycle; if rejected, retain the rejection and stop application. | Financial Adjustment Management | The approved financial modification becomes authoritative or the rejected proposal remains unapplied. |
| 8 | For an approved and applicable write-off, incorporate the authoritative adjustment fact into the `Receivable`, update the outstanding and collection position, and preserve all historical evidence. | Accounts Receivable | The receivable reflects the authorized write-off without deletion or loss of lineage. |
| 9 | Validate aggregate consistency and publish `ReceivableWrittenOff`; retain the approval, Adjustment, Accounting-readiness, and receivable evidence. | Accounts Receivable | The approved write-off outcome is complete, visible, and auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Write-off rejected | Enterprise Governance returns a negative approval outcome. | Preserve the request, evidence, decision, and reason; do not publish or apply an approved write-off effect to the receivable. | The receivable remains outstanding in its prior valid collection position. | Enterprise Governance, Financial Adjustment Management, and Accounts Receivable |
| Request returned for more evidence | Decision authority cannot determine eligibility from the submitted evidence. | Retain the case under its last valid position and obtain the missing collection, obligation, financial, or policy evidence before resubmission. | Accounts Receivable and Enterprise Governance |
| Partial write-off approved | Approval covers less than the full outstanding amount. | Apply only the approved amount, retain the remaining balance, and continue the applicable collection or settlement lifecycle for the remainder. | Financial Adjustment Management and Accounts Receivable |
| Full write-off approved | Approval covers the entire valid outstanding amount. | Apply the approved Adjustment, reduce collectible outstanding to the governed written-off position, complete collection activity as permitted, and publish `ReceivableWrittenOff`. | Financial Adjustment Management and Accounts Receivable |
| New financial fact before decision | Payment, refund, adjustment, reversal, or obligation change alters the current receivable position while approval is pending. | Recalculate the eligible write-off basis from authoritative facts and amend, withdraw, or resubmit the proposal under governance. | Accounts Receivable and relevant source owner |
| Proposal suspended or cancelled | Financial Adjustment lifecycle enters `Suspended` or `Cancelled`. | Preserve the state and evidence and do not apply any write-off consequence to the receivable. | Financial Adjustment Management and Enterprise Governance |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Collection, aging, source, or uncollectibility evidence is incomplete or contradictory. | The institution cannot justify the financial modification. | Withhold approval submission or return the case for governed evidence completion; preserve the receivable unchanged. | Accounts Receivable and policy owner |
| Proposed amount exceeds the current outstanding amount. | The write-off would distort the receivable and financial position. | Reject or correct the proposal before approval; never force a negative receivable balance. | Accounts Receivable and Financial Adjustment Management |
| Approval outcome is missing, ambiguous, expired, or not traceable. | Financial Adjustment cannot establish a valid approved milestone. | Do not publish or apply the write-off; obtain a valid immutable outcome from Enterprise Governance. | Enterprise Governance and Financial Adjustment Management |
| An unauthorized write-off or direct Receivable modification is attempted. | Approval, aggregate ownership, and financial integrity would be bypassed. | Reject the action and preserve the attempted exception in audit evidence. | Accounts Receivable and Financial Adjustment Management |
| Adjustment publication or application fails validation. | The approved decision has not yet become an authoritative financial-position change. | Retain the approved decision, contain the exception within Financial Adjustment governance, and do not mark the receivable written off. | Financial Adjustment Management |
| Accounting consequence is not ready or traceable. | The operational write-off and financial books may diverge. | Preserve the approved Adjustment and receivable evidence, withhold final completion where Accounting-readiness is mandatory, and resolve the Accounting participation gap under its canonical owner. | Financial Adjustment Management, Accounts Receivable, and Accounting & General Ledger |
| Duplicate write-off fact is received. | Reapplication would understate the receivable and duplicate the financial modification. | Retain the original Adjustment reference and reject duplicate financial effect. | Accounts Receivable |

No write-off-request rejected state or event is defined for the `Receivable`; rejection remains evidence attached to the unapplied decision process rather than an invented Receivable state.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial Adjustment is the only canonical aggregate for an approved modification to an institutional financial position, and Write-Off is a supported Adjustment category. | Financial Adjustment Management | A write-off cannot be represented as an informal Receivable edit or a separate adjustment entity. | `L2-F04`, Sections 2, 4 and 9.4 |
| External domains may reference but never create or modify a Financial Adjustment aggregate. | Financial Adjustment Management | Accounts Receivable supplies context and consumes the result but does not own the write-off Adjustment. | `L2-F04`, Section 4.2 |
| Financial Adjustments participate in enterprise approval; approval policies, workflow, matrix, routing, hierarchy, rules, and execution remain externally governed. | Enterprise Governance | A write-off requires an externally governed immutable approval outcome. | `L2-F04`, Section 11 |
| Only validated and approved Financial Adjustments may be published; the financial modification becomes part of the institutional position at `Applied`. | Financial Adjustment Management | The receivable cannot be written off merely because a request was raised or approved if the authoritative application milestone is still incomplete. | `L2-F04`, Sections 14-16 |
| Adjustment lifecycle progresses forward, completed milestones are immutable, and only one operational state is active. | Financial Adjustment Management | Approval, rejection, suspension, cancellation, publication, application, and completion evidence cannot be overwritten. | `L2-F04`, Sections 14, 15 and 24 |
| Receivable balance remains authoritative within its aggregate and reflects upstream financial events. | Accounts Receivable and source owners | The write-off effect is incorporated only from the canonical Adjustment fact. | `L2-F07`, Section 10.4 and `INV-AR-02` |
| Every receivable retains one originating obligation reference and complete financial traceability. | Accounts Receivable | Write-off never deletes or detaches the source obligation and history. | `L2-F07`, `INV-AR-03` and `INV-AR-04` |
| `ReceivableWrittenOff` represents a completed Receivable business fact and is published only after aggregate validation. | Accounts Receivable | The event cannot be emitted for a proposed, rejected, or unapplied write-off. | `L2-F07`, Sections 11.3-11.4 |
| Waivers, write-offs, concessions, penalties, and operational adjustments are owned by Financial Adjustment Management, while Accounting owns resulting Journals and Ledger effects. | Financial Adjustment Management and Accounting & General Ledger | Write-off decision and Accounting consequence remain separately owned and traceable. | `L2-F05`, Section 5.8 boundary statement |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| `Receivable` | Outstanding with active, reviewed, suspended, or escalated collection visibility | An approved applied Write-Off Adjustment is incorporated while original balance, collection, and obligation history are preserved. | `Written Off` is implied by `ReceivableWrittenOff`; rejected decisions leave the prior valid position unchanged | Accounts Receivable | `L2-F07`, Sections 6.5, 10.7, 11.3 and States appendix |
| Financial Adjustment | `Draft` or `Pending Validation` | Progresses through validation and external approval participation; approved cases proceed through publication and application. | Approved path: `Applied` then `Completed`; rejected path is not canonically defined; `Suspended` or `Cancelled` may apply where governed | Financial Adjustment Management | `L2-F04`, Sections 14-15 |
| Originating Financial Obligation | Existing source-owned position | Remains historically referenced and is not directly deleted or modified by Accounts Receivable. | Unchanged unless the source owner defines a corresponding authoritative effect | Originating obligation owner | `L2-F07`, Sections 2.2 and 6.7 |
| Accounting consequence | Not yet recorded for the proposed write-off | Approved applied Adjustment becomes available for Accounting participation. | Source-defined Journal and Ledger lifecycle positions | Accounting & General Ledger | `L2-F04`, Sections 16-17; `L2-F05` |

The exact Receivable Written-Off transition, Financial Adjustment rejection state, obligation consequence, and Accounting completion dependency are not fully defined upstream and remain open in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AdjustmentValidated` | Financial Adjustment Management | Enterprise Governance and governed consumers | Confirms the proposed Write-Off Adjustment is ready for approval participation. | `L2-F04`, Section 17.4 |
| `AdjustmentApproved` | Financial Adjustment Management | Accounts Receivable, Accounting, Reporting, and other governed consumers where applicable | Records the positive approval milestone within the Adjustment lifecycle. | `L2-F04`, Section 17.4 |
| `AdjustmentPublicationCompleted` | Financial Adjustment Management | Accounts Receivable, Accounting, Reporting, and Analytics | Makes the approved Write-Off Adjustment available for enterprise consumption. | `L2-F04`, Sections 16 and 17.4 |
| `AdjustmentApplicationCompleted` or `AdjustmentFinancialPositionUpdated` | Financial Adjustment Management | Accounts Receivable and Accounting & General Ledger | Communicates that the approved write-off has become part of the institutional financial position. | `L2-F04`, Sections 14.4 and 17.4-17.5 |
| `AdjustmentCancelled` or `AdjustmentSuspended` | Financial Adjustment Management | Accounts Receivable and governed decision participants | Communicates a non-applicable exceptional Adjustment outcome where used. | `L2-F04`, Section 17.4 |
| `ReceivableWrittenOff` | Accounts Receivable | Accounting, Financial Reporting, Treasury, dashboards, Audit, Enterprise Analytics, and other governed consumers | Announces the completed written-off receivable outcome after authoritative application and aggregate validation. | `L2-F07`, Sections 11.3-11.4 and 12.4 |

No canonical write-off-requested, write-off-rejected, or approval-outcome event is defined across the applicable sources. This remains recorded in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Accounts Receivable | Financial Adjustment Management | Write-off context containing receivable identity, obligation reference, current outstanding, proposed amount, reason, collection history, and audit evidence | Establishes the source context for one canonical Write-Off Financial Adjustment. | `L2-F04`, Sections 8-13; `L2-F07`, Sections 10.4, 10.7 and 10.9 |
| Financial Adjustment Management | Enterprise Governance | Validated Adjustment approval context | Enables external approval without transferring Adjustment ownership. | `L2-F04`, Section 11 |
| Enterprise Governance | Financial Adjustment Management | Immutable approved or rejected outcome with traceability | Controls whether the Adjustment may progress toward publication and application. | `L2-F04`, Section 11 |
| Financial Adjustment Management | Accounts Receivable and Accounting & General Ledger | Approved, published, and applied Write-Off Adjustment fact | Enables receivable and Accounting consequences under separate ownership. | `L2-F04`, Sections 14-17 |
| Accounts Receivable | Governed downstream consumers | `ReceivableWrittenOff` with preserved source and Adjustment references | Provides final receivable visibility for reporting, treasury, analytics, dashboards, and audit. | `L2-F07`, Sections 11-12 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Propose a receivable for write-off | No canonical actor defined | Proposal requires a valid outstanding receivable and documented eligibility evidence; proposal alone has no financial effect. | Accounts Receivable and institutional policy owner |
| Validate and classify the Write-Off Adjustment | No canonical actor defined | Must preserve source, reason, amount, reference, aggregate consistency, and Adjustment lifecycle rules. | Financial Adjustment Management |
| Approve or reject write-off | No canonical actor, matrix, threshold, or hierarchy defined | Decision execution belongs to Enterprise Governance and must return an immutable auditable outcome. | Enterprise Governance |
| Publish and apply approved Adjustment | No canonical actor defined | Only a validated and positively approved Adjustment may progress; exact application authority remains open. | Financial Adjustment Management |
| Record `ReceivableWrittenOff` | No canonical actor defined | Permitted only after the authoritative write-off effect is applicable and Receivable invariants remain satisfied. | Accounts Receivable |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Proposal versus approval | Accounts Receivable context provider and Enterprise Governance decision authority | The proposing responsibility cannot self-authorize write-off unless upstream governance explicitly permits it. |
| Approval versus Adjustment ownership | Enterprise Governance and Financial Adjustment Management | Governance owns the approval process; Financial Adjustment retains aggregate ownership and lifecycle execution. |
| Adjustment versus Receivable consequence | Financial Adjustment Management and Accounts Receivable | The Adjustment owner publishes the applicable fact; Accounts Receivable alone updates the receivable representation. |
| Operational versus Accounting consequence | Financial Adjustment Management, Accounts Receivable, and Accounting & General Ledger | Financial write-off and Accounting posting remain separately owned but completely traceable. |
| Rejection, suspension, cancellation, or exception | Decision authority and canonical aggregate owners | No local override may create a written-off receivable without a valid approved and applicable Adjustment. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove write-off eligibility | Receivable, obligation, outstanding balance, aging, collection history, settlement and adjustment history, uncollectibility reason, and policy basis | Accounts Receivable and institutional policy owner |
| Prove Adjustment integrity | Adjustment identity, category `Write-Off`, amount, source, reason, references, validation, state, lifecycle, and immutable history | Financial Adjustment Management |
| Prove approval outcome | Approval context, governing rule, decision, reason, authority, date or business boundary, and traceability | Enterprise Governance |
| Prove application | Publication and application milestones, financial-position fact, receivable previous and resulting position, and Accounting-readiness reference | Financial Adjustment Management and Accounts Receivable |
| Prove historical continuity | Original obligation, receivable, collection activity, proposal, decision, Adjustment, Accounting, and events retained without deletion or overwrite | Each canonical owner |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The correct receivable, originating obligation, current outstanding amount, and proposed write-off amount are identified and traceable.
- Required collection, aging, settlement, adjustment, exception, and policy evidence has been assembled.
- One canonical Write-Off Financial Adjustment represents the proposed modification.
- Adjustment validation and external approval participation have completed with an immutable decision.
- A rejected decision has no financial or receivable write-off effect.
- An approved decision progresses through the authoritative publication and application milestone before the receivable is marked written off.
- Accounts Receivable validates the resulting aggregate and publishes `ReceivableWrittenOff` only for the completed approved path.

## 16.2 Postconditions

- For approval, the receivable reflects only the authorized write-off amount and preserves complete obligation, collection, Adjustment, approval, and Accounting lineage.
- For rejection, the receivable remains in its previous valid outstanding collection position and the decision evidence remains auditable.
- Financial Adjustment Management retains ownership of the Write-Off Adjustment.
- Enterprise Governance retains ownership of approval policy and decision execution.
- Accounts Receivable retains ownership of the `Receivable` and final written-off visibility.
- The original obligation and historical receivable evidence are not physically deleted or silently rewritten.
- Later recovery, reversal, or new evidence is handled through separately governed scenarios.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| No canonical actors, approval matrix, routing, hierarchy, threshold, monetary limit, separation-of-duty rule, or delegated authority are defined for write-off. | Sections 6, 9, and 15 cannot lock who proposes, reviews, approves, rejects, applies, or confirms the decision. | Enterprise Governance, Finance owners, and Lead Architect must define canonical write-off roles and approval authority. |
| No authoritative eligibility policy defines uncollectibility, minimum aging, required dunning stages, legal recovery prerequisites, evidence sufficiency, limitation period, or permitted exception categories. | The Draft cannot determine when a receivable qualifies for write-off. | Institutional policy, Legal/Compliance, Accounts Receivable, and Enterprise Governance owners must define the write-off policy upstream. |
| `L2-F04` defines Write-Off as an illustrative Adjustment category but no write-off-specific validation, reason taxonomy, amount rule, effective-date rule, or approval outcome contract. | The scenario can use the canonical Adjustment but cannot lock its write-off-specific business rules. | Financial Adjustment Management owner must define the Write-Off Adjustment specialization upstream. |
| Financial Adjustment state architecture defines `Approved`, `Published`, `Pending Application`, `Applied`, and `Completed`, but has no `Rejected` state or rejection event. | A negative approval outcome cannot be mapped to a canonical Adjustment lifecycle position or event. | Financial Adjustment Management and Enterprise Governance must define rejection handling and event publication. |
| `L2-F07` defines `ReceivableWrittenOff` but no canonical Receivable state machine, write-off transition guard, partial-write-off state, or relationship to `Collection Completed` and `ReceivableClosed`. | Section 13 cannot lock the receivable lifecycle after partial or full write-off. | Accounts Receivable owner and Lead Architect must define the lifecycle and closure semantics. |
| The sources do not define whether `AdjustmentApproved`, `AdjustmentPublicationCompleted`, `AdjustmentApplicationCompleted`, or `AdjustmentFinancialPositionUpdated` is the exact authoritative trigger for receivable write-off. | Section 8 and Section 14 cannot lock one cross-domain trigger without choosing among source milestones. | Financial Adjustment Management and Accounts Receivable owners must define the canonical application contract. |
| Accounting-readiness and completion dependency are not explicit for `ReceivableWrittenOff`. | The scenario cannot determine whether the Receivable event precedes, follows, or is independent of Journal posting. | Financial Adjustment, Accounts Receivable, and Accounting owners must define event and completion sequencing. |
| The business meaning of write-off relative to the legal obligation, future recovery rights, payer statements, tax treatment, statutory reporting, and later collection is not defined. | Sections 3, 4, 13, and 16 cannot determine whether the underlying obligation is extinguished or only removed from active receivable recognition. | Finance, Legal/Compliance, Tax, Accounting, and Accounts Receivable owners must define the legal and financial effect upstream. |
| No canonical event represents write-off proposal, rejection, resubmission, partial write-off, or failed application. | Cross-domain visibility and audit handling for non-approved and partial outcomes remain incomplete. | Financial Adjustment and Accounts Receivable owners must define required events or approve reliance on generic lifecycle facts. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-042**
