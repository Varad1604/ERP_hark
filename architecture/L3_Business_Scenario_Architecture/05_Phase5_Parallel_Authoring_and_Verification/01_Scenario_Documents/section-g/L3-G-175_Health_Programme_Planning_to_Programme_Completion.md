# Health Programme Planning to Programme Completion

**Scenario ID:** L3-G-175  
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

Institutional health programmes provide organised preventive and community-oriented healthcare activities rather than individual episodic treatment. The institution needs a governed business process through which a healthcare programme is planned, made active, conducted with its intended participants, its relevant findings/outcomes are recorded, and the programme is brought to completion.

The scenario covers the institutional business outcome only. It does not define clinical protocols, implementation mechanisms, technical scheduling, notification delivery technology, or external healthcare ownership.

---

## 3. Business Outcome

The planned institutional Health Programme has been conducted within its authorised scope, the materially required programme participation and outcomes have been recorded, and the Health Programme has reached its authoritative **Completed** lifecycle state.

The completed programme remains traceable as an institutional healthcare activity and can support permitted downstream reporting, notification and healthcare-history use.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A healthcare programme requirement has been identified and is ready to be established as a Health Programme under G06 ownership.

### 4.2 End Boundary

The programme has been conducted, required programme outcomes have been recorded, and the Health Program is in the authoritative `Completed` state.

### 4.3 In Scope

- Health Programme establishment and planning.
- Programme scope and purpose.
- Applicable programme type.
- Programme participation.
- Healthcare professional/staff involvement.
- Programme conduct.
- Recording of programme findings/outcomes.
- Follow-up recommendation where supported by the programme outcome.
- Programme completion.
- Student and employee context consumption.
- Notification participation through H where configured.
- Healthcare audit and reporting traceability.

### 4.4 Out of Scope

- Ownership of student identity.
- Ownership of employee/workforce identity.
- External healthcare provider operations.
- Financial accounting.
- Clinical protocols not defined by G06.
- Technical scheduling, APIs, databases, UI, infrastructure or deployment.
- Independent ownership of notification delivery.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the Health Program aggregate and the complete lifecycle of the institutional healthcare programme.

### Section C — Student Lifecycle

C provides authoritative student identity/context where students participate. G06 does not recreate or own student identity.

### Section E — People & Workforce

E provides authoritative workforce/personnel context, including healthcare staff context where required. G06 performs healthcare operational assignments but does not become owner of workforce records.

### Section H — Institutional Services & Intelligence

H may consume business facts for notification and institutional service purposes. H does not own the Health Programme or its outcome.

---

## 6. Business Actors, Roles and Responsibilities

### Medical Administrator

Coordinates establishment and operational governance of the Health Programme within G06 authority.

### Chief Medical Officer / Authorised Medical Professional

Provides healthcare authority for programme scope and healthcare outcomes where professional judgement is required.

### Medical Staff

Conducts healthcare programme activities and records relevant programme outcomes within assigned responsibilities.

### Programme Participant

Participates in the institutional health programme and provides the information required for the healthcare activity.

### Section C / Section E Representatives

Provide authoritative student or workforce context where required by the programme.

### Notification Services

H provides notification delivery where configured. It does not decide programme state.

No additional actor or approval role is invented where the upstream architecture does not define one.

---

## 7. Preconditions and Entry Criteria

1. The programme requirement is within institutional healthcare scope.
2. G06 is the appropriate owner of the programme.
3. The programme can be represented as a Health Program.
4. Relevant participants can be identified from authoritative institutional context where participation is required.
5. Required healthcare personnel can be assigned within authorised G06 operational responsibility.
6. Applicable programme configuration is available.
7. Any required institutional approvals are governed by the applicable upstream policy rather than invented locally.

---

## 8. Trigger

An authorised requirement for an institutional health programme is established and the Health Programme is ready to be planned.

---

## 9. Main Business Flow

