# Referral Follow-up to Updated Healthcare History

**Scenario ID:** L3-G-172  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
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
| Final L3 ID | L3-G-172 |
| Canonical Title | Referral Follow-up to Updated Healthcare History |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

A referral does not necessarily end the institution's healthcare responsibility. Where a referral generates a subsequent institutional follow-up need, the institution must be able to establish the follow-up healthcare activity and preserve the resulting information in the patient's authoritative institutional medical history.

The L2-G06 architecture explicitly includes Referral Management, Follow-up Management, Medical History, Medical Encounter and Referral History. It also states that external hospital management remains outside G06. This scenario therefore reconciles relevant follow-up information into institutional healthcare history without importing external clinical ownership.

---

## 3. Business Outcome

The required referral follow-up has been completed within institutional healthcare scope, the relevant healthcare outcome has been reviewed by the authorised healthcare professional, and the institutional medical history now reflects the resulting healthcare position.

The outcome may establish continued treatment, further follow-up, observation, referral, or another supported healthcare action. The scenario does not assert that the external provider's records are owned or altered by G06.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A completed or otherwise actionable referral creates a requirement for institutional healthcare follow-up.

### 4.2 End Boundary

The institutional follow-up activity has been completed and its authorised outcome has been incorporated into the patient's institutional healthcare history.

### 4.3 In Scope

- Identification of the originating Referral.
- Identification of the patient and Medical Encounter.
- Follow-up requirement.
- Review of referral-related information available to the institution.
- Healthcare professional assessment.
- Recording of follow-up outcome.
- Updating institutional medical history.
- New follow-up recommendation where required.
- Further referral/observation/consultation where required.
- Student and workforce context consumption.
- Audit and medical confidentiality.

### 4.4 Out of Scope

- External hospital clinical record ownership.
- External provider treatment.
- External provider billing.
- Direct modification of external medical records.
- Financial accounting.
- Student identity ownership.
- Employee identity ownership.
- Technical data exchange mechanisms.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns:

- Medical Encounter.
- Referral.
- Follow-up healthcare activity.
- Institutional medical history.
- Referral history.

The L2 explicitly identifies Medical Encounter as the aggregate owning follow-up recommendation and Referral as the aggregate owning referral reason, provider, status and history.

### Section C — Student Lifecycle

C remains authoritative for student identity and status where the patient is a student. G06 does not recreate or own student identity.

### Section E — People & Workforce

E remains authoritative for healthcare-professional and employee information. G06 uses professional context for healthcare operations without acquiring personnel ownership.

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Reviews the referral-related follow-up information and determines the appropriate institutional healthcare outcome.

### Medical Staff

Supports follow-up registration, review and recording within authorised healthcare responsibilities.

### Patient

Provides information relevant to the follow-up and receives the resulting healthcare outcome.

### External Healthcare Provider

Remains responsible for external medical care. Any information available to G06 is treated as information relevant to the institutional follow-up, not as transfer of external ownership.

### Section C / Section E

Provide authoritative identity/personnel context where applicable.

No new technical or clinical authority is introduced.

---

## 7. Preconditions and Entry Criteria

1. A patient has an authoritative institutional healthcare record.
2. A Referral exists or a referral-related follow-up requirement has been established.
3. The patient can be associated with the relevant Medical Encounter.
4. An authorised healthcare professional is available to review the follow-up.
5. Relevant referral/follow-up information is available to the institution.
6. The follow-up requirement is within G06 healthcare scope.

---

## 8. Trigger

A referral follow-up becomes due or otherwise requires institutional healthcare review.

---

## 9. Main Business Flow

### Phase A — Establish Follow-up Requirement

1. Identify the patient.
2. Identify the originating Medical Encounter.
3. Identify the associated Referral and referral history.
4. Establish why institutional follow-up is required.
5. Register the follow-up healthcare activity within the existing healthcare context.

### Phase B — Review Available Referral Information

6. Review the referral reason.
7. Review the external healthcare provider associated with the Referral.
8. Review the follow-up information available to the institution.
9. Distinguish information supplied for institutional follow-up from the external provider's authoritative medical record.
10. Identify any unresolved healthcare concern requiring institutional assessment.

### Phase C — Institutional Healthcare Assessment

11. Conduct the required follow-up consultation/review.
12. Assess the patient's current healthcare position.
13. Record the relevant clinical/operational outcome.
14. Determine whether the patient's existing institutional medical history requires an additional healthcare-history entry.
15. Preserve the relationship between the new follow-up activity, Medical Encounter and Referral.

### Phase D — Determine Continuing Care

16. If no further institutional healthcare action is required, complete the follow-up activity.
17. If another follow-up is required, establish the applicable follow-up recommendation.
18. If observation is required, route to Observation Care.
19. If another external referral is required, route to Referral Management.
20. If prescription treatment is required, route through the Prescription workflow.
21. If a medical certificate is required, route through the Medical Certificate workflow.

### Phase E — Update Healthcare History

22. Record the authorised follow-up outcome in institutional medical history.
23. Preserve referral history and the relationship between referral and follow-up.
24. Ensure historical information remains traceable and is not silently deleted or rewritten.
25. Where configured, make relevant business facts available to downstream notification/reporting consumers.
26. Complete the follow-up healthcare activity.

---

## 10. Alternate Business Flows

### 10.1 Follow-up Confirms Recovery / No Further Action

