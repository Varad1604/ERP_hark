# Bank Statement to Reconciled Cash Position

# 1. Scenario Metadata

**Scenario ID:** L3-F-058  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; `L3-F-056_Bank_Account_Request_to_Active_or_Closed_Treasury_Account.md`; `L3-F-057_Approved_Payment_Population_to_Bank_Execution_Outcome.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs intake of authoritative external bank-statement activity, registration of Treasury's immutable Bank Transaction representations, comparison with Treasury payment, settlement, receipt, account, and cash facts, identification and governed resolution of exceptions, and coordination of a reconciled cash-position outcome. Treasury coordinates operational bank reconciliation and owns Cash Position; Accounting & General Ledger independently owns the final financial reconciliation outcome and any accounting entries.

---

# 3. Business Outcome

Every accepted bank-statement line is preserved and classified as matched, unmatched, partially matched, duplicate, timing-related, conflicting, or requiring investigation against authoritative Treasury facts. Reconciliation coordination completes with immutable status and exception evidence, Cash Position remains consistent with successfully completed settlements and authoritative bank activity under Treasury policy, and Accounting independently determines the financial reconciliation outcome without Treasury creating Journal Entries or overwriting bank records.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Treasury receives an authoritative bank statement or equivalent bank-activity population for a governed Bank Account Reference, Institution, currency, and statement period.

## 4.2 End Boundary

The scenario ends when statement integrity is accepted, Treasury Bank Transaction representations are registered, all lines have a current reconciliation classification, exceptions have an owner and disposition, Treasury publishes reconciliation status, and the Cash Position is confirmed or governed through source-supported Treasury behaviour. Accounting's final financial reconciliation and Journal postings remain separately owned.

## 4.3 In Scope

- Identification and validation of bank, branch/account reference, Institution, currency, statement period, opening/closing information where supplied, and source authenticity.
- Duplicate statement and duplicate transaction detection.
- Translation of external banking semantics into canonical Treasury business meaning.
- Registration of incoming and outgoing Bank Transaction representations.
- Preservation of bank reference, Treasury reference, amount, currency, transaction date, type, reconciliation status, and immutable history.
- Comparison with Payment Execution, Settlement Execution, Cash Position, receipts, and other authoritative Treasury activity.
- Identification, classification, investigation, and tracking of reconciliation exceptions.
- Publication of reconciliation start, completion, exception, coordination, and Bank Transaction facts where canonically defined.
- Coordination with Accounting for independently owned financial reconciliation outcomes.

## 4.4 Out of Scope

- Modifying the original bank statement or external bank transaction.
- Creating, approving, executing, or settling payments.
- Creating Receivables, Payables, refunds, fees, interest, or other obligations.
- Creating Journal Entries, changing Ledger balances, or declaring Accounting reconciliation complete.
- Defining statement file formats, bank APIs, parsers, matching algorithms, storage, credentials, encryption, or integration technology.
- Inventing reconciliation tolerances, automatic write-off rules, or state transitions absent from locked architecture.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-056` Bank Account Request to Active or Closed Treasury Account | Provides the governed Treasury Account and Bank Account Reference. |
| Preceded by | `L3-F-057` Approved Payment Population to Bank Execution Outcome | Supplies payment execution facts that may appear in bank activity. |
| Related | `L3-F-029` Accounting Source and Ledger to Reconciled Position | Accounting independently owns financial/ledger reconciliation. |
| Related | `L3-F-040` Overdue Receivable to Governed Dunning Outcome | Receipt exceptions may affect AR but Treasury cannot change Receivables. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Treasury & Banking / Section F | Owns Cash Position, Bank Transaction business representations, reconciliation coordination, Treasury references/status, aggregate version, and Treasury events. |
| External Banking Institution | Owns the original statement, bank-account activity, transaction references, and source banking facts. |
| Accounting & General Ledger / Section F | Owns financial reconciliation outcomes, accounting classifications, Journals, and Ledger balances. |
| Accounts Payable / Section F | Owns Payables and reacts independently to outgoing settlement/reconciliation facts. |
| Accounts Receivable / Section F | Owns Receivables and reacts independently to incoming bank/receipt facts. |
| Authorization & Policy / Enterprise Governance | Own authorization, exception approval, escalation, and governance outcomes. |
| Financial Reporting | Consumes completed Cash Position and reconciliation facts for read-only reporting. |
| Audit | Reviews immutable bank, Treasury, matching, exception, and reconciliation evidence. |
| Institution and Organisation Management | Own institutional and organisational context referenced by Treasury. |

