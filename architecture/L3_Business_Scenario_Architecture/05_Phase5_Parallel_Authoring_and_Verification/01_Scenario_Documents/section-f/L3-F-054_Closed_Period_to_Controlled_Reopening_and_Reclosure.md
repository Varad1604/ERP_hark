# Closed Period to Controlled Reopening and Reclosure

# 1. Scenario Metadata
**Scenario ID:** L3-F-054  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Period & Closing  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L3-F-052`

---
# 2. Scenario Purpose
Govern exceptional reopening of a Closed Financial Period and its subsequent reclosure without erasing the prior close evidence.

---
# 3. Business Outcome
A previously Closed Financial Period is exceptionally reopened to `Open`, permitted corrective accounting activity is governed, and the period is subsequently reclosed and, where required, relocked with all prior and successor evidence preserved.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
A Financial Period is Closed and an authoritative business need for exceptional reopening is established.
## 4.2 End Boundary
The corrective window has concluded and the period has returned through the normal F09 close controls to Closed/Locked status.
## 4.3 In Scope
Reopening authority, reopening evidence, corrective-window governance, close-readiness reassessment and reclosure/relock.
## 4.4 Out of Scope
Defining the corrective Journal itself, year reopening, technical rollback or changing historical close evidence.

---
# 5. Ownership and Participating Domains
F09 owns reopening/reclosure of the Financial Period. F05 owns any accounting corrections and supplies updated readiness evidence.

---
# 6. Business Actors, Roles and Responsibilities
| Authority | Responsibility |
|---|---|
| Period Reopening Authority | Authorizes exceptional reopening of the Closed period. |
| Period Closing Authority | Reconfirms close prerequisites after corrective activity. |
| Period Lock Authority | Re-establishes protected Locked status where applicable. |
| F05 accounting actors | Perform separately governed corrective accounting and supply readiness evidence. |

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §4; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§2 and 8.

---
# 7. Preconditions and Entry Criteria
The period is Closed; reopening basis is documented; scope and authority are explicit; prior closure evidence remains available.

---
# 8. Trigger
An exceptional, authorized Finance requirement requires accounting activity in the previously Closed period.

---
# 9. Main Business Flow
1. F09 records the reopening request and business justification.
2. The Period Reopening Authority reviews and authorizes the exceptional reopening.
3. The Financial Period returns to `Open`; prior closure evidence remains historical and unchanged.
4. F05 performs only the authorized corrective accounting activity under normal accounting controls.
5. F05 produces updated close-readiness evidence.
6. F09 repeats the normal close controls.
7. The Period Closing Authority returns the period to `Closed`.
8. Where protected historical status is required, the Period Lock Authority returns it to `Locked`.
9. Reopening, corrective references and successor close/lock evidence remain linked and auditable.

---
# 10. Alternate Business Flows
If reopening is denied, the period remains Closed and no corrective posting is permitted through this scenario.

---
# 11. Failure, Interruption and Business Recovery
An interrupted reopening/reclosure preserves the current authoritative F09 state. The process resumes under business governance; no technical rollback, persistence or retry mechanism is defined.

---
# 12. Business Rules and Decision Constraints
- Reopening is exceptional and requires explicit F09 authority.
- Reopening never erases prior closure evidence.
- Corrective activity remains owned by its accounting/business workflow.
- Reopened periods pass the same close controls before reclosure.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§3–5; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §8.

---
# 13. State and Lifecycle Impact
`Closed → Open → Closing → Closed → Locked` where relocking applies.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §3.

---
# 14. Business Events and Cross-Domain Collaboration
Financial Period lifecycle facts remain F09-owned. F05 provides corrective accounting/readiness facts only.

**Authoritative source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §5; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §8.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve reopening reason, Reopening Authority, prior close evidence, corrective references, updated readiness, Closing/Lock authorities and resulting lifecycle states.

---
# 16. Completion Criteria and Postconditions
The exceptional corrective need is governed; prior closure history remains immutable; the period is reclosed/relocked under F09 authority; all evidence is reconstructable.

---
# 17. Open Issues, Assumptions and Deferred Questions
None material after the 2026-08-21 F09 amendment.

---
**End of Scenario L3-F-054**