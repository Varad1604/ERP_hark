# Employee Separation Case to Accepted Exit Handoffs

**Scenario ID:** L3-E-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; L2-E01 — Workforce & Engagement Workflows; L2-E02 — Position & Assignment Workflows; applicable organisational/exit context  
**Referenced By:** None yet

## 1. Scenario Metadata

The canonical identity and frozen Stage 3 semantics are preserved. This scenario applies the locked upstream architecture and does not redefine ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose

Coordinate an employee separation through the HR separation case, including exit approval, clearance and final verification, and complete the accepted exit handoff to the authoritative workforce owner.

## 3. Business Outcome

The separation case reaches an accepted exit outcome and the cleared separation is handed to Workforce & Engagement for authoritative employment closure.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A resignation/separation request enters the HR separation workflow.

### 4.2 End Boundary

Exit clearance and final verification are complete and the accepted separation handoff to Workforce & Engagement is complete.

### 4.3 In Scope

Separation request intake; exit approval; clearance; final verification; accepted handoff; HR case closure.

### 4.4 Out of Scope

HR does not own the resulting employment lifecycle fact, employee master/profile data, organisational structure, position assignment, or other specialised domain state.

### 4.5 Related Scenarios

None specified in the authoritative sources reviewed for this scenario.

## 5. Ownership and Participating Domains

| Domain | Business responsibility |
|---|---|
| HR / L2-E04 | Owns the HR employment-change/separation/compliance case, workflow execution, approvals, decision record and case outcome. |
| Workforce & Engagement / L2-E01 | Owns authoritative employment lifecycle state and publishes authoritative employment events after successful workforce state transitions. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

- Employee — submits or is subject to the separation case.
- HR Administrator / HR case owner — manages the separation workflow and clearance.
- Approving business authority — participates in exit approval.
- Workforce & Engagement — owns authoritative employment closure.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- The employee/workforce engagement is identifiable.
- A separation/resignation request exists.
- Required exit approval and clearance responsibilities can be determined.

## 8. Trigger

A resignation request is submitted.

## 9. Main Business Flow

| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Establish the separation case from the resignation/separation request. | HR | Separation case is initiated. |
| 2 | Obtain the applicable exit approval decision. | HR / Approving authority | Exit decision is established. |
| 3 | Complete the required exit clearance activities and record clearance outcomes. | HR / participating business authorities | Clearance outcome is established. |
| 4 | Perform final verification of the separation case. | HR | Case is ready for accepted exit handoff. |
| 5 | Hand the cleared separation to Workforce & Engagement for authoritative employment closure. | HR / E01 | Exit handoff is accepted by the canonical workforce owner. |
| 6 | Record the HR separation case outcome. | HR | HR separation case reaches its accepted exit handoff outcome. |

The flow stops when the singular institutional outcome and end boundary are reached.

## 10. Alternate Business Flows

- If exit approval is not granted, the separation does not proceed to accepted exit handoff and the HR case records the decision.
- If clearance is incomplete, the case does not reach final verification until the applicable business clearance outcome is established.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Exit approval is rejected | No accepted separation exists. | Record the decision and resolve the HR case according to its lifecycle. | HR |
| Required clearance is incomplete | Separation cannot yet reach final verification. | Complete the applicable business clearance activities and re-evaluate the case. | HR / relevant business authority |
| Workforce closure is not yet completed | HR case cannot claim completed employment closure. | Preserve the accepted handoff and await the authoritative workforce outcome. | HR / E01 |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- E04 owns the separation case, resignation intake, notice tracking, exit clearance and case completion state.
- Employment closure as an employment fact is owned by E01.
- The authoritative EmployeeSeparated-class employment event is published by E01, not HR.
- E04 does not own final settlement calculation.
- Every approval records actor, timestamp and decision.
- External domain updates occur through governed contracts/events.

These rules remain owned by authoritative upstream architecture.

## 13. State and Lifecycle Impact

HR separation case follows the applicable HR case lifecycle. E01 owns the employment lifecycle transition from separation processing to authoritative employment closure. No HR-owned employment closure state is introduced.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

**EmployeeSeparated** is the canonical primary event for the Separation workflow and is authoritative to the workforce employment outcome. E04 may publish its HR case event such as **HRSeparationCaseClosed**, but must not represent that event as the authoritative employment separation event.

### 14.2 Synchronous Business Contracts / Hand-offs

The accepted, cleared separation is handed from HR to Workforce & Engagement so E01 can close the Workforce Engagement. This is a business handoff and does not transfer ownership.

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Exit approval and clearance outcomes are recorded as business evidence. HR maintains the separation case history. The authoritative employment closure and EmployeeSeparated event remain with E01. Technical access-control mechanisms are outside L3.

## 16. Completion Criteria and Postconditions

- Exit approval is recorded.
- Required clearance and final verification are complete.
- The accepted separation is handed to E01.
- HR records the separation case outcome without duplicating employment closure.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified that materially affect ownership, outcome, lifecycle, event publisher, authority or scenario boundary.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
