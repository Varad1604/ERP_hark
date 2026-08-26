# Annual Health Checkup to Recorded Assessment Outcome

**Scenario ID:** L3-G-177  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E, H  
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
| Final L3 ID | L3-G-177 |
| Canonical Title | Annual Health Checkup to Recorded Assessment Outcome |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Annual Health Checkups provide a structured institutional health assessment rather than an isolated walk-in consultation. The institution needs an end-to-end process that establishes the checkup, identifies participants, conducts the assessment, records findings and recommendations within G06, and completes the checkup while preserving the resulting healthcare history.

Where a finding requires further healthcare action, the resulting follow-up, referral, observation or prescription remains separately governed by the relevant G06 capability.

---

## 3. Business Outcome

The Annual Health Checkup has been conducted for the applicable participant population, the resulting assessment outcomes have been recorded in institutional healthcare history, any required follow-up recommendations have been established, and the Annual Health Checkup has reached the authoritative `Completed` state.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An institutional annual health assessment requirement is established and ready to be represented as an Annual Health Checkup.

### 4.2 End Boundary

The health assessment has been conducted, relevant assessment outcomes have been recorded, required follow-up recommendations have been established, and the Annual Health Checkup is `Completed`.

### 4.3 In Scope

- Annual Health Checkup establishment.
- Checkup scheduling/organisation at business level.
- Participant context.
- Healthcare professional assignment.
- Health assessment.
- Recording findings.
- Recording recommendations.
- Follow-up recommendation where required.
- Completion of the Annual Health Checkup.
- Student and employee context.
- Notification participation through H where configured.
- Healthcare audit/history.

### 4.4 Out of Scope

- Student identity ownership.
- Employee identity ownership.
- External hospital/diagnostic ownership.
- Laboratory system ownership.
- Financial accounting.
- Clinical criteria not explicitly defined by G06.
- Technical scheduling, APIs, databases, UI or infrastructure.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns Annual Health Checkup, its lifecycle, assessment outcome and healthcare-history relationship.

### Section C — Student Lifecycle

C provides authoritative student identity/context where student checkups are involved.

### Section E — People & Workforce

E provides authoritative employee/personnel context where employee/occupational checkups are involved.

### Section H — Institutional Services & Intelligence

H may consume configured healthcare facts for notification or institutional service purposes. It does not own the Annual Health Checkup.

---

## 6. Business Actors, Roles and Responsibilities

### Medical Administrator

Coordinates the Annual Health Checkup programme within G06.

### Authorised Medical Professional

Conducts the healthcare assessment and records the relevant medical outcome within authorised responsibility.

### Medical Staff

Supports checkup execution and recording within assigned healthcare responsibilities.

### Participant

Participates in the health assessment.

### Section C / Section E

Provide authoritative participant and personnel context.

### Notification Services

H may deliver configured reminders or health-program notifications without owning the checkup.

No additional actor or approval authority is introduced.

---

## 7. Preconditions and Entry Criteria

1. The annual health assessment requirement is within G06 scope.
2. An Annual Health Checkup can be established under G06.
3. Participant identity/context is available from the appropriate canonical domain.
4. Required medical personnel can be assigned.
5. The checkup can be conducted within the institution's configured healthcare capability.
6. Applicable healthcare programme policy is available.

---

## 8. Trigger

An authorised requirement for an Annual Health Checkup is established.

---

## 9. Main Business Flow

### Phase A — Establish the Checkup

1. Identify the institutional annual health assessment requirement.
2. Establish the Annual Health Checkup under G06 ownership.
3. Establish the relevant checkup context and intended participant population.
4. Identify required healthcare professionals.
5. Establish the checkup in the authoritative `Scheduled` state.

### Phase B — Prepare and Conduct the Assessment

6. Confirm the checkup is ready for conduct.
7. Identify participants using authoritative C/E context.
8. Conduct the health assessment under authorised medical responsibility.
9. Record the relevant assessment findings.
10. Record healthcare recommendations resulting from the assessment.
11. Where a finding requires additional healthcare activity, establish the appropriate follow-up recommendation.
12. Where required, route the participant to consultation, observation, referral, prescription or another applicable G06 workflow.

### Phase C — Record the Checkup Outcome

13. Preserve the participant's assessment outcome in institutional healthcare history.
14. Ensure the assessment remains associated with the Annual Health Checkup.
15. Complete the applicable assessment activity.
16. Move the Annual Health Checkup to `Conducted` once the assessment activity has occurred according to the authoritative lifecycle.

### Phase D — Complete the Checkup

17. Confirm the Annual Health Checkup's completion condition is satisfied.
18. Record the overall checkup outcome.
19. Transition the Annual Health Checkup to `Completed`.
20. Preserve the completed checkup and associated assessment outcomes.
21. Publish the applicable canonical G06 healthcare event.
22. Where configured, provide relevant facts to H for notification/service consumption.
23. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Normal Assessment With No Follow-up

The participant's assessment is recorded and no additional healthcare activity is required. The checkup proceeds toward completion.

### 10.2 Assessment Produces Follow-up Recommendation

The healthcare professional establishes a follow-up recommendation. The follow-up becomes a separate G06 healthcare activity while the Annual Health Checkup retains its own lifecycle.

### 10.3 Referral Required

The assessment identifies a need for external healthcare. A Referral is established through the authoritative G06 Referral workflow.

### 10.4 Observation Required

Temporary institutional observation is required. The participant is routed into Observation Care.

