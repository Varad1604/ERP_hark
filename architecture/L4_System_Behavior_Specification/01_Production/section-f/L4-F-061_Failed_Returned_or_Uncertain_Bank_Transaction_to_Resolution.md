# Failed, Returned, or Uncertain Bank Transaction to Resolution

## 1. Specification Metadata

**Specification ID:** L4-F-061  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Treasury & Banking  
**Realizes Scenario:** L3-F-061  
**Primary Business Owner:** Treasury & Banking  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-061_Failed_Returned_or_Uncertain_Bank_Transaction_to_Resolution.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L3-F-057`; `L3-F-058`; `L3-F-060`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall resolve a failed, returned, or uncertain banking outcome to a conclusive Treasury business disposition using authoritative evidence.

The behavior begins when Treasury receives evidence that a payment, settlement, or bank transaction failed, was returned, or has an uncertain external outcome. It ends when Treasury establishes the authoritative business outcome and Cash Position reflects only confirmed completed settlement facts.

The behavior shall not repair bank infrastructure, perform technical retries, roll back data, invent settlement, or modify upstream obligations/receivables.

---

## 3. Required Business Context

Required context is:

- identifiable Treasury payment, settlement, or Bank Transaction;
- original execution and settlement references;
- known prior Cash Position;
- authoritative external banking evidence, or an explicitly unresolved evidence position;
- affected upstream Finance references.

Treasury owns its payment/settlement/Bank Transaction representations and Cash Position. External banks retain ownership of original banking activity. Upstream Finance domains retain obligation/receivable ownership.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-061-BH-001 | When resolution begins, the system shall identify the affected Treasury transaction and preserve its existing history. | L3-F-061 §§7-9, 12 |
| L4-F-061-BH-002 | The system shall classify the condition as failed, returned, or uncertain according to authoritative evidence. | L3-F-061 §§3, 9 |
| L4-F-061-BH-003 | The system shall compare external banking evidence with existing Treasury execution and settlement facts. | L3-F-061 §9 |
| L4-F-061-BH-004 | Until final settlement is authoritatively confirmed, the system shall not invent or retain a successful cash consequence. | L3-F-061 §§9-10, 12 |
| L4-F-061-BH-005 | When payment execution failure is conclusively established, the system shall record `PaymentExecutionFailed` where applicable. | L3-F-061 §§9, 14 |
| L4-F-061-BH-006 | When settlement failure or return is conclusively established, the system shall record `SettlementFailed` where applicable and preserve Cash Position accordingly. | L3-F-061 §§9, 12, 14 |
| L4-F-061-BH-007 | An uncertain outcome shall remain unresolved until authoritative evidence supports a conclusive fact. | L3-F-061 §§10-11 |
| L4-F-061-BH-008 | Uncertainty shall not authorize duplicate settlement or automatic reversal. | L3-F-061 §10 |
| L4-F-061-BH-009 | If later authoritative evidence confirms settlement, the system shall record `SettlementCompleted`. | L3-F-061 §§9, 14 |
| L4-F-061-BH-010 | Cash Position shall change only after confirmed `SettlementCompleted`. | L3-F-061 §§3, 9, 12, 16 |
| L4-F-061-BH-011 | When Cash Position changes from confirmed settlement, the system shall publish `CashPositionUpdated`. | L3-F-061 §§9, 14 |
| L4-F-061-BH-012 | The system shall reconcile the final Treasury transaction and cash relationship from the conclusive evidence. | L3-F-061 §§9, 16 |
| L4-F-061-BH-013 | `ReconciliationCompleted` shall be established only when its canonical Treasury condition is actually satisfied. | L3-F-061 §14 |
| L4-F-061-BH-014 | The system shall make the conclusive Treasury outcome available to affected upstream Finance owners without modifying their aggregates. | L3-F-061 §§5, 9, 12 |
| L4-F-061-BH-015 | Historical Treasury facts and referenced external banking evidence shall remain traceable throughout resolution. | L3-F-061 §§9, 12, 15-16 |

---

## 5. Cross-Domain & Event Behavior

Applicable Treasury facts include:

- `PaymentExecutionFailed`
- `SettlementFailed`
- `SettlementCompleted`
- `BankTransactionRegistered`
- `ReconciliationCompleted`
- `CashPositionUpdated`

Each fact shall be used only when its canonical business condition is satisfied. Treasury remains the canonical publisher of Treasury facts.

Upstream domains consume the conclusive Treasury result without gaining Treasury cash ownership.

---

## 6. Failure / Rejection / Continuation

If the outcome remains uncertain, the system shall preserve the last authoritative Treasury state and keep the business condition unresolved until additional authoritative evidence is available.

No automatic reversal, technical retry count, reload, queue recovery, concurrency recovery, database rollback, or publication recovery is defined.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- original transaction identity;
- execution and settlement references;
- exception classification;
- external banking evidence;
- Treasury determination;
- failure/return/uncertainty chronology;
- final settlement outcome;
- Cash Position consequence;
- reconciliation evidence;
- canonical events;
- communications to affected upstream Finance owners.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. a conclusive Treasury business outcome exists;
2. Cash Position reflects only confirmed completed settlement;
3. no duplicate or false settlement is implied;
4. external-bank and upstream-domain ownership remain unchanged;
5. the complete resolution evidence is reconstructable.

---

## 9. Traceability & Provenance

**Parent:** L3-F-061 — Failed, Returned, or Uncertain Bank Transaction to Resolution  
**Primary verification source:** L2-F10 — Treasury & Banking  
**Material amendment:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`  
**Related parents:** L3-F-057; L3-F-058; L3-F-060  

Parent readiness rechecked against the active 1,084-parent Locked L4 baseline on 2026-08-30.

---

## 10. Open Issues / Deferred Design

External-bank timing remains a business-evidence dependency.

API/interface design, network/gateway recovery, retry orchestration, persistence recovery, event delivery, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft. It requires self-review and independent review before Architecture Approval.
