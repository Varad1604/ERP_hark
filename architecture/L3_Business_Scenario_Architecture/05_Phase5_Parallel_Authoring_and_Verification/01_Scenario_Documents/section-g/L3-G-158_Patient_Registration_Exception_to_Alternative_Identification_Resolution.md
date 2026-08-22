# Patient Registration Exception to Alternative Identification Resolution

**Scenario ID:** L3-G-158  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-158 |
| Canonical Title | Patient Registration Exception to Alternative Identification Resolution |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E, A |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional outcome when normal Patient Registration cannot be completed because the individual's identification/registration condition is exceptional, and an authorised alternative identification/resolution path establishes sufficient patient identity/context for healthcare registration.

The locked L2 defines Patient Registration as:

`Patient Identified → Identity Validation → Medical History Retrieved → Visit Registered → Consultation Ready`

and identifies configurable validation considerations including Institutional Identity, Campus, Active Status, Emergency Classification and Healthcare Eligibility. It also explicitly states that Institutional Health Services references existing institutional identities and never owns person identities.

This scenario therefore resolves the **registration exception**, not the underlying identity ownership.

## 3. Business Outcome

A patient-registration exception has been resolved through an authorised alternative identification path, allowing the healthcare visit to be registered with a sufficiently established patient identity/context, or the case is kept unresolved when authoritative identification cannot be established.

The scenario does not create or transfer ownership of Student or Employee identity.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Normal Patient Registration cannot complete because the expected identification/validation path has failed or is insufficient.

### 4.2 End Boundary

An authorised alternative identification resolution establishes the patient context sufficiently for the normal registration boundary to continue, or the exception remains unresolved under the applicable business policy.

### 4.3 In Scope

- Identification exception recognition.
- Determine applicable source identity domain.
- Alternative identification/resolution.
- Student context from C where applicable.
- Employee context from E where applicable.
- Platform/Foundation participation from A where required by upstream workflow/configuration.
- Validation of the alternative resolution.
- Handoff back to Patient Registration.
- Audit of the exception and resolution.

### 4.4 Out of Scope

- Creation of a new canonical Student identity.
- Creation of a new canonical Employee identity.
- Clinical treatment.
- Emergency treatment.
- Medical diagnosis.
- Technical identity/authentication mechanisms.
- Database matching algorithms.
- API design.
- Infrastructure.
- Notification implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the healthcare Patient Registration exception and its business resolution.

### Section C — Student Lifecycle

C remains owner of student identity and student status. G06 may consume C's authoritative information.

### Section E — People & Workforce

E remains owner of employee/personnel information. G06 may consume E's authoritative information.

### Section A — Platform Foundation

A provides enterprise foundation services including authentication, authorization, workflow, configuration, audit and document management. These are supporting capabilities; A does not acquire ownership of the patient or person identity.

## 6. Business Actors, Roles and Responsibilities

### Healthcare Registration / Medical Staff

Identify the exception, establish the appropriate business resolution and validate the resulting patient context.

### Student Lifecycle Authority — C

Confirms authoritative student identity/status when the individual is a student.

### People & Workforce Authority — E

Confirms authoritative employee/personnel information when the individual is an employee.

### Platform/Foundation Authority — A

Provides the applicable enterprise foundation boundary where required.

No technical identity-resolution role is invented.

## 7. Preconditions and Entry Criteria

1. A person is seeking healthcare.
2. Normal Patient Registration has encountered an identification/validation exception.
3. The exception is sufficiently understood to determine an appropriate alternative business path.
4. Applicable source-domain information can be consulted where relevant.
5. Healthcare registration remains a valid institutional need.

## 8. Trigger

Normal patient identification or validation cannot establish the required healthcare registration context.

## 9. Main Business Flow

### Phase A — Establish the Exception

1. Identify the attempted healthcare registration.
2. Identify the reason normal registration cannot complete.
3. Determine whether the issue concerns student identity, employee identity, campus, status, emergency classification, healthcare eligibility or another configured registration condition.
4. Preserve the exception context.

### Phase B — Determine Authoritative Identity Source

5. Determine whether the individual should be validated against C, E or another permitted institutional identity category.
6. Request/obtain the authoritative identity/context from the applicable owner.
7. Do not create a replacement canonical identity merely because the normal lookup failed.

### Phase C — Alternative Identification Resolution

8. Apply the authorised alternative identification/resolution procedure.
9. Validate the resulting patient context against the applicable registration rules.
10. Confirm campus where required.
11. Confirm active/status context where applicable.
12. Confirm emergency classification where applicable.
13. Confirm healthcare eligibility where applicable.

### Phase D — Resume Patient Registration

14. Establish the patient identity/context sufficiently for G06 Patient Registration.
15. Retrieve existing medical history where available.
16. Register the healthcare visit.
17. Establish Consultation Ready.

### Phase E — Complete

18. Preserve the exception and resolution history.
19. End the scenario.

The resulting visit follows the normal G06 lifecycle; this scenario does not perform consultation or treatment.

## 10. Alternate Business Flows

### 10.1 Student Identity Exception

C remains authoritative. G06 uses the authoritative student context once established.

