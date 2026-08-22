# Walk-in Consultation to Completed Medical Encounter

**Scenario ID:** L3-G-160  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional walk-in consultation outcome from a consultation-ready patient through clinical assessment, diagnosis, treatment, optional prescription/follow-up, and consultation completion.

The locked L2 states that walk-in consultations are the default operational model and defines the consultation workflow:

`Patient Registered → Clinical Assessment → Diagnosis → Treatment → Prescription (If Required) → Consultation Closed`.

Consultation information may include Consultation Number, Healthcare Professional, Consultation Type, Symptoms, Diagnosis, Treatment Notes and Follow-up Recommendation. Consultation types and follow-up policies remain configurable. fileciteturn257file0

## 3. Business Outcome

A walk-in healthcare consultation has been clinically completed, the Medical Encounter has reached its completed/closed business condition, and any required downstream healthcare outcome such as prescription or follow-up recommendation has been established within its own authoritative workflow.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A patient is registered and ready for consultation.

### 4.2 End Boundary

The consultation is clinically completed and the consultation lifecycle reaches its authoritative closed outcome.

### 4.3 In Scope

- Walk-in consultation initiation.
- Clinical assessment.
- Symptoms/context.
- Diagnosis.
- Treatment.
- Prescription creation where required.
- Follow-up recommendation where required.
- Consultation completion/closure.
- Audit/event publication.
- Patient/employee/student context as applicable.

### 4.4 Out of Scope

- Patient identity ownership.
- Medical history ownership outside consultation use.
- Medicine dispensing.
- Referral completion.
- Emergency treatment.
- Hospital management.
- Financial accounting/billing.
- Notification delivery.
- Technical implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the Medical Encounter and consultation lifecycle.

### Section C — Student Lifecycle

C remains authoritative for student identity/status.

### Section E — People & Workforce

E remains authoritative for healthcare professional/personnel information.

### Section H — Notifications

H owns notification delivery where configured.

Cross-domain participation does not transfer consultation ownership.

## 6. Business Actors, Roles and Responsibilities

### Doctor / Authorised Medical Professional

Performs clinical assessment, diagnosis and treatment within professional authority.

### Medical Staff

Support consultation operations within configured responsibilities.

### Patient

Participates in the healthcare consultation and provides relevant symptoms/history information.

### Section E

Provides authoritative healthcare professional/personnel context.

### Section H

Delivers notifications when a canonical healthcare event requires notification.

The L2 explicitly states that medical professionals retain operational authority and the ERP supports recording, governance and coordination without replacing clinical judgement. fileciteturn244file0

## 7. Preconditions and Entry Criteria

1. Patient is registered.
2. Consultation-ready context exists.
3. Healthcare professional is available and authorised.
4. Relevant medical history has been made available.
5. Applicable consultation type/policy is established.
6. The consultation is not already closed.

## 8. Trigger

A registered patient presents for the walk-in consultation.

## 9. Main Business Flow

### Phase A — Consultation Start

1. Identify the patient and Medical Encounter.
2. Confirm consultation readiness.
3. Identify the responsible healthcare professional.
4. Establish the consultation type as Walk-in Consultation.
5. Record the consultation context.

### Phase B — Clinical Assessment

6. Assess the patient's presenting symptoms and relevant clinical context.
7. Consider the available medical history.
8. Establish the clinical assessment.
9. Record the assessment in the Medical Encounter.

### Phase C — Diagnosis

10. Establish the diagnosis based on professional clinical judgement.
11. Record the diagnosis.
12. Determine the appropriate treatment.

### Phase D — Treatment

13. Record treatment provided/recommended.
14. Determine whether a prescription is required.
15. Determine whether follow-up is required.
16. Determine whether another healthcare workflow, such as referral or observation, is appropriate.

### Phase E — Downstream Outcomes

17. If prescription is required, initiate the Prescription workflow.
18. If follow-up is required, establish the follow-up recommendation.
19. If referral or observation is required, hand off to the applicable authoritative workflow without absorbing its lifecycle.

### Phase F — Consultation Completion

20. Confirm required consultation information is recorded.
21. Complete/close the consultation.
22. Publish `ConsultationCompleted`.
23. Preserve the auditable Medical Encounter history.
24. End the scenario.

The L2 defines `ConsultationStarted` and `ConsultationCompleted` events and requires every significant healthcare activity to publish immutable domain events. fileciteturn250file0

## 10. Alternate Business Flows

### 10.1 Prescription Required

Prescription creation begins through the separate prescription workflow. The consultation does not dispense the medicine.

### 10.2 No Prescription Required

The consultation proceeds directly toward completion after treatment is recorded.

### 10.3 Follow-up Recommended

A `FollowUpRecommended` business outcome is established; subsequent follow-up activity remains within its own scenario.

