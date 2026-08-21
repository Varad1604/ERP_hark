# Authorized Fund Transfer to Settled Inter-Account Movement

# 1. Scenario Metadata

**Scenario ID:** L3-F-060  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-B01_Institution_Workflow.md`; `L2-B02_Organisation_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; `L3-F-056_Bank_Account_Request_to_Active_or_Closed_Treasury_Account.md`; `L3-F-057_Approved_Payment_Population_to_Bank_Execution_Outcome.md`; `L3-F-058_Bank_Statement_to_Reconciled_Cash_Position.md`; `L3-F-059_Treasury_Demand_to_Current_Cash_Position_and_Liquidity_Decision.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs an authorized movement of institutional funds between two governed Treasury Account/Bank Account relationships, from request and independent authorization through Treasury execution, external bank outcome, settlement completion, Cash Position impact, and auditable inter-account traceability. It prevents a transfer from being treated as an external expense, revenue, liability settlement, implicit currency conversion, or unilateral cash change.

`L2-F10` defines Payment Execution, Settlement Execution, Treasury Accounts, Bank Transactions, Cash Position, and Liquidity Position, but it does not define a dedicated Fund Transfer aggregate, capability, lifecycle, command, or event. This draft therefore uses only source-supported Treasury concepts and records the missing transfer architecture in Section 17.

---

# 3. Business Outcome

An authorized fund-transfer request is either rejected/blocked without cash movement, or executed and settled as one traceable institutional movement linking an eligible source account, eligible destination account, amount, currency, authorization, outgoing and incoming banking evidence, Settlement Execution, Cash Position effects, and immutable events. The same movement is not duplicated as income/expense, and Accounting independently determines any required accounting treatment.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an institutional need is raised to move a specified amount and currency between two identifiable governed Treasury Accounts/Bank Account References, supported by purpose, requested timing, authorization context, and correlation.

## 4.2 End Boundary

The scenario ends when the request is rejected/blocked; or when both sides of the transfer have a governed terminal outcome, successful settlement is confirmed, related Bank Transaction representations are registered/reconciled, Cash Position and Liquidity Position remain correct, and all events/evidence are linked. Unknown or partial outcomes remain unresolved and cannot be declared settled.

## 4.3 In Scope

- Capture and validation of source account, destination account, Institution, Organisation context, amount, currency, purpose, requested timing, and correlation.
- Verification that source and destination are distinct, governed, available, and permitted for the requested transfer.
- Authorization, policy, approval hierarchy, restrictions, liquidity, reserve, and Financial Period evaluation.
- Duplicate/conflicting request detection.
- Treasury-owned execution and Settlement Execution tracking using source-supported behaviour.
- External banking request and acknowledgement translation.
- Preservation of outgoing and incoming bank evidence as one correlated movement.
- Cash Position/Liquidity Position update only after successful settlement completion.
- Bank Transaction registration and reconciliation coordination.
- Immutable versions, events, failure facts, history, and audit evidence.

## 4.4 Out of Scope

- Creating an expense, revenue, Payable, Receivable, budget allocation, or financial obligation.
- Treating the transfer as a vendor/customer payment unless a separately owned obligation exists.
- Opening/closing physical bank accounts or modifying bank-owned records.
- Implicit currency conversion, foreign exchange dealing, or rate determination.
- Creating Journal Entries or changing Ledger balances.
- Defining payment rails, bank APIs, file formats, credentials, storage, or retry implementation.
- Inventing a Fund Transfer aggregate or events absent from locked L2-F10.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-056` Bank Account Request to Active or Closed Treasury Account | Establishes both governed account relationships. |
| Related | `L3-F-057` Approved Payment Population to Bank Execution Outcome | Supplies source-supported Payment Execution behaviour but not transfer semantics. |
| Related | `L3-F-058` Bank Statement to Reconciled Cash Position | Reconciles outgoing and incoming bank sides of the movement. |
| Constrained by | `L3-F-059` Treasury Demand to Current Cash Position and Liquidity Decision | Determines source liquidity supportability before execution. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Treasury & Banking / Section F | Owns Treasury Accounts, Cash/Liquidity Positions, Payment/Settlement Executions, Bank Transaction representations, execution history, aggregate version, and Treasury events. |
| Authorization & Policy / Enterprise Governance | Own authorization, approval hierarchy, restrictions, exceptions, emergency controls, and immutable governance outcomes. |
| Institution and Organisation Management | Own source/destination institutional and organisational context. |
| External Banking Platform | Owns physical account activity, banking execution, acknowledgements, references, and original transactions. |
| Financial Period & Closing | Owns period lifecycle/restrictions consumed by Treasury. |
| Accounting & General Ledger | Owns any transfer accounting classification, Journals, and Ledger balances. |
| Financial Reporting | Consumes completed movement/cash facts for read-only reporting without treating transfer as income/expense by default. |
| Audit | Reviews immutable request, authority, execution, settlement, cash, bank, and reconciliation evidence. |

