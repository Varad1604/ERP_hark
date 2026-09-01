# L4-E-019 — Reporting Relationship Change

**Scenario ID:** L4-E-019  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-019 — Reporting Relationship Change  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-019. Position & Assignment remains the canonical owner.

## 2. System Behavior Purpose
Specify observable behavior for governing an authorized change to an existing reporting relationship while preserving historical lineage.

## 3. System Behavior Outcome
The approved successor reporting relationship becomes authoritative at its effective boundary and the prior relationship remains traceable history.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An existing reporting relationship is subject to an authorized organizational change.
### 4.2 End Boundary
The successor reporting relationship is authoritative at its effective boundary.
### 4.3 In Scope
Validate existing relationship and target context, apply governed change, preserve prior relationship history, establish effective successor, publish completed fact.
### 4.4 Out of Scope
Position Definition lifecycle, Workforce Engagement lifecycle, payroll, leave and technical implementation.

## 5. Ownership and Participating Domains
Position & Assignment owns the reporting relationship change. Other domains provide authoritative organizational and workforce context.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional authority governs the change; Position & Assignment executes the relationship lifecycle.

## 7. Preconditions and Entry Criteria
- Existing reporting relationship is valid.
- Target relationship/context is authoritative.
- Change is authorized.
- Effective dates are valid.

## 8. Trigger
An authorized change to an existing reporting relationship is established.

## 9. Main System Behavior
1. Resolve the existing reporting relationship.
2. Resolve the target reporting context.
3. Validate eligibility, authority and effective dates.
4. Establish the approved successor relationship.
5. Preserve prior relationship lineage.
6. Publish the completed reporting-change fact.

## 10. Alternate System Behaviors
- Future-effective changes remain non-current until effective.
- Invalid target context rejects the change.
- Governance failure leaves the current relationship authoritative.

## 11. Failure, Interruption and Business Recovery
If validation or governance fails, the current authoritative reporting relationship remains unchanged. No technical rollback or retry behavior is specified.

## 12. Business Rules and Decision Constraints
- Reporting relationship ownership remains Position & Assignment.
- Effective dating must remain valid.
- Prior relationship history remains traceable.

## 13. State and Lifecycle Impact
The reporting relationship follows its authoritative change lifecycle; no new state is introduced.

## 14. Business Events and Cross-Domain Collaboration
Completed reporting relationship changes are published by the canonical assignment owner using authoritative upstream event vocabulary.

## 15. Business Authorization, Approval and Audit Obligations
Changes follow institutional governance and remain auditable.

## 16. Completion Criteria and Postconditions
- Successor reporting relationship is authoritative when effective.
- Prior relationship lineage is preserved.
- Organizational reporting facts are consistent with the approved outcome.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
