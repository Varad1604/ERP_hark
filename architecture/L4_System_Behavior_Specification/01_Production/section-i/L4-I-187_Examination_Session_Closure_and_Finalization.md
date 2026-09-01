# Examination Session Closure and Finalization

## 1. Specification Metadata


**Specification ID:** ** `L4-I-187`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Session Closure & Archival (I16)  
**Realizes Scenario:** ** `L3-I-187` — Examination Session Closure and Finalization  
**Primary Business Owner:** Examination Session Closure & Archival (I16)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-I-187`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support formal institutional finalization of a completed examination session only after evaluation, result declaration, revaluation, grievance, and other parent-defined closure obligations are complete, locking the session against further ordinary modification.

**Start Boundary:** Result declaration is complete and the statutory 30-day revaluation/grievance window has expired.

**End Boundary:** The session reaches `SESSION_FINALIZED_AND_LOCKED`, its mark ledgers are frozen against ordinary edit, and session-finalization evidence is preserved.

This specification remains implementation-independent. It does not prescribe APIs, data schemas, software components, persistence, transport, cryptographic implementation, physical storage design, infrastructure, technical access control, or UI/UX.

---

## 3. Required Business Context

- Canonical owner: Examination Session Closure & Archival (I16).
- Canonical aggregate: **Examination Session Aggregate**.
- Entry state: `EVALUATION_CONCLUDED`.
- Resulting state: `SESSION_FINALIZED_AND_LOCKED`.
- Primary parent rule: `BR-I16-001`.
- `BR-I-AUTH` requires independent executive approval where applicable.
- `BR-I-AUDIT` is interpreted at L4 as a durable and attributable evidence obligation, not a technology mandate.
- Records, legal directives, finance facts, governance facts, campus facts, and platform capabilities remain owned by their canonical domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-187-BH-001` | When Session finalization is requested, the system shall require the session to be in `EVALUATION_CONCLUDED`. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-002` | When Results and evaluation prerequisites are reviewed, the system shall verify that all parent-defined evaluation, marks-entry, result-declaration, revaluation, and grievance obligations are complete. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-003` | When A statutory revaluation/grievance window applies, the system shall prevent finalization before the parent-defined 30-day window expires or all authorized matters are resolved. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-004` | When Mark ledgers are ready for closure, the system shall enforce `BR-I16-001` so ordinary post-closure mark edits are prohibited. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-005` | When Financial or other cross-domain completion facts are required, the system shall consume authoritative completion evidence from the canonical owner without transferring ownership. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-006` | When Academic performance metrics are finalized, the system shall preserve the finalized metrics as the session-level outcome without redefining their source calculations. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-007` | When Board of Examinations approval is required, the system shall preserve executive approval and independent attestation evidence. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-008` | When All closure requirements are satisfied, the system shall transition the session to `SESSION_FINALIZED_AND_LOCKED`. | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-009` | When A material unresolved discrepancy is detected, the system shall block finalization and preserve the discrepancy for correction. | L3-I-187 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-187-BH-010` | When `Operational_Directive_Issued` is received as the parent-defined scenario-entry fact, the system shall recognize the directive from University Leadership / CoE without transferring ownership of that decision to Examination Session Closure & Archival (I16). | L3-I-187 §14.1 |
| `L4-I-187-BH-011` | When The canonical scenario result is reached, the system shall publish `Examination_Session_Finalized` on behalf of Examination Session Closure & Archival (I16). | L3-I-187 §§4, 9, 13 |
| `L4-I-187-BH-012` | When the parent-defined audit checkpoint is authoritatively sealed, the system shall publish `Examination_Audit_Checkpoint_Sealed` on behalf of Examination Session Closure & Archival (I16). | L3-I-187 §14.1 |

---

## 6. Failure / Rejection / Continuation

- If any required evaluation, result, revaluation, grievance, or closure obligation remains unresolved, finalization shall not complete.
- If a post-finalization change is later required, it must proceed through a separately authorized exception/reopening path rather than ordinary edit.
- If required cross-domain completion evidence is unavailable, the session shall remain at its last valid state.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- session identity and examination scope;
- evaluation/result completion evidence;
- revaluation/grievance-window evidence;
- mark-ledger closure status;
- cross-domain completion attestations;
- Board approval;
- finalization certificate;
- state transition and events;

Original and superseded records, approvals, custody evidence, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the session is `SESSION_FINALIZED_AND_LOCKED`;
- ordinary mark-ledger edits are blocked by the parent rule;
- all statutory closure prerequisites are satisfied;
- approval and finalization evidence are attributable;
- `Examination_Session_Finalized` is published;

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-187`  
**Canonical Aggregate:** Examination Session Aggregate  
**Entry State:** `EVALUATION_CONCLUDED`  
**Resulting State:** `SESSION_FINALIZED_AND_LOCKED`  
**Canonical Event:** `Examination_Session_Finalized`  
**Owning Domain:** Examination Session Closure & Archival (I16)  
**Primary Rule:** `BR-I16-001`

No new business rule, lifecycle state, business event, canonical owner, or business authority is introduced.

---

## 10. Open Issues / Deferred Design

**Upstream inconsistency:** The Locked metadata lists participating domains I, F, and A, while the Ownership section instead names Governance & Legal (B) and A and omits F. L4 preserves the Locked metadata and treats any Finance participation strictly as a cross-domain fact/hand-off. The parent also self-references `L3-I-187` as a successor; this is recorded as an upstream traceability defect and is not reproduced as a new L4 dependency.

Implementation details deferred to later architecture levels include physical vault design, barcodes, scanners, cryptographic hashing/signing, blockchain or ledger technology, storage partitions, databases, message brokers, APIs, persistence, event transport, technical access controls, infrastructure, and UI/UX.
