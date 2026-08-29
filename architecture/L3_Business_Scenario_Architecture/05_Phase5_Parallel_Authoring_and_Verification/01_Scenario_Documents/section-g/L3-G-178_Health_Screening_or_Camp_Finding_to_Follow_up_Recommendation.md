# Health Screening or Camp Finding to Follow-up Recommendation

**Scenario ID:** L3-G-178  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E, H  
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
| Final L3 ID | L3-G-178 |
| Canonical Title | Health Screening or Camp Finding to Follow-up Recommendation |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Health screening and health camp activities can identify a participant condition that cannot be treated as merely a completed programme observation. The institution needs a governed way to recognise a material finding, assess its significance within institutional healthcare operations, and establish an appropriate follow-up recommendation when further care is required.

The purpose is to preserve a clear boundary between the originating programme activity and the subsequent individual healthcare action. The screening/camp remains a G06 Health Program concern; the individual follow-up recommendation is carried through the appropriate G06 healthcare workflow.

---

## 3. Business Outcome

A material finding from an institutional health screening or health camp has been assessed and, where further care is warranted, an authoritative follow-up recommendation has been recorded for the affected participant, with the participant and originating programme context preserved.

If no follow-up is required, the finding is recorded as such and the scenario ends without creating a follow-up recommendation.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A healthcare finding has been identified during an institutional health screening or health camp and is available for healthcare assessment.

### 4.2 End Boundary

The finding has been assessed and the appropriate follow-up outcome has been recorded: either a governed follow-up recommendation exists or the finding has been determined not to require follow-up.

### 4.3 In Scope

- Recognition of screening/camp finding.
- Participant identification/context.
- Healthcare assessment of the finding.
- Determination of whether follow-up is required.
- Follow-up recommendation.
- Routing into an applicable G06 healthcare activity where required.
- Preservation of originating programme context.
- Student/employee contextual participation.
- Healthcare audit and traceability.
- Configured downstream notification/service consumption.

### 4.4 Out of Scope

- Ownership of student identity.
- Ownership of workforce identity.
- External hospital operations.
- Diagnostic laboratory ownership.
- Financial accounting.
- Clinical protocols not defined by G06.
- Technical notification delivery.
- APIs, databases, services, UI, infrastructure or implementation mechanics.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the healthcare finding, assessment and follow-up recommendation within the applicable healthcare workflow. It also owns the Health Program from which the finding originated.

### Section C — Student Lifecycle

C provides authoritative student identity/context when the affected participant is a student. C does not own the medical finding or recommendation.

### Section E — People & Workforce

E provides authoritative employee/personnel context when the affected participant is an employee. E does not own the healthcare outcome.

### Section H — Institutional Services & Intelligence

H may consume permitted healthcare facts for notification or service purposes. H does not own the finding, recommendation or healthcare record.

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Assesses the health finding within institutional healthcare authority and determines the appropriate healthcare recommendation.

### Medical Staff

Supports screening/camp activity and records healthcare observations and outcomes within assigned responsibilities.

### Programme/Health Services Coordinator

Coordinates the originating health screening or camp activity where that responsibility is defined by G06 operational practice.

### Participant

Provides the individual context required for the healthcare activity.

### Section C / Section E Representatives

Provide authoritative identity or workforce context when needed.

No unsupported clinical role, approval role or external healthcare actor is introduced.

---

## 7. Preconditions and Entry Criteria

1. The finding originated from a G06 health screening or health camp activity.
2. The affected participant can be identified from authoritative institutional context.
3. The finding is sufficiently recorded to permit healthcare assessment.
4. An authorised healthcare professional is responsible for the assessment.
5. The finding has not already been resolved through another authoritative healthcare workflow.
6. The applicable G06 healthcare capability for follow-up is available.

---

## 8. Trigger

A material health screening or camp finding is identified and requires healthcare assessment to determine whether further action is appropriate.

---

## 9. Main Business Flow

### Phase A — Establish the Finding Context

1. Recognise the finding from the originating health screening or health camp.
2. Preserve the relationship to the originating Health Program.
3. Identify the affected participant using the authoritative participant context.
4. Record the finding within the appropriate G06 healthcare record.
5. Establish the healthcare context necessary for assessment.

### Phase B — Assess the Finding

6. An authorised medical professional reviews the finding.
7. Relevant healthcare information available within G06 is considered.
8. Determine whether the finding requires further healthcare attention.
9. Preserve the assessment outcome.

### Phase C — Determine Follow-up

10. If no further care is required, record that no follow-up recommendation is necessary.
11. If further care is required, determine the appropriate follow-up category within the capabilities already defined by G06.
12. Record the follow-up recommendation.
13. Preserve the relationship between the recommendation and originating finding.
14. Where the recommendation requires a separate consultation, observation, referral, prescription or other healthcare activity, establish that activity through the canonical G06 workflow.

### Phase D — Complete the Scenario

15. Confirm that the finding has a recorded assessment outcome.
16. Confirm that any required follow-up recommendation is recorded.
17. Preserve the healthcare history and audit evidence.
18. Publish only the applicable canonical G06 business event where one is defined upstream.
19. Where configured, allow H to consume the resulting business fact for downstream institutional services.
20. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Finding Does Not Require Follow-up

The medical professional determines that no further healthcare activity is required. The finding remains part of the relevant healthcare history and the scenario ends without creating a follow-up recommendation.

### 10.2 Follow-up Consultation Required

A follow-up consultation is recommended. The recommendation becomes the entry condition for the relevant Medical Encounter workflow.

### 10.3 External Referral Required

