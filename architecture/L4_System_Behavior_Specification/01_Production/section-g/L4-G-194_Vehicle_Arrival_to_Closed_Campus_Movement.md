# Vehicle Arrival to Closed Campus Movement

## 1. Specification Metadata


**Specification ID:** ** `L4-G-194`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-194` — Vehicle Arrival to Closed Campus Movement  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-194_Vehicle_Arrival_to_Closed_Campus_Movement.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed recording and completion of a campus vehicle movement from arrival at the security boundary through permitted campus transit and authoritative exit closure.

The behavior begins when a vehicle arrives at the campus vehicular security barrier. It ends when the Vehicle Movement Record reaches `VEHICLE_TRANSIT_CLOSED`.

This specification does not own vehicle master data outside Security Operations, parking infrastructure, physical barrier actuation, traffic enforcement execution, or downstream incident handling.

---

## 3. Required Business Context

- Active Security Office and vehicular gate context.
- Vehicle and driver/occupant identity or credential facts required by the parent.
- Applicable parking-zone and vehicle-compliance context.
- Required physical/cargo inspection result.
- Transport Officer collaboration where applicable.
- Security Operations ownership of the Vehicle Movement Record Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-194-BH-001` | When a vehicle arrives at the campus security boundary, the system shall establish a Vehicle Movement Record in `VEHICLE_INTAKE`. | L3-G-194 §§4, 9, 13 |
| `L4-G-194-BH-002` | The system shall evaluate the vehicle movement against authoritative vehicle/occupant credentials and the parent-defined security inspection conditions. | L3-G-194 §§2, 9, 14 |
| `L4-G-194-BH-003` | The system shall require the applicable parking-zone and parent-defined vehicle-compliance conditions before recording permitted campus movement. | L3-G-194 §§2–3, 12 |
| `L4-G-194-BH-004` | The system shall prevent normal progression when required credential or physical verification fails. | L3-G-194 §11 |
| `L4-G-194-BH-005` | The system shall preserve the governed parking-zone allocation associated with the permitted vehicle movement. | L3-G-194 §3 |
| `L4-G-194-BH-006` | The system shall preserve the required entry and exit movement evidence needed to demonstrate the closed campus movement. | L3-G-194 §§2–4, 16 |
| `L4-G-194-BH-007` | On successful governed exit completion, the system shall transition the Vehicle Movement Record Aggregate from `VEHICLE_INTAKE` to `VEHICLE_TRANSIT_CLOSED`. | L3-G-194 §§9, 13, 16 |
| `L4-G-194-BH-008` | Where the parent requires supervisory clearance or an authorised exception, the system shall preserve the relevant decision and justification. | L3-G-194 §§10, 15 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-194-BH-009` | The system shall recognize authoritative identity and Transport-domain facts used in vehicle-movement governance without transferring their ownership to Security Operations. | L3-G-194 §§5, 14 |
| `L4-G-194-BH-010` | When the vehicle movement reaches `VEHICLE_TRANSIT_CLOSED`, the system shall produce the canonical `Vehicle_Movement_Completed` business event on behalf of Security Operations. | L3-G-194 §§14, 16 |

No RFID, ANPR, barrier-control, parking-sensor, or event-transport design is defined.

---

## 6. Failure / Rejection / Continuation

- If required vehicle, occupant, or physical inspection verification fails, the system shall prevent normal campus-movement progression.
- If a parent-supported exception is authorised, the system shall preserve its authority and justification.
- If the vehicle movement cannot be validly closed, the system shall retain the last valid business position and expose the unresolved condition.
- If a security breach interrupts the movement, the system shall preserve the movement record for authorised escalation without defining the downstream response implementation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- vehicle movement intake;
- identity/credential facts considered;
- inspection evidence;
- applicable parking-zone allocation;
- supervisory or exception decision where applicable;
- entry and exit movement evidence;
- previous and resulting Vehicle Movement Record position;
- publication of `Vehicle_Movement_Completed`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Vehicle Movement Record is `VEHICLE_TRANSIT_CLOSED`;
- required vehicle/credential/inspection conditions are attributable;
- parking-zone allocation and exit completion are recorded;
- `Vehicle_Movement_Completed` has been produced;
- physical barrier operation and downstream traffic/security enforcement remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-194` — Vehicle Arrival to Closed Campus Movement  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Vehicle-reader technology, parking-device integration, barrier control, persistence, event transport, audit implementation, and UI/UX remain deferred.
