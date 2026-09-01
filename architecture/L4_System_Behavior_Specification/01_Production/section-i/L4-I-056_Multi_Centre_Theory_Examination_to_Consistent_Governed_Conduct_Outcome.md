# Multi Centre Theory Examination to Consistent Governed Conduct Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-056`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-056` — Multi Centre Theory Examination to Consistent Governed Conduct Outcome  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-056_Multi_Centre_Theory_Examination_to_Consistent_Governed_Conduct_Outcome.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support synchronized and governed theory-examination conduct across multiple centres, culminating in reconciled centre outcomes and a unified master conduct result.

The behavior begins when Central Examination Command initiates synchronized unsealing/start coordination. It ends when the Multi-Centre Exam Master Aggregate reaches `MULTI_CENTRE_CONDUCT_CONCLUDED`.

---

## 3. Required Business Context

- Scheduled multi-centre examination.
- Central Examination Controller.
- Centre Chief Superintendents.
- Flying Squad Coordinators.
- Controller of Examinations.
- Centre attendance, custody, and conduct records.
- Parent-defined synchronized-start, custody, and attendance-script reconciliation rules.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-056-BH-001` | The system shall establish the Multi-Centre Exam Master Aggregate in `MULTI_CENTRE_SYNCHRONIZING` when coordinated centre conduct begins. | L3-I-056 §§4, 9, 13 |
| `L4-I-056-BH-002` | The system shall preserve a common governed start condition across participating centres under BR-I07-002. | L3-I-056 §§2–3, 12 |
| `L4-I-056-BH-003` | The system shall require each participating centre to satisfy the parent-defined custody/seal verification obligations. | L3-I-056 §§9, 12 |
| `L4-I-056-BH-004` | The system shall preserve centre-level attendance and absentee outcomes for central reconciliation. | L3-I-056 §§2–3 |
| `L4-I-056-BH-005` | The system shall require each centre's script/attendance reconciliation before its conduct result is accepted into the multi-centre master. | L3-I-056 §§9, 12 |
| `L4-I-056-BH-006` | The system shall prevent unified conduct closure while a constituent centre remains unreconciled or unresolved. | L3-I-056 §§3, 11 |
| `L4-I-056-BH-007` | The system shall preserve centre-level attestations and the resulting consolidated master tally. | L3-I-056 §§3, 9 |
| `L4-I-056-BH-008` | On successful consolidation, the system shall transition the Multi-Centre Exam Master Aggregate to `MULTI_CENTRE_CONDUCT_CONCLUDED`. | L3-I-056 §§9, 13, 16 |
| `L4-I-056-BH-009` | The system shall preserve readiness of constituent sealed scripts for downstream custody without defining transport implementation. | L3-I-056 §16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-056-BH-010` | The system shall recognize upstream scheduling and hall-ticket facts without assuming ownership of them. | L3-I-056 §14 |
| `L4-I-056-BH-011` | The system shall produce `Examination_Attendance_Finalized` for finalized multi-centre attendance outcomes. | L3-I-056 §14.1 |
| `L4-I-056-BH-012` | When consolidation completes, the system shall produce the canonical `Multi_Centre_Conduct_Reconciled` business event. | L3-I-056 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- A centre with a custody breach or unresolved script discrepancy shall not be silently treated as reconciled.
- Central conclusion shall remain blocked until constituent centre outcomes are sufficiently resolved under the parent rules.
- Manual centre evidence may be recognized only as a business fallback, without prescribing technical synchronization.

---

## 7. Audit & Evidence Behavior

The system shall preserve centre-by-centre start, attendance, custody, reconciliation, attestations, exceptions, consolidated tally, final state, and published events.

---

## 8. Observable Completion Conditions

Completion requires:

- `MULTI_CENTRE_CONDUCT_CONCLUDED`;
- all accepted centre records reconciled;
- consolidated attendance and conduct outcome attributable;
- `Multi_Centre_Conduct_Reconciled` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-056`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

All BHs derive from the Locked parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains.

Cross-centre synchronization technology, messaging, scanners, persistence, dashboards, and UI/UX remain deferred.
