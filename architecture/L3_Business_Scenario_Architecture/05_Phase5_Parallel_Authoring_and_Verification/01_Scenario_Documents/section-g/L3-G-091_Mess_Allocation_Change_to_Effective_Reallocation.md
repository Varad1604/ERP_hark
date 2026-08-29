# Mess Allocation Change to Effective Reallocation

**Scenario ID:** L3-G-091  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, H  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-091 |
| Canonical Title | Mess Allocation Change to Effective Reallocation |
| Primary Owner | Hostel Management |
| Participating Domains | G, H |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern an authorised change to an existing resident mess allocation so that the revised dining assignment becomes effective while preserving the prior allocation history.

## 3. Business Outcome

The authorised change is completed and the resident has one effective current Mess Allocation, with the prior allocation retained as historical information.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a change to an existing Mess Allocation is requested or otherwise required under hostel operations.

### 4.2 End Boundary

The scenario ends when the revised allocation is effective and the allocation history records the change.

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
| Resident / Student | Is the subject of the allocation change. |
| Hostel Administrator | Reviews and executes the operational allocation change. |
| Hostel Warden | Provides configured hostel governance/approval where required. |
| Notifications | May communicate the revised authoritative outcome. |

## 7. Preconditions and Entry Criteria

- The resident has an existing Mess Allocation.
- The requested change is permitted by configured hostel policy.
- A valid revised assignment can be established.

## 8. Trigger

A change to an existing Mess Allocation is initiated.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain / Role | Result |
|---|---|---|---|
| 1 | Hostel Management identifies the current Mess Allocation and the requested change. | G03 | Existing allocation context is established. |
| 2 | The proposed change is evaluated against configured mess allocation rules and applicable dining context. | G03 | A permitted revised assignment is identified. |
| 3 | Required hostel approval is completed where configured. | G03 | The change is authorised. |
| 4 | Hostel Management makes the revised Mess Allocation effective for the applicable period. | G03 | The new dining assignment is current. |
| 5 | The previous allocation remains historically traceable and the revised outcome is available to authorised notification consumers. | G03 / H02 | Reallocation is complete. |

## 10. Alternate Business Flows

| Condition | Business Path | Outcome |
|---|---|---|
| Proposed change is not permitted. | Retain the current valid allocation and close the change request under the configured process. | No invalid reallocation occurs. |
| Revised assignment cannot be established. | Keep the current allocation unchanged until a valid business resolution exists. | Existing authoritative assignment is preserved. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution | Owner |
|---|---|---|---|
| Requested change conflicts with configured allocation policy. | Reallocation cannot be authorised. | Apply the configured review/decision path. | G03 |
| Revised assignment is unavailable. | Current allocation must not be replaced by an invalid assignment. | Preserve the current allocation and resolve through hostel operations. | G03 |
| Notification is unavailable. | Communication is affected, not the authoritative allocation. | Preserve the effective reallocation; downstream communication remains separate. | G03 / H02 |

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Effect |
|---|---|---|
| Mess Allocation owns assignment, effective period and allocation status. | G03 | Reallocation changes the G03-owned allocation lifecycle. |
| Mess allocation rules are configuration driven. | G03 | The change is evaluated using institutional hostel configuration. |
| Historical residency/operational information is not deleted. | G03 | Prior allocation remains traceable. |

## 13. State and Lifecycle Impact

G03 defines Mess Allocation as an aggregate with assignment, effective period and allocation status. The retrieved source does not expose a dedicated canonical state-transition table for reallocation, so no new lifecycle state names are introduced. The business lifecycle impact is the replacement of the effective assignment with a governed successor allocation while retaining history.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03 requires significant operational transitions to publish immutable domain events. The exact canonical event name for Mess Allocation change/reallocation was not exposed in the retrieved material and is therefore not invented. The authoritative business fact remains the effective revised allocation published by G03.

### 14.2 Synchronous Business Contracts / Hand-offs

Cross-domain hand-offs must use the authoritative business contracts/read context defined upstream. They do not transfer ownership. Exact event names are included only where exposed by the authoritative source; otherwise they are explicitly deferred rather than invented.

## 15. Business Authorization, Approval and Audit Obligations

- Hostel Management owns the reallocation decision under configured policy.
- Required approval is determined by the configured hostel workflow.
- Allocation history and the effective period are auditable.
- No technical authorization or notification-delivery mechanism is defined at L3.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Existing allocation identified.
- Change evaluated under configured policy.
- Required decision completed.
- Revised Mess Allocation becomes effective.
- Previous allocation remains historically traceable.

### 16.2 Postconditions

The authoritative hostel outcome is established, historically traceable, and remains owned by Hostel Management. Participating domains retain their canonical ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical Mess Allocation change event/state names are not exposed in the retrieved L2-G03 material. | Exact names cannot safely be asserted. | Confirm against the authoritative G03 event/state catalogue before Architecture Approval/Lock. |

### 17.2 Assumptions

Only source-supported assumptions are used. Configuration values, approval paths, and operational constraints remain institution-configurable where L2-G03 states they are configuration driven.

### 17.3 Deferred Questions

Implementation mechanics and any exact canonical event names not exposed in the retrieved authoritative L2 material are deferred to the appropriate downstream architecture level or source confirmation.

