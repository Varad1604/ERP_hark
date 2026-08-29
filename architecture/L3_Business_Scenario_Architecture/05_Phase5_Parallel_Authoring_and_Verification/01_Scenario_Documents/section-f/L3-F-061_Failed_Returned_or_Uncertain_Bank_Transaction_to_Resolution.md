# Failed, Returned, or Uncertain Bank Transaction to Resolution

# 1. Scenario Metadata
**Scenario ID:** L3-F-061  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L3-F-057`; `L3-F-058`; `L3-F-060`

---
# 2. Scenario Purpose
Govern a failed, returned or uncertain banking outcome to a conclusive Treasury business resolution without inventing cash settlement or rewriting prior Treasury facts.

---
# 3. Business Outcome
The affected Treasury transaction has a conclusive, auditable business disposition, and Cash Position reflects only confirmed completed settlement facts.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
Treasury has evidence that a payment, settlement or bank transaction failed, was returned, or has an uncertain external outcome.
## 4.2 End Boundary
Treasury has established the authoritative business outcome—failed/returned with no settlement, subsequently confirmed settlement, or another governed resolution—and reconciled the cash consequence accordingly.
## 4.3 In Scope
Exception evidence, bank/Treasury fact reconciliation, outcome determination, cash-position consequence and upstream notification.
## 4.4 Out of Scope
Bank infrastructure repair, network retries, database rollback, message queues, persistence recovery and source-obligation modification.

---
# 5. Ownership and Participating Domains
Treasury owns its payment/settlement/Bank Transaction representation and Cash Position. External banks own original banking activity. AP/AR/source Finance domains own their obligations/receivables.

---
# 6. Business Actors, Roles and Responsibilities
Treasury governance reviews the exception and establishes the Treasury business outcome from authoritative evidence. Source-domain actors consume the result but do not determine Treasury cash truth.

**Authoritative source:** canonical `L2-F10_Treasury_and_Banking.md`; Treasury ownership and authority boundaries remain locked by `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md` §§1, 3 and 7.

---
# 7. Preconditions and Entry Criteria
The affected Treasury transaction is identifiable; original execution/settlement references exist; external banking evidence is available or explicitly pending; prior cash position is known.

---
# 8. Trigger
Treasury receives authoritative evidence of failure, return, or uncertainty affecting a previously initiated banking transaction.

---
# 9. Main Business Flow
1. Treasury identifies the affected payment/settlement and preserves its existing history.
2. Treasury classifies the business condition as failed, returned or uncertain.
3. Available external banking evidence is compared with Treasury execution and settlement facts.
4. Until final settlement is confirmed, Treasury does not invent or retain a successful cash consequence.
5. If failure/return is conclusively established, Treasury records the applicable failure fact and preserves cash accordingly.
6. If later evidence conclusively establishes settlement, Treasury records the completed settlement fact and then updates Cash Position.
7. Treasury reconciles the final transaction/cash relationship and makes the result available to affected upstream Finance owners.
8. All evidence and resolution decisions remain auditable.

---
# 10. Alternate Business Flows
An uncertain outcome remains unresolved until authoritative evidence supports a conclusive business fact. Uncertainty itself does not authorize duplicate settlement or automatic reversal.

---
# 11. Failure, Interruption and Business Recovery
Recovery is business-evidence driven. Treasury preserves the last authoritative state and resumes resolution when additional evidence becomes available. No technical retry count, reload, concurrency, rollback or publication-recovery mechanism is specified.

---
# 12. Business Rules and Decision Constraints
- Cash Position changes only after successful settlement completion.
- Failed/returned facts remain distinct from successful settlement.
- External bank records are not overwritten.
- Historical Treasury facts remain immutable.
- Source obligations/receivables are not directly modified by Treasury resolution.

**Authoritative source:** canonical `L2-F10_Treasury_and_Banking.md`; Treasury ownership and downstream technical-recovery boundary: 2026-08-21 amendment §§1 and 7.

---
# 13. State and Lifecycle Impact
Payment/Settlement Execution uses only F10-defined business states/outcomes. No software-recovery state is introduced.

**Authoritative source:** canonical payment, settlement and reconciliation semantics in `L2-F10_Treasury_and_Banking.md`; amendment §7 prohibits implementation-recovery mechanics in L3-F-056 through L3-F-061.

---
# 14. Business Events and Cross-Domain Collaboration
Applicable established F10 facts include `PaymentExecutionFailed`, `SettlementFailed`, `SettlementCompleted`, `BankTransactionRegistered`, `ReconciliationCompleted` and `CashPositionUpdated` when their business conditions are actually satisfied.

**Authoritative source:** canonical event definitions in `L2-F10_Treasury_and_Banking.md`; Treasury remains the owning domain under the 2026-08-21 amendment §§1 and 7.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve original transaction identity, exception classification, external evidence, Treasury determination, settlement/cash outcome, reconciliation evidence and communications to source Finance owners.

---
# 16. Completion Criteria and Postconditions
A conclusive Treasury outcome exists; Cash Position matches only confirmed settlement; no duplicate/false settlement is implied; all ownership and historical evidence remain intact.

---
# 17. Open Issues, Assumptions and Deferred Questions
None material after the 2026-08-21 F10 amendment. External-bank timing remains a business-evidence dependency, not an L3 technical design issue.

---
**End of Scenario L3-F-061**