# Open Financial Period to Locked Close

## 1. Specification Metadata

**Specification ID:** L4-F-052  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Financial Period & Closing  
**Realizes Scenario:** L3-F-052  
**Primary Business Owner:** Financial Period & Closing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-F-052_Open_Financial_Period_to_Locked_Close.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L3-F-031`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern an `Open` Financial Period through accounting close-readiness, `Closing`, `Closed`, and protected `Locked` status under exclusive F09 lifecycle ownership.

The behavior begins when a Financial Period is `Open` and F09 initiates its governed close cycle. It ends when the Financial Period reaches `Locked` and ordinary posting/modification is no longer permitted.

This specification does not produce Journals, reconciliation, or Trial Balance evidence, govern reopening or archival, close the Financial Year, or define technical close execution.

---

## 3. Required Business Context

The behavior requires:

- Financial Period currently `Open`;
- resolved or governed pending accounting work;
- applicable reconciliation evidence;
- verified Trial Balance evidence where required;
- no unresolved material close blocker;
- Period Closing Authority;
- Period Lock Authority;
- F05 accounting readiness evidence under L3-F-031.

F09 exclusively owns the Financial Period lifecycle. F05 owns only the Accounting readiness facts supplied to F09.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-052-BH-001 | When F09 initiates the close cycle, the system shall move the Financial Period from `Open` to `Closing`. | L3-F-052 §§4.1, 8-9 step 1, 13 |
| L4-F-052-BH-002 | On entry to `Closing`, the system shall require applicable Finance close-readiness evidence. | L3-F-052 §§7, 9 steps 1-2 |
| L4-F-052-BH-003 | The system shall consume F05 pending-Journal, reconciliation, and Trial Balance readiness without transferring Financial Period ownership to F05. | L3-F-052 §§5-6, 9 step 2, 14 |
| L4-F-052-BH-004 | The system shall prevent close progression while a material accounting-readiness exception remains unresolved. | L3-F-052 §§7, 9 step 3, 10, 12 |
| L4-F-052-BH-005 | When close prerequisites are satisfied, the system shall permit the Period Closing Authority to move the Financial Period from `Closing` to `Closed`. | L3-F-052 §§6, 9 step 4, 13 |
| L4-F-052-BH-006 | The system shall preserve closure evidence when `Closing → Closed` completes. | L3-F-052 §§9 step 5, 15 |
| L4-F-052-BH-007 | The system shall require Period Lock Authority confirmation before a `Closed` Financial Period may become `Locked`. | L3-F-052 §§6, 9 step 6, 12 |
| L4-F-052-BH-008 | When lock prerequisites are satisfied, the system shall move the Financial Period from `Closed` to `Locked`. | L3-F-052 §§3, 9 step 7, 13 |
| L4-F-052-BH-009 | The system shall not permit direct `Closing → Locked` or `Open → Locked` bypass of `Closed`. | L3-F-052 §§10, 12-13 |
| L4-F-052-BH-010 | Once `Closed` or `Locked`, the system shall prevent ordinary posting into the period according to authoritative F09 state. | L3-F-052 §§4.2, 12, 16 |
| L4-F-052-BH-011 | F09 shall remain the canonical owner and publisher of Financial Period lifecycle facts throughout the close/lock process. | L3-F-052 §§5, 9 step 8, 14 |
| L4-F-052-BH-012 | F05 readiness facts shall not be substituted for F09-owned period lifecycle events or transitions. | L3-F-052 §§5, 14 |

---

## 5. Cross-Domain & Event Behavior

F05 supplies accounting-readiness facts only. F09 remains the canonical publisher and owner of Financial Period lifecycle facts. No F05 event shall be used as a substitute for an F09 close/lock lifecycle fact.

---

## 6. Failure / Rejection / Continuation

If a material close prerequisite fails, the Financial Period shall remain `Closing` until the owning workflow resolves the issue. The system shall not skip directly to `Closed` or `Locked`. Business interruption shall preserve the current F09 state and close evidence. No technical retry or rollback is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve Financial Period identity, close-initiation evidence, Accounting readiness references, unresolved exceptions, Period Closing Authority decision, closure evidence, Period Lock Authority decision, timestamps/chronology, and each authoritative lifecycle transition.

---

## 8. Observable Completion Conditions

Completion requires the Financial Period to be `Locked`, ordinary posting to be prevented by authoritative F09 state, `Open → Closing → Closed → Locked` chronology to be reconstructable, and all close/lock authority evidence to be preserved.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-052` — Open Financial Period to Locked Close  
**Primary verification source:** `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Accounting-readiness source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L3-F-031`

The locked F09 amendment verifies the canonical lifecycle `Planned/Registered → Open → Closing → Closed → Locked → Archived`, the Period Closing Authority, Period Lock Authority, and exclusive F09 ownership.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 blocker is identified from the parent semantics.

API/interface design, persistence, technical close jobs, posting-engine mechanics, event transport, retry/rollback, infrastructure, and UI/UX remain deferred.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
