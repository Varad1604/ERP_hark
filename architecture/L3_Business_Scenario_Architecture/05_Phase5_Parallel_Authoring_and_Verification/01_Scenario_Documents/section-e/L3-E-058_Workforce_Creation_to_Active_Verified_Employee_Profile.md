# Workforce Creation to Active Verified Employee Profile

**Scenario ID:** L3-E-058  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile  
**Participating Domains:** A — Platform Foundation; E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E01 — Workforce & Engagement Workflows; L2-E07 — Employee Profile Architecture; L2-A02 — Person & Identity Workflows  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Establish the employee's canonical workforce relationship and complete the dependent Employee Profile lifecycle so that the institution has an active, verified employee profile available for governed workforce use.

The scenario coordinates the boundary between Person and Identity, Workforce & Engagement, and Employee Profile while preserving the ownership of each domain. Workforce creation and engagement activation remain owned by Workforce & Engagement; employee profile information and profile lifecycle remain owned by Employee Profile; Person and Identity remain owned by Platform Foundation.

## 3. Business Outcome

A workforce member has an active workforce engagement and a corresponding Employee Profile that has completed profile verification and reached its Active lifecycle state.

The resulting employee profile is therefore an authoritative, verified profile available for subsequent employee-profile and workforce-dependent business activity.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an institutional workforce appointment has been approved and the required Person/Identity foundation and organisational context are available for Workforce & Engagement to establish the workforce relationship.

### 4.2 End Boundary

The scenario ends when the resulting Employee Profile has completed verification and has reached the Active lifecycle state, with the workforce engagement also active.

### 4.3 In Scope

- Establishing the pending Workforce Engagement following approved appointment.
- Activating the Workforce Engagement after its business preconditions are satisfied.
- Creating the Employee Profile after workforce creation.
- Submitting the Employee Profile for verification.
- Completing profile verification.
- Activating the verified Employee Profile.
- Preserving canonical ownership between Platform Foundation, Workforce & Engagement, and Employee Profile.
- Publishing the authoritative business events associated with the completed lifecycle transitions.

### 4.4 Out of Scope

- Recruitment or candidate selection.
- Creation or modification of the canonical Person or Identity lifecycle beyond the required verified foundation.
- Position Assignment lifecycle, which is owned by L2-E02.
- Responsibility Assignment and Reporting Relationship lifecycle.
- Payroll processing.
- Leave management.
- Faculty capability lifecycle.
- Authentication or authorization lifecycle.
- Employee profile maintenance after activation.
- Qualification, certification, skill, language, or profile-document maintenance after profile activation.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to clarify this boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Section E — Workforce & Engagement | Establishes and activates the Workforce Engagement and remains the canonical owner of the workforce employment relationship and operational workforce state. |
| Section E — Employee Profile | Creates, verifies, and activates the EmployeeProfile aggregate and remains the canonical owner of employee profile information and profile lifecycle. |
| Section A — Platform Foundation | Owns the canonical Person and Identity foundation that must already be valid for workforce establishment; it does not own the employee relationship or Employee Profile. |

Participation does not transfer ownership. The Workforce Engagement and Employee Profile remain distinct canonical aggregates.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authorized HR personnel | Establish and activate the workforce engagement in accordance with the approved appointment and applicable institutional policy. | Performs workforce lifecycle actions. |
| HR Administrator | May participate in employee-profile administration and profile lifecycle activities supported by Employee Profile architecture. | Performs profile administration. |
| HR Executive | May participate in employee-profile administration and verification preparation. | Performs profile administration. |
| Verification Officer | Performs the business verification activity required to move the Employee Profile through verification. | Verifies employee profile information/evidence. |
| Compliance Officer | May participate where profile verification requires compliance review supported by the Employee Profile context. | Reviews verification-related obligations where applicable. |
| Employee | Provides or confirms employee information required for the profile. | Participates in profile information provision/confirmation where required. |

The canonical L2 sources define these business roles but do not define a separate scenario-specific approval authority beyond the workforce workflow and profile verification responsibilities above.

## 7. Preconditions and Entry Criteria

- An institutional appointment has been approved for workforce establishment.
- A valid Person exists and has completed the required Person verification lifecycle.
- The required Identity foundation is valid according to L2-A02.
- The organisational context required for the workforce engagement is valid.
- The Workforce Engagement does not already exist for the appointment.
- The business workflow approval required for workforce engagement establishment is complete.
- Applicable institutional policy requirements are satisfied.
- The Employee Profile does not already exist when profile creation is initiated.
- Required employee identifier and mandatory profile information are available for Employee Profile creation.
- Reference data required by the Employee Profile context is valid.

