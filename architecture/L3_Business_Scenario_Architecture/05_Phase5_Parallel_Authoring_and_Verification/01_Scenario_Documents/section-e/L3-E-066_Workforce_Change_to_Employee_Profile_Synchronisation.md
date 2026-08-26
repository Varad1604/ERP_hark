# Workforce Change to Employee Profile Synchronisation

**Scenario ID:** L3-E-066  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E07 — Employee Profile Architecture; L2-E01 — Workforce & Engagement Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-066 |
| Canonical Title | Workforce Change to Employee Profile Synchronisation |
| Primary Owner | Employee Profile |
| Participating Domains | E — People & Workforce |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Keep the canonical EmployeeProfile aligned with material workforce lifecycle changes that affect employee-profile information, without allowing Employee Profile to become the owner of Workforce & Engagement state.

The scenario coordinates the business fact that a workforce change has occurred with the corresponding employee-profile information that must remain current for employee-profile consumers.

## 3. Business Outcome

A material workforce change that affects employee-profile information is reflected in the canonical EmployeeProfile, while Workforce & Engagement remains the authoritative owner of the underlying workforce lifecycle decision.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when Workforce & Engagement has completed an authoritative workforce change that has a defined business impact on employee-profile information.

### 4.2 End Boundary

The scenario ends when Employee Profile has applied the relevant profile-side update and the EmployeeProfile reflects the current workforce-related information within its own ownership boundary.

### 4.3 In Scope

- Receiving authoritative workforce change facts.
- Determining whether the change affects EmployeeProfile information.
- Applying the corresponding EmployeeProfile-side update where the profile architecture defines such information.
- Publishing the resulting Employee Profile event.
- Preserving Workforce & Engagement ownership of workforce lifecycle state.

### 4.4 Out of Scope

- Deciding or changing the Workforce Engagement lifecycle.
- Position Assignment lifecycle.
- Reporting Relationship lifecycle.
- Responsibility Assignment.
- Payroll processing.
- Leave processing.
- Direct modification of Workforce aggregates by Employee Profile.
- Technical synchronization mechanisms, queues, retries or event-delivery implementation.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to define this boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Employee Profile — Section E | Owns the employee profile and applies profile-side changes within EmployeeProfile. |
| Workforce & Engagement — Section E | Owns the originating workforce lifecycle change and publishes the authoritative workforce business fact. |

Both capabilities are within Section E, but they remain distinct bounded contexts and canonical owners.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility |
|---|---|
| Authorized HR personnel | Initiates or performs the originating workforce business change within the applicable workforce workflow. |
| Employee Profile business context | Determines and applies the profile-side consequence within EmployeeProfile. |
| Employee | Receives or provides information where the profile-side business process requires employee participation. |

The scenario does not invent a separate synchronization operator or technical event-consumer role.

## 7. Preconditions and Entry Criteria

- The employee exists.
- A canonical EmployeeProfile exists.
- Workforce & Engagement has completed an authoritative workforce change.
- The workforce change is represented by a canonical workforce business event or approved business hand-off.
- The profile-side information affected by the workforce change belongs to EmployeeProfile.
- Employee Profile does not attempt to modify the workforce-owned source state.

## 8. Trigger

A canonical workforce lifecycle event representing a completed workforce change is made available to Employee Profile.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Workforce & Engagement completes a material workforce change under its canonical workflow. | Workforce & Engagement | An authoritative workforce business fact exists. |
| 2 | The corresponding canonical workforce event is made available to Employee Profile. | Workforce & Engagement | Employee Profile receives the authoritative source fact without acquiring workforce ownership. |
| 3 | Employee Profile determines whether the workforce change has a defined profile-side business consequence. | Employee Profile | Only relevant profile information is considered for synchronization. |
| 4 | Where a defined profile-side consequence exists, Employee Profile applies the corresponding change to EmployeeProfile. | Employee Profile | The employee profile reflects the applicable workforce change. |
| 5 | Employee Profile publishes the canonical profile event associated with the completed profile change. | Employee Profile | Dependent consumers receive the updated profile fact. |

The flow ends when the EmployeeProfile-side consequence is authoritative.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | A workforce event has no defined EmployeeProfile consequence. | Employee Profile records no profile mutation. | Workforce change remains authoritative solely within Workforce & Engagement. | Employee Profile |
| A2 | Multiple workforce changes affect profile information in sequence. | Employee Profile applies each relevant authoritative business fact according to its governed profile rules. | EmployeeProfile remains aligned with the applicable completed workforce facts. | Employee Profile |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Workforce change does not satisfy its own business prerequisites. | No authoritative workforce change exists for Employee Profile to synchronize. | Workforce & Engagement resolves the workforce business condition before a completed workforce event is published. | Workforce & Engagement |
| Workforce event contains information that cannot validly be applied to EmployeeProfile. | Profile synchronization cannot complete for that change. | Preserve the authoritative workforce fact and route the profile-side issue for governed correction rather than modifying the workforce source. | Employee Profile |
| The workforce change affects a concept not owned by Employee Profile. | Employee Profile must not assume ownership. | Leave the source concept under Workforce & Engagement or route to its canonical owner. | Workforce & Engagement / relevant owner |

