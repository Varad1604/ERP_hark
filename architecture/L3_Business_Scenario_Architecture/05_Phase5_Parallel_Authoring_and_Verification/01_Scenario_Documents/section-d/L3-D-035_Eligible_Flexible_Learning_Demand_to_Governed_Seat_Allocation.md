# Eligible Flexible Learning Demand to Governed Seat Allocation

# 1. Scenario Metadata

**Scenario ID:** L3-D-035  
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
**Frozen Stage 3 Source:** P2W-D-0756 / CAN-L3-D-036 (final L3-D-035)

---

# 2. Scenario Purpose

Allocate available flexible-learning capacity to eligible Approved demand under governed D06 priority/capacity rules without exceeding capacity or creating duplicate allocation.

# 3. Business Outcome

Eligible Approved demand receives one governed Seat Allocation where capacity permits; otherwise the demand remains unallocated and may proceed to the D06 waitlist outcome.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A D06 Flexible Learning Enrollment is Approved and requires seat allocation.

## 4.2 End Boundary

The demand has an Allocated or governed Unallocated/no-seat outcome and the applicable D06 allocation fact is published.

## 4.3 In Scope

- Establish authoritative capacity and eligibility/priority context.
- Apply D06 capacity, reservation and priority policy.
- Confirm one valid allocation where capacity permits.
- Route eligible unallocated demand to the governed no-seat/waitlist path.

## 4.4 Out of Scope

- Locks, transactions, atomic updates or concurrency control.
- Exceeding capacity.
- Changing Section C-owned facts.

## 4.5 Related Scenarios — Optional

L3-D-036 handles the governed waitlist outcome. L3-D-048 handles a detected allocation conflict.

# 5. Ownership and Participating Domains

D06 owns capacity, Seat Allocation and allocation decision. Section C supplies canonical student/core-registration references only.

# 6. Business Actors, Roles and Responsibilities

Where policy requires a human exceptional allocation decision, Academic Affairs may act within its assigned authority. Normal allocation evaluation is a D06 business responsibility and is not represented by a `Seat Allocation Engine` actor.

# 7. Preconditions and Entry Criteria

Approved enrollment exists; capacity policy is active; relevant eligibility/priority references are authoritative.

# 8. Trigger

Approved eligible demand becomes ready for D06 allocation.

# 9. Main Business Flow

1. D06 establishes current governed capacity and Approved enrollment demand.
2. D06 validates eligibility, priority, reservations and applicable capacity policy.
3. D06 determines the next valid allocation position under policy.
4. Where capacity permits, D06 records one Seat Allocation and the applicable allocation event.
5. Where no capacity remains, D06 records the unallocated business outcome and routes eligible demand to L3-D-036 where waitlisting is enabled.

# 10. Alternate Business Flows

## 10.1 Capacity Exhausted

Where all valid capacity is consumed, eligible Approved demand reaches the governed Unallocated/no-seat outcome and may proceed to the D06 waitlist path; no over-allocation occurs.

# 11. Failure, Interruption and Business Recovery

Missing/invalid capacity, eligibility or priority facts block allocation until authoritative business evidence is resolved. An interrupted allocation preserves the last authoritative D06 capacity/allocation position. Any detected conflict follows L3-D-048 rather than technical rollback/retry behaviour here.

# 12. Business Rules and Decision Constraints

D06 owns capacity and allocation; capacity must not be exceeded; duplicate allocation must not be created; Section C facts remain externally owned; repeated same business fact must not create duplicate institutional allocation outcomes. Technical locks/atomicity/idempotency are deferred.

# 13. State and Lifecycle Impact

Seat Allocation: eligible Approved demand → Allocated or governed Unallocated/no-seat outcome under D06. Source: L2-D06 seat-allocation workflows and Aggregate 2.

# 14. Business Events and Cross-Domain Collaboration

Existing D06 allocation/waitlist events and publishers remain unchanged as defined in L2-D06. Section C supplies authoritative eligibility references without ownership transfer.

# 15. Business Authorization, Approval and Audit Obligations

D06 applies normal allocation policy. Any exceptional human allocation decision requires the business authority explicitly established by D06 policy. Audit retains demand, eligibility/priority, capacity position, applicable policy and final allocation/no-seat outcome.

# 16. Completion Criteria and Postconditions

Capacity remains consistent; no duplicate/over-capacity allocation exists; demand has a governed outcome; D06 ownership is unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Locking, transactions, concurrency control, atomic updates, retry and technical deduplication are deferred to L4/L5.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
