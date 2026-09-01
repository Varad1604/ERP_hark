# L4-E-018 — Reporting Relationship Establishment and Governance

**Scenario ID:** L4-E-018  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-018 — Reporting Relationship Establishment and Governance  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-018. Reporting Relationship establishment remains within the canonical assignment boundary.

## 2. System Behavior Purpose
Specify observable behavior for establishing a governed reporting relationship between eligible organisational assignments.

## 3. System Behavior Outcome
The authorized reporting relationship becomes authoritative within the applicable organizational context and effective period.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
Eligible source and reporting assignments and an authorized reporting relationship decision exist.
### 4.2 End Boundary
The reporting relationship reaches its authoritative governed outcome.
### 4.3 In Scope
Resolve assignments, validate organizational context and reporting eligibility, establish relationship, preserve effective dating/history, publish applicable fact.
### 4.4 Out of Scope
Position Definition lifecycle, Workforce Engagement lifecycle, payroll, leave and technical implementation.

## 5. Ownership and Participating Domains
Position & Assignment owns the assignment relationship outcome; organizational and workforce domains supply authoritative context.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional authority governs the relationship; Position & Assignment executes the governed assignment outcome.

## 7. Preconditions and Entry Criteria
- Source assignment is valid.
- Reporting assignment is valid.
- Organizational context is authoritative.
- Reporting relationship is authorized.

## 8. Trigger
An authorized business decision requires a reporting relationship to be established.

## 9. Main System Behavior
1. Resolve source and reporting assignments.
2. Validate reporting eligibility and organizational context.
3. Validate effective period and governance.
4. Establish the reporting relationship.
5. Preserve effective dates and historical lineage.
6. Publish the applicable completed relationship fact.

## 10. Alternate System Behaviors
- Invalid assignment or organizational context rejects establishment.
- Future-effective relationship remains non-current until effective.
- Governance failure prevents activation.

## 11. Failure, Interruption and Business Recovery
The existing authoritative reporting state remains unchanged if establishment cannot satisfy business conditions. Technical recovery is outside L4.

## 12. Business Rules and Decision Constraints
- Reporting relationships require authoritative assignment context.
- Effective periods must remain valid.
- Historical relationship facts remain traceable.

## 13. State and Lifecycle Impact
The relationship follows the authoritative assignment lifecycle; no unsupported state is introduced.

## 14. Business Events and Cross-Domain Collaboration
Completed reporting relationship facts are published by the canonical assignment owner using authoritative event vocabulary.

## 15. Business Authorization, Approval and Audit Obligations
Reporting relationship establishment follows institutional governance and remains auditable.

## 16. Completion Criteria and Postconditions
- Reporting relationship is authoritative.
- Effective period is established.
- Historical lineage is preserved.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
