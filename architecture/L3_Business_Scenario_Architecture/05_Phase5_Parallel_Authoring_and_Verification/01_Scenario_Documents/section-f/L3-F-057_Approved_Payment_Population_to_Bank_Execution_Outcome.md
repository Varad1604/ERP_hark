# Approved Payment Population to Bank Execution Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-F-057  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** E, F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-B01_Institution_Workflow.md`; `L2-B02_Organisation_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; `L3-F-045_Approved_Payable_to_Financial_Settlement.md`; `L3-F-056_Bank_Account_Request_to_Active_or_Closed_Treasury_Account.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs the controlled conversion of an already approved population of financial payment obligations into scheduled and batched Treasury Payment Executions, external bank execution requests, and immutable per-payment execution outcomes. It preserves upstream obligation and approval ownership, Treasury ownership of cash execution, individual payment traceability within batches, liquidity and restriction controls, and the distinction between execution and final settlement.

---

# 3. Business Outcome

Every eligible approved payment in the population has a deterministic and auditable bank-execution outcome: successfully executed, failed, pending/unknown, excluded before execution, or routed for governed review according to authoritative facts. Successful execution publishes `PaymentExecuted`; failure publishes `PaymentExecutionFailed`. No payment is treated as settled and no Cash Position is changed until the separately governed settlement completion fact exists.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Treasury receives an authoritative population of approved payment obligations with immutable payment/obligation references, Institution and Organisation context, beneficiary/payment destination, amount, currency, approval evidence, requested execution timing, and correlation.

## 4.2 End Boundary

The scenario ends when every accepted population member has an immutable current execution outcome, including successful bank execution, failed execution, pending/unknown bank response, or governed non-execution disposition. Final settlement, Cash Position update, accounting recognition, and reconciliation continue in separately owned scenario stages.

## 4.3 In Scope

- Intake and correlation of an approved payment population.
- Verification that approval remains authoritative and externally owned.
- Validation of Institution, Organisation, obligation/payment reference, currency, beneficiary destination, amount, period governance, budget fact where required, Treasury status, account availability, liquidity, restrictions, and execution schedule.
- Detection of duplicates and already processed payment references.
- Scheduling of eligible payments and publication of `PaymentScheduled`.
- Creation and controlled maintenance of Payment Batches while preserving individual membership and traceability.
- Bank-execution initiation, progress monitoring, and acknowledgement translation.
- Recording of individual successful, failed, pending, or review-required execution outcomes.
- Preservation of partial/mixed batch outcomes without collapsing individual truth.
- Immutable events, versions, failure reasons, correlation, and audit evidence.

## 4.4 Out of Scope