### Phase A — Establish the Programme

1. Identify the institutional healthcare requirement.
2. Determine that the requirement is appropriately handled as a Health Programme.
3. Establish the Health Programme under G06 ownership.
4. Record the programme purpose and applicable programme category.
5. Establish the planned programme scope.
6. Identify the intended participant population or participation context where applicable.
7. Identify the healthcare personnel required to conduct the programme.
8. Establish the programme in the authoritative `Planned` lifecycle state.

### Phase B — Prepare for Programme Activity

9. Confirm that the planned programme is ready to become active under applicable healthcare policy.
10. Establish the operational programme arrangements that are within G06 business scope.
11. Where student participation is required, consume authoritative student context from C.
12. Where employee participation is required, consume authoritative workforce context from E.
13. Confirm the healthcare activity can be conducted within the institution's configured healthcare capability.
14. Move the Health Programme to `Active` when the authoritative activation condition is satisfied.

### Phase C — Conduct the Programme

15. Conduct the healthcare programme.
16. Register or otherwise recognise relevant participant activity according to the programme's configured participation model.
17. Perform the healthcare activities applicable to the programme type.
18. Record relevant programme findings and outcomes.
19. Where a programme finding requires individual healthcare attention, establish the appropriate follow-up healthcare activity through the relevant G06 workflow rather than extending the Health Programme lifecycle with a new state.
20. Preserve the relationship between programme activity and its participant/context.

### Phase D — Complete the Programme

21. Confirm that the planned programme activity has been completed.
22. Confirm that materially required programme outcomes have been recorded.
23. Record the final programme outcome.
24. Transition the Health Programme to the authoritative `Completed` state.
25. Preserve the completed programme as institutional healthcare history.
26. Publish the applicable canonical G06 programme business facts.
27. Where configured, make relevant facts available to H for downstream notification or institutional service consumption.
28. End the scenario at programme completion.

---

## 10. Alternate Business Flows

### 10.1 Programme Conducted With No Individual Follow-up

The programme is completed and its aggregate outcome is recorded without creating additional individual healthcare activity.

### 10.2 Programme Finding Requires Individual Follow-up

A programme finding identifies a participant requiring additional healthcare attention. The participant is routed into the applicable G06 follow-up, consultation, observation or referral workflow. The Health Programme itself remains governed by its own lifecycle.

### 10.3 Student-focused Programme

Student identity/context is consumed from C. G06 retains ownership of the programme and healthcare outcome.

### 10.4 Employee-focused Programme

Workforce/personnel context is consumed from E. G06 retains ownership of the programme and healthcare outcome.

### 10.5 Programme Includes Multiple Participant Groups

Where configuration permits different participant categories, the programme remains one G06 Health Program. Participation context does not transfer ownership of the programme to C or E.

### 10.6 Programme Completion With Outstanding Follow-up

The programme may reach completion while an individually required follow-up remains governed by its separate healthcare workflow, provided the programme's own completion criteria are satisfied. The follow-up must not be represented as a hidden Health Program lifecycle state.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Programme cannot be conducted within its authorised scope | Keep the programme in its valid lifecycle position and resolve the business constraint before completion. |
| Required participant context is unavailable | Resolve authoritative participant context before recording affected healthcare outcomes. |
| Required healthcare professional is unavailable | Re-establish an authorised operational assignment or otherwise follow the applicable healthcare operational policy. |
| Programme activity is interrupted before completion | Preserve the programme in its current valid lifecycle position and resume or otherwise resolve it under the applicable business policy. |
| Programme finding requires additional healthcare | Route the affected participant to the applicable G06 healthcare workflow. |
| Programme outcome is incomplete | Do not mark the Health Program `Completed` until its authoritative completion criteria are satisfied. |
| Programme is determined to be no longer required before activation | Apply the applicable upstream programme cancellation/closure policy if one exists; do not invent a new lifecycle state locally. |

