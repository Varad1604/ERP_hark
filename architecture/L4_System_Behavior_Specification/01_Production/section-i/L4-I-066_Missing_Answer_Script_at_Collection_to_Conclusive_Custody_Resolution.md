# Missing Answer Script at Collection to Conclusive Custody Resolution

## 1. Specification Metadata


**Specification ID:** ** `L4-I-066`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-066` — Missing Answer Script at Collection to Conclusive Custody Resolution  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-066_Missing_Answer_Script_at_Collection_to_Conclusive_Custody_Resolution.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed resolution of an answer-script shortfall detected during collection by preserving search, reconciliation, evidence, and final custody disposition.

The behavior begins when script reconciliation detects a shortfall against attendance. It ends when the Answer Script Custody Ledger Aggregate reaches `SCRIPT_RECONCILED_OR_DEFICIT_SEALED`.

---

## 3. Required Business Context

- Completed or ending examination session.
- Attendance muster.
- Collected answer-script count.
- Chief Superintendent.
- University Observer.
- Hall Invigilators.
- Controller of Examinations.
- Parent-defined 100% physical reconciliation rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-066-BH-001` | When an answer-script shortfall is detected, the system shall establish the Answer Script Custody Ledger in `SHORTFALL_DETECTED`. | L3-I-066 §§4, 9, 13 |
| `L4-I-066-BH-002` | The system shall preserve expected script count, actual script count, and affected candidate/session context. | L3-I-066 §§2–3 |
| `L4-I-066-BH-003` | The system shall require the parent-defined physical reconciliation and evidence review under BR-I07-012. | L3-I-066 §12 |
| `L4-I-066-BH-004` | The system shall preserve hall-search, candidate-signature, and invigilator-testimony evidence used in the resolution. | L3-I-066 §§2–3 |
| `L4-I-066-BH-005` | If the missing script is located, the system shall preserve recovery and custody restoration evidence. | L3-I-066 §3 |
| `L4-I-066-BH-006` | If the deficit is conclusively unresolved as stolen/unsubmitted, the system shall preserve the formal deficit disposition and related incident/malpractice handoff without conducting those downstream proceedings. | L3-I-066 §3 |
| `L4-I-066-BH-007` | The system shall prevent normal custody closure until the shortfall has a governed conclusive disposition. | L3-I-066 §§11–12 |
| `L4-I-066-BH-008` | On completion, the system shall transition the Answer Script Custody Ledger to `SCRIPT_RECONCILED_OR_DEFICIT_SEALED`. | L3-I-066 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-066-BH-009` | The system shall preserve handoff to security/police/malpractice processes where required without transferring ownership of those proceedings to I07. | L3-I-066 §§3, 5 |
| `L4-I-066-BH-010` | When the shortfall is conclusively resolved, the system shall produce the canonical `Script_Collection_Deficit_Resolved` event. | L3-I-066 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- An unexplained shortfall shall block normal sealing/closure.
- A recovered script shall not be treated as reconciled without custody evidence.
- A confirmed loss/theft outcome shall remain linked to its downstream incident or malpractice case, but those cases are outside this specification.

---

## 7. Audit & Evidence Behavior

Preserve expected/actual counts, candidate references, search record, attendance signature evidence, invigilator statements, recovery or deficit disposition, witness certification, final state, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `SCRIPT_RECONCILED_OR_DEFICIT_SEALED`;
- shortfall has a conclusive governed outcome;
- reconciliation memo/evidence attributable;
- `Script_Collection_Deficit_Resolved` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-066`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains.

Physical search procedure, police interfaces, malpractice case implementation, persistence, event transport, and UI/UX remain deferred.
