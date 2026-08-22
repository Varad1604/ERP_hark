# Occupational Health Assessment to Medical Outcome

**Scenario ID:** L3-G-179  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
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
| Final L3 ID | L3-G-179 |
| Canonical Title | Occupational Health Assessment to Medical Outcome |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Occupational health activities allow the institution to assess workforce-related health conditions within its institutional healthcare capability. The scenario establishes a clear boundary: E remains authoritative for the employee/workforce relationship, while G06 owns the healthcare assessment and its medical outcome.

The outcome may be a recorded assessment result, recommendation, medical certificate or another healthcare outcome supported by the G06 capability. The scenario does not invent a specific fitness classification, employment decision or clinical standard.

---

## 3. Business Outcome

An occupational health assessment has been completed for the applicable workforce participant and the resulting medical outcome has been recorded under G06 healthcare ownership, with relevant workforce context obtained from E and any necessary downstream institutional service consumption preserved.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A workforce participant has an institutional occupational health assessment requirement and the assessment is ready to be conducted under G06 healthcare authority.

### 4.2 End Boundary

The occupational health assessment has a recorded medical outcome.

### 4.3 In Scope

- Occupational health assessment.
- Workforce participant context.
- Healthcare professional assessment.
- Medical findings/outcome.
- Follow-up recommendation where clinically required.
- Medical certificate where supported and required.
- Referral/observation/prescription where applicable.
- Audit and healthcare history.
- Configured downstream notification/service consumption.

### 4.4 Out of Scope

- Ownership of employee identity.
- Employment status or position assignment.
- Payroll.
- Leave approval.
- Workforce disciplinary decisions.
- Financial accounting.
- External hospital ownership.
- Clinical rules not defined by G06.
- Technical implementation.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the occupational healthcare assessment, medical record and resulting healthcare outcome.

### Section E — People & Workforce

E provides authoritative employee/workforce context. It retains ownership of employee profile, employment relationship and workforce lifecycle.

### Section H — Institutional Services & Intelligence

H may consume permitted healthcare facts for notification or institutional service purposes. It does not own the medical outcome.

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Conducts the occupational health assessment and determines the medical outcome within healthcare authority.

### Medical Staff

Supports assessment and healthcare recording within assigned responsibilities.

### Workforce Participant

Participates in the occupational health assessment.

### Workforce / People Representative

Provides authoritative workforce context where required.

### Notification Services

H may support configured communication of permitted business facts.

No employment authority is assigned to G06 merely because G06 records a medical outcome.

---

## 7. Preconditions and Entry Criteria

1. An occupational health assessment requirement exists within institutional healthcare scope.
2. The workforce participant can be identified from E.
3. The required healthcare capability is available.
4. An authorised medical professional is responsible for the assessment.
5. Applicable occupational health policy is available where such policy is defined upstream.

---

## 8. Trigger

A workforce participant's occupational health assessment requirement is established and ready for healthcare assessment.

---

## 9. Main Business Flow

### Phase A — Establish Assessment Context

1. Identify the occupational health assessment requirement.
2. Obtain authoritative workforce context from E.
3. Establish the healthcare assessment under G06.
4. Preserve the relationship between the healthcare assessment and the workforce participant.
5. Identify the authorised medical professional responsible for the assessment.

### Phase B — Conduct Assessment

6. Conduct the occupational health assessment.
7. Record the relevant medical findings and healthcare observations.
8. Apply only the authoritative healthcare decision criteria available to G06.
9. Determine the medical outcome.
10. Record the medical outcome within the G06 healthcare record.

### Phase C — Determine Additional Healthcare Action

11. If no additional healthcare action is required, preserve the assessment as the completed medical outcome.
12. If follow-up is required, record the follow-up recommendation.
13. If observation, prescription or referral is required, route the participant into the applicable G06 healthcare workflow.
14. If a medical certificate is required and supported by the applicable healthcare outcome, establish it through the canonical Medical Certificate workflow.
15. Preserve all resulting healthcare relationships without transferring ownership to E.

### Phase D — Complete

16. Confirm the occupational health assessment has a recorded medical outcome.
17. Preserve the healthcare history and audit evidence.
18. Publish the applicable canonical G06 business event where defined.
19. Where configured, provide permitted facts to H.
20. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 No Further Healthcare Action

The assessment is completed with a medical outcome and no further healthcare activity is required.

### 10.2 Follow-up Recommendation

A follow-up recommendation is recorded and the affected workforce participant enters the relevant G06 follow-up workflow.

### 10.3 Referral

External healthcare assessment is required. The G06 Referral workflow governs the referral.

### 10.4 Observation

Temporary institutional observation is required. The G06 Observation workflow governs the observation.

### 10.5 Prescription

Treatment requires a prescription. The G06 Prescription workflow governs prescription creation.

