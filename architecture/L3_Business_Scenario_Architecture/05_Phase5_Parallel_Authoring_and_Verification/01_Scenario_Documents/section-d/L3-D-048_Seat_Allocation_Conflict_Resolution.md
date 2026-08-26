# Seat Allocation Conflict Resolution

# 1. Scenario Metadata

**Scenario ID:** L3-D-048  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0768 / CAN-L3-D-048

---

# 2. Scenario Purpose

Resolve a D06 seat-allocation conflict so that flexible-learning capacity remains consistent, no approved enrollment receives a duplicate seat, and no capacity is exceeded.

# 3. Business Outcome

The allocation conflict is resolved against the latest authoritative D06 capacity position. One eligible allocation may be confirmed where capacity permits; otherwise the demand follows the governed no-seat/waitlist outcome. `SeatConflictResolved` is published by D06.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

D06 detects that competing or inconsistent seat-allocation facts prevent a trustworthy allocation outcome.

## 4.2 End Boundary

D06 capacity and enrollment-allocation facts are consistent and the conflict has a governed Allocated or Unallocated/waitlist outcome.

## 4.3 In Scope

- Identify the conflicting allocation demand and affected capacity position.
- Preserve the last authoritative D06 allocation and capacity facts.
- Reassess current capacity, eligibility and priority under the applicable allocation policy.
- Confirm one valid allocation where capacity permits or route eligible unallocated demand to the governed no-seat/waitlist outcome.
- Record the resolved business outcome and evidence.

## 4.4 Out of Scope

- Transaction, lock, rollback, retry or concurrency-control mechanisms.
- Exceeding capacity to satisfy competing requests.
- Changing Section C-owned student or core-registration facts.
- Erasing prior conflict evidence or historical allocation facts.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related allocation path | L3-D-035 — Eligible Flexible Learning Demand to Governed Seat Allocation | Governs the normal D06 seat-allocation outcome. |
| Related no-seat path | L3-D-036 — Unallocated Eligible Demand to Governed D06 Waitlist Outcome | Governs eligible demand that cannot receive a seat. |

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns flexible-learning capacity, Seat Allocation facts, conflict resolution and the final allocation/no-seat outcome. |
| Section C | Owns canonical student and core-registration facts referenced during revalidation. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Affairs | Reviews a conflict only where institutional policy requires a governed manual or exceptional allocation decision. | Policy-bound business decision authority where explicitly applicable. |

Normal conflict detection, capacity reassessment and consistency restoration are D06 business responsibilities and are not represented as a technical `Seat Allocation Engine` actor at L3.

# 7. Preconditions and Entry Criteria

- An allocation conflict or inconsistent allocation/capacity fact has been identified within D06.
- The latest authoritative D06 capacity and allocation facts are available.
- Relevant Section C eligibility references remain available where revalidation is required.

# 8. Trigger

D06 identifies competing or inconsistent allocation facts that prevent a trustworthy seat-allocation outcome.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the conflict case and identify the affected enrollment demand and D06 capacity facts. | D06 | Conflict scope is traceable. |
| 2 | Preserve the last authoritative allocation and capacity position while the conflict is assessed. | D06 | Prior valid business truth remains intact. |
| 3 | Reassess available, occupied and reserved capacity under the governing D06 allocation policy. | D06 | Current authoritative capacity position is established. |
| 4 | Revalidate enrollment eligibility and priority using authoritative references where required. | D06 using Section C facts | Eligible demand is confirmed or rejected. |
| 5 | Where capacity permits, confirm one governed allocation for the eligible demand. | D06 | Seat is Allocated without duplicate allocation or capacity breach. |
| 6 | Where capacity does not permit allocation, route eligible demand to the supported D06 no-seat/waitlist outcome. | D06 | Governed unallocated outcome is established. |
| 7 | Record and publish the conflict-resolution outcome. | D06 | `SeatConflictResolved`. |

# 10. Alternate Business Flows

