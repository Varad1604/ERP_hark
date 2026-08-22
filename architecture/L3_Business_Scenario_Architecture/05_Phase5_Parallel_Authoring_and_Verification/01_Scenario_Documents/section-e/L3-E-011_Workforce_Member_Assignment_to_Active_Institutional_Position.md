# Workforce Member Assignment to Active Institutional Position

**Scenario ID:** L3-E-011
**Primary Owner:** Position & Assignment
**Participating Domains:** B, E
**Scenario Type:** Critical
**Priority:** P0
**Architecture Level:** L3 — Business Scenario Architecture
**Status:** Draft
**Version:** 1.0
**Owner:** Varad
**Last Updated:** 2026-08-20

**Depends On:** L2-E02 — Position & Assignment Workflows; L2-E01 — Workforce & Engagement Workflows; Organisation Management
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Section E identity is preserved and E02 remains the canonical owner of Position Assignment.

## 2. Scenario Purpose
Establish an authoritative Position Assignment linking an active Workforce Engagement to an eligible institutional Position.

## 3. Business Outcome
An eligible Workforce Engagement occupies an institutional Position through the canonical Position Assignment lifecycle and reaches the applicable operational outcome.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An active Workforce Engagement, eligible Position Definition, and authorised assignment decision exist.
### 4.2 End Boundary
The Position Assignment has reached its authoritative operational outcome.
### 4.3 In Scope
- Workforce Engagement resolution.
- Position Definition resolution.
- Occupancy validation.
- Position Assignment creation.
- Required approval/activation.
- Effective dating and history.
- Canonical assignment facts.
### 4.4 Out of Scope
Workforce Engagement creation, Position Definition lifecycle, recruitment, payroll, leave processing, and technical implementation.
### 4.5 Related Scenarios
E001 — Workforce Engagement Establishment and Activation; E010 — Bulk Workforce Organisational Transfer; E012 — Position Assignment Change or Reassignment.

## 5. Ownership and Participating Domains
Position & Assignment owns Position Assignment. Workforce & Engagement supplies Workforce Engagement. Organisation Management supplies Position Definition and organisational structure.

## 6. Business Actors, Roles and Responsibilities
The authorised institutional authority governs assignment approval; Position & Assignment executes the assignment lifecycle; Workforce & Engagement supplies the active engagement; Organisation Management supplies the authoritative Position. No unsupported technical role is introduced.

## 7. Preconditions and Entry Criteria
- Workforce Engagement is Active.
- Position Definition exists and is eligible.
- Organisation context is authoritative.
- Dates are valid.
- Required governance conditions are satisfied.

## 8. Trigger
An authorised business process requires the active Workforce Engagement to occupy an institutional Position.

## 9. Main Business Flow
1. Resolve the active Workforce Engagement.
2. Resolve the authoritative Position Definition.
3. Validate occupancy eligibility.
4. Execute **W-001 — Create Position Assignment**.
5. Establish the Position Assignment in its initial lifecycle position.
6. Apply the E02 approval/activation lifecycle where required.
7. Establish organisational occupancy.
8. Preserve effective dates and assignment history.
9. Publish the canonical completed Position Assignment facts.

## 10. Alternate Business Flows
- Future-effective assignment remains non-operational until its effective boundary.
- Ineligible Position rejects the assignment.
- Inactive Workforce Engagement cannot be assigned.
- Conflicting primary assignment conditions are governed by E02.

## 11. Failure, Interruption and Business Recovery
If validation or governance fails, no new operational assignment outcome is established. Existing Workforce Engagement remains authoritative. Technical recovery is outside L3.

## 12. Business Rules and Decision Constraints
- Every Position Assignment references one Workforce Engagement.
- Every Position Assignment references one Position Definition.
- Only active Position Assignments establish organisational authority.
- Assignment lifecycle uses approved E02 workflows.
- Historical assignment records are immutable.
- Effective dates remain valid.
- Position Assignment ownership is exclusively E02.

## 13. State and Lifecycle Impact
Position Assignment follows the E02 lifecycle. Core creation is **W-001 — Create Position Assignment**, followed by the applicable approval/activation path.

## 14. Business Events and Cross-Domain Collaboration
| Event | Publisher |
|---|---|
| `PositionAssignmentCreated` | Position Assignment |
| `AssignmentVersionCreated` | Position Assignment |
| `OrganizationalAuthorityEstablished` | Position Assignment |

## 15. Business Authorization, Approval and Audit Obligations
Assignment approval and activation follow E02 governance and institution-configurable policy. The scenario does not invent a fixed approver.

## 16. Completion Criteria and Postconditions
- Position Assignment exists for the active Workforce Engagement.
- Position references authoritative Position Definition.
- Required approval/activation is complete.
- Organisational occupancy is authoritative.
- History/effective dating is preserved.
- Canonical events are published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, outcome, lifecycle, event publisher or boundary was identified in the authoritative sources read for this batch.

**Authoring classification: PASS — source-grounded production-quality Draft.**
