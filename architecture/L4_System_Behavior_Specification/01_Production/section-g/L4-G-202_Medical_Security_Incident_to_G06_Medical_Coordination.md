# Medical Security Incident to G06 Medical Coordination

## 1. Specification Metadata


**Specification ID:** ** `L4-G-202`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-202` — Medical Security Incident to G06 Medical Coordination  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-202_Medical_Security_Incident_to_G06_Medical_Coordination.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support the Security Operations portion of a medical-security incident from distress intake through coordinated first response, priority security transit, authoritative G06 medical handover, and closure of the Security Operations incident.

The behavior begins when a medical distress call is logged at the security control room or gate post. It ends when the Medical Security Incident Aggregate reaches `PATIENT_TRANSFERRED_AND_CLOSED`.

This specification does not own diagnosis, treatment, clinical decisions, patient medical records, ambulance technology, or G06 medical lifecycle outcomes.

---

## 3. Required Business Context

- Active Security Office.
- Medical distress incident.
- Security First Responder.
- Authoritative G06 medical participation through the Campus Medical Officer.
- Ambulance / patient-transfer context where required.
- Parent-defined priority green-corridor rule.
- Required joint medical-security handover.
- Security Operations ownership of the Medical Security Incident Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-202-BH-001` | When a medical distress call is accepted by Security Operations, the system shall establish the Medical Security Incident Aggregate in `MEDICAL_ALERT_ACTIVE`. | L3-G-202 §§4, 9, 13 |
| `L4-G-202-BH-002` | The system shall preserve the Security Operations first-response context required for coordinated handover to G06 without representing security personnel as the medical authority. | L3-G-202 §§2, 5–6 |
| `L4-G-202-BH-003` | The system shall support the parent-defined priority green-corridor security disposition for authorised patient or ambulance movement. | L3-G-202 §§2–3, 12 |
| `L4-G-202-BH-004` | The system shall recognize the Campus Medical Officer / G06 as the authoritative medical participant in the joint coordination and shall not re-own clinical facts or decisions. | L3-G-202 §§2, 5–6 |
| `L4-G-202-BH-005` | Before Security Operations closure, the system shall preserve evidence of the joint medical-security handover required by the parent scenario. | L3-G-202 §§3–4 |
| `L4-G-202-BH-006` | The system shall prevent normal closure while the patient-transfer or authoritative G06 handover remains unresolved. | L3-G-202 §§3–4, 16 |
| `L4-G-202-BH-007` | On successful governed transfer, the system shall transition the Medical Security Incident Aggregate from `MEDICAL_ALERT_ACTIVE` to `PATIENT_TRANSFERRED_AND_CLOSED`. | L3-G-202 §§9, 13, 16 |
| `L4-G-202-BH-008` | Where a parent-supported supervisory exception applies, the system shall preserve the Security Shift Supervisor authority and justification without altering G06 medical ownership. | L3-G-202 §§10, 15 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-202-BH-009` | The system shall preserve the cross-domain handover between Security Operations and G06 while keeping medical facts and medical authority with G06. | L3-G-202 §§2–6 |
| `L4-G-202-BH-010` | When the Security Operations incident reaches `PATIENT_TRANSFERRED_AND_CLOSED`, the system shall produce the canonical `Medical_Security_Coordination_Completed` business event on behalf of Security Operations. | L3-G-202 §§14, 16 |

No clinical workflow, ambulance dispatch implementation, message schema, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If Security Operations cannot establish the required medical handover, the incident shall remain unresolved from the Security Operations perspective.
- If the green-corridor condition cannot be safely established, the system shall preserve the unresolved security disposition for authorised escalation.
- If G06 has assumed medical responsibility but Security Operations transfer evidence is incomplete, the system shall not silently mark the security incident closed.
- Technical communications or dispatch failures are outside L4 and are not modeled as implementation recovery behavior.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- medical distress intake;
- Security First Responder participation;
- G06 medical participant;
- green-corridor disposition;
- ambulance/patient transfer context where applicable;
- joint medical-security handover;
- supervisory exception where applicable;
- resulting `PATIENT_TRANSFERRED_AND_CLOSED` position;
- publication of `Medical_Security_Coordination_Completed`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Medical Security Incident Aggregate is `PATIENT_TRANSFERRED_AND_CLOSED`;
- authoritative G06 medical handover is attributable;
- the Security Operations green-corridor and transfer obligations are complete;
- `Medical_Security_Coordination_Completed` has been produced;
- medical treatment and clinical outcomes remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-202` — Medical Security Incident to G06 Medical Coordination  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Cross-domain participant:** G06 — Institutional Health Services  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

This specification explicitly preserves G06 medical ownership.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Ambulance dispatch technology, clinical-system integration, persistence, event transport, audit implementation, and UI/UX remain deferred.