Treasury owns cash movement, not governance authority, bank infrastructure, or accounting entries. Both accounts remain within their established ownership and Institution boundaries.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F10` defines no canonical transfer requester, verifier, approver, maker, checker, bank signatory, executor, settlement reviewer, reconciler, or exception authority. It also contains no actor model for inter-account transfers. No roles are invented; required authority and segregation gaps are listed in Section 17.

---

# 7. Preconditions and Entry Criteria

- One operational Treasury Aggregate exists for the Institution.
- Source and destination Treasury Accounts and Bank Account References are identifiable, governed, available, and not the same account.
- Both accounts have authoritative Institution, currency, relationship status, operational status, and version information.
- The request identifies amount, currency, purpose, timing, requester context, authorization evidence, and immutable correlation.
- Source Cash Position, available/reserved/restricted cash, Liquidity Position, restrictions, and scheduled activity can be evaluated.
- Financial Period and institutional policy outcomes can be consumed where applicable.
- Duplicate, reversed-direction, previously executed, and simultaneous requests can be detected.
- The external bank can return authoritative acknowledgement/outcome facts through canonical translation.
- Accounting and reconciliation consumers can independently react to completed facts.
- No implicit currency conversion or cross-Institution ownership transfer is assumed.

---

# 8. Trigger

An authorized institutional treasury need requires funds to be repositioned between two governed Treasury Account/banking relationships for liquidity, operational, reserve, settlement, or cash-management purposes.

No canonical FundTransferRequested/Authorized event, transfer request model, or dedicated transfer trigger is defined in `L2-F10`.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Capture the transfer request with Institution, source/destination account references, amount, currency, purpose, timing, authorization context, and correlation. | Requesting institutional function / Treasury & Banking | A traceable transfer intent exists. |
| 2 | Validate tenant/Institution isolation, source/destination distinction, account identity, currency, relationship availability, operational status, purpose, and completeness. | Treasury & Banking | Invalid account or scope combinations are rejected. |
| 3 | Detect duplicate, reverse-duplicate, already executed/settled, conflicting, or simultaneous transfer requests. | Treasury & Banking | Duplicate cash movement is prevented. |
| 4 | Obtain externally owned authorization/approval outcome and validate authority, policy, segregation, effective timing, and conditions. | Authorization & Policy / Enterprise Governance | Transfer cannot self-authorize within Treasury. |
| 5 | Evaluate source current/available/reserved/restricted Cash Position, Liquidity Position, Treasury restrictions, scheduled activity, threshold rules, and required source balance. | Treasury & Banking | Source supportability is established. |
| 6 | Evaluate destination account eligibility, banking relationship, restrictions, currency compatibility, receipt availability, and institutional policy. | Treasury & Banking | Destination can receive the movement under policy. |
| 7 | Validate that the movement will not create an unauthorized negative liquidity position, consume protected funds, cross prohibited boundaries, or perform implicit currency conversion. | Treasury & Banking | Cash and governance invariants are protected. |
| 8 | Establish one immutable movement correlation and source-supported Payment/Settlement Execution identities linking both sides without inventing a new L2 aggregate. | Treasury & Banking | The movement is end-to-end traceable. |
| 9 | Begin execution, preserve attempt history/version, publish only source-supported execution events, and issue the external banking instruction. | Treasury & Banking / External Bank | Bank execution is requested under governed authority. |
| 10 | Translate bank acknowledgement and correlate it to exactly one request/attempt; preserve pending, failed, successful, or unknown outcome without overwriting prior facts. | Treasury & Banking | Bank outcome is represented canonically. |
| 11 | On failed/unknown execution, preserve immutable failure/uncertainty evidence and prohibit blind retry until duplicate risk is resolved. | Treasury & Banking / Governance | No false settled movement is declared. |
| 12 | On execution success, continue Settlement Execution and verify the authoritative outgoing and incoming movement evidence according to settlement policy. | Treasury & Banking | Execution progresses toward final cash movement. |
| 13 | Complete settlement only when source-supported settlement prerequisites succeed; publish `SettlementCompleted` and applicable `CashPositionUpdated` after aggregate consistency. | Treasury & Banking | The movement becomes a completed cash fact. |
| 14 | Update/confirm source and destination Cash/Liquidity positions as a balanced institutional movement, preserving account-level traceability and aggregate invariants. | Treasury & Banking | No unexplained creation or destruction of institutional cash occurs, subject to external charges. |
| 15 | Register outgoing/incoming Bank Transaction representations and coordinate reconciliation through `L3-F-058`. | Treasury & Banking | External bank evidence is linked to the settled movement. |
| 16 | Publish completed facts to Accounting, Reporting, Audit, and other consumers; Accounting independently determines accounting treatment. | Treasury & Banking / consumers | Movement is auditable without Treasury creating Journal Entries. |

The dedicated transfer state and event names required to make this flow fully canonical are absent from L2-F10 and remain unresolved in Section 17.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Authorization rejected | Approval/authority fails. | Preserve rejection and perform no execution. | No cash movement. | Governance / Treasury |
| Insufficient source liquidity | Available cash cannot support transfer and reserves. | Block/defer or route exception review under policy. | No overdraft unless explicitly permitted. | Treasury & Banking |
| Source/destination unavailable | Account or banking relationship is restricted, suspended, closed, or unavailable. | Block transfer and resolve account status through its owner. | Treasury & Banking / Bank |
| Same-account request | Source and destination are identical. | Reject as invalid/no-op and preserve evidence. | Treasury & Banking |
| Cross-currency request | Source and destination currencies differ. | Block or route to a separately governed FX process; no implicit conversion. | Treasury / Accounting Governance |
| Bank execution fails | External bank rejects movement. | Publish source-supported failure, preserve reason, and reassess retry. | Treasury / External Bank |
| Outcome unknown | Delivery/acknowledgement is inconclusive. | Hold unresolved and investigate before any retry. | Treasury / External Bank |
| Outgoing succeeds, incoming pending | Bank presents asymmetric/timing evidence. | Preserve partial state, prohibit settled classification, and reconcile until authoritative completion/failure. | Treasury / External Bank |
| Bank charge applies | Movement incurs an external fee. | Register fee bank fact and route accounting/business recognition to canonical owner. | Treasury / Accounting & GL |
| Transfer cancelled before execution | Governed withdrawal occurs before bank initiation. | Preserve cancellation/decision evidence using available behaviour; no cash movement. | Governance / Treasury |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Account identity/Institution/currency is inconsistent. | Funds may move to the wrong account or tenant. | Reject and obtain authoritative corrected references. | Treasury & Banking |
| Approval is stale, missing, expired, or untraceable. | Execution lacks authority. | Preserve current state and obtain a valid new outcome. | Enterprise Governance |
| Duplicate request/attempt arrives. | Same funds may move twice. | Resolve by immutable correlation and prior outcomes before any new execution. | Treasury & Banking |
| Concurrent aggregate version changes. | Cash/liquidity eligibility becomes stale. | Reload aggregate, re-evaluate all guards, and create a new governed attempt if eligible. | Treasury & Banking |
| Partial aggregate update occurs. | Source/destination, settlement, or cash facts may diverge. | Roll back, preserve prior version, and publish no false event. | Treasury & Banking |
| Bank request delivery is uncertain. | Blind retry risks duplicate movement. | Preserve attempt and seek authoritative bank outcome before retry. | Treasury / External Bank |
| Execution commits but event publication fails. | Settlement/consumers may not receive the fact. | Recover publication without executing again. | Treasury & Banking |
| Settlement completes but only one account effect is represented. | Inter-account movement appears unbalanced. | Block final acceptance, preserve facts, and reconcile/correct through governed Treasury behaviour. | Treasury & Banking |
| Cash Position changes before settlement. | Institutional cash is misstated. | Reject/contain change; update only after qualifying completion. | Treasury & Banking |
| Accounting treats transfer as income/expense without authority. | Reporting may be misstated. | Preserve Treasury fact and route accounting correction to GL owner. | Accounting & GL |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Treasury owns institutional cash movement, Treasury Accounts, Cash/Liquidity Positions, Payment and Settlement Executions. | Treasury & Banking | Inter-account movement must occur within Treasury aggregate behaviour. | `L2-F10`, Scope and Ownership |
| Authorization and governance authority remain externally owned. | Authorization & Policy / Governance | Treasury cannot approve its own transfer request. | `L2-F10`, Governance/Security Constraints |
| Every Payment Execution references approved authority/obligation context and preserves immutable execution history. | Treasury & Banking | Transfer execution must be traceable and authorized. | `L2-F10`, Payment Invariants |
| Settlement completion finalizes cash movement; Cash Position changes only after successful settlement. | Treasury & Banking | Execution acknowledgement alone is not settled movement. | `L2-F10`, Settlement/Cash Invariants |
| Cash Position preserves financial correctness and deterministic calculation. | Treasury & Banking | Source/destination effects cannot create/destroy unexplained cash. | `L2-F10`, Cash Integrity Constraints |
| Liquidity must not become negative unless institutional policy explicitly permits it. | Treasury / Governance | Insufficient source liquidity blocks normal transfer. | `L2-F10`, Liquidity Invariants |
| Treasury performs no implicit currency conversion. | Treasury & Banking | Cross-currency transfers require separate governed behaviour. | `L2-F10`, Currency Constraints |
| External bank records remain authoritative and cannot be overwritten. | External Bank | Treasury stores immutable business representations only. | `L2-F10`, Bank Transaction Invariants |
| Treasury never creates Journal Entries. | Accounting & GL | Accounting independently determines transfer accounting. | `L2-F10`, GL Integration |
| Every successful modification advances version; failure preserves version/history. | Treasury & Banking | Concurrent movement decisions are protected. | `L2-F10`, Version/Failure Constraints |
| Events publish after completed consistent aggregate facts. | Treasury & Banking | No intent, partial, or failed transfer can be published as settled. | `L2-F10`, Event Constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Fund Transfer Request | No canonical model/state | Validate, authorize, reject, execute, or hold. | Scenario outcome only; exact lifecycle undefined | Owner unresolved / Treasury execution | Canonical register; L2 gap |
| Payment Execution | No complete state enumeration | Start and record success/failure/unknown execution attempt. | Event/status-driven outcome | Treasury & Banking | `L2-F10`, Payment Execution |
| Settlement Execution | Current/initiated | Track and complete both-sided cash movement. | Completed/failed/current status; full lifecycle undefined | Treasury & Banking | `L2-F10`, Settlement Execution |
| Source Cash/Liquidity Position | Current authoritative | Reduce availability/position only under valid completed behaviour. | Updated source position after settlement | Treasury & Banking | `L2-F10`, Cash/Liquidity Management |
| Destination Cash/Liquidity Position | Current authoritative | Increase/reflect received movement only under valid completed behaviour. | Updated destination position after settlement | Treasury & Banking | `L2-F10`, Cash/Liquidity Management |
| Bank Transaction representations | New external activity | Register outgoing/incoming banking evidence and reconciliation status. | Immutable registered facts | Treasury & Banking | `L2-F10`, Bank Transaction Management |
| Treasury Aggregate | `Operational` unless independently restricted | Enclose consistent movement and version changes. | Remains valid current state | Treasury & Banking | `L2-F10`, Aggregate State Model |

L2-F10 does not define a Fund Transfer state machine or account-level Cash Position composition. This scenario cannot finalize exact transfer states or atomic source/destination position semantics without the Section 17 resolutions.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PaymentExecutionStarted` | Treasury & Banking | Monitoring, Audit | Announces source-supported start of execution attempt. | `L2-F10`, Payment Events |
| `PaymentExecuted` / `PaymentExecutionFailed` | Treasury & Banking | Settlement, GL, Reporting, Audit | Announces execution result; not settlement. | `L2-F10`, Payment Events |
| `SettlementStarted` / `SettlementCompleted` / `SettlementFailed` | Treasury & Banking | GL, Reporting, Audit | Announces settlement progression/finality. | `L2-F10`, Settlement Events |
| `CashPositionUpdated` | Treasury & Banking | GL, Reporting, dashboards, Audit | Announces completed cash-position change. | `L2-F10`, Cash Management Events |
| `LiquidityPositionUpdated` | Treasury & Banking | Treasury operations, Reporting, Audit | Announces completed liquidity change. | `L2-F10`, Liquidity Events |
| `BankTransactionRegistered` | Treasury & Banking | Reconciliation, GL, Reporting, Audit | Announces recorded outgoing/incoming bank activity. | `L2-F10`, Banking Events |
| Reconciliation events | Treasury & Banking | GL, Reporting, Audit | Coordinate verification of both bank sides. | `L2-F10`, Reconciliation Events |

