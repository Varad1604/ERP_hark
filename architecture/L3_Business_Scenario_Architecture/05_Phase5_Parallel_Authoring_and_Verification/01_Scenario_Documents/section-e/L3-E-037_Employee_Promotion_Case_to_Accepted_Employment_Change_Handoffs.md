# Employee Promotion Case to Accepted Employment-Change Handoffs

**Scenario ID:** L3-E-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; L2-E01 — Workforce & Engagement Workflows; applicable organisational context  
**Referenced By:** None yet.

## 1. Scenario Metadata

The canonical identity and frozen Stage 3 semantics are preserved exactly. This document applies the locked upstream architecture and does not redefine ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose

Manage an employee promotion as an HR employment-change case through eligibility review and approval, then complete the governed handoff so that the accepted employment change can be performed by the canonical workforce owner.

## 3. Business Outcome

The promotion case reaches an accepted employment-change outcome and the approved change is handed to the authoritative workforce lifecycle owner without HR taking ownership of the resulting employment fact.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A promotion request is raised and enters the HR promotion workflow.

### 4.2 End Boundary

The promotion case has an accepted decision and the approved employment-change handoff has been completed to the authoritative workforce domain.

### 4.3 In Scope

Promotion request intake; eligibility review; approval chain; promotion decision; accepted handoff to the workforce lifecycle owner; recording of the HR case outcome.

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

- Employee / requester where supported by the HR process.
- HR Administrator / HR case owner.
- Approving business authority participating in the approval chain.
- Workforce & Engagement domain for the authoritative employment transition.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- A promotion request exists.
- The employee can be identified from authoritative workforce/employee context.
- The HR case can be evaluated against applicable promotion eligibility and policy constraints.
- Required approval authority is available.

## 8. Trigger

A promotion request is submitted into the HR promotion workflow.

## 9. Main Business Flow

| Step | Business behaviour | Responsible owner / participant | Business result |
|---|---|---|---|
| 1 | Create and classify the promotion employment-change case. | HR | Promotion case is established. |
| 2 | Review eligibility against the applicable business requirements. | HR | Eligibility review is completed. |
| 3 | Route the case through the applicable approval chain and record the business decisions. | HR / Approving authority | Promotion decision is established. |
| 4 | Where approved, accept the employment-change outcome for handoff. | HR | Approved promotion is ready for authoritative workforce execution. |
| 5 | Hand the accepted employment change to Workforce & Engagement, which remains authoritative for the employment lifecycle transition. | HR / E01 | The approved employment-change handoff is accepted by the canonical workforce owner. |
| 6 | Record the HR case outcome without duplicating the resulting employment state. | HR | Promotion case reaches its accepted handoff outcome. |

The flow stops when the institutional business outcome and end boundary are reached.

## 10. Alternate Business Flows

- If eligibility is not satisfied, the promotion decision does not proceed as an approved promotion; the HR case records the applicable decision.
- If approval is not granted, the HR case records the rejection and does not perform an employment transition.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Eligibility requirements are not met | Promotion cannot be accepted. | Record the eligibility outcome and resolve the case according to HR case governance. | HR |
| Approval is rejected | No approved employment change exists. | Record the rejection and close the HR case according to its lifecycle. | HR |
| Approved change cannot yet be accepted by the workforce owner | The HR case cannot truthfully record completed employment change. | Preserve the approved case outcome and complete the handoff when the authoritative workforce process can accept it. | HR / E01 |

No technical retry, queue, rollback, infrastructure failure or implementation recovery is defined.

## 12. Business Rules and Decision Constraints

- HR owns the promotion workflow and case.
- The Workforce & Engagement domain is the authoritative owner of workforce lifecycle information.
- Business approvals, authorization rules and workflow routing are governed through the enterprise policy/workflow architecture.
- HR does not directly modify workforce or employee master state.
- Approved employment changes are performed by the authoritative workforce owner.

These rules remain owned by their authoritative upstream domains.

## 13. State and Lifecycle Impact

HR Case: Request Created → Case Classification → Assign HR Owner → Investigation/Eligibility Review → Resolution Decision → Case Closed, as applicable.

The resulting workforce employment state is owned by E01; HR does not create a local promotion lifecycle.

No new state or transition is introduced by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

The locked E04 workflow explicitly identifies **PromotionApproved Event** as the promotion workflow event.

The authoritative workforce domain publishes business facts after successful workforce state transitions. HR must not substitute its approval event for an authoritative workforce-state event.

### 14.2 Synchronous Business Contracts / Hand-offs

The material handoff is the accepted promotion case to L2-E01 for execution of the authoritative employment change. The handoff does not transfer workforce ownership to HR.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Every approval records actor, timestamp and decision. HR maintains immutable business process history. The promotion decision must be traceable to the applicable eligibility review and approval decision. Technical permission enforcement is outside L3.

## 16. Completion Criteria and Postconditions

- Promotion eligibility has been evaluated.
- Required approval decision is recorded.
- If approved, the employment-change handoff is accepted by the canonical workforce owner.
- HR records the case outcome without duplicating employment state.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified that materially block this scenario after applying the locked ownership clarification in L2-E01 and the HR workflow boundaries in L2-E04.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
