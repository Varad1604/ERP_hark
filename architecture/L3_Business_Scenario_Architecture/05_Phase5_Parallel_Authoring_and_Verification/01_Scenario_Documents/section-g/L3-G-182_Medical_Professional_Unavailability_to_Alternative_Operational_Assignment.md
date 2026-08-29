# Medical Professional Unavailability to Alternative Operational Assignment

**Scenario ID:** L3-G-182  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E  
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
| Final L3 ID | L3-G-182 |
| Canonical Title | Medical Professional Unavailability to Alternative Operational Assignment |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Institutional healthcare depends on qualified medical personnel being available for consultations, emergency care, observation, programmes, and other healthcare activities. A medical professional may become unavailable for a business reason, creating an operational assignment concern.

The institution therefore requires a governed business outcome in which the affected healthcare responsibility is reviewed and, where possible and authorised, an alternative operational assignment is established. The scenario protects continuity of healthcare operations without inventing technical scheduling or staffing mechanics.

---

## 3. Business Outcome

A medical professional's unavailability has been recognised, the affected healthcare responsibility has been assessed, and an authorised alternative operational assignment has been established where available and appropriate.

Where no suitable alternative can be established, the affected healthcare activity is placed into the applicable governed operational restriction, rescheduling, referral, or other business resolution rather than assigning an unsupported or unqualified substitute.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A medical professional assigned to an active or planned G06 healthcare responsibility becomes unavailable.

### 4.2 End Boundary

The affected healthcare responsibility has either:

1. been transferred to an authorised alternative operational assignment; or
2. been placed into an applicable governed alternative business resolution because no suitable assignment is available.

### 4.3 In Scope

- Recognition of medical professional unavailability.
- Identification of affected healthcare responsibilities.
- Review of operational impact.
- Identification of an authorised alternative medical professional.
- Alternative operational assignment.
- Continuity of affected healthcare activities.
- Controlled restriction/rescheduling when no alternative is available.
- Workforce context consumption from E.
- Audit and traceability.

### 4.4 Out of Scope

- Employee/workforce identity ownership.
- Employment lifecycle changes.
- Leave approval.
- Payroll.
- Technical scheduling engines.
- APIs, databases, queues, infrastructure or deployment.
- Clinical qualification policy not established upstream.
- Creation of a new workforce role or employee.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the medical operational responsibility and its healthcare assignment context.

### Section E — People & Workforce

E owns workforce information, including the authoritative personnel context used to identify available medical professionals.

The cross-domain interaction does not transfer workforce ownership to G06.

---

## 6. Business Actors, Roles and Responsibilities

### Medical Operations Coordinator / Authorised Healthcare Administrator

Reviews the operational impact of the unavailable medical professional and coordinates the alternative business assignment.

### Authorised Medical Professional

Performs the affected healthcare responsibility where assigned and authorised.

### Medical Staff

May receive an alternative operational assignment within the healthcare capability where supported.

### Workforce / People Function

Provides authoritative workforce context and availability information where applicable.

No new clinical authority, qualification authority, employment authority, or technical scheduling role is invented.

---

## 7. Preconditions and Entry Criteria

1. A medical professional is associated with a G06 healthcare responsibility.
2. The professional is determined to be unavailable for the affected responsibility.
3. The affected healthcare activity can be identified.
4. Authoritative workforce context is available from E.
5. An alternative operational assignment is permitted by the applicable healthcare policy.
6. Where continuity cannot be safely maintained, an alternative business resolution exists or can be governed.

---

## 8. Trigger

A medical professional assigned to a G06 healthcare responsibility becomes unavailable.

---

## 9. Main Business Flow

### Phase A — Recognise Unavailability

1. Recognise that the assigned medical professional is unavailable.
2. Identify the healthcare responsibilities affected by that unavailability.
3. Determine whether the affected responsibility is currently active, planned, or otherwise operationally relevant.
4. Obtain authoritative workforce/personnel context from E where required.
5. Assess the business impact on patient care and healthcare operations.

### Phase B — Review Alternative Assignment

6. Determine whether another appropriately authorised medical professional is available for the affected responsibility.
7. Consider the healthcare capability and responsibility requirements applicable to the affected activity.
8. Select an appropriate alternative professional where supported by authoritative business policy.
9. Establish the alternative operational assignment under G06 ownership.
10. Preserve the relationship between the affected healthcare responsibility and the replacement assignment.

### Phase C — Continue or Re-establish Healthcare Operations

11. Confirm that the alternative assignment provides the necessary operational continuity.
12. Continue or re-establish the affected healthcare activity under the alternative assignment.
13. Preserve any affected patient or programme context.
14. Record the assignment change and business reason as auditable operational history.

### Phase D — No Alternative Available

15. If no suitable alternative professional is available, do not assign an unsupported substitute.
16. Apply the applicable business resolution for the affected healthcare activity, such as controlled rescheduling, operational restriction, referral, or other authorised alternative care.
17. Preserve the reason for the alternative resolution.

### Phase E — Complete

18. Confirm the affected healthcare responsibility has an authorised operational disposition.
19. Confirm any required patient-facing or programme-level business consequences are handled through the relevant G06 workflow.
20. Preserve audit/history.
21. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Immediate Alternative Assignment

A suitable medical professional is available and the affected healthcare responsibility can continue without material interruption.

### 10.2 Temporary Alternative Assignment

The alternative professional is assigned for the affected operational period only. The underlying workforce record remains E-owned.

### 10.3 Multiple Healthcare Responsibilities Affected

A single unavailability event affects several consultations, programmes, or operational responsibilities. Each affected responsibility is reviewed and given an appropriate business disposition. The scenario does not collapse unrelated healthcare outcomes into one lifecycle.

