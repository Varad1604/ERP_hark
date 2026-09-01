# Compromised Question Paper to Authoritative Replacement or Exception Outcome

## 1. Specification Metadata


**Specification ID:** ** `L4-I-064`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-064` — Compromised Question Paper to Authoritative Replacement or Exception Outcome  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-064_Compromised_Question_Paper_to_Authoritative_Replacement_or_Exception_Outcome.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed response to a confirmed question-paper compromise, resulting in either authoritative reserve-set activation or formal examination postponement.

The behavior begins when a seal breach or question-paper leak is confirmed. It ends when the Question Paper Security Docket Aggregate reaches `RESERVE_SET_ISSUED_OR_POSTPONED`.

This specification does not conduct leak investigations, monitor social media, define paper-distribution technology, or create new statutory authority.

---

## 3. Required Business Context

- Confirmed question-paper compromise.
- Controller of Examinations.
- Vice Chancellor / Executive Head.
- Chief Superintendent.
- University Observer.
- Reserve question-paper set.
- Parent-defined immediate invalidation and reserve-activation rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-064-BH-001` | When a question-paper compromise is confirmed, the system shall establish the Question Paper Security Docket in `COMPROMISE_CONFIRMED`. | L3-I-064 §§4, 9, 13 |
| `L4-I-064-BH-002` | The system shall preserve the compromise evidence and affected examination-session context. | L3-I-064 §§2–4 |
| `L4-I-064-BH-003` | The system shall treat the compromised paper as invalid under BR-I07-010. | L3-I-064 §12 |
| `L4-I-064-BH-004` | The system shall support only the parent-defined resolution outcomes: reserve-set activation or formal postponement. | L3-I-064 §§2–3 |
| `L4-I-064-BH-005` | The system shall preserve the authority basis for the selected replacement/exception outcome. | L3-I-064 §§6, 15 |
| `L4-I-064-BH-006` | If reserve-set activation is chosen, the system shall preserve its authorized issue and distribution status without defining technical distribution mechanics. | L3-I-064 §§2–4 |
| `L4-I-064-BH-007` | If postponement is chosen, the system shall preserve the formal postponement outcome and notification requirement without inventing a new lifecycle state. | L3-I-064 §§2–4, 13 |
| `L4-I-064-BH-008` | On governed resolution, the system shall transition the Question Paper Security Docket from `COMPROMISE_CONFIRMED` to `RESERVE_SET_ISSUED_OR_POSTPONED`. | L3-I-064 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-064-BH-009` | The system shall recognize authoritative scheduling/candidate facts and external compromise evidence without transferring their ownership to I07. | L3-I-064 §§5, 14 |
| `L4-I-064-BH-010` | When the compromise is resolved, the system shall produce the canonical `Question_Paper_Compromise_Resolved` event. | L3-I-064 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- A confirmed compromised paper shall not continue as the valid examination paper.
- If neither reserve activation nor valid postponement can be authorized, the case shall remain unresolved.
- Leak investigation and disciplinary/statutory follow-up remain outside this specification.

---

## 7. Audit & Evidence Behavior

Preserve compromise confirmation, invalidated paper identity, selected resolution, decision authority, reserve-set issue or postponement evidence, final state, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `RESERVE_SET_ISSUED_OR_POSTPONED`;
- compromised paper no longer treated as valid;
- selected outcome and authority attributable;
- `Question_Paper_Compromise_Resolved` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-064`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

The Locked parent contains generic answer-script/attendance closure wording not necessary for this question-paper security exception boundary. L4 follows the specific aggregate, state transition, rule, and event.

Leak-detection tooling, social-media monitoring, distribution technology, persistence, event transport, and UI/UX remain deferred.
