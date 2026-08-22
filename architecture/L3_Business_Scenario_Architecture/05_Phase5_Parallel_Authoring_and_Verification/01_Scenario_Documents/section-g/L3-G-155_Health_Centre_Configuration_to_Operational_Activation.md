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
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which an institutional healthcare facility is configured, its operational readiness is validated, and the Health Centre becomes active and available for healthcare services.

The locked L2-G06 states that Institutional Health Services owns Health Centre Operations and that the Health Centre aggregate owns Health Centre Information, Operating Hours, Medical Facility Configuration and Operational Status. It defines the workflow:

`Health Centre Created → Configuration → Operational Validation → Activation → Healthcare Services Available`.

Supported facility types include Medical Room, Campus Clinic, Health Centre, First Aid Centre and Emergency Medical Unit, with additional types configurable. Operational statuses include Draft, Active, Temporarily Closed, Under Maintenance, Emergency Only and Retired, with status values configurable. fileciteturn234file0

## 3. Business Outcome

A configured institutional Health Centre has passed the applicable operational validation and is **Active**, making healthcare services available within the authorised scope.

The scenario establishes activation; it does not perform patient registration, consultations, emergency care, medicine dispensing or other downstream clinical workflows.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An existing Health Centre record is ready for configuration and operational validation.

### 4.2 End Boundary

The Health Centre reaches the authoritative Active condition and healthcare services become available.

### 4.3 In Scope

- Health Centre identification.
- Facility type.
- Campus context.
- Operating hours.
- Emergency availability.
- Contact information.
- Medical facility configuration.
- Operational validation.
- Activation.
- Preservation of configuration/audit history.

### 4.4 Out of Scope

- Patient registration.
- Clinical consultation.
- Emergency treatment.
- Medicine dispensing.
- Billing/accounting.
- Biomedical-equipment maintenance execution.
- Technical authentication/authorization.
- Infrastructure/deployment.
- UI.
- Event bus implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns:

- Health Centre.
- Health Centre information.
- Operating hours.
- Medical facility configuration.
- Operational status.
- Activation outcome.

### Platform Foundation — A

A provides configuration/workflow/audit foundation capabilities. A does not own the Health Centre business lifecycle.

The L2 identifies Platform Foundation as an upstream dependency providing Configuration Framework and Workflow Engine among other foundation capabilities. fileciteturn231file0

Participation does not transfer Health Centre ownership from G06.

## 6. Business Actors, Roles and Responsibilities

### Health Services Operational Authority

Owns the business decision to configure and activate the institutional Health Centre.

### Health Services Operational Personnel

Support validation of operational readiness according to Health Services policy.

### Platform/Foundation Authority

Provides the enterprise configuration/workflow foundation under A's ownership boundary.

No technical platform role is invented at L3.

## 7. Preconditions and Entry Criteria

1. A Health Centre aggregate exists.
2. Facility type is established.
3. Campus/location context is established.
4. Required operating hours/configuration are available.
5. Applicable emergency availability setting is established.
6. Required operational validation can be performed.
7. No authoritative condition prevents activation.

## 8. Trigger

The configured Health Centre reaches the business point at which operational validation may begin.

## 9. Main Business Flow

### Phase A — Establish Health Centre Configuration

1. Identify the Health Centre.
2. Establish the Health Centre Code and Name where applicable.
3. Establish campus.
4. Establish facility type.
5. Establish operating hours.
6. Establish emergency availability.
7. Establish contact information.
8. Establish other institution-specific configuration allowed by the L2.

### Phase B — Operational Validation

9. Review the configured Health Centre information.
10. Validate that the intended facility type is supported.
11. Validate operating hours.
12. Validate emergency-availability configuration.
13. Validate the Health Centre's operational readiness within the scope of institutional policy.
14. Resolve any business-level configuration discrepancy.

### Phase C — Activation

15. Establish the Health Centre as **Active**.
16. Make healthcare services available within the authorised operational scope.
17. Preserve the activation and configuration history.

### Phase D — Complete

18. End the scenario.

The subsequent Patient Registration or clinical workflow begins only after the Health Centre is operationally available.

## 10. Alternate Business Flows

### 10.1 Basic Medical Room

The facility may be configured as a Medical Room under the supported facility types. Additional capability is not invented.

