# Authorized Fund Transfer to Settled Inter-Account Movement

## 1. Specification Metadata

**Specification ID:** L4-F-060  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Treasury & Banking  
**Realizes Scenario:** L3-F-060  
**Primary Business Owner:** Treasury & Banking  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-060_Authorized_Fund_Transfer_to_Settled_Inter_Account_Movement.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L3-F-056`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorized institutional fund transfer between eligible Treasury Accounts through completed Treasury settlement.

The behavior begins when an authorized inter-account transfer instruction becomes eligible for settlement. It ends when settlement is completed and source and destination Cash Positions reflect the settled movement.

The behavior shall not create Accounting Journals, alter external bank records, or change cash before successful settlement completion.

---

## 3. Required Business Context

Required context is:

- source Treasury Account is `Active`;
- destination Treasury Account is `Active`;
- both accounts are operationally permitted;
- valid amount and currency;
- sufficient governed cash/liquidity;
- recorded Treasury authorization;
- applicable Treasury restrictions.

Treasury owns transfer execution, settlement, and Cash Position. Accounting owns resulting Journal/ledger consequences.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-060-BH-001 | Before settlement begins, the system shall validate source account, destination account, amount, currency, authority, and restrictions. | L3-F-060 §§7-9 |
| L4-F-060-BH-002 | The system shall prevent progression unless both Treasury Accounts are `Active` and operationally permitted. | L3-F-060 §§7, 12 |
| L4-F-060-BH-003 | The system shall confirm the transfer is supportable under current cash/liquidity governance. | L3-F-060 §§7, 9, 12 |
| L4-F-060-BH-004 | The system shall require recorded Treasury authorization before settlement initiation. | L3-F-060 §§6-8, 12 |
| L4-F-060-BH-005 | When governed settlement begins, the system shall establish `SettlementStarted` where its canonical F10 condition is satisfied. | L3-F-060 §§9, 14 |
| L4-F-060-BH-006 | Only successful final settlement shall establish `SettlementCompleted`. | L3-F-060 §§9, 12, 14 |
| L4-F-060-BH-007 | Source and destination Cash Positions shall change only after `SettlementCompleted`. | L3-F-060 §§3, 9, 12, 16 |
| L4-F-060-BH-008 | On completed settlement, the system shall apply consistent source and destination cash consequences for the settled amount. | L3-F-060 §§3, 9 |
| L4-F-060-BH-009 | When Cash Position changes, the system shall publish `CashPositionUpdated` with Treasury & Banking as canonical publisher. | L3-F-060 §§9, 14 |
| L4-F-060-BH-010 | If settlement fails, the system shall establish `SettlementFailed` where applicable and shall not claim a settled transfer. | L3-F-060 §§10-11, 14 |
| L4-F-060-BH-011 | Treasury Account lifecycle state shall remain unchanged by the transfer unless separately governed. | L3-F-060 §13 |
| L4-F-060-BH-012 | Transfer, settlement, and cash evidence shall remain correlated for Accounting and audit consumption. | L3-F-060 §§9, 15-16 |

---

## 5. Cross-Domain & Event Behavior

Treasury owns `SettlementStarted`, `SettlementCompleted`, `SettlementFailed`, and `CashPositionUpdated`.

Accounting may consume completed Treasury facts to establish its own Journal/ledger consequences. Accounting does not modify Treasury state through that consumption.

---

## 6. Failure / Rejection / Continuation

If settlement fails or remains uncertain, the system shall preserve the last authoritative Cash Position and shall not claim the inter-account movement as settled.

Resolution follows L3-F-061.

No technical retry, rollback, gateway recovery, or publication recovery is defined.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- source Treasury Account identity;
- destination Treasury Account identity;
- transfer amount and currency;
- authorization;
- cash/liquidity supportability evidence;
- settlement outcome;
- source/destination Cash Position effects;
- event chronology;
- downstream Accounting correlation references.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. `SettlementCompleted` exists;
2. source and destination Cash Positions reflect the settled movement consistently;
3. Treasury ownership remains intact;
4. Accounting ownership remains separate;
5. the transfer and settlement evidence are reconstructable.

---

## 9. Traceability & Provenance

**Parent:** L3-F-060 — Authorized Fund Transfer to Settled Inter-Account Movement  
**Primary verification source:** L2-F10 — Treasury & Banking  
**Material amendment:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`  
**Related parent:** L3-F-056 — Bank Account Request to Active or Closed Treasury Account  

Parent readiness rechecked against the active 1,084-parent Locked L4 baseline on 2026-08-30.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue remains.

API/interface design, banking protocol, settlement implementation, persistence, retry/rollback mechanics, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft. It requires self-review and independent review before Architecture Approval.