- Creating, approving, rejecting, adjusting, reversing, or writing off the underlying payable, receivable, refund, grant, payroll, tax, or other obligation.
- Determining whether the obligation should exist or whether its business approval is valid in the originating domain.
- Opening or closing a bank account or Treasury Account.
- Treating scheduling as execution or execution as settlement completion.
- Updating Cash Position before successful settlement completion.
- Creating Journal Entries, modifying Ledgers, producing financial statements, or completing bank reconciliation.
- Defining external bank APIs, payment rails, files, credentials, encryption, message formats, protocols, or retry implementation.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-045` Approved Payable to Financial Settlement | Supplies approved Payable settlement eligibility without transferring liability ownership. |
| Preceded by | `L3-F-056` Bank Account Request to Active or Closed Treasury Account | Establishes the governed Treasury Account and Bank Account Reference. |
| Related | `L3-F-058` subsequent Treasury scenario | Expected to govern settlement/bank-result continuation according to the canonical sequence. |
| Related | `L3-F-025` Financial Transaction to Balanced Ledger Posting | General Ledger independently records completed financial facts. |
| Constrained by | `L3-F-052` Open Financial Period to Locked Close | Treasury respects period governance without owning period state. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Treasury & Banking / Section F | Solely owns Payment Execution, Payment Batch, Settlement Execution reference/state within Treasury, cash/liquidity positions, execution history, aggregate version, and Treasury events. |
| Accounts Payable / Section F | Owns vendor liability, Payable approval, due/payment eligibility facts, beneficiary business context, and payable settlement reaction. |
| Accounts Receivable / Section F | Owns customer receivable/refund obligation where a payment population member originates there. |
| Authorization & Policy / Section A | Owns authorization decisions consumed by Treasury. |
| Enterprise Governance | Owns approval hierarchy, exceptions, emergency controls, and governance outcomes; Treasury never becomes the business approver. |
| Budget Management | Owns budget availability/consumption facts where applicable. |
| Financial Period & Closing | Owns period lifecycle and restrictions consumed by Treasury. |
| External Banking Platform | Executes external banking operations and remains authoritative for original bank acknowledgements and transactions. |
| Accounting & General Ledger | Independently determines accounting treatment from completed Treasury facts and owns Journal/Ledger state. |
| Financial Reporting | Consumes completed facts for read-only reporting. |
| Audit | Reviews immutable approval, scheduling, batching, execution, failure, settlement, and correlation evidence. |
| Institution and Organisation Management | Own institutional and organisational context referenced by Treasury. |

Treasury governs cash execution. It does not approve liabilities, recognize obligations, allocate budgets, govern financial periods, or create accounting entries.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F10` defines no canonical actors for submitting a payment population, validating eligibility, scheduling payments, creating or modifying batches, initiating bank execution, monitoring acknowledgements, reviewing failures, authorizing retry, or approving exceptions. Upstream approval remains external, but its canonical actor and evidence contract are also incomplete. No operational role is invented; gaps are recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- One active, governed Treasury Aggregate exists for the Institution.
- Eligible Treasury Account and Bank Account Reference information is available under `L3-F-056`, subject to its unresolved account-state gaps.
- Every population member possesses one immutable payment/obligation reference and references one approved business obligation.
- Approval originated outside Treasury and remains traceable and current.
- Institution, Organisation where applicable, beneficiary/payment destination, amount, currency, due/requested date, and correlation are available.
- Financial Period, budget, and other upstream eligibility facts can be evaluated where applicable.
- Current Cash Position, Liquidity Position, reserved liquidity, Treasury restrictions, execution schedule, settlement policy, and banking availability can be evaluated.
- Duplicate references, prior executions, active batch membership, and conflicting instructions can be detected.
- External banking acknowledgements can be translated without leaking banking-platform semantics into Treasury.
- Failed operations can preserve aggregate consistency, version, and historical truth.

---

# 8. Trigger

Treasury receives a governed population of payment obligations that upstream domains have already approved for cash execution, or the scheduled execution window arrives for previously accepted payments.

