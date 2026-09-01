# Pre Resolution Exception Reopening to Updated Authoritative Facts

## 1. Specification Metadata


**Specification ID:** ** `L4-I-182`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Exception & Contingency (I15)  
**Realizes Scenario:** ** `L3-I-182` — Pre Resolution Exception Reopening to Updated Authoritative Facts  
**Primary Business Owner:** Examination Exception & Contingency (I15)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-I-182`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed reopening of a not-yet-finalized examination exception when admissible new evidence materially changes the authoritative factual basis, while preserving the prior factual record and decision history.

**Start Boundary:** New forensic or physical evidence is submitted to the CoE before final exception-case closure.

**End Boundary:** The case reaches `FACTS_REVISED_AND_AUTHENTICATED` with updated authoritative facts and a revised remediation recommendation.

This specification remains implementation-independent and does not prescribe APIs, schemas, software components, persistence, message brokers, cryptographic mechanisms, infrastructure, technical IAM, hardware, or UI/UX.

---

## 3. Required Business Context

- Canonical owner: Examination Exception & Contingency (I15).
- Canonical aggregate: **Examination Exception Case Aggregate**.
- Entry state: `IMPACT_ASSESSED_AND_CLASSIFIED`.
- Resulting state: `FACTS_REVISED_AND_AUTHENTICATED`.
- Primary parent rule: `BR-I15-011`.
- `BR-I-AUTH` requires independent executive approval where the Locked parent applies it.
- `BR-I-AUDIT` requires durable, attributable audit evidence without prescribing implementation technology.
- Parent-defined external facts and records remain owned by their canonical source domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-182-BH-001` | When New evidence is submitted before final closure, the system shall verify that the case is still eligible for pre-resolution reopening. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-002` | When New evidence is proposed for use, the system shall apply `BR-I15-011` and preserve the admissibility determination. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-003` | When Evidence is admitted, the system shall preserve its provenance, submitting authority, relation to the case, and verification status. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-004` | When Prior findings exist, the system shall retain the prior findings and identify which conclusions are revised rather than overwrite them. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-005` | When The evidence changes material facts, the system shall update the authoritative factual matrix and preserve the reason for each material revision. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-006` | When A revised factual matrix is established, the system shall require an updated remediation recommendation to be formulated against those facts. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-007` | When Independent approval is required, the system shall preserve the parent-defined separation between operational assessment and approval. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-008` | When Required facts and approvals are complete, the system shall transition the aggregate to `FACTS_REVISED_AND_AUTHENTICATED`. | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-009` | When New evidence is not admitted, the system shall retain the submission and rejection basis without changing authoritative case facts. | L3-I-182 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-182-BH-010` | When `Operational_Directive_Issued` is received as the parent-defined scenario-entry fact, the system shall recognize the directive from University Leadership / CoE without transferring ownership of that decision to Examination Exception & Contingency (I15). | L3-I-182 §14.1 |
| `L4-I-182-BH-011` | When The canonical scenario result is reached, the system shall publish `Exception_Facts_Updated_and_Reopened` on behalf of Examination Exception & Contingency (I15). | L3-I-182 §§4, 9, 13 |
| `L4-I-182-BH-012` | When the parent-defined audit checkpoint is authoritatively sealed, the system shall publish `Examination_Audit_Checkpoint_Sealed` on behalf of Examination Exception & Contingency (I15). | L3-I-182 §14.1 |

---

## 6. Failure / Rejection / Continuation

- If the case is already finally closed, this pre-resolution reopening path shall not complete; the authorized post-closure review path is separate.
- If evidence authenticity or admissibility remains unresolved, authoritative facts shall not be revised.
- If the revised recommendation is incomplete, the case shall remain open at its last valid state.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- original case facts and preliminary findings;
- new evidence and provenance;
- admissibility/authentication decision;
- fact-by-fact revisions;
- revised remediation recommendation;
- approval/attestation evidence;
- state transition and canonical events;

Prior decisions, facts, evidence, and state history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the aggregate is `FACTS_REVISED_AND_AUTHENTICATED`;
- the admitted evidence and its provenance are traceable;
- prior findings remain historically preserved;
- the revised factual matrix is explicit;
- a revised remediation recommendation exists;
- `Exception_Facts_Updated_and_Reopened` is published;

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-182`  
**Canonical Aggregate:** Examination Exception Case Aggregate  
**Entry State:** `IMPACT_ASSESSED_AND_CLASSIFIED`  
**Resulting State:** `FACTS_REVISED_AND_AUTHENTICATED`  
**Canonical Event:** `Exception_Facts_Updated_and_Reopened`  
**Owning Domain:** Examination Exception & Contingency (I15)  
**Primary Rule:** `BR-I15-011`

No new business rule, lifecycle state, event, canonical owner, or authority is introduced.

---

## 10. Open Issues / Deferred Design

**Upstream inconsistency:** The Locked parent metadata names participating domains I, G, and A, while its Ownership section instead lists Governance & Legal (B) and A and omits G. L4 preserves the Locked metadata and does not silently reconcile the discrepancy. Parent examples referring to CCTV, cyber logs, scanners, vaults, cryptographic hashes, blockchains, and message brokers are treated only as business evidence/implementation examples, not L4 technology requirements.

Implementation-specific details such as persistence, interfaces, transport, cryptographic methods, scanners, vault technology, access-control mechanisms, infrastructure, and UI/UX remain deferred to later architecture levels.