No canonical FundTransferRequested, Authorized, Rejected, Started, PartiallyCompleted, Settled, Failed, Cancelled, Reversed, or source/destination-account-updated event exists.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Requesting function | Governance / Treasury | Source/destination accounts, Institution, amount, currency, purpose, timing, and correlation | Establishes transfer intent. | Canonical scenario; exact contract absent |
| Authorization & Policy / Governance | Treasury & Banking | Approval/rejection, authority, conditions, restrictions, effective window, and evidence | Permits/denies execution. | `L2-F10`, Authorization Integration; exact transfer contract absent |
| Treasury & Banking | External Banking Platform | Correlated source/destination transfer instruction and settlement references | Requests bank execution without exposing aggregate internals. | `L2-F10`, Banking Integration; transfer business contract content undefined |
| External Bank | Treasury & Banking | Acknowledgement, bank reference, source/destination outcome, timing, fee, and failure reason | Enables canonical execution/settlement representation. | `L2-F10`, Banking Integration; exact contract incomplete |
| Treasury & Banking | GL / Reporting / Audit | Execution, settlement, cash/liquidity, bank, and reconciliation facts | Enables independent accounting/reporting/audit reaction. | `L2-F10`, Event Integration |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request transfer | No canonical actor defined | Must provide valid purpose, accounts, amount, currency, timing, and evidence. | Requesting function unresolved |
| Approve/reject transfer | No canonical role/hierarchy defined | Authority remains external to Treasury. | Enterprise Governance |
| Validate liquidity/accounts | No canonical actor defined | Must use current Treasury-owned facts and policy. | Treasury & Banking |
| Initiate bank execution | No canonical actor/signatory defined | Requires fresh approval, eligibility, restrictions, and version checks. | Treasury & Banking / external mandate authority |
| Authorize retry/cancellation/reversal | No canonical role/rules defined | Must preserve original facts and prevent duplication. | Enterprise Governance / Treasury |
| Determine accounting | No canonical actor defined in L2-F10 | Treasury facts are input; GL owns Journals. | Accounting & GL |
| Review movement evidence | No canonical reviewer defined | Read-only review cannot alter facts. | Audit |

