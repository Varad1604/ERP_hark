# Employee Language Profile Maintenance

**Scenario ID:** L3-E-063  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E07 — Employee Profile Workflows  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Maintain the employee's canonical language profile so that languages known by the employee and their supported reading, writing and speaking proficiency are represented accurately within EmployeeProfile.

The scenario keeps language information within the Employee Profile bounded context and does not turn language information into an independent workforce, academic or capability lifecycle.

## 3. Business Outcome

The employee's language profile contains the accepted language information and corresponding proficiency levels, with the resulting records maintained as canonical EmployeeProfile information.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an employee or authorized HR participant submits a language for registration or requests a supported change to an existing language record.

### 4.2 End Boundary

The scenario ends when the requested language information and supported proficiency values have been accepted into EmployeeProfile.

### 4.3 In Scope

- Adding an employee language.
- Updating an employee language.
- Recording reading proficiency.
- Recording writing proficiency.
- Recording speaking proficiency.
- Recording a certification reference where supported by the Employee Profile language model.
- Publishing canonical language events.

### 4.4 Out of Scope

- Professional certification lifecycle.
- Academic qualification lifecycle.
- Skill profile maintenance.
- Workforce engagement.
- Faculty teaching eligibility.
- Recruitment language assessment.
- External language-provider verification.
- Technical authorization or implementation mechanics.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to define this boundary.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Employee Profile — Section E | Sole canonical owner of the Language entity and its profile-maintenance lifecycle. |

No other domain materially participates in the language-profile outcome.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Employee | Provides or requests language information for the employee profile. |
| HR Administrator | Maintains employee language information within Employee Profile. |
| HR Executive | Supports language-profile administration. |
| Verification Officer | Participates where a language record uses a supported verification or certification reference. |

These roles are supported by the locked Employee Profile architecture. No additional actor is invented.

## 7. Preconditions and Entry Criteria

- The employee exists.
- A canonical EmployeeProfile exists.
- The requested language belongs within the EmployeeProfile language context.
- The language and proficiency information are available.
- Any certification reference supplied for the language is supported by the profile context.
- The requested action does not attempt to modify another canonical aggregate.

## 8. Trigger

An employee language is submitted for registration or an existing language record requires an authorized profile update.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain / Role | Business Result |
|---|---|---|---|
| 1 | The employee or authorized HR participant identifies the language to be recorded or maintained. | Employee / HR Administrator / HR Executive | A language-maintenance request exists. |
| 2 | Employee Profile validates that the language information belongs to the EmployeeProfile language entity. | Employee Profile | The language change is eligible for application. |
| 3 | Employee Profile records the language and applicable reading, writing and speaking proficiency. | Employee Profile | The canonical language record is established or updated. |
| 4 | Where a supported certification reference exists, Employee Profile records that reference with the language information. | Employee Profile | The language record contains the supported evidence reference. |
| 5 | Employee Profile publishes the corresponding canonical language event. | Employee Profile | The enterprise has an authoritative language-profile change fact. |

The flow stops after the EmployeeProfile language record is authoritative.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Outcome |
|---|---|---|---|
| A1 | The language already exists for the employee. | Employee Profile uses the supported update path rather than creating an unnecessary duplicate representation. | The existing language record is maintained. |
| A2 | Different proficiency levels are supplied for reading, writing and speaking. | Employee Profile records each proficiency independently as defined by the language model. | The language profile reflects the distinct proficiencies. |
| A3 | A certification reference is supplied. | Employee Profile records the supported certification reference. | The language record retains the provided evidence reference. |
| A4 | Required language information is incomplete. | The requested change is not accepted until the information is complete. | Existing authoritative language information remains unchanged. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution | Owner |
|---|---|---|---|
| Language information is incomplete or invalid. | The requested change cannot be accepted. | Correct the business information and resubmit. | Employee Profile |
| The requested action targets a capability outside Employee Profile. | Employee Profile cannot assume ownership of that capability. | Route the request to the canonical owning domain. | Relevant owning domain |
| A supplied certification reference is not valid for the language context. | The language record cannot represent the unsupported reference as authoritative. | Correct or remove the unsupported reference before acceptance. | Employee Profile |

Technical retries, queues, rollback and infrastructure recovery are excluded.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Language is a child entity of EmployeeProfile. | Employee Profile | Language ownership remains within E07. |
| Language records include reading, writing and speaking proficiency. | Employee Profile | Proficiency is maintained separately for each supported dimension. |
| Multiple languages are supported. | Employee Profile | The profile may contain multiple language records. |
| Language records may contain a certification reference. | Employee Profile | Evidence references are kept within the language-profile context where supplied. |
| Language changes are maintained through the EmployeeProfile aggregate. | Employee Profile | No independent language lifecycle is created. |

## 13. State and Lifecycle Impact

| Concept | Entry Position | Scenario Impact | Result | Owner |
|---|---|---|---|---|
| EmployeeProfile | Existing valid profile | Language record is added or updated. | EmployeeProfile remains in its current lifecycle state with updated language information. | Employee Profile |
| Language | New or existing language | Language/proficiency information is recorded or maintained. | Current authoritative language record | Employee Profile |

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `LanguageAdded` | Employee Profile | Establishes that a language has been added to an employee profile. |
| `LanguageUpdated` | Employee Profile | Establishes that an existing language record has been changed. |
| `LanguageRemoved` | Employee Profile | Establishes removal where the canonical language lifecycle permits it. |

Only events defined by the authoritative E07 event catalogue are used.

### 14.2 Business Contracts and Cross-Domain Hand-offs

No material synchronous cross-domain business contract is required for the canonical language-maintenance outcome.

Employee Profile may publish language facts for dependent consumers, but consumption does not transfer ownership.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

| Action | Business Actor / Role | Authority |
|---|---|---|
| Add or update language | Employee / HR Administrator / HR Executive | Performs supported language-profile maintenance. |
| Review supported evidence | Verification Officer | Participates where a language certification/reference requires review. |

### 15.2 Separation of Duties

No additional scenario-specific separation-of-duty rule is defined in the authoritative Employee Profile architecture.

### 15.3 Audit / Evidence

Language additions and changes remain part of the EmployeeProfile historical business record. Supported certification references remain associated with the language record.

Section A remains the canonical owner of audit infrastructure.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The requested language information is valid and accepted.
- The EmployeeProfile contains the authoritative language record.
- Reading, writing and speaking proficiency are represented as supplied and supported.
- Any supported certification reference is retained with the language record.
- The canonical language event corresponding to the completed change has been published.

### 16.2 Postconditions

- The employee language profile reflects the accepted information.
- Dependent consumers can use published language facts without owning the language record.
- Further language changes remain governed by Employee Profile.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material architecture issue identified.

### 17.2 Assumptions

No assumptions that override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
