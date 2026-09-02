# Approved Payment Population to Bank Execution Outcome

## 1. Specification Metadata

**Specification ID:** L4-F-057  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Treasury & Banking  
**Realizes Scenario:** L3-F-057  
**Primary Business Owner:** Treasury & Banking  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-057_Approved_Payment_Population_to_Bank_Execution_Outcome.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`; `L3-F-045`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern Treasury execution of an approved payment population while preserving upstream obligation ownership and updating institutional cash only after authoritative successful settlement.

The behavior begins when Treasury receives an approved, payment-ready population from authoritative upstream owners. It ends when each in-scope payment has a governed Treasury execution/settlement outcome and Cash Position reflects only successfully completed settlements.

This specification does not approve liabilities/payroll/procurement obligations, define banking protocols/APIs, create accounting Journals, or prescribe technical retries.

---

## 3. Required Business Context

The behavior requires:

- approved and traceable payment population;
- valid payment readiness and beneficiary references;
- required Treasury Accounts in `Active`;
- no Treasury restriction preventing execution;
- upstream approval retained by source capabilities;
- Treasury eligibility and settlement governance.

Treasury owns execution, settlement, and cash. Upstream domains retain the approved obligations.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-057-BH-001 | The system shall accept only approved, traceable payment instructions from authoritative upstream owners. | L3-F-057 §§4.1, 7-9 step 1 |
| L4-F-057-BH-002 | Before execution, the system shall validate Treasury settlement eligibility, including operationally permitted Treasury Accounts. | L3-F-057 §§7, 9 step 1, 12 |
| L4-F-057-BH-003 | Treasury scheduling/grouping shall not change ownership of the underlying obligations. | L3-F-057 §§5, 9 step 2 |
| L4-F-057-BH-004 | When Treasury begins payment execution, the system shall publish/record `PaymentExecutionStarted` where the canonical F10 lifecycle fact occurs. | L3-F-057 §§9 step 3, 14 |
| L4-F-057-BH-005 | When payment execution succeeds, the system shall establish `PaymentExecuted`; execution alone shall not be treated as final settlement. | L3-F-057 §§9 step 4, 12, 14 |
| L4-F-057-BH-006 | When final settlement begins, the system shall use the canonical `SettlementStarted` business fact where applicable. | L3-F-057 §14 |
| L4-F-057-BH-007 | Only successful final settlement shall establish `SettlementCompleted`. | L3-F-057 §§9 step 5, 12, 14 |
| L4-F-057-BH-008 | The system shall update Cash Position only after `SettlementCompleted`. | L3-F-057 §§3, 9 step 6, 12, 16 |
| L4-F-057-BH-009 | When Cash Position changes from successful settlement, the system shall publish `CashPositionUpdated` with Treasury & Banking as canonical publisher. | L3-F-057 §§9 step 6, 14 |
| L4-F-057-BH-010 | Failed execution shall establish `PaymentExecutionFailed` rather than a successful payment fact. | L3-F-057 §§9 step 7, 12, 14 |
| L4-F-057-BH-011 | Failed settlement shall establish `SettlementFailed` and shall not create a false cash movement. | L3-F-057 §§9 step 7, 12, 14 |
| L4-F-057-BH-012 | Where a population contains mixed results, each individual payment outcome shall remain separately traceable. | L3-F-057 §10 |
| L4-F-057-BH-013 | Upstream owners shall consume Treasury outcomes to update their own representations without ownership transfer. | L3-F-057 §§5, 9 step 8 |

---

## 5. Cross-Domain & Event Behavior

Treasury owns `PaymentExecutionStarted`, `PaymentExecuted`, `PaymentExecutionFailed`, `SettlementStarted`, `SettlementCompleted`, `SettlementFailed`, and `CashPositionUpdated`. AP/payroll/procurement domains retain obligation ownership and may only consume Treasury facts.

---

## 6. Failure / Rejection / Continuation

Failed or uncertain bank outcomes shall remain governed Treasury business conditions. A failed payment shall never be represented as settled. Cash shall remain unchanged until successful settlement. Resolution of failed/uncertain outcomes belongs to F-061. No technical retry, gateway recovery, rollback, or persistence behavior is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve source approval, payment population identity, each payment instruction identity, Treasury Account reference, beneficiary reference, execution outcome, settlement outcome, cash consequence, canonical events, and correlation to each source obligation.

---

## 8. Observable Completion Conditions

Completion requires every in-scope payment to have an authoritative Treasury outcome, successful settlements to be the only outcomes changing Cash Position, failed outcomes to remain distinct, and upstream obligation ownership to remain intact.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-057` — Approved Payment Population to Bank Execution Outcome  
**Primary verification source:** `L2-F10` — Treasury & Banking  
**Locked amendment:** `L2-F10_Treasury_and_Banking_Amendment_2026-08-21.md`  
**Additional verification:** F08 amendment and `L3-F-045`.

No behavior creates upstream approval, changes obligation ownership, or equates execution with settlement.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 blocker is identified from the parent semantics.

API/interface design, banking protocols, gateway mechanics, persistence, technical retries, batch execution implementation, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
