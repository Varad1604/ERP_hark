# Failed, Returned, or Uncertain Bank Transaction to Resolution

# 1. Scenario Metadata

**Scenario ID:** L3-F-061  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; `L3-F-057_Approved_Payment_Population_to_Bank_Execution_Outcome.md`; `L3-F-058_Bank_Statement_to_Reconciled_Cash_Position.md`; `L3-F-060_Authorized_Fund_Transfer_to_Settled_Inter_Account_Movement.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs resolution of a Treasury bank transaction or execution whose outcome is failed, returned, reversed, disputed, contradictory, delayed, or uncertain. It preserves every original execution, settlement, acknowledgement, bank, cash, reconciliation, and governance fact; prevents duplicate cash movement through blind retry; obtains authoritative evidence; coordinates correction through canonical owners; and establishes a traceable resolved outcome without rewriting history.

---

# 3. Business Outcome

The affected transaction reaches a governed resolution supported by authoritative bank and Treasury evidence: confirmed failed with no settlement, confirmed successful and settled, returned/reversed with corrective facts, retry approved and completed through a new linked attempt, cancelled/abandoned under authority, or retained as an escalated unresolved exception. Cash Position, settlement, upstream obligation, reconciliation, and accounting consequences remain owned and corrected by their canonical domains.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Treasury or an external bank reports `PaymentExecutionFailed`, `SettlementFailed`, a return/reversal, inconsistent acknowledgement, missing finality, statement exception, duplicate suspicion, or other evidence that a payment, transfer, settlement, or Bank Transaction outcome is not reliably final.

## 4.2 End Boundary

The scenario ends when authoritative evidence supports a final governed classification and every required Treasury, bank, Cash Position, settlement, reconciliation, notification, upstream-domain, and accounting hand-off is recorded; or when the case is formally escalated as unresolved with execution blocked and ownership preserved.

## 4.3 In Scope

- Identification and correlation of original obligation/payment, execution attempt, batch where applicable, Settlement Execution, bank instruction, acknowledgement, Bank Transaction, account, amount, currency, and Institution.
- Classification of failure, return, reversal, uncertainty, contradiction, duplicate suspicion, or timing issue.
- Preservation of immutable original and subsequent facts.
- Containment of duplicate execution/settlement risk and temporary Treasury restriction where governed.
- Acquisition and translation of authoritative bank evidence.
- Reconciliation investigation and exception publication.
- Determination of retry eligibility under Settlement Policy and external governance.
- Creation of a new linked attempt where retry is authorized; never rewriting the original attempt.
- Cash Position/Liquidity correction only through completed source-supported Treasury behaviour.
- Cross-domain hand-offs to AP, AR, GL, Reporting, Audit, and Governance.

## 4.4 Out of Scope

- Directly changing the underlying Payable, Receivable, refund, grant, payroll, tax, or other obligation.
- Inventing a successful/failed bank outcome without authoritative evidence.
- Deleting, overwriting, or repurposing original execution, settlement, bank, reconciliation, cash, or event history.
- Blindly retrying an uncertain transaction.
- Creating Journal Entries or modifying Ledgers.
- Defining bank APIs, status codes, payment rails, timeout technology, retry engines, or storage mechanisms.
- Inventing returned/uncertain transaction lifecycle states or events absent from locked L2 architecture.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-057` Approved Payment Population to Bank Execution Outcome | Produces execution success, failure, or uncertainty requiring resolution. |
| Related | `L3-F-058` Bank Statement to Reconciled Cash Position | Detects contradictory, returned, reversed, duplicate, or unmatched bank facts. |
| Related | `L3-F-060` Authorized Fund Transfer to Settled Inter-Account Movement | Inter-account movement may have asymmetric or uncertain outcomes. |
| Related | `L3-F-047` Payable Change to Approved Adjustment or Reversal | AP independently resolves liability consequences. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Treasury & Banking / Section F | Owns Payment/Settlement Execution state and history, Bank Transaction representations, Cash/Liquidity Positions, reconciliation coordination, aggregate version, restrictions, and Treasury events. |
| External Banking Platform | Owns original bank acknowledgements, account activity, returns/reversals, reason/reference, and external transaction finality evidence. |
| Authorization & Policy / Enterprise Governance | Own retry, cancellation, exception, write-back, escalation, emergency restriction, and approval outcomes. |
| Accounts Payable | Owns liability/payment eligibility and independently reacts to failure/return/settlement facts. |
| Accounts Receivable | Owns receivable/refund/receipt consequences where applicable. |
| Accounting & General Ledger | Owns Journal/Ledger correction and financial reconciliation outcomes. |
| Financial Period & Closing | Owns period lifecycle and closed-period restrictions affecting correction timing. |
| Financial Reporting / Audit | Consume immutable resolved/unresolved facts for visibility and review. |
| Institution and Organisation Management | Own institutional and organisational context. |