No technical retries, queues, rollback, server failures or implementation exceptions are described.

---

## 12. Business Rules and Decision Constraints

1. Health Program is owned by G06.
2. Health Program lifecycle is `Planned → Active → Completed`.
3. Health programmes remain configuration-driven.
4. Medical professionals retain operational healthcare authority.
5. Student identity remains C-owned.
6. Workforce identity remains E-owned.
7. Notification delivery remains H-owned.
8. Programme findings that require individual healthcare treatment must be handled through the relevant G06 healthcare workflow.
9. The programme must not be extended with locally invented lifecycle states.
10. Significant healthcare activity must remain auditable.

The scenario does not invent programme-specific clinical criteria, participant eligibility thresholds, programme duration, approval chains, or completion metrics.

---

## 13. State and Lifecycle Impact

The authoritative Health Program lifecycle from L2-G06 is:

`Planned → Active → Completed`

### Planned

The Health Programme has been established and is being prepared for conduct.

### Active

The programme is operational and its healthcare activities are being conducted.

### Completed

The programme has reached its authoritative completion outcome and its relevant results have been recorded.

No additional lifecycle state is introduced.

Individual healthcare follow-up, observation, referral or prescription outcomes remain governed by their own G06 aggregates and lifecycles.

---

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of Health Program business facts because it owns the Health Program aggregate.

The L2-G06 event catalogue identifies:

- `HealthCampCreated`
- `HealthCampCompleted`
- `HealthCheckCompleted`
- `VaccinationRecorded`

For a Health Programme that is specifically a health camp, the applicable canonical events are `HealthCampCreated` and `HealthCampCompleted`.

For other Health Programme categories, the exact canonical event depends on the programme type. This scenario does not invent a generic `HealthProgramCompleted` event.

C and E provide authoritative participant/personnel context where applicable. H may consume G06-published facts for configured notification/service purposes.

Events remain distinct from any synchronous business hand-off used to obtain authoritative participant or workforce context.

---

## 15. Business Authorization, Approval and Audit Obligations

Programme planning and healthcare conduct must remain within G06's configured authorization and professional-healthcare authority.

Audit evidence should preserve, where applicable:

- Health Programme identity.
- Programme type/purpose.
- Planned lifecycle position.
- Responsible healthcare staff.
- Participant context.
- Material programme findings/outcomes.
- Completion decision.
- Final lifecycle transition.
- Relevant canonical business event.
- Follow-up healthcare actions established from programme findings.

Medical confidentiality applies to individual participant healthcare information.

The scenario does not invent a separate approval chain or separation-of-duty requirement.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Health Programme is established under G06 ownership.
- Programme purpose and scope are recorded.
- Required participant and workforce context is established.
- Programme reaches `Active`.
- Programme activity is conducted.
- Relevant findings/outcomes are recorded.
- Required individual follow-up is separately established where applicable.
- Programme completion condition is satisfied.
- Health Program reaches `Completed`.
- Relevant canonical G06 business facts are published.
- Audit/history remains traceable.

### Postconditions

- The institutional Health Programme is completed.
- Programme outcomes remain available as governed healthcare history.
- Any individual follow-up remains separately owned by the appropriate G06 healthcare aggregate.
- C/E ownership remains unchanged.
- H may consume the resulting business facts without acquiring programme ownership.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. L2-G06 identifies several programme-specific events but does not define a single generic event for every Health Programme category.
2. The exact completion evidence for programme types other than the explicitly named Health Camp/Vaccination/Annual Checkup workflows is not fully enumerated.

### Assumptions

- Health Programme planning and completion are governed by the Health Program aggregate.
- Programme-specific events are selected only where explicitly defined upstream.
- Individual follow-up remains a separate healthcare outcome.

### Deferred Questions

Confirm the canonical event and completion evidence for any Health Programme category not explicitly represented in the L2-G06 event catalogue before Architecture Approval/Lock.
