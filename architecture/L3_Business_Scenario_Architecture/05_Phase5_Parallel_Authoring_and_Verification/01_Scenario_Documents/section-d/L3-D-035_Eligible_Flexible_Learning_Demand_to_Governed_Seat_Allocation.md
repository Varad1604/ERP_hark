# Eligible Flexible Learning Demand to Governed Seat Allocation

**Scenario ID:** L3-D-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0756 / CAN-L3-D-036 (final L3-D-035)

---

# 2. Scenario Purpose

Allocate one available seat to one approved flexible-learning enrollment under capacity, quota, priority and concurrency rules.

# 3. Business Outcome

The approved enrollment receives exactly one governed seat, capacity statistics are updated atomically and `SeatAllocated` is published.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Flexible Learning Enrollment is Approved and the target Active opportunity has governed available capacity.

## 4.2 End Boundary

The Seat Allocation reaches Allocated and the enrollment reflects its allocated-seat outcome.

## 4.3 In Scope

- Validate approval, active opportunity and available capacity.
- Apply quota/priority rules and atomic inventory control.
- Allocate one seat and update capacity.
- Route capacity exhaustion to the separate D06 waitlist outcome.

## 4.4 Out of Scope

- Approval of the enrollment request.
- Core Section C registration mutation.
- Over-allocation or non-deterministic allocation.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Seat Allocation, flexible-learning capacity usage and allocation event. |
| Section C | Owns canonical student and core registration facts referenced during allocation. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Seat Allocation Engine | Performs deterministic, concurrency-safe seat allocation. | Authorized automated service only. |
| Academic Affairs | Authorizes governed manual or reserved-quota allocation where policy permits. | Policy-bound exception authority. |

# 7. Preconditions and Entry Criteria

- Enrollment is Approved.
- Opportunity is Active and a seat is available.
- Capacity and allocation policy versions are valid.

# 8. Trigger

Enrollment approval requests seat allocation.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive approved enrollment and validate continued eligibility/context. | D06 Seat Allocation Engine | Demand confirmed eligible. |
| 2 | Retrieve capacity and applicable reserved-category/priority rules. | D06 | Allocation basis established. |
| 3 | Lock seat inventory and revalidate availability. | Seat Allocation Engine | Concurrent over-allocation prevented. |
| 4 | Allocate exactly one seat and update capacity statistics atomically. | Seat Allocation Engine | Seat Allocation becomes Allocated. |
| 5 | Publish allocation and notify student. | D06 | SeatAllocated. |
| 6 | If capacity is exhausted, do not allocate; invoke governed waitlist evaluation. | D06 | No negative capacity or duplicate seat. |

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
| Seat Allocation | Available | Reserve if applicable, then atomically allocate to one approved enrollment. | Available → optional Reserved → Allocated | D06 | L2-D06 Aggregate 2; W26–W27 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| SeatAllocated / SeatReserved | D06 | Flexible Learning Enrollment, Section C and authorized consumers | Records governed reservation/allocation outcome. | L2-D06 W26–W27; Event Catalogue |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | Open Electives & MOOCs | Owns canonical student and core registration facts referenced during allocation. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Performs deterministic, concurrency-safe seat allocation. | Seat Allocation Engine | Authorized automated service only. | Open Electives & MOOCs |
| Authorizes governed manual or reserved-quota allocation where policy permits. | Academic Affairs | Policy-bound exception authority. | Open Electives & MOOCs |

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

- The approved enrollment receives exactly one governed seat, capacity statistics are updated atomically and `SeatAllocated` is published.
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
