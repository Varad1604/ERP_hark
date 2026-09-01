# Theory Seating Conflict to Valid Seating Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-059`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-059` — Theory Seating Conflict to Valid Seating Outcome  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-059_Theory_Seating_Conflict_to_Valid_Seating_Outcome.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed resolution of a theory-examination seating conflict by assigning valid buffer seating and updating the authoritative seating/attendance context.

The behavior begins when a seating conflict is reported or detected. It ends when the Hall Seating Allocation Aggregate reaches `SEATING_REALLOCATED_AND_VERIFIED`.

This specification does not redesign examination halls, control physical furniture, or govern the full examination session lifecycle.

---

## 3. Required Business Context

- Candidate and scheduled seating assignment.
- Hall seating plan.
- Hall Invigilator.
- Centre Chief Superintendent.
- Estate Support participation where physical seating remediation is required.
- University Observer.
- Parent-defined 10% buffer-desk capacity rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-059-BH-001` | When a seating conflict is detected, the system shall establish the Hall Seating Allocation Aggregate in `CONFLICT_DETECTED`. | L3-I-059 §§4, 9, 13 |
| `L4-I-059-BH-002` | The system shall preserve the conflict reason, including duplicate allocation, inaccessible seating, or damaged furniture where applicable. | L3-I-059 §2 |
| `L4-I-059-BH-003` | The system shall preserve the candidate's valid examination eligibility while resolving only the seating conflict. | L3-I-059 §§2–4 |
| `L4-I-059-BH-004` | The system shall enforce the parent-defined 10% buffer-desk capacity invariant at the business-rule level. | L3-I-059 §12 |
| `L4-I-059-BH-005` | The system shall assign a valid replacement seat from the governed buffer arrangement without creating duplicate concurrent seat allocation. | L3-I-059 §§2–3 |
| `L4-I-059-BH-006` | The system shall update the seating and attendance context so that the candidate's actual examination position is attributable. | L3-I-059 §3 |
| `L4-I-059-BH-007` | The system shall preserve the required Invigilator/Superintendent verification of the reallocation. | L3-I-059 §§6, 15 |
| `L4-I-059-BH-008` | On successful resolution, the system shall transition the Hall Seating Allocation Aggregate from `CONFLICT_DETECTED` to `SEATING_REALLOCATED_AND_VERIFIED`. | L3-I-059 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-059-BH-009` | The system shall recognize physical hall-readiness facts from Campus Operations without transferring ownership of facilities to Theory Examination Conduct. | L3-I-059 §5 |
| `L4-I-059-BH-010` | When the seating conflict is resolved, the system shall produce the canonical `Theory_Seating_Conflict_Resolved` event. | L3-I-059 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If no valid governed seating can be established, the system shall not represent the conflict as resolved.
- Physical furniture remediation remains a Campus Operations responsibility where applicable.
- The candidate shall not be silently marked absent merely because a seating conflict exists.

---

## 7. Audit & Evidence Behavior

Preserve original seat, conflict reason, replacement seat, buffer-capacity basis, verification authority, attendance adjustment, final state, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `SEATING_REALLOCATED_AND_VERIFIED`;
- valid replacement seating attributable;
- seating plan and attendance context aligned;
- `Theory_Seating_Conflict_Resolved` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-059`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

No new L4 authority is introduced from the parent’s unusual actor-label wording; L4 preserves only the unambiguous seating-resolution obligations.

Seat-map UI, accessibility tooling, facilities systems, persistence, event transport, and UI/UX remain deferred.
