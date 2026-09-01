# Examination Exception Case Resolution to Closure

## 1. Specification Metadata


**Specification ID:** ** `L4-I-184`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Exception & Contingency (I15)  
**Realizes Scenario:** ** `L3-I-184` — Examination Exception Case Resolution to Closure  
**Primary Business Owner:** Examination Exception & Contingency (I15)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-I-184`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support formal closure of an examination exception only after every mandated remediation action has been verified, preserving closure authority, completion evidence, and the complete historical dossier.

**Start Boundary:** All remediation milestones are reported as completed and presented for verification.

**End Boundary:** The case reaches `EXCEPTION_CASE_CLOSED_AND_ARCHIVED` with the closure decision and supporting evidence preserved.

This specification remains implementation-independent and does not prescribe APIs, schemas, software components, persistence, message brokers, cryptographic mechanisms, infrastructure, technical IAM, hardware, or UI/UX.

---

## 3. Required Business Context

- Canonical owner: Examination Exception & Contingency (I15).
- Canonical aggregate: **Examination Exception Case Aggregate**.
- Entry state: `REMEDIATION_IN_PROGRESS`.
- Resulting state: `EXCEPTION_CASE_CLOSED_AND_ARCHIVED`.
- Primary parent rule: `BR-I15-013`.
- `BR-I-AUTH` requires independent executive approval where the Locked parent applies it.
- `BR-I-AUDIT` requires durable, attributable audit evidence without prescribing implementation technology.
- Parent-defined external facts and records remain owned by their canonical source domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-184-BH-001` | When Closure is requested, the system shall require the case to be in `REMEDIATION_IN_PROGRESS`. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-002` | When Remediation milestones exist, the system shall enumerate all mandated remediation obligations for the case and their completion evidence. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-003` | When Any mandated remediation remains incomplete, the system shall enforce `BR-I15-013` and block closure. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-004` | When All remediation is reported complete, the system shall require independent verification/attestation of completion evidence. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-005` | When Remediation includes outcomes owned by other domains, the system shall use authoritative completion evidence from those owners rather than recreate their records. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-006` | When Internal audit evidence is required, the system shall preserve the audit determination and any unresolved exceptions. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-007` | When The CoE authorizes closure, the system shall preserve the closure certificate/order and required independent approval evidence. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-008` | When All closure conditions are satisfied, the system shall transition the aggregate to `EXCEPTION_CASE_CLOSED_AND_ARCHIVED`. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-009` | When An unresolved remediation discrepancy is found during closure review, the system shall keep the case in its last valid non-closed state and preserve the discrepancy for continuation. | L3-I-184 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-184-BH-010` | When A parent-defined cross-domain fact is required, the system shall use the fact only from its canonical owner and shall not duplicate its ownership. | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-011` | When The canonical scenario result is reached, the system shall publish `Examination_Exception_Closed` on behalf of Examination Exception & Contingency (I15). | L3-I-184 §§4, 9, 13 |
| `L4-I-184-BH-012` | When A parent-required audit checkpoint is completed, the system shall preserve the audit checkpoint as governed evidence without prescribing hashing, blockchain, vault, broker, or transport technology. | L3-I-184 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If any remediation obligation lacks verified completion evidence, closure shall be rejected.
- If audit identifies an unresolved material discrepancy, the case shall remain open for correction.
- Archival implementation technology shall not determine business closure; closure depends on governed evidence and authorization.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- mandated remediation register;
- completion evidence from canonical owners;
- independent verification;
- audit findings;
- closure certificate/order;
- approval evidence;
- closure timestamp/history;
- state transition and canonical events;

Prior decisions, facts, evidence, and state history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the aggregate is `EXCEPTION_CASE_CLOSED_AND_ARCHIVED`;
- 100% of mandated remediation is verified;
- no unresolved material remediation discrepancy remains;
- closure authority is preserved;
- the historical case dossier remains traceable;
- `Examination_Exception_Closed` is published;

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-184`  
**Canonical Aggregate:** Examination Exception Case Aggregate  
**Entry State:** `REMEDIATION_IN_PROGRESS`  
**Resulting State:** `EXCEPTION_CASE_CLOSED_AND_ARCHIVED`  
**Canonical Event:** `Examination_Exception_Closed`  
**Owning Domain:** Examination Exception & Contingency (I15)  
**Primary Rule:** `BR-I15-013`

No new business rule, lifecycle state, event, canonical owner, or authority is introduced.

---

## 10. Open Issues / Deferred Design

**Upstream inconsistency:** The Locked parent metadata names I, G, and A, while the Ownership section lists B and A and omits G. L4 does not repair this conflict. The parent also defines closure using cryptographic hash, permanent vault, blockchain/ledger, scanners, and broker acknowledgements; L4 preserves only the business requirement for durable archival/audit evidence and defers technology.

Implementation-specific details such as persistence, interfaces, transport, cryptographic methods, scanners, vault technology, access-control mechanisms, infrastructure, and UI/UX remain deferred to later architecture levels.
