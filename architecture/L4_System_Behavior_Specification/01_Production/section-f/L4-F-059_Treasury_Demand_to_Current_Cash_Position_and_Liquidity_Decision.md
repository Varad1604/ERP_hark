# Treasury Demand to Current Cash Position and Liquidity Decision

## 1. Specification Metadata

**Specification ID:** L4-F-059  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Treasury & Banking  
**Realizes Scenario:** L3-F-059  
**Primary Business Owner:** Treasury & Banking  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-059_Treasury_Demand_to_Current_Cash_Position_and_Liquidity_Decision.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L3-F-058`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall evaluate a valid Treasury demand against authoritative Cash Position and Liquidity Position evidence and establish a governed Treasury supportability decision.

The behavior begins when a valid Treasury demand requires cash/liquidity assessment. It ends when the current cash/liquidity assessment and Treasury decision are recorded and traceable.

The behavior shall not approve the underlying obligation, execute payment, change budgets, post accounting entries, or move cash merely because the decision is positive.

---

## 3. Required Business Context

Required context is:

- valid demand reference;
- identifiable amount, timing, and currency;
- authoritative Treasury cash and settlement facts;
- current Liquidity Position and applicable forecast context;
- effective reserve, restriction, and liquidity policies.

Treasury owns Cash Position and Liquidity Position. The requesting domain retains ownership of its demand/obligation.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-059-BH-001 | When assessment begins, the system shall validate the demand reference, amount, timing, and currency. | L3-F-059 §§7-9 |
| L4-F-059-BH-002 | The system shall determine current Cash Position only from authoritative completed Treasury facts. | L3-F-059 §§9, 12-13 |
| L4-F-059-BH-003 | The system shall evaluate current Liquidity Position and applicable forecast liquidity context. | L3-F-059 §§3, 9 |
| L4-F-059-BH-004 | Forecast information shall not modify actual Cash Position. | L3-F-059 §12 |
| L4-F-059-BH-005 | The system shall apply applicable reserve, restriction, and liquidity-policy constraints. | L3-F-059 §§9, 12 |
| L4-F-059-BH-006 | The system shall record the demand as supportable, constrained, or not currently supportable according to Treasury governance. | L3-F-059 §§3, 9 |
| L4-F-059-BH-007 | Where policy permits a constrained result, the system shall preserve the permitted portion or timing without changing the source obligation. | L3-F-059 §10 |
| L4-F-059-BH-008 | A positive supportability decision shall not create settlement or cash movement. | L3-F-059 §§9, 12, 16 |
| L4-F-059-BH-009 | If authoritative cash/liquidity evidence is insufficient, the system shall withhold a definitive decision. | L3-F-059 §11 |
| L4-F-059-BH-010 | The system shall preserve the decision and supporting evidence for the requesting business process. | L3-F-059 §§9, 15-16 |

---

## 5. Cross-Domain & Event Behavior

`CashPositionUpdated`, `LiquidityPositionUpdated`, and `LiquidityForecastUpdated` remain Treasury-owned facts only when their underlying Treasury business facts occur.

No liquidity-decision event shall be invented where F10 has not canonically defined one.

The requesting domain may consume the Treasury assessment without acquiring Cash Position or Liquidity Position ownership.

---

## 6. Failure / Rejection / Continuation

If current cash/liquidity evidence is not authoritative enough to support a decision, the system shall preserve the last valid Treasury position and withhold a definitive result until the business evidence is reconciled or clarified.

No technical retry, reload, rollback, or recovery behavior is defined.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- demand identity;
- amount, timing, and currency;
- Cash Position evidence;
- Liquidity Position evidence;
- forecast references where applicable;
- policy/restriction basis;
- Treasury decision;
- responsible governance authority.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. a current Treasury supportability decision exists on authoritative evidence;
2. the demand remains owned by its source domain;
3. no cash movement is implied merely by the assessment;
4. the decision and evidence are reconstructable.

---

## 9. Traceability & Provenance

**Parent:** L3-F-059 — Treasury Demand to Current Cash Position and Liquidity Decision  
**Primary verification source:** L2-F10 — Treasury & Banking  
**Material amendment:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`  
**Related parent:** L3-F-058 — Bank Statement to Reconciled Cash Position  

Parent readiness rechecked against the active 1,084-parent Locked L4 baseline on 2026-08-30.

---

## 10. Open Issues / Deferred Design

Institution-specific liquidity and reserve thresholds remain configuration-governed.

API/interface design, persistence, forecasting algorithms, calculation implementation, event transport, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft. It requires self-review and independent review before Architecture Approval.
