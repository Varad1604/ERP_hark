# Receipt or Adjustment to Receivable Settlement

## 1. Specification Metadata

**Specification ID:** L4-F-041  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounts Receivable  
**Realizes Scenario:** L3-F-041  
**Primary Business Owner:** Accounts Receivable  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-041_Receipt_or_Adjustment_to_Receivable_Settlement.md`; `L2-F02_Payment_Management_Architecture.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall incorporate authoritative completed settlement, payment-allocation, or Financial Adjustment facts into an existing Receivable while preserving ownership of the source financial fact and originating obligation.

The behavior begins when Accounts Receivable receives an authoritative completed Finance fact that changes an existing Receivable position. It ends when the Receivable accurately reflects the authoritative partial or full settlement position, remaining outstanding balance, settlement references, and source lineage.

This specification does not execute payment, approve or apply Financial Adjustments, mutate the source obligation, perform Treasury settlement, or post accounting entries.

---

## 3. Required Business Context

The behavior requires:

- an existing Receivable linked to its originating obligation;
- an authoritative completed settlement/payment-allocation or adjustment fact;
- identifiable source reference;
- authoritative amount;
- a determinable relationship to the Receivable;
- Settlement Reconciliation Authority;
- Receivable Review Authority for exceptions.

Accounts Receivable owns the Receivable balance, settlement references, and AR settlement interpretation. Payment, Financial Adjustment, Treasury, and source-obligation ownership remain external.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-041-BH-001 | When a source Finance fact is received, the system shall identify the affected Receivable and validate the incoming source reference. | L3-F-041 §§4.1, 7-9 step 1 |
| L4-F-041-BH-002 | The system shall require Settlement Reconciliation Authority confirmation of the authoritative source amount and Receivable relationship. | L3-F-041 §§6, 9 step 2 |
| L4-F-041-BH-003 | The system shall record the source settlement/adjustment reference without assuming ownership of the source fact. | L3-F-041 §§3, 5, 9 step 3, 12 |
| L4-F-041-BH-004 | The system shall update the Receivable outstanding position only from authoritative completed Finance facts. | L3-F-041 §§2-3, 9 step 4, 12 |
| L4-F-041-BH-005 | The system shall prevent AR from independently inferring settlement where the source owner has not established one. | L3-F-041 §12 |
| L4-F-041-BH-006 | For partial settlement, the system shall preserve the Receivable as outstanding for the authoritative residual amount. | L3-F-041 §§9 step 5, 10.1 |
| L4-F-041-BH-007 | For full settlement, the system shall establish the existing fully-settled Receivable outcome without inventing a new state. | L3-F-041 §§9 step 6, 10.2, 13 |
| L4-F-041-BH-008 | The system shall preserve settlement references as traceable historical evidence. | L3-F-041 §§12, 15-16 |
| L4-F-041-BH-009 | The system shall preserve Payment, Financial Adjustment, Treasury, and source-obligation ownership outside AR. | L3-F-041 §§4.4-5, 12 |
| L4-F-041-BH-010 | The system shall preserve prior and resulting Receivable balances and end-to-end source lineage. | L3-F-041 §§3, 9 step 7, 15-16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-041-BH-011 | When authoritative settlement information has been incorporated into AR, the system shall publish `ReceivableSettlementRecorded` with Accounts Receivable as canonical publisher. | L3-F-041 §14 |
| L4-F-041-BH-012 | When the Receivable is completely satisfied, the system shall publish `ReceivableFullySettled` with Accounts Receivable as canonical publisher. | L3-F-041 §14 |
| L4-F-041-BH-013 | Source Finance events shall remain published by their canonical owners and shall not be republished as AR-owned source facts. | L3-F-041 §§5, 14 |

---

## 6. Failure / Rejection / Continuation

A source-reference, amount, or allocation inconsistency shall prevent AR from applying an uncertain settlement position. The Receivable Review Authority may review the interpretation exception, but correction remains with the source-owning Finance capability. The prior valid Receivable position shall remain authoritative until corrected source evidence exists. No technical retry or rollback is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve Receivable identity, source fact/event, source publisher, amount applied, prior outstanding amount, resulting outstanding amount, settlement/adjustment reference, Settlement Reconciliation Authority participation, exception-review evidence where applicable, and the resulting AR event facts.

---

## 8. Observable Completion Conditions

Completion requires the Receivable to accurately reflect the authoritative source fact, preserve source ownership, maintain the correct residual or fully-settled position, retain immutable settlement references, and preserve reconstructable AR settlement evidence.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-041` — Receipt or Adjustment to Receivable Settlement  
**Primary verification source:** `L2-F07` — Accounts Receivable Management  
**Additional sources:** `L2-F02` Payment Management; `L2-F04` Financial Adjustment Management; F07 locked amendment.

No behavior creates a new settlement source fact, source owner, AR state, authority, event, or publisher.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the parent semantics.

API/interface design, persistence, settlement-calculation implementation, event transport, technical retries, accounting/Treasury mechanics, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