## 8. Trigger

An approved institutional workforce appointment requires establishment of the corresponding workforce relationship and employee profile.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The approved workforce appointment is accepted as the basis for workforce establishment. | Workforce & Engagement / Authorized HR personnel | Workforce establishment may begin. |
| 2 | Workforce & Engagement creates the Workforce Engagement in its pending lifecycle state. | Workforce & Engagement | A Pending Workforce Engagement exists. |
| 3 | Workforce & Engagement confirms that the pending engagement, verified Person, prepared Position Assignment context, valid Organisation, completed workflow approval, and applicable policy conditions satisfy activation prerequisites. | Workforce & Engagement | Activation eligibility is established. |
| 4 | Workforce & Engagement activates the Workforce Engagement. | Workforce & Engagement / Authorized HR personnel | The workforce relationship becomes active. |
| 5 | Employee Profile establishes the EmployeeProfile for the newly created employee, provided no existing profile exists and the required employee identifier, mandatory personal information, and reference data are available. | Employee Profile | An Employee Profile is created. |
| 6 | Employee Profile moves the profile into its verification process and records that verification has been requested. | Employee Profile / Verification Officer | The profile is awaiting completion of verification. |
| 7 | The required profile verification is completed and the Employee Profile records the verified outcome. | Employee Profile / Verification Officer | The Employee Profile reaches Verified. |
| 8 | Employee Profile activates the verified profile. | Employee Profile / authorized profile administrator | The Employee Profile reaches Active. |
| 9 | The completed workforce and profile lifecycle facts remain available to dependent business domains through their authoritative business events and governed read/collaboration mechanisms. | Workforce & Engagement / Employee Profile | The institutional employee relationship is represented by an active workforce engagement and active verified Employee Profile. |

The flow stops once the Business Outcome and End Boundary are achieved.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | Required Employee Profile information is not yet available when profile creation is reached. | Employee Profile creation does not proceed until the required information becomes available. | Workforce state remains governed by Workforce & Engagement; profile activation is not completed. | Employee Profile |
| A2 | The profile verification is not yet complete after submission. | The Employee Profile remains in the verification portion of its lifecycle until the verification outcome is completed. | Employee Profile does not reach Active until verification completes. | Employee Profile |
| A3 | The profile verification is rejected. | The Employee Profile follows the governed verification rejection path rather than proceeding to activation. | The Employee Profile is not activated from the rejected verification outcome. | Employee Profile |

No alternate path changes the canonical owner or creates a second Employee Profile.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Workforce activation prerequisites are not satisfied. | The Workforce Engagement cannot progress to Active. | The engagement remains governed by the Workforce lifecycle until the business prerequisites are satisfied through the applicable business process. | Workforce & Engagement |
| A required Person/Identity foundation is not valid. | Workforce establishment cannot validly complete. | Resolve the Person/Identity business prerequisite under Platform Foundation before workforce activation proceeds. | Platform Foundation |
| An Employee Profile already exists for the employee. | A second canonical Employee Profile must not be created. | Use the existing canonical profile rather than creating a duplicate. | Employee Profile |
| Employee Profile verification is incomplete. | The profile cannot reach its verified/active outcome. | Complete the governed verification process or address the business reason for the pending verification. | Employee Profile |
| Employee Profile verification is rejected. | The intended active verified profile outcome is not achieved. | Follow the governed verification correction/re-submission path where defined by Employee Profile governance. | Employee Profile |

