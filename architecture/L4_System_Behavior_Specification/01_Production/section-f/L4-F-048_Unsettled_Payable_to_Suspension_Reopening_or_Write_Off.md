# Unsettled Payable to Suspension, Reopening, or Write-Off

## 1. Specification Metadata

**Specification ID:** L4-F-048  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounts Payable  
**Realizes Scenario:** L3-F-048  
**Primary Business Owner:** Accounts Payable  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-048_Unsettled_Payable_to_Suspension_Reopening_or_Write_Off.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern an unsettled Payable through AP-owned suspension or reopening decisions, or through referral to the separately owned Financial Adjustment write-off process.

The behavior begins when an unsettled Payable requires exceptional AP governance because ordinary settlement should not continue. It ends when AP has established the current suspension/reopening position or completed a write-off referral; no actual write-off is claimed until authoritative F04 evidence exists.

This specification does not approve or apply a Write-Off Adjustment, execute Treasury settlement, correct procurement facts, or define technical recovery.

---

## 3. Required Business Context

The behavior requires:

- an unsettled Payable;
- supported exceptional reason;
- known current liability;
- known settlement references;
- F08-supported suspension/reopening semantics;
- applicable AP authority;
- F04-owned write-off governance when referral is required.

F08 owns Payable lifecycle/readiness. F04 owns the Write-Off Adjustment. Section J retains procurement/supplier ownership.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-048-BH-001 | The system shall validate the current outstanding Payable position before exceptional governance is applied. | L3-F-048 §§7-9 step 1 |
| L4-F-048-BH-002 | The system shall review the exceptional reason and relevant source evidence without taking ownership of those source facts. | L3-F-048 §§7-9 step 2, 12 |
| L4-F-048-BH-003 | Where settlement should not proceed, the system shall apply only an F08-supported suspension/blocking position. | L3-F-048 §§3, 9 step 3, 13 |
| L4-F-048-BH-004 | When authoritative evidence resolves the condition, the system shall permit reopening of payment readiness under governed AP review. | L3-F-048 §§3, 9 step 4, 10 |
| L4-F-048-BH-005 | Where the liability appears eligible for write-off, the system shall record a referral to F04 write-off governance. | L3-F-048 §§3-5, 9 step 5 |
| L4-F-048-BH-006 | The system shall not reduce or write off the Payable merely because AP created a referral. | L3-F-048 §§4.2, 9 step 6, 12-13 |
| L4-F-048-BH-007 | The system shall change the Payable for write-off only after an authoritative approved/applied F04 fact exists. | L3-F-048 §§4.2, 9 step 6, 14, 16 |
| L4-F-048-BH-008 | If F04 declines the referral, the system shall preserve AP history and retain the appropriate current Payable position. | L3-F-048 §10 |
| L4-F-048-BH-009 | The system shall use only F08-supported states/readiness conditions and shall not create a new write-off state from referral alone. | L3-F-048 §§12-13 |
| L4-F-048-BH-010 | All suspension, reopening, referral, and resulting Payable decisions shall remain traceable. | L3-F-048 §§9 step 7, 15-16 |

---

## 5. Cross-Domain & Event Behavior

AP shall use only established F08 Payable events where defined. Actual write-off facts remain published by F04 and shall not be republished under AP ownership. Suspension/reopening does not alter procurement ownership.

---

## 6. Failure / Rejection / Continuation

Uncertain or contradictory evidence shall preserve the current safe AP position. Business recovery depends on authoritative evidence or an owning-domain decision. No technical retry, rollback, or recovery mechanism is specified.

---

## 7. Audit & Evidence Behavior

Evidence shall preserve Payable identity, exceptional reason, source evidence, AP authority, suspension/reopening decision, referral reference, F04 outcome reference where applicable, prior/resulting Payable position, and proof that no unauthorized settlement/write-off occurred.

---

## 8. Observable Completion Conditions

Completion requires one governed AP position: suspension/block remains, payment readiness is reopened, or write-off referral has been completed. No actual write-off may be represented without authoritative F04 evidence.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-048` — Unsettled Payable to Suspension, Reopening, or Write-Off  
**Primary verification source:** `L2-F08` — Accounts Payable Management  
**Additional verification:** `L2-F04` — Financial Adjustment Management.

No behavior creates a new write-off authority, F08 state, event, publisher, or ownership transfer.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 blocker is identified from the parent semantics.

API/interface design, persistence, workflow implementation, event transport, technical retries, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