The finding requires care outside the institution. A Referral is established through the authoritative G06 Referral workflow.

### 10.4 Observation Required

Temporary institutional observation is appropriate. The participant is routed to the Observation workflow.

### 10.5 Prescription Required

The assessment indicates a treatment requiring a prescription. Prescription creation proceeds through the canonical Prescription workflow.

### 10.6 Student Participant

Student identity is consumed from C. G06 remains the sole owner of the healthcare finding and recommendation.

### 10.7 Employee Participant

Employee/personnel context is consumed from E. G06 remains the sole owner of the healthcare finding and recommendation.

### 10.8 Finding Requires Urgent Attention

Where the assessment identifies an urgent healthcare situation, the participant is routed to the applicable emergency healthcare workflow rather than waiting for routine follow-up.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Participant cannot be reliably identified | Do not attach the finding to an uncertain identity; resolve authoritative participant context first. |
| Finding is insufficiently documented for assessment | Preserve the finding and obtain the missing business information before making a final follow-up decision. |
| Medical professional is unavailable | Resolve authorised healthcare responsibility before completing the assessment. |
| Assessment is interrupted | Preserve the finding in its valid business state and resume assessment under applicable healthcare policy. |
| Finding requires urgent care | Escalate into the applicable emergency healthcare pathway. |
| Follow-up recommendation cannot yet be determined | Keep the finding under appropriate healthcare review until a supported decision is available; do not invent a lifecycle state. |
| Recommended follow-up is not completed | The follow-up activity remains governed by its own canonical workflow; the original finding does not silently become closed merely because the recommendation was issued. |

No technical retry, queue, rollback or infrastructure recovery behaviour is described.

---

## 12. Business Rules and Decision Constraints

1. G06 owns institutional health screening and camp healthcare outcomes.
2. Medical Encounter owns follow-up recommendation within the consultation/encounter context.
3. Student identity remains C-owned.
4. Workforce identity remains E-owned.
5. H does not acquire healthcare ownership by consuming healthcare facts.
6. Follow-up consultation, referral, observation and prescription are separate G06 healthcare capabilities.
7. A follow-up recommendation must be supported by the healthcare assessment.
8. No new screening-specific lifecycle state is invented.
9. Medical history and significant healthcare activity remain traceable and auditable.
10. Clinical judgement remains with authorised medical professionals.

The scenario does not invent clinical thresholds, diagnostic criteria, disease classifications or mandatory follow-up rules.

---

## 13. State and Lifecycle Impact

The originating Health Program retains its own authoritative lifecycle. This scenario does not alter that lifecycle merely because a finding is generated.

The individual healthcare assessment/follow-up uses the relevant G06 aggregate lifecycle, depending on the resulting healthcare activity.

No local `FindingPending`, `FindingResolved`, `FollowUpRequired` or similar state is introduced unless such a state is explicitly defined by upstream architecture.

The scenario's business outcome is the **recorded assessment and follow-up recommendation**, not completion of the recommended future healthcare activity.

---

## 14. Business Events and Cross-Domain Collaboration

The canonical event vocabulary must be selected from the originating G06 workflow.

For a health camp, L2-G06 explicitly identifies `HealthCampCreated` and `HealthCampCompleted`. For the individual finding, this scenario does not invent a generic `HealthScreeningFindingRecorded` or `FollowUpRecommended` event because such names are not established by the retrieved L2 event catalogue.

Where a downstream event is required and an authoritative G06 event exists for the resulting healthcare activity, G06 remains the canonical publisher.

C/E provide participant context through their authoritative responsibilities. H may consume published healthcare facts for configured institutional services.

Events remain distinct from synchronous business context hand-offs.

---

## 15. Business Authorization, Approval and Audit Obligations

Healthcare assessment and recommendation must remain within the authority of authorised medical professionals.

Audit evidence should preserve, where applicable:

- Originating Health Program.
- Participant identity/context.
- Finding.
- Healthcare assessment.
- Assessing medical professional.
- Follow-up decision.
- Follow-up recommendation.
- Resulting healthcare activity reference.
- Relevant lifecycle transition.
- Applicable canonical event.
- Date/time business evidence where defined by the authoritative healthcare record.

Individual medical information remains subject to healthcare confidentiality and applicable institutional policy.

No unsupported approval chain or separation-of-duty rule is invented.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Originating screening/camp finding is identified.
- Participant context is authoritative.
- Finding is assessed by authorised healthcare personnel.
- Assessment outcome is recorded.
- Decision on follow-up is recorded.
- If required, a follow-up recommendation is established.
- Any resulting healthcare activity is routed into its canonical G06 workflow.
- Healthcare history and audit evidence are preserved.
- Relevant canonical G06 business facts are published where defined.

### Postconditions

- The finding has an authoritative assessment outcome.
- Where required, the affected participant has a governed follow-up recommendation.
- The originating Health Program remains owned by G06.
- Any subsequent consultation/referral/observation/prescription remains separately governed.
- C/E ownership remains unchanged.
- H may consume permitted business facts without acquiring healthcare ownership.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The retrieved L2-G06 event catalogue does not define a generic event specifically named for a screening finding or follow-up recommendation.
2. Detailed clinical finding-severity and follow-up criteria are not enumerated in the L2 source.

### Assumptions

- Follow-up recommendation is part of the G06 Medical Encounter/healthcare workflow where applicable.
- The originating Health Program remains the source context for the finding.
- No new finding lifecycle is introduced at L3.

### Deferred Questions

If a future governed L2 decision introduces a canonical finding event or explicit screening-finding lifecycle, this scenario must be aligned with that source before Architecture Approval/Lock.
