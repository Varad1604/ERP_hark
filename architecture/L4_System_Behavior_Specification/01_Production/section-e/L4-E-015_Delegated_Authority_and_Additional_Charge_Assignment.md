# L4-E-015 — Delegated Authority and Additional Charge Assignment

**Scenario ID:** L4-E-015  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-015 — Delegated Authority and Additional Charge Assignment  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-015. Position & Assignment owns Assignment Delegation and Responsibility Assignment.

## 2. System Behavior Purpose
Specify observable behavior for extending operational authority or responsibility without transferring organisational occupancy.

## 3. System Behavior Outcome
Governed delegated authority and/or Additional Charge becomes effective within its defined scope and period while original Position Assignment ownership remains unchanged.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An active Position Assignment and authorized need for temporary delegation or additional responsibility exist.
### 4.2 End Boundary
Delegated authority/additional responsibility reaches its authoritative active outcome.
### 4.3 In Scope
Validate source/receiving assignments, authority scope and period; apply canonical delegation/additional-charge workflow; preserve occupancy; publish completed facts.
### 4.4 Out of Scope
Adjacent domain-owned outcomes and implementation-level behavior.

## 5. Ownership and Participating Domains
Position & Assignment owns delegation and responsibility assignment. Organisation/Position owner and Workforce & Engagement supply authoritative context.

## 6. Business Actors, Roles and Responsibilities
Authorized Position Holder or institutional administrator establishes the temporary delegation/additional charge; Position & Assignment governs execution.

## 7. Preconditions and Entry Criteria
- Source Position Assignment is Active.
- Delegate assignment, where used, is Active and eligible.
- Scope and effective period are valid.
- Applicable policy and governance are satisfied.

## 8. Trigger
An authorized Position Holder or institutional administrator establishes temporary delegated authority or additional operational responsibility.

## 9. Main System Behavior
1. Validate source/receiving assignment, scope and effective period.
2. For delegation, establish bounded authority without transferring Position occupancy.
3. For Additional Charge, establish responsibility without creating a Position Assignment.
4. Resolve effective authority according to policy.
5. Publish applicable delegation/responsibility business facts.

## 10. Alternate System Behaviors
- Future delegation/charge remains non-effective until its boundary.
- Invalid delegate or scope rejects the request.
- Governance failure prevents activation.
- Expiry/revocation ends delegated authority or responsibility through the canonical lifecycle.

## 11. Failure, Interruption and Business Recovery
When business conditions cannot be satisfied, the existing authoritative Position Assignment remains unchanged. No technical recovery mechanism is specified.

## 12. Business Rules and Decision Constraints
- Delegation does not transfer Position occupancy.
- Delegated authority is explicitly scoped and effective-dated.
- Delegated authority cannot exceed delegator authority.
- Additional Charge does not create a Position Assignment.
- Responsibility changes remain auditable.

## 13. State and Lifecycle Impact
Assignment Delegation and Responsibility Assignment follow their authoritative upstream lifecycles; no new state is introduced.

## 14. Business Events and Cross-Domain Collaboration
Applicable completed delegation/responsibility facts are published by Position & Assignment using authoritative upstream event vocabulary.

## 15. Business Authorization, Approval and Audit Obligations
Delegation and Additional Charge follow institutional governance and remain auditable.

## 16. Completion Criteria and Postconditions
- Authority/responsibility is active within approved scope and period.
- Underlying Position occupancy remains unchanged.
- Completed business facts are available for downstream consumption.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
