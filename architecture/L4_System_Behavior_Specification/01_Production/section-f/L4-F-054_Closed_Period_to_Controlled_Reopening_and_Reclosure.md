# Closed Period to Controlled Reopening and Reclosure

## 1. Specification Metadata

**Specification ID:** L4-F-054  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Financial Period & Closing  
**Realizes Scenario:** L3-F-054  
**Primary Business Owner:** Financial Period & Closing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-054_Closed_Period_to_Controlled_Reopening_and_Reclosure.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L3-F-052`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern exceptional reopening of a `Closed` Financial Period and its subsequent reclosure without erasing or rewriting prior close evidence.

The behavior begins when a `Closed` Financial Period has an authoritative and documented business need for exceptional reopening. It ends when the corrective window concludes and the period has passed the normal F09 close controls again to `Closed` and, where required, `Locked`.

This specification does not define the corrective Journal itself, reopen the Financial Year, alter prior closure evidence, or define technical rollback.

---

## 3. Required Business Context

The behavior requires:

- Financial Period currently `Closed`;
- documented reopening basis and scope;
- explicit Period Reopening Authority;
- preserved prior closure evidence;
- separately governed F05 corrective accounting where needed;
- updated close-readiness evidence;
- Period Closing Authority;
- Period Lock Authority where protected status is required.

F09 owns reopening/reclosure. F05 owns corrective accounting and readiness evidence only.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-054-BH-001 | The system shall record the exceptional reopening request and business justification against the authoritative Financial Period. | L3-F-054 §§7-9 step 1 |
| L4-F-054-BH-002 | The system shall require Period Reopening Authority approval before a `Closed` period may reopen. | L3-F-054 §§6, 9 step 2, 12 |
| L4-F-054-BH-003 | When reopening is authorized, the system shall move the Financial Period from `Closed` to `Open`. | L3-F-054 §§3, 9 step 3, 13 |
| L4-F-054-BH-004 | The system shall preserve prior closure evidence unchanged when the period reopens. | L3-F-054 §§2-4, 9 step 3, 12 |
| L4-F-054-BH-005 | The system shall permit only separately governed corrective accounting under F05 during the reopened window. | L3-F-054 §§4.3-5, 9 step 4, 12 |
| L4-F-054-BH-006 | The system shall consume updated F05 close-readiness evidence after corrective activity without transferring lifecycle ownership. | L3-F-054 §§5, 9 step 5 |
| L4-F-054-BH-007 | The reopened Financial Period shall pass the same normal close controls before reclosure. | L3-F-054 §§9 step 6, 12 |
| L4-F-054-BH-008 | When close prerequisites are satisfied, the system shall permit Period Closing Authority to return the period to `Closed`. | L3-F-054 §§6, 9 step 7, 13 |
| L4-F-054-BH-009 | Where protected historical status is required, the system shall require Period Lock Authority before moving the period to `Locked`. | L3-F-054 §§6, 9 step 8, 13 |
| L4-F-054-BH-010 | The system shall preserve linked reopening, corrective-reference, successor close, and successor lock evidence. | L3-F-054 §§9 step 9, 15-16 |
| L4-F-054-BH-011 | If reopening is denied, the system shall retain the period as `Closed` and prevent corrective posting through this scenario. | L3-F-054 §10 |

---

## 5. Cross-Domain & Event Behavior

Financial Period lifecycle facts remain F09-owned. F05 supplies only corrective accounting facts and close-readiness evidence. No F05 event or correction fact shall be republished as an F09 lifecycle fact.

---

## 6. Failure / Rejection / Continuation

An interrupted reopening/reclosure process shall preserve the current authoritative F09 state. The process resumes under the same business governance after missing prerequisites are satisfied. No technical rollback, persistence recovery, or retry behavior is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve reopening reason, reopening scope, Period Reopening Authority decision, prior closure evidence, corrective accounting references, updated readiness evidence, Period Closing Authority decision, Period Lock Authority decision where applicable, and resulting lifecycle states.

---

## 8. Observable Completion Conditions

Completion requires the authorized corrective need to be governed, prior closure history to remain immutable, the same Financial Period to be reclosed and where applicable relocked under F09 authority, and all reopening-to-reclosure evidence to remain reconstructable.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-054` — Closed Period to Controlled Reopening and Reclosure  
**Primary verification source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Accounting source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L3-F-052`

No behavior invents a new reopening state, authority, publisher, correction mechanism, or ownership transfer.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 blocker is identified from the parent semantics.

API/interface design, persistence, corrective-Journal implementation, event transport, technical rollback/retry, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
