# Payable Change to Approved Adjustment or Reversal

# 1. Scenario Metadata

**Scenario ID:** L3-F-047  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Payable  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F14_Procurement_Finance_Consumption_Workflows.md`; relevant locked Section J source architecture; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs a requested change to an existing Payable when new authoritative evidence requires an approved financial adjustment or compensating reversal. It protects the immutable Payable identity and history, uses the canonical `Financial Adjustment` aggregate for monetary modification, preserves source and approval ownership, and updates Accounts Payable only from a completed applicable adjustment fact.

---

# 3. Business Outcome

A Payable change has received a governed approved adjustment or compensating reversal outcome with immutable source, category, reason, approval, publication, application, Accounting, and audit lineage; Accounts Payable has incorporated the applicable financial effect without rewriting the original obligation, liability, settlement, or historical evidence.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid Payable change condition is established by authoritative evidence such as a credit note, debit note, liability correction, amended source obligation, duplicate or erroneous amount, approved commercial change, settlement correction, or required reversal of a prior financial effect.

## 4.2 End Boundary

The scenario ends when the proposed change has received a governed approval outcome and either: the approved Financial Adjustment/compensating reversal is published and applied, then incorporated into the Payable; or the request is rejected/cancelled and the Payable remains in its previous valid position with complete decision evidence.

## 4.3 In Scope

- Identification of the Payable, originating obligation, current outstanding amount, due schedule, readiness, settlement history, and proposed change.
- Validation of authoritative source evidence and change eligibility.
- Selection of the canonical Financial Adjustment category and reason.
- Creation and validation of one `Financial Adjustment` aggregate for the requested monetary change.
- External approval participation and preservation of the decision outcome.
- Publication and application of an approved adjustment.
- Use of a separately traceable compensating adjustment where reversal is required.
- Incorporation of the completed financial effect into Outstanding Liability, Due Schedule, Payment Readiness, ageing, and Payable lifecycle as applicable.
- Publication of supported Financial Adjustment and Accounts Payable events.

## 4.4 Out of Scope

- Destructive editing or deletion of the original Payable, source obligation, invoice, procurement, settlement, Financial Adjustment, Journal, or Ledger history.
- Accounts Payable creation or modification of the canonical Financial Adjustment.
- Financial Adjustment modification of the Payable aggregate.
- Source invoice, credit note, debit note, Purchase Order, contract, receipt, or supplier lifecycle execution.
- Treasury settlement execution, return, cancellation, or banking resolution.
- Accounting Journal reversal or posting execution within this scenario.
- Payable dispute/hold investigation, write-off decision, reopening, reconciliation, or archival closure.
- Lower-level document-processing, exchange, persistence, or financial-calculation design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | `L3-F-044` Vendor Invoice to Approved Payable Liability | Establishes the original Payable and source liability evidence. |
| Related | `L3-F-045` Approved Payable to Financial Settlement | Settlement history and in-flight payment status affect adjustment eligibility and amount. |
| Related | `L3-F-046` Payable Dispute or Hold to Governed Resolution | A resolved dispute may require this adjustment/reversal path. |
| Related | `L3-F-048` Unsettled Payable to Suspension Reopening or Write-Off | Exceptional lifecycle treatment may follow or constrain the adjustment outcome. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Adjustment Management / Section F | Exclusively owns the Financial Adjustment, source, category, reason, validation, approval participation, references, lifecycle, state, publication, application milestone, and final approved adjustment/compensating-reversal outcome. |
| Accounts Payable / Section F | Owns the Payable, Outstanding Liability, Due Schedule, Payment Readiness, ageing, settlement references, lifecycle, version, and incorporation of the completed Adjustment fact. |
| Originating obligation owner | Owns the underlying obligation and any authoritative amendment, cancellation, correction, credit/debit evidence, or commercial conclusion. |
| Procurement and Resource Management / Section J | Owns vendor, invoice, credit/debit source document, Purchase Order, contract, receipt, acceptance, and related procurement evidence. |
| Enterprise Governance | Owns approval policy, workflow, matrix, routing, hierarchy, execution, and immutable approval outcome. |
| Treasury & Banking / Section F | Owns payment/settlement history and any in-flight or completed cash outcome relevant to change eligibility. |
| Accounting & General Ledger / Section F | Owns Journal, Ledger, and compensating Accounting consequences arising from the approved adjustment. |

No approved change transfers aggregate ownership. Cross-domain effects occur through immutable completed facts and references.

---

# 6. Business Actors, Roles and Responsibilities

The applicable L2 sources define no canonical actors for requesting, validating, reviewing, approving, rejecting, publishing, applying, reversing, or incorporating a Payable adjustment. Financial Adjustment architecture explicitly leaves approval execution external. This Draft therefore assigns responsibilities to canonical domains and records the actor and authority gaps in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid Accounts Payable-owned `Payable` exists with immutable identity and exactly one originating financial-obligation reference.
- Its current outstanding liability, due schedule, readiness, settlement references, lifecycle, and version are available.
- The change request identifies an authoritative source fact, proposed financial effect, category, reason, amount, currency, institution, financial context, and affected Payable/reference.
- Relevant invoice, credit/debit note, procurement, contract, receipt, approval, settlement, dispute/hold, and Accounting evidence is available where applicable.
- The proposed effect has not already been incorporated through another Financial Adjustment.
- The Payable's current lifecycle permits the change or an approved suspension/reopening path has been completed.
- The requested amount and direction will not produce an invalid Outstanding Liability or conflict with immutable settlement history.

---

# 8. Trigger

A canonical source owner publishes or confirms new evidence requiring a monetary change to the Payable, or a governed decision identifies the need to compensate for a prior incorrect or no-longer-valid financial effect.

`L2-F08` supports liability change through financial adjustment, credit note, debit note, liability correction, payment allocation, and approved write-off. Exact source events and the canonical post-application reversal model are not defined and remain open in Section 17.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Payable, originating obligation, current Outstanding Liability, source references, settlement history, due schedule, readiness, and proposed change. | Accounts Payable | The affected financial position and historical baseline are established. |
| 2 | Provide the authoritative invoice, credit/debit, procurement, contract, legal, settlement, or correction evidence supporting the change. | Canonical source owner | A traceable business basis exists without modifying the Payable. |
| 3 | Validate source ownership, uniqueness, amount, direction, currency, institution, financial context, prior adjustments, settlement impact, and Payable eligibility. | Accounts Payable and Financial Adjustment Management | The proposed modification is confirmed as valid for adjustment consideration. |
| 4 | Determine whether the outcome is a new adjustment or a separately traceable compensating reversal of a prior effect; do not overwrite the original record. | Financial Adjustment Management | The correct non-destructive correction path is selected. |
| 5 | Establish one canonical `Financial Adjustment` with immutable identity, one source, one category, one reason, institutional/external references, proposed amount/effect, and complete Payable lineage. | Financial Adjustment Management | The proposed Payable change has an authoritative adjustment representation. |
| 6 | Validate the Adjustment and submit its approval context to Enterprise Governance. | Financial Adjustment Management | A valid proposal enters governed approval without transferring ownership. |
| 7 | Execute approval and return an immutable approved or rejected outcome with authority, reason, and decision evidence. | Enterprise Governance | A governed adjustment/reversal decision is established. |
| 8 | For approval, publish and apply the Financial Adjustment according to its canonical lifecycle; for rejection/cancellation, retain the decision and stop financial application. | Financial Adjustment Management | The approved modification becomes part of the institutional financial position or remains unapplied. |
| 9 | Consume the applicable Adjustment fact, associate its immutable reference with the Payable, and update Outstanding Liability exactly once. | Accounts Payable | The Payable reflects the authoritative monetary effect without modifying the Adjustment. |
| 10 | Re-evaluate Due Schedule, Payment Readiness, ageing, settlement consistency, and lifecycle; advance aggregate version and validate invariants. | Accounts Payable | The Payable's operational state remains consistent with its updated liability. |
| 11 | Publish `OutstandingLiabilityUpdated` and any supported schedule/readiness/lifecycle facts; make the Adjustment and Payable outcomes available to Accounting, Treasury, Reporting, and Audit. | Accounts Payable and Financial Adjustment Management | The approved change is complete and cross-domain traceability is preserved. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Approved credit adjustment | A valid credit note or approved reduction decreases the liability. | Create and apply the appropriate credit/balance-correction Adjustment, reduce Outstanding Liability, and re-evaluate due/readiness/settlement status. | The Payable liability decreases without erasing the original amount. | Source owner, Financial Adjustment Management, and Accounts Payable |
| Approved debit adjustment | A valid debit note or approved increase raises the liability. | Create and apply the appropriate debit/balance-correction Adjustment and update the Payable under financial and settlement constraints. | The Payable liability increases with complete new-source lineage. | Source owner, Financial Adjustment Management, and Accounts Payable |
| Full compensating reversal | A prior Payable financial effect must be completely neutralized. | Create a new compensating Adjustment referencing the original effect and apply the exact approved opposite financial consequence. | The net effect is reversed while both original and compensating histories remain visible. | Financial Adjustment Management and Accounts Payable |
| Partial compensating reversal | Only part of a prior effect is invalid. | Create a separately identifiable compensating Adjustment for the approved portion and preserve the remaining original effect. | The Payable is corrected only by the authorized amount. | Financial Adjustment Management and Accounts Payable |
| Request rejected | Approval returns a negative outcome. | Preserve the proposal, evidence, decision, and reason; do not publish/apply an approved financial effect. | The Payable remains in its prior valid position. | Enterprise Governance and Financial Adjustment Management |
| Request suspended or cancelled | Adjustment evidence, approval, or policy prevents progression. | Retain the Adjustment in its canonical exceptional state and keep the Payable unchanged or suspended as governed. | No unapproved Payable change occurs. | Financial Adjustment Management and Accounts Payable |
| Settlement already completed or in progress | Treasury facts overlap the proposed change. | Reconcile the change against immutable settlement history and route any cash return/recovery to Treasury governance before updating Payable effect. | No liability change silently rewrites a completed or uncertain settlement. | Treasury, Financial Adjustment Management, and Accounts Payable |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Source evidence is missing, duplicated, contradictory, or not authoritative. | The Adjustment cannot preserve valid business origin or justification. | Withhold Adjustment creation/application and obtain corrected source-owned evidence. | Canonical source owner and Financial Adjustment Management |
| Proposed amount/direction conflicts with current liability or settlement history. | The Payable could become financially invalid or inconsistent with cash outcomes. | Retain the last valid Payable position and reconcile source, Adjustment, Treasury, and Accounts Payable facts. | Accounts Payable and affected owners |
| Approval outcome is missing, ambiguous, expired, or untraceable. | The Adjustment cannot progress to an approved milestone. | Do not publish or apply the change; obtain a valid immutable outcome from Enterprise Governance. | Enterprise Governance and Financial Adjustment Management |
| A direct Payable edit or destructive reversal is attempted. | Aggregate ownership, auditability, and financial history would be bypassed. | Reject the action and require a canonical source fact plus Financial Adjustment/compensating path. | Accounts Payable and Financial Adjustment Management |
| Adjustment publication/application fails validation. | The approved proposal has not become an authoritative financial-position change. | Contain the exception within Financial Adjustment governance and leave the Payable unchanged. | Financial Adjustment Management |
| Duplicate Adjustment fact reaches Accounts Payable. | Reapplication would duplicate the financial effect. | Preserve the existing immutable reference and reject duplicate Payable mutation. | Accounts Payable |
| Payable aggregate validation fails after Adjustment receipt. | The Adjustment and Payable states cannot be synchronized safely. | Preserve the authoritative Adjustment history, withhold Payable event publication, and reconcile the inconsistent reference, amount, lifecycle, or settlement fact. | Accounts Payable and Financial Adjustment Management |
| Accounting consequence conflicts with operational change. | Financial books and Payable may diverge. | Preserve both histories and resolve through separately governed Accounting correction/compensating Journal behaviour; do not rewrite either aggregate. | Accounting & General Ledger and Finance owners |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Outstanding Liability may evolve through payment allocation, Financial Adjustment, credit note, debit note, liability correction, or approved write-off, and every modification preserves financial consistency. | Accounts Payable and source owners | The Payable may change only from an authoritative supported financial fact. | `L2-F08`, Capability 3 |
| Financial Adjustment is the only canonical aggregate for an approved modification to an institutional financial position. | Financial Adjustment Management | Payable monetary changes cannot be represented as direct edits or independent adjustment entities. | `L2-F04`, Sections 2-4 |
| Every Financial Adjustment has one canonical source, category, reason, and institutional reference, all permanently auditable. | Financial Adjustment Management | The proposed change must preserve complete business origin, classification, justification, and reference lineage. | `L2-F04`, Sections 8-13 |
| Financial Adjustment approval is externally governed; only validated and approved Adjustments may be published. | Enterprise Governance and Financial Adjustment Management | An unapproved change has no Payable financial effect. | `L2-F04`, Sections 11, 14-16 |
| At `Applied`, the approved Financial Adjustment becomes part of the institutional financial position. | Financial Adjustment Management | Accounts Payable must not incorporate a Draft, merely requested, or unapproved change. | `L2-F04`, Section 14.4 |
| Published and completed Adjustment milestones, categories, reasons, and references remain immutable. | Financial Adjustment Management | Reversal uses a new compensating fact rather than mutation of history. | `L2-F04`, Sections 8-17 and 24 |
| Outstanding Liability remains financially valid, never conflicts with settlement history, and negative balances are prohibited unless explicitly configured. | Accounts Payable | Applied adjustment must pass Payable monetary and settlement invariants. | `L2-F08`, Financial Invariants |
| Settlement references are immutable and history is append-only. | Accounts Payable and Treasury & Banking | Adjustment cannot delete, alter, or conceal completed settlement evidence. | `L2-F08`, Settlement Invariants |
| External domains influence Payable state only through events; ownership never transfers. | Accounts Payable and participating owners | Financial Adjustment publishes the fact; Accounts Payable alone changes its liability representation. | `L2-F08`, Sections 5, 10 and 13 |
| Accounting corrections preserve posted financial history through compensating behaviour. | Accounting & General Ledger | Operational Payable adjustment does not rewrite Journals or Ledger history. | `L2-F05` Accounting immutability and reversal constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Adjustment | Does not yet exist, or prior Adjustment remains immutable when compensation is required | New Adjustment progresses through validation, external approval, publication, application, and completion; rejection path is not canonically defined. | Approved path: `Applied` then `Completed`; exceptional `Suspended` or `Cancelled` where governed | Financial Adjustment Management | `L2-F04`, Sections 14-17 |
| `Payable` | Existing valid active, suspended, partially settled, fully settled, closed, or written-off position subject to transition rules | Incorporates the applicable Adjustment once and re-evaluates liability, schedule, readiness, ageing, settlement, and lifecycle. | Source-permitted updated position; exact transitions vary and are incomplete upstream | Accounts Payable | `L2-F08`, Capabilities 3-5 and Aggregate State Model/Invariants |
| Outstanding Liability | Current authoritative unpaid amount | Increases, decreases, or is compensated according to the approved applied Adjustment. | Financially valid updated amount | Accounts Payable | `L2-F08`, Capability 3 and Financial Invariants |
| Originating obligation/source record | Source-defined existing position | Supplies immutable change evidence and remains owned outside Accounts Payable/Adjustment. | Source-owned amended, corrected, credited, debited, cancelled, or unchanged position | Canonical source owner | `L2-F08`, upstream relationships; `L2-F14` where applicable |
| Accounting records | Existing or not yet posted for the change | Receive applied Adjustment/Payable facts for separate Journal and Ledger treatment or compensation. | Source-defined Accounting lifecycle position | Accounting & General Ledger | `L2-F04`, Section 17; `L2-F05` |

Exact Payable transitions after increase/decrease, post-settlement correction, closed/written-off change, and compensating reversal are not defined and remain in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Authoritative invoice/credit/debit/obligation/correction event | Section J or other canonical source owner | Financial Adjustment Management and Accounts Payable | Supplies immutable evidence for the proposed change; exact event catalogue is incomplete. | `L2-F08`, Capability 3; `L2-F14` |
| `AdjustmentValidated` | Financial Adjustment Management | Enterprise Governance and governed Finance consumers | Confirms that the proposed Payable modification passes Adjustment validation. | `L2-F04`, Section 17.4 |
| `AdjustmentApproved` | Financial Adjustment Management | Accounts Payable, Accounting, Reporting, Audit | Records the positive approval milestone in the Adjustment lifecycle. | `L2-F04`, Section 17.4 |
| `AdjustmentPublicationCompleted` | Financial Adjustment Management | Accounts Payable, Accounting, Reporting, Analytics | Makes the approved Adjustment available for enterprise consumption. | `L2-F04`, Sections 16 and 17.4 |
| `AdjustmentApplicationCompleted` or `AdjustmentFinancialPositionUpdated` | Financial Adjustment Management | Accounts Payable and Accounting & General Ledger | Supplies the authoritative applied financial effect for Payable incorporation. | `L2-F04`, Sections 14.4 and 17.4-17.5 |
| `AdjustmentCancelled` or `AdjustmentSuspended` | Financial Adjustment Management | Accounts Payable and governed participants | Communicates an exceptional unapplied Adjustment outcome. | `L2-F04`, Section 17.4 |
| `OutstandingLiabilityUpdated` | Accounts Payable | Treasury, Accounting, Reporting, Audit, dashboards | Announces incorporation of the authoritative change into the remaining liability. | `L2-F08`, Section 13 |
| `DueScheduleUpdated` / `PaymentReadinessChanged` | Accounts Payable | Treasury and governed Finance consumers | Communicates resulting timing or readiness changes. | `L2-F08`, Section 13 |

No canonical FinancialAdjustmentReversed, PayableAdjustmentRecorded, PayableReversed, adjustment-rejected, or compensating-adjustment-linked event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section J/originating obligation/Treasury/other source owner | Financial Adjustment Management and Accounts Payable | Immutable change evidence with source identity, affected Payable/obligation, amount, direction, reason, effective boundary, and audit lineage | Establishes the business basis without transferring source ownership. | `L2-F04`, Sections 8-13; `L2-F08`, Capability 3 |
| Financial Adjustment Management | Enterprise Governance | Validated Adjustment approval context | Enables external approval execution while Financial Adjustment retains aggregate ownership. | `L2-F04`, Section 11 |
| Enterprise Governance | Financial Adjustment Management | Immutable approved or rejected decision | Determines whether the Adjustment may progress to publication/application. | `L2-F04`, Section 11 |
| Financial Adjustment Management | Accounts Payable and Accounting & General Ledger | Approved, published, applied Adjustment/compensating fact with source, category, reason, amount, references, and history | Enables independent Payable and Accounting consequences. | `L2-F04`, Sections 14-17 |
| Accounts Payable | Treasury, Accounting, Reporting, Audit | Updated Outstanding Liability, schedule, readiness, lifecycle, and Adjustment references | Communicates the Payable consequence without transferring ownership. | `L2-F08`, Sections 10, 13 and 14 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request Payable change | No canonical actor defined | Must provide authoritative source, reason, amount, direction, affected Payable, and evidence; request alone has no financial effect. | Canonical source owner / Accounts Payable context provider |
| Determine adjustment versus compensating reversal | No canonical actor defined | Must preserve original history and use one canonical Financial Adjustment rather than destructive mutation. | Financial Adjustment Management |
| Validate and classify Adjustment | No canonical actor defined | Requires one source, category, reason, institutional reference, financial integrity, and traceability. | Financial Adjustment Management |
| Approve or reject Adjustment/reversal | No canonical role, matrix, hierarchy, or threshold defined | Approval execution remains external and must produce immutable evidence. | Enterprise Governance |
| Publish/apply approved Adjustment | No canonical actor defined | Only validated and positively approved Adjustment may progress. | Financial Adjustment Management |
| Incorporate Payable effect | No canonical actor defined | Requires completed applicable Adjustment fact, unique reference, valid amount, and Payable invariant success. | Accounts Payable |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Source owner versus Adjustment owner | Source domain and Financial Adjustment Management | Source supplies business evidence; Financial Adjustment owns the modification aggregate. |
| Adjustment preparation versus approval | Financial Adjustment responsibility and Enterprise Governance | Proposal/validation cannot self-create approval authority. |
| Adjustment versus Payable mutation | Financial Adjustment Management and Accounts Payable | Adjustment publishes the applicable fact; Accounts Payable alone updates Outstanding Liability. |
| Payable change versus Treasury cash | Accounts Payable and Treasury & Banking | Liability adjustment does not silently reverse or create cash movement. |
| Operational adjustment versus Accounting correction | Financial Adjustment, Accounts Payable, and Accounting | Each domain applies only its own consequence and preserves compensating history. |
| Rejection or failed application | Governance, Financial Adjustment, and Accounts Payable | No override may force an unapplied or rejected modification into the Payable. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove original position | Payable identity, obligation, source documents, previous liability, schedule, readiness, settlement, prior adjustments, and Accounting references | Accounts Payable and source owners |
| Prove change basis | New source identity, credit/debit/correction/reversal evidence, amount, direction, reason, effective boundary, and uniqueness | Canonical source owner |
| Prove Adjustment integrity | Adjustment identity, source, category, reason, references, validation, state/lifecycle, and immutable history | Financial Adjustment Management |
| Prove approval | Approval context, outcome, authority, reason, sequence, and decision evidence | Enterprise Governance |
| Prove application and Payable effect | Adjustment publication/application, previous/resulting liability, schedule/readiness/lifecycle effect, version, and Accounts Payable events | Financial Adjustment Management and Accounts Payable |
| Prove compensating lineage | Explicit original-to-compensating relationship and net financial effect without deletion or overwrite | Financial Adjustment Management, Accounts Payable, and Accounting |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The affected Payable, original obligation, current Outstanding Liability, settlement history, and source evidence are complete and traceable.
- The proposed amount, direction, currency, institution, financial context, reason, category, and uniqueness are validated.
- One canonical Financial Adjustment represents the requested change or compensating reversal.
- External approval returns an immutable governed outcome.
- A rejected, suspended, cancelled, or unapplied Adjustment has no Payable financial effect.
- An approved Adjustment progresses to the authoritative application milestone before Payable incorporation.
- Accounts Payable applies the effect exactly once and preserves financial, settlement, lifecycle, and aggregate invariants.
- Supported Adjustment and Payable events are published in causal order with complete audit lineage.

## 16.2 Postconditions

- Financial Adjustment Management owns the approved adjustment/compensating-reversal history.
- Accounts Payable retains ownership of the updated Payable and Outstanding Liability.
- The originating obligation and source documents remain owned by their canonical domains.
- Treasury cash movement and Accounting Journals remain separately owned and are not silently modified.
- Original Payable, Adjustment, settlement, and Accounting records remain immutable and reconstructable.
- Rejected or unapplied requests leave the Payable in its previous valid position.
- Any resulting readiness, schedule, settlement, suspension, reopening, write-off, or closure action proceeds only through its separately governed rules.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Financial Adjustment architecture has no canonical post-application reversal aggregate, lifecycle transition, event, or explicit original-to-compensating relationship. | Sections 9, 13, and 14 cannot lock the exact reversal model; the Draft uses non-destructive compensating behaviour as an architecture-preserving boundary. | Financial Adjustment owner and Lead Architect must define canonical adjustment reversal/compensation upstream. |
| The Phase 5 scenario is grouped with Accounts Payable, but its named final outcome is an approved Adjustment or reversal owned by Financial Adjustment Management. | Primary Owner is assigned under the template's final-outcome rule but requires confirmation before locking. | Lead Architect must confirm canonical L3 primary ownership for F-047. |
| No canonical actors, requester, validator, reviewer, approver, reversal authority, application authority, monetary threshold, matrix, hierarchy, or separation-of-duty policy are defined. | Sections 6 and 15 cannot lock who may initiate, approve, reject, apply, or incorporate the change. | Enterprise Governance and Finance domain owners must define canonical roles and authorities. |
| Exact source events and contracts are absent for vendor credit note, debit note, invoice amendment, procurement correction, obligation cancellation, settlement correction, and Payable liability correction. | Section 8 and Section 14 cannot enumerate the complete trigger set or evidence schema. | Section J, Treasury, obligation owners, Financial Adjustment, and Accounts Payable must define the source contracts. |
| It is unclear whether Accounts Payable should incorporate on `AdjustmentApproved`, `AdjustmentPublicationCompleted`, `AdjustmentApplicationCompleted`, or `AdjustmentFinancialPositionUpdated`. | The authoritative financial-effect trigger remains ambiguous and may cause premature or delayed liability updates. | Financial Adjustment and Accounts Payable owners must lock the application contract. |
| Financial Adjustment has no `Rejected` state/event, although approval participation permits a negative outcome. | Rejected changes cannot be mapped to a canonical Adjustment lifecycle position or event. | Financial Adjustment and Enterprise Governance owners must define rejection handling. |
| `L2-F08` defines liability change sources but no Payable adjustment event, reversal event, transition rules, or exact lifecycle effects for increases/decreases after partial/full settlement, closure, suspension, or write-off. | Section 13 cannot lock all resulting Payable states. | Accounts Payable owner must define change transition guards and event catalogue. |
| Currency conversion, exchange-rate ownership, rounding, tolerance, effective-date ordering, tax/withholding treatment, and negative-balance configuration are not fully defined. | Sections 9-12 cannot guarantee deterministic calculation for every adjustment. | Finance foundation, Tax, Accounting, Financial Adjustment, and Accounts Payable owners must define the calculation policy upstream. |
| Treasury and Accounting consequences are not sequenced when a change affects an in-flight/completed payment or a posted Journal. | The scenario cannot lock cash recovery, payment cancellation, Journal reversal, or compensating-posting order. | Treasury, Accounting, Financial Adjustment, and Accounts Payable owners must define cross-domain correction sequencing. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-047**