Treasury resolves execution and cash truth; it does not decide the continuing validity of an upstream obligation or create accounting corrections.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F10` defines no canonical incident initiator, transaction investigator, bank liaison, retry approver, exception authority, settlement reviewer, cash-correction approver, reconciler, or closure reviewer. No actor or escalation hierarchy is invented. Section 17 records the missing authority and segregation model.

---

# 7. Preconditions and Entry Criteria

- An authoritative Treasury Aggregate and affected Institution/account context exist.
- Original payment/transfer/settlement/bank transaction identifiers and correlation are available or can be investigated.
- The latest Treasury aggregate version, execution/settlement history, Cash Position, and reconciliation evidence are accessible.
- Failure, return, reversal, uncertainty, contradiction, duplicate suspicion, or timeout evidence is preserved.
- External bank evidence can be requested/received and translated through the anti-corruption boundary.
- Upstream obligation owners and Accounting can receive completed facts without Treasury changing their aggregates.
- Authorization and governance decisions can be obtained for retry, cancellation, exception, restriction, or corrective behaviour.
- Historical facts remain immutable and new correction/resolution facts can be appended.
- No retry proceeds while a prior attempt may still settle unless authoritative policy and evidence permit it.

---

# 8. Trigger

Treasury receives `PaymentExecutionFailed`, `SettlementFailed`, a reconciliation exception, a bank-return/reversal/contradiction, a missing or delayed final acknowledgement, or another authoritative indication that the bank transaction outcome requires governed resolution.

`L2-F10` defines failure and reconciliation events but no canonical returned, reversed, uncertain, timed-out, disputed, or resolution-requested event.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify Institution, account, obligation/payment reference, execution attempt, batch, Settlement Execution, bank request/reference, amount, currency, timing, and correlation. | Treasury & Banking | The exact affected business movement is established. |
| 2 | Preserve the triggering failure/return/uncertainty evidence and original state/version without modification. | Treasury & Banking | Investigation begins from immutable facts. |
| 3 | Detect duplicate requests/attempts, concurrent resolution cases, related bank statement lines, and already completed resolution outcomes. | Treasury & Banking | Duplicate recovery action is prevented. |
| 4 | Classify the case initially as execution failed, settlement failed, returned, reversed, uncertain, contradictory, duplicate suspected, timing-related, or requires investigation. | Treasury & Banking | A current non-final resolution classification exists. |
| 5 | Evaluate whether further execution/settlement must be held or Treasury restricted under governance while finality is uncertain. | Treasury & Banking / Enterprise Governance | Duplicate cash movement and cascading exposure are contained. |
| 6 | Obtain authoritative external bank acknowledgement, transaction status, statement evidence, return/reversal reference, reason, value date, account, amount, currency, and finality information. | External Bank / Treasury & Banking | Bank-side evidence is available. |
| 7 | Translate external evidence into canonical Treasury meaning and correlate it to exactly one original attempt/movement. | Treasury & Banking | External status does not directly replace Treasury semantics. |
| 8 | Compare bank evidence with Payment Execution, Settlement Execution, Bank Transaction, Cash Position, batch, and reconciliation history. | Treasury & Banking | Contradictions and required corrections are determined. |
| 9 | Start/continue reconciliation coordination and publish `ReconciliationExceptionDetected` for unresolved operational differences. | Treasury & Banking | Exception evidence and ownership are visible. |
| 10 | Determine the authoritative resolution path: confirm no execution; confirm execution awaiting/achieving settlement; record return/reversal; approve new retry attempt; cancel/abandon; or escalate unresolved. | Treasury & Banking / Enterprise Governance | Resolution is selected under owned facts and authority. |
| 11 | If retry is authorized, create a new immutable execution attempt linked to the original, revalidate approval, eligibility, restrictions, liquidity, account, bank, period, and version, then execute through `L3-F-057`. | Treasury & Banking | Original failure remains intact and duplicate risk is controlled. |
| 12 | If returned/reversed, register new Bank Transaction representation and settlement/failure facts; do not delete the original successful or failed fact. | Treasury & Banking | Return/reversal becomes an auditable new business fact. |
| 13 | Update Cash Position/Liquidity only after qualifying completed settlement/return/reversal behaviour and aggregate consistency; publish applicable source-supported events. | Treasury & Banking | Cash truth is corrected without premature mutation. |
| 14 | Publish `PaymentExecutionFailed`, `SettlementFailed`, `BankTransactionRegistered`, reconciliation, or completed Treasury events as applicable; no invented success event is used. | Treasury & Banking | Resolution chronology is immutable. |
| 15 | Hand off resolved facts to AP/AR for obligation consequences, GL for accounting, Reporting for presentation, and Audit/Governance for oversight. | Treasury & Banking / consumers | Each domain independently changes only its own state. |
| 16 | Close the resolution case only when classification, authority, cash/settlement/bank/reconciliation status, downstream hand-offs, version, and evidence are complete; otherwise retain escalated unresolved status. | Treasury & Banking / Governance | Final or governed unresolved outcome is reconstructable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Confirmed pre-execution failure | Bank never accepted/executed instruction. | Preserve failure; release/reschedule only through governed policy. | No settlement/cash change. | Treasury & Banking |
| Confirmed executed, not yet settled | Bank accepted execution but settlement finality is pending. | Retain in-progress/uncertain resolution and monitor; do not retry. | Await authoritative settlement. | Treasury / External Bank |
| Confirmed successful settlement | Later evidence proves completion. | Record/confirm settlement and Cash Position through source-supported behaviour. | Original uncertainty resolved as successful. | Treasury & Banking |
| Returned payment | Previously initiated/successful movement is returned. | Register return Bank Transaction, update settlement/cash through new facts, notify source owner. | Return resolved without history rewrite. | Treasury / External Bank |
| Bank reversal | External bank reverses prior entry. | Register reversal fact and coordinate cash/reconciliation/accounting consequences. | Reversal traceable to original. | Treasury / GL / Bank |
| Duplicate suspected | Two attempts/references may represent one or two movements. | Restrict further retry and reconcile authoritative bank evidence. | Duplicate risk contained. | Treasury / External Bank |
| Retry eligible | Confirmed failure and policy/approval permit retry. | Create a new linked attempt after full revalidation. | New outcome; old failure immutable. | Treasury / Governance |
| Retry not eligible | Policy, approval, period, account, liquidity, or risk blocks retry. | Cancel/defer/escalate and notify source owner. | No further movement. | Governance / Treasury |
| Evidence remains contradictory | Bank and Treasury facts cannot establish finality. | Keep escalated unresolved, preserve restriction, and continue investigation. | No false resolution. | Treasury / Bank / Audit |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Wrong transaction/correlation is investigated. | Correct payment may be blocked or incorrect one retried. | Reject linkage and reconstruct authoritative identity first. | Treasury & Banking |
| Bank evidence is stale/unverified. | Finality may be misclassified. | Retain unresolved status and obtain authoritative current evidence. | External Bank / Treasury |
| Blind retry occurs while outcome uncertain. | Duplicate cash movement may occur. | Contain/restrict, preserve both attempts, reconcile bank facts, and escalate. | Treasury / Governance |
| Concurrent aggregate version changes. | Resolution decision becomes stale. | Reload aggregate and re-evaluate all facts before modification. | Treasury & Banking |
| Partial correction occurs. | Execution, settlement, cash, or bank representations diverge. | Roll back, preserve prior version, and publish no false completed event. | Treasury & Banking |
| Resolution commits but event publication fails. | Upstream/GL/reconciliation consumers miss the result. | Recover event publication without repeating cash movement or correction. | Treasury & Banking |
| Cash updated for return/reversal without completed evidence. | Institutional cash becomes incorrect. | Reject/contain and apply only source-supported completed behaviour. | Treasury & Banking |
| Upstream obligation is changed directly by Treasury. | Ownership boundary is violated. | Reject change and route completed resolution facts to AP/AR owner. | Treasury / AP / AR |
| Accounting conclusion conflicts with Treasury/bank finality. | Financial and cash records diverge. | Preserve owned outcomes and route governed reconciliation; no cross-domain overwrite. | Treasury / GL / Audit |
| Late bank finality arrives after case closure. | Prior resolution may be invalidated. | Append new evidence, reopen resolution under governance, and preserve the earlier decision. | Treasury / Governance |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Treasury owns Payment Execution, Settlement Execution, Bank Transaction representations, Cash Position, and reconciliation coordination. | Treasury & Banking | Resolution changes only Treasury-owned state. | `L2-F10`, Scope and Ownership |
| `PaymentExecutionFailed` and `SettlementFailed` are immutable completed failure facts. | Treasury & Banking | Failure history cannot be overwritten by retry/success. | `L2-F10`, Payment/Settlement Events |
| Settlement completion finalizes cash movement; Cash Position updates only after successful qualifying completion. | Treasury & Banking | Uncertainty or execution alone cannot update cash. | `L2-F10`, Settlement/Cash Invariants |
| Every Payment Execution preserves immutable identifier, approved reference, aggregate association, and history. | Treasury & Banking | Each retry is a new linked attempt, not replacement. | `L2-F10`, Payment Invariants |
| Settlement Policy may consider retry eligibility, sequencing, priority, and timing. | Treasury & Banking | Retry requires policy and governance; it is not automatic. | `L2-F10`, Settlement Policy |
| Bank Transactions remain immutable, traceable, linked to Treasury activity, and retain reconciliation status. | Treasury & Banking | Returns/reversals become new representations. | `L2-F10`, Bank Transaction Invariants |
| External bank records are authoritative and never overwritten. | External Bank | Treasury translates and references bank evidence. | `L2-F10`, Banking Integration |
| Historical Treasury information and events remain immutable. | Treasury & Banking | Corrections append new facts only. | `L2-F10`, Historical/Event Constraints |
| Authorization/governance remain externally owned. | Governance / Authorization & Policy | Retry/cancellation/exception cannot self-authorize. | `L2-F10`, Governance Constraints |
| Treasury never creates Journal Entries or modifies external obligations. | GL / AP / AR | Resolution facts are hand-offs, not cross-domain commands. | `L2-F10`, Boundary Constraints |
| Successful modifications advance version; failures preserve version and history. | Treasury & Banking | Resolution is protected from conflicting simultaneous decisions. | `L2-F10`, Version/Failure Constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Resolution Case | No canonical model | Classify, investigate, decide, resolve, or escalate. | Resolved/unresolved as scenario outcome; exact lifecycle undefined | Owner unresolved / Treasury coordination | Canonical register; L2 gap |
| Payment Execution | Failed/successful/started/unknown through facts | Preserve original; optionally create linked retry. | Original unchanged; new attempt has own outcome | Treasury & Banking | `L2-F10`, Payment Events/Invariants |
| Settlement Execution | Failed/completed/in-progress/uncertain through facts | Confirm, fail, return/reverse through new owned facts. | Current authoritative settlement outcome | Treasury & Banking | `L2-F10`, Settlement Events |
| Bank Transaction representation | Existing/new external activity | Register return, reversal, correction, or late finality; maintain status. | Immutable appended bank fact/history | Treasury & Banking | `L2-F10`, Bank Transaction Management |
| Cash/Liquidity Position | Current authoritative | Update only after qualifying completed resolution behaviour. | Correct current position | Treasury & Banking | `L2-F10`, Cash/Liquidity Constraints |
| Upstream obligation | Externally owned | Consume/notify only. | Independently updated by owner | AP/AR/other owner | `L2-F10`, Ownership Matrix |

L2-F10 defines failure events but no returned/uncertain transaction or resolution state machines. This draft does not infer final states from external status codes.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PaymentExecutionFailed` | Treasury & Banking | AP/AR source, Notification, Audit, monitoring | Announces immutable execution failure. | `L2-F10`, Payment Events |
| `SettlementFailed` | Treasury & Banking | Source owner, GL, Reporting, Audit | Announces immutable settlement failure. | `L2-F10`, Settlement Events |
| `PaymentExecuted` / `SettlementCompleted` | Treasury & Banking | Source owner, GL, Reporting, Audit | May resolve prior uncertainty as completed facts; never overwrite old evidence. | `L2-F10`, Payment/Settlement Events |
| `BankTransactionRegistered` | Treasury & Banking | Reconciliation, GL, Reporting, Audit | Records return/reversal/late bank activity representation. | `L2-F10`, Banking Events |
| `ReconciliationExceptionDetected` | Treasury & Banking | Relevant owner, GL, Audit, Notification | Announces unresolved bank/Treasury contradiction. | `L2-F10`, Reconciliation Events |
| `ReconciliationCompleted` / `ReconciliationCoordinated` | Treasury & Banking | GL, Reporting, Audit | Announces completed coordination, not financial reconciliation ownership. | `L2-F10`, Reconciliation Events/Behaviour |
| `CashPositionUpdated` / `LiquidityPositionUpdated` | Treasury & Banking | GL, Reporting, dashboards, Audit | Announces completed position correction. | `L2-F10`, Cash/Liquidity Events |