### 10.2 Employee Identity Exception

E remains authoritative. G06 uses the authoritative employee context once established.

### 10.3 Guest/Visitor Category

Where configured patient categories permit guests/visitors, the alternative identification process follows the applicable institutional policy.

### 10.4 Emergency Patient

Where emergency classification applies, the registration exception must not prevent the applicable emergency-care business response. G06's emergency workflow separately governs immediate response. The L2 defines:

`Incident Reported → Emergency Assessment → Immediate Treatment → Referral Decision → Patient Stabilised → Case Closed`.

### 10.5 Existing Medical History Found After Resolution

The authoritative history is retrieved and remains immutable.

### 10.6 Alternative Resolution Cannot Establish Sufficient Identity

The healthcare visit is not registered against invented identity information. The exception remains unresolved.

## 11. Failure, Interruption and Business Recovery

### 11.1 No Authoritative Identity Source Can Confirm the Person

No canonical person identity is created by G06.

### 11.2 Student Status Cannot Be Confirmed

C remains authoritative; G06 does not infer student status.

### 11.3 Employee Status Cannot Be Confirmed

E remains authoritative; G06 does not infer employee status.

### 11.4 Alternative Identification Is Insufficient

The case remains unresolved until the applicable business policy establishes sufficient identity/context.

### 11.5 Healthcare Eligibility Remains Unresolved

No visit is treated as validly registered until the applicable configurable eligibility condition is resolved, except where a separate emergency policy governs immediate care.

### 11.6 Medical History Cannot Be Reliably Matched

No historical record is silently attached to the wrong patient.

### 11.7 Recovery

When authoritative identity/context becomes available, the scenario resumes at alternative-resolution validation and then returns to Patient Registration.

No technical retry, queue, rollback, API, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The L2 establishes:

- Institutional Health Services never owns person identities.
- Student identity is owned by C.
- Employee identity is owned by E.
- Patient categories are configurable.
- Patient registration validation may consider Institutional Identity, Campus, Active Status, Emergency Classification and Healthcare Eligibility.
- Validation rules are configurable.

The scenario must not create an invented canonical person identity as a workaround.

The G06 invariants remain applicable:

- Every Medical Encounter references exactly one Patient.
- Medical history is never deleted.
- Every lifecycle transition is audited.

## 13. State and Lifecycle Impact

The normal Patient Registration workflow remains:

`Patient Identified → Identity Validation → Medical History Retrieved → Visit Registered → Consultation Ready`.

The exception itself is not introduced as a new Patient or Medical Encounter lifecycle state.

Once resolved, the visit enters the normal registration boundary.

The Medical Encounter lifecycle remains:

`Registered → Consultation → Treatment → Completed → Closed`.

## 14. Business Events and Cross-Domain Collaboration

The L2 defines:

**`PatientRegistered` — Patient visit registered.**

G06 is the canonical publisher because it owns Patient Registration.

The retrieved L2 does not define a dedicated patient-registration-exception or alternative-identification event.

Therefore this scenario does not invent:

- `PatientRegistrationExceptionRaised`
- `AlternativeIdentityResolved`
- `PatientIdentityRecovered`

C/E provide authoritative identity/context but do not publish a G06 patient-registration event.

A participates only through its upstream platform-foundation boundary where required; it does not become publisher or owner of the healthcare business outcome.

## 15. Business Authorization, Approval and Audit Obligations

The L2 states that healthcare authorization is configurable and identifies typical operational roles such as Medical Staff for patient registration. Platform authorization remains an enterprise foundation responsibility.

Alternative identification must be performed under the applicable healthcare business authority.

The exception and resolution should preserve:

- Original registration context.
- Exception reason.
- Identity source consulted.
- Alternative identification evidence/context.
- Resolution decision.
- Patient registration result.
- Responsible business actor.
- Relevant audit timestamps/history.

Medical confidentiality and need-to-know principles apply.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Registration exception identified.
2. Exception cause established.
3. Correct authoritative identity source identified.
4. Alternative identification/resolution performed under applicable business authority.
5. Patient context validated.
6. Medical history retrieved where available.
7. Healthcare visit registered.
8. Visit reaches Consultation Ready.
9. Exception and resolution history audited.
10. `PatientRegistered` is published by G06.

### Postconditions

- The healthcare visit is associated with one validated patient.
- Student/employee identity remains owned by C/E.
- No duplicate or invented person identity has been created by G06.
- Medical history remains immutable.
- Consultation may proceed through the normal downstream workflow.
- Exception history remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 defines configurable registration validation but does not define a dedicated alternative-identification procedure.
2. It does not enumerate the exact evidence permitted for each exceptional patient category.
3. It does not define a dedicated exception event.

### Assumptions

- C/E remain authoritative identity sources.
- G06 owns the healthcare registration outcome.
- Emergency handling may follow the separately defined emergency workflow where applicable.
- A supplies enterprise foundation capabilities without identity ownership.

### Deferred Questions

Confirm the institution's authoritative alternative-identification policy and evidence requirements before Architecture Approval/Lock.

No unsupported identity-resolution rule or event is invented.