### 10.5 Prescription Required

A prescription is required. Prescription creation proceeds through the authoritative Prescription workflow.

### 10.6 Student Checkup

C provides authoritative student context. G06 owns the health assessment outcome.

### 10.7 Employee Checkup

E provides authoritative workforce context. G06 owns the health assessment outcome.

### 10.8 Checkup Completed With Outstanding Individual Follow-up

The Annual Health Checkup may reach its own completion state when its completion criteria are met, while separately established individual healthcare follow-up continues under its own G06 lifecycle.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Participant identity/context is unavailable | Resolve authoritative participant context before recording the assessment outcome. |
| Required medical professional is unavailable | Resolve authorised healthcare assignment under applicable policy before continuing the affected assessment activity. |
| Assessment is interrupted | Preserve the checkup in its valid lifecycle position and resume or otherwise resolve the business interruption under applicable policy. |
| Assessment findings are incomplete | Do not record an unsupported final assessment outcome. Complete the healthcare assessment first. |
| Finding requires urgent healthcare | Route to the applicable urgent/emergency healthcare workflow without falsely completing the assessment outcome. |
| Follow-up is required | Establish follow-up through the relevant G06 workflow. |
| Checkup completion condition is not satisfied | Do not move the Annual Health Checkup to `Completed`. |
| Participant cannot complete the assessment | Apply the authoritative checkup participation/exception policy if defined; do not invent a new checkup lifecycle state. |

No technical retries, queues, rollback or infrastructure failure handling are included.

---

## 12. Business Rules and Decision Constraints

1. Annual Health Checkup is owned by G06.
2. Annual Health Checkup lifecycle is `Scheduled → Conducted → Completed`.
3. Medical professionals retain healthcare authority.
4. Assessment findings and recommendations are part of the institutional healthcare outcome.
5. Student identity remains C-owned.
6. Workforce identity remains E-owned.
7. Notification delivery remains H-owned.
8. Follow-up, referral, observation and prescription remain separate G06 healthcare workflows.
9. Healthcare history must remain traceable.
10. Significant healthcare activity must be audited.

The scenario does not invent medical screening thresholds, laboratory interpretation rules, fitness criteria, disease classifications, or mandatory referral criteria.

---

## 13. State and Lifecycle Impact

The authoritative Annual Health Checkup lifecycle is:

`Scheduled → Conducted → Completed`

### Scheduled

The Annual Health Checkup has been established for the applicable institutional assessment activity.

### Conducted

The assessment activity has been performed and the relevant assessment outcome is being brought to completion.

### Completed

The Annual Health Checkup's completion condition is satisfied and its outcome is recorded.

No additional state is introduced.

Individual healthcare actions resulting from findings use their canonical G06 aggregate lifecycles.

---

## 14. Business Events and Cross-Domain Collaboration

The L2-G06 event catalogue explicitly identifies:

`HealthCheckCompleted`

as the canonical event for completed annual health assessment activity.

G06 is the canonical publisher because it owns Annual Health Checkup.

C and E provide authoritative participant/personnel context.

H may consume the completed healthcare fact for configured notification/service purposes.

The scenario does not invent an event for `Scheduled` or `Conducted` because the retrieved L2 event catalogue does not explicitly identify those event names.

Where an immediate authoritative participant/context answer is required, the interaction remains a business-level cross-domain hand-off; it does not transfer ownership.

---

## 15. Business Authorization, Approval and Audit Obligations

Annual Health Checkups must be conducted under the authority of authorised medical professionals and configured healthcare responsibilities.

Audit evidence should preserve:

- Annual Health Checkup.
- Participant context.
- Responsible medical professional.
- Assessment activity.
- Findings.
- Recommendations.
- Follow-up/referral/observation/prescription decisions where established.
- Lifecycle transitions.
- `HealthCheckCompleted`.
- Relevant completion outcome.

Medical confidentiality applies to individual health assessment information.

No unsupported approval chain or separation-of-duty rule is introduced.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Annual Health Checkup established.
- Participant population/context identified.
- Required healthcare personnel established.
- Assessment conducted.
- Findings recorded.
- Recommendations recorded where applicable.
- Required follow-up healthcare activities established.
- Annual Health Checkup reaches `Conducted`.
- Completion condition satisfied.
- Annual Health Checkup reaches `Completed`.
- `HealthCheckCompleted` is published by G06.
- Assessment history and audit evidence are preserved.

### Postconditions

- The Annual Health Checkup is completed.
- The participant's institutional healthcare history reflects the recorded assessment outcome.
- Any resulting follow-up remains separately governed by G06.
- C/E ownership remains unchanged.
- H may consume the completed-health-check business fact without acquiring healthcare ownership.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. L2-G06 does not define detailed clinical assessment criteria for Annual Health Checkups.
2. The exact evidence required to establish the `Conducted` transition is not fully enumerated beyond the lifecycle definition.
3. The L2 identifies `HealthCheckCompleted` but does not enumerate separate events for scheduling or conducting the assessment.

### Assumptions

- Annual Health Checkup is the G06-owned aggregate identified by L2-G06.
- `HealthCheckCompleted` is the canonical completion event.
- Individual findings that require further care are handled through existing G06 healthcare workflows.

### Deferred Questions

If institution-specific annual health assessment criteria, mandatory clinical tests, or regulatory reporting obligations are required, confirm their authoritative upstream source before Architecture Approval/Lock.
