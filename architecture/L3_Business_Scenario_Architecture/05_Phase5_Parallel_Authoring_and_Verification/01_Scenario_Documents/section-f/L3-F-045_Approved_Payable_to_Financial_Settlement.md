# Approved Payable to Financial Settlement

# 1. Scenario Metadata

**Scenario ID:** L3-F-045  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Payable  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F06_Budget_and_Cost_Center_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs execution and completion of an approved institutional outgoing payment against an Accounts Payable liability. It ensures that Accounts Payable readiness and obligation ownership, external authorization, Treasury cash and settlement ownership, Accounting consequences, Financial Period controls, budget information, and immutable settlement traceability remain separated and consistent.

---

# 3. Business Outcome

An approved and payment-ready payable has produced a successful Treasury-owned financial settlement with an immutable settlement identity and reference, after which Accounts Payable has incorporated the settlement fact into its outstanding liability and preserved complete obligation-to-cash-to-Accounting lineage.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Accounts Payable has a validated payable with a valid due schedule, applicable approvals, no prohibiting dispute or suspension, a qualifying payment-readiness outcome, and an authoritative outgoing-payment obligation available to Treasury.

## 4.2 End Boundary

The scenario ends when Treasury has successfully completed settlement, published `SettlementCompleted`, updated the authoritative cash position, and Accounts Payable has recorded the immutable settlement reference and updated the remaining liability to partial or full settlement.

## 4.3 In Scope

- Validation of Payable identity, outstanding amount, due schedule, readiness, approvals, dispute/suspension status, and policy eligibility.
- Consumption of authorization and financial-governance outcomes required for Treasury execution.
- Treasury scheduling or batching where applicable.
- Execution of approved cash movement and completion of Treasury settlement.
- Preservation of payment, batch, settlement, bank, cash-position, Payable, and Accounting references.
- Incorporation of the completed settlement into Accounts Payable exactly once.
- Update of outstanding liability to partial or full settlement.
- Publication of Treasury and Accounts Payable completion facts.

## 4.4 Out of Scope

- Vendor invoice matching, liability creation, or initial Payable approval.
- Creation or modification of supplier, procurement, contract, invoice, or source-obligation records.
- Treasury determination of business approval or Accounts Payable execution of payment.
- Bank-account onboarding or banking-relationship governance.
- Failed, returned, uncertain, cancelled, or reversed bank-transaction resolution beyond recording the non-successful outcome.
- Accounting Journal or Ledger posting execution within Treasury or Accounts Payable.
- Payable dispute/hold resolution, liability adjustment, write-off, reopening, or historical closure.
- Lower-level banking exchange, transport, and processing design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-044` Vendor Invoice to Approved Payable Liability | Establishes the validated liability, due schedule, and readiness information. |
| Related | `L3-F-046` Payable Dispute or Hold to Governed Resolution | A dispute or hold may block settlement readiness. |
| Related | `L3-F-049` Payable Population to Reconciled Period Outcome | Uses settlement and outstanding-liability evidence in period reconciliation. |
| Related | `L3-F-061` Failed Returned or Uncertain Bank Transaction to Resolution | Governs exceptional bank outcomes that do not complete ordinary settlement. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Treasury & Banking / Section F | Owns institutional cash state, payment execution, Payment Batch, Settlement Execution, settlement status, settlement reference, completed cash movement, and final financial-settlement outcome. |
| Accounts Payable / Section F | Owns the Payable obligation, outstanding liability, due schedule, payment readiness, liability ageing, lifecycle state, and immutable association to Treasury settlement facts. |
| Authorization & Policy | Owns authorization logic and outcomes consumed by Treasury; authorization ownership does not transfer to Treasury. |
| Accounting & General Ledger / Section F | Owns Journal, Ledger, and Accounting treatment arising from completed Treasury and Payable facts. |
| Financial Period & Closing / Section F | Owns the financial-period state and governance Treasury must respect. |
| Budget Management / Section F | Owns budget availability and allocation facts referenced for settlement governance. |

Treasury executes approved cash movement but never owns the Payable obligation. Accounts Payable owns readiness but never authorizes or executes Treasury settlement.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F08` and `L2-F10` do not define canonical business actors for payment preparation, readiness confirmation, authorization, release, execution, settlement confirmation, exception review, or Payable update. This Draft therefore assigns responsibilities to canonical domains and records the missing roles and separation-of-duty authority in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid Accounts Payable-owned `Payable` exists with immutable identity and exactly one originating financial-obligation reference.
- The Payable is successfully validated and has a positive outstanding liability.
- Its due schedule is valid and the amount is due or otherwise eligible under institutional policy.
- Mandatory approvals and references are complete.
- No unresolved dispute, suspension, cancellation, write-off, closure, or compliance condition blocks progression.
- Accounts Payable has reached the applicable readiness position and supplied an approved outgoing-payment obligation to Treasury.
- Treasury has an eligible operational state, valid cash/bank context, and sufficient authorization to execute the settlement.
- Applicable Financial Period and budget-governance facts permit execution.
- The proposed amount does not exceed the Payable's authoritative outstanding liability.

