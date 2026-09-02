# Accounting Batch to Governed Posting Outcome

## 1. Specification Metadata

**Specification ID:** L4-F-027  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounting & General Ledger  
**Realizes Scenario:** L3-F-027  
**Primary Business Owner:** Accounting & General Ledger  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-29  
**Depends On:** `L3-F-027_Accounting_Batch_to_Governed_Posting_Outcome.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorized Posting Batch containing approved Journals so that the batch reaches one auditable accounting outcome without retaining a partial authoritative ledger consequence.

The behavior begins when a Posting Batch has been formed from one or more approved Journals eligible for posting. It ends when the batch reaches `Completed`, or reaches a conclusively recorded `Failed` outcome awaiting or ending governed retry action.

This specification does not create or approve individual Journals, own or transition the Financial Period lifecycle, define technical retry counts, or prescribe transaction/rollback mechanics.

---

## 3. Required Business Context

The behavior requires:

- a defined Posting Batch containing one or more Journal Entries;
- every batch member already being `Approved`;
- traceable batch membership;
- eligible Ledger Accounts;
- the authoritative F09-owned Financial Period state;
- the governing Financial Period being `Open`;
- batch authorization by the Senior Accountant;
- an authorized posting actor;
- the canonical Posting Batch lifecycle and event model.

Accounting & General Ledger owns the Posting Batch and resulting ledger consequence. F09 retains ownership of the Financial Period and Financial Year lifecycle.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-027-BH-001 | When a Posting Batch is formed, the system shall retain a traceable set of included Journal Entries for that batch outcome. | L3-F-027 §§4.1, 7, 9 step 1 |
| L4-F-027-BH-002 | The system shall prevent batch validation from succeeding when any included Journal is not `Approved`. | L3-F-027 §§7, 9 step 1, 12 |
| L4-F-027-BH-003 | The system shall prevent batch validation from succeeding when required Ledger Accounts are not eligible. | L3-F-027 §§7, 9 step 1 |
| L4-F-027-BH-004 | The system shall consume the authoritative F09-owned Financial Period state and prevent batch posting when the governing period is not `Open`. | L3-F-027 §§5, 7, 9 step 1, 12 |
| L4-F-027-BH-005 | When batch membership, Journal approval, ledger eligibility, period eligibility, and required authorization are valid, the system shall move the Posting Batch from `Created` to `Validated`. | L3-F-027 §§7, 9 steps 1-2, 13 |
| L4-F-027-BH-006 | When governed batch posting begins, the system shall move the Posting Batch from `Validated` to `Processing`. | L3-F-027 §§8-9 step 2, 13 |
| L4-F-027-BH-007 | The system shall treat all included Journals as one governed batch posting outcome for purposes of authoritative ledger consequence. | L3-F-027 §§2-3, 9 step 3, 11-12 |
| L4-F-027-BH-008 | When every included Journal posts consistently, the system shall move the Posting Batch from `Processing` to `Completed`. | L3-F-027 §§3, 9 step 4, 13, 16 |
| L4-F-027-BH-009 | On a `Completed` batch outcome, the system shall expose the resulting Journal and ledger facts as the authoritative completed posting consequence. | L3-F-027 §§3, 9 step 4, 16 |
| L4-F-027-BH-010 | When the batch cannot complete consistently, the system shall move the Posting Batch from `Processing` to `Failed`. | L3-F-027 §§3, 9 step 5, 11, 13 |
| L4-F-027-BH-011 | On a `Failed` batch outcome, the system shall prevent any partial authoritative ledger consequence from being retained as the batch result. | L3-F-027 §§2-3, 9 step 5, 11-12, 16 |
| L4-F-027-BH-012 | The system shall not expose successful posting or ledger-state completion facts for a failed financial posting. | L3-F-027 §§9 step 6, 11-12 |
| L4-F-027-BH-013 | When Senior Accountant retry authorization exists for a failed batch, the system shall move the batch from `Failed` to `Retried`. | L3-F-027 §§6, 9 step 7, 10.1, 12-13 |
| L4-F-027-BH-014 | After `Retried`, the system shall require the batch to re-enter `Validated` and then `Processing` under the same business controls before another governed posting outcome is established. | L3-F-027 §§9 step 7, 10.1, 13 |
| L4-F-027-BH-015 | If a retried batch fails again, the system shall return the Posting Batch to `Failed`. | L3-F-027 §§10.1, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-027-BH-016 | When batch validation completes successfully, the system shall publish `PostingBatchValidated` with Accounting & General Ledger retained as the canonical publisher. | L3-F-027 §14 |
| L4-F-027-BH-017 | When governed batch posting begins, the system shall publish `PostingBatchStarted` with Accounting & General Ledger retained as the canonical publisher. | L3-F-027 §14 |
| L4-F-027-BH-018 | When the Posting Batch reaches `Completed`, the system shall publish `PostingBatchCompleted` with Accounting & General Ledger retained as the canonical publisher. | L3-F-027 §14 |
| L4-F-027-BH-019 | When the Posting Batch reaches a conclusively recorded `Failed` outcome, the system shall publish `PostingBatchFailed` with Accounting & General Ledger retained as the canonical publisher. | L3-F-027 §§9 step 6, 11, 14 |
| L4-F-027-BH-020 | When an authorized retry moves the batch into `Retried`, the system shall publish `PostingBatchRetried` with Accounting & General Ledger retained as the canonical publisher. | L3-F-027 §§9 step 7, 14 |
| L4-F-027-BH-021 | The system shall consume F09-owned Financial Period state for eligibility without republishing or assuming ownership of that lifecycle. | L3-F-027 §§5, 12 |

`PostingBatchFailed` is a governed batch-lifecycle failure fact and is not a successful financial-posting event.

---

## 6. Failure / Rejection / Continuation

- If any included Journal is not approved, the system shall prevent batch validation.
- If Ledger Account eligibility or F09 period eligibility is not satisfied, the system shall prevent governed posting progression.
- If the batch cannot complete consistently, the system shall establish `Failed` as the governed outcome and shall not retain a partial authoritative ledger consequence.
- The system shall publish `PostingBatchFailed` only after the failure outcome is conclusively recorded.
- A failed batch may continue only through the source-defined governed retry lifecycle after Senior Accountant authorization.
- A repeat failure after retry returns the batch to `Failed`.
- Retry behavior here is a business lifecycle transition only; no technical retry count, rollback, or transaction-recovery design is defined.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to reconstruct:

- Posting Batch identity;
- complete batch membership;
- approval status of each included Journal at batch entry;
- Ledger Account eligibility;
- the F09-owned Financial Period and `Open` state used for eligibility;
- Senior Accountant batch authorization;
- posting actor identity;
- lifecycle progression through `Created`, `Validated`, `Processing`, `Completed`, `Failed`, and `Retried` where applicable;
- the completed or failed batch outcome;
- evidence that no partial authoritative ledger consequence remained after failure;
- every governed retry authorization and retry cycle;
- the occurrence of `PostingBatchValidated`, `PostingBatchStarted`, `PostingBatchCompleted`, `PostingBatchFailed`, and `PostingBatchRetried` where applicable.

Internal Auditor access remains read-only and does not alter ownership or outcome.

---

## 8. Observable Completion Conditions

The behavior is complete through one of two governed outcomes:

**Completed outcome**
- all included Journals were eligible and approved;
- the governing Financial Period was `Open`;
- the Posting Batch reached `Completed`;
- all included Journals posted consistently;
- the resulting ledger consequence is complete and authoritative;
- required batch completion evidence and events are reconstructable.

**Failed outcome**
- the Posting Batch reached a conclusively recorded `Failed` state;
- no partial authoritative ledger consequence remains;
- `PostingBatchFailed` represents the governed failure fact;
- any later retry is subject to Senior Accountant authorization and the canonical retry lifecycle.

A retried batch is not complete merely because `Retried` was entered; it must subsequently reach another governed `Completed` or `Failed` outcome.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-027` — Accounting Batch to Governed Posting Outcome  
**Primary verification source:** `L2-F05` — Accounting & General Ledger Management  
**Material amendment:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Additional verification source:** `L2-F09` — Financial Period & Closing amendment  

Behavioral provenance is anchored in `L3-F-027`.

The locked F05 amendment verifies Posting Batch ownership, retry authorization by Senior Accountant, the canonical lifecycle `Created → Validated → Processing → Completed` and `Processing → Failed → Retried → Validated → Processing`, the `PostingBatchFailed` event, and the rule that no partial ledger consequence survives a failed batch.

No L4 behavior in this specification creates a new retry rule, batch state, actor, authority, Financial Period state, event, publisher, or ownership assignment.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the supplied parent semantics.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry-count mechanics, rollback/transaction design, infrastructure/deployment, and UI/UX remain deferred to downstream architecture levels.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
