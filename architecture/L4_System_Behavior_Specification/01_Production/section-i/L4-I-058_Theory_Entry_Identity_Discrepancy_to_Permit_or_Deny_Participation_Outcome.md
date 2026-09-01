# Theory Entry Identity Discrepancy to Permit or Deny Participation Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-058`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-058` — Theory Entry Identity Discrepancy to Permit or Deny Participation Outcome  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-058_Theory_Entry_Identity_Discrepancy_to_Permit_or_Deny_Participation_Outcome.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed resolution of a candidate identity discrepancy at theory-examination entry, resulting in either provisional participation under undertaking or formal denial.

The behavior begins when an invigilator detects an identity mismatch. It ends when the Candidate Identity Discrepancy Docket reaches `PROVISIONAL_ENTRY_PERMITTED_OR_DENIED`.

---

## 3. Required Business Context

- Candidate hall-ticket identity.
- Authoritative identity/photo/biometric facts.
- Centre Chief Superintendent.
- University Observer.
- Hall Invigilator.
- Controller of Examinations.
- Parent-defined photo-undertaking and biometric-attestation rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-058-BH-001` | When an identity mismatch is detected, the system shall establish the Candidate Identity Discrepancy Docket in `DISCREPANCY_FLAGGED`. | L3-I-058 §§4, 9, 13 |
| `L4-I-058-BH-002` | The system shall preserve the mismatch basis, including parent-supported photo, biometric, or naming discrepancies. | L3-I-058 §2 |
| `L4-I-058-BH-003` | The system shall evaluate authoritative identity evidence without transferring ownership of that identity data to Theory Examination Conduct. | L3-I-058 §§5, 14 |
| `L4-I-058-BH-004` | The system shall require the parent-defined photo undertaking and biometric attestation before provisional participation is recorded. | L3-I-058 §12 |
| `L4-I-058-BH-005` | The system shall preserve a clear distinction between provisional permission and formal denial. | L3-I-058 §§2–4 |
| `L4-I-058-BH-006` | Where suspected impersonation leads to denial, the system shall preserve that outcome for downstream inquiry without conducting the inquiry itself. | L3-I-058 §§3–4 |
| `L4-I-058-BH-007` | The system shall preserve Superintendent/Observer authority and attestation for the discrepancy resolution. | L3-I-058 §§6, 15 |
| `L4-I-058-BH-008` | On resolution, the system shall transition the docket from `DISCREPANCY_FLAGGED` to `PROVISIONAL_ENTRY_PERMITTED_OR_DENIED`. | L3-I-058 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-058-BH-009` | The system shall recognize hall-ticket and identity facts from their owning domains without ownership transfer. | L3-I-058 §14 |
| `L4-I-058-BH-010` | When the identity discrepancy is resolved, the system shall produce the canonical `Theory_Identity_Discrepancy_Resolved` event. | L3-I-058 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If identity cannot be resolved to the standard required for provisional participation, the system shall not represent the candidate as normally cleared.
- Suspected impersonation shall remain available for downstream governed inquiry.
- Technical biometric failure alone shall not silently determine identity ownership or final authority beyond the parent rules.

---

## 7. Audit & Evidence Behavior

Preserve discrepancy type, source identity evidence, undertaking/attestation, decision, authority, provisional or denied outcome, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `PROVISIONAL_ENTRY_PERMITTED_OR_DENIED`;
- resolution basis attributable;
- permitted versus denied outcome explicit;
- `Theory_Identity_Discrepancy_Resolved` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-058`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

The parent includes generic conduct/sealing language beyond the identity-discrepancy boundary. L4 does not turn this candidate-entry exception into a full examination-session closure scenario.

Biometric devices, image matching, persistence, investigation systems, and UI/UX remain deferred.
