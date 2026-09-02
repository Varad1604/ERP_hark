# Prescription to Medicine Dispensing

## 1. Specification Metadata

**Specification ID:** L4-G-166  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-166 — Prescription to Medicine Dispensing  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-166; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern truthful institutional medicine dispensing against a valid G06 Prescription, recording only medicines actually authorised and issued and advancing the Prescription lifecycle only according to authoritative business conditions.

## 3. Required Business Context

- Every Medicine Dispense references one valid Prescription.
- Every Prescription belongs to one Medical Encounter.
- Prescription details include prescribed medicines, dosage, duration and instructions.
- Medicine Dispense records dispensed medicines, quantity, dispensing date and dispensing professional.
- Prescription lifecycle is `Created → Issued → Dispensed → Completed`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-166-BH-001 | Validate the Prescription, its Medical Encounter/patient association and appropriate dispensing lifecycle position. | L3-G-166 §§7,9,12–13 |
| L4-G-166-BH-002 | Evaluate prescribed medicines against applicable G06 institutional availability and dispensing conditions before recording issue. | L3-G-166 §§3,9,12 |
| L4-G-166-BH-003 | When dispensing is permitted, record the dispensed medicine(s), quantity, dispensing date and authorised dispensing professional. | L3-G-166 §§3,9,15 |
| L4-G-166-BH-004 | Associate each Medicine Dispense with the valid Prescription and preserve medicine issue history. | L3-G-166 §§9,12,16 |
| L4-G-166-BH-005 | Advance the Prescription only through the authoritative `Created → Issued → Dispensed → Completed` lifecycle and never infer `Dispensed` from prescription creation alone. | L3-G-166 §§13,16 |
| L4-G-166-BH-006 | Route unavailable medicines to the separate availability-exception path and do not falsely record them as dispensed. | L3-G-166 §§3,10–11,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-166-BH-007 | The exact canonical medicine-dispensing event name is not exposed in the Locked parent. No `MedicineDispensed` or substitute event shall be invented. G06 owns Prescription and Medicine Dispense; E provides personnel context and H owns notification delivery. | L3-G-166 §§5,14,17 |

## 6. Failure / Rejection / Continuation

Prevent dispensing against an invalid Prescription/Encounter or without authorised dispensing context. Preserve unresolved evidence rather than asserting completion. Partial dispensing or substitution shall occur only where authoritative policy permits it.

## 7. Audit & Evidence Behavior

Preserve Prescription, Medical Encounter, patient, prescribed/dispensed medicines, quantity, date, dispensing professional, outcome and lifecycle transition.

## 8. Observable Completion Conditions

Completion occurs when the permitted medicine issue is truthfully recorded against the valid Prescription, unavailable medicines are not falsely represented as dispensed, and the Prescription is positioned according to its authoritative lifecycle.

## 9. Traceability & Provenance

**Parent:** L3-G-166 — Prescription to Medicine Dispensing  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The canonical dispensing event, universal partial-dispensing/substitution rules and precise `Issued`/`Dispensed` semantics remain unresolved upstream. Enterprise pharmacy/procurement and technical inventory implementation are outside scope/deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