Treasury coordinates operational reconciliation. Accounting owns financial reconciliation outcomes. The bank remains the authoritative source for external banking activity.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F10` defines no canonical actors for statement submission, source verification, statement acceptance, matching, exception investigation, reconciliation review, approval, or closure. It also does not define the human/system responsibility boundary between Treasury reconciliation coordination and Accounting reconciliation. No roles are invented; gaps appear in Section 17.

---

# 7. Preconditions and Entry Criteria

- A governed Treasury Aggregate, Treasury Account, and Bank Account Reference exist for the Institution.
- The bank statement/activity population identifies its bank account, currency, covered period, and authoritative source.
- Treasury can establish statement identity and detect duplicate submission or overlapping scope.
- Relevant Payment Execution, Settlement Execution, receipt, Bank Transaction, Cash Position, and correlation facts are available.
- External banking acknowledgements can be translated through the anti-corruption boundary.
- Bank Transaction registration can preserve immutable source and Treasury references.
- Authorization and exception-governance outcomes can be obtained.
- Accounting can independently receive reconciliation evidence without Treasury modifying Journals or Ledgers.
- Historical Treasury and bank evidence remains available and unaltered.

---

# 8. Trigger

Treasury receives a scheduled or ad hoc authoritative bank statement/bank-activity population, or a bank transaction fact requiring reconciliation coordination.

No canonical BankStatementReceived/Accepted/Rejected event, statement aggregate, intake state model, or mandatory reconciliation cadence exists upstream.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the bank statement/activity population with source, account, currency, statement identity, period, and transaction lines. | External Bank / Treasury & Banking | A traceable reconciliation input exists. |
| 2 | Validate Institution and Bank Account Reference mapping, source authenticity, currency, statement period, completeness, continuity, and duplicate/overlap indicators. | Treasury & Banking | Invalid or ambiguous statements are stopped before reconciliation. |
| 3 | Preserve the external statement and references without altering bank-owned facts, then translate each line into canonical Treasury business meaning. | Treasury & Banking | External semantics do not leak into the Treasury model. |
| 4 | Detect duplicate bank transactions using authoritative bank reference, account, amount, currency, date, and previously registered evidence. | Treasury & Banking | Duplicate Treasury representations are prevented. |
| 5 | Register each valid incoming/outgoing Bank Transaction representation with immutable identifier, bank reference, Treasury reference where known, amount, currency, date, type, and initial reconciliation status. | Treasury & Banking | Treasury obtains canonical bank-activity representations. |
| 6 | Advance aggregate version and publish `BankTransactionRegistered` only after each governed registration succeeds. | Treasury & Banking | Completed bank-activity facts become traceable. |
| 7 | Start reconciliation coordination and publish `ReconciliationStarted` after the start fact is recorded. | Treasury & Banking | The reconciliation run is identifiable and auditable. |
| 8 | Compare statement lines against Payment Executions, Settlement Executions, receipts, previously registered Bank Transactions, Cash Position movements, account, amount, currency, date, and correlation references. | Treasury & Banking | Candidate matches and discrepancies are identified. |
| 9 | Classify each line/current Treasury fact as matched, unmatched, partially matched, duplicate, timing-related, conflicting, or requires investigation, without overwriting source facts. | Treasury & Banking | Every item receives a current reconciliation status. |
| 10 | For exceptions, preserve evidence, reason, amount/currency difference, owner, impact, and investigation status; publish `ReconciliationExceptionDetected`. | Treasury & Banking | Operational exceptions are visible and immutable. |
| 11 | Route payment/settlement issues to Treasury, Payable issues to AP, receipt/Receivable issues to AR, accounting classification issues to GL, and bank-source issues to the external bank. | Treasury & Banking / canonical owners | Resolution ownership remains bounded-context correct. |
| 12 | Consume new authoritative resolution facts, re-evaluate affected matches, and append status/history without deleting original evidence. | Treasury & Banking | Exceptions progress through governed facts. |
| 13 | Validate aggregate consistency, Bank Transaction invariants, reconciliation coverage, unresolved exceptions, Cash Position consistency, version, and authorization. | Treasury & Banking | Completion eligibility is established. |
| 14 | Publish `ReconciliationCoordinated` and, when coordination completes under policy, `ReconciliationCompleted`; Accounting independently determines financial reconciliation outcome. | Treasury & Banking / Accounting & GL | Treasury coordination completes without claiming Accounting ownership. |
| 15 | Confirm or update Cash Position only through source-supported completed business behaviour; publish `CashPositionUpdated` only after successful qualifying completion. | Treasury & Banking | The authoritative Cash Position remains financially correct and auditable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Duplicate statement | Same statement identity/scope was processed. | Return existing authoritative outcome and do not re-register transactions. | Treasury & Banking |
| Overlapping statement | New input overlaps an earlier period but contains new authoritative facts. | Isolate duplicates, register only valid new facts, and preserve overlap evidence. | Treasury & Banking |
| Incomplete statement | Continuity, totals, pages/segments, or required source evidence is missing. | Keep intake pending/rejected and request authoritative completion. | External Bank / Treasury & Banking |
| Exact match | Bank line and Treasury fact agree under policy. | Mark operational reconciliation status matched and retain linkage. | Treasury & Banking |
| Timing difference | Legitimate settlement/booking timing differs. | Preserve timing exception and re-evaluate when new authoritative facts arrive. | Treasury & Banking |
| One-to-many/many-to-one | Aggregated bank booking corresponds to multiple Treasury facts or vice versa. | Apply only governed matching policy and preserve all member references. | Treasury & Banking |
| Unknown incoming credit | No Treasury/AR source is identifiable. | Register bank fact, raise exception, and route investigation without creating a Receivable. | Treasury & Banking / AR |
| Unknown outgoing debit | No executed payment/settlement is identifiable. | Register bank fact, restrict/escalate where policy requires, and investigate without fabricating execution. | Treasury & Banking / Governance |
| Bank fee/interest | Statement contains externally originated charge/credit. | Register bank fact and route accounting/business recognition to canonical owner. | Treasury & Banking / Accounting & GL |
| Unresolved exceptions remain | Policy prevents clean completion. | Publish/retain exception and blocked coordination status; do not assert fully reconciled outcome. | Treasury & Banking / Accounting & GL |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Statement maps to wrong Institution/account/currency. | Cash may be reconciled against unrelated activity. | Reject and establish authoritative mapping before retry. | Treasury & Banking |
| Source authenticity is unverified. | False bank activity may enter Treasury. | Keep input unaccepted and obtain authoritative evidence. | External Bank / Treasury & Banking |
| Duplicate transaction is registered. | Cash/reconciliation may be overstated. | Preserve evidence, prevent further impact, and correct through governed new facts rather than deletion. | Treasury & Banking |
| Concurrent version change occurs. | Matching or status update is stale. | Reload aggregate, re-evaluate affected items, and retry governed behaviour. | Treasury & Banking |
| Partial aggregate update occurs. | Transaction, status, Cash Position, or history may diverge. | Roll back, preserve prior version, and publish no false event. | Treasury & Banking |
| Reconciliation starts but processing is interrupted. | Some lines lack current status. | Resume from immutable item/run evidence without duplicating registered facts. | Treasury & Banking |
| Exception event publication fails after commit. | Owners may not investigate the recorded issue. | Recover publication from committed state without recreating exception. | Treasury & Banking |
| External bank later corrects/reverses a line. | Prior matching may become invalid. | Register new authoritative bank fact, reopen/re-evaluate affected reconciliation under governance, and preserve old history. | External Bank / Treasury & Banking |
| Treasury and Accounting conclusions conflict. | Operational and financial reconciliation diverge. | Preserve both owned outcomes and route cross-domain governance resolution. | Treasury & Banking / Accounting & GL |
| Cash Position is updated without qualifying completed fact. | Institutional cash becomes inaccurate. | Reject/contain update and rebuild from authoritative completed Treasury facts. | Treasury & Banking |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Treasury owns Cash Position, Bank Transaction representations, and reconciliation coordination. | Treasury & Banking | Other domains cannot directly modify Treasury reconciliation state. | `L2-F10`, Scope and Aggregate Ownership |
| Banks remain the source of external banking activity; Treasury owns only its business representation. | External Bank / Treasury | Original bank records are never overwritten. | `L2-F10`, Bank Transaction Management |
| Every Bank Transaction remains immutable after recording, preserves banking traceability, references Treasury activity, and maintains reconciliation status. | Treasury & Banking | Registration and matching must preserve full history. | `L2-F10`, Bank Transaction Invariants |
| Treasury coordinates reconciliation; Accounting owns financial reconciliation outcomes. | Treasury / Accounting & GL | Treasury cannot declare ledger reconciliation or create correction entries. | `L2-F10`, Bank Reconciliation Coordination |
| Cash Position is authoritative Treasury business state and changes only after successful qualifying business completion. | Treasury & Banking | Statement intake or matching alone cannot arbitrarily change cash. | `L2-F10`, Cash/Settlement Invariants |
| External banking semantics enter through the anti-corruption boundary. | Treasury & Banking | Bank status/format cannot directly replace canonical Treasury concepts. | `L2-F10`, Anti-Corruption Boundary |
| Treasury never creates Journal Entries or modifies Ledger balances. | Accounting & GL | Accounting reacts independently to completed facts. | `L2-F10`, Architectural Boundaries and GL Integration |
| Aggregate consistency and invariants precede persistence/event publication. | Treasury & Banking | Partial reconciliation modifications and false events are prohibited. | `L2-F10`, Aggregate Consistency |
| Successful modification advances version; failure preserves version and history. | Treasury & Banking | Reconciliation is protected from conflicting simultaneous decisions. | `L2-F10`, Version and Failure Constraints |
| Domain Events remain immutable, ordered, correlated, and historically available. | Treasury & Banking | Reconciliation chronology remains reconstructable. | `L2-F10`, Event Constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| External Bank Statement | Bank-owned source fact | Validate and reference without modification. | Unchanged external evidence | External Bank | `L2-F10`, Banking Independence |
| Bank Transaction representation | New or existing | Register immutable bank activity and maintain reconciliation status. | Registered with current status/history | Treasury & Banking | `L2-F10`, Bank Transaction entity/invariants |
| Reconciliation coordination | No canonical state machine | Start, identify exceptions, investigate, coordinate, and complete where eligible. | Represented through events/status; exact lifecycle undefined | Treasury & Banking | `L2-F10`, Reconciliation capability/events |
| Cash Position | Current authoritative position | Confirm or update only through qualifying completed Treasury behaviour. | Reconciled/consistent position intended by title | Treasury & Banking | `L2-F10`, Cash Position and invariants |
| Accounting reconciliation | Accounting-owned state | Receives Treasury/bank evidence; no direct transition by Treasury. | Independently determined | Accounting & GL | `L2-F10`, Reconciliation boundary |
| Treasury Aggregate | `Operational` unless separately restricted | Encloses consistent changes and versioning. | Remains valid current aggregate state | Treasury & Banking | `L2-F10`, Aggregate model |

The canonical title uses â€œReconciled Cash Position,â€ but `L2-F10` does not define a Cash Position reconciliation state or transition. The outcome is therefore treated as a governed consistency conclusion pending Section 17 resolution.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `BankTransactionRegistered` | Treasury & Banking | Accounting & GL, Reporting, Audit | Announces Treasury's recorded banking-activity representation. | `L2-F10`, Banking Events |
| `ReconciliationStarted` | Treasury & Banking | Accounting & GL, Audit, monitoring | Announces reconciliation coordination start. | `L2-F10`, Reconciliation Events |
| `ReconciliationExceptionDetected` | Treasury & Banking | Relevant exception owner, Accounting, Audit, Notification | Announces an operational reconciliation exception. | `L2-F10`, Reconciliation Events |
| `ReconciliationCoordinated` | Treasury & Banking | Accounting & GL, Reporting, Audit | Announces completed coordination behaviour. | `L2-F10`, Aggregate Behaviour |
| `ReconciliationCompleted` | Treasury & Banking | Accounting & GL, Reporting, Audit | Announces coordination completion; not Accounting's financial outcome. | `L2-F10`, Reconciliation Events |
| `CashPositionUpdated` | Treasury & Banking | GL, Reporting, dashboards, Audit | Announces completed Cash Position change. | `L2-F10`, Cash Management Events |
| Payment/Settlement events | Treasury & Banking | Reconciliation coordination | Supply Treasury-owned candidate facts for outgoing bank activity. | `L2-F10`, Payment/Settlement Events |

No canonical BankStatementReceived/Accepted/Rejected, line-matched/unmatched, duplicate-detected, timing-difference, investigation-started/resolved, reconciliation-blocked, or CashPositionReconciled event exists.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| External Bank | Treasury & Banking | Statement identity, bank/account reference, currency, period, transaction reference/type/amount/date, balance information, and source evidence | Supplies authoritative banking activity. | `L2-F10`, External Banking Integration; exact contract incomplete |
| Treasury & Banking | Accounting & GL | Registered bank facts, Treasury activity links, reconciliation status, exceptions, Cash Position facts, and correlation | Enables independent financial reconciliation and accounting. | `L2-F10`, GL/Reconciliation Integration; exact contract incomplete |
| Treasury & Banking | AP/AR/other owner | Exception or matched fact referencing owned obligation/receipt | Enables canonical owners to resolve business discrepancies. | `L2-F10`, event-driven collaboration; exact contract incomplete |
| Accounting & GL / AP / AR / Bank | Treasury & Banking | Authoritative resolution/correction facts | Supports re-evaluation without ownership transfer. | Cross-domain architecture; exact contract incomplete |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Submit/obtain bank statement | No canonical actor defined | Must use authoritative bank source and governed account scope. | External Bank / Treasury context |
| Accept statement for reconciliation | No canonical actor defined | Requires identity, authenticity, completeness, continuity, and duplicate checks. | Treasury & Banking |
| Register Bank Transaction | No canonical actor defined | Only Treasury aggregate after invariants succeed. | Treasury & Banking |
| Determine operational match/exception | No canonical actor defined | Must preserve bank and Treasury facts and configured policy. | Treasury & Banking |
| Approve exception resolution | No canonical role/hierarchy defined | Must originate with authoritative business/governance owner. | Relevant owner / Enterprise Governance |
| Determine financial reconciliation | No canonical actor defined in L2-F10 | Treasury evidence is input; Accounting owns outcome. | Accounting & GL |
| Review reconciliation evidence | No canonical auditor role defined | Read-only review cannot modify facts. | Audit |

## 15.2 Segregation of Duties

- Statement acquisition, acceptance, transaction registration, matching, exception resolution, reconciliation review, Cash Position change, financial reconciliation, and audit should be independently governed; exact mandatory separation is undefined.
- Treasury cannot create accounting entries to force a match.
- Accounting cannot overwrite Treasury's Bank Transaction representation or Cash Position.
- External bank evidence must not be editable by the matching/reconciliation actor.
- Audit and Reporting remain read-only consumers.

## 15.3 Mandatory Audit Evidence

- Statement identity, bank/account reference, Institution, currency, period, source, receipt/acceptance sequence, and integrity checks.
- Every original statement line and immutable source reference.
- Bank Transaction identifier, type, amount, currency, date, bank reference, Treasury reference, status, version, and event.
- Candidate match inputs, rules/policy version, match classification, confidence/tolerance where later defined, and decision evidence.
- Duplicate, unmatched, partial, timing, conflict, unknown credit/debit, fee/interest, and other exception evidence.
- Exception owner, investigation history, resolution fact, approval, and re-evaluation.
- Reconciliation run identity, start/completion, coverage, unresolved exceptions, and events.
- Cash Position before/after where changed, qualifying completed facts, version, and `CashPositionUpdated`.
- Accounting hand-off/outcome reference without Treasury-owned Journal data mutation.
- Failures, interruptions, simultaneous-change conflicts, published-event outcome confirmation, and later bank corrections.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

| Measure | Business Meaning | Expected Direction / Acceptance Intent |
|---|---|---|
| Statement intake completeness | Accepted statements passing identity, source, continuity, currency, and duplicate checks | 100% |
| Bank-line registration coverage | Valid statement lines represented as traceable Treasury Bank Transactions | 100% |
| Reconciliation classification coverage | Accepted lines with explicit current status | 100% |
| Unexplained cash difference | Difference lacking governed owner/disposition | Zero at accepted completion, subject to undefined tolerance |
| Duplicate representation count | Same bank transaction registered more than once | Zero |
| Historical mutation count | Bank or Treasury evidence overwritten/deleted | Zero |
| Audit reconstruction rate | Statement-to-transaction-to-match-to-outcome histories reconstructable | 100% |

No canonical matching tolerance, timing window, reconciliation frequency, completion SLA, exception aging threshold, approval threshold, or materiality rule is defined upstream.

## 16.2 Postconditions

- Filename and Scenario ID match the frozen canonical register.
- External statement/bank activity remains bank-owned and unmodified.
- Every accepted valid line has an immutable Treasury Bank Transaction representation or governed exclusion reason.
- Duplicate statements and transactions do not create duplicate cash effects.
- Every in-scope line has a current matching/exception classification and correlation evidence.
- Treasury coordinates operational reconciliation; Accounting independently owns financial reconciliation and Journal entries.
- Cash Position changes only through successful qualifying Treasury behaviour and a completed aggregate modification.
- Reconciliation events publish only after corresponding facts succeed.
- Unresolved exceptions prevent a false clean outcome when policy requires resolution.
- Failures preserve prior valid state, aggregate version, bank evidence, Treasury history, and auditability.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Gap | Architectural Impact | Required Resolution Owner |
|---|---|---|---|
| OI-F058-001 | No Bank Statement aggregate/entity, intake lifecycle, owner, or received/accepted/rejected events are defined. | Source completeness, duplicate scope, and acceptance cannot be standardized. | Treasury & Banking / Banking Integration |
| OI-F058-002 | No complete Bank Transaction lifecycle or allowed reconciliation-status values/transitions exist. | Transaction classification may diverge across implementations. | Treasury & Banking |
| OI-F058-003 | Reconciliation has events but no canonical lifecycle, guards, completion criteria, reopening, or cancellation rules. | Start, blocked, completed, and corrected outcomes are ambiguous. | Treasury & Banking / Accounting & GL |
| OI-F058-004 | â€œReconciled Cash Positionâ€ is not a defined Cash Position state or event in L2-F10. | The scenario's terminal outcome cannot be mechanically validated. | Treasury & Banking / Finance Architecture Governance |
| OI-F058-005 | Boundary between Treasury coordination completion and Accounting financial reconciliation outcome lacks an exact contract. | Both domains may claim or omit final responsibility. | Treasury & Banking / Accounting & GL |
| OI-F058-006 | Matching rules, tolerances, date windows, reference hierarchy, one-to-many/many-to-one logic, partial matches, and confidence are undefined. | Identical facts may receive different match conclusions. | Treasury & Banking / Accounting Governance |
| OI-F058-007 | Statement opening/closing balances, running-balance validation, missing-sequence detection, and overlapping/corrected statement rules are absent. | Statement integrity cannot be fully accepted. | Treasury & Banking / Banking Integration |
| OI-F058-008 | Unknown credits/debits, bank charges, interest, reversals, chargebacks, returned payments, stale instruments, and bank corrections lack canonical ownership flows. | Exceptions may remain unresolved or create incorrect obligations/accounting. | Treasury / AP / AR / GL Governance |
| OI-F058-009 | Cash Position update rules for externally discovered bank facts versus completed Settlement are incomplete. | Cash may be stale, duplicated, or updated prematurely. | Treasury & Banking |
| OI-F058-010 | No canonical line-matched, unmatched, duplicate, timing-difference, investigation, blocked, or CashPositionReconciled events exist. | Cross-domain coordination and audit chronology are incomplete. | Treasury & Banking |
| OI-F058-011 | Actors, maker-checker controls, reconciliation reviewer, exception approver, financial reconciler, and segregation rules are absent. | Authorization and accountability cannot be fully accepted. | Enterprise Governance / Authorization & Policy |
| OI-F058-012 | Exact bank statement, Bank Transaction, matching, exception, Cash Position, and Accounting hand-off business contract content are incomplete. | End-to-end traceability cannot be mechanically verified. | Relevant contract owners |
| OI-F058-013 | Duplicate/overlap repeat-request handling, late bank corrections, simultaneous reconciliation decisions, stale versions, and event recovery rules are incomplete. | Repeated or inconsistent cash/reconciliation facts may occur. | Treasury & Banking / Banking Integration |
| OI-F058-014 | Multi-currency statement treatment, exchange-rate ownership, conversion differences, and currency-specific Cash Position rules are undefined. | Reconciled amounts may be financially inconsistent. | Treasury & Banking / Accounting & GL |
| OI-F058-015 | Retention, masking, access, statement evidence, investigation history, and post-close correction requirements are incomplete. | Audit and compliance evidence may be unavailable or overexposed. | Treasury / Compliance / Data Governance |
| OI-F058-016 | No canonical cadence, tolerance, completion SLA, exception-aging threshold, materiality limit, or acceptance target exists. | Measures remain directional rather than fully testable. | Treasury & Banking / Enterprise Governance |

This scenario remains **Draft** until statement ownership, reconciliation and Bank Transaction state models, Cash Position reconciliation semantics, Treasury-Accounting boundary, matching rules, events, contracts, authorization, and measurable acceptance criteria are defined. It does not overwrite bank facts or allow Treasury to create accounting outcomes.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-058**
