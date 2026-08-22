# Position Assignment Rollback to Restored Valid Organisational State

**Scenario ID:** L3-E-072  
**Status:** Draft  
**Primary Owner:** Position & Assignment  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P1  
**Architecture Level:** L3 — Business Scenario Architecture  
**Author:** Varad

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-072 |
| Canonical Title | Position Assignment Rollback to Restored Valid Organisational State |
| Primary Owner | Position & Assignment |
| Participating Domains | E — People & Workforce |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Restore a Position Assignment to the last valid governed organisational state when an assignment change is determined to have produced an invalid or incorrect organisational occupancy outcome.

The rollback is a business restoration of assignment state within L2-E02. It does not imply technical rollback, database restoration, or reversion of unrelated domains.

## 3. Business Outcome

The Position Assignment is returned to a valid authoritative organisational state through the governed assignment lifecycle, while the invalid or superseded state remains historically traceable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authoritative Position Assignment is found to require restoration to a previously valid organisational state.

### 4.2 End Boundary

The scenario ends when the restored assignment state is authoritative, valid, and historically traceable.

### 4.3 In Scope

- Identification of an invalid assignment outcome.
- Validation of the previously valid assignment state.
- Business restoration of Position Assignment.
- Preservation of assignment lineage and effective history.
- Publication of the canonical restored assignment event.

### 4.4 Out of Scope

- Technical rollback of implementation state.
- Changing Position Definition.
- Changing Workforce Engagement.
- Reversing unrelated business transactions.
- Authorization infrastructure.
- Database or infrastructure recovery.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Position & Assignment | Canonical owner of Position Assignment restoration and assignment history. |
| Workforce & Engagement | Remains owner of the workforce relationship referenced by the assignment. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| HR Administrator | Raises or administers the business restoration request. |
| Department Administrator | Provides organisational context for the correction where required. |
| Authorised organisational administrator | Participates in the governed restoration decision. |

## 7. Preconditions and Entry Criteria

- Position Assignment exists.
- A prior valid organisational state can be established from authoritative history.
- Workforce Engagement remains valid for the restored assignment.
- Restored Position Definition context remains valid.
- The business authority required for restoration is satisfied.

## 8. Trigger

An authoritative finding determines that the current Position Assignment must be restored to a valid prior organisational state.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | The current assignment state is determined to be invalid or incorrect. | Position & Assignment | Restoration need is established. |
| 2 | The authoritative prior valid organisational state is identified from assignment history. | Position & Assignment | Restoration target is established. |
| 3 | The restoration target is validated against current Position Definition and Workforce Engagement ownership boundaries. | Position & Assignment | Restoration is eligible. |
| 4 | Position Assignment is restored through its governed business lifecycle. | Position & Assignment | Valid organisational occupancy is restored. |
| 5 | The prior invalid/superseded state remains historically traceable. | Position & Assignment | Complete assignment lineage is preserved. |
| 6 | Position & Assignment publishes the canonical restored-state event. | Position & Assignment | Downstream consumers receive the restored authoritative occupancy fact. |

## 10. Alternate Business Flows

### A1 — Prior State No Longer Valid

If the prior state is no longer valid because the referenced Position Definition or Workforce Engagement has changed authoritatively, the restoration cannot blindly recreate that state and must follow the current canonical assignment rules.

### A2 — Restoration Would Cross Ownership Boundary

If restoring the assignment would require changing a Workforce Engagement or Position Definition fact, the relevant owning domain must perform that change first.

## 11. Failure, Interruption and Business Recovery

| Situation | Resolution |
|---|---|
| No valid historical state can be established | Do not invent one; retain the current authoritative state pending governed resolution. |
| Restored Position Definition is no longer valid | Route the issue to Organisation; do not create an invalid assignment. |
| Workforce Engagement is incompatible with restored occupancy | Resolve the workforce-side condition before restoring the assignment. |
| Restoration violates an assignment invariant | Keep the current authoritative state until the business condition is resolved. |

Technical rollback/retry infrastructure is outside L3.

## 12. Business Rules and Decision Constraints

- Position Assignment is canonically owned by L2-E02.
- Historical assignment states remain immutable and reconstructable.
- Every Position Assignment references one Workforce Engagement and one Position Definition.
- Business evolution produces versioned lineage.
- Organisational authority is derived from active Position Assignments.
- Restoration cannot silently rewrite history.

## 13. State and Lifecycle Impact

| Concept | Impact | Owner |
|---|---|---|
| Position Assignment | Restored to a valid governed state through the canonical assignment lifecycle. | Position & Assignment |
| Historical assignment version | Preserved as part of assignment lineage. | Position & Assignment |
| Workforce Engagement | Remains unchanged unless a separate workforce scenario applies. | Workforce & Engagement |
| Position Definition | Remains unchanged unless a separate organisation scenario applies. | Organisation |

No new lifecycle state is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

Position & Assignment publishes the canonical event associated with restoration.

The exact frozen rollback/restoration event name must be taken from the authoritative Stage 3/L2 mapping before Architecture Approval.

### 14.2 Business Hand-offs

No new cross-domain contract is introduced.

Position & Assignment consumes authoritative Workforce Engagement and Position Definition context while retaining sole ownership of the Position Assignment lifecycle.

## 15. Business Authorization, Approval and Audit Obligations

Restoration must be governed by the authorised Position & Assignment business process.

The restoration decision, target prior state, evidence for restoration, and resulting assignment lineage remain traceable.

## 16. Completion Criteria and Postconditions

- A valid prior assignment state has been established.
- The restored state satisfies current canonical assignment rules.
- Position Assignment is authoritative in the restored state.
- Prior invalid/superseded state remains historically traceable.
- Workforce Engagement and Position Definition ownership remain unchanged.
- Canonical restoration event is published where defined.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material ownership conflict identified.

The exact frozen rollback/restoration lifecycle transition and event vocabulary should be confirmed against the Stage 3/L2-E02 authoritative mapping before Architecture Approval.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Confirm the exact canonical restoration event and state-transition terminology.
