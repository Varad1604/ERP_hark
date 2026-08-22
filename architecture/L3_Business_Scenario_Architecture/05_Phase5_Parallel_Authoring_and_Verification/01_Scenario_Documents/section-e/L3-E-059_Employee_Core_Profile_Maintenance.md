# Employee Core Profile Maintenance

**Scenario ID:** L3-E-059  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile  
**Participating Domains:** A — Platform Foundation; E — People & Workforce  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E07 — Employee Profile Architecture; L2-A02 — Person & Identity Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-059 |
| Canonical Title | Employee Core Profile Maintenance |
| Primary Owner | Employee Profile |
| Participating Domains | A — Platform Foundation; E — People & Workforce |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Maintain the core employee profile after employee creation and activation so that employee personal, communication, address, and emergency-contact information remains governed by the canonical Employee Profile context.

The scenario provides a complete business path for updating employee profile information without transferring ownership of canonical Person identity or workforce employment state.

## 3. Business Outcome

The employee's requested and valid core profile information is incorporated into the canonical EmployeeProfile aggregate, with the resulting profile state and historical business change remaining governed by Employee Profile.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an employee or authorized business participant has a valid business need to establish, correct, or maintain core employee profile information for an existing Employee Profile.

### 4.2 End Boundary

The scenario ends when the applicable core profile change has been accepted by Employee Profile and the resulting profile information is authoritative within that context.

### 4.3 In Scope

- Updating legal and demographic employee information supported by Employee Profile.
- Maintaining employee communication channels.
- Maintaining employee addresses.
- Maintaining emergency contacts.
- Applying the business validations and invariants governing these profile elements.
- Preserving the EmployeeProfile as the single canonical owner of these profile records.
- Publishing the authoritative Employee Profile change events.

### 4.4 Out of Scope

- Creating the Employee Profile.
- Workforce engagement creation, modification, transfer, suspension, or closure.
- Position Assignment.
- Reporting Relationship.
- Responsibility Assignment.
- Payroll and compensation.
- Leave management.
- Authentication credentials or sessions.
- Authorization role assignment.
- Qualification, certification, skill, language, and profile-document scenarios except where a core-profile change is necessary to maintain the surrounding employee profile context.
- Direct modification of the canonical Person aggregate.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to clarify this boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Section E — Employee Profile | Solely owns EmployeeProfile, including personal information, contacts, addresses, and emergency contacts, and owns the resulting profile lifecycle facts. |
| Section A — Person & Identity | Owns canonical Person and Identity. Employee Profile may reference Person identity but does not transfer or duplicate Person ownership. |

Employee Profile maintenance does not modify workforce employment state or create a second identity model.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Employee | Provides or requests valid profile information for the employee's own profile where the business process permits employee participation. | Initiates or supplies profile information. |
| HR Administrator | Maintains employee profile information within the Employee Profile context. | Performs profile maintenance. |
| HR Executive | Performs employee profile maintenance and administrative corrections. | Performs profile maintenance. |
| Department Administrator | May participate in profile maintenance where the business context authorizes departmental administration. | Performs supported administrative action. |
| Compliance Officer | May participate where a profile change has a compliance implication supported by the Employee Profile context. | Reviews supported compliance-related information. |

The authoritative Employee Profile architecture does not define a separate scenario-specific approval role for ordinary core profile maintenance.

## 7. Preconditions and Entry Criteria

- The employee exists.
- A canonical EmployeeProfile exists.
- The EmployeeProfile is in a lifecycle state that permits the requested maintenance.
- The requested information belongs to the EmployeeProfile aggregate.
- The supplied information satisfies applicable Employee Profile validation rules.
- Any referenced master/reference information is valid.
- The change does not attempt to create a duplicate Employee Profile.
- The requested change does not attempt to alter ownership of Person, Identity, Workforce Engagement, Position Assignment, or another canonical aggregate.

## 8. Trigger

A valid employee-profile maintenance request is raised for core employee personal, contact, address, or emergency-contact information.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | A valid employee profile maintenance need is identified and the employee profile is selected as the business context for the change. | Employee / HR Administrator / HR Executive | The intended EmployeeProfile is identified. |
| 2 | The requested information is classified as core profile information owned by Employee Profile. | Employee Profile | The change is kept within the canonical EmployeeProfile boundary. |
| 3 | The requested personal or demographic information is validated against Employee Profile business rules. | Employee Profile | Valid personal information is eligible for update. |
| 4 | Where communication information is being maintained, the profile ensures that the resulting communication information satisfies its business invariants. | Employee Profile | Communication information remains valid. |
| 5 | Where addresses are being added, changed, or removed, the EmployeeProfile applies the address rules and preserves the resulting profile context. | Employee Profile | Address information is maintained within the canonical profile. |
| 6 | Where emergency contacts are being maintained, the profile preserves the emergency-contact rules, including the primary-contact constraint. | Employee Profile | Emergency-contact information remains valid. |
| 7 | Employee Profile applies the accepted business change to the EmployeeProfile aggregate. | Employee Profile | The canonical employee profile is updated. |
| 8 | Employee Profile publishes the authoritative business event(s) corresponding to the completed profile change. | Employee Profile | Dependent consumers can react to the completed profile fact without acquiring ownership. |

