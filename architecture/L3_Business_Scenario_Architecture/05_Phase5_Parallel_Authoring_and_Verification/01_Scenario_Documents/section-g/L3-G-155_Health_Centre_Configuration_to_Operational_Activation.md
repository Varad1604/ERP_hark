# Health Centre Configuration to Operational Activation

**Scenario ID:** L3-G-155  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L1 Business Domain Architecture; L2-G06 Institutional Health Services & Medical Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-G-155` |
| Canonical Title | Health Centre Configuration to Operational Activation |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, A |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |
| Authoritative L2 | `L2-G06` — Institutional Health Services & Medical Workflows |

Frozen Phase 5 identity and ownership are preserved.

## 2. Scenario Purpose

Govern the institutional outcome in which a Health Centre is configured, its operational readiness is validated, and the Health Centre becomes active and available for healthcare services.

Locked L2-G06 assigns Health Centre Information, Operating Hours, Medical Facility Configuration, and Operational Status to the Health Centre aggregate and defines the business progression `Health Centre Created → Configuration → Operational Validation → Activation → Healthcare Services Available`.

## 3. Business Outcome

A configured institutional Health Centre has passed applicable operational validation and reaches `Active`, making healthcare services available within its authorised scope.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Health Centre is ready for configuration and operational validation.

### 4.2 End Boundary
The Health Centre is `Active` and healthcare services are operationally available.

### 4.3 In Scope
- Health Centre identification and facility type.
- Campus context.
- Operating hours and emergency availability.
- Medical facility configuration.
- Operational validation.
- Activation.
- Preservation of configuration and activation history.

### 4.4 Out of Scope
- Patient registration or clinical consultation.
- Emergency treatment or medicine dispensing.
- Finance accounting.
- Biomedical-equipment maintenance execution.
- Technical authentication, infrastructure, UI, databases, or event-bus implementation.

### 4.5 Related Scenarios — Optional
Patient registration, consultation, emergency care, prescription, dispensing, observation, referral, and health-program outcomes remain separate G06 scenarios.

## 5. Ownership and Participating Domains

Institutional Health Services (G06) owns the Health Centre aggregate and its operational status. Platform Foundation (A) provides configuration, workflow, and audit foundation capabilities but does not own or publish the Health Centre lifecycle.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility |
|---|---|
| Health Services Operational Authority | Governs configuration and activation of the Health Centre. |
| Health Services Operational Personnel | Validate operational readiness according to applicable policy. |
| Platform/Foundation capability owner | Provides enterprise foundation capabilities without taking business ownership. |

No technical platform role or IAM design is introduced at L3.

## 7. Preconditions and Entry Criteria

- A Health Centre exists.
- Facility type and campus/location context are established.
- Required operating hours and medical-facility configuration are available.
- Emergency availability is established where applicable.
- Operational validation can be performed.
- No authoritative condition prevents activation.

## 8. Trigger

The configured Health Centre becomes ready for operational validation and activation consideration.

## 9. Main Business Flow

1. Identify the Health Centre and its institutional/campus context.
2. Establish the applicable facility type.
3. Establish operating hours, emergency availability, contact information, and allowed configuration.
4. Review the complete Health Centre configuration.
5. Validate operational readiness under Institutional Health Services policy.
6. Resolve any business configuration deficiency before activation.
7. Authorise activation through the applicable business authority.
8. Transition the Health Centre to `Active`.
9. Publish `HealthCentreActivated` through G06.
10. Preserve activation and configuration history.
11. Make healthcare services available within the configured scope.

## 10. Alternate Business Flows

- A basic Medical Room, Campus Clinic, Health Centre, First Aid Centre, or Emergency Medical Unit may be configured where supported by policy.
- If configuration requires correction, activation is withheld until corrected and revalidated.
- If activation cannot be authorised, the Health Centre remains in its applicable existing state; no new exception state is invented.

## 11. Failure, Interruption and Business Recovery

If facility type, operating hours, emergency availability, or another required configuration fact cannot be authoritatively established, activation does not proceed. Once the business deficiency is resolved, operational validation may resume. No technical retry, queue, rollback, deployment, or persistence behaviour is specified.

## 12. Business Rules and Decision Constraints

- Health Centre availability and configuration are configuration driven under G06.
- Health Centre information may include code, name, campus, facility type, operating hours, emergency availability, contact information, and operational status.
- Supported operational statuses include `Draft`, `Active`, `Temporarily Closed`, `Under Maintenance`, `Emergency Only`, and `Retired`, with institution-specific configuration governed upstream.
- G01 retains biomedical-equipment maintenance ownership.
- Finance retains accounting and billing ownership.
- Platform Foundation retains foundation capability ownership only.

## 13. State and Lifecycle Impact

Locked L2-G06 defines the Health Centre workflow:

`Health Centre Created → Configuration → Operational Validation → Activation → Healthcare Services Available`

For this scenario the resulting operational status is `Active`. No `Pending Activation`, `Ready`, or `Validation Failed` state is invented.

## 14. Business Events and Cross-Domain Collaboration

Locked L2-G06 defines the Health Centre events:

| Business Event | Canonical Publisher | Scenario Purpose |
|---|---|---|
| `HealthCentreCreated` | G06 Institutional Health Services | Records establishment of the Health Centre. |
| `HealthCentreActivated` | G06 Institutional Health Services | Records operational activation of the Health Centre. |

This scenario concludes with `HealthCentreActivated`. Platform Foundation participates only as an upstream foundation provider and does not republish the G06 fact.

## 15. Business Authorization, Approval and Audit Obligations

Health Centre activation follows the applicable Institutional Health Services authority. Configuration changes and significant healthcare activity must remain auditable. The audit requirement is expressed at business level; technical audit storage or implementation is outside L3.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Health Centre identity and context established.
- Required operational configuration established.
- Operational validation completed.
- Activation authorised.
- Health Centre reaches `Active`.
- `HealthCentreActivated` published by G06.
- Configuration and activation history preserved.

### 16.2 Postconditions
- Healthcare services are available within the configured scope.
- Downstream patient and clinical workflows may begin under their own scenario boundaries.
- Health Centre ownership remains with G06.
- Platform Foundation remains a supporting dependency only.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No event-name, owner, publisher, or activation-state ambiguity remains after direct reconciliation with locked L2-G06. The detailed institution-specific operational-validation checklist remains configuration governed rather than canonical L3 content.

### 17.2 Assumptions
Activation is a G06-owned business decision and institution-specific facility configuration remains governed by authoritative policy.

### 17.3 Deferred Questions
Technical implementation and institution-specific operational checklist details remain outside this canonical L3 scenario.
