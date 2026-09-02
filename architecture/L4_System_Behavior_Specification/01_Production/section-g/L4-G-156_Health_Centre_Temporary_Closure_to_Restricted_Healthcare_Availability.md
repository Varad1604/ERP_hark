# Health Centre Temporary Closure to Restricted Healthcare Availability

## 1. Specification Metadata

**Specification ID:** L4-G-156  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-156 — Health Centre Temporary Closure to Restricted Healthcare Availability  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-156; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised temporary Health Centre closure by establishing the canonical `Temporarily Closed` operational status and restricting ordinary healthcare availability for the affected scope without retiring the Health Centre or altering patient lifecycles.

## 3. Required Business Context

- A Health Centre exists with an authoritative current operational status.
- A G06-owned business condition requiring temporary closure is established.
- Applicable scope/period and operational authority are available where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-156-BH-001 | Identify the affected Health Centre, current status, closure condition and affected scope/period where defined. | L3-G-156 §§3,7,9 |
| L4-G-156-BH-002 | Distinguish `Temporarily Closed` from other supported statuses including `Under Maintenance`, `Emergency Only` and `Retired`. | L3-G-156 §§10,12–13 |
| L4-G-156-BH-003 | When authorised, transition the Health Centre from its applicable current state to `Temporarily Closed`. | L3-G-156 §§9,13 |
| L4-G-156-BH-004 | Restrict ordinary healthcare-service availability for the affected scope without creating patient, consultation or emergency lifecycle outcomes. | L3-G-156 §§3,9,12 |
| L4-G-156-BH-005 | Preserve closure reason, previous/new status and relevant scope/period as auditable business history. | L3-G-156 §§9,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-156-BH-006 | No dedicated canonical temporary-closure event is exposed by the retrieved upstream architecture; no `HealthCentreTemporarilyClosed`, `HealthcareAvailabilityRestricted` or substitute event shall be invented. G06 remains owner; G01 retains maintenance ownership where applicable and H retains notification delivery. | L3-G-156 §§10,14,17 |

## 6. Failure / Rejection / Continuation

If closure condition, authority, scope or period cannot be established, retain the current authoritative status. Do not silently convert temporary closure into retirement or invent a `Restricted` state.

## 7. Audit & Evidence Behavior

Preserve Health Centre identity, prior status, `Temporarily Closed` status, reason, scope/period, responsible authority and relevant notification outcome.

## 8. Observable Completion Conditions

Completion occurs when the Health Centre is authoritatively `Temporarily Closed`, ordinary healthcare availability is restricted accordingly, history is preserved, and no unsupported lifecycle/event has been created.

## 9. Traceability & Provenance

**Parent:** L3-G-156 — Health Centre Temporary Closure to Restricted Healthcare Availability  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The canonical closure-event name, closure-reason taxonomy and universal status-selection decision matrix are not defined upstream. L4 preserves these gaps. Technical notification, storage, UI and infrastructure mechanics are deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
