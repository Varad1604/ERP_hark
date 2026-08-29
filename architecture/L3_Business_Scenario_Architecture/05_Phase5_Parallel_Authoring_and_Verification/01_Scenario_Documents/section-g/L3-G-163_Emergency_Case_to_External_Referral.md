# Emergency Case to External Referral

**Scenario ID:** L3-G-163  
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
| Final L3 ID | L3-G-163 |
| Canonical Title | Emergency Case to External Referral |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which an emergency case is assessed, immediate institutional medical response is provided, the need for external healthcare is determined, and an external referral is established and handed over as the authoritative next-care pathway.

The locked L2 defines Emergency & First Aid Management as the institutional response to medical incidents, with the emergency flow:

`Incident Reported → Emergency Assessment → Immediate Treatment → Referral Decision → Patient Stabilised → Case Closed`.

It also states that Institutional Health Services may refer patients to external hospitals, while hospital management remains outside G06.

## 3. Business Outcome

The emergency case has received the required institutional response, an external referral has been appropriately determined, the receiving external healthcare pathway has been established to the extent supported by institutional operations, and the institutional emergency case has a traceable referral outcome.

G06 does not become owner of the external hospital's care.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An active Emergency Case reaches a point where institutional assessment determines that external healthcare is required.

### 4.2 End Boundary

The external referral is established and the institutional emergency case has reached its applicable terminal/closed business outcome.

### 4.3 In Scope

- Emergency case identification.
- Emergency assessment.
- Immediate institutional treatment.
- Referral decision.
- External healthcare provider identification.
- Referral reason and referral status.
- Emergency-case-to-referral association.
- Patient stabilisation within institutional scope.
- Referral handoff/coordination.
- Audit and medical history continuity.

### 4.4 Out of Scope

- External hospital admission.
- Surgery, ICU, inpatient care or hospital treatment.
- External hospital billing.
- Insurance settlement.
- External clinical records ownership.
- Ambulance fleet management.
- Technical communication mechanisms.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

Owns Emergency Case, emergency treatment, referral decision and institutional Referral.

The L2 identifies Emergency Case and Referral as G06 aggregates and states that hospital ownership remains external.

### Section C — Student Lifecycle

Provides student identity/context where the patient is a student. C retains ownership of student identity.

### Section E — People & Workforce

Provides healthcare-professional/personnel context. E retains personnel ownership.

### Section H — Notifications

Provides notification delivery where configured; H does not own the emergency or referral outcome.

## 6. Business Actors, Roles and Responsibilities

### Medical Professional / Medical Staff

Performs emergency assessment, immediate treatment and referral decision within professional authority.

### Patient

Receives emergency care and participates in referral arrangements where possible.

### External Healthcare Provider

Receives the referral as the external care destination. The external provider's internal workflow remains outside G06.

### Student/Employee Authority

C/E provide authoritative identity/context where applicable.

No technical role is invented.

## 7. Preconditions and Entry Criteria

1. An Emergency Case exists.
2. Emergency assessment has established the need for care beyond the institutional scope.
3. The patient identity/context is sufficiently established.
4. An authorised medical professional can make the referral decision.
5. The applicable external healthcare provider/referral policy is available.

## 8. Trigger

Emergency assessment determines that external healthcare referral is required.

## 9. Main Business Flow

### Phase A — Emergency Assessment and Immediate Response

1. Confirm the Emergency Case and patient.
2. Perform emergency assessment.
3. Classify the emergency according to the applicable configured category/policy.
4. Provide immediate institutional treatment within permitted scope.
5. Determine whether institutional care is sufficient.

### Phase B — Referral Decision

6. Determine that external healthcare is required.
7. Establish the referral reason.
8. Identify the appropriate external healthcare provider/destination.
9. Record the referral against the Emergency Case/Medical Encounter.
10. Establish the Referral as `Recommended`.

The G06 Referral lifecycle is `Recommended → Accepted → Completed`.

### Phase C — Referral Coordination

11. Communicate the referral information through the applicable authorised business process.
12. Confirm the receiving provider/pathway where the institutional policy requires confirmation.
13. Preserve the relationship between Emergency Case, patient and Referral.
14. Where applicable, coordinate emergency notifications through H.

### Phase D — Stabilisation and Case Closure

15. Stabilise the patient within institutional capability and applicable medical authority.
16. Record the referral outcome and relevant emergency treatment.
17. Close the institutional emergency case when the applicable closure conditions are satisfied.
18. Preserve the complete auditable emergency history.
19. End the scenario.

The external hospital's subsequent treatment is not part of this L3 boundary.