No canonical BankTransactionReturned/Reversed/Uncertain/Disputed, ResolutionStarted/Completed/Escalated, RetryAuthorized, PaymentCancelled, OutcomeConfirmed, or LateFinalityReceived event exists.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| External Bank | Treasury & Banking | Transaction/attempt reference, status/finality, return/reversal reference, reason, amount, currency, account, value date, and evidence | Establishes authoritative bank-side outcome. | `L2-F10`, Banking Integration; exact contract incomplete |
| Treasury & Banking | Governance / Audit | Correlated execution, settlement, bank, cash, reconciliation, contradiction, risk, and proposed path | Supports exception/retry decision. | Canonical scenario; exact contract absent |
| Governance / Authorization & Policy | Treasury & Banking | Retry/cancel/escalate/restrict approval, authority, conditions, and evidence | Authorizes governed resolution behaviour. | `L2-F10`, Governance Integration; exact contract incomplete |
| Treasury & Banking | AP/AR/GL/Reporting | Final resolved/unresolved fact, original/new references, cash/settlement outcome, and correlation | Enables independent obligation/accounting/reporting response. | `L2-F10`, Event Integration; exact resolution contract incomplete |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Raise resolution case | No canonical actor defined | Must reference authoritative failure/return/uncertainty evidence. | Treasury context / source owner |
| Obtain/verify bank finality | No canonical actor defined | Must preserve bank authority and translation boundary. | Treasury / External Bank |
| Apply operational restriction | No canonical actor defined | Requires governed Treasury restriction behaviour. | Treasury / Enterprise Governance |
| Authorize retry/cancellation | No canonical role/hierarchy defined | Must resolve duplicate risk and preserve original attempt. | Enterprise Governance |
| Apply Cash Position correction | No canonical actor defined | Only after qualifying completed Treasury facts/invariants. | Treasury & Banking |
| Resolve upstream obligation/accounting | No canonical actors defined here | Each owner independently reacts to completed facts. | AP/AR / Accounting & GL |
| Close/escalate case | No canonical actor defined | Requires complete evidence or explicit governed unresolved disposition. | Treasury / Enterprise Governance |

