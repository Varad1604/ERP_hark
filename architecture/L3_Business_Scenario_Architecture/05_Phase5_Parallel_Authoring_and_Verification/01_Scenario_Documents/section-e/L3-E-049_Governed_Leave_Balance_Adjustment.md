# Governed Leave Balance Adjustment

**Scenario ID:** L3-E-049  
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
Provide a controlled business correction to a leave balance while preserving the immutable Leave Ledger as the authoritative history.

## 3. Business Outcome
A governed leave adjustment is recorded through the Leave Ledger and the resulting balance representation is updated.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorized adjustment request is raised for an employee leave balance.

### 4.2 End Boundary
The adjustment has been approved where required, the reversal/correction ledger entries have been recorded, and the balance projection reflects the resulting authoritative ledger position.

### 4.3 In Scope
Adjustment request; authority validation; reason review; approval decision; reversal where required; correction transaction; ledger update; balance projection; adjustment event.

### 4.4 Out of Scope
New leave accrual, leave request approval, normal leave consumption, carry-forward and cancellation are governed by separate workflows.

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
An authorized adjustment request is raised for an employee leave balance.

## 9. Main Business Flow
## Main Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Receive and validate the adjustment request and its authority. | Leave Management | Adjustment is eligible for review. |
| 2 | Review the business reason and applicable leave context. | Leave Management | Reason and adjustment basis are established. |
| 3 | Obtain the required business approval. | Leave Management / approving authority | Adjustment decision is established. |
| 4 | Where correcting an existing balance-affecting transaction, create the required reversal transaction. | Leave Management | Original history remains immutable and is offset by a governed reversal. |
| 5 | Create the correction/adjustment transaction. | Leave Management | Corrected leave position is recorded in the authoritative ledger. |
| 6 | Update the balance projection from the ledger. | Leave Management | Current balance representation reflects the correction. |
| 7 | Publish the applicable adjustment/balance-update outcome. | Leave Management | Corrected leave outcome is available to relevant participants. |

## 10. Alternate Business Flows
- If the adjustment is not approved, no corrective ledger outcome is established.
- If no reversal is required, the governed adjustment transaction is recorded directly according to the authoritative workflow.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Authority validation fails | Adjustment cannot proceed. | Preserve the existing authoritative leave position. | Leave Management |
| Adjustment is rejected | No correction is established. | Record the rejected business decision. | Leave Management / approving authority |
| Existing transaction requires correction | Original history must remain unchanged. | Use reversal followed by the governed correction transaction. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns leave balance adjustment.
- The Leave Ledger is the authoritative record of leave transactions.
- Historical transactions are immutable.
- Corrections use reversal transactions followed by corrective transactions where required.
- The balance projection is derived from ledger transactions and is not directly mutated.

## 13. State and Lifecycle Impact
The Leave Balance representation changes only as a consequence of authoritative ledger transactions. No new local balance lifecycle is introduced.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
The L2 workflow defines adjustment and balance-update business events, but the accessible source does not expose a canonical identifier/name for the adjustment event. `EmployeeLeaveBalanceUpdated` is the explicitly identified balance-update event.

### 14.2 Synchronous Business Contracts / Hand-offs
Corrected leave facts may be consumed by downstream Payroll, Analytics, Notification and Audit participants according to L2-E05 dependencies. Leave Management remains canonical owner.

Events remain distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
The adjustment request, business reason, approval decision and resulting ledger transactions must be traceable. Original transactions remain immutable.

## 16. Completion Criteria and Postconditions
- Authority and business reason are validated.
- Required approval is complete.
- Required reversal/correction ledger transactions are recorded.
- Balance projection reflects the authoritative ledger.
- Applicable business event outcome is published.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
The accessible L2-E05 evidence does not expose the canonical identifier/name of the adjustment event. No event name is invented.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
No unresolved business question is introduced beyond the issue stated above.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft; adjustment-event identifier requires confirmation from the authoritative event register.
