# Candidate Withdrawal During Theory Conduct to Recorded Participation Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-069`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-069` — Candidate Withdrawal During Theory Conduct to Recorded Participation Outcome  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-069_Candidate_Withdrawal_During_Theory_Conduct_to_Recorded_Participation_Outcome.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed recording of a candidate's voluntary withdrawal from an ongoing theory examination after the permissible minimum in-hall duration.

The behavior begins when the candidate requests early submission and withdrawal. It ends when the Candidate Participation Record Aggregate reaches `PARTICIPATION_WITHDRAWN_AND_CLOSED`.

---

## 3. Required Business Context

- Candidate participating in an active theory examination.
- Examination start and elapsed-time context.
- Hall Invigilator.
- Chief Superintendent.
- University Observer.
- Candidate's answer script/question-paper custody context.
- Parent-defined minimum permissible in-hall duration rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-069-BH-001` | When a candidate requests voluntary withdrawal, the system shall establish the Candidate Participation Record in `WITHDRAWAL_REQUESTED`. | L3-I-069 §§4, 9, 13 |
| `L4-I-069-BH-002` | The system shall evaluate the withdrawal request against the parent-defined minimum permissible in-hall duration under BR-I07-015. | L3-I-069 §§2, 12 |
| `L4-I-069-BH-003` | The system shall prevent normal voluntary withdrawal before the minimum permitted time unless an upstream governed exception explicitly allows it. | L3-I-069 §12 |
| `L4-I-069-BH-004` | The system shall preserve the candidate's withdrawal undertaking/acknowledgment. | L3-I-069 §3 |
| `L4-I-069-BH-005` | The system shall preserve collection and custody of the candidate's submitted script and any parent-required examination materials. | L3-I-069 §§2–3 |
| `L4-I-069-BH-006` | The system shall mark the participation outcome as withdrawn rather than absent or normally completed. | L3-I-069 §3 |
| `L4-I-069-BH-007` | The system shall preserve candidate departure timing and the governed handoff for exit without redefining Campus Operations security ownership. | L3-I-069 §§3–5 |
| `L4-I-069-BH-008` | On successful withdrawal closure, the system shall transition the Candidate Participation Record to `PARTICIPATION_WITHDRAWN_AND_CLOSED`. | L3-I-069 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-069-BH-009` | The system shall preserve any exit/security handoff as a cross-domain fact without transferring Campus Operations ownership to I07. | L3-I-069 §5 |
| `L4-I-069-BH-010` | When withdrawal is recorded, the system shall produce the canonical `Candidate_Withdrawal_Recorded` business event. | L3-I-069 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- A withdrawal request made before the permissible minimum time shall not be treated as an authorized normal withdrawal.
- If script custody or withdrawal acknowledgment is unresolved, the participation record shall not be closed.
- Medical or emergency departures are governed by their own scenarios and shall not be silently reclassified as voluntary withdrawal.

---

## 7. Audit & Evidence Behavior

Preserve request time, elapsed exam time, eligibility under minimum-duration rule, withdrawal undertaking, script custody, participation status, departure time, final state, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `PARTICIPATION_WITHDRAWN_AND_CLOSED`;
- minimum-duration rule satisfied or governed exception recorded;
- script and withdrawal undertaking accounted for;
- participation status recorded as withdrawn;
- `Candidate_Withdrawal_Recorded` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-069`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

The parent actor table contains generic role-description mismatches. L4 preserves only the supported withdrawal, custody, attestation, and lifecycle obligations.

Exit-control technology, signature implementation, persistence, event transport, and UI/UX remain deferred.
