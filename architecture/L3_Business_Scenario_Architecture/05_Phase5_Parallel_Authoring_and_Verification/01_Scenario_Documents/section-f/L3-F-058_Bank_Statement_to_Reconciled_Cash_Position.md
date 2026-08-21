# Bank Statement to Reconciled Cash Position

# 1. Scenario Metadata
**Scenario ID:** L3-F-058  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L3_Business_Scenario_Template_v2.0.md`

---
# 2. Scenario Purpose
Reconcile authoritative external banking activity with Treasury's cash and bank-transaction representation while preserving Accounting ownership of financial reconciliation outcomes.

---
# 3. Business Outcome
Treasury has a reconciled cash-position view for the applicable bank-statement scope, with matched banking activity, governed exceptions and complete traceability to Treasury transactions.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
Authoritative bank-statement activity and the corresponding Treasury transaction/cash scope are available.
## 4.2 End Boundary
Treasury reconciliation coordination is complete for the defined scope and the current Cash Position reflects only authoritative completed cash facts.
## 4.3 In Scope
Bank-activity representation, comparison, exception identification, reconciliation coordination and cash-position evidence.
## 4.4 Out of Scope
Changing external bank records, owning accounting reconciliation, technical bank-file parsing or integration protocols.

---
# 5. Ownership and Participating Domains
Treasury owns its Bank Transaction representation, Cash Position and reconciliation coordination. The bank owns original banking activity. Accounting owns financial reconciliation outcomes/ledger correction.

---
# 6. Business Actors, Roles and Responsibilities
Treasury actors perform governed reconciliation coordination under F10; Accounting actors independently govern accounting correction/reconciliation where required. No external bank actor is treated as a SutraERP aggregate owner.

---
# 7. Preconditions and Entry Criteria
Bank activity is identifiable and traceable; Treasury transaction scope is available; relevant Treasury Accounts and cash references are known.

---
# 8. Trigger
A bank statement or equivalent authoritative banking activity becomes available for Treasury reconciliation.

---
# 9. Main Business Flow
1. Treasury records/references the authoritative banking activity without altering the external bank record.
2. Treasury compares bank activity with Treasury payment, settlement and Bank Transaction representations.
3. Matched items are associated with their Treasury business references.
4. Unmatched or inconsistent items are recorded as reconciliation exceptions.
5. Required corrections are routed to the canonical owning workflow; Treasury does not rewrite Accounting/source history.
6. Reconciliation is repeated for corrected/clarified items.
7. Treasury completes reconciliation coordination and records the reconciled cash-position evidence.
8. Accounting may consume Treasury facts for its independent financial reconciliation.

---
# 10. Alternate Business Flows
Unresolved exceptions remain explicitly open and prevent a false reconciled conclusion for the affected scope.

---
# 11. Failure, Interruption and Business Recovery
Business interruption preserves matched/unmatched evidence and current authoritative cash truth. Recovery resumes from governed business evidence, not technical reload/retry or rollback logic.

---
# 12. Business Rules and Decision Constraints
- External bank records are never overwritten.
- Treasury owns its business representation, not original bank infrastructure.
- Cash Position changes only from completed Treasury settlement facts.
- Accounting owns financial reconciliation outcomes.

---
# 13. State and Lifecycle Impact
Treasury reconciliation progresses through F10-defined reconciliation business milestones; no software-processing state is added.

---
# 14. Business Events and Cross-Domain Collaboration
Established Treasury events include `BankTransactionRegistered`, `ReconciliationStarted`, `ReconciliationCompleted`, `ReconciliationExceptionDetected` and `CashPositionUpdated` where their completed business facts occur.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve statement scope, bank references, Treasury transaction references, matches/exceptions, resolutions, reconciliation outcome and cash-position lineage.

---
# 16. Completion Criteria and Postconditions
The defined Treasury scope is reconciled or its unresolved exceptions are explicit; cash truth is not fabricated; bank and Accounting ownership remain unchanged.

---
# 17. Open Issues, Assumptions and Deferred Questions
Institution-specific reconciliation tolerances remain configuration policy. No material Treasury lifecycle blocker remains.

---
**End of Scenario L3-F-058**