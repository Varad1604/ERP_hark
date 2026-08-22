# Seat Allocation Conflict Resolution

**Scenario ID:** L3-D-048  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0768 / CAN-L3-D-048

---

# 2. Scenario Purpose

Resolve a concurrent D06 seat-allocation conflict without exceeding capacity, duplicating allocation or leaving inventory inconsistent.

# 3. Business Outcome

The conflicting allocation is rolled back, capacity is deterministically recalculated and the eligible allocation is retried or routed to a governed no-seat outcome, with `SeatConflictResolved` published.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A seat-allocation transaction detects concurrency conflict or fails consistency validation.

## 4.2 End Boundary

Seat inventory and enrollment allocation are consistent and the conflict resolution is recorded; unresolved exhaustion routes to waitlist/no-allocation.

## 4.3 In Scope

- Detect and isolate concurrency conflict.
- Roll back uncommitted conflicting effect.
- Recalculate capacity and revalidate demand.
- Retry atomically or route to governed alternate outcome.

## 4.4 Out of Scope

- Exceeding capacity to satisfy both requests.
- Changing Section C-owned registration facts.
- Erasing failed-attempt evidence.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns flexible-learning seat inventory, conflict resolution, retry and allocation consistency. |
| Section C | Owns canonical student/core-registration facts referenced during revalidation. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Seat Allocation Engine | Detects, rolls back, recalculates and retries atomically. | Automated execution. |
| Academic Affairs | Reviews persistent conflict or applies authorized manual allocation policy. | Policy-bound exception authority. |

# 7. Preconditions and Entry Criteria

- A D06 allocation attempt failed because of concurrent demand or inventory inconsistency.
- Last committed allocation and capacity state are recoverable.

# 8. Trigger

Concurrent allocation conflict is detected.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Capture conflict and identify affected requests/inventory version. | Seat Allocation Engine | Conflict case and audit correlation exist. |
| 2 | Roll back the uncommitted conflicting allocation effect. | D06 | Last committed state restored. |
| 3 | Recalculate available, occupied and reserved capacity deterministically. | D06 | Canonical inventory established. |
| 4 | Revalidate enrollment eligibility and priority. | Seat Allocation Engine using references | Retry candidate confirmed. |
| 5 | Retry one atomic allocation under current capacity. | Seat Allocation Engine | Seat allocated or no-seat result. |
| 6 | If capacity exhausted/persistent conflict, route to waitlist or authorized manual review. | D06 / Academic Affairs | No over-allocation occurs. |
| 7 | Publish resolution with final inventory outcome. | D06 | SeatConflictResolved. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Open Electives & MOOCs |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Open Electives & MOOCs |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Open Electives & MOOCs |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Open Electives & MOOCs | Prevents cross-domain ownership leakage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Open Electives & MOOCs | Prevents governance bypass. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Published or historical business facts remain immutable. | Open Electives & MOOCs | Preserves auditability and lineage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Seat Allocation Conflict | Detected | Rollback, recalculate and retry from last committed truth. | Resolved with Allocated or governed Unallocated outcome | D06 | L2-D06 W69 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| SeatConflictResolved | D06 | Seat allocation, waitlist, Section C and audit consumers | Records restored inventory consistency and conflict outcome. | L2-D06 W69; D06-E60 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | Open Electives & MOOCs | Owns canonical student/core-registration facts referenced during revalidation. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Detects, rolls back, recalculates and retries atomically. | Seat Allocation Engine | Automated execution. | Open Electives & MOOCs |
| Reviews persistent conflict or applies authorized manual allocation policy. | Academic Affairs | Policy-bound exception authority. | Open Electives & MOOCs |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Open Electives & MOOCs |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Open Electives & MOOCs |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Open Electives & MOOCs |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The conflicting allocation is rolled back, capacity is deterministically recalculated and the eligible allocation is retried or routed to a governed no-seat outcome, with `SeatConflictResolved` published.
- All required authority, validation and audit evidence is retained.
- Applicable completed business facts are published exactly once in business effect.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical decisions and prior states remain immutable.
- Downstream domains consume facts without modifying the owning aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
