# Accounting Period Activity to Trial Balance

## 1. Specification Metadata

**Specification ID:** L4-F-030  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounting & General Ledger  
**Realizes Scenario:** L3-F-030  
**Primary Business Owner:** Accounting & General Ledger  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-29  
**Depends On:** `L3-F-030_Accounting_Period_Activity_to_Trial_Balance.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall derive, verify, and publish an immutable Trial Balance Snapshot from posted General Ledger activity for a defined accounting boundary, or retain the generation attempt as terminally `Invalid` when verification cannot succeed.

The behavior begins when posted ledger activity exists for a defined ledger, institution, Financial Period/as-of boundary, and currency context and Trial Balance generation is required. It ends when the Trial Balance Snapshot reaches either `Published` or terminal `Invalid`.

This specification does not own or transition the Financial Period/Financial Year lifecycle, prepare financial statements, govern archival, or prescribe calculation, storage, or reporting implementation.

---

## 3. Required Business Context

The behavior requires:

- identifiable General Ledger population;
- explicit ledger and institutional boundary;
- explicit Financial Period or as-of boundary;
- explicit currency context;
- only posted accounting records contributing to the snapshot;
- an Accountant with generation responsibility;
- a Senior Accountant with verification authority;
- the canonical Trial Balance Snapshot lifecycle and event model.

Accounting & General Ledger owns the Trial Balance Snapshot. F09 retains ownership of the Financial Period and Financial Year lifecycle referenced by the snapshot.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-030-BH-001 | When Trial Balance generation begins, the system shall establish a new Trial Balance Snapshot instance in `Generating` for the defined ledger, institution, period/as-of, and currency boundary. | L3-F-030 §§4.1, 7-9 step 1 |
| L4-F-030-BH-002 | The system shall derive the snapshot exclusively from accounting records that are already posted to the General Ledger. | L3-F-030 §§7, 9 step 2, 12 |
| L4-F-030-BH-003 | The system shall preserve the accounting boundary and ledger lineage used to derive the snapshot. | L3-F-030 §§3, 7, 9 steps 2-3, 15 |
| L4-F-030-BH-004 | The system shall require Senior Accountant verification of debit-credit equality and source lineage before the snapshot may become `Verified`. | L3-F-030 §§6, 9 step 3, 12, 15 |
| L4-F-030-BH-005 | When debit-credit equality and ledger lineage verification succeed, the system shall move the Trial Balance Snapshot from `Generating` to `Verified`. | L3-F-030 §§9 steps 3-4, 13 |
| L4-F-030-BH-006 | When a `Verified` snapshot is eligible for publication, the system shall move it to `Published`. | L3-F-030 §§9 step 4, 13 |
| L4-F-030-BH-007 | The system shall recognize reporting eligibility only when the Trial Balance Snapshot has reached `Published`. | L3-F-030 §§3, 9 step 5, 12, 16 |
| L4-F-030-BH-008 | The system shall not treat `Generating` or `Verified` alone as authoritative Financial Reporting eligibility. | L3-F-030 §§9 step 5, 12 |
| L4-F-030-BH-009 | When debit-credit equality fails or ledger lineage cannot be verified, the system shall move the snapshot from `Generating` to terminal `Invalid`. | L3-F-030 §§10.1, 13, 16 |
| L4-F-030-BH-010 | The system shall prevent an `Invalid` snapshot from being published or consumed as authoritative Financial Reporting evidence. | L3-F-030 §§3, 4.2, 10.1, 12, 16 |
| L4-F-030-BH-011 | The system shall preserve an `Invalid` snapshot as immutable historical evidence of that failed generation attempt. | L3-F-030 §§10.1-11, 12 |
| L4-F-030-BH-012 | The system shall prevent an `Invalid` snapshot instance from being converted into a valid one; a corrected attempt shall use a new Trial Balance Snapshot instance. | L3-F-030 §§10.1-11, 12 |
| L4-F-030-BH-013 | The system shall preserve historical Trial Balance Snapshots as immutable accounting evidence. | L3-F-030 §§12, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-030-BH-014 | When a Trial Balance generation result exists for accounting review, the system shall publish `TrialBalanceGenerated` with Accounting & General Ledger retained as the canonical publisher. | L3-F-030 §14 |
| L4-F-030-BH-015 | When debit-credit equality and lineage verification succeed, the system shall publish `TrialBalanceVerified` with Accounting & General Ledger retained as the canonical publisher. | L3-F-030 §14 |
| L4-F-030-BH-016 | When the snapshot reaches `Published`, the system shall publish `TrialBalancePublished` with Accounting & General Ledger retained as the canonical publisher. | L3-F-030 §§9 step 5, 14 |
| L4-F-030-BH-017 | The system shall expose only a `Published` Trial Balance Snapshot as eligible for downstream Financial Reporting consumption. | L3-F-030 §§3, 9 step 5, 12, 16 |
| L4-F-030-BH-018 | The system shall reference F09-owned period/year context without assuming ownership or republishing F09 lifecycle facts. | L3-F-030 §§4.4-5 |

No downstream Financial Reporting consumption transfers ownership of the Trial Balance Snapshot.

---

## 6. Failure / Rejection / Continuation

- If debit-credit equality fails, the system shall establish terminal `Invalid` for that snapshot instance.
- If ledger lineage cannot be verified, the system shall establish terminal `Invalid`.
- An `Invalid` snapshot shall not be published.
- An `Invalid` snapshot shall remain historical evidence and shall not be recalculated into a valid snapshot.
- Recovery requires a new governed Trial Balance Snapshot generation attempt after the underlying accounting condition has been resolved.
- The system shall not define technical retry, recalculation, rollback, persistence, or repair mechanics in this specification.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to reconstruct:

- Trial Balance Snapshot identity;
- ledger and institutional scope;
- Financial Period/as-of boundary;
- currency context;
- posted-ledger population used as the basis;
- generating Accountant;
- verifying Senior Accountant;
- debit and credit totals;
- verification result;
- ledger/source lineage evidence;
- lifecycle progression through `Generating`, `Verified`, `Published`, or terminal `Invalid`;
- publication or invalid outcome;
- the occurrence of `TrialBalanceGenerated`, `TrialBalanceVerified`, and `TrialBalancePublished` where applicable.

Internal Auditor access remains read-only and does not alter the snapshot outcome.

---

## 8. Observable Completion Conditions

The behavior is complete through one of two governed outcomes:

**Published outcome**
1. a new Trial Balance Snapshot was generated from posted ledger activity only;
2. debit and credit totals are equal;
3. source/ledger lineage was verified by the Senior Accountant;
4. the snapshot reached `Verified`;
5. the snapshot reached `Published`;
6. `TrialBalancePublished` was published;
7. the immutable snapshot is eligible for Financial Reporting.

**Invalid outcome**
1. debit-credit equality or ledger-lineage verification did not succeed;
2. the snapshot reached terminal `Invalid`;
3. the snapshot was not published;
4. the snapshot is not eligible for Financial Reporting;
5. the failed attempt remains immutable evidence;
6. any corrected attempt requires a new snapshot instance.

Archival is outside this scenario's completion boundary.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-030` — Accounting Period Activity to Trial Balance  
**Primary verification source:** `L2-F05` — Accounting & General Ledger Management  
**Material amendment:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Additional verification source:** `L2-F09` — Financial Period & Closing amendment  

Behavioral provenance is anchored in `L3-F-030`.

The locked F05 amendment verifies the Trial Balance Snapshot lifecycle `Generating → Verified → Published → Archived` and `Generating → Invalid`, the terminal nature of `Invalid`, the requirement that corrected attempts use a new snapshot, and the rule that `TrialBalancePublished` establishes reporting eligibility.

No L4 behavior in this specification creates a new Trial Balance state, verification authority, reporting-eligibility rule, event, publisher, Financial Period state, or ownership assignment.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the supplied parent semantics.

API/interface design, persistence/data model, service/component design, balance-calculation algorithms, technical validation logic, event transport, technical access control, retry/recalculation mechanics, infrastructure/deployment, Financial Reporting presentation, and UI/UX remain deferred to downstream architecture levels.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
