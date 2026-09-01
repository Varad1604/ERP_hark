# Uphold Malpractice Decision After Review

## 1. Specification Metadata


**Specification ID:** ** `L4-I-164`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-164` — Uphold Malpractice Decision After Review  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-164_Uphold_Malpractice_Decision_After_Review.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support final appellate dismissal of a malpractice appeal when the authorised appellate review confirms the original penalty.

The behavior begins with a Malpractice Appeal Record in `APPEAL_HEARING_CONCLUDED` and an appellate finding that the original decision is lawful and proportionate.

The behavior ends when the Malpractice Appeal Record reaches `APPEAL_DISMISSED_PENALTY_UPHELD`.

This specification does not modify the penalty, create a new penalty category, or redefine Student Lifecycle ownership of the downstream student record.

---

## 3. Required Business Context

The system behavior requires:

- Malpractice Appeal Record in `APPEAL_HEARING_CONCLUDED`;
- original malpractice penalty;
- completed appellate findings;
- Appellate Committee Chair;
- Controller of Examinations;
- candidate/appellant;
- preserved natural-justice evidence;
- preserved evidence custody.

Parent rules:

- `BR-I14-003` — Finality of Appellate Dismissal Decree;
- `BR-I14-NAT` — Principle of Natural Justice Invariant;
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-164-BH-001` | The system shall require the Malpractice Appeal Record to be in `APPEAL_HEARING_CONCLUDED` before an uphold decision is finalized. | L3-I-164 §§4, 9, 13 |
| `L4-I-164-BH-002` | The system shall require an appellate finding that the original penalty is lawful and proportionate. | L3-I-164 §§2–3 |
| `L4-I-164-BH-003` | On an uphold outcome, the system shall preserve the existing penalty enforcement status and confirm the active disciplinary hold without creating a redundant penalty record. | L3-I-164 §§3, 9, 13 |
| `L4-I-164-BH-004` | The system shall preserve the appellate findings and evidence basis supporting the decision. | L3-I-164 §§2–3, 9 |
| `L4-I-164-BH-004` | The system shall preserve natural-justice and appellate-independence evidence. | L3-I-164 §§12, 15.2 |
| `L4-I-164-BH-005` | The system shall preserve the original penalty as the authoritative consequence after dismissal. | L3-I-164 §§2–3 |
| `L4-I-164-BH-006` | The system shall preserve formal service of the final dismissal decree to the candidate. | L3-I-164 §§3–4 |
| `L4-I-164-BH-007` | The system shall prevent final dismissal where material procedural defect or evidence compromise invalidates the appellate basis. | L3-I-164 §11 |
| `L4-I-164-BH-008` | On valid dismissal, the system shall transition the Malpractice Appeal Record to `APPEAL_DISMISSED_PENALTY_UPHELD`. | L3-I-164 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-164-BH-009` | The system shall preserve downstream Student Lifecycle and Result Processing consequences as cross-domain effects without assuming ownership of those domains. | L3-I-164 §§5, 16 |
| `L4-I-164-BH-010` | When the upheld decision becomes final, the system shall publish `Malpractice_Penalty_Upheld`. | L3-I-164 §§14, 16 |
| `L4-I-164-BH-011` | Where applicable under the Locked parent, the system shall publish/preserve `Student_Disciplinary_Hold_Applied`. | L3-I-164 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If the appellate committee does not confirm the original penalty, this specification shall not produce an upheld outcome.
- Material procedural defect or compromised evidence shall block final dismissal until governed resolution.
- Penalty modification belongs to a different scenario and shall not be performed here.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- original penalty;
- appellate findings;
- evidence basis;
- appellate bench composition;
- natural-justice evidence;
- dismissal decree;
- service evidence;
- lifecycle transition;
- published penalty and hold events.

---

## 8. Observable Completion Conditions

The specification is complete when:

- the appeal record is `APPEAL_DISMISSED_PENALTY_UPHELD`;
- the original penalty remains authoritative;
- final dismissal is evidenced and served;
- `Malpractice_Penalty_Upheld` has been published;
- any parent-defined disciplinary hold consequence is preserved.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-164`  
**Canonical Aggregate:** Malpractice Appeal Record Aggregate  
**Entry State:** `APPEAL_HEARING_CONCLUDED`  
**Resulting State:** `APPEAL_DISMISSED_PENALTY_UPHELD`  
**Canonical Event:** `Malpractice_Penalty_Upheld`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No modification, replacement, or new penalty is introduced.

---

## 10. Open Issues / Deferred Design

The Locked parent contains generic actor-description mismatches. This specification follows the explicit lifecycle, rule, event, and authorization sections rather than silently redefining those actors.

Deferred:

- student-record synchronization implementation;
- notification transport;
- persistence model;
- event transport;
- evidence storage technology;
- UI/UX.