The exact canonical approved-population event, population owner, approval contract, readiness event, and execution-request trigger are not defined in `L2-F10`.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the approved payment population with population identity, Institution, Organisation context, member references, amounts, currencies, beneficiaries, requested dates, approval evidence, and correlation. | Upstream obligation owner / Treasury & Banking | A traceable execution candidate population is established. |
| 2 | Validate population completeness, tenant isolation, source ownership, approval traceability, and immutable individual payment references. | Treasury & Banking / Authorization & Policy | Invalid or unapproved members do not enter execution. |
| 3 | Detect duplicates, previously executed/settled members, conflicting instructions, and membership in another active Payment Batch. | Treasury & Banking | Duplicate cash movement and conflicting batch membership are prevented. |
| 4 | Evaluate each member's current payment readiness, period status, budget fact where applicable, beneficiary/payment destination, amount, currency, Treasury Account, Bank Account Reference, and banking availability. | Treasury & Banking / source-domain owners | Member-level business eligibility is established without taking ownership of source obligations. |
| 5 | Evaluate Treasury operational status, restrictions, current and reserved liquidity, payment execution policy, execution window, settlement priority, and timing. | Treasury & Banking | Treasury-owned execution eligibility is established. |
| 6 | Classify each population member as eligible, excluded, pending, blocked, or requires review and preserve the authoritative reason. | Treasury & Banking | The population is deterministically partitioned without losing individual traceability. |
| 7 | Schedule eligible payments for their permitted execution window, create Payment Execution identities, advance version, and publish `PaymentScheduled` after success. | Treasury & Banking | Scheduling becomes an immutable fact and does not imply execution. |
| 8 | Create one or more Payment Batches according to compatible execution characteristics and policy, preserving immutable batch identity and individual payment membership. | Treasury & Banking | Grouped execution is prepared without merging payment identities. |
| 9 | Publish `PaymentBatchCreated`; publish `PaymentBatchUpdated` only after a governed membership change succeeds. | Treasury & Banking | Batch composition and history are auditable. |
| 10 | Revalidate payment approval/readiness, restrictions, liquidity, account/bank availability, schedule, batch membership, version, and invariants immediately before execution initiation. | Treasury & Banking | Stale or newly blocked members do not execute. |
| 11 | Begin each eligible Payment Execution and publish `PaymentExecutionStarted` after the aggregate records the start fact. | Treasury & Banking | The execution attempt is independently identifiable and traceable. |
| 12 | Issue the external banking execution request/instruction through the governed banking relationship using the individual payment, batch, bank, and correlation references. | Treasury & Banking / External Banking Platform | The bank receives an execution instruction without acquiring Treasury ownership. |
| 13 | Translate the external acknowledgement/outcome into Treasury business meaning and correlate it to exactly one execution attempt and payment reference. | Treasury & Banking | External semantics are isolated and authoritative bank evidence is preserved. |
| 14 | On successful execution, record execution status/time/reference, advance version, preserve history, and publish `PaymentExecuted`. | Treasury & Banking | A completed execution fact exists, but settlement is not yet implied. |
| 15 | On execution failure, preserve failure reason and bank evidence, advance version for the completed failure fact, and publish `PaymentExecutionFailed`. | Treasury & Banking | Failure is immutable and no successful cash movement is asserted. |
| 16 | For pending, unknown, timed-out, or mixed responses, retain each member's last authoritative status and prohibit blind re-execution until outcome ambiguity is governed. | Treasury & Banking / External Banking Platform | Duplicate payment risk is contained. |
| 17 | Summarize batch outcome from member facts without replacing them and hand successful executions to settlement management; route failures/pending members to governed resolution. | Treasury & Banking | Every member retains a deterministic bank-execution outcome and downstream ownership is clear. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Population partly eligible | Some members fail readiness or policy checks. | Schedule/batch only eligible members and preserve excluded/member-specific reasons. | Valid members proceed without falsifying population-wide success. | Treasury & Banking |
| Insufficient liquidity | Available liquidity cannot support all eligible members. | Apply configured priority/sequencing, retain blocked members, and publish no execution success for them. | Treasury & Banking |
| Treasury restricted/suspended | Operational governance prohibits execution. | Keep payments scheduled/pending or blocked under policy and preserve restriction evidence. | Treasury & Banking / Governance |
| Execution window not reached | Payment is eligible but scheduled for later. | Retain scheduled state and revalidate at the permitted time. | Treasury & Banking |
| Bank unavailable | Banking relationship or platform cannot accept execution. | Retain last valid status and reschedule/review only under governed policy. | Treasury & Banking / External Bank |
| Mixed batch outcome | Some members succeed, fail, or remain pending. | Record each outcome independently; do not mark the whole batch successfully settled. | Treasury & Banking |
| Bank rejects member | External validation rejects one payment. | Record immutable failure reason and route correction to the authoritative owner. | External Bank / Treasury & Banking / source owner |
| Acknowledgement unknown | Request may have reached bank but definitive outcome is absent. | Hold member in governed uncertainty; investigate before retry to prevent duplicate payment. | Treasury & Banking / External Bank |
| Upstream approval withdrawn | Authoritative owner withdraws eligibility before execution begins. | Exclude/cancel under governed policy; preserve both original approval and later fact. | Upstream owner / Treasury & Banking |
| Batch membership changes | Eligible member is added/removed before execution. | Validate change, preserve history, advance version, and publish `PaymentBatchUpdated`. | Treasury & Banking |
| Retry permitted | A failed execution meets settlement/payment policy retry eligibility. | Create a traceable new attempt linked to the original; never rewrite the failed fact. | Treasury & Banking / Governance |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Approval or obligation reference is missing/invalid. | Treasury may execute an unauthorized payment. | Reject the member and obtain an authoritative corrected eligibility fact. | Upstream owner / Treasury & Banking |
| Duplicate population/member/reference arrives. | Same obligation could be paid twice. | Identify prior state and return/reuse the authoritative outcome without creating duplicate execution. | Treasury & Banking |
| One payment belongs to multiple active batches. | Execution ordering and duplicate prevention fail. | Reject conflicting membership and restore one authoritative batch association. | Treasury & Banking |
| Liquidity changes after scheduling. | Scheduled population may no longer be executable. | Revalidate immediately before execution and block/resequence affected members. | Treasury & Banking |
| Aggregate version changes concurrently. | Scheduling, batching, or execution decision is stale. | Reload aggregate, re-evaluate every guard, and submit a new attempt. | Treasury & Banking |
| Partial aggregate modification occurs. | Batch, execution, liquidity, or history may become inconsistent. | Roll back, preserve prior version, and publish no false event. | Treasury & Banking |
| Execution starts but bank request delivery is uncertain. | Blind retry may duplicate cash movement. | Preserve attempt/correlation, obtain authoritative bank outcome, and prohibit re-execution until governed. | Treasury & Banking / External Bank |
| Success is recorded but event publication fails. | Settlement/accounting consumers may not receive execution fact. | Recover publication from committed state without executing again. | Treasury & Banking |
| Bank success later conflicts with Treasury failure. | Cash movement truth is contradictory. | Preserve both facts, route controlled investigation/reconciliation, and do not overwrite history. | Treasury & Banking / External Bank / Audit |
| Member succeeds but batch processing stops. | Remaining members lack outcomes. | Retain successful member fact and safely resume only unresolved members after current-state validation. | Treasury & Banking |
| Cash Position changes on execution alone. | Cash may be misstated before settlement. | Reject/contain the change and allow Cash Position update only after `SettlementCompleted`. | Treasury & Banking |
| External semantics cannot be translated reliably. | Bank acknowledgement may be misclassified. | Hold outcome as unresolved and correct the canonical business translation before progression. | Treasury & Banking / Banking Integration |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Treasury owns institutional cash movement, Payment Execution, Payment Batch, and Settlement Execution. | Treasury & Banking | No upstream domain may directly change Treasury execution state. | `L2-F10`, Purpose, Scope, and Aggregate Ownership |
| Treasury executes approved payments but never approves liabilities or obligations. | Upstream obligation/governance owners | Every execution must reference an externally approved obligation. | `L2-F10`, Core Principle and Payment Invariants |
| Every Payment Execution has one immutable identifier, one approved obligation reference, one Treasury Aggregate, and preserved execution history. | Treasury & Banking | Individual identity survives scheduling, batching, retries, and outcomes. | `L2-F10`, Payment Invariants |
| A payment cannot belong to multiple active batches simultaneously. | Treasury & Banking | Duplicate/conflicting grouped execution is prohibited. | `L2-F10`, Batch Invariants |
| Payment Execution Policy evaluates readiness, Treasury restrictions, liquidity availability, and operational schedule. | Treasury & Banking | Every member is revalidated before execution. | `L2-F10`, Payment Execution Policy |
| Settlement Policy governs sequencing, priority, timing, and retry eligibility. | Treasury & Banking | Batch ordering and retry remain policy-driven. | `L2-F10`, Settlement Policy |
| Scheduling does not imply execution. | Treasury & Banking | `PaymentScheduled` cannot be treated as bank movement. | `L2-F10`, Payment Scheduling and Events |
| `PaymentExecuted` does not necessarily imply settlement completion. | Treasury & Banking | Execution success cannot directly finalize cash movement. | `L2-F10`, Execute Payment behaviour and Payment Events |
| Settlement completion finalizes cash movement; Cash Position updates only after successful settlement. | Treasury & Banking | No cash update occurs from scheduling or execution alone. | `L2-F10`, Settlement Invariants and Events |
| External banking acknowledgements must be translated through the anti-corruption boundary. | Treasury & Banking / external bank | External status codes cannot directly mutate aggregate state. | `L2-F10`, External Banking Integration |
| Every successful aggregate modification advances version; failed operations preserve version. | Treasury & Banking | Business-version checks prevent payment and batch decisions from using superseded facts. | `L2-F10`, Aggregate Versioning and Failure Constraints |
| Events represent completed immutable facts and publish after consistency. | Treasury & Banking | Failed or partial operations cannot publish success. | `L2-F10`, Event and Aggregate Constraints |
| Treasury never creates Journal Entries; GL independently determines accounting treatment. | Accounting & General Ledger | Execution events are inputs, not accounting state changes. | `L2-F10`, GL Integration |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Approved obligation/payment reference | Upstream-approved | Consumed as immutable execution eligibility; never modified. | Remains externally owned | AP/AR/other obligation owner | `L2-F10`, Integration Matrix |
| Payment Execution | No complete canonical state enumeration | Schedule, start, execute successfully, fail, or retain unresolved status. | Status represented by completed events and execution fields | Treasury & Banking | `L2-F10`, Payment Execution entity/events |
| Payment Batch | Created/current lifecycle | Establish membership, preserve batch integrity, coordinate execution, summarize member facts. | Current batch lifecycle position; exact states undefined | Treasury & Banking | `L2-F10`, Payment Batch Management |
| Settlement Execution | Not completed | Successful payment execution becomes eligible for settlement continuation. | Settlement not yet complete | Treasury & Banking | `L2-F10`, Settlement Management |
| Cash Position | Current authoritative position | No change from scheduling or execution alone. | Unchanged until successful settlement | Treasury & Banking | `L2-F10`, Settlement Invariants |
| Treasury Aggregate | `Operational` under illustrative aggregate lifecycle | Enclose consistent payment/batch changes and advance version. | Remains Operational unless separately restricted/suspended | Treasury & Banking | `L2-F10`, Aggregate Lifecycle |

