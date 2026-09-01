# Overturn Malpractice Finding After Review

## 1. Specification Metadata


**Specification ID:** ** `L4-I-166`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-166` — Overturn Malpractice Finding After Review  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-166_Overturn_Malpractice_Finding_After_Review.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support authoritative exoneration of a candidate when appellate or court review conclusively disproves the malpractice finding. The behavior shall overturn the finding, preserve the adjudication history, and establish the exonerated case state without inventing a replacement penalty.

**Start Boundary:** An appellate committee or court ruling conclusively disproves the malpractice allegation.

**End Boundary:** The case reaches `EXONERATED_CHARGES_DROPPED` and the exoneration outcome is formally recorded and published.

This specification remains implementation-independent and does not define APIs, persistence, software components, transport mechanisms, technical access control, infrastructure, or UI/UX.

---

## 3. Required Business Context

- Canonical aggregate: **Malpractice Case Dossier Aggregate**.
- Entry state: `CASE_REOPENED_UNDER_REVIEW`.
- Parent-defined governing rule: `BR-I14-005`.
- `BR-I14-NAT` — Principle of Natural Justice Invariant.
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.
- Malpractice & Examination Committee remains the canonical owner.
- Student Lifecycle, Governance/Legal, Result Processing, and Platform Foundation participate only through parent-defined collaboration.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-166-BH-001` | The system shall require the Malpractice Case Dossier to be in `CASE_REOPENED_UNDER_REVIEW` before an overturn outcome is finalized. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-002` | The system shall require conclusive innocence, false allegation, or procedural invalidity evidence sufficient to satisfy `BR-I14-005`. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-003` | The system shall preserve the complete prior allegation, hearing, evidence, and adjudication history. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-004` | The system shall preserve natural-justice evidence and evidence-custody continuity throughout the overturn decision. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-005` | The system shall record the authoritative exoneration decree and the authority issuing it. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-006` | The system shall prevent any continuing malpractice penalty from being represented as active once the exoneration outcome becomes authoritative. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-007` | The system shall preserve the parent-defined requirement for disciplinary-record expungement as a governed downstream effect without taking ownership of Student Lifecycle records. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-008` | On valid overturn, the system shall transition the Malpractice Case Dossier to `EXONERATED_CHARGES_DROPPED`. | L3-I-166 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-166-BH-009` | The system shall consume relevant upstream examination, student, appeal, governance, or incident facts only from their canonical publishers and shall not transfer their ownership to I14. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-010` | When the governed result is reached, the system shall publish `Malpractice_Finding_Overturned` on behalf of Malpractice & Examination Committee. | L3-I-166 §§4, 9, 13 |
| `L4-I-166-BH-011` | The system shall not reinterpret the parent-listed `Student_Disciplinary_Hold_Applied` as an active post-exoneration hold; any such parent event reference shall remain traceable as an upstream semantic conflict. | L3-I-166 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If conclusive exonerating authority is not established, the system shall not produce an overturned finding.
- If evidence integrity is materially compromised, the system shall preserve the compromised condition for authorised resolution rather than infer innocence or guilt.
- If the ruling only modifies or upholds the penalty, this specification shall not be used.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- exoneration decree and issuing authority;
- prior case and penalty history;
- evidence index and custody continuity;
- hearing/appellate record;
- record-expungement instruction or evidence;
- final case state;
- canonical published event.

Original case, evidence, decision, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the case is `EXONERATED_CHARGES_DROPPED`;
- the exoneration authority and basis are attributable;
- prior disciplinary history remains traceable while current penal effect is removed;
- `Malpractice_Finding_Overturned` is published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-166`  
**Canonical Aggregate:** Malpractice Case Dossier Aggregate  
**Entry State:** `CASE_REOPENED_UNDER_REVIEW`  
**Resulting State:** `EXONERATED_CHARGES_DROPPED`  
**Canonical Event:** `Malpractice_Finding_Overturned`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No new business rule, event, lifecycle state, aggregate owner, or authority is introduced by this specification.

---

## 10. Open Issues / Deferred Design

**Upstream issue:** The Locked parent also lists `Student_Disciplinary_Hold_Applied` even though the scenario fully exonerates the candidate and states disciplinary flags are expunged. This specification preserves the parent event definition only as an upstream inconsistency and does not apply or prolong a disciplinary hold in contradiction to the explicit exoneration outcome.

Deferred to later architecture levels: evidence-storage mechanism, cryptographic implementation, persistence model, notification transport, event transport, technical access controls, infrastructure, and UI/UX.
