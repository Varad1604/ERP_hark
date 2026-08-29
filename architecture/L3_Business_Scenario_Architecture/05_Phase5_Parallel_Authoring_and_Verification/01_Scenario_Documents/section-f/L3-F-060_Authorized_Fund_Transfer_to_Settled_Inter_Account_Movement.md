# Authorized Fund Transfer to Settled Inter-Account Movement

# 1. Scenario Metadata
**Scenario ID:** L3-F-060  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L3-F-056`

---
# 2. Scenario Purpose
Govern an authorized transfer of institutional funds between permitted Treasury Accounts through completed Treasury settlement while preserving cash integrity and accounting ownership boundaries.

---
# 3. Business Outcome
The authorized inter-account movement has completed settlement, both Treasury positions reflect the authoritative cash consequence, and transfer/settlement history is fully traceable.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
An authorized inter-account fund-transfer requirement exists between eligible Treasury Accounts.
## 4.2 End Boundary
Settlement is completed and Treasury cash positions reflect the settled movement.
## 4.3 In Scope
Transfer eligibility, Treasury authorization, settlement and cash-position consequence.
## 4.4 Out of Scope
Bank protocol/API design, source accounting Journal creation, external-bank ownership and technical transaction handling.

---
# 5. Ownership and Participating Domains
Treasury owns the transfer execution, settlement and Cash Position. Accounting owns any resulting Journal/ledger consequence. External banks own their original banking records.

---
# 6. Business Actors, Roles and Responsibilities
Treasury governance authorizes and executes the transfer under F10 rules. The initiating actor cannot bypass enterprise authorization or Treasury restrictions.

**Authoritative source:** canonical `L2-F10_Treasury_and_Banking.md`; Treasury Account authority and separation boundaries: `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md` §§2–3 and 6.

---
# 7. Preconditions and Entry Criteria
Source/destination Treasury Accounts are Active and permitted for the transfer; amount/currency are valid; sufficient governed cash/liquidity exists; authorization is recorded.

---
# 8. Trigger
An authorized Treasury fund-transfer instruction becomes eligible for settlement.

---
# 9. Main Business Flow
1. Treasury validates source/destination accounts, amount, currency, authority and restrictions.
2. Treasury confirms the transfer is supportable by current cash/liquidity governance.
3. Treasury initiates the governed settlement movement.
4. Settlement is tracked as a Treasury-owned business activity.
5. On successful final settlement, Treasury records `SettlementCompleted`.
6. Source and destination Cash Positions are updated consistently and `CashPositionUpdated` records the completed cash fact.
7. Transfer, settlement and cash evidence remain correlated for Accounting/audit consumption.

---
# 10. Alternate Business Flows
If settlement cannot complete, no settled inter-account movement is claimed and the matter follows F061.

---
# 11. Failure, Interruption and Business Recovery
A failed or uncertain settlement preserves the last authoritative cash position until a conclusive Treasury business outcome exists. No database rollback, technical retry, gateway recovery or event-publication recovery is defined.

---
# 12. Business Rules and Decision Constraints
- Cash changes only after successful settlement completion.
- Treasury Accounts must be Active and operationally permitted.
- Transfer authorization precedes settlement.
- Treasury does not create Accounting Journals.
- Historical settlement evidence is immutable.

**Authoritative source:** canonical `L2-F10_Treasury_and_Banking.md`; Treasury Account Active-state and authorization constraints: 2026-08-21 amendment §§2–3 and 6; L3 technical-recovery exclusions: amendment §7.

---
# 13. State and Lifecycle Impact
Settlement follows F10-owned Settlement Execution semantics. Treasury Account lifecycle remains unchanged unless separately governed.

**Authoritative source:** canonical settlement semantics in `L2-F10_Treasury_and_Banking.md`; Treasury Account lifecycle: 2026-08-21 amendment §2.

---
# 14. Business Events and Cross-Domain Collaboration
Established F10 events such as `SettlementStarted`, `SettlementCompleted`, `SettlementFailed` and `CashPositionUpdated` remain Treasury-owned. Accounting consumes completed facts without modifying Treasury state.

**Authoritative source:** canonical event definitions in `L2-F10_Treasury_and_Banking.md`; Treasury ownership remains locked under the 2026-08-21 amendment §§1 and 7.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve source/destination account identities, transfer amount/currency, authorization, settlement outcome, cash-position effect and correlation to downstream Accounting evidence.

---
# 16. Completion Criteria and Postconditions
Settlement is completed; cash positions reflect the movement consistently; ownership boundaries remain intact; evidence is reconstructable.

---
# 17. Open Issues, Assumptions and Deferred Questions
No material Treasury lifecycle/authority blocker remains after the 2026-08-21 F10 amendment.

---
**End of Scenario L3-F-060**