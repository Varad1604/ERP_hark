# Approved Payment Population to Bank Execution Outcome

# 1. Scenario Metadata
**Scenario ID:** L3-F-057  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** E, F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`; `L3-F-045`

---
# 2. Scenario Purpose
Govern Treasury execution of an approved payment population while preserving ownership of upstream obligations and producing authoritative Treasury payment/settlement outcomes.

---
# 3. Business Outcome
Each approved payment instruction in scope has a governed Treasury execution outcome, with completed settlement facts changing cash only after successful settlement and failed outcomes preserved without false settlement.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
Treasury receives an approved, payment-ready population from authoritative upstream Finance/business owners.
## 4.2 End Boundary
Treasury has recorded the governed execution/settlement outcome for the population and updated cash only for successfully completed settlement.
## 4.3 In Scope
Treasury eligibility, scheduling/batching where applicable, payment execution, settlement outcome and cash consequence.
## 4.4 Out of Scope
Approving liabilities/payroll/procurement obligations, bank protocol/API design, accounting Journal creation and technical retries.

---
# 5. Ownership and Participating Domains
Treasury owns payment execution, settlement and cash. AP/payroll/other source capabilities own the approved obligations. Section J retains supplier/procurement ownership.

---
# 6. Business Actors, Roles and Responsibilities
Treasury executes only approved settlement under F10 governance. Upstream authorities own obligation/payment approval; Treasury does not reinterpret that approval as liability ownership.

**Authoritative source:** `L2-F10_Treasury_and_Banking.md`; Treasury Account authority/operational constraints are reinforced by `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md` §§2–3; AP liability ownership remains fixed by the F08 amendment §4.

---
# 7. Preconditions and Entry Criteria
The payment population is approved and traceable; Treasury Accounts needed for execution are Active; payment readiness and beneficiary references are valid; Treasury restrictions permit execution.

---
# 8. Trigger
An approved payment population becomes eligible for Treasury execution.

---
# 9. Main Business Flow
1. Treasury accepts the approved population and validates settlement eligibility.
2. Payments may be scheduled/grouped under Treasury governance without changing obligation ownership.
3. Treasury begins execution and records the execution state.
4. A successful execution establishes `PaymentExecuted`; execution alone does not necessarily mean final settlement.
5. Successful final settlement establishes `SettlementCompleted`.
6. Cash Position is updated only after successful settlement completion and `CashPositionUpdated` is published.
7. A failed execution/settlement is recorded through the established Treasury failure facts and does not create a false cash movement.
8. Upstream owners consume Treasury outcomes to update their own representations.

---
# 10. Alternate Business Flows
A population may contain completed and non-completed payment outcomes only where each individual payment remains separately traceable; no failed payment is represented as settled.

---
# 11. Failure, Interruption and Business Recovery
Failed or uncertain bank outcomes remain governed Treasury business conditions. Resolution follows F061. L3 does not prescribe message retries, gateway recovery, rollback or persistence behavior.

---
# 12. Business Rules and Decision Constraints
- Treasury never approves the underlying liability.
- Cash changes only after successful settlement completion.
- Treasury Accounts used for execution must be operationally permitted.
- Failed payment/settlement facts remain distinct from successful settlement.

**Authoritative source:** canonical `L2-F10_Treasury_and_Banking.md`; Treasury Account operational rules: 2026-08-21 amendment §§1–2; no-implementation-recovery boundary: amendment §7; AP liability ownership: F08 amendment §4.

---
# 13. State and Lifecycle Impact
Payment Execution and Settlement Execution progress only through F10-defined business states. This scenario does not invent software-processing states.

**Authoritative source:** canonical payment and settlement lifecycles in `L2-F10_Treasury_and_Banking.md`; the 2026-08-21 amendment §7 prohibits downstream implementation-recovery mechanics.

---
# 14. Business Events and Cross-Domain Collaboration
Established F10 events include `PaymentExecutionStarted`, `PaymentExecuted`, `PaymentExecutionFailed`, `SettlementStarted`, `SettlementCompleted`, `SettlementFailed`, and `CashPositionUpdated`, all Treasury-owned.

**Authoritative source:** canonical event definitions in `L2-F10_Treasury_and_Banking.md`; Treasury ownership remains authoritative under the 2026-08-21 amendment §§1 and 7.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve source approval, payment population identity, Treasury Account reference, execution/settlement outcome, cash consequence and correlation to each source obligation.

---
# 16. Completion Criteria and Postconditions
Every in-scope payment has an authoritative Treasury outcome; successful settlements update cash; failures do not; upstream ownership remains intact.

---
# 17. Open Issues, Assumptions and Deferred Questions
No material Treasury lifecycle/authority blocker remains after the 2026-08-21 F10 amendment.

---
**End of Scenario L3-F-057**