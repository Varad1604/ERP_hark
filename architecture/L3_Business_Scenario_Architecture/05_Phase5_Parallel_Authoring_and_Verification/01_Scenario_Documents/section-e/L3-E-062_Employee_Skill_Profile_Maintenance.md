# Employee Skill Profile Maintenance

**Scenario ID:** L3-E-062  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E07 — Employee Profile Workflows  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Maintain the employee's canonical skill profile so that workforce-relevant competencies are recorded accurately within EmployeeProfile and remain available for authorized employee-profile and workforce consumers.

The scenario supports recording and maintaining skill information without transferring ownership to workforce assignment, academic, or other domains.

## 3. Business Outcome

The employee's skill profile accurately reflects the accepted skill information, including applicable proficiency, experience and validation status, while preserving the canonical EmployeeProfile as the sole owner of the skill records.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an employee or authorized HR participant submits a skill for registration or requests a supported change to an existing employee skill.

### 4.2 End Boundary

The scenario ends when the requested skill information has been accepted and the EmployeeProfile contains the resulting authoritative skill record.

### 4.3 In Scope

- Adding an employee skill.
- Updating an existing employee skill.
- Recording skill name and category.
- Recording proficiency level.
- Recording years of experience and last-used information where applicable.
- Recording validation status.
- Supporting self-declared or verified skills as defined by Employee Profile.
- Publishing canonical skill events.

### 4.4 Out of Scope

- Academic qualification registration.
- Professional certification lifecycle.
- Language profile maintenance.
- Workforce position assignment.
- Faculty capability or teaching eligibility decisions.
- Recruitment assessment.
- External skill-provider integration.
- Technical authorization or implementation mechanisms.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to define this boundary.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Employee Profile — Section E | Sole canonical owner of the Skill entity and its profile maintenance lifecycle. |

No other domain participates materially in the canonical skill-maintenance outcome.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Employee | Provides or requests skill information for the employee profile. |
| HR Administrator | Maintains employee skill information within Employee Profile. |
| HR Executive | Supports skill-profile administration. |
| Verification Officer | Participates where a skill is subject to the supported verification process. |

These roles are supported by the locked Employee Profile architecture. No new role is introduced.

## 7. Preconditions and Entry Criteria

- The employee exists.
- A canonical EmployeeProfile exists.
- The requested skill belongs within the EmployeeProfile skill context.
- Required skill information is available.
- Any referenced skill category or proficiency information is valid.
- The requested update does not attempt to alter another canonical business aggregate.

## 8. Trigger

An employee skill is submitted for registration or an existing skill requires an authorized profile update.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain / Role | Business Result |
|---|---|---|---|
| 1 | The employee or authorized HR participant identifies the skill to be recorded or maintained. | Employee / HR Administrator / HR Executive | A skill-maintenance request exists. |
| 2 | Employee Profile validates that the requested information belongs to the EmployeeProfile skill entity and satisfies applicable profile validation. | Employee Profile | The skill change is eligible for application. |
| 3 | Employee Profile adds a new skill or updates the existing skill record as appropriate. | Employee Profile | The canonical skill profile is updated. |
| 4 | Employee Profile records the applicable proficiency, experience, last-used and validation information supported by the profile model. | Employee Profile | The skill record accurately reflects the accepted information. |
| 5 | Employee Profile publishes the corresponding canonical skill event. | Employee Profile | The enterprise has an authoritative skill-profile change fact. |

The flow stops after the EmployeeProfile skill record is authoritative.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Outcome |
|---|---|---|---|
| A1 | The skill already exists. | Employee Profile uses the supported update path rather than creating an unnecessary second representation. | The existing skill record is maintained. |
| A2 | The skill is self-declared. | Employee Profile records the skill with its supported validation status. | Skill remains distinguishable as self-declared or otherwise governed. |
| A3 | The skill is verified. | Employee Profile records the supported verified validation status. | Skill is represented as verified within EmployeeProfile. |
| A4 | Required skill information is incomplete. | The change is not accepted until the required information is available. | Existing authoritative skill information remains unchanged. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution | Owner |
|---|---|---|---|
| Skill information is incomplete or invalid. | The requested change cannot be accepted. | Correct the business information and resubmit. | Employee Profile |
| Requested change targets a capability owned outside Employee Profile. | Employee Profile cannot assume ownership of the other capability. | Route the request to the canonical owning domain. | Relevant owning domain |
| Verification information is not sufficient where verification is required. | The skill cannot be represented as verified. | Complete the supported verification activity before recording the verified outcome. | Employee Profile |

Technical retries, queues, rollback and infrastructure recovery are excluded.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Skill is a child entity of EmployeeProfile. | Employee Profile | Skill ownership remains within E07. |
| Skill records include skill name, skill category, proficiency level, years of experience, last used and validation status. | Employee Profile | These attributes define the business information maintained here. |
| Skills may be self-declared or verified. | Employee Profile | Validation status must represent the actual business position. |
| Skills may be added and updated through the canonical Employee Profile lifecycle. | Employee Profile | Maintenance remains within E07. |
| EmployeeProfile child entities have no independent lifecycle outside the aggregate. | Employee Profile | Skill changes remain governed by EmployeeProfile. |

## 13. State and Lifecycle Impact

| Concept | Entry Position | Scenario Impact | Result | Owner |
|---|---|---|---|---|
| EmployeeProfile | Existing valid profile | Skill record is added or updated. | EmployeeProfile remains in its current lifecycle state with updated skill information. | Employee Profile |
| Skill | New or existing skill | Skill information is recorded or maintained. | Current authoritative skill record | Employee Profile |

No new lifecycle state or transition is introduced for the scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `SkillAdded` | Employee Profile | Establishes that a skill has been recorded. |
| `SkillUpdated` | Employee Profile | Establishes that an existing skill has been changed. |
| `SkillRemoved` | Employee Profile | Establishes removal where the canonical skill lifecycle permits it. |

Only events defined by the authoritative E07 event catalogue are used.

### 14.2 Business Contracts and Cross-Domain Hand-offs

No material synchronous cross-domain business contract is required for the canonical skill-maintenance outcome.

Employee Profile may publish skill facts for dependent consumers, but consumption does not transfer ownership.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

| Action | Business Actor / Role | Authority |
|---|---|---|
| Add or update skill | Employee / HR Administrator / HR Executive | Performs supported skill-profile maintenance. |
| Verify skill | Verification Officer | Participates where a skill is subject to the supported verification process. |

### 15.2 Separation of Duties

The authoritative E07 architecture distinguishes profile administration from verification responsibility. No additional scenario-specific separation-of-duty rule is defined.

### 15.3 Audit / Evidence

Skill additions and changes remain part of the EmployeeProfile historical business record. Any verification outcome remains associated with the EmployeeProfile skill context.

Section A remains the canonical owner of audit infrastructure.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The requested skill information is valid and accepted.
- The EmployeeProfile contains the authoritative skill record.
- Proficiency, experience, last-used and validation information are represented where applicable.
- The skill's validation status accurately reflects whether it is self-declared or verified.
- The canonical skill event corresponding to the completed change has been published.

### 16.2 Postconditions

- The employee skill profile reflects the accepted information.
- Dependent consumers can use the published skill facts without owning the skill record.
- Further skill changes remain governed by Employee Profile.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material architecture issue identified.

### 17.2 Assumptions

No assumptions that override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
