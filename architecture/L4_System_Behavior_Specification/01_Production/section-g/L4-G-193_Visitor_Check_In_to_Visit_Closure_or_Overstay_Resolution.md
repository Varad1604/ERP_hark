# Visitor Check In to Visit Closure or Overstay Resolution

## 1. Specification Metadata


**Specification ID:** ** `L4-G-193`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-193` — Visitor Check In to Visit Closure or Overstay Resolution  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-193_Visitor_Check_In_to_Visit_Closure_or_Overstay_Resolution.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed completion of an active visitor visit through normal checkout or parent-supported overstay escalation and resolution.

The behavior begins with a visitor pass in `PASS_ACTIVE` when a visitor check-in/exit activity or overstay condition is evaluated. It ends when the visit is closed or the overstay is resolved under the parent-defined terminal outcome.

This specification does not create the visitor pass, define physical badge-reader technology, conduct security searches, or absorb downstream disciplinary/investigation scenarios.

---

## 3. Required Business Context

- Visitor Pass Aggregate currently in `PASS_ACTIVE`.
- Applicable visit duration and curfew constraints.
- Authoritative visitor, host, and active-pass context.
- Exit/check-in evidence or overstay condition.
- Security Shift Supervisor, Gate Exit Guard, and escalation authority established by the parent.
- Security Operations ownership of the Visitor Pass Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-193-BH-001` | When an active visitor presents for checkout or a relevant visitor movement is recorded, the system shall evaluate the active visit against the parent-defined visit conditions. | L3-G-193 §§2, 4, 9 |
| `L4-G-193-BH-002` | The system shall use the authoritative active-pass and visit-duration context when determining whether the visit may close normally or requires overstay handling. | L3-G-193 §§3, 12–13 |
| `L4-G-193-BH-003` | When the visitor completes the governed exit conditions, the system shall record the visit as closed under the parent-defined completion outcome. | L3-G-193 §§3–4, 16 |
| `L4-G-193-BH-004` | When the maximum permitted visitor duration is exceeded, the system shall expose the parent-defined overstay condition for Security Operations resolution rather than silently closing the visit. | L3-G-193 §§2–3, 12 |
| `L4-G-193-BH-005` | The system shall preserve the authorised disposition used to resolve an overstay condition. | L3-G-193 §§3, 10, 15 |
| `L4-G-193-BH-006` | The system shall prevent the operational actor executing the visit action from being treated as the approving authority for the same supervisory clearance where the parent requires separation. | L3-G-193 §15.2 |
| `L4-G-193-BH-007` | On governed completion, the system shall transition the Visitor Pass Aggregate from `PASS_ACTIVE` to `PASS_CLOSED_OR_OVERSTAY_RESOLVED`. | L3-G-193 §§9, 13, 16 |
| `L4-G-193-BH-008` | The system shall distinguish evidence of normal visit closure from evidence of overstay resolution within the completed outcome. | L3-G-193 §§2–3, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-193-BH-009` | The system shall recognize authoritative host and identity facts needed by the visit lifecycle without transferring ownership of those facts. | L3-G-193 §§5, 14 |
| `L4-G-193-BH-010` | When the visit reaches `PASS_CLOSED_OR_OVERSTAY_RESOLVED`, the system shall produce the canonical `Visitor_Visit_Closed_or_Overstay_Resolved` business event on behalf of Security Operations. | L3-G-193 §§14, 16 |

No location-tracking technology, scanner protocol, or event-delivery mechanism is defined.

---

## 6. Failure / Rejection / Continuation

- If required credential or physical verification prevents normal visit closure, the system shall preserve the active or unresolved business position pending governed resolution.
- If an overstay remains unresolved, the system shall not represent the visit as normally closed.
- If a parent-supported exception is authorised, the system shall preserve the decision and justification.
- If an unresolved security breach interrupts the scenario, the system shall preserve the last valid business position and expose the condition for authorised escalation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- active visitor-pass context;
- visit timing/duration basis considered;
- visitor movement or exit evidence;
- normal checkout outcome where applicable;
- overstay condition and resolution where applicable;
- relevant supervisory decision or exception;
- previous and resulting Visitor Pass Aggregate position;
- publication of `Visitor_Visit_Closed_or_Overstay_Resolved`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Visitor Pass Aggregate is `PASS_CLOSED_OR_OVERSTAY_RESOLVED`;
- either normal checkout or overstay resolution is attributable;
- required evidence is preserved;
- the canonical completion event has been produced;
- security investigation execution and physical tracking remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-193` — Visitor Check In to Visit Closure or Overstay Resolution  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Reader/device integration, tracking technology, persistence, event transport, technical access control, audit implementation, and UI/UX remain deferred.