### 10.6 Medical Certificate

The assessment supports a medical certificate. The G06 Medical Certificate workflow governs certificate issuance.

### 10.7 Workforce Context Change During Assessment

If E's authoritative workforce information changes while the healthcare assessment is being conducted, G06 does not overwrite or become owner of the workforce record. The healthcare assessment retains its G06-owned medical outcome and uses the authoritative workforce context according to the applicable cross-domain policy.

### 10.8 Confidential Medical Outcome

Where the medical outcome contains confidential healthcare information, downstream sharing is restricted to the permitted institutional business purpose. G06 remains owner of the medical information.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Workforce participant cannot be identified | Resolve authoritative E workforce context before recording the medical outcome. |
| Required healthcare professional unavailable | Establish authorised healthcare responsibility before completing the assessment. |
| Assessment interrupted | Preserve the valid business assessment context and resume under applicable healthcare policy. |
| Medical finding is inconclusive | Do not create an unsupported final outcome; continue the appropriate healthcare assessment or follow-up. |
| Follow-up required | Establish the appropriate G06 follow-up activity. |
| External care required | Establish the canonical G06 Referral outcome. |
| Workforce information is inconsistent | Resolve the authoritative E context; do not duplicate or locally correct the employee record inside G06. |

No technical retries, queues, rollback or infrastructure failure handling is included.

---

## 12. Business Rules and Decision Constraints

1. G06 owns occupational healthcare outcomes.
2. E owns workforce information.
3. A medical assessment must be conducted under authorised healthcare responsibility.
4. G06 must not alter E-owned workforce lifecycle or employment information.
5. Follow-up, referral, observation, prescription and medical certificate activities remain within their canonical G06 capabilities.
6. Medical information remains traceable and auditable.
7. H may consume permitted healthcare facts but does not acquire healthcare ownership.
8. Clinical judgement remains with authorised medical professionals.

The scenario does not invent fitness-for-duty rules, employment consequences, leave entitlements, payroll effects or disciplinary consequences.

---

## 13. State and Lifecycle Impact

The scenario consumes the applicable G06 healthcare lifecycle rather than creating a new Occupational Health lifecycle.

Where the assessment is represented through Medical Encounter, its authoritative Medical Encounter lifecycle governs the healthcare activity.

Where a Medical Certificate, Referral, Observation Stay or Prescription results, the corresponding canonical G06 aggregate lifecycle governs that outcome.

E's workforce lifecycle remains unchanged by the G06 healthcare outcome unless a separate authoritative business workflow consumes the medical fact and performs an independently governed workforce decision.

---

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of medical facts because it owns the healthcare outcome.

The exact event must be selected from the canonical G06 event vocabulary applicable to the resulting healthcare activity. The scenario does not invent an `OccupationalHealthAssessmentCompleted` event because such a canonical event is not established by the retrieved L2-G06 event catalogue.

E provides authoritative workforce context. H may consume permitted G06 facts for downstream institutional services.

Medical facts remain distinct from synchronous workforce-context hand-offs.

No G06 event transfers ownership of the employee record to G06 or H.

---

## 15. Business Authorization, Approval and Audit Obligations

Occupational health assessment must remain within authorised medical-professional responsibility.

Audit evidence should preserve:

- Workforce participant reference.
- Occupational health assessment.
- Assessing healthcare professional.
- Material findings.
- Medical outcome.
- Follow-up/referral/observation/prescription/certificate action where applicable.
- Relevant lifecycle transition.
- Applicable canonical G06 event.
- Evidence of the healthcare decision.

Medical confidentiality is preserved.

The scenario does not create a separate employment approval or management decision because no such upstream rule is established in L2-G06.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Workforce participant is identified from E.
- Occupational health assessment is established under G06.
- Assessment is conducted.
- Medical findings are recorded.
- Medical outcome is determined and recorded.
- Required follow-up healthcare activity is established where applicable.
- Audit evidence is preserved.
- Applicable G06 business fact/event is published where defined.

### Postconditions

- G06 contains the authoritative occupational medical outcome.
- E remains authoritative for the workforce relationship.
- Any follow-up healthcare activity is separately governed.
- H may consume permitted downstream facts.
- No employment, payroll or disciplinary decision is implicitly made by this scenario.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. L2-G06 does not define a dedicated occupational-health event name.
2. Detailed occupational health assessment criteria and resulting employment implications are not enumerated in the retrieved L2.

### Assumptions

- Occupational health medical outcomes remain G06-owned.
- Workforce context remains E-owned.
- Resulting healthcare actions use existing G06 aggregates.

### Deferred Questions

Any future requirement for a formal fitness classification, employment restriction, statutory occupational-health reporting, or dedicated event must be governed by the appropriate upstream architecture before inclusion in the L3 scenario.
