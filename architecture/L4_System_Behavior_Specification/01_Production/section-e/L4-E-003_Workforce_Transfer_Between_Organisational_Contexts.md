# L4-E-003 — Workforce Transfer Between Organisational Contexts

**Scenario ID:** L4-E-003  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-003 — Workforce Transfer Between Organisational Contexts  
**Primary Owner:** People & Workforce  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata

Derived from locked L3-E-003; transfer ownership remains with People & Workforce.

## 2. System Behavior Purpose

Specify observable behavior for transferring an existing workforce engagement between governed organisational contexts.

## 3. System Behavior Outcome

The workforce engagement reflects the authoritative destination organisational context and preserves the transfer history.

## 4. System Behavior Scope and Boundary

### 4.1 Start Boundary
An active workforce engagement is eligible for a governed organisational transfer.

### 4.2 End Boundary
The destination context becomes the authoritative organisational position.

### 4.3 In Scope
- Identify source and destination organisational contexts.
- Validate transfer eligibility and authority.
- Establish the destination engagement position.
- Preserve transfer history.

### 4.4 Out of Scope
- Creation or alteration of organisational master data.
- Unrelated employment settlement.
- Technical transfer mechanisms.

## 5. Ownership and Participating Domains

People & Workforce owns the workforce transfer process. Organisational context remains authoritative in its owning domain.

## 6. Business Actors, Roles and Responsibilities

Use authoritative L2 workforce transfer participants only. No new authority is introduced.

## 7. Preconditions and Entry Criteria

- A transferable workforce engagement exists.
- Source and destination organisational contexts are authoritative.
- Required transfer authority is established.

## 8. Trigger

A governed workforce transfer request is initiated.

## 9. Main System Behavior

1. Identify the affected engagement and organisational contexts.
2. Validate transfer eligibility and applicable authority.
3. Establish the transfer decision.
4. Record the destination organisational position.
5. Preserve the transfer history.

## 10. Alternate System Behaviors

### 10.1 Invalid Destination Context
The transfer is not completed until the destination context is authoritative.

### 10.2 Transfer Not Authorised
The current organisational position remains authoritative.

## 11. Failure, Interruption and Business Recovery

A transfer that cannot satisfy business conditions leaves the current authoritative position unchanged. No technical retry, rollback, queue, or infrastructure behavior is specified.

## 12. Business Rules and Decision Constraints

- Workforce transfer remains People & Workforce owned.
- Organisational context remains source-owned.
- Transfer history remains traceable.

## 13. State and Lifecycle Impact

The workforce engagement lifecycle remains governed by its canonical states; transfer does not introduce a new lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

Only canonical workforce transfer events defined upstream may be published. No unverified event name is invented.

## 15. Business Authorization, Approval and Audit Obligations

Transfer authority follows applicable workforce governance. The decision and history remain auditable.

## 16. Completion Criteria and Postconditions

- Transfer eligibility and authority are established.
- Destination organisational context is authoritative.
- Transfer history is preserved.

## 17. Open Issues, Assumptions and Deferred Questions

Exact state/event vocabulary not exposed by authoritative upstream sources remains subject to verification before lock.

## Production Verification Record

- [x] L3 identity and ownership preserved.
- [x] No implementation architecture introduced.
- [x] No unverified state, event, role, or ownership invented.
- [x] Status retained as Draft.
