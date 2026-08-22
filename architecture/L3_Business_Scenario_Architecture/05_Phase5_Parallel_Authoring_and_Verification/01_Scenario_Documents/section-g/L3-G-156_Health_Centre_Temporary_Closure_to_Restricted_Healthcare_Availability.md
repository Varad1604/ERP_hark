# Health Centre Temporary Closure to Restricted Healthcare Availability

**Scenario ID:** L3-G-156  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which an operational Health Centre is temporarily closed or otherwise restricted under an authoritative business condition, the new operational status is established, and healthcare-service availability is correspondingly restricted.

The locked G06 model defines the Health Centre Operational Status values as Draft, Active, Temporarily Closed, Under Maintenance, Emergency Only and Retired, with status values configurable. It also defines the Health Centre workflow from creation and configuration through validation, activation and healthcare-service availability. fileciteturn234file0

The scenario deliberately treats **Temporarily Closed** as the canonical Health Centre status and does not invent a separate `Restricted` lifecycle state. Restricted healthcare availability is the business consequence of the temporary-closure condition.

## 3. Business Outcome

The affected Health Centre has been authoritatively placed into **Temporarily Closed**, and healthcare services are restricted for the affected operational period and scope.

The scenario does not terminate the Health Centre, retire it, close patient records, or execute unrelated clinical or facilities workflows.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An Active Health Centre encounters an authorised business condition requiring temporary closure or restricted healthcare availability.

### 4.2 End Boundary

The Health Centre is authoritatively in Temporarily Closed status and its healthcare availability is restricted accordingly.

### 4.3 In Scope

- Identify Health Centre.
- Establish closure condition/reason.
- Validate operational authority.
- Change operational status to Temporarily Closed.
- Restrict ordinary healthcare availability.
- Preserve business/audit history.
- Communicate the restriction where configured.
- Preserve the distinction between temporary closure and retirement.

### 4.4 Out of Scope

- Permanent retirement.
- Patient lifecycle changes.
- Clinical consultation.
- Emergency treatment.
- Medicine management.
- Billing/accounting.
- Biomedical-equipment maintenance execution.
- Technical access control.
- Infrastructure/deployment.
- Notification implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns:

- Health Centre operational status.
- Healthcare availability.
- Health Centre configuration.
- Temporary closure decision.

The L2's canonical ownership matrix identifies Health Centre as owned by G06. fileciteturn240file0

No other domain participates in ownership of this scenario.

## 6. Business Actors, Roles and Responsibilities

### Health Services Operational Authority

Establishes the temporary closure decision under applicable healthcare operational policy.

### Health Services Operational Personnel

Support operational closure and service restriction where required.

No facilities, security or technical role is invented merely because the closure may affect physical availability.

## 7. Preconditions and Entry Criteria

1. A Health Centre exists.
2. Its current operational status is Active or another status from which temporary closure is permitted.
3. An authoritative business condition requiring temporary closure is established.
4. The responsible Health Services authority can establish the new status.
5. The Health Centre's current status is not already Temporarily Closed.

## 8. Trigger

An authorised business condition requires temporary restriction of Health Centre operations.

Examples may include a configuration-driven closure, operational unavailability, maintenance-related restriction, or another healthcare policy condition, but no specific trigger is made mandatory unless upstream policy defines it.

## 9. Main Business Flow

### Phase A — Establish Closure Context

1. Identify the affected Health Centre.
2. Confirm its current operational status.
3. Establish the business condition requiring temporary closure.
4. Establish the affected operational scope and period where defined.
5. Confirm that temporary closure is the appropriate status rather than permanent retirement or another configured status.

### Phase B — Establish Temporary Closure

6. Establish the Health Centre status as **Temporarily Closed**.
7. Restrict ordinary healthcare-service availability within the affected scope.
8. Preserve the closure reason/business context.
9. Preserve the previous and new business state in audit history.

### Phase C — Protect Downstream Clinical Boundaries

10. Do not create new patient or medical encounter outcomes merely because the Health Centre is temporarily closed.
11. Existing patient/medical records remain governed by their own authoritative lifecycles.
12. Where configured healthcare notifications are required, provide the business restriction outcome to the notification process.

### Phase D — Complete

13. End the scenario with the Health Centre in Temporarily Closed status.

The scenario does not define reopening or reactivation; that is a separate business outcome.

## 10. Alternate Business Flows

### 10.1 Planned Temporary Closure

A configured/authorised planned closure establishes Temporarily Closed for the applicable operating period.

### 10.2 Maintenance-Related Restriction

Where a Health Centre becomes unavailable due to maintenance, the scenario establishes the G06 operational restriction. G01 remains owner of facilities/biomedical equipment maintenance execution where applicable. The L2 explicitly states that biomedical equipment and facility maintenance are G01-owned and Health Services may recommend maintenance. fileciteturn231file0

### 10.3 Emergency-Only Operation

