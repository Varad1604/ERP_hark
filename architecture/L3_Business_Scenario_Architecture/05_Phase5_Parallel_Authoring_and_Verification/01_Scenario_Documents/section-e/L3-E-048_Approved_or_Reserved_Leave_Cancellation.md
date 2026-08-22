# Approved or Reserved Leave Cancellation

**Scenario ID:** L3-E-048  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Leave Management  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E05 — Leave Management Workflows  
**Referenced By:** None yet

## 1. Scenario Metadata
Frozen Stage 3 semantics are preserved. This Draft applies the authoritative L2 workflow architecture without changing canonical ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose
Process an authorized cancellation of approved or reserved leave while preserving leave-request and ledger integrity.

## 3. Business Outcome
The applicable approved or reserved leave is cancelled and the corresponding business history and balance effect are established through the governed cancellation outcome.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An employee or authorized user requests cancellation of existing leave.
### 4.2 End Boundary
Cancellation is accepted under applicable policy and the required reservation/consumption reversal and ledger outcome are recorded.
### 4.3 In Scope
Cancellation request validation; policy restriction check; reversal of reservation or consumption where applicable; reversal ledger entry; leave-request status update; cancellation and balance-update events.
### 4.4 Out of Scope
Initial leave request approval, new leave submission, year-end carry-forward, accrual and independent balance adjustment.
### 4.5 Related Scenarios
None required for the boundary.

## 5. Ownership and Participating Domains
| Domain | Business responsibility |
|---|---|
| Leave Management / E05 | Owns leave cancellation, Leave Request lifecycle, policy restrictions and reversal ledger entry. |

Leave Management retains canonical ownership of the cancelled leave fact and its ledger history.

## 6. Business Actors, Roles and Responsibilities
- Employee — may request cancellation where permitted.
- Approver / authorized business participant — participates where the cancellation workflow requires approval authority.
- Leave Management Domain — validates cancellation, applies policy restrictions, performs the governed reversal and updates the request outcome.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- The leave request exists.
- The applicable cancellation policy permits the requested action.
- The leave is in a state for which cancellation is allowed.

## 8. Trigger
An employee or authorized user requests cancellation.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Validate the cancellation request against the leave request and applicable policy. | Leave Management | Cancellation eligibility is established. |
| 2 | Check policy restrictions governing cancellation. | Leave Management | Cancellation may proceed or be rejected. |
| 3 | Reverse the applicable reservation or consumption according to the authoritative cancellation path. | Leave Management | The prior balance-affecting effect is reversed as governed. |
| 4 | Create the required reversal ledger entry. | Leave Management | Correction is preserved as immutable ledger history. |
| 5 | Update the leave request outcome/status. | Leave Management | Cancellation is recorded on the Leave Request Aggregate. |
| 6 | Publish the cancellation and balance-update business events. | Leave Management | Cancellation outcome is available to relevant participants. |

## 10. Alternate Business Flows
- If cancellation policy restrictions do not permit the action, the cancellation is not completed.
- The authoritative workflow distinguishes reversal of a reservation from reversal of consumption according to the leave's current business position.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Cancellation policy does not allow the action | Leave remains in its existing business state. | Reject the cancellation request and preserve the existing leave history. | Leave Management |
| Cancellation targets an invalid/non-cancellable state | No cancellation outcome is established. | Follow the applicable governed leave workflow. | Leave Management |
| A prior consumption must be corrected | The original ledger transaction remains immutable. | Create the governed reversal ledger entry. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- Cancellation is governed by applicable cancellation policy.
- The Leave Request Aggregate owns the cancellation lifecycle.
- Balance-affecting corrections are represented through ledger transactions rather than direct balance mutation.
- Historical ledger transactions are immutable; reversal transactions preserve the correction history.
- External domains cannot directly mutate leave aggregate state.

## 13. State and Lifecycle Impact
The Leave Request Aggregate moves to the cancellation outcome from an approved or reserved position as permitted by the authoritative state machine. The ledger records any required reversal. No new lifecycle state is invented.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
- **EmployeeLeaveCancelled** — published for the cancellation outcome.
- **EmployeeLeaveBalanceUpdated** — published when the cancellation changes the represented balance.
### 14.2 Synchronous Business Contracts / Hand-offs
Cancellation outcomes are available to downstream business participants such as Payroll, Attendance and Notification according to the authoritative dependency model. Leave Management remains the owner of the leave fact.

## 15. Business Authorization, Approval and Audit Obligations
The cancellation request, applicable policy decision, reversal action and resulting ledger entry must be traceable. The original transaction is preserved; corrections are represented through reversal history.

## 16. Completion Criteria and Postconditions
- Cancellation request has been validated.
- Applicable cancellation policy has been checked.
- Reservation or consumption has been reversed where required.
- Reversal ledger entry is created where required.
- Leave request cancellation outcome is recorded.
- EmployeeLeaveCancelled and applicable EmployeeLeaveBalanceUpdated events are published.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
None material identified in the authoritative L2 evidence reviewed.
### 17.2 Assumptions
None.
### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
