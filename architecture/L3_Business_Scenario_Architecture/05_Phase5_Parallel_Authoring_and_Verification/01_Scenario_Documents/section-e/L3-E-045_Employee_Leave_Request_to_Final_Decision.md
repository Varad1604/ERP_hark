# Employee Leave Request to Final Decision

**Scenario ID:** L3-E-045  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E05 — Leave Management Workflows; applicable employee context and leave-policy governance  
**Referenced By:** None yet

## 1. Scenario Metadata
This Draft preserves frozen Stage 3 semantics and applies locked upstream architecture without redesigning ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose
Process an employee leave request through validation, eligibility, balance checking and governed approval so that the institution reaches a final leave decision.

## 3. Business Outcome
A final approved or rejected leave decision is recorded for the employee leave request.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An employee initiates a leave request.
### 4.2 End Boundary
The leave approval decision is recorded as approved or rejected and the request reaches the decision outcome boundary.
### 4.3 In Scope
Leave request submission; request validation; eligibility evaluation; balance availability check; approval routing; final approval/rejection decision; decision event publication.
### 4.4 Out of Scope
Post-decision balance reservation/consumption and ledger processing after approval, which continue in the authoritative Leave Management lifecycle beyond this scenario's final-decision boundary.
### 4.5 Related Scenarios
None identified as necessary to clarify this boundary.

## 5. Ownership and Participating Domains
| Domain | Business responsibility |
|---|---|
| Leave Management / L2-E05 | Owns the Leave Request Aggregate, policy/eligibility evaluation, approval lifecycle and final leave decision. |
| Employee context provider | Provides authoritative employee context without owning the leave decision. |

Leave Management remains the canonical owner of the leave request and decision.

## 6. Business Actors, Roles and Responsibilities
- Employee — initiates the leave request and provides the requested leave period/type.
- Approver — reviews the validated request and records the approval decision.
- Leave Management Domain — owns request validation, eligibility evaluation, approval workflow and decision outcome.
- Policy/rule capability — supplies the applicable leave-policy evaluation context.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- The employee exists in authoritative employee context.
- An active leave policy exists.
- The requested leave type is available.
- The employee is eligible for the requested leave under applicable policy.
- An approval route is available.

## 8. Trigger
Employee initiates a leave request.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Select the leave type and provide the requested leave period. | Employee | Leave request intent is established. |
| 2 | Validate the request. | Leave Management | Request validation result is established. |
| 3 | Evaluate eligibility under the active leave policy. | Leave Management / policy capability | Eligibility result is established. |
| 4 | Check available leave balance. | Leave Management | Balance-availability result is established. |
| 5 | Route the validated request through the applicable approval workflow. | Leave Management | Approval decision is ready. |
| 6 | Review the request and record the final decision. | Approver | Approved or rejected decision is established. |
| 7 | Publish the applicable leave decision event. | Leave Management | Final decision is available as an authoritative leave outcome. |

## 10. Alternate Business Flows
- If the employee is not eligible, the request is rejected or directed to the applicable exception workflow.
- If policy validation fails, the request is rejected or handled through the governed exception path.
- If leave balance is insufficient, the request is rejected or handled through the controlled exception path.
- If approval times out, the authoritative approval workflow supports escalation back to the approver.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Policy violation | Request cannot proceed as a normal approved request. | Reject or follow the governed exception workflow. | Leave Management |
| Insufficient balance | Requested leave cannot proceed normally. | Reject or follow the controlled exception path. | Leave Management |
| Approval timeout | Final decision is delayed. | Escalate through the authoritative approval workflow. | Leave Management / Approver |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns all leave decisions.
- The Leave Request Aggregate owns request lifecycle and approval state.
- An active leave policy and available leave type are required before normal request processing.
- Eligibility and policy rules are evaluated before approval.
- Balance availability is checked before the approval decision proceeds.
- Approved requests proceed to balance reservation; rejected requests close the request.
- Every workflow transition produces traceable business history.

## 13. State and Lifecycle Impact
The scenario covers the Leave Request Aggregate from request creation/validation through the approval decision. The authoritative post-approval flow continues to balance reservation, leave consumption and ledger processing; those later activities are outside this scenario boundary.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
- **EmployeeLeaveRequested** — published when the leave request is created.
- **EmployeeLeaveApproved** — authoritative approval outcome when the request is approved.
- Other leave events exist for later processing stages; this scenario stops at the final decision.
### 14.2 Synchronous Business Contracts / Hand-offs
The final leave decision is a business outcome consumed by relevant downstream business participants. Subsequent reservation/consumption/ledger activities remain within L2-E05.

## 15. Business Authorization, Approval and Audit Obligations
The approval decision must be recorded with the responsible approver and traceable business history. Policy/eligibility evaluation and final decision must remain auditable. Technical authorization enforcement is outside L3.

## 16. Completion Criteria and Postconditions
- Leave request has been validated.
- Eligibility and balance availability have been evaluated.
- The applicable approval route has completed its decision.
- The final approved or rejected outcome is recorded.
- The applicable leave decision event is published.
- The scenario ends before post-approval reservation/consumption processing.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
None material identified in the authoritative evidence reviewed for this scenario.
### 17.2 Assumptions
None.
### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
