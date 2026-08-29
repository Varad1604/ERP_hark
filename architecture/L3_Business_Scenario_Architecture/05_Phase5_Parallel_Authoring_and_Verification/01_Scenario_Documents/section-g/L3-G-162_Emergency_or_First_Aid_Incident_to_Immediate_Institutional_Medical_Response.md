# Emergency or First-Aid Incident to Immediate Institutional Medical Response

**Scenario ID:** L3-G-162  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-162 |
| Canonical Title | Emergency or First-Aid Incident to Immediate Institutional Medical Response |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Execute immediate institutional first-aid and medical emergency response for a student, employee, or visitor suffering an acute medical event on campus, stabilizing the individual and determining the need for on-site care or external emergency escalation.

---

## 3. Business Outcome

The patient receives immediate, documented medical triage and stabilization at the institutional health facility or on-scene. An authoritative Emergency Encounter record is established in the **Stabilized** or **Referred** state, and critical stakeholder notifications are dispatched.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins upon notification of an acute medical incident or arrival of an emergency patient at the Campus Health Centre.

### 4.2 End Boundary
The scenario concludes when primary on-site medical stabilization is complete and an institutional medical disposition decision (discharge, observation, or external ambulance transfer) is recorded.

### 4.3 In Scope
- Immediate emergency intake and clinical triage by qualified medical personnel.
- Delivery of on-scene first-aid, resuscitation, or clinical stabilization.
- Rapid patient identity linkage against Student/Employee registries (or anonymous emergency context).
- Generation of the Emergency Encounter record.

### 4.4 Out of Scope
- Specialized tertiary surgical interventions (handled by external hospital network).
- Long-term occupational health rehabilitation.
- Health insurance claim processing and final financial billing.

---

## 5. Ownership and Participating Domains

- **Primary Owner:** Institutional Health Services (Section G) owns clinical triage, emergency protocols, and medical encounter documentation.
- **Participating Domains:**
  - *Student Lifecycle (Section C):* Consumes emergency incident alerts for student patients (parent/guardian notification protocol).
  - *People & Workforce (Section E):* Consumes emergency alerts for staff patients.
  - *Campus Security & Infrastructure (Section G):* Coordinates rapid ambulance gate access and campus crowd control.

---

## 6. Business Actors, Roles and Responsibilities

- **Campus Medical Officer / Emergency Nurse:** Conducts triage, administers immediate first-aid/treatment, and authorizes clinical disposition.
- **First-Aid Responder / Security Officer:** Delivers initial on-scene care and alerts the Health Centre.
- **Emergency Contact / Guardian:** Contacted in accordance with institutional medical emergency protocols.

---

## 7. Preconditions and Entry Criteria

1. An acute medical condition, physical trauma, or medical distress incident occurs on campus grounds.
2. Health Centre emergency response team is operational.

---

## 8. Trigger

An emergency distress call is received by the Health Centre or an acute patient is admitted directly to the emergency triage room.

---

## 9. Main Business Flow

1. First-Aid Responder alerts the Health Centre of a medical emergency.
2. Campus Medical Officer initiates immediate triage and clinical assessment.
3. Medical team administers life-stabilization protocols (first-aid, oxygen, wound care, medication).
4. Patient identity is captured via fast-track identification or assigned temporary emergency context.
5. Medical Officer records clinical observations, vitals, and administered treatments in the Emergency Encounter log.
6. Medical Officer determines clinical disposition: Patient stabilized for observation or cleared for discharge.
7. Health Services publishes the `MedicalEmergencyIncidentRecorded` business event.
8. Relevant institutional stakeholders (Department Head, Proctor, or Family Contact) are securely notified per privacy regulations.

---

## 10. Alternate Business Flows

- **Critical Escalation to External Hospital:** If the condition requires tertiary care, the Medical Officer triggers external ambulance coordination (hand-off to L3-G-164).
- **Unidentified Patient Protocol:** If patient is unconscious without credentials, an anonymous emergency record is opened and reconciled once identity is confirmed.

---

## 11. Failure, Interruption and Business Recovery

- **Health Centre Equipment / Medication Deficit:** If critical medical supplies are depleted, emergency transfer to nearest partner hospital is immediately executed.
- **Privacy Barrier During Emergency:** Emergency medical care proceeds unconditionally prior to consent verification in life-threatening scenarios.

---

## 12. Business Rules and Decision Constraints

- **BR-G06-01 (Care Without Precondition):** Immediate medical emergency triage must never be delayed for fee verification, administrative clearance, or credential validation.
- **BR-G06-02 (Confidentiality & Need-to-Know):** Emergency medical clinical details remain strictly confidential; institutional notifications contain only operational status facts.
- **BR-G06-03 (Mandatory Medical Officer Sign-Off):** All emergency encounters require formal electronic or written sign-off by a licensed Medical Officer.

---

## 13. State and Lifecycle Impact

- **Medical Encounter Lifecycle:** Created in `Active` → transitions to `Stabilized`, `AdmittedForObservation`, or `TransferredToExternalCare`.

---

## 14. Business Events and Cross-Domain Collaboration

- **Emits:** `MedicalEmergencyIncidentRecorded` (Publisher: Institutional Health Services).
  - *Payload/Data:* Encounter ID, Patient ID (or Temp ID), Incident Location, Triage Category, Timestamp, Disposition Code.
  - *Consumers:* Student Lifecycle (Section C), People & Workforce (Section E), Campus Security.

---

## 15. Business Authorization, Approval and Audit Obligations

- **Authorization Role:** Licensed Medical Officer.
- **Audit Obligation:** Immutable timestamped clinical log of triage category, administered medications, vital signs, and disposition decision.

---

## 16. Completion Criteria and Postconditions

- Patient has received primary clinical stabilization.
- Emergency encounter record is formally documented and signed off.
- `MedicalEmergencyIncidentRecorded` business event is emitted.

---

## 17. Open Issues, Assumptions and Deferred Questions

- **Assumptions:** On-campus health centre maintains standard basic life support (BLS) equipment and stock medications.
- **Open Issues:** None.
