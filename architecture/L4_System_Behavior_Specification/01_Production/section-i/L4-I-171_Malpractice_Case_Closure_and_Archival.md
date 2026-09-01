# Malpractice Case Closure and Archival

## 1. Specification Metadata


**Specification ID:** ** `L4-I-171`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-171` — Malpractice Case Closure and Archival  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-171_Malpractice_Case_Closure_and_Archival.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed terminal closure and archival of a malpractice case after the penalty/restitution lifecycle and appeal obligations are complete, preserving the complete legal and evidentiary record for the statutory retention period.

**Start Boundary:** The penalty term is served or restitution is complete, the appeal cycle is exhausted, and no pending litigation remains.

**End Boundary:** The case reaches `CASE_CLOSED_AND_ARCHIVED` and the complete dossier is retained under the parent-defined 10-year statutory retention rule.

This specification remains implementation-independent and does not define APIs, persistence, software components, transport mechanisms, technical access control, infrastructure, or UI/UX.

---

## 3. Required Business Context

- Canonical aggregate: **Malpractice Case Dossier Aggregate**.
- Entry state: `PENALTY_SERVED`.
- Parent-defined governing rule: `BR-I14-010`.
- `BR-I14-NAT` — Principle of Natural Justice Invariant.
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.
- Malpractice & Examination Committee remains the canonical owner.
- Student Lifecycle, Governance/Legal, Result Processing, and Platform Foundation participate only through parent-defined collaboration.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-171-BH-001` | The system shall require the Malpractice Case Dossier to be eligible for closure, including completion of the applicable penalty/restitution path and exhaustion of pending appeal/litigation conditions. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-002` | The system shall identify the dossier in the parent-defined entry state `PENALTY_SERVED` for the canonical lifecycle transition. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-003` | The system shall preserve the complete case dossier, hearing record, orders, and evidence index before closure. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-004` | The system shall preserve evidence-custody integrity under `BR-I14-SEAL` through the point of archival. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-005` | The system shall preserve the statutory retention obligation under `BR-I14-010` for 10 years. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-006` | The system shall prevent ordinary case mutation after terminal archival except through a separately authorised reopening scenario. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-007` | The system shall preserve any lawful student/result consequences already established by preceding scenarios without redefining them during archival. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-008` | The system shall preserve the archival authority and closure rationale. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-009` | On valid closure, the system shall transition the Malpractice Case Dossier Aggregate to `CASE_CLOSED_AND_ARCHIVED`. | L3-I-171 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-171-BH-010` | The system shall consume relevant upstream examination, student, appeal, governance, or incident facts only from their canonical publishers and shall not transfer their ownership to I14. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-011` | When the governed result is reached, the system shall publish `Malpractice_Case_Archived` on behalf of Malpractice & Examination Committee. | L3-I-171 §§4, 9, 13 |
| `L4-I-171-BH-012` | The system shall not create or renew a disciplinary hold solely because the case is archived; the parent-listed `Student_Disciplinary_Hold_Applied` remains traceable as an upstream semantic conflict unless a preceding authoritative penalty scenario already established it. | L3-I-171 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If an appeal, litigation, restitution obligation, or penalty obligation remains open, the system shall not close the case.
- If the dossier or evidence archive is materially incomplete, closure shall remain unresolved.
- Archival shall not itself impose, renew, or alter a disciplinary penalty.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- complete case dossier;
- hearing transcripts;
- orders and decisions;
- evidence index/custody;
- closure eligibility evidence;
- archival authority;
- retention obligation;
- final lifecycle state;
- canonical event.

Original case, evidence, decision, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the case is `CASE_CLOSED_AND_ARCHIVED`;
- all closure preconditions are satisfied;
- the complete dossier is retained under the 10-year rule;
- no unresolved appeal/litigation remains;
- `Malpractice_Case_Archived` is published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-171`  
**Canonical Aggregate:** Malpractice Case Dossier Aggregate  
**Entry State:** `PENALTY_SERVED`  
**Resulting State:** `CASE_CLOSED_AND_ARCHIVED`  
**Canonical Event:** `Malpractice_Case_Archived`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No new business rule, event, lifecycle state, aggregate owner, or authority is introduced by this specification.

---

## 10. Open Issues / Deferred Design

**Upstream issue:** The Locked parent lists `Student_Disciplinary_Hold_Applied` in this terminal archival scenario, even though closure/archival should not itself create a new disciplinary sanction. L4 preserves the parent event reference only as an upstream inconsistency and does not infer a renewed hold from archival.

Deferred to later architecture levels: evidence-storage mechanism, cryptographic implementation, persistence model, notification transport, event transport, technical access controls, infrastructure, and UI/UX.
