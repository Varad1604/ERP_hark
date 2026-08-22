# Patient Identification to Healthcare Visit Registration

**Scenario ID:** L3-G-157  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E  
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

This scenario governs the complete institutional outcome in which an individual seeking healthcare is identified, the relevant institutional identity is validated, applicable healthcare context is established, medical history is retrieved where available, and the healthcare visit is registered and becomes ready for consultation.

The locked L2 defines Patient Registration as an Institutional Health Services responsibility. It explicitly states that Institutional Health Services references existing institutional identities and never owns person identities. The registration workflow is:

`Patient Identified → Identity Validation → Medical History Retrieved → Visit Registered → Consultation Ready`.

Validation may consider Institutional Identity, Campus, Active Status, Emergency Classification and Healthcare Eligibility, with validation rules configurable. fileciteturn243file0turn254file0

## 3. Business Outcome

The individual has been correctly identified against authoritative institutional identity information, the applicable healthcare eligibility/context has been established, relevant medical history has been retrieved without changing its historical record, and a healthcare visit has been registered and is **Consultation Ready**.

The scenario does not perform the consultation itself.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An individual presents for institutional healthcare and is identified as a potential patient.

### 4.2 End Boundary

The healthcare visit is registered and reaches the `Consultation Ready` business condition.

### 4.3 In Scope

- Patient identification.
- Institutional identity validation.
- Student identity consumption from C where applicable.
- Employee identity/personnel context consumption from E where applicable.
- Campus and active-status validation.
- Emergency classification.
- Healthcare eligibility validation.
- Retrieval of existing medical history.
- Visit registration.
- Consultation readiness.

### 4.4 Out of Scope

- Ownership or creation of Student/Employee identity.
- Clinical diagnosis.
- Treatment.
- Prescription.
- Medicine dispensing.
- Emergency treatment.
- Medical history modification.
- Technical authentication/authorization.
- Database/API implementation.
- Notification delivery.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns:

- Patient Registration.
- Medical Encounter.
- Healthcare visit registration.
- Consultation readiness.
- Medical history as healthcare operational history.

The L2 states that Institutional Health Services owns Patient Registration and Medical History while referencing external institutional identities. fileciteturn243file0turn244file0

### Section C — Student Lifecycle

C remains authoritative for Student Identity, Student Status, Programme and related student information. G06 consumes this information.

### Section E — People & Workforce

E remains authoritative for employee/personnel information, including Doctors, Nurses, Medical Officers, Healthcare Staff and Employee Information. G06 performs operational healthcare assignment/consumption only. fileciteturn245file0

Cross-domain participation never transfers identity ownership to G06.

## 6. Business Actors, Roles and Responsibilities

### Healthcare Registration Staff / Medical Staff

Identify the individual, validate the healthcare registration context and register the visit.

### Patient

Provides/identifies the information necessary for healthcare registration.

### Student Lifecycle Authority — C

Provides authoritative student identity/status context when the patient is a student.

### People & Workforce Authority — E

Provides authoritative employee/personnel context when the patient is an employee.

No new business actor or technical role is invented.

## 7. Preconditions and Entry Criteria

1. The individual is seeking institutional healthcare.
2. A patient identity can be presented or otherwise established for validation.
3. The applicable institutional identity source can be consulted.
4. The relevant campus context can be determined.
5. Applicable healthcare eligibility rules are available.
6. Existing medical history can be retrieved where it exists.

## 8. Trigger

A person seeking institutional healthcare is identified for a new healthcare visit.

## 9. Main Business Flow

### Phase A — Patient Identification

1. Identify the individual seeking healthcare.
2. Determine whether the individual is represented by an institutional identity.
3. Determine the relevant patient category where configured, such as Student, Faculty, Employee, Guest or Visitor. fileciteturn248file0
4. Preserve the distinction between patient registration and person-identity ownership.

### Phase B — Identity and Eligibility Validation

5. Validate the institutional identity.
6. Where the patient is a student, consume authoritative student identity/status information from C.
7. Where the patient is an employee, consume authoritative employee information from E.
8. Establish the relevant campus.
9. Validate Active Status.
10. Consider Emergency Classification.
11. Evaluate Healthcare Eligibility.
12. Apply the configured validation rules.

### Phase C — Medical History Retrieval

13. Retrieve relevant existing medical history where available.
14. Preserve historical information as immutable.
15. Do not overwrite or reinterpret historical clinical information during registration.

The L2 states that medical history may include previous visits, allergies, chronic conditions, vaccinations, previous prescriptions and previous referrals, and that historical information shall remain immutable. fileciteturn247file0

### Phase D — Visit Registration

16. Create/register the healthcare visit within the G06 Patient Registration boundary.
17. Associate the visit with the validated patient.
18. Preserve the relevant campus and registration context.
19. Establish the visit as **Consultation Ready**.

### Phase E — Complete

20. End the scenario.

The subsequent consultation begins under the separate Consultation Management workflow.

