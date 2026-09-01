# L4-E-011 — Workforce Member Assignment to Active Institutional Position

**Scenario ID:** L4-E-011  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-011 — Workforce Member Assignment to Active Institutional Position  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-011. Position & Assignment remains the canonical owner of Position Assignment.

## 2. System Behavior Purpose
Specify observable behavior for establishing an authoritative Position Assignment linking an active Workforce Engagement to an eligible institutional Position.

## 3. System Behavior Outcome
An eligible Workforce Engagement occupies an institutional Position through the canonical Position Assignment lifecycle and reaches its applicable operational outcome.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An active Workforce Engagement, eligible Position Definition, and authorized assignment decision exist.
### 4.2 End Boundary
The Position Assignment reaches its authoritative operational outcome.
### 4.3 In Scope
- Resolve Workforce Engagement and Position Definition.
- Validate occupancy eligibility and dates.
- Establish Position Assignment.
- Apply required approval/activation lifecycle.
- Preserve assignment history.
### 4.4 Out of Scope
Workforce Engagement creation, Position Definition lifecycle, recruitment, payroll, leave processing, and technical implementation.

## 5. Ownership and Participating Domains
Position & Assignment owns Position Assignment. Workforce & Engagement supplies the active engagement. Organisation Management supplies authoritative organisational/Position context.

## 6. Business Actors, Roles and Responsibilities
Authorized institutional authority governs assignment approval; Position & Assignment executes the lifecycle; participating domains supply authoritative business context.

## 7. Preconditions and Entry Criteria
- Workforce Engagement is Active.
- Position Definition exists and is eligible.
- Organisation context is authoritative.
- Dates and governance conditions are valid.

## 8. Trigger
An authorized business process requires the active Workforce Engagement to occupy an institutional Position.

## 9. Main System Behavior
1. Resolve the active Workforce Engagement.
2. Resolve the authoritative Position Definition.
3. Validate occupancy eligibility.
4. Execute **W-001 — Create Position Assignment**.
5. Establish the assignment in its applicable lifecycle position.
6. Apply required approval/activation.
7. Establish organisational occupancy.
8. Preserve effective dates and assignment history.
9. Publish the canonical completed Position Assignment facts.

## 10. Alternate System Behaviors
- A future-effective assignment remains non-operational until its effective boundary.
- An ineligible Position rejects the assignment.
- An inactive Workforce Engagement cannot be assigned.
- Conflicting primary assignment conditions are governed by E02.

## 11. Failure, Interruption and Business Recovery
If validation or governance fails, no new operational assignment outcome is established and the existing authoritative Workforce Engagement remains unchanged. Technical recovery is outside L4.

## 12. Business Rules and Decision Constraints
- Every Position Assignment references one Workforce Engagement and one Position Definition.
- Only active Position Assignments establish organisational authority.
- Assignment lifecycle follows approved E02 workflows.
- Historical assignment records remain immutable.

## 13. State and Lifecycle Impact
Position Assignment follows the E02 lifecycle; no new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `PositionAssignmentCreated` | Position & Assignment | Position Assignment has been established. |
| `AssignmentVersionCreated` | Position & Assignment | A governed assignment version has been established. |
| `OrganizationalAuthorityEstablished` | Position & Assignment | Organisational occupancy/authority has been established. |

## 15. Business Authorization, Approval and Audit Obligations
Assignment approval and activation follow E02 governance and remain auditable. No fixed technical approver is introduced.

## 16. Completion Criteria and Postconditions
- Position Assignment reaches the authoritative operational outcome.
- Organisational occupancy is established where applicable.
- Effective dates and history are preserved.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, boundary, lifecycle or outcome was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
