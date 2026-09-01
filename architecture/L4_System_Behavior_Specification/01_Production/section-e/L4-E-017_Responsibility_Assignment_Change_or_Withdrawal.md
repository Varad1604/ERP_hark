# L4-E-017 — Responsibility Assignment Change or Withdrawal

**Scenario ID:** L4-E-017  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-017 — Responsibility Assignment Change or Withdrawal  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-017. Responsibility Assignment remains E02-owned.

## 2. System Behavior Purpose
Specify observable behavior for governed change or withdrawal of a Responsibility Assignment while preserving underlying Position Assignment ownership.

## 3. System Behavior Outcome
The responsibility assignment reaches its authorized successor or withdrawn outcome, with effective dating and history preserved.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An existing Responsibility Assignment is subject to an authorized change or withdrawal.
### 4.2 End Boundary
The approved responsibility outcome is authoritative.
### 4.3 In Scope
Validate existing responsibility, apply governed change/withdrawal, preserve assignment and historical lineage, publish applicable completed fact.
### 4.4 Out of Scope
Position Definition lifecycle, Workforce Engagement lifecycle, payroll, leave and technical implementation.

## 5. Ownership and Participating Domains
Position & Assignment owns Responsibility Assignment. Other domains retain their authoritative context.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional authority governs the change; Position & Assignment executes the responsibility lifecycle.

## 7. Preconditions and Entry Criteria
- Existing Responsibility Assignment is valid.
- Change/withdrawal is authorized.
- Effective period and resulting scope are valid.

## 8. Trigger
An authorized responsibility change or withdrawal is established.

## 9. Main System Behavior
1. Resolve the existing Responsibility Assignment.
2. Validate authorization and effective period.
3. Determine the approved successor or withdrawal outcome.
4. Apply the canonical responsibility lifecycle.
5. Preserve prior responsibility history.
6. Publish the completed responsibility fact.

## 10. Alternate System Behaviors
- Invalid authorization prevents change.
- Future-effective changes remain non-current until effective.
- Withdrawal ends responsibility without changing underlying Position occupancy.

## 11. Failure, Interruption and Business Recovery
If validation fails, the current authoritative Responsibility Assignment remains unchanged. No technical recovery mechanism is specified.

## 12. Business Rules and Decision Constraints
- Responsibility Assignment ownership remains E02.
- Responsibility scope must remain governed.
- Effective dating and historical traceability are preserved.
- Withdrawal does not terminate Position occupancy.

## 13. State and Lifecycle Impact
Responsibility Assignment follows its authoritative lifecycle; no new state is introduced.

## 14. Business Events and Cross-Domain Collaboration
Completed responsibility changes are published by Position & Assignment using the authoritative upstream event vocabulary.

## 15. Business Authorization, Approval and Audit Obligations
Responsibility changes and withdrawals follow institutional governance and remain auditable.

## 16. Completion Criteria and Postconditions
- Approved responsibility successor or withdrawal is authoritative.
- Prior history remains preserved.
- Underlying Position Assignment remains governed independently.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
