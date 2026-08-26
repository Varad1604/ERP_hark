# Multi-Domain Action Plan to Verified Closure

# 1. Scenario Metadata

**Scenario ID:** L3-G-006  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance; applicable G01–G10 operational action owners  
**Referenced By:** Campus governance reviews, risk governance, and corrective-action follow-up

# 2. Scenario Purpose

Coordinate a G00 governance action plan from a recorded governance finding through assignment, source-domain execution, progress governance, verification, and formal closure without G00 taking ownership of operational execution.

# 3. Business Outcome

Responsible domains execute assigned governance actions, completion is verified, and the action plan is closed

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A governance finding requires corrective action.

## 4.2 End Boundary

The scenario ends when the Multi-Domain Action Plan reaches `Closed` after the assigned governance actions have been executed by their responsible owners and completion has been verified.

## 4.3 In Scope

- One Multi-Domain Action Plan linked to one governance finding.
- Action assignment, responsible-domain/owner references, due dates, dependencies, and progress governance.
- Source-domain execution evidence.
- Verification and closure.
- Source-defined Action Plan lifecycle and events.

## 4.4 Out of Scope

- Operational execution inside G01–G10.
- Project-management execution.
- Changing the source governance finding after action-plan creation.
- Inventing action states, verification roles, or events beyond G00.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Preceded by | L3-G-003 — Campus Operational Review to Governance Closure | A review may create the governance finding that requires an action plan. |
| Related | L3-G-005 — Operational Risk Identification to Risk Closure | Risk mitigation may be coordinated through action-plan governance. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Campus Operations Governance / G | Owns the Multi-Domain Action Plan, assignment/progress/verification/closure governance, and action-plan history. |
| Responsible Campus Operations domains within Section G | Execute their own assigned operational actions and supply progress/completion evidence; participation does not transfer their operational ownership to G00. |

The frozen participating-domain value remains exactly `G`.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Source-Supported Authority |
|---|---|---|
| Governance Coordinator | Creates and coordinates the Action Plan. | L2-G00 authorization matrix explicitly assigns `Create Action Plan` to this role. |
| Quality Assurance Officer | Verifies action completion. | L2-G00 authorization matrix explicitly assigns `Verify Action Completion` to this role. |
| Responsible Owner | Performs the assigned governance action within the responsible operational domain. | L2-G00 Action Plan information explicitly includes `Responsible Owner`; no broader authority is inferred. |

# 7. Preconditions and Entry Criteria

- One governance finding exists and requires corrective action.
- The Action Plan can reference that finding as required by the G00 invariant.
- Responsible domain(s), responsible owner(s), due-date policy, dependencies, and verification rules can be established.
- Operational execution remains within each responsible source domain.

# 8. Trigger

A governance finding requires corrective action

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create one Multi-Domain Action Plan referencing the governance finding. | Governance Coordinator / G00 | The plan enters `Created`. |
| 2 | Establish the assigned actions, responsible domains/owners, due dates, dependencies, and verification expectations. | Governance Coordinator / G00 | The action boundary is established. |
| 3 | Assign the actions to their responsible owners. | Governance Coordinator / G00 | The plan reaches `Assigned`. |
| 4 | Responsible operational domains perform their assigned work under their own workflows while G00 records governance progress. | Responsible G domains; G00 | The plan progresses to `In Progress` without ownership transfer. |
| 5 | Record progress, delays, blockers, dependencies, and completion evidence as governance references. | G00 | Current action status is governed and traceable. |
| 6 | Where an action becomes overdue, record/publish the source-defined overdue governance fact and apply configured follow-up. | G00 | Overdue governance is visible without inventing a new lifecycle state. |
| 7 | Verify completion evidence against the configured verification rules. | Quality Assurance Officer / G00 | The plan reaches `Verified` when the required actions are satisfactorily completed. |
| 8 | Close the verified Action Plan and preserve its full governance history. | G00 | The plan reaches `Closed` and the frozen business outcome is achieved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Reassignment required | The responsible owner changes under governed conditions | Reassign the action while preserving prior ownership history. | The plan continues within its existing lifecycle. | G00 |
| Verification not satisfied | Completion evidence does not meet the configured verification rule | Do not enter `Verified`; continue action/progress governance. | The plan remains open. | G00 / responsible source domain |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Governance finding reference is absent | G00 invariant is violated | Do not create a standalone Action Plan; correct the governance reference first. | G00 |
| Responsible owner is missing | Action assignment cannot be governed | Resolve/reassign ownership before valid assignment or continuation. | G00 |
| Action execution is incomplete | Verification and closure are not permitted | Continue source-domain execution and G00 progress governance. | Responsible source domain / G00 |
| Verification fails | Closure evidence is insufficient | Keep the plan outside `Verified`/`Closed` until corrective work is completed and reverified. | G00 / responsible source domain |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Action Plan shall reference one governance finding. | G00 | A plan cannot exist without its source finding. | L2-G00 §13 |
| G00 action tracking is governance only; project/operational execution is outside G00. | G00 / source domains | Responsible domains execute the assigned work. | L2-G00 §4, §19 |
| The Action Plan lifecycle is `Created → Assigned → In Progress → Verified → Closed`. | G00 | Only source-defined states may be used. | L2-G00 §23 |
| Verification rules, assignment policies, due-date policies, and escalation rules are configurable. | G00 | Active governance configuration determines handling. | L2-G00 §26 |
| Governance history is immutable and all transitions are audited. | G00 | Assignment, progress, verification, and closure remain reconstructable. | L2-G00 §13, §27 |

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Multi-Domain Action Plan | Created | Assigned, progressed, verified, and closed | `Created → Assigned → In Progress → Verified → Closed` | G00 |
| Source-domain operational work | Domain-owned state | Executed under the source domain’s own lifecycle | No G00-owned state change | Respective G operational domain |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Scenario Purpose |
|---|---|---|
| `ActionPlanCreated` | Campus Operations Review (G00) | Represents creation of the governance Action Plan. |
| `ActionAssigned` | Campus Operations Review (G00) | Represents governed assignment. |
| `ActionOverdue` | Campus Operations Review (G00) | Represents an overdue action where applicable. |
| `ActionVerified` | Campus Operations Review (G00) | Represents successful completion verification. |
| `ActionPlanClosed` | Campus Operations Review (G00) | Represents formal plan closure. |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Hand-off | Purpose |
|---|---|---|---|
| G00 | Responsible G operational domain/owner | Assigned governance action, due date, dependencies, and verification expectations | Coordinates corrective action without taking execution ownership. |
| Responsible G domain | G00 | Progress and completion evidence | Supports governance monitoring and verification. |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

`Governance Coordinator` is source-authorized to create the Action Plan. `Quality Assurance Officer` is source-authorized to verify completion. L2-G00 does not name a separate universal actor for the final `Close Action Plan` service, so no additional closer role is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

The source explicitly distinguishes operational execution from G00 governance and separately identifies Quality Assurance verification. No additional separation-of-duty rule is invented.

## 15.3 Business Audit and Evidence Obligations

The source finding, assignments, responsible domains/owners, due dates, progress, overdue conditions, dependencies, completion evidence, verification result, lifecycle transitions, and closure must remain in immutable governance history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Action Plan references one governance finding.
- Assigned actions have been executed by their responsible domains.
- Required completion evidence has been verified by the source-supported verification authority.
- The Action Plan reaches `Closed` through `Verified`.
- Action-plan history and canonical events are preserved.

## 16.2 Postconditions

- A verified and closed Multi-Domain Action Plan exists.
- Source-domain operational ownership remains unchanged.
- The governance finding and action history remain traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
