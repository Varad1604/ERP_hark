# Posted Journal to Accounting Reversal

## 1. Specification Metadata

**Specification ID:** L4-F-028  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounting & General Ledger  
**Realizes Scenario:** L3-F-028  
**Primary Business Owner:** Accounting & General Ledger  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-29  
**Depends On:** `L3-F-028_Posted_Journal_to_Accounting_Reversal.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall correct an authorized full or partial reversal of a `Posted` Journal by creating, approving, and posting a separate compensating Journal while preserving the original Journal lines as immutable accounting history.

The behavior begins when a `Posted` Journal requires full or partial reversal under approved accounting governance. It ends when the compensating Journal reaches `Posted`, the General Ledger reflects the approved reversal amount, and original-to-reversal lineage is reconstructable.

This specification does not edit the original Journal, correct the originating source-domain transaction, own or transition the Financial Period lifecycle, or prescribe technical rollback mechanisms.

---

## 3. Required Business Context

The behavior requires:

- one original Journal in `Posted`;
- the original Journal remaining immutable;
- an established reversal business basis and reversal scope;
- the required reversal authorization;
- Finance Manager authorization before compensating-Journal creation when the reversal is partial;
- an eligible F09-governed posting period;
- an Accountant acting as maker for the compensating Journal;
- a Senior Accountant acting as independent checker;
- an authorized posting actor;
- the canonical Journal validation, approval, posting, and reversal lifecycle.

Accounting & General Ledger owns both the original Journal and the compensating Journal. F09 retains ownership of the Financial Period lifecycle.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-028-BH-001 | When reversal is initiated, the system shall anchor the reversal to one immutable original Journal in `Posted` and preserve the approved reversal scope. | L3-F-028 §§4.1, 7-9 step 1 |
| L4-F-028-BH-002 | The system shall prevent reversal behavior from editing or replacing the original Journal lines. | L3-F-028 §§2-4, 10-12 |
| L4-F-028-BH-003 | When the reversal is partial, the system shall prevent creation of the compensating Journal unless Finance Manager authorization is present. | L3-F-028 §§6-7, 9 step 2, 10.2, 12, 15 |
| L4-F-028-BH-004 | When the required reversal basis and authorization are satisfied, the system shall permit creation of a new compensating Journal referencing the original Journal. | L3-F-028 §§3, 9 step 3, 12 |
| L4-F-028-BH-005 | The system shall require the compensating Journal to follow the normal `Draft → Validated → Approved → Posted` lifecycle. | L3-F-028 §§9 steps 4-5, 12-13 |
| L4-F-028-BH-006 | The system shall prevent the compensating Journal from reaching `Approved` without independent maker-checker approval. | L3-F-028 §§6, 9 step 4, 12, 15 |
| L4-F-028-BH-007 | The system shall prevent compensating-Journal posting when the governing Financial Period is not eligible under the F09-owned period state. | L3-F-028 §§7, 12 |
| L4-F-028-BH-008 | When an authorized posting actor posts the eligible approved compensating Journal, the system shall move it to `Posted`. | L3-F-028 §§6, 9 step 5, 13 |
| L4-F-028-BH-009 | On successful compensating-Journal posting, the system shall expose the approved reversal amount in the General Ledger while preserving the original Journal lines unchanged. | L3-F-028 §§3, 9 step 6, 16 |
| L4-F-028-BH-010 | For a full reversal, after the compensating Journal has successfully posted and neutralized the entire original financial effect, the system may move the original Journal from `Posted` to `Reversed`. | L3-F-028 §§10.1, 13-14 |
| L4-F-028-BH-011 | For a partial reversal, the system shall retain the original Journal in `Posted` because part of its financial effect remains authoritative. | L3-F-028 §§10.2, 13 |
| L4-F-028-BH-012 | The system shall not introduce or assign a `Partially Reversed` state to the original Journal. | L3-F-028 §§10.2, 12-13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-028-BH-013 | When the compensating Journal reaches `Posted`, the system shall publish `JournalPosted` with Accounting & General Ledger retained as the canonical publisher. | L3-F-028 §14 |
| L4-F-028-BH-014 | When a full reversal has successfully neutralized the entire original Journal effect and the original reaches `Reversed`, the system shall publish `JournalReversed` with Accounting & General Ledger retained as the canonical publisher. | L3-F-028 §§10.1, 13-14 |
| L4-F-028-BH-015 | The system shall preserve the original-to-compensating-Journal reference as authoritative accounting lineage without transferring ownership outside Accounting & General Ledger. | L3-F-028 §§3, 5, 9 step 7, 15-16 |
| L4-F-028-BH-016 | The system shall consume F09-owned Financial Period eligibility without republishing or assuming ownership of the Financial Period lifecycle. | L3-F-028 §§5, 7, 12 |

A partial reversal does not establish a `JournalReversed` outcome for the original because the original remains `Posted`.

---

## 6. Failure / Rejection / Continuation

- If required reversal authorization is absent, the system shall prevent progression to a valid compensating-Journal outcome.
- If a partial reversal lacks Finance Manager authorization, the system shall prevent compensating-Journal creation.
- If validation or independent approval does not complete successfully, the system shall prevent compensating-Journal posting.
- If posting eligibility is not satisfied, the system shall prevent posting.
- If compensating-Journal posting cannot complete, the system shall recognize no partial reversal effect and shall leave the original `Posted` Journal unchanged.
- The system shall not move the original Journal to `Reversed` unless a full compensating reversal has successfully posted and neutralized the entire original financial effect.

No technical retry, rollback, persistence, queue, or transaction-recovery mechanism is specified here.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to reconstruct:

- original Journal identity and immutable posted lines;
- reversal reason and approved reversal scope;
- whether the reversal is full or partial;
- Finance Manager authorization for partial reversal where applicable;
- compensating Journal identity and reference to the original;
- preparer, independent reviewer, approval decision, and posting actor;
- F09 posting-period eligibility;
- compensating Journal progression through `Draft`, `Validated`, `Approved`, and `Posted`;
- the resulting ledger effect;
- original Journal disposition as `Reversed` for successful full reversal or retained `Posted` for partial reversal;
- the occurrence of `JournalPosted` and, where applicable, `JournalReversed`.

Evidence shall remain implementation-independent.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. the original `Posted` Journal remains historically reconstructable and its lines are unchanged;
2. a separate compensating Journal references the original;
3. required reversal authorization is present;
4. Finance Manager authorization exists for partial reversal;
5. the compensating Journal has completed validation and independent approval;
6. the compensating Journal has reached `Posted`;
7. the General Ledger reflects the approved reversal amount;
8. for full reversal, the original may be `Reversed` only after its entire effect is neutralized;
9. for partial reversal, the original remains `Posted` and no `Partially Reversed` state exists;
10. reversal authority, accounting lineage, posting, and outcome evidence is reconstructable.

Source-domain correction and downstream period lifecycle behavior are outside this completion boundary.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-028` — Posted Journal to Accounting Reversal  
**Primary verification source:** `L2-F05` — Accounting & General Ledger Management  
**Material amendment:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Additional verification source:** `L2-F09` — Financial Period & Closing amendment  

Behavioral provenance is anchored in `L3-F-028`.

The locked F05 amendment verifies that full and partial reversals use new compensating Journal Entries, historical Journal lines remain immutable, partial reversal requires Finance Manager authorization, full reversal may move the original to `Reversed` after successful compensating posting, partial reversal leaves the original `Posted`, and no `Partially Reversed` state exists.

No L4 behavior in this specification creates a new reversal mechanism, actor, authority, Journal state, Financial Period state, event, publisher, or ownership assignment.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the supplied parent semantics.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/rollback/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to downstream architecture levels.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
