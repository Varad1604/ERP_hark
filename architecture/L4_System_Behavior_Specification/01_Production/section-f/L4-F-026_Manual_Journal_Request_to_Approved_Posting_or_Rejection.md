# Manual Journal Request to Approved Posting or Rejection

## 1. Specification Metadata

**Specification ID:** L4-F-026  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounting & General Ledger  
**Realizes Scenario:** L3-F-026  
**Primary Business Owner:** Accounting & General Ledger  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-29  
**Depends On:** `L3-F-026_Manual_Journal_Request_to_Approved_Posting_or_Rejection.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorized manual Journal request through preparation, validation, independent accounting review, and one of two valid terminal outcomes: successful posting of an approved Journal or conclusive rejection with no General Ledger effect.

The behavior begins when an authorized manual accounting requirement exists with supporting business justification. It ends when the Journal Entry reaches either `Posted` or terminal `Rejected`.

This specification does not correct the source business transaction, own or transition the Financial Period lifecycle, govern later Journal reversal, or prescribe technical accounting execution.

---

## 3. Required Business Context

The behavior requires:

- an authorized manual accounting requirement;
- a valid business reason and accounting references;
- Ledger Accounts and amount/currency context;
- an Accountant acting as Journal preparer;
- a Senior Accountant acting as independent reviewer/decision authority;
- an authorized posting actor for the approved path;
- the authoritative Financial Period state supplied by F09;
- the governing Financial Period being `Open` when posting is required.

Accounting & General Ledger owns the Journal Entry and its final disposition. F09 retains ownership of the governing Financial Period lifecycle. Any referenced source transaction remains owned by its source capability.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-026-BH-001 | When an authorized manual accounting requirement is submitted, the system shall permit creation of a Journal Entry with its business justification and source references. | L3-F-026 §§4.1, 7-9 step 1 |
| L4-F-026-BH-002 | The system shall prevent Journal validation from succeeding unless total debit equals total credit. | L3-F-026 §§9 step 2, 12 |
| L4-F-026-BH-003 | The system shall prevent Journal validation from succeeding when required Ledger Accounts are not eligible. | L3-F-026 §§7, 9 step 2, 12 |
| L4-F-026-BH-004 | When posting eligibility is evaluated, the system shall consume the authoritative F09-owned Financial Period state and require `Open` for the posting path. | L3-F-026 §§5, 7, 9 step 2, 12 |
| L4-F-026-BH-005 | When all required validation conditions succeed, the system shall move the Journal Entry from `Draft` to `Validated`. | L3-F-026 §§9 step 2, 13 |
| L4-F-026-BH-006 | The system shall require independent Senior Accountant review before a validated Journal can reach either `Approved` or `Rejected`. | L3-F-026 §§6, 9 step 3, 12, 15 |
| L4-F-026-BH-007 | The system shall prevent the Journal preparer from satisfying the independent approval responsibility by preparation alone. | L3-F-026 §§6, 12, 15 |
| L4-F-026-BH-008 | When the independent review outcome is approval, the system shall move the Journal Entry from `Validated` to `Approved`. | L3-F-026 §§9 step 4, 13 |
| L4-F-026-BH-009 | The system shall prevent posting of a Journal Entry that has not reached `Approved`. | L3-F-026 §§9 steps 4-5, 12 |
| L4-F-026-BH-010 | When an authorized posting actor posts an eligible `Approved` Journal in an `Open` Financial Period, the system shall move the Journal Entry to `Posted`. | L3-F-026 §§6-7, 9 step 5, 12-13 |
| L4-F-026-BH-011 | On successful posting, the system shall expose the resulting ledger consequence as complete and authoritative. | L3-F-026 §§3, 9 step 5, 16 |
| L4-F-026-BH-012 | When the independent review outcome is rejection, the system shall move the Journal Entry from `Validated` to terminal `Rejected`. | L3-F-026 §§3, 9 step 6, 10.1, 12-13 |
| L4-F-026-BH-013 | When a Journal reaches `Rejected`, the system shall prevent any General Ledger consequence from that Journal instance. | L3-F-026 §§3, 9 step 6, 10.1, 12, 16 |
| L4-F-026-BH-014 | The system shall prevent a terminally `Rejected` Journal from being resurrected for later posting; a later attempt must be a new Journal Entry. | L3-F-026 §§10.1, 12 |
| L4-F-026-BH-015 | After successful posting, the system shall preserve the posted Journal as immutable accounting history. | L3-F-026 §12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-026-BH-016 | When Journal validation completes successfully, the system shall publish `JournalValidated` with Accounting & General Ledger retained as the canonical publisher. | L3-F-026 §14 |
| L4-F-026-BH-017 | When independent approval is recorded, the system shall publish `JournalApproved` with Accounting & General Ledger retained as the canonical publisher. | L3-F-026 §14 |
| L4-F-026-BH-018 | When rejection is conclusively recorded, the system shall publish `JournalRejected` with Accounting & General Ledger retained as the canonical publisher. | L3-F-026 §§10.1, 13-14 |
| L4-F-026-BH-019 | When an approved Journal is successfully posted, the system shall publish `JournalPosted` with Accounting & General Ledger retained as the canonical publisher. | L3-F-026 §14 |
| L4-F-026-BH-020 | The system shall consume the F09-owned Financial Period state for posting eligibility without republishing or assuming ownership of that lifecycle. | L3-F-026 §§5, 7, 12 |

No cross-domain consumption transfers ownership of the Journal Entry or Financial Period lifecycle.

---

## 6. Failure / Rejection / Continuation

- If double-entry balance, account eligibility, required references, or other validation conditions fail, the system shall prevent progression from `Draft` to `Validated`.
- If review does not result in approval, the system may reach the governed terminal `Rejected` outcome.
- A `Rejected` Journal shall have no ledger effect.
- A rejected Journal instance shall not later continue to approval or posting; a renewed business attempt requires a new Journal Entry.
- If an approved Journal cannot satisfy posting eligibility because the Financial Period is not `Open`, the system shall prevent posting and retain the last valid governed Journal position.
- A business interruption shall preserve the current governed Journal state and decision evidence.

No technical retry, rollback, persistence, queue, or transaction-recovery mechanism is specified here.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to reconstruct:

- the manual accounting requirement and business justification;
- Journal identity and source references;
- debit/credit lines and balance validation;
- Ledger Account eligibility;
- the F09 Financial Period state used for posting eligibility;
- the Accountant who prepared the Journal;
- the Senior Accountant who independently reviewed it;
- the approval or rejection decision;
- the authorized posting actor where applicable;
- progression through `Draft`, `Validated`, `Approved`, `Posted`, or terminal `Rejected`;
- the presence or absence of ledger consequence corresponding to the final disposition;
- the occurrence of `JournalValidated`, `JournalApproved`, `JournalRejected`, and `JournalPosted` where applicable.

Evidence shall remain implementation-independent.

---

## 8. Observable Completion Conditions

The behavior is complete through one of two valid terminal outcomes:

**Posted outcome**
- the Journal has completed validation;
- independent approval is positive;
- the governing Financial Period was `Open`;
- an authorized posting actor has posted the Journal;
- the Journal Entry is `Posted`;
- the ledger consequence is complete and authoritative.

**Rejected outcome**
- the Journal has completed validation and independent review;
- the review outcome is rejection;
- the Journal Entry is terminally `Rejected`;
- the rejection decision is preserved;
- the General Ledger remains unchanged by that Journal instance.

In both outcomes, authority, state, and decision evidence must be reconstructable.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-026` — Manual Journal Request to Approved Posting or Rejection  
**Primary verification source:** `L2-F05` — Accounting & General Ledger Management  
**Material amendment:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Additional verification source:** `L2-F09` — Financial Period & Closing amendment  

Behavioral provenance is anchored in `L3-F-026`.

The locked F05 amendment verifies actor responsibilities, maker-checker separation, the canonical `Rejected` state, the `Validated → Rejected` transition, `JournalRejected`, the no-ledger-effect rule for rejection, and the rule that a later attempt requires a new Journal instance.

No L4 behavior in this specification creates a new accounting rule, actor, authority, Financial Period state, Journal state, event, publisher, or ownership assignment.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the supplied parent semantics.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/rollback/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to downstream architecture levels.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