## 10.1 No Seat Available After Reassessment

Where the conflict is resolved but no governed capacity remains, the affected eligible demand does not receive a seat and proceeds to the D06 waitlist/no-allocation path. Capacity remains consistent and `SeatConflictResolved` records the conflict outcome.

## 10.2 Policy-Governed Exceptional Review

Where the authoritative D06 policy explicitly requires a human decision for a reserved or exceptional allocation, Academic Affairs performs that business review before the final allocation/no-seat outcome is recorded.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Current capacity or allocation facts cannot be established | The conflict cannot be resolved reliably. | Preserve the last authoritative D06 position and resolve the missing/inconsistent business evidence before continuing. | D06 |
| Eligibility or priority reference cannot be validated | The affected demand cannot proceed to allocation. | Resolve the authoritative reference with its canonical owner before reassessment continues. | D06 with Section C participation where applicable |
| Required exceptional decision is unavailable or rejected | The requested exceptional allocation cannot proceed. | Retain the prior authoritative position and follow the governed unallocated/waitlist or later review path supported by policy. | D06 |
| Conflict resolution is interrupted before a final business outcome | No resolved allocation/no-seat outcome has occurred. | Preserve the last valid business position and continue only when the blocking business condition is resolved. | D06 |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only D06 may change D06-owned allocation and capacity facts. | D06 | Prevents cross-domain ownership leakage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Capacity may not be exceeded and duplicate allocation may not be created. | D06 | Defines the business integrity requirement of conflict resolution. | L2-D06 W69 and seat-allocation governance |
| Eligibility and priority use authoritative references and governed policy. | D06 | Prevents unsupported allocation decisions. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Prior authoritative business facts and conflict evidence remain historically traceable. | D06 | Prevents destructive reconciliation. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Seat Allocation Conflict | Detected | Reassess authoritative capacity/eligibility and determine the governed outcome. | Resolved with Allocated or governed Unallocated outcome | D06 | L2-D06 W69 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `SeatConflictResolved` | D06 | Seat allocation, waitlist, Section C and authorized audit consumers | Records the restored D06 allocation/capacity consistency and conflict outcome. | L2-D06 W69; D06-E60 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | D06 | Authoritative student/core-registration facts referenced during eligibility revalidation. | Enable D06 decision-making without transferring Section C ownership. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role / Domain | Constraint | Owner |
|---|---|---|---|
| Reassess and resolve normal allocation conflict | D06 | Must apply the authoritative D06 capacity, eligibility and priority rules. | D06 |
| Decide an exceptional/manual allocation where policy requires | Academic Affairs | Authority exists only where established by the governing D06 policy. | D06 |

## 15.2 Separation of Duties, Overrides and Exceptions

A business exception may override the normal allocation path only where authoritative policy grants such authority. The exception must preserve justification, affected capacity, prior allocation facts and final outcome.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Conflict and resolution | Conflicting demand, prior authoritative capacity/allocation facts, reassessment evidence, policy and final outcome. | D06 |
| Cross-domain traceability | Section C references used for eligibility/priority revalidation. | D06 with Section C ownership retained |
| Failure and recovery | Blocking business condition, governed resolution and final allocation/no-seat outcome. | D06 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- D06 capacity and allocation facts are consistent.
- No capacity breach or duplicate seat allocation exists as a result of this conflict.
- The affected demand has a governed Allocated or Unallocated/waitlist outcome.
- `SeatConflictResolved` is published by D06.

## 16.2 Postconditions

- D06 retains ownership of seat-allocation and capacity facts.
- Section C retains ownership of student/core-registration facts.
- Prior conflict and decision evidence remain traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issue identified by this L3-purity correction.

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Transaction isolation, locking, rollback, retries, atomic updates, concurrency control and technical deduplication are deferred to L4/L5.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** Section D canonical register; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; `L3_Business_Scenario_Template_v2.0.md`; `Architecture_Level_Validation_Standards.md`.
