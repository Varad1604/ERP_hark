# Accounting Batch to Governed Posting Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-F-027  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs high-volume posting of one or more approved Journal Entries as a single Accounting Batch. It ensures deterministic processing, double-entry integrity, complete auditability, and an atomic outcome in which either every Journal in the batch is posted consistently or the batch fails without retaining a partial ledger consequence.

---

# 3. Business Outcome

The Accounting Batch has reached a governed and auditable posting outcome: either all included approved Journals are posted and the General Ledger is updated consistently, or the complete batch is recorded as failed and financial consistency is restored without partial completion.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when one or more approved Journal Entries are selected as a governed population for coordinated batch posting.

## 4.2 End Boundary

The scenario ends when the Posting Batch reaches `Completed` with every included Journal posted and all ledger consequences recognized, or reaches `Failed` after complete withdrawal of the attempted batch consequences and preservation of the failure audit outcome.

## 4.3 In Scope

- Creation of a Posting Batch containing one or more approved Journal Entries.
- Validation of batch membership and posting readiness.
- Deterministic and atomic execution of the included Journal postings.
- Complete update of affected Ledger Accounts and the General Ledger on success.
- Complete restoration of financial consistency on batch failure.
- Recording of the batch result, audit history, and applicable canonical events.
- Governed retry of a failed Posting Batch where permitted by its lifecycle.

## 4.4 Out of Scope

- Creation, validation, approval, or rejection of individual Journal Entries.
- Manual Journal request governance, which belongs to `L3-F-026`.
- Correction or reversal of a successfully posted Journal, which belongs to `L3-F-028`.
- Partial completion or retention of only selected successful Journal postings; authoritative L2 explicitly prohibits partial batch completion.
- Reopening or unlocking an Accounting Period.
- Trial Balance generation, period closing, reconciliation, and financial reporting.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-025` Financial Transaction to Balanced Ledger Posting | Establishes the governed individual-Journal posting conditions applied within the batch. |
| Related | `L3-F-026` Manual Journal Request to Approved Posting or Rejection | May produce approved Manual Journals eligible for later batch inclusion. |
| Followed by | `L3-F-028` Posted Journal to Accounting Reversal | Governs a later correction after successful posting rather than recovery from a failed batch. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounting & General Ledger / Section F | Owns the Posting Batch, included Journal posting eligibility, deterministic execution, atomic batch outcome, Ledger Account and General Ledger consequences, batch events, and accounting audit history. |

The frozen Stage 3 row lists only Section F as participating. Operational domains may own the transactions from which individual Journals originated, but they do not participate materially in this Accounting-owned batch-posting outcome.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F05` does not assign batch creation, validation, execution, failure determination, or retry to a canonical business actor or role. Accounting & General Ledger owns those responsibilities, but this Draft does not invent an operator or approving role. The authority gap is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- At least one Journal Entry exists for inclusion in the Posting Batch.
- Every included Journal Entry is `Approved` and has completed successful validation.
- Every included Journal remains balanced and traceable to its valid business transaction.
- The applicable Accounting Period is `Open` and the Fiscal Year is active.
- Every affected Ledger Account exists, belongs to an approved Chart of Accounts, and is active and posting-eligible.
- The proposed batch membership and posting order can be determined reproducibly.

---

# 8. Trigger

