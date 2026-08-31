# L4-E-002 — Active Workforce Engagement Modification

**Scenario ID:** L4-E-002  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-002 — Active Workforce Engagement Modification  
**Primary Owner:** People & Workforce  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata

Derived from locked L3-E-002; canonical workforce engagement ownership is preserved.

## 2. System Behavior Purpose

Specify observable behavior for modifying an active workforce engagement under governed conditions.

## 3. System Behavior Outcome

A valid modification becomes the authoritative current engagement position while prior history remains traceable.

## 4. System Behavior Scope and Boundary

### 4.1 Start Boundary
An active workforce engagement requires an authorised business modification.

### 4.2 End Boundary
The valid modified engagement position is recorded as authoritative.

### 4.3 In Scope
- Identify the active engagement.
- Validate the proposed modification against authoritative facts and applicable workforce governance.
- Record the valid modified position.
- Preserve modification history.

### 4.4 Out of Scope
- Upstream correction of source-owned facts.
- Payroll or financial settlement.
- Technical implementation mechanisms.

## 5. Ownership and Participating Domains

People & Workforce owns the workforce engagement lifecycle and modification within this boundary. Source-owned facts remain with their canonical domains.

## 6. Business Actors, Roles and Responsibilities

Use only authoritative L2 workforce participants. No new role or authority is introduced.

## 7. Preconditions and Entry Criteria

- An active workforce engagement exists.
- The requested change is within the engagement boundary.
- Required authority and evidence are available.

## 8. Trigger

A governed request to modify an active workforce engagement is received.

## 9. Main System Behavior

1. Identify the affected engagement.
2. Validate the proposed change.
3. Establish the applicable business decision.
4. Record the modified authoritative position.
5. Preserve prior engagement history.

## 10. Alternate System Behaviors

### 10.1 Invalid Modification
The existing authoritative position remains unchanged until the business issue is resolved.

### 10.2 Upstream Fact Conflict
The modification is not finalised until the authoritative source fact is resolved.

## 11. Failure, Interruption and Business Recovery

A modification that fails business validation does not replace the current authoritative position. No technical retry, rollback, queue, or infrastructure behavior is specified.

## 12. Business Rules and Decision Constraints

- Active engagement remains source-owned by People & Workforce.
- Modifications require applicable business authority.
- Prior history remains traceable.

## 13. State and Lifecycle Impact

The engagement remains within its canonical active lifecycle unless the governed modification changes that lifecycle. No new state is introduced.

## 14. Business Events and Cross-Domain Collaboration

Only verified canonical workforce engagement events may be published. No unverified event name is introduced.

## 15. Business Authorization, Approval and Audit Obligations

Modification authority follows applicable workforce governance. The decision and resulting engagement history remain auditable.

## 16. Completion Criteria and Postconditions

- The modification is business-valid and authorised.
- The current engagement position reflects the accepted change.
- Prior history remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions

Any exact state/event vocabulary not exposed by the authoritative upstream sources remains subject to verification before lock.

## Production Verification Record

- [x] L3 identity and ownership preserved.
- [x] No implementation architecture introduced.
- [x] No unverified state, event, role, or ownership invented.
- [x] Status retained as Draft.
