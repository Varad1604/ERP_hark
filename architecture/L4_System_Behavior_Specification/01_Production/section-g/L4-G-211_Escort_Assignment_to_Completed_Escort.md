# Escort Assignment to Completed Escort

## 1. Specification Metadata


**Specification ID:** ** `L4-G-211`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-211` — Escort Assignment to Completed Escort  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-211_Escort_Assignment_to_Completed_Escort.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed execution of an already assigned security escort from rendezvous through monitored transit, safe-arrival confirmation, and closure.

The behavior begins when an assigned escort detail meets the requestor and movement starts. It ends when the Escort Detail Aggregate reaches `ESCORT_COMPLETED_AND_CLOSED`.

This specification does not assign the escort, define GPS/radio technology, prescribe physical protection tactics, or govern downstream incidents arising during the escort.

---

## 3. Required Business Context

- Escort detail already assigned.
- Escort requestor and destination context.
- Lead Escort Guard and assigned escort personnel.
- Control Room Dispatcher participation.
- Parent-defined continuous radio/checkpoint muster rule.
- Safe-arrival confirmation requirement.
- Security Operations ownership of the Escort Detail Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-211-BH-001` | When an assigned escort begins movement, the system shall recognize the Escort Detail Aggregate in `IN_TRANSIT`. | L3-G-211 §§4, 9, 13 |
| `L4-G-211-BH-002` | The system shall preserve the escort requestor, assigned guards, destination, and governed movement context required by the parent scenario. | L3-G-211 §§2–4 |
| `L4-G-211-BH-003` | The system shall require the parent-defined continuous checkpoint/radio muster evidence throughout the escort lifecycle. | L3-G-211 §12 |
| `L4-G-211-BH-004` | If required checkpoint or communication accountability is lost, the system shall not represent the escort as an unqualified normal completion. | L3-G-211 §§11–12 |
| `L4-G-211-BH-005` | The system shall preserve the safe-arrival confirmation provided at the destination. | L3-G-211 §§2–4 |
| `L4-G-211-BH-006` | The system shall require the parent-defined supervisory/attestation evidence before final closure where applicable. | L3-G-211 §§9, 15 |
| `L4-G-211-BH-007` | On successful governed completion, the system shall transition the Escort Detail Aggregate from `IN_TRANSIT` to `ESCORT_COMPLETED_AND_CLOSED`. | L3-G-211 §§9, 13, 16 |
| `L4-G-211-BH-008` | The system shall preserve return of the escort guards to available operational status as a downstream operational fact without defining workforce scheduling implementation. | L3-G-211 §3 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-211-BH-009` | The system shall recognize authoritative requestor and personnel identity facts without transferring their ownership to Security Operations. | L3-G-211 §§5, 14 |
| `L4-G-211-BH-010` | When the Escort Detail Aggregate reaches `ESCORT_COMPLETED_AND_CLOSED`, the system shall produce the canonical `Security_Escort_Completed` business event on behalf of Security Operations. | L3-G-211 §§14, 16 |

No GPS protocol, radio infrastructure, route-navigation implementation, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If required checkpoint or escort accountability is lost, the escort shall remain unresolved until authorised security handling occurs.
- If a security breach interrupts the escort, the system shall preserve the last valid escort position for escalation.
- Parent-supported manual evidence may be recognized without defining technical synchronization.
- Downstream security incident handling remains outside this specification.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- escort start;
- requestor and destination context;
- assigned escort personnel;
- checkpoint/radio muster evidence;
- any exception or interruption;
- safe-arrival confirmation;
- resulting `ESCORT_COMPLETED_AND_CLOSED` position;
- publication of `Security_Escort_Completed`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Escort Detail Aggregate is `ESCORT_COMPLETED_AND_CLOSED`;
- continuous escort accountability is evidenced;
- safe arrival is confirmed;
- `Security_Escort_Completed` has been produced;
- downstream incident handling remains outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-211` — Escort Assignment to Completed Escort  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

GPS/radio technology, route tracking, personnel scheduling, persistence, event transport, audit implementation, and UI/UX remain deferred.
