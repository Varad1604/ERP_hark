# ADR-FIN-GL-001 — Accounting & General Ledger Posting and Close (Decision Package FIN-GL)

| Attribute | Value |
|---|---|
| ADR ID | ADR-FIN-GL-001 |
| Decision Package | FIN-GL (per `L3_Phase5_Chetan_Upstream_Architecture_Decision_Proposal_2026-08-20.md`, Section 4) |
| Target Document | `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F05_Accounting_and_General_Ledger_Management.md` (Status: Locked, Version 1.0) |
| Blocked Downstream Documents | `L3-F-025` through `L3-F-032` (8 scenarios, all Draft, Owner: Chetan; five explicitly marked `BLOCKED` in their own Draft Validation Classification) |
| Status | **Proposed — Non-Authoritative** |
| Decision Effect | None until approved by the named repository roles and incorporated into a re-locked `L2-F05` |
| Domain Owner | Finance — Accounting & General Ledger (Chetan) |
| Date | 2026-08-21 |
| Prepared By | Drafted on request, for review by Chetan (Finance domain owner), Dhruv (Lead Architect, lock approval), and the Finance Review Team (Varad, Devang) per `governance/constitution/Volume_5_Change_Management.md` §5. Template and drafting discipline follow `ADR-FIN-ADJ-001`. |

---

## 1. Need

`L2-F05` is Locked but its own Canonical Section Crosswalk records one gap (`Locked Decisions: — not present —`), and — more materially — the document contains **internal contradictions**, not just omissions. Two independent descriptions of the same Accounting Period lifecycle disagree with each other, a domain-wide event rule is directly contradicted by a specific aggregate's event catalogue, and several lifecycles are incomplete for outcomes the document itself requires (rejection, partial reversal, retry, archival).

Eight drafted L3 scenarios (`L3-F-025`–`L3-F-032`) already isolate these exact gaps; five are explicitly self-classified `BLOCKED`:

| Scenario | Blocking gap traceable to `L2-F05` |
|---|---|
| `L3-F-025` Financial Transaction → Balanced Ledger Posting | §48 defers the exact upstream triggering event to an "Enterprise Event Catalog (L3)" that doesn't exist as an authoritative source; posting action has no assigned canonical role |
| `L3-F-026` Manual Journal Request → Approved Posting or Rejection | Journal Entry lifecycle (§28.6) has no `Rejected` state despite `Reject Journal` command (§28.7) and `JournalRejected` event (§28.8) existing — **BLOCKED** |
| `L3-F-027` Accounting Batch → Governed Posting Outcome | `PostingBatchFailed` (§30) directly contradicts §37.1 ("failed transactions shall not publish business events"); retry lifecycle `Failed → Retried → Completed` has no `Retried → Processing` or repeat-failure transition — **BLOCKED** |
| `L3-F-028` Posted Journal → Accounting Reversal | Partial reversal is required and requires authorization (BR-053) but the Journal Entry lifecycle has only `Posted → Reversed`, no partial-reversal state |
| `L3-F-029` Accounting Source and Ledger → Reconciled Position | No Reconciliation aggregate, record, or lifecycle exists anywhere in `L2-F05` despite a full reconciliation workflow (§35.9) and event (`LedgerReconciled`) — **BLOCKED** |
| `L3-F-030` Accounting Period Activity → Trial Balance | Trial Balance Snapshot lifecycle (§33) has no failed/invalid state for a debit-credit imbalance; event-consumption sequencing (`Generated` vs `Verified` vs `Published`) is undefined — **BLOCKED** |
| `L3-F-031` Ledger Period → Controlled Accounting Close | **The central FIN-GL ambiguity**: §24.7 (`Closed → Reopened → Closed`, no `Locked` state) directly contradicts §31 (`Closed → Locked → Archived` and separately `Closed → Reopened → Open`) for the same Accounting Period aggregate |
| `L3-F-032` Closed Ledger Evidence → Auditable Financial History | Archival scope/sequencing across six different aggregates is undefined; Journal lifecycle has no `Reversed → Archived` transition; most archival events are missing; independently re-discovers the §24.7/§31 terminology inconsistency (there labeled `Generating` vs `Generated` for Trial Balance Snapshot, a distinct but analogous contradiction) |

As with FIN-ADJ, every L3 draft correctly stopped short of inventing an answer. This ADR is the proposal that lets a real answer be reviewed and, if approved, locked.

