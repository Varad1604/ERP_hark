# Unsettled Payable to Suspension, Reopening, or Write-Off

# 1. Scenario Metadata
**Scenario ID:** L3-F-048  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Payable  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F04_Financial_Adjustment_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`

---
# 2. Scenario Purpose
Govern an unsettled Payable through AP-owned suspension/reopening decisions or referral to the separately owned Finance write-off process.

---
# 3. Business Outcome
The unsettled Payable has a governed AP disposition: it remains suspended/blocked, is reopened to eligible AP processing, or is referred for separately authorized write-off treatment, with history preserved.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
An unsettled Payable requires exceptional AP governance because ordinary settlement should not continue.
## 4.2 End Boundary
AP has established the current suspension/reopening position or completed a write-off referral; an actual write-off is not claimed until authoritative F04 evidence is received.
## 4.3 In Scope
AP suspension, evidence review, reopening and write-off referral.
## 4.4 Out of Scope
Approving/applying Financial Adjustment write-off, Treasury settlement, procurement correction or technical recovery.

---
# 5. Ownership and Participating Domains
F08 owns Payable lifecycle/readiness. F04 owns any Write-Off Adjustment. Section J retains supplier/procurement ownership.

---
# 6. Business Actors, Roles and Responsibilities
AP authorities govern suspension/reopening under F08 policy. Any write-off approval remains with F04/enterprise approval governance; AP initiation or referral alone does not authorize it.

**Authoritative source:** `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md` §4; write-off authority remains governed by `L2-F04_Financial_Adjustment_Management.md` and its locked amendment.

---
# 7. Preconditions and Entry Criteria
A Payable remains unsettled; the exceptional reason is supported by evidence; current liability and settlement references are known.

---
# 8. Trigger
A material AP condition requires suspension, reconsideration, reopening or write-off assessment.

---
# 9. Main Business Flow
1. AP validates the current outstanding Payable position.
2. AP reviews the exceptional reason and relevant source evidence.
3. Where settlement should not proceed, AP applies the F08-supported suspension/blocking position.
4. When authoritative evidence resolves the condition, AP may reopen payment readiness under governed review.
5. Where the liability appears eligible for write-off, AP records a referral to the F04 write-off process.
6. AP does not reduce/write off the liability until an authoritative approved/applied F04 fact exists.
7. All decisions and resulting Payable positions remain traceable.

---
# 10. Alternate Business Flows
Suspension may remain effective; reopening may restore ordinary processing; write-off referral may be declined by the owning Finance approval process without changing AP history.

---
# 11. Failure, Interruption and Business Recovery
Uncertain evidence preserves the current safe AP position. Business recovery depends on authoritative evidence or a completed owning-domain decision, not technical retry/rollback.

---
# 12. Business Rules and Decision Constraints
- AP suspension/reopening does not alter procurement ownership.
- Write-off referral is not write-off approval.
- F04 owns the adjustment and approval/application path.
- Historical Payable evidence remains immutable.

**Authoritative source:** `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md` §§1 and 4; write-off ownership and approval remain with F04.

---
# 13. State and Lifecycle Impact
Only states/readiness conditions already supported by F08 are used. No new write-off state is introduced by referral alone.

**Authoritative source:** canonical `L2-F08_Accounts_Payable_Management.md`, constrained by the 2026-08-21 amendment §4.

---
# 14. Business Events and Cross-Domain Collaboration
AP uses only established F08 Payable events where defined. Actual write-off facts remain F04-published; this scenario does not republish them under AP ownership.

**Authoritative source:** `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md` §§2–4; F04 remains canonical publisher for its own adjustment/write-off facts.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve Payable identity, exceptional reason, evidence, AP authority, suspension/reopening decision and any F04 referral/outcome reference.

---
# 16. Completion Criteria and Postconditions
The Payable has one governed current AP position; no unauthorized settlement/write-off occurred; any write-off remains separately authorized and traceable.

---
# 17. Open Issues, Assumptions and Deferred Questions
No material event-ownership blocker remains after the 2026-08-21 amendments.

---
**End of Scenario L3-F-048**