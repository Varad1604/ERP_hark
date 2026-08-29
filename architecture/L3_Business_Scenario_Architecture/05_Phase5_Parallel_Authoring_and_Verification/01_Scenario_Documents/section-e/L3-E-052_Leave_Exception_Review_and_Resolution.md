# Leave Exception Review and Resolution

**Scenario ID:** L3-E-052  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E05 — Leave Management Workflows  
**Referenced By:** None yet

## 1. Scenario Metadata

This scenario preserves the frozen Stage 3 identity and applies the locked L2 business workflow architecture. It does not redefine canonical ownership, lifecycle authority, business rules, event publishers, or downstream dependencies.

## 2. Scenario Purpose

Provide controlled review and resolution of exceptional leave situations while preserving Leave Management as the authoritative owner of leave decisions.

## 3. Business Outcome

The leave exception is reviewed under the applicable authority and resolved as approved or rejected, with the required business evidence and leave outcome recorded.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An exceptional leave situation is identified and enters the controlled exception workflow.

### 4.2 End Boundary
The exception decision is recorded as approved or rejected and the applicable leave outcome is established.

### 4.3 In Scope
Exception identification; authority validation; business-reason review; required approval; exception decision; applicable leave outcome; audit evidence and canonical exception events.

### 4.4 Out of Scope
Technical authorization enforcement, notification delivery, salary calculation, document storage and any unrelated workforce lifecycle change.

### 4.5 Related Scenarios
The exception workflow may be invoked from ordinary leave request processing when policy violation, insufficient balance or another governed exceptional condition is identified.

## 5. Ownership and Participating Domains

Leave Management is the canonical owner of leave policy, eligibility, entitlement, leave requests, approvals, balances, adjustments, carry-forward, encashment eligibility and leave-domain events. External domains provide context but do not own the leave decision.

## 6. Business Actors, Roles and Responsibilities

- Employee — subject of the exceptional leave case where applicable.
- Leave Management Domain — validates and manages the exception workflow.
- Authorized approver — reviews and records the exception decision where approval is required.
- Relevant workforce/organizational context providers — provide authoritative context without taking ownership of the leave decision.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria

- A leave-related exceptional condition exists.
- The employee and relevant leave context are identifiable.
- The applicable policy/rule context is available.
- Required authority and approval path can be determined.

## 8. Trigger

A policy exception, emergency leave condition, negative-balance approval request, manual correction or other governed exceptional leave situation is identified.

## 9. Main Business Flow

| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Identify the exceptional leave condition and establish the exception case. | Leave Management | Exception is formally under review. |
| 2 | Validate the authority for the exception and establish the business reason. | Leave Management | Authority and reason are recorded. |
| 3 | Review the exception against applicable leave policy and business constraints. | Leave Management | Exception eligibility and decision basis are established. |
| 4 | Obtain the required approval where the exception requires approval. | Authorized approver / Leave Management | Approval decision is established. |
| 5 | If approved, apply the permitted leave outcome through the authoritative Leave Management workflow. | Leave Management | Exception-approved leave outcome is established. |
| 6 | If rejected, record the rejection and preserve the ordinary leave outcome. | Leave Management | Exception-rejected outcome is established. |
| 7 | Preserve the complete business audit trail and publish the canonical exception decision event. | Leave Management | Exception case reaches its final resolution. |

## 10. Alternate Business Flows

- A policy exception may be approved when the required authority, reason and approval conditions are satisfied.
- An exception may be rejected when the required authority, reason, approval or applicable business condition is not satisfied.
- An ordinary leave request may be routed into this workflow when the normal path identifies a policy violation or insufficient balance requiring controlled exception handling.

## 11. Failure, Interruption and Business Recovery

| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Authority is insufficient | Exception cannot be approved. | Preserve the case as unresolved/rejected according to the applicable business workflow. | Leave Management |
| Required business reason is absent | Exception decision cannot be properly established. | Obtain the required business reason before decision. | Leave Management |
| Required approval is not granted | Exception is rejected. | Record the rejection and preserve the applicable ordinary leave outcome. | Leave Management / Approver |
| Exceptional condition remains unresolved | No exceptional leave outcome is established. | Continue the governed exception review until a decision is reached. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints

- Exceptions require an authorized role/authority, a business reason, approval where required and a complete audit trail.
- Overrides may exist for emergency leave, exceptional approval and policy exceptions.
- Leave Management owns all leave decisions.
- External domains cannot directly mutate Leave Management aggregate state.
- Every balance-affecting correction follows the Leave Ledger principle; the ledger is authoritative and historical transactions remain immutable.

## 13. State and Lifecycle Impact

The exception workflow produces an approved or rejected business outcome. Where an approved exception changes a leave request, balance or ledger position, the authoritative Leave Request, Leave Ledger and Leave Balance lifecycles remain governed by E05. No exception-specific lifecycle state is invented.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

- **LeaveExceptionApproved** — published for an approved exception outcome.
- **LeaveExceptionRejected** — published for a rejected exception outcome.

### 14.2 Synchronous Business Contracts / Hand-offs

An approved exception may result in a governed leave outcome consumed by Payroll, Analytics, Notification or Audit participants according to E05 dependencies. Such participation does not transfer ownership from Leave Management.

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Every exception requires authority, business reason, approval where required and a complete audit trail. The exception decision and any resulting leave/ledger outcome must remain traceable.

## 16. Completion Criteria and Postconditions

- Authority and business reason are established.
- Applicable policy/business constraints are reviewed.
- Required approval is recorded where applicable.
- Exception is resolved as approved or rejected.
- Canonical exception event is published.
- Resulting leave outcome is recorded without duplicating ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