---

## 2. Proposal

Every proposed answer below either (a) picks one of two existing-but-conflicting descriptions already in `L2-F05` and states why, or (b) extends an existing lifecycle/category list the document already treats as extensible (mirroring the reuse discipline `ADR-FIN-ADJ-001` established), or (c) is flagged as a genuine open decision rather than drafted. None introduces a new aggregate.

### 2.1 Actors and Authority (fills the recurring "posting authority not assigned" gap in all eight scenarios)

`L2-F05` §39.4 already lists typical, institution-configurable roles (Accountant, Senior Accountant, Finance Manager, Chief Financial Officer, Internal Auditor, System Administrator). What is missing is not a role list — it is which of those roles holds which of the specific decision points every scenario needs. Proposal: bind the existing roles to the specific actions, explicitly preserving §39.4's "configurable" framing (this maps typical defaults, it does not mint new immutable role names):

| Business Action | Bound Role (default, institution-configurable) | Scenarios |
|---|---|---|
| Prepare / create a Journal Entry | Accountant | 025, 026, 028, 029, 030, 032 |
| Independently review and approve/reject a Journal Entry (maker-checker) | Senior Accountant | 025, 026, 028 |
| **Post** an approved Journal Entry or Posting Batch | Accountant or Senior Accountant acting under system-enforced posting permission (§39.3 "Post Journal" permission) — posting is an authorized *action*, not a distinct *role*; whichever actor holds the "Post Journal" permission for that institution may execute it once the prior gates (validation, approval, period-open) are satisfied | 025, 026, 027 |
| Authorize a Posting Batch or its retry | Senior Accountant | 027 |
| Authorize a partial reversal (BR-053) | Finance Manager | 028 |
| Perform / review reconciliation and accept discrepancy resolution | Accountant (perform) / Senior Accountant (review and closure) | 029 |
| Generate / verify / publish a Trial Balance Snapshot | Accountant (generate) / Senior Accountant (verify) — publication is a system action following verification, not a separate decision | 030 |
| Approve Accounting Period close and lock | Finance Manager | 031 |
| Confirm closed-ledger evidence as auditable financial history | Finance Manager | 032 |
| Independent audit examination (never modification) | Internal Auditor | all |

This resolves the "posting not assigned to one canonical role" line item in `L3-F-025`, `026`, `027`, `029`, `030`, `031`, and `032` by binding §39.3's existing permission list to §39.4's existing role list — no new permission or role name is introduced.

### 2.2 Journal Entry `Rejected` state (resolves `L3-F-026`, the sharpest lifecycle gap)

`L2-F05` §28.7 already lists `Reject Journal` as a command and §28.8 already lists `JournalRejected` as an event — both exist. Only the lifecycle diagram at §28.6 is incomplete. Proposal: add `Rejected` as a terminal state reachable from `Validated`, parallel to how `Posted` is reachable from `Approved`:

```
Draft → Validated → Approved → Posted → Reversed
                  ↳ Rejected
Draft → Validated → Approved → Posted → Archived
```

