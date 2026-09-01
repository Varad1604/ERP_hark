# Scheduled Theory Examination to Completed Conduct Record

## 1. Specification Metadata


**Specification ID:** ** `L4-I-055`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-055` — Scheduled Theory Examination to Completed Conduct Record  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-055_Scheduled_Theory_Examination_to_Completed_Conduct_Record.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed conduct of a scheduled theory examination from centre initialization through candidate processing, attendance reconciliation, script collection, observer certification, and sealed conduct closure.

The behavior begins when the examination centre opens for the scheduled session. It ends when the Theory Examination Session Aggregate reaches `CONDUCT_RECORDED_AND_SEALED`.

This specification does not govern post-examination evaluation, marks entry, physical perimeter patrol, scanner hardware, or custody-transport implementation.

---

## 3. Required Business Context

- Scheduled examination session and active centre/hall context.
- Verified Chief Superintendent and University Observer appointments.
- Candidate eligibility and hall-ticket facts.
- Question-paper custody context.
- Hall Invigilator.
- Parent-defined dual-witness seal verification, custody-sealing, and attendance-script reconciliation rules.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-055-BH-001` | When the scheduled theory session begins centre operations, the system shall establish the Theory Examination Session Aggregate in `SESSION_INITIALIZED`. | L3-I-055 §§4, 9, 13 |
| `L4-I-055-BH-002` | The system shall require dual-witness verification of the question-paper custody/seal condition before normal conduct begins. | L3-I-055 §§9, 12 |
| `L4-I-055-BH-003` | The system shall recognize candidate eligibility and hall-ticket status from their owning sources before treating a candidate as admitted for conduct. | L3-I-055 §§5, 14 |
| `L4-I-055-BH-004` | The system shall preserve candidate attendance and absentee outcomes during the examination session. | L3-I-055 §§2–3, 9 |
| `L4-I-055-BH-005` | The system shall require the collected answer-script count plus absentees to reconcile exactly with the registered candidate population under BR-I07-MUST. | L3-I-055 §12 |
| `L4-I-055-BH-006` | The system shall prevent final sealing while an answer-script reconciliation discrepancy remains unresolved. | L3-I-055 §11 |
| `L4-I-055-BH-007` | The system shall require the parent-defined observer certification for final conduct closure. | L3-I-055 §§6, 9, 15 |
| `L4-I-055-BH-008` | On successful conduct closure, the system shall transition the Theory Examination Session Aggregate from `SESSION_INITIALIZED` to `CONDUCT_RECORDED_AND_SEALED`. | L3-I-055 §§9, 13, 16 |
| `L4-I-055-BH-009` | The system shall preserve sealed-script custody readiness as the final handoff condition without defining the physical transport mechanism. | L3-I-055 §§4, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-055-BH-010` | The system shall consume `Examination_Session_Scheduled` and `Hall_Ticket_Verified` only as authoritative upstream facts and shall not transfer ownership of those lifecycles to I07. | L3-I-055 §14.1 |
| `L4-I-055-BH-011` | When attendance is finalized, the system shall produce the canonical `Examination_Attendance_Finalized` business event on behalf of Theory Examination Conduct. | L3-I-055 §14.1 |
| `L4-I-055-BH-012` | When the Theory Examination Session reaches `CONDUCT_RECORDED_AND_SEALED`, the system shall produce the canonical `Theory_Examination_Conducted_and_Sealed` business event. | L3-I-055 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- A custody breach or packet-seal tampering condition shall prevent normal conduct continuation until governed emergency resolution.
- An unreconciled answer-script count shall prevent bundle closure.
- Parent-supported manual attendance fallback may be recognized without defining technical synchronization.
- Minor anomalies may follow the parent-supported joint Superintendent/Observer exception path only when authorized and justified.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- centre/session initialization;
- question-paper seal verification;
- candidate eligibility evidence used;
- attendance/absentee roll;
- answer-script reconciliation;
- discrepancy records;
- Superintendent and Observer attestations;
- final conduct state;
- both canonical published events.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Theory Examination Session is `CONDUCT_RECORDED_AND_SEALED`;
- attendance and script counts reconcile;
- observer certification is attributable;
- sealed-script custody readiness is recorded;
- `Examination_Attendance_Finalized` and `Theory_Examination_Conducted_and_Sealed` have been produced as applicable.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-055`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)  
**Higher-level verification:** Locked L1 architecture and dependency map.

All BHs derive from the Locked parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains.

Barcode/scanner implementation, biometric technology, physical custody transport, persistence, event transport, audit implementation, and UI/UX remain deferred.
