# Medicine Availability Exception to Alternative Medical Action

## 1. Specification Metadata

**Specification ID:** L4-G-167  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-167 — Medicine Availability Exception to Alternative Medical Action  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-167; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall preserve a prescribed-medicine availability exception, prevent false dispensing, and record only an authorised alternative medical action or explicit unresolved healthcare outcome under G06 governance.

## 3. Required Business Context

- A valid Prescription and associated Medical Encounter exist.
- A prescribed medicine cannot be dispensed as intended.
- Patient context is authoritative.
- An authorised medical professional can establish any alternative medical action.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-167-BH-001 | Identify the affected Prescription, medicine, Medical Encounter and patient and record the authoritative availability exception. | L3-G-167 §§3,7–9 |
| L4-G-167-BH-002 | Prevent the unavailable medicine from being represented as dispensed and preserve the original Prescription as historically accurate. | L3-G-167 §§9,12–13,16 |
| L4-G-167-BH-003 | Refer the exception to an authorised medical professional and record only the alternative action established through professional judgement. | L3-G-167 §§9,12,15 |
| L4-G-167-BH-004 | If no acceptable alternative is established, preserve the explicit unresolved/next-care outcome rather than inventing a substitute medicine or treatment. | L3-G-167 §§10–11,16 |
| L4-G-167-BH-005 | If the outcome requires a new/revised Prescription, follow-up or Referral, hand off to the corresponding authoritative G06 workflow without absorbing its lifecycle. | L3-G-167 §§9–10,13 |
| L4-G-167-BH-006 | Preserve the exception and resulting healthcare action in the patient's auditable healthcare history. | L3-G-167 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-167-BH-007 | The exact canonical medicine-availability-exception event name is not exposed in the Locked parent. No `MedicineUnavailable`, `MedicineAvailabilityException` or substitute event shall be invented. G06 remains owner; E supplies professional context and H owns notification delivery. | L3-G-167 §§5,14,17 |

## 6. Failure / Rejection / Continuation

Do not initiate alternative action on an unverified shortage, without appropriate medical authority, or against unresolved patient/encounter context. Procurement, supplier management or enterprise pharmacy processes shall not be silently introduced.

## 7. Audit & Evidence Behavior

Preserve original Prescription, affected medicine, availability condition, Medical Encounter/patient, responsible professional, alternative action or unresolved outcome and relevant lifecycle evidence.

## 8. Observable Completion Conditions

Completion occurs when the exception is authoritatively recorded and either an authorised alternative medical action or explicit unresolved healthcare outcome is established, with no false dispensing fact created.

## 9. Traceability & Provenance

**Parent:** L3-G-167 — Medicine Availability Exception to Alternative Medical Action  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No universal substitution/therapeutic-equivalence policy, canonical availability-exception event or universal terminal unresolved state is defined upstream. L4 does not invent them. Technical inventory/procurement mechanisms are outside scope/deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
