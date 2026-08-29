# Emergency Case to Ambulance Coordination

**Scenario ID:** L3-G-164  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, H  
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
| Final L3 ID | L3-G-164 |
| Canonical Title | Emergency Case to Ambulance Coordination |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional healthcare outcome in which an Emergency Case requires ambulance coordination, the ambulance request and relevant provider/destination information are established, departure is coordinated within institutional scope, and the ambulance coordination outcome is recorded.

The locked L2 explicitly states that Institutional Health Services may record Ambulance Request, Provider, Departure Time, Destination and Arrival Confirmation, while ambulance fleet management remains outside the domain.

## 3. Business Outcome

An ambulance has been appropriately requested and coordinated for the emergency case, the relevant provider/destination and coordination evidence have been recorded, and the institutional healthcare record contains the resulting ambulance coordination outcome.

G06 does not become the owner of the ambulance provider's fleet or transport operations.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An Emergency Case requires ambulance support following emergency assessment.

### 4.2 End Boundary

The ambulance coordination outcome is recorded, including the applicable departure/arrival confirmation available to institutional operations.

### 4.3 In Scope

- Emergency case identification.
- Ambulance need determination.
- Ambulance request.
- Provider identification.
- Destination.
- Departure time.
- Arrival confirmation where available.
- Emergency-case association.
- Coordination with H for configured notifications.
- Audit and medical record continuity.

### 4.4 Out of Scope

- Ambulance fleet management.
- Ambulance vehicle maintenance.
- Ambulance crew management.
- External hospital operations.
- External billing/insurance.
- Technical communication mechanisms.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

Owns Emergency Case and records ambulance coordination as part of institutional emergency operations.

### Section H — Notifications

May deliver configured emergency notifications. H does not own ambulance coordination.

The L2 expressly states that ambulance fleet management remains outside G06 and notification delivery remains within H.

## 6. Business Actors, Roles and Responsibilities

### Medical Professional / Medical Staff

Determines whether ambulance support is required and provides relevant clinical/coordination information.

### Healthcare Operations Staff

Coordinates the ambulance request and records the business outcome.

### Ambulance Provider

Provides the external ambulance service.

### Notification Services — H

Delivers configured emergency notifications.

No fleet-management actor is invented inside G06.

## 7. Preconditions and Entry Criteria

1. An Emergency Case exists.
2. Emergency assessment establishes the need for ambulance coordination.
3. The patient and destination context are sufficiently established.
4. An authorised healthcare actor can request/coordinate the ambulance.
5. Applicable emergency notification policy is available where configured.

## 8. Trigger

Emergency assessment determines that ambulance transportation is required.

## 9. Main Business Flow

### Phase A — Ambulance Need

1. Identify the Emergency Case and patient.
2. Complete the emergency assessment necessary to determine transport need.
3. Determine that ambulance support is required.
4. Establish the intended destination according to the emergency/referral decision.

### Phase B — Ambulance Request

5. Create the ambulance request within institutional healthcare operations.
6. Identify the ambulance provider.
7. Associate the request with the Emergency Case.
8. Record the destination.
9. Preserve the request context.

The L2 explicitly identifies Ambulance Request and Provider as ambulance-coordination information.

### Phase C — Coordination

10. Coordinate ambulance arrival through the applicable institutional process.
11. Record departure time when the ambulance departs.
12. Maintain emergency medical support until the applicable handoff condition is satisfied.
13. Where configured, provide emergency notifications through H.

### Phase D — Arrival/Completion Evidence

14. Record arrival confirmation where available.
15. Record the resulting ambulance coordination outcome.
16. Preserve the relationship to the Emergency Case and any external referral.
17. End the scenario.

The external ambulance provider's operational lifecycle remains outside G06.

## 10. Alternate Business Flows

### 10.1 Ambulance Provider Available

The request is accepted and coordination proceeds through departure and applicable arrival confirmation.

### 10.2 Provider Unavailable

