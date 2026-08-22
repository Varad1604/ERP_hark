# Mess Allocation to Active Dining Assignment

**Scenario ID:** L3-G-090  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, H  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Govern assignment of an eligible hostel resident to a hostel mess or dining facility so that the resident has an effective, active dining assignment under Hostel Management policy.

## 3. Business Outcome

The eligible resident has an active Mess Allocation with an effective period and an authoritative dining assignment recorded by Hostel Management.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a mess allocation is required for an eligible hostel resident.

### 4.2 End Boundary

The scenario ends when the Mess Allocation is active and the dining assignment is effective.

### 4.3 In Scope

- Complete business outcome from trigger through closure/effective outcome.
- Applicable Hostel Management decisions, rules, lifecycle impact, evidence and history.
- Required cross-domain business collaboration without transferring ownership.

### 4.4 Out of Scope

- APIs, databases, services, Business Data Structures, queues, technical retries, deployment or infrastructure mechanics.
- Re-definition of upstream domain ownership or business rules.
- Any lifecycle owned by another domain except where its authoritative outcome is consumed.

### 4.5 Related Scenarios

Related hostel admission, residency, room, leave, discipline, complaint and exit scenarios remain separate canonical outcomes.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Hostel Management (G03) | Owns the scenario's authoritative hostel business outcome and Mess Allocation / residency-policy operational capability as applicable. |
| Notifications (H02) | Consumes the authoritative hostel outcome for governed communication where configured; does not own Mess Allocation. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Resident / Student | Is the subject of the dining assignment and may participate in the allocation process. |
| Hostel Administrator | Performs operational mess allocation and review. |
| Hostel Warden | Provides hostel governance where the configured approval path requires it. |
| Notifications | May communicate the authoritative allocation outcome; it does not decide the allocation. |

## 7. Preconditions and Entry Criteria

- The resident is within Hostel Management's applicable residency scope.
- A mess/dining allocation capability is applicable to the resident.
- Mess allocation rules and operational constraints are available through configured hostel policy.

## 8. Trigger

A mess allocation is initiated for an eligible hostel resident.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain / Role | Result |
|---|---|---|---|
| 1 | Hostel Management identifies the resident and applicable dining/mess allocation context. | G03 | Allocation can be evaluated. |
| 2 | The applicable mess allocation rules and availability context are reviewed. | G03 | A permitted assignment is identified. |
| 3 | Where approval is required by configured policy, the authorised hostel authority reviews the allocation. | G03 | Allocation reaches an authorised decision. |
| 4 | Hostel Management establishes the resident's Mess Allocation with its effective period and allocation status. | G03 | The dining assignment becomes active. |
| 5 | The authoritative outcome is made available for governed notification where applicable. | G03 / H02 | Relevant communication can consume the outcome. |

## 10. Alternate Business Flows

| Condition | Business Path | Outcome |
|---|---|---|
| No permitted mess assignment is available. | Keep the resident without an active assignment and resolve through the configured allocation process. | No invalid dining assignment is established. |
| Approval is required and not granted. | Do not activate the allocation. | Resident remains without the proposed active assignment. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution | Owner |
|---|---|---|---|
| Allocation information is incomplete. | An authoritative assignment cannot be established. | Correct the business information before activation. | G03 |
| No permitted dining assignment can be established. | Active allocation cannot be completed. | Resolve under the configured mess-allocation process without creating an unsupported assignment. | G03 |
| Notification cannot be consumed. | Communication is affected, but the hostel outcome remains authoritative. | Preserve the active Mess Allocation; communication remains downstream. | G03 / H02 |

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Effect |
|---|---|---|
| Mess Allocation owns mess assignment, effective period and allocation status. | G03 | The dining assignment is authoritative within Hostel Management. |
| Mess allocation rules are configuration driven. | G03 | Institutional operational policy determines applicable allocation conditions. |
| Notifications do not own hostel business facts. | H02 | Notification participation cannot change allocation ownership. |

## 13. State and Lifecycle Impact

The authoritative G03 source defines Mess Allocation as an aggregate owning:
- Mess Assignment
- Effective Period
- Allocation Status

The retrieved source does not expose a dedicated canonical Mess Allocation state-transition table beyond these lifecycle attributes. Therefore no additional state names are invented. The business impact is transition to an active/effective dining assignment.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03's locked architecture requires significant operational transitions to publish immutable domain events. The exact canonical Mess Allocation event name was not exposed in the retrieved source, so it is intentionally not invented. The authoritative business fact is the active Mess Allocation outcome published by G03 for approved consumers.

### 14.2 Synchronous Business Contracts / Hand-offs

Cross-domain hand-offs must use the authoritative business contracts/read context defined upstream. They do not transfer ownership. Exact event names are included only where exposed by the authoritative source; otherwise they are explicitly deferred rather than invented.

## 15. Business Authorization, Approval and Audit Obligations

- Hostel Management is the authority for the Mess Allocation business decision under its configured operational policy.
- Any approval path is governed by the configured hostel workflow rather than a technical permission model.
- The allocation, decision and effective period must be auditable in Hostel Management.
- Notification delivery does not become part of the authoritative completion condition.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Eligible resident identified.
- Applicable allocation rules evaluated.
- Required approval completed.
- Mess Allocation established with effective period/status.
- Active dining assignment is authoritative and historically traceable.

### 16.2 Postconditions

The authoritative hostel outcome is established, historically traceable, and remains owned by Hostel Management. Participating domains retain their canonical ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical Mess Allocation event name is not exposed in the retrieved L2-G03 material. | Naming one would risk fabrication. | Confirm the canonical event catalogue before Architecture Approval/Lock. |

### 17.2 Assumptions

Only source-supported assumptions are used. Configuration values, approval paths, and operational constraints remain institution-configurable where L2-G03 states they are configuration driven.

### 17.3 Deferred Questions

Implementation mechanics and any exact canonical event names not exposed in the retrieved authoritative L2 material are deferred to the appropriate downstream architecture level or source confirmation.