---

# 8. Trigger

Treasury consumes an approved outgoing-payment obligation from Accounts Payable together with a qualifying settlement-readiness and authorization outcome.

`L2-F10` gives `VendorPaymentApproved` and `LiabilityReadyForSettlement` as illustrative consumed facts, while `L2-F08` publishes `PaymentReadinessChanged` rather than those exact events. The event-contract mismatch remains open in Section 17.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the Payable identity, approved amount, outstanding liability, due schedule, readiness, supplier/payment references, and absence of blocking dispute or suspension. | Accounts Payable | A valid outgoing-payment obligation is established for Treasury consideration. |
| 2 | Publish or provide the approved liability and qualifying readiness fact without initiating cash movement. | Accounts Payable | Treasury receives the obligation and readiness context while Accounts Payable retains ownership. |
| 3 | Validate external authorization, Financial Period governance, budget facts, Treasury restrictions, cash availability, bank context, settlement eligibility, and institutional policy. | Treasury & Banking with participating governance domains | The proposed settlement is eligible for execution or is withheld. |
| 4 | Establish an individual Payment Execution and, where applicable, associate it with one valid Payment Batch while preserving Payable and payment traceability. | Treasury & Banking | The approved payment is ready for controlled execution. |
| 5 | Initiate and execute the approved cash movement under Treasury ownership. | Treasury & Banking | Payment execution completes successfully or produces a separately governed failure outcome. |
| 6 | Publish `PaymentExecuted` after successful execution while recognizing that execution alone does not prove final settlement. | Treasury & Banking | Consumers receive the completed execution fact. |
| 7 | Validate and complete Settlement Execution, establish immutable settlement identity, status, timestamp/business boundary, outcome, and reference, and update cash position. | Treasury & Banking | Final cash movement is successfully settled and historically traceable. |
| 8 | Publish `SettlementCompleted` in causal order after settlement integrity and Treasury invariants succeed. | Treasury & Banking | The final Treasury-owned financial-settlement fact becomes authoritative. |
| 9 | Consume `SettlementCompleted`, associate the immutable Treasury settlement reference with the Payable, and reduce outstanding liability exactly once. | Accounts Payable | The Payable reflects the authoritative settlement without executing cash movement. |
| 10 | Determine partial or full settlement, advance Payable version and lifecycle consistently, and publish `SettlementReferenceRecorded`, `OutstandingLiabilityUpdated`, and, when zero remains, `PayableFullySettled`. | Accounts Payable | The Payable's operational and financial state is synchronized with the completed settlement. |
| 11 | Make completed Treasury and Payable facts available to Accounting, Reporting, Audit, and dashboards under their own ownership boundaries. | Treasury & Banking and Accounts Payable | Cross-domain financial lineage is complete without direct aggregate modification. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Partial settlement | The approved settlement amount is less than the Payable's outstanding liability. | Complete Treasury settlement for the approved amount, record its reference, and reduce the outstanding liability while retaining the Payable in `Partially Settled`. | The Payable remains active for later settlement of the balance. | Treasury & Banking and Accounts Payable |
| Full settlement | The approved settlement exactly equals the current outstanding liability. | Complete settlement, record its reference, reduce outstanding liability to zero, and publish `PayableFullySettled`. | The Payable reaches `Fully Settled` and may later close under policy. | Treasury & Banking and Accounts Payable |
| Multiple settlements | Institutional policy permits the liability to be settled through more than one approved payment. | Execute and record each settlement separately in causal order and update the outstanding amount once per authoritative settlement. | The Payable progresses through partial to full settlement with reconstructable history. | Treasury & Banking and Accounts Payable |
| Batched execution | Multiple approved payments are eligible for grouped Treasury execution. | Place each payment in no more than one active Payment Batch, preserve individual Payable/payment identity, and complete each settlement outcome independently. | Execution efficiency is achieved without losing individual liability traceability. | Treasury & Banking |
| Scheduled settlement | The approved liability is payable at a future governed date. | Record `PaymentScheduled`, preserve readiness and approval validity, and revalidate execution conditions at the due boundary. | No cash movement occurs until the permitted execution point. | Treasury & Banking and Accounts Payable |
| Readiness changes before execution | Documentation, dispute, suspension, approval, due schedule, or policy status changes. | Accounts Payable publishes the new readiness position; Treasury withholds execution until valid readiness and authorization are restored. | No ineligible payment is executed. | Accounts Payable and Treasury & Banking |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Payable is not Ready, is disputed/suspended, or lacks mandatory references. | Treasury cannot rely on the outgoing obligation for settlement. | Withhold execution and return the business condition to Accounts Payable governance; do not override readiness. | Accounts Payable and Treasury & Banking |
| Authorization, Financial Period, budget, Treasury restriction, or policy validation fails. | Cash movement is not permitted. | Do not execute payment; preserve the failed decision evidence and resolve the governing condition under its canonical owner. | Relevant governance owner and Treasury & Banking |
| Proposed settlement exceeds current outstanding liability or conflicts with currency/institution context. | Payment could overstate cash outflow or misapply liability. | Reject the settlement basis and reconcile Accounts Payable and Treasury facts before execution. | Accounts Payable and Treasury & Banking |
| Payment execution fails. | No successful execution fact or final settlement exists. | Publish the source-defined failure fact where applicable, preserve Treasury consistency, and route the outcome to the governed bank-transaction resolution scenario; do not reduce Payable liability. | Treasury & Banking |
| Payment executes but settlement fails or remains uncertain. | Cash movement is not authoritatively final and the Payable cannot be safely settled. | Preserve `PaymentExecuted`, publish `SettlementFailed` where defined, retain the Payable's previous outstanding position, and resolve the uncertainty separately. | Treasury & Banking and Accounts Payable |
| Duplicate `SettlementCompleted` is received. | Reapplication would understate the Payable and duplicate settlement history. | Retain the original immutable reference and reject duplicate financial effect. | Accounts Payable |
| Accounts Payable validation fails when incorporating settlement. | Treasury cash history and Payable state may diverge. | Preserve the Treasury fact, withhold Payable event publication, and reconcile the identity, amount, reference, or prior state without modifying Treasury history. | Accounts Payable and Treasury & Banking |
| A completed payment is later returned, reversed, or cancelled. | The prior settlement may no longer satisfy the liability. | Preserve the original settlement and process the new authoritative fact through the governed adjustment/reversal or reopening scenario. | Treasury & Banking and Accounts Payable |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Accounts Payable owns Payment Readiness, but readiness neither authorizes nor initiates Treasury execution. | Accounts Payable and Treasury & Banking | A Ready Payable still requires external authorization and Treasury eligibility. | `L2-F08`, Capability 5 |
| Payment Readiness evaluates completeness, approvals, dispute, suspension, due schedule, lifecycle, and institutional policy. | Accounts Payable | Treasury must not execute a Payable that fails any mandatory readiness condition. | `L2-F08`, Capability 5 |
| Settlement references are immutable, cannot exist independently, and preserve Treasury identity, date, type, and correlation. | Accounts Payable | Completed Treasury settlement is associated once and never rewritten. | `L2-F08`, Capability 7 |
| Outstanding liability remains financially valid and consistent with settlement history. | Accounts Payable | Every recorded settlement must reconcile to the remaining liability. | `L2-F08`, Aggregate Invariants |
| `Payment Ready -> Partially Settled` requires a Treasury settlement reference, and `Partially Settled -> Fully Settled` requires outstanding liability to equal zero. | Accounts Payable | Lifecycle movement follows authoritative settlement rather than execution intent. | `L2-F08`, Aggregate State Transition Rules |
| Treasury executes approved settlements but never determines payment approval. | Treasury & Banking and Authorization & Policy | Treasury cannot self-create the business authority to pay. | `L2-F10`, architectural boundaries and Accounts Payable integration |
| Every Payment Execution has valid authorization and settlement context; Treasury never executes an unapproved payment. | Treasury & Banking | Execution requires a completed valid approval basis. | `L2-F10`, Payment Invariants |
| Settlement completion is published only after successful settlement, and Cash Position updates only after completion. | Treasury & Banking | `PaymentExecuted` alone cannot reduce the Payable or finalize cash movement. | `L2-F10`, Settlement Invariants and event catalogue |
| Payment Batch membership preserves individual payment traceability and one payment cannot belong to multiple active batches. | Treasury & Banking | Batching cannot merge or obscure individual Payable settlement evidence. | `L2-F10`, Payment Batch Invariants |
| Accounting independently determines treatment from completed Treasury facts; Treasury never creates Journal Entries. | Accounting & General Ledger | Financial settlement and Accounting posting remain separate outcomes. | `L2-F10`, downstream Accounting integration |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| `Payable` | `Validated` / `Payment Ready` with positive Outstanding Liability | Receives immutable completed settlement reference and updates remaining liability. | `Partially Settled` or `Fully Settled`; closure is separate | Accounts Payable | `L2-F08`, Aggregate State Model and transition rules |
| Payment Readiness | `Ready` or source-equivalent qualifying position | Provides eligibility evidence but does not authorize or execute payment; may change before execution. | Remains or changes under Accounts Payable policy based on resulting liability | Accounts Payable | `L2-F08`, Capability 5 |
| Treasury Payment Execution | Eligible approved execution context | Progresses through scheduling/batching where applicable and successful execution. | `PaymentExecuted` business fact; exact state names are distributed in `L2-F10` | Treasury & Banking | `L2-F10`, Payment Execution capability and events |
| Treasury Settlement Execution | Initiated after eligible Payment Execution | Validates and finalizes cash movement with immutable identity and outcome. | Successfully completed settlement | Treasury & Banking | `L2-F10`, Settlement Management, Settlement Invariants, and `SettlementCompleted` |
| Cash Position | Current authoritative Treasury cash state | Reflects completed settlement only after success. | Updated authoritative cash position | Treasury & Banking | `L2-F10`, Settlement Invariants |
| Accounting records | No Journal for this settlement outcome yet | Consume completed Treasury and Payable facts for independent Accounting treatment. | Source-defined Journal/Ledger position | Accounting & General Ledger | `L2-F10`, Accounting integration; `L2-F05` |

