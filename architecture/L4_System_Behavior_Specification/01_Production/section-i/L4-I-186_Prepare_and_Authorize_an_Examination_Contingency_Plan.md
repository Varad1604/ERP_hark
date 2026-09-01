# Prepare and Authorize an Examination Contingency Plan

## 1. Specification Metadata


**Specification ID:** ** `L4-I-186`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Exception & Contingency (I15)  
**Realizes Scenario:** ** `L3-I-186` — Prepare and Authorize an Examination Contingency Plan  
**Primary Business Owner:** Examination Exception & Contingency (I15)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-I-186`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support preparation and authorization of an examination contingency plan before an examination series begins, ensuring all parent-required reserve resources and contingency arrangements are identified, verified, and authorized for activation when needed.

**Start Boundary:** The pre-examination planning cycle is initiated by the Controller of Examinations.

**End Boundary:** The Examination Contingency Plan reaches `PLAN_AUTHORIZED_ACTIVE` with complete reserve-resource coverage and authorised contingency arrangements.

This specification remains implementation-independent and does not prescribe APIs, schemas, software components, persistence, message brokers, cryptographic mechanisms, infrastructure, technical IAM, hardware, or UI/UX.

---

## 3. Required Business Context

- Canonical owner: Examination Exception & Contingency (I15).
- Canonical aggregate: **Examination Contingency Plan Aggregate**.
- Entry state: `PLAN_DRAFT`.
- Resulting state: `PLAN_AUTHORIZED_ACTIVE`.
- Primary parent rule: `BR-I15-015`.
- `BR-I-AUTH` requires independent executive approval where the Locked parent applies it.
- `BR-I-AUDIT` requires durable, attributable audit evidence without prescribing implementation technology.
- Parent-defined external facts and records remain owned by their canonical source domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-186-BH-001` | When A new planning cycle begins, the system shall establish the Examination Contingency Plan in `PLAN_DRAFT` for the relevant examination series. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-002` | When Contingency risks are assessed, the system shall preserve the parent-defined risk/contingency categories and planned responses without prescribing technical solutions. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-003` | When Reserve resources are defined, the system shall apply `BR-I15-015` and require complete pre-allocation of the reserve resources mandated by the parent plan. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-004` | When Reserve facts belong to Campus Operations or another domain, the system shall use authoritative availability/commitment evidence from that canonical owner. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-005` | When Alternative centres, transport, power, security, or paper arrangements are included, the system shall preserve their business commitments and activation conditions without prescribing devices, generators, routes, or storage technology. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-006` | When The plan defines emergency activation/dispatch responsibilities, the system shall preserve the authorised responsibility tree and escalation contacts as business roles. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-007` | When Plan readiness is reviewed, the system shall require parent-defined Campus Operations/security attestations and examination-domain review evidence. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-008` | When The Controller of Examinations authorizes the plan, the system shall preserve authorization and independent sign-off required by the parent. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-009` | When All mandatory plan elements and reserve commitments are satisfied, the system shall transition the aggregate to `PLAN_AUTHORIZED_ACTIVE`. | L3-I-186 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-186-BH-010` | When A parent-defined cross-domain fact is required, the system shall use the fact only from its canonical owner and shall not duplicate its ownership. | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-011` | When The canonical scenario result is reached, the system shall publish `Contingency_Plan_Authorized` on behalf of Examination Exception & Contingency (I15). | L3-I-186 §§4, 9, 13 |
| `L4-I-186-BH-012` | When A parent-required audit checkpoint is completed, the system shall preserve the audit checkpoint as governed evidence without prescribing hashing, blockchain, vault, broker, or transport technology. | L3-I-186 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If mandatory reserve-resource coverage is incomplete, the plan shall remain Draft and shall not be represented as active.
- If a required external-domain commitment is unavailable or unverified, authorization shall be blocked or remain unresolved.
- If the plan is revised after authorization, the system shall preserve version/history rather than silently overwrite the authorised plan.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- planning-cycle identity;
- risk/contingency assessment;
- reserve-resource register;
- canonical-owner resource commitments;
- activation conditions and responsibility tree;
- Campus Operations/security attestations;
- CoE authorization;
- state transition and canonical events;

Prior decisions, facts, evidence, and state history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the aggregate is `PLAN_AUTHORIZED_ACTIVE`;
- all parent-mandated reserve resources are pre-allocated;
- required external-domain commitments are verified;
- activation roles and conditions are explicit;
- authorization evidence is preserved;
- `Contingency_Plan_Authorized` is published;

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-186`  
**Canonical Aggregate:** Examination Contingency Plan Aggregate  
**Entry State:** `PLAN_DRAFT`  
**Resulting State:** `PLAN_AUTHORIZED_ACTIVE`  
**Canonical Event:** `Contingency_Plan_Authorized`  
**Owning Domain:** Examination Exception & Contingency (I15)  
**Primary Rule:** `BR-I15-015`

No new business rule, lifecycle state, event, canonical owner, or authority is introduced.

---

## 10. Open Issues / Deferred Design

**Upstream inconsistency:** The Locked parent metadata names I, G, and A, while its Ownership section lists B and A and omits G even though G01 and G07 actors are explicit. L4 preserves the Locked metadata and treats Campus Operations/security resource commitments as cross-domain facts. Parent references to reserve paper sets, generators, physical storage, scanners, cryptographic vaults, blockchain/ledger, and message brokers are not converted into L4 technical design.

Implementation-specific details such as persistence, interfaces, transport, cryptographic methods, scanners, vault technology, access-control mechanisms, infrastructure, and UI/UX remain deferred to later architecture levels.