The Main Business Flow stops after the requested core profile maintenance has been accepted and the resulting profile information is authoritative.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | The request changes communication information but would leave the profile without an active communication channel. | The requested change is not accepted until the communication information satisfies the profile rule. | Existing valid communication information remains authoritative. | Employee Profile |
| A2 | The request concerns a profile element owned by another bounded context. | The request is redirected to the owning business context rather than changing EmployeeProfile ownership. | No unauthorized cross-domain profile mutation occurs. | Employee Profile |
| A3 | The requested information is invalid or reference data is not valid. | The change is not accepted until the business information satisfies the applicable validation. | The previous valid EmployeeProfile information remains authoritative. | Employee Profile |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid personal or demographic information is submitted. | The requested profile change cannot be accepted. | Correct the business information and resubmit the profile maintenance request. | Employee Profile |
| A communication change would violate the requirement for at least one active communication channel. | The resulting profile would be business-invalid. | Preserve the valid communication state and correct the requested change. | Employee Profile |
| An attempted change targets canonical Person identity rather than EmployeeProfile information. | Employee Profile cannot become the owner of Person identity. | Route the identity change through Person & Identity. | Person & Identity |
| An attempted change targets employment state, position, responsibility, or reporting relationship. | Employee Profile cannot alter the authoritative workforce/assignment state. | Route the business change to the canonical owning workflow. | Workforce & Engagement / Position & Assignment |

No technical retries, rollback, queues, or infrastructure recovery are defined.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| EmployeeProfile is the single authoritative employee profile. | Employee Profile | All core profile maintenance remains within EmployeeProfile. | L2-E07 |
| At least one active communication channel must exist. | Employee Profile | Communication changes must preserve this invariant. | L2-E07 — Contact Information |
| Official communication channels require verification where applicable. | Employee Profile | Core communication maintenance must respect applicable verification requirements. | L2-E07 |
| Only one primary emergency contact may exist. | Employee Profile | Emergency-contact changes must preserve the primary-contact constraint. | L2-E07 — Emergency Contact |
| Multiple secondary emergency contacts are permitted. | Employee Profile | Maintenance may preserve additional contacts without creating duplicate primary status. | L2-E07 |
| EmployeeProfile child information is owned by the EmployeeProfile aggregate and is not independently owned. | Employee Profile | Profile maintenance is performed against the canonical employee profile. | L2-E07 |
| The canonical Person aggregate remains owned by L2-A02. | Person & Identity | Employee Profile does not become an identity owner. | L2-A02 |
| Workforce employment state is owned by L2-E01. | Workforce & Engagement | Core profile maintenance does not change employment lifecycle state. | L2-E01 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| EmployeeProfile | Active | Core profile information is maintained without changing the employment lifecycle. | Active EmployeeProfile with updated authoritative profile information | Employee Profile | L2-E07 |
| EmployeeProfile | Verified | Supported profile information may be maintained while preserving the profile's governed lifecycle. | Verified EmployeeProfile with updated profile information | Employee Profile | L2-E07 |
| Person | Existing canonical state | Person identity is referenced but not re-owned or re-lifecycled by this scenario. | Unchanged | Person & Identity | L2-A02 |
| Workforce Engagement | Active workforce relationship | Employee profile maintenance does not modify workforce employment state. | Unchanged | Workforce & Engagement | L2-E01 |

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PersonalInformationUpdated | Employee Profile | Employee profile consumers | Establishes that employee personal/demographic information was updated. | L2-E07 |
| ContactInformationUpdated | Employee Profile | Employee directory/profile consumers | Establishes that employee communication information was updated. | L2-E07 |

No new event is introduced for address or emergency-contact maintenance because the authoritative E07 event catalogue was not established here with a scenario-specific event name for those changes.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Person & Identity | Employee Profile | Canonical Person / Identity reference | Maintains the identity boundary while Employee Profile manages employee-specific profile information. | L2-A02; L2-E07 |
| Employee Profile | Enterprise consumers | Approved employee profile information through governed profile/read collaboration | Allows dependent domains to consume current employee profile information without owning it. | L2-E07 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Maintain core employee profile | Employee / HR Administrator / HR Executive | Must act within the Employee Profile business context and applicable business validation. | Employee Profile |
| Review compliance-sensitive profile information where required | Compliance Officer | Participates only where supported by the business context. | Employee Profile |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override rule is explicitly defined for ordinary core profile maintenance in the authoritative sources reviewed.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Employee profile change | Historical EmployeeProfile business change history | Employee Profile |
| Identity boundary preservation | Canonical Person/Identity reference remains under Platform Foundation | Person & Identity |

Platform Audit infrastructure remains owned by Section A and is not redefined here.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The requested core profile information has been accepted by Employee Profile.
- The resulting EmployeeProfile remains the single canonical employee profile.
- Applicable profile invariants remain satisfied.
- The employee's canonical Person and Identity remain owned by Platform Foundation.
- Workforce employment state remains unchanged unless a separate canonical workforce scenario is initiated.
- Relevant authoritative profile-change events are published where defined.

### 16.2 Postconditions

- The EmployeeProfile contains the accepted core profile information.
- The employee's profile remains governed by the Employee Profile lifecycle.
- Subsequent changes remain traceable to the EmployeeProfile business context.
- No duplicate Person, EmployeeProfile, workforce engagement, or assignment ownership is created.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