The exact Treasury Payment Execution and Settlement state names and the precise Payable readiness event that initiates them are not consolidated into one cross-domain lifecycle and remain open in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PaymentReadinessChanged` | Accounts Payable | Treasury & Banking and governed consumers | Communicates the Payable's current readiness without authorizing payment. | `L2-F08`, Section 13 |
| `VendorPaymentApproved` / `LiabilityReadyForSettlement` | Accounts Payable or upstream authorization capability, exact publisher unresolved | Treasury & Banking | Illustrative Treasury-consumed facts identifying an approved outgoing obligation. | `L2-F10`, Accounts Payable integration |
| `TreasuryOperationAuthorized` or applicable approval outcome | Authorization & Policy | Treasury & Banking | Supplies external authorization required before Treasury execution. | `L2-F10`, Authorization & Policy integration |
| `PaymentScheduled` | Treasury & Banking | Accounts Payable, Reporting, Audit, and governed consumers | Announces future Treasury execution where scheduling applies. | `L2-F10`, Payment Events |
| `PaymentExecutionStarted` | Treasury & Banking | Accounts Payable, Audit, and operational consumers | Announces commencement of payment execution without implying completion. | `L2-F10`, Payment Events |
| `PaymentExecuted` | Treasury & Banking | Accounts Payable, Accounting & General Ledger, Reporting, Audit | Announces successful execution but not necessarily final settlement. | `L2-F10`, Payment Events and Accounting integration |
| `SettlementCompleted` | Treasury & Banking | Accounts Payable, Accounting & General Ledger, Reporting, Audit, dashboards | Finalizes cash movement and supplies the authoritative settlement fact. | `L2-F10`, Settlement Events and downstream integrations |
| `SettlementReferenceRecorded` | Accounts Payable | Treasury, Accounting, Reporting, and Audit | Announces successful immutable association of Treasury settlement information with the Payable. | `L2-F08`, Section 13 |
| `OutstandingLiabilityUpdated` | Accounts Payable | Treasury, Accounting, Reporting, Audit, dashboards | Announces the updated remaining institutional liability. | `L2-F08`, Section 13 |
| `PayableFullySettled` | Accounts Payable | Treasury, Accounting, Reporting, Audit, dashboards | Announces that no remaining Payable obligation exists. | `L2-F08`, Section 13 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Accounts Payable | Treasury & Banking | Approved outgoing obligation with Payable identity, amount, due schedule, readiness, supplier/payment references, institution, currency, and approval lineage | Enables Treasury eligibility evaluation while Accounts Payable retains obligation ownership. | `L2-F08`, Capabilities 3-7; `L2-F10`, Accounts Payable integration |
| Authorization & Policy | Treasury & Banking | Immutable authorized or rejected Treasury-operation outcome | Supplies execution authority without transferring approval ownership. | `L2-F10`, Authorization & Policy integration |
| Financial Period & Closing and Budget Management | Treasury & Banking | Period-governance and budget facts | Ensures settlement respects financial controls without Treasury modifying those aggregates. | `L2-F10`, upstream integrations |
| Treasury & Banking | Accounts Payable | `PaymentExecuted`, `SettlementCompleted`, settlement identity, status, outcome, date/business boundary, and immutable reference | Enables Accounts Payable to update settlement references and liability after final cash movement. | `L2-F10`, Payment/Settlement events; `L2-F08`, Capability 7 |
| Treasury & Banking and Accounts Payable | Accounting & General Ledger | Completed cash-settlement and updated liability facts with source lineage | Enables independent Journal and Ledger treatment. | `L2-F10`, Accounting integration; `L2-F08`, downstream relationships |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Confirm Payable readiness | No canonical actor defined in `L2-F08` | Must evaluate completeness, approvals, disputes, suspension, schedule, lifecycle, and policy; readiness does not authorize payment. | Accounts Payable |
| Authorize outgoing payment/Treasury operation | No canonical role, matrix, threshold, or hierarchy defined | Authorization logic remains external and must produce a traceable positive outcome. | Authorization & Policy / Enterprise Governance |
| Schedule, batch, execute, and complete settlement | No canonical Treasury actor defined | Treasury may act only on approved eligible obligations and must preserve individual payment and settlement integrity. | Treasury & Banking |
| Incorporate settlement into Payable | No canonical Accounts Payable actor defined | Requires authoritative `SettlementCompleted`, unique reference, valid amount, and successful aggregate validation. | Accounts Payable |
| Determine Accounting treatment | No canonical Accounting actor identified in this scenario's sources | Accounting independently owns Journal and Ledger consequences. | Accounting & General Ledger |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Payable readiness versus payment authorization | Accounts Payable and Authorization & Policy | Readiness is not approval; authorization cannot rewrite Payable state. |
| Authorization versus Treasury execution | Authorization authority and Treasury execution responsibility | Treasury consumes the outcome and cannot self-approve payment. |
| Execution versus settlement completion | Treasury payment and settlement responsibilities | `PaymentExecuted` does not by itself prove final cash settlement. |
| Treasury settlement versus Payable update | Treasury & Banking and Accounts Payable | Treasury owns cash movement; Accounts Payable owns the liability consequence and reference. |
| Settlement versus Accounting | Treasury, Accounts Payable, and Accounting & General Ledger | Neither Treasury nor Accounts Payable creates or modifies Accounting Journals. |
| Failure or return | Treasury exception responsibility and Accounts Payable | Non-successful or uncertain cash movement cannot be treated as final Payable settlement. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove payable eligibility | Payable identity, obligation, outstanding amount, validation, due schedule, readiness, approvals, dispute/suspension status, policy, and source references | Accounts Payable |
| Prove authorization | Authorization context, outcome, authority, rule/threshold where defined, sequence, and audit reference | Authorization & Policy |
| Prove Treasury execution | Payment identity, Payable reference, batch membership where applicable, execution state, amount, currency, bank/cash context, outcome, and `PaymentExecuted` | Treasury & Banking |
| Prove settlement | Settlement identity, status, timestamp/business boundary, reference, outcome, validation, Cash Position update, and `SettlementCompleted` | Treasury & Banking |
| Prove Payable update | Unique settlement reference, previous and resulting outstanding liability, aggregate version, lifecycle result, and Accounts Payable events | Accounts Payable |
| Preserve Accounting lineage | Completed Treasury and Payable facts linked to resulting Journal and Ledger evidence | Accounting & General Ledger |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Payable is validated, approved, eligible, and not blocked by dispute, suspension, missing documentation, policy, Financial Period, budget, or Treasury restriction.
- External authorization and Treasury settlement eligibility are complete and traceable.
- Payment Execution preserves valid Payable identity, amount, currency, approval, bank/cash context, and batch membership where applicable.
- `PaymentExecuted` is distinguished from final settlement.
- Settlement Execution completes successfully with immutable identity, reference, outcome, and cash-position effect.
- `SettlementCompleted` is published after Treasury invariants succeed.
- Accounts Payable incorporates the settlement exactly once, updates outstanding liability, and publishes the applicable partial/full settlement facts.
- Complete Payable-to-Treasury-to-Accounting lineage is retained.

## 16.2 Postconditions

- Treasury & Banking owns the completed financial settlement and resulting cash-position history.
- Accounts Payable retains ownership of the Payable, settlement references, and remaining liability.
- A partially settled Payable remains active for later governed settlement.
- A fully settled Payable has zero outstanding liability and may close only through its separate closure policy.
- Accounting may consume the completed settlement and liability facts but determines its own Journal and Ledger treatment.
- Failed, returned, uncertain, cancelled, reversed, duplicated, or unfinalized cash movement has not been treated as successful Payable settlement.
- Historical payment, settlement, Payable, authorization, and Accounting evidence remains immutable and reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F10` consumes `VendorPaymentApproved` and `LiabilityReadyForSettlement`, while `L2-F08` defines `PaymentReadinessChanged` but does not publish those exact events. | Sections 8 and 14 cannot lock the canonical hand-off event, publisher, or semantics. | Accounts Payable, Treasury & Banking, Authorization & Policy, and Lead Architect must normalize the contract. |
| The Phase 5 scenario is grouped with Accounts Payable, but its named final outcome, financial settlement, is explicitly owned by Treasury. | Primary Owner is assigned to Treasury under the template's final-outcome rule, but register-level ownership confirmation is required before locking. | Lead Architect must confirm canonical L3 primary ownership for F-045. |
| No canonical business actors, payment-preparation role, authorization matrix, monetary threshold, payment-release authority, Treasury executor, settlement confirmer, or separation-of-duty mapping are defined. | Sections 6 and 15 cannot lock who prepares, approves, releases, executes, confirms, or reviews settlement. | Authorization, Accounts Payable, Treasury, and governance owners must define canonical roles and authorities. |
| Readiness states in `L2-F08` are illustrative and configurable, and the exact state corresponding to `LiabilityReadyForSettlement` is undefined. | Section 7 and Section 13 cannot lock the precise eligibility transition. | Accounts Payable owner must define canonical readiness states and transition rules. |
| Payment Execution and Settlement Execution state models are distributed in `L2-F10` without a concise canonical state sequence covering scheduled, batched, started, executed, settled, failed, returned, cancelled, and reversed outcomes. | Section 13 cannot fully map every Treasury lifecycle transition. | Treasury & Banking owner and Lead Architect must normalize the execution and settlement state machines. |
| The sources do not define exact settlement amount tolerances, currency conversion, exchange-rate ownership, fees, withholding, deductions, partial-payment policy, priority, batching rules, or effective-date ordering. | Sections 9-12 cannot lock all financial calculation and sequencing decisions. | Treasury, Accounts Payable, Accounting, Tax, and Finance foundation owners must define the financial settlement policy upstream. |
| `L2-F10` defines `PaymentExecutionFailed` and `SettlementFailed`, but returned, reversed, uncertain, duplicate, and bank-rejected outcomes and their impact on Accounts Payable are incomplete. | Section 11 cannot establish the full recovery and liability-restoration lifecycle. | Treasury and Accounts Payable owners must define exceptional settlement contracts, coordinated with F-061. |
| No exact rule defines whether Accounts Payable updates on `PaymentExecuted` or only on `SettlementCompleted`; source principles indicate completion, but the cross-domain contract is not explicit. | Premature or delayed liability reduction remains an integration risk. | Treasury and Accounts Payable owners must formally lock `SettlementCompleted` as the authoritative Payable financial-effect trigger or define another event. |
| Accounting sequencing relative to Treasury cash completion and Accounts Payable liability update is not defined. | The scenario cannot determine whether Accounting posts from Treasury, Payable, or coordinated events and in what order. | Accounting, Treasury, and Accounts Payable owners must define the authoritative Accounting contract. |
| No canonical event represents Accounts Payable settlement rejection, deferred incorporation, duplicate settlement ignored, or settlement-reference conflict. | Exceptional Payable synchronization outcomes cannot be communicated using source-defined events. | Accounts Payable owner must define required events or approve evidence-only handling. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-045**