`L2-F10` defines event milestones but no complete Payment Execution or Payment Batch state machine. This scenario does not infer hidden states from event names.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Approved obligation/payment eligibility fact | AP, AR, or other canonical owner | Treasury & Banking | Authorizes consideration for execution without transferring obligation ownership. | `L2-F10`, Upstream Relationships; exact event undefined |
| `PaymentScheduled` | Treasury & Banking | Treasury monitoring, Audit, source owner as configured | Announces scheduled execution; not execution. | `L2-F10`, Payment Events |
| `PaymentBatchCreated` | Treasury & Banking | Treasury monitoring, Audit | Announces immutable batch formation. | `L2-F10`, Payment Events |
| `PaymentBatchUpdated` | Treasury & Banking | Treasury monitoring, Audit | Announces a completed governed batch change. | `L2-F10`, Payment Events |
| `PaymentExecutionStarted` | Treasury & Banking | Treasury monitoring, Audit | Announces beginning of a specific execution attempt. | `L2-F10`, Payment Events |
| `PaymentExecuted` | Treasury & Banking | Settlement Management, GL, source domain, Reporting, Audit | Announces successful execution, not necessarily settlement. | `L2-F10`, Payment Events and GL Integration |
| `PaymentExecutionFailed` | Treasury & Banking | Source owner, Treasury monitoring, Notification, Audit | Announces immutable execution failure. | `L2-F10`, Payment Events |
| `SettlementStarted` / `SettlementCompleted` / `SettlementFailed` | Treasury & Banking | GL, source owner, Reporting, Audit | Continue successful execution toward final cash movement. | `L2-F10`, Settlement Events |
| `CashPositionUpdated` | Treasury & Banking | GL, Reporting, dashboards, Audit | Announces cash change only after settlement completion. | `L2-F10`, Settlement behaviour |

