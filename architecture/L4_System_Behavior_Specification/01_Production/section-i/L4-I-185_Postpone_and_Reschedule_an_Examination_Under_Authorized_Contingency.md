# Postpone and Reschedule an Examination Under Authorized Contingency

## 1. Specification Metadata


**Specification ID:** ** `L4-I-185`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Examination Exception & Contingency (I15)  
**Realizes Scenario:** ** `L3-I-185` — Postpone and Reschedule an Examination Under Authorized Contingency  
**Primary Business Owner:** Examination Exception & Contingency (I15)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-I-185`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support authorized postponement of a scheduled examination before conduct begins when a qualifying contingency applies, preserving the original schedule, the postponement authority, and the governed replacement date/venue.

**Start Boundary:** A qualifying contingency advisory or directive is issued before the scheduled examination begins.

**End Boundary:** The session reaches `SESSION_POSTPONED_AND_RESCHEDULED` with a revised examination occurrence established and available for downstream candidate communication.

This specification remains implementation-independent and does not prescribe APIs, schemas, software components, persistence, message brokers, cryptographic mechanisms, infrastructure, technical IAM, hardware, or UI/UX.

---

## 3. Required Business Context

- Canonical owner: Examination Exception & Contingency (I15).
- Canonical aggregate: **Examination Session Schedule Aggregate**.
- Entry state: `SESSION_SCHEDULED`.
- Resulting state: `SESSION_POSTPONED_AND_RESCHEDULED`.
- Primary parent rule: `BR-I15-014`.
- `BR-I-AUTH` requires independent executive approval where the Locked parent applies it.
- `BR-I-AUDIT` requires durable, attributable audit evidence without prescribing implementation technology.
- Parent-defined external facts and records remain owned by their canonical source domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-185-BH-001` | When A contingency postponement is proposed, the system shall require an existing session in `SESSION_SCHEDULED` and verify that conduct has not begun. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-002` | When The contingency basis is recorded, the system shall preserve the authoritative advisory/directive and affected examination scope. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-003` | When Advance notice is assessed, the system shall enforce `BR-I15-014` and preserve whether the minimum 24-hour requirement is satisfied or an authorized exception applies. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-004` | When The original session is postponed, the system shall retain the original schedule and mark its contingency disposition without erasing historical scheduling facts. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-005` | When A replacement occurrence is proposed, the system shall require a revised date and venue supported by parent-defined operational facts from their canonical owners. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-006` | When Candidate eligibility/identity artefacts need revision, the system shall preserve the required downstream hand-off for updated candidate access/hall-ticket information without prescribing token technology. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-007` | When The revised schedule requires approval, the system shall preserve the CoE/Vice-Chancellor authorization and required independent sign-off. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-008` | When All required schedule facts and approvals are complete, the system shall transition the aggregate to `SESSION_POSTPONED_AND_RESCHEDULED`. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-009` | When Affected stakeholders require notice, the system shall expose the authoritative postponement/reschedule outcome for the parent-defined communication scenario rather than duplicate communication ownership. | L3-I-185 §§4, 9, 13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-185-BH-010` | When A parent-defined cross-domain fact is required, the system shall use the fact only from its canonical owner and shall not duplicate its ownership. | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-011` | When The canonical scenario result is reached, the system shall publish `Examination_Postponed_and_Rescheduled` on behalf of Examination Exception & Contingency (I15). | L3-I-185 §§4, 9, 13 |
| `L4-I-185-BH-012` | When A parent-required audit checkpoint is completed, the system shall preserve the audit checkpoint as governed evidence without prescribing hashing, blockchain, vault, broker, or transport technology. | L3-I-185 §§4, 9, 13 |

---

## 6. Failure / Rejection / Continuation

- If conduct has already begun, this pre-conduct postponement scenario shall not be used.
- If the minimum-notice invariant is not met and no authorized exception exists, postponement shall not be represented as compliant.
- If a valid replacement date/venue is not established, the session shall remain visibly postponed/unresolved rather than falsely complete rescheduling.

---

## 7. Audit & Evidence Behavior

The system shall preserve:
- contingency advisory/directive;
- original session schedule;
- notice-rule assessment;
- postponement decree;
- replacement date/venue facts;
- candidate-impact hand-off;
- approval evidence;
- state transition and events;

Prior decisions, facts, evidence, and state history shall remain traceable and shall not be silently overwritten.

---

## 8. Observable Completion Conditions

Completion requires:
- the aggregate is `SESSION_POSTPONED_AND_RESCHEDULED`;
- the original schedule remains traceable;
- the contingency and notice basis are preserved;
- the replacement date and venue are authoritative;
- required candidate-impact and communication hand-offs are available;
- `Examination_Postponed_and_Rescheduled` is published;

---

## 9. Traceability & Provenance

**Realizes Scenario:** `L3-I-185`  
**Canonical Aggregate:** Examination Session Schedule Aggregate  
**Entry State:** `SESSION_SCHEDULED`  
**Resulting State:** `SESSION_POSTPONED_AND_RESCHEDULED`  
**Canonical Event:** `Examination_Postponed_and_Rescheduled`  
**Owning Domain:** Examination Exception & Contingency (I15)  
**Primary Rule:** `BR-I15-014`

No new business rule, lifecycle state, event, canonical owner, or authority is introduced.

---

## 10. Open Issues / Deferred Design

**Upstream inconsistency:** The Locked parent metadata names I, G, and A, while its Ownership section lists B and A and omits G. L4 preserves the Locked metadata and uses G-sourced operational facts without redefining ownership. The parent phrase 'hall ticket access tokens' is treated as a business requirement to refresh candidate examination-access information, not as a technical token design.

Implementation-specific details such as persistence, interfaces, transport, cryptographic methods, scanners, vault technology, access-control mechanisms, infrastructure, and UI/UX remain deferred to later architecture levels.
