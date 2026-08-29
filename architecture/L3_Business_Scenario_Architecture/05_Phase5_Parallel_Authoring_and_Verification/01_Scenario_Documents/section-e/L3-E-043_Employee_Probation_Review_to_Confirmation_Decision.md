# Employee Probation Review to Confirmation Decision

**Scenario ID:** L3-E-043  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; L2-E01 — Workforce & Engagement Workflows  
**Referenced By:** None yet

## 1. Scenario Metadata
This Draft preserves frozen Stage 3 semantics and applies locked upstream architecture without redesigning ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose
Manage the HR probation/confirmation case through review, appraisal capture and approval so that the institution reaches a governed confirmation decision without HR taking ownership of the resulting employment fact.

## 3. Business Outcome
A governed confirmation decision is established for the employee probation case.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An employee probation review case enters the HR workflow.
### 4.2 End Boundary
The required probation review and approval are complete and a confirmation decision is recorded.
### 4.3 In Scope
Probation review scheduling; appraisal capture; review and approval; confirmation decision; HR case outcome recording.
### 4.4 Out of Scope
The authoritative employment probation/confirmation status and any employment lifecycle transition, which remain owned by L2-E01.
### 4.5 Related Scenarios
None identified as necessary to clarify this boundary.

## 5. Ownership and Participating Domains
| Domain | Business responsibility |
|---|---|
| HR / L2-E04 | Owns the probation and confirmation case, review scheduling, appraisal capture, approval and decision record. |
| Workforce & Engagement / L2-E01 | Owns the resulting employment lifecycle fact and any authoritative employment state transition. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities
- HR Administrator / HR case owner — manages the probation and confirmation case.
- Reviewing/approving business authority — performs the applicable review and approval decision.
- Employee / workforce member — subject of the probation review.
- Workforce & Engagement — authoritative owner of any resulting employment lifecycle state.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- An employee/workforce engagement exists and is identifiable from authoritative workforce context.
- A probation/confirmation case exists or is due for review.
- The applicable institutional probation review and approval requirements are available.

## 8. Trigger
A probation review becomes due and the corresponding HR probation/confirmation case enters review.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Establish or advance the probation review case into the required review activity. | HR | Review is initiated. |
| 2 | Conduct the applicable probation review and capture the appraisal outcome. | HR / Reviewing authority | Review evidence and appraisal outcome are recorded. |
| 3 | Route the case for the required confirmation decision. | HR | Case reaches the decision stage. |
| 4 | Record the confirmation decision. | Approving authority / HR | Confirmation decision is established. |
| 5 | Record the HR case outcome without duplicating the authoritative employment fact. | HR | Probation/confirmation case reaches its business outcome. |

## 10. Alternate Business Flows
- If the review does not support confirmation, the case records the applicable decision rather than treating confirmation as achieved.
- If additional review is required, the case remains within the governed HR review process until a business decision is established.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Required review evidence is incomplete | A confirmation decision cannot yet be established. | Complete the required business review activities. | HR |
| Approval does not support confirmation | Confirmation is not established by this case outcome. | Record the decision and follow applicable upstream employment/probation governance. | HR / approving authority |
| A resulting employment transition is required after the decision | HR cannot claim the employment fact has changed. | Hand the applicable decision to L2-E01 for authoritative employment lifecycle handling. | HR / E01 |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- L2-E04 owns probation and confirmation cases as HR workflow processes.
- Employment activation, probation status, confirmation status and other employment facts are owned by L2-E01.
- HR validates, routes and approves the case; it does not own the resulting employment lifecycle state.
- Every approval records actor, timestamp and decision.
- Workflow history is maintained as business history.

## 13. State and Lifecycle Impact
The HR case lifecycle is affected through review and decision. Any authoritative employment probation/confirmation state remains owned by E01. No new confirmation lifecycle is created in L3.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
No exact scenario-specific canonical event name is required to establish the frozen decision outcome. The L2 architecture states that authoritative employment-domain events for employment-state transitions are published by E01.
### 14.2 Synchronous Business Contracts / Hand-offs
Where the confirmation decision results in an employment-state transition, the business decision is handed to Workforce & Engagement for the authoritative employment outcome. This does not transfer ownership.

## 15. Business Authorization, Approval and Audit Obligations
Review evidence, appraisal outcome and approval decision must be traceable in the HR case. Approval records actor, timestamp and decision. Technical authorization mechanisms are outside L3.

## 16. Completion Criteria and Postconditions
- Probation review is completed.
- Appraisal/review evidence is recorded.
- Required approval is recorded.
- Confirmation decision is established as the HR case outcome.
- Any employment-state consequence remains with E01.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
The accessible locked L2-E04 evidence establishes probation and confirmation case ownership but does not expose a dedicated canonical probation/confirmation event name. No event name has been invented.
### 17.2 Assumptions
None.
### 17.3 Deferred Questions
Confirm whether a dedicated probation/confirmation event is defined in the authoritative event register before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft; canonical event detail is not explicitly defined in the accessible L2 evidence.
