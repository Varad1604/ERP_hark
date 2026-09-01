# L4-E-014 — Supplementary Position Occupancy Assignment

**Scenario ID:** L4-E-014  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-014 — Supplementary Position Occupancy Assignment  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-014. Position & Assignment owns supplementary Position Assignment lifecycle and precedence.

## 2. System Behavior Purpose
Specify observable behavior for establishing supplementary organisational occupancy without replacing the Primary Assignment.

## 3. System Behavior Outcome
The supplementary Position Assignment reaches its applicable active outcome while the Primary Assignment remains authoritative for enterprise default behavior.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An active Workforce Engagement has a Primary Assignment and authorization for supplementary occupancy.
### 4.2 End Boundary
The supplementary assignment reaches its applicable active outcome and coexists with the Primary Assignment.
### 4.3 In Scope
Validate engagement, Primary Assignment, target Position and capacity; establish additional assignment; preserve precedence; publish applicable facts.
### 4.4 Out of Scope
Adjacent domain-owned outcomes and implementation-level behavior.

## 5. Ownership and Participating Domains
Position & Assignment owns supplementary assignment. Organisation/Position owner supplies Position context; Workforce & Engagement supplies active engagement.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional administration establishes the additional appointment; Position & Assignment governs execution.

## 7. Preconditions and Entry Criteria
- Workforce Engagement is Active.
- Primary Assignment exists.
- Target Position is active and eligible.
- Policy permits an additional assignment.
- Required governance is satisfied.

## 8. Trigger
An authorized administrator establishes an additional organisational appointment.

## 9. Main System Behavior
1. Validate Primary Assignment and target Position.
2. Validate occupancy eligibility.
3. Execute **W-009 — Additional Position Assignment**.
4. Preserve the Primary Assignment.
5. Establish deterministic assignment precedence.
6. Publish **`AdditionalAssignmentCreated`** and applicable precedence facts.

## 10. Alternate System Behaviors
- Future supplementary assignments remain non-operational until effective.
- Missing Primary Assignment prevents supplementary establishment.
- Invalid capacity/policy rejects the additional assignment.
- Ambiguous precedence prevents safe activation until governed resolution.

## 11. Failure, Interruption and Business Recovery
Existing authoritative assignment remains unchanged when validation or governance fails. No technical recovery behavior is specified.

## 12. Business Rules and Decision Constraints
- A Workforce Engagement has exactly one Primary Assignment.
- Additional Assignments do not replace the Primary Assignment.
- Precedence remains deterministic.
- Closing an Additional Assignment does not invalidate the Primary Assignment.

## 13. State and Lifecycle Impact
Supplementary Position Assignment follows the applicable effective-dated lifecycle; no new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `AdditionalAssignmentCreated` | Position & Assignment | Supplementary appointment has been established. |
| `AdditionalAssignmentEnded` | Position & Assignment | Supplementary appointment has ended, where applicable. |
| `AssignmentPriorityChanged` | Position & Assignment | Governed assignment precedence has changed, where applicable. |
| `EnterpriseDefaultAssignmentChanged` | Position & Assignment | Enterprise default assignment has changed, where applicable. |

## 15. Business Authorization, Approval and Audit Obligations
Additional assignment follows institutional governance and remains auditable.

## 16. Completion Criteria and Postconditions
- Supplementary assignment reaches its applicable active/scheduled outcome.
- Primary Assignment remains authoritative.
- Assignment precedence is deterministic.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
