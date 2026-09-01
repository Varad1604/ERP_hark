# L4-E-020 — Service Jurisdiction Assignment to Workforce Authority

**Scenario ID:** L4-E-020  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-020 — Service Jurisdiction Assignment to Workforce Authority  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-020. Position & Assignment remains the canonical owner of the assignment outcome.

## 2. System Behavior Purpose
Specify observable behavior for assigning a workforce-related service jurisdiction to the authoritative workforce context.

## 3. System Behavior Outcome
The authorized service jurisdiction assignment becomes authoritative within its applicable organisational and effective context.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An eligible workforce/assignment context and authorized service jurisdiction decision exist.
### 4.2 End Boundary
The service jurisdiction assignment reaches its authoritative governed outcome.
### 4.3 In Scope
Resolve workforce and jurisdiction context, validate eligibility and authority, establish assignment, preserve effective history, publish applicable business fact.
### 4.4 Out of Scope
Jurisdiction definition ownership, Workforce Engagement lifecycle, payroll, leave and technical implementation.

## 5. Ownership and Participating Domains
Position & Assignment owns the assignment outcome. Organisation and Workforce & Engagement provide authoritative context.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional authority governs the assignment; Position & Assignment executes the governed outcome.

## 7. Preconditions and Entry Criteria
- Workforce context is authoritative.
- Service jurisdiction is defined and eligible.
- Assignment is authorized.
- Effective period is valid.

## 8. Trigger
An authorized institutional decision requires a service jurisdiction to be assigned to the workforce authority context.

## 9. Main System Behavior
1. Resolve the authoritative workforce/assignment context.
2. Resolve the applicable service jurisdiction.
3. Validate eligibility, authority and effective period.
4. Establish the jurisdiction assignment.
5. Preserve effective history.
6. Publish the applicable completed business fact.

## 10. Alternate System Behaviors
- Invalid jurisdiction or workforce context rejects the assignment.
- Future-effective assignment remains non-current until effective.
- Governance failure leaves the current authoritative context unchanged.

## 11. Failure, Interruption and Business Recovery
If validation or governance fails, no new authoritative jurisdiction assignment is established. Existing authoritative business facts remain unchanged.

## 12. Business Rules and Decision Constraints
- Assignment ownership remains Position & Assignment.
- Jurisdiction must be authoritative and eligible.
- Effective dates remain valid.
- Historical assignment facts remain traceable.

## 13. State and Lifecycle Impact
The assignment follows the authoritative upstream lifecycle; no new state is introduced.

## 14. Business Events and Cross-Domain Collaboration
Completed service jurisdiction assignment facts are published by the canonical assignment owner using authoritative event vocabulary.

## 15. Business Authorization, Approval and Audit Obligations
Assignment authorization follows institutional governance and remains auditable.

## 16. Completion Criteria and Postconditions
- Service jurisdiction assignment is authoritative.
- Effective period is established.
- Resulting workforce authority context is traceable.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
