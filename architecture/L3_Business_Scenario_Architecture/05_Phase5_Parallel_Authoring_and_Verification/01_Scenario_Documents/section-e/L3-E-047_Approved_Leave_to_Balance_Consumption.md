# Approved Leave to Balance Consumption

**Scenario ID:** L3-E-047  
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
Frozen Stage 3 semantics are preserved. This Draft applies the authoritative L2 workflow architecture without changing canonical ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose
Convert an approved leave reservation into actual leave consumption at the applicable consumption point while preserving the authoritative leave ledger and balance representation.

## 3. Business Outcome
Approved leave is consumed and the corresponding ledger and balance outcomes are established.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
Approved leave reaches its applicable consumption date with a valid balance reservation.
### 4.2 End Boundary
Leave consumption is recorded in the ledger, the balance projection is updated, and the consumption outcome is published.
### 4.3 In Scope
Consumption eligibility; reservation-to-consumption transition; consumption ledger transaction; balance projection update; consumption event publication.
### 4.4 Out of Scope
Initial leave request approval, policy evaluation, leave cancellation and later balance adjustment.
### 4.5 Related Scenarios
Preceded by the employee leave approval/reservation lifecycle represented by L2-E05.

## 5. Ownership and Participating Domains
| Domain | Business responsibility |
|---|---|
| Leave Management / E05 | Owns leave consumption, ledger transaction and balance projection. |
| Attendance Domain | Participates as an upstream/downstream business context where defined by the authoritative architecture; it does not own leave consumption. |

Cross-domain participation does not transfer Leave Management ownership.

## 6. Business Actors, Roles and Responsibilities
- Leave Management Domain — owns consumption and resulting leave facts.
- Attendance Domain — participates in the authoritative consumption context where applicable.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- Leave has been approved.
- A valid balance reservation exists.
- The applicable consumption date has been reached.

## 8. Trigger
Approved leave reaches its consumption date.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Confirm that approved leave has reached the consumption point and a reservation exists. | Leave Management | Consumption is eligible to proceed. |
| 2 | Convert the valid reservation into leave consumption. | Leave Management | Leave consumption is established. |
| 3 | Create the consumption transaction in the Leave Ledger. | Leave Management | Consumption becomes authoritative ledger history. |
| 4 | Update the balance projection from the ledger transaction. | Leave Management | Current leave balance reflects consumption. |
| 5 | Publish the consumption and balance-update business events. | Leave Management | Consumption outcome is available to relevant participants. |

## 10. Alternate Business Flows
- If the preconditions for consumption are not satisfied, consumption is not established until the authoritative business conditions are met.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| No valid reservation exists | Consumption cannot be established through this workflow. | Preserve the existing leave state and follow the applicable upstream leave workflow. | Leave Management |
| Consumption conditions are not met | Leave remains unconsumed. | Re-evaluate at the applicable governed business point. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- Approved leave must have a balance reservation before consumption.
- Leave Ledger is the authoritative record of balance-affecting transactions.
- Balance is not directly mutated; the consumption transaction precedes the balance projection update.
- Historical ledger transactions are immutable.

## 13. State and Lifecycle Impact
The Leave Request Aggregate moves from Reserved to Consumed for the applicable approved leave. The ledger records the consumption and the balance projection is updated. Later closure is outside this scenario boundary.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
- **LeaveConsumed** — published for the consumption outcome.
- **EmployeeLeaveBalanceUpdated** — published after the balance projection is updated.
### 14.2 Synchronous Business Contracts / Hand-offs
The consumption outcome is made available to the Attendance Domain and other relevant downstream participants according to the authoritative L2 dependency model. Leave Management remains the owner of the leave fact.

## 15. Business Authorization, Approval and Audit Obligations
The reservation-to-consumption decision and resulting ledger transaction must be traceable. The immutable ledger preserves historical consumption evidence.

## 16. Completion Criteria and Postconditions
- Approved leave has reached its consumption date.
- Valid reservation has been converted into consumption.
- Consumption ledger transaction is created.
- Balance projection is updated.
- LeaveConsumed and EmployeeLeaveBalanceUpdated outcomes are published.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
None material identified in the authoritative L2 evidence reviewed.
### 17.2 Assumptions
None.
### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
