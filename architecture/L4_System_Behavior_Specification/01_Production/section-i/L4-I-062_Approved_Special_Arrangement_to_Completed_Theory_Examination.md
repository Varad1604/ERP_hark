# Approved Special Arrangement to Completed Theory Examination

## 1. Specification Metadata


**Specification ID:** ** `L4-I-062`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-062` — Approved Special Arrangement to Completed Theory Examination  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-062_Approved_Special_Arrangement_to_Completed_Theory_Examination.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed theory-examination conduct for a candidate with an approved special arrangement, preserving approved accommodation conditions, scribe verification, compensatory time, and compliant closure.

The behavior begins when the candidate with an approved accommodation arrives with the designated scribe where applicable. It ends when the Special Accommodation Docket Aggregate reaches `ACCOMMODATION_COMPLETED_AND_SEALED`.

---

## 3. Required Business Context

- Approved special arrangement.
- Candidate identity and eligibility.
- Certified Scribe where applicable.
- Special Hall Invigilator.
- Centre Chief Superintendent.
- University Observer.
- Approved compensatory-time and room/accessibility conditions.
- Parent-defined statutory scribe-qualification rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-062-BH-001` | When an approved accommodation is presented for conduct, the system shall establish the Special Accommodation Docket in `ARRANGEMENT_VERIFIED`. | L3-I-062 §§4, 9, 13 |
| `L4-I-062-BH-002` | The system shall preserve the exact approved accommodation terms applicable to the candidate. | L3-I-062 §§2–3 |
| `L4-I-062-BH-003` | Where a scribe is used, the system shall require compliance with BR-I07-008 before treating the arrangement as valid. | L3-I-062 §12 |
| `L4-I-062-BH-004` | The system shall preserve the candidate/scribe relationship and required undertaking or declaration. | L3-I-062 §§2–4 |
| `L4-I-062-BH-005` | The system shall preserve approved compensatory time and the actual accommodated conduct duration. | L3-I-062 §§2–3 |
| `L4-I-062-BH-006` | The system shall preserve the approved accessible room/location condition without assuming ownership of physical facilities. | L3-I-062 §§2, 5 |
| `L4-I-062-BH-007` | The system shall require applicable observer/invigilator attestation before accommodation closure. | L3-I-062 §§6, 15 |
| `L4-I-062-BH-008` | On successful completion, the system shall transition the Special Accommodation Docket to `ACCOMMODATION_COMPLETED_AND_SEALED`. | L3-I-062 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-062-BH-009` | The system shall recognize authoritative candidate and facilities facts without transferring ownership to I07. | L3-I-062 §§5, 14 |
| `L4-I-062-BH-010` | When the accommodated examination is completed, the system shall produce the canonical `Special_Arrangement_Conduct_Completed` event. | L3-I-062 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If the accommodation or scribe qualification cannot be validated, the approved arrangement shall not be silently altered or ignored.
- If a required accommodation cannot be delivered, the condition shall remain unresolved for authorised examination handling.
- This specification does not create new accommodation policy.

---

## 7. Audit & Evidence Behavior

Preserve approved accommodation, candidate/scribe identities, scribe qualification evidence, compensatory-time entitlement and usage, room/accessibility context, attestation, final state, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `ACCOMMODATION_COMPLETED_AND_SEALED`;
- approved accommodation applied as authorized;
- scribe/time/accessibility evidence attributable;
- `Special_Arrangement_Conduct_Completed` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-062`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains.

Accessibility UI, facilities systems, timekeeping technology, identity devices, persistence, and event transport remain deferred.
