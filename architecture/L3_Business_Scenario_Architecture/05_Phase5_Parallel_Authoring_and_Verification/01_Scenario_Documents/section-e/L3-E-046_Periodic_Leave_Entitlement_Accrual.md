# Periodic Leave Entitlement Accrual

**Scenario ID:** L3-E-046  
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
Generate employee leave entitlement for an applicable accrual cycle using the active leave policy and configured accrual rules.

## 3. Business Outcome
The applicable leave entitlement accrual is established through an authoritative ledger transaction and reflected in the leave balance projection.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An applicable scheduled accrual cycle or policy-defined accrual trigger is reached.
### 4.2 End Boundary
Eligible accrual has been recorded as a ledger transaction and the resulting balance projection is updated.
### 4.3 In Scope
Accrual trigger; active-policy evaluation; employee eligibility evaluation; accrual calculation; ledger transaction; balance projection update; accrual event publication.
### 4.4 Out of Scope
Leave request approval, leave consumption, carry-forward, cancellation and manual adjustment, which are governed by separate leave workflows.
### 4.5 Related Scenarios
None required for the boundary.

## 5. Ownership and Participating Domains
| Domain | Business responsibility |
|---|---|
| Leave Management / E05 | Owns leave entitlement, accrual rules, eligibility evaluation, ledger transactions and balance projection outcome. |

External domains provide employee and contextual facts; they do not own the leave accrual decision.

## 6. Business Actors, Roles and Responsibilities
- Leave Management Domain — executes the accrual business workflow.
- Employee context provider — provides authoritative employee context.
- Policy/rule capability — supplies applicable accrual and eligibility rules.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- An active leave policy is available.
- The accrual trigger is applicable.
- Employee eligibility can be evaluated from authoritative context.

## 8. Trigger
A scheduled accrual cycle, policy-defined event, or other authoritative accrual trigger occurs.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Identify the applicable accrual trigger and active leave policy. | Leave Management | Accrual context is established. |
| 2 | Evaluate employee eligibility under the active policy. | Leave Management | Eligible population is determined. |
| 3 | Calculate the applicable leave accrual under configured rules. | Leave Management | Accrual amount/outcome is established. |
| 4 | Create the authoritative accrual transaction in the Leave Ledger. | Leave Management | Leave entitlement is recorded as ledger history. |
| 5 | Update the leave balance projection from the ledger transaction. | Leave Management | Current balance representation reflects the accrual. |
| 6 | Publish the applicable accrual business event. | Leave Management | Accrual outcome is available to relevant business participants. |

## 10. Alternate Business Flows
- If an employee is not eligible, no entitlement accrual is established for that employee for the applicable cycle.
- If a policy-defined condition excludes the accrual, the governed rules determine the resulting non-accrual outcome.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Employee is ineligible | No accrual entitlement is established. | Preserve the governed non-accrual outcome. | Leave Management |
| Accrual rule does not apply | No accrual is created for the affected rule/employee. | Apply the active policy rules for the applicable cycle. | Leave Management |
| A balance correction is subsequently required | The historical accrual must not be edited directly. | Use the governed adjustment/reversal business process. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns leave entitlement and accrual.
- Accrual is based on configured accrual rules and employee eligibility.
- The Leave Ledger is the authoritative record of leave transactions.
- Balance changes are represented through ledger transactions followed by balance projection update.
- Historical transactions are immutable; corrections use reversal/correction transactions.
- Policies and rules support effective dating and version history.

## 13. State and Lifecycle Impact
The accrual workflow advances the entitlement/balance business position through a ledger transaction and corresponding balance projection. It does not create a new Leave Request lifecycle state.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
The L2 workflow requires publication of an accrual event after the balance projection update. The accessible source evidence does not expose a canonical event identifier/name for that accrual event, so no name is invented.
### 14.2 Synchronous Business Contracts / Hand-offs
The accrued entitlement is a business outcome available to downstream leave-related processing. Leave Management retains ownership of the entitlement and ledger.

## 15. Business Authorization, Approval and Audit Obligations
The accrual trigger, applicable policy/rule evaluation, eligibility result and resulting ledger transaction must remain traceable. Historical leave transactions are immutable.

## 16. Completion Criteria and Postconditions
- Applicable accrual rules have been evaluated.
- Eligibility has been determined.
- The accrual has been recorded as a ledger transaction where applicable.
- The balance projection reflects the ledger transaction.
- The applicable accrual event is published.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
The accessible L2-E05 workflow defines publication of an accrual event but does not expose its canonical event identifier/name.
### 17.2 Assumptions
None.
### 17.3 Deferred Questions
Confirm the canonical accrual-event identifier/name from the authoritative event register before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft; canonical accrual event identifier is not explicitly exposed in the accessible L2 evidence.
