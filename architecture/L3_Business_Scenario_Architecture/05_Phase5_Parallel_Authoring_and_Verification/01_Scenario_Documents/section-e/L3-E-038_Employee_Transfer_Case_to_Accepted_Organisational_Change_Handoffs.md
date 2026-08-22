# Employee Transfer Case to Accepted Organisational-Change Handoffs

**Scenario ID:** L3-E-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; L2-E01 — Workforce & Engagement Workflows; L2-E02 — Position & Assignment Workflows; applicable organisational context  
**Referenced By:** None yet.

## 1. Scenario Metadata

The canonical identity and frozen Stage 3 semantics are preserved exactly. This document applies the locked upstream architecture and does not redefine ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose

Coordinate an employee transfer through the HR case workflow and complete the accepted organisational-change handoff while preserving the authoritative ownership of workforce engagement and position assignment.

## 3. Business Outcome

The transfer case reaches an accepted organisational-change outcome and the approved change is handed to the authoritative workforce/assignment domains for execution, without HR duplicating the resulting organisational or workforce state.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An employee transfer request enters the HR transfer workflow.

### 4.2 End Boundary

The transfer case has an accepted organisational-change outcome and the required handoff to the authoritative workforce/assignment domains is complete.

### 4.3 In Scope

Transfer request; manager recommendation; HR validation; approval/decision; accepted handoff; recording of case outcome.

### 4.4 Out of Scope

The scenario does not own the resulting employment fact, employee master data, organisational structure, position assignment, document storage, or any other specialised state owned by another canonical domain.

### 4.5 Related Scenarios

None yet.

## 5. Ownership and Participating Domains

| Domain | Business responsibility |
|---|---|
| HR (L2-E04) | Owns the HR case, approval process, decision record and accepted handoff outcome. |
| Workforce & Engagement (L2-E01) | Owns authoritative workforce/employment state and performs the employment lifecycle transition where applicable. |
| Position & Assignment (L2-E02) | Owns position/assignment state when the approved organisational change affects assignment placement. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

- Employee.
- Manager.
- HR Administrator / HR case owner.
- Workforce & Engagement domain.
- Position & Assignment domain where organisational placement changes.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- A transfer request has been raised.
- Employee and current organisational context are available from authoritative sources.
- Applicable transfer policy and approval requirements can be evaluated.
- Required business authorities can participate.

## 8. Trigger

An employee transfer request is submitted into the HR transfer workflow.

## 9. Main Business Flow

| Step | Business behaviour | Responsible owner / participant | Business result |
|---|---|---|---|
| 1 | Establish the transfer HR case from the employee request/recommendation. | HR | Transfer case is established. |
| 2 | Validate the request against applicable policy and organisational context. | HR | Transfer case is ready for decision. |
| 3 | Obtain the required business decision and record it. | HR / Manager / approving authority | Transfer decision is established. |
| 4 | For an approved transfer, initiate the accepted organisational-change handoff to the authoritative workforce domain. | HR / E01 | Workforce change is placed under E01 ownership. |
| 5 | Where organisational placement changes, the workforce domain coordinates with L2-E02, which owns Position Assignment and related assignment state. | E01 / E02 | Assignment change is handled by its canonical owner. |
| 6 | Record the accepted handoff and complete the HR case without owning the resulting employment or assignment state. | HR | Transfer case reaches its accepted handoff outcome. |

The flow stops when the institutional business outcome and end boundary are reached.

## 10. Alternate Business Flows

- If the transfer is not approved, no accepted organisational-change handoff is created; HR records the decision.
- If the requested organisational placement cannot be established, the affected assignment decision remains with the authoritative assignment owner and the HR case records the applicable business outcome.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Policy validation fails | Transfer cannot proceed as requested. | Record the business decision and resolve the HR case according to its lifecycle. | HR |
| Approval is rejected | No accepted transfer outcome exists. | Record rejection and close the HR case according to governance. | HR |
| Assignment change cannot be established | The organisational transfer is not complete. | Preserve the approved case outcome while the authoritative assignment process determines the valid business outcome. | E01 / E02 |

No technical retry, queue, rollback, infrastructure failure or implementation recovery is defined.

## 12. Business Rules and Decision Constraints

- HR owns the transfer process/case, not the resulting employment or assignment facts.
- L2-E01 is authoritative for Workforce Engagement.
- L2-E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship, Assignment History and derived operational authority.
- E01 may coordinate transfer but E02 performs and publishes assignment-state changes.
- External domain data is not directly modified by HR.
- Business approvals record actor, timestamp and decision.

These rules remain owned by their authoritative upstream domains.

## 13. State and Lifecycle Impact

HR Case follows the applicable HR case lifecycle.

Workforce Engagement lifecycle remains owned by E01.

Position/assignment lifecycle remains owned by E02. No transfer lifecycle state is invented inside HR.

No new state or transition is introduced by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

The transfer scenario must consume the authoritative business outcomes from E01/E02 rather than invent a new HR-owned transfer event.

The locked E04 workflow explicitly describes transfer completion as a cross-domain outcome and the E01 ownership clarification establishes E02 as the owner of assignment-state changes.

### 14.2 Synchronous Business Contracts / Hand-offs

The accepted handoff proceeds from HR to E01. Where organisational placement changes, E01 coordinates with E02. This is a business handoff, not a transfer of ownership.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Transfer approval and business decision evidence are recorded in the HR case. HR maintains process history. Workforce and assignment state changes remain auditable in their canonical owners. Technical IAM/RBAC is outside L3.

## 16. Completion Criteria and Postconditions

- Transfer decision is recorded.
- Approved transfer is handed to the canonical workforce owner.
- Required assignment change is handled by E02 where applicable.
- HR case records the accepted organisational-change handoff and does not duplicate external state.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified that materially block this scenario after applying the locked ownership clarification in L2-E01 and the HR workflow boundaries in L2-E04.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