No technical retry, queue, rollback, server failure, or infrastructure recovery is defined at L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Workforce Engagement is the canonical aggregate for the workforce relationship. | Workforce & Engagement | Workforce creation and activation are performed through the Workforce lifecycle. | L2-E01 — Workforce & Engagement Workflows |
| Person must be verified before Workforce Engagement activation. | Platform Foundation / Workforce & Engagement dependency | The scenario cannot validly activate a workforce relationship without a verified Person foundation. | L2-E01; L2-A02 |
| Position Assignment is owned by L2-E02 and is not created or modified by E01. | Position & Assignment | The scenario may depend on prepared assignment context but must not create or re-own Position Assignment. | L2-E01 ownership clarification |
| Employee Profile is created only when an employee exists and no existing profile exists. | Employee Profile | Prevents duplicate EmployeeProfile creation. | L2-E07 — CreateEmployeeProfile |
| Mandatory employee information and valid reference data are required for profile creation. | Employee Profile | Profile creation cannot complete without these business prerequisites. | L2-E07 — CreateEmployeeProfile |
| Qualification, certification, skills, language, and document records are part of Employee Profile but are separate profile-maintenance capabilities. | Employee Profile | They are not expanded into this workforce-creation scenario. | L2-E07 |
| Verification must be completed before profile activation. | Employee Profile | An unverified profile does not become Active through this scenario. | L2-E07 lifecycle and verification workflows |
| Workforce lifecycle ownership remains with Workforce & Engagement; Employee Profile lifecycle ownership remains with Employee Profile. | People & Workforce | Cross-domain participation does not transfer lifecycle ownership. | L2-E01; L2-E07 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Workforce Engagement | Approved appointment / pre-engagement condition | Workforce engagement is established and activated. | Active Workforce Engagement | Workforce & Engagement | L2-E01 |
| EmployeeProfile | Employee exists; no profile | Employee profile is created and enters its governed verification lifecycle. | Created → PendingVerification → Verified → Active | Employee Profile | L2-E07 |
| Person | Verified | Person remains under its own canonical lifecycle; the scenario consumes its valid state. | Active Person foundation remains unchanged by this scenario | Person & Identity | L2-A02 |
| Identity | Verified / valid identity foundation | Identity remains under Platform Foundation ownership. | Active Identity foundation remains unchanged by this scenario | Person & Identity | L2-A02 |

The scenario does not create a combined lifecycle between Workforce Engagement and EmployeeProfile; it coordinates two separately owned lifecycles.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PendingWorkforceEngagementCreated | Workforce & Engagement | Dependent workforce domains | Establishes the business fact that a pending workforce engagement has been created. | L2-E01 |
| WorkforceEngagementActivated | Workforce & Engagement | Dependent workforce domains | Establishes the authoritative fact that the workforce engagement is active. | L2-E01 |
| EmployeeProfileCreated | Employee Profile | Workforce-dependent and enterprise consumers | Establishes the authoritative fact that the employee profile exists. | L2-E07 |
| ProfileSubmittedForVerification | Employee Profile | Verification-related participants | Establishes that profile verification has been requested. | L2-E07 |
| ProfileVerified | Employee Profile | Workforce-dependent and profile consumers | Establishes that profile verification has completed. | L2-E07 |
| EmployeeProfileActivated | Employee Profile | Employee directory and dependent workforce consumers | Establishes that the Employee Profile is active. | L2-E07 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Person & Identity | Workforce & Engagement | Verified Person / Identity business context | Provides the verified identity foundation required for workforce establishment. | L2-A02; L2-E01 |
| Workforce & Engagement | Employee Profile | Workforce creation / employee establishment hand-off | Provides the business fact that the employee exists so the Employee Profile can be created. | L2-E01; L2-E07 |
| Employee Profile | Workforce-dependent domains | Active verified employee profile fact | Makes the completed profile outcome available to dependent domains. | L2-E07 |

The scenario does not define APIs, payloads, transport mechanisms, queues, or implementation-level integration details.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish workforce engagement | Authorized HR personnel | Workforce establishment follows the approved appointment and applicable institutional policy. | Workforce & Engagement |
| Activate workforce engagement | Authorized HR personnel | Workforce activation requires the L2-E01 activation preconditions to be satisfied. | Workforce & Engagement |
| Administer Employee Profile | HR Administrator / HR Executive | Profile actions are performed within the Employee Profile business context. | Employee Profile |
| Complete profile verification | Verification Officer | Verification is a distinct business responsibility and must complete before profile activation. | Employee Profile |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is explicitly defined in the authoritative sources reviewed beyond the distinct verification responsibility and the governed workforce approval process.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Workforce establishment and activation | Authoritative workforce lifecycle outcome and associated business history | Workforce & Engagement |
| Employee profile creation and verification | Employee Profile lifecycle and verification history | Employee Profile |
| Person/Identity verification prerequisite | Authoritative Person and Identity lifecycle evidence | Person & Identity |

Section A remains the canonical owner of platform audit infrastructure; this scenario does not define technical audit mechanics.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A valid Workforce Engagement exists and is Active.
- The corresponding Employee Profile exists without creating a duplicate profile.
- The Employee Profile has completed the required verification and is in Verified state before activation.
- The Employee Profile is Active.
- Workforce Engagement and EmployeeProfile remain owned by their respective canonical domains.
- The authoritative workforce and employee-profile business events have been published for the completed lifecycle transitions.

### 16.2 Postconditions

- The institution has an active workforce relationship for the employee.
- The employee has an active, verified Employee Profile.
- The canonical Person and Identity remain owned by Platform Foundation.
- Subsequent employee profile maintenance is governed by Employee Profile scenarios rather than this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
