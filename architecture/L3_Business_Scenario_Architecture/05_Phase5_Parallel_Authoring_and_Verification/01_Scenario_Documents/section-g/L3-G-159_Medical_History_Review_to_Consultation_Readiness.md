# Medical History Review to Consultation Readiness

**Scenario ID:** L3-G-159  
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

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-159 |
| Canonical Title | Medical History Review to Consultation Readiness |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario establishes the healthcare business outcome in which the medical history relevant to an identified and registered patient is reviewed, the patient context required for care is established, and the healthcare visit is ready for consultation.

The locked L2 identifies Medical History as an Institutional Health Services responsibility. Patient Registration retrieves medical history before a visit reaches Consultation Ready. The L2 states that history may include previous visits, allergies, chronic conditions, vaccinations, previous prescriptions and previous referrals, and that historical information remains immutable.

The scenario therefore performs **review and readiness**, not clinical diagnosis or treatment.

## 3. Business Outcome

The healthcare professional has access to the relevant, authoritative medical history for the registered patient, material healthcare context has been reviewed within the applicable business boundary, and the patient/visit is **Consultation Ready**.

No historical medical fact is altered as a consequence of review.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A healthcare visit has been registered and relevant medical history is available for review.

### 4.2 End Boundary

Medical history review is complete for the current healthcare context and the visit is ready for consultation.

### 4.3 In Scope

- Patient/visit identification.
- Retrieval of relevant medical history.
- Review of previous visits.
- Review of allergies.
- Review of chronic conditions.
- Review of vaccination information.
- Review of previous prescriptions.
- Review of previous referrals.
- Identification of relevant follow-up context.
- Consultation readiness.

### 4.4 Out of Scope

- Creating or correcting Student/Employee identity.
- Clinical diagnosis.
- Treatment.
- Prescription creation.
- Medicine dispensing.
- Referral execution.
- Medical certificate issuance.
- Modification/deletion of historical medical records.
- Technical access control or storage implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns medical history and the Medical Encounter/healthcare operational context.

The L2 explicitly assigns Medical History and Patient Registration to Institutional Health Services.

### Section C — Student Lifecycle

C remains authoritative for student identity/status information used to identify the patient.

### Section E — People & Workforce

E remains authoritative for employee/personnel information and healthcare staff context.

Participation does not transfer ownership of identity or workforce information to G06.

## 6. Business Actors, Roles and Responsibilities

### Medical Professional / Medical Staff

Reviews the patient's relevant medical history and establishes consultation readiness.

### Patient

Provides relevant information when required by healthcare practice.

### Student Lifecycle Authority — C

Provides authoritative student context where the patient is a student.

### People & Workforce Authority — E

Provides authoritative employee context where the patient is an employee.

Clinical judgement remains with medical professionals; the scenario does not replace it with system-derived decisions.

## 7. Preconditions and Entry Criteria

1. A patient is identified.
2. A healthcare visit has been registered or is otherwise within the Patient Registration boundary.
3. The patient has an identifiable medical history context.
4. Applicable healthcare staff are available.
5. Medical history can be reviewed under the applicable confidentiality/authorization policy.

## 8. Trigger

A registered healthcare visit reaches the point where relevant medical history must be reviewed before consultation.

## 9. Main Business Flow

### Phase A — Establish Patient Context

1. Identify the patient and current healthcare visit.
2. Confirm the patient identity remains linked to the authoritative institutional identity context.
3. Confirm the applicable campus/context.

### Phase B — Retrieve Medical History

4. Retrieve relevant previous visits.
5. Review recorded allergies.
6. Review chronic conditions.
7. Review vaccination information.
8. Review previous prescriptions.
9. Review previous referrals.
10. Establish which historical information is relevant to the current consultation context.

The L2 expressly identifies these categories of medical history and requires historical information to remain immutable.

### Phase C — Clinical Context Review

11. Review relevant historical context.
12. Identify information that may require attention during the upcoming consultation.
13. Preserve the distinction between historical facts and current clinical assessment.
14. Do not create a diagnosis, treatment or prescription within this scenario.

### Phase D — Consultation Readiness

15. Confirm that the relevant history-review activity is complete.
16. Establish the visit as **Consultation Ready**.
17. End the scenario.

The consultation itself begins under the separate Consultation Management workflow.

