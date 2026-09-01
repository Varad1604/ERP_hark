# Classify an Examination Exception and Assess Operational Impact

## 1. Specification Metadata


**Specification ID:** ** `L4-I-173`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Exception & Contingency (I15)  
**Realizes Scenario:** ** `L3-I-173` — Classify an Examination Exception and Assess Operational Impact  
**Primary Business Owner:** Examination Exception & Contingency (I15)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-173_Classify_an_Examination_Exception_and_Assess_Operational_Impact.md`; `L2-I15_Examination_Exception_and_Contingency_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed assessment of a registered examination exception using the parent-defined severity model, including affected-candidate and operational-scope assessment, so that the required contingency-plan level is determined authoritatively.

**Start Boundary:** A registered and triaged examination exception has sufficient affected-centre information for operational impact assessment.

**End Boundary:** The exception reaches `IMPACT_ASSESSED_AND_CLASSIFIED` with an approved severity classification and corresponding contingency-plan level.

This specification does not define physical hardware, cryptographic mechanisms, persistence, APIs, message transport, technical identity/access controls, infrastructure, scanners, storage technologies, or UI/UX.

---

## 3. Required Business Context

- Canonical owner: Examination Exception & Contingency (I15).
- Canonical aggregate: **Examination Exception Case Aggregate**.
- Entry state: `EXCEPTION_REGISTERED_AND_TRIAGED`.
- Parent-defined primary rule: `BR-I15-002`.
- `BR-I-AUTH` — Dual Executive Sign-off Rule.
- `BR-I-AUDIT` — Immutable Digital Audit Ledger Rule, interpreted at L4 only as a requirement for durable, tamper-evident business evidence rather than a prescribed technology.
- Relevant upstream examination, campus-operations, governance, and platform facts remain owned by their canonical domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-173-BH-001` | The system shall require an existing exception case in `EXCEPTION_REGISTERED_AND_TRIAGED` before impact classification begins. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-002` | The system shall preserve the affected centres, halls, sessions, candidate counts, and operational conditions used in the assessment. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-003` | The system shall classify severity using `BR-I15-002` and the parent-defined categories Minor Local, Major Centre, or Catastrophic Multi-Centre. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-004` | The system shall determine and preserve the corresponding contingency level Plan A, Plan B, or Plan C as supported by the parent. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-005` | The system shall preserve affected-centre attestations supporting the impact assessment. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-006` | The system shall preserve the required decision and independent approval evidence for the classification. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-007` | The system shall prevent classification completion if material impact inputs remain unresolved or contradictory. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-008` | On valid approval, the system shall transition the exception case to `IMPACT_ASSESSED_AND_CLASSIFIED`. | L3-I-173 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-173-BH-009` | The system shall consume parent-defined operational directives and relevant examination/campus/governance facts from their canonical publishers without transferring ownership to I15. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-010` | When the governed outcome is reached, the system shall publish `Exception_Impact_Classified` on behalf of Examination Exception & Contingency. | L3-I-173 §§4, 9, 13 |
| `L4-I-173-BH-011` | Where the Locked parent requires `Examination_Audit_Checkpoint_Sealed`, the system shall record the corresponding governed audit checkpoint without prescribing cryptographic or ledger technology. | L3-I-173 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If the affected scope cannot be established reliably, impact classification shall remain pending.
- If required approval is withheld, the case shall not be represented as classified.
- A changed disruption scope shall require the assessment evidence to remain traceable rather than silently replacing the earlier assessment.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- impact inputs;
- affected-candidate count;
- severity classification;
- contingency-plan level;
- centre attestations;
- approval evidence;
- assessment rationale;
- final state and events.

Original evidence, decisions, timings, authorities, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the case is `IMPACT_ASSESSED_AND_CLASSIFIED`;
- severity classification is attributable to `BR-I15-002`;
- affected operational scope is recorded;
- Plan A/B/C requirement is determined;
- `Exception_Impact_Classified` is published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-173`  
**Canonical Aggregate:** Examination Exception Case Aggregate  
**Entry State:** `EXCEPTION_REGISTERED_AND_TRIAGED`  
**Resulting State:** `IMPACT_ASSESSED_AND_CLASSIFIED`  
**Canonical Event:** `Exception_Impact_Classified`  
**Owning Domain:** Examination Exception & Contingency (I15)

No new business rule, lifecycle state, event, aggregate owner, or authority is introduced by this specification.

---

## 10. Open Issues / Deferred Design

**Upstream issue:** The parent narrative business outcome calls the state `IMPACT_ASSESSED`, but the explicit lifecycle table and completion criteria use `IMPACT_ASSESSED_AND_CLASSIFIED`. L4 follows the explicit lifecycle state. Technology-specific telemetry, hashing, vault, blockchain, and broker mechanics are deferred.

Implementation topics explicitly deferred: physical vault design, cryptographic hashing/sealing, scanners, blockchain/ledger technology, message brokers, persistence, APIs, event transport, notification mechanisms, technical access control, infrastructure, and UI/UX.
