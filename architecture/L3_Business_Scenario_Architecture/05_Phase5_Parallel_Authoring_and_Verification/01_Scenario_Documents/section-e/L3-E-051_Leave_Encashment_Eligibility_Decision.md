# Leave Encashment Eligibility Decision

**Scenario ID:** L3-E-051  
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
Frozen Stage 3 identity and semantics are preserved. This scenario applies the locked Leave Management workflow architecture and does not redefine ownership, rules, lifecycle or event publishers.

## 2. Scenario Purpose
Determine whether an employee's leave position meets the applicable policy conditions for leave encashment and establish the governed eligibility decision.

## 3. Business Outcome
An authoritative leave encashment eligibility decision is established under the applicable leave policy.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An employee leave position enters encashment eligibility evaluation.

### 4.2 End Boundary
The eligibility decision has been evaluated and recorded as the governed business outcome.

### 4.3 In Scope
Policy eligibility evaluation; employee leave context; encashment rule evaluation; eligibility decision; business outcome publication where defined.

### 4.4 Out of Scope
Payroll calculation/payment execution and financial settlement are outside Leave Management ownership.

### 4.5 Related Scenarios
Related leave workflows may consume or precede this outcome, but ownership remains with Leave Management.

## 5. Ownership and Participating Domains
Leave Management is the canonical owner of leave entitlements, accrual, balances, ledger transactions, leave requests, approvals, cancellations and related leave events. External domains provide context or consume leave facts; they do not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Leave Management Domain — owns and executes the business workflow.
- Employee context — supplies authoritative workforce context where required.
- Other participants are included only where supported by the locked L2 workflow.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- The applicable leave policy and relevant rules are available.
- Required employee/leave context is available.
- The business condition for this scenario has been reached.

## 8. Trigger
An employee leave position enters encashment eligibility evaluation.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Establish the employee leave position and applicable active policy. | Leave Management | Evaluation context is established. |
| 2 | Evaluate the leave encashment eligibility rules. | Leave Management | Rule evaluation result is established. |
| 3 | Determine the employee's eligibility outcome. | Leave Management | Eligible/not-eligible decision is established. |
| 4 | Record the eligibility decision as the authoritative Leave Management outcome. | Leave Management | Encashment eligibility is governed by the recorded decision. |
| 5 | Make the business outcome available to downstream participants where required. | Leave Management | Downstream processing can use the eligibility decision without transferring ownership. |

## 10. Alternate Business Flows
- If eligibility criteria are not satisfied, the outcome is not eligible and no encashment entitlement is established by this scenario.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Eligibility condition is not satisfied | Employee is not eligible under the applicable policy. | Record the non-eligible outcome. | Leave Management |
| Required policy/context is unavailable | Eligibility cannot be validly determined. | Complete the required business context before deciding. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns leave encashment eligibility.
- Eligibility is determined from the applicable leave policy and employee leave context.
- Payroll owns salary calculation and financial settlement; Leave Management does not perform those downstream calculations.

## 13. State and Lifecycle Impact
The scenario establishes an eligibility decision; it does not alter the employee's leave-request lifecycle or perform payroll settlement.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
No canonical encashment-specific event identifier/name is exposed in the accessible L2-E05 evidence. No event name is invented.

### 14.2 Synchronous Business Contracts / Hand-offs
The eligibility decision may be consumed by Payroll for downstream settlement according to the L2 dependency model. This is a business hand-off; ownership remains with the originating domain.

Events remain distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
The eligibility evaluation and decision must be traceable to the applicable policy/rules and employee leave context.

## 16. Completion Criteria and Postconditions
- Applicable policy and leave context are established.
- Encashment eligibility rules are evaluated.
- Eligibility decision is recorded.
- Downstream consumers can use the decision where required.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
The accessible L2-E05 evidence does not expose a canonical encashment-eligibility event identifier/name.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
No unresolved business question is introduced beyond the issue stated above.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft; canonical event detail is not explicitly exposed in the accessible L2 evidence.