Accounting initiates coordinated posting of a defined governed population of approved Journal Entries.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Group one or more approved Journal Entries into a Posting Batch and establish their deterministic posting order. | Accounting & General Ledger | A Posting Batch reaches `Created` with defined membership and sequence. |
| 2 | Validate that every included Journal is approved, balanced, posting-eligible, and associated with valid accounts, an Open Accounting Period, and the active Fiscal Year. | Accounting & General Ledger | The complete Posting Batch reaches `Validated`; any ineligible Journal prevents batch progression. |
| 3 | Start coordinated execution of the validated Posting Batch. | Accounting & General Ledger | The Posting Batch reaches `Processing` and its posting operation is treated as one atomic accounting outcome. |
| 4 | Apply every included Journal in deterministic sequence while preserving double-entry balance, chronology, aggregate boundaries, and complete traceability. | Accounting & General Ledger | The included Journals and all related ledger consequences remain mutually consistent throughout the governed batch outcome. |
| 5 | Recognize the complete batch only after every included Journal has posted successfully and every affected Ledger Account and the General Ledger reflect the full balanced consequence. | Accounting & General Ledger | All included Journals reach `Posted`; no partial completion is retained. |
| 6 | Complete and preserve the batch result and accounting audit history. | Accounting & General Ledger | The Posting Batch reaches `Completed`, becomes immutable, and `PostingBatchCompleted`, applicable `JournalPosted`, and `LedgerUpdated` facts represent the completed outcome. |

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow toward the same completed batch outcome is defined by the authoritative L2 architecture. Failure and retry interrupt the normal path and are governed in Section 11.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Batch validation finds an unapproved, unbalanced, duplicate, period-ineligible, account-ineligible, or otherwise invalid Journal. | The Posting Batch cannot progress from validation to processing. | Correct or remove the ineligible Journal through its governed individual workflow, then validate the complete batch again. No ledger consequence occurs. | Accounting & General Ledger |
| Any included Journal cannot be posted during batch processing. | The Posting Batch cannot reach `Completed`; partial batch completion is prohibited. | Withdraw the complete attempted batch consequence, restore financial consistency, record the batch as `Failed`, and preserve the failure audit history. | Accounting & General Ledger |
| A failed Posting Batch is authorized for another attempt after the underlying accounting deficiency is resolved. | The failed outcome remains historical while the batch enters its defined recovery position. | Move the Posting Batch from `Failed` to `Retried` and attempt the complete governed posting again. The upstream lifecycle defines only `Retried -> Completed`; retry failure handling is unresolved in Section 17. | Accounting & General Ledger |
| The Accounting Period closes or becomes locked before posting completes. | Posting eligibility is lost and the batch cannot complete validly. | Treat the posting as failed, retain no partial ledger consequence, and require separately governed period resolution before any later retry. | Accounting & General Ledger |

No path may retain successful postings for only part of the batch. The source requires complete batch withdrawal on failure.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Posting Batch shall contain one or more Journal Entries, and only approved Journals may be included. | Accounting & General Ledger | Empty batches and batches containing unapproved Journals are invalid. | `L2-F05`, Section 30 |
| A Posting Batch executes atomically; partial completion is prohibited. | Accounting & General Ledger | The batch produces either one complete posting outcome or no retained batch posting consequence. | `L2-F05`, Section 30 |
| Completed Posting Batches are immutable. | Accounting & General Ledger | A completed batch result cannot be altered after recognition. | `L2-F05`, Section 30 |
| Posting order shall be deterministic and every execution auditable. | Accounting & General Ledger | Batch results must be reproducible and fully traceable. | `L2-F05`, Sections 30 and 35.10 |
| `BR-001` and `BR-002`: Every Journal maintains double-entry accounting and total debit equals total credit. | Accounting & General Ledger | Every included Journal and the complete batch consequence must remain balanced. | `L2-F05`, Section 36.1 |
| `BR-011`, `BR-018`, and `BR-019`: Ledger balances change only through successful Journal posting, ledger updates are atomic, and failed ledger updates are completely withdrawn. | Accounting & General Ledger | No failed or partial batch may leave a recognized ledger change. | `L2-F05`, Section 36.2 |
| `BR-029` and `BR-030`: Posting is permitted only in an Open Accounting Period; closed periods reject posting. | Accounting & General Ledger | Every Journal in the batch must remain period-eligible through completion. | `L2-F05`, Section 36.4 |
| `BR-041` and `BR-042`: Only approved Journal Entries may be posted, and posting requires successful validation. | Accounting & General Ledger | Batch membership and readiness must be validated before processing. | `L2-F05`, Section 36.6 |
| `BR-043` through `BR-046`: Posting updates every affected Ledger Account and the General Ledger, preserves chronology, and completely withdraws failed posting effects. | Accounting & General Ledger | Success requires a complete ordered ledger outcome; failure retains none of it. | `L2-F05`, Section 36.6 |
| `BR-047` and `BR-048`: Posting publishes domain events and generates audit logs. | Accounting & General Ledger | Completed posting consequences and execution history require canonical events and audit evidence. | `L2-F05`, Section 36.6 |
| `BR-055` and `BR-056`: Posting authority follows governed role authority and maker-checker separation applies where applicable. | Accounting & General Ledger | Batch execution requires authorized participation without bypassing separation obligations. | `L2-F05`, Section 36.8 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Posting Batch | New batch requirement | One or more approved Journals are grouped in deterministic order. | `Created` | Accounting & General Ledger | `L2-F05`, Section 30 |
| Posting Batch | `Created` | Complete batch membership and posting readiness are validated. | `Validated` | Accounting & General Ledger | `L2-F05`, Section 30 |
| Posting Batch | `Validated` | Coordinated atomic posting begins. | `Processing` | Accounting & General Ledger | `L2-F05`, Section 30 |
| Posting Batch | `Processing` | Every included Journal and ledger consequence completes successfully. | `Completed`, then eligible for `Archived` | Accounting & General Ledger | `L2-F05`, Section 30 |
| Posting Batch | `Processing` | Any batch member or ledger consequence fails and the complete attempted consequence is withdrawn. | `Failed` | Accounting & General Ledger | `L2-F05`, Sections 30 and 35.5 |
| Posting Batch | `Failed` | A governed subsequent attempt is initiated after correction. | `Retried`, then `Completed` on success | Accounting & General Ledger | `L2-F05`, Section 30 |
| Included Journal Entries | `Approved` | Successful atomic batch execution posts every included Journal. | `Posted` and immutable | Accounting & General Ledger | `L2-F05`, Sections 28.5-28.6, 30 and 36.6 |
| Accounting Ledger and affected Ledger Accounts | Active and posting-eligible | Complete success updates balances; batch failure retains no attempted batch consequence. | Updated balances on completion; unchanged consistent balances after failure recovery | Accounting & General Ledger | `L2-F05`, Sections 23, 25, 30 and 36.2 |

