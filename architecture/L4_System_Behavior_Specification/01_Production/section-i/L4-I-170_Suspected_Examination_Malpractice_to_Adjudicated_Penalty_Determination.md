# Suspected Examination Malpractice to Adjudicated Penalty Determination

## 1. Specification Metadata


**Specification ID:** ** `L4-I-170`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-170` — Suspected Examination Malpractice to Adjudicated Penalty Determination  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-170_Suspected_Examination_Malpractice_to_Adjudicated_Penalty_Determination.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support end-to-end statutory examination-malpractice adjudication from a booked incident through notice, hearing, evidence evaluation, committee determination, CoE approval, and authoritative penalty outcome.

**Start Boundary:** A sealed malpractice incident report and confiscated material are submitted from examination conduct for statutory adjudication.

**End Boundary:** The case reaches `PENALTY_ADJUDICATED_AND_ENFORCED`, the penalty order is served, and the canonical adjudication event is published.

This specification remains implementation-independent and does not define APIs, persistence, software components, transport mechanisms, technical access control, infrastructure, or UI/UX.

---

## 3. Required Business Context

- Canonical aggregate: **Malpractice Case Dossier Aggregate**.
- Entry state: `MALPRACTICE_BOOKED`.
- Parent-defined governing rule: `BR-I14-009`.
- `BR-I14-NAT` — Principle of Natural Justice Invariant.
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.
- Malpractice & Examination Committee remains the canonical owner.
- Student Lifecycle, Governance/Legal, Result Processing, and Platform Foundation participate only through parent-defined collaboration.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-170-BH-001` | The system shall establish the Malpractice Case Dossier in `MALPRACTICE_BOOKED` from a valid malpractice incident report. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-002` | The system shall preserve the incident report, evidence index, and seized-evidence custody references. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-003` | The system shall require written show-cause notice and fair-hearing opportunity under `BR-I14-009` and `BR-I14-NAT`. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-004` | The system shall preserve statutory committee quorum and separation-of-duty constraints. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-005` | The system shall preserve the candidate defense, witness testimony, hearing minutes, and reviewed evidence. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-006` | The system shall preserve violation categorization and penalty recommendation according to the parent-defined Malpractice Ordinance categories. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-007` | The system shall require CoE approval before the penalty becomes authoritative/enforced. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-008` | The system shall preserve the signed penalty order and service to the candidate. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-009` | On valid adjudication, the system shall transition the Malpractice Case Dossier Aggregate to `PENALTY_ADJUDICATED_AND_ENFORCED`. | L3-I-170 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-170-BH-010` | The system shall consume relevant upstream examination, student, appeal, governance, or incident facts only from their canonical publishers and shall not transfer their ownership to I14. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-011` | When the governed result is reached, the system shall publish `Malpractice_Penalty_Adjudicated` on behalf of Malpractice & Examination Committee. | L3-I-170 §§4, 9, 13 |
| `L4-I-170-BH-012` | Where the adjudicated penalty requires the parent-defined disciplinary hold, the system shall publish/preserve `Student_Disciplinary_Hold_Applied` for authorised downstream consumers. | L3-I-170 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If show-cause notice or fair-hearing opportunity is absent, the system shall not complete penalty adjudication.
- If quorum is invalid, adjudication shall remain unresolved.
- If evidence is lost or materially tampered with, the system shall preserve the compromised-case status for authorised resolution.
- Criminal/police proceedings remain outside this scenario.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- incident report;
- evidence index and custody;
- show-cause notice/service;
- committee quorum;
- candidate defense and hearing minutes;
- violation category;
- penalty recommendation;
- CoE approval;
- penalty order/service;
- canonical events.

Original case, evidence, decision, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the case is `PENALTY_ADJUDICATED_AND_ENFORCED`;
- natural-justice obligations are evidenced;
- penalty category and authority are attributable;
- formal penalty order is served;
- `Malpractice_Penalty_Adjudicated` is published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-170`  
**Canonical Aggregate:** Malpractice Case Dossier Aggregate  
**Entry State:** `MALPRACTICE_BOOKED`  
**Resulting State:** `PENALTY_ADJUDICATED_AND_ENFORCED`  
**Canonical Event:** `Malpractice_Penalty_Adjudicated`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No new business rule, event, lifecycle state, aggregate owner, or authority is introduced by this specification.

---

## 10. Open Issues / Deferred Design

The parent contains some generic/misaligned actor descriptions. L4 relies on the explicit adjudication flow, lifecycle, business rules, and approval sections and does not silently rewrite upstream role definitions.

Deferred to later architecture levels: evidence-storage mechanism, cryptographic implementation, persistence model, notification transport, event transport, technical access controls, infrastructure, and UI/UX.