## 15.2 Segregation of Duties

- Original execution, bank-finality investigation, retry approval, retry execution, cash correction, reconciliation, accounting correction, and audit review should remain independently governed; exact separation is absent.
- The original executor must not unilaterally classify unknown outcome or authorize blind retry.
- Bank evidence verifier should not be able to edit original bank facts.
- Treasury cannot directly restore/reopen an upstream Payable/Receivable or create GL corrections.
- Audit and Reporting remain read-only.

## 15.3 Mandatory Audit Evidence

- Institution, account, obligation/payment, execution attempt, batch, settlement, bank transaction, amount, currency, timing, and correlation.
- Original approval/eligibility, execution request, acknowledgement, events, versions, and state/history.
- Triggering failure/return/reversal/uncertainty/contradiction evidence and classification.
- Bank investigation requests/responses, authoritative source, finality, reason/reference, value date, and translated meaning.
- Duplicate/concurrent attempt checks, restrictions, risk, reconciliation, and Cash/Liquidity positions.
- Governance decision for retry, cancellation, restriction, escalation, or exception, including authority/conditions.
- New retry/corrective attempt identities and links to originals.
- Return/reversal Bank Transactions, settlement/cash corrections, prior/resulting positions, versions, and events.
- AP/AR/GL/Reporting hand-offs and independent outcome references.
- Interruptions, late evidence, re-opened cases, unresolved escalation, published-event outcome confirmation, and proof of no historical overwrite.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