- `Validated → Rejected`: the maker-checker reviewer declines the Journal. `JournalRejected` publishes. No ledger consequence.
- `Rejected` is terminal for that Journal Entry instance. Resubmission is a **new** Journal Entry instance (mirrors `ADR-FIN-ADJ-001` §2.2's "rejection is terminal; resubmission is a new instance" pattern) — this keeps `Rejected` outside the immutability constraints that apply to `Posted`/`Reversed` (BR-007) since a rejected Journal never became an authoritative accounting fact.

This directly resolves `L3-F-026`'s blocking issue and its dependent "resubmission path" question.

### 2.3 Posting Batch: event rule conflict and retry lifecycle (resolves `L3-F-027`)

**`PostingBatchFailed` vs §37.1.** §37.1's "failed transactions shall not publish business events" is a *domain-wide* principle written for individual financial postings (its context is Journal/Ledger posting, where a failed posting must leave no trace precisely because nothing happened). A Posting Batch's `Failed` outcome is different in kind: the batch itself completed its governed lifecycle (validation → processing → failure-detection → complete withdrawal) and that *governed completion* is the fact being published — not the underlying financial transaction. Proposal: `PostingBatchFailed` is retained as canonical, publishing only after the complete withdrawal/rollback is confirmed; it represents "the batch process concluded, with this governed outcome," not "a financial posting succeeded." §37.1 is clarified (not contradicted) to read as applying to the *financial posting fact*, not to *process-lifecycle facts* about batches, reconciliations, or other coordination aggregates. Consumers: Accounting & General Ledger, Audit & Compliance (matching the treatment already given to `PostingBatchRetried`).

**Retry lifecycle completion.** §30's `Failed → Retried → Completed` is missing the step between `Retried` and re-attempted processing. Proposal, reusing the batch's own existing states rather than inventing new ones:

```
Failed → Retried → Validated → Processing → Completed
                                          ↳ Failed (repeat failure re-enters this same cycle)
```

`Retried` re-enters the existing `Validated → Processing` path (both states already exist in §30's primary lifecycle); a repeat failure produces `Failed` again, and the batch may be retried again — the cycle is bounded only by the institution's own retry-count policy, which is explicitly out of scope for L2/L3 business architecture (implementation-level, per the source proposal's own §10 acceptance criterion excluding "retry-count... infrastructure design").

### 2.4 Partial reversal lifecycle (resolves `L3-F-028`)

`L2-F05` supports partial reversal (BR-053, §34.4) but the Journal Entry lifecycle only has `Posted → Reversed`. Applying the same reasoning `ADR-FIN-ADJ-001` §2.2 used for Financial Adjustment reversal: **a reversal — full or partial — is represented by a new compensating Journal Entry, never by a state change on the original beyond what full reversal already does.** Proposal:

- **Full reversal**: original moves `Posted → Reversed` (already defined). This is correct because the original's *entire* financial effect has been neutralized — `Reversed` accurately describes its current net effect.
- **Partial reversal**: the original **remains `Posted`** (not `Reversed`) because part of its financial effect is still authoritative and standing. The compensating Journal (carrying a mandatory reference to the original, per BR-051) is the sole record of the partial reversal; the original's immutable line items are unchanged and continue to speak for themselves alongside the compensating entry. No new state is added to the Journal Entry lifecycle.
- This also resolves the "authorization sequencing" question in `L3-F-028`'s Section 17: partial-reversal authorization (BR-053, bound to Finance Manager per §2.1) is a precondition gate *before* the compensating Journal is even created — analogous to how enterprise approval is a precondition for publication in FIN-ADJ. The compensating Journal then follows the ordinary `Draft → Validated → Approved → Posted` path with its own maker-checker review; the two are sequential, not parallel or duplicate, decisions.

### 2.5 Reconciliation Record (resolves `L3-F-029`)

No aggregate owns `LedgerReconciled`. Proposal: introduce **Reconciliation Record** as a new supporting aggregate under Accounting & General Ledger ownership (consistent with §22's aggregate-overview pattern), minimal by design:

- **Owns**: reconciliation scope (Accounting Source identity, corresponding ledger position, accounting period, population boundary), comparison result, identified differences and their resolution references, closure evidence.
- **Does not own**: the source financial fact (remains with its source-owning capability, per the existing §5/§77 pattern used throughout `L2-F05`), any correction itself (corrections remain governed by their own owning workflow — Journal reversal, adjustment, etc.).
- **Lifecycle** (new, minimal — mirrors Trial Balance Snapshot's shape since both are derived/verification aggregates): `Comparing → Reconciled` (no difference found or all differences resolved) or `Comparing → Exception` (unresolved difference; non-terminal, returns to `Comparing` after the owning workflow resolves the difference and comparison is repeated).
- **Events**: `LedgerReconciled` (already canonical, now has an owning aggregate and fires on `Comparing → Reconciled`) plus new `ReconciliationExceptionRaised` (fires on `Comparing → Exception`, consumed by Audit & Compliance and the source-owning capability, mirroring the notification pattern already used for `PostingBatchFailed` in §2.3).
- **Accounting Source contract**: rather than enumerate every source contract (Ledger, Bank, control account, receivables, payables — each a different owning capability), proposal is to require that *any* Accounting Source used for reconciliation supply: business reference, amount, classification, and accounting chronology — the same four elements §38.5's generic Integration Contract already mandates for every upstream domain. No new contract type is introduced; reconciliation reuses the existing Integration Contract.
- Discrepancy categories, tolerance/materiality rules, and timing-difference rules remain a genuine open item — flagged, not drafted, since they are policy decisions (e.g., what tolerance is acceptable) rather than architecture gaps.

### 2.6 Trial Balance Snapshot: failure state and terminology (resolves `L3-F-030`, part of `L3-F-032`)

- **Terminology inconsistency** (also independently flagged by `L3-F-032`): §24.9 uses `Generated` as the entry state; §33 uses `Generating`. Proposal: `Generating` is correct — it is a state name (a condition something is *in*), matching the verb-form pattern used by every other lifecycle in this document (`Closing`, `Processing`, `Validating`). `Generated` in §24.9 is a copy-edit inconsistency, not a second intended state. §24.9 is corrected to read `Generating` to match §33.
- **Failure state**: add `Invalid` as a terminal, non-`Published` state reachable from `Generating`, mirroring `Rejected`'s relationship to Journal Entry in §2.2:

```
Generating → Verified → Published → Archived
          ↳ Invalid  (debit ≠ credit, or ledger lineage cannot be traced)
```

`Invalid` snapshots are retained (not deleted, consistent with §36.9's audit-trail principles) as evidence of the failed generation attempt but are never published and never contribute to reporting. A corrected generation attempt is a **new** Snapshot instance — historical Snapshots are never recalculated (existing §33 rule), so `Invalid` cannot transition to `Verified`.
- **Event-consumption eligibility**: `Financial Reporting` (§37.8) consumes `TrialBalancePublished`, not `TrialBalanceGenerated` or `TrialBalanceVerified` — publication is the point at which the snapshot becomes the authoritative read-only fact (§33 "Published snapshots support financial reporting and audit"); the earlier two events are internal lifecycle milestones consumed only by Accounting & General Ledger itself and Audit & Compliance.
- **Canonical scope/identity**: a Trial Balance Snapshot's identity is the tuple (Accounting Ledger, institution, Accounting Period or as-of boundary, currency context) — reusing exactly the dimensions §38.5's Integration Contract already requires as mandatory information, rather than inventing new identity fields.

### 2.7 Accounting Period lifecycle: `Closed` / `Locked` / `Reopened` (the FIN-GL centerpiece — resolves `L3-F-031`)

Three fragments of `L2-F05` currently disagree or leave a gap:
1. §24.7: `Planned → Open → Closing → Closed → Reopened → Closed` (no `Locked` state at all).
2. §31: `Planned → Open → Closing → Closed → Locked → Archived`, separately `Closed → Reopened → Open`.
3. §25.7 lists an `Unlock Accounting Period` command among the Accounting Ledger's aggregate commands — but neither diagram defines any transition *out of* `Locked` that this command could execute.

`L3-F-031` itself already implicitly adopted §31 over §24.7 (its own Section 13 cites `Closed -> Reopened -> Open` and `Locked -> Archived` as "the authoritative lifecycle") while flagging the immediate-locking-timing question and the orphaned reopening-from-Locked question. This ADR confirms and completes that choice:

**Proposed canonical Accounting Period lifecycle:**

```
Planned → Open → Closing → Closed → Locked → Archived
                            Closed → Reopened → Open
                            Locked → Reopened → Open   (governed unlock)
```

- §24.7 is corrected to match §31 (its `Reopened → Closed` edge is removed; `Reopened` always returns to `Open`, since a reopened period is, by definition, open for the governed correction it was reopened for).
- **`Closed` and `Locked` are separate governed decisions, not one atomic transition.** `Closed` means new postings are rejected (BR-030) but the period has not yet received its final protection; `Locked` means modifications are rejected outright (BR-031) — a strictly stronger protection. This gives institutions a governed interval in `Closed` (e.g., for final audit review) before the irreversible-in-practice `Locked` state, which resolves `L3-F-031`'s "must locking be immediate" question: no, it is a distinct, separately authorized action.
- **`Locked → Reopened → Open` is the transition `Unlock Accounting Period` (§25.7) executes.** This resolves the previously-orphaned command: unlocking a Locked period is exceptional and requires the same (or stricter — a genuine open item, not drafted) authority as reopening a merely-Closed period, per §39.6 ("Reopening requires explicit approval... Year-end closure supersedes period controls").
- **BR-034 minimum evidence for closure**: `Verified` is the required Trial Balance Snapshot state for closure eligibility — BR-034's literal text is "Trial Balance shall be verified before closure," and `Published` (§33's next state) governs external/downstream availability, not internal closure-readiness. Publication is not required to close a period.
- **`AccountingPeriodLocked` consumers**: extends beyond Audit & Compliance to also include Financial Reporting (a Locked period's figures are now the institution's non-negotiable historical position for that boundary — Financial Reporting needs to know this the same way it needs `AccountingPeriodClosed`).

### 2.8 Archival scope and events (resolves `L3-F-032`)

- **`HistoricalArchive` eligibility for the Accounting Ledger** (§25.6): the Accounting Ledger itself is continuous and never archived as a whole (§25.6 already states this explicitly — "never deleted or recreated"). `HistoricalArchive` in §25.6's diagram is proposed to mean the Ledger *entering a state where it contains at least one archived period boundary*, not the Ledger itself becoming historical — i.e., it is a derived/reporting condition of the Ledger, not a terminal lifecycle position competing with `Active`. The Ledger remains `Active` and `HistoricalArchive`-flagged simultaneously; these are not mutually exclusive per §25.5's existing "Governance State" category, which already tracks such metadata separately from "Operational State."
- **Archival sequencing**: proposal is dependency order, not simultaneity — Trial Balance Snapshot and Journal Entries for a period must reach their own terminal/archival-eligible position *before* the Accounting Period itself is marked `Archived` (i.e., `Locked → Archived` per §2.7 requires the period's Trial Balance to already be `Published` and its Journals to already be `Posted`/`Reversed`/`Rejected` — no `Draft` or `Validated` Journal may remain, consistent with BR-033's pre-closing requirement already forcing this before `Closed` is even reached).
- **`Reversed → Archived` for Journal Entries**: added as a valid transition, parallel to the existing `Posted → Archived` — a reversed Journal is exactly as immutable and archival-eligible as a posted one; the omission in §28.6 is a completeness gap, not a deliberate exclusion (nothing in §28's invariants treats `Reversed` differently from `Posted` for archival purposes).
- **Missing archival events**: add `AccountingPeriodArchived`, `LedgerAccountArchived`, `TrialBalanceArchived`, `FiscalYearArchived` — each following the exact naming and single-publisher pattern already used by `JournalArchived` (§28.8). No combined "historical evidence complete" event is introduced; `L3-F-032`'s outcome remains represented by the conjunction of these per-aggregate archival facts, consistent with this document's existing preference for per-aggregate events over cross-aggregate composite events (no such composite event exists anywhere else in `L2-F05`).
- **Retention/disposal policy, statutory retention period, examination-access contract**: genuine open items, flagged not drafted — these are institutional/regulatory policy decisions, not architecture gaps, and squarely match the source proposal's own exclusion of "no implementation-level... infrastructure design" plus a compliance-policy dimension that belongs with Audit & Compliance governance, not Accounting alone.

### 2.9 Generic upstream trigger for `L3-F-025` (resolves `L3-F-025`)

§48 explicitly defers the exhaustive operational-event-to-Journal mapping to an "Enterprise Event Catalog (L3)," stating this is a deliberate architectural choice, not an oversight — the mapping is source-domain-specific (Payments, Refunds, Adjustments, etc. each publish their own canonical completion events) and cataloguing all of them in `L2-F05` would violate this document's own bounded-context separation from those domains. Proposal: `L3-F-025` is confirmed as an intentionally **generic pattern scenario** — its Section 14.1 should state that the consumed event is *the canonical completion event published by whichever specific Finance capability is the operational transaction's owner* (e.g., a Payment domain event, a Refund domain event, `AdjustmentApplicationCompleted` from `L2-F04`/`ADR-FIN-ADJ-001`), not one single fixed event name. This is not a gap to fill with an invented event; it is confirmation that the deferral in §48 was correct and `L3-F-025` should encode that polymorphism explicitly rather than searching for a single answer that does not exist by design.

---

## 3. Impact Analysis

- **L1 boundary check**: same as `ADR-FIN-ADJ-001` §3 — Section F depends on Sections A, B, C; nothing here assigns Accounting-domain actors any authority over Section A-owned approval execution, audit infrastructure, or the Workflow Engine.
- **`DEPENDENCY_MAP.md`**: no entry names `L2-F05` specifically; nothing here conflicts with it.
- **Cross-package boundary flag — FIN-PERIOD overlap**: `L2-F05` owns "Accounting Period" and "Fiscal Year" as Accounting-domain aggregates (§22, §24.7/§31, §32). The separate Decision Package **FIN-PERIOD** (`L2-F09` "Financial Period and Closing," scenarios `L3-F-051`–`055`, not yet drafted as an ADR) governs "Financial Year and Financial Period" as an apparently enterprise-wide concept per the source proposal's Section 7. **This ADR does not resolve whether these are the same concept under two names, or whether Accounting Period is a derived/subordinate concept scoped within a Finance-wide Financial Period.** Recommend Architecture Review flag this explicitly before approving both ADR-FIN-GL-001 and a future ADR-FIN-PERIOD-001, since an inconsistent answer would create two competing period-governance models in the same Finance domain.
- **Ripple to declared integrations**: the new Reconciliation Record aggregate (§2.5) and its events are new inputs to Audit & Compliance; the four new archival events (§2.8) are new inputs to whichever domains already consume the non-archival counterpart of each aggregate's events (Financial Reporting, Audit & Compliance per §37.8/§41.6). No existing declared consumer relationship is altered.
- **Consistency with `ADR-FIN-ADJ-001`**: the "reversal/correction = new instance, not a state mutation on the original" pattern (§2.4 here) directly reuses the pattern `ADR-FIN-ADJ-001` §2.2 established for Financial Adjustment reversal — flagged there as a candidate reusable pattern, now the second confirmed application of it. Recommend Architecture Review treat this as an emerging cross-package convention worth stating explicitly once a third package confirms it (e.g., FIN-AP).
- **Minor doc-quality findings, not fixed here**: (a) the mermaid diagram at §47.3 references node `F08` without ever defining it (uses `F07[Procurement]` instead of a Section-F08/Accounts-Payable node) — cosmetic, flagged for correction alongside this amendment; (b) §36.8 security rule `BR-056` ("Maker and Checker shall be different users") is the same control already described narratively throughout §2.1–2.4 above by name rather than rule number — no action needed, noted for completeness.

---

## 4. Decision

*(Blank pending Architecture Review. Per `governance/constitution/Volume_5_Change_Management.md` §6, no implementation or Repository Update shall precede a recorded Decision here.)*

| Role | Name | Decision | Date |
|---|---|---|---|
| Finance domain/implementation owner | Chetan | — | — |
| Finance Review Team | Varad | — | — |
| Finance Review Team | Devang | — | — |
| Lead Architect (lock approval) | Dhruv | — | — |

Two items from Section 2 are deliberately left unresolved and are **not** approved by implication if this ADR is approved: the FIN-PERIOD boundary question (§3) and the discrepancy tolerance/materiality rules (§2.5).

---

## 5. Acceptance Checklist

Cross-referenced against the source proposal's Section 10 Acceptance Checklist (via Section 4's FIN-GL-specific requirements):

| Required element | Present in this ADR? |
|---|---|
| Which authoritative business fact initiates each scenario | Yes — §2.9 (generic trigger for 025); the rest already had defined triggers |
| Actor and authority separation for prep/review/post/close/lock/reopen/exception | Yes — §2.1 |
| Exact canonical GL lifecycle states, including whether Closed/Locked/Reopened are distinct | Yes — §2.7, the centerpiece |
| Exact canonical events and publishers for posting/rejection/close/lock/reopen/correction | Yes — §2.2, §2.3, §2.6, §2.8 |
| Relationship between journal/ledger-period/financial-period/financial-year state without merging ownership | Partial — journal/ledger-period resolved (§2.2, §2.7); financial-period vs Accounting Period relationship explicitly flagged unresolved (§3), not merged, not drafted |
| Minimum business evidence/checklist for close or reopen | Yes — §2.7 (BR-034 minimum), though full discrepancy tolerance rules remain open (§2.5) |
| No implementation-level API, persistence, queue, retry-count, idempotency, infrastructure design | Confirmed — §2.3 explicitly excludes retry-count policy; §2.8 explicitly excludes storage/retention implementation |
| Traceability from amended L2 material to every affected L3 scenario | Yes — §1 table maps every gap to `L3-F-025`–`032` |

Items not fully closed, and not approved by implication: the FIN-PERIOD boundary (§3), reconciliation tolerance/materiality rules (§2.5), retention/disposal policy (§2.8), and Locked-period unlock authority strictness (§2.7).
