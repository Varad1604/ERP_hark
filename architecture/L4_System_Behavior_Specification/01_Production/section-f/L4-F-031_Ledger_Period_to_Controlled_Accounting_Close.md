# Ledger Period to Controlled Accounting Close

## 1. Specification Metadata

**Specification ID:** L4-F-031  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounting & General Ledger  
**Realizes Scenario:** L3-F-031  
**Primary Business Owner:** Accounting & General Ledger  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-29  
**Depends On:** `L3-F-031_Ledger_Period_to_Controlled_Accounting_Close.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish complete and traceable Accounting & General Ledger close-readiness evidence for a governing Financial Period and supply that evidence to Financial Period & Closing for the canonical close/lock decision.

The behavior begins when F09 initiates governed period-closing preparation and requires Accounting close-readiness evidence. It ends when Accounting close-readiness evidence is complete, Finance Manager participation confirmation has been recorded, and the readiness facts have been supplied to F09.

This specification does not own, redefine, execute, or publish the Financial Period close, lock, reopen, or archival lifecycle. It does not govern Financial Year close or technical close execution.

---

## 3. Required Business Context

The behavior requires:

- a Financial Period in a close-eligible F09-owned state;
- identifiable posted ledger activity for that period;
- assessable pending Journal population;
- defined reconciliation scope;
- a Trial Balance Snapshot capable of governed generation and verification;
- Accountant preparation responsibility;
- Senior Accountant review responsibility;
- Finance Manager close-readiness participation authority;
- F09 Period Closing / Lock Authority for the actual lifecycle decision.

Accounting & General Ledger owns accounting readiness facts and ledger evidence. Financial Period & Closing retains ownership of the Financial Period lifecycle and its close/lock authority.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-031-BH-001 | When F09 requests accounting close-readiness assessment, the system shall associate the assessment with the authoritative Financial Period reference supplied by F09. | L3-F-031 §§4.1, 7-9 |
| L4-F-031-BH-002 | The system shall identify unresolved Journals that affect the period before Accounting may claim close readiness. | L3-F-031 §§7, 9 step 1, 12 |
| L4-F-031-BH-003 | The system shall prevent Accounting close readiness from being asserted while material unresolved Journals remain without an explicit governed disposition. | L3-F-031 §§9 step 2, 10-12 |
| L4-F-031-BH-004 | The system shall require applicable ledger reconciliation evidence to be completed and reviewed before Accounting close readiness may be asserted. | L3-F-031 §§9 step 3, 12 |
| L4-F-031-BH-005 | The system shall require a Trial Balance Snapshot generated and verified under the governed Trial Balance scenario before Accounting close readiness may be asserted where Trial Balance evidence is applicable. | L3-F-031 §§7, 9 step 4, 12 |
| L4-F-031-BH-006 | The system shall record accounting close-readiness evidence together with any unresolved blocking accounting exception. | L3-F-031 §§9 step 5, 10-11 |
| L4-F-031-BH-007 | The system shall prevent a positive Accounting close-readiness outcome while a material Journal, reconciliation, or Trial Balance blocker remains unresolved. | L3-F-031 §§10-12 |
| L4-F-031-BH-008 | When the required accounting evidence is complete, the system shall permit the Finance Manager to confirm Accounting participation readiness for F09 close governance. | L3-F-031 §§6, 9 step 6, 15 |
| L4-F-031-BH-009 | After Finance Manager confirmation, the system shall make the Accounting close-readiness facts and supporting evidence available to F09 for its canonical close/lock decision. | L3-F-031 §§2-3, 9 step 7, 16 |
| L4-F-031-BH-010 | The system shall not establish, alter, or publish a Financial Period lifecycle state as an F05-owned outcome. | L3-F-031 §§2-5, 12-14, 16 |
| L4-F-031-BH-011 | The system shall preserve the distinction between Accounting close-readiness evidence and the F09-owned period close/lock decision. | L3-F-031 §§3, 5, 9 step 8, 13-16 |
| L4-F-031-BH-012 | The system shall consume the F09-owned Financial Period state as authoritative and shall not redefine the canonical lifecycle `Open → Closing → Closed → Locked → Archived`. | L3-F-031 §§12-13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-031-BH-013 | The system shall retain or consume Accounting-owned readiness facts such as `LedgerReconciled` and `TrialBalancePublished` as supporting evidence for close readiness. | L3-F-031 §14 |
| L4-F-031-BH-014 | The system shall provide Accounting close-readiness evidence to F09 without transferring ownership of the Financial Period lifecycle to Accounting & General Ledger. | L3-F-031 §§5, 9 steps 7-8, 14 |
| L4-F-031-BH-015 | The system shall not publish an F05-owned Financial Period close, lock, reopen, or archive event. | L3-F-031 §§13-14 |
| L4-F-031-BH-016 | The system shall recognize F09 as the sole publisher of canonical Financial Period lifecycle facts used to establish the close/lock outcome. | L3-F-031 §§13-14, 16 |

The F05-to-F09 relationship is a governed business hand-off of readiness evidence, not a transfer of lifecycle ownership.

---

## 6. Failure / Rejection / Continuation

- If material unresolved Journals remain, the system shall withhold positive Accounting close readiness and preserve the blocking evidence.
- If reconciliation remains in an unresolved exception condition, the system shall withhold positive Accounting close readiness.
- If required Trial Balance evidence is invalid or not verified/published as required by its governing scenario, the system shall withhold positive Accounting close readiness.
- Recovery shall occur through the canonically owning Journal, reconciliation, or Trial Balance workflow before Accounting reassesses readiness.
- The system shall not use technical rollback, close-job retry, or infrastructure recovery mechanics in this specification.
- F09 may independently determine its own period outcome only through F09-governed lifecycle rules after consuming the readiness evidence.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to reconstruct:

- the authoritative Financial Period reference;
- identified pending Journals and their disposition;
- ledger reconciliation status and supporting reconciliation evidence;
- Trial Balance Snapshot identity and verification/publication evidence;
- unresolved accounting blockers where applicable;
- Accountant preparation participation;
- Senior Accountant review participation;
- Finance Manager close-readiness confirmation;
- the readiness facts supplied to F09;
- evidence that Accounting did not establish or publish the Financial Period lifecycle outcome.

F09 separately preserves the authoritative close/lock decision and its authority evidence.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. pending accounting work affecting the period has been resolved or explicitly governed;
2. applicable reconciliation evidence is complete and reviewed;
3. applicable Trial Balance evidence has been generated and verified under its governing scenario;
4. no material Accounting blocker remains;
5. Finance Manager has confirmed Accounting participation readiness;
6. Accounting close-readiness evidence is complete and traceable;
7. the readiness facts have been supplied to F09;
8. no F05-owned Financial Period state transition or lifecycle event has been established.

Any subsequent `Closed` or `Locked` Financial Period status is authoritative only when established under F09 ownership.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-031` — Ledger Period to Controlled Accounting Close  
**Primary verification source:** `L2-F05` — Accounting & General Ledger Management  
**Material amendment:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Additional verification source:** `L2-F09` — Financial Period & Closing amendment  

Behavioral provenance is anchored in `L3-F-031`.

The locked F05/F09 amendments verify that Accounting owns close-readiness facts and ledger evidence while F09 exclusively owns Financial Period close, lock, reopen, and archival lifecycle governance. They also verify the Accountant, Senior Accountant, Finance Manager, and F09 Period Closing / Lock Authority boundaries.

No L4 behavior in this specification creates a new Financial Period state, close authority, event, publisher, lifecycle transition, or ownership assignment.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue is identified from the supplied parent semantics.

API/interface design, persistence/data model, service/component design, technical close orchestration, job scheduling, event transport, technical access control, retry/rollback mechanics, infrastructure/deployment, and UI/UX remain deferred to downstream architecture levels.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
