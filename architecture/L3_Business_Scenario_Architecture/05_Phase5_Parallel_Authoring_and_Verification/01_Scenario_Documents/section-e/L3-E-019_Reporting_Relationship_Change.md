# Reporting Relationship Change

**Scenario ID:** L3-E-019  
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
Govern an authorised change to an existing Reporting Relationship while preserving the canonical Position Assignment ownership, reporting hierarchy integrity, effective dating and historical traceability.

## 3. Business Outcome
The Reporting Relationship reaches the authoritative changed business outcome, with reporting accountability remaining valid, non-circular and historically reconstructable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An existing Reporting Relationship between active Position Assignments is identified for an authorised change.

### 4.2 End Boundary
The changed Reporting Relationship is authoritative for its applicable effective period and `ReportingRelationshipChanged` has been published.

### 4.3 In Scope
- Validate the existing Reporting Relationship.
- Validate the participating active Position Assignments.
- Validate the proposed relationship type/target and hierarchy consequences.
- Apply the governed reporting-relationship change.
- Preserve effective dating and historical lineage.
- Publish the canonical reporting-change fact.

### 4.4 Out of Scope
Technical implementation, Position Assignment lifecycle, Workforce Engagement lifecycle, and creation of unrelated organisational structure.

### 4.5 Related Scenarios
No additional canonical scenario relationship is asserted beyond the dependencies explicitly stated in this document.

## 5. Ownership and Participating Domains
Position & Assignment is the canonical owner of the outcome. Participating domains provide authoritative upstream business context within their own ownership boundaries; participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- The authorised business/organisational administrator initiates the change under applicable institutional governance.
- Position & Assignment owns the Reporting Relationship lifecycle.
- Workforce & Engagement supplies the underlying Workforce Engagement context.
- Organisation/Position Definition supplies authoritative organisational Position context.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- Existing Reporting Relationship exists.
- Participating Position Assignments remain active.
- Proposed relationship satisfies applicable reporting constraints.
- Required organisational governance is satisfied.

## 8. Trigger
An authorised organisational change requires an existing Reporting Relationship to be changed.

## 9. Main Business Flow
1. Identify the existing Reporting Relationship.
2. Validate the reporting and supervising Position Assignments.
3. Validate the proposed change against canonical reporting rules.
4. Confirm that the resulting hierarchy remains non-circular.
5. Apply the governed Reporting Relationship change for its effective period.
6. Preserve the prior relationship history.
7. Publish `ReportingRelationshipChanged`.
8. Stop when the changed relationship is authoritative.

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- A future-effective change remains non-operational until its effective boundary.
- If the proposed relationship would create circular reporting, the change does not become authoritative.
- If either participating Position Assignment is not active, the changed relationship cannot become an active reporting relationship.
- If a proposed primary relationship would create more than one primary relationship, the change does not become authoritative until the conflict is resolved.

## 11. Failure, Interruption and Business Recovery
If the proposed change cannot validly complete, the existing authoritative Reporting Relationship remains unchanged. Historical reporting information is preserved. No technical rollback or retry behaviour is defined.

Only business-level recovery is described. Technical retries, queues, rollback, database behaviour, infrastructure and implementation exception handling are outside L3.

## 12. Business Rules and Decision Constraints
- Every Reporting Relationship connects exactly one Reporting Position Assignment to one Supervising Position Assignment.
- Reporting Relationships reference active Position Assignments only.
- Reporting hierarchies form directed acyclic graphs; circular reporting is prohibited.
- Every active Position Assignment has zero or one Primary Reporting Relationship; additional reporting paths must be explicitly classified.
- Reporting Relationships are effective-dated.
- Reporting is independent of operational authority.

## 13. State and Lifecycle Impact
**Reporting Relationship:** existing relationship → changed relationship under the authoritative reporting lifecycle, effective for its governed period.

The L2 architecture identifies `ReportingRelationshipChanged` as the canonical business event. It does not introduce a new state name for this change scenario.

No new state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
`ReportingRelationshipChanged` — canonical publisher: Reporting Relationship / Position & Assignment.

The event records the completed business fact that the reporting relationship has changed.

### 14.2 Business Contracts and Cross-Domain Hand-offs
Workforce and Organisation provide authoritative context; neither dependency transfers ownership of Reporting Relationships. Downstream domains consume the resulting business fact according to their own responsibilities.

## 15. Business Authorization, Approval and Audit Obligations
Change follows applicable institutional organisational governance. No unsupported fixed approving role or technical permission mechanism is invented. The changed reporting relationship and effective period remain auditable.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Changed Reporting Relationship is authoritative.
- Reporting hierarchy remains valid and non-circular.
- Primary reporting remains deterministic.
- Effective dating is preserved.
- Historical relationship lineage is preserved.
- `ReportingRelationshipChanged` is published.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring classification:** PASS — source-grounded Draft.
