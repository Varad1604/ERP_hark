# Position Assignment Suspension or Freeze to Operational Restoration

**Scenario ID:** L3-E-024  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20

## 1. Scenario Metadata
Frozen Stage 3 identity, title, owner, participating domains, Scenario Type and Priority are preserved. This L3 scenario applies the locked upstream architecture and does not redesign it.

## 2. Scenario Purpose
Govern the suspension or freeze of a Position Assignment and its subsequent restoration to an operational state while preserving canonical assignment ownership and history.

## 3. Business Outcome
The Position Assignment reaches the appropriate governed suspended/frozen or restored operational outcome without loss of historical lineage.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An active Position Assignment requires a governed suspension or freeze, or an existing suspended/frozen assignment is eligible for restoration.

### 4.2 End Boundary
The Position Assignment is either in the governed non-operational state or restored to Active, as applicable, with the lifecycle outcome authoritative.

### 4.3 In Scope
Position Assignment suspension; Position Assignment freeze/unfreeze; applicable restoration; effective dating; historical lineage; business governance and auditability.

### 4.4 Out of Scope
Technical implementation, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ implementation concerns are outside this L3 scenario. Upstream lifecycle ownership remains with its canonical owner.

### 4.5 Related Scenarios
Only the upstream and downstream relationships explicitly stated by the authoritative architecture are considered related; no new canonical scenario is created.

## 5. Ownership and Participating Domains
**Canonical Owner:** Position & Assignment.

The Position & Assignment domain owns the relevant Position Assignment / organisational authority / occupancy lifecycle. Organisation (B) and Workforce & Engagement (E) provide authoritative business context within their own ownership boundaries. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Authorised governance administrator/authority, as supported by the L2 workflow.
- Position & Assignment owns the Position Assignment lifecycle.
- Workforce & Engagement and Organisation provide authoritative context where relevant.

No unsupported technical role or permission model is introduced.

## 7. Preconditions and Entry Criteria
Position Assignment is in a lifecycle state from which the applicable suspension/freeze or restoration transition is permitted.

## 8. Trigger
An authorised governance decision requires the Position Assignment to be suspended/frozen or restored.

## 9. Main Business Flow
1. Identify the Position Assignment and its current lifecycle state.
2. Validate the business reason and applicable governance authority.
3. Apply the permitted suspension or freeze transition.
4. Preserve effective dating and historical lineage.
5. Where restoration is authorised, validate the restoration conditions.
6. Restore the Position Assignment to the applicable operational state.
7. Publish the canonical lifecycle event for the completed transition.
8. Stop when the governed operational outcome is reached.

The flow stops when the institutional business outcome and end boundary are reached.

## 10. Alternate Business Flows
- **Suspension path:** Active → Suspended → Active through W-014, with `PositionAssignmentSuspended` / `PositionAssignmentReactivated`.
- **Freeze path:** Active → Frozen → Active through W-018, with `AssignmentFrozen` / `AssignmentUnfrozen`.
- Suspension and freeze are distinct canonical lifecycle paths and are not conflated.

## 11. Failure, Interruption and Business Recovery
If the requested transition is not valid for the current lifecycle state or governance conditions are not satisfied, the Position Assignment remains in its current authoritative state. No technical rollback or retry behaviour is introduced.

Recovery is expressed only as a business outcome; no technical retry, queue, rollback or infrastructure behaviour is defined.

## 12. Business Rules and Decision Constraints
- The Position Assignment lifecycle is canonically owned by Position & Assignment.
- W-014 governs Suspend and Reactivate Position Assignment.
- W-018 governs Assignment Freeze / Unfreeze.
- The locked lifecycle matrix defines Active → Suspended → Active and Active → Frozen → Active.
- Historical assignment lineage remains preserved.
- No new lifecycle state is introduced.

## 13. State and Lifecycle Impact
**Suspension:** Active → Suspended → Active.

**Freeze:** Active → Frozen → Active.

Canonical lifecycle events:
- `PositionAssignmentSuspended`
- `PositionAssignmentReactivated`
- `AssignmentFrozen`
- `AssignmentUnfrozen`

No new state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events
The L2 lifecycle matrix explicitly associates W-014 with `PositionAssignmentSuspended` / `PositionAssignmentReactivated` and W-018 with `AssignmentFrozen` / `AssignmentUnfrozen`. The event publisher is Position & Assignment / Position Assignment, consistent with the canonical ownership of the lifecycle.

### 14.2 Business Contracts / Hand-offs
Workforce & Engagement and Organisation provide contextual business facts; they do not own or alter the Position Assignment lifecycle. Downstream consumers use the resulting canonical business facts.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Governance approval/authorization is required according to the applicable locked workflow. The L2 material identifies authorised governance participation for controlled conflict overrides; this scenario does not invent a more specific approving role. The lifecycle transition and resulting state remain auditable.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Position Assignment reaches the requested governed non-operational state or is restored to Active.
- Correct canonical transition is used.
- No ownership transfer occurs.
- Historical lineage is preserved.
- Applicable canonical event is published.

### 16.2 Postconditions
The resulting business state is authoritative for subsequent institutional use and remains historically traceable.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
