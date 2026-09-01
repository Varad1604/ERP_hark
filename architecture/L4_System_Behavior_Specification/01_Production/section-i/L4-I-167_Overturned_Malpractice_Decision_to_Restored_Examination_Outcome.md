# Overturned Malpractice Decision to Restored Examination Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-167`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-167` — Overturned Malpractice Decision to Restored Examination Outcome  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-167_Overturned_Malpractice_Decision_to_Restored_Examination_Outcome.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support academic restitution after a malpractice finding has been overturned, enabling the affected examination outcome to be restored through governed evaluation or special-examination handling while preserving evaluation neutrality.

**Start Boundary:** A formal exoneration decree is received for a candidate whose examination result is withheld because of malpractice.

**End Boundary:** The Candidate Examination Result Aggregate reaches `RESULT_RESTORED_AND_PUBLISHED` and the restored examination outcome is made authoritative.

This specification remains implementation-independent and does not define APIs, persistence, software components, transport mechanisms, technical access control, infrastructure, or UI/UX.

---

## 3. Required Business Context

- Canonical aggregate: **Candidate Examination Result Aggregate**.
- Entry state: `RESULT_WITHHELD_MALPRACTICE`.
- Parent-defined governing rule: `BR-I14-006`.
- `BR-I14-NAT` — Principle of Natural Justice Invariant.
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.
- Malpractice & Examination Committee remains the canonical owner.
- Student Lifecycle, Governance/Legal, Result Processing, and Platform Foundation participate only through parent-defined collaboration.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-167-BH-001` | The system shall require an authoritative exoneration decree before result restoration begins. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-002` | The system shall identify the Candidate Examination Result Aggregate in `RESULT_WITHHELD_MALPRACTICE`. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-003` | The system shall preserve the confiscated-script or special-examination restitution basis supported by the parent scenario. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-004` | The system shall preserve evaluation neutrality under `BR-I14-006` and shall not allow the previous malpractice allegation to bias academic evaluation. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-005` | The system shall preserve evidence-custody continuity where the original answer script is used for evaluation. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-006` | The system shall preserve the authoritative marks/grade outcome produced by the governed restitution process. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-007` | The system shall make the restored result available for downstream grade/ledger processing without taking ownership of those downstream result functions. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-008` | On valid restitution, the system shall transition the Candidate Examination Result Aggregate to `RESULT_RESTORED_AND_PUBLISHED`. | L3-I-167 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-167-BH-009` | The system shall consume relevant upstream examination, student, appeal, governance, or incident facts only from their canonical publishers and shall not transfer their ownership to I14. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-010` | When the governed result is reached, the system shall publish `Examination_Outcome_Restored` on behalf of Malpractice & Examination Committee. | L3-I-167 §§4, 9, 13 |
| `L4-I-167-BH-011` | The system shall preserve the parent-listed `Student_Disciplinary_Hold_Applied` only as an upstream semantic conflict and shall not infer a new post-restoration disciplinary hold from it. | L3-I-167 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If no authoritative exoneration decree exists, restoration shall not proceed.
- If the original script cannot be validly evaluated and no parent-authorised alternate restitution path is available, the result shall remain unresolved rather than fabricated.
- If evidence custody is compromised, the system shall preserve the exception for authorised resolution.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- exoneration decree;
- withheld result state;
- script/evaluation custody evidence;
- restoration method;
- marks/grade determination;
- result publication evidence;
- canonical published event.

Original case, evidence, decision, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the result is `RESULT_RESTORED_AND_PUBLISHED`;
- the restoration basis is attributable;
- evaluation neutrality is evidenced;
- the restored marks/grade are authoritative;
- `Examination_Outcome_Restored` is published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-167`  
**Canonical Aggregate:** Candidate Examination Result Aggregate  
**Entry State:** `RESULT_WITHHELD_MALPRACTICE`  
**Resulting State:** `RESULT_RESTORED_AND_PUBLISHED`  
**Canonical Event:** `Examination_Outcome_Restored`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No new business rule, event, lifecycle state, aggregate owner, or authority is introduced by this specification.

---

## 10. Open Issues / Deferred Design

**Upstream issue:** The Locked parent also lists `Student_Disciplinary_Hold_Applied` and postconditions about withholding/cancelling marks, which conflict with its explicit restitution purpose and `RESULT_RESTORED_AND_PUBLISHED` outcome. L4 follows the explicit aggregate lifecycle and restoration event and does not silently reimpose a penalized result state.

Deferred to later architecture levels: evidence-storage mechanism, cryptographic implementation, persistence model, notification transport, event transport, technical access controls, infrastructure, and UI/UX.
