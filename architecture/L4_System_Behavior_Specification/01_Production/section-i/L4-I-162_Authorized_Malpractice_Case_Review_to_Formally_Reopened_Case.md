# Authorized Malpractice Case Review to Formally Reopened Case

## 1. Specification Metadata


**Specification ID:** ** `L4-I-162`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-162` — Authorized Malpractice Case Review to Formally Reopened Case  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-162_Authorized_Malpractice_Case_Review_to_Formally_Reopened_Case.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support the governed reopening of a previously closed malpractice case when a formally authorised reopening basis exists.

The behavior begins when the CoE or other parent-authorised appellate authority issues a formal reopening decree against an existing malpractice case in `CASE_CLOSED`.

The behavior ends when the Malpractice Case Dossier Aggregate reaches `CASE_REOPENED_UNDER_REVIEW` and is assigned for fresh review.

This specification does not:

- re-adjudicate the reopened case;
- determine a new penalty;
- conduct police or criminal investigation;
- redefine Student Lifecycle disciplinary ownership;
- define evidence hashing, storage, notification, vault, or workflow implementation.

---

## 3. Required Business Context

The system behavior requires:

- an existing Malpractice Case Dossier in `CASE_CLOSED`;
- a formal reopening decree;
- a parent-supported reopening basis such as new exculpatory evidence, procedural defect, or court directive;
- preserved prior case history and evidence index;
- competent Malpractice Review Committee assignment;
- applicable natural-justice obligations;
- preserved evidence-custody status.

Parent rules governing this specification:

- `BR-I14-001` — Substantive New Evidence Threshold for Reopening;
- `BR-I14-NAT` — Principle of Natural Justice Invariant;
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-162-BH-001` | When a reopening request is initiated, the system shall identify and preserve the existing Malpractice Case Dossier in `CASE_CLOSED`. | L3-I-162 §§2–4, 13 |
| `L4-I-162-BH-002` | The system shall require a formal reopening decree from an authorised authority before reopening proceeds. | L3-I-162 §§4, 6, 15 |
| `L4-I-162-BH-003` | The system shall require the reopening basis to satisfy `BR-I14-001`. | L3-I-162 §12 |
| `L4-I-162-BH-004` | The system shall preserve the complete prior case history and evidence index while reopening the dossier. | L3-I-162 §§2–3, 12 |
| `L4-I-162-BH-005` | The system shall preserve evidence-custody continuity under `BR-I14-SEAL`. | L3-I-162 §12 |
| `L4-I-162-BH-006` | The system shall preserve the candidate notice and fair-hearing obligations required by `BR-I14-NAT`. | L3-I-162 §§10, 12 |
| `L4-I-162-BH-007` | The system shall assign the reopened dossier to the competent Malpractice Review Committee. | L3-I-162 §§3–4 |
| `L4-I-162-BH-008` | The system shall prevent reopening completion while decree authority, reopening threshold, or evidence custody remains unresolved. | L3-I-162 §§11–12 |
| `L4-I-162-BH-009` | On valid reopening, the system shall transition the Malpractice Case Dossier from `CASE_CLOSED` to `CASE_REOPENED_UNDER_REVIEW`. | L3-I-162 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-162-BH-010` | The system shall consume relevant student, governance/legal, incident, and appeal facts from their canonical owners without transferring ownership to I14. | L3-I-162 §§5, 14 |
| `L4-I-162-BH-011` | When the dossier reaches `CASE_REOPENED_UNDER_REVIEW`, the system shall publish `Malpractice_Case_Reopened` on behalf of Malpractice & Examination Committee. | L3-I-162 §§14, 16 |
| `L4-I-162-BH-012` | Where the Locked parent requires `Student_Disciplinary_Hold_Applied`, the system shall preserve that published event without redefining the hold lifecycle or ownership. | L3-I-162 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If the reopening threshold is not satisfied, the dossier shall remain `CASE_CLOSED`.
- If reopening authority cannot be established, reopening shall not proceed.
- If evidence loss or tampering compromises the case, the system shall preserve the compromised condition for authorised resolution rather than treating the case as validly reopened.
- Reopening completion shall not be interpreted as a new malpractice finding or penalty decision.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- reopening decree and issuing authority;
- reopening basis and supporting evidence reference;
- prior case status and adjudication history;
- evidence index and custody continuity;
- notice/hearing-readiness evidence;
- review committee assignment;
- lifecycle transition evidence;
- canonical published events.

No original case history shall be silently overwritten.

---

## 8. Observable Completion Conditions

The specification is complete when:

- the Malpractice Case Dossier is `CASE_REOPENED_UNDER_REVIEW`;
- reopening authority is attributable;
- the reopening threshold is evidenced;
- prior case history and evidence custody remain preserved;
- the review assignment is established;
- `Malpractice_Case_Reopened` has been published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-162`  
**Canonical Aggregate:** Malpractice Case Dossier Aggregate  
**Entry State:** `CASE_CLOSED`  
**Resulting State:** `CASE_REOPENED_UNDER_REVIEW`  
**Canonical Event:** `Malpractice_Case_Reopened`  
**Owning Domain:** Malpractice & Examination Committee (I14)

All behavior statements derive from the Locked parent and its cited L2 source. No new business rule, lifecycle state, event, or ownership assignment is introduced.

---

## 10. Open Issues / Deferred Design

### Upstream Issue

The Locked parent also publishes `Student_Disciplinary_Hold_Applied` even though the scenario outcome is case reopening rather than final penalty adjudication. This L4 specification preserves that event because it is parent-defined but does not infer a new hold lifecycle, hold duration, or additional enforcement rule.

### Deferred to Later Levels

- evidence hashing mechanism;
- evidence storage/vault implementation;
- notification transport;
- persistence model;
- event transport;
- access-control implementation;
- UI/UX.
