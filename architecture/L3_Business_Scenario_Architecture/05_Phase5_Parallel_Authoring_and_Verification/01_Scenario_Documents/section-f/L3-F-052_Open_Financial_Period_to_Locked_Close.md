# Open Financial Period to Locked Close

# 1. Scenario Metadata
**Scenario ID:** L3-F-052  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Period & Closing  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L3-F-031`

---
# 2. Scenario Purpose
Govern an Open Financial Period through close readiness, closure and protected Locked status under F09 ownership.

---
# 3. Business Outcome
The Financial Period reaches `Locked` after required accounting readiness, close review and lock authority have been satisfied, with closure evidence preserved.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
A Financial Period is `Open` and its governed close cycle is initiated.
## 4.2 End Boundary
The Financial Period is `Locked` and no ordinary posting/modification is permitted.
## 4.3 In Scope
Close initiation, accounting-readiness consumption, close authority, lock authority and lifecycle evidence.
## 4.4 Out of Scope
Producing Journals/reconciliation/Trial Balance evidence, reopening, archival, Financial Year close and technical close execution.

---
# 5. Ownership and Participating Domains
F09 exclusively owns the Financial Period lifecycle. F05 supplies accounting readiness facts and does not own close/lock transitions.

---
# 6. Business Actors, Roles and Responsibilities
| Authority | Responsibility |
|---|---|
| Period Closing Authority | Confirms close prerequisites and performs `Closing → Closed`. |
| Period Lock Authority | Confirms the Closed period is ready for protected `Locked` status. |
| F05 Finance Manager/accounting actors | Supply/approve accounting close-readiness evidence only. |

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §4; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§2 and 8.

---
# 7. Preconditions and Entry Criteria
The period is Open; pending accounting work is resolved or governed; reconciliation and verified Trial Balance evidence are available where applicable; no blocking close exception remains.

---
# 8. Trigger
F09 initiates governed closing of the Open Financial Period.

---
# 9. Main Business Flow
1. F09 moves the period `Open → Closing` and requests required Finance readiness evidence.
2. F05 supplies pending-Journal, reconciliation and Trial Balance readiness under L3-F-031.
3. F09 reviews all close prerequisites and unresolved exceptions.
4. The Period Closing Authority closes the period: `Closing → Closed`.
5. Closure evidence is preserved.
6. The Period Lock Authority confirms protected historical readiness.
7. The period progresses `Closed → Locked`.
8. F09 publishes/maintains the authoritative period lifecycle facts for Finance consumers.

---
# 10. Alternate Business Flows
If a material close prerequisite fails, the period remains `Closing` until the owning business workflow resolves the issue; it does not skip directly to Closed or Locked.

---
# 11. Failure, Interruption and Business Recovery
Business interruption preserves the current F09 state and close evidence. Recovery resumes under the same close governance after missing prerequisites are resolved. No technical rollback/retry behavior is defined.

---
# 12. Business Rules and Decision Constraints
- F09 owns close and lock.
- Close cannot bypass unresolved material accounting readiness exceptions.
- Lock follows Closed and is stronger historical protection.
- Closed/Locked periods do not accept ordinary postings.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§1, 3–5; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §8.

---
# 13. State and Lifecycle Impact
`Open → Closing → Closed → Locked`.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §3.

---
# 14. Business Events and Cross-Domain Collaboration
F05 supplies accounting facts; F09 remains the canonical publisher/owner of Financial Period lifecycle facts. No F05 period event is substituted for F09 ownership.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §5; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §8.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve period identity, close evidence, accounting-readiness references, Closing Authority, Lock Authority, decisions, timestamps and lifecycle transitions.

---
# 16. Completion Criteria and Postconditions
The Financial Period is Locked; ordinary posting is prevented by the authoritative F09 state; all close/lock evidence is reconstructable.

---
# 17. Open Issues, Assumptions and Deferred Questions
None material after the 2026-08-21 F09/F05 amendments.

---
**End of Scenario L3-F-052**