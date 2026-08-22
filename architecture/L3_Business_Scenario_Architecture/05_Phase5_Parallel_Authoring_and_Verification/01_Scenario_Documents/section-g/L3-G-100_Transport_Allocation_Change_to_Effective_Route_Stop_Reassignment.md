# Transport Allocation Change to Effective Route/Stop Reassignment

**Scenario ID:** L3-G-100  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose
This scenario governs an authorised change to an existing Transport Allocation so its effective route and/or stop assignment is replaced with a new operational relationship. G04 explicitly identifies Route Change and Stop Change as allocation changes.

## 3. Business Outcome
An authorised allocation has an effective revised route/stop relationship and its previous relationship remains historically traceable.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An existing transport allocation requires a route or stop change.
### 4.2 End Boundary
The revised route/stop relationship is effective on the allocation.
### 4.3 In Scope
- Identify current allocation.
- Establish requested route/stop change.
- Validate availability and configured allocation constraints.
- Review/approve.
- Apply effective reassignment.
- Preserve change history.
- Publish the canonical update event.
### 4.4 Out of Scope
- Vehicle ownership or maintenance execution.
- Student/workforce lifecycle ownership.
- Financial accounting.
- Notification delivery mechanics.
- APIs, databases, services, queues, UI, deployment or infrastructure.
- Separate downstream transport-pass, trip, financial or communication outcomes.

## 5. Ownership and Participating Domains
Transport Management (G04) is the canonical owner of the Transport Allocation outcome. Participation does not transfer ownership.

G04 defines Transport Allocation as owning Student Allocation, Staff Allocation, Guest Allocation, Allocation Status and Allocation History. Student information remains externally owned by Section C; workforce information remains externally owned by Section E. Finance consumes transport allocation but does not modify transport operations. Parent Communication consumes transport information. None of these relationships transfers allocation ownership.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Administrator | Manages allocation changes. |
| Transport Manager | Typical authority for transport-change approval. |
| Student / Staff representative | May initiate or receive the business change where applicable. |

## 7. Preconditions and Entry Criteria
- An existing allocation exists.
- Current route/stop relationship is known.
- Requested route/stop change is identifiable.
- Target route is active and target stop is operationally available.

## 8. Trigger
An authorised transport allocation route/stop change is requested.

## 9. Main Business Flow
1. Identify the active allocation and current route/stop.
2. Establish the requested reassignment.
3. Validate the target route and stop.
4. Assess the impact on the existing allocation.
5. Complete required review/approval.
6. Apply the effective route/stop reassignment.
7. Preserve previous and new allocation history.
8. Publish `TransportAllocationUpdated`.
9. Make the changed allocation fact available to participating consumers.

## 10. Alternate Business Flows
- Requested route is inactive: reassignment is not made effective.
- Requested stop is unavailable: an available authorised stop must be identified.
- Approval is rejected: the current allocation remains effective.
- The request actually requires permanent route redesign: use the distinct route-revision scenario instead.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Target route inactive | Reassignment cannot become effective | Select an active route |
| Target stop unavailable | Reassignment cannot use the requested stop | Select an authorised available stop |
| Approval rejected | Current allocation remains | Retain or revise request |
| Change violates configured validation | Reassignment cannot apply | Correct condition and re-evaluate |

No technical retries, queues, rollback or infrastructure recovery mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule | Effect |
|---|---|
| Transport Allocation owns allocation status/history. | Reassignment remains G04-owned. |
| Every Allocation shall reference an active Route. | Inactive routes cannot be assigned. |
| Route Change and Stop Change are authorised allocation changes in G04. | The scenario follows the locked workflow catalogue. |
| Approval policies remain configurable. | No local approval rule is invented. |
| Allocation history preserves route, stop and change history. | Previous relationship remains traceable. |
| Route and Stop retain their own aggregate ownership. | Reassignment does not transfer route/stop ownership. |

## 13. State and Lifecycle Impact
Allocation lifecycle remains `Applied → Approved → Allocated → Active → Withdrawn`. A route/stop reassignment does not create a new allocation lifecycle state.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
**`TransportAllocationUpdated`** — allocation modified. **Canonical publisher: Transport Management (G04).** No new reassignment event is invented.
### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participating domains are G, C, E and H. G04 owns the allocation change; C/E provide or consume authoritative student/workforce context; H may consume the changed allocation for governed communication. Participation does not transfer ownership.

## 15. Business Authorization, Approval and Audit Obligations
G04 identifies Transport Administrator for allocation work and Transport Manager for transport-change approval. Exact approval policy is configurable. Old/new route/stop relationship, approval, effective change and history are auditable.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Existing allocation identified.
- Target route/stop validated.
- Approval completed.
- New relationship becomes effective.
- `TransportAllocationUpdated` is published.
- Previous relationship remains historically traceable.
### 16.2 Postconditions
- The named allocation outcome is authoritative within G04.
- Allocation history remains traceable.
- No unrelated domain lifecycle is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material source conflict identified. Route redesign remains separate from allocation reassignment.
### 17.2 Assumptions
- G04 configuration-driven policies remain authoritative.
- No additional actor, rule, state, event, publisher, owner or dependency is invented.
### 17.3 Deferred Questions
Any material upstream ambiguity must be resolved through architecture governance before Architecture Approval/Lock.
