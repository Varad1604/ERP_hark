# Observation Capacity Exception to Alternative Care Decision

## 1. Specification Metadata

**Specification ID:** L4-G-170  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-170 — Observation Capacity Exception to Alternative Care Decision  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-170; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall recognise when clinically required observation cannot be accommodated, preserve the capacity exception, and support an authorised alternative-care decision without falsely advancing an Observation Stay admission.

## 3. Required Business Context

- Observation Stay is G06-owned with lifecycle `Admitted → Observed → Released`.
- Capacity unavailability is an exception, not a new Observation Stay state.
- Medical Encounter remains the healthcare context.
- Referral, follow-up and alternative institutional care retain their own ownership.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-170-BH-001 | When required observation cannot be accommodated, the system shall record the authoritative capacity exception in the patient's healthcare decision context. | L3-G-170 §§3, 8, 9 |
| L4-G-170-BH-002 | The system shall prevent an Observation Stay from being represented as successfully admitted solely because observation was clinically required. | L3-G-170 §§9, 13, 16 |
| L4-G-170-BH-003 | When an authorised healthcare professional establishes an alternative-care decision, the system shall preserve the selected healthcare path and its relationship to the Medical Encounter. | L3-G-170 §§9, 15, 16 |
| L4-G-170-BH-004 | When external referral is required, the system shall preserve the hand-off to the authoritative G06 Referral workflow without claiming external hospital ownership. | L3-G-170 §§9, 13 |
| L4-G-170-BH-005 | When authoritative capacity becomes available before the alternative decision is finalised, the system shall permit reassessment through the normal observation-admission behavior rather than duplicating the outcome. | L3-G-170 §10 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-170-BH-006 | The retrieved upstream architecture does not define a canonical Observation Capacity Exception event. No event name shall be invented; G06 remains owner of the healthcare decision. | L3-G-170 §§14,17 |
| L4-G-170-BH-007 | If the alternative path creates a G06 Referral, Referral remains G06-owned and follows `Recommended → Accepted → Completed`; H may consume configured healthcare facts for notification. | L3-G-170 §§13–14 |

## 6. Failure / Rejection / Continuation

- If capacity cannot be established authoritatively, the system shall not invent an unavailable-capacity condition.
- If authorised medical authority is unavailable, the system shall retain the unresolved requirement until the decision can be established.
- If no acceptable institutional alternative exists, the system shall govern the authoritative referral path.
- If capacity changes, the system shall use the authoritative current business condition and avoid duplicate observation outcomes.
- If patient/encounter context is unresolved, the alternative outcome shall not be attached to invented context.

## 7. Audit & Evidence Behavior

The system shall preserve observation requirement, capacity exception, alternative-care decision, patient/Medical Encounter relationship and referral outcome where applicable.

## 8. Observable Completion Conditions

Completion is observable when the capacity exception is recorded, an authorised alternative-care decision is established, and the patient is routed to the applicable healthcare path without falsely advancing Observation Stay admission.

## 9. Traceability & Provenance

**Parent:** L3-G-170 — Observation Capacity Exception to Alternative Care Decision  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No universal observation-capacity threshold/prioritisation rule or dedicated capacity-exception event is defined upstream. Those gaps remain upstream-owned. Technical capacity calculation, routing, APIs, persistence, UI and infrastructure remain deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
