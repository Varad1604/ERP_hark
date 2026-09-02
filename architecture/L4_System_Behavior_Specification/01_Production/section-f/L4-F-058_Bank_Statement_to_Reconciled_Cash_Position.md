# Bank Statement to Reconciled Cash Position

## 1. Specification Metadata

**Specification ID:** L4-F-058  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Treasury & Banking  
**Realizes Scenario:** L3-F-058  
**Primary Business Owner:** Treasury & Banking  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-058_Bank_Statement_to_Reconciled_Cash_Position.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall reconcile authoritative external banking activity with Treasury's Bank Transaction and Cash Position representations while preserving external bank ownership and Accounting ownership of financial reconciliation outcomes.

The behavior begins when an authoritative bank statement or equivalent banking activity and the corresponding Treasury transaction/cash scope are available. It ends when Treasury reconciliation coordination is complete for the defined scope and current Cash Position reflects only authoritative completed Treasury cash facts.

This specification does not alter external bank records, own Accounting reconciliation, parse bank files, or define integration protocols.

---

## 3. Required Business Context

The behavior requires:

- identifiable authoritative external bank activity;
- Treasury transaction scope;
- relevant Treasury Account references;
- current Cash Position context;
- traceable payment/settlement references;
- Treasury reconciliation coordination;
- F05 Accounting reconciliation ownership where financial reconciliation/correction is required.

Treasury owns Bank Transaction representation, Cash Position, and reconciliation coordination. The bank owns original banking activity. Accounting owns its own ledger reconciliation outcomes.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-058-BH-001 | The system shall record/reference authoritative bank activity without altering the external bank record. | L3-F-058 §§4.1, 9 step 1, 12 |
| L4-F-058-BH-002 | Where a Treasury Bank Transaction representation is established, the system shall preserve the external banking reference and publish `BankTransactionRegistered` where canonically applicable. | L3-F-058 §§3, 14 |
| L4-F-058-BH-003 | When reconciliation coordination begins, the system shall establish the canonical `ReconciliationStarted` fact where applicable. | L3-F-058 §§8-9, 14 |
| L4-F-058-BH-004 | The system shall compare bank activity with Treasury payment, settlement, and Bank Transaction representations. | L3-F-058 §§9 step 2 |
| L4-F-058-BH-005 | Matched items shall be associated with their Treasury business references. | L3-F-058 §9 step 3 |
| L4-F-058-BH-006 | Unmatched or inconsistent items shall be recorded as reconciliation exceptions. | L3-F-058 §§9 step 4, 10 |
| L4-F-058-BH-007 | When an exception is identified, the system shall publish `ReconciliationExceptionDetected` where the canonical Treasury fact occurs. | L3-F-058 §14 |
| L4-F-058-BH-008 | Required corrections shall be routed to the canonical owning workflow; Treasury shall not rewrite Accounting or source history. | L3-F-058 §§5, 9 step 5, 12 |
| L4-F-058-BH-009 | After authoritative correction or clarification, the system shall permit re-comparison of affected items. | L3-F-058 §§9 step 6, 11 |
| L4-F-058-BH-010 | The system shall prevent a false reconciled conclusion while material unresolved exceptions remain. | L3-F-058 §§10, 16 |
| L4-F-058-BH-011 | When the defined Treasury scope is reconciled, the system shall establish `ReconciliationCompleted` and preserve reconciled cash-position evidence. | L3-F-058 §§3, 9 step 7, 14 |
| L4-F-058-BH-012 | Cash Position shall change only from authoritative completed Treasury settlement facts, not merely from bank-statement comparison. | L3-F-058 §§4.2, 12 |
| L4-F-058-BH-013 | Where Cash Position changes from authoritative completed Treasury cash facts, the system shall publish `CashPositionUpdated`. | L3-F-058 §14 |
| L4-F-058-BH-014 | Accounting may consume Treasury reconciliation facts for independent F05 reconciliation without ownership transfer. | L3-F-058 §§5, 9 step 8, 12, 14 |

---

## 5. Cross-Domain & Event Behavior

Treasury owns `BankTransactionRegistered`, `ReconciliationStarted`, `ReconciliationCompleted`, `ReconciliationExceptionDetected`, and `CashPositionUpdated` where those completed Treasury facts occur. Accounting reconciliation remains independently owned by F05. External banks remain authoritative for original banking records.

---

## 6. Failure / Rejection / Continuation

Unresolved exceptions shall remain explicit and prevent a false reconciled conclusion for the affected scope. Business interruption shall preserve matched/unmatched evidence and current authoritative Cash Position. Recovery resumes from authoritative business evidence. No technical reload, retry, parsing recovery, or rollback is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve bank statement scope, external bank references, Treasury transaction references, payment/settlement links, matched items, unmatched items, exceptions, correction references, re-comparison outcomes, reconciliation completion evidence, Cash Position lineage, and Accounting hand-off references where applicable.

---

## 8. Observable Completion Conditions

Completion requires the defined Treasury scope to be reconciled or unresolved exceptions to remain explicitly identified; Cash Position must reflect only completed Treasury settlement facts; external bank records and Accounting ownership must remain unchanged.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-058` — Bank Statement to Reconciled Cash Position  
**Primary verification source:** `L2-F10` — Treasury & Banking  
**Locked amendment:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`  
**Accounting source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`

No behavior creates bank records, changes Accounting ownership, or changes Cash Position solely because bank statement activity exists.

---

## 10. Open Issues / Deferred Design

Institution-specific reconciliation tolerances remain configuration-governed and are not fixed here.

API/interface design, bank-file parsing, integration protocols, matching algorithms, persistence, technical retries, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
