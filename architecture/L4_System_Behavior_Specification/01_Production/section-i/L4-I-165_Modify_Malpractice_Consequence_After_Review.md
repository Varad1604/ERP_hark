# Modify Malpractice Consequence After Review

## 1. Specification Metadata


**Specification ID:** ** `L4-I-165`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-165` — Modify Malpractice Consequence After Review  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-165_Modify_Malpractice_Consequence_After_Review.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed modification of an existing malpractice penalty after authorised review establishes that the original consequence is disproportionate or that mitigating circumstances justify revision.

The behavior begins with an existing Malpractice Penalty Order in `PENALTY_AWARDED` and an appellate finding supporting modification.

The behavior ends when the Malpractice Penalty Order Aggregate reaches `PENALTY_MODIFIED_AND_ENFORCED`.

This specification does not create an unrelated new sanction framework or take ownership of downstream Student Lifecycle eligibility.

---

## 3. Required Business Context

The system behavior requires:

- existing Malpractice Penalty Order in `PENALTY_AWARDED`;
- appellate findings supporting modification;
- mitigating circumstances or proportionality basis;
- Appellate Committee Chair;
- Controller of Examinations;
- candidate/appellant;
- preserved evidence custody;
- preserved natural-justice evidence.

Parent rules:

- `BR-I14-004` — Proportionality & Mitigating Circumstances Standard;
- `BR-I14-NAT` — Principle of Natural Justice Invariant;
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-165-BH-001` | The system shall identify the existing Malpractice Penalty Order in `PENALTY_AWARDED` as the consequence under review. | L3-I-165 §§4, 9, 13 |
| `L4-I-165-BH-002` | The system shall require a supported proportionality or mitigating-circumstances basis under `BR-I14-004`. | L3-I-165 §§2, 12 |
| `L4-I-165-BH-003` | The system shall preserve the original penalty and proposed revised penalty as distinct traceable records of consequence. | L3-I-165 §§2–3 |
| `L4-I-165-BH-004` | The system shall preserve appellate findings and evidence supporting the modification. | L3-I-165 §§2–3, 9 |
| `L4-I-165-BH-005` | The system shall preserve natural-justice and evidence-custody obligations during penalty modification. | L3-I-165 §12 |
| `L4-I-165-BH-006` | The system shall revoke the original sanction only as part of its authoritative replacement by the revised sanction and shall not erase the original sanction history. | L3-I-165 §3 |
| `L4-I-165-BH-007` | The system shall preserve the revised eligibility or suspension consequence for downstream Student Lifecycle handling without taking ownership of that lifecycle. | L3-I-165 §§3, 5 |
| `L4-I-165-BH-008` | The system shall prevent modification completion while the proportionality basis, approval authority, or evidence integrity remains unresolved. | L3-I-165 §§11–12 |
| `L4-I-165-BH-009` | On valid modification, the system shall transition the Malpractice Penalty Order Aggregate to `PENALTY_MODIFIED_AND_ENFORCED`. | L3-I-165 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-165-BH-010` | The system shall preserve Student Lifecycle and Result Processing impacts as downstream effects without transferring ownership to I14. | L3-I-165 §§5, 14, 16 |
| `L4-I-165-BH-011` | When the modified penalty is authorised, the system shall publish `Malpractice_Penalty_Modified`. | L3-I-165 §§14, 16 |
| `L4-I-165-BH-012` | Where applicable under the Locked parent, the system shall preserve `Student_Disciplinary_Hold_Applied` for the revised consequence without inventing additional hold states or rules. | L3-I-165 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If mitigating/proportionality grounds are not established, the existing penalty shall remain unchanged.
- If required approval authority is absent, modification shall not complete.
- Evidence compromise or procedural defect shall prevent ordinary modification completion.
- The original penalty history shall remain preserved after replacement.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- original penalty;
- modification basis;
- mitigating circumstances;
- appellate findings;
- evidence index;
- revised penalty;
- approval authority;
- service evidence;
- downstream eligibility consequence;
- lifecycle transition;
- canonical events.

---

## 8. Observable Completion Conditions

The specification is complete when:

- the Malpractice Penalty Order is `PENALTY_MODIFIED_AND_ENFORCED`;
- the original and revised penalties are both traceable;
- the modification basis and approval authority are evidenced;
- the revised downstream consequence is available to authorised consumers;
- `Malpractice_Penalty_Modified` has been published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-165`  
**Canonical Aggregate:** Malpractice Penalty Order Aggregate  
**Entry State:** `PENALTY_AWARDED`  
**Resulting State:** `PENALTY_MODIFIED_AND_ENFORCED`  
**Canonical Event:** `Malpractice_Penalty_Modified`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No new penalty state, rule, or domain ownership is introduced.

---

## 10. Open Issues / Deferred Design

The Locked parent contains generic actor-description mismatches. This specification follows the explicit lifecycle, rule, event, and authorization sections rather than silently altering upstream role definitions.

Deferred:

- eligibility synchronization implementation;
- evidence-storage technology;
- notification transport;
- persistence model;
- event transport;
- technical access control;
- UI/UX.