### 10.4 No Suitable Alternative

No authorised alternative professional is available. The affected activity follows the applicable restriction, rescheduling, referral, or alternative-care business path.

### 10.5 Emergency Responsibility

If the unavailable professional affects an emergency healthcare responsibility, continuity is prioritised under the applicable emergency healthcare workflow. The scenario does not invent a new emergency state.

### 10.6 Health Programme Responsibility

If a Health Program is affected, its lifecycle remains owned by the Health Program aggregate. The staffing change does not create a new Health Program lifecycle state.

### 10.7 Consultation Responsibility

If a Medical Encounter/consultation is affected, the consultation outcome remains owned by the Medical Encounter workflow. The assignment change is operational context rather than a transfer of medical-record ownership.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Unavailability cannot be reliably established | Preserve the existing assignment until the business availability position is confirmed. |
| Workforce context is unavailable | Obtain authoritative workforce context from E before selecting an alternative. |
| Candidate alternative is not authorised for the responsibility | Do not assign the candidate; continue alternative-resolution assessment. |
| No suitable alternative exists | Apply the applicable rescheduling, restriction, referral, or alternative-care outcome. |
| Patient care is immediately affected | Escalate into the applicable emergency or urgent healthcare workflow. |
| Multiple activities conflict for the same alternative professional | Resolve assignments according to the applicable healthcare operational policy without inventing a local priority rule. |
| Original professional becomes available again | Apply the authoritative operational assignment policy; do not automatically overwrite an established alternative assignment without a governed decision. |

No technical retry, queue, rollback, failover, infrastructure or implementation behaviour is described.

---

## 12. Business Rules and Decision Constraints

1. G06 owns healthcare operational assignments.
2. E owns workforce/personnel information.
3. An alternative medical professional must be appropriately authorised for the affected healthcare responsibility under applicable business policy.
4. G06 must not create or alter E-owned workforce records.
5. An unavailable professional must not remain the sole operational assignment for an activity that requires active healthcare coverage.
6. No unsupported substitute may be assigned merely to preserve an operational schedule.
7. If no suitable alternative exists, the affected healthcare activity must receive an explicit business disposition.
8. Patient healthcare records remain owned by their canonical G06 aggregates.
9. Health Program lifecycle remains owned by Health Program where applicable.
10. Significant assignment changes remain auditable.

The scenario does not invent qualification thresholds, staffing ratios, clinical specialty rules, or priority-ranking rules for competing assignments.

---

## 13. State and Lifecycle Impact

This scenario does not create a new lifecycle for the medical professional or workforce record.

The affected G06 healthcare responsibility remains governed by its own aggregate/workflow lifecycle.

An alternative operational assignment is a business assignment outcome; it is not a new lifecycle state of:

- Employee/Workforce;
- Medical Encounter;
- Health Program;
- Emergency Case;
- Medical Certificate;
- or another G06 aggregate.

Where an affected healthcare activity must be rescheduled, restricted, referred, or otherwise resolved, the applicable canonical G06 workflow governs that resulting outcome.

---

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2-G06 event catalogue does not identify a dedicated canonical event named for medical-professional unavailability or alternative operational assignment.

Therefore this scenario **does not invent** events such as:

- `MedicalProfessionalUnavailable`
- `MedicalProfessionalReassigned`
- `AlternativeMedicalAssignmentCreated`

Where the affected healthcare workflow produces a canonical event already defined by G06, that event remains published by the owning G06 aggregate.

E provides authoritative workforce context. This workforce-context consumption does not transfer workforce ownership to G06.

Any synchronous assignment/context hand-off remains distinct from published domain events.

---

## 15. Business Authorization, Approval and Audit Obligations

Alternative operational assignment must remain within the healthcare authority defined by G06 and the applicable institutional policy.

Audit evidence should preserve:

- Original medical professional assignment.
- Unavailability business fact.
- Affected healthcare responsibility.
- Workforce context used for alternative selection.
- Alternative professional assignment, where established.
- Business reason for the reassignment.
- Resulting operational disposition.
- Any resulting rescheduling, restriction, referral, or alternative-care action.
- Relevant lifecycle changes in affected G06 aggregates.

The scenario does not introduce a separate approval chain unless one is defined by upstream policy.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Medical professional unavailability is recognised.
- Affected healthcare responsibility is identified.
- Workforce context is authoritative.
- Alternative operational assignment is established where a suitable authorised professional exists; otherwise an explicit alternative business disposition is established.
- Patient/programme continuity implications are handled through applicable G06 workflows.
- Assignment and outcome evidence is auditable.

### Postconditions

- The affected healthcare responsibility has a governed operational disposition.
- Where appropriate, a replacement medical professional is assigned.
- E remains owner of workforce/personnel information.
- G06 remains owner of healthcare operational assignment and healthcare outcomes.
- No unsupported clinical or workforce decision is introduced.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The retrieved L2-G06 event catalogue does not define a dedicated medical-professional-unavailability or alternative-assignment event.
2. The exact upstream rule set for selecting among multiple qualified alternative professionals is not enumerated in the retrieved L2-G06 source.

### Assumptions

- G06 owns operational medical assignments.
- E remains authoritative for workforce/personnel context.
- Alternative assignment is permissible only where existing healthcare policy supports it.
- Where no suitable alternative exists, the affected healthcare workflow determines the business disposition.

### Deferred Questions

Before Architecture Approval/Lock, confirm whether an authoritative healthcare staffing/assignment policy exists elsewhere in L2 or governance architecture for:
- professional qualification matching;
- competing alternative assignments;
- emergency coverage priority;
- temporary versus permanent operational assignment.

No such policy is invented in this scenario.