### 10.2 Campus Clinic / Health Centre

The configured facility type governs the applicable operational context.

### 10.3 Emergency Medical Unit

Emergency availability is established according to configured policy. No additional emergency workflow is absorbed into activation.

### 10.4 Configuration Requires Correction

Activation does not proceed until the business configuration is corrected and revalidated.

### 10.5 Health Centre Cannot Be Activated

The Health Centre remains in its existing authoritative state; no invented activation exception state is created.

### 10.6 Foundation Configuration Dependency

A participates through its configuration foundation. G06 retains ownership of the Health Centre business outcome.

## 11. Failure, Interruption and Business Recovery

### 11.1 Facility Type Cannot Be Established

No activation occurs using an invented facility type.

### 11.2 Operating Hours Cannot Be Established

The Health Centre does not become Active using an assumed schedule.

### 11.3 Emergency Availability Is Ambiguous

Activation remains unresolved until the authorised configuration is established.

### 11.4 Operational Validation Fails

The Health Centre remains outside the Active condition until the business issue is resolved.

### 11.5 Recovery

Once configuration and operational validation conditions are satisfied, the Health Centre proceeds to Activation.

No technical retry, queue, rollback, deployment, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The L2 explicitly makes Health Centre availability and configuration configuration-driven. fileciteturn234file0

Supported Health Centre types include Medical Room, Campus Clinic, Health Centre, First Aid Centre and Emergency Medical Unit, with additional types configurable.

Health Centre information may include:

- Health Centre Code.
- Health Centre Name.
- Campus.
- Facility Type.
- Operating Hours.
- Emergency Availability.
- Contact Information.
- Operational Status.

Operational status values include Draft, Active, Temporarily Closed, Under Maintenance, Emergency Only and Retired, with values configurable. fileciteturn234file0

## 13. State and Lifecycle Impact

The authoritative Health Centre business workflow is:

`Health Centre Created → Configuration → Operational Validation → Activation → Healthcare Services Available`. fileciteturn234file0

Operational Status supports `Draft → Active` as applicable, while additional configured statuses exist for temporary closure, maintenance, emergency-only operation and retirement.

This scenario establishes **Active** and does not invent `Pending Activation`, `Ready`, or `Validation Failed` lifecycle states.

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2-G06 material establishes that every significant healthcare activity shall publish immutable domain events and that Healthcare Services publishes operational information to downstream consumers. fileciteturn233file0turn240file0

However, the retrieved Health Centre section does **not** expose a dedicated canonical Health Centre activation event name.

Therefore this L3 does not invent `HealthCentreActivated` or `HealthServicesAvailable`.

The canonical publisher remains G06 whenever a corresponding upstream event is defined.

A participates as the platform foundation dependency; it does not become the publisher of the G06 business lifecycle.

## 15. Business Authorization, Approval and Audit Obligations

Health Centre activation follows the applicable Institutional Health Services operational authority.

G06 requires every significant healthcare activity to be auditable and states that configuration changes are among audited activities. Audit coverage includes business action, previous state, new state, business reason and source-system context. fileciteturn240file0

Configuration is also required to support version control, effective dating, institution isolation, rollback and complete audit history. Those are architectural configuration principles; this L3 does not describe their technical implementation. fileciteturn240file0

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Health Centre identified.
2. Facility type and campus established.
3. Operating hours established.
4. Emergency availability established.
5. Required configuration established.
6. Operational validation completed.
7. Health Centre reaches Active.
8. Configuration/activation history preserved.

### Postconditions

- Health Centre is Active.
- Healthcare services are available within the configured scope.
- Patient registration and clinical workflows may begin according to their own boundaries.
- Health Centre ownership remains with G06.
- A's configuration foundation remains an upstream dependency only.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The retrieved L2 does not expose a dedicated Health Centre activation event name.
2. The detailed operational-validation checklist is not enumerated.
3. The L2 does not define a separate activation-approval state.

### Assumptions

- Activation is a G06-owned business decision.
- Facility types and operational configuration remain configurable.
- A provides foundation configuration capability without acquiring business ownership.

### Deferred Questions

Confirm the final Health Centre event vocabulary and any required institutional activation checklist before Architecture Approval/Lock.

No unsupported event or approval rule is invented.
