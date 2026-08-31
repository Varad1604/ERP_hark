# L4-E-005 — Workforce Deputation Lifecycle

**Scenario ID:** L4-E-005  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-005 — Workforce Deputation Lifecycle  
**Primary Owner:** People & Workforce  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata

Derived from locked L3-E-005 and preserves canonical workforce deputation ownership.

## 2. System Behavior Purpose

Specify observable behavior for establishing, governing, modifying, and concluding a workforce deputation lifecycle.

## 3. System Behavior Outcome

A governed deputation reaches its applicable authoritative lifecycle position with continuity of workforce history.

## 4. System Behavior Scope and Boundary

### 4.1 Start Boundary
An eligible workforce engagement requires a governed deputation arrangement.

### 4.2 End Boundary
The deputation reaches its applicable canonical lifecycle position or concludes under governed conditions.

### 4.3 In Scope
- Establish deputation eligibility and authority.
- Record the deputation arrangement.
- Apply governed lifecycle changes.
- Preserve deputation history.

### 4.4 Out of Scope
- External organisation master-data maintenance.
- Payroll settlement.
- Technical implementation details.

## 5. Ownership and Participating Domains

People & Workforce owns the deputation lifecycle. Organisational contexts retain ownership of their authoritative context information.

## 6. Business Actors, Roles and Responsibilities

Use authoritative L2 workforce deputation participants. No new role or authority is introduced.

## 7. Preconditions and Entry Criteria

- A valid workforce engagement exists.
- Deputation conditions and applicable organisational contexts are established.
- Required authority is available.

## 8. Trigger

A governed deputation request or lifecycle condition occurs.

## 9. Main System Behavior

1. Identify the affected workforce engagement.
2. Validate deputation conditions and authority.
3. Establish or update the deputation position.
4. Record the resulting lifecycle position.
5. Preserve deputation history.

## 10. Alternate System Behaviors

### 10.1 Deputation Conditions Not Met
The existing workforce position remains authoritative.

### 10.2 Deputation Conclusion
The deputation concludes according to its governed conditions and the workforce engagement returns to its applicable authoritative position.

## 11. Failure, Interruption and Business Recovery

Unresolved business conditions prevent the affected deputation lifecycle change. No technical retry, rollback, queue, or infrastructure behavior is specified.

## 12. Business Rules and Decision Constraints

- Deputation lifecycle ownership remains with People & Workforce.
- Organisational context remains source-owned.
- Deputation history remains traceable.

## 13. State and Lifecycle Impact

Only canonical deputation lifecycle positions are used. No new state vocabulary is introduced.

## 14. Business Events and Cross-Domain Collaboration

Only verified canonical workforce/deputation events may be published. No unverified event name is invented.

## 15. Business Authorization, Approval and Audit Obligations

Deputation authority follows applicable workforce governance. Deputation decisions and history remain auditable.

## 16. Completion Criteria and Postconditions

- Applicable deputation conditions and authority are established.
- The authoritative deputation lifecycle position is recorded.
- History remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

Exact upstream state/event vocabulary not exposed by authoritative sources remains subject to verification before lock.

## Production Verification Record

- [x] L3 identity and ownership preserved.
- [x] No implementation architecture introduced.
- [x] No unverified state, event, role, or ownership invented.
- [x] Status retained as Draft.
