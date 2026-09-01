# Duplicate Malpractice Case to Single Authoritative Case

## 1. Specification Metadata


**Specification ID:** ** `L4-I-168`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-168` — Duplicate Malpractice Case to Single Authoritative Case  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-168_Duplicate_Malpractice_Case_to_Single_Authoritative_Case.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed consolidation of multiple malpractice dockets that represent the same underlying incident into one authoritative master case, preserving all evidence and retiring duplicate case identities with explicit cross-reference.

**Start Boundary:** Multiple malpractice dockets are identified as referring to the same candidate, examination paper, and examination session/incident.

**End Boundary:** One authoritative master case exists, duplicate dockets are retired/cross-referenced, and the aggregate reaches `CASE_MERGED_AND_UNIFIED`.

This specification remains implementation-independent and does not define APIs, persistence, software components, transport mechanisms, technical access control, infrastructure, or UI/UX.

---

## 3. Required Business Context

- Canonical aggregate: **Malpractice Case Dossier Aggregate**.
- Entry state: `DUPLICATE_REPORTED`.
- Parent-defined governing rule: `BR-I14-007`.
- `BR-I14-NAT` — Principle of Natural Justice Invariant.
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.
- Malpractice & Examination Committee remains the canonical owner.
- Student Lifecycle, Governance/Legal, Result Processing, and Platform Foundation participate only through parent-defined collaboration.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-168-BH-001` | The system shall identify the set of malpractice dockets suspected to represent the same underlying incident. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-002` | The system shall require sufficient identity correlation across candidate, examination paper, session, incident facts, or other parent-supported evidence before merging. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-003` | The system shall enforce `BR-I14-007` so that one underlying incident is not adjudicated as multiple independent cases. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-004` | The system shall select/preserve one authoritative master case identity without losing the identities of duplicate dockets. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-005` | The system shall preserve every duplicate docket identifier as a cross-reference to the master case. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-006` | The system shall consolidate the evidence references into one unified evidence dossier while preserving source provenance. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-007` | The system shall mark duplicate dockets as merged/retired rather than deleting them. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-008` | The system shall preserve natural-justice and evidence-custody obligations after consolidation. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-009` | On valid consolidation, the system shall transition the Malpractice Case Dossier Aggregate to `CASE_MERGED_AND_UNIFIED`. | L3-I-168 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-168-BH-010` | The system shall consume relevant upstream examination, student, appeal, governance, or incident facts only from their canonical publishers and shall not transfer their ownership to I14. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-011` | When the governed result is reached, the system shall publish `Duplicate_Malpractice_Cases_Merged` on behalf of Malpractice & Examination Committee. | L3-I-168 §§4, 9, 13 |
| `L4-I-168-BH-012` | Where the Locked parent requires `Student_Disciplinary_Hold_Applied`, the system shall preserve that event reference without treating case consolidation itself as a new disciplinary determination. | L3-I-168 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If the dockets cannot be proven to represent the same underlying incident, the system shall keep them distinct.
- If evidence provenance cannot be reconciled, consolidation shall remain unresolved rather than discard conflicting evidence.
- Case merging shall not itself determine guilt or penalty.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- all source docket identifiers;
- master case identifier;
- duplicate-to-master cross-references;
- evidence-source provenance;
- merge authority and rationale;
- final merged state;
- canonical event.

Original case, evidence, decision, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- one master case is authoritative;
- all duplicates remain traceable to the master;
- the aggregate is `CASE_MERGED_AND_UNIFIED`;
- `Duplicate_Malpractice_Cases_Merged` is published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-168`  
**Canonical Aggregate:** Malpractice Case Dossier Aggregate  
**Entry State:** `DUPLICATE_REPORTED`  
**Resulting State:** `CASE_MERGED_AND_UNIFIED`  
**Canonical Event:** `Duplicate_Malpractice_Cases_Merged`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No new business rule, event, lifecycle state, aggregate owner, or authority is introduced by this specification.

---

## 10. Open Issues / Deferred Design

**Upstream issue:** The parent includes generic adjudication/hearing language and `Student_Disciplinary_Hold_Applied`, although this scenario's explicit purpose is duplicate-case consolidation. L4 does not treat merge completion as a finding of guilt or a new penalty decision.

Deferred to later architecture levels: evidence-storage mechanism, cryptographic implementation, persistence model, notification transport, event transport, technical access controls, infrastructure, and UI/UX.