## 15.2 Segregation of Duties

- Request, approval, source/destination verification, liquidity validation, execution initiation, bank mandate/signing, settlement confirmation, reconciliation, accounting, and audit should remain independently governed; exact separation is absent.
- Treasury execution ownership does not grant approval authority.
- The requester should not resolve an unknown outcome through an uncontrolled retry.
- Reconciliation/accounting cannot rewrite Treasury or bank facts to force balance.
- Reporting and Audit remain read-only.

## 15.3 Mandatory Audit Evidence

- Request identity, Institution, source/destination accounts and bank references, amount, currency, purpose, timing, requester context, and correlation.
- Account identity/status/relationship/currency evidence and duplicate/conflict checks.
- Source Cash/Liquidity Position, reserves/restrictions, thresholds, scheduled activity, and policy/configuration versions.
- Authorization/approval/rejection, authority, conditions, effective window, segregation, and exception evidence.
- Execution and Settlement identities, attempts, versions, external request, acknowledgement, bank references, outcomes, failure/unknown reasons, and events.
- Source/destination Cash/Liquidity positions before/after and qualifying completion facts.
- Outgoing/incoming Bank Transaction representations and reconciliation evidence.
- Bank fees, timing/asymmetric results, cancellations, retries, reversals, exceptions, simultaneous-change conflicts, and recovery.
- Accounting hand-off reference and proof that Treasury created no Journal Entry.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

