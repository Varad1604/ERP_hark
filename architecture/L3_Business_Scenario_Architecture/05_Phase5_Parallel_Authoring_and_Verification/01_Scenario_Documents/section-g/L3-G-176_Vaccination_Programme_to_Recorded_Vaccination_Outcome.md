# Vaccination Programme to Recorded Vaccination Outcome

**Scenario ID:** L3-G-176  
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

Institutional vaccination programmes coordinate organised immunisation activity while preserving the institution's ownership of its own vaccination records and healthcare history. The scenario ensures that planned vaccination activity becomes an operational campaign, participants receive the applicable healthcare activity, and completed vaccination outcomes are recorded under G06.

The scenario does not define vaccine-specific clinical policy, dosage rules, legal requirements, or external public-health system ownership unless such rules are explicitly supplied by upstream architecture.

---

## 3. Business Outcome

The institutional vaccination programme has been conducted, vaccination outcomes for the applicable participants have been recorded, the campaign has reached its authoritative `Completed` lifecycle state, and the resulting healthcare facts are available as governed institutional medical history.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An institutional requirement for a vaccination programme is authorised for establishment under G06.

### 4.2 End Boundary

The vaccination campaign has been completed and applicable vaccination outcomes have been recorded.

### 4.3 In Scope

- Vaccination Campaign establishment.
- Programme planning.
- Participant context.
- Healthcare professional involvement.
- Vaccination activity.
- Recording vaccination outcomes.
- Campaign completion.
- Healthcare history.
- Student and employee context.
- Notification participation where configured.
- Audit.

### 4.4 Out of Scope

- External vaccination registry ownership.
- Government/public-health programme ownership.
- Vaccine procurement ownership unless explicitly governed elsewhere.
- Financial accounting.
- Student identity ownership.
- Employee identity ownership.
- Technical scheduling or inventory implementation.
- Clinical vaccine protocols not defined by G06.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns Vaccination Campaign and the institutional vaccination outcome.

### Section C — Student Lifecycle

C provides authoritative student identity/context where student vaccination participation is involved.

### Section E — People & Workforce

E provides authoritative workforce/personnel context where employee vaccination participation is involved.

### Section H — Institutional Services & Intelligence

H may provide configured notification services and downstream service projections. It does not own vaccination records.

---

## 6. Business Actors, Roles and Responsibilities

### Medical Administrator

Coordinates the institutional vaccination campaign.

### Authorised Medical Professional / Medical Staff

Conducts or oversees the healthcare activity and records the vaccination outcome within authorised responsibility.

### Participant

Participates in the vaccination programme and provides required information/consent where such requirements are governed by applicable policy.

### Section C / Section E

Provide authoritative participant identity/context.

### Notification Services

H may communicate configured programme or vaccination notifications.

No unsupported external actor, consent authority or legal role is introduced as a canonical role.

---

## 7. Preconditions and Entry Criteria

1. The vaccination requirement is within institutional healthcare scope.
2. A Vaccination Campaign can be established under G06.
3. Participant identity/context is available from the appropriate canonical source.
4. Appropriate medical staff can conduct the vaccination activity.
5. The applicable vaccination programme policy/configuration is available.
6. Any required institutional healthcare authorization is satisfied.

---

## 8. Trigger

An authorised institutional vaccination requirement is established and a vaccination campaign is ready to be planned.

---

## 9. Main Business Flow

### Phase A — Plan the Campaign

1. Identify the vaccination programme requirement.
2. Establish the Vaccination Campaign under G06 ownership.
3. Record the campaign purpose and applicable programme context.
4. Establish the intended participant population where applicable.
5. Establish the required healthcare personnel.
6. Prepare the campaign under the authoritative `Planned` state.

### Phase B — Start the Campaign

7. Confirm that the campaign is ready for operational conduct.
8. Establish the campaign as `Running`.
9. Identify participants through authoritative C/E context as applicable.
10. Confirm participant context before recording an individual vaccination outcome.
11. Conduct vaccination activity under authorised medical responsibility.

### Phase C — Record Vaccination Outcomes

12. For each participant receiving the institutional vaccination activity, record the applicable vaccination outcome.
13. Associate the outcome with the correct participant and institutional healthcare history.
14. Preserve the vaccination record as an authoritative G06 healthcare record.
15. Publish `VaccinationRecorded` for each material recorded vaccination outcome.
16. Where a participant requires separate healthcare follow-up, route that need into the applicable G06 healthcare workflow rather than creating a new vaccination-campaign state.

### Phase D — Complete the Campaign

17. Confirm that the campaign's configured completion condition is satisfied.
18. Record the campaign outcome.
19. Transition the Vaccination Campaign to `Completed`.
20. Preserve the completed campaign and vaccination history.
21. Where configured, make relevant facts available to H for notification/service consumption.
22. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Participant Does Not Receive Vaccination

Where a participant does not receive the vaccination activity, the campaign records the applicable business outcome only if the authoritative programme policy supports such an outcome. No new status or reason is invented.

### 10.2 Participant Requires Further Healthcare Attention

