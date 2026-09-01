# Consultation to Prescription Creation

## 1. Specification Metadata

**Specification ID:** L4-G-165  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-165 — Consultation to Prescription Creation  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Locked L3-G-165; Locked L2-G06; Locked L1 Business Domain Architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support creation of a valid G06 Prescription when an authorised healthcare professional determines through consultation that prescription treatment is required. The behavior ends when the prescription is available for the downstream dispensing workflow; dispensing itself is out of scope.

## 3. Required Business Context

- Medical Encounter is the authoritative healthcare context.
- Prescription is G06-owned and belongs to exactly one Medical Encounter.
- Prescription information includes prescribed medicines, dosage, duration and instructions.
- E remains authoritative for healthcare-professional/personnel context.
- Clinical judgement remains with the authorised medical professional.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-165-BH-001 | When consultation determines that prescription treatment is required, the system shall establish the prescription requirement within the relevant Medical Encounter context. | L3-G-165 §§3, 8, 9 |
| L4-G-165-BH-002 | When prescription information is established, the system shall associate the Prescription with exactly one Medical Encounter and preserve the responsible healthcare-professional context. | L3-G-165 §§5, 9, 12 |
| L4-G-165-BH-003 | The system shall record prescribed medicines, dosage, duration and instructions required for the prescription outcome. | L3-G-165 §§3, 4, 12 |
| L4-G-165-BH-004 | When required prescription information is complete, the system shall create the G06 Prescription without replacing clinical judgement with system-defined clinical rules. | L3-G-165 §§9, 12 |
| L4-G-165-BH-005 | The system shall make the created prescription available for the separate medicine-dispensing workflow without representing medicine as dispensed. | L3-G-165 §§3, 9, 16 |

## 5. Cross-Domain & Event Behavior

G06 owns and publishes Prescription business facts. E supplies authoritative professional context and does not acquire prescription ownership. The exact canonical prescription-creation event is not exposed in the retrieved L3/L2 material; no event name is invented.

## 6. Failure / Rejection / Continuation

- If Medical Encounter association is invalid, the system shall prevent creation against an invalid encounter.
- If professional authority cannot be established, the system shall prevent prescription creation.
- If required prescription information is incomplete, the system shall retain the unresolved business condition until the responsible professional establishes the required information.
- If clinical judgement changes before creation, the system shall not create a prescription based on an obsolete decision.
- If no prescription is required, the system shall not create one merely to complete the scenario.

## 7. Audit & Evidence Behavior

The system shall preserve Medical Encounter, patient context, responsible professional, prescription details, medicines, dosage, duration, instructions, creation outcome and relevant lifecycle transition.

## 8. Observable Completion Conditions

Completion is observable when a valid G06 Prescription is created, associated with exactly one Medical Encounter, contains the required prescription information, and is available for downstream dispensing. No dispensing outcome is implied by creation.

## 9. Traceability & Provenance

**Parent:** L3-G-165 — Consultation to Prescription Creation  
**Primary verification source:** L2-G06 — Institutional Health Services & Medical Workflows  
**L3 anchors:** §§3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16

## 10. Open Issues / Deferred Design

The exact Prescription-created event and the authoritative `Created`/`Issued` distinction require upstream verification before Architecture Approval/Lock. Technical prescription storage, interfaces, access-control implementation and downstream dispensing mechanics remain deferred.