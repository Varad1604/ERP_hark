# Employee Profile Suspension and Restoration

**Scenario ID:** L3-E-069  
**Status:** Draft  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Simple  
**Priority:** P1  
**Architecture Level:** L3 — Business Scenario Architecture  
**Author:** Varad  

## 1. Scenario Summary

This scenario governs suspension and restoration of the EmployeeProfile within the profile lifecycle, preserving canonical ownership and historical traceability.

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-069 |
| Canonical Title | Employee Profile Suspension and Restoration |
| Primary Owner | Employee Profile |
| Participating Domains | E — People & Workforce |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Provide a complete business lifecycle for placing an EmployeeProfile into its governed suspended position and subsequently restoring it when the authoritative business condition permits restoration.

## 3. Business Outcome

The EmployeeProfile accurately reflects its governed suspended or restored position, with profile ownership and historical traceability preserved.

## 4. Scenario Boundary and Scope

### Start Boundary

The scenario begins when an authorized business condition requires the EmployeeProfile to enter its governed suspended state.

### End Boundary

The scenario ends when the profile is either suspended under the canonical lifecycle or restored through the corresponding authorized restoration path.

### In Scope

- Initiating a supported profile suspension.
- Validating the suspension condition.
- Applying the canonical profile suspension lifecycle.
- Preserving historical profile information.
- Applying the canonical restoration lifecycle.
- Publishing the corresponding Employee Profile business events.

### Out of Scope

- Workforce employment suspension.
- Identity/account suspension.
- Payroll suspension.
- Technical access control.
- Infrastructure state.
- Any lifecycle owned by another domain.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Employee Profile | Sole canonical owner of EmployeeProfile suspension and restoration. |

No participating domain is given ownership of EmployeeProfile lifecycle.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| HR Administrator | Performs supported EmployeeProfile lifecycle administration. |
| HR Executive | Supports profile lifecycle administration. |
| Verification Officer | Participates where restoration or suspension requires supported verification. |
| Compliance Officer | Participates where a supported compliance condition applies. |

No new actor or authority is invented.

## 7. Preconditions and Entry Criteria

- Employee exists.
- EmployeeProfile exists.
- A supported business condition requiring suspension or restoration exists.
- The requested transition is permitted by the authoritative Employee Profile lifecycle.
- Any required business evidence is available.

## 8. Trigger

An authorized business request is raised to suspend or restore an EmployeeProfile.

## 9. Main Business Flow

### Suspension

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | A supported business condition requiring suspension is identified. | Authorized HR role |
| 2 | Employee Profile validates that the condition permits the suspension transition. | Employee Profile |
| 3 | Employee Profile applies the canonical suspension transition. | Employee Profile |
| 4 | Employee Profile preserves the profile's historical information. | Employee Profile |
| 5 | Employee Profile publishes the canonical suspension event. | Employee Profile |

### Restoration

| Step | Business Behaviour | Owner |
|---|---|---|
| 6 | A supported restoration condition is established. | Authorized HR role |
| 7 | Employee Profile validates the restoration transition. | Employee Profile |
| 8 | Employee Profile applies the canonical restoration transition. | Employee Profile |
| 9 | Employee Profile publishes the canonical restoration event. | Employee Profile |

## 10. Alternate Business Flows

### A1 — Suspension Already Applied

If the profile is already suspended, no duplicate suspension transition is performed.

### A2 — Restoration Not Permitted

If the restoration prerequisites are not satisfied, the profile remains suspended until the canonical restoration condition exists.

### A3 — Another Domain Owns the Condition

If the business condition is actually a Workforce, Identity or other domain lifecycle, Employee Profile does not assume ownership of that lifecycle. Only the defined profile consequence is applied.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Suspension prerequisites are not satisfied | Do not transition the profile. Resolve the business prerequisite. |
| Restoration prerequisites are not satisfied | Preserve the suspended state until restoration is legitimately permitted. |
| Requested transition belongs to another domain | Route the source lifecycle to its canonical owner. |
| A later correction is required | Apply a new governed lifecycle action while retaining history. |

Technical rollback, retries and infrastructure recovery are excluded.

## 12. Business Rules and Decision Constraints

- EmployeeProfile owns its own lifecycle.
- Profile suspension must use the canonical Employee Profile transition.
- Restoration must use the canonical restoration transition.
- A profile lifecycle transition must not be used to change Workforce or Identity state.
- Historical profile information remains traceable.
- Cross-domain participation does not transfer ownership.

## 13. State and Lifecycle Impact

| Concept | Transition | Owner |
|---|---|---|
| EmployeeProfile | Active/eligible → Suspended, where the canonical lifecycle permits it | Employee Profile |
| EmployeeProfile | Suspended → Restored/active position, where the canonical lifecycle permits it | Employee Profile |

No new state is introduced by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### Business Events

Employee Profile publishes the canonical event corresponding to suspension and restoration.

The exact frozen event names must be confirmed against the Stage 3 canonical mapping before Architecture Approval; no event names are invented.

### Business Hand-offs

No material cross-domain business contract is required for the core profile suspension/restoration outcome.

Where another domain supplies the business condition, that domain remains owner of its own lifecycle.

## 15. Business Authorization, Approval and Audit Obligations

Suspension and restoration must be performed by authorized HR business actors under the applicable Employee Profile governance.

Where verification or compliance review is required, the supported Verification Officer or Compliance Officer participates.

The lifecycle transitions and their evidence remain historically traceable.

## 16. Completion Criteria and Postconditions

### Suspension

- Profile is in the canonical suspended position.
- Historical profile information remains preserved.
- Suspension event is published.

### Restoration

- Restoration prerequisites have been satisfied.
- Profile is returned to the canonical restored/active position.
- Restoration event is published.
- Historical suspension remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issue

The exact frozen Stage 3 state names, transitions and event names for E069 must be confirmed against the authoritative Stage 3 row before Architecture Approval because that row is not currently retrievable.

### Assumption

No assumption overrides locked architecture.

### Deferred Question

Confirm the exact canonical EmployeeProfile suspension/restoration state and event vocabulary.
