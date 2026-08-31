# L4-E-006 — Workforce Leave Lifecycle

**Scenario ID:** L4-E-006  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-006 — Workforce Leave Lifecycle  
**Primary Owner:** People & Workforce  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata

Derived from locked L3-E-006 and preserves workforce leave ownership.

## 2. System Behavior Purpose

Specify observable behavior for governed workforce leave request, decision, modification, and closure.

## 3. System Behavior Outcome

The leave position is established according to applicable workforce policy and remains traceable through its lifecycle.

## 4. System Behavior Scope and Boundary

### 4.1 Start Boundary
A workforce leave requirement is submitted or otherwise becomes applicable.

### 4.2 End Boundary
The leave lifecycle reaches its applicable canonical position.

### 4.3 In Scope
- Identify the workforce engagement and leave context.
- Validate leave eligibility and applicable policy.
- Establish the governed leave decision.
- Record leave lifecycle changes and history.

### 4.4 Out of Scope
- Payroll calculation or settlement.
- Technical attendance integrations.
- Infrastructure or implementation mechanisms.

## 5. Ownership and Participating Domains

People & Workforce owns the workforce leave lifecycle within this boundary. Other domains retain ownership of their authoritative facts.

## 6. Business Actors, Roles and Responsibilities

Use authoritative L2 leave workflow participants only. No new role or authority is introduced.

## 7. Preconditions and Entry Criteria

- A valid workforce engagement exists.
- Applicable leave policy and eligibility information are available.
- Required business authority is established.

## 8. Trigger

A governed leave request or leave lifecycle condition occurs.

## 9. Main System Behavior

1. Identify the affected workforce engagement.
2. Validate the leave request against applicable policy and authoritative facts.
3. Establish the governed leave decision.
4. Record the resulting leave position.
5. Preserve leave history.

## 10. Alternate System Behaviors

### 10.1 Ineligible Leave
The request is not accepted into the applicable approved position and the existing workforce position remains authoritative.

### 10.2 Leave Modification or Cancellation
The system validates the change and updates the leave position when governed conditions are satisfied.

## 11. Failure, Interruption and Business Recovery

Business-invalid leave decisions do not replace the current authoritative position. No technical retry, rollback, queue, or infrastructure behavior is specified.

## 12. Business Rules and Decision Constraints

- Leave lifecycle ownership remains with People & Workforce.
- Leave decisions follow applicable workforce policy.
- Leave history remains traceable.

## 13. State and Lifecycle Impact

Only canonical leave lifecycle positions are used. No new state vocabulary is introduced.

## 14. Business Events and Cross-Domain Collaboration

Only verified canonical leave events may be published. No unverified event name is invented.

## 15. Business Authorization, Approval and Audit Obligations

Leave authorization follows applicable workforce governance. Leave decisions and history remain auditable.

## 16. Completion Criteria and Postconditions

- Leave eligibility and authority are established.
- The applicable leave position is recorded.
- Leave history remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

Exact state/event vocabulary not exposed by authoritative upstream sources remains subject to verification before lock.

## Production Verification Record

- [x] L3 identity and ownership preserved.
- [x] No implementation architecture introduced.
- [x] No unverified state, event, role, or ownership invented.
- [x] Status retained as Draft.
