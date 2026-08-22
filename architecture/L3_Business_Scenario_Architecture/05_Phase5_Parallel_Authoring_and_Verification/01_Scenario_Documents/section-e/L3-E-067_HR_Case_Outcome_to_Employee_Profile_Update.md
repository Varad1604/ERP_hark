# HR Case Outcome to Employee Profile Update

**Scenario ID:** L3-E-067  
**Status:** Draft  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Architecture Level:** L3 — Business Scenario Architecture  
**Author:** Varad  

## 1. Scenario Summary

This scenario governs the business outcome in which a completed HR case produces an approved change to information owned by Employee Profile.

HR Workflows owns the HR case, including its intake, routing, approvals, evidence and closure. Employee Profile remains the canonical owner of employee profile information and applies only the profile-side consequence of the approved HR case outcome.

## 2. Scenario Purpose

Ensure that an approved HR case outcome is reflected accurately in the canonical EmployeeProfile without transferring ownership of either the HR case or the employee profile.

## 3. Business Outcome

The approved HR case outcome is applied to the appropriate EmployeeProfile information, the resulting profile information becomes authoritative, and the HR case remains owned by HR Workflows.

## 4. Scenario Boundary and Scope

### Start Boundary

The scenario begins when an HR case has reached its authoritative business outcome and that outcome requires a defined EmployeeProfile change.

### End Boundary

The scenario ends when EmployeeProfile has applied the approved profile-side consequence and published the applicable canonical profile event.

### In Scope

- Consumption of an authoritative HR case outcome.
- Determination of the EmployeeProfile consequence.
- Validation of the profile-side change.
- Application of the profile-side change.
- Preservation of HR case ownership by HR Workflows.
- Publication of the canonical EmployeeProfile event.

### Out of Scope

- HR case creation, routing or approval.
- Changing the HR case lifecycle.
- Creating or modifying workforce lifecycle state owned by another bounded context.
- Technical APIs, queues, retries, database operations or implementation mechanics.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Employee Profile | Owns employee profile information and its lifecycle. |
| HR Workflows | Owns the originating HR case and its business outcome. |

A case outcome does not transfer ownership of employee profile information to HR Workflows.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| HR Administrator | Performs the HR case process and provides the approved outcome. |
| HR Executive | Supports HR case administration. |
| Employee Profile business context | Applies the profile-side consequence. |

No additional scenario-specific authority is introduced.

## 7. Preconditions and Entry Criteria

- Employee exists.
- Canonical EmployeeProfile exists.
- HR case exists and has reached an authoritative outcome.
- The outcome identifies a profile-side change.
- The affected information belongs to Employee Profile.

## 8. Trigger

An authoritative HR case outcome requiring an EmployeeProfile change is available.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | HR Workflows completes the HR case outcome. | HR Workflows |
| 2 | The approved outcome is made available to Employee Profile. | HR Workflows |
| 3 | Employee Profile determines the affected profile information and confirms ownership. | Employee Profile |
| 4 | Employee Profile validates the proposed profile-side change. | Employee Profile |
| 5 | Employee Profile applies the approved change. | Employee Profile |
| 6 | Employee Profile publishes the canonical event corresponding to the completed change. | Employee Profile |

The flow ends when the profile-side business result is authoritative.

## 10. Alternate Business Flows

### A1 — No Profile Consequence

If the HR case outcome has no defined EmployeeProfile consequence, no profile mutation occurs.

### A2 — Outcome Targets Another Owner

If the outcome concerns a concept owned by another domain, Employee Profile does not assume ownership and the originating case follows the applicable canonical domain boundary.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Outcome is incomplete or ambiguous | Do not apply a profile change until the authoritative business outcome is clarified. |
| Proposed profile value violates an EmployeeProfile rule | Reject the profile-side change and return it for governed correction. |
| Outcome concerns another canonical owner | Preserve that owner's authority and route the business consequence appropriately. |

Technical retries and infrastructure recovery are outside L3.

## 12. Business Rules and Decision Constraints

- HR Workflows owns the HR case.
- Employee Profile owns employee profile information.
- Cross-domain collaboration does not transfer ownership.
- EmployeeProfile business rules remain authoritative for profile changes.
- No profile-side change is applied from an unapproved or ambiguous case outcome.

## 13. State and Lifecycle Impact

The HR case remains in its completed outcome state under HR Workflows.

EmployeeProfile remains in its existing lifecycle state; only the applicable owned profile information changes.

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### Business Events

The originating HR event remains published by HR Workflows. The resulting profile event is published by Employee Profile.

The exact frozen event names for this scenario must be confirmed against the applicable Stage 3 event mapping before Architecture Approval; no new event name is invented here.

### Business Hand-off

HR Workflows → Employee Profile: authoritative HR case outcome requiring a profile-side change.

Employee Profile → downstream consumers: canonical profile-change event.

## 15. Business Authorization, Approval and Audit Obligations

The HR case outcome must already have satisfied the authoritative HR workflow approval requirements.

Employee Profile applies only the resulting profile-side change.

The HR case and its approval evidence remain owned by HR Workflows; the resulting profile change remains traceable within Employee Profile.

## 16. Completion Criteria and Postconditions

- Approved HR outcome has been applied where applicable.
- EmployeeProfile contains the resulting authoritative information.
- HR Workflows remains owner of the HR case.
- Employee Profile remains owner of the profile information.
- Applicable profile-change event has been published.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issue

The exact frozen Stage 3 event/contract mapping for E067 must be confirmed before Architecture Approval because the authoritative Stage 3 row was not available in the currently retrievable source set.

### Assumption

No assumption overrides locked architecture.

### Deferred Question

Confirm the exact canonical HR Workflows → Employee Profile business hand-off and event mapping.
