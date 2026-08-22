# Treasury Demand to Current Cash Position and Liquidity Decision

# 1. Scenario Metadata
**Scenario ID:** L3-F-059  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L3-F-058`

---
# 2. Scenario Purpose
Evaluate a Treasury demand against the current authoritative Cash Position and Liquidity Position to produce a governed Treasury liquidity decision without approving the underlying business obligation.

---
# 3. Business Outcome
Treasury has an authoritative current cash/liquidity assessment and a governed decision on whether the demand is financially supportable under Treasury policy and restrictions.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
A valid Treasury demand requiring cash/liquidity assessment exists.
## 4.2 End Boundary
The current Cash Position/Liquidity Position and the Treasury decision are recorded and traceable.
## 4.3 In Scope
Cash/liquidity assessment, reserve/restriction considerations, decision evidence and forecast context where applicable.
## 4.4 Out of Scope
Approving the source obligation, executing payment, changing budgets, accounting posting or technical calculation implementation.

---
# 5. Ownership and Participating Domains
Treasury exclusively owns Cash Position and Liquidity Position. Upstream domains own their demands/obligations and consume the Treasury assessment without gaining cash ownership.

---
# 6. Business Actors, Roles and Responsibilities
Treasury governance evaluates liquidity/cash under F10 policies. Source-domain approval authorities remain responsible for the business obligation; Treasury does not replace them.

**Authoritative source:** canonical `L2-F10_Treasury_and_Banking.md`; Treasury ownership and authority boundaries remain reinforced by `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md` §§1, 3 and 7.

---
# 7. Preconditions and Entry Criteria
Current Treasury cash/settlement evidence is available; applicable liquidity/reserve policies and restrictions are effective; the demand amount, timing and currency are identifiable.

---
# 8. Trigger
A Treasury demand requires confirmation of current cash and liquidity supportability.

---
# 9. Main Business Flow
1. Treasury validates the demand reference, amount, timing and currency.
2. Treasury determines the current authoritative Cash Position from completed Treasury facts.
3. Treasury evaluates current and applicable forecast Liquidity Position.
4. Treasury applies reserve, restriction and liquidity policy constraints.
5. Treasury records whether the demand is supportable, constrained or not currently supportable under Treasury governance.
6. The decision and supporting cash/liquidity evidence are preserved for the requesting business process.
7. No cash movement occurs merely because the decision is positive.

---
# 10. Alternate Business Flows
A constrained decision may identify that only a permitted portion/timing is supportable where institutional policy allows; this does not alter the source obligation or execute payment.

---
# 11. Failure, Interruption and Business Recovery
If current cash/liquidity evidence is not sufficiently authoritative, Treasury withholds a definitive decision until business evidence is reconciled. No cache refresh, reload, persistence retry or technical recovery is specified.

---
# 12. Business Rules and Decision Constraints
- Treasury owns cash and liquidity.
- Forecasts do not modify actual cash state.
- Treasury assessment does not approve the underlying obligation.
- Cash movement occurs only through governed settlement execution.
- Treasury restrictions and reserve policies remain applicable.

**Authoritative source:** canonical `L2-F10_Treasury_and_Banking.md`; Treasury Account restrictions/operational governance: 2026-08-21 amendment §§1–2; downstream L3 purity: amendment §7.

---
# 13. State and Lifecycle Impact
Cash/Liquidity Position may be updated only from authoritative Treasury business facts; this assessment itself does not create a settlement lifecycle transition.

**Authoritative source:** canonical Cash Position, Liquidity Position and settlement semantics in `L2-F10_Treasury_and_Banking.md`.

---
# 14. Business Events and Cross-Domain Collaboration
`CashPositionUpdated`, `LiquidityPositionUpdated` and `LiquidityForecastUpdated` remain Treasury-owned facts when their underlying business changes occur. The liquidity decision is not represented by an invented event if F10 has not named one.

**Authoritative source:** canonical event definitions in `L2-F10_Treasury_and_Banking.md`; no new downstream event is introduced under the 2026-08-21 amendment §7.

---
# 15. Business Authorization, Approval and Audit Obligations
Preserve demand reference, cash/liquidity evidence, applicable policy/restriction basis, Treasury decision and responsible governance authority.

---
# 16. Completion Criteria and Postconditions
A current Treasury decision exists on authoritative cash/liquidity evidence; no obligation ownership or cash movement is implied by the assessment; evidence is reconstructable.

---
# 17. Open Issues, Assumptions and Deferred Questions
Institution-specific liquidity thresholds remain configuration policy. No material Treasury architecture blocker remains.

---
**End of Scenario L3-F-059**