No canonical approved-payment-population, payment-ready, member-excluded, batch-execution-completed, bank-acknowledgement-received, execution-pending/unknown, retry-authorized, or execution-cancelled event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| AP/AR/other obligation owner | Treasury & Banking | Approved obligation/payment reference, amount, currency, beneficiary, timing, approval, eligibility, and correlation | Supplies execution candidates without transferring obligation ownership. | `L2-F10`, Upstream Relationships; exact contract incomplete |
| Authorization & Policy / Financial Period / Budget | Treasury & Banking | Current authorization, period, and budget outcomes where applicable | Supports execution eligibility under independent ownership. | `L2-F10`, Integration Architecture; exact contract incomplete |
| Treasury & Banking | External Banking Platform | Payment execution request/settlement instruction with Treasury, account, payment, batch, amount, currency, beneficiary, and correlation | Requests banking execution without exposing aggregate internals. | `L2-F10`, External Banking Platforms; exact contract incomplete |
| External Banking Platform | Treasury & Banking | Authoritative acknowledgement, outcome, bank reference, reason, status sequence, and timing | Enables canonical execution outcome translation. | `L2-F10`, Banking Integration; exact contract incomplete |
| Treasury & Banking | GL / source owner / Reporting / Audit | Payment execution success/failure and correlation | Enables independent settlement, accounting, liability, reporting, and audit reactions. | `L2-F10`, Event Consumer Mapping |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Approve underlying obligation/payment | No canonical role defined in L2-F10 | Must occur upstream; Treasury cannot self-approve. | AP/AR/other owner / Enterprise Governance |
| Submit approved population | No canonical actor defined | Must supply immutable approval and obligation references. | Upstream owner |
| Validate/schedule payment | No canonical actor defined | Requires payment readiness, restrictions, liquidity, and schedule policy. | Treasury & Banking |
| Create/modify Payment Batch | No canonical actor defined | Must preserve membership, identity, history, and no multiple active batches. | Treasury & Banking |
| Initiate bank execution | No canonical actor defined | Requires fresh eligibility, authority, account, bank, liquidity, and version checks. | Treasury & Banking |
| Authorize retry/exception | No canonical role/hierarchy defined | Must preserve original failed/unknown outcome and prevent duplicate movement. | Enterprise Governance / Treasury & Banking |
| Review execution evidence | No canonical reviewer defined | Read-only review cannot alter completed facts. | Audit |

