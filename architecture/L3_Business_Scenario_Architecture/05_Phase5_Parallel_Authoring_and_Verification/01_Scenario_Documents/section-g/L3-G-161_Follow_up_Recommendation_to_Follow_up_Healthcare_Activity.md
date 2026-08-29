# Follow-up Recommendation to Follow-up Healthcare Activity

**Scenario ID:** L3-G-161  
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
| Final L3 ID | L3-G-161 |
| Canonical Title | Follow-up Recommendation to Follow-up Healthcare Activity |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional healthcare outcome in which a follow-up recommendation is established from a healthcare encounter, the appropriate follow-up activity is identified and coordinated, and the resulting healthcare activity is recorded within the patient's ongoing medical context.

The locked L2 states that follow-up recommendations may include Return Visit, Referral, Observation, Medical Certificate and Health Program Participation, and that follow-up policies remain configurable. It defines `FollowUpRecommended` as the canonical event for a follow-up recommendation.

Because the canonical scenario is broader than a single return visit, this L3 treats the recommendation as the entry condition and routes the recommendation into the appropriate authoritative follow-up activity without inventing a universal follow-up lifecycle.

## 3. Business Outcome

A valid follow-up recommendation has been established and the applicable healthcare follow-up activity has been initiated or completed according to its own authoritative workflow, with the resulting healthcare context preserved.

The scenario does not force every recommendation into the same follow-up type.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A healthcare encounter produces a follow-up recommendation.

### 4.2 End Boundary

The recommended follow-up activity reaches its defined business completion boundary, or the recommendation is explicitly resolved through the applicable healthcare pathway.

### 4.3 In Scope

- Follow-up recommendation.
- Recommendation type.
- Patient/Medical Encounter context.
- Return visit.
- Referral.
- Observation.
- Medical certificate.
- Health-program participation.
- Cross-domain context.
- Follow-up recording.
- Patient medical-history continuity.
- Appropriate notification handoff.

### 4.4 Out of Scope

- Re-performing the original consultation.
- Creating a universal follow-up lifecycle.
- Referral completion details beyond the referral workflow.
- Observation-care execution beyond the observation workflow.
- Medical certificate issuance beyond its authoritative workflow.
- Health-program execution beyond its authoritative workflow.
- Technical scheduling, notification or messaging implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns follow-up recommendations and healthcare operational history.

### Section C — Student Lifecycle

C provides student context where the patient is a student.

### Section E — People & Workforce

E provides employee/personnel context and healthcare professional context where required.

### Section H — Notifications

H handles notification delivery where configured.

Participation does not transfer ownership of healthcare history or student/employee identity.

## 6. Business Actors, Roles and Responsibilities

### Medical Professional

Establishes the follow-up recommendation based on professional clinical judgement.

### Medical Staff

Coordinates/records the applicable follow-up healthcare activity.

### Patient

Participates in the recommended follow-up activity.

### C / E

Provide authoritative institutional context where required.

### H

Delivers configured follow-up notifications.

## 7. Preconditions and Entry Criteria

1. A Medical Encounter exists.
2. A healthcare professional has established a follow-up recommendation.
3. The recommendation is recorded with sufficient patient/encounter context.
4. The applicable follow-up policy is available.
5. The recommended follow-up type can be determined.

## 8. Trigger

A healthcare encounter produces a follow-up recommendation.

## 9. Main Business Flow

### Phase A — Establish Recommendation

1. Identify the patient and Medical Encounter.
2. Record the follow-up recommendation.
3. Determine the recommended follow-up type.
4. Apply the configured follow-up policy.
5. Publish `FollowUpRecommended`.

The L2 explicitly defines this event as “Follow-up advised.”

### Phase B — Determine Follow-up Path

6. If the recommendation is a Return Visit, establish the applicable return-visit activity.
7. If the recommendation is a Referral, enter the Referral workflow.
8. If the recommendation is Observation, enter the Observation Care workflow.
9. If the recommendation is a Medical Certificate, enter the Medical Certificate workflow.
10. If the recommendation is Health Program Participation, enter the applicable Health Program workflow.

### Phase C — Execute the Applicable Healthcare Activity

11. Maintain the association with the original Medical Encounter.
12. Execute the selected follow-up activity according to its authoritative lifecycle.
13. Record the resulting healthcare outcome.

### Phase D — Preserve Healthcare Continuity

14. Preserve the resulting follow-up outcome in the patient's healthcare history.
15. Maintain immutable medical history.
16. Where configured, provide the appropriate business event to H for notification delivery.

### Phase E — Complete

17. End when the selected follow-up pathway reaches its own defined completion boundary.

The scenario does not create one generic “Follow-up Completed” state or event because the L2 does not define one.

## 10. Alternate Business Flows

### 10.1 Return Visit

The patient returns for additional healthcare activity. The new activity follows the applicable consultation/healthcare workflow.

### 10.2 Referral

