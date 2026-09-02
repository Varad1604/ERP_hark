# Observation Admission to Observation Completion

## 1. Specification Metadata

**Specification ID:** L4-G-169  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-169 — Observation Admission to Observation Completion  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-169; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

the system shall govern the complete G06 Observation Stay lifecycle from authorised admission through observation and release. It shall preserve the healthcare outcome and must not absorb external hospital care or other domain ownership.

## 3. Required Business Context

- Observation Stay is a G06 aggregate.
- Observation Stay lifecycle is `Admitted → Observed → Released`.
- Medical Encounter provides the healthcare context.
- E provides professional context; H may support notifications.
- Referral/follow-up outcomes remain separately governed.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-169-BH-001 | When authorised observation care is required, the system shall establish an Observation Stay associated with the patient and Medical Encounter and position it at `Admitted`. | L3-G-169 §§3, 8, 9, 13 |
| L4-G-169-BH-002 | When the observation phase begins, the system shall position the Observation Stay at `Observed` and preserve the relevant observation outcome. | L3-G-169 §§9, 13 |
| L4-G-169-BH-003 | When an authorised release decision is established, the system shall position the Observation Stay at `Released`. | L3-G-169 §§9, 13, 16 |
| L4-G-169-BH-004 | The system shall preserve the relationship between Observation Stay, patient and Medical Encounter throughout the scenario. | L3-G-169 §§4, 9, 15 |
| L4-G-169-BH-005 | When observation establishes a need for referral or follow-up, the system shall preserve the appropriate hand-off without closing the patient's healthcare need incorrectly. | L3-G-169 §§10, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-169-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Institutional Health Services (G06) ownership. | L3-G-169 §14 |

## 6. Failure / Rejection / Continuation

- If admission cannot be associated with a valid Medical Encounter, the system shall not create an unassociated Observation Stay.
- If observation capability is unavailable, the system shall not falsely represent admission and shall support an alternative-care path.
- If authorised release is not established, the system shall retain the Observation Stay in its applicable active lifecycle position.
- If external care is required, the system shall preserve the referral path without representing external treatment as G06-completed.

## 7. Audit & Evidence Behavior

The system shall preserve observation admission, observation outcome, release decision, patient/Medical Encounter association, responsible healthcare professional and relevant lifecycle history.

## 8. Observable Completion Conditions

Completion is observable when the Observation Stay reaches `Released`, the authorised observation outcome is recorded, and any subsequent referral/follow-up path is established where required.

## 9. Traceability & Provenance

**Parent:** L3-G-169 — Observation Admission to Observation Completion  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