## 10. Alternate Business Flows

### 10.1 Referral Accepted

The receiving provider accepts the referral and the referral proceeds toward external care.

### 10.2 Referral Provider Changed

If the originally considered provider cannot accept the patient, another appropriate external provider is selected under applicable policy.

### 10.3 Patient Stabilised Before Handoff

Institutional care continues until the patient is stable enough for the applicable referral handoff.

### 10.4 Student Emergency

C provides student context; G06 remains owner of the emergency and referral.

### 10.5 Hostel Emergency

Where the incident originates in hostel operations, G03 may provide residency context; G06 remains owner of the medical outcome.

### 10.6 Referral Not Required

If reassessment establishes that institutional care is sufficient, the case follows the normal emergency closure path rather than creating an external referral.

## 11. Failure, Interruption and Business Recovery

### 11.1 External Provider Cannot Accept

The referral remains unresolved and an alternative authorised provider is considered.

### 11.2 Provider Information Is Insufficient

The referral is not treated as completed until the required business information is established.

### 11.3 Patient Identity Cannot Be Confirmed

The referral is not attached to an invented patient.

### 11.4 Emergency Condition Worsens

Emergency care remains the immediate business priority; the referral decision may be revisited by the authorised medical professional.

### 11.5 Referral Handoff Is Not Confirmed

The institutional case remains in the applicable unresolved/referral condition rather than being falsely marked complete.

### 11.6 Recovery

Once the missing business condition is resolved, the referral resumes from the unresolved business boundary.

No technical retries, queues, rollbacks, APIs, database mechanics or infrastructure behaviour are specified.

## 12. Business Rules and Decision Constraints

The locked L2 establishes:

- Institutional Health Services owns Emergency Care and Referral Management.
- Hospital management remains outside G06.
- External hospital referral is supported.
- Emergency categories remain configurable.
- Medical professionals retain operational authority.
- Every healthcare activity is auditable.

The Referral aggregate owns referral reason, external provider, referral status and referral history.

No institution-specific provider-selection matrix is invented.

## 13. State and Lifecycle Impact

Emergency Case follows the L2 lifecycle:

`Reported → Assessment → Treatment → Referral (Optional) → Closed`.

Referral follows:

`Recommended → Accepted → Completed`.

This scenario may cross the emergency/referral boundary but does not own the external hospital's lifecycle.

## 14. Business Events and Cross-Domain Collaboration

The retrieved locked L2 establishes that significant healthcare activities publish immutable domain events and identifies the Domain Events section as the authoritative event source.

For this draft, no new event name is invented for external referral.

The canonical publisher of Emergency Case and Referral business facts remains G06 because G06 owns those aggregates.

H may consume configured emergency/referral facts for notification delivery; it does not publish the medical outcome.

### Event-definition gap

The exact canonical event name for external-referral creation/completion is not exposed in the authoritative content retrieved for this drafting pass. It is therefore recorded as an upstream verification item rather than fabricated.

## 15. Business Authorization, Approval and Audit Obligations

The emergency referral decision must be made by an appropriately authorised medical professional under the configurable healthcare authorization model.

Medical confidentiality and need-to-know obligations apply.

Audit evidence should preserve:

- Emergency Case identity.
- Patient context.
- Emergency assessment.
- Immediate treatment.
- Referral decision.
- Referral reason.
- External provider.
- Referral status.
- Case closure.
- Responsible medical professional.

Every lifecycle transition must be audited.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Emergency Case identified.
2. Emergency assessment completed.
3. Immediate institutional response provided.
4. External referral decision established.
5. Referral reason recorded.
6. External provider/destination established.
7. Referral entered into its authoritative lifecycle.
8. Patient stabilisation/closure condition established.
9. Institutional emergency history audited.
10. External hospital ownership remains outside G06.

### Postconditions

- Referral is traceable to the originating emergency/medical encounter.
- G06 retains institutional referral history.
- External hospital care remains externally owned.
- No false external-care completion is recorded.
- Medical history remains auditable and traceable.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. Exact external-provider selection criteria are configurable/not fully enumerated in the retrieved L2.
2. Exact canonical event name(s) for external referral are not exposed in the retrieved authoritative excerpt.
3. Exact acceptance evidence required from an external provider is not defined.

### Assumptions

- G06 owns the institutional Referral aggregate.
- External hospital care remains outside G06.
- Medical professionals retain referral authority.

### Deferred Questions

Verify the canonical external-referral event name and any mandatory provider-acceptance evidence against the complete locked L2 event/contract definitions before Architecture Approval/Lock.
