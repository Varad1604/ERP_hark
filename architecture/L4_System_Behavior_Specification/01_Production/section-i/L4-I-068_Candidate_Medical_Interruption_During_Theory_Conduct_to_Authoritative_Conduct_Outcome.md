# Candidate Medical Interruption During Theory Conduct to Authoritative Conduct Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-068`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-068` — Candidate Medical Interruption During Theory Conduct to Authoritative Conduct Outcome  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-068_Candidate_Medical_Interruption_During_Theory_Conduct_to_Authoritative_Conduct_Outcome.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support the Examination-domain response to a candidate medical interruption during live theory conduct, preserving examination custody, exit timing, partial-script disposition, and handoff to medical authority.

The behavior begins when a candidate becomes medically unable to continue. It ends when the Candidate Medical Interruption Case Aggregate reaches `PARTIAL_SCRIPT_SEALED_AND_DOCUMENTED`.

Medical diagnosis, treatment, and clinical records remain owned by G06 and are outside this specification.

---

## 3. Required Business Context

- Active theory examination.
- Candidate and attempted answer script.
- Chief Superintendent.
- University Observer.
- Campus Medical Officer (G06).
- Controller of Examinations.
- Parent-defined partial-script time-stamped impounding rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-068-BH-001` | When a candidate becomes medically unable to continue, the system shall establish the Candidate Medical Interruption Case in `MEDICAL_INTERRUPTED`. | L3-I-068 §§4, 9, 13 |
| `L4-I-068-BH-002` | The system shall preserve the candidate's exact examination-exit/interruption time. | L3-I-068 §§2–3, 12 |
| `L4-I-068-BH-003` | The system shall preserve the partially attempted answer script as an examination artifact under BR-I07-014. | L3-I-068 §12 |
| `L4-I-068-BH-004` | The system shall preserve Chief Superintendent/Observer examination attestation for the interrupted attempt. | L3-I-068 §§6, 15 |
| `L4-I-068-BH-005` | The system shall recognize G06 medical handover/treatment status without representing I07 as the owner of medical decisions. | L3-I-068 §§2–6 |
| `L4-I-068-BH-006` | The system shall preserve the institutional examination exemption/medical-interruption docket required by the parent outcome without defining its downstream adjudication. | L3-I-068 §3 |
| `L4-I-068-BH-007` | The system shall prevent the interrupted script from being treated as an ordinary uninterrupted completion record. | L3-I-068 §§2–4 |
| `L4-I-068-BH-008` | On governed closure, the system shall transition the Candidate Medical Interruption Case to `PARTIAL_SCRIPT_SEALED_AND_DOCUMENTED`. | L3-I-068 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-068-BH-009` | The system shall preserve the handoff to G06 while keeping diagnosis, treatment, and clinical facts canonically owned by G06. | L3-I-068 §§5–6 |
| `L4-I-068-BH-010` | When the examination interruption is resolved, the system shall produce the canonical `Medical_Interruption_Conduct_Resolved` event. | L3-I-068 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If the candidate's exit time or partial-script custody cannot be established, examination closure shall remain unresolved.
- Medical treatment shall continue under G06 regardless of whether the examination docket is complete.
- Downstream exemption approval or re-examination eligibility is outside this specification unless separately governed.

---

## 7. Audit & Evidence Behavior

Preserve candidate/session identity, interruption time, partial-script custody, Superintendent/Observer attestations, G06 handoff reference, medical-interruption/exemption docket reference, final state, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `PARTIAL_SCRIPT_SEALED_AND_DOCUMENTED`;
- exact interruption/exit time attributable;
- partial script securely accounted for;
- G06 handoff recognized;
- `Medical_Interruption_Conduct_Resolved` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-068`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)  
**Cross-domain participant:** G06 — Institutional Health Services

---

## 10. Open Issues / Deferred Design

The parent uses generic examination role language for the Campus Medical Officer; L4 preserves G06 medical ownership and does not reinterpret the medical officer as an Examination-domain authority.

Clinical systems, ambulance/medical workflows, persistence, event transport, and UI/UX remain deferred.
