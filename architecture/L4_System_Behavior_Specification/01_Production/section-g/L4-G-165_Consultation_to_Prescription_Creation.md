# Consultation to Prescription Creation

## 1. Specification Metadata

**Specification ID:** L4-G-165  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-165 — Consultation to Prescription Creation  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-165; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

the system shall govern creation of a valid G06 Prescription when an authorised healthcare professional determines through consultation that prescription treatment is required. The behavior ends when the prescription is available for the downstream dispensing workflow; dispensing itself is out of scope.

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

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-165-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Institutional Health Services (G06) ownership. | L3-G-165 §14 |

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
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
