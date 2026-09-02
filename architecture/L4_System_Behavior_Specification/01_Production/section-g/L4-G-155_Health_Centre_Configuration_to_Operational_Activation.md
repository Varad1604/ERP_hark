# Health Centre Configuration to Operational Activation

## 1. Specification Metadata

**Specification ID:** L4-G-155  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-155 — Health Centre Configuration to Operational Activation  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-155; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall validate an existing Health Centre's configured operational context and establish the G06-owned `Active` status only after applicable readiness and activation authority are established. The behavior ends when healthcare services are available within the configured scope.

## 3. Required Business Context

- A Health Centre exists with facility type and campus/location context.
- Operating hours, emergency availability and medical-facility configuration are available.
- Operational validation can be performed.
- Applicable G06 activation authority is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-155-BH-001 | Identify the Health Centre, campus context, facility type and required operational configuration. | L3-G-155 §§3,7,9 |
| L4-G-155-BH-002 | Validate operating hours, emergency availability, contact/configuration information and operational readiness under G06 policy. | L3-G-155 §§9,12 |
| L4-G-155-BH-003 | Withhold activation while an authoritative configuration deficiency or activation-preventing condition remains unresolved. | L3-G-155 §§10–11 |
| L4-G-155-BH-004 | When readiness and authority are established, transition the Health Centre to `Active`. | L3-G-155 §§9,13 |
| L4-G-155-BH-005 | Make healthcare services available only within the authorised configured scope and preserve configuration/activation history. | L3-G-155 §§3,9,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-155-BH-006 | On activation, publish `HealthCentreActivated`; G06 is the canonical publisher. Platform Foundation (A) remains a supporting capability provider and does not publish or own the Health Centre lifecycle. | L3-G-155 §§5,14,16 |

## 6. Failure / Rejection / Continuation

If required facility facts, emergency availability, readiness or activation authority cannot be established, activation shall not proceed. Resume from the unresolved business boundary after authoritative correction; no technical retry/deployment behavior is introduced.

## 7. Audit & Evidence Behavior

Preserve Health Centre identity, configuration, readiness validation, activation authority, transition to `Active`, event publication and configuration/activation history.

## 8. Observable Completion Conditions

Completion requires validated operational configuration, authorised transition to `Active`, `HealthCentreActivated` published by G06, auditable history preserved, and healthcare services available within configured scope.

## 9. Traceability & Provenance

**Parent:** L3-G-155 — Health Centre Configuration to Operational Activation  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific readiness checklists remain configuration governed. APIs, persistence, UI, infrastructure and deployment mechanisms are deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
