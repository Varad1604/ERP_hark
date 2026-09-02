# Prescription to Medicine Dispensing

## 1. Specification Metadata

**Specification ID:** L4-G-166  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-166 — Prescription to Medicine Dispensing  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-166; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

the system shall govern truthful institutional medicine dispensing against a valid Prescription. It begins when a valid prescription is ready for dispensing and ends when the dispensing outcome is recorded and the prescription lifecycle is advanced only according to authoritative business rules.

## 3. Required Business Context

- Every Medicine Dispense references one valid Prescription.
- Every Prescription belongs to one Medical Encounter.
- Prescribed medicines, dosage, duration and instructions are authoritative prescription facts.
- Medicine Dispense records dispensed medicines, quantity, dispensing date and dispensing professional.
- G06 owns Prescription and Medicine Dispense; E supplies personnel context; H may support notifications.
- Prescription lifecycle is `Created → Issued → Dispensed → Completed`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-166-BH-001 | When a valid Prescription is ready for dispensing, the system shall verify its association with exactly one Medical Encounter and the patient context required for dispensing. | L3-G-166 §§7, 9 |
| L4-G-166-BH-002 | The system shall evaluate prescribed medicines against the applicable institutional availability and dispensing conditions before recording an issue. | L3-G-166 §§3, 9, 12 |
| L4-G-166-BH-003 | When dispensing is permitted, the system shall record the dispensed medicine(s), quantity, dispensing date and dispensing professional. | L3-G-166 §§3, 9, 15 |
| L4-G-166-BH-004 | The system shall associate each Medicine Dispense outcome with the valid Prescription. | L3-G-166 §§12, 16 |
| L4-G-166-BH-005 | When the dispensing outcome is established, the system shall advance the Prescription only according to its authoritative lifecycle and shall not imply dispensing merely from prescription creation. | L3-G-166 §§13, 16 |
| L4-G-166-BH-006 | The system shall preserve the dispensing outcome as part of the patient's healthcare history. | L3-G-166 §§3, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-166-BH-007 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Institutional Health Services (G06) ownership. | L3-G-166 §14 |

## 6. Failure / Rejection / Continuation

- If the Prescription is invalid or lacks a valid Medical Encounter, the system shall prevent dispensing against it.
- If a prescribed medicine is unavailable, the system shall not record it as dispensed and shall support the separate availability-exception path.
- If dispensing authority is unavailable, the system shall not represent an unauthorised issue as completed.
- If dispensing evidence is incomplete, the system shall retain the unresolved business condition rather than asserting a completed issue.
- Institution-specific partial-dispensing or substitution rules shall be applied only where authoritatively defined.

## 7. Audit & Evidence Behavior

The system shall preserve Prescription and Medical Encounter references, patient context, prescribed and dispensed medicines, quantity, dispensing date, dispensing professional, dispensing outcome and relevant lifecycle transition.

## 8. Observable Completion Conditions

Completion is observable when the permitted medicine issue is truthfully recorded against a valid Prescription, the relevant dispensing evidence is preserved, and the Prescription lifecycle is positioned according to the authoritative definition. Unavailable medicines are not falsely represented as dispensed.

## 9. Traceability & Provenance

**Parent:** L3-G-166 — Prescription to Medicine Dispensing  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
