# L4-E-009 — Workforce Separation and Exit

**Scenario ID:** L4-E-009  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-009 — Workforce Separation and Exit  
**Primary Owner:** People & Workforce  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata

Derived from locked L3-E-009 and preserves workforce separation ownership.

## 2. System Behavior Purpose

Specify observable behavior for governed workforce separation and completion of the workforce engagement exit position.

## 3. System Behavior Outcome

The workforce engagement reaches its canonical separation/exit position with required business history preserved.

## 4. System Behavior Scope and Boundary

### 4.1 Start Boundary
A governed workforce separation condition or decision is established.

### 4.2 End Boundary
The engagement reaches its authoritative separation/exit position.

### 4.3 In Scope
- Identify the workforce engagement.
- Validate separation conditions and authority.
- Establish the separation decision.
- Record the resulting exit position and history.

### 4.4 Out of Scope
- Payroll settlement execution.
- Technical deprovisioning.
- Infrastructure or implementation mechanisms.

## 5. Ownership and Participating Domains

People & Workforce owns workforce separation and exit within this boundary. Other domains retain ownership of their downstream business processes and authoritative facts.

## 6. Business Actors, Roles and Responsibilities

Use authoritative L2 workforce separation participants only. No new role or authority is introduced.

## 7. Preconditions and Entry Criteria

- A workforce engagement exists.
- Applicable separation conditions are established.
- Required authority and evidence are available.

## 8. Trigger

A governed workforce separation decision or qualifying separation condition occurs.

## 9. Main System Behavior

1. Identify the affected engagement.
2. Validate separation conditions and authority.
3. Establish the separation decision.
4. Record the canonical exit position.
5. Preserve separation history.

## 10. Alternate System Behaviors

### 10.1 Separation Not Authorised
The active engagement position remains authoritative.

### 10.2 Separation Condition Disputed
The current governed position remains until the business issue is resolved.

## 11. Failure, Interruption and Business Recovery

A separation that cannot satisfy business conditions does not replace the current authoritative engagement position. No technical retry, rollback, queue, or infrastructure behavior is specified.

## 12. Business Rules and Decision Constraints

- Separation ownership remains with People & Workforce.
- Required authority must be established before the exit position is made authoritative.
- Separation history remains traceable.

## 13. State and Lifecycle Impact

The engagement reaches only canonical separation/exit lifecycle positions defined upstream. No new state is invented.

## 14. Business Events and Cross-Domain Collaboration

Only verified canonical workforce separation events may be published. No unverified event name is introduced.

## 15. Business Authorization, Approval and Audit Obligations

Separation decisions follow workforce governance and remain auditable. Downstream technical deprovisioning is outside this specification.

## 16. Completion Criteria and Postconditions

- Separation authority and conditions are established.
- The canonical exit position is recorded.
- Separation history remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

Exact upstream lifecycle/event vocabulary not exposed by authoritative sources remains subject to verification before lock.

## Production Verification Record

- [x] L3 identity and ownership preserved.
- [x] No implementation architecture introduced.
- [x] No unverified state, event, role, or ownership invented.
- [x] Status retained as Draft.