If the vaccination activity produces a need for additional healthcare attention, the participant is routed to the applicable G06 healthcare workflow. The vaccination campaign lifecycle remains separate.

### 10.3 Student Vaccination

C provides student identity/context. G06 owns the vaccination record and healthcare outcome.

### 10.4 Employee Vaccination

E provides employee/personnel context. G06 owns the vaccination record and healthcare outcome.

### 10.5 Campaign Conducted Across Multiple Participant Groups

The campaign remains one G06 Vaccination Campaign; participant domain membership does not transfer ownership.

### 10.6 Campaign Completion With Outstanding Individual Follow-up

The campaign may complete if its own completion criteria are met while individual healthcare follow-up continues separately under the appropriate G06 lifecycle.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Participant identity cannot be verified | Do not attach vaccination outcome to an unverified participant. Resolve authoritative context first. |
| Medical staff cannot conduct the activity | Preserve the campaign in its current valid lifecycle position and resolve the operational healthcare assignment. |
| Campaign activity is interrupted | Preserve the current business lifecycle position and resume or resolve under applicable campaign policy. |
| Individual vaccination outcome is uncertain | Do not record an unsupported completed vaccination outcome; resolve the healthcare assessment first. |
| Participant requires additional care | Route to the relevant G06 healthcare workflow. |
| Campaign completion condition is not met | Do not transition the campaign to `Completed`. |
| Campaign is no longer required | Apply any authoritative campaign closure/cancellation policy if defined upstream; do not invent a new lifecycle state. |

No technical error handling or infrastructure recovery is defined.

---

## 12. Business Rules and Decision Constraints

1. Vaccination Campaign is owned by G06.
2. Vaccination Campaign lifecycle is `Planned → Running → Completed`.
3. `VaccinationRecorded` is the canonical vaccination event identified by L2-G06.
4. Vaccination records are healthcare records owned by G06.
5. Student identity remains C-owned.
6. Workforce identity remains E-owned.
7. Notification delivery remains H-owned.
8. Medical professionals retain healthcare authority.
9. Individual follow-up remains separately governed by the relevant healthcare workflow.
10. Healthcare activity remains auditable.

The scenario does not invent vaccine eligibility criteria, dosage schedules, consent rules, medical contraindications, or public-health reporting obligations.

---

## 13. State and Lifecycle Impact

The authoritative Vaccination Campaign lifecycle is:

`Planned → Running → Completed`

### Planned

Campaign established and being prepared.

### Running

Campaign is actively conducting vaccination activity.

### Completed

Campaign's authoritative completion condition has been met and campaign outcome has been recorded.

Individual vaccination records are outcomes associated with campaign activity and do not create additional campaign lifecycle states.

---

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of vaccination business facts.

The L2-G06 event catalogue explicitly defines:

`VaccinationRecorded`

This event represents the fact that a vaccination outcome has been recorded within institutional healthcare.

C and E provide participant context where applicable.

H may consume G06 vaccination facts for configured notifications or institutional service purposes.

No generic campaign-start or campaign-completed event is invented because the retrieved L2 event catalogue does not explicitly define those names for Vaccination Campaign.

Events remain distinct from any synchronous participant-context hand-off.

---

## 15. Business Authorization, Approval and Audit Obligations

Vaccination activity must be conducted by appropriately authorised healthcare personnel within the institution's healthcare authority.

Audit evidence should preserve:

- Vaccination Campaign.
- Participant.
- Healthcare activity.
- Responsible medical professional.
- Vaccination outcome.
- Relevant date/time business fact where defined by the healthcare record.
- Campaign lifecycle transition.
- `VaccinationRecorded` event.
- Follow-up healthcare action where applicable.

Medical confidentiality applies to individual vaccination information.

The scenario does not invent an additional consent or approval authority where upstream architecture does not define one.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Vaccination Campaign established.
- Campaign reaches `Running`.
- Applicable participants are identified.
- Vaccination activities are conducted under authorised healthcare responsibility.
- Vaccination outcomes are recorded.
- `VaccinationRecorded` is published for recorded vaccination outcomes.
- Campaign completion condition is satisfied.
- Campaign reaches `Completed`.
- Vaccination history remains traceable.
- Configured downstream notification/service consumption can occur.

### Postconditions

- The vaccination campaign is completed.
- Recorded vaccination outcomes are part of G06 institutional healthcare history.
- Participant identity ownership remains with C/E.
- H remains responsible for downstream notification/service behaviour.
- Any individual follow-up remains separately governed.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. L2-G06 does not enumerate vaccine-specific clinical, eligibility, consent or public-health reporting rules.
2. The exact campaign completion evidence is not fully enumerated in the retrieved L2 state model.

### Assumptions

- `VaccinationRecorded` is the canonical event for each recorded vaccination outcome.
- Vaccination records are owned by G06.
- Campaign lifecycle remains exactly `Planned → Running → Completed`.

### Deferred Questions

If an institution requires a specific vaccine eligibility/consent policy or external public-health reporting obligation, confirm the authoritative upstream source before encoding it in L3.