### 10.4 Referral Required

Referral is recommended/initiated according to the referral workflow. Referral completion is not included here.

### 10.5 Observation Required

Observation is handed to Observation Care; this scenario does not absorb the observation lifecycle.

### 10.6 Appointment-Configured Consultation

Appointment scheduling is optional through configuration, but walk-in remains the default operational model. The scenario remains a walk-in consultation only when the canonical trigger is a walk-in.

### 10.7 Clinical Assessment Requires Emergency Handling

If the patient is determined to require emergency response, the emergency workflow governs that outcome rather than continuing an ordinary consultation as though no emergency existed.

## 11. Failure, Interruption and Business Recovery

### 11.1 Patient Context Cannot Be Confirmed

Consultation does not proceed against an invented patient.

### 11.2 Healthcare Professional Context Cannot Be Established

The consultation remains unresolved until an authorised professional can perform it.

### 11.3 Clinical Assessment Incomplete

The encounter does not close with an invented diagnosis or treatment.

### 11.4 Diagnosis Not Established

The encounter remains clinically unresolved.

### 11.5 Treatment Requires Another Workflow

The appropriate downstream workflow is invoked at the business boundary.

### 11.6 Consultation Cannot Be Closed

Required business information remains outstanding; no fabricated completion is recorded.

### 11.7 Recovery

The consultation resumes from the last unresolved business stage once the required clinical/operational condition is established.

No technical retry, queue, rollback, API, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The L2 establishes:

- Walk-in consultation is the default operational model.
- Appointment scheduling is optional through configuration.
- Consultation types remain configurable.
- Follow-up policies remain configurable.
- Medical professionals retain operational authority.
- The ERP supports recording and coordination without replacing clinical judgement. fileciteturn257file0turn244file0

G06 invariants include:

- Every Medical Encounter references exactly one Patient.
- Every Prescription belongs to one Medical Encounter.
- Referral references one Medical Encounter.
- Medical certificates reference one authorised Medical Encounter.
- Every lifecycle transition is audited. fileciteturn253file0

No local diagnosis, treatment, prescription or follow-up rule is invented.

## 13. State and Lifecycle Impact

The Medical Encounter lifecycle is:

`Registered → Consultation → Treatment → Completed → Closed`. fileciteturn251file0

The consultation workflow is:

`Patient Registered → Clinical Assessment → Diagnosis → Treatment → Prescription (If Required) → Consultation Closed`. fileciteturn257file0

This scenario owns the consultation completion boundary but does not absorb prescription, medicine-dispensing, observation or referral lifecycles.

## 14. Business Events and Cross-Domain Collaboration

G06 defines:

- `ConsultationStarted` — Consultation initiated.
- `ConsultationCompleted` — Consultation completed.
- `FollowUpRecommended` — Follow-up advised. fileciteturn250file0

G06 is the canonical publisher.

H may consume business events for configured notification delivery, but notification delivery remains H-owned. C/E participate through identity/personnel context and do not acquire Medical Encounter ownership.

## 15. Business Authorization, Approval and Audit Obligations

The L2 states that healthcare authorization is configurable and identifies Doctor as the typical authorization for conducting consultation, while medical confidentiality is governed by need-to-know, role-based access and institutional privacy policies. fileciteturn251file0

Clinical judgement remains with the medical professional.

Audit evidence should preserve:

- Patient/encounter context.
- Healthcare professional.
- Consultation type.
- Symptoms.
- Assessment.
- Diagnosis.
- Treatment notes.
- Follow-up recommendation.
- Consultation completion.
- Relevant downstream outcomes.

Every lifecycle transition must be audited. fileciteturn253file0

No technical access control/IAM implementation is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Walk-in consultation identified.
2. Patient and Medical Encounter confirmed.
3. Healthcare professional established.
4. Clinical assessment completed.
5. Diagnosis established.
6. Treatment recorded.
7. Prescription/follow-up/referral/observation path determined where required.
8. Consultation completed/closed.
9. `ConsultationCompleted` published by G06.
10. Audit history preserved.

### Postconditions

- Medical Encounter has completed consultation outcome.
- Clinical history contains the consultation outcome.
- Any prescription/referral/observation activity follows its own authoritative lifecycle.
- Patient/student/employee identity ownership remains unchanged.
- Notification delivery, if applicable, remains H-owned.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 leaves consultation types and follow-up policies configurable.
2. It does not define a universal clinical assessment checklist.
3. It does not define a universal diagnosis/treatment decision matrix.

### Assumptions

- Medical professionals exercise clinical judgement.
- Prescription, referral and observation are separate authoritative workflows.
- `ConsultationCompleted` is the canonical completion event.

### Deferred Questions

Confirm any institution-specific consultation documentation checklist before Architecture Approval/Lock.