## 10. Alternate Business Flows

### 10.1 Student Patient

C supplies authoritative student identity/status information; G06 uses it for healthcare registration without becoming owner of the student identity.

### 10.2 Employee Patient

E supplies authoritative employee/personnel information; G06 uses it for healthcare registration without becoming owner of employee identity.

### 10.3 Configured Guest/Visitor

Where patient categories permit guests/visitors, registration follows the applicable configured identity and eligibility rules. No external identity ownership model is invented.

### 10.4 Emergency Classification

If the registration is classified as an emergency, the emergency workflow may become the applicable downstream path. Patient Registration itself remains the boundary defined by the scenario.

### 10.5 Existing Medical History

Existing history is retrieved and remains immutable. Registration does not create duplicate historical facts.

### 10.6 No Prior Medical History

The visit can proceed without inventing historical records.

### 10.7 Healthcare Eligibility Requires Review

The visit remains unresolved until the applicable configurable eligibility decision is established.

## 11. Failure, Interruption and Business Recovery

### 11.1 Institutional Identity Not Found

The registration cannot be completed against an invented identity. The business case follows the applicable Patient Registration exception/alternative-identification scenario.

### 11.2 Student/Employee Status Cannot Be Established

G06 does not infer authoritative status. C/E remain the source owners.

### 11.3 Campus Cannot Be Established

Registration does not proceed against an invented campus.

### 11.4 Healthcare Eligibility Cannot Be Established

The registration remains unresolved until the applicable configurable validation outcome is established.

### 11.5 Medical History Retrieval Is Unavailable

No fabricated history is created. The registration outcome follows the applicable business exception policy.

### 11.6 Emergency Classification

Where the individual requires emergency handling, the emergency workflow provides the applicable response. G06 remains owner of the healthcare business process.

### 11.7 Recovery

Once authoritative identity/context is established, registration continues from the unresolved business boundary.

No technical retry, queue, rollback, API, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The L2 states:

- Institutional Health Services references existing institutional identities.
- It never owns person identities.
- Patient categories are configurable.
- Registration validation may consider Institutional Identity, Campus, Active Status, Emergency Classification and Healthcare Eligibility.
- Validation rules remain configurable. fileciteturn248file0

The following G06 invariants apply:

- Every Medical Encounter references exactly one Patient.
- Medical history is never deleted.
- Every lifecycle transition is audited. fileciteturn253file0

This scenario does not invent an eligibility matrix or identity-resolution rule.

## 13. State and Lifecycle Impact

The L2 registration workflow is:

`Patient Identified → Identity Validation → Medical History Retrieved → Visit Registered → Consultation Ready`. fileciteturn254file0

The broader Medical Encounter lifecycle is:

`Registered → Consultation → Treatment → Completed → Closed`. fileciteturn246file0

This scenario ends at registration/consultation readiness and does not execute the later consultation, treatment or closure lifecycle.

## 14. Business Events and Cross-Domain Collaboration

G06 defines the canonical business event:

**`PatientRegistered` — Patient visit registered.**

Institutional Health Services is the canonical publisher because it owns Patient Registration and Medical Encounter. fileciteturn250file0

C and E provide authoritative identity/context information but do not publish the G06 PatientRegistered event.

The event may support downstream notification/reporting/analytics consumers according to the upstream architecture; delivery mechanisms remain outside L3.

## 15. Business Authorization, Approval and Audit Obligations

Healthcare registration follows the applicable G06 business authorization policy. The L2 identifies configurable authorization, including typical operational roles such as Medical Staff for patient registration, while technical authentication/authorization infrastructure remains a Platform responsibility. fileciteturn251file0

The registration must preserve auditable business history, including the lifecycle transition and relevant business context.

Medical confidentiality and need-to-know principles apply to healthcare information.

No technical access control/IAM implementation is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Patient identified.
2. Authoritative identity validated.
3. Relevant C/E context consumed where applicable.
4. Campus/status/emergency/eligibility considerations evaluated.
5. Existing medical history retrieved where available.
6. Patient visit registered.
7. Visit reaches Consultation Ready.
8. PatientRegistered event is published by G06.
9. Registration transition is auditable.

### Postconditions

- A valid healthcare visit exists for exactly one patient.
- Relevant identity remains owned by its source domain.
- Medical history remains immutable.
- The visit is ready for the separate Consultation Management workflow.
- No clinical diagnosis or treatment has been performed by this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 makes validation rules configurable but does not provide a universal healthcare-eligibility decision matrix.
2. Patient categories beyond the listed examples remain configurable.
3. The exact business exception path for unidentified patients is not defined within this scenario.

### Assumptions

- C and E remain authoritative for student and employee identity/context.
- G06 remains owner of the healthcare visit/Medical Encounter.
- `PatientRegistered` is the canonical registration event.

### Deferred Questions

Confirm any institution-specific eligibility and alternative-identification policies before Architecture Approval/Lock.
