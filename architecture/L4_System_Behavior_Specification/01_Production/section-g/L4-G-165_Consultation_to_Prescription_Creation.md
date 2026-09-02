# Consultation to Prescription Creation

## 1. Specification Metadata

**Specification ID:** L4-G-165  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-165 — Consultation to Prescription Creation  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-165; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall support creation of a valid G06 Prescription when an authorised medical professional determines through consultation that prescription treatment is required, ending before medicine dispensing.

## 3. Required Business Context

- A valid Medical Encounter exists.
- The consultation has reached the treatment decision.
- An authorised medical professional is established from E-owned personnel context.
- Required prescription details can be established through professional judgement.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-165-BH-001 | Confirm the patient, Medical Encounter and responsible medical professional before prescription creation. | L3-G-165 §§7–9 |
| L4-G-165-BH-002 | Record prescribed medicines, dosage, duration and instructions as established by the authorised professional. | L3-G-165 §§3,9,12 |
| L4-G-165-BH-003 | Associate every Prescription with exactly one Medical Encounter and preserve responsible professional context. | L3-G-165 §§5,9,12 |
| L4-G-165-BH-004 | Create the Prescription only when the clinical decision and required prescription information are complete; do not substitute system-defined clinical judgement. | L3-G-165 §§9,11–12,15 |
| L4-G-165-BH-005 | Preserve prescription creation as healthcare history and make the Prescription available to the separate medicine-dispensing workflow. | L3-G-165 §§9,16 |
| L4-G-165-BH-006 | Do not represent medicine as dispensed merely because a Prescription has been created. | L3-G-165 §§3,4,13,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-165-BH-007 | The exact canonical prescription-creation event name is not exposed in the Locked parent; no `PrescriptionCreated` or substitute event shall be invented. G06 owns Prescription; E supplies personnel context only. | L3-G-165 §§5,14,17 |

## 6. Failure / Rejection / Continuation

Prevent creation against an invalid encounter, missing professional authority or incomplete prescription information. If no prescription is required, do not create one. If the clinical decision changes before creation, preserve the latest authoritative professional decision.

## 7. Audit & Evidence Behavior

Preserve Medical Encounter, patient context, responsible professional, medicines, dosage, duration, instructions, creation outcome and relevant lifecycle position.

## 8. Observable Completion Conditions

Completion occurs when a valid G06 Prescription exists, is associated with exactly one Medical Encounter, contains the required prescription information and is available for downstream dispensing without implying dispense completion.

## 9. Traceability & Provenance

**Parent:** L3-G-165 — Consultation to Prescription Creation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The exact canonical prescription event and precise `Created`/`Issued` transition semantics remain unresolved upstream. Clinical drug/dosage rules are not invented. Technical prescription generation/storage is deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