If the applicable policy establishes `Emergency Only` rather than `Temporarily Closed`, the Health Centre follows that supported status rather than being incorrectly marked closed.

### 10.4 Already Temporarily Closed

A duplicate closure outcome is not created.

### 10.5 Temporary Closure Should Become Permanent Retirement

Retirement is not silently performed. The applicable retirement decision is a separate business boundary.

### 10.6 Notification Required

Where configured, H handles notification delivery. The closure status remains G06-owned.

## 11. Failure, Interruption and Business Recovery

### 11.1 Closure Condition Cannot Be Established

The Health Centre remains in its current authoritative status.

### 11.2 Authority Cannot Be Established

No substitute authority is invented.

### 11.3 Scope/Period Is Ambiguous

The closure does not become effective against an invented scope or period.

### 11.4 Clinical Consequence Requires Separate Handling

Existing healthcare records remain governed by their own lifecycles. The temporary closure does not create unsupported patient, consultation or emergency states.

### 11.5 Facilities/Maintenance Dependency

Where maintenance is the reason for restriction, G06 can coordinate/recommend maintenance while G01 retains maintenance ownership. fileciteturn231file0

### 11.6 Recovery Boundary

Once the authoritative closure condition is established, Temporarily Closed becomes the effective Health Centre status.

No technical retry, queue, rollback, deployment, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The Health Centre operational statuses include:

- Draft.
- Active.
- Temporarily Closed.
- Under Maintenance.
- Emergency Only.
- Retired.

Status values are configurable. fileciteturn234file0

The scenario must therefore:

1. Use **Temporarily Closed** for the frozen outcome.
2. Not invent `Restricted` as a new Health Centre lifecycle state.
3. Not turn temporary closure into retirement.
4. Not transfer Health Centre ownership to G01 merely because maintenance may be involved.
5. Not modify patient identity or medical-encounter lifecycles.
6. Preserve the configuration/audit principles defined by G06.

G06 states that configuration supports effective dating and complete audit history, and every significant healthcare activity is auditable. fileciteturn240file0

## 13. State and Lifecycle Impact

The Health Centre workflow is:

`Health Centre Created → Configuration → Operational Validation → Activation → Healthcare Services Available`. fileciteturn234file0

The Health Centre status model additionally supports **Temporarily Closed**.

This scenario establishes the status:

`Active → Temporarily Closed`

No new states such as:

- `Closure Pending`.
- `Restricted`.
- `Awaiting Reopening`.
- `Service Suspended`.

are invented.

A later reopening/reactivation is outside this scenario.

## 14. Business Events and Cross-Domain Collaboration

The retrieved G06 source establishes that significant healthcare activities publish immutable domain events, but the specific Health Centre event catalogue excerpt available here does not expose a dedicated event name for temporary closure.

Therefore this scenario does **not** invent:

- `HealthCentreTemporarilyClosed`.
- `HealthcareAvailabilityRestricted`.
- `HealthCentreClosed`.

The canonical publisher remains G06 if a corresponding upstream event exists.

Where maintenance is required, G01 remains the authoritative maintenance owner. Where notifications are configured, H owns delivery.

## 15. Business Authorization, Approval and Audit Obligations

Temporary closure follows the applicable Institutional Health Services operational authority.

The audit record should preserve:

- Health Centre identity.
- Previous operational status.
- New operational status.
- Business reason.
- Effective scope/period where applicable.
- Responsible authority/personnel context.
- Relevant notification outcome.

G06 requires immutable healthcare audit history and identifies configuration changes as audited activity. Audit coverage includes user, timestamp, business action, previous state, new state, business reason and source-system context. fileciteturn240file0

No technical access control/IAM is described.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Health Centre identified.
2. Current status established.
3. Temporary-closure condition established.
4. Applicable scope/period established where required.
5. Status changed to Temporarily Closed.
6. Healthcare availability restricted accordingly.
7. Closure history preserved.
8. Required notification outcome provided through the appropriate communication boundary.

### Postconditions

- Health Centre status is Temporarily Closed.
- Ordinary healthcare services are not treated as available for the affected scope.
- The Health Centre remains a live institutional asset and is not retired.
- Patient/medical encounter lifecycles remain unchanged.
- Maintenance ownership remains G01 where maintenance is involved.
- G06 retains Health Centre ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The retrieved L2 does not expose a dedicated Health Centre temporary-closure event name.
2. It does not provide a detailed closure-reason taxonomy.
3. It does not define a universal rule for when Temporary Closure versus Under Maintenance versus Emergency Only must be selected.

### Assumptions

1. Temporary closure is a G06-owned operational decision.
2. Status values remain configuration driven.
3. Maintenance-related consequences remain G01-owned.
4. Notification delivery remains outside G06.

### Deferred Questions

Confirm the final G06 Health Centre event vocabulary and any authoritative status-selection decision matrix before Architecture Approval/Lock.

No unsupported closure rule or event is invented.
