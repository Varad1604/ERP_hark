# Service Jurisdiction Assignment to Workforce Authority

**Scenario ID:** L3-E-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows; Organisation/Position Definition architecture; Workforce Engagement architecture  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Stage 3 scenario identity, title, owner, participating domains, Type and Priority are preserved. The scenario applies locked upstream architecture without changing ownership.

## 2. Scenario Purpose
Establish a Service Jurisdiction for an existing Position Assignment so that the assignment may operate across its authorised service boundary without creating another organisational appointment.

## 3. Business Outcome
The Position Assignment has an authoritative Service Jurisdiction covering the approved operational service boundary, while the underlying organisational appointment remains a single Position Assignment.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An active Position Assignment requires an authorised operational service boundary.

### 4.2 End Boundary
The approved Service Jurisdiction is authoritative for the Position Assignment and applicable period.

### 4.3 In Scope
- Validate the active Position Assignment.
- Validate the service boundary/jurisdiction.
- Establish the Service Jurisdiction.
- Preserve the single underlying organisational appointment.
- Preserve effective dating and historical traceability.

### 4.4 Out of Scope
Creating an additional Position Assignment, changing Position Definition ownership, or changing Workforce Engagement lifecycle.

### 4.5 Related Scenarios
No additional canonical scenario relationship is asserted beyond the dependencies explicitly stated in this document.

## 5. Ownership and Participating Domains
Position & Assignment is the canonical owner of the outcome. Participating domains provide authoritative upstream business context within their own ownership boundaries; participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Position & Assignment owns Service Jurisdiction.
- Workforce & Engagement supplies the Workforce Engagement context.
- Organisation/Position Definition supplies authoritative organisational context.
- The authorised institutional authority governs the operational service boundary.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- Position Assignment is active.
- Proposed jurisdiction is valid under applicable organisational policy.
- The jurisdiction can be associated with the existing Position Assignment.
- Required business authorization is complete.

## 8. Trigger
An authorised operational requirement requires an existing Position Assignment to serve an additional or defined service jurisdiction.

## 9. Main Business Flow
1. Identify the active Position Assignment.
2. Validate the proposed Service Jurisdiction and its organisational applicability.
3. Confirm that the jurisdiction extends operational scope rather than creating a new organisational appointment.
4. Establish the Service Jurisdiction against the Position Assignment.
5. Preserve effective dates and historical lineage.
6. Make the resulting jurisdiction authoritative for the applicable period.

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- A future-effective jurisdiction becomes applicable only at its effective boundary.
- Multiple jurisdictions may be associated with one Position Assignment where permitted.
- A jurisdiction request that would require a separate organisational appointment is outside this scenario and must follow the Position Assignment lifecycle instead.

## 11. Failure, Interruption and Business Recovery
If the proposed Service Jurisdiction cannot validly be established, the existing Position Assignment remains authoritative without the new jurisdiction. Business correction follows the applicable governance path.

Only business-level recovery is described. Technical retries, queues, rollback, database behaviour, infrastructure and implementation exception handling are outside L3.

## 12. Business Rules and Decision Constraints
- Service Jurisdiction defines the organisational scope served by a Position Assignment.
- One Position Assignment may legitimately operate across multiple jurisdictions.
- Service Jurisdiction shall not create additional Position Assignments.
- Service Jurisdiction expands operational scope without altering organisational placement.
- Service Jurisdiction is owned by Position & Assignment.

## 13. State and Lifecycle Impact
**Service Jurisdiction:** established as an operational-scope construct associated with an existing Position Assignment.

The underlying Position Assignment remains unchanged as the canonical organisational appointment.

No new state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
The locked L2 material identifies Service Jurisdiction as a canonical L2-E02 concept/aggregate and states that it extends operational scope without altering organisational placement.

No separate canonical Service Jurisdiction event name was found in the locked L2 material read for this batch; therefore no event name is invented here.

### 14.2 Business Contracts and Cross-Domain Hand-offs
Workforce Engagement and Position Definition are consumed as authoritative context. The Service Jurisdiction dependency does not transfer ownership of either upstream concept.

## 15. Business Authorization, Approval and Audit Obligations
Applicable institutional governance determines the authorised service boundary. No unsupported named approver is invented.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Service Jurisdiction is established for the existing Position Assignment.
- Operational service scope is authoritative.
- No additional Position Assignment is created.
- Organisational placement remains unchanged.
- Effective/historical traceability is preserved.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
No scenario-specific deferred question; event publication remains governed by the canonical L2 event catalogue if one is subsequently defined.

**Authoring classification:** PASS — source-grounded Draft.
