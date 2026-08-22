# L2-F05 — Accounting & General Ledger Management — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F05 — Accounting & General Ledger Management v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-21 |
| Approved Through | ADR-FIN-GL-001 |
| Authority | Lead Architect approval recorded 2026-08-21 |

This amendment is part of the canonical L2-F05 architecture. Where this amendment conflicts with L2-F05 v1.0, this amendment is authoritative. Unaffected L2-F05 content remains unchanged.

## 1. Canonical Ownership Correction

L2-F05 owns Journal Entries, Journal Vouchers, Posting Batches, Accounting Ledger, Ledger Accounts, Chart of Accounts, Trial Balance Snapshots and accounting reconciliation records.

**Financial Year and Financial Period lifecycle governance are canonically owned by L2-F09 — Financial Period & Closing.** L2-F05 consumes the authoritative period/year state and enforces posting eligibility; it does not independently own or redefine those lifecycles. Any L2-F05 text claiming exclusive ownership of Accounting Period or Fiscal Year lifecycle is superseded by this rule.

## 2. Actors and Authority

Existing L2-F05 roles are bound as follows, subject to institution-configured authorization without changing business responsibility:

| Business action | Canonical business responsibility |
|---|---|
| Prepare Journal | Accountant |
| Review and approve/reject Journal | Senior Accountant; maker-checker separation applies |
| Post approved Journal or Posting Batch | Authorized Accountant or Senior Accountant holding the existing Post Journal permission |
| Authorize Posting Batch retry | Senior Accountant |
| Authorize partial reversal | Finance Manager |
| Perform reconciliation | Accountant |
| Review and close reconciliation | Senior Accountant |
| Generate Trial Balance | Accountant |
| Verify Trial Balance | Senior Accountant |
| Approve accounting close/lock participation | Finance Manager |
| Audit examination | Internal Auditor, read-only |

## 3. Journal Rejection

The canonical Journal Entry lifecycle includes `Rejected` as a terminal pre-posting outcome:

`Draft → Validated → Approved → Posted`

`Validated → Rejected`

`Posted → Reversed`

`Posted/Reversed → Archived`

`JournalRejected` is canonical and is published after rejection is conclusively recorded. A rejected Journal has no ledger effect. A later attempt is a new Journal instance.

## 4. Posting Batch Failure and Retry

`PostingBatchFailed` is canonical. L2-F05 §37.1 is clarified: a failed **financial posting** shall not publish a success or ledger-state event, but a conclusively recorded process-lifecycle failure may publish its failure fact.

Canonical retry lifecycle:

`Created → Validated → Processing → Completed`

`Processing → Failed → Retried → Validated → Processing`

A repeat failure returns to `Failed`. No partial ledger consequence survives a failed batch.

## 5. Reversal

Full and partial reversal are represented through new compensating Journal Entries referencing the original Journal. Historical Journal lines are never edited.

- Full reversal: after the compensating Journal is successfully posted, the original may be represented as `Reversed`.
- Partial reversal: the original remains `Posted` because part of its financial effect remains authoritative. The compensating Journal records only the approved portion.
- Partial reversal requires Finance Manager authorization before creation of the compensating Journal; the compensating Journal then follows the normal maker-checker and posting lifecycle.

No `Partially Reversed` state is introduced.

## 6. Reconciliation Record

A **Ledger Reconciliation Record** is a supporting aggregate owned by Accounting & General Ledger. It owns reconciliation scope, comparison result, identified differences, resolution references and closure evidence. It does not own source-domain facts or corrective transactions.

Canonical lifecycle:

`Comparing → Reconciled`

`Comparing → Exception → Comparing`

`LedgerReconciled` is published on successful reconciliation closure. `ReconciliationExceptionRaised` is published when an unresolved difference is formally recorded. Tolerance/materiality policies remain institution-governed and are not fixed by this amendment.

## 7. Trial Balance Snapshot

The canonical lifecycle is:

`Generating → Verified → Published → Archived`

`Generating → Invalid`

`Invalid` is terminal for that snapshot instance and is never published for financial reporting. A corrected attempt creates a new snapshot. `TrialBalancePublished` is the event that makes the snapshot eligible for Financial Reporting; `TrialBalanceGenerated` and `TrialBalanceVerified` remain lifecycle evidence.

The term `Generated` used as an entry-state elsewhere in L2-F05 is superseded by `Generating`.

## 8. Period and Close Participation

L2-F05 no longer owns the Financial Period lifecycle. For accounting participation, the authoritative period states are consumed from L2-F09. Accounting may post only when the governing Financial Period is `Open` and applicable Finance controls permit posting.

L2-F05 close readiness, reconciliation, Trial Balance verification and ledger evidence are Finance facts supplied to L2-F09's enterprise period-closing governance. L2-F05 does not independently reopen or archive the canonical Financial Period.

## 9. Archival

Archival is historical preservation, never deletion. Journal Entries may be archived after Posted or Reversed business effects are complete; Posting Batches after completion; Trial Balance Snapshots after publication; reconciliation records after closure; and Accounting Ledger evidence according to institutional retention governance. Archival never changes the underlying financial effect.

## 10. Locked Decisions

The rules in §§1–9 are Locked L2 decisions under ADR-FIN-GL-001. Downstream L3-F-025 through L3-F-032 shall consume these decisions and shall not redefine them.
