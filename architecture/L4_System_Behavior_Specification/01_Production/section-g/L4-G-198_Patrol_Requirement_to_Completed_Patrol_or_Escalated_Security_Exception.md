# Patrol Requirement to Completed Patrol or Escalated Security Exception

## 1. Specification Metadata


**Specification ID:** ** `L4-G-198`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-198` — Patrol Requirement to Completed Patrol or Escalated Security Exception  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-198_Patrol_Requirement_to_Completed_Patrol_or_Escalated_Security_Exception.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed security patrol execution from dispatch through verified completion or recording/escalation of the parent-defined security exception.

The behavior begins when a patrol requirement is dispatched. It ends when the Patrol Run Aggregate reaches `PATROL_COMPLETED_OR_EXCEPTION_LOGGED`.

This specification does not define patrol-device technology, route-optimization algorithms, physical response tactics, or the downstream incident-resolution workflow.

---

## 3. Required Business Context

- Active Security Office and patrol context.
- Patrol requirement and route/checkpoint context.
- Eligible patrol personnel.
- Parent-defined checkpoint time-window rule.
- Patrol observations/anomalies.
- Security Operations ownership of the Patrol Run Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-198-BH-001` | When a patrol requirement is dispatched, the system shall establish the Patrol Run Aggregate in `PATROL_DISPATCHED`. | L3-G-198 §§4, 9, 13 |
| `L4-G-198-BH-002` | The system shall retain the governed patrol route and checkpoint requirements applicable to the dispatched patrol. | L3-G-198 §§2–3 |
| `L4-G-198-BH-003` | The system shall evaluate checkpoint completion against the parent-defined mandatory checkpoint time-window constraint. | L3-G-198 §12 |
| `L4-G-198-BH-004` | The system shall preserve patrol observations and anomalies required to determine whether the patrol completed normally or requires security-exception handling. | L3-G-198 §§2–3 |
| `L4-G-198-BH-005` | If required checkpoints are skipped, required verification fails, or a security breach is identified, the system shall prevent the patrol from being represented as an unqualified normal completion. | L3-G-198 §§3, 11–12 |
| `L4-G-198-BH-006` | The system shall preserve the parent-defined exception/escalation outcome when the patrol does not complete normally. | L3-G-198 §§3, 11 |
| `L4-G-198-BH-007` | On governed completion or exception logging, the system shall transition the Patrol Run Aggregate to `PATROL_COMPLETED_OR_EXCEPTION_LOGGED`. | L3-G-198 §§9, 13, 16 |
| `L4-G-198-BH-008` | Where a parent-supported supervisory exception applies, the system shall preserve the authorised decision and justification. | L3-G-198 §§10, 15 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-198-BH-009` | The system shall recognize authoritative personnel and other cross-domain facts required by the patrol scenario without transferring their ownership to Security Operations. | L3-G-198 §§5, 14 |
| `L4-G-198-BH-010` | When the patrol reaches the parent-defined completed aggregate outcome, the system shall produce the canonical `Security_Patrol_Completed` business event on behalf of Security Operations. | L3-G-198 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If required patrol credentials or verification fail, the system shall prevent normal patrol progression.
- If checkpoint or breach conditions require escalation, the system shall preserve the security-exception condition for downstream handling.
- If a parent-supported manual fallback is used, the system may recognize resulting business evidence without defining technical synchronization.
- Unresolved security incidents remain outside this specification after the patrol exception has been recorded/escalated.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- patrol dispatch;
- route/checkpoint requirements;
- checkpoint verification evidence;
- checkpoint timing compliance;
- patrol anomalies or breaches;
- normal-completion or exception disposition;
- any authorised supervisory exception;
- resulting `PATROL_COMPLETED_OR_EXCEPTION_LOGGED` position;
- publication of `Security_Patrol_Completed` where applicable to the parent-defined completion.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Patrol Run Aggregate is `PATROL_COMPLETED_OR_EXCEPTION_LOGGED`;
- route/checkpoint evidence is attributable;
- any security exception is preserved for escalation;
- the canonical completion event is produced when the parent-defined completed outcome applies;
- downstream incident investigation is not absorbed into this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-198` — Patrol Requirement to Completed Patrol or Escalated Security Exception  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent. No new exception event is introduced.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Checkpoint-reader technology, route telemetry implementation, geolocation, persistence, event transport, audit mechanisms, and UI/UX remain deferred.