The Posting Batch lifecycle does not define a transition from `Retried` back to `Processing` or `Failed`. This Draft preserves the stated `Failed -> Retried -> Completed` path and records the incomplete retry lifecycle in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PostingBatchCreated` | Accounting & General Ledger | Accounting & General Ledger | Records establishment of the governed Posting Batch. | `L2-F05`, Section 30 |
| `PostingBatchValidated` | Accounting & General Ledger | Accounting & General Ledger | Records successful validation of complete batch readiness. | `L2-F05`, Section 30 |
| `PostingBatchStarted` | Accounting & General Ledger | Accounting & General Ledger | Records entry into coordinated batch processing. | `L2-F05`, Section 30 |
| `PostingBatchCompleted` | Accounting & General Ledger | Accounting & General Ledger; Audit & Compliance | Records successful atomic completion of every included Journal posting. | `L2-F05`, Section 30 |
| `PostingBatchFailed` | Accounting & General Ledger | Not authoritatively defined | Named by the Posting Batch aggregate, but its publication conflicts with the domain-wide rule that failed transactions shall not publish business events. | `L2-F05`, Sections 30 and 37.1 |
| `PostingBatchRetried` | Accounting & General Ledger | Accounting & General Ledger; Audit & Compliance | Records the governed retry lifecycle occurrence. | `L2-F05`, Section 30 |
| `JournalPosted` | Accounting & General Ledger | Procurement Finance, Financial Analytics, Audit & Compliance | Communicates successful posting of each included Journal only after complete batch success. | `L2-F05`, Sections 28.8, 37.2 and 37.8 |
| `LedgerUpdated` | Accounting & General Ledger | Budget Management, Treasury Management, Financial Analytics, Audit & Compliance | Communicates the completed General Ledger consequence only after complete batch success. | `L2-F05`, Sections 25.8, 37.3 and 37.8 |

The exact publication treatment for `PostingBatchFailed` remains blocked by the upstream contradiction recorded in Section 17. This Draft does not silently publish or suppress it.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain synchronous contract or hand-off materially participates. Posting Batch membership, validation, posting, ledger effects, failure recovery, and retry are owned within Accounting & General Ledger.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Create and validate a Posting Batch | Not assigned to a canonical business role in `L2-F05` | Batch membership must contain only approved, validated, posting-eligible Journals. | Accounting & General Ledger |
| Execute the validated Posting Batch | Not assigned to a canonical business role in `L2-F05` | Posting authority must follow governed role authority and applicable separation of duties. | Accounting & General Ledger |
| Authorize or initiate retry after failure | Not assigned to a canonical business role in `L2-F05` | Retry may occur only after the cause of failure is resolved and financial consistency is preserved. | Accounting & General Ledger |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Posting authority | Journal approval participants and batch posting authority | Only approved Journals may be posted, and the posting authority must be authorized independently under the institution's governed role model. |
| Maker-checker separation | Journal Creator and Journal Approver | Maker and Checker shall be different users where applicable; batching does not bypass the individual Journal approval control. |
| Period protection | Batch posting authority and period-governance authority | A closed or locked Accounting Period cannot be bypassed through batch execution or retry. |
| Administrative override | Authorized financial authority | Any permitted administrative override must be fully audited and cannot permit partial batch completion. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve batch composition | Posting Batch identity, included Journal references, membership, and deterministic order | Accounting & General Ledger |
| Preserve readiness validation | Approval, balance, account, period, fiscal, duplicate, currency, and mandatory-reference validation outcomes for included Journals | Accounting & General Ledger |
| Preserve execution outcome | Batch lifecycle history, completion or failure result, and complete ledger consequence or restoration outcome | Accounting & General Ledger |
| Preserve retry lineage | Original failure, corrected condition, retry lifecycle occurrence, and subsequent result | Accounting & General Ledger |
| Preserve completed accounting facts | Applicable Posting Batch, Journal, and ledger events with immutable audit history | Accounting & General Ledger |

Section A retains ownership of authorization and audit infrastructure; Accounting owns the batch decisions, financial evidence, and accounting records.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Posting Batch contains one or more approved Journal Entries with deterministic membership and order.
- The complete batch has passed readiness validation before processing.
- Exactly one governed batch result is recognized: every included Journal reaches `Posted` and the Posting Batch reaches `Completed`, or the Posting Batch reaches `Failed` with no retained partial ledger consequence.
- On completion, all affected Ledger Accounts and the General Ledger reflect the full balanced outcome and applicable completed events are published.
- On failure, financial consistency is restored and the failure and recovery evidence are preserved without inventing an event-publication decision.
- The complete batch lifecycle and accounting audit history are reconstructable.

## 16.2 Postconditions

- A `Completed` Posting Batch and all included `Posted` Journals are immutable.
- A failed batch leaves no recognized partial Journal or ledger consequence from the attempted execution.
- A `Failed` batch may progress to `Retried` only through the source-defined recovery lifecycle after correction of the failure cause.
- Completed ledger facts are available for later Trial Balance, reconciliation, reporting, analytics, treasury, budget, and audit outcomes.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F05` defines `PostingBatchFailed` as a Posting Batch event, while Section 37.1 states that failed transactions shall not publish business events. | Section 14 cannot determine whether the canonical failure fact must be published or suppressed. This materially affects event correctness and prevents locking. | Accounting & General Ledger owner and Lead Architect must reconcile the Posting Batch event catalog with the domain event-publishing rule. |
| The Posting Batch lifecycle defines `Failed -> Retried -> Completed` but does not define `Retried -> Processing`, a second validation position, or a transition for a retry that fails. | The complete business recovery lifecycle cannot be represented without inventing a direct success assumption or an unsupported repeat-failure transition. | Accounting & General Ledger owner must complete the retry lifecycle and define repeat-failure handling in authoritative L2 architecture. |
| `L2-F05` assigns no canonical business role to batch creation, validation, execution, failure determination, or retry authorization. | Sections 6 and 15 can preserve Accounting ownership but cannot establish complete business authority or separation responsibilities for batch operations. | Accounting & General Ledger owner must define or confirm the configuration-governed business authority mapping for Posting Batch operations. |
| `L2-F05` uses â€œatomic,â€ â€œrollback,â€ and â€œscheduled accounting jobsâ€ within the Posting Batch definition but does not fully express their institutional business meaning or evidence requirements. | This Draft can express only the supported business invariant of complete success or no retained partial financial consequence; later behaviour specifications lack a fully normalized business contract. | Accounting & General Ledger owner should clarify the business semantics and evidence of complete batch failure recovery without delegating business decisions to later technical design. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

## Draft Validation Classification

**BLOCKED** - The document is complete as a Template v2.0 Draft, but the authoritative architecture conflicts on publication of `PostingBatchFailed`, leaves the retry lifecycle incomplete, and does not define the required batch-operation authority. These material event, lifecycle, and authorization gaps prevent Architecture Approval and locking.

---

**End of Scenario L3-F-027**