The healthcare professional confirms that no additional institutional healthcare activity is required. The follow-up outcome is recorded and the institutional medical history is updated.

### 10.2 Further Follow-up Required

A new follow-up recommendation is established through the Medical Encounter workflow. The scenario does not invent a new lifecycle state.

### 10.3 Further Referral Required

The follow-up assessment determines that external healthcare is again required. A new Referral is created through the authoritative Referral workflow.

### 10.4 Observation Required

The patient requires temporary institutional monitoring. The case moves into the Observation Care workflow.

### 10.5 Prescription Required

The healthcare professional determines that prescription treatment is required. Prescription creation is handled through the Prescription workflow.

### 10.6 Medical Certificate Required

The follow-up assessment establishes a need for an institution-issued medical certificate. The Medical Certificate workflow governs issuance.

### 10.7 External Information Is Incomplete

The institution records only the information actually available and does not manufacture external clinical findings.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Referral cannot be associated with the patient | Resolve authoritative patient/Medical Encounter context before updating history. |
| Follow-up requirement is unclear | Obtain authorised healthcare assessment rather than inventing a follow-up reason. |
| External information is unavailable | Continue only with information available to the institution and record the unresolved follow-up condition where necessary. |
| Patient requires urgent care during follow-up | Route to the applicable emergency/medical workflow without falsely closing the follow-up. |
| Further referral is required | Establish a new Referral through the authoritative G06 referral lifecycle. |
| Follow-up outcome cannot yet be determined | Preserve the healthcare activity in its valid business position until authorised assessment is completed. |
| Historical information conflicts | Preserve the authoritative institutional history and record the new outcome as a traceable successor fact rather than silently rewriting history. |

No technical retries, queues, rollback, APIs, databases or infrastructure mechanics are defined.

---

## 12. Business Rules and Decision Constraints

1. G06 owns institutional healthcare operations.
2. Medical Encounter owns follow-up recommendation and clinical encounter context.
3. Referral owns referral reason, external healthcare provider, referral status and referral history.
4. Medical history is not to be silently deleted.
5. Student identity remains owned by Section C.
6. Employee/personnel information remains owned by Section E.
7. External hospital management remains outside G06.
8. Medical professionals retain operational authority.
9. Significant healthcare activities are auditable.
10. Follow-up policies remain configuration-driven.

The scenario does not invent clinical follow-up intervals, external-result validation criteria, or medical treatment rules.

---

## 13. State and Lifecycle Impact

### Referral

The authoritative Referral lifecycle is:

`Recommended → Accepted → Completed`

This scenario consumes referral history and may follow a completed referral; it does not create a new Referral lifecycle state.

### Medical Encounter

The Medical Encounter lifecycle remains:

`Registered → Consultation → Treatment → Completed → Closed`

The follow-up activity extends the healthcare history without redefining the lifecycle.

### Medical History

Medical history is treated as an enduring historical record. New follow-up information is recorded as additional authoritative healthcare history rather than destructive replacement of prior history.

---

## 14. Business Events and Cross-Domain Collaboration

G06 remains the canonical publisher of healthcare facts because it owns the Medical Encounter, Referral and healthcare-history outcomes.

The L2-G06 document defines canonical healthcare domain events including `FollowUpRecommended`, `ReferralRecommended`, and other healthcare events. The exact event applicable to completion of referral follow-up and history update is not explicitly identified in the retrieved authoritative event list.

Therefore no new event name is invented.

Section C may consume relevant healthcare information for student context where permitted. Section E provides authoritative healthcare-professional/personnel context.

Downstream consumers must not modify G06-owned healthcare state directly.

---

## 15. Business Authorization, Approval and Audit Obligations

Follow-up assessment and resulting healthcare action must be performed within the authority of an appropriately authorised healthcare professional.

Audit evidence should preserve:

- Patient reference.
- Medical Encounter.
- Referral reference.
- Referral reason/provider context.
- Follow-up requirement.
- Follow-up assessment.
- Healthcare outcome.
- Responsible healthcare professional.
- Resulting follow-up/referral/prescription/certificate action.
- Relevant lifecycle transition.

Medical confidentiality and need-to-know requirements apply.

Medical history must remain permanently traceable according to applicable institutional retention policy.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Referral and patient context are identified.
- Follow-up requirement is established.
- Available referral information is reviewed.
- Authorised healthcare assessment is completed.
- Follow-up outcome is recorded.
- Institutional medical history is updated.
- Referral history remains traceable.
- Any subsequent healthcare workflow is established where required.
- Audit evidence is preserved.

### Postconditions

- Institutional healthcare history reflects the latest authorised follow-up outcome.
- Original referral history remains preserved.
- External provider ownership remains external.
- Any new follow-up/referral/prescription/certificate remains governed by its canonical G06 lifecycle.
- C/E ownership boundaries remain unchanged.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The exact canonical event for completed referral follow-up/history update is not explicitly identified in the retrieved L2 event catalogue.
2. The L2 does not define a universal external clinical-result reconciliation policy.
3. The exact permitted content of external referral follow-up information is not enumerated.

### Assumptions

- Institutional follow-up uses the existing Medical Encounter/healthcare-history ownership of G06.
- External clinical records remain externally owned.
- Historical institutional medical information is preserved rather than destructively overwritten.

### Deferred Questions

Confirm the authoritative event/contract semantics for referral follow-up completion before Architecture Approval/Lock.
