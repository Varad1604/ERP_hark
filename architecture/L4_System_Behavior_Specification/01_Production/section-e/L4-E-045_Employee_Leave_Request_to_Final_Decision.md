# Employee Leave Request to Final Decision

## 1. Specification Metadata

**Specification ID:** L4-E-045  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-045 — Employee Leave Request to Final Decision  
**Primary Business Owner:** Leave Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-045; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall process an employee leave request through validation, eligibility evaluation, balance checking and governed approval until the Leave Request Aggregate reaches the authoritative final decision state `Approved` or `Rejected` and the matching decision event is published.

## 3. Required Business Context

- The Employee exists in authoritative employee context.
- An active leave policy exists.
- The requested leave type is available under the active policy.
- Employee eligibility and balance availability can be evaluated.
- An approval route to the authorized Manager / Approver can be determined.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-045-BH-001 | The system shall establish the submitted Leave Request under Leave Management ownership and validate the requested leave type and period. | L3-E-045 §§3, 4, 5, 9, 13 |
| L4-E-045-BH-002 | The system shall evaluate Employee eligibility under the active leave policy before approval progression. | L3-E-045 §§7, 9, 12 |
| L4-E-045-BH-003 | The system shall evaluate available leave balance before the request proceeds through normal approval. | L3-E-045 §§9, 11, 12 |
| L4-E-045-BH-004 | The system shall route the validated request to the assigned Manager / Approver; the Employee shall not approve their own request. | L3-E-045 §§6, 9, 12, 15 |
| L4-E-045-BH-005 | The system shall record the final approval or rejection decision and transition the Leave Request from `PendingApproval` to `Approved` or `Rejected` accordingly. | L3-E-045 §§3, 9, 13, 16 |
| L4-E-045-BH-006 | The system shall preserve request validation, eligibility, balance evaluation, approval routing, decision authority and final-decision history. | L3-E-045 §§15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-045-BH-007 | On request submission, the Leave Request Aggregate shall publish `EmployeeLeaveRequested`. | L3-E-045 §14.1 |
| L4-E-045-BH-008 | If the final state is `Approved`, the Leave Request Aggregate shall publish `EmployeeLeaveApproved`; if the final state is `Rejected`, it shall publish `EmployeeLeaveRejected`. | L3-E-045 §§3, 14.1, 16 |
| L4-E-045-BH-009 | No synchronous cross-domain handoff is required to establish the final leave decision beyond the authoritative L2-E05 workflow; downstream consumers may consume the published decision event without acquiring Leave Request ownership. | L3-E-045 §14.2 |

## 6. Failure / Rejection / Continuation

- If request validation fails, the request shall not proceed through normal approval and shall follow the governed rejection/exception path.
- If the Employee is ineligible, the request shall not proceed as a normal approvable request.
- If available balance is insufficient where balance is required, the request shall follow the governed rejection/exception path.
- If approval is incomplete, the request shall remain pending within the authoritative approval/escalation workflow until a decision is established.
- Technical retry, rollback, queue and infrastructure recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of request submission, validation, policy/eligibility evaluation, balance evaluation, approval routing, Manager / Approver identity, decision and final Leave Request state.

## 8. Observable Completion Conditions

Completion requires validation, eligibility and balance evaluation to be complete, the approval route to reach a decision, the Leave Request to reach `Approved` or `Rejected`, and the matching canonical decision event (`EmployeeLeaveApproved` or `EmployeeLeaveRejected`) to be published.

## 9. Traceability & Provenance

**Parent:** L3-E-045 — Employee Leave Request to Final Decision  
**Primary verification source:** L2-E05 Leave Management Workflows as traced by the Locked parent.

## 10. Open Issues / Deferred Design

No material event, ownership or lifecycle uncertainty is identified by the Locked parent for this scenario. Institution-specific policy configuration and technical APIs, persistence schemas, UI, transport and retry mechanics remain outside L4 / deferred to later technical design.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
