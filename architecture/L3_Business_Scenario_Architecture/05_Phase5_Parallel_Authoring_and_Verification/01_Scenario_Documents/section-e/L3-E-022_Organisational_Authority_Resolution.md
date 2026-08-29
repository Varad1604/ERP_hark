# Organisational Authority Resolution

**Scenario ID:** L3-E-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20

## 1. Scenario Metadata
Frozen Stage 3 identity, title, owner, participating domains, Scenario Type and Priority are preserved. This L3 scenario applies the locked upstream architecture and does not redesign it.

## 2. Scenario Purpose
Resolve the authoritative organisational authority for an institutional business action from the canonical Position Assignment context.

## 3. Business Outcome
The applicable organisational authority is deterministically resolved from the authoritative Position Assignment context, without deriving authority directly from a person.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An institutional business action requires organisational authority to be resolved.

### 4.2 End Boundary
The applicable organisational authority has been resolved from the authoritative Position Assignment context and is available to the consuming business capability.

### 4.3 In Scope
Position-based authority resolution; active Position Assignment context; organisational placement and reporting context; support for concurrent and acting assignments, delegated authority, future-effective reporting structures and historical reconstruction where applicable.

### 4.4 Out of Scope
Technical implementation, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ implementation concerns are outside this L3 scenario. Upstream lifecycle ownership remains with its canonical owner.

### 4.5 Related Scenarios
Only the upstream and downstream relationships explicitly stated by the authoritative architecture are considered related; no new canonical scenario is created.

## 5. Ownership and Participating Domains
**Canonical Owner:** Position & Assignment.

The Position & Assignment domain owns the relevant Position Assignment / organisational authority / occupancy lifecycle. Organisation (B) and Workforce & Engagement (E) provide authoritative business context within their own ownership boundaries. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Institutional business actor requiring authority resolution.
- Position & Assignment owns organisational authority resolution.
- Organisation supplies organisational/Position Definition context.
- Workforce & Engagement supplies Workforce Engagement context.

No unsupported technical role or permission model is introduced.

## 7. Preconditions and Entry Criteria
Applicable Position Assignment context exists and the business action requires organisational authority.

## 8. Trigger
An institutional business action requires a determination of the authoritative organisational position/authority.

## 9. Main Business Flow
1. Identify the business action requiring organisational authority.
2. Resolve the relevant active Position Assignment context.
3. Apply the canonical position-based authority rules.
4. Account for applicable concurrent, acting or delegated authority and effective organisational context.
5. Resolve the authoritative organisational authority.
6. Make the resolution available to the consuming business capability.
7. Stop when the authority outcome is resolved.

The flow stops when the institutional business outcome and end boundary are reached.

## 10. Alternate Business Flows
- Where concurrent assignments exist, authority resolution supports concurrent assignments.
- Acting appointments are supported.
- Delegated authority is supported.
- Future-effective reporting structures are respected for their applicable period.
- Historical organisational authority can be reconstructed for historical business context.

## 11. Failure, Interruption and Business Recovery
If authority cannot be resolved deterministically from authoritative Position Assignment context, no ambiguous organisational authority becomes operational. The business action remains subject to the applicable governance path until a valid authority outcome exists.

Recovery is expressed only as a business outcome; no technical retry, queue, rollback or infrastructure behaviour is defined.

## 12. Business Rules and Decision Constraints
- **ARM-001:** Authority SHALL be Position-based.
- **ARM-002:** Authority SHALL never be Person-based.
- **ARM-003:** Authority SHALL support concurrent assignments.
- **ARM-004:** Authority SHALL support acting appointments.
- **ARM-005:** Authority SHALL support delegated authority.
- **ARM-006:** Authority SHALL support future effective reporting structures.
- **ARM-007:** Historical reporting structures SHALL remain reconstructable.

## 13. State and Lifecycle Impact
Organisational Authority is an L2-E02 canonical concept representing runtime authority resolution. The resolution does not create or transfer ownership of Position Assignment, Workforce Engagement or Position Definition.

No new state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events
No new event name is invented for this scenario. The locked L2 material states that Position & Assignment publishes authoritative organisational assignment information and that downstream consumers use canonical contracts/events; a distinct OrganisationalAuthorityResolved event was not identified in the authoritative material reviewed.

### 14.2 Business Contracts / Hand-offs
Consumers obtain the resolved authority through the Position & Assignment business collaboration boundary. Organisation and Workforce & Engagement remain authoritative for their respective upstream concepts.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Authority is resolved according to the locked position-based governance model. No unsupported named approver is invented. The resulting authority decision remains auditable.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Correct organisational authority is resolved.
- Resolution is position-based.
- No person-based authority is introduced.
- Applicable concurrent, acting and delegated authority is respected.
- Historical/future-effective context is preserved where applicable.

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
