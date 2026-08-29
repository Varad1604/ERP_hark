# Accounting Batch to Governed Posting Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-F-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Govern a Posting Batch containing approved Journals so the batch reaches one auditable outcome without retaining a partial ledger consequence.

---

# 3. Business Outcome

The Posting Batch is either `Completed` with all included approved Journals posted consistently, or `Failed` with no partial authoritative ledger consequence; an authorized retry may re-enter the governed posting lifecycle.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Posting Batch has been formed from one or more approved Journals eligible for posting.

## 4.2 End Boundary

The batch reaches `Completed`, or reaches a conclusively recorded `Failed` outcome awaiting or ending governed retry action.

## 4.3 In Scope

Batch validation, authorization, posting, failure fact, governed retry and audit evidence.

## 4.4 Out of Scope

Creation/approval of individual Journals, Financial Period lifecycle ownership and technical retry-count or transaction mechanics.

---

# 5. Ownership and Participating Domains

Accounting & General Ledger owns the Posting Batch and ledger consequence. F09 owns the period/year lifecycle whose `Open` status is consumed for posting eligibility.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Senior Accountant | Authorizes the Posting Batch and any governed retry. | Cannot bypass Journal approval or period eligibility. |
| Authorized Accountant or Senior Accountant | Executes the posting action for the approved batch. | Existing posting authority. |
| Internal Auditor | Examines batch outcome evidence. | Read-only. |

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §2.

---

# 7. Preconditions and Entry Criteria

- Every batch member is an approved Journal.
- Batch membership is defined and traceable.
- The governing Financial Period is `Open`.
- Required Ledger Accounts remain eligible.
- Batch authorization is available.

---

# 8. Trigger

An authorized Posting Batch is released for governed accounting posting.

---

# 9. Main Business Flow

1. Accounting validates batch membership, Journal approval, posting eligibility and the F09-owned period status.
2. The batch progresses `Created → Validated → Processing`.
3. All included Journals are posted as one governed batch outcome.
4. If every posting completes consistently, the batch reaches `Completed` and applicable Journal/ledger facts are published.
5. If the posting cannot complete consistently, no partial authoritative ledger consequence is retained and the batch reaches `Failed`.
6. `PostingBatchFailed` records the conclusively governed batch failure fact; it is not a successful financial-posting event.
7. When an authorized retry is approved, the batch progresses `Failed → Retried → Validated → Processing` and is assessed again under the same business controls.

---

# 10. Alternate Business Flows

## 10.1 Governed Retry

A failed batch may be retried after the underlying business condition has been resolved and Senior Accountant authorization is recorded. A repeat failure returns the batch to `Failed`.

---

# 11. Failure, Interruption and Business Recovery

- A failed batch preserves financial consistency and does not produce partial authoritative ledger state.
- `PostingBatchFailed` may be published only after the failure outcome is conclusively recorded.
- A retry is a business-governed lifecycle transition, not a technical retry specification.

---

# 12. Business Rules and Decision Constraints

- Only approved Journals may belong to the posting batch.
- Batch posting must preserve double-entry and ledger consistency.
- Failed financial posting does not publish success or ledger-state events.
- A conclusively recorded batch-lifecycle failure may publish `PostingBatchFailed`.
- Retry requires Senior Accountant authorization.
- Posting eligibility depends on the F09-owned Financial Period being `Open`.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§4 and 8; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§3 and 5.

---

# 13. State and Lifecycle Impact

`Created → Validated → Processing → Completed`

`Processing → Failed → Retried → Validated → Processing`

Repeat failure returns to `Failed`.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §4.

---

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Meaning |
|---|---|---|
| `PostingBatchValidated` | Accounting & General Ledger | Batch is eligible to proceed. |
| `PostingBatchStarted` | Accounting & General Ledger | Governed batch posting has begun. |
| `PostingBatchCompleted` | Accounting & General Ledger | Batch completed consistently. |
| `PostingBatchFailed` | Accounting & General Ledger | Batch concluded in a governed failure with no partial ledger consequence. |
| `PostingBatchRetried` | Accounting & General Ledger | An authorized retry has been initiated. |

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §4 and unaffected canonical L2-F05 event definitions.

---

# 15. Business Authorization, Approval and Audit Obligations

Batch and retry authorization must be attributable. Evidence must preserve batch identity, members, governing period, authorizing actor, posting actor, completed/failed outcome and any retry cycle.

---

# 16. Completion Criteria and Postconditions

A completed path has all included Journals posted consistently. A failed path has no retained partial ledger consequence and a preserved failure fact. Any later retry remains governed by the same lifecycle and authority rules.

---

# 17. Open Issues, Assumptions and Deferred Questions

No material upstream blocker remains after the 2026-08-21 F05 locked amendment.

---

**End of Scenario L3-F-027**