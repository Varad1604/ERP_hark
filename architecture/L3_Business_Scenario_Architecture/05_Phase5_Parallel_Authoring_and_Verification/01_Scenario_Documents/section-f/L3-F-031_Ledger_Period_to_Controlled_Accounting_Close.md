# Ledger Period to Controlled Accounting Close

# 1. Scenario Metadata

**Scenario ID:** L3-F-031  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** L3-F-032; L3-F-052  

---

# 2. Scenario Purpose

Establish Accounting & General Ledger close-readiness evidence for a Financial Period and hand that evidence to F09 for the canonical close/lock decision.

---

# 3. Business Outcome

Accounting has completed its period-close responsibilities—pending Journal resolution, reconciliation review and Trial Balance verification—and supplied authoritative close-readiness evidence to F09, enabling the Financial Period to reach its governed Closed/Locked outcome under F09 ownership.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governing Financial Period is approaching close and accounting close-readiness assessment begins.

## 4.2 End Boundary

Accounting close-readiness evidence is complete and accepted for F09 close governance; the period lifecycle outcome itself is owned and published by F09.

## 4.3 In Scope

Pending-Journal review, ledger/reconciliation readiness, Trial Balance verification, accounting close evidence and hand-off to F09.

## 4.4 Out of Scope

Owning or redefining Financial Period close, lock, reopen or archival states; Financial Year close; technical close execution.

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Accounting & General Ledger / F05 | Owns accounting readiness facts and ledger evidence. |
| Financial Period & Closing / F09 | Owns Financial Period close, lock, reopen and archival lifecycle. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Accountant | Resolves/identifies pending accounting work and prepares reconciliation/Trial Balance evidence. | Accounting preparation. |
| Senior Accountant | Reviews reconciliation and Trial Balance evidence. | Accounting verification. |
| Finance Manager | Approves Accounting participation in close/lock readiness. | Does not own the F09 lifecycle transition. |
| Period Closing / Lock Authority | Executes the canonical F09 close/lock decision. | F09-owned authority. |

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §2; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §4.

---

# 7. Preconditions and Entry Criteria

- The Financial Period is in a close-eligible F09 state.
- Posted ledger activity for the period is identifiable.
- Pending Journals and reconciliation scope can be assessed.
- Trial Balance evidence can be generated and verified.

---

# 8. Trigger

F09 initiates governed period-closing preparation requiring Accounting close-readiness evidence.

---

# 9. Main Business Flow

1. Accounting identifies unresolved Journals affecting the period.
2. Required Journals are resolved or explicitly governed before close readiness is claimed.
3. Reconciliation evidence for the applicable ledger scope is completed/reviewed.
4. A Trial Balance Snapshot is generated and verified under L3-F-030.
5. Accounting records its close-readiness evidence and any unresolved blocking exception.
6. The Finance Manager confirms Accounting participation is ready for F09 close governance.
7. Accounting supplies the readiness facts to F09.
8. F09 applies its own Period Closing/Lock authority and publishes the authoritative lifecycle outcome outside F05 ownership.

---

# 10. Alternate Business Flows

If unresolved Journals, reconciliation exceptions or invalid Trial Balance evidence remain material, Accounting withholds close readiness and identifies the blocking business evidence for resolution.

---

# 11. Failure, Interruption and Business Recovery

Close readiness is not asserted while blocking accounting evidence remains unresolved. Recovery occurs through the owning Journal, reconciliation or Trial Balance workflow. L3 defines no technical rollback or close-job behavior.

---

# 12. Business Rules and Decision Constraints

- F05 does not own the Financial Period lifecycle.
- F09 is the canonical owner of close/lock/reopen/archive decisions.
- Accounting close readiness requires pending-journal resolution, reconciliation status and verified Trial Balance evidence where applicable.
- A period not confirmed `Open` by F09 cannot accept ordinary posting.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§1 and 8; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§1, 3 and 5.

---

# 13. State and Lifecycle Impact

Accounting evidence has no independent Financial Period state machine. F09 consumes the evidence to govern its canonical progression:

`Open → Closing → Closed → Locked → Archived`

This scenario must not publish an F05-owned period state transition.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §3; F05 participation boundary: `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §8.

---

# 14. Business Events and Cross-Domain Collaboration

F05 may publish/retain its own accounting facts such as `LedgerReconciled` and `TrialBalancePublished`. F09 remains the sole publisher of canonical Financial Period lifecycle facts used to close/lock the period.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§6–8; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §5.

---

# 15. Business Authorization, Approval and Audit Obligations

Accounting evidence must preserve the period reference, pending-Journal disposition, reconciliation status, Trial Balance identity, reviewers and Finance Manager close-readiness confirmation. F09 separately preserves its close/lock authority evidence.

---

# 16. Completion Criteria and Postconditions

Accounting close-readiness evidence is complete, traceable and supplied to F09. Any resulting Closed/Locked period status is authoritative only when established by F09.

---

# 17. Open Issues, Assumptions and Deferred Questions

No material ownership or lifecycle blocker remains after the 2026-08-21 F05/F09 amendments.

---

**End of Scenario L3-F-031**