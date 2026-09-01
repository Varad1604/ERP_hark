# Authorized Theory Re Conduct Decision to Completed Re Examination

## 1. Specification Metadata


**Specification ID:** ** `L4-I-061`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-061` — Authorized Theory Re Conduct Decision to Completed Re Examination  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-061_Authorized_Theory_Re_Conduct_Decision_to_Completed_Re_Examination.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed re-conduct of an invalidated theory examination from valid re-examination authorization through fresh-paper use, candidate conduct, attendance reconciliation, and completed re-examination record.

The behavior begins when the CoE and Academic Council issue a valid Re-Examination Authorization Order. It ends when the Re-Examination Session Aggregate reaches `RE_EXAMINATION_COMPLETED`.

---

## 3. Required Business Context

- Valid invalidation/re-conduct decree.
- Controller of Examinations.
- Academic Council.
- Chief Superintendent and University Observer.
- Fresh question-paper version.
- Candidate eligibility and attendance context.
- Parent-defined fresh-question-paper/versioning rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-061-BH-001` | When a valid re-conduct order is issued, the system shall establish the Re-Examination Session Aggregate in `RECONDUCT_AUTHORIZED`. | L3-I-061 §§4, 9, 13 |
| `L4-I-061-BH-002` | The system shall preserve linkage between the re-examination and the authoritative invalidation/re-conduct decree. | L3-I-061 §§2–3 |
| `L4-I-061-BH-003` | The system shall require a fresh question-paper version for the re-examination under BR-I07-007. | L3-I-061 §12 |
| `L4-I-061-BH-004` | The system shall preserve candidate eligibility and attendance outcomes for the re-conduct without re-owning upstream eligibility facts. | L3-I-061 §§5, 14 |
| `L4-I-061-BH-005` | The system shall require the same governed custody, observer, and reconciliation obligations applicable to theory conduct where the parent invokes them. | L3-I-061 §§9, 12, 15 |
| `L4-I-061-BH-006` | The system shall prevent final completion while attendance/script reconciliation remains unresolved. | L3-I-061 §11 |
| `L4-I-061-BH-007` | The system shall preserve the completed re-examination conduct record separately from the invalidated original sitting. | L3-I-061 §§2–4 |
| `L4-I-061-BH-008` | On successful re-conduct, the system shall transition the Re-Examination Session Aggregate to `RE_EXAMINATION_COMPLETED`. | L3-I-061 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-061-BH-009` | The system shall preserve the upstream authorization and candidate/hall-ticket facts without transferring their ownership to I07. | L3-I-061 §14 |
| `L4-I-061-BH-010` | When the re-examination is completed, the system shall produce the canonical `Theory_Re_Conduct_Completed` business event. | L3-I-061 §§14, 16 |
| `L4-I-061-BH-011` | Where attendance is finalized, the system shall preserve the canonical `Examination_Attendance_Finalized` outcome. | L3-I-061 §14.1 |

---

## 6. Failure / Rejection / Continuation

- Without valid re-conduct authorization, the new examination shall not be represented as an authorised re-examination.
- If fresh-paper requirements fail, conduct shall not proceed as valid re-conduct.
- Any new custody breach or reconciliation failure shall prevent completion.

---

## 7. Audit & Evidence Behavior

Preserve re-conduct order, invalidation reference, fresh-paper identity/version, candidate attendance, custody/observer evidence, reconciliation, completion state, and canonical events.

---

## 8. Observable Completion Conditions

Completion requires:

- `RE_EXAMINATION_COMPLETED`;
- re-conduct linked to its authorization;
- fresh question-paper requirement satisfied;
- attendance/custody reconciliation attributable;
- `Theory_Re_Conduct_Completed` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-061`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains.

Scheduling implementation, question-paper generation technology, persistence, event transport, evaluation systems, and UI/UX remain deferred.
