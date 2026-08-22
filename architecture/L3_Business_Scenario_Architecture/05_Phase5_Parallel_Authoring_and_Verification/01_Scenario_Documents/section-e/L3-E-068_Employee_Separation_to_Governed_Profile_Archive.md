# Employee Separation to Governed Profile Archive

**Scenario ID:** L3-E-068  
**Status:** Draft  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Architecture Level:** L3 — Business Scenario Architecture  
**Author:** Varad  

## 1. Scenario Summary

This scenario governs the profile-side business consequence of a completed employee separation. Workforce & Engagement remains the authority for the employment lifecycle and separation outcome. Employee Profile applies the corresponding governed archival consequence to the employee profile.

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-068 |
| Canonical Title | Employee Separation to Governed Profile Archive |
| Primary Owner | Employee Profile |
| Participating Domains | E — People & Workforce |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Ensure that a completed employee separation results in the appropriate governed EmployeeProfile archival treatment while preserving the employee's historical profile information.

## 3. Business Outcome

The employee profile is placed into its governed archival position following an authoritative separation outcome, while the employment separation itself remains owned by Workforce & Engagement.

## 4. Scenario Boundary and Scope

### Start Boundary

The scenario begins when the canonical workforce lifecycle has reached an authoritative separation outcome.

### End Boundary

The scenario ends when Employee Profile has applied the governed profile archival consequence and the employee's historical profile remains available according to its business lifecycle.

### In Scope

- Receipt of the authoritative separation outcome.
- Confirmation that the employee profile is the affected profile.
- Application of the EmployeeProfile archival consequence.
- Preservation of historical profile information.
- Publication of the canonical profile event.

### Out of Scope

- Resignation approval.
- Employment separation decision.
- WorkforceEngagement lifecycle ownership.
- Payroll settlement.
- Exit clearance case ownership.
- Technical archival/storage mechanisms.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Employee Profile | Owns EmployeeProfile lifecycle and archival treatment. |
| Workforce & Engagement | Owns employment lifecycle and the authoritative separation outcome. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Authorized HR personnel | Complete the originating separation process under the workforce/HR architecture. |
| Employee Profile business context | Applies the profile archival consequence. |

No new actor or authority is introduced.

## 7. Preconditions and Entry Criteria

- Employee exists.
- EmployeeProfile exists.
- Workforce & Engagement has completed the authoritative separation outcome.
- The separation applies to the employee represented by the profile.
- The profile archival consequence is governed by Employee Profile.

## 8. Trigger

A canonical employee separation outcome is completed and made available to Employee Profile.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Workforce & Engagement completes the separation lifecycle outcome. | Workforce & Engagement |
| 2 | The authoritative separation fact is made available to Employee Profile. | Workforce & Engagement |
| 3 | Employee Profile identifies the corresponding EmployeeProfile. | Employee Profile |
| 4 | Employee Profile applies the governed archival treatment. | Employee Profile |
| 5 | Historical profile information remains preserved according to the profile lifecycle. | Employee Profile |
| 6 | Employee Profile publishes the canonical profile lifecycle event. | Employee Profile |

## 10. Alternate Business Flows

### A1 — Separation Outcome Does Not Match a Profile

The profile-side action is not applied until the employee/profile relationship is resolved through the authoritative business context.

### A2 — Subsequent Rejoin

If a later authoritative workforce rejoin occurs, Employee Profile follows the canonical rejoin/profile lifecycle rather than independently reversing the archived state.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Separation outcome is incomplete | Do not apply archival treatment until the authoritative outcome exists. |
| Employee cannot be matched to the profile | Resolve the business identity/profile relationship through the canonical ownership boundary. |
| Rejoin subsequently occurs | Apply the canonical rejoin consequence rather than manually bypassing the lifecycle. |

Technical retries and storage recovery are outside L3.

## 12. Business Rules and Decision Constraints

- Workforce & Engagement owns the employment separation lifecycle.
- Employee Profile owns EmployeeProfile lifecycle.
- Historical profile information must remain preserved according to the authoritative lifecycle.
- A downstream profile consequence must not alter the upstream employment decision.
- Rejoin follows its canonical business lifecycle.

## 13. State and Lifecycle Impact

### Workforce Engagement

Remains in the authoritative separated/closed employment position established by Workforce & Engagement.

### EmployeeProfile

Moves to its governed archival position under Employee Profile.

No new lifecycle state is invented.

## 14. Business Events and Cross-Domain Collaboration

### Business Events

The separation event is published by Workforce & Engagement.

The resulting profile archival event is published by Employee Profile.

Exact event names must be confirmed against the frozen Stage 3 mapping before Architecture Approval; none are invented here.

### Business Hand-off

Workforce & Engagement → Employee Profile: authoritative separation outcome.

Employee Profile → downstream consumers: canonical profile archival/lifecycle fact.

## 15. Business Authorization, Approval and Audit Obligations

The separation must already be authorized under the canonical workforce/HR workflow.

Employee Profile does not approve or reverse employment separation.

The separation decision remains traceable in Workforce & Engagement; the profile archival consequence remains traceable in Employee Profile.

## 16. Completion Criteria and Postconditions

- Authoritative separation outcome exists.
- EmployeeProfile receives the applicable profile consequence.
- Historical profile information remains preserved.
- Workforce ownership of separation is unchanged.
- Employee Profile owns the resulting profile lifecycle state.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issue

The exact frozen Stage 3 lifecycle/event mapping for E068 must be confirmed against the authoritative Stage 3 row before Architecture Approval because that row is not currently retrievable.

### Assumption

No assumption overrides locked architecture.

### Deferred Question

Confirm the exact canonical separation event and EmployeeProfile archival transition.