The recommendation enters the Referral lifecycle:

`Recommended → Accepted → Completed`.

Referral completion remains a separate business outcome.

### 10.3 Observation

The recommendation enters Observation Care:

`Admitted → Observed → Released`.

### 10.4 Medical Certificate

The recommendation enters the Medical Certificate workflow:

`Medical Assessment → Eligibility Verified → Certificate Generated → Authorised Approval → Certificate Issued`.

### 10.5 Health Program Participation

The recommendation enters the applicable Health Program workflow.

### 10.6 Multiple Follow-up Recommendations

Where policy permits multiple recommendations, each follows its applicable authoritative pathway. No duplicate generic follow-up lifecycle is created.

## 11. Failure, Interruption and Business Recovery

### 11.1 Follow-up Type Cannot Be Established

The recommendation remains unresolved until applicable policy/clinical authority establishes the path.

### 11.2 Patient Context Cannot Be Confirmed

The follow-up activity does not proceed against an invented patient.

### 11.3 Referral Cannot Progress

The recommendation remains associated with the originating Medical Encounter while the Referral workflow handles its own business condition.

### 11.4 Observation Cannot Be Initiated

The recommendation remains unresolved until Observation Care can establish its own entry condition.

### 11.5 Medical Certificate Eligibility Not Established

Certificate issuance does not occur against an invented eligibility outcome.

### 11.6 Health Program Participation Unavailable

The follow-up recommendation remains recorded; the programme-specific workflow determines the appropriate outcome.

### 11.7 Recovery

Once the applicable business condition is resolved, the recommendation resumes into its authoritative follow-up path.

No technical retry, queue, rollback, API, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The L2 explicitly states:

- Follow-up recommendations may include Return Visit, Referral, Observation, Medical Certificate and Health Program Participation.
- Follow-up policies remain configurable.
- Medical professionals retain operational authority.
- Every significant healthcare activity publishes immutable business events.

The scenario must not invent a single universal completion rule for all follow-up types.

The G06 invariants remain applicable:

- Every Medical Encounter references exactly one Patient.
- Referral references one Medical Encounter.
- Medical certificates reference one authorised Medical Encounter.
- Medical history is never deleted.
- Every lifecycle transition is audited.

## 13. State and Lifecycle Impact

No generic Follow-up lifecycle is created.

The scenario transitions from:

`Medical Encounter → Follow-up Recommended → Applicable Follow-up Workflow`

The applicable downstream lifecycle remains authoritative:

- Referral: `Recommended → Accepted → Completed`.
- Observation: `Admitted → Observed → Released`.
- Medical Certificate: `Draft → Approved → Issued → Archived`.
- Other paths retain their own upstream lifecycle.

## 14. Business Events and Cross-Domain Collaboration

The canonical event is:

**`FollowUpRecommended` — Follow-up advised.**

G06 is the canonical publisher.

The L2 does not define a generic `FollowUpCompleted` event.

Therefore this scenario does not invent one.

Where the selected follow-up pathway has its own canonical events, those events remain owned/published by the corresponding G06 lifecycle.

H handles notification delivery where configured.

C/E provide identity/personnel context and do not acquire ownership of the Medical Encounter or follow-up recommendation.

## 15. Business Authorization, Approval and Audit Obligations

The follow-up recommendation is established by an appropriately authorised healthcare professional under configurable healthcare authorization policy. The L2 identifies Doctor as a typical consultation authority and requires medical confidentiality.

Audit evidence should preserve:

- Originating Medical Encounter.
- Follow-up recommendation.
- Recommendation type.
- Responsible healthcare professional.
- Applicable policy/context.
- Downstream follow-up outcome.
- Relevant lifecycle transitions.

Every lifecycle transition is audited.

No technical authorization model is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Originating Medical Encounter identified.
2. Follow-up recommendation established.
3. `FollowUpRecommended` published.
4. Follow-up type determined.
5. Appropriate authoritative workflow entered.
6. Follow-up outcome recorded.
7. Healthcare history preserved.
8. Applicable downstream lifecycle reaches its defined completion boundary or explicit unresolved outcome.

### Postconditions

- Follow-up recommendation remains traceable to the originating Medical Encounter.
- The applicable healthcare activity has its own authoritative business outcome.
- Medical history remains immutable.
- No generic follow-up lifecycle or event has been invented.
- Student/employee identity ownership remains with C/E.
- Notification delivery remains H-owned.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. Follow-up policies are configurable and no universal selection matrix is defined.
2. The L2 does not define a generic Follow-up aggregate lifecycle.
3. The L2 does not define a generic FollowUpCompleted event.

### Assumptions

- Follow-up recommendation remains G06-owned.
- Each follow-up type uses its own authoritative workflow.
- `FollowUpRecommended` is the canonical recommendation event.

### Deferred Questions

Confirm institution-specific follow-up selection and completion policies before Architecture Approval/Lock.
