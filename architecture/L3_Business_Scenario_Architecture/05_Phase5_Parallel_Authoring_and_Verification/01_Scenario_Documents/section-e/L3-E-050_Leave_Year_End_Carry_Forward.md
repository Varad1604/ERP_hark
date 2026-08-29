# Leave Year-End Carry Forward

**Scenario ID:** L3-E-050  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E05 — Leave Management Workflows  
**Referenced By:** None yet

## 1. Scenario Metadata
Frozen Stage 3 identity and semantics are preserved. This scenario applies the locked Leave Management workflow architecture and does not redefine ownership, rules, lifecycle or event publishers.

## 2. Scenario Purpose
Process eligible unused leave at the end of a leave year according to active carry-forward rules, preserving the result in the authoritative leave ledger.

## 3. Business Outcome
Eligible unused leave is carried forward through a governed ledger transaction, while ineligible remaining balance is handled according to the applicable expiry rules.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
Leave year closure is reached for the applicable leave policy period.

### 4.2 End Boundary
The carry-forward/expiry outcomes have been recorded in the Leave Ledger and the balance projection reflects the resulting position.

### 4.3 In Scope
Year-end closure; active policy evaluation; carry-forward eligibility; carry-forward or expiry transaction; ledger update; balance projection.

### 4.4 Out of Scope
Leave accrual, ordinary leave requests, leave approval and manual adjustment.

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
Leave year closure is reached for the applicable leave policy period.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Identify the applicable leave-year closure and active policy. | Leave Management | Year-end processing context is established. |
| 2 | Evaluate carry-forward rules. | Leave Management | Eligibility is determined. |
| 3 | For eligible balance, create the carry-forward transaction. | Leave Management | Carry-forward is recorded as authoritative ledger history. |
| 4 | For ineligible remaining balance, apply the applicable expiry outcome. | Leave Management | Expiry is recorded as governed ledger history. |
| 5 | Update the balance projection from the ledger. | Leave Management | New-period balance representation is established. |

## 10. Alternate Business Flows
- Eligible balance follows the carry-forward rule.
- Ineligible balance follows the applicable expiry rule.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Employee is not eligible for carry-forward | Unused balance is handled by the applicable expiry rule. | Record the governed expiry transaction. | Leave Management |
| Policy rule excludes part of the balance | Only the eligible portion is carried forward. | Apply the active policy rule and record the resulting transactions. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- Carry-forward is governed by the active leave policy.
- Leave Ledger is the authoritative transaction history.
- Balance is a projection from ledger transactions.
- Historical transactions are immutable.
- Carry-forward and expiry are separate business outcomes.

## 13. State and Lifecycle Impact
The scenario covers leave-year closure through carry-forward/expiry ledger outcomes and resulting balance projection. It does not create a new leave-request lifecycle.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
The L2 workflow defines a carry-forward/expiry processing outcome, but the accessible evidence does not expose a canonical event identifier/name for the carry-forward event.

### 14.2 Synchronous Business Contracts / Hand-offs
Resulting leave facts are available to downstream leave consumers such as Payroll and Analytics according to L2-E05 dependencies.

Events remain distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Year-end rule evaluation and the resulting carry-forward/expiry transactions must be traceable. The ledger preserves immutable historical evidence.

## 16. Completion Criteria and Postconditions
- Year-end context is established.
- Carry-forward rules are evaluated.
- Eligible balance is recorded through carry-forward transaction.
- Ineligible balance is handled through expiry transaction.
- Balance projection reflects the resulting ledger position.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
The accessible L2-E05 evidence does not expose the canonical carry-forward event identifier/name.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
No unresolved business question is introduced beyond the issue stated above.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft; carry-forward event identifier requires confirmation from the authoritative event register.