Another permitted ambulance provider may be considered according to the applicable institutional policy.

### 10.3 Destination Changes

Where the authorised medical decision changes the destination, the recorded destination is updated according to business authority and the resulting outcome remains traceable.

### 10.4 Emergency Stabilisation Removes Ambulance Need

If the authorised medical professional determines ambulance transport is no longer required, the ambulance coordination path is closed without representing a transport that did not occur.

### 10.5 Student/Hostel Emergency

C or G03 may provide relevant identity/residency context. G06 retains emergency ownership.

### 10.6 Emergency Notifications

Where configured, H delivers emergency notifications to applicable recipients; H does not alter G06 emergency state.

## 11. Failure, Interruption and Business Recovery

### 11.1 Ambulance Provider Cannot Accept

The request remains unresolved while an alternative permitted provider is considered.

### 11.2 Destination Cannot Be Established

Ambulance coordination does not proceed against an invented destination.

### 11.3 Patient Context Uncertain

The request remains associated with the unresolved emergency until authoritative patient context is established.

### 11.4 Ambulance Departure Not Confirmed

No departure is recorded as a completed fact without business evidence.

### 11.5 Arrival Confirmation Unavailable

The institutional record distinguishes an unconfirmed arrival from a confirmed arrival.

### 11.6 Recovery

Once the relevant business fact is established, coordination continues from the unresolved business boundary.

No technical retry, queue, rollback, API, database or infrastructure behaviour is specified.

## 12. Business Rules and Decision Constraints

The locked L2 establishes that G06 may record:

- Ambulance Request.
- Provider.
- Departure Time.
- Destination.
- Arrival Confirmation.

It explicitly excludes ambulance fleet management from G06.

Emergency categories remain configurable and medical professionals retain operational authority.

No fleet, vehicle, crew or dispatch lifecycle is created in this scenario.

## 13. State and Lifecycle Impact

The Emergency Case lifecycle is:

`Reported → Assessment → Treatment → Referral (Optional) → Closed`.

Ambulance coordination is a supporting emergency outcome; it is not introduced as a new Emergency Case state.

The ambulance provider's own operational lifecycle remains external.

## 14. Business Events and Cross-Domain Collaboration

G06 owns the Emergency Case and therefore remains the canonical publisher of emergency-domain business facts.

The retrieved authoritative content does not expose a dedicated canonical ambulance event name. No new event name is invented.

H consumes configured emergency facts for notification delivery; H does not own or publish the ambulance business outcome.

## 15. Business Authorization, Approval and Audit Obligations

Ambulance need and destination must be determined by an appropriately authorised healthcare actor.

Audit evidence should preserve:

- Emergency Case.
- Ambulance Request.
- Provider.
- Destination.
- Departure Time.
- Arrival Confirmation.
- Responsible healthcare actor.
- Resulting emergency outcome.

The L2 requires immutable audit history for healthcare activities and lifecycle transitions.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Emergency Case identified.
2. Ambulance need determined.
3. Ambulance request recorded.
4. Provider recorded.
5. Destination recorded.
6. Departure time recorded where departure occurs.
7. Arrival confirmation recorded where available.
8. Emergency/ambulance coordination history audited.

### Postconditions

- Ambulance coordination is traceable to the Emergency Case.
- G06 retains the institutional coordination record.
- External ambulance fleet management remains outside G06.
- Notification delivery, where applicable, remains H-owned.
- No unconfirmed transport fact is represented as confirmed.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 does not expose a detailed ambulance-provider selection policy.
2. The exact canonical ambulance event name is not exposed in the retrieved authoritative excerpt.
3. Exact business evidence for arrival confirmation is not enumerated.

### Assumptions

- G06 records ambulance coordination but does not manage the ambulance fleet.
- H remains notification owner.
- Medical professionals determine ambulance need.

### Deferred Questions

Verify the canonical ambulance event and any mandatory provider/arrival evidence against the complete locked event and contract definitions before Architecture Approval/Lock.