## 15.2 Segregation of Duties

- Obligation creation, business approval, payment-population submission, Treasury scheduling, batch preparation, bank execution initiation, exception/retry authorization, and audit review should remain independently governed; exact mandatory separation is undefined.
- Treasury must never create or approve the source obligation it executes.
- Batch grouping cannot remove individual approval or payment traceability.
- An operator cannot resolve an unknown bank outcome by blind re-execution.
- GL, Reporting, and Audit consume events without changing Treasury execution state.

## 15.3 Mandatory Audit Evidence

- Population identity/source, Institution, Organisation, population timestamp/sequence, and member count/value by currency.
- Each obligation/payment reference, owner, beneficiary destination, amount, currency, due/requested date, approval, and correlation.
- Duplicate/prior-execution/batch-membership checks and results.
- Readiness, Financial Period, Budget, Treasury restriction, liquidity, account, banking availability, schedule, priority, and policy evaluations.
- Payment Execution and Payment Batch identifiers, membership changes, versions, scheduling facts, and events.
- Execution start, external request, acknowledgement, bank reference, outcome, failure reason, timestamps/sequence, and translation evidence.
- Successful, failed, pending, excluded, blocked, review, retry, and cancellation evidence for every member.
- Settlement hand-off and proof that execution alone did not update Cash Position.
- Simultaneous-change conflicts, interruptions, partial batch outcomes, publication failures, investigation, and recovery actions.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

| Measure | Business Meaning | Expected Direction / Acceptance Intent |
|---|---|---|
| Approved-payment execution rate | Executed members backed by authoritative upstream approval | 100% |
| Duplicate execution count | Same approved obligation executed more than permitted | Zero |
| Member traceability rate | Population members traceable through schedule, batch, attempt, bank outcome, and event | 100% |
| Outcome completeness rate | Accepted members with explicit current outcome | 100% |
| Unauthorized/unapproved execution count | Treasury executions without valid approval | Zero |
| Multiple-active-batch violations | Payment present in more than one active batch | Zero |
| Execution-to-premature-cash-update count | Cash updated before settlement completion | Zero |

No canonical execution SLA, bank-acknowledgement timeout, batch size, liquidity threshold, priority matrix, reattempt-governance rule, materiality threshold, or tolerance is defined upstream.

## 16.2 Postconditions