| Measure | Business Meaning | Expected Direction / Acceptance Intent |
|---|---|---|
| Authorized transfer rate | Executed movements backed by valid authority | 100% |
| Duplicate movement count | Same request/authority executed more than permitted | Zero |
| Balanced movement traceability | Settled transfers with linked source, destination, settlement, cash, and bank facts | 100% |
| Premature cash-update count | Cash changed before settlement completion | Zero |
| Protected-cash misuse count | Restricted/reserved funds used without authority | Zero |
| Historical reconstruction rate | Request-to-authorization-to-settlement histories reconstructable | 100% |

No canonical approval threshold, transfer limit, liquidity minimum, execution window, settlement SLA, reattempt-governance rule, bank-finality timeout, or tolerance exists upstream.

## 16.2 Postconditions

- Filename and Scenario ID match the frozen canonical register.
- Source and destination are distinct, governed, in-scope account relationships.
- Transfer executes only with valid external authorization and current liquidity/account eligibility.
- No implicit currency conversion or cross-Institution ownership violation occurs.
- Execution does not imply settlement; Cash Position changes only after successful settlement.
- Settled movement preserves linked source and destination effects and bank evidence.
- Unknown/partial outcomes cannot be labelled settled or blindly retried.
- Events publish only after corresponding aggregate facts succeed.
- Treasury creates no Journal Entry and does not treat transfer as income/expense by default.
- Failed operations preserve prior valid version, cash integrity, history, and auditability.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Gap | Architectural Impact | Required Resolution Owner |
|---|---|---|---|
| OI-F060-001 | L2-F10 defines no Fund Transfer capability, aggregate/entity, command, policy, lifecycle, or event. | The canonical scenario lacks a direct locked L2 realization. | Treasury & Banking / Finance Architecture Governance |
| OI-F060-002 | No Fund Transfer Request owner, state model, approval contract, amendment, cancellation, expiry, or rejection behaviour exists. | Initiation and governance cannot be standardized. | Treasury / Enterprise Governance |
| OI-F060-003 | Relationship between Fund Transfer and Payment Execution/Settlement Execution is undefined. | Transfer may be incorrectly treated as vendor payment or ordinary settlement. | Treasury & Banking |
| OI-F060-004 | Source/destination Cash Position semantics and whether positions exist per Treasury Account are not defined. | Balanced inter-account effects cannot be mechanically validated. | Treasury & Banking |
| OI-F060-005 | No transfer-specific events exist for requested, authorized, rejected, started, partial, settled, failed, cancelled, reversed, or corrected outcomes. | Consumers cannot distinguish transfer facts from payments. | Treasury & Banking |
| OI-F060-006 | Source/destination account eligibility, status, relationship, allowed combinations, and same-account rules are incomplete. | Invalid account pairs may execute. | Treasury & Banking |
| OI-F060-007 | Cross-Institution, cross-Organisation, cross-bank, cross-currency, cash-pool, trust/fund-restricted, and inter-campus transfer rules are undefined. | Ownership, restriction, and legal boundaries may be violated. | Treasury / Institutional Governance |
| OI-F060-008 | Authorization thresholds, hierarchy, maker-checker, bank mandate/signatory, emergency transfer, and segregation rules are absent. | High-risk cash movement authority cannot be fully accepted. | Enterprise Governance / Authorization & Policy |
| OI-F060-009 | Liquidity reservation, release, simultaneous transfer, overdraft, priority, and scheduled-payment interaction rules are incomplete. | Multiple movements may overcommit source cash. | Treasury & Banking |
| OI-F060-010 | All-or-none business completion and finality semantics for outgoing and incoming sides and treatment of partial/asymmetric bank outcomes are undefined. | One-sided movements may be incorrectly declared settled. | Treasury / Banking Integration |
| OI-F060-011 | Bank fees, value dates, transfer timing, reversals, returns, rejected credits, and corrected bank transactions lack canonical treatment. | Cash and settlement outcomes may be incomplete. | Treasury / Accounting / Banking Integration |
| OI-F060-012 | Accounting treatment and event contract to avoid false income/expense or duplicate Journal entries are incomplete. | General Ledger may misclassify the movement. | Accounting & GL / Treasury |
| OI-F060-013 | Exact request, approval, execution, settlement, source/destination position, bank, reconciliation, and accounting business contract content are undefined. | End-to-end traceability cannot be mechanically validated. | Relevant contract owners |
| OI-F060-014 | Duplicate and reverse-duplicate treatment, repeated submissions, late acknowledgements, simultaneous movement decisions, and published-event outcome confirmation are incomplete. | Duplicate or missing movements may occur. | Treasury & Banking / Banking Integration |
| OI-F060-015 | Transfer retention, cancellation/reversal history, authorized access, investigation, and post-settlement correction rules are absent. | Audit and historical accountability may be impaired. | Treasury / Audit / Compliance |
| OI-F060-016 | No canonical transfer limit, liquidity minimum, approval threshold, execution/settlement SLA, reattempt-governance rule, finality timeout, or acceptance tolerance exists. | Measures remain directional rather than fully testable. | Treasury / Enterprise Governance |

This scenario remains **Draft** until a canonical Fund Transfer model, its relationship to Payment/Settlement Execution, account-level cash semantics, authorization, source/destination finality, events, contracts, accounting hand-off, and measurable acceptance criteria are defined. It does not invent a new aggregate or treat one-sided execution as settled inter-account movement.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-060**
