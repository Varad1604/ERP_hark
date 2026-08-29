# Service Jurisdiction Change or Withdrawal

**Scenario ID:** L3-E-021  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows; Organisation/Position Definition architecture; Workforce Engagement architecture  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Stage 3 scenario identity, title, owner, participating domains, Type and Priority are preserved. The scenario applies locked upstream architecture without changing ownership.

## 2. Scenario Purpose
Govern an authorised change or withdrawal of an existing Service Jurisdiction while preserving the underlying Position Assignment, organisational placement and historical service-boundary lineage.

## 3. Business Outcome
The Service Jurisdiction reaches the authoritative changed or withdrawn outcome for its applicable period without altering the canonical Position Assignment.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Service Jurisdiction associated with a Position Assignment requires an authorised change or withdrawal.

### 4.2 End Boundary
The changed/withdrawn Service Jurisdiction is authoritative for the applicable period and the underlying Position Assignment remains valid unless separately changed.

### 4.3 In Scope
- Identify and validate the existing Service Jurisdiction.
- Validate the governing Position Assignment.
- Validate the proposed jurisdiction change or withdrawal.
- Apply the business change.
- Preserve effective dating and historical lineage.
- Leave the underlying organisational appointment unchanged.

### 4.4 Out of Scope
Position Assignment transfer, Position Definition lifecycle, Workforce Engagement lifecycle, and creation of a new organisational appointment.

### 4.5 Related Scenarios
No additional canonical scenario relationship is asserted beyond the dependencies explicitly stated in this document.

## 5. Ownership and Participating Domains
Position & Assignment is the canonical owner of the outcome. Participating domains provide authoritative upstream business context within their own ownership boundaries; participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Position & Assignment owns Service Jurisdiction.
- Workforce & Engagement provides Workforce Engagement context.
- Organisation/Position Definition provides organisational context.
- Authorised institutional governance determines the permitted jurisdiction change.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- Existing Service Jurisdiction exists.
- Governing Position Assignment remains valid.
- Proposed change/withdrawal satisfies applicable organisational policy.
- Required business authorization is complete.

## 8. Trigger
An authorised operational or organisational decision requires the existing Service Jurisdiction to change or cease.

## 9. Main Business Flow
1. Identify the existing Service Jurisdiction.
2. Validate the governing Position Assignment.
3. Validate the proposed new boundary or withdrawal.
4. Confirm that the change does not incorrectly alter organisational placement.
5. Apply the changed/withdrawn jurisdiction for its effective period.
6. Preserve prior jurisdiction history.
7. Stop when the resulting Service Jurisdiction outcome is authoritative.

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- A future-effective change does not alter the current service boundary before its effective date.
- Withdrawal removes the jurisdictional scope but does not itself terminate the Position Assignment.
- If the requested change actually requires a different organisational appointment, the request follows the Position Assignment lifecycle instead.

## 11. Failure, Interruption and Business Recovery
If the jurisdiction change/withdrawal cannot validly complete, the existing authoritative Service Jurisdiction remains unchanged. Historical scope is preserved. No technical retry or rollback is defined.

Only business-level recovery is described. Technical retries, queues, rollback, database behaviour, infrastructure and implementation exception handling are outside L3.

## 12. Business Rules and Decision Constraints
- Service Jurisdiction is an operational service boundary for a Position Assignment.
- Service Jurisdiction does not create or replace Position Assignments.
- Operational scope may change independently of organisational placement.
- Historical information remains reconstructable.
- Position Assignment remains the canonical organisational appointment.

## 13. State and Lifecycle Impact
**Service Jurisdiction:** existing jurisdiction → revised/withdrawn jurisdictional outcome according to the governed effective period.

**Position Assignment:** remains in its existing lifecycle; jurisdiction change does not itself change organisational occupancy.

No new state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
The locked L2 material identifies Service Jurisdiction as a canonical L2-E02 concept/aggregate and defines its purpose and ownership. No separate canonical Service Jurisdiction change/withdrawal event name was found in the locked L2 material read for this batch; none is invented.

### 14.2 Business Contracts and Cross-Domain Hand-offs
Cross-domain participants consume authoritative Position Assignment and Workforce/Organisation context. Ownership remains within Position & Assignment.

## 15. Business Authorization, Approval and Audit Obligations
Applicable institutional governance and effective-date rules apply. No unsupported named approver or technical authorization mechanism is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Service Jurisdiction reaches the requested authoritative changed/withdrawn outcome.
- Underlying Position Assignment remains valid and unchanged by this scenario.
- Historical jurisdiction lineage is preserved.
- Effective period is respected.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
Event publication for jurisdiction change/withdrawal remains subject to the canonical L2 event catalogue; no undefined event is invented.

**Authoring classification:** PASS — source-grounded Draft.