| Measure | Business Meaning | Expected Direction / Acceptance Intent |
|---|---|---|
| Resolution traceability | Cases linked from original attempt through evidence and outcome | 100% |
| Historical mutation count | Original execution/settlement/bank facts overwritten/deleted | Zero |
| Duplicate movement caused by resolution | Resolution action creating duplicate cash movement | Zero |
| Cash correction integrity | Position changes backed by completed qualifying facts | 100% |
| Exception ownership completeness | Unresolved cases with explicit owner/escalation | 100% |
| Downstream hand-off completeness | Resolved cases communicated to affected owners | 100% |

No canonical uncertainty timeout, investigation SLA, retry waiting period/limit, escalation threshold, resolution materiality, bank-finality standard, or acceptance tolerance is defined upstream.

## 16.2 Postconditions

- Filename and Scenario ID match the frozen canonical register.
- Original failure, execution, settlement, bank, and reconciliation facts remain immutable.
- Uncertain outcomes prohibit blind retry until authoritative finality/authority exists.
- Every retry is a new linked attempt with fresh eligibility and authorization.
- Returns/reversals are new Bank Transaction/settlement/cash facts, not deletion of originals.
- Cash/Liquidity positions update only through qualifying completed Treasury behaviour.
- Resolution does not directly change AP/AR obligations or create Journal Entries.
- Resolved classification is supported by authoritative bank and Treasury evidence.
- Unresolved cases retain restriction/escalation/ownership rather than false closure.
- Events publish only after corresponding consistent aggregate facts succeed.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Gap | Architectural Impact | Required Resolution Owner |
|---|---|---|---|
| OI-F061-001 | No Bank Transaction Resolution aggregate/entity, lifecycle, owner, command, or events are defined. | Case initiation, investigation, decision, closure, reopening, and escalation cannot be standardized. | Treasury & Banking / Finance Architecture Governance |
| OI-F061-002 | Returned, reversed, uncertain, timed-out, disputed, duplicate-suspected, cancelled, and abandoned statuses are absent from canonical state models. | Cases may be classified inconsistently. | Treasury & Banking |
| OI-F061-003 | Payment Execution and Settlement Execution lack complete state machines and finality/transition guards. | Failure versus uncertainty versus late success cannot be mechanically validated. | Treasury & Banking |
| OI-F061-004 | Exact bank status/finality hierarchy, acknowledgement translation, return/reversal semantics, and evidence authority are undefined. | Bank outcomes may be misinterpreted. | Treasury / Banking Integration |
| OI-F061-005 | No resolution, return, reversal, uncertainty, retry-authorization, cancellation, late-finality, or escalated events exist. | Cross-domain visibility and audit chronology are incomplete. | Treasury & Banking |
| OI-F061-006 | Retry eligibility is illustrative only; permitted reattempt conditions, waiting periods, unknown-outcome prohibition, new authorization, and duplicate checks are undefined. | Resolution may create duplicate payments. | Treasury / Enterprise Governance |
| OI-F061-007 | Cash/Liquidity correction rules for return, reversal, late settlement, duplicate debit/credit, chargeback, and bank fee are incomplete. | Institutional cash may be misstated. | Treasury & Banking |
| OI-F061-008 | Relationship among execution, settlement, bank transaction, statement line, reconciliation case, and upstream obligation is incomplete. | End-to-end ownership/finality may diverge. | Treasury / AP / AR / GL Governance |
| OI-F061-009 | Closed-period treatment for late return/reversal/correction and interaction with L3-F-054 are undefined. | Corrective facts may violate period governance. | Financial Period & Closing / Accounting / Treasury |
| OI-F061-010 | Actors, bank liaison, investigator, retry approver, maker-checker, cash-correction authority, case closer, and segregation rules are absent. | Authorization and accountability cannot be fully accepted. | Enterprise Governance / Authorization & Policy |
| OI-F061-011 | Required operational restriction scope during uncertainty and release conditions are undefined. | Treasury may be over-restricted or remain exposed. | Treasury / Enterprise Governance |
| OI-F061-012 | Exact bank-evidence, resolution, governance, retry, correction, reconciliation, and downstream hand-off business contract content are incomplete. | Resolution cannot be mechanically verified. | Relevant contract owners |
| OI-F061-013 | Duplicate/simultaneous cases, late evidence, case reopening, repeat-request handling, stale versions, and published-event outcome confirmation are incomplete. | Multiple conflicting resolutions may be recorded. | Treasury & Banking |
| OI-F061-014 | Upstream Payable/Receivable reopen, retain, replace, or close reactions to each resolution outcome are undefined here. | Liability/receivable state may diverge from cash truth. | AP / AR Architecture Governance |
| OI-F061-015 | Retention, evidence access, bank correspondence, investigation notes, privacy/masking, and unresolved-case archival are undefined. | Audit/compliance evidence may be incomplete or overexposed. | Treasury / Audit / Compliance |
| OI-F061-016 | No canonical finality timeout, investigation SLA, reattempt-governance rule, escalation threshold, materiality, or acceptance tolerance exists. | Measures remain directional rather than fully testable. | Treasury / Enterprise Governance |

This scenario remains **Draft** until a resolution model, execution/settlement finality, returned/uncertain semantics, retry governance, Cash Position correction rules, cross-domain contracts, authorization, events, and measurable acceptance criteria are defined. Uncertainty is never converted into success, failure, or permission to retry without authoritative evidence.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-061**