## 10. Alternate Business Flows

### 10.1 No Prior Medical History

The patient remains Consultation Ready after confirming that no prior history is available.

### 10.2 Extensive Medical History

Only the history relevant to the current healthcare context is reviewed; historical records remain unchanged.

### 10.3 Allergy Information Present

Recorded allergy information is reviewed as part of the healthcare context. No new allergy is created merely from the review.

### 10.4 Chronic Condition Present

Existing chronic-condition information is considered during readiness; clinical interpretation remains within consultation.

### 10.5 Previous Referral Present

The referral history is reviewed. The referral lifecycle itself is not executed here.

### 10.6 Previous Prescription Present

Prescription history is reviewed. A new prescription is outside this scenario.

### 10.7 Current History Requires Clinical Attention

The matter is carried into the consultation for clinical assessment rather than resolved prematurely in the history-review scenario.

## 11. Failure, Interruption and Business Recovery

### 11.1 Patient Context Cannot Be Established

Review does not proceed against an invented patient.

### 11.2 Historical Record Cannot Be Reliably Associated

The record is not silently attached to the wrong patient.

### 11.3 Historical Information Is Incomplete

The available history is preserved as-is; missing facts are not fabricated.

### 11.4 Conflicting Historical Information

The conflict is preserved for clinical/business review rather than rewriting immutable history.

### 11.5 Medical Staff Cannot Complete Review

The visit remains outside Consultation Ready until the required business review is completed or the applicable operational policy permits an alternative path.

### 11.6 Recovery

Once authoritative patient/history context is established, the review resumes from the unresolved business boundary.

No technical retry, queue, rollback, API, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The L2 states:

- Medical history includes previous visits, allergies, chronic conditions, vaccinations, previous prescriptions and previous referrals.
- Historical information shall remain immutable.
- Medical history is owned by Institutional Health Services.
- Every lifecycle transition shall be audited.

Medical history review must not be used to overwrite the underlying historical record.

The scenario does not invent a universal definition of "clinically relevant history"; that remains within applicable healthcare operational policy and professional judgement.

## 13. State and Lifecycle Impact

The Patient Registration workflow ends in:

`Visit Registered → Consultation Ready`.

The Consultation workflow then proceeds:

`Patient Registered → Clinical Assessment → Diagnosis → Treatment → Prescription (If Required) → Consultation Closed`.

This scenario establishes readiness and does not execute the consultation lifecycle.

No new `History Reviewed` aggregate state is invented.

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2 defines:

- `PatientRegistered` — Patient visit registered.
- `ConsultationStarted` — Consultation initiated.
- `ConsultationCompleted` — Consultation completed.
- `FollowUpRecommended` — Follow-up advised.

The L2 does **not** define a dedicated `MedicalHistoryReviewed` event.

Therefore this scenario does not invent one.

The canonical publisher for the defined Patient/Consultation events remains G06.

C/E remain authoritative sources for identity/personnel context and do not acquire ownership of G06 medical history.

## 15. Business Authorization, Approval and Audit Obligations

The L2 states that healthcare authorization is configurable and that medical confidentiality is governed through need-to-know, role-based access and institutional privacy policies.

Medical history review must be performed by an appropriately authorised healthcare actor.

Audit evidence should preserve the business action and lifecycle transition without exposing unnecessary medical information.

Every lifecycle transition is required to be audited.

No technical access control/IAM mechanism is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Patient and visit identified.
2. Relevant medical history retrieved.
3. Relevant history categories reviewed.
4. Historical records remain immutable.
5. Relevant context identified for the upcoming consultation.
6. Consultation readiness established.
7. Required lifecycle/audit history preserved.

### Postconditions

- Patient is ready for consultation.
- Medical history remains unchanged.
- No diagnosis/treatment has been recorded by this scenario.
- Existing referral/prescription/vaccination history remains owned by G06.
- The next consultation workflow can begin.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 does not define a formal medical-history review checklist.
2. It does not define a dedicated MedicalHistoryReviewed event.
3. The exact business definition of "relevant history" is not enumerated.

### Assumptions

- Medical professionals apply applicable clinical judgement.
- Historical information remains immutable.
- Consultation readiness is the boundary immediately preceding consultation.

### Deferred Questions

Confirm any institution-specific history-review checklist before Architecture Approval/Lock.
