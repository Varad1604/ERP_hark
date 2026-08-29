# Employee Leave Case Coordination to Leave Decision

**Scenario ID:** L3-E-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E04 — HR Workflows; L2-E05 — Leave Management Workflows; applicable employee/organisation context  
**Referenced By:** None yet.

## 1. Scenario Metadata

This scenario preserves the frozen Stage 3 canonical title, Primary Owner, Participating Domains, Scenario Type and Priority. It applies the locked upstream architecture without changing ownership, rules, lifecycle, events, contracts or dependency direction.

## 2. Scenario Purpose

Coordinate an employee leave-related HR case with the authoritative Leave Management domain so that the leave decision is established and the HR case records the resulting business outcome without taking ownership of leave lifecycle state.

## 3. Business Outcome

The HR case reaches its governed outcome based on the authoritative Leave Management decision, while Leave Management remains the sole owner of the leave request, approval and leave lifecycle.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An employee leave/absence matter requires HR case coordination or follow-up.

### 4.2 End Boundary

The HR case has recorded the authoritative leave decision and reached its corresponding HR case outcome.

### 4.3 In Scope

- HR case coordination around an employee leave matter.
- Handoff to Leave Management for leave lifecycle and decision authority.
- Receipt of the authoritative leave outcome.
- Recording and closure of the HR case based on that outcome.

### 4.4 Out of Scope

The scenario does not own or redefine business data, lifecycle, rules or decisions belonging to another canonical domain. Technical implementation concerns are outside L3.

### 4.5 Related Scenarios

None yet.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| HR | Owns the final HR business-case outcome defined by this scenario. |
| Participating People & Workforce domain(s) | Provides the authoritative domain-owned business decision or fact required by the HR case. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

- Employee.
- HR Administrator.
- Manager / Approver where the authoritative leave workflow requires an approver.
- Leave Management domain.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- An employee context is available.
- A leave-related matter requires HR case coordination.
- Leave Management is available to determine the authoritative leave outcome.

## 8. Trigger

An HR case is raised for an employee leave/absence matter requiring coordination with Leave Management.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the HR case and identify the employee leave matter. | HR | HR case is initiated for the leave-related matter. |
| 2 | Coordinate the leave matter with Leave Management without taking ownership of leave lifecycle state. | HR / Leave Management | Leave decision is placed under the authoritative E05 workflow. |
| 3 | Leave Management evaluates the leave request using its authoritative policy, eligibility, balance and approval workflow. | Leave Management | Leave decision is established by the owning domain. |
| 4 | HR receives and records the authoritative leave outcome relevant to the HR case. | HR | HR case reflects the leave decision. |
| 5 | Complete the HR case according to the resulting business outcome. | HR | HR case reaches its governed outcome. |

The Main Business Flow stops once the Business Outcome and End Boundary are achieved.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined beyond the authoritative leave approval/rejection outcomes; those outcomes are handled as business decision results rather than new HR-owned lifecycle states.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Leave decision is rejected | The requested leave outcome is not established as approved. | HR records the authoritative rejection outcome and applies the applicable HR case resolution. | Leave Management / HR |
| Required leave information or eligibility is not satisfied | The leave decision cannot progress to approval. | Leave Management keeps the leave request in its governed lifecycle until correction or the applicable terminal outcome. | Leave Management |

No technical retry, queue, rollback or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

- Leave Management is authoritative for leave-related business decisions.
- Leave request lifecycle and approval state are owned by the Leave Request Aggregate in E05.
- Leave balance is not directly mutated; balance-affecting operations are represented through the Leave Ledger.
- HR Workflows owns only the HR case wrapper/coordination and does not own leave state.
- Leave policy and eligibility rules remain owned by Leave Management.

The scenario applies these upstream-owned rules; it does not become their owner.

## 13. State and Lifecycle Impact

**HR Case:** the case follows the applicable HR case lifecycle through completion.

**Leave Request:** the leave lifecycle remains owned by E05. Relevant canonical states include Submitted, Validated, PendingApproval, Approved, Rejected, Reserved, Consumed and Closed, with cancellation paths defined upstream.

No lifecycle state is invented or renamed.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

`LeaveApproved` is the canonical leave business fact identified by the repository's architecture baseline. It is published by Leave Management, not HR.

HR may consume the authoritative leave outcome for case coordination; event publication does not transfer leave ownership.

### 14.2 Synchronous Business Contracts / Hand-offs

The material cross-domain hand-off is from the HR case to the authoritative Leave Management workflow and back as a business outcome. The hand-off does not transfer ownership of Leave Request, Leave Policy, Leave Balance or Leave Ledger.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

HR case handling follows the HR workflow's business approval and audit requirements. Leave approval authority remains governed by Leave Management. Approval decisions record the required business decision evidence; technical authorization mechanisms are outside L3.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The HR case has a recorded authoritative leave decision.
- Leave ownership remains with Leave Management.
- The HR case reaches its governed outcome.
- Required business history is preserved.

### 16.2 Postconditions

The completed HR case outcome remains historically traceable. Any domain-owned state or decision remains authoritative in its canonical owning domain.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified from the authoritative sources reviewed for this scenario.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