- Filename and Scenario ID match the frozen canonical register.
- Every executed payment references one approved business obligation and one Treasury Aggregate.
- Population members retain individual immutable identity and outcome even within a batch.
- No payment belongs to multiple active batches simultaneously.
- Scheduling, execution, settlement, and Cash Position update are treated as distinct completed facts.
- `PaymentExecuted` and `PaymentExecutionFailed` publish only after the corresponding aggregate outcome succeeds.
- Mixed batch outcomes do not overwrite member-level truth.
- Pending/unknown bank outcomes block unsafe re-execution.
- Successful executions hand off to settlement; Cash Position remains unchanged until `SettlementCompleted`.
- Treasury does not approve obligations, create Journal Entries, or modify upstream liabilities.
- Failed operations preserve prior valid state, version, history, and audit evidence.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Gap | Architectural Impact | Required Resolution Owner |
|---|---|---|---|
| OI-F057-001 | No canonical approved-payment-population event, owner, aggregate, intake state, or contract exists. | Population completeness and authoritative hand-off cannot be fully verified. | AP/AR/Treasury / Finance Architecture Governance |
| OI-F057-002 | Payment Execution has status fields and events but no explicit canonical state machine or transition guards. | Scheduling, started, executed, failed, pending, cancelled, and retry semantics may diverge. | Treasury & Banking |
| OI-F057-003 | Payment Batch lifecycle states, batch-level completion rules, mixed-outcome semantics, cancellation, and closure are undefined. | Batch status may obscure individual outcomes or remain indeterminate. | Treasury & Banking |
| OI-F057-004 | Exact upstream approval event, validity, withdrawal, amendment, expiry, and revalidation contract are absent. | Treasury may act on stale or ambiguous approval. | Upstream obligation owners / Enterprise Governance |
| OI-F057-005 | Bank acknowledgement statuses, canonical translation, finality, duplicate reference rules, and authoritative outcome hierarchy are undefined. | Successful, failed, and unknown outcomes may be misclassified. | Treasury & Banking / Banking Integration |
| OI-F057-006 | No canonical event exists for payment readiness, exclusion, blocked/review status, bank acknowledgement, pending/unknown outcome, cancellation, or retry authorization. | Cross-domain visibility and audit chronology are incomplete. | Treasury & Banking |
| OI-F057-007 | Relationship among Payment Execution, Settlement Execution, external bank execution, and bank transaction is not fully specified. | Execution may be confused with settlement or bank booking. | Treasury & Banking |
| OI-F057-008 | Liquidity reservation, release, reallocation, multi-currency treatment, and insufficient-liquidity sequencing rules are incomplete. | Concurrent batches may overcommit cash or apply inconsistent priority. | Treasury & Banking |
| OI-F057-009 | Batch grouping compatibility rules for bank account, currency, payment rail, date, priority, beneficiary, Institution, and restrictions are undefined. | Invalid or operationally incompatible batches may form. | Treasury & Banking |
| OI-F057-010 | Retry eligibility is illustrative, but attempt identity, permitted reattempt conditions, waiting rules, unknown-outcome prohibition, and approval authority are undefined. | Retrying may create duplicate payments. | Treasury & Banking / Enterprise Governance |
| OI-F057-011 | Actors, execution authority, batching authority, maker-checker controls, bank signatory/mandate rules, and segregation are absent. | Authorization and operational accountability cannot be fully accepted. | Enterprise Governance / Authorization & Policy |
| OI-F057-012 | Payment destination/beneficiary verification ownership and interaction with vendor/customer master data are undefined. | Payments may be executed to stale or unauthorized destinations. | AP/AR / Identity or Master Data Governance |
| OI-F057-013 | Financial Period and Budget facts are referenced by Treasury architecture, but mandatory execution-time applicability and stale-fact handling are incomplete. | Period/budget governance may be inconsistently enforced. | Treasury / Financial Period / Budget owners |
| OI-F057-014 | Partial submission, delivery uncertainty, late acknowledgements, simultaneous execution decisions, repeat-request handling, and recovery rules are incomplete. | Duplicate or missing execution facts may occur. | Treasury & Banking / Banking Integration |
| OI-F057-015 | Exact cross-domain business contract content, identifiers, event identity and chronology, ordering, correlation, and evidence requirements are incomplete. | End-to-end traceability cannot be mechanically validated. | Relevant contract owners |
| OI-F057-016 | No canonical SLA, acknowledgement timeout, batch limit, liquidity threshold, priority rule, reattempt-governance rule, or acceptance tolerance exists. | Measures remain directional rather than fully testable. | Treasury & Banking / Enterprise Governance |

This scenario remains **Draft** until payment and batch state machines, approved-population ownership, banking outcome semantics, liquidity reservation, retry governance, authorization, contracts, and measurable acceptance criteria are defined. It does not treat scheduling as execution, execution as settlement, or a mixed batch as a single uniform outcome.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-057**