Technical event-delivery failure, queues, retries, dead-letter handling and infrastructure recovery are excluded from L3.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Workforce & Engagement is the authoritative source of workforce lifecycle information. | Workforce & Engagement | Employee Profile consumes completed workforce facts and does not decide workforce state. |
| All workforce changes originate from Workforce business workflows. | Workforce & Engagement | Employee Profile cannot independently create a workforce change. |
| Downstream domains consume workforce information through canonical read contracts and published domain events. | Workforce & Engagement | Synchronization is based on governed cross-context collaboration. |
| EmployeeProfile is the authoritative employee profile. | Employee Profile | Profile-side information remains owned by E07. |
| Cross-domain participation does not transfer ownership. | Enterprise architecture | Employee Profile cannot become owner of Workforce Engagement or its lifecycle. |

## 13. State and Lifecycle Impact

| Concept | Entry Position | Scenario Impact | Result | Canonical Owner |
|---|---|---|---|---|
| Workforce Engagement | Completed workforce change | Source lifecycle remains under Workforce & Engagement. | Unchanged by Employee Profile | Workforce & Engagement |
| EmployeeProfile | Existing valid lifecycle state | Applicable profile-side information is synchronized. | Same EmployeeProfile lifecycle state with current profile information | Employee Profile |

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Scenario Purpose |
|---|---|---|
| `WorkforceEngagementModified` | Workforce & Engagement | Provides an authoritative completed workforce modification fact. |
| `WorkforceTransferred` | Workforce & Engagement | Provides an authoritative completed workforce transfer fact. |
| `WorkforcePromoted` | Workforce & Engagement | Provides an authoritative completed workforce promotion fact. |
| `WorkforceSuspended` | Workforce & Engagement | Provides an authoritative workforce suspension fact where profile-side information is affected. |
| `WorkforceResumed` | Workforce & Engagement | Provides an authoritative workforce resumption fact where profile-side information is affected. |
| `WorkforceEngagementClosed` | Workforce & Engagement | Provides an authoritative workforce-closure fact; any profile archival consequence is handled by its own canonical scenario. |
| `WorkforceRejoined` | Workforce & Engagement | Provides an authoritative workforce-rejoin fact where profile-side information is affected. |
| `PersonalInformationUpdated` | Employee Profile | Establishes the resulting profile personal-information change where applicable. |
| `ContactInformationUpdated` | Employee Profile | Establishes the resulting profile communication-information change where applicable. |

The workforce events above are taken from the Workforce & Engagement downstream-consumer/event catalogue; no new workforce event is introduced.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Workforce & Engagement | Employee Profile | Canonical workforce lifecycle event / approved workforce business fact | Provides the source fact for any defined EmployeeProfile consequence. |
| Employee Profile | Workforce-dependent consumers | Canonical profile-change event | Makes the updated profile fact available without transferring workforce ownership. |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

| Action | Business Actor / Role | Authority |
|---|---|---|
| Perform workforce change | Authorized HR personnel / Workforce & Engagement workflow | Workforce & Engagement owns the lifecycle decision. |
| Apply profile-side consequence | Employee Profile | Employee Profile owns the resulting EmployeeProfile change. |

### 15.2 Separation of Duties

The scenario preserves a clear separation between the workforce decision and the employee-profile consequence. No additional synchronization-specific separation-of-duty rule is defined.

### 15.3 Business Audit and Evidence

The authoritative workforce lifecycle change remains traceable in Workforce & Engagement. The corresponding profile-side change remains traceable in EmployeeProfile.

Section A retains ownership of enterprise audit infrastructure.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A completed authoritative workforce change exists.
- The workforce event retains Workforce & Engagement as its canonical publisher.
- Employee Profile determines whether a profile-side consequence exists.
- Any applicable EmployeeProfile update is applied under E07 ownership.
- The resulting profile change is represented by its canonical Employee Profile event where defined.
- Workforce lifecycle ownership remains unchanged.

### 16.2 Postconditions

- EmployeeProfile reflects the applicable completed workforce change.
- Workforce & Engagement remains the sole authority for the workforce lifecycle decision.
- Consumers may use the updated profile fact without becoming owners.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen scenario title establishes a dedicated Workforce Change → Employee Profile Synchronisation outcome, while E01 explicitly identifies workforce events and downstream consumers, but the reviewed E07 material does not explicitly enumerate the exact set of workforce events consumed by Employee Profile or a dedicated synchronization contract. | The ownership direction and canonical workforce publishers are clear, but the exact E07 synchronization subscription/contract boundary is not explicitly named upstream. | Confirm the canonical E01→E07 event/contract mapping before Architecture Approval if the scenario is required to prescribe the exact synchronization trigger set. |

### 17.2 Assumptions

No assumptions that override locked architecture.

### 17.3 Deferred Questions

Confirm the authoritative E01→E07 synchronization mapping and the exact set of workforce events that have defined EmployeeProfile consequences.
