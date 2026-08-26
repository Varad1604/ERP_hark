# Leave Policy Establishment and Activation

**Scenario ID:** L3-E-044  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Leave Management  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E05 — Leave Management Workflows; applicable organisational governance context  
**Referenced By:** None yet

## 1. Scenario Metadata
This Draft preserves frozen Stage 3 semantics and applies locked upstream architecture without redesigning ownership, lifecycle, rules, events, contracts or dependencies.

## 2. Scenario Purpose
Establish and activate a governed leave policy so that approved leave rules become effective for institutional leave decision-making.

## 3. Business Outcome
A validated and approved leave policy is activated and becomes the effective policy governing applicable leave decisions.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A leave policy exists in draft form and enters validation for establishment/activation.
### 4.2 End Boundary
The policy has passed required validation and approval and is activated as the effective leave policy.
### 4.3 In Scope
Policy establishment; rule validation; conflict identification and governance resolution; policy approval; activation.
### 4.4 Out of Scope
Leave request processing, leave approvals, leave balances, accrual, entitlement, ledger transactions and employee leave decisions, which belong to other leave-management workflows.
### 4.5 Related Scenarios
None identified as necessary to clarify this boundary.

## 5. Ownership and Participating Domains
| Domain | Business responsibility |
|---|---|
| Leave Management / L2-E05 | Owns the Leave Policy Aggregate, policy rules, policy version lifecycle and policy activation decision. |
| Organisational governance participants | Participate in governance resolution/approval where required by the locked workflow. |

The Leave Management domain remains the canonical owner of the leave policy lifecycle.

## 6. Business Actors, Roles and Responsibilities
- Leave Management Domain — owns policy establishment and activation.
- Governance/approving authority — participates in policy approval and governance resolution where required.
- Policy/rule reviewer — participates in validation of leave rules where applicable.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
- A leave policy draft exists.
- The rules to be validated are available.
- Required governance and approval authority is available.
- Policy versioning/effective dating can be applied according to the authoritative leave-policy lifecycle.

## 8. Trigger
A leave policy draft enters the policy validation and activation workflow.

## 9. Main Business Flow
| Step | Business behaviour | Responsible participant | Business result |
|---|---|---|---|
| 1 | Establish the leave policy in its draft state. | Leave Management | Policy is ready for validation. |
| 2 | Validate the policy rules. | Leave Management / rule reviewer | Rule validation result is established. |
| 3 | Determine whether a policy conflict exists. | Leave Management / governance participant | Conflict decision is established. |
| 4 | If a conflict exists, resolve it through the governed business process and return the policy to validation. | Governance / Leave Management | Policy is ready for re-validation. |
| 5 | Obtain the required policy approval when validation is satisfactory. | Approving authority / Leave Management | Policy approval is established. |
| 6 | Activate the approved policy. | Leave Management | Policy becomes active. |
| 7 | Publish the policy-activation business event required by the authoritative L2 workflow. | Leave Management | Policy activation is available as a business outcome. |

## 10. Alternate Business Flows
- If validation identifies a conflict, governance resolution occurs before the policy returns to validation.
- If approval is not granted, the policy does not become active and remains governed by its pre-activation state.

## 11. Failure, Interruption and Business Recovery
| Situation | Business consequence | Business recovery | Owner |
|---|---|---|---|
| Policy validation identifies a conflict | Policy cannot proceed directly to approval/activation. | Resolve the conflict and return the policy to validation. | Leave Management / governance participant |
| Required approval is not obtained | Policy does not become active. | Retain the governed pre-activation outcome and follow applicable policy governance. | Leave Management |
| Activation is not yet appropriate | No effective policy is established from this scenario. | Preserve the applicable pre-activation state. | Leave Management |

No technical retries, queues, rollback, infrastructure failure handling or implementation mechanics are defined.

## 12. Business Rules and Decision Constraints
- The Leave Policy Aggregate owns policy decisions, rule evaluation and policy version lifecycle.
- The locked policy lifecycle is Draft → UnderReview → Approved → Scheduled → Active → Superseded → Archived.
- Policies and rules support effective dating with start date, end date and version history.
- All leave decisions are controlled by Leave Management.
- External domains cannot directly mutate the Leave Policy Aggregate.

## 13. State and Lifecycle Impact
The authoritative Leave Policy lifecycle is Draft → UnderReview → Approved → Scheduled → Active → Superseded → Archived. This scenario covers establishment through activation; later supersession and archival are outside its completion boundary.

No new local lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Canonical Business Events
The locked L2 workflow requires publication of a **Policy Activated** business event after activation, but the accessible authoritative evidence does not expose a canonical event identifier/name. No event name is invented here.
### 14.2 Synchronous Business Contracts / Hand-offs
Policy activation provides governed business context to downstream leave decision workflows. The business outcome remains owned by Leave Management.

## 15. Business Authorization, Approval and Audit Obligations
Policy validation, conflict resolution and approval decisions must remain traceable. Policy version history and effective dating are part of the authoritative lifecycle. Approval evidence includes actor, timestamp and decision.

## 16. Completion Criteria and Postconditions
- Policy rules have been validated.
- Any identified conflict has been governed/resolved.
- Required approval is complete.
- The policy is activated.
- The activated policy becomes effective according to its governed effective dating/version.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
The accessible L2-E05 workflow explicitly requires a 'Policy Activated Event' but does not expose a canonical event identifier/name. This prevents inventing an event name.
### 17.2 Assumptions
None.
### 17.3 Deferred Questions
Confirm the canonical identifier/name of the policy-activation event from the authoritative event register before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft; canonical event identifier is not explicitly defined in the accessible L2 evidence.
