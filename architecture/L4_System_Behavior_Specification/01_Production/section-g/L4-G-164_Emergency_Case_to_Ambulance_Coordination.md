# Emergency Case to Ambulance Coordination

## 1. Specification Metadata

**Specification ID:** L4-G-164  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-164 — Emergency Case to Ambulance Coordination  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Locked L3-G-164; Locked L2-G06; Locked L1 Business Domain Architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support ambulance coordination for an Emergency Case when ambulance transport is determined to be required. The behavior ends with the applicable institutional coordination outcome recorded. Ambulance fleet, vehicle and crew management remain external.

## 3. Required Business Context

- Emergency Case and patient context are G06-owned facts.
- Ambulance Request, Provider, Destination, Departure Time and Arrival Confirmation are material business facts identified by L3-G-164.
- G06 records institutional coordination; it does not own external ambulance operations.
- H may support configured emergency notifications.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-164-BH-001 | When ambulance support is required for an Emergency Case, the system shall establish an ambulance request associated with the Emergency Case and patient context. | L3-G-164 §§3, 8, 9 |
| L4-G-164-BH-002 | When the request is established, the system shall preserve the applicable ambulance provider and destination information. | L3-G-164 §§9, 12 |
| L4-G-164-BH-003 | When ambulance departure occurs, the system shall record the departure time as a business fact when supported by the institutional process. | L3-G-164 §§9, 16 |
| L4-G-164-BH-004 | When arrival confirmation becomes available, the system shall distinguish confirmed arrival from an unconfirmed arrival condition. | L3-G-164 §§9, 11 |
| L4-G-164-BH-005 | The system shall preserve traceability between the ambulance coordination outcome and the originating Emergency Case. | L3-G-164 §§3, 15, 16 |

## 5. Cross-Domain & Event Behavior

G06 remains the canonical owner/publisher of Emergency Case and ambulance-coordination facts. H may consume configured emergency facts for notification. The authoritative L3 source does not expose a canonical ambulance event name; none is invented.

## 6. Failure / Rejection / Continuation

- If an ambulance provider cannot accept, the system shall retain the unresolved coordination condition and permit an authorised alternative-provider path.
- If destination is not established, the system shall not record an invented destination.
- If departure is not evidenced, the system shall not record departure as confirmed.
- If arrival confirmation is unavailable, the system shall preserve the unconfirmed condition rather than asserting arrival.
- If ambulance need is withdrawn by authorised medical reassessment, the system shall not represent a transport as having occurred.

## 7. Audit & Evidence Behavior

The system shall preserve Emergency Case, Ambulance Request, provider, destination, departure time where applicable, arrival confirmation where available, responsible healthcare actor and resulting coordination outcome.

## 8. Observable Completion Conditions

Completion is observable when the applicable ambulance coordination outcome has been recorded with truthful evidence and remains linked to the Emergency Case. External ambulance fleet operations are not represented as G06-owned outcomes.

## 9. Traceability & Provenance

**Parent:** L3-G-164 — Emergency Case to Ambulance Coordination  
**Primary verification source:** L2-G06 — Institutional Health Services & Medical Workflows  
**L3 anchors:** §§3, 4, 5, 8, 9, 11, 12, 13, 15, 16

## 10. Open Issues / Deferred Design

The canonical ambulance event name and exact arrival evidence are not exposed in the retrieved authoritative L2 content and require verification before Architecture Approval/Lock. Technical communication and implementation mechanics remain deferred.