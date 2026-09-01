# Conflicting Malpractice Decisions to Single Authoritative Decision

## 1. Specification Metadata


**Specification ID:** ** `L4-I-169`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Malpractice & Examination Committee (I14)  
**Realizes Scenario:** ** `L3-I-169` — Conflicting Malpractice Decisions to Single Authoritative Decision  
**Primary Business Owner:** Malpractice & Examination Committee (I14)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-169_Conflicting_Malpractice_Decisions_to_Single_Authoritative_Decision.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed reconciliation of conflicting malpractice decisions concerning the same case by escalating them to the Full Malpractice Committee for one binding authoritative decision.

**Start Boundary:** The Controller of Examinations detects conflicting rulings or recommendations from separate malpractice benches/sub-committees for the same case.

**End Boundary:** The Full Committee's binding decision is recorded and the case reaches `SINGLE_AUTHORITATIVE_DECISION_SEALED`.

This specification remains implementation-independent and does not define APIs, persistence, software components, transport mechanisms, technical access control, infrastructure, or UI/UX.

---

## 3. Required Business Context

- Canonical aggregate: **Malpractice Case Dossier Aggregate**.
- Entry state: `CONFLICTING_RULINGS_PENDING`.
- Parent-defined governing rule: `BR-I14-008`.
- `BR-I14-NAT` — Principle of Natural Justice Invariant.
- `BR-I14-SEAL` — Evidence Tamper-Proof Custody Rule.
- Malpractice & Examination Committee remains the canonical owner.
- Student Lifecycle, Governance/Legal, Result Processing, and Platform Foundation participate only through parent-defined collaboration.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-169-BH-001` | The system shall identify the malpractice case and preserve each conflicting ruling/recommendation as distinct evidence. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-002` | The system shall place the case in `CONFLICTING_RULINGS_PENDING` while authoritative resolution is outstanding. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-003` | The system shall require Full Malpractice Committee review under `BR-I14-008`. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-004` | The system shall preserve the plenary quorum and voting evidence required for the binding resolution. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-005` | The system shall preserve candidate notice and fair-hearing obligations under `BR-I14-NAT` where the resolution affects the candidate. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-006` | The system shall preserve evidence-custody continuity under `BR-I14-SEAL`. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-007` | The system shall record the majority/casting-vote basis and final authoritative adjudication order. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-008` | The system shall supersede conflicting recommendations only through the authoritative decision while retaining them historically. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-009` | On valid resolution, the system shall transition the Malpractice Case Dossier Aggregate to `SINGLE_AUTHORITATIVE_DECISION_SEALED`. | L3-I-169 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-169-BH-010` | The system shall consume relevant upstream examination, student, appeal, governance, or incident facts only from their canonical publishers and shall not transfer their ownership to I14. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-011` | When the governed result is reached, the system shall publish `Conflicting_Malpractice_Decisions_Resolved` on behalf of Malpractice & Examination Committee. | L3-I-169 §§4, 9, 13 |
| `L4-I-169-BH-012` | The system shall preserve the parent-defined `Student_Disciplinary_Hold_Applied` only when consistent with the authoritative resolved decision, without inventing a hold merely because conflicting rulings existed. | L3-I-169 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If full-committee quorum is not satisfied, the conflict shall remain unresolved.
- If evidence integrity or material procedure is compromised, the system shall not seal a final authoritative decision.
- The system shall not select one conflicting ruling merely because it was earlier or later in time unless the parent-defined authority decides it.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- each conflicting ruling;
- case identity;
- full-committee composition/quorum;
- candidate notice/hearing evidence;
- committee vote/casting-vote evidence;
- final authoritative adjudication order;
- canonical event.

Original case, evidence, decision, and lifecycle history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the case is `SINGLE_AUTHORITATIVE_DECISION_SEALED`;
- one binding decision is authoritative;
- all conflicting prior decisions remain traceable;
- `Conflicting_Malpractice_Decisions_Resolved` is published.

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-169`  
**Canonical Aggregate:** Malpractice Case Dossier Aggregate  
**Entry State:** `CONFLICTING_RULINGS_PENDING`  
**Resulting State:** `SINGLE_AUTHORITATIVE_DECISION_SEALED`  
**Canonical Event:** `Conflicting_Malpractice_Decisions_Resolved`  
**Owning Domain:** Malpractice & Examination Committee (I14)

No new business rule, event, lifecycle state, aggregate owner, or authority is introduced by this specification.

---

## 10. Open Issues / Deferred Design

**Upstream issue:** The parent also lists `Student_Disciplinary_Hold_Applied` generically. L4 preserves that parent event only where the final authoritative decision actually supports such a consequence; conflict resolution itself is not treated as an automatic new disciplinary hold.

Deferred to later architecture levels: evidence-storage mechanism, cryptographic implementation, persistence model, notification transport, event transport, technical access controls, infrastructure, and UI/UX.
